use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::fork_id::ForkId;

/// Hex-encoded quantity helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HexU64(pub u64);

impl Serialize for HexU64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("0x{:x}", self.0))
    }
}

impl<'de> Deserialize<'de> for HexU64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        u64::from_str_radix(s, 16)
            .map(HexU64)
            .map_err(|e| serde::de::Error::custom(format!("invalid hex quantity: {e}")))
    }
}

/// Blob gas scheduling parameters (EIP-4844 style).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlobSchedule {
    pub max_blob_gas_per_block: HexU64,
    pub target_blob_gas_per_block: HexU64,
    pub blob_gasprice_update_fraction: HexU64,
}

/// Fork-level configuration object as returned by eth_config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForkConfig {
    /// Human-readable fork name (e.g. "cancun").
    pub name: String,
    /// Block number at which the fork activates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<HexU64>,
    /// Optional blob gas schedule parameters for blob-enabled forks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_schedule: Option<BlobSchedule>,
    /// List of enabled precompiles at this fork (hex addresses as strings).
    #[serde(default)]
    pub precompiles: BTreeMap<String, String>,
    /// List of system contracts active at this fork.
    #[serde(default)]
    pub system_contracts: BTreeMap<String, SystemContract>,
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
    pub current: ForkConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<ForkConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<ForkConfig>,
    pub fork_id: ForkId,
}
