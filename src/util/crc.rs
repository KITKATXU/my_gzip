const CRC32_POLYNOMIAL: u32 = 0xEDB88320;
lazy_static::lazy_static! {
    pub static ref CRC32_TABLE: [u32; 256] = {
        let mut table = [0u32; 256];
        for i in 0..256 {
            let mut crc = i as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ CRC32_POLYNOMIAL;
                } else {
                    crc >>= 1;
                }
            }
            table[i] = crc;
        }
        table
    };
}

pub fn updcrc(crc: u32, buffer: &[u8]) -> u32 {
    let mut crc = crc ^ 0xFFFFFFFF; // 初始 XOR
    for &byte in buffer {
        let table_index = (crc ^ (byte as u32)) & 0xFF;
        crc = CRC32_TABLE[table_index as usize] ^ (crc >> 8);
    }
    crc ^ 0xFFFFFFFF // 最终 XOR
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc() {
        let data: &[u8] = b"Hello, World!";
        let crc = updcrc(0, data);
        assert_eq!(crc, 0xec4ac3d0); // 使用正确的预期 CRC 值
    }
}
