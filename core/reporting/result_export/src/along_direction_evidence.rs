//! R-6/R-7 S-K1: the
//! independent reader's re-check of `support_reaction_along_direction_v1`. It imports
//! no product code: the row's exact strings, one row per skewed record and case beside
//! that record's Fx/Fy/Fz rows, and, with the model, s·|d| = dᵀR within a
//! representation guard (d the record's authored `direction`, R its three global
//! support-on-pipe component rows).
use serde_json::Value;
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;

pub(crate) const KIND: &str = "support_reaction_along_direction_v1";
const COMPONENT: &str = "along_direction";
const COORDINATE_SYSTEM: &str = "support_direction";
const SIGN: &str = "support_on_pipe_positive_along_authored_direction";
const BASIS: &str = "recovered_from_assembled_support_law";
const SUPPORT_COMPONENT: &str = "support_reaction_component_v2";

/// This module's refusal by code. Named for the module (not `require`): the retained
/// pricing's call graph resolves calls by name, and every reader has a private `require`.
fn along_require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(code.to_string())
    }
}

/// The row's declared semantics: a case row's strings exactly; a combination row's
/// component, coordinate system, location and unit.
pub(crate) fn row_semantics(row: &Value, direct: bool) -> Check {
    let md = &row["metadata"];
    along_require(
        row["unit"] == "N" && md["component"] == COMPONENT && md["coordinate_system"] == COORDINATE_SYSTEM && md["location"] == "node",
        "ROW_SEMANTICS",
    )?;
    if direct {
        along_require(md["basis"] == BASIS && md["sign_convention"] == SIGN, "ROW_SEMANTICS")?;
    }
    Ok(())
}

/// A skewed record's authored direction: two or more nonzero components (an exactly
/// axis-aligned direction is lowered and publishes no row).
/// Whether the document or its model has anything for `check_case` to read: an
/// along-direction row, or a skewed record in the model. Without either every case's
/// check is a no-op, so the reader groups nothing (nothing is formed on D1).
pub(crate) fn applies(rows: &[Value], model: Option<&Value>) -> bool {
    rows.iter().any(|row| row["kind"] == KIND)
        || model.is_some_and(|m| m["supports"].as_array().into_iter().flatten().any(|s| skewed_direction(s).is_some()))
}

fn skewed_direction(support: &Value) -> Option<[f64; 3]> {
    let d = &support["direction"];
    let v = [d["x"].as_f64()?, d["y"].as_f64()?, d["z"].as_f64()?];
    (v.iter().filter(|c| **c != 0.0).count() >= 2 && support["family"] != "snubber").then_some(v)
}

/// The row id's exact source identity: `{L(part)}:{part}` concatenated, L the UTF-8 byte
/// length (the agreed `exact_source_identity([case, support])`, no separator).
fn exact_source_identity(parts: &[&str]) -> String {
    parts.iter().map(|part| format!("{}:{part}", part.len())).collect()
}

/// d scaled by a power of two to max|d_i| ∈ [1, 2): s·|d| = dᵀR is invariant under a
/// positive scale, and the scaled norm never under- or overflows.
///
/// Astra re-check item 3: the scaling is exact on the admitted domain only, every nonzero
/// component of the canonical direction normal. There each step down keeps every
/// intermediate between the original and the final normal value, and each step up is exact,
/// so no admitted component is rounded. Outside it, for example d = (8, 5·2^-1074, 0), repeated
/// halving may round or flush a component that becomes subnormal; the product refuses such a
/// direction (`SUPPORT_DIRECTION_RANGE_UNSUPPORTED`) and publishes no row. The Python
/// verifier repeats these steps one for one (re-check 2), so both readers form the same bits
/// for every input.
fn canonical(mut d: [f64; 3]) -> [f64; 3] {
    let largest = |d: &[f64; 3]| d.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
    while largest(&d) >= 2.0 {
        let step = if largest(&d) >= 2f64.powi(512) { 2f64.powi(-511) } else { 0.5 };
        d = d.map(|x| x * step);
    }
    while largest(&d) < 1.0 {
        let step = if largest(&d) < 2f64.powi(-511) { 2f64.powi(511) } else { 2.0 };
        d = d.map(|x| x * step);
    }
    d
}

/// Astra B2: R and s scaled together by one exact power of two 2^k so that the largest of
/// |R_i| and |s| lies in [1/2, 1) (all zero is left as is). s·|d| = dᵀR is invariant under a
/// common positive scale, and the residual and allowance formed from the scaled values never
/// overflow, so a large finite R can never make the allowance infinite.
///
/// Astra B2′: the allowance's absolute quantization floor belongs to the unscaled
/// quantities (64ε·2^-1022, the guard's floor before B2), so it is returned scaled by the
/// same 2^k (`floor`), never re-floored at the scaled magnitude: an honest subnormal reaction
/// (R = (−m, −m, 0), s = −m) reads, as it did before the joint scaling.
///
/// Scaling down may flush a value more than 2^1074 below the largest; that changes the
/// residual by at most 6·2^-1075 (scaled), inside the allowance. The Python verifier repeats
/// these steps one for one (re-check 2), so a flushed value rounds the same in both readers.
fn joint_scale(mut r: [f64; 3], mut s: f64) -> ([f64; 3], f64, f64) {
    let largest = |r: &[f64; 3], s: f64| r.iter().fold(s.abs(), |m, x| m.max(x.abs()));
    let mut floor = f64::MIN_POSITIVE;
    if largest(&r, s) == 0.0 || !largest(&r, s).is_finite() {
        return (r, s, floor);
    }
    while largest(&r, s) >= 1.0 {
        let step = if largest(&r, s) >= 2f64.powi(512) { 2f64.powi(-511) } else { 0.5 };
        (r, s, floor) = (r.map(|x| x * step), s * step, floor * step);
    }
    while largest(&r, s) < 0.5 {
        let step = if largest(&r, s) < 2f64.powi(-511) { 2f64.powi(511) } else { 2.0 };
        (r, s, floor) = (r.map(|x| x * step), s * step, floor * step);
    }
    (r, s, floor)
}

/// The along guard: |s·|d| − dᵀR| ≤ 64ε·max(|d|·Σ|R_i|, floor) on the canonical d and the
/// jointly scaled R, s and floor. A non-finite residual or allowance refuses; it never accepts.
///
/// Re-check 2 (verdict parity): every sum is left to right, with no compensation and no fused
/// multiply-add, and the Python verifier (`_along_guard`) repeats each operation in this order,
/// so the two readers give one verdict on every input, at the allowance boundary too
/// (`fixtures/results/skew_along_direction_guard_parity.json`).
///
/// This is a consistency check within a finite allowance, not an exact-integrity check: an
/// alteration of s inside the allowance reads. That includes a subnormal alteration inside the
/// absolute floor 64·2^-1074 (unscaled): R = (−m, −m, 0) with s changed from −m to 0 reads
/// (m = 2^-1074, d = (1, 1, 0)).
fn guard(d: [f64; 3], r: [f64; 3], s: f64) -> bool {
    let d = canonical(d);
    let (r, s, floor) = joint_scale(r, s);
    let norm = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    let dot = d[0] * r[0] + d[1] * r[1] + d[2] * r[2];
    let scale = norm * (r[0].abs() + r[1].abs() + r[2].abs());
    let residual = (s * norm - dot).abs();
    let allowance = 64.0 * f64::EPSILON * scale.max(floor);
    residual.is_finite() && allowance.is_finite() && residual <= allowance
}

/// One case's along-direction rows (`rows`: the case's direct rows; `case_id`; `model`).
pub(crate) fn check_case(rows: &[&Value], case_id: &str, model: Option<&Value>) -> Check {
    let mut along: HashMap<&str, &Value> = HashMap::new();
    let mut components: HashMap<(&str, &str), f64> = HashMap::new();
    for row in rows {
        let entity = row["entity_ref"].as_str().unwrap_or("");
        if row["kind"] == KIND {
            along_require(
                row["id"].as_str() == Some(&format!("result:support-along-direction-v1:{}", exact_source_identity(&[case_id, entity]))),
                "ALONG_DIRECTION_ROW_ID",
            )?;
            along_require(along.insert(entity, row).is_none(), "ALONG_DIRECTION_DUPLICATE")?;
        } else if row["kind"] == SUPPORT_COMPONENT {
            if let (Some(c), Some(v)) = (row["metadata"]["component"].as_str(), row["value"].as_f64()) {
                components.insert((entity, c), v);
            }
        }
    }
    for (entity, row) in &along {
        let r: Option<Vec<f64>> = ["Fx", "Fy", "Fz"].iter().map(|c| components.get(&(*entity, *c)).copied()).collect();
        along_require(r.is_some(), "ALONG_DIRECTION_COMPONENTS_MISSING")?;
        let Some(model) = model else { continue };
        let support = model["supports"].as_array().into_iter().flatten().find(|s| s["id"] == *entity);
        let d = support.and_then(skewed_direction);
        along_require(d.is_some(), "ALONG_DIRECTION_UNEXPECTED")?;
        let (d, r) = (d.unwrap(), r.unwrap());
        let s = row["value"].as_f64().ok_or("ALONG_DIRECTION_VALUE")?;
        along_require(s.is_finite() && r.iter().all(|x| x.is_finite()), "ALONG_DIRECTION_INCONSISTENT")?;
        along_require(guard(d, [r[0], r[1], r[2]], s), "ALONG_DIRECTION_INCONSISTENT")?;
    }
    if let Some(model) = model {
        // Every skewed record that publishes its component rows publishes its along row.
        let published: HashSet<&str> = components.keys().map(|(e, _)| *e).collect();
        for support in model["supports"].as_array().into_iter().flatten() {
            let id = support["id"].as_str().unwrap_or("");
            if skewed_direction(support).is_some() && published.contains(id) {
                along_require(along.contains_key(id), "ALONG_DIRECTION_MISSING")?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const CASE: &str = "case:skew";

    fn rows(r: [f64; 3], s: f64) -> Vec<Value> {
        let id = format!("result:support-along-direction-v1:{}", exact_source_identity(&[CASE, "stop_B"]));
        let mut out = vec![json!({"id": id, "kind": KIND, "entity_ref": "stop_B", "value": s, "unit": "N",
            "metadata": {"component": COMPONENT, "coordinate_system": COORDINATE_SYSTEM, "location": "node", "basis": BASIS, "sign_convention": SIGN}})];
        for (c, v) in ["Fx", "Fy", "Fz"].iter().zip(r) {
            out.push(json!({"id": format!("r:{c}"), "kind": SUPPORT_COMPONENT, "entity_ref": "stop_B", "value": v, "metadata": {"component": c}}));
        }
        out
    }
    fn model(d: [f64; 3]) -> Value {
        json!({"supports": [{"id": "stop_B", "family": "line_stop", "direction": {"x": d[0], "y": d[1], "z": d[2]}}]})
    }
    fn check(d: [f64; 3], r: [f64; 3], s: f64) -> Check {
        let rows = rows(r, s);
        check_case(&rows.iter().collect::<Vec<_>>(), CASE, Some(&model(d)))
    }

    /// Astra B2: with R = (8e307, 8e307, 0) the unscaled allowance norm·Σ|R_i| overflows and
    /// accepted any s. The joint power-of-two scale keeps it finite: the true value is
    /// accepted, a wrong one (0, or off by 1e-9 relative) is refused.
    /// Astra B2′: an honest subnormal reaction, the finite-κ spring outputs for k = 2^-1074 and
    /// u = (1, 1, 0) (R = (−m, −m, 0), s = −m), reads; the floor is the unscaled one carried
    /// through the scaling. A subnormal tamper beyond the floor is refused.
    #[test]
    fn an_honest_subnormal_reaction_reads_and_a_subnormal_tamper_is_refused() {
        let m = f64::from_bits(1);
        let d = [1.0, 1.0, 0.0];
        assert_eq!(check(d, [-m, -m, 0.0], -m), Ok(()));
        assert_eq!(check(d, [-m, -m, 0.0], -100.0 * m), Err("ALONG_DIRECTION_INCONSISTENT".into()));
        assert_eq!(check(d, [-1000.0 * m, -1000.0 * m, 0.0], 1000.0 * m), Err("ALONG_DIRECTION_INCONSISTENT".into()));
    }

    #[test]
    fn a_large_finite_reaction_never_makes_the_guard_accept_a_wrong_value() {
        let d = [1.0, 1.0, 0.0];
        let r = [8e307, 8e307, 0.0];
        let s = 8e307 * std::f64::consts::SQRT_2;
        assert_eq!(check(d, r, s), Ok(()));
        assert_eq!(check(d, r, 0.0), Err("ALONG_DIRECTION_INCONSISTENT".into()));
        assert_eq!(check(d, r, s * (1.0 + 1e-9)), Err("ALONG_DIRECTION_INCONSISTENT".into()));
        assert_eq!(check(d, r, -s), Err("ALONG_DIRECTION_INCONSISTENT".into()));
        // The largest finite reactions, and an s near MAX.
        let r = [f64::MAX, -f64::MAX, f64::MAX];
        assert_eq!(check([1.0, 1.0, 1.0], r, f64::MAX / 3f64.sqrt()), Ok(()));
        assert_eq!(check([1.0, 1.0, 1.0], r, 0.0), Err("ALONG_DIRECTION_INCONSISTENT".into()));
    }

    /// Astra B3 (the Rust half): a direction rescaled by 2^-1026 (a subnormal pair) reads as
    /// its unscaled twin (an admitted direction: every nonzero canonical component normal);
    /// `canonical` steps by powers of two and never forms the scale.
    #[test]
    fn a_subnormal_scaled_direction_reads_as_its_twin() {
        let tiny = 2f64.powi(-1000) * 2f64.powi(-26);
        assert!(tiny > 0.0 && tiny < f64::MIN_POSITIVE);
        let (r, s) = ([-3.0, -4.0, 0.0], -5.0);
        assert_eq!(check([3.0, 4.0, 0.0], r, s), Ok(()));
        assert_eq!(check([3.0 * tiny, 4.0 * tiny, 0.0], r, s), Ok(()));
        assert_eq!(check([3.0 * tiny, 4.0 * tiny, 0.0], r, -s), Err("ALONG_DIRECTION_INCONSISTENT".into()));
    }

    /// Re-check 2: d = R = (1.0, 0.1, 0.6), s = 0x1.2ba3ebdc9a779p+0. Exactly,
    /// |s − dᵀR/|d|| exceeds 64ε·Σ|R_i| by about 6.55e-17, so the guard refuses; the Python
    /// verifier's compensated `sum` accepted it.
    #[test]
    fn the_boundary_counterexample_is_refused() {
        let d = [1.0, 0.1, 0.6];
        let s = f64::from_bits(0x3ff2_ba3e_bdc9_a779);
        assert_eq!(check(d, d, s), Err("ALONG_DIRECTION_INCONSISTENT".into()));
    }

    /// Re-check 2: the shared parity sweep. Both readers reach each committed verdict
    /// (the Python half: `tests/test_skew_along_direction_reader.py`).
    #[test]
    fn the_shared_parity_sweep_reads_with_its_committed_verdicts() {
        let sweep: Value =
            serde_json::from_str(include_str!("../../../../fixtures/results/skew_along_direction_guard_parity.json")).unwrap();
        let bits = |v: &Value| f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap());
        let three = |v: &Value| [bits(&v[0]), bits(&v[1]), bits(&v[2])];
        let cases = sweep["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 385);
        let (mut accepted, mut refused) = (0, 0);
        for (n, case) in cases.iter().enumerate() {
            let verdict = check(three(&case["d"]), three(&case["r"]), bits(&case["s"]));
            if case["accept"].as_bool().unwrap() {
                assert_eq!(verdict, Ok(()), "case {n}");
                accepted += 1;
            } else {
                assert_eq!(verdict, Err("ALONG_DIRECTION_INCONSISTENT".into()), "case {n}");
                refused += 1;
            }
        }
        assert_eq!((accepted, refused), (192, 193));
    }

    /// Re-check 2 (non-blocking, by design): the guard is a consistency check within a
    /// finite allowance. A subnormal alteration inside the absolute floor reads.
    #[test]
    fn a_subnormal_alteration_inside_the_floor_reads_by_design() {
        let m = f64::from_bits(1);
        assert_eq!(check([1.0, 1.0, 0.0], [-m, -m, 0.0], 0.0), Ok(()));
    }
}
