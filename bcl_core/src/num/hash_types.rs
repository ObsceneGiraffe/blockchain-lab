//! A set of hash types to communicate the intent the various hashes used to implement bitcoin.
//! Highlighting the intent of a hash makes the code easier to reason about.
//! Hashes used to implement bitcoin are SHA256, SHA256d, and RIPEMD160

use crate::num::uint::U256;

/// The hash of a single bitcoin block
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct BlockHash(U256);

/// The hash of two Merkle tree nodes
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TxMerkleNode(U256);

/// The transaction ID os a single bitcoin Transaction
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Txid(U256);
