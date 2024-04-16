#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    std::{
        env,
        path::PathBuf,
    },
    itertools::Itertools as _,
    tokio::{
        io::{
            self,
            AsyncReadExt as _,
            AsyncWriteExt as _,
            BufReader,
        },
        process::Command,
    },
    wheel::{
        fs::{
            self,
            File,
        },
        traits::{
            AsyncCommandOutputExt as _,
            IoResultExt as _,
        },
    },
};

fn format_segment(start_address: usize, data: Vec<u8>) -> String {
    format!("    ({:#010x}, ::std::borrow::Cow::Borrowed(&[{:#04x}])),\n", start_address, data.into_iter().format(", "))
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Wheel(#[from] wheel::Error),
    #[error("assets/base.n64 should be 0x4000000 bytes (64 MiB), but yours is {0:#x} bytes ({} MiB). Make sure you have an uncompressed base ROM (use bin/Decompress from OoTR to decompress one).", .0 / 1024_u64.pow(2))]
    BaseRomSize(u64),
}

#[wheel::main(debug)]
async fn main() -> Result<(), Error> {
    println!("cargo::rerun-if-changed=assets/asm");
    println!("cargo::rerun-if-changed=assets/base.n64");
    // give a better error when a compressed base rom is supplied
    let base_rom_size = fs::metadata("assets/base.n64").await?.len();
    if base_rom_size != 0x0400_0000 {
        return Err(Error::BaseRomSize(base_rom_size))
    }
    // assemble patches
    Command::new("armips").arg("assets/asm/main.asm").check("armips").await?;
    // create a diff of the patched rom
    let (base_rom, patched_rom) = tokio::try_join!(File::open("assets/base.n64"), File::open("assets/generated/asm-patched.n64"))?;
    let mut base_rom = BufReader::new(base_rom);
    let mut patched_rom = BufReader::new(patched_rom);
    let mut current_segment = None;
    let rom_patch_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("rom-patch.rs");
    let mut rom_patch = File::create(&rom_patch_path).await?;
    rom_patch.write_all(b"vec![\n").await.at(&rom_patch_path)?;
    for start_address in (0..).step_by(4) {
        match tokio::try_join!(base_rom.read_u32(), patched_rom.read_u32()) {
            Ok((base_word, patched_word)) => match (&mut current_segment, base_word == patched_word) {
                (None, true) => {}
                (None, false) => current_segment = Some((start_address, patched_word.to_be_bytes().to_vec())),
                (Some(_), true) => {
                    let (start_address, data) = current_segment.take().unwrap();
                    rom_patch.write_all(format_segment(start_address, data).as_bytes()).await.at(&rom_patch_path)?;
                }
                (Some((_, data)), false) => data.extend_from_slice(&patched_word.to_be_bytes()),
            },
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => Err(e).at_unknown()?,
        }
    }
    if let Some((start_address, data)) = current_segment {
        rom_patch.write_all(format_segment(start_address, data).as_bytes()).await.at(&rom_patch_path)?;
    }
    rom_patch.write_all(b"]\n").await.at(&rom_patch_path)?;
    Ok(())
}
