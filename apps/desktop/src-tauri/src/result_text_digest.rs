//! The attestation digest of a JSON text, streamed from the text (RT-U4;
//! DEL-08-01 Design `report-user-release.md` §2.4): the worker's `d_S`
//! without materializing the whole result value.
//!
//! The meaning, unchanged from the former parse-back: for a domain `D`,
//!
//! ```text
//! f(text) = serde_json::from_str::<Value>(text).ok()
//!               .and_then(|value| streamed_digest(D, &value))
//! ```
//!
//! under this crate's serde_json features (`cargo tree -e features`:
//! `float_roundtrip` and `raw_value` on; `preserve_order` and
//! `arbitrary_precision` off in every crate of the build; `unbounded_depth`,
//! enabled by `cargo_metadata`, only adds an opt-in method that no parse here
//! calls, so `from_str` keeps its recursion limit), where
//! [`crate::solve_attestation::streamed_digest`] is the SHA-256
//! of `checked_v1_domain_canonical(D, DomainPayload::Value(&value), …)`, or
//! `None` when that refuses. [`text_digest`] computes exactly `f`: the same
//! `Some(digest)` for every text, and `None` exactly when `f` gives `None`.
//! What `f` does, spelled out:
//! - Parse: invalid JSON, trailing characters (other than JSON whitespace)
//!   and a number out of range give `None`. Objects are `BTreeMap`s: a key
//!   met again replaces the value, so the last duplicate wins, and an earlier
//!   duplicate is parsed but neither validated nor written. serde_json's
//!   recursion limit refuses the 128th nested container of the text,
//!   counting the root as the first and counting discarded duplicates too.
//!   With `raw_value` on, an object whose first key (decoded) is serde_json's
//!   private RawValue token is parsed as the JSON text held in its string
//!   value, by a fresh parse with its own limit; any other member, or a value
//!   that is not a string, is an error.
//! - Digest: the canonical writer refuses a value holding a container at
//!   level 128 of the wrapper `{"domain": D, "payload": value}` (the payload
//!   is level 2, so a 127th nested container of the value); the checked
//!   validation refuses an integer outside ±(2^53 − 1) and an integral float
//!   of magnitude above 2^53 − 1 (integers inside that range keep their exact
//!   rendering; the crate's documented divergence above it never arises
//!   because such integers are refused). Otherwise the digest is over the
//!   wrapper's canonical text: members sorted by the UTF-16 code units of the
//!   decoded keys, strings escaped as `JSON.stringify`, numbers in shortest
//!   round-trip form with `-0` as `0`, no whitespace.
//!
//! How it is computed (the design): the text is walked over borrowed slices.
//! A value whose slice is at most [`SPLIT_THRESHOLD_BYTES`] long, or that is
//! not a container, is a leaf: its slice is parsed into a `Value`, which is
//! validated and written by the canonical crate's own
//! `checked_v1_domain_canonical` into a sink that drops that call's wrapper
//! framing ([`PayloadOnly`]). A larger array is parsed element by element as
//! `&RawValue` slices and each element is walked as it arrives (nothing is
//! collected); a larger object is parsed into its members as (decoded key,
//! `&RawValue`) pairs, sorted in canonical order, and its kept members are
//! walked in that order. This writer frames the split containers and the
//! digest's wrapper itself. The largest value held at once is one leaf, plus
//! the member list of each split object on the open path.
//!
//! The canonical crate is used only through its existing public functions:
//! its writer is bound by the retained pricing's S6-HS guard (DEL-04-08
//! `retained-pricing-rules.md`), so this digest adds nothing there. The three
//! pieces of framing written here repeat the crate's private one-liners, and
//! the tests check them against it: a string (key or domain) is
//! `serde_json::to_string` of it (`write_string`), keys sort by
//! `encode_utf16()` (`sorted_keys`), and a container is refused at wrapper
//! level 128 (`too_deep` with serde_json's parse depth).
//!
//! Why the result equals `f`, rule by rule:
//! - Syntax. Every byte of the text is parsed by serde_json's own parser
//!   with the same grammar: the structure of a split container (brackets,
//!   commas, colons, whitespace, trailing-comma and end-of-input errors) by
//!   its `SeqAccess`/`MapAccess`, each key by the same string parse a `Value`
//!   key takes, and each leaf, kept or discarded, by `from_str::<Value>`.
//!   Capturing a `&RawValue` only skips a value with a looser check (no depth
//!   limit, no number range, no surrogate pairing), and the slice it skips is
//!   then parsed in full, so it rejects nothing the full parse accepts. The
//!   top-level text keeps its leading and trailing whitespace and is checked
//!   to its end.
//! - Duplicate keys. A split object's members are sorted by canonical key
//!   order and then by position in the text (a `&RawValue` borrows the text,
//!   so a later member has a larger address); UTF-16 order is `Equal` only
//!   for equal keys, so each key's members are adjacent and the
//!   last of them is the one `f`'s map keeps. The others are walked in
//!   discard mode: parsed and depth-checked as `f` parses them, but neither
//!   validated nor written.
//! - Key order and escaping. Keys are the decoded strings `f`'s map holds,
//!   ordered by their UTF-16 code units and written as `serde_json::to_string`
//!   writes them, as `checked_v1_domain_canonical` orders and writes keys;
//!   leaves are written by the crate's own writer.
//! - Numbers (`-0`, shortest round-trip, integers past 2^53). Each number is
//!   parsed by serde_json from its own characters, which context cannot
//!   change, and rendered and validated by the crate as before.
//! - Validation. A kept leaf is validated by the crate's own call; the
//!   whole value validates exactly when every kept leaf does, since the check
//!   is per number and `f` validates only the kept members. Any refusal
//!   anywhere gives `None`.
//! - Depth. A value at nesting `L` (it has `L` enclosing containers) is
//!   parsed at its true depth: a leaf container's slice is parsed inside `L`
//!   added `[`…`]` pairs (one-element arrays, unwrapped after), so
//!   serde_json's own counter applies the text limit with the enclosing
//!   containers counted, including inside a RawValue-token object (whose
//!   inner parse is fresh in both). A kept leaf sits at wrapper level `2 + L`
//!   and is refused when it holds a container at level 128 or deeper there,
//!   exactly when the whole value would be refused (the crate's own call
//!   counts from level 2, a weaker check this one implies). A split container
//!   at nesting `L + 1` is refused when it is kept and its level `2 + L`
//!   reaches 128, or when it is discarded and `L + 1` passes the parse's 127
//!   containers.
//! - The RawValue token. A split object whose first key is the token is not
//!   split: it is parsed whole as a leaf, so `f`'s special parse applies to it.
//!   A leaf's own objects meet the same special parse inside `from_str`.
//! - Order of failure does not matter: any failure gives `None`, and a
//!   partly fed hasher is dropped.
//!
//! Differential tests below (`g == f` on edge corpora at several thresholds,
//! including 0, which splits every container) and the real-envelope checks
//! recorded with RT-U4 back this argument.

use open_pipe_stress_canonical_json::{checked_v1_domain_canonical, ByteSink, CanonicalSink, DomainPayload};
use serde::de::{Deserializer as _, Error as _, MapAccess, SeqAccess, Visitor};
use serde_json::value::RawValue;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::cell::Cell;
use std::fmt;

/// The longest slice parsed as one leaf; longer arrays and objects are split.
/// One leaf's value is the largest value the digest holds.
pub(crate) const SPLIT_THRESHOLD_BYTES: usize = 256 << 10;

/// The most containers serde_json's parse nests (its recursion limit refuses
/// the 128th).
const PARSE_NESTING_LIMIT: usize = 127;

/// The nesting level of the payload in `checked_v1_domain_canonical`'s
/// wrapper (the wrapper object is level 1).
const PAYLOAD_LEVEL: usize = 2;

/// The wrapper level at which `checked_v1_domain_canonical` refuses a
/// container: serde_json's parse depth (128) over the serialized wrapper.
const REFUSED_LEVEL: usize = 128;

/// What `checked_v1_domain_canonical("", …)` writes before the payload.
const LEAF_PREFIX: &str = "{\"domain\":\"\",\"payload\":";

/// serde_json's private RawValue token: with `raw_value` on, an object whose
/// first key is this string is parsed as the JSON text in its value.
const RAW_VALUE_TOKEN: &str = "$serde_json::private::RawValue";

/// `f(text)` for `domain` (see the module docs), computed from the text.
pub(crate) fn text_digest(domain: &str, text: &str) -> Option<[u8; 32]> {
    text_digest_split_above(domain, text, SPLIT_THRESHOLD_BYTES)
}

/// [`text_digest`] with a chosen split threshold (tests use small ones so that
/// every path is split).
pub(crate) fn text_digest_split_above(domain: &str, text: &str, threshold: usize) -> Option<[u8; 32]> {
    let mut hasher = Sha256::new();
    let mut sink = ByteSink(|bytes: &[u8]| hasher.update(bytes));
    // `checked_v1_domain_canonical`'s wrapper, in member order: "domain" < "payload".
    sink.put("{\"domain\":");
    write_string(domain, &mut sink);
    sink.put(",\"payload\":");
    Walk { threshold, sink: &mut sink }.value(text, 0, true).ok()?;
    sink.put("}");
    Some(hasher.finalize().into())
}

/// Any refusal: invalid JSON, a depth limit or the checked validation.
#[derive(Debug)]
struct Refused;

struct Walk<'s, S: CanonicalSink> {
    threshold: usize,
    sink: &'s mut S,
}

impl<S: CanonicalSink> Walk<'_, S> {
    /// One value's slice at nesting `depth`; written when `keep`, otherwise
    /// only parsed (an earlier duplicate member, or inside one).
    fn value(&mut self, text: &str, depth: usize, keep: bool) -> Result<(), Refused> {
        if text.len() > self.threshold {
            match text.trim_start_matches([' ', '\n', '\t', '\r']).as_bytes().first() {
                Some(b'[') => return self.array(text, depth, keep),
                Some(b'{') => {
                    if let Some(result) = self.object(text, depth, keep) {
                        return result;
                    }
                }
                _ => {}
            }
        }
        self.leaf(text, depth, keep)
    }

    /// A split container's own nesting `depth + 1`: kept, the canonical
    /// writer's level must stay below its refused level; discarded, the parse
    /// limit applies.
    fn container_admitted(depth: usize, keep: bool) -> Result<(), Refused> {
        let admitted = if keep {
            PAYLOAD_LEVEL + depth < REFUSED_LEVEL
        } else {
            depth < PARSE_NESTING_LIMIT
        };
        admitted.then_some(()).ok_or(Refused)
    }

    fn leaf(&mut self, text: &str, depth: usize, keep: bool) -> Result<(), Refused> {
        let value = parse_at_depth(text, depth)?;
        if keep {
            if holds_refused_container(&value, PAYLOAD_LEVEL + depth) {
                return Err(Refused);
            }
            let mut payload = PayloadOnly { sink: &mut *self.sink, skip: LEAF_PREFIX.len(), held: None };
            checked_v1_domain_canonical("", DomainPayload::Value(&value), &mut payload).map_err(|_| Refused)?;
            debug_assert!(payload.skip == 0 && payload.held == Some('}'), "the wrapper's framing was dropped");
        }
        Ok(())
    }

    /// A split array: each element walked as it is parsed.
    fn array(&mut self, text: &str, depth: usize, keep: bool) -> Result<(), Refused> {
        Self::container_admitted(depth, keep)?;
        struct Elements<'w, 's, S: CanonicalSink> {
            walk: &'w mut Walk<'s, S>,
            depth: usize,
            keep: bool,
        }
        impl<'de, S: CanonicalSink> Visitor<'de> for Elements<'_, '_, S> {
            type Value = ();
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an array")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
                let mut first = true;
                while let Some(element) = seq.next_element::<&'de RawValue>()? {
                    if self.keep && !first {
                        self.walk.sink.put(",");
                    }
                    first = false;
                    self.walk.value(element.get(), self.depth + 1, self.keep).map_err(|_| A::Error::custom("refused"))?;
                }
                Ok(())
            }
        }
        if keep {
            self.sink.put("[");
        }
        let mut parser = serde_json::Deserializer::from_str(text);
        parser.deserialize_seq(Elements { walk: self, depth, keep }).map_err(|_| Refused)?;
        parser.end().map_err(|_| Refused)?;
        if keep {
            self.sink.put("]");
        }
        Ok(())
    }

    /// A split object: its members collected, sorted and walked, the last of
    /// each key kept. `None` when its first key is the RawValue token, so
    /// that the caller parses it whole.
    fn object(&mut self, text: &str, depth: usize, keep: bool) -> Option<Result<(), Refused>> {
        let token = Cell::new(false);
        let members = members(text, &token);
        if token.get() {
            return None;
        }
        Some(self.members(members, depth, keep))
    }

    fn members(&mut self, members: Result<Vec<Member<'_>>, Refused>, depth: usize, keep: bool) -> Result<(), Refused> {
        let mut members = members?;
        Self::container_admitted(depth, keep)?;
        // Canonical key order, then text order (a later member borrows a later slice).
        members.sort_unstable_by(|(left, left_value), (right, right_value)| {
            canonical_key_order(left, right).then(left_value.get().as_ptr().cmp(&right_value.get().as_ptr()))
        });
        if keep {
            self.sink.put("{");
        }
        let mut first = true;
        for (index, (key, value)) in members.iter().enumerate() {
            let last_of_key = members.get(index + 1).is_none_or(|(next, _)| next != key);
            let kept = keep && last_of_key;
            if kept {
                if !first {
                    self.sink.put(",");
                }
                first = false;
                write_string(key, self.sink);
                self.sink.put(":");
            }
            self.value(value.get(), depth + 1, kept)?;
        }
        if keep {
            self.sink.put("}");
        }
        Ok(())
    }
}

/// The sink a leaf's `checked_v1_domain_canonical("", leaf, …)` writes into:
/// it drops the wrapper's framing, the first `LEAF_PREFIX.len()` bytes and the
/// final `}`, and passes the payload's text on unchanged. The last character
/// seen is held back until another arrives, so the one left at the end is the
/// wrapper's `}`.
struct PayloadOnly<'s, S: CanonicalSink> {
    sink: &'s mut S,
    skip: usize,
    held: Option<char>,
}

impl<S: CanonicalSink> CanonicalSink for PayloadOnly<'_, S> {
    fn put(&mut self, text: &str) {
        // The prefix is ASCII, so its end is a character boundary.
        let skipped = self.skip.min(text.len());
        self.skip -= skipped;
        let text = &text[skipped..];
        let Some(last) = text.chars().next_back() else { return };
        if let Some(held) = self.held.take() {
            self.sink.put(held.encode_utf8(&mut [0; 4]));
        }
        self.sink.put(&text[..text.len() - last.len_utf8()]);
        self.held = Some(last);
    }
}

/// A string as the canonical writer writes it (`write_string`: its escaping
/// is `JSON.stringify`'s).
fn write_string<S: CanonicalSink>(text: &str, sink: &mut S) {
    sink.put(&serde_json::to_string(text).expect("strings always encode as JSON"));
}

/// RFC 8785 §3.2.3 key order, as the canonical writer sorts keys: arrays of
/// UTF-16 code units. `Equal` only for equal keys.
fn canonical_key_order(left: &str, right: &str) -> std::cmp::Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

/// Whether `value`, placed at wrapper `level`, holds a container at
/// [`REFUSED_LEVEL`] or deeper (the canonical writer's depth refusal).
fn holds_refused_container(value: &Value, level: usize) -> bool {
    match value {
        Value::Array(items) => level >= REFUSED_LEVEL || items.iter().any(|item| holds_refused_container(item, level + 1)),
        Value::Object(map) => level >= REFUSED_LEVEL || map.values().any(|item| holds_refused_container(item, level + 1)),
        _ => false,
    }
}

/// One object member: its decoded key (borrowed when it has no escapes) and
/// its value's slice.
type Member<'t> = (Cow<'t, str>, &'t RawValue);

/// An object's members in text order, parsed by serde_json's `MapAccess` with
/// keys decoded as a `Value`'s keys are. Sets `token` (and refuses) when the
/// first key is the RawValue token.
fn members<'t>(text: &'t str, token: &Cell<bool>) -> Result<Vec<Member<'t>>, Refused> {
    struct Key;
    impl<'de> Visitor<'de> for Key {
        type Value = Cow<'de, str>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a key")
        }
        fn visit_borrowed_str<E>(self, key: &'de str) -> Result<Self::Value, E> {
            Ok(Cow::Borrowed(key))
        }
        fn visit_str<E>(self, key: &str) -> Result<Self::Value, E> {
            Ok(Cow::Owned(key.to_owned()))
        }
        fn visit_string<E>(self, key: String) -> Result<Self::Value, E> {
            Ok(Cow::Owned(key))
        }
    }
    impl<'de> serde::de::DeserializeSeed<'de> for Key {
        type Value = Cow<'de, str>;
        fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
            deserializer.deserialize_str(self)
        }
    }
    struct Members<'c> {
        token: &'c Cell<bool>,
    }
    impl<'de> Visitor<'de> for Members<'_> {
        type Value = Vec<Member<'de>>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("an object")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut members = Vec::new();
            while let Some(key) = map.next_key_seed(Key)? {
                if members.is_empty() && key == RAW_VALUE_TOKEN {
                    self.token.set(true);
                    return Err(A::Error::custom("RawValue token"));
                }
                members.push((key, map.next_value::<&'de RawValue>()?));
            }
            Ok(members)
        }
    }
    let mut parser = serde_json::Deserializer::from_str(text);
    let members = parser.deserialize_map(Members { token }).map_err(|_| Refused)?;
    parser.end().map_err(|_| Refused)?;
    Ok(members)
}

/// A leaf's slice parsed into a `Value` as `f`'s parse would parse it at
/// nesting `depth`: a container inside `depth` one-element arrays, so the
/// parser's recursion limit counts its enclosing containers; a scalar alone
/// (no depth to count).
fn parse_at_depth(text: &str, depth: usize) -> Result<Value, Refused> {
    let container = matches!(text.trim_start_matches([' ', '\n', '\t', '\r']).as_bytes().first(), Some(b'[' | b'{'));
    if depth == 0 || !container {
        return serde_json::from_str(text).map_err(|_| Refused);
    }
    let mut padded = String::with_capacity(text.len() + 2 * depth);
    padded.extend(std::iter::repeat_n('[', depth));
    padded.push_str(text);
    padded.extend(std::iter::repeat_n(']', depth));
    let mut value: Value = serde_json::from_str(&padded).map_err(|_| Refused)?;
    drop(padded);
    for _ in 0..depth {
        value = match value {
            Value::Array(mut items) if items.len() == 1 => items.pop().expect("one element"),
            _ => unreachable!("a padded value parses as one-element arrays"),
        };
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve_attestation::{streamed_digest, SOLVED_MODEL_DOMAIN, SOLVED_RESULT_DOMAIN};
    use serde_json::json;

    /// The meaning: the former parse-back.
    fn f(text: &str) -> Option<[u8; 32]> {
        serde_json::from_str::<Value>(text).ok().and_then(|value| streamed_digest(SOLVED_RESULT_DOMAIN, &value))
    }

    const THRESHOLDS: [usize; 6] = [0, 1, 8, 32, 200, SPLIT_THRESHOLD_BYTES];

    /// `g == f` at every threshold; returns whether `f` digested.
    fn same(text: &str) -> bool {
        let expected = f(text);
        for threshold in THRESHOLDS {
            assert_eq!(
                text_digest_split_above(SOLVED_RESULT_DOMAIN, text, threshold),
                expected,
                "threshold {threshold}, text {:?}",
                &text[..text.len().min(300)]
            );
        }
        assert_eq!(text_digest(SOLVED_RESULT_DOMAIN, text), expected);
        expected.is_some()
    }

    fn arrays(depth: usize, leaf: &str) -> String {
        format!("{}{leaf}{}", "[".repeat(depth), "]".repeat(depth))
    }

    fn objects(depth: usize, leaf: &str) -> String {
        format!("{}{leaf}{}", "{\"k\":".repeat(depth), "}".repeat(depth))
    }

    /// Nesting `depth` containers, alternating, with padding members so that
    /// small thresholds split every level (an empty padding container beside
    /// the innermost one makes `depth + 1` levels).
    fn mixed(depth: usize, leaf: &str) -> String {
        let mut text = leaf.to_string();
        for level in 0..depth {
            text = if level % 2 == 0 {
                format!("[1.5,\"pad\",{text},{{}}]")
            } else {
                format!("{{\"z\":[],\"a\":{text},\"m\":-0}}")
            };
        }
        text
    }

    /// The framing written here is the crate's: the leaf call's prefix, the
    /// payload left by `PayloadOnly`, string escaping, key order and the
    /// refused level.
    #[test]
    fn the_framing_is_the_canonical_writers() {
        use open_pipe_stress_canonical_json::canonical_json;
        let mut whole = String::new();
        checked_v1_domain_canonical("", DomainPayload::Value(&Value::Null), &mut whole).unwrap();
        assert_eq!(whole, format!("{LEAF_PREFIX}null}}"));
        let strings = ["", "a", "\u{0}\u{1f}\"\\/\u{7f}\u{2028}😀ｚé", "}", "\u{e000}", "\u{10000}"];
        let values = [
            json!(null), json!(-0.0), json!("}"), json!("😀"), json!([]), json!({}), json!([1, "}", {"}": "}"}]),
            json!({"\u{e000}": 1, "\u{10000}": [2.5, "é"], "a": {}}),
        ];
        for value in &values {
            let mut payload = String::new();
            let mut only = PayloadOnly { sink: &mut payload, skip: LEAF_PREFIX.len(), held: None };
            checked_v1_domain_canonical("", DomainPayload::Value(value), &mut only).unwrap();
            assert_eq!((only.skip, only.held), (0, Some('}')));
            assert_eq!(payload, canonical_json(value), "{value}");
        }
        for text in strings {
            let mut written = String::new();
            write_string(text, &mut written);
            assert_eq!(written, canonical_json(&json!(text)));
            for other in strings {
                let pair = canonical_json(&json!({ text: 0, other: 1 }));
                let first = if text == other || canonical_key_order(text, other).is_lt() { text } else { other };
                assert!(pair.starts_with(&format!("{{{}:", canonical_json(&json!(first)))), "{pair}");
                assert_eq!(canonical_key_order(text, other).is_eq(), text == other);
            }
        }
        let mut deep = json!(0);
        for nesting in 1..=128 {
            deep = json!([deep]);
            let refused = streamed_digest(SOLVED_RESULT_DOMAIN, &deep).is_none();
            assert_eq!(refused, holds_refused_container(&deep, PAYLOAD_LEVEL), "{nesting}");
            assert_eq!(refused, nesting + 1 >= REFUSED_LEVEL, "{nesting}");
        }
    }

    #[test]
    fn numbers_at_the_edges() {
        let numbers = [
            "0", "-0", "0.0", "-0.0", "1", "-1", "1e-7", "1e21", "1E21", "1e+21", "5e-324", "-5e-324", "1.7976931348623157e308",
            "-1.7976931348623157e308", "2.2250738585072014e-308", "9007199254740991", "-9007199254740991", "9007199254740992",
            "-9007199254740992", "9007199254740993", "9007199254740991.0", "9007199254740992.0", "9007199254740993.5",
            "4503599627370495.5", "18446744073709551615", "18446744073709551616", "-9223372036854775808",
            "-9223372036854775809", "9223372036854775807", "123456789.987654", "333333333.3333332", "0.1", "1e400", "-1e400",
            "1e-400", "100000000000000000000000000000", "0.30000000000000004", "4.0e15", "1.5e-300", "200.0", "1e9",
        ];
        let mut digested = 0;
        for number in numbers {
            for text in [
                number.to_string(),
                format!("[{number}]"),
                format!("{{\"n\":{number}}}"),
                format!("[1,{{\"a\":[{number},2]}},{number}]"),
                format!(" \n\t\r{number}\r\n "),
            ] {
                digested += usize::from(same(&text));
            }
        }
        assert!(digested > 100, "{digested}");
        // Refused by the checked validation, so no digest: unsafe integers and integral floats.
        for refused in ["9007199254740992", "-9007199254740992", "18446744073709551615", "-9223372036854775808", "1e21", "1.7976931348623157e308"] {
            assert!(!same(&format!("[0,{refused}]")), "{refused}");
        }
        // Inside the checked envelope: digested, with -0 written as 0.
        for admitted in ["-0", "-0.0", "1e-7", "5e-324", "9007199254740991", "-9007199254740991"] {
            assert!(same(&format!("[0,{admitted}]")), "{admitted}");
        }
    }

    #[test]
    fn strings_keys_and_ordering() {
        let strings = [
            r#""""#, r#""a""#, r#""\"""#, r#""\\""#, r#""\/""#, r#""\u0000""#, r#""\u001f""#, r#""\b\f\n\r\t""#,
            "\"\u{7f}\u{80}é€\"", "\"😀\"", r#""\ud83d\ude00""#, r#""\uD83D\uDE00""#, "\"\u{e000}\"", "\"\u{10000}\"",
            "\"\u{10ffff}\"", r#""\u2028\u2029""#, "\"e\u{301}\"", r#""\ud800""#, r#""\udc00""#, r#""\ud800x""#,
            "\"raw\u{1}control\"", "\"tab\tinside\"", r#""bad \x escape""#, r#""\u12""#, "\"unterminated",
        ];
        for value in strings {
            same(value);
            same(&format!("[{value},{value}]"));
            for key in strings {
                same(&format!("{{{key}:{value},\"b\":1}}"));
                same(&format!("{{\"z\":{{{key}:[{value}]}},{key}:0}}"));
            }
        }
        // UTF-16 order differs from code-point order: U+10000 (D800 DC00) sorts before U+E000.
        assert!(same("{\"\u{e000}\":1,\"\u{10000}\":2,\"\u{ff5a}\":3,\"\u{1f600}\":4,\"a\":5,\"\\u0061b\":6}"));
        // An escaped key equals its unescaped form: a duplicate.
        assert!(same(r#"{"a":1,"\u0061":2}"#));
    }

    #[test]
    fn duplicate_keys_last_wins_top_level_and_nested() {
        let cases = [
            r#"{"a":1,"a":2}"#,
            r#"{"a":1,"b":0,"a":2,"a":3}"#,
            r#"{"a":{"x":1,"x":[2,{"y":1,"y":2}]},"a":{"x":3}}"#,
            r#"[{"k":1,"k":2},{"k":[1,2],"j":0,"k":{"k":1,"k":-0}}]"#,
            // An earlier duplicate's refused number is discarded, not validated.
            r#"{"a":9007199254740993,"a":1}"#,
            r#"{"a":[1e300],"a":1}"#,
            // A later refused duplicate refuses.
            r#"{"a":1,"a":9007199254740993}"#,
            // An earlier duplicate must still parse.
            r#"{"a":1e400,"a":1}"#,
            r#"{"a":"\ud800","a":1}"#,
            r#"{"a":[1,],"a":1}"#,
            r#"{"\u0061":[1,2,3],"a":{}}"#,
        ];
        let expected = [true, true, true, true, true, true, false, false, false, false, true];
        for (text, digested) in cases.iter().zip(expected) {
            assert_eq!(same(text), digested, "{text}");
        }
    }

    #[test]
    fn empty_containers_whitespace_and_invalid_text() {
        let digested = [
            "{}", "[]", " {} ", "\n[ ]\t", "[{},[],{\"\":{}}]", "{ \"a\" : [ 1 , 2 ] , \"b\" : { } }", "null", "true", "\"s\"",
            "[\n\n1\r,\t2 ]",
        ];
        for text in digested {
            assert!(same(text), "{text}");
        }
        let invalid = [
            "", " ", "{", "[", "]", "}", "{\"a\"}", "{\"a\":}", "{\"a\":1,}", "[1,]", "[1 2]", "{\"a\":1 \"b\":2}", "{1:2}",
            "{} {}", "[] x", "[]]", "{}}", "nul", "tru", "01", "1.", ".5", "-", "+1", "1e", "[1e+]", "NaN", "Infinity",
            "[1]\u{0}", "\u{feff}[]", "[\u{a0}1]", "'a'", "{\"a\":1}//", "[1]\u{b}",
        ];
        for text in invalid {
            assert!(!same(text), "{text:?}");
        }
    }

    /// The text limit (127 containers parse, 128 do not) and the canonical
    /// limit (126 nest under the wrapper, 127 do not), for arrays, objects and
    /// mixed padded containers that small thresholds split at every level,
    /// and inside a split container's discarded duplicate.
    #[test]
    fn nesting_at_the_limits() {
        for depth in 120..=131 {
            // (text, its containers nested)
            let nested = [
                (arrays(depth, "0"), depth),
                (objects(depth, "0"), depth),
                // The padding beside the innermost container is one deeper.
                (mixed(depth, "0"), depth + 1),
                (arrays(depth - 1, "[]"), depth),
                (objects(depth - 1, "{}"), depth),
            ];
            for (text, containers) in nested {
                assert_eq!(same(&text), containers <= 126, "{containers} containers: {}", &text[..40]);
            }
            // Inside a large split container, at its second member.
            let wide = format!("[{},{}]", "1,".repeat(300) + "0", mixed(depth - 1, "0"));
            assert_eq!(same(&wide), depth + 1 <= 126, "wide depth {depth}");
            // A discarded duplicate: parsed (the text limit applies) but not written (the canonical one does not).
            let discarded = format!("{{\"a\":{},\"a\":0}}", arrays(depth - 1, "0"));
            assert_eq!(same(&discarded), depth <= 127, "discarded depth {depth}");
            let discarded_split = format!("{{\"a\":{},\"a\":0}}", mixed(depth - 1, "0"));
            assert_eq!(same(&discarded_split), depth + 1 <= 127, "discarded split depth {depth}");
        }
    }

    /// With `raw_value` on, an object whose first key is the RawValue token is
    /// the value parsed from its string (a fresh parse with its own depth).
    #[test]
    fn the_raw_value_token_object() {
        let token = RAW_VALUE_TOKEN;
        let cases = [
            format!(r#"{{"{token}":"[1,2,{{\"b\":1,\"a\":2}}]"}}"#),
            format!(r#"[0,{{"{token}":"{{\"x\":-0.0}}"}}]"#),
            format!(r#"{{"k":{{"{token}":"9007199254740993"}}}}"#),
            format!(r#"{{"{token}":"[1"}}"#),
            format!(r#"{{"{token}":5}}"#),
            format!(r#"{{"{token}":"1","b":2}}"#),
            format!(r#"{{"b":2,"{token}":"1"}}"#),
            format!(r#"{{"\u0024serde_json::private::RawValue":"[true]"}}"#),
            format!(r#"[{},{{"{token}":"{}"}}]"#, "7,".repeat(100) + "7", arrays(126, "0").replace('"', "\\\"")),
            format!(r#"{{"{token}":"{}"}}"#, arrays(127, "0")),
            format!(r#"{{"{token}":"{}"}}"#, arrays(128, "0")),
        ];
        let mut digested = 0;
        for text in &cases {
            digested += usize::from(same(text));
            for depth in [0, 1, 60, 125, 126, 127] {
                same(&arrays(depth, text));
                same(&mixed(depth, text));
            }
        }
        assert!(digested >= 4, "{digested}");
    }

    /// Deterministic random documents: every kind of value and key, duplicate
    /// keys, refused numbers, whitespace, and sometimes a corrupted byte.
    #[test]
    fn random_documents_alike() {
        let mut x: u64 = 0x2545_f491_4f6c_dd1d;
        let mut next = move |bound: u64| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x % bound
        };
        const KEYS: [&str; 8] = ["a", "b", "\\u0061", "\u{e000}", "\u{10000}", "é", "\\n", ""];
        const SCALARS: [&str; 14] = [
            "0", "-0", "-0.0", "1.5", "1e-7", "1e21", "9007199254740993", "-9007199254740991", "5e-324", "null", "true",
            "\"s\\u0000\"", "\"😀\"", "123456789.987654",
        ];
        const SPACE: [&str; 4] = ["", " ", "\n", "\t\r"];
        fn document(next: &mut impl FnMut(u64) -> u64, depth: usize, out: &mut String) {
            let space = SPACE[next(4) as usize];
            out.push_str(space);
            match if depth > 6 { 2 } else { next(4) } {
                0 => {
                    out.push('[');
                    for index in 0..next(5) {
                        if index > 0 {
                            out.push(',');
                        }
                        document(next, depth + 1, out);
                    }
                    out.push(']');
                }
                1 => {
                    out.push('{');
                    for index in 0..next(5) {
                        if index > 0 {
                            out.push(',');
                        }
                        out.push('"');
                        out.push_str(KEYS[next(8) as usize]);
                        out.push_str("\":");
                        document(next, depth + 1, out);
                    }
                    out.push('}');
                }
                _ => out.push_str(SCALARS[next(14) as usize]),
            }
            out.push_str(space);
        }
        let (mut digested, mut refused, mut invalid) = (0, 0, 0);
        for round in 0..3000 {
            let mut text = String::new();
            document(&mut next, 0, &mut text);
            if round % 5 == 4 && !text.is_empty() {
                let at = next(text.len() as u64) as usize;
                if text.is_char_boundary(at) && text.is_char_boundary(at + 1) {
                    text.replace_range(at..at + 1, ["]", "}", ",", ":", "\"", "x", ""][next(7) as usize]);
                }
            }
            match (serde_json::from_str::<Value>(&text).is_ok(), same(&text)) {
                (true, true) => digested += 1,
                (true, false) => refused += 1,
                (false, _) => invalid += 1,
            }
        }
        eprintln!("RESULT-TEXT-DIGEST random documents: {digested} digested, {refused} refused, {invalid} invalid");
        assert!(digested > 500 && refused > 100 && invalid > 100, "{digested} {refused} {invalid}");
    }

    /// The solved demo's result in both modes, and the domains separate.
    #[test]
    fn the_solved_demo_digests_alike() {
        let model = crate::load_preview_model().unwrap();
        for mode in [open_pipe_stress_product_physics::PreviewSolverMode::SparseInteractive, open_pipe_stress_product_physics::PreviewSolverMode::DenseScrutiny] {
            let text = crate::solve_preview_mechanics_with_mode(model.clone(), mode).unwrap();
            assert!(same(&text));
            let value: Value = serde_json::from_str(&text).unwrap();
            assert_eq!(text_digest(SOLVED_MODEL_DOMAIN, &text), streamed_digest(SOLVED_MODEL_DOMAIN, &value));
            assert_ne!(text_digest(SOLVED_MODEL_DOMAIN, &text), text_digest(SOLVED_RESULT_DOMAIN, &text));
        }
    }

    /// The real-envelope check (RT-U4): `g == f` on a result file named by
    /// `RESULT_TEXT_DIGEST_FILE`, and which of them `RESULT_TEXT_DIGEST_RUN`
    /// (`f`, `g` or `both`) runs. With `RESULT_TEXT_DIGEST_MODEL` set it first
    /// writes that file: the sparse_interactive solve of the model document,
    /// as the worker produces it. Ignored by default (its files are large and
    /// never committed).
    #[test]
    #[ignore]
    fn real_envelope_digests_alike() {
        let path = std::env::var("RESULT_TEXT_DIGEST_FILE").expect("RESULT_TEXT_DIGEST_FILE");
        if let Ok(model) = std::env::var("RESULT_TEXT_DIGEST_MODEL") {
            let model: Value = serde_json::from_slice(&std::fs::read(model).unwrap()).unwrap();
            let mode = open_pipe_stress_product_physics::PreviewSolverMode::SparseInteractive;
            std::fs::write(&path, crate::solve_preview_mechanics_with_mode(model, mode).unwrap()).unwrap();
            return;
        }
        let run = std::env::var("RESULT_TEXT_DIGEST_RUN").unwrap_or_else(|_| "both".into());
        let text = std::fs::read_to_string(&path).unwrap();
        // Functional observations of the process: its footprint with the text
        // held, then its lifetime maximum after each digest.
        eprintln!("RESULT-TEXT-DIGEST text held: phys_footprint {}", crate::worker_alloc::phys_footprint());
        let timed = |name: &str, digest: &dyn Fn(&str) -> Option<[u8; 32]>| {
            let start = std::time::Instant::now();
            let result = digest(&text);
            eprintln!(
                "RESULT-TEXT-DIGEST {name} on {} bytes: {:?} in {:.2?}; lifetime max phys_footprint {}",
                text.len(),
                result.map(hex),
                start.elapsed(),
                crate::worker_alloc::lifetime_max_footprint()
            );
            result
        };
        match run.as_str() {
            "f" => assert!(timed("f", &f).is_some()),
            "g" => assert!(timed("g", &|t| text_digest(SOLVED_RESULT_DOMAIN, t)).is_some()),
            _ => {
                let g = timed("g", &|t| text_digest(SOLVED_RESULT_DOMAIN, t));
                assert!(g.is_some());
                assert_eq!(g, timed("f", &f));
            }
        }
    }

    fn hex(digest: [u8; 32]) -> String {
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}
