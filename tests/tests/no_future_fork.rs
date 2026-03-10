mod common;
use common::load_example;

#[test]
fn next_and_last_null_when_no_future_fork() {
    let cfg = load_example("cancun_sample.json");

    assert!(cfg.next.is_none());
    assert!(cfg.last.is_some());
}
