use aurora_engine_transactions::eip_1559::{SignedTransaction1559, Transaction1559};
use aurora_engine_transactions::eip_2930::{AccessTuple, SignedTransaction2930, Transaction2930};
use aurora_engine_transactions::legacy::{LegacyEthSignedTransaction, TransactionLegacy};
use aurora_engine_transactions::EthTransactionKind;
use aurora_engine_types::types::Address;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum JsEthTransactionKind {
    Legacy(JsLegacyEthSignedTransaction),
    Eip2930(JsSignedTransaction2930),
    Eip1559(JsSignedTransaction1559),
}

impl From<EthTransactionKind> for JsEthTransactionKind {
    fn from(rust_struct: EthTransactionKind) -> Self {
        match rust_struct {
            EthTransactionKind::Legacy(tx) => {
                JsEthTransactionKind::Legacy(JsLegacyEthSignedTransaction::from(tx))
            }
            EthTransactionKind::Eip2930(tx) => {
                JsEthTransactionKind::Eip2930(JsSignedTransaction2930::from(tx))
            }
            EthTransactionKind::Eip1559(tx) => {
                JsEthTransactionKind::Eip1559(JsSignedTransaction1559::from(tx))
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct JsLegacyEthSignedTransaction {
    pub transaction: JsTransactionLegacy,
    pub v: String,
    pub r: String,
    pub s: String,
}

impl From<LegacyEthSignedTransaction> for JsLegacyEthSignedTransaction {
    fn from(rust_struct: LegacyEthSignedTransaction) -> Self {
        let sender = rust_struct.sender().unwrap();

        JsLegacyEthSignedTransaction {
            transaction: JsTransactionLegacy::from((rust_struct.transaction, sender)),
            v: format!("0x{:x}", rust_struct.v),
            r: format!("0x{:x}", rust_struct.r),
            s: format!("0x{:x}", rust_struct.s),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct JsTransactionLegacy {
    pub nonce: String,
    pub gas_price: String,
    pub gas_limit: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub data: Vec<u8>,
}

impl From<(TransactionLegacy, Address)> for JsTransactionLegacy {
    fn from((rust_struct, from): (TransactionLegacy, Address)) -> Self {
        JsTransactionLegacy {
            nonce: rust_struct.nonce.to_string(),
            gas_price: rust_struct.gas_price.to_string(),
            gas_limit: rust_struct.gas_limit.to_string(),
            from: to_zero_prefixed_address(&Some(from)),
            to: to_zero_prefixed_address(&rust_struct.to),
            value: rust_struct.value.to_string(),
            data: rust_struct.data,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct JsSignedTransaction2930 {
    pub transaction: JsTransaction2930,
    pub parity: u8,
    pub r: String,
    pub s: String,
}

impl From<SignedTransaction2930> for JsSignedTransaction2930 {
    fn from(rust_struct: SignedTransaction2930) -> Self {
        let sender = rust_struct.sender().unwrap();

        JsSignedTransaction2930 {
            transaction: JsTransaction2930::from((rust_struct.transaction, sender)),
            parity: rust_struct.parity,
            r: rust_struct.r.to_string(),
            s: rust_struct.s.to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct JsTransaction2930 {
    pub chain_id: String,
    pub nonce: String,
    pub gas_price: String,
    pub gas_limit: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub data: Vec<u8>,
    pub access_list: Vec<JsAccessTuple>,
}

impl From<(Transaction2930, Address)> for JsTransaction2930 {
    fn from((rust_struct, from): (Transaction2930, Address)) -> Self {
        JsTransaction2930 {
            chain_id: rust_struct.chain_id.to_string(),
            nonce: rust_struct.nonce.to_string(),
            gas_price: rust_struct.gas_price.to_string(),
            gas_limit: rust_struct.gas_limit.to_string(),
            from: to_zero_prefixed_address(&Some(from)),
            to: to_zero_prefixed_address(&rust_struct.to),
            value: rust_struct.value.to_string(),
            data: rust_struct.data,
            access_list: rust_struct
                .access_list
                .into_iter()
                .map(JsAccessTuple::from)
                .collect(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct JsAccessTuple {
    address: String,
    storage_keys: Vec<String>,
}

impl From<AccessTuple> for JsAccessTuple {
    fn from(rust_struct: AccessTuple) -> Self {
        JsAccessTuple {
            address: rust_struct.address.to_string(),
            storage_keys: rust_struct
                .storage_keys
                .iter()
                .map(|key| key.to_string())
                .collect(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct JsSignedTransaction1559 {
    pub transaction: JsTransaction1559,
    pub parity: u8,
    pub r: String,
    pub s: String,
}

impl From<SignedTransaction1559> for JsSignedTransaction1559 {
    fn from(rust_struct: SignedTransaction1559) -> Self {
        let sender = rust_struct.sender().unwrap();

        JsSignedTransaction1559 {
            transaction: JsTransaction1559::from((rust_struct.transaction, sender)),
            parity: rust_struct.parity,
            r: rust_struct.r.to_string(),
            s: rust_struct.s.to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct JsTransaction1559 {
    pub chain_id: String,
    pub nonce: String,
    pub max_priority_fee_per_gas: String,
    pub max_fee_per_gas: String,
    pub gas_limit: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub data: Vec<u8>,
    pub access_list: Vec<JsAccessTuple>,
}

impl From<(Transaction1559, Address)> for JsTransaction1559 {
    fn from((rust_struct, from): (Transaction1559, Address)) -> Self {
        JsTransaction1559 {
            chain_id: rust_struct.chain_id.to_string(),
            nonce: rust_struct.nonce.to_string(),
            max_priority_fee_per_gas: rust_struct.max_priority_fee_per_gas.to_string(),
            max_fee_per_gas: rust_struct.max_fee_per_gas.to_string(),
            gas_limit: rust_struct.gas_limit.to_string(),
            from: to_zero_prefixed_address(&Some(from)),
            to: to_zero_prefixed_address(&rust_struct.to),
            value: rust_struct.value.to_string(),
            data: rust_struct.data,
            access_list: rust_struct
                .access_list
                .into_iter()
                .map(JsAccessTuple::from)
                .collect(),
        }
    }
}

fn to_zero_prefixed_address(address: &Option<Address>) -> String {
    address
        .map(|address| address.encode())
        .or_else(|| Some("0000000000000000000000000000000000000000".to_string()))
        .map(|address| format!("0x{}", address))
        .unwrap()
}
