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
    let _vm_env: VMEnvironment = VMEnvironment::from_configuration(&config);
}
