use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;
use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of ports
    #[clap(short, long)]
    path: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = args.path;
    let re = Regex::new(r"SHA256 \((.+)\) = (\w+)").unwrap();
    let mut result = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();
        match path.file_name().and_then(std::ffi::OsStr::to_str) {
            Some("distinfo") => {}
            _ => continue,
        }
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(line) = line {
                if line.starts_with("SHA256") {
                    // extract filename and sha256
                    let caps = re.captures(&line).unwrap();
                    let filename = caps.get(1).unwrap().as_str();
                    let sha256 = caps.get(2).unwrap().as_str();
                    result.push((filename.to_owned(), sha256.to_owned()));
                    break;
                }
            }
        }
    }
    result.sort();
    for item in result {
        println!("{} {}", item.1, item.0);
    }
    Ok(())
}
