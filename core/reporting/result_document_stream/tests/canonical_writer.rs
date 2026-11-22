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

/// Strings are escaped byte for byte as `serde_json` (and so `canonical_json`) escapes
/// them, as values and as member names: every ASCII character alone and together, BMP and
/// astral characters, the two line separators, and a string of hundreds of KiB with escapes
/// scattered through it.
#[test]
fn the_writer_escapes_strings_as_serde_json_does() {
    let mut strings: Vec<String> = (0u8..=0x7f).map(|byte| char::from(byte).to_string()).collect();
    strings.push((0u8..=0x7f).map(char::from).collect());
    strings.extend(
        ["", "é", "\u{e000}", "\u{fffd}", "\u{ffff}", "\u{2028}", "\u{2029}", "a\u{2028}b\u{2029}c", "\u{10000}", "\u{1f600}", "\u{10ffff}", "x\u{1f600}\"\u{1f}\u{7f}y"]
            .map(str::to_owned),
    );
    let mut long = String::new();
    let mut at = 0u32;
    while long.len() < 400 * 1024 {
        long.push_str("plain run of text ");
        long.push(char::from_u32([0x22, 0x5c, 0x0a, 0x09, 0x0d, 0x08, 0x0c, 0x00, 0x1f, 0x7f, 0x2f, 0xe9, 0x2028, 0x1f600][at as usize % 14]).unwrap());
        at += 1;
    }
    strings.push(long);
    for text in &strings {
        let value = json!(text);
        let expected = serde_json::to_string(text).unwrap();
        assert_eq!(canonical_text(&value), expected.as_bytes(), "{expected}");
        assert_eq!(canonical_text(&value), canonical_json(&value).as_bytes(), "{expected}");
        let named = json!({ text.as_str(): text.as_str(), "z": 1 });
        assert_eq!(canonical_text(&named), canonical_json(&named).as_bytes(), "{expected}");
    }
}
