use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use blake2::{Blake2b512, Digest};

pub mod helpers;
pub mod parameters;
pub mod superscalar;
pub mod vm;

/// Implement [AesGenerator1R](https://github.com/tevador/RandomX/blob/master/doc/specs.md#32-aesgenerator1r).
///
/// TODO: implement it in assembly. It is pretty straightforward.
/// Specification goes like this:
///
/// ### 3.2 AesGenerator1R
/// AesGenerator1R produces a sequence of pseudo-random bytes.
///
/// The internal state of the generator consists of 64 bytes arranged into four
/// columns of 16 bytes each. During each output iteration, every column is
/// decrypted (columns 0, 2) or encrypted (columns 1, 3) with one AES round
/// using the following round keys (one key per column):
///
/// ```text
/// key0 = 53 a5 ac 6d 09 66 71 62 2b 55 b5 db 17 49 f4 b4
/// key1 = 07 af 7c 6d 0d 71 6a 84 78 d3 25 17 4e dc a1 0d
/// key2 = f1 62 12 3f c6 7e 94 9f 4f 79 c0 f4 45 e3 20 3e
/// key3 = 35 81 ef 6a 7c 31 ba b1 88 4c 31 16 54 91 16 49
/// ```
/// These keys were generated as:
/// ```text
/// key0, key1, key2, key3 = Hash512("RandomX AesGenerator1R keys")
/// ```
///
/// Single iteration produces 64 bytes of output which also become the new generator state.
/// ```text
/// state0 (16 B)    state1 (16 B)    state2 (16 B)    state3 (16 B)
///      |                |                |                |
///  AES decrypt      AES encrypt      AES decrypt      AES encrypt
///    (key0)           (key1)           (key2)           (key3)
///      |                |                |                |
///      v                v                v                v
///   state0'          state1'          state2'          state3'
/// ```
pub fn aes_generator_1r(input: [u8; 64]) -> [u8; 64] {
    // key0 - decrypt
    let key0 = GenericArray::from(parameters::AES_GENERATOR_1R_K0);
    let cipher = Aes128::new(&key0);
    let state0: [u8; 16] = input[0..16].try_into().unwrap();
    let mut state0 = GenericArray::from(state0);
    cipher.decrypt_block(&mut state0);

    // key1 - encrypt
    let key1 = GenericArray::from(parameters::AES_GENERATOR_1R_K1);
    let cipher = Aes128::new(&key1);
    let state1: [u8; 16] = input[16..32].try_into().unwrap();
    let mut state1 = GenericArray::from(state1);
    cipher.encrypt_block(&mut state1);

    // key2 - decrypt
    let key2 = GenericArray::from(parameters::AES_GENERATOR_1R_K2);
    let cipher = Aes128::new(&key2);
    let state2: [u8; 16] = input[32..48].try_into().unwrap();
    let mut state2 = GenericArray::from(state2);
    cipher.decrypt_block(&mut state2);

    // key3 - encrypt
    let key3 = GenericArray::from(parameters::AES_GENERATOR_1R_K3);
    let cipher = Aes128::new(&key3);
    let state3: [u8; 16] = input[48..64].try_into().unwrap();
    let mut state3 = GenericArray::from(state3);
    cipher.encrypt_block(&mut state3);

    let mut output: [u8; 64] = [0; 64];
    output[0..16].copy_from_slice(state0.as_slice());
    output[16..32].copy_from_slice(state1.as_slice());
    output[32..48].copy_from_slice(state2.as_slice());
    output[48..64].copy_from_slice(state3.as_slice());

    output
}

/// Implement [AesGenerator4R](https://github.com/tevador/RandomX/blob/master/doc/specs.md#32-aesgenerator4r).
///
/// TODO: implement it in assembly. It is pretty straightforward.
///
/// Specification goes like this:
///
/// ### 3.3 AesGenerator4R
///
/// AesGenerator4R works similar way as AesGenerator1R, except it uses 4 rounds
/// per column. Columns 0 and 1 use a different set of keys than columns 2 and
/// 3.
///
/// ```text
/// state0 (16 B)    state1 (16 B)    state2 (16 B)    state3 (16 B)
///      |                |                |                |
///  AES decrypt      AES encrypt      AES decrypt      AES encrypt
///    (key0)           (key0)           (key4)           (key4)
///      |                |                |                |
///      v                v                v                v
///  AES decrypt      AES encrypt      AES decrypt      AES encrypt
///    (key1)           (key1)           (key5)           (key5)
///      |                |                |                |
///      v                v                v                v
///  AES decrypt      AES encrypt      AES decrypt      AES encrypt
///    (key2)           (key2)           (key6)           (key6)
///      |                |                |                |
///      v                v                v                v
///  AES decrypt      AES encrypt      AES decrypt      AES encrypt
///    (key3)           (key3)           (key7)           (key7)
///      |                |                |                |
///      v                v                v                v
///   state0'          state1'          state2'          state3'
/// ```
pub fn aes_generator_4r(input: [u8; 64]) -> [u8; 64] {
    // state0
    let key0 = GenericArray::from(parameters::AES_GENERATOR_4R_K0);
    let key1 = GenericArray::from(parameters::AES_GENERATOR_4R_K1);
    let key2 = GenericArray::from(parameters::AES_GENERATOR_4R_K2);
    let key3 = GenericArray::from(parameters::AES_GENERATOR_4R_K3);
    let key4 = GenericArray::from(parameters::AES_GENERATOR_4R_K4);
    let key5 = GenericArray::from(parameters::AES_GENERATOR_4R_K5);
    let key6 = GenericArray::from(parameters::AES_GENERATOR_4R_K6);
    let key7 = GenericArray::from(parameters::AES_GENERATOR_4R_K7);
    let state0 = {
        let cipher = Aes128::new(&key0);
        let state: [u8; 16] = input[0..16].try_into().unwrap();
        let mut state = GenericArray::from(state);
        cipher.decrypt_block(&mut state);

        let cipher = Aes128::new(&key1);
        cipher.decrypt_block(&mut state);

        let cipher = Aes128::new(&key2);
        cipher.decrypt_block(&mut state);

        let cipher = Aes128::new(&key3);
        cipher.decrypt_block(&mut state);
        state
    };

    let state1 = {
        let cipher = Aes128::new(&key0);
        let state: [u8; 16] = input[16..32].try_into().unwrap();
        let mut state = GenericArray::from(state);
        cipher.encrypt_block(&mut state);

        let cipher = Aes128::new(&key1);
        cipher.encrypt_block(&mut state);

        let cipher = Aes128::new(&key2);
        cipher.encrypt_block(&mut state);

        let cipher = Aes128::new(&key3);
        cipher.encrypt_block(&mut state);
        state
    };

    let state2 = {
        let cipher = Aes128::new(&key4);
        let state: [u8; 16] = input[32..48].try_into().unwrap();
        let mut state = GenericArray::from(state);
        cipher.decrypt_block(&mut state);

        let cipher = Aes128::new(&key5);
        cipher.decrypt_block(&mut state);

        let cipher = Aes128::new(&key6);
        cipher.decrypt_block(&mut state);

        let cipher = Aes128::new(&key7);
        cipher.decrypt_block(&mut state);
        state
    };

    let state3 = {
        let cipher = Aes128::new(&key4);
        let state: [u8; 16] = input[48..64].try_into().unwrap();
        let mut state = GenericArray::from(state);
        cipher.encrypt_block(&mut state);

        let cipher = Aes128::new(&key5);
        cipher.encrypt_block(&mut state);

        let cipher = Aes128::new(&key6);
        cipher.encrypt_block(&mut state);

        let cipher = Aes128::new(&key7);
        cipher.encrypt_block(&mut state);
        state
    };

    let mut output: [u8; 64] = [0; 64];
    output[0..16].copy_from_slice(state0.as_slice());
    output[16..32].copy_from_slice(state1.as_slice());
    output[32..48].copy_from_slice(state2.as_slice());
    output[48..64].copy_from_slice(state3.as_slice());

    output
}

pub fn aes_hash1r(block: Vec<u8>) -> [u8; 64] {
    // Let's suppose the block is 64bytes for now.
    // I don't see how padding works.
    assert_eq!(block.len(), 64);
    let input: [u8; 64] = block.try_into().unwrap();

    let mut state0 = GenericArray::from(parameters::AES_HASH1R_STATE0);
    {
        let key0: [u8; 16] = input[0..16].try_into().unwrap();
        let key0 = GenericArray::from(key0);
        let cipher = Aes128::new(&key0);
        cipher.decrypt_block(&mut state0);
    };

    // key1 - encrypt
    let mut state1 = GenericArray::from(parameters::AES_HASH1R_STATE1);
    {
        let key1: [u8; 16] = input[16..32].try_into().unwrap();
        let key1 = GenericArray::from(key1);
        let cipher = Aes128::new(&key1);
        cipher.encrypt_block(&mut state1);
    };

    // key2 - decrypt
    let mut state2 = GenericArray::from(parameters::AES_HASH1R_STATE2);
    {
        let key2: [u8; 16] = input[32..48].try_into().unwrap();
        let key2 = GenericArray::from(key2);
        let cipher = Aes128::new(&key2);
        cipher.decrypt_block(&mut state2);
    };

    // key3 - encrypt
    let mut state3 = GenericArray::from(parameters::AES_HASH1R_STATE3);
    {
        let key3: [u8; 16] = input[48..64].try_into().unwrap();
        let key3 = GenericArray::from(key3);
        let cipher = Aes128::new(&key3);
        cipher.encrypt_block(&mut state3);
    };

    // TODO: process other blocks.
    // Now the two final rounds. We are supposed to have state0, state1, state2
    // and state3

    // Compute finalState0
    {
        let key = GenericArray::from(parameters::AES_HASH1R_XKEY0);
        let cipher = Aes128::new(&key);
        cipher.encrypt_block(&mut state0);
        let key = GenericArray::from(parameters::AES_HASH1R_XKEY1);
        let cipher = Aes128::new(&key);
        cipher.encrypt_block(&mut state0);
    }

    // Compute finalState1
    {
        let key = GenericArray::from(parameters::AES_HASH1R_XKEY0);
        let cipher = Aes128::new(&key);
        cipher.decrypt_block(&mut state1);
        let key = GenericArray::from(parameters::AES_HASH1R_XKEY1);
        let cipher = Aes128::new(&key);
        cipher.decrypt_block(&mut state1);
    }

    // Compute finalState2
    {
        let key = GenericArray::from(parameters::AES_HASH1R_XKEY0);
        let cipher = Aes128::new(&key);
        cipher.encrypt_block(&mut state2);
        let key = GenericArray::from(parameters::AES_HASH1R_XKEY1);
        let cipher = Aes128::new(&key);
        cipher.encrypt_block(&mut state2);
    }

    // Compute finalState3
    {
        let key = GenericArray::from(parameters::AES_HASH1R_XKEY0);
        let cipher = Aes128::new(&key);
        cipher.decrypt_block(&mut state3);
        let key = GenericArray::from(parameters::AES_HASH1R_XKEY1);
        let cipher = Aes128::new(&key);
        cipher.decrypt_block(&mut state3);
    }

    let mut output: [u8; 64] = [0; 64];
    output[0..16].copy_from_slice(state0.as_slice());
    output[16..32].copy_from_slice(state1.as_slice());
    output[32..48].copy_from_slice(state2.as_slice());
    output[48..64].copy_from_slice(state3.as_slice());

    output
}

pub struct BlakeGenerator {
    // The reference implementation seems to keep the nonce, as a c int, at the
    // end of the data field, 4 bytes.
    // The first 60 bytes are for the seed
    pub data: [u8; 60 + 4],
    // The data index is initialized to 64, the size of data.
    data_index: usize,
}

impl BlakeGenerator {
    pub fn from_seed(seed: Vec<u8>, nonce: i32) -> Self {
        // FIXME: the reference implementation do not raise any error here. It
        // only truncates the seed to 60 bytes.
        // Q: Any potential attack?
        assert!(seed.len() <= parameters::BLAKE_GENERATOR_SEED_MAX_SIZE as usize);

        let mut data: [u8; 64] = [0; 64];
        // We initialize with only zeroes. The seed must be padded with zeroes if
        // its length is not 60.
        // Equivalent that copying into the first N bytes.
        data[0..seed.len()].copy_from_slice(seed.as_slice());
        // FIXME: could be inlined for speed.
        data[60] = (nonce as u32) as u8;
        data[61] = ((nonce as u32) >> 8) as u8;
        data[62] = ((nonce as u32) >> 16) as u8;
        data[63] = ((nonce as u32) >> 24) as u8;
        Self {
            data,
            data_index: 64,
        }
    }

    pub fn update_state(&mut self) {
        // Run blake2b and updates the data.
        let mut hasher = Blake2b512::new();
        hasher.update(self.data);
        let res = hasher.finalize();
        assert_eq!(res.len(), 64);
        self.data.copy_from_slice(res.as_slice());
        self.data_index = 0;
    }

    pub fn get_byte(&mut self) -> u8 {
        if self.data_index + 1 > 64 {
            self.update_state()
        }
        self.data[self.data_index]
    }

    pub fn get_u32(&mut self) -> u32 {
        if self.data_index + 4 > 64 {
            self.update_state()
        }
        let mut b: [u8; 4] = [0; 4];
        b.copy_from_slice(&self.data[self.data_index..self.data_index + 4]);
        u32::from_be_bytes(b)
    }
}
