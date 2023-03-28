use std::env;
use std::fs::File;
use std::io::{self, Read};
use walkdir::{DirEntry, WalkDir};
use clipboard::{ClipboardProvider, ClipboardContext};
use colored::*;

fn blacklist(entry: &DirEntry) -> bool {
    !entry
        .path()
        .components()
        .any(|c| c.as_os_str() == "target" || c.as_os_str() == "node_modules")
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = args.last().unwrap();

    let mut output = String::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| blacklist(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let ext = entry.path().extension().unwrap().to_str().unwrap(); 
            let ext = match ext {
                "rs" | "json" => {
                    ext
                },
                "svelte" => {
                    "html"
                },
                "js" => {
                    "javascript"
                },
                "ts" => {
                    "typescript"
                },
                _ => continue,
            };
            let mut file = File::open(entry.path())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            output.push_str(&format!("{}", entry.path().display()));
            output.push_str(&format!("\n```{ext}\n"));
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