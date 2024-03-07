use core::convert::TryFrom;
use orchard::note::{ExtractedNoteCommitment, Nullifier};
use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use zcash_note_encryption::{EphemeralKeyBytes, COMPACT_NOTE_SIZE};

#[wasm_bindgen]
#[allow(dead_code)]
pub struct CompactTx {
    index: u64,
    #[wasm_bindgen(getter_with_clone)]
    pub hash: Box<[u8]>,
    fee: u32,
    spends: Box<[CompactSaplingSpend]>,
    #[wasm_bindgen(getter_with_clone)]
    pub outputs: Box<[CompactSaplingOutput]>,
    #[wasm_bindgen(getter_with_clone)]
    pub actions: Box<[CompactOrchardAction]>,
}

#[wasm_bindgen]
impl CompactTx {
    #[wasm_bindgen(constructor)]
    pub fn new(
        index: u64,
        hash: &[u8],
        fee: u32,
        spends: Box<[CompactSaplingSpend]>,
        outputs: Box<[CompactSaplingOutput]>,
        actions: Box<[CompactOrchardAction]>,
    ) -> Self {
        assert!(hash.len() == 32);
        Self {
            index,
            hash: Box::from(hash),
            fee,
            spends,
            outputs,
            actions,
        }
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct CompactSaplingSpend {
    nf: Box<[u8]>,
}

#[wasm_bindgen]
impl CompactSaplingSpend {
    #[wasm_bindgen(constructor)]
    pub fn new(nf: &[u8]) -> Self {
        assert!(nf.len() == 32);

        Self { nf: Box::from(nf) }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CompactSaplingOutput {
    #[wasm_bindgen(getter_with_clone)]
    pub cmu: Box<[u8]>,
    #[wasm_bindgen(getter_with_clone)]
    pub ephemeral_key: Box<[u8]>,
    #[wasm_bindgen(getter_with_clone)]
    pub enc_ciphertext: Box<[u8]>,
}

#[wasm_bindgen]
impl CompactSaplingOutput {
    #[wasm_bindgen(constructor)]
    pub fn new(cmu: &[u8], ephemeral_key: &[u8], enc_ciphertext: &[u8]) -> Self {
        assert!(cmu.len() == 32);
        assert!(ephemeral_key.len() == 32);
        assert!(enc_ciphertext.len() == COMPACT_NOTE_SIZE);
        Self {
            cmu: Box::from(cmu),
            ephemeral_key: Box::from(ephemeral_key),
            enc_ciphertext: Box::from(enc_ciphertext),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CompactOrchardAction {
    #[wasm_bindgen(getter_with_clone)]
    pub nullifier: Box<[u8]>,
    #[wasm_bindgen(getter_with_clone)]
    pub cmx: Box<[u8]>,
    #[wasm_bindgen(getter_with_clone)]
    pub ephemeral_key: Box<[u8]>,
    #[wasm_bindgen(getter_with_clone)]
    pub enc_ciphertext: Box<[u8]>,
}

#[wasm_bindgen]
impl CompactOrchardAction {
    #[wasm_bindgen(constructor)]
    pub fn new(nullifier: &[u8], cmx: &[u8], ephemeral_key: &[u8], enc_ciphertext: &[u8]) -> Self {
        assert!(nullifier.len() == 32);
        assert!(cmx.len() == 32);
        assert!(ephemeral_key.len() == 32);
        assert!(enc_ciphertext.len() == COMPACT_NOTE_SIZE);

        Self {
            nullifier: Box::from(nullifier),
            cmx: Box::from(cmx),
            ephemeral_key: Box::from(ephemeral_key),
            enc_ciphertext: Box::from(enc_ciphertext),
        }
    }
}

// Conversions
impl TryFrom<CompactOrchardAction> for orchard::note_encryption::CompactAction {
    type Error = anyhow::Error;

    fn try_from(action: CompactOrchardAction) -> anyhow::Result<Self> {
        let nullifier = Nullifier::from_bytes(action.nullifier.as_ref().try_into()?).unwrap();
        let cmx = ExtractedNoteCommitment::from_bytes(action.cmx.as_ref().try_into()?).unwrap();
        let ephemeral_key_bytes: [u8; 32] = action.ephemeral_key.as_ref().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(ephemeral_key_bytes);
        let enc_ciphertext_bytes: [u8; COMPACT_NOTE_SIZE] =
            action.enc_ciphertext.as_ref().try_into()?;

        Ok(orchard::note_encryption::CompactAction::from_parts(
            nullifier,
            cmx,
            ephemeral_key,
            enc_ciphertext_bytes,
        ))
    }
}

impl TryFrom<&CompactOrchardAction> for orchard::note_encryption::CompactAction {
    type Error = anyhow::Error;

    fn try_from(action: &CompactOrchardAction) -> anyhow::Result<Self> {
        let nullifier = Nullifier::from_bytes(action.nullifier.as_ref().try_into()?).unwrap();
        let cmx = ExtractedNoteCommitment::from_bytes(action.cmx.as_ref().try_into()?).unwrap();
        let ephemeral_key_bytes: [u8; 32] = action.ephemeral_key.as_ref().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(ephemeral_key_bytes);
        let enc_ciphertext_bytes: [u8; COMPACT_NOTE_SIZE] =
            action.enc_ciphertext.as_ref().try_into()?;

        Ok(orchard::note_encryption::CompactAction::from_parts(
            nullifier,
            cmx,
            ephemeral_key,
            enc_ciphertext_bytes,
        ))
    }
}
impl TryFrom<CompactSaplingOutput> for sapling::note_encryption::CompactOutputDescription {
    type Error = anyhow::Error;

    fn try_from(output: CompactSaplingOutput) -> Result<Self, Self::Error> {
        let ephemeral_key_bytes: [u8; 32] = output.ephemeral_key.as_ref().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(ephemeral_key_bytes);
        let enc_ciphertext: [u8; COMPACT_NOTE_SIZE] = output.enc_ciphertext.as_ref().try_into()?;
        let cmu =
            sapling::note::ExtractedNoteCommitment::from_bytes(output.cmu.as_ref().try_into()?)
                .unwrap();

        Ok(sapling::note_encryption::CompactOutputDescription {
            ephemeral_key,
            cmu,
            enc_ciphertext,
        })
    }
}

impl TryFrom<&CompactSaplingOutput> for sapling::note_encryption::CompactOutputDescription {
    type Error = anyhow::Error;

    fn try_from(output: &CompactSaplingOutput) -> Result<Self, Self::Error> {
        let ephemeral_key_bytes: [u8; 32] = output.ephemeral_key.as_ref().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(ephemeral_key_bytes);
        let enc_ciphertext: [u8; COMPACT_NOTE_SIZE] = output.enc_ciphertext.as_ref().try_into()?;
        let cmu =
            sapling::note::ExtractedNoteCommitment::from_bytes(output.cmu.as_ref().try_into()?)
                .unwrap();

        Ok(sapling::note_encryption::CompactOutputDescription {
            ephemeral_key,
            cmu,
            enc_ciphertext,
        })
    }
}
