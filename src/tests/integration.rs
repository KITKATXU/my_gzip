#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::compression::deflate;
    use crate::decompression::inflate;

    #[test]
    fn test_deflate_inflate_roundtrip() {
        let input_data = b"Hello, this is a test for deflate and inflate!";
        let mut compressed_data = Vec::new();
        let mut decompressed_data = Vec::new();

        // Compress the data
        let _ = deflate::deflate(&mut Cursor::new(input_data), &mut compressed_data).expect("Deflate failed");

        // Decompress the data
        let _ = inflate::inflate(&mut Cursor::new(compressed_data), &mut decompressed_data).expect("Inflate failed");

        // Validate that decompressed data matches the original data
        assert_eq!(input_data, &decompressed_data[..]);
    }

    #[test]
    fn test_inflate_fixed_block() {
        let fixed_block_data = b"Fixed block data test"; // Replace with actual fixed block data
        let mut decompressed_data = Vec::new();

        // Attempt to inflate a fixed block
        let _ = inflate::inflate(&mut Cursor::new(fixed_block_data), &mut decompressed_data).expect("Inflate failed");

        // Verify that the decompressed data matches expected output
        // Expected output should be added here based on the fixed block specifics
        // assert_eq!(expected_data, &decompressed_data[..]);
    }

    #[test]
    fn test_inflate_dynamic_block() {
        let dynamic_block_data = b"Dynamic block data test"; // Replace with actual dynamic block data
        let mut decompressed_data = Vec::new();

        // Attempt to inflate a dynamic block
        let _ = inflate::inflate(&mut Cursor::new(dynamic_block_data), &mut decompressed_data).expect("Inflate failed");

        // Verify that the decompressed data matches expected output
        // Expected output should be added here based on the dynamic block specifics
        // assert_eq!(expected_data, &decompressed_data[..]);
    }

    #[test]
    fn test_crc_validation() {
        let data = b"CRC data check";
        let crc_result = util::updcrc(0, data);

        // Replace with actual expected CRC value
        let expected_crc = 0; // Replace with computed value
        assert_eq!(crc_result, expected_crc);
    }

    #[test]
    fn test_inflate_error_handling() {
        let corrupted_data = b"\x00\x00\x00"; // Corrupt or incomplete data for error testing
        let mut output = Vec::new();

        let result = inflate::inflate(&mut Cursor::new(corrupted_data), &mut output);
        assert!(result.is_err(), "Expected error for corrupt data, got success");
    }
}
