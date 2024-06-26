use std::str::FromStr;

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;
use blake2::{Blake2b512, Digest};
use randomx::BlakeGenerator;

#[test]
// Test the keys have been generated as described in the specification.
// At the same time, we check that the version of Blake2b512 we use is
// consistent.
fn test_aesgenerator_1r_key_consistency() {
    let mut hasher = Blake2b512::new();

    hasher.update(b"RandomX AesGenerator1R keys");

    let exp_k0: [u8; 16] = [
        0x53, 0xa5, 0xac, 0x6d, 0x09, 0x66, 0x71, 0x62, 0x2b, 0x55, 0xb5, 0xdb, 0x17, 0x49, 0xf4,
        0xb4,
    ];

    let exp_k1: [u8; 16] = [
        0x07, 0xaf, 0x7c, 0x6d, 0x0d, 0x71, 0x6a, 0x84, 0x78, 0xd3, 0x25, 0x17, 0x4e, 0xdc, 0xa1,
        0x0d,
    ];

    let exp_k2: [u8; 16] = [
        0xf1, 0x62, 0x12, 0x3f, 0xc6, 0x7e, 0x94, 0x9f, 0x4f, 0x79, 0xc0, 0xf4, 0x45, 0xe3, 0x20,
        0x3e,
    ];

    let exp_k3: [u8; 16] = [
        0x35, 0x81, 0xef, 0x6a, 0x7c, 0x31, 0xba, 0xb1, 0x88, 0x4c, 0x31, 0x16, 0x54, 0x91, 0x16,
        0x49,
    ];
    let res = hasher.finalize();

    assert_eq!(exp_k0, res[0..16]);
    assert_eq!(exp_k1, res[16..32]);
    assert_eq!(exp_k2, res[32..48]);
    assert_eq!(exp_k3, res[48..64]);
}

#[test]
fn test_aesgenerator_4r_key_consistency() {
    let key0: [u8; 16] = [
        0xdd, 0xaa, 0x21, 0x64, 0xdb, 0x3d, 0x83, 0xd1, 0x2b, 0x6d, 0x54, 0x2f, 0x3f, 0xd2, 0xe5,
        0x99,
    ];
    let key1: [u8; 16] = [
        0x50, 0x34, 0x0e, 0xb2, 0x55, 0x3f, 0x91, 0xb6, 0x53, 0x9d, 0xf7, 0x06, 0xe5, 0xcd, 0xdf,
        0xa5,
    ];
    let key2: [u8; 16] = [
        0x04, 0xd9, 0x3e, 0x5c, 0xaf, 0x7b, 0x5e, 0x51, 0x9f, 0x67, 0xa4, 0x0a, 0xbf, 0x02, 0x1c,
        0x17,
    ];
    let key3: [u8; 16] = [
        0x63, 0x37, 0x62, 0x85, 0x08, 0x5d, 0x8f, 0xe7, 0x85, 0x37, 0x67, 0xcd, 0x91, 0xd2, 0xde,
        0xd8,
    ];

    let mut hasher = Blake2b512::new();
    hasher.update(b"RandomX AesGenerator4R keys 0-3");
    let res = hasher.finalize();

    assert_eq!(key0, res[0..16]);
    assert_eq!(key1, res[16..32]);
    assert_eq!(key2, res[32..48]);
    assert_eq!(key3, res[48..64]);

    let key4: [u8; 16] = [
        0x73, 0x6f, 0x82, 0xb5, 0xa6, 0xa7, 0xd6, 0xe3, 0x6d, 0x8b, 0x51, 0x3d, 0xb4, 0xff, 0x9e,
        0x22,
    ];
    let key5: [u8; 16] = [
        0xf3, 0x6b, 0x56, 0xc7, 0xd9, 0xb3, 0x10, 0x9c, 0x4e, 0x4d, 0x02, 0xe9, 0xd2, 0xb7, 0x72,
        0xb2,
    ];
    let key6: [u8; 16] = [
        0xe7, 0xc9, 0x73, 0xf2, 0x8b, 0xa3, 0x65, 0xf7, 0x0a, 0x66, 0xa9, 0x2b, 0xa7, 0xef, 0x3b,
        0xf6,
    ];
    let key7: [u8; 16] = [
        0x09, 0xd6, 0x7c, 0x7a, 0xde, 0x39, 0x58, 0x91, 0xfd, 0xd1, 0x06, 0x0c, 0x2d, 0x76, 0xb0,
        0xc0,
    ];

    let mut hasher = Blake2b512::new();
    hasher.update(b"RandomX AesGenerator4R keys 4-7");
    let res = hasher.finalize();

    assert_eq!(key4, res[0..16]);
    assert_eq!(key5, res[16..32]);
    assert_eq!(key6, res[32..48]);
    assert_eq!(key7, res[48..64]);
}

#[test]
fn test_regtest_aes_generator_1r() {
    let key0: [u8; 16] = [
        0x53, 0xa5, 0xac, 0x6d, 0x09, 0x66, 0x71, 0x62, 0x2b, 0x55, 0xb5, 0xdb, 0x17, 0x49, 0xf4,
        0xb4,
    ];
    let key0 = GenericArray::from(key0);

    let cipher = Aes128::new(&key0);

    let mut state0 = GenericArray::from([0; 16]);
    cipher.decrypt_block(&mut state0);
    let exp_output: [u8; 16] = [
        202, 211, 173, 142, 251, 44, 222, 33, 36, 58, 219, 164, 0, 30, 149, 104,
    ];
    assert_eq!(state0, exp_output.into());
}

#[test]
fn test_vectors_aes_generator_1r() {
    // FIXME: https://github.com/tevador/RandomX/blob/master/src/tests/tests.cpp#L172
    let mut input: [u8; 64] = [0; 64];
    let test_vector: [u8; 32] = [
        0x6c, 0x19, 0x53, 0x6e, 0xb2, 0xde, 0x31, 0xb6, 0xc0, 0x06, 0x5f, 0x7f, 0x11, 0x6e, 0x86,
        0xf9, 0x60, 0xd8, 0xaf, 0x0c, 0x57, 0x21, 0x0a, 0x65, 0x84, 0xc3, 0x23, 0x7b, 0x9d, 0x06,
        0x4d, 0xc7,
    ];
    input[0..32].copy_from_slice(&test_vector);

    let exp_output: [u8; 32] = [
        0xfa, 0x89, 0x39, 0x7d, 0xd6, 0xca, 0x42, 0x25, 0x13, 0xae, 0xad, 0xba, 0x3f, 0x12, 0x4b,
        0x55, 0x40, 0x32, 0x4c, 0x4a, 0xd4, 0xb6, 0xdb, 0x43, 0x43, 0x94, 0x30, 0x7a, 0x17, 0xc8,
        0x33, 0xab,
    ];

    let output = randomx::aes_generator_1r(input);
    println!("{:?}", output);
    println!("{:?}", exp_output);
}

#[test]
fn test_vectors_aes_generator_4r() {
    // FIXME
}

#[test]
fn test_blake2_generator_init_with_nonce() {
    // Test generated from reference implementation, commit 89aba80
    let seed: String = String::from_str("test key 000").unwrap();
    let seed_u8: Vec<u8> = seed.into_bytes();
    {
        let blake2_generator = BlakeGenerator::from_seed(seed_u8.clone(), 32);
        assert_eq!(blake2_generator.data[60], 32);
        assert_eq!(blake2_generator.data[61], 0);
        assert_eq!(blake2_generator.data[62], 0);
        assert_eq!(blake2_generator.data[63], 0);
    }
    {
        let blake2_generator = BlakeGenerator::from_seed(seed_u8.clone(), 257);
        assert_eq!(blake2_generator.data[60], 1);
        assert_eq!(blake2_generator.data[61], 1);
        assert_eq!(blake2_generator.data[62], 0);
        assert_eq!(blake2_generator.data[63], 0);
    }

    {
        let blake2_generator = BlakeGenerator::from_seed(seed_u8.clone(), 256 * 256 + 1);
        assert_eq!(blake2_generator.data[60], 1);
        assert_eq!(blake2_generator.data[61], 0);
        assert_eq!(blake2_generator.data[62], 1);
        assert_eq!(blake2_generator.data[63], 0);
    }
}

#[test]
fn test_blake2_generator_get_byte() {
    // Test generated from reference implementation, commit 89aba80
    let seed: String = String::from_str("test key 000").unwrap();
    let seed_u8: Vec<u8> = seed.into_bytes();
    let mut blake2_generator = BlakeGenerator::from_seed(seed_u8, 0);
    let res = blake2_generator.get_byte();
    let exp_res: u8 = 216;
    assert_eq!(exp_res, res);
}
