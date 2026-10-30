//! Exact temperature identity for selection decisions.
//!
//! Equality, bracketing, endpoint and actual-versus-selected decisions must not
//! depend on binary64 affine-conversion rounding (for example 242 degC versus
//! 467.6 degF). Each authored absolute temperature becomes an exact kelvin value
//! N / (9·10^24) from the canonical decimal of its parsed number (DEL-02-01
//! Design `model-document-0.9.0.md` §1.5.1: the closest shortest round-trip
//! decimal, exact ties to the even digit, never a float formatter) and the
//! unit's exact affine definition. The domain is |value| <= 10^6 in the authored
//! unit with a canonical decimal exponent >= -24; a temperature outside it is
//! refused with its own code, `LOAD_STATE_TEMPERATURE_RANGE_UNSUPPORTED`.
//! Exactly equal temperatures then share one binary64 representative;
//! numerical evaluation stays in binary64. There is no tolerance snapping:
//! distinct exact temperatures stay distinct, and a set whose binary64
//! representatives cannot preserve their exact order is refused
//! (`LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED`).
pub(crate) mod canonical_decimal;

use crate::Quantity;
use canonical_decimal::{canonical_decimal, kelvin_numerator, CanonicalError};
use open_pipe_stress_units::{unit_by_symbol, Dimension};

/// Exact kelvin value N / `canonical_decimal::KELVIN_DENOMINATOR`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ExactKelvin(i128);

/// Code of a temperature outside the exact identity's domain.
pub(crate) const TEMPERATURE_RANGE_UNSUPPORTED: &str = "LOAD_STATE_TEMPERATURE_RANGE_UNSUPPORTED";
/// Code of every other temperature identity refusal.
pub(crate) const TEMPERATURE_IDENTITY_UNRESOLVED: &str =
    "LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED";
const RANGE_MESSAGE: &str = "absolute temperature is outside the exact identity domain (|value| <= 1e6 in its authored unit, canonical decimal exponent >= -24)";

/// Exact kelvin value of an authored absolute temperature, using the units
/// catalog only to identify the unit and exact affine definitions to convert.
/// A refusal carries its diagnostic code.
pub(crate) fn exact_kelvin(quantity: &Quantity) -> Result<ExactKelvin, (&'static str, String)> {
    let unresolved = |message: String| (TEMPERATURE_IDENTITY_UNRESOLVED, message);
    let unit = unit_by_symbol(&quantity.unit, Dimension::Temperature)
        .map_err(|e| unresolved(e.to_string()))?;
    let decimal = canonical_decimal(quantity.value).map_err(|error| match error {
        CanonicalError::NonFinite => unresolved("temperature is not finite".into()),
        CanonicalError::RangeUnsupported => (TEMPERATURE_RANGE_UNSUPPORTED, RANGE_MESSAGE.into()),
    })?;
    let kelvin = kelvin_numerator(decimal, unit).ok_or_else(|| {
        unresolved(format!(
            "no exact absolute-temperature definition for {unit:?}"
        ))
    })?;
    if kelvin < 0 {
        return Err(unresolved(
            "absolute temperature cannot be below zero kelvin".into(),
        ));
    }
    Ok(ExactKelvin(kelvin))
}

/// One member's consumed absolute temperatures, keyed by exact identity.
#[derive(Debug, Default)]
pub(crate) struct TemperatureIdentity {
    classes: Vec<(ExactKelvin, f64)>,
}

impl TemperatureIdentity {
    /// Register a temperature with its ordinary binary64 kelvin conversion.
    /// The first binary64 seen for an exact value becomes its representative.
    pub(crate) fn register(
        &mut self,
        quantity: &Quantity,
        binary64_kelvin: f64,
    ) -> Result<(), (&'static str, String)> {
        let exact = exact_kelvin(quantity)?;
        if !self.classes.iter().any(|(value, _)| *value == exact) {
            self.classes.push((exact, binary64_kelvin));
        }
        Ok(())
    }
    /// Refuse when binary64 representatives cannot preserve the exact order.
    pub(crate) fn check_order(&mut self) -> Result<(), String> {
        self.classes.sort_by_key(|class| class.0);
        if self.classes.windows(2).any(|pair| pair[0].1 >= pair[1].1) {
            return Err("distinct exact temperatures do not keep a strictly increasing binary64 representation; no tolerance snapping is applied".into());
        }
        Ok(())
    }
    /// Canonical binary64 kelvin for an authored temperature already registered.
    pub(crate) fn kelvin(&self, quantity: &Quantity) -> Result<f64, String> {
        let exact = exact_kelvin(quantity).map_err(|(_, message)| message)?;
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
    use open_pipe_stress_units::{convert_for_dimension, UnitId};
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
        assert_eq!(
            exact_kelvin(&q(1e300, "K")).unwrap_err().0,
            "LOAD_STATE_TEMPERATURE_RANGE_UNSUPPORTED"
        );
        assert_eq!(
            exact_kelvin(&q(-1.0, "K")).unwrap_err().0,
            "LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED"
        );
    }
    /// The ordinary binary64 kelvin conversion, as the resolver registers it.
    fn binary(t: &Quantity) -> f64 {
        let from = unit_by_symbol(&t.unit, Dimension::Temperature).unwrap();
        convert_for_dimension(t.value, Dimension::Temperature, from, UnitId::Kelvin).unwrap()
    }
    fn identity_of(temperatures: &[Quantity]) -> Result<TemperatureIdentity, String> {
        let mut identity = TemperatureIdentity::default();
        for t in temperatures {
            identity
                .register(t, binary(t))
                .map_err(|(_, message)| message)?;
        }
        identity.check_order()?;
        Ok(identity)
    }
    #[test]
    fn sixteen_and_seventeen_digit_ties_take_the_canonical_decimal() {
        // P1: exactly equal under the canonical decimal, one shared representative.
        let (k, c) = (
            q(f64::from_bits(0x409f400010000000), "K"),
            q(1726.8500610351562, "degC"),
        );
        let identity = identity_of(&[k.clone(), c.clone()]).unwrap();
        assert_eq!(exact_kelvin(&k), exact_kelvin(&c));
        assert_eq!(identity.kelvin(&c), identity.kelvin(&k));
        // P5: P1's pair with 20 degC: two classes, ordered and admitted.
        let warm = q(20.0, "degC");
        let identity = identity_of(&[k.clone(), c.clone(), warm.clone()]).unwrap();
        assert_eq!(identity.classes.len(), 2);
        assert_eq!(identity.kelvin(&c), identity.kelvin(&k));
        assert!(exact_kelvin(&warm).unwrap() < exact_kelvin(&k).unwrap());
        assert!(identity.kelvin(&warm).unwrap() < identity.kelvin(&k).unwrap());
        // P6: the degC member takes the K member's representative.
        let (k, c) = (
            q(f64::from_bits(0x408fc00020000000), "K"),
            q(f64::from_bits(0x408736cceccccccc), "degC"),
        );
        let identity = identity_of(&[k, c.clone()]).unwrap();
        assert_eq!(identity.kelvin(&c).unwrap().to_bits(), 0x408fc00020000000);
        // P2, P3: distinct exact values with no strictly increasing representatives.
        for pair in [
            [q(2000.0000610351562, "degC"), q(2273.1500610351563, "K")],
            [q(2000.0000610351562, "degF"), q(2459.6700610351563, "degR")],
        ] {
            assert_ne!(exact_kelvin(&pair[0]), exact_kelvin(&pair[1]));
            assert!(identity_of(&pair).is_err(), "{pair:?}");
        }
        // P7: the canonical order agrees with the representatives' order.
        let (k, r) = (
            q(f64::from_bits(0x409f400110000000), "K"),
            q(3600.0018676757813, "degR"),
        );
        identity_of(&[k.clone(), r.clone()]).unwrap();
        assert!(exact_kelvin(&k).unwrap() < exact_kelvin(&r).unwrap());
        // P4: the integer form compares any two in-domain values without overflow.
        let (c, k) = (q(f64::from_bits(0x3f80010000000000), "degC"), q(1e-18, "K"));
        identity_of(&[c.clone(), k.clone()]).unwrap();
        assert!(exact_kelvin(&k).unwrap() < exact_kelvin(&c).unwrap());
        // W-17's ties, stored as their canonical decimals' exact kelvin values.
        let n = |digits: i128, exponent: u32| Ok(ExactKelvin(9 * digits * 10i128.pow(exponent)));
        assert_eq!(
            exact_kelvin(&q(f64::from_bits(0x3f80010000000000), "K")),
            n(7814407348632812, 6)
        );
        assert_eq!(
            exact_kelvin(&q(f64::from_bits(0x3f50640000000000), "K")),
            n(10004043579101562, 5)
        );
        assert_eq!(
            exact_kelvin(&q(f64::from_bits(0x3e60000000000000), "K")),
            n(29802322387695312, 0)
        );
        assert_eq!(
            exact_kelvin(&q(f64::from_bits(0x3e70000000000000), "K")),
            n(5960464477539063, 1)
        );
    }
    #[test]
    fn the_domain_is_refused_by_name_in_the_authored_unit() {
        let refused = Err((
            "LOAD_STATE_TEMPERATURE_RANGE_UNSUPPORTED",
            RANGE_MESSAGE.to_string(),
        ));
        let above = f64::from_bits(1e6f64.to_bits() + 1);
        for unit in ["K", "degC", "degF", "degR"] {
            assert!(exact_kelvin(&q(1e6, unit)).is_ok(), "{unit}");
            assert_eq!(exact_kelvin(&q(above, unit)), refused, "{unit}");
        }
        assert_eq!(exact_kelvin(&q(1e-24, "K")), Ok(ExactKelvin(9)));
        assert_eq!(exact_kelvin(&q(1e-25, "K")), refused);
        assert_eq!(
            exact_kelvin(&q(20.000000000000004, "degC")).map(|_| ()),
            Ok(())
        );
        let mut identity = TemperatureIdentity::default();
        assert_eq!(
            identity.register(&q(above, "K"), above),
            refused.map(|_| ())
        );
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
