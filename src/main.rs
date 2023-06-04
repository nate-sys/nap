#![allow(unused)]
use std::{thread, time::Duration};

use anyhow::Result;
use clap::Parser;

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

mod time;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// duration formatted as HhMmSs, i.e '1h20m4s' or '5m' or '3h8s'
    time: time::Time,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let style =
        ProgressStyle::with_template("{elapsed_precise:^} {wide_bar:.blue/black} {msg:^12}")?
            .progress_chars("━━━");
    let bar = ProgressBar::new(args.time.0 as u64).with_style(style);
    bar.set_message(HumanDuration(Duration::from_secs(args.time.0 as u64)).to_string());

    for secs in 0..args.time.0 {
        thread::sleep(Duration::from_secs(1));
        bar.inc(1);
    }
    bar.finish_with_message("Time's up!");

    Ok(())
}
