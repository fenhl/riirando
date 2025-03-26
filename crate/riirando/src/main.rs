use {
    std::{
        num::NonZeroU8,
        path::PathBuf,
    },
    crossterm::tty::IsTty as _,
    tokio::io::{
        AsyncReadExt as _,
        stdin,
        stdout,
    },
    tokio_util::either::Either,
    wheel::fs::File,
};

mod logic;
mod patch;
mod search;

#[derive(Default, Clone, clap::ValueEnum)]
enum OutputKind {
    None,
    Patch,
    #[default]
    UncompressedRom,
}

#[derive(clap::Parser)]
struct Args {
    /// Read the base ROM from the given path instead of standard input.
    #[clap(short, long)]
    input: Option<PathBuf>,
    #[clap(short = 't', long, value_enum, default_value_t)]
    output_type: OutputKind,
    /// Write the selected output type to the given path instead of standard output. If there's an existing file at that path, it will be overwritten!
    #[clap(short, long)]
    output: Option<PathBuf>,
    #[clap(short, long, default_value = "1")]
    world_count: NonZeroU8,
    #[clap(short = 'p', long)]
    world: Option<NonZeroU8>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Decompress(#[from] decompress::Error),
    #[error(transparent)] Io(#[from] tokio::io::Error),
    #[error(transparent)] Search(#[from] search::Error),
    #[error(transparent)] Wheel(#[from] wheel::Error),
    #[error("standard input is not a valid OoT 1.0 NTSC ROM")]
    BaseRom,
    #[error("specify the world number to output or choose a different output type")]
    MultipleOutputs,
    #[error("standard input is an OoT PAL ROM, but we need an NTSC ROM")]
    PalBaseRom,
    #[error("standard input is a TTY")]
    Stdin,
    #[error("standard output is a TTY")]
    Stdout,
}

#[wheel::main]
async fn main(args: Args) -> Result<(), Error> {
    let mut input = if let Some(input) = args.input {
        Either::Left(File::open(input).await?)
    } else {
        let stdin = stdin();
        if stdin.is_tty() { return Err(Error::Stdin) }
        Either::Right(stdin)
    };
    let mut input_rom = vec![0; 0x0200_0000];
    input.read_exact(&mut input_rom).await?;
    let crc = &input_rom[0x10..0x18];
    let base_rom = match crc {
        [0xEC, 0x70, 0x11, 0xB7, 0x76, 0x16, 0xD7, 0x2B] | // regular compressed
        [0x70, 0xEC, 0xB7, 0x11, 0x16, 0x76, 0x2B, 0xD7] => { // byteswap compressed
            decompress::decompress(&mut input_rom)?
        }
        [0x93, 0x52, 0x2E, 0x7B, 0xE5, 0x06, 0xD4, 0x27] => { // decompressed
            let mut base_rom = vec![0; 0x0400_0000];
            base_rom[..0x0200_0000].copy_from_slice(&input_rom);
            input.read_exact(&mut base_rom[0x0200_0000..]).await?;
            base_rom
        }
        [0x44, 0xB0, 0x69, 0xB5, 0x3C, 0x37, 0x85, 0x19] | // PAL (regular compressed)
        [0xB0, 0x44, 0xB5, 0x69, 0x37, 0x3C, 0x19, 0x85] | // PAL (byteswap compressed)
        [0xEE, 0x9D, 0x53, 0xB5, 0xBC, 0x01, 0xD0, 0x15] => return Err(Error::PalBaseRom), // PAL (decompressed)
        _ => return Err(Error::BaseRom),
    };
    let worlds = vec![(); args.world_count.get().into()];
    //TODO actually randomize stuff
    search::check_reachability(&worlds)?;
    let patch = patch::patch_rom(&base_rom);
    let output = if let Some(output) = args.output {
        Either::Left(File::create(output).await?)
    } else {
        let stdout = stdout();
        if stdout.is_tty() { return Err(Error::Stdout) }
        Either::Right(stdout)
    };
    if let Some(_output_world) = args.world.or_else(|| (args.world_count.get() == 1).then_some(NonZeroU8::MIN)) {
        match args.output_type {
            OutputKind::None => {}
            OutputKind::Patch => patch.write_zpf(output).await?,
            OutputKind::UncompressedRom => patch.write_uncompressed_rom(output).await?,
        }
    } else {
        match args.output_type {
            OutputKind::None => {}
            OutputKind::Patch => unimplemented!(), //TODO write zpfz
            OutputKind::UncompressedRom => return Err(Error::MultipleOutputs), //TODO output zip archive of roms?
        }
    }
    Ok(())
}
