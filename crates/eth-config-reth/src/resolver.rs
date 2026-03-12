use eth_config_api::{EthConfigResponse, ForkConfig, ForkId};

/// Resolves fork configuration from a chain specification at runtime.
///
/// Reth plumbing will populate this with access to `ChainSpec` and consensus params; the logic
/// belongs here so it can be tested separately from RPC wiring.
#[derive(Debug, Clone)]
pub struct ForkConfigResolver {
    genesis_hash: [u8; 32],
    forks: Vec<ForkConfig>,
}

impl ForkConfigResolver {
    /// Create a new resolver with a pre-resolved fork list and genesis hash.
    pub fn new(genesis_hash: [u8; 32], mut forks: Vec<ForkConfig>) -> Self {
        forks.sort_by_key(|f| f.block.map(|b| b.0).unwrap_or(u64::MAX));
        Self { genesis_hash, forks }
    }

    /// Compute eth_config response for the provided block height.
    pub fn resolve_at_block(&self, block_number: u64) -> EthConfigResponse {
        let mut current: Option<ForkConfig> = None;
        let mut last: Option<ForkConfig> = None;
        let mut next: Option<ForkConfig> = None;

        for fork in &self.forks {
            let fork_block = fork.block.map(|b| b.0).unwrap_or(u64::MAX);
            if fork_block <= block_number {
                last = current.replace(fork.clone());
                continue;
            }
            next = Some(fork.clone());
            break;
        }

        let current = current.unwrap_or_else(|| self.forks.first().cloned().unwrap_or_default());

        let fork_blocks: Vec<u64> = self
            .forks
            .iter()
            .filter_map(|f| f.block.map(|b| b.0))
            .filter(|b| *b <= block_number)
            .collect();

        let fork_id = ForkId::compute(
            self.genesis_hash,
            &fork_blocks,
            next.as_ref().and_then(|f| f.block.map(|b| b.0)),
        );

        EthConfigResponse {
            current,
            next,
            last,
            fork_id,
        }
    }
}
