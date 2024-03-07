use core::convert::TryFrom;
use orchard::note::{ExtractedNoteCommitment, Nullifier};
use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use zcash_note_encryption::{EphemeralKeyBytes, COMPACT_NOTE_SIZE};

#[wasm_bindgen]
#[allow(dead_code)]
pub struct CompactTx {
    index: u64,
    hash: Box<[u8]>,
    fee: u32,
    spends: Box<[CompactSaplingSpend]>,
    outputs: Box<[CompactSaplingOutput]>,
    actions: Box<[CompactOrchardAction]>,
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
        let mut hash_buf = [0; 32];
        hash_buf.copy_from_slice(hash);
        Self {
            index,
            hash: Box::from(hash_buf),
            fee,
            spends,
            outputs,
            actions,
        }
    }
}

impl CompactTx {
    pub fn actions(&self) -> &[CompactOrchardAction] {
        &self.actions
    }
    pub fn outputs(&self) -> &[CompactSaplingOutput] {
        &self.outputs
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
pub struct CompactSaplingOutput {
    // pub ephemeral_key: EphemeralKeyBytes,
    // pub cmu: ExtractedNoteCommitment,
    // pub enc_ciphertext: [u8; COMPACT_NOTE_SIZE]
    buf: Box<[u8]>,
}

#[wasm_bindgen]
impl CompactSaplingOutput {
    #[wasm_bindgen(constructor)]
    pub fn new(cmu: &[u8], ephemeral_key: &[u8], enc_ciphertext: &[u8]) -> Self {
        assert!(cmu.len() == 32);
        assert!(ephemeral_key.len() == 32);
        assert!(enc_ciphertext.len() == COMPACT_NOTE_SIZE);
        let mut buf = [0; 32 + 32 + COMPACT_NOTE_SIZE];
        buf[..32].copy_from_slice(cmu);
        buf[32..64].copy_from_slice(ephemeral_key);
        buf[64..].copy_from_slice(enc_ciphertext);
        Self {
            buf: Box::from(buf),
        }
    }
}

impl CompactSaplingOutput {
    pub fn get_cmu(&self) -> &[u8; 32] {
        self.buf[..32].try_into().unwrap()
    }
    pub fn get_ephemeral_key(&self) -> &[u8; 32] {
        self.buf[32..64].try_into().unwrap()
    }
    pub fn get_enc_ciphertext(&self) -> &[u8; COMPACT_NOTE_SIZE] {
        self.buf[64..].try_into().unwrap()
    }
}

#[wasm_bindgen]
pub struct CompactOrchardAction {
    // nullifier: Nullifier,
    // cmx: ExtractedNoteCommitment,
    // ephemeral_key: EphemeralKeyBytes,
    // enc_ciphertext: [u8; 52],
    buf: Box<[u8]>,
}

#[wasm_bindgen]
impl CompactOrchardAction {
    #[wasm_bindgen(constructor)]
    pub fn new(nullifier: &[u8], cmx: &[u8], ephemeral_key: &[u8], enc_ciphertext: &[u8]) -> Self {
        assert!(nullifier.len() == 32);
        assert!(cmx.len() == 32);
        assert!(ephemeral_key.len() == 32);
        assert!(enc_ciphertext.len() == COMPACT_NOTE_SIZE);
        let mut buf = [0; 32 + 32 + 32 + COMPACT_NOTE_SIZE];
        buf[..32].copy_from_slice(nullifier);
        buf[32..64].copy_from_slice(cmx);
        buf[64..96].copy_from_slice(ephemeral_key);
        buf[96..].copy_from_slice(enc_ciphertext);
        Self {
            buf: Box::from(buf),
        }
    }
}

impl CompactOrchardAction {
    pub fn get_nullifier(&self) -> &[u8; 32] {
        self.buf[..32].try_into().unwrap()
    }
    pub fn get_cmx(&self) -> &[u8; 32] {
        self.buf[32..64].try_into().unwrap()
    }
    pub fn get_ephemeral_key(&self) -> &[u8; 32] {
        self.buf[64..96].try_into().unwrap()
    }
    pub fn get_enc_ciphertext(&self) -> &[u8; COMPACT_NOTE_SIZE] {
        self.buf[96..].try_into().unwrap()
    }
}

// Conversions
impl TryFrom<CompactOrchardAction> for orchard::note_encryption::CompactAction {
    type Error = anyhow::Error;

    fn try_from(action: CompactOrchardAction) -> anyhow::Result<Self> {
        let nullifier = Nullifier::from_bytes(action.get_nullifier()).unwrap();
        let cmx = ExtractedNoteCommitment::from_bytes(action.get_cmx()).unwrap();
        let ephemeral_key = EphemeralKeyBytes::from(*action.get_ephemeral_key());
        let enc_ciphertext_bytes: [u8; 52] = *action.get_enc_ciphertext();

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
        let nullifier = Nullifier::from_bytes(action.get_nullifier()).unwrap();
        let cmx = ExtractedNoteCommitment::from_bytes(action.get_cmx()).unwrap();
        let ephemeral_key = EphemeralKeyBytes::from(*action.get_ephemeral_key());
        let enc_ciphertext_bytes: [u8; 52] = *action.get_enc_ciphertext();

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
        let ephemeral_key = EphemeralKeyBytes::from(*output.get_ephemeral_key());
        let enc_ciphertext: [u8; COMPACT_NOTE_SIZE] = *output.get_enc_ciphertext();
        let cmu = sapling::note::ExtractedNoteCommitment::from_bytes(output.get_cmu()).unwrap();

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
        let ephemeral_key = EphemeralKeyBytes::from(*output.get_ephemeral_key());
        let enc_ciphertext: [u8; COMPACT_NOTE_SIZE] = *output.get_enc_ciphertext();
        let cmu = sapling::note::ExtractedNoteCommitment::from_bytes(output.get_cmu()).unwrap();

        Ok(sapling::note_encryption::CompactOutputDescription {
            ephemeral_key,
            cmu,
            enc_ciphertext,
        })
    }
}

// #[wasm_bindgen]
// pub fn s(x: Vec<CompactOrchardAction>) -> u8 {
//     11
// }
// #[wasm_bindgen]
// pub struct WasmUint8Array(Vec<u8>);

// #[wasm_bindgen]
// impl WasmUint8Array {
//     #[wasm_bindgen(constructor)]
//     pub fn new(size: usize) -> Self {
//         let buffer = vec![0; size];
//         Self { 0: buffer }
//     }

//     #[wasm_bindgen(getter)]
//     pub fn view(&mut self) -> js_sys::Uint8Array {
//         unsafe { js_sys::Uint8Array::view_mut_raw(self.0.as_mut_ptr(), self.0.len()) }
//     }
// }
