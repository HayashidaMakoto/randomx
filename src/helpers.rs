use crate::parameters::{
    FLOAT_EXPONENT_MASK, FLOAT_MANTISSA_MASK, FLOAT_MANTISSA_SIZE, RANDOMX_CONST_EXPONENT_BITS,
    RANDOMX_DYNAMIC_EXPONENT_BITS, RANDOMX_STATIC_EXPONENT_BITS,
};

/// The fraction has full 52 bits of precision and the exponent value ranges
/// from 0 to 31. These values are obtained from the initialization quadword (in
/// little endian format) according to Table below:
/// -----------------------
/// | bits  | description |
/// | 0-51  | mantissa    |
/// | 52-58 | (reserved)  |
/// | 59-63 | exponent    |
/// -----------------------
/// The function returns a u64 only because we do care about th eactual bytes,
/// and not the type/representation f64.
pub fn f64_from_u64(v: u64) -> u64 {
    // We must shift the 4 bits of the exponent on the right of the fraction
    // get bits 59-63
    let exponent: u64 = v >> 59;
    let mantissa: u64 = FLOAT_MANTISSA_MASK & v;
    let exponent: u64 = exponent + (1 << 10) - 1;
    let exponent: u64 = FLOAT_EXPONENT_MASK & exponent;
    let exponent: u64 = exponent << 52;
    exponent | mantissa
}

pub fn static_exponent(v: u64) -> u64 {
    let exponent = RANDOMX_CONST_EXPONENT_BITS;
    let exponent =
        exponent | (v >> (64 - RANDOMX_STATIC_EXPONENT_BITS) << RANDOMX_DYNAMIC_EXPONENT_BITS);
    exponent << FLOAT_MANTISSA_SIZE
}

pub fn float_mask(v: u64) -> u64 {
    (v & ((1 << 22) - 1)) | (static_exponent(v))
}
