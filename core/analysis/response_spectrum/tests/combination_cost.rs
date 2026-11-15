//! Cost measurements of the certified combinations (unit S4), the empirical evidence for the
//! precision cap. They are `#[ignore]`d: run them in release, one at a time, under a resource
//! meter, for example
//! `cargo test --release --test combination_cost -- --ignored --nocapture --test-threads 1 <name>`.
//! They assert outcomes (combined or refused), not timings.
mod support;

use open_pipe_stress_response_spectrum::{
    combine, ClusterIndex, CombinationRequest, Direction, DirectionalRule, DisplacementComponent, MissingMass,
    ModalContribution, ModalMode, ModalRule, ModeIndex, Quantity, RigidRule,
};
use serde_json::Value;
use std::time::Instant;
use support::*;

const C3: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c3_cantilever_3d.json");
const C4: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c4_close_pair.json");
const C9: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c9_pipe_diagonal.json");
const C10: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c10_cluster_witnesses.json");

fn rigid_rule(name: &str) -> Option<RigidRule> {
    match name {
        "OFF" => None,
        other => RigidRule::parse(other),
    }
}

/// Every base request and mutation of the frozen discrete references, in global axes.
#[test]
#[ignore]
fn cost_frozen_references() {
    let mut calls = 0;
    let mut values = 0;
    let start = Instant::now();
    for text in [C3, C4, C9, C10] {
        let doc: Value = serde_json::from_str(text).unwrap();
        for case in doc["cases"].as_array().unwrap() {
            for analysis in case["analyses"].as_array().unwrap() {
                let a = discrete_analysis(case, analysis, None);
                let request = &analysis["request"];
                let mut choices = vec![(
                    modal_rule(request["modal_combination"].as_str().unwrap()),
                    request["missing_mass"]["rigid_periodic_combination"].as_str().and_then(rigid_rule),
                    directional_rule(request["directional_combination"].as_str().unwrap()),
                )];
                for m in analysis["mutations"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
                    choices.push((
                        modal_rule(m["modal_combination"].as_str().unwrap()),
                        rigid_rule(m["missing_mass"].as_str().unwrap()),
                        directional_rule(m["directional_combination"].as_str().unwrap()),
                    ));
                }
                for (modal, rigid, directional) in choices {
                    let c = combine(&a.request(modal, rigid, directional)).expect("combined");
                    calls += 1;
                    values += c.rows.len() * (1 + 2 * c.directions.len());
                }
            }
        }
    }
    let elapsed = start.elapsed();
    eprintln!("COST frozen references: {calls} combinations, {values} published magnitudes, {elapsed:?} ({:?} per combination)",
        elapsed / calls as u32);
}

/// A deterministic pseudo-random sequence in [-1, 1).
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    }
}

fn many_mode_request(modes: usize, rows: usize, rule: ModalRule) -> CombinationRequest {
    let mut rng = Lcg(0x5eed);
    let directions = vec![Direction::X, Direction::Y, Direction::Z];
    // Frequencies from 1 Hz rising 4 % per mode, each mode its own cluster.
    let modes_meta: Vec<ModalMode> = (0..modes)
        .map(|i| ModalMode { mode: ModeIndex(i as u32 + 1), cluster: ClusterIndex(i as u32 + 1), frequency_hz: 1.04f64.powi(i as i32) })
        .collect();
    let row_ids: Vec<Quantity> = (0..rows)
        .map(|i| Quantity::NodeDisplacement { node: format!("N{}", i / 6 + 1), component: [
            DisplacementComponent::Ux, DisplacementComponent::Uy, DisplacementComponent::Uz,
            DisplacementComponent::Rx, DisplacementComponent::Ry, DisplacementComponent::Rz][i % 6] })
        .collect();
    let mut periodic = Vec::new();
    for &d in &directions {
        for m in &modes_meta {
            periodic.push(ModalContribution { mode: m.mode, direction: d, values: (0..rows).map(|_| rng.next() * 1e-3).collect() });
        }
    }
    CombinationRequest {
        damping_ratio: 0.05,
        modal_rule: rule,
        directional_rule: DirectionalRule::Srss,
        missing_mass: MissingMass::NotApplied,
        directions,
        modes: modes_meta,
        rows: row_ids,
        periodic,
        rigid: vec![],
    }
}

fn many_mode(modes: usize, rows: usize, rule: ModalRule) {
    let request = many_mode_request(modes, rows, rule);
    let start = Instant::now();
    let c = combine(&request).expect("combined");
    let elapsed = start.elapsed();
    eprintln!("COST many-mode {rule:?}: {modes} modes, {rows} rows, 3 directions: {elapsed:?} ({:?} per row)",
        elapsed / rows as u32);
    assert_eq!(c.rows.len(), rows);
}

#[test]
#[ignore]
fn cost_many_mode_cqc_30_by_300() {
    many_mode(30, 300, ModalRule::Cqc);
}

#[test]
#[ignore]
fn cost_many_mode_cqc_60_by_300() {
    many_mode(60, 300, ModalRule::Cqc);
}

#[test]
#[ignore]
fn cost_many_mode_cqc_120_by_300() {
    many_mode(120, 300, ModalRule::Cqc);
}

#[test]
#[ignore]
fn cost_many_mode_cqc_120_by_1200() {
    many_mode(120, 1200, ModalRule::Cqc);
}

#[test]
#[ignore]
fn cost_many_mode_srss_120_by_1200() {
    many_mode(120, 1200, ModalRule::Srss);
}

/// The evenly spaced alternating-binomial family (`PLAN.md` M3): f_i = 1 + i·2^-19 Hz,
/// R_k = (−1)^k C(n−1, k) N, ζ = 0.2, CQC, one row and direction.
pub fn binomial_request(n: usize, scale: f64) -> CombinationRequest {
    binomial_rows(n, scale, 1)
}

/// The same family with `rows` rows, row k scaled by 2^k (scaling leaves the conditioning
/// unchanged, so every row needs the same precision).
pub fn binomial_rows(n: usize, scale: f64, rows: usize) -> CombinationRequest {
    let modes = (0..n)
        .map(|i| ModalMode { mode: ModeIndex(i as u32 + 1), cluster: ClusterIndex(i as u32 + 1), frequency_hz: 1.0 + i as f64 * 2f64.powi(-19) })
        .collect();
    let mut c = 1.0f64;
    let mut periodic = Vec::new();
    for k in 0..n {
        let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
        periodic.push(ModalContribution { mode: ModeIndex(k as u32 + 1), direction: Direction::X,
            values: (0..rows).map(|r| sign * c * scale * 2f64.powi(r as i32)).collect() });
        c = c * (n - 1 - k) as f64 / (k + 1) as f64;
    }
    CombinationRequest {
        damping_ratio: 0.2,
        modal_rule: ModalRule::Cqc,
        directional_rule: DirectionalRule::Srss,
        missing_mass: MissingMass::NotApplied,
        directions: vec![Direction::X],
        modes,
        rows: (0..rows).map(|r| Quantity::NodeDisplacement { node: format!("N{}", r + 1), component: DisplacementComponent::Ux }).collect(),
        periodic,
        rigid: vec![],
    }
}

fn timed(label: &str, request: &CombinationRequest) {
    let start = Instant::now();
    let outcome = combine(request);
    let rows = request.rows.len();
    let elapsed = start.elapsed();
    let what = match &outcome {
        Ok(_) => "certified".to_string(),
        Err(r) => format!("refused ({} refusals)", r.len()),
    };
    eprintln!("COST {label}: {rows} rows, {what}, {elapsed:?} ({:?} per row)", elapsed / rows as u32);
}

#[test]
#[ignore]
fn cost_demanding_n20_rows_10() {
    timed("demanding n=20", &binomial_rows(20, 1.0, 10));
}

#[test]
#[ignore]
fn cost_demanding_n20_rows_50() {
    timed("demanding n=20", &binomial_rows(20, 1.0, 50));
}

#[test]
#[ignore]
fn cost_refusal_n40_rows_10() {
    timed("refusal n=40", &binomial_rows(40, 1.0, 10));
}

#[test]
#[ignore]
fn cost_refusal_n40_rows_50() {
    timed("refusal n=40", &binomial_rows(40, 1.0, 50));
}

#[test]
#[ignore]
fn cost_refusal_n80_rows_1() {
    timed("refusal n=80", &binomial_rows(80, 1.0, 1));
}

/// A demanding refinement: n = 20 needs about 605 bits under the pricing model and is certified
/// at the 1,024-bit level.
#[test]
#[ignore]
fn cost_demanding_refinement_n20() {
    let start = Instant::now();
    let c = combine(&binomial_request(20, 1.0)).expect("certified");
    eprintln!("COST demanding refinement n=20: {:?}, value {:e}", start.elapsed(), c.rows[0].combined.value());
}

/// The refusal path: n = 40 needs more than the cap; every level is tried and the request is
/// refused.
#[test]
#[ignore]
fn cost_refusal_path_n40() {
    let start = Instant::now();
    let refused = combine(&binomial_request(40, 1.0)).expect_err("refused at the cap");
    eprintln!("COST refusal path n=40: {:?}, {}", start.elapsed(), refused[0]);
    assert_eq!(refused[0].code.as_str(), "SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED");
}

/// The refusal path with many clusters: 120 clusters (the n = 40 refusing pattern on the first 40,
/// zero contributions on 80 more at higher frequencies), so the ρ table of every level covers all
/// 7,140 pairs.
#[test]
#[ignore]
fn cost_refusal_n40_with_120_clusters_rows_10() {
    let mut request = binomial_rows(40, 1.0, 10);
    for i in 40..120u32 {
        request.modes.push(ModalMode { mode: ModeIndex(i + 1), cluster: ClusterIndex(i + 1), frequency_hz: 2.0 + i as f64 });
        request.periodic.push(ModalContribution { mode: ModeIndex(i + 1), direction: Direction::X, values: vec![0.0; 10] });
    }
    timed("refusal n=40 among 120 clusters", &request);
}
