use randomx::helpers::f64_from_u64;

#[test]
fn test_unitest_from_ref_implementation() {
    let input: u64 = 14955972954624606980;
    let res: f64 = f64_from_u64(input);
    let exp_output: f64 = 63522095.84363368;
    assert_eq!(res, exp_output)
}
