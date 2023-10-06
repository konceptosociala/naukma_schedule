mod group;
mod error;
mod macros;
mod schedule;

use clap::Parser;

use schedule::*;
use error::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    files: Vec<String>,
}

fn main() -> ScheduleResult<()> {
    let args = Args::parse();
    // let _schedule = Schedule::new(&args.files)?;

    let schedule = Schedule::default();
    println!("{}", serde_json::to_string_pretty(&schedule).unwrap());
    
    Ok(())
}