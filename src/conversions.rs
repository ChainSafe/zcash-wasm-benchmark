// Conversions between our generated protobuf types and librustzcash types

use crate::codegen::compact_formats as pb;
use orchard::note::{ExtractedNoteCommitment, Nullifier};
use std::convert::TryInto;
use zcash_note_encryption::EphemeralKeyBytes;

impl std::convert::TryFrom<pb::CompactOrchardAction> for orchard::note_encryption::CompactAction {
    type Error = anyhow::Error;

    fn try_from(pb_action: pb::CompactOrchardAction) -> anyhow::Result<Self> {
        let nullifier = Nullifier::from_bytes(pb_action.nullifier.as_slice().try_into()?).unwrap();
        let cmx =
            ExtractedNoteCommitment::from_bytes(pb_action.cmx.as_slice().try_into()?).unwrap();
        let key_bytes: [u8; 32] = pb_action.ephemeralKey.as_slice().try_into()?;
        let ephemeral_key = EphemeralKeyBytes::from(key_bytes);
        let enc_ciphertext_bytes: [u8; 52] = pb_action.ciphertext.as_slice().try_into()?;

        Ok(orchard::note_encryption::CompactAction::from_parts(
            nullifier,
            cmx,
            ephemeral_key,
            enc_ciphertext_bytes,
        ))
    }
}
