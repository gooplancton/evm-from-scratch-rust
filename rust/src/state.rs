use std::collections::HashMap;

use primitive_types::U256;
use serde::{de::Visitor, Deserialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SerializedU256 {
    value: U256,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SerializedBytes {
    value: Vec<u8>,
}

impl SerializedBytes {
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl Into<Vec<u8>> for SerializedBytes {
    fn into(self) -> Vec<u8> {
        self.value
    }
}

impl From<Vec<u8>> for SerializedBytes {
    fn from(value: Vec<u8>) -> Self {
        SerializedBytes { value }
    }
}

impl Into<U256> for SerializedU256 {
    fn into(self) -> U256 {
        self.value
    }
}

impl From<U256> for SerializedU256 {
    fn from(value: U256) -> Self {
        SerializedU256 { value }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TxData {
    pub to: Option<SerializedU256>,
    pub from: Option<SerializedU256>,
    pub origin: Option<SerializedU256>,
    pub gasprice: Option<SerializedU256>,
    pub value: Option<SerializedU256>,
    pub data: Option<SerializedBytes>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BlockData {
    pub basefee: Option<SerializedU256>,
    pub coinbase: Option<SerializedU256>,
    pub timestamp: Option<SerializedU256>,
    pub number: Option<SerializedU256>,
    pub difficulty: Option<SerializedU256>,
    pub gaslimit: Option<SerializedU256>,
    pub chainid: Option<SerializedU256>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContractsStateDataEntry {
    pub balance: Option<SerializedU256>,
    pub code: Option<ContractsStateDataEntryCode>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContractsStateDataEntryCode {
    pub bin: Option<SerializedBytes>,
}

impl ContractsStateDataEntryCode {
    pub fn len(&self) -> usize {
        self.bin.as_ref().map(|bin| bin.len()).unwrap_or_default()
    }
}

pub type ContractsStateData = HashMap<SerializedU256, ContractsStateDataEntry>;

#[derive(Clone, Debug)]
pub struct BlockchainState {
    pub tx: TxData,
    pub block: BlockData,
    pub contracts_state: ContractsStateData,
}

struct SerializedU256Visitor;
impl Visitor<'_> for SerializedU256Visitor {
    type Value = SerializedU256;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string representation of an u256 integer")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let value = v
            .parse::<U256>()
            .expect("Invalid u256 string representation");

        Ok(SerializedU256 { value })
    }
}

impl<'de> Deserialize<'de> for SerializedU256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(SerializedU256Visitor)
    }
}

struct SerializedBytesVisitor;
impl Visitor<'_> for SerializedBytesVisitor {
    type Value = SerializedBytes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string representation of a bytes vector")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let value = hex::decode(v).unwrap_or_default();

        Ok(SerializedBytes { value })
    }
}

impl<'de> Deserialize<'de> for SerializedBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(SerializedBytesVisitor)
    }
}
