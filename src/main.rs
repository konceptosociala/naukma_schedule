use clap::Parser;
use docx_rs::*;
use serde_json::Value;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn parse_docx(file_name: &str) -> anyhow::Result<()> {
    let data: Value = serde_json::from_str(&read_docx(&read_to_vec(file_name)?)?.json())?;
    if let Some(children) = data["document"]["children"].as_array() {
        children.iter().for_each(read_children);
    }
    Ok(())
}

fn read_children(node: &Value) {
    if let Some(children) = node["data"]["children"].as_array() {
        children.iter().for_each(|child| {
            if child["type"] != "text" {
                read_children(child);
            } else {
                println!("{}", child["data"]["text"]);
            }
        });
    }
}

fn read_to_vec(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    std::fs::File::open(file_name)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    parse_docx(&args.name)?;
    Ok(())
}