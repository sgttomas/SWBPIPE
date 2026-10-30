//! RFC 8785 (JCS) canonical JSON serialization for OpenPipeStress hash seams.
//!
//! Single canonical-JSON text authority (completion-plan hardening row H5,
//! aligning the DEC-010 "JCS-compatible hashing" baseline): every surface
//! that hashes JSON text — the editor-operation engine, the wasm-routed
//! frontend hash service, and the headless runner checksums — renders
//! through this crate so identical values produce identical canonical bytes
//! regardless of which lane (JS `JSON.stringify` transport, raw-file serde
//! parse, engine-internal serde value) produced the JSON.
//!
//! Conformance to RFC 8785:
//! - object keys sorted by UTF-16 code units (§3.2.3);
//! - strings escaped exactly as ECMAScript `JSON.stringify` (§3.2.2.2;
//!   delegated to `serde_json`, whose escape table is identical);
//! - numbers rendered with the ECMAScript `Number::toString` shortest
//!   round-trip decimal form (§3.2.2.3), with `-0` rendered as `0`; digit
//!   selection comes from `ryu` (closest shortest candidate, exact ties to
//!   even — the same rule ECMAScript engines apply);
//! - arrays in order, no insignificant whitespace, UTF-8 output.
//!
//! One deliberate, documented divergence: `serde_json` preserves i64/u64
//! integers exactly, so integers beyond 2^53 keep their full precision here
//! instead of collapsing to the nearest IEEE double. Such values sit outside
//! the I-JSON interoperability envelope RFC 8785 assumes, and they cannot
//! arrive distinct from their rounded double through any JavaScript lane
//! (`JSON.parse` rounds first) — see `fixtures/canonical_hash/README.md`.

pub mod binary64;
pub mod binary64_cli;

use serde_json::{Map, Number, Value};

pub const CHECKED_PROFILE_V1: &str = "openpipestress_jcs_ijson_v1";
pub const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;

/// Parse and canonicalize an I-JSON-compatible document for versioned result
/// records. This is additive: `canonical_json` deliberately retains its
/// historical, unrestricted `serde_json::Value` behavior.
pub fn canonical_json_checked_v1_text(input: &str) -> Result<String, String> {
    let value = checked_parse(input)?;
    validate_checked_value(&value)?;
    Ok(canonical_json(&value))
}

fn checked_parse(input: &str) -> Result<Value, String> {
    use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
    use std::collections::HashSet;
    use std::fmt;

    struct Seed;
    struct ValueVisitor;
    impl<'de> DeserializeSeed<'de> for Seed {
        type Value = Value;
        fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
            d.deserialize_any(ValueVisitor)
        }
    }
    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = Value;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("a JSON value") }
        fn visit_bool<E>(self, v: bool) -> Result<Value, E> { Ok(Value::Bool(v)) }
        fn visit_i64<E>(self, v: i64) -> Result<Value, E> { Ok(Value::Number(v.into())) }
        fn visit_u64<E>(self, v: u64) -> Result<Value, E> { Ok(Value::Number(v.into())) }
        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Value, E> {
            Number::from_f64(v).map(Value::Number).ok_or_else(|| E::custom("non-finite number"))
        }
        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Value, E> { Ok(Value::String(v.to_owned())) }
        fn visit_string<E>(self, v: String) -> Result<Value, E> { Ok(Value::String(v)) }
        fn visit_none<E>(self) -> Result<Value, E> { Ok(Value::Null) }
        fn visit_unit<E>(self) -> Result<Value, E> { Ok(Value::Null) }
        fn visit_some<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> { Seed.deserialize(d) }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element_seed(Seed)? { values.push(value); }
            Ok(Value::Array(values))
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
            let mut seen = HashSet::new();
            let mut values = serde_json::Map::new();
            while let Some(key) = map.next_key::<String>()? {
                if !seen.insert(key.clone()) { return Err(A::Error::custom(format!("duplicate object member: {key}"))); }
                values.insert(key, map.next_value_seed(Seed)?);
            }
            Ok(Value::Object(values))
        }
    }

    let mut deserializer = serde_json::Deserializer::from_str(input);
    let value = Seed.deserialize(&mut deserializer).map_err(|e| format!("CHECKED-JSON-INVALID: {e}"))?;
    deserializer.end().map_err(|e| format!("CHECKED-JSON-INVALID: {e}"))?;
    Ok(value)
}

fn validate_checked_value(value: &Value) -> Result<(), String> {
    match value {
        Value::Number(number) => {
            if let Some(v) = number.as_i64() {
                if !(-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&v) {
                    return Err(format!("CHECKED-JSON-UNSAFE-INTEGER: {v}"));
                }
            } else if let Some(v) = number.as_u64() {
                if v > MAX_SAFE_INTEGER as u64 { return Err(format!("CHECKED-JSON-UNSAFE-INTEGER: {v}")); }
            } else {
                let v = number.as_f64().ok_or_else(|| "CHECKED-JSON-NUMBER-INVALID".to_string())?;
                if !v.is_finite() { return Err("CHECKED-JSON-NON-FINITE".to_string()); }
                if v.fract() == 0.0 && v.abs() > MAX_SAFE_INTEGER as f64 {
                    return Err(format!("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT: {v}"));
                }
            }
        }
        Value::Array(values) => for item in values { validate_checked_value(item)?; },
        Value::Object(values) => for item in values.values() { validate_checked_value(item)?; },
        _ => {}
    }
    Ok(())
}

/// Render a JSON value as RFC 8785 canonical JSON text.
pub fn canonical_json(value: &Value) -> String {
    let mut out = String::new();
    write_canonical(value, &mut out);
    out
}

/// A consumer of canonical JSON text, written in order as UTF-8 pieces.
/// `String` collects the text; [`ByteSink`] feeds the bytes to a closure (for
/// example a SHA-256 hasher's `update`), so a digest never holds the text.
/// Every writer here emits the same pieces in the same order whatever the
/// sink, so the bytes a sink receives are exactly the text a `String` sink
/// would hold.
pub trait CanonicalSink {
    fn put(&mut self, text: &str);
}

impl CanonicalSink for String {
    fn put(&mut self, text: &str) {
        self.push_str(text);
    }
}

/// Adapts a byte consumer to a [`CanonicalSink`].
pub struct ByteSink<F: FnMut(&[u8])>(pub F);

impl<F: FnMut(&[u8])> CanonicalSink for ByteSink<F> {
    fn put(&mut self, text: &str) {
        (self.0)(text.as_bytes())
    }
}

/// The payload of a domain-separated hash, borrowed: a whole value, or an
/// object without one of its top-level members (a publication without its
/// receipt), so that no copy of the payload is made to hash it.
#[derive(Clone, Copy, Debug)]
pub enum DomainPayload<'a> {
    Value(&'a Value),
    ObjectWithout(&'a Map<String, Value>, &'a str),
}

/// Writes or refuses exactly as
/// `canonical_json_checked_v1_text(&serde_json::to_string(&json!({"domain": domain, "payload": payload}))?)`
/// does, where an `ObjectWithout(map, omit)` payload is `map`'s object less
/// its `omit` member, but over the borrowed values: no wrapper `Value`, no
/// serialized text and no re-parse. The text goes to `sink` (a `String` for
/// the text itself, a [`ByteSink`] over a hasher for a streamed digest);
/// nothing is written when the call refuses.
///
/// Why the result is the same (T4-S6; checked against the former path by
/// `tests/checked_domain.rs` and the callers' corpus tests):
/// - Serializing a `Value` cannot fail, and the checked parse of that text
///   rebuilds the same `Value`: a `Map` holds each key once, so the
///   duplicate-member refusal cannot arise; a `Number` holds an i64, a u64 or
///   a finite f64 (`Number::from_f64` refuses NaN and infinities), so the
///   non-finite refusal cannot arise; serde_json writes an f64 as its
///   shortest round-trip digits and, with `float_roundtrip` (enabled by this
///   crate, so in every build that links it), parses them back to the same
///   bits; integers and strings round-trip exactly. No build enables
///   `arbitrary_precision` or `preserve_order` (T4-S6 checked the feature
///   graph), so maps iterate in key order in both values.
/// - The one parse refusal a `Value` can still meet is serde_json's recursion
///   limit: its parser starts with 128 levels and refuses the container that
///   would take the last, i.e. the first container (in text order) nested 128
///   deep, counting the wrapper object as the first. [`recursion_refusal`]
///   reproduces that message, including its column, from the borrowed values.
/// - The validation then sees the same values in the same order, so it
///   refuses with the same first message; the canonical text depends only on
///   the values.
pub fn checked_v1_domain_canonical<S: CanonicalSink>(
    domain: &str,
    payload: DomainPayload<'_>,
    sink: &mut S,
) -> Result<(), String> {
    if let Some(refusal) = recursion_refusal(domain, payload) {
        return Err(refusal);
    }
    match payload {
        DomainPayload::Value(value) => validate_checked_value(value)?,
        DomainPayload::ObjectWithout(map, omit) => {
            for (key, value) in map {
                if key != omit {
                    validate_checked_value(value)?;
                }
            }
        }
    }
    // RFC 8785 member order of the wrapper: "domain" < "payload".
    sink.put("{\"domain\":");
    write_string(domain, sink);
    sink.put(",\"payload\":");
    match payload {
        DomainPayload::Value(value) => write_canonical(value, sink),
        DomainPayload::ObjectWithout(map, omit) => write_object_without(map, omit, sink),
    }
    sink.put("}");
    Ok(())
}

/// serde_json's parser depth: `Deserializer` starts `remaining_depth` at 128
/// and refuses a `[` or `{` whose decrement reaches 0.
const PARSE_DEPTH_LIMIT: usize = 128;

/// The checked parse's refusal of the serialized wrapper for depth, if any:
/// `CHECKED-JSON-INVALID: recursion limit exceeded at line 1 column N`, where
/// column N is the 1-based byte position, in the compact serialized text, of
/// the first container nested at the limit (serde_json reports the peeked
/// byte; the compact text has no newline, since strings escape it).
fn recursion_refusal(domain: &str, payload: DomainPayload<'_>) -> Option<String> {
    let deep = match payload {
        DomainPayload::Value(value) => too_deep(value, 2),
        DomainPayload::ObjectWithout(map, omit) => {
            map.iter().any(|(key, value)| key != omit && too_deep(value, 3))
        }
    };
    if !deep {
        return None;
    }
    // `{"domain":` + the serialized domain + `,"payload":`, then the payload
    // in serialization (map) order.
    let mut position = 10 + serialized_len(domain) + 11;
    let found = match payload {
        DomainPayload::Value(value) => first_too_deep(value, 2, &mut position),
        DomainPayload::ObjectWithout(map, omit) => {
            position += 1;
            first_too_deep_in(map.iter().filter(|(key, _)| *key != omit), 3, &mut position)
        }
    };
    let index = found.expect("a container past the parse depth is located in serialization order");
    Some(format!(
        "CHECKED-JSON-INVALID: recursion limit exceeded at line 1 column {}",
        index + 1
    ))
}

/// True when `value`, placed at nesting `level`, holds a container at or past
/// the parse depth. Never descends past the limit.
fn too_deep(value: &Value, level: usize) -> bool {
    match value {
        Value::Array(items) => level >= PARSE_DEPTH_LIMIT || items.iter().any(|item| too_deep(item, level + 1)),
        Value::Object(map) => level >= PARSE_DEPTH_LIMIT || map.values().any(|item| too_deep(item, level + 1)),
        _ => false,
    }
}

/// The byte index, in serde_json's compact serialization, of the first
/// container at or past the parse depth, advancing `position` over what
/// precedes it. The prefix it walks lies above the limit.
fn first_too_deep(value: &Value, level: usize, position: &mut usize) -> Option<usize> {
    match value {
        Value::Array(items) => {
            if level >= PARSE_DEPTH_LIMIT {
                return Some(*position);
            }
            *position += 1;
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    *position += 1;
                }
                if let Some(found) = first_too_deep(item, level + 1, position) {
                    return Some(found);
                }
            }
            *position += 1;
            None
        }
        Value::Object(map) => {
            if level >= PARSE_DEPTH_LIMIT {
                return Some(*position);
            }
            *position += 1;
            first_too_deep_in(map.iter(), level + 1, position)
        }
        scalar => {
            *position += serialized_len(scalar);
            None
        }
    }
}

/// `first_too_deep` over an object's members, after its `{`.
fn first_too_deep_in<'a>(
    members: impl Iterator<Item = (&'a String, &'a Value)>,
    level: usize,
    position: &mut usize,
) -> Option<usize> {
    for (index, (key, value)) in members.enumerate() {
        if index > 0 {
            *position += 1;
        }
        *position += serialized_len(key.as_str()) + 1;
        if let Some(found) = first_too_deep(value, level, position) {
            return Some(found);
        }
    }
    *position += 1;
    None
}

/// The length of serde_json's compact serialization of `value`, counted
/// without allocating it.
fn serialized_len<T: serde::Serialize + ?Sized>(value: &T) -> usize {
    struct Count(usize);
    impl std::io::Write for Count {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 += bytes.len();
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut count = Count(0);
    serde_json::to_writer(&mut count, value).expect("a string or JSON scalar always serializes");
    count.0
}

fn write_string<S: CanonicalSink>(text: &str, out: &mut S) {
    out.put(&serde_json::to_string(text).expect("strings always encode as JSON"));
}

fn write_canonical<S: CanonicalSink>(value: &Value, out: &mut S) {
    match value {
        Value::Null => out.put("null"),
        Value::Bool(true) => out.put("true"),
        Value::Bool(false) => out.put("false"),
        Value::Number(number) => out.put(&canonical_number(number)),
        Value::String(text) => write_string(text, out),
        Value::Array(items) => {
            out.put("[");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.put(",");
                }
                write_canonical(item, out);
            }
            out.put("]");
        }
        Value::Object(map) => {
            out.put("{");
            for (index, key) in sorted_keys(map, None).iter().enumerate() {
                if index > 0 {
                    out.put(",");
                }
                write_string(key, out);
                out.put(":");
                write_canonical(&map[key.as_str()], out);
            }
            out.put("}");
        }
    }
}

/// An object less its `omit` member, as `write_canonical` writes an object.
fn write_object_without<S: CanonicalSink>(map: &Map<String, Value>, omit: &str, out: &mut S) {
    out.put("{");
    for (index, key) in sorted_keys(map, Some(omit)).iter().enumerate() {
        if index > 0 {
            out.put(",");
        }
        write_string(key, out);
        out.put(":");
        write_canonical(&map[key.as_str()], out);
    }
    out.put("}");
}

/// An object's keys (less `omit`) in canonical order.
fn sorted_keys<'a>(map: &'a Map<String, Value>, omit: Option<&str>) -> Vec<&'a String> {
    let mut keys: Vec<&String> = Vec::with_capacity(map.len());
    keys.extend(map.keys().filter(|key| Some(key.as_str()) != omit));
    // RFC 8785 §3.2.3: property names sort as arrays of UTF-16 code
    // units (what ECMAScript string comparison does), not as Unicode
    // code points — the two differ for supplementary-plane keys.
    keys.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
    keys
}

/// Render a JSON number per RFC 8785 §3.2.2.3. Integer-repr numbers keep
/// serde's exact rendering (identical to ECMAScript inside 2^53; see the
/// crate-level divergence note); float-repr numbers use the ECMAScript
/// `Number::toString` algorithm.
pub fn canonical_number(number: &Number) -> String {
    if let Some(value) = number.as_i64() {
        return value.to_string();
    }
    if let Some(value) = number.as_u64() {
        return value.to_string();
    }
    let value = number
        .as_f64()
        .expect("serde_json numbers are i64, u64, or finite f64");
    ecma_number_to_string(value)
}

/// ECMAScript `Number::toString` (ECMA-262 §6.1.6.1.20, radix 10) for finite
/// doubles: shortest round-trip digits in the notation ECMAScript selects
/// (plain decimal for exponents in (-7, 21], exponential with an explicit
/// sign otherwise). `-0` renders as `0`, matching `JSON.stringify`.
pub fn ecma_number_to_string(value: f64) -> String {
    assert!(
        value.is_finite(),
        "JSON cannot carry NaN or infinite numbers"
    );
    if value == 0.0 {
        return "0".to_string();
    }
    let negative = value.is_sign_negative();
    let magnitude = value.abs();
    // Shortest round-trip digits from ryu, which selects the same digit
    // string ECMAScript engines do: the closest shortest candidate, exact
    // ties to the even significand. The standard library's formatter breaks
    // exact ties differently (measured: 7 divergences in 20k random
    // doubles), so it cannot be the digit source here.
    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format_finite(magnitude);
    let (digits, first_digit_exponent) = shortest_digits(printed);
    // ECMA-262 notation: value = s × 10^(n−k), where s is the k-digit
    // shortest integer significand and n positions the decimal point.
    let k = i32::try_from(digits.len()).expect("shortest f64 digits fit in i32");
    let n = first_digit_exponent + 1;

    let mut rendered = String::new();
    if negative {
        rendered.push('-');
    }
    if k <= n && n <= 21 {
        rendered.push_str(&digits);
        for _ in 0..(n - k) {
            rendered.push('0');
        }
    } else if 0 < n && n <= 21 {
        rendered.push_str(&digits[..n as usize]);
        rendered.push('.');
        rendered.push_str(&digits[n as usize..]);
    } else if -6 < n && n <= 0 {
        rendered.push_str("0.");
        for _ in 0..(-n) {
            rendered.push('0');
        }
        rendered.push_str(&digits);
    } else {
        rendered.push_str(&digits[..1]);
        if k > 1 {
            rendered.push('.');
            rendered.push_str(&digits[1..]);
        }
        rendered.push('e');
        rendered.push(if n - 1 >= 0 { '+' } else { '-' });
        rendered.push_str(&(n - 1).abs().to_string());
    }
    rendered
}

/// Extract the significand digits (no leading or trailing zeros) and the
/// decimal exponent of the first digit from a ryu-rendered positive finite
/// float, whichever notation ryu chose (`200.0`, `0.001`, `1e-7`, `1.5e20`).
fn shortest_digits(printed: &str) -> (String, i32) {
    let (mantissa, exponent) = match printed.split_once('e') {
        Some((mantissa, exponent)) => (
            mantissa,
            exponent.parse::<i32>().expect("ryu exponents are integers"),
        ),
        None => (printed, 0),
    };
    let (int_part, frac_part) = match mantissa.split_once('.') {
        Some((int_part, frac_part)) => (int_part, frac_part),
        None => (mantissa, ""),
    };
    let raw = format!("{int_part}{frac_part}");
    let without_leading = raw.trim_start_matches('0');
    let digits = without_leading.trim_end_matches('0');
    let trailing_zeros = i32::try_from(without_leading.len() - digits.len())
        .expect("shortest f64 digit counts fit in i32");
    let last_digit_exponent =
        exponent - i32::try_from(frac_part.len()).expect("ryu fraction lengths fit in i32")
            + trailing_zeros;
    let first_digit_exponent =
        last_digit_exponent + i32::try_from(digits.len()).expect("digit counts fit in i32") - 1;
    (digits.to_string(), first_digit_exponent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn number_from_text(text: &str) -> String {
        let value: Value = serde_json::from_str(text).expect("test number parses");
        canonical_json(&value)
    }

    /// RFC 8785 Appendix B vectors (bit patterns), verified against node
    /// `JSON.stringify` on 2026-06-11.
    #[test]
    fn rfc8785_appendix_b_bit_pattern_vectors() {
        let vectors: [(u64, &str); 13] = [
            (0x0000000000000000, "0"),
            (0x8000000000000000, "0"),
            (0x0000000000000001, "5e-324"),
            (0x7fefffffffffffff, "1.7976931348623157e+308"),
            (0x4340000000000000, "9007199254740992"),
            (0x4340000000000001, "9007199254740994"),
            (0x444b1ae4d6e2ef50, "1e+21"),
            (0x3eb0c6f7a0b5ed8d, "0.000001"),
            (0x3eb0c6f7a0b5ed8c, "9.999999999999997e-7"),
            (0x41b3de4355555553, "333333333.3333332"),
            (0x41b3de4355555554, "333333333.33333325"),
            (0x41b3de4355555555, "333333333.3333333"),
            (0x41b3de4355555556, "333333333.3333334"),
        ];
        for (bits, expected) in vectors {
            assert_eq!(
                ecma_number_to_string(f64::from_bits(bits)),
                expected,
                "bit pattern {bits:#018x}"
            );
        }
    }

    /// Literal vectors across every ECMAScript notation branch, verified
    /// against node `JSON.stringify` on 2026-06-11.
    #[test]
    fn ecma_notation_branch_vectors() {
        let vectors: [(f64, &str); 16] = [
            (200.0, "200"),
            (1e9, "1000000000"),
            (1e15, "1000000000000000"),
            (1e16, "10000000000000000"),
            (1e20, "100000000000000000000"),
            (1e21, "1e+21"),
            (1.23e21, "1.23e+21"),
            (1e-6, "0.000001"),
            (1e-7, "1e-7"),
            (0.1, "0.1"),
            (0.30000000000000004, "0.30000000000000004"),
            (-1.5e25, "-1.5e+25"),
            (4.5e-9, "4.5e-9"),
            (123.456, "123.456"),
            (-123456789.987654, "-123456789.987654"),
            (0.0000012345678901234567, "0.0000012345678901234567"),
        ];
        for (value, expected) in vectors {
            assert_eq!(ecma_number_to_string(value), expected, "value {value:?}");
        }
        assert_eq!(ecma_number_to_string(-0.0), "0");
        assert_eq!(ecma_number_to_string(5e-324), "5e-324");
        assert_eq!(ecma_number_to_string(1e100), "1e+100");
        assert_eq!(
            ecma_number_to_string(2.2250738585072014e-308),
            "2.2250738585072014e-308"
        );
    }

    /// The three JSON-text producers converge: a float-repr integral value
    /// (raw-file `200.0`), an integer-repr value (JS-transported `200`), and
    /// an in-process f64 all render the same canonical bytes.
    #[test]
    fn float_and_integer_repr_converge() {
        assert_eq!(number_from_text("200.0"), "200");
        assert_eq!(number_from_text("200"), "200");
        assert_eq!(number_from_text("1e9"), "1000000000");
        assert_eq!(number_from_text("1000000000"), "1000000000");
        assert_eq!(number_from_text("1e21"), "1e+21");
        assert_eq!(number_from_text("-0.0"), "0");
    }

    /// Documented divergence: integers beyond 2^53 keep exact precision
    /// (outside the I-JSON envelope; unreachable through a JS lane).
    #[test]
    fn integers_beyond_2_53_keep_exact_precision() {
        assert_eq!(
            number_from_text("9007199254740993"),
            "9007199254740993",
            "u64-exact rendering is preserved"
        );
        assert_eq!(number_from_text("18446744073709551615"), "18446744073709551615");
        assert_eq!(number_from_text("-9223372036854775808"), "-9223372036854775808");
    }

    #[test]
    fn objects_sort_keys_by_utf16_code_units() {
        // U+1F600 encodes as the surrogate pair D83D DE00; U+FF5A is a single
        // BMP unit. UTF-16 order puts the surrogate pair first; code-point
        // order would reverse them.
        let value = json!({
            "\u{ff5a}": 1,
            "\u{1f600}": 2,
            "b": 3,
            "a": 4,
        });
        assert_eq!(
            canonical_json(&value),
            "{\"a\":4,\"b\":3,\"\u{1f600}\":2,\"\u{ff5a}\":1}"
        );
    }

    /// String escaping matches `JSON.stringify` (RFC 8785 §3.2.2.2):
    /// two-character escapes where defined, lowercase `\u00xx` for the other
    /// control characters, non-ASCII characters literal.
    #[test]
    fn strings_escape_exactly_as_json_stringify() {
        let value = json!("\u{8}\t\n\u{c}\r\"\\\u{1f}\u{7f}\u{80}é€");
        assert_eq!(
            canonical_json(&value),
            "\"\\b\\t\\n\\f\\r\\\"\\\\\\u001f\u{7f}\u{80}é€\""
        );
    }

    /// Composite document pinned against node
    /// `JSON.stringify(sortedDeep(value))` output (verified 2026-06-11).
    #[test]
    fn composite_document_matches_ecmascript_rendering() {
        let value: Value = serde_json::from_str(
            r#"{"z": 1e9, "a": [200.0, 1e-7, {"nested": -0.0}], "m": "text", "empty_obj": {}, "empty_arr": []}"#,
        )
        .expect("composite parses");
        assert_eq!(
            canonical_json(&value),
            r#"{"a":[200,1e-7,{"nested":0}],"empty_arr":[],"empty_obj":{},"m":"text","z":1000000000}"#
        );
    }

    #[test]
    fn null_and_bool_scalars_render_literally() {
        assert_eq!(canonical_json(&Value::Null), "null");
        assert_eq!(canonical_json(&json!(true)), "true");
        assert_eq!(canonical_json(&json!(false)), "false");
    }
}
