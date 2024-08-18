use std::mem;

use primitive_types::U256;

use super::read_memory_bytes;

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

