//! Bitcoin BLock
//!
//! A block is a collection of transactions and a proof-of-work (POW)
//!

use crate::blockdata::transaction::Transaction;
// use crate::mem;
use crate::num::hash_types::{BlockHash, TxMerkleNode};
use crate::num::uint::U256;

use chrono::prelude::*;
// use ring::digest;

/// The block header contains all the blocks information except for the transactions
/// https://en.bitcoin.it/wiki/Protocol_documentation#Block_Headers
pub struct BlockHeader {
    /// The block / protocol version number.
    /// Updated when you upgrade the software and it specifies a new version.
    #[allow(dead_code)]
    version: i32,
    /// 256-bit hash reference to the previous block header in the blockchain.
    /// Updated when a new block comes in.
    #[allow(dead_code)]
    hash_prev_block: BlockHash,
    /// 256-bit hash based on all of the transactions in the block.
    /// Updated when a transaction is accepted.
    #[allow(dead_code)]
    hash_merkle_root: TxMerkleNode,
    /// Current block timestamp as seconds since 1970-01-01T00:00 UTC.
    /// Update every few seconds.
    #[allow(dead_code)]
    time: u32,
    /// Current target in compact format
    /// The target value below which the blockhash must lie, encoded as a
    /// a float (with well-defined rounding, of course)
    /// Updated when the difficulty is adjusted.
    #[allow(dead_code)]
    bits: u32,
    /// The nonce, selected to obtain a low enough blockhash
    /// Updated when a hash is tried (increments).
    #[allow(dead_code)]
    nonce: u32,
}
/// A bitcoin block, a set of transactions and attached POW header
#[allow(dead_code)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    /// Create a new block
    pub fn new(hash_prev_block: BlockHash, transactions: Vec<Transaction>) -> Block {
        let hash_merkle_root = merkle_root_hash(&transactions);
        Block {
            header: BlockHeader {
                version: 0,
                hash_prev_block,
                hash_merkle_root,
                time: Utc::now().timestamp() as u32,
                bits: 0,
                nonce: 0,
            },
            transactions,
        }
    }
}

/// compute the merkle hash root from a list of transactions
fn merkle_root_hash(_transactions: &Vec<Transaction>) -> TxMerkleNode {
    todo!()
}

impl BlockHeader {
    /// computes the target [0, T], that a blockhash be within to be valid
    pub fn target(&self) -> U256 {
        target(self.bits)
    }
}

/// Each block stores a packed representation (called "Bits") for its actual hexadecimal target.
/// The target can be derived from it via a predefined formula.
/// For example, if the packed target in the block is 0x1b0404cb (stored in little-endian order: cb 04 04 1b), the hexadecimal target is
///
/// 0x0404cb * 2**(8*(0x1b - 3)) = 0x00000000000404CB000000000000000000000000000000000000000000000000

/// computes the target [0, T], that a blockhash be within to be valid
pub fn target(bits: u32) -> U256 {
    // Docs from https://bitcoinj.github.io/javadoc/0.12/org/bitcoinj/core/Utils.html#decodeCompactBits-long-
    // The "compact" format is a representation of a whole number N using an unsigned 32 bit number similar to a floating point format.
    // The most significant 8 bits are the unsigned exponent of base 256. This exponent can be thought of as "number of bytes of N".
    // The lower 23 bits are the mantissa. Bit number 24 (0x800000) represents the sign of N.
    // Therefore, N = (-1^sign) * mantissa * 256^(exponent-3).
    // Satoshi's original implementation used BN_bn2mpi() and BN_mpi2bn().
    // MPI uses the most significant bit of the first byte as sign.
    // Thus 0x1234560000 is compact 0x05123456 and 0xc0de000000 is compact 0x0600c0de. Compact 0x05c0de00 would be -0x40de000000.
    // Bitcoin only uses this "compact" format for encoding difficulty targets, which are unsigned 256bit quantities.
    // Thus, all the complexities of the sign bit and using base 256 are probably an implementation accident.
    let (_mant, _expt) = {
        let unshifted_expt = bits >> 24;
        if unshifted_expt <= 3 {
            ((bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)), 0)
        } else {
            (bits & 0xFFFFFF, 8 * (unshifted_expt - 3))
        }
    };

    // // The mantissa is signed but may not be negative
    // if mant > 0x7FFFFF {
    //     Default::default()
    // } else {
    //     Uint256::from_u64(mant as u64).unwrap() << (expt as usize)
    // }

    // code from C++ bitcoin repo
    // int nShift = (blockindex->nBits >> 24) & 0xff;
    // double dDiff =
    //     (double)0x0000ffff / (double)(blockindex->nBits & 0x00ffffff);

    // while (nShift < 29)
    // {
    //     dDiff *= 256.0;
    //     nShift++;
    // }
    // while (nShift > 29)
    // {
    //     dDiff /= 256.0;
    //     nShift--;
    // }

    // return dDiff;

    todo!()
}

// impl bytes::traits::ToBytes for Block {
//     fn to_bytes(self) -> Bytes {

//     }
// }

/// The hash function should follow the documentation: https://en.bitcoin.it/wiki/Block_hashing_algorithm
pub fn hash(_block: &Block, _nonce: usize) -> BlockHash {
    // let bytes_in = unsafe { mem::any_as_u8_slice(block) };
    // let digest = digest::digest(&digest::SHA256, bytes_in);
    // let digest_bytes = digest.as_ref();
    // assert_eq!(BlockHash::byte_len(), digest::SHA256_OUTPUT_LEN);
    // assert_eq!(digest_bytes.len(), digest::SHA256_OUTPUT_LEN);
    // let mut hash_bytes: BlockHash = BlockHash::from_u8_slice(digest_bytes);
    // hash_bytes
    todo!()
}
