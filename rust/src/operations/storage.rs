use std::collections::HashMap;

use primitive_types::U256;

pub fn storage_store(stack: &mut Vec<U256>, storage: &mut HashMap<U256, U256>) {
    let key = stack.pop().unwrap();
    let value = stack.pop().unwrap();

    storage.insert(key, value);
}


pub fn storage_load(stack: &mut Vec<U256>, storage: &mut HashMap<U256, U256>) {
    let key = stack.pop().unwrap();

    let value = storage.get(&key).unwrap_or(&U256::zero()).clone();

    stack.push(value);
}
