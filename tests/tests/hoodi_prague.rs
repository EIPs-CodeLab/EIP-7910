mod common;
use common::load_example;

#[test]
fn prague_sample_deserializes() {
    let cfg = load_example("prague_sample.json");

    assert_eq!(cfg.current.config.name, "prague");
    assert!(cfg.current.config.blob_schedule.is_none());
    assert!(cfg.last.is_some());
    assert!(cfg.fork_id.next.is_some());
}
