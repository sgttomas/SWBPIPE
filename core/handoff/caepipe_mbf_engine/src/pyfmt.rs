//! Python's number formatting and `json.dumps` as the importer uses them.

use crate::pystr::{py_repr_str, PyError};
use crate::rational::{pow10, Q, INT_MAX_STR_DIGITS};
use crate::value::Value;
use num_bigint::BigInt;
use num_integer::Integer;

use std::cmp::Ordering;
use std::fmt::Write as _;

/// The shortest round-trip digits of a finite non-zero |x| (closest candidate, ties to even) and the decimal point
/// position n, x = 0.d1 d2 ... x 10^n.
fn shortest_digits(value: f64) -> (String, i64) {
    let mut buffer = ryu::Buffer::new();
    let text = buffer.format_finite(value.abs());
    let (mantissa, exponent) = match text.find(['e', 'E']) {
        Some(at) => (&text[..at], text[at + 1..].parse::<i64>().expect("ryu exponent")),
        None => (text, 0),
    };
    let (whole, fraction) = match mantissa.find('.') {
        Some(at) => (&mantissa[..at], &mantissa[at + 1..]),
        None => (mantissa, ""),
    };
    let all = format!("{whole}{fraction}");
    let leading = all.len() - all.trim_start_matches('0').len();
    let digits = all.trim_start_matches('0').trim_end_matches('0').to_string();
    let point = whole.len() as i64 + exponent - leading as i64;
    (digits, point)
}

/// `repr(x)` for a binary64, as Python writes it (also `str(x)` and json's number text). The text is read back
/// and must give `x` again, or the formatting fails closed.
pub fn py_float_repr(value: f64) -> Result<String, PyError> {
    if value.is_nan() {
        return Ok("nan".into());
    }
    if value.is_infinite() {
        return Ok(if value > 0.0 { "inf".into() } else { "-inf".into() });
    }
    if value == 0.0 {
        return Ok(if value.is_sign_negative() { "-0.0".into() } else { "0.0".into() });
    }
    let (digits, point) = shortest_digits(value);
    let sign = if value < 0.0 { "-" } else { "" };
    let count = digits.len() as i64;
    let body = if point <= -4 || point > 16 {
        let exponent = point - 1;
        let mantissa = if count > 1 { format!("{}.{}", &digits[..1], &digits[1..]) } else { digits.clone() };
        format!("{mantissa}e{}{:02}", if exponent < 0 { '-' } else { '+' }, exponent.abs())
    } else if point <= 0 {
        format!("0.{}{}", "0".repeat((-point) as usize), digits)
    } else if point >= count {
        format!("{}{}.0", digits, "0".repeat((point - count) as usize))
    } else {
        format!("{}.{}", &digits[..point as usize], &digits[point as usize..])
    };
    let text = format!("{sign}{body}");
    match text.parse::<f64>() {
        Ok(back) if back.to_bits() == value.to_bits() => Ok(text),
        _ => Err(PyError::value(format!("float formatting of {:#018x} does not read back", value.to_bits()))),
    }
}

/// floor(log10 |q|) for a positive exact q.
fn decimal_exponent(q: &Q) -> i64 {
    let approx = (q.numer_bits() as i64 - q.denom_bits() as i64) as f64 * std::f64::consts::LOG10_2;
    let mut exponent = approx.floor() as i64 - 1;
    while Q::pow10(exponent) > *q {
        exponent -= 1;
    }
    while Q::pow10(exponent + 1) <= *q {
        exponent += 1;
    }
    exponent
}

/// round_half_even(q) for q >= 0.
fn round_half_even(q: &Q) -> BigInt {
    let (whole, rest) = q.numer_ref().div_rem(q.denom_ref());
    match (&rest * BigInt::from(2)).cmp(q.denom_ref()) {
        Ordering::Less => whole,
        Ordering::Greater => whole + 1,
        Ordering::Equal => {
            if whole.is_odd() {
                whole + 1
            } else {
                whole
            }
        }
    }
}

/// `format(x, ".{precision}g")`.
pub fn py_format_g(value: f64, precision: usize) -> String {
    let precision = precision.max(1) as i64;
    if value.is_nan() {
        return "nan".into();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf".into() } else { "-inf".into() };
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    if value == 0.0 {
        return format!("{sign}0");
    }
    format_g_exact(sign, &Q::from_f64(value.abs()).expect("finite"), precision)
}

/// `format(q, ".{precision}g")` for an exact rational: Python's `Fraction.__format__` (3.12+) rounds the exact value
/// to `precision` significant figures, ties to even, and places it as `float.__format__` does, so the text is the
/// one [`py_format_g`] gives a binary64 of the same exact value.
pub fn py_format_g_q(value: &Q, precision: usize) -> String {
    let precision = precision.max(1) as i64;
    if value.is_zero() {
        return "0".into();
    }
    format_g_exact(if value.is_negative() { "-" } else { "" }, &value.abs(), precision)
}

/// The `g` text of the positive exact `exact` with its sign.
fn format_g_exact(sign: &str, exact: &Q, precision: i64) -> String {
    let mut exponent = decimal_exponent(exact);
    let mut digits = round_half_even(&exact.mul(&Q::pow10(precision - 1 - exponent)));
    if digits == pow10(precision as u64) {
        digits /= 10;
        exponent += 1;
    }
    let text = digits.to_string();
    if -4 <= exponent && exponent < precision {
        // fixed, precision - 1 - exponent decimals, trailing zeros removed
        let decimals = precision - 1 - exponent;
        let (whole, fraction) = if decimals > 0 {
            let padded = if (text.len() as i64) <= decimals {
                format!("{}{}", "0".repeat((decimals + 1 - text.len() as i64) as usize), text)
            } else {
                text.clone()
            };
            let split = padded.len() - decimals as usize;
            (padded[..split].to_string(), padded[split..].to_string())
        } else {
            (format!("{}{}", text, "0".repeat((-decimals) as usize)), String::new())
        };
        let fraction = fraction.trim_end_matches('0');
        if fraction.is_empty() {
            format!("{sign}{whole}")
        } else {
            format!("{sign}{whole}.{fraction}")
        }
    } else {
        let fraction = text[1..].trim_end_matches('0');
        let mantissa = if fraction.is_empty() { text[..1].to_string() } else { format!("{}.{}", &text[..1], fraction) };
        format!("{sign}{mantissa}e{}{:02}", if exponent < 0 { '-' } else { '+' }, exponent.abs())
    }
}

/// `str(int)` with Python's digit limit.
pub fn py_int_str(value: &BigInt) -> Result<String, PyError> {
    let text = value.to_string();
    let digits = text.trim_start_matches('-').len();
    if digits > INT_MAX_STR_DIGITS {
        return Err(PyError::value(format!(
            "Exceeds the limit ({INT_MAX_STR_DIGITS} digits) for integer string conversion; use \
             sys.set_int_max_str_digits() to increase the limit"
        )));
    }
    Ok(text)
}

/// `repr(value)` for a JSON-shaped value.
pub fn py_repr(value: &Value) -> Result<String, PyError> {
    Ok(match value {
        Value::Null => "None".into(),
        Value::Bool(true) => "True".into(),
        Value::Bool(false) => "False".into(),
        Value::Int(number) => py_int_str(number)?,
        Value::Float(number) => py_float_repr(*number)?,
        Value::Str(text) => py_repr_str(text),
        Value::List(items) => {
            let parts: Result<Vec<String>, PyError> = items.iter().map(py_repr).collect();
            format!("[{}]", parts?.join(", "))
        }
        Value::Dict(dict) => {
            let mut parts = Vec::new();
            for (key, item) in dict.iter() {
                parts.push(format!("{}: {}", py_repr_str(key), py_repr(item)?));
            }
            format!("{{{}}}", parts.join(", "))
        }
        Value::Bad => "<object object>".into(),
    })
}

/// json's `ensure_ascii` string encoding.
pub fn json_ascii_string(text: &str, out: &mut String) {
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            ' '..='~' => out.push(c),
            _ => {
                let mut units = [0u16; 2];
                for unit in c.encode_utf16(&mut units) {
                    let _ = write!(out, "\\u{:04x}", unit);
                }
            }
        }
    }
    out.push('"');
}

fn json_float(value: f64, allow_nan: bool) -> Result<String, PyError> {
    if value.is_finite() {
        return py_float_repr(value);
    }
    if !allow_nan {
        return Err(PyError::value(format!(
            "Out of range float values are not JSON compliant: {}",
            py_float_repr(value)?
        )));
    }
    Ok(if value.is_nan() {
        "NaN".into()
    } else if value > 0.0 {
        "Infinity".into()
    } else {
        "-Infinity".into()
    })
}

fn sorted_items(dict: &crate::value::Dict) -> Vec<(&String, &Value)> {
    let mut items: Vec<(&String, &Value)> = dict.iter().collect();
    items.sort_by(|a, b| a.0.cmp(b.0));
    items
}

fn dumps_indented(value: &Value, level: usize, out: &mut String) -> Result<(), PyError> {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Int(number) => out.push_str(&py_int_str(number)?),
        Value::Float(number) => out.push_str(&json_float(*number, false)?),
        Value::Str(text) => json_ascii_string(text, out),
        Value::List(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return Ok(());
            }
            out.push('[');
            let indent = "\n".to_string() + &"  ".repeat(level + 1);
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push_str(&indent);
                dumps_indented(item, level + 1, out)?;
            }
            out.push('\n');
            out.push_str(&"  ".repeat(level));
            out.push(']');
        }
        Value::Dict(dict) => {
            if dict.is_empty() {
                out.push_str("{}");
                return Ok(());
            }
            out.push('{');
            let indent = "\n".to_string() + &"  ".repeat(level + 1);
            for (index, (key, item)) in sorted_items(dict).into_iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push_str(&indent);
                json_ascii_string(key, out);
                out.push_str(": ");
                dumps_indented(item, level + 1, out)?;
            }
            out.push('\n');
            out.push_str(&"  ".repeat(level));
            out.push('}');
        }
        Value::Bad => return Err(PyError::type_error("Object of type object is not JSON serializable")),
    }
    Ok(())
}

/// `(json.dumps(value, sort_keys=True, indent=2, ensure_ascii=True, allow_nan=False) + "\n").encode("ascii")`.
pub fn canonical_bytes(value: &Value) -> Result<Vec<u8>, PyError> {
    let mut out = String::new();
    dumps_indented(value, 0, &mut out)?;
    out.push('\n');
    Ok(out.into_bytes())
}

/// `json.dumps(value, sort_keys=True)`: separators ", " and ": ", ASCII, NaN and Infinity allowed.
pub fn dumps_sorted(value: &Value) -> Result<String, PyError> {
    let mut out = String::new();
    dumps_sorted_into(value, &mut out)?;
    Ok(out)
}

fn dumps_sorted_into(value: &Value, out: &mut String) -> Result<(), PyError> {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Int(number) => out.push_str(&py_int_str(number)?),
        Value::Float(number) => out.push_str(&json_float(*number, true)?),
        Value::Str(text) => json_ascii_string(text, out),
        Value::List(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push_str(", ");
                }
                dumps_sorted_into(item, out)?;
            }
            out.push(']');
        }
        Value::Dict(dict) => {
            out.push('{');
            for (index, (key, item)) in sorted_items(dict).into_iter().enumerate() {
                if index > 0 {
                    out.push_str(", ");
                }
                json_ascii_string(key, out);
                out.push_str(": ");
                dumps_sorted_into(item, out)?;
            }
            out.push('}');
        }
        Value::Bad => return Err(PyError::type_error("Object of type object is not JSON serializable")),
    }
    Ok(())
}

