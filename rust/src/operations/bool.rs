use primitive_types::U256;

use crate::helpers::AsI256;

pub fn not(stack: &mut Vec<U256>) {
    let n = stack.pop().unwrap();

    stack.push(!n);
}

pub fn and(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();

    stack.push(n1 & n2);
}

pub fn or(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();

    stack.push(n1 | n2);
}

pub fn xor(stack: &mut Vec<U256>) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();

    stack.push(n1 ^ n2);
}

pub fn byte(stack: &mut Vec<U256>) {
    let index = stack.pop().unwrap().as_usize();
    let n = stack.pop().unwrap().as_i256();
    let mut res = U256::zero();
    if index <= 32 {
        res = n.to_be_bytes()[index].into();
    }

    stack.push(res);
}

pub fn shl(stack: &mut Vec<U256>) {
    let shift = stack.pop().unwrap().as_usize();
    let n = stack.pop().unwrap();
    let res = if shift > 255 {
        U256::zero()
    } else {
        n << shift
    };

    stack.push(res);
}

pub fn shr(stack: &mut Vec<U256>) {
    let shift = stack.pop().unwrap().as_usize();
    let n = stack.pop().unwrap();
    let res = if shift > 255 {
        U256::zero()
    } else {
        n >> shift
    };

    stack.push(res);
}

pub fn sar(stack: &mut Vec<U256>) {
    let shift = stack.pop().unwrap().as_usize();
    let n = stack.pop().unwrap();
    let mut res = n >> shift;
    if n.as_i256().is_negative() {
        if shift > 255 {
            res = U256::MAX;
        } else {
            res += U256::MAX << (256 - shift);
        }
    }

    stack.push(res);
}
