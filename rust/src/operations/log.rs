use primitive_types::U256;

use crate::{
    operations::read_memory_bytes,
    state::{BlockchainState, SerializedU256},
    EvmLog,
};

pub trait IntoHexString {
    fn into_hex_string(self) -> String;
}

impl IntoHexString for U256 {
    fn into_hex_string(self) -> String {
        let mut bytes = [0u8; 32];
        self.to_big_endian(&mut bytes);
        let encoded = hex::encode(bytes);

        "0x".to_owned() + encoded.trim_start_matches("0")
    }
}

impl IntoHexString for SerializedU256 {
    fn into_hex_string(self) -> String {
        let deserialized: U256 = self.into();
        deserialized.into_hex_string()
    }
}

pub fn log(
    n_topics: u8,
    stack: &mut Vec<U256>,
    memory: &mut Vec<u8>,
    state: &BlockchainState,
) -> EvmLog {
    let offset = stack.pop().unwrap().as_usize();
    let size = stack.pop().unwrap().as_usize();
    let topics = (0..n_topics)
        .map(|_| stack.pop().unwrap())
        .map(U256::into_hex_string)
        .collect::<Vec<_>>();

    let data_bytes = read_memory_bytes(memory, offset, size);
    let data = hex::encode(data_bytes);

    let tx_to = state.tx.to.unwrap();
    let address = tx_to.into_hex_string();
    dbg!(&address);

    EvmLog {
        address,
        topics,
        data,
    }
}
