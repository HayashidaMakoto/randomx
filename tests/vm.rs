use randomx::vm::VMEnvironment;

#[test]
pub fn test_vm_environment_from_configuration() {
    // Generated from reference implementation, took the first output
    let config = [
        14955972954624606980,
        1160178653978888361,
        7804042326029050762,
        18106730710792880309,
        13963013834657230551,
        14182137327961841000,
        12343878495449872070,
        7430133026238860952,
        7984991304302844783,
        9016528251757111484,
        4740653391750521392,
        12036502532476750352,
        7396202974033214510,
        5473894568723909973,
        13992127780735554655,
        11551971154970399794,
    ];
    let vm_env: VMEnvironment = VMEnvironment::from_configuration(config);

    let hexa_exp_a0_lo: [u8; 8] = [0x41, 0x8e, 0x4a, 0x29, 0x7e, 0xbf, 0xc3, 0x04];
    let hexa_exp_a0_hi: [u8; 8] = [0x40, 0x19, 0xc8, 0x56, 0xc2, 0x67, 0x08, 0xa9];
    let hexa_exp_a1_lo: [u8; 8] = [0x40, 0xcd, 0x87, 0x25, 0xdf, 0x13, 0x23, 0x8a];
    let hexa_exp_a1_hi: [u8; 8] = [0x41, 0xe8, 0x07, 0xa5, 0xdc, 0x77, 0x40, 0xb5];
    let hexa_exp_a2_lo: [u8; 8] = [0x41, 0x76, 0x97, 0x1a, 0x78, 0x9b, 0xee, 0xd7];
    let hexa_exp_a2_hi: [u8; 8] = [0x41, 0x71, 0x12, 0xc2, 0x74, 0xf9, 0x1d, 0x68];
    let hexa_exp_a3_lo: [u8; 8] = [0x41, 0x4e, 0x44, 0x17, 0x47, 0xdf, 0x76, 0xc6];
    let hexa_exp_a3_hi: [u8; 8] = [0x40, 0xbd, 0x22, 0x9e, 0xee, 0xdd, 0x8e, 0x98];
    let hexa_exp_emask0: [u8; 8] = [0x3c, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x14, 0x5f];
    let hexa_exp_emask1: [u8; 8] = [0x3a, 0x00, 0x00, 0x00, 0x00, 0x11, 0xd4, 0x32];
    assert_eq!(vm_env.a_registers[0][1].to_be_bytes(), hexa_exp_a0_lo);
    assert_eq!(vm_env.a_registers[0][0].to_be_bytes(), hexa_exp_a0_hi);
    assert_eq!(vm_env.a_registers[1][1].to_be_bytes(), hexa_exp_a1_lo);
    assert_eq!(vm_env.a_registers[1][0].to_be_bytes(), hexa_exp_a1_hi);
    assert_eq!(vm_env.a_registers[2][1].to_be_bytes(), hexa_exp_a2_lo);
    assert_eq!(vm_env.a_registers[2][0].to_be_bytes(), hexa_exp_a2_hi);
    assert_eq!(vm_env.a_registers[3][1].to_be_bytes(), hexa_exp_a3_lo);
    assert_eq!(vm_env.a_registers[3][0].to_be_bytes(), hexa_exp_a3_hi);

    assert_eq!(vm_env.configuration.emask[0].to_be_bytes(), hexa_exp_emask0);
    assert_eq!(vm_env.configuration.emask[1].to_be_bytes(), hexa_exp_emask1);
}
