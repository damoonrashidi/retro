use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct RetroArgs {
    /// room id
    #[arg(short, long)]
    pub room: String,

    /// display name for you in your team
    #[arg(short, long)]
    pub display_name: String,
}

impl RetroArgs {
    pub fn new() -> RetroArgs {
        RetroArgs::parse()
    }
}
