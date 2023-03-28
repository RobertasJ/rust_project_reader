use std::{fs::File, io::Read};

use walkdir::DirEntry;

pub fn print_entry(entry: &DirEntry, &mut mut output: &mut String) {
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
                _ => return,
            };
            let mut file = File::open(entry.path())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            output.push_str(&format!("{}", entry.path().display()));
            output.push_str(&format!("\n```{ext}\n"));
            output.push_str(&contents);
            output.push_str("```\n\n");
}