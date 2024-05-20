use std::ffi::c_float;

use crate::parameters::{RANDOMX_PROGRAM_ITERATIONS, RANDOMX_PROGRAM_SIZE, RANDOMX_SCRATCHPAD_L3};

/// Each instruction word is 64 bits long
/// 63         32       24      16       8         0
/// ------------------------------------------------
/// |  imm32    |    mod |   src |   dst |  opcode |
/// ------------------------------------------------
pub type EncodedInstruction = u64;

/// > A 32-bit immediate value that can be used as the source operand and is
/// > used to calculate addresses for memory operations. The immediate value is
/// > sign-extended to 64 bits unless specified otherwise.
pub fn imm32(i: EncodedInstruction) -> u32 {
    (i >> 32) as u32
}

pub fn mod_(i: EncodedInstruction) -> u8 {
    (i >> 24) as u8
}

pub fn src(i: EncodedInstruction) -> u8 {
    (i >> 16) as u8
}

pub fn dst(i: EncodedInstruction) -> u8 {
    (i >> 8) as u8
}

/// Return the opcode of the instruction
/// > There are 256 opcodes, which are distributed between 29 distinct
/// > instructions. Each instruction can be encoded using multiple opcodes (the
/// > number of opcodes specifies the frequency of the instruction in a random
/// > program).
pub fn opcode(i: EncodedInstruction) -> u8 {
    i as u8
}

// FIXME: enforce alignment
pub struct ProgramConfiguration {
    pub emask: [u64; 2],
    pub read_reg0: u32,
    pub read_reg1: u32,
    pub read_reg2: u32,
    pub read_reg3: u32,
}

pub struct VMEnvironment {
    pub r_registers: [u64; 8],
    // FIXME: check the type of float to be used.
    pub f_registers: [f64; 4],
    pub e_registers: [f64; 4],
    pub a_registers: [[f64; 2]; 4],
    // FIXME: ma and mx must be aligned.
    /// Contains the memory address of the next Dataset read
    pub ma: u32,
    /// Contains the memory address of the next Dataset prefetch
    pub mx: u32,

    pub fprc: [bool; 2],

    // FIXME: correct type?
    pub ic: u32,
    pub sp_addr0: u32,
    pub sp_addr1: u32,
    pub program_buffer: [u64; RANDOMX_PROGRAM_SIZE as usize],

    pub scratchpad: [u8; RANDOMX_SCRATCHPAD_L3 as usize],
    pub configuration: ProgramConfiguration,
}

impl Default for VMEnvironment {
    /// Initialize a new VMEnvironment.
    /// It also performs the initialization described in
    /// [4.6.1](https://github.com/tevador/RandomX/blob/master/doc/specs.md#461-initialization).
    fn default() -> VMEnvironment {
        let ma = 0;
        let mx = 0;
        let r_registers = [0; 8];
        VMEnvironment {
            r_registers,
            f_registers: [0.0; 4],
            e_registers: [0.0; 4],
            a_registers: [[0.0; 2]; 4],
            ma,
            mx,
            fprc: [false; 2],
            ic: RANDOMX_PROGRAM_ITERATIONS,
            sp_addr0: mx,
            sp_addr1: ma,
            program_buffer: [0; RANDOMX_PROGRAM_SIZE as usize],
            scratchpad: [0; RANDOMX_SCRATCHPAD_L3 as usize],
            // FIXME:
            configuration: ProgramConfiguration {
                emask: [0; 2],
                read_reg0: 0,
                read_reg1: 0,
                read_reg2: 0,
                read_reg3: 0,
            },
        }
    }
}

impl VMEnvironment {
    /// Load the program into the program buffer of the environment
    pub fn load_program(_env: &mut Self, _filename: String) {
        //
    }
}

#[allow(non_camel_case_types)]
pub enum Instruction {
    // Integer instruction
    IADD_RS = 0,
    IADD_M = 1,
    ISUB_R = 2,
    ISUB_M = 3,
    IMUL_R = 4,
    IMUL_M = 5,
    IMULH_R = 6,
    IMULH_M = 7,
    ISMULH_R = 8,
    ISMULH_M = 9,
    IMUL_RCP = 10,
    INEG_R = 11,
    IXOR_R = 12,
    IXOR_M = 13,
    IROR_R = 14,
    IROL_R = 15,
    ISWAP_R = 16,
    // Float instruction
    FSWAP_R = 17,
    FADD_R = 18,
    FADD_M = 19,
    FSUB_R = 20,
    FSUB_M = 21,
    FSCAL_R = 22,
    FMUL_R = 23,
    FDIV_M = 24,
    FSQRT_R = 25,
    // Control instruction
    CBRANCH = 26,
    CFROUND = 27,
    // Store instruction
    ISTORE = 28,
    NOP = 29,
}

pub fn interpreter(_env: &mut VMEnvironment) {
    // TODO
}
