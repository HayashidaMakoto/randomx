use std::ffi::c_float;

use crate::parameters::{RANDOMX_PROGRAM_ITERATIONS, RANDOMX_PROGRAM_SIZE, RANDOMX_SCRATCHPAD_L3};

/// Each instruction word is 64 bits long
/// 63         32       24      16       8         0
/// ------------------------------------------------
/// |  imm32    |    mod |   src |   dst |  opcode |
/// ------------------------------------------------
pub type Instruction = u64;

/// > A 32-bit immediate value that can be used as the source operand and is
/// > used to calculate addresses for memory operations. The immediate value is
/// > sign-extended to 64 bits unless specified otherwise.
pub fn imm32(i: Instruction) -> u32 {
    (i >> 32) as u32
}

pub fn mod_(i: Instruction) -> u8 {
    (i >> 24) as u8
}

pub fn src(i: Instruction) -> u8 {
    (i >> 16) as u8
}

pub fn dst(i: Instruction) -> u8 {
    (i >> 8) as u8
}

/// Return the opcode of the instruction
/// > There are 256 opcodes, which are distributed between 29 distinct
/// > instructions. Each instruction can be encoded using multiple opcodes (the
/// > number of opcodes specifies the frequency of the instruction in a random
/// > program).
pub fn opcode(i: Instruction) -> u8 {
    i as u8
}

pub struct VMEnvironment {
    pub r_registers: [u64; 8],
    // FIXME: check the type of float to be used.
    pub f_registers: [c_float; 4],
    pub e_registers: [c_float; 4],
    pub a_registers: [c_float; 4],
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
    pub program_buffer: [u64; RANDOMX_PROGRAM_SIZE],

    pub scratchpad: [u8; RANDOMX_SCRATCHPAD_L3],
}

impl Default for VMEnvironment {
    fn default() -> VMEnvironment {
        let ma = 0;
        let mx = 0;
        let r_registers = [0; 8];
        VMEnvironment {
            r_registers,
            f_registers: [0.0; 4],
            e_registers: [0.0; 4],
            a_registers: [0.0; 4],
            ma,
            mx,
            fprc: [false; 2],
            ic: RANDOMX_PROGRAM_ITERATIONS,
            sp_addr0: mx,
            sp_addr1: ma,
            program_buffer: [0; RANDOMX_PROGRAM_SIZE],
            scratchpad: [0; RANDOMX_SCRATCHPAD_L3],
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
pub enum IntegerInstruction {
    IADD_RS,
    IADD_M,
    ISUB_R,
    ISUB_M,
    IMUL_R,
    IMUL_M,
    IMULH_R,
    IMULH_M,
    IMUL_RCP,
    INEG_R,
    IXOR_R,
    IXOR_M,
    IROR_R,
    IROL_R,
    ISWAP_R,
}

#[allow(non_camel_case_types)]
pub enum FloatInstruction {
    FSWAP_R,
    FADD_R,
    FADD_M,
    FSUB_R,
    FSUB_M,
    FSCAL_R,
    FMUL_R,
    FDIV_M,
    FSQRT_R,
}

#[allow(non_camel_case_types)]
pub enum ControlInstruction {
    CFROUND,
    CBRANCH,
}

#[allow(non_camel_case_types)]
pub enum StoreInstruction {
    ISTORE,
}

pub fn interpreter(_env: &mut VMEnvironment) {
    // TODO
}
