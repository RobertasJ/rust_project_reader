use std::env;
use std::fs::File;
use std::io::{self, Read};
use walkdir::{DirEntry, WalkDir};

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

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| is_not_target(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let mut file = File::open(entry.path())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            println!("{}", entry.path().display());
            println!("```rust");
            println!("{}", contents);
            println!("```");
        }
    }

    Ok(())
}