use anyhow::Result;
use clap::Parser;

mod cli;
mod constants;
mod handler;
mod steganography;
use cli::{Cli, Commands};

/// 程序的主入口点。
///
/// 负责解析命令行参数，并根据指定的子命令（`hide` 或 `recover`）
/// 将执行分派到相应的处理函数。
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hide(args) => handler::handle_hide(args),
        Commands::Recover(args) => handler::handle_recover(args),
    }
}
