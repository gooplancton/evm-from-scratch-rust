use std::collections::HashMap;

use primitive_types::U256;

use crate::{
    evm,
    state::{
        BlockchainState, ContractsStateDataEntry, ContractsStateDataEntryCode, SerializedBytes,
        SerializedU256,
    },
};

use super::read_memory_bytes;

pub fn create_contract(
    stack: &mut Vec<U256>,
    memory: &mut Vec<u8>,
    storage: &mut HashMap<U256, U256>,
    chain_state: &mut BlockchainState,
) {
    let balance = stack.pop().unwrap();
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();

    let code = if size > 0 {
        read_memory_bytes(memory, offset, size)
    } else {
        vec![]
    };
    let address = U256::from(1234); // TODO:

    let res = evm(&code, memory, storage, chain_state, false);
    let contract_code = res.ret.unwrap_or_default();
    let has_reverted = !res.success;

    if !has_reverted {
        chain_state.contracts_state.insert(
            SerializedU256 { value: address },
            ContractsStateDataEntry {
                code: Some(ContractsStateDataEntryCode {
                    bin: Some(SerializedBytes {
                        value: contract_code,
                    }),
                }),
                balance: Some(SerializedU256 { value: balance }),
            },
        );
    }

    stack.push(if has_reverted { U256::zero() } else { address });
}

pub fn self_destruct(stack: &mut Vec<U256>, chain_state: &mut BlockchainState) {
    let address = stack.pop().unwrap();
    let current_address = chain_state.tx.to.unwrap();

    let balance = chain_state
        .contracts_state
        .remove(&current_address)
        .unwrap()
        .balance
        .unwrap_or_default();

    let destination_contract_key = SerializedU256 { value: address };
    let mut destination_contract = chain_state
        .contracts_state
        .remove(&destination_contract_key)
        .unwrap_or_default();

    destination_contract.balance = {
        let b1: U256 = destination_contract.balance.unwrap_or_default().into();
        let b2: U256 = balance.into();

        Some(SerializedU256 { value: b1 + b2 })
    };

    chain_state
        .contracts_state
        .insert(destination_contract_key, destination_contract);
}
