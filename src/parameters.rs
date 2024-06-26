//! This file contains all the constants related to RANDOMX.
//! They are listed on [RandomX
//! specs](https://github.com/tevador/RandomX/blob/master/doc/specs.md)
/// The number of 1 KiB Argon2 blocks in the Cache
pub const RANDOMX_ARGON_MEMORY: u64 = 262144;

/// The number of Argon2d iterations for Cache initialization
pub const RANDOMX_ARGON_ITERATIONS: u64 = 3;

/// The number of parallel lanes for Cache initialization
pub const RANDOMX_ARGON_LANES: u64 = 1;

pub const RANDOMX_ARGON_BLOCK_SIZE: u64 = 1024;

/// Argon2 salt
pub const RANDOMX_ARGON_SALT: [u8; 8] = [b'R', b'a', b'n', b'd', b'o', b'm', b'X', b'\x03'];

/// The number of random Cache accesses per Dataset item
pub const RANDOMX_CACHE_ACCESSES: u64 = 8;

/// Target latency for SuperscalarHash (in cycles of the reference CPU)
pub const RANDOMX_SUPERSCALAR_LATENCY: u64 = 170;

pub const RANDOMX_SUPERSCALAR_MAX_SIZE: u64 = 3 * RANDOMX_SUPERSCALAR_LATENCY + 2;

pub const RANDOMX_CACHE_LINE_SIZE: u64 = RANDOMX_DATASET_INDEX_SIZE;

pub const RANDOMX_CACHE_LINE_ASSIGN_MASK: u64 =
    (RANDOMX_DATASET_BASE_SIZE - 1) & !(RANDOMX_CACHE_LINE_SIZE);

pub const RANDOMX_CACHE_SIZE: u64 = RANDOMX_ARGON_MEMORY * RANDOMX_ARGON_BLOCK_SIZE;

/// Dataset base size in bytes
pub const RANDOMX_DATASET_BASE_SIZE: u64 = 2147483648;

/// Dataset extra size in bytes
pub const RANDOMX_DATASET_EXTRA_SIZE: u64 = 33554368;

pub const RANDOMX_DATASET_INDEX_SIZE: u64 = 64;

/// Data extra items
pub const RANDOMX_DATASET_EXTRA_ITEMS: u64 =
    RANDOMX_DATASET_EXTRA_SIZE / RANDOMX_DATASET_INDEX_SIZE;

/// The number of instructions in a RandomX program
pub const RANDOMX_PROGRAM_SIZE: u64 = 256;

/// The number of iterations per program
pub const RANDOMX_PROGRAM_ITERATIONS: u32 = 2048;

/// The number of programs per hash
pub const RANDOMX_PROGRAM_COUNT: u64 = 8;

/// Jump condition mask size in bits
pub const RANDOMX_JUMP_BITS: u64 = 8;

/// Jump condition mask offset in bits
pub const RANDOMX_JUMP_OFFSET: u64 = 8;

/// Scratchpad L3 size in bytes
pub const RANDOMX_SCRATCHPAD_L3: u64 = 2097152;

/// Scratchpad L2 size in bytes
pub const RANDOMX_SCRATCHPAD_L2: u64 = 262144;

/// Scratchpad L1 size in bytes
pub const RANDOMX_SCRATCHPAD_L1: u64 = 16384;

/// Keys used for
/// [AesGenerator1R](https://github.com/tevador/RandomX/blob/master/doc/specs.md#32-aesgenerator1r)
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

/// Constants for
/// [AESGenerator4R](https://github.com/tevador/RandomX/blob/master/doc/specs.md#33-aesgenerator4r)
pub const AES_GENERATOR_4R_K0: [u8; 16] = [
    0xdd, 0xaa, 0x21, 0x64, 0xdb, 0x3d, 0x83, 0xd1, 0x2b, 0x6d, 0x54, 0x2f, 0x3f, 0xd2, 0xe5, 0x99,
];

pub const AES_GENERATOR_4R_K1: [u8; 16] = [
    0x50, 0x34, 0x0e, 0xb2, 0x55, 0x3f, 0x91, 0xb6, 0x53, 0x9d, 0xf7, 0x06, 0xe5, 0xcd, 0xdf, 0xa5,
];

pub const AES_GENERATOR_4R_K2: [u8; 16] = [
    0x04, 0xd9, 0x3e, 0x5c, 0xaf, 0x7b, 0x5e, 0x51, 0x9f, 0x67, 0xa4, 0x0a, 0xbf, 0x02, 0x1c, 0x17,
];

pub const AES_GENERATOR_4R_K3: [u8; 16] = [
    0x63, 0x37, 0x62, 0x85, 0x08, 0x5d, 0x8f, 0xe7, 0x85, 0x37, 0x67, 0xcd, 0x91, 0xd2, 0xde, 0xd8,
];

pub const AES_GENERATOR_4R_K4: [u8; 16] = [
    0x73, 0x6f, 0x82, 0xb5, 0xa6, 0xa7, 0xd6, 0xe3, 0x6d, 0x8b, 0x51, 0x3d, 0xb4, 0xff, 0x9e, 0x22,
];

pub const AES_GENERATOR_4R_K5: [u8; 16] = [
    0xf3, 0x6b, 0x56, 0xc7, 0xd9, 0xb3, 0x10, 0x9c, 0x4e, 0x4d, 0x02, 0xe9, 0xd2, 0xb7, 0x72, 0xb2,
];

pub const AES_GENERATOR_4R_K6: [u8; 16] = [
    0xe7, 0xc9, 0x73, 0xf2, 0x8b, 0xa3, 0x65, 0xf7, 0x0a, 0x66, 0xa9, 0x2b, 0xa7, 0xef, 0x3b, 0xf6,
];

pub const AES_GENERATOR_4R_K7: [u8; 16] = [
    0x09, 0xd6, 0x7c, 0x7a, 0xde, 0x39, 0x58, 0x91, 0xfd, 0xd1, 0x06, 0x0c, 0x2d, 0x76, 0xb0, 0xc0,
];

/// Constants for
/// [AESHash1R](https://github.com/tevador/RandomX/blob/master/doc/specs.md#34-aeshash1r).
pub const AES_HASH1R_STATE0: [u8; 16] = [
    0x0d, 0x2c, 0xb5, 0x92, 0xde, 0x56, 0xa8, 0x9f, 0x47, 0xdb, 0x82, 0xcc, 0xad, 0x3a, 0x98, 0xd7,
];

pub const AES_HASH1R_STATE1: [u8; 16] = [
    0x6e, 0x99, 0x8d, 0x33, 0x98, 0xb7, 0xc7, 0x15, 0x5a, 0x12, 0x9e, 0xf5, 0x57, 0x80, 0xe7, 0xac,
];

pub const AES_HASH1R_STATE2: [u8; 16] = [
    0x17, 0x00, 0x77, 0x6a, 0xd0, 0xc7, 0x62, 0xae, 0x6b, 0x50, 0x79, 0x50, 0xe4, 0x7c, 0xa0, 0xe8,
];

pub const AES_HASH1R_STATE3: [u8; 16] = [
    0x0c, 0x24, 0x0a, 0x63, 0x8d, 0x82, 0xad, 0x07, 0x05, 0x00, 0xa1, 0x79, 0x48, 0x49, 0x99, 0x7e,
];

pub const AES_HASH1R_XKEY0: [u8; 16] = [
    0x89, 0x83, 0xfa, 0xf6, 0x9f, 0x94, 0x24, 0x8b, 0xbf, 0x56, 0xdc, 0x90, 0x01, 0x02, 0x89, 0x06,
];

pub const AES_HASH1R_XKEY1: [u8; 16] = [
    0xd1, 0x63, 0xb2, 0x61, 0x3c, 0xe0, 0xf4, 0x51, 0xc6, 0x43, 0x10, 0xee, 0x9b, 0xf9, 0x18, 0xed,
];

pub const BLAKE_GENERATOR_SEED_MAX_SIZE: u64 = 60;

/// Instruction frequencies

/// Integer instructions
pub const RANDOMX_FREQ_IADD_RS: u64 = 16;
pub const RANDOMX_FREQ_IADD_M: u64 = 7;
pub const RANDOMX_FREQ_ISUB_R: u64 = 16;
pub const RANDOMX_FREQ_ISUB_M: u64 = 7;
pub const RANDOMX_FREQ_IMUL_R: u64 = 16;
pub const RANDOMX_FREQ_IMUL_M: u64 = 4;
pub const RANDOMX_FREQ_IMULH_R: u64 = 4;
pub const RANDOMX_FREQ_IMULH_M: u64 = 1;
pub const RANDOMX_FREQ_ISMULH_R: u64 = 4;
pub const RANDOMX_FREQ_ISMULH_M: u64 = 1;
pub const RANDOMX_FREQ_IMUL_RCP: u64 = 8;
pub const RANDOMX_FREQ_INEG_R: u64 = 2;
pub const RANDOMX_FREQ_IXOR_R: u64 = 15;
pub const RANDOMX_FREQ_IXOR_M: u64 = 5;
pub const RANDOMX_FREQ_IROR_R: u64 = 8;
pub const RANDOMX_FREQ_IROL_R: u64 = 2;
pub const RANDOMX_FREQ_ISWAP_R: u64 = 4;

/// Floating point instructions
pub const RANDOMX_FREQ_FSWAP_R: u64 = 4;
pub const RANDOMX_FREQ_FADD_R: u64 = 16;
pub const RANDOMX_FREQ_FADD_M: u64 = 5;
pub const RANDOMX_FREQ_FSUB_R: u64 = 16;
pub const RANDOMX_FREQ_FSUB_M: u64 = 5;
pub const RANDOMX_FREQ_FSCAL_R: u64 = 6;
pub const RANDOMX_FREQ_FMUL_R: u64 = 32;
pub const RANDOMX_FREQ_FDIV_M: u64 = 4;
pub const RANDOMX_FREQ_FSQRT_R: u64 = 6;

/// Control instructions
pub const RANDOMX_FREQ_CBRANCH: u64 = 25;
pub const RANDOMX_FREQ_CFROUND: u64 = 1;

/// Store instruction
pub const RANDOMX_FREQ_ISTORE: u64 = 16;

/// No-op instruction
pub const RANDOMX_FREQ_NOP: u64 = 0;

/// Float configuration
pub const FLOAT_MANTISSA_SIZE: u64 = 52;
pub const FLOAT_EXPONENT_SIZE: u64 = 11;

pub const FLOAT_MANTISSA_MASK: u64 = (1 << FLOAT_MANTISSA_SIZE) - 1;
pub const FLOAT_EXPONENT_MASK: u64 = (1 << FLOAT_EXPONENT_SIZE) - 1;

pub const RANDOMX_CONST_EXPONENT_BITS: u64 = 0x300;
pub const RANDOMX_STATIC_EXPONENT_BITS: u64 = 4;
pub const RANDOMX_DYNAMIC_EXPONENT_BITS: u64 = 4;
