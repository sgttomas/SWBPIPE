//! The crate's streamed canonical writer gives `canonical_json`'s text, on the values whose
//! form is most delicate: member order by UTF-16 code units, escapes, number forms and
//! nesting. Its agreement on whole documents is held by `committed_parity.rs`.

use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_result_document_stream::canonical_text;
use serde_json::{json, Value};

#[test]
fn the_writer_gives_canonical_json_text() {
    let values = [
        json!(null),
        json!(true),
        json!({"10": 10, "2": 2, "é": "accent", "𐀀": "supplementary", "\u{e000}": "bmp", "zero": -0.0}),
        json!({"\u{ff2e}": 1, "\u{1f600}": 2, "a": {"b": [], "a": {}}}),
        json!(["\u{0}\u{1f}\"\\/\u{7f}\u{2028}\u{2029}", "tab\tnew\nline", "\u{feff}"]),
        json!([0.1, 1e21, 1e-7, 123456789012345680000.0, 5e-324, -1.5, 1.0, 9007199254740991u64, -9007199254740991i64, 1e300]),
        json!({"deep": [[[[{"x": [1, {"y": null}]}]]]]}),
    ];
    for value in values {
        assert_eq!(canonical_text(&value), canonical_json(&value).as_bytes(), "{value}");
    }
    let parsed: Value = serde_json::from_str(r#"{"b":1.50,"a":[1E2,0.000001,-0]}"#).unwrap();
    assert_eq!(canonical_text(&parsed), canonical_json(&parsed).as_bytes());
}
