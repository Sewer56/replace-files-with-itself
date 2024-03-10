#![windows_subsystem = "windows"]

use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::process;
use winapi::um::winuser::{MessageBoxW, MB_OK};

fn show_message_box(message: &str) {
    let wide_message: Vec<u16> = OsStr::new(message).encode_wide().chain(Some(0)).collect();
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            wide_message.as_ptr(),
            wide_message.as_ptr(),
            MB_OK,
        );
    }
}

fn process_files(file_list_path: &Path, log_file: &mut File) -> io::Result<()> {
    let file = File::open(file_list_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let path = Path::new(&line);
        let new_path = path.with_extension("new");
        let old_path = path.with_extension("old");

        // Perform the file operations, logging before each action
        writeln!(
            log_file,
            "Copying {} -> {}",
            path.display(),
            new_path.display()
        )?;
        if let Err(e) = fs::copy(path, &new_path) {
            writeln!(log_file, "Failed to copy: {}", e)?;
            continue;
        }

        writeln!(
            log_file,
            "Renaming {} -> {}",
            path.display(),
            old_path.display()
        )?;
        if let Err(e) = fs::rename(path, &old_path) {
            writeln!(log_file, "Failed to rename original to .old: {}", e)?;
            continue;
        }

        writeln!(
            log_file,
            "Renaming {} -> {}",
            new_path.display(),
            path.display()
        )?;
        if let Err(e) = fs::rename(&new_path, path) {
            writeln!(log_file, "Failed to rename .new to original: {}", e)?;
            let _ = fs::rename(&old_path, path);
            continue;
        }

        writeln!(log_file, "Removing {}", old_path.display())?;
        if let Err(e) = fs::remove_file(&old_path) {
            writeln!(log_file, "Failed to remove .old file: {}", e)?;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <executable> <path_to_file_list>");
        return;
    }

    let file_list_path = Path::new(&args[1]);
    let log_path = file_list_path.with_extension("log");

    let mut log_file = match OpenOptions::new().create(true).write(true).open(&log_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
            show_message_box(&format!(
                "Failed to open log file: {}. See log at {}",
                e,
                log_path.display()
            ));
            process::exit(1);
        }
    };

    if let Err(e) = process_files(file_list_path, &mut log_file) {
        let error_message = format!(
            "Error processing files: {}. See log at {}",
            e,
            log_path.display()
        );
        writeln!(log_file, "{}", error_message).expect("Failed to write final error to log");
        eprintln!("{}", error_message);
        show_message_box(&error_message);
        process::exit(1);
    }
}
