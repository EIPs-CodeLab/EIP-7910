#![allow(unused)]

use crate::{EthConfigApi, ForkConfigCache, ForkConfigResolver};

/// Hook to install the `eth_config` namespace into reth's RPC builder.
pub fn install_namespace() {
    let _api = EthConfigApi::new(ForkConfigCache::new(), ForkConfigResolver::new());
    // TODO: wire `_api` into reth's RpcModule when integrating with the node.
}
