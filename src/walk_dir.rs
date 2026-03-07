use std::{env::args};
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

fn process_entry (entry: &DirEntry, pat: &str) {
    let Ok(re) = Regex::new(&pat) else {return};
    let Some(file_name) = entry.file_name().to_str() else {return};

    if re.is_match(file_name) {
        println!("{}", file_name)
    }
}

fn main () {
    let args: Vec<String> = args().collect();
    match &args[..] {
        [_, pth, pat] => {
            for entry in WalkDir::new(&pth) {
                match entry {
                    Ok(entry) => process_entry(&entry, &pat),
                    Err(e) => eprintln!("Error {}", e)
                }
            }
        },
        [..] => println!("Usage")
    }
}