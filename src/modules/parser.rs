use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Timer's time
    pub time: String,

    /// Suppress notification messages when set
    #[arg(short, long, default_value_t = false)]
    pub silent: bool,

    /// Is timer should be reversed
    #[arg(short, long, default_value_t = false)]
    pub countdown: bool,

    /// Timer name (notification summary)
    #[arg(short, long, default_value_t = String::from("BetterTimer"))]
    pub name: String,

    /// Timer message (notification body)
    #[arg(short, long, default_value_t = String::from("Time's up"))]
    pub message: String,

    #[arg(short = 'l', long, default_value_t = 40)]
    pub progressbar_length: u32,

    #[arg(short = 'C', long, default_value_t = String::from("cyan"))]
    pub progressbar_color: String,

    #[arg(short = 'b', long, default_value_t = String::from("blue"))]
    pub progressbar_background_color: String,
}

pub async fn parse_args() -> Result<Args> {
    Ok(Args::parse())
}
