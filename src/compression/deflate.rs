use std::io::{self, Read, Write};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::collections::HashMap;
const GZIP_MAGIC: u16 = 0x8b1f;
const COMPRESSION_METHOD_DEFLATE: u8 = 8;
const FLAG_FNAME: u8 = 0x08;
// Constants used for hashing, bit manipulation, etc.
pub const HASH_BITS: usize = 15;
pub const HASH_SIZE: usize = 1 << HASH_BITS;
pub const HASH_MASK: usize = HASH_SIZE - 1;
pub const TOO_FAR: usize = 4096;
// const MIN_MATCH: usize = 3;
pub const MIN_MATCH: usize = 3;
pub const MAX_MATCH: usize = 256;
pub const WSIZE: usize = 32_768; // Typical window size for gzip compression
pub const MAX_DIST: usize = 32768; // Example value, adjust as needed

use crate::compression::NIL;
use crate::util::crc::updcrc;
use crate::decompression::huft::build_huffman_tree;
use crate::decompression::huft::huft_free;
use crate::decompression::huft::generate_encoding_table;
use crate::compression::lm_init::longest_match;
use crate::compression::initialize_longest_match;
use crate::compression::lm_init::DeflateState;
fn update_hash(h: &mut usize, c: u8) {
    const H_SHIFT: usize = 5; // H_SHIFT 的值通常取决于哈希函数的实现
    *h = ((*h << H_SHIFT) ^ (c as usize)) & HASH_MASK;
}


fn insert_string(
    window: &[u8],
    s: usize,
    ins_h: &mut usize,
    prev: &mut [usize],
    head: &mut [usize]
) -> Option<usize> {
    if s + MIN_MATCH - 1 >= window.len() {
        return None; // 防止越界访问
    }
//     println!("ins_h: {:?}, windows:{:?}", ins_h,  window[s + MIN_MATCH - 1]);
    update_hash(ins_h, window[s + MIN_MATCH - 1]);

    let match_head = head[*ins_h];
    prev[s & (WSIZE - 1)] = match_head;
    head[*ins_h] = s;
//     println!("head: {:?}", head[*ins_h]);
    Some(match_head)
}



pub fn write_gzip_header<W: Write + ?Sized>(output: &mut W, filename: Option<&str>) -> io::Result<()> {
    // Write the GZIP magic number
    output.write_all(&GZIP_MAGIC.to_le_bytes())?;

    // Write compression method (deflate)
    output.write_all(&[COMPRESSION_METHOD_DEFLATE])?;

    // Write flags
    let mut flags: u8 = 0;
    if filename.is_some() {
        flags |= FLAG_FNAME;
    }
    output.write_all(&[flags])?;

    // Write timestamp
    let start = SystemTime::now();
    let epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    output.write_all(&epoch.to_le_bytes())?;

    // Write extra flags (OS-specific)
    output.write_all(&[0])?; // Extra flags
    output.write_all(&[3])?; // OS (Unix)

    // Write filename if provided
    if let Some(name) = filename {
        output.write_all(name.as_bytes())?;
        output.write_all(&[0])?; // Null terminator
    }

    Ok(())
}

pub fn write_gzip_footer<W: Write + ?Sized>(output: &mut W, crc: u32, input_size: u32) -> io::Result<()> {
    // Write the CRC32 value
    output.write_all(&crc.to_le_bytes())?;

    // Write the input size (mod 2^32)
    output.write_all(&input_size.to_le_bytes())?;

    Ok(())
}


pub fn deflate(input: &mut dyn Read, output: &mut dyn Write, filename: Option<&str>) -> io::Result<usize> {
     write_gzip_header(output, filename)?;
    let max_bits = 15; // 设定Huffman编码允许的最大位数
    let mut compressed_length: usize = 0;
    let crc: u32 = 0;
    let mut ins_h: usize = 0; // 初始化 ins_h 用于滚动哈希值
    let mut buffer = [0u8; 8192];
    let mut lookahead: usize;
    let mut strstart = 0;
    let mut hash_head = None;
    let mut compressed_data = Vec::new();
     // 设置压缩级别和标志位
    let compression_level = 6;
    let mut flags: u16 = 0;

    let mut state = DeflateState {
        window: vec![0; 2 * WSIZE],
        prev: vec![NIL; HASH_SIZE],
        hash_chain: vec![NIL; HASH_SIZE],
        strstart,
        block_start: 0,
        lookahead: 0,
        max_chain_length: 0, // 你可以使用 initialize_longest_match 进行初始化
    };
//     println!("{}, {}", state.strstart, state.lookahead);

    initialize_longest_match(&mut state, compression_level, &mut flags)
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
//     println!("{}, {}", state.strstart, state.lookahead);

    // 假设你在这里计算符号频率（例如 lit_freqs 和 dist_freqs）
    let mut lit_freqs = [0u16; 256];
    let mut dist_freqs = [0u16; 30];
    let mut block_data = Vec::new();
//     let dist_freqs_u8: Vec<u8> = dist_freqs.iter().map(|&x| x as u8).collect();

    let lit_symbols: Vec<u16> = (0..lit_freqs.len() as u16).collect();
    let dist_symbols: Vec<u16> = (0..dist_freqs.len() as u16).collect();

        // 调用 build_huffman_tree 函数替代 huft_build
//     let literal_tree =
//             build_huffman_tree(&lit_freqs);
//
//     let distance_tree =
//             build_huffman_tree(&dist_freqs);
//

    loop {

        fill_window(&mut state, input)?;

        if state.lookahead == state.strstart {
            break;
        }

        for &byte in &state.window[state.strstart..state.lookahead] {
            hash_head = insert_string(&state.window, state.strstart, &mut ins_h, &mut state.prev, &mut state.hash_chain);
            if let Some(hash) = hash_head {
                let match_length = longest_match(hash, &state); // 获取最长匹配长度
                let distance = state.strstart - hash;

                if match_length >= MIN_MATCH {
                    lit_freqs[match_length as usize] += 1;
                    let (distance_code, extra_bits) = calculate_distance_code(distance);
                    dist_freqs[distance_code as usize] += 1;
                    block_data.push((match_length as u8, distance_code as u8, extra_bits));
                } else {
                    lit_freqs[byte as usize] = lit_freqs[byte as usize].saturating_add(1);
                    block_data.push((byte, 0, 0)); // 字面符号没有对应的距离
                }
            }else {
                lit_freqs[byte as usize] = lit_freqs[byte as usize].saturating_add(1);
                block_data.push((byte, 0, 0));
            }
            state.strstart += 1;
            updcrc(crc, &[byte]);
        }

//         println!("{:?}, {:?}", &lit_freqs, &dist_freqs);
        let literal_tree =
                build_huffman_tree(&lit_freqs);

        let distance_tree =
                build_huffman_tree(&dist_freqs);


        let mut literal_encoding_table = HashMap::new();
        generate_encoding_table(&literal_tree, vec![], &mut literal_encoding_table);

        let mut distance_encoding_table = HashMap::new();
        generate_encoding_table(&distance_tree, vec![], &mut distance_encoding_table);


            // 对该区块的数据进行编码
        for &(symbol, dist, extra_bits) in &block_data {
            if dist == 0 {
                if let Some(encoded_literal) = literal_encoding_table.get(&symbol) {
                    compressed_data.extend(encoded_literal.clone());
                }
            } else {
                if let Some(encoded_length) = literal_encoding_table.get(&symbol) {
                    compressed_data.extend(encoded_length.clone());
                }

                if let Some(encoded_distance) = distance_encoding_table.get(&dist) {
                    compressed_data.extend(encoded_distance.clone());
                }

                compressed_data.push(extra_bits);
            }
        }

        lit_freqs.fill(0);
        dist_freqs.fill(0);
        block_data.clear();

    }


    // 刷新最后一块
    compressed_length += flush_block(output, true, &compressed_data)?;
    write_gzip_footer(output, crc, state.strstart as u32)?;
    println!("Total compressed length: {}", compressed_length);
    // 释放哈夫曼树
//     huft_free(literal_tree);
//     huft_free(distance_tree);

    Ok(compressed_length)
}

fn fill_window(state: &mut DeflateState, input: &mut dyn Read) -> io::Result<()> {
    // Check if we need to slide the window to avoid overflow
//     println!("{}, {}", state.strstart, state.lookahead);
    if state.strstart >= WSIZE + MAX_DIST {
        // Slide the window contents by WSIZE to the beginning
        state.window.copy_within(WSIZE..2 * WSIZE, 0);
//         println!("{:?}", state.hash_chain);
        // Update hash_chain and prev to reflect the shifted positions
        for i in 0..HASH_SIZE {
            state.hash_chain[i] = if state.hash_chain[i] >= WSIZE {
                state.hash_chain[i] - WSIZE
            } else {
                NIL
            };

            state.prev[i] = if state.prev[i] >= WSIZE {
                state.prev[i] - WSIZE
            } else {
                NIL
            };
        }

        // Adjust strstart and block_start to the new positions within the window
        state.strstart -= WSIZE;
        state.lookahead -= WSIZE;
//         state.block_start -= WSIZE;
    }

    let remaining_space = std::cmp::min(WSIZE, 2 * WSIZE - state.strstart);


    // Read more data into the window buffer if there’s space available
    let read_amount = input.read(&mut state.window[state.strstart..state.strstart + remaining_space])?;
    state.lookahead += read_amount;


    Ok(())
}


fn flush_block(output: &mut dyn Write, eof: bool, compressed_data: &[u8]) -> io::Result<usize> {
    let mut bytes_written = 0;

    // 编码块头部信息
    let block_header: u8 = if eof { 0b0000_0001 } else { 0b0000_0000 }; // 示例：最后一个块标志
    output.write_all(&[block_header])?;
    bytes_written += 1;

    // 写入实际压缩数据
    output.write_all(compressed_data)?;
    bytes_written += compressed_data.len();

    Ok(bytes_written)
}

fn calculate_distance_code(distance: usize) -> (u8, u8) {
    // 根据 `deflate` 算法的距离范围
    if distance <= 1 {
        return (0, 0);
    } else if distance <= 2 {
        return (1, 0);
    } else if distance <= 3 {
        return (2, 0);
    } else if distance <= 4 {
        return (3, 0);
    } else if distance <= 6 {
        return (4, (distance - 5) as u8);
    } else if distance <= 8 {
        return (5, (distance - 7) as u8);
    } else if distance <= 12 {
        return (6, (distance - 9) as u8);
    } else if distance <= 16 {
        return (7, (distance - 13) as u8);
    } else if distance <= 24 {
        return (8, (distance - 17) as u8);
    } else if distance <= 32 {
        return (9, (distance - 25) as u8);
    } else if distance <= 48 {
        return (10, (distance - 33) as u8);
    } else if distance <= 64 {
        return (11, (distance - 49) as u8);
    } else if distance <= 96 {
        return (12, (distance - 65) as u8);
    } else if distance <= 128 {
        return (13, (distance - 97) as u8);
    } else if distance <= 192 {
        return (14, (distance - 129) as u8);
    } else if distance <= 256 {
        return (15, (distance - 193) as u8);
    } else if distance <= 384 {
        return (16, (distance - 257) as u8);
    } else if distance <= 512 {
        return (17, (distance - 385) as u8);
    } else if distance <= 768 {
        return (18, (distance - 513) as u8);
    } else if distance <= 1024 {
        return (19, (distance - 769) as u8);
    } else if distance <= 1536 {
        return (20, (distance - 1025) as u8);
    } else if distance <= 2048 {
        return (21, (distance - 1537) as u8);
    } else if distance <= 3072 {
        return (22, (distance - 2049) as u8);
    } else if distance <= 4096 {
        return (23, (distance - 3073) as u8);
    } else if distance <= 6144 {
        return (24, (distance - 4097) as u8);
    } else if distance <= 8192 {
        return (25, (distance - 6145) as u8);
    } else if distance <= 12288 {
        return (26, (distance - 8193) as u8);
    } else if distance <= 16384 {
        return (27, (distance - 12289) as u8);
    } else if distance <= 24576 {
        return (28, (distance - 16385) as u8);
    } else {
        return (29, (distance - 24577) as u8);
    }
}

