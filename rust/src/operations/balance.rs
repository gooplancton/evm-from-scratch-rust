use primitive_types::U256;

use crate::state::{BlockchainState, SerializedU256};

pub fn get_balance(stack: &mut Vec<U256>, state: &BlockchainState) {
    let address: SerializedU256 = stack.pop().unwrap().into();
    let entry = state
        .contracts_state
        .get(&address)
        .map(|entry| entry.balance)
        .flatten()
        .unwrap_or_default();

    stack.push(entry.into());
}

pub fn self_balance(stack: &mut Vec<U256>, state: &BlockchainState) {
    let address: SerializedU256 = state.tx.to.unwrap();
    let entry = state
        .contracts_state
        .get(&address)
        .map(|entry| entry.balance)
        .flatten()
        .unwrap_or_default();

    stack.push(entry.into());
}
