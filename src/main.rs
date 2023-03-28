use std::env;
use std::fs::File;
use std::io::{self, Read};
use walkdir::{DirEntry, WalkDir};
use clipboard::{ClipboardProvider, ClipboardContext};
use colored::*;

mod print;

fn whitelist(entry: &DirEntry) -> bool {
    entry
        .path()
        .components()
        .any(|c| c.as_os_str() == "src" || c.as_os_str() == "src-tauri")
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = args.last().unwrap();

    let mut output = String::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| whitelist(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            print::print_entry(&entry, &mut output);
        }
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output).unwrap();

    let message = "contents copied to clipboard!";
    println!("{} {}", "        Done".green().bold(), message);

    Ok(())
}