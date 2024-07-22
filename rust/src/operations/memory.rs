use primitive_types::U256;

fn expand_memory(memory: &mut Vec<u8>, new_size: usize) {
    let current_size = memory.len();
    let padding = if new_size % 32 == 0 { 0 } else { 31 - (new_size % 32) };
    for _ in current_size..=(new_size + padding) {
        memory.push(0);
    }
}

pub fn memstore(stack: &mut Vec<U256>, memory: &mut Vec<u8>) {
    let offset = stack.pop().unwrap().as_usize();
    let word = stack.pop().unwrap();

    let mut word_bytes = [0u8; 32];
    word.to_big_endian(&mut word_bytes);

    let current_size = memory.len();
    let new_size = offset + 32 - 1;
    if new_size >= current_size {
        expand_memory(memory, new_size);
    }

    word_bytes
        .into_iter()
        .enumerate()
        .for_each(move |(i, byte)| memory[offset + i] = byte);
}

pub fn memstore8(stack: &mut Vec<U256>, memory: &mut Vec<u8>) {
    let offset = stack.pop().unwrap().as_usize();
    let word = (stack.pop().unwrap() << 248) >> 248;
    let byte: u8 = word.try_into().unwrap();

    let current_size = memory.len();
    let new_size = offset - 1;
    if new_size >= current_size {
        expand_memory(memory, new_size);
    }

    memory[offset] = byte;
}

pub fn memload(stack: &mut Vec<U256>, memory: &mut Vec<u8>) {
    let offset = stack.pop().unwrap().as_usize();

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

    stack.push(value);
}

pub fn memsize(stack: &mut Vec<U256>, memory: &Vec<u8>) {
    stack.push(memory.len().into());
}
