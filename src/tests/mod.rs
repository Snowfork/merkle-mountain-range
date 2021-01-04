mod test_accumulate_headers;
mod test_helper;
mod test_mmr;
use faster_hex::{hex_decode, hex_string};

use crate::Merge;
use blake2b_rs::{Blake2b, Blake2bBuilder};
use bytes::Bytes;

use std::str;
use sha3::{Digest, Sha3_256, Keccak256};

fn new_blake2b() -> Blake2b {
    Blake2bBuilder::new(32).build()
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct NumberHash(pub Bytes);
impl From<u32> for NumberHash {
    fn from(num: u32) -> Self {
        let hash = Keccak256::digest(&num.to_le_bytes());
        println!("SHA3-256 Hash: {:x}", hash);
        NumberHash(hash.to_vec().into())
    }
}

struct MergeNumberHash;

impl Merge for MergeNumberHash {
    type Item = NumberHash;
    // TODO: prefix by index to avoid hash collisions
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        let mut hasher = Keccak256::new();
        println!("MERGING {:?} and {:?}", hex_string(&lhs.0).unwrap(), hex_string(&rhs.0).unwrap());
        hasher.update(&lhs.0);
        hasher.update(&rhs.0);
        let result = hasher.finalize();
        println!("MERGED INTO: {:?}", hex_string(&result).unwrap());
        NumberHash(result.to_vec().into())
    }
}
