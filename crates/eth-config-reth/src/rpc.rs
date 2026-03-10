#![allow(unused)]

use eth_config_api::EthConfigResponse;

use crate::{ForkConfigCache, ForkConfigResolver};

/// Jsonrpsee-facing API surface for `eth_config`.
pub struct EthConfigApi {
    pub cache: ForkConfigCache,
    pub resolver: ForkConfigResolver,
}

impl EthConfigApi {
    pub fn new(cache: ForkConfigCache, resolver: ForkConfigResolver) -> Self {
        Self { cache, resolver }
    }

    /// Placeholder handler; will be wired to jsonrpsee RpcModule later.
    pub fn eth_config(&self, block_number: u64) -> EthConfigResponse {
        self.cache
            .get(block_number, |block| self.resolver.resolve_at_block(block))
    }
}
