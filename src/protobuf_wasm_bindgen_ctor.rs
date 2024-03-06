use crate::codegen::compact_formats::{
    CompactOrchardAction, CompactSaplingOutput, CompactSaplingSpend, CompactTx,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl CompactTx {
    #[wasm_bindgen(constructor)]
    pub fn create(
        index: u64,
        hash: Vec<u8>,
        fee: u32,
        spends: Vec<CompactSaplingSpend>,
        outputs: Vec<CompactSaplingOutput>,
        actions: Vec<CompactOrchardAction>,
    ) -> Self {
        Self {
            special_fields: Default::default(),
            index,
            hash,
            fee,
            spends,
            outputs,
            actions,
        }
    }
}
#[wasm_bindgen]
impl CompactSaplingSpend {
    #[wasm_bindgen(constructor)]
    pub fn create(nf: Vec<u8>) -> Self {
        Self {
            nf,
            special_fields: Default::default(),
        }
    }
}

#[wasm_bindgen]
impl CompactSaplingOutput {
    #[wasm_bindgen(constructor)]
    pub fn create(cmu: Vec<u8>, ephemeral_key: Vec<u8>, ciphertext: Vec<u8>) -> Self {
        Self {
            special_fields: Default::default(),
            cmu,
            ephemeralKey: ephemeral_key,
            ciphertext,
        }
    }
}

#[wasm_bindgen]
impl CompactOrchardAction {
    #[wasm_bindgen(constructor)]
    pub fn create(
        nullifier: Vec<u8>,
        cmx: Vec<u8>,
        ephemeral_key: Vec<u8>,
        ciphertext: Vec<u8>,
    ) -> Self {
        Self {
            nullifier,
            cmx,
            ephemeralKey: ephemeral_key,
            ciphertext,
            special_fields: Default::default(),
        }
    }
}
