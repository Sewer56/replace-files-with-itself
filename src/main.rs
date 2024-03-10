#![windows_subsystem = "console"]
#![feature(start)]

extern crate libc;

use libc::{c_char, c_int, fclose, fgets, fopen, printf, remove, rename, strcat, strcpy, strlen};
const BUFFER_SIZE: usize = 65536;

#[no_mangle]
unsafe extern "C" fn process_files(file_list_path: *const c_char) -> c_int {
    // Open the file list
    let file = fopen(file_list_path, "r".as_ptr() as *const _);
    if file.is_null() {
        return -1;
    }

    let mut buffer: [c_char; BUFFER_SIZE] = [0; BUFFER_SIZE];
    while !fgets(buffer.as_mut_ptr(), BUFFER_SIZE as c_int, file).is_null() {
        // Get file list item.
        let len = strlen(buffer.as_ptr());
        let eol_ptr = buffer.as_mut_ptr().offset(len as isize - 1);
        if len > 0 && *eol_ptr as u8 as char == '\n' {
            *eol_ptr = 0;
        }

        // Manual string appending for ".new" and ".old"
        let mut new_exe_path = [0 as c_char; BUFFER_SIZE];
        let mut moved_old_exe_path = [0 as c_char; BUFFER_SIZE];

        strcpy(new_exe_path.as_mut_ptr(), buffer.as_ptr());
        strcat(new_exe_path.as_mut_ptr(), ".new\0".as_ptr() as *const _);

        strcpy(moved_old_exe_path.as_mut_ptr(), buffer.as_ptr());
        strcat(
            moved_old_exe_path.as_mut_ptr(),
            ".old\0".as_ptr() as *const _,
        );

        // Perform file operations here
        // Example: libc::system(concat!("cp ", exe_file, " ", new_exe_path).as_ptr() as *const _);
        rename(buffer.as_ptr(), moved_old_exe_path.as_ptr());
        rename(new_exe_path.as_ptr(), buffer.as_ptr());
        remove(moved_old_exe_path.as_ptr());
    }

    fclose(file);

    0
}

/// # Safety
#[start]
pub fn main(argc: isize, argv: *const *const u8) -> isize {
    unsafe {
        if argc < 2 {
            // If not enough arguments are provided, print an error message.
            printf("Usage: <executable> <path_to_file_list>\0".as_ptr() as *const _);
            return -1;
        }

        let file_list_path = *argv.offset(1);
        process_files(file_list_path as *const c_char) as isize
    }
}
