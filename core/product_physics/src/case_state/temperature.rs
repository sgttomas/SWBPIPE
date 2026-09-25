//! Exact temperature identity for selection decisions.
//!
//! Equality, bracketing, endpoint and actual-versus-selected decisions must not
//! depend on binary64 affine-conversion rounding (for example 242 degC versus
//! 467.6 degF). Each authored absolute temperature becomes an exact rational
//! kelvin value from the shortest round-trip decimal of its parsed number and
//! the unit's exact affine definition. Exactly equal temperatures then share
//! one binary64 representative; numerical evaluation stays in binary64. There
//! is no tolerance snapping: distinct exact temperatures stay distinct, and a
//! set whose binary64 representatives cannot preserve their exact order is
//! refused.
use crate::Quantity;
use open_pipe_stress_units::{unit_by_symbol, Dimension, UnitId};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Rational {
    num: i128,
    den: i128,
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl Rational {
    fn new(num: i128, den: i128) -> Option<Self> {
        if den == 0 {
            return None;
        }
        let sign = if den < 0 { -1 } else { 1 };
        let divisor = gcd(num, den).max(1);
        Some(Self {
            num: sign * (num / divisor),
            den: sign * (den / divisor),
        })
    }
    fn add(self, other: Self) -> Option<Self> {
        let num = self
            .num
            .checked_mul(other.den)?
            .checked_add(other.num.checked_mul(self.den)?)?;
        Self::new(num, self.den.checked_mul(other.den)?)
    }
    fn mul(self, other: Self) -> Option<Self> {
        Self::new(
            self.num.checked_mul(other.num)?,
            self.den.checked_mul(other.den)?,
        )
    }
    fn compare(self, other: Self) -> Option<Ordering> {
        Some(
            self.num
                .checked_mul(other.den)?
                .cmp(&other.num.checked_mul(self.den)?),
        )
    }
    /// Exact value of the shortest round-trip decimal of a finite binary64.
    fn from_authored(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let text = format!("{value:e}");
        let (mantissa, exponent) = text.split_once('e')?;
        let exponent: i32 = exponent.parse().ok()?;
        let negative = mantissa.starts_with('-');
        let mantissa = mantissa.trim_start_matches('-');
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let digits: i128 = format!("{whole}{fraction}").parse().ok()?;
        let scale = exponent - fraction.len() as i32;
        let power = 10i128.checked_pow(scale.unsigned_abs())?;
        let signed = if negative { -digits } else { digits };
        if scale >= 0 {
            Self::new(signed.checked_mul(power)?, 1)
        } else {
            Self::new(signed, power)
        }
    }
}

/// Exact kelvin value of an authored absolute temperature, using the units
/// catalog only to identify the unit and exact affine definitions to convert.
pub(crate) fn exact_kelvin(quantity: &Quantity) -> Result<Rational, String> {
    let unit = unit_by_symbol(&quantity.unit, Dimension::Temperature).map_err(|e| e.to_string())?;
    let value = Rational::from_authored(quantity.value)
        .ok_or("temperature is not finite or its decimal identity exceeds the exact range")?;
    let r = |num, den| Rational::new(num, den).expect("nonzero literal denominator");
    let kelvin = match unit {
        UnitId::Kelvin => Some(value),
        UnitId::DegreeCelsius => value.add(r(27315, 100)),
        UnitId::DegreeFahrenheit => value.add(r(45967, 100)).and_then(|v| v.mul(r(5, 9))),
        UnitId::DegreeRankine => value.mul(r(5, 9)),
        other => {
            return Err(format!(
                "no exact absolute-temperature definition for {other:?}"
            ))
        }
    }
    .ok_or("temperature decimal identity exceeds the exact range")?;
    if kelvin.num < 0 {
        return Err("absolute temperature cannot be below zero kelvin".into());
    }
    Ok(kelvin)
}

/// One member's consumed absolute temperatures, keyed by exact identity.
#[derive(Debug, Default)]
pub(crate) struct TemperatureIdentity {
    classes: Vec<(Rational, f64)>,
}

impl TemperatureIdentity {
    /// Register a temperature with its ordinary binary64 kelvin conversion.
    /// The first binary64 seen for an exact value becomes its representative.
    pub(crate) fn register(
        &mut self,
        quantity: &Quantity,
        binary64_kelvin: f64,
    ) -> Result<(), String> {
        let exact = exact_kelvin(quantity)?;
        if !self.classes.iter().any(|(value, _)| *value == exact) {
            self.classes.push((exact, binary64_kelvin));
        }
        Ok(())
    }
    /// Refuse when binary64 representatives cannot preserve the exact order.
    pub(crate) fn check_order(&mut self) -> Result<(), String> {
        let mut failure = None;
        self.classes.sort_by(|a, b| {
            a.0.compare(b.0).unwrap_or_else(|| {
                failure = Some("temperature identity comparison exceeds the exact range");
                Ordering::Equal
            })
        });
        if let Some(message) = failure {
            return Err(message.into());
        }
        if self.classes.windows(2).any(|pair| pair[0].1 >= pair[1].1) {
            return Err("distinct exact temperatures do not keep a strictly increasing binary64 representation; no tolerance snapping is applied".into());
        }
        Ok(())
    }
    /// Canonical binary64 kelvin for an authored temperature already registered.
    pub(crate) fn kelvin(&self, quantity: &Quantity) -> Result<f64, String> {
        let exact = exact_kelvin(quantity)?;
        self.classes
            .iter()
            .find(|(value, _)| *value == exact)
            .map(|(_, binary)| *binary)
            .ok_or_else(|| "temperature was not registered for identity".into())
    }
    /// The same temperature as a canonical kelvin quantity for the selector.
    pub(crate) fn canonical(&self, quantity: &Quantity) -> Result<Quantity, String> {
        Ok(Quantity {
            value: self.kelvin(quantity)?,
            unit: "K".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn q(value: f64, unit: &str) -> Quantity {
        Quantity {
            value,
            unit: unit.into(),
        }
    }
    #[test]
    fn equivalent_authored_units_have_one_exact_identity() {
        // Reviewer regressions: each pair is exactly equal but some differ by
        // an ulp after ordinary binary64 affine conversion.
        for (a, b) in [
            (q(-50.0, "degC"), q(223.15, "K")),
            (q(242.0, "degC"), q(467.6, "degF")),
            (q(467.6, "degF"), q(515.15, "K")),
            (q(20.0, "degC"), q(527.67, "degR")),
            (q(20.0, "C"), q(293.15, "K")),
        ] {
            assert_eq!(exact_kelvin(&a).unwrap(), exact_kelvin(&b).unwrap());
        }
        assert_ne!(
            exact_kelvin(&q(20.0, "degC")).unwrap(),
            exact_kelvin(&q(293.1500000000001, "K")).unwrap()
        );
        assert!(exact_kelvin(&q(-1.0, "K")).is_err());
        assert!(exact_kelvin(&q(20.0, "m")).is_err());
        assert!(exact_kelvin(&q(f64::NAN, "K")).is_err());
        assert!(exact_kelvin(&q(1e300, "K")).is_err());
    }
    #[test]
    fn exactly_equal_temperatures_share_a_representative_and_order_is_checked() {
        let mut identity = TemperatureIdentity::default();
        let a = q(242.0, "degC");
        let b = q(467.6, "degF");
        identity.register(&a, 242.0 + 273.15).unwrap();
        identity.register(&b, (467.6 + 459.67) * 5.0 / 9.0).unwrap();
        identity.register(&q(20.0, "degC"), 293.15).unwrap();
        identity.check_order().unwrap();
        assert_eq!(identity.kelvin(&a).unwrap(), identity.kelvin(&b).unwrap());
        assert!(identity.kelvin(&q(1.0, "K")).is_err());
        // Distinct exact values whose representatives do not preserve order are refused.
        let mut inverted = TemperatureIdentity::default();
        inverted.register(&q(300.0, "K"), 300.0).unwrap();
        inverted.register(&q(301.0, "K"), 300.0).unwrap();
        assert!(inverted.check_order().is_err());
    }
}
