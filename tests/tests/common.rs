use std::path::PathBuf;

use eth_config_api::EthConfigResponse;

pub fn load_example(name: &str) -> EthConfigResponse {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("../examples").join(name);
    let data = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    serde_json::from_str(&data)
        .unwrap_or_else(|e| panic!("failed to parse {}: {}", path.display(), e))
}
