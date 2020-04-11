use crate::crypto::hash::{Hashable, H256};
use keccak_hash::keccak;

// TODO: Add the address of the miner

/// The header of a block.
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Copy)]
pub struct Header {
    /// Hash of the parent proposer block.
    pub parent: H256,
    /// Block creation time in UNIX format.
    pub timestamp: u128,
    /// Proof of work nonce.
    pub nonce: u32,
    /// Merkle root of the block content.
    pub content_merkle_root: H256,
    /// Extra content for debugging purposes.
    pub extra_content: [u8; 32],
    /// Mining difficulty of this block.
    pub difficulty: H256,
}

impl Header {
    /// Create a new block header.
    pub fn new(
        parent: H256,
        timestamp: u128,
        nonce: u32,
        content_merkle_root: H256,
        extra_content: [u8; 32],
        difficulty: H256,
    ) -> Self {
        Self {
            parent,
            timestamp,
            nonce,
            content_merkle_root,
            extra_content,
            difficulty,
        }
    }
}

impl Hashable for Header {
    fn hash(&self) -> H256 {
        let serialized = bincode::serialize(self).unwrap();
        keccak(&serialized).into()
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::header::Header;

    use crate::crypto::hash::{Hashable, H256};

    ///The hash should match
    #[test]
    fn test_hash() {
        let header = sample_header();
        let header_hash_should_be = sample_header_hash_should_be();
        assert_eq!(header.hash(), header_hash_should_be);
    }

    #[macro_export]
    macro_rules! gen_hashed_data {
        () => {{
            vec![
                (&hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (&hex!("0102010201020102010201020102010201020102010201020102010201020102")).into(),
                (&hex!("0a0a0a0a0b0b0b0b0a0a0a0a0b0b0b0b0a0a0a0a0b0b0b0b0a0a0a0a0b0b0b0b")).into(),
                (&hex!("0403020108070605040302010807060504030201080706050403020108070605")).into(),
                (&hex!("1a2a3a4a1a2a3a4a1a2a3a4a1a2a3a4a1a2a3a4a1a2a3a4a1a2a3a4a1a2a3a4a")).into(),
                (&hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")).into(),
                (&hex!("0000000100000001000000010000000100000001000000010000000100000001")).into(),
            ]
        }};
    }

    // Header stuff
    pub fn sample_header() -> Header {
        let parent_hash: H256 =
            (&hex!("0102010201020102010201020102010201020102010201020102010201020102")).into();
        let timestamp: u128 = 7_094_730;
        let nonce: u32 = 839_782;
        let content_root: H256 =
            (&hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")).into();
        let extra_content: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let difficulty: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 20, 10,
        ];
        let difficulty = (&difficulty).into();
        let header = Header::new(
            parent_hash,
            timestamp,
            nonce,
            content_root,
            extra_content,
            difficulty,
        );
        header
    }

    pub fn sample_header_hash_should_be() -> H256 {
        let header_hash_should_be =
            (&hex!("a34291a7290e7036c18903b867b39ff0609301673e153f1b9e199663fe1622c5")).into(); // Calculated on Jan 23, 2020
        header_hash_should_be
    }
}
