use crate::parameters::{FLOAT_EXPONENT_MASK, FLOAT_MANTISSA_MASK};

pub fn f64_from_u64(v: u64) -> f64 {
    // We must shift the 4 bits of the exponent on the right of the fraction
    // get bits 59-63
    let exponent: u64 = v >> 59;
    let mantissa: u64 = FLOAT_MANTISSA_MASK & v;
    let exponent: u64 = exponent + (1 << 10) - 1;
    let exponent: u64 = FLOAT_EXPONENT_MASK & exponent;
    let exponent: u64 = exponent << 52;
    let res = exponent | mantissa;
    f64::from_bits(res)
}
