use clap::Parser;

use lsb_hide::{
    cli::{Cli, Commands},
    handler::{handle_hide, handle_recover},
};

/// 程序的主入口点
///
/// 负责解析命令行参数，并根据指定的子命令（`hide` 或 `recover`）
/// 将执行分派到相应的处理函数
fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 根据子命令调用相应的处理函数
    match cli.command {
        Commands::Hide(args) => handle_hide(args),
        Commands::Recover(args) => handle_recover(args),
    }
}
