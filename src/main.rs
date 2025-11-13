use anyhow::Result;
use clap::Parser;

mod cli;
mod constants;
mod handler;
mod steganography;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hide(args) => handler::handle_hide(args),
        Commands::Recover(args) => handler::handle_recover(args),
    }
}
