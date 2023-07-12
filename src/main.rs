#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    std::num::NonZeroU8,
    crossterm::tty::IsTty as _,
    tokio::io::{
        AsyncReadExt as _,
        stdin,
        stdout,
    },
};

mod patch;
mod search;

#[derive(clap::Parser)]
struct Args {
    #[clap(short, long, default_value = "1")]
    world_count: NonZeroU8,
    #[clap(short = 'p', long)]
    world: Option<NonZeroU8>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Io(#[from] tokio::io::Error),
    #[error("standard input is not a valid OoT 1.0 NTSC ROM")]
    BaseRom,
    #[error("standard input is an OoT PAL ROM, but we need an NTSC ROM")]
    PalBaseRom,
    #[error("cannot beat game")]
    Search,
    #[error("standard input is a TTY")]
    Stdin,
}

#[wheel::main(debug)]
async fn main(args: Args) -> Result<(), Error> {
    let mut stdin = stdin();
    if stdin.is_tty() { return Err(Error::Stdin) }
    let mut input_rom = vec![0; 0x0200_0000];
    stdin.read_exact(&mut input_rom).await?;
    let crc = &input_rom[0x10..0x18];
    let mut base_rom = vec![0; 0x0400_0000];
    match crc {
        [0xEC, 0x70, 0x11, 0xB7, 0x76, 0x16, 0xD7, 0x2B] | // regular compressed
        [0x70, 0xEC, 0xB7, 0x11, 0x16, 0x76, 0x2B, 0xD7] => { // byteswap compressed
            unimplemented!() //TODO (compression isn't simply yaz0)
        }
        [0x93, 0x52, 0x2E, 0x7B, 0xE5, 0x06, 0xD4, 0x27] => { // decompressed
            base_rom[..0x0200_0000].copy_from_slice(&input_rom);
            stdin.read_exact(&mut base_rom[0x0200_0000..]).await?;
        }
        [0x44, 0xB0, 0x69, 0xB5, 0x3C, 0x37, 0x85, 0x19] | // PAL (regular compressed)
        [0xB0, 0x44, 0xB5, 0x69, 0x37, 0x3C, 0x19, 0x85] | // PAL (byteswap compressed)
        [0xEE, 0x9D, 0x53, 0xB5, 0xBC, 0x01, 0xD0, 0x15] => return Err(Error::PalBaseRom), // PAL (decompressed)
        _ => return Err(Error::BaseRom),
    }
    let worlds = vec![(); args.world_count.get().into()];
    //TODO actually randomize stuff
    if !search::can_win(&worlds) {
        return Err(Error::Search)
    }
    let patch = patch::patch_rom(&base_rom);
    if let Some(_) = args.world {
        patch.write_uncompressed_rom(stdout()).await?;
    } else {
        if args.world_count.get() > 1 {
            unimplemented!() //TODO write zpfz
        } else {
            patch.write_zpf(stdout()).await?;
        }
    }
    Ok(())
}
