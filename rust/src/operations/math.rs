use arrow_buffer::i256;
use primitive_types::U256;
use crate::helpers::*;

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

    let mut res = (x << x_size_bits) >> x_size_bits;
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

pub fn signed_modulo(stack: &mut Vec<U256>) {
    let n1: i256 = stack.pop().unwrap().as_i256();
    let n2 = stack.pop().unwrap().as_i256();
    let res = if n2 == 0.into() { n2 } else { n1 % n2 };

    stack.push(res.as_u256());
}

pub fn less_than(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let res = if n1 < n2 { U256::one() } else { U256::zero() };

    stack.push(res);
}

pub fn greater_than(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let res = if n1 > n2 { U256::one() } else { U256::zero() };

    stack.push(res);
}

pub fn signed_less_than(stack: &mut Vec<U256>) {
    let n1: i256 = stack.pop().unwrap().as_i256();
    let n2 = stack.pop().unwrap().as_i256();
    let res = if n1 < n2 { U256::one() } else { U256::zero() };

    stack.push(res);
}

pub fn signed_greater_than(stack: &mut Vec<U256>) {
    let n1: i256 = stack.pop().unwrap().as_i256();
    let n2 = stack.pop().unwrap().as_i256();

    let res = if n1 > n2 { U256::one() } else { U256::zero() };

    stack.push(res);
}

pub fn equal(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    let res = if n1 == n2 { U256::one() } else { U256::zero() };

    stack.push(res);
}

pub fn is_zero(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let res = if n1 == U256::zero() { U256::one() } else { U256::zero() };

    stack.push(res);
}

