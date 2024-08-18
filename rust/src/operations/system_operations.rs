use std::mem;
use std::{cmp::min, collections::HashMap};

use primitive_types::U256;

use crate::{
    evm,
    state::{BlockchainState, SerializedBytes, SerializedU256},
};

use super::{read_memory_bytes, write_memory};

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

pub fn return_value(stack: &mut Vec<U256>, memory: &mut Vec<u8>) -> Vec<u8> {
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();

    read_memory_bytes(memory, offset, size)
}

pub fn revert_context(stack: &mut Vec<U256>, memory: &mut Vec<u8>) -> Vec<u8> {
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();

    let value = read_memory_bytes(memory, offset, size);

    stack.iter().for_each(mem::drop);

    value
}

#[derive(PartialEq, Eq)]
pub enum ContextType<'a> {
    Writeable,
    WriteableDelegate(&'a mut HashMap<U256, U256>),
    Static,
}

pub fn call_context(
    stack: &mut Vec<U256>,
    memory: &mut Vec<u8>,
    chain_state: &mut BlockchainState,
    context_type: ContextType<'_>,
    parent_context_is_static: bool,
) -> Option<Vec<u8>> {
    let _gas = stack.pop().unwrap();
    let address = stack
        .pop()
        .map(|address| SerializedU256 { value: address })
        .unwrap();

    if context_type == ContextType::Writeable {
        let _value = stack.pop().unwrap();

        // TODO: disallow CALL operation if parent context is static and value != 0
        // if parent_context_is_static && value != U256::zero() {
        //     return Some(revert_context(stack, memory));
        // }
    }

    let args_offset = stack.pop().unwrap().as_usize();
    let args_size = stack.pop().unwrap().as_usize();

    let ret_offset = stack.pop().unwrap().as_usize();
    let _ret_size = stack.pop().unwrap().as_usize();

    let code: Vec<u8> = chain_state
        .contracts_state
        .get(&address)
        .unwrap()
        .code
        .clone()
        .unwrap()
        .bin
        .unwrap()
        .into();

    let new_call_data = if args_size > 0 {
        let args = read_memory_bytes(memory, args_offset, args_size);
        Some(SerializedBytes { value: args })
    } else {
        None
    };

    let previous_call_data = chain_state.tx.data.clone();
    let previous_address = chain_state.tx.to;
    let previous_caller = chain_state.tx.from;

    if context_type == ContextType::Writeable || context_type == ContextType::Static {
        chain_state.tx.to = Some(address);
        chain_state.tx.from = previous_address;
    }

    chain_state.tx.data = new_call_data;

    let is_new_context_static = context_type == ContextType::Static;
    let storage: &mut HashMap<U256, U256> =
        if let ContextType::WriteableDelegate(storage) = context_type {
            storage
        } else {
            &mut HashMap::new()
        };

    let res = evm(&code, memory, storage, chain_state, is_new_context_static);
    let has_reverted = !res.success;

    if let Some(ref ret) = res.ret {
        write_memory(memory, ret_offset, ret.clone()); // TODO: check return size
    }

    chain_state.tx.data = previous_call_data;
    chain_state.tx.from = previous_caller;
    chain_state.tx.to = previous_address;

    let ret_code = if has_reverted {
        U256::zero()
    } else {
        U256::one()
    };
    stack.push(ret_code);

    res.ret
}
