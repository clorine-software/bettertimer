use anyhow::{Result, anyhow};
use humantime::{FormattedDuration, format_duration, parse_duration};
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use tokio::time::{Duration, Instant, Interval, interval, sleep};

use crate::parser::{self, Args};

pub async fn logic() -> Result<()> {
    let args = parser::parse_args().await?;

    let total = match parse_duration(&args.time) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Time parsing error: {}", e);
            return Err(anyhow!("{}", e));
        }
    };

    timer(total, &args).await?;

    if !args.silent {
        notify(&args.name, &args.message).await?;
    }

    Ok(())
}

async fn timer(total: Duration, args: &Args) -> Result<()> {
    let start = Instant::now();

    let formatted_total = format_duration(total);

    let bar = ProgressBar::new(total.as_millis() as u64)
        .with_message(format!("{}", formatted_total))
        .with_finish(indicatif::ProgressFinish::WithMessage(
            std::borrow::Cow::Owned(args.message.clone()),
        ));

    bar.set_style(ProgressStyle::with_template(
        &("[{msg}] {bar:".to_string()
            + &args.progressbar_length.to_string()
            + "."
            + &args.progressbar_color
            + "/"
            + &args.progressbar_background_color
            + "} {percent}%"),
    )?);

    let mut interval = interval(Duration::from_millis(args.update_interval));

    match args.countdown {
        false => loop_standart(bar, start, total, &mut interval, formatted_total).await?,
        true => loop_countdown(bar, start, total, &mut interval).await?,
    }

    Ok(())
}

async fn loop_standart(
    bar: ProgressBar,
    start: Instant,
    total: Duration,
    interval: &mut Interval,
    formatted_total: FormattedDuration,
) -> Result<()> {
    let timer = sleep(total);
    tokio::pin!(timer);
    loop {
        tokio::select! {
            _ = &mut timer => {
                bar.set_position(total.as_millis() as u64);
                let msg = format!("{}/{}", formatted_total, formatted_total);
                bar.set_message(msg);
                bar.finish_using_style();
                break
            },
            _ = interval.tick() => {
                let elapsed = start.elapsed();
                bar.set_position(elapsed.as_millis() as u64);
                let msg = format!("{}/{}", format_duration(Duration::from_millis(elapsed.as_millis() as u64)), formatted_total);
                bar.set_message(msg);
            }
        }
    }

    Ok(())
}

async fn loop_countdown(
    bar: ProgressBar,
    start: Instant,
    total: Duration,
    interval: &mut Interval,
) -> Result<()> {
    let timer = sleep(total);
    tokio::pin!(timer);
    loop {
        tokio::select! {
            _ = &mut timer => {
                bar.set_position(total.as_millis() as u64);
                bar.set_message("0ns");
                bar.finish_using_style();
                break
            },
            _ = interval.tick() => {
                let elapsed = start.elapsed();
                bar.set_position(elapsed.as_millis() as u64);
                let msg = format!("{}", format_duration(Duration::from_millis( (total - elapsed).as_millis() as u64)));
                bar.set_message(msg);
            }
        }
    }

    Ok(())
}

async fn notify(summary: &str, body: &str) -> Result<()> {
    Notification::new().summary(summary).body(&body).show()?;
    Ok(())
}
