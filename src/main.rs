use anyhow::{Result, anyhow};
use clap::Parser;
use humantime::{format_duration, parse_duration};
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use tokio::time::{Duration, interval, sleep};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Timer's time
    time: String,

    /// Suppress notification messages when set
    #[arg(short, long, default_value_t = false)]
    silent: bool,

    /// Timer name (notification summary)
    #[arg(short, long, default_value_t = String::from("BetterTimer"))]
    name: String,

    /// Timer message (notification body)
    #[arg(short, long, default_value_t = String::from("Time's up"))]
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let total = match parse_duration(&args.time) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Time parsing error: {}", e);
            return Err(anyhow!("{}", e));
        }
    };

    let start = tokio::time::Instant::now();

    let timer = sleep(total);
    tokio::pin!(timer);

    let formatted_total = format_duration(total);

    let bar =
        ProgressBar::new(total.as_millis() as u64).with_message(format!("{}", formatted_total));
    bar.set_style(ProgressStyle::with_template(
        "[{msg}] {bar:40.cyan/blue} {percent}%",
    )?);

    let mut interval = interval(Duration::from_millis(10));
    loop {
        tokio::select! {
            _ = &mut timer => {
                bar.set_position(total.as_millis() as u64);
                bar.set_message(format!("{}/{}", formatted_total, formatted_total));
                bar.finish();
                break
            },
            _ = interval.tick() => {
                let elapsed = start.elapsed();
                bar.set_position(elapsed.as_millis() as u64);
                bar.set_message(format!("{}/{}", format_duration(Duration::from_millis(elapsed.as_millis() as u64)), formatted_total));
            }
        }
    }

    if !args.silent {
        Notification::new()
            .summary(&args.name)
            .body(&args.message)
            .show()?;
    }

    Ok(())
}
