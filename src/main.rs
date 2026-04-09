use anyhow::Result;
use clap::Parser;
use codachi::app::App;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "codachi", about = "A terminal Tamagotchi pet that reacts to your code")]
struct Cli {
    /// Project directory to watch (defaults to current directory)
    #[arg(short, long)]
    watch: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let project_dir = cli
        .watch
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    let mut terminal = ratatui::init();
    let result = App::new(&project_dir)?.run(&mut terminal);
    ratatui::restore();
    result
}
