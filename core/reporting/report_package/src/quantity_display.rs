//! The report's display rule for result-row values and exact stress headlines
//! (DEL-08-01 Design `report-package-current-results.md` §2.3 item 6 and the
//! display resolution beside it; `exact-route-report-sections.md` §3.3).
//!
//! Display only: stored values and units, the package's machine-readable
//! members, hashes and identifiers never change. The rule starts from the
//! value's canonical text (the ECMAScript Number-to-String form,
//! `canonical_number`) and works on its decimal digits as text, with no
//! floating-point arithmetic:
//! - a value stored in `Pa` is shifted six decimal places and labelled `MPa`
//!   (1 MPa = 10^6 Pa, exact);
//! - the decimal significand is rounded to six significant digits, half away
//!   from zero;
//! - the result is printed with the ECMAScript Number-to-String layout (plain
//!   decimal for decimal-point positions in (-6, 21], exponent form with an
//!   explicit sign otherwise; trailing zeros dropped); 0 and -0 print "0".
//!
//! The desktop applies the identical rule
//! (`apps/desktop/src/features/report/reportQuantityDisplay.ts`); both are
//! checked against `fixtures/reports/invented/report_quantity_display_parity.json`.

use open_pipe_stress_canonical_json::canonical_number;
use serde_json::{Number, Value};

/// The report's display policy text (§2.3 item 5), the renderer's: the
/// producer requires it, and the renderer states the display rule near the
/// results when a report declares it.
pub const QUANTITY_DISPLAY_POLICY: &str = open_pipe_stress_report_renderer::QUANTITY_DISPLAY_POLICY;

/// The stored unit that is displayed scaled.
pub const PASCAL: &str = "Pa";
/// The unit a `Pa` value is displayed in.
pub const MEGAPASCAL: &str = "MPa";
/// Displayed values keep this many significant digits.
pub const SIGNIFICANT_DIGITS: usize = 6;
/// Decimal places a `Pa` value is shifted for display in `MPa`.
const PASCAL_TO_MEGAPASCAL_SHIFT: i64 = 6;

/// The displayed text `<display value> <display unit>` of a stored value and
/// its stored unit.
pub fn display_quantity(value: &Number, unit: &str) -> String {
    let (shift, shown_unit) = if unit == PASCAL { (PASCAL_TO_MEGAPASCAL_SHIFT, MEGAPASCAL) } else { (0, unit) };
    format!("{} {shown_unit}", display_number(&canonical_number(value), shift))
}

/// Whether a value stored in `unit` is displayed scaled (`conversion_performed`).
pub fn is_displayed_scaled(unit: &str) -> bool {
    unit == PASCAL
}

/// Whether the report built from `source` displays a value stored in `Pa`:
/// a result row, or, on an exact-route source, a stress headline that is not
/// withheld (§2.3 item 5, `conversion_performed`).
pub fn source_displays_scaled_value(source: &Value) -> bool {
    let rows = source["results"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    if rows.iter().any(|row| row["unit"].as_str().is_some_and(is_displayed_scaled)) {
        return true;
    }
    let exact = source
        .pointer("/producer/semantic_contract_id")
        .and_then(Value::as_str)
        .is_some_and(crate::exact_sections::is_exact_identity);
    exact
        && ["max_open_formula_stress_nominal_section", "max_open_formula_stress_reduced_section"]
            .iter()
            .any(|key| source["summary"][*key]["unit"].as_str().is_some_and(is_displayed_scaled))
}

/// The display rule on a canonical number text, after shifting the decimal
/// point `shift` places to the left. `canonical` is ECMAScript
/// Number-to-String output (`-?digits[.digits][e(+|-)digits]`).
pub fn display_number(canonical: &str, shift: i64) -> String {
    let (negative, unsigned) = match canonical.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, canonical),
    };
    let (mantissa, exponent) = match unsigned.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (
            mantissa,
            exponent.trim_start_matches('+').parse::<i64>().expect("a canonical number's exponent"),
        ),
        None => (unsigned, 0),
    };
    let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    assert!(
        !integer.is_empty() && integer.bytes().chain(fraction.bytes()).all(|byte| byte.is_ascii_digit()),
        "a canonical number text"
    );
    // value = 0.<digits> x 10^point (ECMA-262 Number::toString's s, k and n).
    let all: String = format!("{integer}{fraction}");
    let leading = all.bytes().take_while(|byte| *byte == b'0').count();
    let digits = all[leading..].trim_end_matches('0');
    if digits.is_empty() {
        return "0".to_string();
    }
    let mut point = integer.len() as i64 + exponent - leading as i64 - shift;
    let mut digits: Vec<u8> = digits.bytes().collect();
    if digits.len() > SIGNIFICANT_DIGITS {
        let round_up = digits[SIGNIFICANT_DIGITS] >= b'5';
        digits.truncate(SIGNIFICANT_DIGITS);
        if round_up {
            let mut index = SIGNIFICANT_DIGITS;
            loop {
                if index == 0 {
                    // Every kept digit was 9: the significand becomes 1 and the
                    // decimal point moves one place.
                    digits.insert(0, b'1');
                    digits.truncate(SIGNIFICANT_DIGITS);
                    point += 1;
                    break;
                }
                index -= 1;
                if digits[index] == b'9' {
                    digits[index] = b'0';
                } else {
                    digits[index] += 1;
                    break;
                }
            }
        }
        while digits.last() == Some(&b'0') {
            digits.pop();
        }
    }
    let digits = String::from_utf8(digits).expect("ASCII digits");
    let k = digits.len() as i64;
    let mut text = String::new();
    if negative {
        text.push('-');
    }
    if k <= point && point <= 21 {
        text.push_str(&digits);
        text.push_str(&"0".repeat((point - k) as usize));
    } else if 0 < point && point <= 21 {
        text.push_str(&digits[..point as usize]);
        text.push('.');
        text.push_str(&digits[point as usize..]);
    } else if -6 < point && point <= 0 {
        text.push_str("0.");
        text.push_str(&"0".repeat((-point) as usize));
        text.push_str(&digits);
    } else {
        text.push_str(&digits[..1]);
        if k > 1 {
            text.push('.');
            text.push_str(&digits[1..]);
        }
        let exponent = point - 1;
        text.push('e');
        text.push(if exponent >= 0 { '+' } else { '-' });
        text.push_str(&exponent.abs().to_string());
    }
    text
}
