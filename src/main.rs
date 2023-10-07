/*!
# NaUKMA Schedule

**FIdo** testing project: Studying schedule parser for National University of Kyiv-Mohyla Academia 

## Usage

```bash
$ naukma_schedule --files <faculty.speciality.xlsx> <faculty.xlsx>
```

Use appropriate filenames for spreadsheet files. Example:

* `Факультет Інформатики.Інженерія програмного забезпечення.xlsx` - faculty is **Факультет Інформатики** and speciality is **Інженерія програмного забезпечення**
* `Факультет Економічних Наук.xlsx` - faculty is **Факультет Економічних Наук**; multiple specialities are defined in the file

## Features
* Schedule fields (de-)serialization and validation
* Nested schedule structure

## Used crates
- `anyhow` - flexible pretty error handling
- `calamine` - xlsx spreadsheet parser
- `clap` - command line argument parser
- `serde` - powerful (de-)serialization framework
- `serde_json` - JSON serialization for serde
- `thiserror` - dedicated error types design
- `validator` - struct fields validation functions

## License

This project is licensed under Unlicense license and is in the **public domain**

Copyright (c) Oleksandr Hnutov

*/

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