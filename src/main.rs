use std::env;
use std::io::{self};
use std::process;

mod compression;
mod decompression;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        try_help();
    }

    let input_file = &args[1];
    let output_file = format!("{}.gz", input_file);

    install_signal_handlers();

    if let Err(e) = compress_file(input_file, &output_file) {
        eprintln!("Error: {}", e);
        remove_output_file(&output_file);
        process::exit(1);
    }

    println!("File {} compressed to {}", input_file, output_file);
}

fn try_help() -> ! {
    eprintln!("Usage: gzip-rs <file>");
    process::exit(1);
}

fn install_signal_handlers() {
    // Example of setting up signal handling in Rust
    ctrlc::set_handler(move || {
        eprintln!("\nReceived interrupt signal. Cleaning up...");
        // Call function to clean up if needed, e.g., `remove_output_file`
        process::exit(1);
    }).expect("Error setting signal handler");
}

fn remove_output_file(filename: &str) {
    if let Err(e) = std::fs::remove_file(filename) {
        eprintln!("Warning: Unable to remove {}: {}", filename, e);
    }
}

fn do_exit(code: i32) -> ! {
    process::exit(code);
}

fn compress_file(input_file: &str, output_file: &str) -> io::Result<()> {
    let mut input = std::fs::File::open(input_file)?;
    let mut output = std::fs::File::create(output_file)?;
    compression::deflate::deflate(&mut input, &mut output, Some(input_file));
    Ok(()) // 将返回值转换为 Result<(), io::Error>
}
