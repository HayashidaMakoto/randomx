use randomx::helpers::f64_from_u64;

#[test]
fn test_unitest_from_ref_implementation() {
    let input: u64 = 14955972954624606980;
    let res: u64 = f64_from_u64(input);
    // Converting into hexadecimal because the print does not work properly
    // Generated from the reference implementation using:
    // ```
    // memcpy(&u, &reg.a[0].lo, sizeof(reg.a[0].lo));
    // std::cout << "reg a0 lo hexa: " << std::hex << u << std::endl;
    // ````

    let exp_hexa_output = [0x41, 0x8e, 0x4a, 0x29, 0x7e, 0xbf, 0xc3, 0x04];
    let hexa_output = res.to_be_bytes();
    assert_eq!(exp_hexa_output, hexa_output)
}
