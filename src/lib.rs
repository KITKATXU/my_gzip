// src/lib.rs

pub mod compression;
pub mod decompression;
pub mod util;

use std::process;

/// Cleans up resources and exits with an error code.
/// This function should handle removing temporary files or cleaning up the environment.
pub fn abort_gzip() -> ! {
    // Placeholder for cleanup tasks like removing output files
    eprintln!("gzip error: exiting due to a fatal error.");

    // Perform any additional cleanup here, if necessary
    remove_output_file(false);

    process::exit(1);
}

/// Handles signal-based interruption by performing cleanup and then re-raising the signal.
pub fn abort_gzip_signal(signal: i32) {
    eprintln!("gzip interrupted by signal: {}", signal);

    // Ensure any temporary files or buffers are safely removed
    remove_output_file(true);

    if signal == 15 { // Replace 15 with the actual signal if necessary
        process::exit(0);
    } else {
        // Re-raise the signal to the default handler
        unsafe {
            libc::signal(signal, libc::SIG_DFL);
            libc::raise(signal);
        }
    }
}

/// Removes temporary output files or cleans up resources.
/// Set `signals_already_blocked` to true if signals are blocked during cleanup.
fn remove_output_file(signals_already_blocked: bool) {
    if signals_already_blocked {
        // Perform specific cleanup with signals blocked
    } else {
        // Perform regular cleanup tasks
        eprintln!("Removing temporary output files...");
    }
}
