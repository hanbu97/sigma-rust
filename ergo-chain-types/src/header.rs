use crate::{ADDigest, BlockId, Digest32};

use ergotree_ir::{
    chain::{
        ergo_box::{box_value::BoxValue, BoxId},
        header::{AutolykosSolution, Header},
        votes::Votes,
    },
    ergo_tree::ErgoTree,
    serialization::{sigma_byte_writer::SigmaByteWriter, SigmaSerializable},
    sigma_protocol::dlog_group::{order, EcPoint},
};

/// Represents data of the block header available in Sigma propositions.
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Header {
    /// Block version, to be increased on every soft and hardfork.
    #[cfg_attr(feature = "json", serde(rename = "version"))]
    pub version: u8,
    /// Bytes representation of ModifierId of this Header
    #[cfg_attr(feature = "json", serde(rename = "id"))]
    pub id: BlockId,
    /// Bytes representation of ModifierId of the parent block
    #[cfg_attr(feature = "json", serde(rename = "parentId"))]
    pub parent_id: BlockId,
    /// Hash of ADProofs for transactions in a block
    #[cfg_attr(feature = "json", serde(rename = "adProofsRoot"))]
    pub ad_proofs_root: Digest32,
    /// AvlTree of a state after block application
    #[cfg_attr(feature = "json", serde(rename = "stateRoot"))]
    pub state_root: ADDigest,
    /// Root hash (for a Merkle tree) of transactions in a block.
    #[cfg_attr(feature = "json", serde(rename = "transactionsRoot"))]
    pub transaction_root: Digest32,
    /// Timestamp of a block in ms from UNIX epoch
    #[cfg_attr(feature = "json", serde(rename = "timestamp"))]
    pub timestamp: u64,
    /// Current difficulty in a compressed view.
    #[cfg_attr(feature = "json", serde(rename = "nBits"))]
    pub n_bits: u64,
    /// Block height
    #[cfg_attr(feature = "json", serde(rename = "height"))]
    pub height: u32,
    /// Root hash of extension section
    #[cfg_attr(feature = "json", serde(rename = "extensionHash"))]
    pub extension_root: Digest32,
    /// Solution for an Autolykos PoW puzzle
    #[cfg_attr(feature = "json", serde(rename = "powSolutions"))]
    pub autolykos_solution: AutolykosSolution,
    /// Miner votes for changing system parameters.
    /// 3 bytes in accordance to Scala implementation, but will use `Vec` until further improvements
    #[cfg_attr(feature = "json", serde(rename = "votes"))]
    pub votes: Votes,
}

/// Solution for an Autolykos PoW puzzle. In Autolykos v.1 all the four fields are used, in
/// Autolykos v.2 only `miner_pk` and `nonce` fields are used.
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AutolykosSolution {
    /// Public key of miner. Part of Autolykos solution.
    #[cfg_attr(feature = "json", serde(rename = "pk"))]
    pub miner_pk: Box<dlog_group::EcPoint>,
    /// One-time public key. Prevents revealing of miners secret.
    #[cfg_attr(feature = "json", serde(default, rename = "w"))]
    pub pow_onetime_pk: Option<Box<dlog_group::EcPoint>>,
    /// nonce
    #[cfg_attr(
        feature = "json",
        serde(
            rename = "n",
            serialize_with = "crate::chain::json::autolykos_solution::as_base16_string",
            deserialize_with = "crate::chain::json::autolykos_solution::from_base16_string"
        )
    )]
    pub nonce: Vec<u8>,
    /// Distance between pseudo-random number, corresponding to nonce `nonce` and a secret,
    /// corresponding to `miner_pk`. The lower `pow_distance` is, the harder it was to find this
    /// solution.
    ///
    /// Note: we serialize/deserialize through custom functions since `BigInt`s serde implementation
    /// encodes the sign and absolute-value of the value separately, which is incompatible with the
    /// JSON representation used by Ergo. ASSUMPTION: we assume that `pow_distance` encoded as a
    /// `u64`.
    #[cfg_attr(
        feature = "json",
        serde(
            default,
            rename = "d",
            serialize_with = "crate::chain::json::autolykos_solution::bigint_as_str",
            deserialize_with = "crate::chain::json::autolykos_solution::bigint_from_serde_json_number"
        )
    )]
    pub pow_distance: Option<BigInt>,
}
