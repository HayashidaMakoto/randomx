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
    pub cpu_latencies: [u32;8],
    pub asic_latencies: [u32; 8],
    // FIXME
    pub ipc: f32,
}
