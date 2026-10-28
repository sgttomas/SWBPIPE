//! Python string semantics the importer's outputs depend on: Unicode `\d` and `\s`, `str.strip`, `repr(str)`, and
//! exact equivalents of the importer's regular expressions.

use crate::unicode_tables::{DECIMAL_DIGITS, NONPRINTABLE, PY_SPACE};

/// A Python exception the importer would raise (not catch): its type name and message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyError {
    pub kind: &'static str,
    pub message: String,
}

impl PyError {
    pub fn value(message: impl Into<String>) -> PyError {
        PyError { kind: "ValueError", message: message.into() }
    }
    pub fn overflow(message: impl Into<String>) -> PyError {
        PyError { kind: "OverflowError", message: message.into() }
    }
    pub fn type_error(message: impl Into<String>) -> PyError {
        PyError { kind: "TypeError", message: message.into() }
    }
}

/// `int(c)` for a character Python's `\d` matches.
pub fn decimal_value(c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        return Some(c as u32 - '0' as u32);
    }
    let code = c as u32;
    if code < 0x80 {
        return None;
    }
    let at = DECIMAL_DIGITS.partition_point(|&(_, high, _)| high < code);
    match DECIMAL_DIGITS.get(at) {
        Some(&(low, high, first)) if low <= code && code <= high => Some(first + (code - low)),
        _ => None,
    }
}

/// Python's `\d` for str patterns.
pub fn is_digit(c: char) -> bool {
    decimal_value(c).is_some()
}

/// Python's `str.isspace` (and `\s`).
pub fn is_py_space(c: char) -> bool {
    PY_SPACE.binary_search(&(c as u32)).is_ok()
}

/// `text.strip()`.
pub fn py_strip(text: &str) -> &str {
    text.trim_matches(is_py_space)
}

/// `text.rstrip(chars)` / `text.lstrip(chars)` with explicit characters.
pub fn rstrip_chars<'a>(text: &'a str, chars: &[char]) -> &'a str {
    text.trim_end_matches(|c| chars.contains(&c))
}
pub fn lstrip_chars<'a>(text: &'a str, chars: &[char]) -> &'a str {
    text.trim_start_matches(|c| chars.contains(&c))
}

/// Rust `str::trim().is_empty()` (the runtime's blank test): only Unicode White_Space.
pub fn rust_blank(text: &str) -> bool {
    text.trim().is_empty()
}

/// Python's `str.isprintable` for one character.
pub fn is_printable(c: char) -> bool {
    let code = c as u32;
    if code < 0x80 {
        return (0x20..0x7f).contains(&code);
    }
    let at = NONPRINTABLE.partition_point(|&(_, high)| high < code);
    !matches!(NONPRINTABLE.get(at), Some(&(low, high)) if low <= code && code <= high)
}

/// `repr(text)` for a str.
pub fn py_repr_str(text: &str) -> String {
    let quote = if text.contains('\'') && !text.contains('"') { '"' } else { '\'' };
    let mut out = String::with_capacity(text.len() + 2);
    out.push(quote);
    for c in text.chars() {
        let code = c as u32;
        if c == quote || c == '\\' {
            out.push('\\');
            out.push(c);
        } else if c == '\t' {
            out.push_str("\\t");
        } else if c == '\n' {
            out.push_str("\\n");
        } else if c == '\r' {
            out.push_str("\\r");
        } else if code < 0x20 || code == 0x7f {
            out.push_str(&format!("\\x{code:02x}"));
        } else if code < 0x7f || is_printable(c) {
            out.push(c);
        } else if code <= 0xff {
            out.push_str(&format!("\\x{code:02x}"));
        } else if code <= 0xffff {
            out.push_str(&format!("\\u{code:04x}"));
        } else {
            out.push_str(&format!("\\U{code:08x}"));
        }
    }
    out.push(quote);
    out
}

/// Python's `str.isupper()`: at least one cased character and no lowercase one.
pub fn py_isupper(text: &str) -> bool {
    let mut cased = false;
    for c in text.chars() {
        if c.is_lowercase() {
            return false;
        }
        if c.is_uppercase() {
            cased = true;
        }
    }
    cased
}

/// The first character and the rest (`text[0]`, `text[1:]`).
pub fn split_first(text: &str) -> Option<(char, &str)> {
    let mut chars = text.chars();
    let first = chars.next()?;
    Some((first, chars.as_str()))
}

/// `text[n:]` by characters.
pub fn skip_chars(text: &str, n: usize) -> &str {
    match text.char_indices().nth(n) {
        Some((at, _)) => &text[at..],
        None => "",
    }
}

/// `text[:n]` by characters.
pub fn take_chars(text: &str, n: usize) -> &str {
    match text.char_indices().nth(n) {
        Some((at, _)) => &text[..at],
        None => text,
    }
}

fn digits_prefix(text: &str) -> usize {
    text.char_indices().find(|&(_, c)| !is_digit(c)).map(|(at, _)| at).unwrap_or(text.len())
}

fn sign_prefix(text: &str) -> usize {
    if text.starts_with('+') || text.starts_with('-') {
        1
    } else {
        0
    }
}

/// `_REAL = [+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?\Z` (match from the start).
pub fn match_real(text: &str) -> bool {
    let mut rest = &text[sign_prefix(text)..];
    let whole = digits_prefix(rest);
    if whole > 0 {
        rest = &rest[whole..];
        if let Some(after) = rest.strip_prefix('.') {
            rest = &after[digits_prefix(after)..];
        }
    } else {
        let Some(after) = rest.strip_prefix('.') else { return false };
        let fraction = digits_prefix(after);
        if fraction == 0 {
            return false;
        }
        rest = &after[fraction..];
    }
    if rest.is_empty() {
        return true;
    }
    let Some(after) = rest.strip_prefix(['e', 'E']) else { return false };
    let after = &after[sign_prefix(after)..];
    let exponent = digits_prefix(after);
    exponent > 0 && exponent == after.len()
}

/// `_FEET_ONLY = ([+-]?)(\d+(?:\.\d+)?|\.\d+)\Z`: (sign, group 2).
pub fn match_feet_only(text: &str) -> Option<(&str, &str)> {
    let sign = sign_prefix(text);
    let body = &text[sign..];
    let whole = digits_prefix(body);
    let end = if whole > 0 {
        match body[whole..].strip_prefix('.') {
            Some(after) => {
                let fraction = digits_prefix(after);
                if fraction == 0 {
                    return None;
                }
                whole + 1 + fraction
            }
            None => whole,
        }
    } else {
        let after = body.strip_prefix('.')?;
        let fraction = digits_prefix(after);
        if fraction == 0 {
            return None;
        }
        1 + fraction
    };
    if end != body.len() {
        return None;
    }
    Some((&text[..sign], body))
}

/// The groups of `_FEET_INCH`.
pub struct FeetInch<'a> {
    pub sign: &'a str,
    pub feet: &'a str,
    pub inches: &'a str,
    pub fraction: Option<(&'a str, &'a str)>,
}

/// `_FEET_INCH = ([+-]?)(\d+)(['’-])(\d+)(?:-(\d+)/(\d+))?(["”]?)\Z`.
pub fn match_feet_inch(text: &str) -> Option<FeetInch<'_>> {
    let sign = sign_prefix(text);
    let body = &text[sign..];
    let feet = digits_prefix(body);
    if feet == 0 {
        return None;
    }
    let after_feet = &body[feet..];
    let (separator, after_separator) = split_first(after_feet)?;
    if !matches!(separator, '\'' | '\u{2019}' | '-') {
        return None;
    }
    let inches = digits_prefix(after_separator);
    if inches == 0 {
        return None;
    }
    let mut rest = &after_separator[inches..];
    let mut fraction = None;
    if let Some(after_dash) = rest.strip_prefix('-') {
        let numerator = digits_prefix(after_dash);
        if numerator > 0 {
            if let Some(after_slash) = after_dash[numerator..].strip_prefix('/') {
                let denominator = digits_prefix(after_slash);
                if denominator > 0 {
                    fraction = Some((&after_dash[..numerator], &after_slash[..denominator]));
                    rest = &after_slash[denominator..];
                }
            }
        }
    }
    if let Some((mark, after_mark)) = split_first(rest) {
        if !matches!(mark, '"' | '\u{201d}') || !after_mark.is_empty() {
            return None;
        }
    }
    Some(FeetInch { sign: &text[..sign], feet: &body[..feet], inches: &after_separator[..inches], fraction })
}

/// `_COMMENT = ([A-Z][A-Z0-9]*)\s*(?:=\s*(.*)|\((.*)\))?\Z` (re.S): (name, value, args).
pub fn match_comment(text: &str) -> Option<(&str, Option<&str>, Option<&str>)> {
    let first = text.chars().next()?;
    if !first.is_ascii_uppercase() {
        return None;
    }
    let name_end = text.char_indices().skip(1).find(|&(_, c)| !(c.is_ascii_uppercase() || c.is_ascii_digit()))
        .map(|(at, _)| at).unwrap_or(text.len());
    let name = &text[..name_end];
    let rest = text[name_end..].trim_start_matches(is_py_space);
    if rest.is_empty() {
        return Some((name, None, None));
    }
    if let Some(after) = rest.strip_prefix('=') {
        return Some((name, Some(after.trim_start_matches(is_py_space)), None));
    }
    if let Some(after) = rest.strip_prefix('(') {
        if let Some(inner) = after.strip_suffix(')') {
            return Some((name, None, Some(inner)));
        }
    }
    None
}

/// `_ROW_KEY = ([FTL])(\d+)(\*?)\Z`: (kind, digits, marker).
pub fn match_row_key(text: &str) -> Option<(char, &str, bool)> {
    let (kind, rest) = split_first(text)?;
    if !matches!(kind, 'F' | 'T' | 'L') {
        return None;
    }
    let digits = digits_prefix(rest);
    if digits == 0 {
        return None;
    }
    match &rest[digits..] {
        "" => Some((kind, &rest[..digits], false)),
        "*" => Some((kind, &rest[..digits], true)),
        _ => None,
    }
}

/// `_KEYWORD_LIKE = [A-Z][A-Z0-9]{2,}\Z`.
pub fn match_keyword_like(text: &str) -> bool {
    let mut chars = text.chars();
    match chars.next() {
        Some(c) if c.is_ascii_uppercase() => {}
        _ => return false,
    }
    let mut count = 0;
    for c in chars {
        if !(c.is_ascii_uppercase() || c.is_ascii_digit()) {
            return false;
        }
        count += 1;
    }
    count >= 2
}

/// `_SHA256 = [0-9a-f]{64}\Z`.
pub fn match_sha256(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// `re.search(r", line (\d+)\Z", text)`: the digits.
pub fn search_line_suffix(text: &str) -> Option<&str> {
    let start = text.char_indices().rev().take_while(|&(_, c)| is_digit(c)).last().map(|(at, _)| at)?;
    if text[..start].ends_with(", line ") {
        Some(&text[start..])
    } else {
        None
    }
}
