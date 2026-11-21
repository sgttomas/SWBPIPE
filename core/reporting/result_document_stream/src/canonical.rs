//! RFC 8785 canonical JSON text of a value, written in order to a byte consumer, so that
//! no member's text is held whole.
//!
//! The bytes are those `open_pipe_stress_canonical_json::canonical_json` returns for the
//! same value: numbers through its public `canonical_number`, strings escaped by
//! `serde_json` as that crate escapes them, object members in ascending UTF-16 code unit
//! order, arrays in order and no insignificant whitespace. That crate's own writer over a
//! sink is private, so it is written here; `tests/canonical_writer.rs` holds the two equal.

use open_pipe_stress_canonical_json::canonical_number;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

/// Writes the canonical text of `value` to `out`, in order.
pub(crate) fn write(value: &Value, out: &mut dyn FnMut(&[u8])) {
    match value {
        Value::Null => out(b"null"),
        Value::Bool(true) => out(b"true"),
        Value::Bool(false) => out(b"false"),
        Value::Number(number) => out(canonical_number(number).as_bytes()),
        Value::String(text) => write_str(text, out),
        Value::Array(items) => {
            out(b"[");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out(b",");
                }
                write(item, out);
            }
            out(b"]");
        }
        Value::Object(map) => {
            out(b"{");
            for (index, key) in sorted_keys(map).into_iter().enumerate() {
                if index > 0 {
                    out(b",");
                }
                write_str(key, out);
                out(b":");
                write(&map[key.as_str()], out);
            }
            out(b"}");
        }
    }
}

/// A JSON string, escaped as `serde_json` (and so `canonical_json`) escapes it.
pub(crate) fn write_str(text: &str, out: &mut dyn FnMut(&[u8])) {
    out(serde_json::to_string(text).expect("a string always encodes as JSON").as_bytes());
}

/// An object's member names in canonical order (RFC 8785 §3.2.3: UTF-16 code units).
pub(crate) fn sorted_keys(map: &Map<String, Value>) -> Vec<&String> {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
    keys
}

/// The canonical text of `value` as bytes (for a row entry, which is small).
pub(crate) fn bytes(value: &Value, into: &mut Vec<u8>) {
    into.clear();
    write(value, &mut |piece| into.extend_from_slice(piece));
}

/// SHA-256 of the canonical text of `value`, without holding the text.
pub(crate) fn sha256(value: &Value) -> [u8; 32] {
    let mut hasher = Sha256::new();
    write(value, &mut |piece| hasher.update(piece));
    hasher.finalize().into()
}

/// Lowercase hexadecimal, as `format!("{:x}", digest)` writes a SHA-256.
pub(crate) fn hex(digest: &[u8; 32]) -> String {
    use std::fmt::Write;
    let mut text = String::with_capacity(64);
    for byte in digest {
        write!(text, "{byte:02x}").expect("writing to a String does not fail");
    }
    text
}

/// The canonical text of `value`, for this crate's tests.
#[doc(hidden)]
pub fn canonical_text(value: &Value) -> Vec<u8> {
    let mut text = Vec::new();
    bytes(value, &mut text);
    text
}
