use std::io::{self};
use std::process;

pub fn gzip_error(message: &str) {
    eprintln!("\ngzip error: {}", message);
    abort_gzip();
}

pub fn xalloc_die() {
    eprintln!("\ngzip error: memory exhausted");
    abort_gzip();
}

pub fn warning(message: &str) {
    eprintln!("Warning: {}", message);
}

pub fn read_error(filename: &str) {
    if let Some(error) = io::Error::last_os_error().raw_os_error() {
        eprintln!("\n{}: {}", filename, error);
    } else {
        eprintln!("{}: unexpected end of file", filename);
    }
    abort_gzip();
}

pub fn write_error(filename: &str) {
    if let Some(error) = io::Error::last_os_error().raw_os_error() {
        eprintln!("\n{}: {}", filename, error);
    } else {
        eprintln!("{}: write error", filename);
    }
    abort_gzip();
}

fn abort_gzip() -> ! {
    process::abort();
}
