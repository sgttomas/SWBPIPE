//! The S4 combinations against the independent same-represented-input oracle
//! (`tests/oracle/represented_input_oracle.py`, which reads no product code): the exact value of each
//! rule on the very binary64 inputs the product receives. Each published magnitude must lie within
//! 1e-9 relative of the oracle's value, and an oracle zero must be produced exactly. This checks
//! S4's arithmetic; the end-to-end comparison with the frozen model references stays in
//! `f2_combinations.rs` at the analytic 1e-9.
//!
//! The review witnesses of round 1 are kept here as regressions, with the refusals that follow from
//! the accuracy rule (no binary64 within 1e-9, or a radicand not resolved at the precision cap).
mod support;

use open_pipe_stress_response_spectrum::{
    combine, ClusterIndex, CombinationRequest, Direction, DirectionalRule, DisplacementComponent, MissingMass,
    ModalContribution, ModalMode, ModalRule, ModeIndex, Quantity, RigidRule, PRECISION_LADDER,
};
use serde_json::Value;
use support::*;

const ORACLE: &str = include_str!("oracle/represented_input_oracle.json");
const C4: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c4_close_pair.json");
const C9: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c9_pipe_diagonal.json");
const C10: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c10_cluster_witnesses.json");

fn oracle() -> Value {
    serde_json::from_str(ORACLE).unwrap()
}

fn decimal(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

fn check(what: &str, actual: f64, expected: f64, largest: &mut f64) {
    if expected == 0.0 {
        assert_eq!(actual, 0.0, "{what}: the oracle's value is exactly zero");
        return;
    }
    let relative = ((actual - expected) / expected).abs();
    assert!(relative <= TOLERANCE, "{what}: {actual:e} against the oracle's {expected:e} ({relative:e})");
    *largest = largest.max(relative);
}

fn rigid_rule(name: &str) -> Option<RigidRule> {
    match name {
        "OFF" => None,
        other => RigidRule::parse(other),
    }
}

#[test]
fn frozen_reference_inputs_against_the_oracle() {
    let o = oracle();
    let mut compared = 0;
    let mut largest = 0.0f64;
    for case_entry in o["frozen"].as_array().unwrap() {
        let source = case_entry["source"].as_str().unwrap();
        let text = if source.ends_with("c4_close_pair.json") {
            C4
        } else if source.ends_with("c9_pipe_diagonal.json") {
            C9
        } else {
            C10
        };
        let doc: Value = serde_json::from_str(text).unwrap();
        let id = case_entry["analysis"].as_str().unwrap();
        let (case, analysis) = doc["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find_map(|c| c["analyses"].as_array().unwrap().iter().find(|a| a["request"]["id"] == id).map(|a| (c, a)))
            .unwrap();
        let a = discrete_analysis(case, analysis, None);
        for result in case_entry["results"].as_array().unwrap() {
            let modal = modal_rule(result["modal_combination"].as_str().unwrap());
            let rigid = rigid_rule(result["missing_mass"].as_str().unwrap());
            let directional = directional_rule(result["directional_combination"].as_str().unwrap());
            let c = combine(&a.request(modal, rigid, directional)).expect("certified");
            for row in &c.rows {
                let key = row.quantity.key();
                let what = format!("{id} {modal:?}/{rigid:?}/{directional:?} {key}");
                check(&what, row.combined.value(), decimal(&result["combined"][&key]), &mut largest);
                compared += 1;
            }
        }
    }
    eprintln!("oracle, frozen inputs: {compared} values, largest relative deviation {largest:e}");
    assert!(compared > 2000);
}

fn single_row(modal: ModalRule, zeta: f64, freqs: &[f64], clusters: &[u32], values: &[f64]) -> CombinationRequest {
    CombinationRequest {
        damping_ratio: zeta,
        modal_rule: modal,
        directional_rule: DirectionalRule::Srss,
        missing_mass: MissingMass::NotApplied,
        directions: vec![Direction::X],
        modes: freqs
            .iter()
            .zip(clusters)
            .enumerate()
            .map(|(i, (&f, &c))| ModalMode { mode: ModeIndex(i as u32 + 1), cluster: ClusterIndex(c), frequency_hz: f })
            .collect(),
        rows: vec![Quantity::NodeDisplacement { node: "N1".into(), component: DisplacementComponent::Ux }],
        periodic: values
            .iter()
            .enumerate()
            .map(|(i, &v)| ModalContribution { mode: ModeIndex(i as u32 + 1), direction: Direction::X, values: vec![v] })
            .collect(),
        rigid: vec![],
    }
}

fn three_directions(magnitudes: [f64; 3]) -> CombinationRequest {
    CombinationRequest {
        damping_ratio: 0.05,
        modal_rule: ModalRule::AbsoluteSum,
        directional_rule: DirectionalRule::HundredFortyForty,
        missing_mass: MissingMass::NotApplied,
        directions: Direction::ALL.to_vec(),
        modes: vec![ModalMode { mode: ModeIndex(1), cluster: ClusterIndex(1), frequency_hz: 1.0 }],
        rows: vec![Quantity::NodeDisplacement { node: "N1".into(), component: DisplacementComponent::Ux }],
        periodic: Direction::ALL
            .iter()
            .zip(magnitudes)
            .map(|(&d, v)| ModalContribution { mode: ModeIndex(1), direction: d, values: vec![v] })
            .collect(),
        rigid: vec![],
    }
}

#[test]
fn review_witnesses_against_the_oracle() {
    let o = oracle();
    let mut largest = 0.0f64;
    for w in o["witnesses"].as_array().unwrap() {
        let id = w["id"].as_str().unwrap();
        let expected = decimal(&w["combined"]);
        let request = if let Some(m) = w.get("directional_magnitudes") {
            let m: Vec<f64> = m.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            three_directions([m[0], m[1], m[2]])
        } else {
            let floats = |k: &str| -> Vec<f64> { w[k].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect() };
            let clusters: Vec<u32> = w["clusters"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect();
            single_row(ModalRule::parse(w["modal_combination"].as_str().unwrap()).unwrap(), w["damping_ratio"].as_f64().unwrap(),
                &floats("frequencies_hz"), &clusters, &floats("contributions"))
        };
        let c = combine(&request).unwrap_or_else(|r| panic!("{id}: refused {r:?}"));
        check(id, c.rows[0].combined.value(), expected, &mut largest);
    }
    eprintln!("oracle, review witnesses: largest relative deviation {largest:e}");
}

fn refusal_codes(request: &CombinationRequest) -> Vec<&'static str> {
    combine(request).expect_err("refused").iter().map(|r| r.code.as_str()).collect()
}

#[test]
fn review_witnesses_refused_by_the_accuracy_rule() {
    let tiny = f64::from_bits(1); // 2^-1074
    // 100-40-40 of three smallest subnormals: exactly (3·1 + 2·3)/5 = 9/5 = 1.8 quanta (analytic);
    // the nearest binary64 is 2 quanta, 1/9 away, so the accuracy cannot be established.
    assert_eq!(refusal_codes(&three_directions([tiny; 3])), ["SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED"]);
    // [2, 1, 1] quanta: exactly (3·2 + 2·4)/5 = 14/5 = 2.8 quanta; the nearest is 3, 1/14 away.
    assert_eq!(refusal_codes(&three_directions([2.0 * tiny, tiny, tiny])), ["SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED"]);
    // An exactly representable result is published: 100-40-40 of [5, 0, 0] quanta is 5 quanta.
    let c = combine(&three_directions([5.0 * tiny, 0.0, 0.0])).unwrap();
    assert_eq!(c.rows[0].combined.value(), 5.0 * tiny);

    // The evenly spaced alternating binomial with n = 40 at ζ = 0.2 needs more than the cap under
    // the pricing model (about 1,020 bits at n = 36, about 25 more per mode); the radicand's
    // enclosure still reaches zero at 1,024 bits, and the request is refused, not published.
    let n = 40;
    let freqs: Vec<f64> = (0..n).map(|i| 1.0 + i as f64 * 2f64.powi(-19)).collect();
    let mut binomial = 1.0f64;
    let mut values = Vec::new();
    for k in 0..n {
        values.push(if k % 2 == 0 { binomial } else { -binomial });
        binomial = binomial * (n - 1 - k) as f64 / (k + 1) as f64;
    }
    let clusters: Vec<u32> = (1..=n as u32).collect();
    let refused = combine(&single_row(ModalRule::Cqc, 0.2, &freqs, &clusters, &values)).expect_err("beyond the cap");
    assert_eq!(refused.len(), 1);
    assert_eq!(refused[0].code.as_str(), "SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED");
    assert!(refused[0].detail.starts_with("this implemented error argument does not establish the required relative accuracy"));
    assert!(refused[0].detail.contains(&format!("{}-bit cap", PRECISION_LADDER[PRECISION_LADDER.len() - 1])));
    assert_eq!(refused[0].field.as_deref(), Some("X"));
    assert_eq!(refused[0].object, "N1.ux");
}

#[test]
fn range_witnesses_keep_small_terms() {
    // [1, 1e-200]: the exact SRSS is 1 + 5e-401, and its published value is 1.
    let c = combine(&single_row(ModalRule::Srss, 0.05, &[1.0, 2.0], &[1, 2], &[1.0, 1e-200])).unwrap();
    assert_eq!(c.rows[0].combined.value(), 1.0);
    // [1e-200, 1e-200]: no square underflows; √2·1e-200.
    let c = combine(&single_row(ModalRule::Srss, 0.05, &[1.0, 2.0], &[1, 2], &[1e-200, -1e-200])).unwrap();
    assert!(((c.rows[0].combined.value() - std::f64::consts::SQRT_2 * 1e-200) / 1e-200).abs() < 1e-15);
    // A lone subnormal is exact under every modal rule.
    for rule in ModalRule::ALL {
        let c = combine(&single_row(rule, 0.05, &[1.0], &[1], &[-3.0 * f64::from_bits(1)])).unwrap();
        assert_eq!(c.rows[0].combined.value(), 3.0 * f64::from_bits(1), "{rule:?}");
    }
}

/// The second comparison, separate from the analytic one: for C10-W2, P1.j.Fz under Y, the
/// product's deviation from the frozen model value (6.98e-10 relative, within the 1e-9 criterion) is
/// explained by the conversion of the reference's 40-digit inputs to binary64. Each binary64 input
/// differs from its decimal by at most u·|c| (u = 2^-53), and SRSS of cluster sums changes by at most
/// Σ|Δc|, so the derived input-conversion term is u·Σ|c|. The product agrees with the oracle on the
/// represented inputs far more closely.
#[test]
fn c10_w2_input_conversion_is_a_separate_derived_term() {
    let doc: Value = serde_json::from_str(C10).unwrap();
    let case = &doc["cases"][0];
    let analysis = &case["analyses"][0];
    let a = discrete_analysis(case, analysis, None);
    let c = combine(&a.request(ModalRule::Srss, None, DirectionalRule::Srss)).unwrap();
    let key = "P1.j.Fz";
    let i = a.keys.iter().position(|k| k == key).unwrap();
    let product = c.rows[i].per_direction.iter().find(|p| p.direction == Direction::Y).unwrap().combined.value();
    let frozen = expected(&analysis["per_direction_combined"]["Y"][key]);
    let operand_sum: f64 = (1..=4).map(|m| a.contributions[&(Direction::Y, m)][i].abs()).sum();
    let conversion = f64::EPSILON / 2.0 * operand_sum;
    let deviation = (product - frozen).abs();
    assert!(deviation <= TOLERANCE * frozen.abs(), "the analytic criterion holds: {deviation:e}");
    assert!(deviation <= conversion, "{deviation:e} against the conversion term {conversion:e}");
    assert!(conversion > TOLERANCE * frozen.abs(), "the conversion term's worst case exceeds 1e-9 of this value");
    // The oracle on the represented inputs: SRSS Y of P1.j.Fz is not separately recorded, but the
    // combined over directions is; the product agrees with it to within a few roundings.
    let o = oracle();
    let entry = o["frozen"].as_array().unwrap().iter().find(|e| e["analysis"] == "C10-W2").unwrap();
    let base = &entry["results"][0];
    assert_eq!(base["modal_combination"], "SRSS");
    let oracle_value = decimal(&base["combined"][key]);
    let published = c.rows[i].combined.value();
    assert!(((published - oracle_value) / oracle_value).abs() < 1e-15);
}
