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

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Definitions related to university disciplines' lesson groups.
mod group;
/// Custom error types and error handling for the university schedule parser.
mod error;
/// Custom macros for parsing university schedule
mod macros;
/// Definitions related to the university schedule, including faculties and specialities.
mod schedule;

use anyhow::Result;
use clap::Parser;
use schedule::*;

/// The command-line arguments parsing structure.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The list of file paths to university schedule Excel files.
    #[arg(short, long, required=true, num_args=1..)]
    files: Vec<String>,
}

/// The entry point of the university schedule parser program.
///
/// This function parses command-line arguments, reads university schedule data from Excel files,
/// and writes the parsed data to a JSON file.
///
/// # Returns
///
/// A `Result` indicating success or failure of the program.
fn main() -> Result<()> {
    let args = Args::parse();
    let schedule = Schedule::new(&args.files)?;

    std::fs::write("schedule.json", serde_json::to_string_pretty(&schedule).unwrap())?;

    Ok(())
}