use std::env;
use std::fs::File;
use std::io::{self, Read};
use walkdir::{DirEntry, WalkDir};
use clipboard::{ClipboardProvider, ClipboardContext};
use colored::*;

fn is_not_target(entry: &DirEntry) -> bool {
    !entry
        .path()
        .components()
        .any(|c| c.as_os_str() == "target")
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: rust_project_reader <path>");
        return Ok(());
    }

    let path = &args[1];

    let mut output = String::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| is_not_target(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let mut file = File::open(entry.path())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            output.push_str(&format!("{}", entry.path().display()));
            output.push_str("\n```rust\n");
            output.push_str(&contents);
            output.push_str("```\n\n");
        }
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output).unwrap();

    let message = "contents copied to clipboard!";
    println!("{} {}", "        Done".green().bold(), message);

    Ok(())
}