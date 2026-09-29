//! K4 tests of the references lane (the brief's K): R1's frozen cases through
//! the adapter (`r1_cases.txt`), the floor and discrimination checks, and the
//! routed cases.
//!
//! S\*-dependent assertions (to be updated mechanically if D1 revision 5a.3
//! changes S\*): `SD-K1` the floor check's not-covered set (S\* from the
//! reference values with K4's §4.1.6.1 functions) equals §4.10's list.
use super::super::source::{PrimitiveSource, SourceError};
use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;

const R1: &str = include_str!("r1_cases.txt");
/// The floor ratio R = 2^-34 (DESIGN revision 3).
const R: f64 = 5.820766091346741e-11;

struct RefRow {
    key: String,
    exp: f64,
    scale: f64,
    kind: String,
    r1: String,
}

struct R1Case {
    model: models::Model,
    family: String,
    basis: String,
    refuse: bool,
    kmem: BTreeMap<u32, (f64, f64)>,
    rows: Vec<RefRow>,
    zeros: usize,
    controls: Vec<(String, bool, Vec<(String, f64)>)>,
    /// Outcome controls: (id, discriminates, the defective outcome).
    outcome_controls: Vec<(String, bool, String)>,
    floor: BTreeSet<String>,
}

fn f64_hex(h: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(h, 16).unwrap())
}

fn r1_cases() -> Vec<R1Case> {
    r1_cases_from(R1)
}

fn r1_cases_from(text: &str) -> Vec<R1Case> {
    let mut out: Vec<R1Case> = models::parse_models(text)
        .into_iter()
        .map(|model| R1Case {
            model,
            family: String::new(),
            basis: String::new(),
            refuse: false,
            kmem: BTreeMap::new(),
            rows: Vec::new(),
            zeros: 0,
            controls: Vec::new(),
            outcome_controls: Vec::new(),
            floor: BTreeSet::new(),
        })
        .collect();
    let mut k = 0;
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.first() == Some(&"end") {
            k += 1;
            continue;
        }
        if k == out.len() {
            continue;
        }
        let c = &mut out[k];
        match f.first().copied() {
            Some("case") => {
                c.family = f[1].into();
                c.basis = f[2].into();
                c.refuse = f[3] == "refuse";
            }
            Some("kmem") => {
                c.kmem
                    .insert(f[1].parse().unwrap(), (f64_hex(f[2]), f64_hex(f[3])));
            }
            Some("ref") => c.rows.push(RefRow {
                key: f[1].into(),
                exp: f64_hex(f[2]),
                scale: f64_hex(f[3]),
                kind: f[4].into(),
                r1: f[5].into(),
            }),
            Some("zero") => c.zeros += 1,
            Some("nc") => {
                let values = f[3..]
                    .iter()
                    .filter(|t| **t != "-")
                    .map(|t| {
                        let (key, v) = t.split_once('=').unwrap();
                        (key.to_string(), f64_hex(v))
                    })
                    .collect();
                c.controls.push((f[1].into(), f[2] == "1", values));
            }
            Some("nco") => c
                .outcome_controls
                .push((f[1].into(), f[2] == "1", f[3].into())),
            Some("floor") => {
                c.floor.insert(f[1].into());
            }
            _ => {}
        }
    }
    assert_eq!(k, out.len());
    out
}

/// The published values by key, with R1's derived tw = fl(T/k_t) and
/// ext = fl(N/k_a) per member (never differenced).
fn observed(
    source: &PrimitiveSource,
    rows: &[PublishedRow],
    kmem: &BTreeMap<u32, (f64, f64)>,
) -> BTreeMap<String, f64> {
    let mut out: BTreeMap<String, f64> = models::published(rows)
        .into_iter()
        .map(|(k, v)| (k, v.0))
        .collect();
    for m in source.members() {
        let (kt, ka) = kmem[&m.id];
        let t = out.get(&format!("T.{}", m.id)).copied();
        let n = out.get(&format!("N.{}", m.id)).copied();
        if let (Some(t), Some(n)) = (t, n) {
            out.insert(format!("tw.{}", m.id), t / kt);
            out.insert(format!("ext.{}", m.id), n / ka);
        }
    }
    out
}

fn kind_of(kind: &str) -> Option<Kind> {
    Some(match kind {
        "translation" => Kind::Translation,
        "rotation" => Kind::Rotation,
        "force" => Kind::Force,
        "moment" => Kind::Moment,
        _ => return None,
    })
}

/// S\* of a compared row from the case's reference values (one body per case,
/// D1 §4.1.6.1 items 4–6 with K4's functions; twist and extension from the
/// member's k_t and k_a, variant F).
fn floor_scale(case: &R1Case, s_star: &[f64; 4], row: &RefRow) -> f64 {
    match kind_of(&row.kind) {
        Some(k) => s_star[k.index()],
        None => {
            let member: u32 = row.key.split('.').nth(1).unwrap().parse().unwrap();
            let (kt, ka) = case.kmem[&member];
            if row.kind == "twist" {
                s_star[Kind::Moment.index()] / kt
            } else {
                s_star[Kind::Force.index()] / ka
            }
        }
    }
}

fn passes(obs: f64, exp: f64, scale: f64) -> bool {
    (obs - exp).abs() <= 1e-9 * exp.abs().max(scale)
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    passes: usize,
    absolute_range_passes: usize,
    not_covered: usize,
    zeros_unpublished: usize,
    refused: usize,
    outcome_controls: usize,
    selected_at: BTreeMap<u32, usize>,
}

#[test]
fn r1s_cases_through_the_adapter_pass_are_refused_or_are_not_covered_as_section_4_10_lists() {
    let cases = r1_cases();
    assert_eq!(cases.len(), 128);
    let Lane {
        tally,
        failures,
        floor_differences,
        undiscriminating,
    } = run_lane(&cases);
    report_lane(&tally, &failures, &floor_differences, &undiscriminating);
    assert!(failures.is_empty(), "{} reference failures", failures.len());
    // SD-K1.
    assert!(
        floor_differences.is_empty(),
        "{} floor differences",
        floor_differences.len()
    );
    // D1 revision 5a.3 (plan §6, R7's R1 lane): all 120 solved cases are
    // selected at 128, as under 5a.2.
    let selected: usize = tally
        .values()
        .map(|t| t.selected_at.values().sum::<usize>())
        .sum();
    let at_128: usize = tally
        .values()
        .map(|t| t.selected_at.get(&128).copied().unwrap_or(0))
        .sum();
    assert_eq!([selected, at_128], [120, 120]);
    lane_counts(&cases, &tally);
}

struct Lane {
    tally: BTreeMap<String, Tally>,
    failures: Vec<String>,
    floor_differences: Vec<String>,
    undiscriminating: Vec<String>,
}

/// Every case of a lane: refused as expected, or selected with every reference
/// row within the predicate (or not covered, §4.10), and its controls
/// discriminated.
fn run_lane(cases: &[R1Case]) -> Lane {
    let mut tally: BTreeMap<String, Tally> = BTreeMap::new();
    let mut failures = Vec::new();
    let mut floor_differences = Vec::new();
    let mut undiscriminating = Vec::new();
    for case in cases {
        let t = tally.entry(case.family.clone()).or_default();
        t.cases += 1;
        let name = &case.model.name;
        if case.refuse {
            let parts = case.model.parts.clone();
            let source = match PrimitiveSource::new(parts.clone()) {
                Ok(s) => s,
                Err(SourceError::NonPositiveSpring { .. }) => {
                    // RF-MECH-K0 [POSITION]: the adapter passes k = 0 and source
                    // validation refuses it; without the zero spring, geometry
                    // refuses the mechanism.
                    assert_eq!(name, "RF-MECH-K0");
                    let mut parts = parts;
                    parts.springs.retain(|s| s.stiffness != 0.0);
                    PrimitiveSource::new(parts).unwrap()
                }
                Err(e) => panic!("{name}: {e:?}"),
            };
            let mut meter = InvocationMeter::new(u64::MAX);
            match solve_case(source, CaseLimit::new(u64::MAX), &mut meter) {
                CaseOutcome::Refused {
                    refusal: Refusal::MechanismWitnessed { .. },
                    geometry,
                } => assert!(geometry.is_empty(), "{name}"),
                other => failures.push(format!("{name}: expected a refusal, got {other:?}")),
            }
            t.refused += 1;
            // Its controls describe publishing a solution: refused, so each is
            // discriminated by the outcome.
            t.outcome_controls += case.outcome_controls.len() + case.controls.len();
            continue;
        }
        // A selected stable case discriminates its outcome controls (a refusal
        // of a stable model).
        for (id, discriminates, defect) in &case.outcome_controls {
            assert!(
                *discriminates && defect.starts_with("refusal"),
                "{name}: {id} {defect}"
            );
        }
        t.outcome_controls += case.outcome_controls.len();
        let source = case.model.source();
        let mut meter = InvocationMeter::new(u64::MAX);
        let solve = match solve_case(source.clone(), CaseLimit::new(u64::MAX), &mut meter) {
            CaseOutcome::Selected(s) => s,
            other => {
                failures.push(format!("{name}: not selected: {other:?}"));
                continue;
            }
        };
        *t.selected_at.entry(solve.selected_precision()).or_default() += 1;
        let obs = observed(&source, &solve.publish().rows, &case.kmem);
        // S from the reference values, coupled with the case's extent.
        let mut s = [0.0f64; 4];
        for row in &case.rows {
            if let Some(k) = kind_of(&row.kind) {
                s[k.index()] = s[k.index()].max(row.exp.abs());
            }
        }
        let s_star = coupled_scales(s, body_extent(source.nodes()));
        let mut not_covered = BTreeSet::new();
        for row in &case.rows {
            let sk = floor_scale(case, &s_star, row);
            let sc = row.exp.abs().max(row.scale);
            if sk > 0.0 && sc < R * sk {
                not_covered.insert(row.key.clone());
                t.not_covered += 1;
                continue;
            }
            let Some(&o) = obs.get(&row.key) else {
                failures.push(format!("{name}: {} ({}) not published", row.key, row.r1));
                continue;
            };
            if passes(o, row.exp, row.scale) {
                t.passes += 1;
                if sk > 0.0 && sk < f64::from_bits(0x0230_0000_0000_0000) {
                    t.absolute_range_passes += 1;
                }
            } else {
                failures.push(format!(
                    "{name}: {} ({}) observed {o:e} expected {:e} ratio {:.3e}",
                    row.key,
                    row.r1,
                    row.exp,
                    (o - row.exp).abs() / (1e-9 * row.exp.abs().max(row.scale))
                ));
            }
        }
        t.zeros_unpublished += case.zeros;
        if not_covered != case.floor {
            floor_differences.push(format!("{name}: K4 {not_covered:?} §4.10 {:?}", case.floor));
        }
        let refs: BTreeMap<&str, &RefRow> = case.rows.iter().map(|r| (r.key.as_str(), r)).collect();
        for (id, discriminates, values) in &case.controls {
            let fails = values.iter().any(|(key, v)| {
                refs.get(key.as_str())
                    .is_some_and(|r| !passes(*v, r.exp, r.scale))
            });
            if *discriminates && !fails {
                failures.push(format!(
                    "{name}: discriminating control {id} passes the predicate"
                ));
            } else if !*discriminates && !fails {
                undiscriminating.push(format!("{name}:{id}"));
            }
        }
    }
    Lane {
        tally,
        failures,
        floor_differences,
        undiscriminating,
    }
}

fn report_lane(
    tally: &BTreeMap<String, Tally>,
    failures: &[String],
    floor_differences: &[String],
    undiscriminating: &[String],
) {
    for (family, t) in tally {
        println!("{family}: {t:?}");
    }
    println!("non-discriminating controls: {}", undiscriminating.len());
    for f in failures {
        println!("FAILURE {f}");
    }
    for d in floor_differences {
        println!("FLOOR {d}");
    }
}

fn lane_counts(cases: &[R1Case], tally: &BTreeMap<String, Tally>) {
    let count = |f: &str| tally.get(f).map_or(0, |t| t.cases);
    assert_eq!(
        [
            count("RF-CHAIN"),
            count("RF-SKEW"),
            count("RF-WEAK"),
            count("RF-FINITE"),
            count("RF-MECH"),
            count("RF-CANCEL")
        ],
        [30, 36, 9, 6, 9, 38]
    );
    assert_eq!(tally["RF-MECH"].refused, 8);
    let not_covered = |f: &str| tally.get(f).map_or(0, |t| t.not_covered);
    assert_eq!(
        [
            not_covered("RF-WEAK"),
            not_covered("RF-CANCEL"),
            not_covered("RF-SKEW")
        ],
        [46, 3, 2]
    );
    assert_eq!(
        tally
            .values()
            .map(|t| t.absolute_range_passes)
            .sum::<usize>(),
        0
    );
    // Both represented-basis cases are compared on their represented values.
    assert_eq!(cases.iter().filter(|c| c.basis == "represented").count(), 2);
}

const R1_LARGE: &str = include_str!("r1_large.txt");

/// R1's RF-LARGE frames through the lane (plan §6; D1 revision 5a.3): each is
/// selected at 128, honest against R1's references under the 1e-9 predicate,
/// with nothing not covered, and R1's discriminating controls fail it.
fn lane_large(names: &[&str]) {
    let cases: Vec<R1Case> = r1_cases_from(R1_LARGE)
        .into_iter()
        .filter(|c| names.contains(&c.model.name.as_str()))
        .collect();
    assert_eq!(cases.len(), names.len());
    let Lane {
        tally,
        failures,
        floor_differences,
        undiscriminating,
    } = run_lane(&cases);
    report_lane(&tally, &failures, &floor_differences, &undiscriminating);
    assert!(failures.is_empty(), "{} reference failures", failures.len());
    assert!(floor_differences.is_empty());
    let t = &tally["RF-LARGE"];
    assert_eq!(t.selected_at.get(&128).copied(), Some(names.len()));
    assert_eq!(t.not_covered, 0);
    assert!(t.passes >= 20 * names.len(), "{t:?}");
}

#[test]
fn rf_large_at_10_members_is_selected_at_128_and_honest() {
    lane_large(&[
        "RF-LARGE-CHAIN-n00010-AX",
        "RF-LARGE-CHAIN-n00010-ROT",
        "RF-LARGE-TREE-n00010-AX",
        "RF-LARGE-TREE-n00010-ROT",
        "RF-LARGE-CONT-n00010-AX",
        "RF-LARGE-CONT-n00010-ROT",
    ]);
}

#[test]
fn rf_large_chain_ax_at_100_members_is_selected_at_128_and_honest() {
    lane_large(&["RF-LARGE-CHAIN-n00100-AX"]);
}

#[test]
fn rf_large_tree_ax_at_100_members_is_selected_at_128_and_honest() {
    lane_large(&["RF-LARGE-TREE-n00100-AX"]);
}

#[test]
fn rf_large_chain_rot_at_100_members_is_selected_at_128_and_honest() {
    lane_large(&["RF-LARGE-CHAIN-n00100-ROT"]);
}

#[test]
fn rf_large_tree_rot_at_100_members_is_selected_at_128_and_honest() {
    lane_large(&["RF-LARGE-TREE-n00100-ROT"]);
}

#[test]
fn rf_large_cont_ax_at_100_members_is_selected_at_128_and_honest() {
    lane_large(&["RF-LARGE-CONT-n00100-AX"]);
}

#[test]
fn rf_large_cont_rot_at_100_members_is_selected_at_128_and_honest() {
    lane_large(&["RF-LARGE-CONT-n00100-ROT"]);
}

// ---------------------------------------------------------------- the N series, NP-A and the routed cases

fn solved(name: &str) -> (models::Model, PrimitiveSource, Box<RetainedSolve>) {
    let m = models::model(name);
    let source = m.source();
    let mut meter = InvocationMeter::new(u64::MAX);
    match solve_case(source.clone(), CaseLimit::new(u64::MAX), &mut meter) {
        CaseOutcome::Selected(s) => (m, source, s),
        other => panic!("{name}: {other:?}"),
    }
}

fn within(name: &str, min_compared: usize) -> Box<RetainedSolve> {
    let (m, source, solve) = solved(name);
    let (worst, at, compared) = models::compare(&source, &solve.publish().rows, &m.expect);
    assert!(compared >= min_compared, "{name}: {compared} compared");
    assert!(worst <= 1.0, "{name}: {worst} at {at}");
    solve
}

fn value(solve: &RetainedSolve, key: &str) -> f64 {
    models::published(&solve.publish().rows)[key].0
}

#[test]
fn n05_and_n06_agree_with_the_intended_basis_and_n05_is_not_np_as_represented_answer() {
    for name in ["N05", "N06", "N05-TRANSVERSE"] {
        let solve = within(name, 40);
        assert_eq!(solve.selected_precision(), 128, "{name}");
        // The member's internal torque and the root spring action (§7.3-2).
        assert!(value(&solve, "T.1") != 0.0, "{name}");
        assert!(value(&solve, "spr.1.3") != 0.0, "{name}");
    }
    let (root, tip) = models::np_a_n05();
    let solve = within("N05", 40);
    for (key, represented) in [("u.0.3", root), ("u.1.3", tip)] {
        let ours = value(&solve, key);
        println!("N05 {key}: K4 {ours:e}, NP-A represented {represented:e}");
        assert!(
            (ours - represented).abs() > 1e-9 * ours.abs(),
            "{key}: K4 gives NP-A's represented answer"
        );
    }
}

#[test]
fn k_d5s_d5c_1_controls_and_rf_skew_t_cant_off_122_r1e_04_are_within_1e_minus_9() {
    for name in ["PROBE_D", "PROBE_C", "BENDING_SOFT"] {
        let solve = within(name, 30);
        println!("{name}: selected {}", solve.selected_precision());
    }
    let case = r1_cases()
        .into_iter()
        .find(|c| c.model.name == "RF-SKEW-T-CANT-OFF-122-r1e-04")
        .unwrap();
    let source = case.model.source();
    let mut meter = InvocationMeter::new(u64::MAX);
    let CaseOutcome::Selected(solve) =
        solve_case(source.clone(), CaseLimit::new(u64::MAX), &mut meter)
    else {
        panic!()
    };
    let obs = observed(&source, &solve.publish().rows, &case.kmem);
    let mut compared = 0;
    for row in &case.rows {
        assert!(
            passes(obs[&row.key], row.exp, row.scale),
            "{} ({})",
            row.key,
            row.r1
        );
        compared += 1;
    }
    assert!(compared > 20);
}

#[test]
fn the_spring_carried_case_is_evaluated_its_underflowing_actions_published_explicitly() {
    // K2b's ruling A: W1 evaluates it. θ = 1/(k + GJ/L) with GJ/L ≈ 2^-1081:
    // exactly 1.0 in binary64; the member torque and the root reaction are
    // about 2^-1081, below binary64's range.
    let (_, _, solve) = solved("SPRING-CARRIED");
    let rows = &solve.publish().rows;
    let by_key: BTreeMap<String, &PublishedRow> =
        rows.iter().map(|r| (models::key(&r.id), r)).collect();
    println!(
        "SPRING-CARRIED: selected {}; u.1.3 {:?}; end.1.j.3 {:?}; R.0.3 {:?}; spr.1.3 {:?}",
        solve.selected_precision(),
        by_key["u.1.3"].value,
        by_key["end.1.j.3"].value,
        by_key["R.0.3"].value,
        by_key["spr.1.3"].value
    );
    assert_eq!(by_key["u.1.3"].value, Binary64Outcome::Normal(1.0));
    assert_eq!(by_key["spr.1.3"].value, Binary64Outcome::Normal(-1.0));
    for key in ["end.1.j.3", "R.0.3"] {
        assert!(
            matches!(by_key[key].value, Binary64Outcome::Underflow { .. }),
            "{key}: {:?}",
            by_key[key].value
        );
        assert_eq!(by_key[key].class, RowClass::Unpublishable);
    }
}

#[test]
fn b1_l_is_exact_with_the_ledger() {
    let solve = within("B1-L", 30);
    assert_eq!(solve.selected_precision(), 128);
}

// ---------------------------------------------------------------- the exact-block oracle (plan §12.2)

/// The represented binary64 system of a source as the product forms it (FK's
/// element formation, springs added to the diagonal, loads summed in order).
fn represented_system(
    source: &PrimitiveSource,
) -> (
    Vec<Vec<f64>>,
    Vec<f64>,
    Vec<crate::structural::StiffnessContribution>,
) {
    let n = source.dof_count();
    let mut k = vec![vec![0.0f64; n]; n];
    let mut contributions = Vec::new();
    for m in source.members() {
        let node = |i: u32| crate::FrameNode::new(i as usize, source.nodes()[i as usize]).unwrap();
        let section = crate::FrameSection {
            elastic_modulus: m.elastic_modulus,
            shear_modulus: m.shear_modulus,
            area: m.area,
            second_moment_y: m.second_moment_y,
            second_moment_z: m.second_moment_z,
            torsion_constant: m.torsion_constant,
        };
        let element =
            crate::FrameElement::new(node(m.node_i), node(m.node_j), section, m.y_reference)
                .unwrap();
        let ke = element.global_stiffness().unwrap();
        let dofs: Vec<usize> = (0..6)
            .map(|c| m.node_i as usize * 6 + c)
            .chain((0..6).map(|c| m.node_j as usize * 6 + c))
            .collect();
        for a in 0..12 {
            for b in 0..12 {
                k[dofs[a]][dofs[b]] += ke[a][b];
                contributions.push(crate::structural::StiffnessContribution {
                    row: dofs[a],
                    col: dofs[b],
                    value: ke[a][b],
                });
            }
        }
    }
    for s in source.springs() {
        let d = s.dof.global();
        k[d][d] += s.stiffness;
        contributions.push(crate::structural::StiffnessContribution {
            row: d,
            col: d,
            value: s.stiffness,
        });
    }
    let mut f = vec![0.0f64; n];
    for l in source.loads() {
        f[l.dof.global()] += l.value;
    }
    (k, f, contributions)
}

#[test]
fn w1_agrees_with_the_exact_block_oracle_on_the_in_scope_cases() {
    use crate::structural::exact_boundary::{Context, ForceBasis, Limits};
    use crate::structural::StructuralSystem;
    let names = [
        "N01", "N05", "N06", "N08-0", "N08-1", "N08-2", "N08-3", "N09-B", "N09-T",
    ];
    for name in names {
        let (_, source, solve) = solved(name);
        let (k, f, contributions) = represented_system(&source);
        let free = source.free_dofs();
        let prescribed: Vec<(usize, f64)> = source
            .constraints()
            .iter()
            .map(|c| (c.dof.global(), c.value))
            .collect();
        let system = StructuralSystem {
            stiffness: &k,
            force: &f,
            free_dofs: &free,
            prescribed: &prescribed,
            contributions: Some(&contributions),
            symmetry: None,
        };
        let context =
            match Context::new(&system, name, ForceBasis::DeclaredVector, Limits::default()) {
                Ok(c) => c,
                Err(e) => panic!("{name}: out of the exact-block scope: {e:?}"),
            };
        let response = context.solve().unwrap();
        let obs = models::published(&solve.publish().rows);
        let mut oracle = BTreeMap::new();
        for &d in &free {
            let p = response
                .project_displacement(d, 1e-9, 10_000_000)
                .result
                .unwrap();
            oracle.insert(format!("u.{}.{}", d / 6, d % 6), p.value());
        }
        for &(d, _) in &prescribed {
            let p = response
                .project_reaction(d, 1e-9, 10_000_000)
                .result
                .unwrap();
            oracle.insert(format!("R.{}.{}", d / 6, d % 6), p.value());
        }
        // The body-level coupled scale of the oracle's values (D1 §4.4.1 item 4).
        let mut s = [0.0f64; 4];
        for (key, &v) in &oracle {
            s[obs[key].1.index()] = s[obs[key].1.index()].max(v.abs());
        }
        let scale = coupled_scales(s, body_extent(source.nodes()));
        let mut worst = (0.0f64, String::new());
        for (key, &o) in &oracle {
            let (w, kind, _) = obs[key];
            let ratio = (w - o).abs() / (1e-9 * o.abs().max(scale[kind.index()]));
            if ratio > worst.0 {
                worst = (ratio, key.clone());
            }
        }
        println!(
            "{name}: {} oracle values, worst {:.3e} at {}",
            oracle.len(),
            worst.0,
            worst.1
        );
        assert!(worst.0 <= 1.0, "{name}: {} at {}", worst.0, worst.1);
    }
}
