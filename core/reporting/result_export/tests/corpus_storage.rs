//! Storage regressions: repeated references preserve values without sharing mutation.
#[path = "support/retained_precision_corpus.rs"]
mod retained_precision_corpus;
use retained_precision_corpus::expand;
use serde_json::json;

#[test]
fn repeated_containers_are_independent_and_number_forms_survive() {
    let definition = json!({"format":"retained-precision-corpus-dag/v1", "layout":"compact",
        "nodes":["x", -0.0, 9007199254740993u64, [0,1,2], [1,0,3], [0,4,4]], "root":5});
    let mut value = expand(definition);
    assert!(value[0]["x"][0].as_f64().unwrap().is_sign_negative());
    assert_eq!(value[0]["x"][1].as_u64(), Some(9007199254740993));
    value[0]["x"][0] = json!(7);
    assert!(value[1]["x"][0].as_f64().unwrap().is_sign_negative());
}

#[test]
fn invalid_versions_and_references_are_refused() {
    for definition in [
        json!({"cases": []}),
        json!({"format":"retained-precision-corpus-dag/v2", "layout":"compact", "nodes":[0], "root":0}),
        json!({"format":"retained-precision-corpus-dag/v1", "layout":"compact", "nodes":[[0,0]], "root":0}),
        json!({"format":"retained-precision-corpus-dag/v1", "layout":"other", "nodes":[0], "root":0}),
        json!({"format":"retained-precision-corpus-dag/v1", "layout":"compact", "nodes":[0,[0,true]], "root":1}),
        json!({"format":"retained-precision-corpus-dag/v1", "layout":"compact", "nodes":[0,[0,0.0]], "root":1}),
        json!({"format":"retained-precision-corpus-dag/v1", "layout":"compact", "nodes":[[1]], "root":1}),
        json!({"format":"retained-precision-corpus-dag/v1", "layout":"compact", "nodes":["x",0,[1,0,1,0,1]], "root":2}),
    ] {
        assert!(std::panic::catch_unwind(|| expand(definition)).is_err());
    }
}
