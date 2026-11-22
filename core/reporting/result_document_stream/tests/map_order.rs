//! A value this crate hands over iterates its object members in sorted order only while
//! serde_json is built without its `preserve_order` feature. This fails if any dependency
//! switches that feature on.

use serde_json::{Map, Value};

#[test]
fn a_json_map_iterates_its_members_in_sorted_order() {
    let mut map = Map::new();
    map.insert("b".into(), Value::Null);
    map.insert("a".into(), Value::Null);
    let names: Vec<&str> = map.keys().map(String::as_str).collect();
    assert_eq!(names, ["a", "b"], "serde_json's map keeps insertion order: is preserve_order enabled?");
}
