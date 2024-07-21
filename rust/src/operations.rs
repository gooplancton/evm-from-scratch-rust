use primitive_types::U256;
use arrow_buffer::i256;
use crate::helpers::*;

pub fn push(from_byte: usize, n_bytes: usize, bytes: &Vec<u8>, stack: &mut Vec<U256>) {
    let mut value = U256::zero();
    for i in from_byte..(from_byte + n_bytes) {
        value <<= 8;
        value += bytes[i].into();
    }

    stack.push(value);
}

pub fn add(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let (res, _) = n1.overflowing_add(n2);

    stack.push(res);
}

pub fn mul(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let (res, _) = n1.overflowing_mul(n2);

    stack.push(res);
}

pub fn sub(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let (res, _) = n1.overflowing_sub(n2);

    stack.push(res);
}

pub fn div(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let res = if n2 == 0.into() { n2 } else { n1 / n2 };

    stack.push(res);
}

pub fn modulo(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let res = if n2 == 0.into() { n2 } else { n1 % n2 };

    stack.push(res);
}

pub fn addmod(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let n3 = stack.pop().unwrap();
    let (res, _) = (n1 % n3).overflowing_add(n2 % n3);

    stack.push(res);
}

pub fn mulmod(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let n3 = stack.pop().unwrap();
    let (res, _) = (n1 % n3).overflowing_mul(n2 % n3);

    stack.push(res);
}

pub fn exp(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let (res, _) = n1.overflowing_pow(n2);

    stack.push(res);
}

pub fn signextend(stack: &mut Vec<U256>) {
    let b = stack.pop().unwrap().as_u32();
    let x = stack.pop().unwrap();

    let x_size_bits = 8 * (b + 1);
    let x = (x << x_size_bits) >> x_size_bits;

    let mut res = x;
    if x >> (x_size_bits - 1) == 1.into() {
        res += U256::MAX << x_size_bits;
    }
    
    stack.push(res);
}

pub fn signed_div(stack: &mut Vec<U256>) {
    let n1: i256 = stack.pop().unwrap().as_i256();
    let n2 = stack.pop().unwrap().as_i256();
    let res = if n2 == 0.into() { n2 } else { n1 / n2 };

    stack.push(res.as_u256());
}

