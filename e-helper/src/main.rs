use clap::{self, Subcommand};
use std::os::unix::fs::chown;

#[derive(Subcommand, Debug)]
enum SubCommands {
    Config {
        /// Write stdin to file in path output
        #[arg(short, long)]
        output: String,
    },
    Rebuild {
        /// Run `nixos-rebuild` with the given arguments
        arguments: Vec<String>,
    },
    WriteRebuild {
        /// Content to write to file
        #[arg(short, long)]
        content: String,
        /// Write config to file in path output
        #[arg(short, long)]
        path: String,
        /// Run `nixos-rebuild` with the given arguments
        arguments: Vec<String>,
    },
}

fn main() {
    if users::get_effective_uid() != 0 {
        eprintln!("EIMZOManager must be run as root");
        std::process::exit(1);
    }

    // Rest logic [you're sudo and implement using native Rust functions (no shell or Command)]
    let _ = std::fs::create_dir_all("/media/DSKEYS");
    let _ = chown("/media/DSKEYS", Some(1000), Some(1000));
}
