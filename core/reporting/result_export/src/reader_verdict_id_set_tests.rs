//! Tests of `reader_verdict.rs`'s `IdSet`. They live here, not in
//! `reader_verdict.rs`, because `product_physics`'s `retained_memory_law_tests`
//! requires that file to carry no test region, and are declared from
//! `arc_evidence.rs`, which no retained-pricing model pins.
use crate::reader_verdict::IdSet;
use serde_json::{json, Value};

/// The set keeps the ids, and answers membership, as the list scan it replaced:
/// strings by text, every other id by `Value` equality (1 is not 1.0).
#[test]
fn id_set_is_the_distinct_list() {
    let ids = [json!("a"), json!("b"), json!("a"), json!(1), json!(1.0), json!(1), json!(null), json!(null),
        json!(["a"]), json!(["a"]), json!({"a": 1}), json!(""), json!("")];
    let mut list: Vec<&Value> = Vec::new();
    let mut set = IdSet::default();
    for id in &ids {
        if !list.contains(&id) {
            list.push(id);
        }
        set.insert(id);
    }
    assert_eq!(set.len(), list.len());
    let probes = [json!("a"), json!("c"), json!(1), json!(1.0), json!(2), json!(null), json!(["a"]), json!({"a": 1}),
        json!({"a": 2}), json!(""), json!(false)];
    for probe in &probes {
        assert_eq!(set.contains(probe), list.contains(&probe), "{probe}");
    }
}
