use {
    async_compression::tokio::write::ZlibEncoder,
    crossterm::tty::IsTty as _,
    rand::prelude::*,
    tokio::io::{
        AsyncReadExt as _,
        AsyncWriteExt as _,
        stdin,
        stdout,
    },
};

const DMADATA_START: u32 = 0x7430;
const XOR_RANGE_START: u32 = 0x00b8ad30;
const XOR_RANGE_END: u32 = 0x00f029a0;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Io(#[from] tokio::io::Error),
    #[error("standard input is not a valid OoT 1.0 US ROM")]
    BaseRom,
    #[error("standard input is a TTY")]
    Stdin,
}

#[wheel::main]
async fn main() -> Result<(), Error> {
    let mut stdin = stdin();
    if stdin.is_tty() { return Err(Error::Stdin) }
    stdin.read_exact(&mut [0; 0x10]).await?; // skip to CRC
    let mut crc = [0; 0x8];
    stdin.read_exact(&mut crc).await?;
    match crc {
        [0xEC, 0x70, 0x11, 0xB7, 0x76, 0x16, 0xD7, 0x2B] => {} //TODO (regular compressed)
        [0x70, 0xEC, 0xB7, 0x11, 0x16, 0x76, 0x2B, 0xD7] => {} //TODO (byteswap compressed)
        [0x93, 0x52, 0x2E, 0x7B, 0xE5, 0x06, 0xD4, 0x27] => {} // decompressed
        _ => return Err(Error::BaseRom),
    }
    //TODO actually randomize stuff
    // write a no-op ZPF file
    let mut zpf_buf = ZlibEncoder::new(stdout());
    // header
    zpf_buf.write_all(b"ZPFv1").await?;
    zpf_buf.write_u32(DMADATA_START).await?;
    zpf_buf.write_u32(XOR_RANGE_START).await?;
    zpf_buf.write_u32(XOR_RANGE_END).await?;
    zpf_buf.write_u32(thread_rng().gen_range(XOR_RANGE_START..XOR_RANGE_END)).await?;
    // DMA updates
    zpf_buf.write_u16(0xffff).await?;
    // XOR data (empty)
    zpf_buf.shutdown().await?; // write zlib trailer
    zpf_buf.into_inner().flush().await?; // make sure data is actually written to stdout
    Ok(())
}
