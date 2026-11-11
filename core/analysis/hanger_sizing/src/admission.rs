//! The path-aware admission layer (DEL-04-11 design §8.3.1).
//!
//! The headless entry and the reader admit their text here before any other
//! parse. The layer parses the text itself, with the same parser and number
//! rules as `open_pipe_stress_canonical_json::canonical_json_checked_v1_text`
//! (`openpipestress_jcs_ijson_v1`): finite numbers, integers within ±(2⁵³ − 1),
//! integral numbers no larger, and objects without duplicate keys. Unlike that
//! function, a refusal names a kind and the JSON path of the offending member,
//! never the offending value. The test `admission_agrees_with_the_checked_profile`
//! shows the two functions' verdicts agreeing across the profile's boundary
//! cases.

use open_pipe_stress_canonical_json::MAX_SAFE_INTEGER;
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;

/// The kind of an admission finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingKind {
    InvalidJson,
    DuplicateKey,
    NumberOutsideProfile,
}

impl FindingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            FindingKind::InvalidJson => "invalid_json",
            FindingKind::DuplicateKey => "duplicate_key",
            FindingKind::NumberOutsideProfile => "number_outside_profile",
        }
    }
}

/// One admission refusal: its kind and the JSON path of the offending member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionFinding {
    pub kind: FindingKind,
    pub path: String,
}

#[derive(Debug, Clone)]
enum Segment {
    Key(String),
    Index(usize),
}

fn simple_key(key: &str) -> bool {
    let mut chars = key.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Appends one member to a JSON path: `.name` for a plain key, `["…"]` (the
/// key as a JSON string) otherwise, `[i]` for an array index.
pub fn push_key(path: &mut String, key: &str) {
    if simple_key(key) {
        path.push('.');
        path.push_str(key);
    } else {
        path.push('[');
        path.push_str(&serde_json::to_string(key).expect("a string serializes"));
        path.push(']');
    }
}

fn render(segments: &[Segment]) -> String {
    let mut path = String::from("$");
    for segment in segments {
        match segment {
            Segment::Key(key) => push_key(&mut path, key),
            Segment::Index(index) => path.push_str(&format!("[{index}]")),
        }
    }
    path
}

struct State {
    path: Vec<Segment>,
    finding: Option<AdmissionFinding>,
}

impl State {
    fn record(&mut self, kind: FindingKind) {
        if self.finding.is_none() {
            self.finding = Some(AdmissionFinding { kind, path: render(&self.path) });
        }
    }
}

struct Seed<'s>(&'s RefCell<State>);

impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;
    fn deserialize<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_any(ValueVisitor(self.0))
    }
}

struct ValueVisitor<'s>(&'s RefCell<State>);

impl ValueVisitor<'_> {
    fn refuse<E: de::Error>(&self, kind: FindingKind) -> E {
        self.0.borrow_mut().record(kind);
        E::custom(kind.as_str())
    }
}

impl<'de> Visitor<'de> for ValueVisitor<'_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value")
    }
    fn visit_bool<E>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
        if (-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&v) {
            Ok(Value::Number(v.into()))
        } else {
            Err(self.refuse(FindingKind::NumberOutsideProfile))
        }
    }
    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
        if v <= MAX_SAFE_INTEGER as u64 {
            Ok(Value::Number(v.into()))
        } else {
            Err(self.refuse(FindingKind::NumberOutsideProfile))
        }
    }
    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Value, E> {
        if in_profile(v) {
            Ok(Value::Number(Number::from_f64(v).expect("finite")))
        } else {
            Err(self.refuse(FindingKind::NumberOutsideProfile))
        }
    }
    fn visit_str<E>(self, v: &str) -> Result<Value, E> {
        Ok(Value::String(v.to_owned()))
    }
    fn visit_string<E>(self, v: String) -> Result<Value, E> {
        Ok(Value::String(v))
    }
    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_some<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        Seed(self.0).deserialize(deserializer)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        loop {
            self.0.borrow_mut().path.push(Segment::Index(values.len()));
            let next = seq.next_element_seed(Seed(self.0))?;
            self.0.borrow_mut().path.pop();
            match next {
                Some(value) => values.push(value),
                None => return Ok(Value::Array(values)),
            }
        }
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut seen = HashSet::new();
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            self.0.borrow_mut().path.push(Segment::Key(key.clone()));
            if !seen.insert(key.clone()) {
                return Err(self.refuse(FindingKind::DuplicateKey));
            }
            let value = map.next_value_seed(Seed(self.0))?;
            self.0.borrow_mut().path.pop();
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}

/// Whether a binary64 is inside the checked profile: finite, and not integral
/// beyond 2⁵³ − 1 (every binary64 of that magnitude is integral).
pub fn in_profile(v: f64) -> bool {
    v.is_finite() && !(v.fract() == 0.0 && v.abs() > MAX_SAFE_INTEGER as f64)
}

/// Admits a text under the checked profile, or names the first offending member.
pub fn admit(text: &str) -> Result<Value, AdmissionFinding> {
    let state = RefCell::new(State { path: Vec::new(), finding: None });
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let parsed = Seed(&state).deserialize(&mut deserializer).and_then(|value| deserializer.end().map(|()| value));
    match parsed {
        Ok(value) => Ok(value),
        Err(error) => {
            let state = state.into_inner();
            Err(state.finding.unwrap_or_else(|| {
                // serde_json refuses a number that overflows binary64 ("number
                // out of range"): a non-finite number, outside the profile.
                let kind = if error.to_string().starts_with("number out of range") {
                    FindingKind::NumberOutsideProfile
                } else {
                    FindingKind::InvalidJson
                };
                AdmissionFinding { kind, path: render(&state.path) }
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;

    fn nested(depth: usize) -> String {
        format!("{}0{}", "[".repeat(depth), "]".repeat(depth))
    }

    #[test]
    fn admission_agrees_with_the_checked_profile() {
        let mut texts: Vec<String> = [
            "{}", "[]", "0", "-0", "0.0", "-0.0", "1.5", "\"text\"", "null", "true",
            "9007199254740991", "-9007199254740991", "9007199254740992", "-9007199254740992",
            "9007199254740991.0", "9007199254740992.0", "-9007199254740992.0", "9007199254740991.5",
            "9.007199254740991e15", "9.007199254740992e15", "1e15", "1e16", "1e20", "1E20", "-1e20",
            "1.5e300", "1e308", "1.7976931348623157e308", "1e309", "-1e400", "1e-400", "5e-324",
            "0.1", "123456789012345.6", "18446744073709551615", "18446744073709551616",
            "-9223372036854775808", "-9223372036854775809", "100000000000000000000",
            "{\"a\":1,\"a\":2}", "{\"a\":1,\"\\u0061\":2}", "{\"a\":{\"b\":1,\"b\":1}}", "[{\"x\":1},{\"x\":1}]",
            "{\"a\":1,\"b\":2}", "{\"a\":[1,2,{\"c\":1e20}]}", "NaN", "Infinity", "-Infinity", "[1,]",
            "{\"a\":1,}", "[1 2]", "", " ", "{} {}", "{}  \n", "\u{feff}{}", "\"\\ud800\"", "\"\\udc00\"",
            "\"\\ud83d\\ude00\"", "\"a\u{1}b\"", "01", "1.", ".5", "+1", "[\"a\" ,  1e5 ]", "{\"\":1,\"\":2}",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        for depth in [126, 127, 128, 129] {
            texts.push(nested(depth));
        }
        for text in &texts {
            let checked = canonical_json_checked_v1_text(text).is_ok();
            let ours = admit(text);
            assert_eq!(ours.is_ok(), checked, "verdicts differ on {text:?}: {ours:?}");
            if let Ok(value) = ours {
                // Admitted text is the same value: its canonical text is the checked function's.
                assert_eq!(open_pipe_stress_canonical_json::canonical_json(&value), canonical_json_checked_v1_text(text).unwrap());
            }
        }
    }

    #[test]
    fn findings_name_a_kind_and_path_and_never_the_value() {
        let cases = [
            ("{\"a\":{\"b\":1,\"b\":2}}", FindingKind::DuplicateKey, "$.a.b"),
            ("{\"a\":[0, 1e20]}", FindingKind::NumberOutsideProfile, "$.a[1]"),
            ("{\"weird key\":[0, 1e400]}", FindingKind::NumberOutsideProfile, "$[\"weird key\"][1]"),
            ("{\"a\":[0, 12345678901234567890123]}", FindingKind::NumberOutsideProfile, "$.a[1]"),
            ("{\"a\":[0, 9007199254740992]}", FindingKind::NumberOutsideProfile, "$.a[1]"),
            ("{\"a\":[0, -9007199254740992]}", FindingKind::NumberOutsideProfile, "$.a[1]"),
            ("{\"a\":tru}", FindingKind::InvalidJson, "$.a"),
            ("", FindingKind::InvalidJson, "$"),
        ];
        for (text, kind, path) in cases {
            let finding = admit(text).unwrap_err();
            assert_eq!((finding.kind, finding.path.as_str()), (kind, path), "{text}");
        }
    }
}
