#![windows_subsystem = "console"]

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process;

fn process_files(file_list_path: &Path) -> io::Result<()> {
    let file = File::open(file_list_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let path = Path::new(&line);
        let new_path = path.with_extension("new");
        let old_path = path.with_extension("old");

        // Perform the file operations
        if let Err(e) = fs::rename(path, &old_path) {
            eprintln!("Failed to rename original to .old: {}", e);
            continue; // Skip to the next line on error
        }

        if let Err(e) = fs::rename(&new_path, path) {
            eprintln!("Failed to rename .new to original: {}", e);
            // Attempt to restore the original file
            let _ = fs::rename(&old_path, path);
            continue; // Skip to the next line on error
        }

        if let Err(e) = fs::remove_file(&old_path) {
            eprintln!("Failed to remove .old file: {}", e);
            // Not critical, so we just log the error and continue
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <executable> <path_to_file_list>");
        eprintln!("File List should use UTF-8");
        process::exit(1);
    }

    let file_list_path = Path::new(&args[1]);
    if let Err(e) = process_files(file_list_path) {
        eprintln!("Error processing files: {}", e);
        process::exit(1);
    }
}
