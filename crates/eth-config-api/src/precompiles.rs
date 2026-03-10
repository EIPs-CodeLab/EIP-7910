use serde::{Deserialize, Serialize};

/// Minimal descriptor for a precompile contract.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrecompileDescriptor {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Registry of precompiles for a fork.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct PrecompileRegistry {
    pub entries: Vec<PrecompileDescriptor>,
}

impl PrecompileRegistry {
    pub fn new(entries: Vec<PrecompileDescriptor>) -> Self {
        Self { entries }
    }
}
