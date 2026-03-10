mod common;
use common::load_example;

#[test]
fn cancun_sample_deserializes() {
    let cfg = load_example("cancun_sample.json");

    assert_eq!(cfg.current.config.name, "cancun");
    assert!(cfg.current.config.blob_schedule.is_some());
    assert!(cfg.next.is_none());
    assert!(cfg.last.is_some());
    assert_eq!(cfg.fork_id.hash, "0xf0afd0c7");
}
