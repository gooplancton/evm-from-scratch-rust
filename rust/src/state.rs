use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TxData {
    pub to: Option<String>,
    pub from: Option<String>,
    pub origin: Option<String>,
    pub gasprice: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlockData {
    pub basefee: Option<String>,
    pub coinbase: Option<String>,
    pub timestamp: Option<String>,
    pub number: Option<String>,
    pub difficulty: Option<String>,
    pub gaslimit: Option<String>,
    pub chainid: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BlockchainState {
    pub tx: Option<TxData>,
    pub block: Option<BlockData>
}

