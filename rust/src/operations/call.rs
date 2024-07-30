use std::cmp::min;

use primitive_types::U256;

use crate::state::BlockchainState;

use super::write_memory;

pub fn call_value(stack: &mut Vec<U256>, chain_state: &mut BlockchainState) {
    let value: U256 = chain_state.tx.value.unwrap_or_default().into();

    stack.push(value);
}

pub fn call_data_load(stack: &mut Vec<U256>, chain_state: &mut BlockchainState) {
    let offset = stack.pop().unwrap().as_usize();
    let data: Vec<u8> = chain_state.tx.data.clone().unwrap_or_default().into();

    let mut buf = vec![0u8; 32];
    let end = min(offset + 32, data.len());
    data[offset..end].clone_into(&mut buf);

    let buf_len = buf.len();
    if buf_len < 32 {
        buf.append(&mut vec![0u8; 32 - buf_len])
    }

    stack.push(U256::from_big_endian(&buf));
}

pub fn call_data_size(stack: &mut Vec<U256>, chain_state: &mut BlockchainState) {
    let data = chain_state.tx.data.clone();
    let size = data.map(|ref data| data.len()).unwrap_or_default();

    stack.push(size.into());
}

pub fn call_data_copy(
    stack: &mut Vec<U256>,
    memory: &mut Vec<u8>,
    chain_state: &mut BlockchainState,
) {
    let data: Vec<u8> = chain_state.tx.data.clone().unwrap_or_default().into();
    let memory_dest_offset = stack.pop().unwrap().as_usize();
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();

    let mut buf = vec![0u8; size];
    let end = min(offset + size, data.len());
    data[offset..end].clone_into(&mut buf);

    let buf_len = buf.len();
    if buf_len < 32 {
        buf.append(&mut vec![0u8; 64 - buf_len])
    }

    write_memory(memory, memory_dest_offset, buf)
}
