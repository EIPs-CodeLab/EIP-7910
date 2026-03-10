use eth_config_api::{EthConfigResponse, ForkConfig};

/// Resolves fork configuration from a chain specification at runtime.
///
/// Reth plumbing will populate this with access to `ChainSpec` and consensus params; the logic
/// belongs here so it can be tested separately from RPC wiring.
#[derive(Debug, Default)]
pub struct ForkConfigResolver;

impl ForkConfigResolver {
    /// Create a new resolver (wire in reth chain spec later).
    pub fn new() -> Self {
        Self
    }

    /// Compute eth_config response for the provided block height.
    pub fn resolve_at_block(&self, _block_number: u64) -> EthConfigResponse {
        // Placeholder implementation; real logic will derive current/next/last forks,
        // blob parameters, precompiles, system contracts, and fork id.
        let placeholder = ForkConfig {
            name: "placeholder".into(),
            block: Some(_block_number),
            blob_schedule: None,
            precompiles: vec![],
            system_contracts: vec![],
        };

        EthConfigResponse {
            current: eth_config_api::types::ForkEntry {
                config: placeholder,
            },
            next: None,
            last: None,
            fork_id: eth_config_api::ForkId::placeholder(),
        }
    }
}
