mod common;
use common::load_example;

#[test]
fn blob_parameter_only_fork_logic() {
    let cfg = load_example("blob_parameter_only.json");

    assert_eq!(cfg.current.name, "bpo");
    let blob = cfg
        .current
        .blob_schedule
        .as_ref()
        .expect("blob schedule present");
    assert_eq!(blob.max_blob_gas_per_block.0, 800_000);
    assert!(cfg.fork_id.next.is_none());
}
