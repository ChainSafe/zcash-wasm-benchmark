use core::convert::TryFrom;
use orchard::note::{ExtractedNoteCommitment, Nullifier};
use std::convert::TryInto;
use zcash_note_encryption::{EphemeralKeyBytes, COMPACT_NOTE_SIZE};

pub(crate) struct CompactSaplingOutput {
    pub(crate) cmu: Box<[u8]>,
    pub(crate) ephemeral_key: Box<[u8]>,
    pub(crate) enc_ciphertext: Box<[u8]>,
}

pub(crate) struct CompactOrchardAction {
    pub(crate) nullifier: Box<[u8]>,
    pub(crate) cmx: Box<[u8]>,
    pub(crate) ephemeral_key: Box<[u8]>,
    pub(crate) enc_ciphertext: Box<[u8]>,
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

impl TryFrom<crate::proto::compact_formats::CompactOrchardAction>
    for orchard::note_encryption::CompactAction
{
    type Error = anyhow::Error;

    fn try_from(
        action: crate::proto::compact_formats::CompactOrchardAction,
    ) -> anyhow::Result<Self> {
        let nullifier = Nullifier::from_bytes(action.nullifier.as_slice().try_into()?).unwrap();
        let cmx = ExtractedNoteCommitment::from_bytes(action.cmx.as_slice().try_into()?).unwrap();
        let ephemeral_key_bytes: [u8; 32] = action.ephemeral_key.as_slice().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(ephemeral_key_bytes);
        let enc_ciphertext_bytes: [u8; COMPACT_NOTE_SIZE] =
            action.ciphertext.as_slice().try_into()?;

        Ok(orchard::note_encryption::CompactAction::from_parts(
            nullifier,
            cmx,
            ephemeral_key,
            enc_ciphertext_bytes,
        ))
    }
}

impl TryFrom<&crate::proto::compact_formats::CompactOrchardAction>
    for orchard::note_encryption::CompactAction
{
    type Error = anyhow::Error;

    fn try_from(
        action: &crate::proto::compact_formats::CompactOrchardAction,
    ) -> anyhow::Result<Self> {
        let nullifier = Nullifier::from_bytes(action.nullifier.as_slice().try_into()?).unwrap();
        let cmx = ExtractedNoteCommitment::from_bytes(action.cmx.as_slice().try_into()?).unwrap();
        let ephemeral_key_bytes: [u8; 32] = action.ephemeral_key.as_slice().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(ephemeral_key_bytes);
        let enc_ciphertext_bytes: [u8; COMPACT_NOTE_SIZE] =
            action.ciphertext.as_slice().try_into()?;

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

impl TryFrom<crate::proto::compact_formats::CompactSaplingOutput>
    for sapling::note_encryption::CompactOutputDescription
{
    type Error = anyhow::Error;

    fn try_from(
        output: crate::proto::compact_formats::CompactSaplingOutput,
    ) -> Result<Self, Self::Error> {
        let ephemeral_key_bytes: [u8; 32] = output.ephemeral_key.as_slice().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(ephemeral_key_bytes);
        let enc_ciphertext: [u8; COMPACT_NOTE_SIZE] = output.ciphertext.as_slice().try_into()?;
        let cmu =
            sapling::note::ExtractedNoteCommitment::from_bytes(output.cmu.as_slice().try_into()?)
                .unwrap();

        Ok(sapling::note_encryption::CompactOutputDescription {
            ephemeral_key,
            cmu,
            enc_ciphertext,
        })
    }
}

impl TryFrom<&crate::proto::compact_formats::CompactSaplingOutput>
    for sapling::note_encryption::CompactOutputDescription
{
    type Error = anyhow::Error;

    fn try_from(
        output: &crate::proto::compact_formats::CompactSaplingOutput,
    ) -> Result<Self, Self::Error> {
        let ephemeral_key_bytes: [u8; 32] = output.ephemeral_key.as_slice().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(ephemeral_key_bytes);
        let enc_ciphertext: [u8; COMPACT_NOTE_SIZE] = output.ciphertext.as_slice().try_into()?;
        let cmu =
            sapling::note::ExtractedNoteCommitment::from_bytes(output.cmu.as_slice().try_into()?)
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
