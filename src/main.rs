#![allow(unused)]
use std::{thread, time::Duration};

use anyhow::Result;
use clap::Parser;

mod time;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// duration formatted as HhMmSs, i.e '1h20m4s' or '5m' or '3h8s'
    time: time::Time,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    for second in 0..args.time.0 {
        println!("{}", args.time.0 - second);
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
