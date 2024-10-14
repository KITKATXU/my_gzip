// src/compression/mod.rs

pub mod deflate;
pub mod lm_init;
pub mod utils;

pub use deflate::deflate;
pub use deflate::MIN_MATCH;
pub use deflate::MAX_MATCH;
pub use lm_init::initialize_longest_match;
pub use lm_init::NIL;
pub use crate::util::crc::CRC32_TABLE;
