mod structs;

use crate::structs::JsEthTransactionKind;
use aurora_engine_transactions::backwards_compatibility::EthTransactionKindAdapter;
use wasm_bindgen::prelude::*;

// https://github.com/aurora-is-near/aurora-engine/blob/9cb8359b2b49fc53bb5fa11509e321b522fb612d/engine/src/engine.rs#L58
pub const ZERO_ADDRESS_FIX_HEIGHT: u64 = 61_200_152;

#[wasm_bindgen(js_name = parseTransaction)]
pub fn parse_transaction(
    bytes: &[u8],
    block_height: Option<u64>,
) -> Result<JsEthTransactionKind, JsError> {
    let adapter = EthTransactionKindAdapter::new(ZERO_ADDRESS_FIX_HEIGHT);

    match adapter.try_parse_bytes(&bytes, block_height.unwrap_or(ZERO_ADDRESS_FIX_HEIGHT + 1)) {
        Ok(tx) => Ok(tx.into()),
        Err(e) => Err(JsError::new(&format!("cannot parse: {:?}", e))),
    }
}
