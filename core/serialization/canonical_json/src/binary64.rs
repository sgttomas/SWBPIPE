//! Strict scientific JSON ingress. This is an explicitly selected successor,
//! not a change to checked I-JSON v1 or the unrestricted historical renderer.
use crate::{ecma_number_to_string, MAX_SAFE_INTEGER};
use serde_json::{Map, Number, Value};
use std::collections::BTreeMap;

pub const BINARY64_PROFILE_V1: &str = "openpipestress_jcs_binary64_v1";
pub const MAX_DOCUMENT_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_DEPTH: usize = 128;
pub const MAX_VALUE_NODES: usize = 262_144;
pub const MAX_NUMBER_BYTES: usize = 4096;
/// Bounds repeated long-key path construction and retained numeric metadata.
pub const MAX_CAPTURE_PATH_BYTES: usize = 16 * 1024 * 1024;

/// A parsed scientific document. Numeric lexemes belong to this actual input;
/// schema adapters must check their exact-integer fields before using `value`.
/// This object makes no raw-byte-custody or typed-record-qualification claim.
#[derive(Debug)]
pub struct Binary64Document {
    value: Value,
    numbers: BTreeMap<String, String>,
}

impl Binary64Document {
    pub fn value(&self) -> &Value {
        &self.value
    }
    pub fn into_value(self) -> Value {
        self.value
    }
    /// Both received text and its canonical rendering obey the document byte
    /// bound, so an admitted canonical result can be parsed again unchanged.
    pub fn canonical_json(&self) -> Result<String, String> {
        canonical_json_binary64_v1_value(&self.value)
    }
    /// Check one field explicitly identified by its schema. Bounds may narrow
    /// but never extend the safe-integer domain. JSON Pointer uses RFC6901.
    pub fn exact_integer_at(
        &self,
        pointer: &str,
        minimum: i64,
        maximum: i64,
    ) -> Result<i64, String> {
        let token = self
            .numbers
            .get(pointer)
            .ok_or_else(|| error("EXACT-INTEGER-REQUIRED"))?;
        exact_integer_token(token, minimum, maximum)
    }
}

fn error(reason: &str) -> String {
    format!("BINARY64-JSON-{reason}")
}

fn valid_string(text: &str) -> Result<(), String> {
    if text.chars().any(|c| {
        let cp = c as u32;
        (0xfdd0..=0xfdef).contains(&cp) || cp & 0xffff == 0xfffe || cp & 0xffff == 0xffff
    }) {
        return Err(error("NONCHARACTER"));
    }
    Ok(())
}

fn integer_bounds(minimum: i64, maximum: i64) -> Result<(), String> {
    if minimum < -MAX_SAFE_INTEGER || maximum > MAX_SAFE_INTEGER || minimum > maximum {
        return Err(error("EXACT-INTEGER-BOUNDS"));
    }
    Ok(())
}

/// Original-token admission for a schema-owned count/index. Decimal/exponent
/// tokens and negative zero fail before binary64 rounding can erase them.
pub fn exact_integer_token(token: &str, minimum: i64, maximum: i64) -> Result<i64, String> {
    integer_bounds(minimum, maximum)?;
    let digits = token.strip_prefix('-').unwrap_or(token);
    if digits.is_empty()
        || !digits.bytes().all(|b| b.is_ascii_digit())
        || (digits.len() > 1 && digits.starts_with('0'))
        || token == "-0"
        || digits.len() > 16
    {
        return Err(error("EXACT-INTEGER-TOKEN"));
    }
    let value = token
        .parse::<i64>()
        .map_err(|_| error("EXACT-INTEGER-RANGE"))?;
    exact_integer_value(value, minimum, maximum)
}

/// Programmatic integer admission. Passing a floating-point value requires an
/// explicit caller conversion and cannot establish that it was an exact input.
pub fn exact_integer_value(value: i64, minimum: i64, maximum: i64) -> Result<i64, String> {
    integer_bounds(minimum, maximum)?;
    if !(-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&value)
        || value < minimum
        || value > maximum
    {
        return Err(error("EXACT-INTEGER-RANGE"));
    }
    Ok(value)
}

/// Parse original UTF-8 text without discarding duplicate names or numeric
/// spellings. Every generic number in this profile denotes a binary64 real.
pub fn parse_binary64_v1_text(input: &str) -> Result<Binary64Document, String> {
    parse_with_byte_limit(input, MAX_DOCUMENT_BYTES)
}

/// Used by the bounded CLI envelope; not a public way around document limits.
pub(crate) fn parse_with_byte_limit(
    input: &str,
    byte_limit: usize,
) -> Result<Binary64Document, String> {
    if input.len() > byte_limit {
        return Err(error("BYTE-LIMIT"));
    }
    let mut parser = Parser {
        input,
        offset: 0,
        nodes: 0,
        path_bytes: 0,
        numbers: BTreeMap::new(),
    };
    let value = parser.value(0, "")?;
    parser.whitespace();
    if parser.offset != input.len() {
        return Err(error("TRAILING-TEXT"));
    }
    Ok(Binary64Document {
        value,
        numbers: parser.numbers,
    })
}

pub fn canonical_json_binary64_v1_text(input: &str) -> Result<String, String> {
    parse_binary64_v1_text(input)?.canonical_json()
}

/// Authenticate an actual supplied serde snapshot, not lost source text.
/// Unsafe i64/u64 values are rejected: a scientific integer must be explicitly
/// converted by its typed owner to f64, or supplied via the raw-text real API.
pub fn canonical_json_binary64_v1_value(value: &Value) -> Result<String, String> {
    validate_value(value, 0, &mut 0, &mut 0, 0, &mut 0)?;
    let mut output = String::new();
    write_value(value, &mut output);
    if output.len() > MAX_DOCUMENT_BYTES {
        return Err(error("BYTE-LIMIT"));
    }
    Ok(output)
}

fn output_bytes(bytes: &mut usize, additional: usize) -> Result<(), String> {
    *bytes = bytes
        .checked_add(additional)
        .filter(|size| *size <= MAX_DOCUMENT_BYTES)
        .ok_or_else(|| error("BYTE-LIMIT"))?;
    Ok(())
}

fn string_output_bytes(text: &str, bytes: &mut usize) -> Result<(), String> {
    // UTF-8 source bytes are a lower bound; check before any escaping/allocation.
    if text.len() > MAX_DOCUMENT_BYTES.saturating_sub(*bytes) {
        return Err(error("BYTE-LIMIT"));
    }
    output_bytes(bytes, 2)?;
    for character in text.chars() {
        let width = match character {
            '"' | '\\' | '\u{8}' | '\t' | '\n' | '\u{c}' | '\r' => 2,
            '\u{0}'..='\u{1f}' => 6,
            _ => character.len_utf8(),
        };
        output_bytes(bytes, width)?;
    }
    valid_string(text)
}

fn validate_value(
    value: &Value,
    depth: usize,
    nodes: &mut usize,
    bytes: &mut usize,
    path_length: usize,
    path_bytes: &mut usize,
) -> Result<(), String> {
    if depth > MAX_DEPTH {
        return Err(error("DEPTH-LIMIT"));
    }
    *nodes += 1;
    if *nodes > MAX_VALUE_NODES {
        return Err(error("NODE-LIMIT"));
    }
    match value {
        Value::Number(n) => {
            if let Some(v) = n.as_i64() {
                exact_integer_value(v, -MAX_SAFE_INTEGER, MAX_SAFE_INTEGER)?;
            } else if let Some(v) = n.as_u64() {
                if v > MAX_SAFE_INTEGER as u64 {
                    return Err(error("UNSAFE-HOST-INTEGER"));
                }
            }
            let v = n.as_f64().ok_or_else(|| error("NON-FINITE"))?;
            if !v.is_finite() {
                return Err(error("NON-FINITE"));
            }
            if v == 0.0 && v.is_sign_negative() {
                return Err(error("NEGATIVE-ZERO"));
            }
            output_bytes(bytes, ecma_number_to_string(v).len())?;
        }
        Value::String(s) => string_output_bytes(s, bytes)?,
        Value::Array(items) => {
            output_bytes(bytes, 2 + items.len().saturating_sub(1))?;
            for (index, item) in items.iter().enumerate() {
                let child = child_path_length(path_length, &index.to_string(), path_bytes)?;
                validate_value(item, depth + 1, nodes, bytes, child, path_bytes)?;
            }
        }
        Value::Object(items) => {
            output_bytes(bytes, 2 + items.len().saturating_sub(1))?;
            for (key, item) in items {
                string_output_bytes(key, bytes)?;
                output_bytes(bytes, 1)?;
                let child = child_path_length(path_length, key, path_bytes)?;
                validate_value(item, depth + 1, nodes, bytes, child, path_bytes)?;
            }
        }
        Value::Null => output_bytes(bytes, 4)?,
        Value::Bool(true) => output_bytes(bytes, 4)?,
        Value::Bool(false) => output_bytes(bytes, 5)?,
    }
    Ok(())
}

// One resource definition for text capture and direct snapshot emission. The
// latter counts pointer bytes without allocating paths that it does not need.
fn child_path_length(
    parent_length: usize,
    component: &str,
    cumulative: &mut usize,
) -> Result<usize, String> {
    let escaped_len = component
        .len()
        .checked_add(
            component
                .bytes()
                .filter(|b| matches!(b, b'~' | b'/'))
                .count(),
        )
        .ok_or_else(|| error("PATH-LIMIT"))?;
    let length = parent_length
        .checked_add(1)
        .and_then(|v| v.checked_add(escaped_len))
        .ok_or_else(|| error("PATH-LIMIT"))?;
    *cumulative = cumulative
        .checked_add(length)
        .filter(|v| *v <= MAX_CAPTURE_PATH_BYTES)
        .ok_or_else(|| error("PATH-LIMIT"))?;
    Ok(length)
}

fn write_value(value: &Value, output: &mut String) {
    match value {
        Value::Number(n) => output.push_str(&ecma_number_to_string(
            n.as_f64().expect("admitted binary64"),
        )),
        Value::Array(items) => {
            output.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                write_value(item, output);
            }
            output.push(']');
        }
        Value::Object(items) => {
            let mut keys: Vec<_> = items.keys().collect();
            keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
            output.push('{');
            for (index, key) in keys.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                output.push_str(&serde_json::to_string(key).expect("valid string"));
                output.push(':');
                write_value(&items[*key], output);
            }
            output.push('}');
        }
        _ => output.push_str(&serde_json::to_string(value).expect("admitted scalar")),
    }
}

struct Parser<'a> {
    input: &'a str,
    offset: usize,
    nodes: usize,
    path_bytes: usize,
    numbers: BTreeMap<String, String>,
}
impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.offset).copied()
    }
    fn whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.offset += 1;
        }
    }
    fn consume(&mut self, expected: u8) -> Result<(), String> {
        self.whitespace();
        if self.peek() != Some(expected) {
            return Err(error("SYNTAX"));
        }
        self.offset += 1;
        Ok(())
    }
    fn value(&mut self, depth: usize, pointer: &str) -> Result<Value, String> {
        if depth > MAX_DEPTH {
            return Err(error("DEPTH-LIMIT"));
        }
        self.nodes += 1;
        if self.nodes > MAX_VALUE_NODES {
            return Err(error("NODE-LIMIT"));
        }
        self.whitespace();
        match self.peek() {
            Some(b'n') => {
                self.literal("null")?;
                Ok(Value::Null)
            }
            Some(b't') => {
                self.literal("true")?;
                Ok(Value::Bool(true))
            }
            Some(b'f') => {
                self.literal("false")?;
                Ok(Value::Bool(false))
            }
            Some(b'"') => self.string().map(Value::String),
            Some(b'[') => {
                self.offset += 1;
                let mut items = Vec::new();
                self.whitespace();
                if self.peek() == Some(b']') {
                    self.offset += 1;
                    return Ok(Value::Array(items));
                }
                loop {
                    let child = self.child_pointer(pointer, &items.len().to_string())?;
                    items.push(self.value(depth + 1, &child)?);
                    self.whitespace();
                    if self.peek() == Some(b']') {
                        self.offset += 1;
                        break;
                    }
                    self.consume(b',')?;
                }
                Ok(Value::Array(items))
            }
            Some(b'{') => {
                self.offset += 1;
                let mut items = Map::new();
                self.whitespace();
                if self.peek() == Some(b'}') {
                    self.offset += 1;
                    return Ok(Value::Object(items));
                }
                loop {
                    self.whitespace();
                    let key = self.string()?;
                    if items.contains_key(&key) {
                        return Err(error("DUPLICATE-KEY"));
                    }
                    self.consume(b':')?;
                    let child = self.child_pointer(pointer, &key)?;
                    let item = self.value(depth + 1, &child)?;
                    items.insert(key, item);
                    self.whitespace();
                    if self.peek() == Some(b'}') {
                        self.offset += 1;
                        break;
                    }
                    self.consume(b',')?;
                }
                Ok(Value::Object(items))
            }
            Some(b'-' | b'0'..=b'9') => self.number(pointer),
            _ => Err(error("SYNTAX")),
        }
    }
    fn child_pointer(&mut self, parent: &str, component: &str) -> Result<String, String> {
        let length = child_path_length(parent.len(), component, &mut self.path_bytes)?;
        let mut result = String::with_capacity(length);
        result.push_str(parent);
        result.push('/');
        for character in component.chars() {
            match character {
                '~' => result.push_str("~0"),
                '/' => result.push_str("~1"),
                _ => result.push(character),
            }
        }
        Ok(result)
    }
    fn literal(&mut self, literal: &str) -> Result<(), String> {
        if !self.input[self.offset..].starts_with(literal) {
            return Err(error("SYNTAX"));
        }
        self.offset += literal.len();
        Ok(())
    }
    fn string(&mut self) -> Result<String, String> {
        if self.peek() != Some(b'"') {
            return Err(error("STRING-REQUIRED"));
        }
        let start = self.offset;
        self.offset += 1;
        // Locate the quoted token without interpreting it. serde validates the
        // token's escapes, scalar values, controls and surrogate pairs exactly.
        loop {
            match self.peek() {
                None => return Err(error("UNTERMINATED-STRING")),
                Some(b'"') => {
                    self.offset += 1;
                    break;
                }
                Some(b'\\') => {
                    self.offset += 1;
                    if self.peek().is_none() {
                        return Err(error("UNTERMINATED-STRING"));
                    }
                    self.offset += 1;
                }
                _ => self.offset += 1,
            }
        }
        let text: String = serde_json::from_str(&self.input[start..self.offset])
            .map_err(|_| error("INVALID-STRING"))?;
        valid_string(&text)?;
        Ok(text)
    }
    fn digits(&mut self) -> usize {
        let start = self.offset;
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.offset += 1;
        }
        self.offset - start
    }
    fn number(&mut self, pointer: &str) -> Result<Value, String> {
        let start = self.offset;
        let negative = self.peek() == Some(b'-');
        if negative {
            self.offset += 1;
        }
        match self.peek() {
            Some(b'0') => self.offset += 1,
            Some(b'1'..=b'9') => {
                self.digits();
            }
            _ => return Err(error("NUMBER-SYNTAX")),
        }
        if self.peek() == Some(b'.') {
            self.offset += 1;
            if self.digits() == 0 {
                return Err(error("NUMBER-SYNTAX"));
            }
        }
        let significand_end = self.offset;
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.offset += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.offset += 1;
            }
            if self.digits() == 0 {
                return Err(error("NUMBER-SYNTAX"));
            }
        }
        if self.offset - start > MAX_NUMBER_BYTES {
            return Err(error("NUMBER-LIMIT"));
        }
        let token = &self.input[start..self.offset];
        let nonzero = self.input[start..significand_end]
            .bytes()
            .any(|b| matches!(b, b'1'..=b'9'));
        // Rust's decimal parser consumes the original token once; there is no
        // intermediate decimal precision or serde integer/float double-rounding.
        let value: f64 = token.parse().map_err(|_| error("NUMBER-SYNTAX"))?;
        if !value.is_finite() {
            return Err(error("OVERFLOW"));
        }
        if value == 0.0 {
            if nonzero {
                return Err(error("NONZERO-UNDERFLOW"));
            }
            if negative {
                return Err(error("NEGATIVE-ZERO"));
            }
        }
        self.numbers.insert(pointer.to_owned(), token.to_owned());
        // Keep admitted safe integer tokens usable by typed integer DTOs.
        if let Ok(integer) = exact_integer_token(token, -MAX_SAFE_INTEGER, MAX_SAFE_INTEGER) {
            Ok(Value::Number(integer.into()))
        } else {
            Ok(Value::Number(
                Number::from_f64(value).expect("finite parsed value"),
            ))
        }
    }
}
