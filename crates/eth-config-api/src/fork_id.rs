use serde::{Deserialize, Serialize};

/// EIP-6122 fork identifier (hash + next fork block number).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForkId {
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<u64>,
}

impl ForkId {
    /// Placeholder constructor for scaffolding.
    pub fn placeholder() -> Self {
        Self {
            hash: String::from("0x00000000"),
            next: None,
        }
    }
}
