//! This file contains all the constants related to RANDOMX.
//! They are listed on [RandomX
//! specs](https://github.com/tevador/RandomX/blob/master/doc/specs.md)
/// The number of 1 KiB Argon2 blocks in the Cache
pub const RANDOMX_ARGON_MEMORY: usize = 262144;

/// The number of Argon2d iterations for Cache initialization
pub const RANDOMX_ARGON_ITERATIONS: usize = 3;

/// The number of parallel lanes for Cache initialization
pub const RANDOMX_ARGON_LANES: usize = 1;

/// Argon2 salt
pub const RANDOMX_ARGON_SALT: [u8; 8] = [b'R', b'a', b'n', b'd', b'o', b'm', b'X', b'\x03'];

/// The number of random Cache accesses per Dataset item
pub const RANDOMX_CACHE_ACCESSES: usize = 8;

/// Target latency for SuperscalarHash (in cycles of the reference CPU)
pub const RANDOMX_SUPERSCALAR_LATENCY: usize = 170;

/// Dataset base size in bytes
pub const RANDOMX_DATASET_BASE_SIZE: usize = 2147483648;

/// Dataset extra size in bytes
pub const RANDOMX_DATASET_EXTRA_SIZE: usize = 33554368;

/// The number of instructions in a RandomX program
pub const RANDOMX_PROGRAM_SIZE: usize = 256;

/// The number of iterations per program
pub const RANDOMX_PROGRAM_ITERATIONS: usize = 2048;

/// The number of programs per hash
pub const RANDOMX_PROGRAM_COUNT: usize = 8;

/// Jump condition mask size in bits
pub const RANDOMX_JUMP_BITS: usize = 8;

/// Jump condition mask offset in bits
pub const RANDOMX_JUMP_OFFSET: usize = 8;

/// Scratchpad L3 size in bytes
pub const RANDOMX_SCRATCHPAD_L3: usize = 2097152;

/// Scratchpad L2 size in bytes
pub const RANDOMX_SCRATCHPAD_L2: usize = 262144;

/// Scratchpad L1 size in bytes
pub const RANDOMX_SCRATCHPAD_L1: usize = 16384;

/// Keys used for AesGenerator1R
pub const AES_GENERATOR_1R_K0: [u8; 16] = [
    0x53, 0xa5, 0xac, 0x6d, 0x09, 0x66, 0x71, 0x62, 0x2b, 0x55, 0xb5, 0xdb, 0x17, 0x49, 0xf4, 0xb4,
];

pub const AES_GENERATOR_1R_K1: [u8; 16] = [
    0x07, 0xaf, 0x7c, 0x6d, 0x0d, 0x71, 0x6a, 0x84, 0x78, 0xd3, 0x25, 0x17, 0x4e, 0xdc, 0xa1, 0x0d,
];

pub const AES_GENERATOR_1R_K2: [u8; 16] = [
    0xf1, 0x62, 0x12, 0x3f, 0xc6, 0x7e, 0x94, 0x9f, 0x4f, 0x79, 0xc0, 0xf4, 0x45, 0xe3, 0x20, 0x3e,
];

pub const AES_GENERATOR_1R_K3: [u8; 16] = [
    0x35, 0x81, 0xef, 0x6a, 0x7c, 0x31, 0xba, 0xb1, 0x88, 0x4c, 0x31, 0x16, 0x54, 0x91, 0x16, 0x49,
];
