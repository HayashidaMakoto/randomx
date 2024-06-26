use crate::parameters::RANDOMX_SUPERSCALAR_MAX_SIZE;
use crate::vm::Instruction;
use crate::BlakeGenerator;

#[allow(non_camel_case_types)]
pub enum SuperscalarInstructionType {
    ISUB_R = 0,    //1               p015                    1               3 (sub)
    IXOR_R = 1,    //1               p015                    1               3 (xor)
    IADD_RS = 2,   //1               p01                     1               4 (lea)
    IMUL_R = 3,    //1               p1                      3               4 (imul)
    IROR_C = 4,    //1               p05                     1               4 (ror)
    IADD_C7 = 5,   //1               p015                    1               7 (add)
    IXOR_C7 = 6,   //1               p015                    1               7 (xor)
    IADD_C8 = 7,   //1+0             p015                    1               7+1 (add+nop)
    IXOR_C8 = 8,   //1+0             p015                    1               7+1 (xor+nop)
    IADD_C9 = 9,   //1+0             p015                    1               7+2 (add+nop)
    IXOR_C9 = 10,  //1+0             p015                    1               7+2 (xor+nop)
    IMULH_R = 11,  //1+2+1           0+(p1,p5)+0             3               3+3+3 (mov+mul+mov)
    ISMULH_R = 12, //1+2+1           0+(p1,p5)+0             3               3+3+3 (mov+imul+mov)
    IMUL_RCP = 13, //1+1             p015+p1                 4              10+4   (mov+imul)

    COUNT = 14,
    INVALID = -1,
}

impl SuperscalarInstructionType {
    pub fn is_multiplication(self) -> bool {
        matches!(
            self,
            SuperscalarInstructionType::IMUL_R
                | SuperscalarInstructionType::IMULH_R
                | SuperscalarInstructionType::ISMULH_R
                | SuperscalarInstructionType::IMUL_RCP
        )
    }
}

pub enum ExecutionPort {
    Null = 0,
    P0 = 1,
    P1 = 2,
    P5 = 4,
    P01 = 1 | 2,
    P05 = 1 | 4,
    P015 = 1 | 2 | 4,
}

// FIXME: check types
pub struct SuperscalarProgram {
    pub size: u32,
    pub addr_reg: u32,
    pub code_size: u32,
    pub macro_ops: u32,
    pub decode_cycles: u32,
    pub cpu_latency: u32,
    pub asic_latency: u32,
    pub mul_count: u32,
    pub cpu_latencies: [u32; 8],
    pub asic_latencies: [u32; 8],
    // FIXME
    pub ipc: f32,
    pub program_buffer: [Instruction; RANDOMX_SUPERSCALAR_MAX_SIZE as usize],
}

impl SuperscalarProgram {
    pub fn generate(_gen: &mut BlakeGenerator) -> Self {
        // let mut current_instruction = Instruction::
        let mut _macro_op_index = 0;
        let mut _code_size = 0;
        let mut _macro_op_count = 0;
        let mut _cycle = 0;
        let mut _dep_cycle = 0;
        let mut _retire_cycle = 0;
        let mut _ports_saturated: bool = false;
        let mut _program_size = 0;
        let mut _mul_count = 0;
        let mut _decode_cycle = 0;
        let mut _throw_away_count = 0;
        unimplemented!()
    }
}
