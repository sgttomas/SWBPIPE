//! RFC 8785 canonical JSON text of a value, written in order to a byte consumer, so that
//! no member's text is held whole.
//!
//! The bytes are those `open_pipe_stress_canonical_json::canonical_json` returns for the
//! same value: numbers through its public `canonical_number`, strings escaped as
//! `serde_json` escapes them for that crate (written here as they are read, not built whole
//! first), object members in ascending UTF-16 code unit
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

/// A JSON string, escaped as `serde_json` (and so `canonical_json`) escapes it, written as
/// it is read: the quotes, each run of characters that needs no escape as a slice of the
/// string's own UTF-8, and each escape. `"` and `\` are escaped with a backslash; of the
/// control characters U+0000 to U+001F, backspace, form feed, line feed, carriage return and
/// tab as `\b`, `\f`, `\n`, `\r` and `\t`, and the others as `\u00xx` in lowercase hexadecimal;
/// every other character, U+007F, U+2028 and U+2029 included, is written as it is.
pub(crate) fn write_str(text: &str, out: &mut dyn FnMut(&[u8])) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = text.as_bytes();
    out(b"\"");
    let mut run = 0;
    for (at, &byte) in bytes.iter().enumerate() {
        let short: &[u8] = match byte {
            b'"' => b"\\\"",
            b'\\' => b"\\\\",
            0x08 => b"\\b",
            0x0c => b"\\f",
            b'\n' => b"\\n",
            b'\r' => b"\\r",
            b'\t' => b"\\t",
            0x00..=0x1f => b"",
            _ => continue,
        };
        if run < at {
            out(&bytes[run..at]);
        }
        if short.is_empty() {
            out(&[b'\\', b'u', b'0', b'0', HEX[usize::from(byte >> 4)], HEX[usize::from(byte & 0x0f)]]);
        } else {
            out(short);
        }
        run = at + 1;
    }
    if run < bytes.len() {
        out(&bytes[run..]);
    }
    out(b"\"");
}

/// An object's member names in canonical order (RFC 8785 §3.2.3: UTF-16 code units).
pub(crate) fn sorted_keys(map: &Map<String, Value>) -> Vec<&String> {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
    keys
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
    write(value, &mut |piece| text.extend_from_slice(piece));
    text
}
