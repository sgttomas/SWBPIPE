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

use serde_json::{Number, Value};

/// Render a JSON value as RFC 8785 canonical JSON text.
pub fn canonical_json(value: &Value) -> String {
    let mut out = String::new();
    write_canonical(value, &mut out);
    out
}

fn write_canonical(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => out.push_str(&canonical_number(number)),
        Value::String(text) => {
            out.push_str(&serde_json::to_string(text).expect("strings always encode as JSON"))
        }
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_canonical(item, out);
            }
            out.push(']');
        }
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            // RFC 8785 §3.2.3: property names sort as arrays of UTF-16 code
            // units (what ECMAScript string comparison does), not as Unicode
            // code points — the two differ for supplementary-plane keys.
            keys.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
            out.push('{');
            for (index, key) in keys.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push_str(&serde_json::to_string(key).expect("keys always encode as JSON"));
                out.push(':');
                write_canonical(&map[key.as_str()], out);
            }
            out.push('}');
        }
    }
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
