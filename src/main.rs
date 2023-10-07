mod group;
mod error;
mod macros;
mod schedule;

use anyhow::Result;
use clap::Parser;
use schedule::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, required=true, num_args=1..)]
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let schedule = Schedule::new(&args.files)?;

    std::fs::write("schedule.json", serde_json::to_string_pretty(&schedule).unwrap())?;

    Ok(())
}