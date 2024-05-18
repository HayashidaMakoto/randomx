//! This file contains all the constants related to RANDOMX.
//! They are listed on [RandomX
//! specs](https://github.com/tevador/RandomX/blob/master/doc/specs.md)
/// The number of 1 KiB Argon2 blocks in the Cache
const RANDOMX_ARGON_MEMORY: usize = 262144;

/// The number of Argon2d iterations for Cache initialization
const RANDOMX_ARGON_ITERATIONS: usize = 3;

/// The number of parallel lanes for Cache initialization
const RANDOMX_ARGON_LANES: usize = 1;

/// Argon2 salt
const RANDOMX_ARGON_SALT: [u8; 8] = [b'R', b'a', b'n', b'd', b'o', b'm', b'X', b'\x03'];

/// The number of random Cache accesses per Dataset item
const RANDOMX_CACHE_ACCESSES: usize = 8;

/// Target latency for SuperscalarHash (in cycles of the reference CPU)
const RANDOMX_SUPERSCALAR_LATENCY: usize = 170;

/// Dataset base size in bytes
const RANDOMX_DATASET_BASE_SIZE: usize = 2147483648;

/// Dataset extra size in bytes
const RANDOMX_DATASET_EXTRA_SIZE: usize = 33554368;

/// The number of instructions in a RandomX program
const RANDOMX_PROGRAM_SIZE: usize = 256;

/// The number of iterations per program
const RANDOMX_PROGRAM_ITERATIONS: usize = 2048;

/// The number of programs per hash
const RANDOMX_PROGRAM_COUNT: usize = 8;

/// Jump condition mask size in bits
const RANDOMX_JUMP_BITS: usize = 8;

/// Jump condition mask offset in bits
const RANDOMX_JUMP_OFFSET: usize = 8;

/// Scratchpad L3 size in bytes
const RANDOMX_SCRATCHPAD_L3: usize = 2097152;

/// Scratchpad L2 size in bytes
const RANDOMX_SCRATCHPAD_L2: usize = 262144;

/// Scratchpad L1 size in bytes
const RANDOMX_SCRATCHPAD_L1: usize = 16384;
