use clap::Parser;

mod chip8;

#[derive(Parser, Debug)]
#[command(version, about = "Chip 8 Emulator")]
struct Args {
    /// Turn on debug mode
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,
    /// Set path for chip8 binary file to run
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    // TODO: should define my raylib/sound/etc handles here eventually and pass them down

    if let Some(filepath) = args.file {
        if args.debug {
            chip8::run(filepath, true);
        } else {
            chip8::run(filepath, false);
        }
    }
}
