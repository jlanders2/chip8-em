#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

use clap::Parser;

mod chip8;

#[derive(Parser, Debug)]
#[command(version, about = "Chip 8 Emulator")]
struct Args {
    /// Set path for chip8 binary file to run
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short = 'q', long = "quirks", value_delimiter = ',')]
    quirks: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let quirks = chip8::Chip8Quirks {
        vf_reset: args.quirks.contains(&String::from("vfreset"))
            || args.quirks.contains(&String::from("chip8")),
        memory: args.quirks.contains(&String::from("memory"))
            || args.quirks.contains(&String::from("chip8")),
        display_wait: args.quirks.contains(&String::from("displaywait"))
            || args.quirks.contains(&String::from("chip8")),
        clipping: args.quirks.contains(&String::from("clipping"))
            || args.quirks.contains(&String::from("chip8")),
        shifting: args.quirks.contains(&String::from("shifting")),
        jumping: args.quirks.contains(&String::from("jumping")),
    };

    if let Some(filepath) = args.file {
        chip8::run(filepath, quirks);
    }
}
