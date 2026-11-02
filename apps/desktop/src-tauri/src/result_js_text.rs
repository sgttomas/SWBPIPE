//! C1 native custody, slice 5's persistence (DEL-04-05
//! `c1-native-custody-plan.md` slice 5): the received result's text as the
//! webview writes it today, from the held producer bytes, without a value
//! tree or a copy of the whole text.
//!
//! Record Run stores `jsonTextKeepingNegativeZero(JSON.parse(bytes))`
//! (`analysisRunService.recordAnalysisRun`); the webview's Save Project sends
//! `JSON.stringify` of the same parse. This writes either text from the bytes:
//! - **Members in JavaScript's order.** `JSON.parse` defines each member on a
//!   plain object, so `Object.entries` lists the array-index keys (canonical
//!   decimal integers below 2^32 - 1) first, ascending, then the other keys in
//!   first-definition order; a repeated key keeps its first position and its
//!   last value.
//! - **Strings** as `JSON.stringify` writes the parsed string: the escapes of
//!   `"`, `\`, `\b`, `\f`, `\n`, `\r`, `\t` and `\u00xx` (lowercase) for the
//!   other control characters, nothing else (serde_json's string writer is the
//!   same table). A string with no escape is written as it stands. A lone
//!   surrogate, which serde_json cannot hold, is refused (the held result's
//!   readers refuse it first).
//! - **Numbers** as JavaScript's binary64 reading of the decimal text
//!   (correctly rounded, as Rust's `f64` parse is) written by
//!   `Number::toString` (`ecma_number_to_string`); a negative zero as `-0`
//!   ([`NegativeZero::Keep`], `jsonTextKeepingNegativeZero`) or `0`
//!   ([`NegativeZero::AsZero`], `JSON.stringify`); a number JavaScript reads
//!   as infinite as `null`.
//! - **No whitespace**, and `true`, `false`, `null` unchanged.
//!
//! Each call reads the bytes once more: [`length`] counts the text into a
//! counter (nothing grows with the text), [`write`] writes it into a sink (the
//! caller's buffer of exactly that length). The working memory is one object's
//! member list (its decoded keys and value spans) per nesting level along the
//! path being written ([`scratch_bytes`] prices the heaviest path over the
//! parsed value, whose keys are the same).
use crate::analysis_run_v03_build::{Count, Sink};
use crate::ingress;
use open_pipe_stress_canonical_json::ecma_number_to_string;
use serde_json::Value;

/// How a negative zero is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NegativeZero {
    /// `-0` (`jsonTextKeepingNegativeZero`).
    Keep,
    /// `0` (`JSON.stringify`; the save's form is its serde_json reading, which
    /// the builder's stored writer writes from the parse).
    #[cfg_attr(not(test), allow(dead_code))]
    AsZero,
}

/// A text the writer cannot read as JSON (the held bytes passed serde_json's
/// parse first, so this is not reached for them).
const NOT_JSON: &str = "RESULT-JS-TEXT-INVALID";

fn invalid(at: usize) -> String {
    format!("{NOT_JSON}: the held result is not JSON at byte {at}")
}

/// The exact length of the text.
pub(crate) fn length(text: &str, zero: NegativeZero) -> Result<u64, String> {
    let mut count = Count::default();
    write(text, zero, &mut count)?;
    Ok(count.0)
}

/// Writes the text into `out`.
pub(crate) fn write<S: Sink>(text: &str, zero: NegativeZero, out: &mut S) -> Result<(), String> {
    let bytes = text.as_bytes();
    let mut reader = Reader { text, bytes, zero };
    let start = reader.skip_ws(0);
    let end = reader.value(start, out)?;
    if reader.skip_ws(end) != bytes.len() {
        return Err(invalid(end));
    }
    Ok(())
}

/// The writer's working memory for `value`'s text (module docs): the heaviest
/// member-list scratch along one path (per object, its member entries and its
/// decoded keys, and its key index once it is not small, each as a growing
/// allocation whose last doubling's old and new buffers are live together),
/// and the largest decoded string (an escaped string is decoded before it is
/// written again).
pub(crate) fn scratch_bytes(value: &Value) -> u64 {
    path_scratch(value).saturating_add(envelope(largest_string(value)).saturating_mul(3))
}

fn envelope(bytes: u64) -> u64 {
    ingress::allocation_envelope(bytes).unwrap_or(u64::MAX)
}

fn path_scratch(value: &Value) -> u64 {
    match value {
        Value::Array(items) => items.iter().map(path_scratch).max().unwrap_or(0),
        Value::Object(map) => {
            let entries = (map.len() as u64).saturating_mul(std::mem::size_of::<Member>() as u64);
            let mut keys = map.keys().fold(0u64, |sum, key| sum.saturating_add(envelope(key.len() as u64)));
            let mut own = envelope(entries).saturating_mul(3);
            if map.len() >= INDEXED_FROM {
                // The key index: a second copy of each key and its table (an
                // entry of a key and a position, at most seven eighths full).
                let table = (map.len() as u64).saturating_mul(2).saturating_mul((std::mem::size_of::<(String, usize)>() + 1) as u64);
                own = own.saturating_add(envelope(table).saturating_mul(3));
                keys = keys.saturating_mul(2);
            }
            own.saturating_add(keys).saturating_add(map.values().map(path_scratch).max().unwrap_or(0))
        }
        _ => 0,
    }
}

fn largest_string(value: &Value) -> u64 {
    match value {
        Value::String(text) => text.len() as u64,
        Value::Array(items) => items.iter().map(largest_string).max().unwrap_or(0),
        Value::Object(map) => map.iter().map(|(key, item)| (key.len() as u64).max(largest_string(item))).max().unwrap_or(0),
        _ => 0,
    }
}

/// An object with this many members finds a repeated key through an index of
/// its keys rather than a scan.
const INDEXED_FROM: usize = 32;

/// One object member: its decoded key, its value's span, and the order key.
struct Member {
    key: String,
    value: (usize, usize),
    index: Option<u32>,
}

struct Reader<'t> {
    text: &'t str,
    bytes: &'t [u8],
    zero: NegativeZero,
}

/// An array index key (ECMAScript): the canonical decimal text of an integer
/// in [0, 2^32 - 2].
fn array_index(key: &str) -> Option<u32> {
    let digits = key.as_bytes();
    if digits.is_empty() || digits.len() > 10 || !digits.iter().all(u8::is_ascii_digit) || (digits.len() > 1 && digits[0] == b'0') {
        return None;
    }
    let value: u64 = key.parse().ok()?;
    (value < u64::from(u32::MAX)).then_some(value as u32)
}

impl Reader<'_> {
    fn skip_ws(&self, mut at: usize) -> usize {
        while at < self.bytes.len() && matches!(self.bytes[at], b' ' | b'\t' | b'\n' | b'\r') {
            at += 1;
        }
        at
    }

    fn expect(&self, at: usize, byte: u8) -> Result<usize, String> {
        if self.bytes.get(at) == Some(&byte) {
            Ok(at + 1)
        } else {
            Err(invalid(at))
        }
    }

    /// The end of the string token starting at `at` (its opening quote), and
    /// whether it holds an escape.
    fn string_end(&self, at: usize) -> Result<(usize, bool), String> {
        let mut index = self.expect(at, b'"')?;
        let mut escaped = false;
        loop {
            match self.bytes.get(index) {
                None => return Err(invalid(index)),
                Some(b'"') => return Ok((index + 1, escaped)),
                Some(b'\\') => {
                    escaped = true;
                    index += 2;
                }
                Some(_) => index += 1,
            }
        }
    }

    /// The decoded string of the token `[start, end)`.
    fn decoded(&self, start: usize, end: usize) -> Result<String, String> {
        serde_json::from_str::<String>(&self.text[start..end]).map_err(|_| invalid(start))
    }

    fn number_end(&self, mut at: usize) -> usize {
        while at < self.bytes.len() && matches!(self.bytes[at], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
            at += 1;
        }
        at
    }

    /// The end of the value starting at `at`, without writing it.
    fn skip_value(&self, at: usize) -> Result<usize, String> {
        match self.bytes.get(at) {
            Some(b'"') => Ok(self.string_end(at)?.0),
            Some(b'{') | Some(b'[') => {
                let mut depth = 0usize;
                let mut index = at;
                loop {
                    match self.bytes.get(index) {
                        None => return Err(invalid(index)),
                        Some(b'"') => index = self.string_end(index)?.0,
                        Some(b'{') | Some(b'[') => {
                            depth += 1;
                            index += 1;
                        }
                        Some(b'}') | Some(b']') => {
                            depth -= 1;
                            index += 1;
                            if depth == 0 {
                                return Ok(index);
                            }
                        }
                        Some(_) => index += 1,
                    }
                }
            }
            Some(b't') | Some(b'n') => Ok(at + 4),
            Some(b'f') => Ok(at + 5),
            Some(_) => {
                let end = self.number_end(at);
                if end == at { Err(invalid(at)) } else { Ok(end) }
            }
            None => Err(invalid(at)),
        }
    }

    /// Writes the value starting at `at`; returns its end.
    fn value<S: Sink>(&mut self, at: usize, out: &mut S) -> Result<usize, String> {
        match self.bytes.get(at) {
            Some(b'{') => self.object(at, out),
            Some(b'[') => {
                out.put(b"[");
                let mut index = self.skip_ws(at + 1);
                if self.bytes.get(index) == Some(&b']') {
                    out.put(b"]");
                    return Ok(index + 1);
                }
                let mut first = true;
                loop {
                    if !first {
                        out.put(b",");
                    }
                    first = false;
                    index = self.value(index, out)?;
                    index = self.skip_ws(index);
                    match self.bytes.get(index) {
                        Some(b',') => index = self.skip_ws(index + 1),
                        Some(b']') => {
                            out.put(b"]");
                            return Ok(index + 1);
                        }
                        _ => return Err(invalid(index)),
                    }
                }
            }
            Some(b'"') => {
                let (end, escaped) = self.string_end(at)?;
                if escaped {
                    let decoded = self.decoded(at, end)?;
                    write_string(&decoded, out);
                } else {
                    out.put(&self.bytes[at..end]);
                }
                Ok(end)
            }
            Some(b't') if self.bytes[at..].starts_with(b"true") => {
                out.put(b"true");
                Ok(at + 4)
            }
            Some(b'f') if self.bytes[at..].starts_with(b"false") => {
                out.put(b"false");
                Ok(at + 5)
            }
            Some(b'n') if self.bytes[at..].starts_with(b"null") => {
                out.put(b"null");
                Ok(at + 4)
            }
            Some(_) => {
                let end = self.number_end(at);
                let number: f64 = self.text[at..end].parse().map_err(|_| invalid(at))?;
                if !number.is_finite() {
                    out.put(b"null");
                } else if number == 0.0 && number.is_sign_negative() && self.zero == NegativeZero::Keep {
                    out.put(b"-0");
                } else {
                    out.put(ecma_number_to_string(number).as_bytes());
                }
                Ok(end)
            }
            None => Err(invalid(at)),
        }
    }

    fn object<S: Sink>(&mut self, at: usize, out: &mut S) -> Result<usize, String> {
        let mut members: Vec<Member> = Vec::new();
        let mut positions: Option<std::collections::HashMap<String, usize>> = None;
        let mut index = self.skip_ws(at + 1);
        let end = if self.bytes.get(index) == Some(&b'}') {
            index + 1
        } else {
            loop {
                let (key_end, escaped) = self.string_end(index)?;
                let key = if escaped { self.decoded(index, key_end)? } else { self.text[index + 1..key_end - 1].to_string() };
                index = self.skip_ws(key_end);
                index = self.skip_ws(self.expect(index, b':')?);
                let value_end = self.skip_value(index)?;
                let span = (index, value_end);
                // A repeated key keeps its first position and takes the last value
                // (a key index once the object is not small).
                if positions.is_none() && members.len() >= INDEXED_FROM {
                    positions = Some(members.iter().enumerate().map(|(at, member)| (member.key.clone(), at)).collect());
                }
                let found = match &positions {
                    Some(positions) => positions.get(&key).copied(),
                    None => members.iter().position(|member| member.key == key),
                };
                match found {
                    Some(at) => members[at].value = span,
                    None => {
                        let order = array_index(&key);
                        if let Some(positions) = positions.as_mut() {
                            positions.insert(key.clone(), members.len());
                        }
                        members.push(Member { key, value: span, index: order });
                    }
                }
                index = self.skip_ws(value_end);
                match self.bytes.get(index) {
                    Some(b',') => index = self.skip_ws(index + 1),
                    Some(b'}') => break index + 1,
                    _ => return Err(invalid(index)),
                }
            }
        };
        // Array-index keys first, ascending; the rest in definition order (a stable sort).
        members.sort_by_key(|member| member.index.map_or((1, 0), |index| (0, index)));
        out.put(b"{");
        for (position, member) in members.iter().enumerate() {
            if position > 0 {
                out.put(b",");
            }
            write_string(&member.key, out);
            out.put(b":");
            self.value(member.value.0, out)?;
        }
        out.put(b"}");
        Ok(end)
    }
}

/// `JSON.stringify` of a string (serde_json's escape table is the same).
fn write_string<S: Sink>(text: &str, out: &mut S) {
    struct Adapter<'s, S: Sink>(&'s mut S);
    impl<S: Sink> std::io::Write for Adapter<'_, S> {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.put(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    serde_json::to_writer(Adapter(out), text).expect("strings always encode as JSON");
}

#[cfg(test)]
#[path = "result_js_text_tests.rs"]
pub(crate) mod tests;
