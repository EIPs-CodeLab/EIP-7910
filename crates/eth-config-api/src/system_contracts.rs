use serde::{Deserialize, Serialize};

/// Descriptor for a system contract active at a given fork.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SystemContractDescriptor {
    pub address: String,
    pub code_hash: String,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Registry wrapper for convenience.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct SystemContractRegistry {
    pub entries: Vec<SystemContractDescriptor>,
}

impl SystemContractRegistry {
    pub fn new(entries: Vec<SystemContractDescriptor>) -> Self {
        Self { entries }
    }
}
