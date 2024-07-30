use std::cmp::min;

use primitive_types::U256;

fn expand_memory(memory: &mut Vec<u8>, new_size: usize) {
    let current_size = memory.len();
    let padding = if new_size % 32 == 0 {
        0
    } else {
        31 - (new_size % 32)
    };

    memory.append(&mut vec![0u8; new_size + padding - current_size + 1]);
}

pub fn read_memory(memory: &mut Vec<u8>, offset: usize) -> U256 {
    let current_size = memory.len();
    let new_size = offset + 32 - 1;
    if new_size >= current_size {
        expand_memory(memory, new_size);
    }

    let mut value = U256::zero();
    for i in offset..=new_size {
        value <<= 8;
        value += memory[i].into();
    }

    value
}

pub fn write_memory(memory: &mut Vec<u8>, offset: usize, bytes: impl IntoIterator<Item = u8>) {
    let current_size = memory.len();
    let bytes = bytes.into_iter().collect::<Vec<_>>();
    let new_size = offset + bytes.len() - 1;
    if new_size >= current_size {
        expand_memory(memory, new_size);
    }

    bytes
        .into_iter()
        .enumerate()
        .for_each(move |(i, byte)| memory[offset + i] = byte);
}

pub fn memstore(stack: &mut Vec<U256>, memory: &mut Vec<u8>) {
    let offset = stack.pop().unwrap().as_usize();
    let word = stack.pop().unwrap();

    let mut word_bytes = [0u8; 32];
    word.to_big_endian(&mut word_bytes);

    write_memory(memory, offset, word_bytes)
}

pub fn memstore8(stack: &mut Vec<U256>, memory: &mut Vec<u8>) {
    let offset = stack.pop().unwrap().as_usize();
    let word = (stack.pop().unwrap() << 248) >> 248;
    let byte: u8 = word.try_into().unwrap();

    write_memory(memory, offset, [byte])
}

pub fn memload(stack: &mut Vec<U256>, memory: &mut Vec<u8>) {
    let offset = stack.pop().unwrap().as_usize();

    let value = read_memory(memory, offset);

    stack.push(value);
}

pub fn memsize(stack: &mut Vec<U256>, memory: &Vec<u8>) {
    stack.push(memory.len().into());
}

