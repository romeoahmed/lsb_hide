//! # 命令行接口模块
//!
//! 使用 `clap` 定义了程序的命令行结构，包括子命令和参数。
//! 所有用户通过命令行与程序交互的入口点都在此模块中定义。

use clap::Parser;
use std::path::PathBuf;

/// 一款基于 LSB (最低有效位) 隐写术的命令行工具，用于在无损格式图像 (如 PNG, BMP) 中隐藏或恢复文本。
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "一款基于 LSB (最低有效位) 隐写术的命令行工具，用于在无损格式图像 (如 PNG, BMP) 中隐藏或恢复文本。"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// 可用的子命令：hide (隐藏) 和 recover (恢复)。
#[derive(Parser, Debug)]
pub enum Commands {
    /// 在无损格式图像 (如 PNG, BMP) 中隐藏文本文件内容。
    Hide(HideArgs),

    /// 从经过隐写的图像中恢复隐藏的文本。
    Recover(RecoverArgs),
}

/// 'hide' 命令所需的参数。
#[derive(Parser, Debug)]
pub struct HideArgs {
    /// 用于隐写的输入图像文件路径 (如 PNG, BMP)。
    #[arg(short, long)]
    pub image: PathBuf,

    /// 要隐藏的文本内容的文件路径。
    #[arg(short, long)]
    pub text: PathBuf,

    /// 隐写完成后，保存结果图像的输出路径。
    #[arg(short, long)]
    pub dest: PathBuf,
}

/// 'recover' 命令所需的参数。
#[derive(Parser, Debug)]
pub struct RecoverArgs {
    /// 已隐藏文本数据的图像文件路径。
    #[arg(short, long)]
    pub image: PathBuf,

    /// 恢复文本后，保存文本内容的输出路径。
    #[arg(short, long)]
    pub text: PathBuf,
}
