//! The importer's `parse_sidecar_json`: strict UTF-8, then CPython 3.13's `json.loads` (its C scanner, with its
//! error texts and positions in code points), with NaN and Infinity refused and repeated member names found by
//! JSON path.

use crate::rational::py_int_digits;
use crate::value::{Dict, Value};

/// Nesting deeper than this returns no result (`ImportError::Unsupported`) before any recursion can exhaust a stack.
/// CPython parses deeper and raises RecursionError at a depth this port does not reproduce; the sidecars the writer
/// produces nest at most 8 deep. Review r1: at 512, an array 402 deep overflowed a debug build's 2 MiB thread stack
/// (about 5 KB a level) and 436 deep a debug wasm build's 1 MiB; at 64 the margin is over sixfold in each.
pub const MAX_NESTING: usize = 64;

/// What `parse_sidecar_json` refuses.
#[derive(Clone, Debug, PartialEq)]
pub enum SidecarJsonError {
    /// `SidecarJsonInvalid(findings)`: (JSON path, reason).
    Invalid(Vec<(String, String)>),
    /// A `\uD800`-`\uDFFF` escape without its pair: Python holds a lone surrogate, which a Rust string cannot.
    LoneSurrogate,
    /// Nesting beyond [`MAX_NESTING`].
    TooDeep,
}

enum ScanError {
    Stop(usize),
    Decode(&'static str, usize),
    Value(String),
    TooDeep,
}

enum J {
    Obj(Vec<(String, J)>),
    Arr(Vec<J>),
    Scalar(Value),
}

struct Scanner<'a> {
    s: &'a [char],
    lone: bool,
}

fn is_ws(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

impl<'a> Scanner<'a> {
    fn len(&self) -> usize {
        self.s.len()
    }
    fn at(&self, index: usize) -> char {
        self.s[index]
    }
    fn skip_ws(&self, mut idx: usize) -> usize {
        while idx < self.len() && is_ws(self.at(idx)) {
            idx += 1;
        }
        idx
    }
    fn matches(&self, idx: usize, word: &str) -> bool {
        word.chars().enumerate().all(|(offset, c)| idx + offset < self.len() && self.at(idx + offset) == c)
    }

    fn scan_once(&mut self, idx: usize, depth: usize) -> Result<(J, usize), ScanError> {
        if idx >= self.len() {
            return Err(ScanError::Stop(idx));
        }
        if depth > MAX_NESTING {
            return Err(ScanError::TooDeep);
        }
        match self.at(idx) {
            '"' => {
                let (text, end) = self.scanstring(idx + 1)?;
                return Ok((J::Scalar(Value::Str(text)), end));
            }
            '{' => return self.parse_object(idx + 1, depth + 1),
            '[' => return self.parse_array(idx + 1, depth + 1),
            'n' if idx + 3 < self.len() && self.matches(idx, "null") => return Ok((J::Scalar(Value::Null), idx + 4)),
            't' if idx + 3 < self.len() && self.matches(idx, "true") => {
                return Ok((J::Scalar(Value::Bool(true)), idx + 4))
            }
            'f' if idx + 4 < self.len() && self.matches(idx, "false") => {
                return Ok((J::Scalar(Value::Bool(false)), idx + 5))
            }
            'N' if idx + 2 < self.len() && self.matches(idx, "NaN") => {
                return Err(ScanError::Value("NaN is not a JSON value".into()))
            }
            'I' if idx + 7 < self.len() && self.matches(idx, "Infinity") => {
                return Err(ScanError::Value("Infinity is not a JSON value".into()))
            }
            '-' if idx + 8 < self.len() && self.matches(idx, "-Infinity") => {
                return Err(ScanError::Value("-Infinity is not a JSON value".into()))
            }
            _ => {}
        }
        self.match_number(idx)
    }

    fn match_number(&self, start: usize) -> Result<(J, usize), ScanError> {
        let end_idx = self.len() as isize - 1;
        let digit = |index: isize| index >= 0 && index <= end_idx && self.at(index as usize).is_ascii_digit();
        let mut idx = start as isize;
        if self.at(start) == '-' {
            idx += 1;
            if idx > end_idx {
                return Err(ScanError::Stop(start));
            }
        }
        let c = self.at(idx as usize);
        if ('1'..='9').contains(&c) {
            idx += 1;
            while digit(idx) {
                idx += 1;
            }
        } else if c == '0' {
            idx += 1;
        } else {
            return Err(ScanError::Stop(start));
        }
        let mut is_float = false;
        if idx < end_idx && self.at(idx as usize) == '.' && digit(idx + 1) {
            is_float = true;
            idx += 2;
            while digit(idx) {
                idx += 1;
            }
        }
        if idx < end_idx && matches!(self.at(idx as usize), 'e' | 'E') {
            let e_start = idx;
            idx += 1;
            if idx < end_idx && matches!(self.at(idx as usize), '-' | '+') {
                idx += 1;
            }
            while digit(idx) {
                idx += 1;
            }
            if digit(idx - 1) {
                is_float = true;
            } else {
                idx = e_start;
            }
        }
        let text: String = self.s[start..idx as usize].iter().collect();
        let value = if is_float {
            Value::Float(text.parse::<f64>().expect("a JSON number"))
        } else {
            let (negative, digits) = match text.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, text.as_str()),
            };
            let magnitude = py_int_digits(digits).map_err(|error| ScanError::Value(error.message))?;
            Value::Int(if negative { -magnitude } else { magnitude })
        };
        Ok((J::Scalar(value), idx as usize))
    }

    fn hex4(&self, from: usize) -> Option<u32> {
        let mut value = 0u32;
        for index in from..from + 4 {
            value = (value << 4) | self.at(index).to_digit(16)?;
        }
        Some(value)
    }

    fn scanstring(&mut self, mut end: usize) -> Result<(String, usize), ScanError> {
        let begin = end - 1;
        let len = self.len();
        let mut out = String::new();
        loop {
            let mut next = end;
            let mut c = '\0';
            while next < len {
                let d = self.at(next);
                c = d;
                if d == '"' || d == '\\' {
                    break;
                }
                if (d as u32) <= 0x1f {
                    return Err(ScanError::Decode("Invalid control character at", next));
                }
                next += 1;
            }
            if next >= len {
                c = '\0';
            }
            if c != '"' && c != '\\' {
                return Err(ScanError::Decode("Unterminated string starting at", begin));
            }
            out.extend(&self.s[end..next]);
            next += 1;
            if c == '"' {
                end = next;
                break;
            }
            if next == len {
                return Err(ScanError::Decode("Unterminated string starting at", begin));
            }
            let escape = self.at(next);
            let code: u32;
            if escape != 'u' {
                end = next + 1;
                code = match escape {
                    '"' => '"' as u32,
                    '\\' => '\\' as u32,
                    '/' => '/' as u32,
                    'b' => 0x8,
                    'f' => 0xc,
                    'n' => 0xa,
                    'r' => 0xd,
                    't' => 0x9,
                    _ => return Err(ScanError::Decode("Invalid \\escape", end - 2)),
                };
                out.push(char::from_u32(code).expect("ASCII"));
                continue;
            }
            next += 1;
            end = next + 4;
            if end >= len {
                return Err(ScanError::Decode("Invalid \\uXXXX escape", next - 1));
            }
            let mut value = match self.hex4(next) {
                Some(value) => value,
                None => return Err(ScanError::Decode("Invalid \\uXXXX escape", end - 5)),
            };
            if (0xd800..0xdc00).contains(&value) && end + 6 < len && self.at(end) == '\\' && self.at(end + 1) == 'u' {
                let pair_end = end + 6;
                let low = match self.hex4(end + 2) {
                    Some(low) => low,
                    None => return Err(ScanError::Decode("Invalid \\uXXXX escape", pair_end - 5)),
                };
                if (0xdc00..0xe000).contains(&low) {
                    value = 0x10000 + (((value - 0xd800) << 10) | (low - 0xdc00));
                    end = pair_end;
                }
            }
            match char::from_u32(value) {
                Some(character) => out.push(character),
                None => {
                    self.lone = true;
                    out.push('\u{fffd}');
                }
            }
        }
        Ok((out, end))
    }

    fn parse_object(&mut self, start: usize, depth: usize) -> Result<(J, usize), ScanError> {
        let mut pairs = Vec::new();
        let mut idx = self.skip_ws(start);
        if idx >= self.len() || self.at(idx) != '}' {
            loop {
                if idx >= self.len() || self.at(idx) != '"' {
                    return Err(ScanError::Decode("Expecting property name enclosed in double quotes", idx));
                }
                let (key, next) = self.scanstring(idx + 1)?;
                idx = self.skip_ws(next);
                if idx >= self.len() || self.at(idx) != ':' {
                    return Err(ScanError::Decode("Expecting ':' delimiter", idx));
                }
                idx = self.skip_ws(idx + 1);
                let (value, next) = self.scan_once(idx, depth)?;
                pairs.push((key, value));
                idx = self.skip_ws(next);
                if idx < self.len() && self.at(idx) == '}' {
                    break;
                }
                if idx >= self.len() || self.at(idx) != ',' {
                    return Err(ScanError::Decode("Expecting ',' delimiter", idx));
                }
                let comma = idx;
                idx = self.skip_ws(idx + 1);
                if idx < self.len() && self.at(idx) == '}' {
                    return Err(ScanError::Decode("Illegal trailing comma before end of object", comma));
                }
            }
        }
        Ok((J::Obj(pairs), idx + 1))
    }

    fn parse_array(&mut self, start: usize, depth: usize) -> Result<(J, usize), ScanError> {
        let mut items = Vec::new();
        let mut idx = self.skip_ws(start);
        if idx >= self.len() || self.at(idx) != ']' {
            loop {
                let (value, next) = self.scan_once(idx, depth)?;
                items.push(value);
                idx = self.skip_ws(next);
                if idx < self.len() && self.at(idx) == ']' {
                    break;
                }
                if idx >= self.len() || self.at(idx) != ',' {
                    return Err(ScanError::Decode("Expecting ',' delimiter", idx));
                }
                let comma = idx;
                idx = self.skip_ws(idx + 1);
                if idx < self.len() && self.at(idx) == ']' {
                    return Err(ScanError::Decode("Illegal trailing comma before end of array", comma));
                }
            }
        }
        Ok((J::Arr(items), idx + 1))
    }
}

/// `_json_path(path, key)`.
pub fn json_path(path: &str, key: &str) -> String {
    if crate::sidecar::KEYED_MAPS.contains(&path) {
        format!("{path}[{key}]")
    } else if path.is_empty() {
        key.to_string()
    } else {
        format!("{path}.{key}")
    }
}

fn decode_error_message(message: &str, s: &[char], pos: usize) -> String {
    let newlines = s[..pos].iter().filter(|&&c| c == '\n').count();
    let column = match s[..pos].iter().rposition(|&c| c == '\n') {
        Some(at) => pos - at,
        None => pos + 1,
    };
    format!("{message}: line {} column {} (char {})", newlines + 1, column, pos)
}

/// `json.loads(text)` for a str, as `parse_sidecar_json` calls it; Err holds str(error).
fn loads(s: &[char]) -> Result<(J, bool), Result<String, SidecarJsonError>> {
    if s.first() == Some(&'\u{feff}') {
        return Err(Ok(decode_error_message("Unexpected UTF-8 BOM (decode using utf-8-sig)", s, 0)));
    }
    let mut scanner = Scanner { s, lone: false };
    let start = scanner.skip_ws(0);
    let outcome = scanner.scan_once(start, 0);
    let (value, end) = match outcome {
        Ok(found) => found,
        Err(ScanError::Stop(pos)) => return Err(Ok(decode_error_message("Expecting value", s, pos))),
        Err(ScanError::Decode(message, pos)) => return Err(Ok(decode_error_message(message, s, pos))),
        Err(ScanError::Value(message)) => return Err(Ok(message)),
        Err(ScanError::TooDeep) => return Err(Err(SidecarJsonError::TooDeep)),
    };
    let end = scanner.skip_ws(end);
    if end != s.len() {
        return Err(Ok(decode_error_message("Extra data", s, end)));
    }
    Ok((value, scanner.lone))
}

fn build(node: J, path: &str, findings: &mut Vec<(String, String)>) -> Value {
    match node {
        J::Scalar(value) => value,
        J::Arr(items) => {
            Value::List(items.into_iter().enumerate().map(|(index, item)| build(item, &format!("{path}[{index}]"), findings)).collect())
        }
        J::Obj(pairs) => {
            // object_pairs_hook: first position, last value; a repeated key is recorded at each repetition
            let mut order: Vec<String> = Vec::new();
            let mut last: std::collections::HashMap<String, J> = std::collections::HashMap::new();
            let mut repeated: Vec<String> = Vec::new();
            for (key, value) in pairs {
                if last.contains_key(&key) {
                    if !repeated.contains(&key) {
                        repeated.push(key.clone());
                    }
                } else {
                    order.push(key.clone());
                }
                last.insert(key, value);
            }
            for key in &repeated {
                findings.push((
                    json_path(path, key),
                    "repeats a member name; JSON does not define which value counts (RFC 8259 §4), and the sidecar \
                     must state each once"
                        .into(),
                ));
            }
            let mut dict = Dict::new();
            for key in order {
                let value = last.remove(&key).expect("recorded");
                let child = build(value, &json_path(path, &key), findings);
                dict.insert(key, child);
            }
            Value::Dict(dict)
        }
    }
}

/// `parse_sidecar_json(text)`.
pub fn parse_sidecar_json(data: &[u8]) -> Result<Value, SidecarJsonError> {
    let text = match std::str::from_utf8(data) {
        Ok(text) => text,
        Err(error) => {
            return Err(SidecarJsonError::Invalid(vec![(
                "sidecar".into(),
                format!("is not strict UTF-8: the byte at offset {} cannot be decoded", error.valid_up_to()),
            )]))
        }
    };
    let chars: Vec<char> = text.chars().collect();
    let (node, lone) = match loads(&chars) {
        Ok(found) => found,
        Err(Ok(message)) => return Err(SidecarJsonError::Invalid(vec![("sidecar".into(), format!("is not JSON: {message}"))])),
        Err(Err(error)) => return Err(error),
    };
    if lone {
        return Err(SidecarJsonError::LoneSurrogate);
    }
    let mut findings = Vec::new();
    let value = build(node, "", &mut findings);
    if findings.is_empty() {
        Ok(value)
    } else {
        Err(SidecarJsonError::Invalid(findings))
    }
}
