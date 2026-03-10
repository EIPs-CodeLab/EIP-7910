use serde::{Deserialize, Serialize};

/// Blob gas scheduling parameters (EIP-4844 style).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlobSchedule {
    pub max_blob_gas_per_block: u64,
    pub target_blob_gas_per_block: u64,
    pub blob_gasprice_update_fraction: u64,
}

/// Fork-level configuration object as returned by eth_config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForkConfig {
    /// Human-readable fork name (e.g. "cancun").
    pub name: String,
    /// Block number at which the fork activates; null in EIP becomes None here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<u64>,
    /// Optional blob gas schedule parameters for blob-enabled forks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_schedule: Option<BlobSchedule>,
    /// List of enabled precompiles at this fork (hex addresses as strings).
    pub precompiles: Vec<String>,
    /// List of system contracts active at this fork.
    #[serde(default)]
    pub system_contracts: Vec<SystemContract>,
}

/// System contract entry in eth_config response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SystemContract {
    pub address: String,
    pub code_hash: String,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Fork metadata combining fork id and config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForkEntry {
    pub config: ForkConfig,
}

/// Top-level eth_config response payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EthConfigResponse {
    pub current: ForkEntry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<ForkEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<ForkEntry>,
    pub fork_id: crate::fork_id::ForkId,
}
