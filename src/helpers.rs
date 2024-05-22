use crate::parameters::{FLOAT_EXPONENT_MASK, FLOAT_MANTISSA_MASK};

/// The fraction has full 52 bits of precision and the exponent value ranges
/// from 0 to 31. These values are obtained from the initialization quadword (in
/// little endian format) according to Table below:
/// -----------------------
/// | bits 	| description |
/// | 0-51 	| mantissa    |
/// | 52-58 | (reserved)  |
/// | 59-63 | exponent    |
/// -----------------------
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
