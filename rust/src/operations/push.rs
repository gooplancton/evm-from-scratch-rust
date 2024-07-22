use primitive_types::U256;

pub fn push(from_byte: usize, n_bytes: usize, bytes: &Vec<u8>, stack: &mut Vec<U256>) {
    let mut value = U256::zero();
    for i in from_byte..(from_byte + n_bytes) {
        value <<= 8;
        value += bytes[i].into();
    }

    stack.push(value);
}

pub fn duplicate(offset: usize, stack: &mut Vec<U256>) {
    let value_idx: usize = stack.len() - offset;
    let value = stack[value_idx].clone();

    stack.push(value);
}

pub fn swap(offset: usize, stack: &mut Vec<U256>) {
    let value_idx: usize = stack.len() - offset - 1;
    let value = stack[value_idx].clone();

    let top_value = stack.pop().unwrap();
    stack.push(value);
    stack[value_idx] = top_value;
}

