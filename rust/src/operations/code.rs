use std::cmp::min;

use primitive_types::U256;

use crate::state::{BlockchainState, ContractsStateDataEntry};

use super::write_memory;

pub fn code_copy(stack: &mut Vec<U256>, memory: &mut Vec<u8>, code: &Vec<u8>) {
    let memory_dest_offset = stack.pop().unwrap().as_usize();
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();

    let mut buf = vec![0u8; size];
    let end = min(offset + size, code.len());
    code[offset..end].clone_into(&mut buf);

    let buf_len = buf.len();
    if buf_len < 32 {
        buf.append(&mut vec![0u8; 64 - buf_len])
    }

    write_memory(memory, memory_dest_offset, buf)
}

pub fn external_code_size(stack: &mut Vec<U256>, state: &BlockchainState) {
    let address = stack.pop().unwrap();

    let code = state.contracts_state.get(&(address.into()));
    let len = code
        .map(|entry| entry.code.clone().unwrap().len())
        .unwrap_or_default();

    stack.push(len.into())
}

pub fn external_code_copy(stack: &mut Vec<U256>, memory: &mut Vec<u8>, state: &BlockchainState) {
    let address = stack.pop().unwrap();
    let memory_dest_offset = stack.pop().unwrap().as_usize();
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();

    let code = state
        .contracts_state
        .get(&(address.into()))
        .map(ContractsStateDataEntry::to_owned)
        .map(|entry| entry.code)
        .flatten();

    if code.is_none() {
        stack.push(0.into());
        return;
    }

    let bin: Vec<u8> = code.unwrap().bin.unwrap_or_default().into();
    let mut buf = vec![0u8; size];
    let end = min(offset + size, bin.len());
    bin[offset..end].clone_into(&mut buf);

    let buf_len = buf.len();
    if buf_len < 32 {
        buf.append(&mut vec![0u8; 64 - buf_len])
    }

    write_memory(memory, memory_dest_offset, buf)
}
