use ream_bls::BLSSignature;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::{VariableList, typenum};
use tree_hash_derive::TreeHash;

use crate::attestation_data::AttestationData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash)]
pub struct IndexedAttestation {
    pub attesting_indices: VariableList<u64, typenum::U2048>,
    pub data: AttestationData,
    pub signature: BLSSignature,
}
