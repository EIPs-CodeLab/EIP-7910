//! Reth integration layer for the EIP-7910 `eth_config` RPC.

#[cfg(feature = "reth")]
pub mod cache;
#[cfg(feature = "reth")]
pub mod namespace;
#[cfg(feature = "reth")]
pub mod resolver;
#[cfg(feature = "reth")]
pub mod rpc;

#[cfg(feature = "reth")]
pub use cache::ForkConfigCache;
#[cfg(feature = "reth")]
pub use resolver::ForkConfigResolver;
#[cfg(feature = "reth")]
pub use rpc::EthConfigApi;
