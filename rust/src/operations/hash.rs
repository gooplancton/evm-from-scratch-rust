use primitive_types::U256;
use sha3::{Digest, Keccak256};

pub fn keccak256(stack: &mut Vec<U256>, memory: &mut Vec<u8>) {
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();

    let value = &memory[offset..(offset + size)];

    let mut hasher = Keccak256::new();
    hasher.update(&value);

    let hashed: U256 = hasher.finalize()[..].into();

    stack.push(hashed);
}

