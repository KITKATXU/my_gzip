use crate::compression::MIN_MATCH;
use crate::compression::MAX_MATCH;
// Constants related to hash size, window size, etc.
const HASH_BITS: usize = 15;
const HASH_SIZE: usize = 1 << HASH_BITS;
const HASH_MASK: usize = HASH_SIZE - 1;
pub const NIL: usize = 0;

const TOO_FAR: usize = 4096;

// Speed flags
const FAST: u16 = 4;
const SLOW: u16 = 2;
use crate::compression::deflate::MAX_DIST;
use crate::compression::deflate::WSIZE;
// Configuration table for compression levels
struct Config {
    max_lazy: usize,
    good_length: usize,
    nice_length: usize,
    max_chain: usize,
}

const CONFIGURATION_TABLE: [Config; 10] = [
    Config { max_lazy: 0, good_length: 0, nice_length: 0, max_chain: 0 },
    Config { max_lazy: 4, good_length: 4, nice_length: 8, max_chain: 4 },
    Config { max_lazy: 4, good_length: 5, nice_length: 16, max_chain: 8 },
    Config { max_lazy: 4, good_length: 6, nice_length: 32, max_chain: 32 },
    Config { max_lazy: 4, good_length: 4, nice_length: 16, max_chain: 16 },
    Config { max_lazy: 8, good_length: 16, nice_length: 32, max_chain: 32 },
    Config { max_lazy: 8, good_length: 16, nice_length: 128, max_chain: 128 },
    Config { max_lazy: 8, good_length: 32, nice_length: 128, max_chain: 256 },
    Config { max_lazy: 32, good_length: 128, nice_length: 258, max_chain: 1024 },
    Config { max_lazy: 32, good_length: 258, nice_length: 258, max_chain: 4096 },
];

pub struct DeflateState {
    pub window: Vec<u8>,       // 滑动窗口缓冲区
    pub hash_chain: Vec<usize>, // 匹配的哈希链
    pub prev: Vec<usize>,      // 用于管理链的前一个元素
    pub strstart: usize,       // 字符串的起始位置
    pub block_start: usize,    // 当前块的起始位置
    pub lookahead: usize,      // 前向查看数据量
    pub max_chain_length: usize, // 最大链长度
}

impl DeflateState {
    pub fn new(window_size: usize) -> Self {
        DeflateState {
            window: vec![0; window_size],
            hash_chain: vec![NIL; HASH_SIZE],
            prev: vec![NIL; WSIZE],
            strstart: 0,
            block_start: 0,
            lookahead: 0,
            max_chain_length: 0,
        }
    }
}
/// Initializes the longest match settings for the deflate algorithm.
pub fn initialize_longest_match(state: &mut DeflateState, compression_level: usize, flags: &mut u16) -> Result<(), &'static str> {
    if compression_level < 1 || compression_level > 9 {
        return Err("Invalid compression level");
    }

    let config = &CONFIGURATION_TABLE[compression_level];

    // 将配置参数应用到 `state` 中
    state.max_chain_length = config.max_chain;
//     state.good_match = config.good_length;
//     state.nice_match = config.nice_length;
//     state.max_lazy_match = config.max_lazy;

    // 设置标志位
    if compression_level == 1 {
        *flags |= FAST;
    } else if compression_level == 9 {
        *flags |= SLOW;
    }

//     // 通过读取滑动窗口的数据设置 `lookahead` 的初始值
//     state.lookahead = read_into_window(&mut state.window)?;

    // 准备初始哈希值
    let mut ins_h = 0;
    for j in 0..(MIN_MATCH - 1) {
        ins_h = update_hash(ins_h, state.window[j]);
        println!("ins_h:{:?}, state.window[j]:{:?}", ins_h, state.window[j]);
    }

    Ok(())
}


pub fn longest_match(cur_match: usize, state: &DeflateState) -> usize {
    let mut best_len = MIN_MATCH - 1;
    let mut chain_length = state.max_chain_length;
    let strstart = state.strstart;
    let window = &state.window;
    let mut match_pos = cur_match;
//     println!("window: {:?}", window);
//     if match_pos>0{
//         println!("match_pos: {:?}, strstart: {:?}", match_pos, strstart);
//     }
    while chain_length > 0 && match_pos > 0 && (strstart - match_pos ) <= MAX_DIST {
        let mut match_len = 0;
//         println!("{:?}", &state.window);
        // 比较窗口中的字节
        while match_len < WSIZE
            && match_len < window.len() - strstart
            && match_len < window.len() - match_pos
            && window[strstart + match_len] == window[match_pos + match_len]
        {
//             println!("{:?}: {:?}: {:?}", strstart, match_pos, match_len);
//             println!("{:?}: {:?}", window[strstart + match_len], window[match_pos + match_len]);
            match_len += 1;
            if match_len >= MAX_MATCH-1 {
                break;
            }
        }

        // 更新 best_len 以存储当前找到的最长匹配
        if match_len > best_len {
            best_len = match_len;
        }

        chain_length -= 1;

        // 获取链中的下一个匹配位置
        match_pos = state.hash_chain[match_pos % WSIZE];
    }

    best_len
}




/// Updates the hash based on input byte
fn update_hash(h: usize, c: u8) -> usize {
    const H_SHIFT: usize = 5;
    ((h << H_SHIFT) ^ (c as usize)) & HASH_MASK
}

/// Reads data into the sliding window buffer
/// This is a placeholder function representing the actual data reading process.
fn read_into_window(window: &mut [u8]) -> Result<usize, &'static str> {
    // Placeholder for reading data and returning the number of bytes read
    Ok(window.len())  // Replace with actual amount read
}
