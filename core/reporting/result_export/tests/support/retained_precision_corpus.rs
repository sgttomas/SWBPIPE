//! Lossless test-fixture storage; the product readers receive the expanded values.
use serde_json::{Map, Value};

/// Decode the versioned DAG, cloning containers at every occurrence so mutations
/// of an expanded base cannot change another base or another occurrence.
pub fn expand(document: Value) -> Value {
    let format = document.get("format").expect("a versioned corpus storage definition");
    assert_eq!(format, "retained-precision-corpus-dag/v1", "unsupported corpus storage format");
    assert_eq!(document.as_object().unwrap().len(), 4, "corpus definition members");
    assert!(matches!(document["layout"].as_str(), Some("compact" | "indent-2")), "corpus layout");
    let nodes = document["nodes"].as_array().unwrap();
    let mut expanded = Vec::<Value>::with_capacity(nodes.len());
    for node in nodes {
        let Some(parts) = node.as_array() else {
            assert!(!node.is_object(), "a corpus node is a scalar or a tagged array");
            expanded.push(node.clone());
            continue;
        };
        let referenced = |value: &Value| {
            let index = usize::try_from(value.as_u64().expect("an unsigned node reference")).unwrap();
            expanded.get(index).expect("corpus references name earlier nodes").clone()
        };
        let value = match parts.first().and_then(Value::as_u64) {
            Some(0) => Value::Array(parts[1..].iter().map(referenced).collect()),
            Some(1) => {
                assert_eq!(parts.len() % 2, 1, "object nodes contain key/value pairs");
                let mut object = Map::new();
                for pair in parts[1..].chunks_exact(2) {
                    let key = referenced(&pair[0]).as_str().expect("a string key").to_owned();
                    assert!(object.insert(key, referenced(&pair[1])).is_none(), "distinct object keys");
                }
                Value::Object(object)
            }
            _ => panic!("invalid corpus node tag"),
        };
        expanded.push(value);
    }
    let root = usize::try_from(document["root"].as_u64().expect("an unsigned root reference")).unwrap();
    expanded.get(root).expect("corpus root exists").clone()
}
