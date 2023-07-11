use {
    std::{
        borrow::Cow,
        cmp::Ordering::*,
        ops::{
            Index,
            Range,
        },
    },
    async_compression::tokio::write::ZlibEncoder,
    rand::prelude::*,
    itertools::Itertools as _,
    tokio::io::{
        self,
        AsyncWrite,
        AsyncWriteExt as _,
    },
};

const DMADATA_START: u32 = 0x7430;
const XOR_RANGE: Range<usize> = 0x00b8_ad30..0x00f0_29a0;
const BLOCK_HEADER_SIZE: usize = 7;

pub(crate) struct Patch<'a> {
    base_rom: &'a [u8],
    /// A list of segments of raw data that should be changed by the patch, identified by their starting addresses and changed data.
    ///
    /// # Invariant
    ///
    /// This vector is sorted at all times, and there is a gap (of at least 1 byte) between adjacent segments.
    changed_segments: Vec<(usize, Cow<'a, [u8]>)>,
}

impl<'a> Patch<'a> {
    fn write_bytes(&mut self, start_address: usize, bytes: impl Into<Cow<'a, [u8]>>) {
        let mut data_to_insert = bytes.into();
        // merge with subsequent adjacent/overlapping segments
        let idx = self.changed_segments.binary_search_by_key(&start_address, |&(addr, _)| addr);
        let end_address = start_address + data_to_insert.len();
        let next_idx = match idx {
            Ok(idx) => idx + 1,
            Err(idx) => idx + 1,
        };
        while let Some(&(next_address, ref next_data)) = self.changed_segments.get(next_idx) {
            if next_address <= end_address {
                if next_address + next_data.len() <= end_address {
                    self.changed_segments.remove(next_idx);
                } else {
                    data_to_insert.to_mut().extend_from_slice(&self.changed_segments.remove(next_idx).1[end_address - next_address..]);
                    break
                }
            } else {
                break
            }
        }
        // merge with peceding adjacent/overlapping segments
        match idx {
            Ok(found_idx) => if data_to_insert.len() < self.changed_segments[found_idx].1.len() {
                self.changed_segments[found_idx].1.to_mut()[..data_to_insert.len()].copy_from_slice(&data_to_insert);
            } else {
                self.changed_segments[found_idx].1 = data_to_insert;
            },
            Err(insert_idx) => match insert_idx.checked_sub(1).map(|prev_idx| start_address.cmp(&(self.changed_segments[prev_idx].0 + self.changed_segments[prev_idx].1.len()))) {
                // comparing the new segment's start address to the previous segment's end address
                Some(Less) => {
                    let (prev_start, ref mut prev_segment) = self.changed_segments[insert_idx - 1];
                    let prev_segment = prev_segment.to_mut();
                    prev_segment.truncate(start_address - prev_start);
                    prev_segment.extend_from_slice(&data_to_insert);
                }
                Some(Equal) => self.changed_segments[insert_idx - 1].1.to_mut().extend_from_slice(&data_to_insert),
                Some(Greater) | None => self.changed_segments.insert(insert_idx, (start_address, data_to_insert)),
            }
        }
    }

    /// get the next XOR key. Uses some location in the source rom.
    /// This will skip of 0s, since if we hit a block of 0s, the
    /// patch data will be raw.
    fn key_next(&self, key_address: &mut usize) -> u8 {
        loop {
            *key_address += 1;
            if *key_address >= XOR_RANGE.end {
                *key_address = XOR_RANGE.start;
            }
            let key = self.base_rom[*key_address];
            if key != 0 { break key }
        }
    }

    async fn write_block_section(&self, start: usize, key_skip: u8, in_data: &[u8], is_continue: bool, writer: &mut (impl AsyncWrite + Unpin)) -> io::Result<()> {
        if !is_continue {
            writer.write_u32(start.try_into().expect("address out of range")).await?;
        } else {
            writer.write_u8(0xff).await?;
            writer.write_u8(key_skip).await?;
        }
        writer.write_u16(in_data.len().try_into().expect("block section too long")).await?;
        writer.write_all(in_data).await?;
        Ok(())
    }

    async fn write_zpf_xor_block(&self, xor_address: &mut usize, block: Range<usize>, writer: &mut (impl AsyncWrite + Unpin)) -> io::Result<()> {
        let mut new_data = Vec::with_capacity(block.end - block.start);
        let mut key_offset = 0;
        let mut continue_block = false;
        for address in block.clone() {
            let byte = self[address];
            if byte == 0 {
                // Leave 0s as 0s. Do not XOR
                new_data.push(0);
            } else {
                let mut key = self.key_next(xor_address);
                // if the XOR would result in 0, change the key.
                // This requires breaking up the block.
                if byte == key {
                    self.write_block_section(block.start, key_offset, &new_data, continue_block, writer).await?;
                    new_data = Vec::with_capacity(block.end - address);
                    continue_block = true;
                    // search for next safe XOR key
                    while byte == key {
                        key_offset += 1;
                        key = self.key_next(xor_address);
                        // if we aren't able to find one quickly, we may need to break again
                        if key_offset == 0xff {
                            self.write_block_section(block.start, key_offset, &new_data, continue_block, writer).await?;
                            new_data = Vec::with_capacity(block.end - address);
                            key_offset = 0;
                            continue_block = true;
                        }
                    }
                }
                // XOR the key with the byte
                new_data.push(byte ^ key);
                // Break the block if it's too long
                if new_data.len() == 0xffff {
                    self.write_block_section(block.start, key_offset, &new_data, continue_block, writer).await?;
                    new_data = Vec::with_capacity(block.end - address);
                    key_offset = 0;
                    continue_block = true;
                }
            }
        }
        // Save the block
        self.write_block_section(block.start, key_offset, &new_data, continue_block, writer).await?;
        Ok(())
    }

    pub(crate) async fn write_zpf(&self, writer: impl AsyncWrite + Unpin) -> io::Result<()> {
        let mut zpf_buf = ZlibEncoder::new(writer);
        // header
        zpf_buf.write_all(b"ZPFv1").await?;
        zpf_buf.write_u32(DMADATA_START).await?;
        zpf_buf.write_u32(XOR_RANGE.start.try_into().expect("address out of range")).await?;
        zpf_buf.write_u32(XOR_RANGE.end.try_into().expect("address out of range")).await?;
        let mut xor_address = thread_rng().gen_range(XOR_RANGE);
        zpf_buf.write_u32(xor_address.try_into().expect("address out of range")).await?;
        // DMA updates (none currently)
        zpf_buf.write_u16(0xffff).await?;
        // XOR data
        //TODO filter addresses to change like in Python? (e.g. exclude DMA table and bytes that stay the same other than force-patched bytes)
        let mut block = None;
        for &(start_address, ref new_data) in &self.changed_segments {
            // Starting a new block to skip unchanged bytes only actually saves space if we skip more than the size of the block header.
            match block {
                None => block = Some(start_address..start_address + new_data.len()),
                Some(old_block) if start_address > old_block.end + BLOCK_HEADER_SIZE => {
                    self.write_zpf_xor_block(&mut xor_address, old_block, &mut zpf_buf).await?;
                    block = Some(start_address..start_address + new_data.len());
                }
                Some(Range { ref mut end, .. }) => *end = start_address + new_data.len(),
            }
        }
        if let Some(block) = block {
            self.write_zpf_xor_block(&mut xor_address, block, &mut zpf_buf).await?;
        }
        zpf_buf.shutdown().await?; // write zlib trailer
        zpf_buf.into_inner().flush().await?; // make sure data is actually written to writer
        Ok(())
    }

    pub(crate) async fn write_uncompressed_rom(&self, mut writer: impl AsyncWrite + Unpin) -> io::Result<()> {
        let mut address = 0;
        for &(start_address, ref new_data) in &self.changed_segments {
            writer.write_all(&self.base_rom[address..start_address]).await?;
            writer.write_all(&new_data).await?;
            address = start_address + new_data.len();
        }
        writer.write_all(&self.base_rom[address..]).await?;
        Ok(())
    }
}

impl<'a> Index<usize> for Patch<'a> {
    type Output = u8;

    fn index(&self, address: usize) -> &u8 {
        match self.changed_segments.binary_search_by_key(&address, |&(addr, _)| addr) {
            Ok(found_idx) => &self.changed_segments[found_idx].1[0],
            Err(insert_idx) => if let Some(prev_idx) = insert_idx.checked_sub(1) {
                let (start_addr, segment) = &self.changed_segments[prev_idx];
                if address < start_addr + segment.len() {
                    &segment[address - start_addr]
                } else {
                    &self.base_rom[address]
                }
            } else {
                &self.base_rom[address]
            },
        }
    }
}

pub(crate) fn patch_rom(base_rom: &[u8]) -> Patch<'_> {
    let mut patch = Patch {
        changed_segments: include!(concat!(env!("OUT_DIR"), "/rom-patch.rs")),
        base_rom,
    };
    let binary_patches = [
        (include_bytes!("../assets/title.bin"), 0x0179_5300), // Randomizer title screen logo
    ];
    for (bytes_diff, write_address) in binary_patches {
        // unlike in the Python randomizer, these files are already decompressed
        let original_bytes = &base_rom[write_address..write_address + bytes_diff.len()];
        let new_bytes = original_bytes.iter().zip_eq(bytes_diff).map(|(original_byte, diff_byte)| original_byte ^ diff_byte).collect_vec();
        patch.write_bytes(write_address, new_bytes);
    }
    patch
}
