use primitive_types::U256;
use sha3::{Digest, Keccak256};

use crate::state::BlockchainState;

pub fn keccak256(stack: &mut Vec<U256>, memory: &mut Vec<u8>) {
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();

    let value = &memory[offset..(offset + size)];

    let mut hasher = Keccak256::new();
    hasher.update(&value);

    let hashed: U256 = hasher.finalize()[..].into();

    stack.push(hashed);
}

pub fn external_code_hash(stack: &mut Vec<U256>, state: &BlockchainState) {
    let address = stack.pop().unwrap();

    let code = state.contracts_state.get(&(address.into()));
    let code = code.map(|entry| entry.code.clone().unwrap_or_default());

    if code.is_none() {
        stack.push(0.into());
        return;
    }

    let bin: Vec<u8> = code.unwrap().bin.unwrap().into();
    let mut hasher = Keccak256::new();
    hasher.update(&bin);

    let hashed: U256 = hasher.finalize()[..].into();

    stack.push(hashed);
}
