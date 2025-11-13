use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Hide text in BMP image.
    Hide(HideArgs),

    /// Recover text from doctored BMP image.
    Recover(RecoverArgs),
}

#[derive(Parser, Debug)]
pub struct HideArgs {
    /// Input image path.
    #[arg(short, long)]
    pub image: PathBuf,

    /// Input text path.
    #[arg(short, long)]
    pub text: PathBuf,
    /// Output doctored image path.
    #[arg(short, long)]
    pub dest: PathBuf,
}

#[derive(Parser, Debug)]
pub struct RecoverArgs {
    /// Input doctored image path.
    #[arg(short, long)]
    pub image: PathBuf,

    /// Output recoverd text path.
    #[arg(short, long)]
    pub text: PathBuf,
}
