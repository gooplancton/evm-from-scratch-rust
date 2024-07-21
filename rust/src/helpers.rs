use arrow_buffer::i256;
use primitive_types::U256;

pub trait AsI256 {
    fn as_i256(&self) -> i256;
}

impl AsI256 for U256 {
    fn as_i256(&self) -> i256 {
        let mut bytes = [0u8; 32];
        self.to_big_endian(&mut bytes);

        i256::from_be_bytes(bytes)
    }
}

pub trait AsU256 {
    fn as_u256(&self) -> U256;
}

impl AsU256 for i256 {
    fn as_u256(&self) -> U256 {
        self.to_be_bytes().into()
    }
}

