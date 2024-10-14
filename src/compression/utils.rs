use std::io::{self};

// Function to update the CRC
pub fn updcrc(crc: u32, buffer: &[u8]) -> u32 {
    // Example placeholder for CRC calculation logic
    let mut crc = crc;
    for &byte in buffer {
        crc ^= byte as u32;
        // Additional CRC calculation logic goes here
    }
    crc
}

// Function to clear buffers
pub fn clear_bufs(buf: &mut [u8]) {
    for byte in buf.iter_mut() {
        *byte = 0;
    }
}

// Error handling function for gzip-specific errors
pub fn gzip_error(message: &str) -> io::Result<()> {
    eprintln!("gzip error: {}", message);
    Err(io::Error::new(io::ErrorKind::Other, message))
}

// Convert a string to lowercase
pub fn strlwr(s: &str) -> String {
    s.to_ascii_lowercase()
}
