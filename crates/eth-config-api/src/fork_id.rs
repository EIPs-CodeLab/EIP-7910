use crc32fast::Hasher;
use serde::{Deserialize, Serialize};

use crate::types::HexU64;

/// EIP-6122 fork identifier (hash + next fork block number).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForkId {
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<HexU64>,
}

impl ForkId {
    /// Compute fork ID per EIP-6122 (CRC32 of genesis hash + fork block numbers, big-endian u64).
    pub fn compute(genesis_hash: [u8; 32], fork_blocks: &[u64], next_fork: Option<u64>) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&genesis_hash);
        for block in fork_blocks {
            hasher.update(&block.to_be_bytes());
        }
        let hash = hasher.finalize();
        Self {
            hash: format!("0x{hash:08x}"),
            next: next_fork.map(HexU64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fork_id_deterministic() {
        let genesis = [0u8; 32];
        let forks = [0u64, 9, 30_000_000];
        let a = ForkId::compute(genesis, &forks, Some(40_000_000));
        let b = ForkId::compute(genesis, &forks, Some(40_000_000));
        assert_eq!(a, b);
    }
}
