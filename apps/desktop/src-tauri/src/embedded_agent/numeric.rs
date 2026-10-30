//! Guard delivered JSON number tokens before this host constructs a `Value`.
//!
//! The caller first performs grammar/resource admission (the existing ingress
//! prescan); this is not another JSON parser or an admission reservation. It
//! rejects a token if serde_json's materialization and decimal rendering would
//! change its JSON decimal value. It cannot recover lexemes normalized by a
//! supplier before the RPC arrived. Native operation decimals remain strings.

use serde_json::Number;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.write_str("JSON number changes value during host materialization")
    }
}
impl std::error::Error for Error {}

/// Requires grammar-checked, byte-bounded input and reservation for the parse
/// and numeric scratch. Only one token's scratch is live at a time; string
/// contents are never interpreted as numbers.
pub fn check_admitted_json(raw: &[u8]) -> Result<(), Error> {
    let mut index = 0;
    let mut in_string = false;
    while index < raw.len() {
        match raw[index] {
            b'\\' if in_string => index = index.saturating_add(2),
            b'"' => {
                in_string = !in_string;
                index += 1;
            }
            b'-' | b'0'..=b'9' if !in_string => {
                let start = index;
                index += 1;
                while index < raw.len()
                    && matches!(raw[index], b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-')
                {
                    index += 1;
                }
                let token = &raw[start..index];
                let number: Number = serde_json::from_slice(token).map_err(|_| Error)?;
                let rendered = number.to_string();
                if Decimal::parse(token)? != Decimal::parse(rendered.as_bytes())? {
                    return Err(Error);
                }
            }
            _ => index += 1,
        }
    }
    Ok(())
}

/// Interpret an already checked JSON number as u64 using its decimal rendering,
/// rather than casting binary64. This preserves delivered JSON integer identity
/// when a legitimate integer uses fractional/exponent notation.
pub fn as_exact_u64(number: &Number) -> Option<u64> {
    let rendered = number.to_string();
    let decimal = Decimal::parse(rendered.as_bytes()).ok()?;
    if decimal.negative {
        return None;
    }
    if decimal.significant_start == decimal.significant_end {
        return Some(0);
    }
    let zeros = usize::try_from(decimal.exponent).ok()?;
    let digits = &decimal.digits[decimal.significant_start..decimal.significant_end];
    if digits.len().checked_add(zeros)? > 20 {
        return None;
    }
    let mut value = 0u64;
    for digit in digits {
        value = value
            .checked_mul(10)?
            .checked_add(u64::from(digit - b'0'))?;
    }
    for _ in 0..zeros {
        value = value.checked_mul(10)?;
    }
    Some(value)
}

/// Decimal identity, without converting the coefficient or exponent to f64.
/// Significant digits stay in a bounded buffer; an exponent changes a scalar,
/// never the size of that buffer.
struct Decimal {
    negative: bool,
    digits: Vec<u8>,
    significant_start: usize,
    significant_end: usize,
    exponent: i64,
}

impl Decimal {
    fn parse(token: &[u8]) -> Result<Self, Error> {
        let negative = token.first() == Some(&b'-');
        let body = if negative { &token[1..] } else { token };
        let exponent_at = body.iter().position(|b| matches!(b, b'e' | b'E'));
        let mantissa = &body[..exponent_at.unwrap_or(body.len())];
        let fractional = mantissa
            .iter()
            .position(|b| *b == b'.')
            .map_or(0, |point| mantissa.len() - point - 1);
        let mut digits = Vec::with_capacity(mantissa.len());
        for byte in mantissa {
            if byte.is_ascii_digit() {
                digits.push(*byte);
            } else if *byte != b'.' {
                return Err(Error);
            }
        }
        if digits.is_empty() {
            return Err(Error);
        }
        let Some(significant_start) = digits.iter().position(|b| *b != b'0') else {
            return Ok(Self {
                negative: false,
                digits,
                significant_start: 0,
                significant_end: 0,
                exponent: 0,
            });
        };
        let significant_end = digits.iter().rposition(|b| *b != b'0').unwrap() + 1;
        let explicit = match exponent_at {
            Some(at) => std::str::from_utf8(&body[at + 1..])
                .map_err(|_| Error)?
                .parse::<i64>()
                .map_err(|_| Error)?,
            None => 0,
        };
        let exponent = explicit
            .checked_sub(i64::try_from(fractional).map_err(|_| Error)?)
            .and_then(|n| n.checked_add(i64::try_from(digits.len() - significant_end).ok()?))
            .ok_or(Error)?;
        Ok(Self {
            negative,
            digits,
            significant_start,
            significant_end,
            exponent,
        })
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.negative == other.negative
            && self.exponent == other.exponent
            && self.digits[self.significant_start..self.significant_end]
                == other.digits[other.significant_start..other.significant_end]
    }
}
impl Eq for Decimal {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delivered_integer_and_boundary_aliases_are_refused() {
        for token in [
            "1.0000000000000000001",
            "0.9999999999999999999",
            "50.000000000000000001",
            "49.999999999999999999",
            "9007199254740993.0",
            "18446744073709551616",
            "1e-999",
        ] {
            assert!(check_admitted_json(token.as_bytes()).is_err(), "{token}");
        }
    }

    #[test]
    fn equivalent_decimal_spellings_and_generated_values_are_preserved() {
        for token in [
            "0",
            "-0",
            "1.0",
            "1e2",
            "0.1",
            "1.2300",
            "1.0000000000000002",
            "9007199254740993",
            "18446744073709551615",
            "-9223372036854775808",
            "5e-324",
            "1e308",
        ] {
            assert!(check_admitted_json(token.as_bytes()).is_ok(), "{token}");
        }
    }

    #[test]
    fn decimal_strings_and_escaped_quotes_are_not_reinterpreted() {
        let raw =
            br#"{"decimal":"1.0000000000000000001","quote":"\"50.0000000000000001\"","n":1.0}"#;
        assert!(check_admitted_json(raw).is_ok());
        assert!(check_admitted_json(br#"{"items":[0,{"n":0.9999999999999999999}]}"#).is_err());
    }

    #[test]
    fn exponent_and_trailing_zero_normalization_is_exact() {
        assert!(Decimal::parse(b"-1200.00e-2").unwrap() == Decimal::parse(b"-12").unwrap());
        assert!(Decimal::parse(b"1000e-3").unwrap() == Decimal::parse(b"1.0").unwrap());
        assert!(Decimal::parse(b"-0.00e+1000").unwrap() == Decimal::parse(b"0").unwrap());
        assert!(Decimal::parse(b"1e9223372036854775808").is_err());
    }

    #[test]
    fn counter_identity_uses_checked_json_decimal_not_float_cast() {
        for (token, expected) in [
            ("1.0", Some(1)),
            ("1e2", Some(100)),
            ("-0", Some(0)),
            ("1.5", None),
            ("-1", None),
            ("18446744073709551615", Some(u64::MAX)),
            ("1.844674407370955e19", Some(18_446_744_073_709_550_000)),
            ("1e30", None),
        ] {
            check_admitted_json(token.as_bytes()).unwrap();
            let number: Number = serde_json::from_str(token).unwrap();
            assert_eq!(as_exact_u64(&number), expected, "{token}");
        }
    }
}
