#![allow(unused)]

use crate::{EthConfigApi, ForkConfigCache, ForkConfigResolver};

/// Hook to install the `eth_config` namespace into reth's RPC builder.
pub fn install_namespace() {
    let zero_genesis = [0u8; 32];
    let resolver = ForkConfigResolver::new(zero_genesis, Vec::new());
    let _api = EthConfigApi::new(ForkConfigCache::new(), resolver);
    // TODO: wire `_api` into reth's RpcModule when integrating with the node.
}
