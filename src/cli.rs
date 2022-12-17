use clap::Parser;

#[derive(Parser, Debug, Default, Clone)]
/// Retro is a CLI tool to host and and run retros
#[command(author, version, about, long_about = None)]
pub struct RetroArgs {
    /// room id to connect  to
    #[arg(short, long)]
    pub room: String,

    /// Your display name during the retro
    #[arg(short, long)]
    pub display_name: String,
}

impl RetroArgs {
    /// Parse the CLI parameters and return a new struct
    pub fn new() -> RetroArgs {
        RetroArgs::parse()
    }
}
