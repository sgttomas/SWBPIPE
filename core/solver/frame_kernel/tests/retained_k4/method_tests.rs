//! K4 tests of D1 revision 5a.3's method, checkpoint A3b (mounted in
//! `adaptive.rs`, so they reach the schedule's internals):
//! - every control's schedule against GEN's (`outcomes.txt`, ROOT's A3-0
//!   ruling Q1) and against R7's expectations (plan §6), with each selected
//!   control honest against its exact solution and passing a test-only G5a
//!   checker (R7 §6.3 items 1–6; D2 owns G5a);
//! - E-CHARGE (`charge.txt`) and E-ESTIMATE (`estimate.txt`) at every
//!   verification, with the named controls' charge, θ and estimate figures;
//! - E-HEADROOM on every state pair;
//! - SD-G5's vectors of `decide`, the gate's best state and the g check.
use super::super::combine::{CombinationOutcome, CombinationReason, RetainedCombination};
use super::super::directed::binary64_up;
use super::super::recover::{End, Kind, QuantityId, QuantityMeta};
use super::super::source::{
    Component, Constraint, Dof, NodalLoad, PrimitiveSource, SourceParts, StraightMember,
};
use super::super::verify::{
    build_verify_shared, e_hat, formation_scale, resolution_scale, verify_state, BlockNorms,
    StageRounding, VerificationReport,
};
use super::*;
use std::collections::BTreeMap;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;
use support::{sha256_hex, tok};

const MODELS_5A3: &str = include_str!("models5a3.txt");
const OUTCOMES: &str = include_str!("outcomes.txt");
const CHARGE: &str = include_str!("charge.txt");
const ESTIMATE: &str = include_str!("estimate.txt");

fn all_models() -> Vec<models::Model> {
    let mut out = models::models();
    out.extend(models::parse_models(MODELS_5A3));
    out
}

/// RF-LARGE at 100 members runs in the references lane (plan §6, Q13).
fn in_default_lane(name: &str) -> bool {
    !name.contains("n00100")
}

/// R7 §7's SEED hook for the duration of a solve (cleared on drop, so a
/// failing test leaves no seed on its thread).
struct Seeded;

impl Seeded {
    fn new(seeds: &[(usize, f64)]) -> Self {
        seed::set(seeds.to_vec());
        Seeded
    }
}

impl Drop for Seeded {
    fn drop(&mut self) {
        seed::set(Vec::new());
    }
}

fn run(m: &models::Model) -> CaseOutcome {
    let _seeded = Seeded::new(&m.seeds);
    let mut meter = InvocationMeter::new(u64::MAX);
    solve_case(m.source(), CaseLimit::new(u64::MAX), &mut meter)
}

fn stop_name(stop: &AttemptStop) -> String {
    match stop {
        AttemptStop::Pivot { .. } => "Pivot".into(),
        AttemptStop::NegativeEnergy { .. } => "NegativeEnergy".into(),
        AttemptStop::Condition => "Condition".into(),
        AttemptStop::ResidualGate { .. } => "ResidualGate".into(),
        AttemptStop::ZeroDiagonal { .. } => "ZeroDiagonal".into(),
        AttemptStop::ResolutionScale { .. } => "ResolutionScale".into(),
        other => format!("{other:?}"),
    }
}

/// GEN's tokens for a list of attempts: `p:outcome[:detail]`, the detail a
/// layout index (stop rule, estimate, charge), a body (uc, θ) or a member id
/// (g); a verification solved but never needed as a candidate is not listed.
fn tokens(attempts: &[AttemptRecord], layout: &[QuantityMeta]) -> Vec<String> {
    let index = |q: &QuantityId| layout.iter().position(|m| m.id == *q).unwrap();
    attempts
        .iter()
        .filter_map(|a| {
            let what = match &a.outcome {
                AttemptOutcome::Accepted => "accepted".to_string(),
                AttemptOutcome::Verified => "verified".to_string(),
                AttemptOutcome::Solved => return None,
                AttemptOutcome::Failed(AttemptReason::Stop(s)) => {
                    format!("failed:{}", stop_name(s))
                }
                AttemptOutcome::Failed(other) => panic!("{other:?}"),
                AttemptOutcome::Rejected(r) => format!(
                    "rejected:{}",
                    match r {
                        AttemptReason::VerificationFailed => "verification_failed".to_string(),
                        AttemptReason::StopRule { quantity, .. } => {
                            format!("stop_rule:{}", index(quantity))
                        }
                        AttemptReason::VerificationEstimate { quantity, .. } => {
                            format!("verification_estimate:{}", index(quantity))
                        }
                        AttemptReason::Uc { body } => format!("uc:{body}"),
                        AttemptReason::Theta { body } => format!("theta:{body}"),
                        AttemptReason::GValidity { member } => format!("g_validity:{member}"),
                        AttemptReason::Charge { quantity, .. } => {
                            format!("charge:{}", index(quantity))
                        }
                        AttemptReason::Stop(s) => panic!("{s:?}"),
                    }
                ),
            };
            Some(format!("{}:{what}", a.precision))
        })
        .collect()
}

struct Outcome {
    selected: Option<u32>,
    tokens: Vec<String>,
    solve: Option<Box<RetainedSolve>>,
    attempts: Vec<AttemptRecord>,
    refused: Option<Refusal>,
    unresolved: Option<String>,
}

fn outcome_of(out: CaseOutcome, layout: &[QuantityMeta]) -> Outcome {
    match out {
        CaseOutcome::Selected(s) => Outcome {
            selected: Some(s.selected_precision()),
            tokens: tokens(&s.evidence().attempts, layout),
            attempts: s.evidence().attempts.clone(),
            solve: Some(s),
            refused: None,
            unresolved: None,
        },
        CaseOutcome::Unresolved {
            reason, attempts, ..
        } => Outcome {
            selected: None,
            tokens: tokens(&attempts, layout),
            attempts,
            solve: None,
            refused: None,
            unresolved: Some(format!("{reason:?}")),
        },
        CaseOutcome::Refused { refusal, .. } => Outcome {
            selected: None,
            tokens: Vec::new(),
            attempts: Vec::new(),
            solve: None,
            refused: Some(refusal),
            unresolved: None,
        },
    }
}

/// GEN's schedule per name: (selected precision or "-", tokens).
fn gen_outcomes() -> BTreeMap<String, (String, Vec<String>)> {
    OUTCOMES
        .lines()
        .map(|l| {
            let f: Vec<&str> = l.split_whitespace().collect();
            assert_eq!(f[0], "outcome");
            (
                f[1].to_string(),
                (
                    f[2].to_string(),
                    f[3..].iter().map(|t| t.to_string()).collect(),
                ),
            )
        })
        .collect()
}

/// Every control (RF-LARGE at 100 members excepted: the references lane) and
/// GEN's 5a.3 combinations.
fn run_controls() -> BTreeMap<String, Outcome> {
    let mut out = BTreeMap::new();
    let all = all_models();
    for m in all.iter().filter(|m| in_default_lane(&m.name)) {
        let layout = CasePrep::new(m.source()).unwrap().layout;
        out.insert(m.name.clone(), outcome_of(run(m), &layout));
    }
    for c in models::parse_combos(MODELS_5A3) {
        let operands: Vec<(f64, Box<RetainedSolve>)> = c
            .operands
            .iter()
            .map(|(f, n)| {
                let m = all.iter().find(|m| m.name == *n).unwrap();
                match run(m) {
                    CaseOutcome::Selected(s) => (*f, s),
                    other => panic!("{}: operand {n}: {other:?}", c.name),
                }
            })
            .collect();
        let refs: Vec<(f64, &RetainedSolve)> =
            operands.iter().map(|(f, s)| (*f, s.as_ref())).collect();
        let layout = operands[0].1.prep.layout.clone();
        let mut meter = InvocationMeter::new(u64::MAX);
        let o = match RetainedCombination::solve(&refs, CaseLimit::new(u64::MAX), &mut meter) {
            CombinationOutcome::Selected(s) => outcome_of(CaseOutcome::Selected(s), &layout),
            CombinationOutcome::Unresolved { reason, attempts } => Outcome {
                selected: None,
                tokens: tokens(&attempts, &layout),
                attempts,
                solve: None,
                refused: None,
                unresolved: Some(format!("{reason:?}")),
            },
        };
        out.insert(c.name.clone(), o);
    }
    out
}

/// R7 §6.3's G5a, items 1–6, in binary64 as D2 pins them, on a selected
/// case's evidence and publication (a test-only checker; D2 owns G5a). The
/// rows are in SI units, so item 4's conversion is the identity.
fn g5a(solve: &RetainedSolve) -> Result<(), String> {
    let ev = solve.evidence();
    let publication = solve.publish();
    let source = solve.source();
    let extents = &solve.prep.extents;
    let c = f64::from_bits(0x3FF0_0000_0000_1000);
    let scale = |body: u32, kind: Kind| -> f64 {
        let bits = publication
            .body_scales
            .iter()
            .find(|s| s.0 == body && s.1 == kind)
            .unwrap()
            .2;
        f64::from_bits(bits)
    };
    for body in 0..source.body_count() {
        let has_fm = publication
            .rows
            .iter()
            .any(|r| r.body == body && matches!(r.kind, Kind::Force | Kind::Moment));
        let entry = ev.resolution_scale.iter().find(|e| e.0 == body);
        // 1. Shape.
        let Some(&(_, fo_bits, mo_bits)) = entry else {
            if has_fm {
                return Err(format!("body {body}: no resolution_scale"));
            }
            continue;
        };
        let e = [f64::from_bits(fo_bits), f64::from_bits(mo_bits)];
        if e.iter().any(|v| !v.is_finite() || v.is_sign_negative()) {
            return Err(format!("body {body}: E {e:?}"));
        }
        let hat = e_hat(e, extents[body as usize]);
        for (k, kind) in [(0, Kind::Force), (1, Kind::Moment)] {
            // 2. The zero rule.
            if e[k] == 0.0 {
                for r in publication
                    .rows
                    .iter()
                    .filter(|r| r.body == body && r.kind == kind)
                {
                    if r.value.value().is_some_and(|v| v.to_bits() != 0) {
                        return Err(format!("body {body}: E = 0 with {:?} published", r.id));
                    }
                }
            }
            // 3. The sanity bound.
            if hat[k] * c < scale(body, kind) {
                return Err(format!("body {body} {kind:?}: ê·c below S*"));
            }
        }
        // 4. The lower bound.
        let value = |g: usize| -> f64 {
            publication
                .rows
                .iter()
                .find(|r| r.id == QuantityId::Displacement(Dof::from_global(g)))
                .and_then(|r| r.value.value())
                .unwrap_or(0.0)
                .abs()
        };
        let norm = |node: u32, offset: usize| {
            let g = 6 * node as usize + offset;
            (value(g) + value(g + 1)) + value(g + 2)
        };
        let (s_tr, s_ro) = (scale(body, Kind::Translation), scale(body, Kind::Rotation));
        let (mut lb_fo, mut lb_mo) = (0.0f64, 0.0f64);
        for m in source
            .members()
            .iter()
            .filter(|m| source.body_of_node(m.node_i) == body)
        {
            let (xi, xj) = (
                source.nodes()[m.node_i as usize],
                source.nodes()[m.node_j as usize],
            );
            let d = [xj[0] - xi[0], xj[1] - xi[1], xj[2] - xi[2]];
            let length = ((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2]).sqrt();
            let ka = (m.elastic_modulus * m.area) / length;
            let kt = (m.shear_modulus * m.torsion_constant) / length;
            let nt = norm(m.node_i, 0) + norm(m.node_j, 0);
            let nr = norm(m.node_i, 3) + norm(m.node_j, 3);
            if nt > 2f64.powi(-59) * s_tr {
                lb_fo = lb_fo.max(ka * (nt - 2f64.powi(-60) * s_tr));
            }
            if nr > 2f64.powi(-59) * s_ro {
                lb_mo = lb_mo.max(kt * (nr - 2f64.powi(-60) * s_ro));
            }
        }
        if hat[0] * c < lb_fo || hat[1] * c < lb_mo {
            return Err(format!(
                "body {body}: lower bound {lb_fo:e} {lb_mo:e} above ê·c {:e} {:e}",
                hat[0] * c,
                hat[1] * c
            ));
        }
    }
    // 5. and 6. The summaries.
    for &(b, k, v) in &ev.verification_estimate {
        if v.is_nan() || v > 0.25 {
            return Err(format!("estimate {b} {k:?} {v:e}"));
        }
    }
    for &(b, k, v) in &ev.verification_charge {
        if v.is_nan() || v > 1.0 {
            return Err(format!("charge {b} {k:?} {v:e}"));
        }
    }
    for &(b, t) in &ev.theta {
        if t.is_nan() || t > 0.5 {
            return Err(format!("theta {b} {t:e}"));
        }
    }
    for &(b, bits) in &ev.certified_bound {
        let v = f64::from_bits(bits);
        if !(v.is_finite() && v > 0.0) {
            return Err(format!("B {b} {v:e}"));
        }
    }
    Ok(())
}

fn strip(token: &str) -> String {
    let f: Vec<&str> = token.split(':').collect();
    f[..f.len().min(3)].join(":")
}

fn reason_kinds(o: &Outcome) -> Vec<String> {
    o.tokens.iter().map(|t| strip(t)).collect()
}

#[test]
#[allow(clippy::too_many_lines)]
fn every_control_follows_gens_schedule_and_r7s_expectations_honestly() {
    let gen = gen_outcomes();
    let runs = run_controls();
    // GEN's schedule, token for token (the geometry refusals K4 makes before
    // any attempt excepted: GEN has no geometry screen).
    let mut compared = 0;
    for (name, o) in &runs {
        let (sel, toks) = gen
            .get(name)
            .unwrap_or_else(|| panic!("{name}: no GEN outcome"));
        if let Some(refusal) = &o.refused {
            assert!(
                ["N02", "N03-RZ", "N04"].contains(&name.as_str())
                    && matches!(refusal, Refusal::MechanismWitnessed { .. })
                    && sel == "-",
                "{name}: {refusal:?}"
            );
            continue;
        }
        let got = o.selected.map_or("-".to_string(), |p| p.to_string());
        assert_eq!((&got, &o.tokens), (sel, toks), "{name}");
        compared += 1;
    }
    assert_eq!(
        gen.keys().filter(|n| in_default_lane(n)).count(),
        runs.len()
    );
    assert!(compared >= 115, "{compared}");

    // R7's expectations (plan §6), independent of GEN.
    let expect = |name: &str, sel: Option<u32>, kinds: &[&str]| {
        let o = &runs[name];
        assert_eq!(o.selected, sel, "{name}");
        assert_eq!(reason_kinds(o), kinds, "{name}");
    };
    let at_128 = ["128:accepted", "256:verified"];
    for k in ["k90-s40", "k100-s40", "k110-s20"] {
        expect(
            &format!("LEVER2-{k}"),
            None,
            &[
                "128:failed:Pivot",
                "256:rejected:verification_estimate",
                "512:rejected:stop_rule",
            ],
        );
        expect(
            &format!("TILT-LEVER-{k}"),
            None,
            &[
                "128:failed:Pivot",
                "256:rejected:stop_rule",
                "512:rejected:stop_rule",
            ],
        );
    }
    for k in ["k90-s40", "k100-s40", "k110-s20"] {
        let o = &runs[&format!("LEVER2-{k}")];
        assert_eq!(o.unresolved.as_deref(), Some("Ceiling"));
        // The gate's residual at 256 is 0 (the assembled residual vanishes).
        let a256 = o.attempts.iter().find(|a| a.precision == 256).unwrap();
        assert_eq!(a256.residual_worst, Some(0.0), "LEVER2-{k}");
    }
    expect(
        "SEEDED-COMMON",
        None,
        &[
            "128:rejected:verification_estimate",
            "256:rejected:verification_estimate",
            "512:rejected:verification_estimate",
        ],
    );
    expect(
        "SEEDED-SOFT",
        None,
        &[
            "128:rejected:stop_rule",
            "256:rejected:stop_rule",
            "512:rejected:stop_rule",
        ],
    );
    // SEEDED-SOFT: at u(node 2, ux), through W⁺.
    assert!(runs["SEEDED-SOFT"]
        .tokens
        .iter()
        .all(|t| t.ends_with(":12")));
    for name in [
        "CHARGE-SLENDER",
        "HH-FOOL-m40",
        "HH-FOOL-m100",
        "HH-FOOL-m40-LOADED",
        "HH-SLENDER-m40",
        "LOADONLY-y345",
        "LOADONLY-y001",
        "GS-ROT-y345-LOADED",
        "THETA-ZERO-BODY",
        "G-FIXED-MEMBER",
        "THETA-STUB",
        "LEDGER-AT-RESTRAINT",
        "TWO-SPAN",
        "B1-L",
        "N05",
        "N06",
        "S8-W-1e-10",
        "S8-W-1e-14",
        "RF-LARGE-CHAIN-n00010-AX",
        "RF-LARGE-CHAIN-n00010-ROT",
        "RF-LARGE-TREE-n00010-AX",
        "RF-LARGE-TREE-n00010-ROT",
        "RF-LARGE-CONT-n00010-AX",
        "RF-LARGE-CONT-n00010-ROT",
    ] {
        expect(name, Some(128), &at_128);
    }
    let probes: Vec<&String> = runs.keys().filter(|n| n.starts_with("PROBE-y")).collect();
    assert_eq!(probes.len(), 20);
    for name in probes {
        expect(name, Some(128), &at_128);
    }
    assert_eq!(
        runs["TWO-SPAN"]
            .solve
            .as_ref()
            .unwrap()
            .evidence()
            .corrections,
        1
    );
    expect(
        "HH-FOOL-m100-LOADED",
        Some(256),
        &["128:rejected:stop_rule", "256:accepted", "512:verified"],
    );
    assert_eq!(
        runs["HH-FOOL-m100-LOADED"].tokens[0],
        "128:rejected:stop_rule:0"
    );
    expect(
        "THETA-STUB-COUPLED",
        Some(256),
        &["128:rejected:theta", "256:accepted", "512:verified"],
    );
    expect(
        "G-PRESC-MEMBER",
        Some(256),
        &["128:rejected:g_validity", "256:accepted", "512:verified"],
    );
    expect(
        "SKEW-K1E-28",
        Some(256),
        &["128:failed:Condition", "256:accepted", "512:verified"],
    );
    expect(
        "PIVOT",
        Some(256),
        &["128:failed:Pivot", "256:accepted", "512:verified"],
    );
    for name in ["SKEW6-K1E-12", "REACTIONS-ONLY", "M10-G"] {
        expect(
            name,
            Some(256),
            &["128:rejected:stop_rule", "256:accepted", "512:verified"],
        );
    }
    // REACTIONS-ONLY: at R(0, Uy).
    let reactions = &runs["REACTIONS-ONLY"];
    let layout = CasePrep::new(models::model("REACTIONS-ONLY").source())
        .unwrap()
        .layout;
    let index: usize = reactions.tokens[0]
        .rsplit(':')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        layout[index].id,
        QuantityId::Reaction(Dof {
            node: 0,
            component: Component::Uy
        })
    );
    for name in [
        "F-2",
        "F-2-SPOS",
        "F-2-CEIL",
        "PRESCRIBED-TAIL",
        "PRESCRIBED-TAIL-FREE",
        "ASSEMBLY-SAT",
        "DEMOTION2",
        "EXACT-RIGID",
        "RIGID-UNLOADED",
        "F-3-FREE",
        "F-3-ROT",
        "GS-TRANS-y345",
        "GS-ROT-y345",
        "M7-GS1",
        "M10-ANISO",
    ] {
        expect(
            name,
            Some(512),
            &[
                "128:rejected:stop_rule",
                "256:rejected:stop_rule",
                "512:accepted",
                "1024:verified",
            ],
        );
    }
    for name in ["R115-SEED3", "SKEW-K1E-60"] {
        expect(
            name,
            Some(512),
            &[
                "128:failed:Pivot",
                "256:rejected:stop_rule",
                "512:accepted",
                "1024:verified",
            ],
        );
    }
    // CEIL5A3 (ROOT's A3-0 ruling Q14, pinned after GEN confirmed it): both
    // operands selected at 512, their combination unresolved.
    for name in ["CEIL5A3-A", "CEIL5A3-B"] {
        expect(
            name,
            Some(512),
            &[
                "128:failed:Pivot",
                "256:rejected:verification_estimate",
                "512:accepted",
                "1024:verified",
            ],
        );
    }
    expect(
        "CEIL5A3",
        None,
        &[
            "128:failed:Pivot",
            "256:rejected:verification_estimate",
            "512:rejected:stop_rule",
        ],
    );
    assert_eq!(
        runs["CEIL5A3"].unresolved.as_deref(),
        Some(format!("{:?}", CombinationReason::CombinationUnresolved).as_str())
    );
    // K4's own controls that 5a.3 moves (reported at A3b): DIRECTIONAL-SPAN is
    // withheld (the estimate at dspr 1's Rx; emu7 agrees), CEIL-A and CEIL-B
    // meet an unencodable E (a 2^1013-rad rigid rotation), RIGID-UNLOADED is
    // selected at 512 (R7).
    expect(
        "DIRECTIONAL-SPAN",
        None,
        &[
            "128:rejected:stop_rule",
            "256:rejected:verification_estimate",
            "512:rejected:verification_estimate",
        ],
    );
    // ROOT's A3b ruling: E finite at the 256 verification while
    // ê_mo = fl(L_b·E_fo) overflows is the same terminal case as E's overflow.
    expect(
        "EHAT-OVERFLOW",
        None,
        &[
            "128:rejected:verification_failed",
            "256:failed:ResolutionScale",
        ],
    );
    assert_eq!(
        runs["EHAT-OVERFLOW"].unresolved.as_deref(),
        Some(
            format!(
                "{:?}",
                UnresolvedReason::ResolutionScaleUnencodable {
                    body: 0,
                    kind: Kind::Moment
                }
            )
            .as_str()
        )
    );
    for name in ["CEIL-A", "CEIL-B"] {
        expect(
            name,
            None,
            &[
                "128:rejected:verification_failed",
                "256:failed:ResolutionScale",
            ],
        );
        assert!(runs[name]
            .unresolved
            .as_deref()
            .unwrap()
            .starts_with("ResolutionScaleUnencodable"));
    }

    // No silent move from 5a.2: every models.txt case selected at 128 passed
    // the coalesced gate at 128 and 256, where 5a.2's weaker rule accepts too.
    for m in models::models() {
        let o = &runs[&m.name];
        if o.selected == Some(128) {
            for a in &o.attempts {
                assert_eq!(
                    a.gate,
                    Some(GateTest::Coalesced),
                    "{} {}",
                    m.name,
                    a.precision
                );
            }
        }
    }

    // Honesty against the exact solutions, and G5a, on every selected control.
    let mut honest = 0;
    let mut expectations: BTreeMap<String, BTreeMap<String, f64>> = all_models()
        .into_iter()
        .map(|m| (m.name, m.expect))
        .collect();
    for c in models::parse_combos(MODELS_5A3) {
        expectations.insert(c.name, c.expect);
    }
    let mut gates = Vec::new();
    for (name, o) in &runs {
        let Some(solve) = &o.solve else { continue };
        g5a(solve).unwrap_or_else(|e| panic!("{name}: G5a {e}"));
        for a in &o.attempts {
            if let Some(GateTest::Bounded { state, evaluated }) = a.gate {
                gates.push(format!("{name} {} {state}/{evaluated}", a.precision));
            }
        }
        let expect = &expectations[name];
        if expect.is_empty() {
            continue;
        }
        let (worst, at, compared) =
            models::compare_honest(solve.source(), &solve.publish().rows, expect);
        assert!(
            compared > 0 && worst <= 1.0,
            "{name}: {worst} at {at} ({compared})"
        );
        honest += 1;
    }
    println!("bounded gates: {gates:?}");
    assert!(honest >= 90, "{honest}");

    // Item 6a on the controls: DEMOTION2's relative rows and EXACT-RIGID's b.
    let classes = |name: &str| {
        let s = runs[name].solve.as_ref().unwrap();
        let floored = s.publish();
        let values: Vec<_> = floored.rows.iter().map(|r| r.value).collect();
        let plain = classify_rows(&s.prep.layout, &values, &s.prep.extents);
        let relative = |p: &Publication| {
            p.rows
                .iter()
                .filter(|r| r.class == RowClass::RelativeVerified)
                .count()
        };
        let worst_b = floored
            .rows
            .iter()
            .filter(|r| matches!(r.kind, Kind::Force | Kind::Moment))
            .filter_map(|r| match r.class {
                RowClass::AbsoluteVerified { bound_bits } => Some(f64::from_bits(bound_bits)),
                _ => None,
            })
            .fold(0.0f64, f64::max);
        (relative(&plain), relative(floored), worst_b)
    };
    // DEMOTION2 (R7 §7: relative rows 7 → 4, b ≤ 4.7e-6 over both kinds, two
    // figures): item 6a demotes three rows, K4's layout counting its node
    // magnitudes as well.
    let (before, after, b) = classes("DEMOTION2");
    println!("DEMOTION2: relative rows {before} -> {after}, b {b:e}");
    assert!(
        before - after == 3 && b <= 4.75e-6,
        "{before} {after} {b:e}"
    );
    let (_, _, b) = classes("EXACT-RIGID");
    println!("EXACT-RIGID: b {b:e}");
    assert!(b > 0.0 && b < 1.2e-150, "{b:e}");
}

// ---------------------------------------------------------------- DIRECTIONAL-SPAN under 5a.2

const DIRECTIONAL_SPAN_EXACT: &str = include_str!("directional_span_exact.txt");

fn span_state(p: u32, prep: &CasePrep, group: &GroupPrep) -> PrecisionState {
    let guard = StageGuard::unlimited();
    match p {
        128 => {
            let sh = build_shared::<4, 4>(128, 192, &prep.source, group, guard)
                .result
                .unwrap();
            PrecisionState::P128(Arc::new(
                solve_case_at::<4, 4>(&sh, prep, group, guard)
                    .result
                    .unwrap(),
            ))
        }
        256 => {
            let sh = build_shared::<4, 8>(256, 320, &prep.source, group, guard)
                .result
                .unwrap();
            PrecisionState::P256(Arc::new(
                solve_case_at::<4, 8>(&sh, prep, group, guard)
                    .result
                    .unwrap(),
            ))
        }
        _ => {
            let sh = build_shared::<8, 16>(512, 576, &prep.source, group, guard)
                .result
                .unwrap();
            PrecisionState::P512(Arc::new(
                solve_case_at::<8, 16>(&sh, prep, group, guard)
                    .result
                    .unwrap(),
            ))
        }
    }
}

/// ROOT's A3b ruling: was 5a.2's publication of DIRECTIONAL-SPAN within its
/// claim? 5a.2's rule is `stop_rule` (no V and no (b)–(d)) on the same states
/// (every gate coalesced); q* is GEN's exact published quantity per layout
/// row, rounded to 512 bits (`directional_span_exact.txt`).
#[test]
fn directional_span_under_5a2_was_selected_at_256_and_published_within_its_claim() {
    let m = models::model("DIRECTIONAL-SPAN");
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    let guard = StageGuard::unlimited();
    let (PrecisionState::P128(s128), PrecisionState::P256(s256), PrecisionState::P512(s512)) = (
        span_state(128, &prep, &group),
        span_state(256, &prep, &group),
        span_state(512, &prep, &group),
    ) else {
        unreachable!()
    };
    for gate in [s128.gate, s256.gate, s512.gate] {
        assert_eq!(gate, GateTest::Coalesced);
    }
    // 5a.2's schedule: 128 rejected at u(0, Rx) (layout index 3), 256 accepted.
    let layout = &prep.layout;
    let extents = &prep.extents;
    let d = stop_rule(
        layout,
        extents,
        &s128.recovered.values,
        &s256.recovered.values,
        256,
        guard,
    );
    assert_eq!((d.result.unwrap(), d.first_failure), (false, Some(3)));
    let d = stop_rule(
        layout,
        extents,
        &s256.recovered.values,
        &s512.recovered.values,
        512,
        guard,
    );
    assert!(d.result.unwrap());
    // Its claim at 256, per row: |q_256 − q*| ≤ 2^-64·M_q with M_q =
    // max(|q_512|, S*) as its stop rule formed it; and each published row it
    // withheld as absolute_verified: |q_pub − q*| ≤ b·(1 + 2^-22).
    let qstar: Vec<Wide<8>> = DIRECTIONAL_SPAN_EXACT
        .lines()
        .map(|l| support::parse::<8>(l.split_whitespace().nth(3).unwrap()))
        .collect();
    assert_eq!(qstar.len(), layout.len());
    let mut ctx = WideContext::<8>::new(512).unwrap();
    let scales = scales_at(&mut ctx, layout, &s512.recovered.values, extents).unwrap();
    let mut values = PrecisionState::P256(s256.clone()).published();
    prep.publish_prescribed(&mut values);
    let publication = classify_rows(layout, &values, extents);
    let mut ctx16 = WideContext::<16>::new(1024).unwrap();
    let (mut worst, mut worst_at, mut absolute) = (0.0f64, 0usize, 0);
    for (index, meta) in layout.iter().enumerate() {
        let q2 = &s512.recovered.values[index];
        let s_star = &scales[meta.body as usize][meta.kind.index()];
        let magnitude = if q2.abs().cmp_value(s_star) == CmpOrdering::Greater {
            q2.abs()
        } else {
            *s_star
        };
        let mut err = ExactWideSum::new();
        err.add_wide(&s256.recovered.values[index], false).unwrap();
        err.add_wide(&qstar[index], true).unwrap();
        err.make_absolute();
        let err = err.round(&mut ctx16).unwrap();
        let r = ratio(&err, &magnitude.widen::<16>().mul_pow2(-64).unwrap());
        if r > worst {
            (worst, worst_at) = (r, index);
        }
        if let RowClass::AbsoluteVerified { bound_bits } = publication.rows[index].class {
            let b = f64::from_bits(bound_bits) * (1.0 + 2f64.powi(-22));
            let mut e = ExactWideSum::new();
            e.add_binary64(values[index].value().unwrap(), false)
                .unwrap();
            e.add_wide(&qstar[index], true).unwrap();
            e.make_absolute();
            e.add_binary64(b, true).unwrap();
            assert!(e.signum() <= 0, "{:?}: beyond b", meta.id);
            absolute += 1;
        }
    }
    println!(
        "DIRECTIONAL-SPAN, 5a.2 at 256: worst |q_256 − q*|/(2^-64·M) {worst:e} at {:?}; \
         {absolute} absolute rows within b",
        layout[worst_at].id
    );
    // Within its claim by far: 5a.3 withholds a correct publication (an
    // availability loss, not a false claim).
    assert!(worst < 2f64.powi(-70), "{worst:e}");
}

// ---------------------------------------------------------------- E-CHARGE and E-ESTIMATE

fn records<'a>(text: &'a str, name: &str, p: u32) -> Vec<Vec<&'a str>> {
    let p = p.to_string();
    text.lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .filter(|f| f.len() > 2 && f[1] == name && f[2] == p)
        .collect()
}

fn digest(lines: &[String]) -> String {
    let mut text = lines.join("\n");
    text.push('\n');
    sha256_hex(text.as_bytes())
}

fn opt_tok<const L: usize>(v: &Option<Wide<L>>) -> String
where
    Wide<L>: SupportedWidth,
{
    v.as_ref().map(tok).unwrap_or_else(|| "-".to_string())
}

fn rows_digest<const L: usize>(values: &[Option<Wide<L>>]) -> String
where
    Wide<L>: SupportedWidth,
{
    let lines: Vec<String> = values
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.as_ref().map(|v| format!("{i} {}", tok(v))))
        .collect();
    digest(&lines)
}

/// fl↑(num/den) for nonnegative values (∞ when only den is zero).
fn ratio<const L: usize>(num: &Wide<L>, den: &Wide<L>) -> f64
where
    Wide<L>: SupportedWidth,
{
    if num.is_zero() {
        return 0.0;
    }
    if den.is_zero() {
        return f64::INFINITY;
    }
    let mut ctx = WideContext::<L>::new(64).unwrap();
    binary64_up(&ctx.div(num, den).unwrap()).unwrap()
}

/// A verification report's figures that R7 names (plan §6's "also asserted").
#[derive(Debug, Default, Clone)]
struct Figures {
    /// max Ŵ_q/V_q and C_q/allowance over force and moment rows (P < 1024).
    estimate: f64,
    charge: f64,
    theta: f64,
    shifts: u8,
    tries: Vec<u8>,
    uc_missing_blocks: usize,
    b_is_uc_after_three: bool,
}

fn figures<const L: usize>(r: &VerificationReport<L>, prep: &CasePrep) -> Figures
where
    Wide<L>: SupportedWidth,
{
    let p = i64::from(r.precision);
    let hats: Vec<[f64; 2]> = r
        .resolution
        .iter()
        .zip(&prep.extents)
        .map(|(e, &x)| e_hat(*e, x))
        .collect();
    let mut f = Figures::default();
    for (index, meta) in prep.layout.iter().enumerate() {
        let k = match meta.kind {
            Kind::Force => 0,
            Kind::Moment => 1,
            _ => continue,
        };
        let e = Wide::<L>::from_f64(hats[meta.body as usize][k]).unwrap();
        if let Some(w) = &r.w[index] {
            f.estimate = f.estimate.max(ratio(w, &e.mul_pow2(8 - p).unwrap()));
        }
        if r.precision < 1024 {
            if let Some(c) = &r.charge[index] {
                let mut ctx = WideContext::<L>::new(64).unwrap();
                let sixty = Wide::<L>::from_f64(60.0).unwrap();
                let allowance = ctx.mul(&e, &sixty).unwrap().mul_pow2(-p).unwrap();
                f.charge = f.charge.max(ratio(c, &allowance));
            }
        }
    }
    let one = Wide::<L>::ONE;
    for t in r.theta.iter().flatten() {
        f.theta = f.theta.max(ratio(t, &one));
    }
    f.shifts = r.shift_factorizations;
    f.tries = r
        .blocks
        .iter()
        .map(|b| b.shift.as_ref().map_or(0, |s| s.tries))
        .collect();
    f.uc_missing_blocks = r
        .blocks
        .iter()
        .filter(|b| b.data && b.bound.uc.is_none())
        .count();
    f.b_is_uc_after_three = r.blocks.iter().any(|b| {
        b.shift
            .as_ref()
            .is_some_and(|s| s.tries == 3 && s.s.is_none())
            && b.b.is_some()
            && b.b == b.bound.uc
    });
    f
}

fn norms_tokens<const L: usize>(n: &BlockNorms<L>) -> Vec<String>
where
    Wide<L>: SupportedWidth,
{
    [&n.sas_one, &n.sas_inf, &n.sau, &n.sr, &n.sr2, &n.sid]
        .iter()
        .map(|v| tok(*v))
        .collect()
}

/// E-CHARGE's and E-ESTIMATE's checks of one model at one verification
/// precision; returns the figures (None when GEN records a stop).
#[allow(clippy::too_many_lines)]
fn verification_at<const L: usize, const R: usize, const W: usize>(
    p: u32,
    q: u32,
    m: &models::Model,
    prep: &CasePrep,
    group: &GroupPrep,
) -> Option<Figures>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
    Wide<W>: SupportedWidth,
{
    let name = m.name.as_str();
    let recs = records(CHARGE, name, p);
    assert!(!recs.is_empty() && recs[0][0] == "chg", "{name} {p}");
    let guard = StageGuard::unlimited();
    let _seeded = Seeded::new(&m.seeds);
    let shared = build_shared::<L, R>(p, q, &prep.source, group, guard).result;
    let solved = shared
        .as_ref()
        .map_err(Clone::clone)
        .and_then(|sh| solve_case_at::<L, R>(sh, prep, group, guard).result);
    let report = solved.and_then(|state| {
        let shared = shared.as_ref().unwrap();
        let vs = build_verify_shared::<L, R, W>(shared, &prep.source, group, guard)
            .result
            .unwrap_or_else(|e| panic!("{name} {p}: {e:?}"));
        verify_state::<L, R, W>(shared, &vs, prep, group, &state, guard).result
    });
    if recs[0][3] == "stop" {
        let stop = report.expect_err(name);
        assert_eq!(stop_name(&stop), recs[0][4], "{name} {p}");
        return None;
    }
    let r = report.unwrap_or_else(|e| panic!("{name} {p}: {e:?}"));
    let head = &recs[0];
    let got = [
        r.q_w.to_string(),
        r.uc_missing.map_or("-".to_string(), |b| b.to_string()),
        r.g_violation.map_or("-".to_string(), |g| g.to_string()),
    ];
    assert_eq!(got.to_vec(), head[4..7].to_vec(), "{name} {p} chg");
    let mut blocks_seen = 0;
    let mut bodies_seen = 0;
    for f in &recs[1..] {
        match f[0] {
            "chgrows" => {
                let r_hat: Vec<Option<Wide<L>>> = r.r_hat.iter().map(|v| Some(*v)).collect();
                let delta: Vec<Option<Wide<L>>> = r.delta.iter().map(|v| Some(*v)).collect();
                let got = [
                    rows_digest(&r_hat),
                    rows_digest(&delta),
                    rows_digest(&r.w),
                    rows_digest(&r.a_s),
                    rows_digest(&r.charge),
                    rows_digest(&r.w_plus),
                ];
                let labels = ["r̂", "δ̂", "Ŵ", "‖ā_q S‖₁", "C", "W⁺"];
                for (k, label) in labels.iter().enumerate() {
                    assert_eq!(got[k], f[3 + k], "{name} {p} {label}");
                }
            }
            "chgblk" => {
                let b: usize = f[3].parse().unwrap();
                let c = &r.blocks[b];
                let mut got = vec![
                    u8::from(c.data).to_string(),
                    opt_tok(&c.b),
                    opt_tok(&r.theta[b]),
                ];
                got.extend(norms_tokens(&r.norms[b]));
                got.push(c.shift.as_ref().map_or(0, |s| s.tries).to_string());
                assert_eq!(got, f[4..14].to_vec(), "{name} {p} block {b}");
                blocks_seen += 1;
            }
            "chgbody" => {
                let b: usize = f[3].parse().unwrap();
                let body = &r.bodies[b];
                let got = [
                    opt_tok(&body.b),
                    tok(&body.theta),
                    tok(&body.n_u),
                    tok(&body.t1),
                    tok(&body.t2),
                    tok(&body.t3),
                ];
                assert_eq!(got.to_vec(), f[4..10].to_vec(), "{name} {p} body {b}");
                bodies_seen += 1;
            }
            "chgcount" => {
                assert_eq!(
                    [r.shift_factorizations.to_string(), r.g_max.to_string()],
                    [f[3].to_string(), f[4].to_string()],
                    "{name} {p} count"
                );
            }
            other => panic!("{other}"),
        }
    }
    assert_eq!(
        (blocks_seen, bodies_seen),
        (r.blocks.len(), r.bodies.len()),
        "{name} {p}"
    );
    // E-ESTIMATE (P = 256 and 512): Ŵ_q against |R*(u_P) − q*| within a
    // relative 2^-8 on the rows GEN lists (error above 2^-(P+20)·ê).
    if p < 1024 {
        let est = records(ESTIMATE, name, p);
        if let Some(count) = est.iter().find(|f| f[0] == "estcount") {
            assert_ne!(count[3], "stop", "{name} {p}");
            let rows: Vec<&Vec<&str>> = est.iter().filter(|f| f[0] == "est").collect();
            assert_eq!(rows.len().to_string(), count[3], "{name} {p}");
            for f in rows {
                let index: usize = f[3].parse().unwrap();
                let exact = support::parse::<L>(f[4]);
                let w = r.w[index].unwrap_or_else(|| panic!("{name} {p} row {index}: no Ŵ"));
                let mut sum = ExactWideSum::new();
                sum.add_wide(&w, false).unwrap();
                sum.add_wide(&exact, true).unwrap();
                sum.make_absolute();
                sum.add_wide_scaled(&exact, true, 1, -8).unwrap();
                assert!(
                    sum.signum() <= 0,
                    "{name} {p} row {index}: Ŵ {} against {}",
                    tok(&w),
                    f[4]
                );
            }
        }
    }
    Some(figures(&r, prep))
}

#[test]
#[allow(clippy::too_many_lines)]
fn e_charge_and_e_estimate_equal_the_emulation_at_every_verification() {
    let mut checked = 0;
    let mut figs: BTreeMap<(String, u32), Figures> = BTreeMap::new();
    for m in all_models().iter().filter(|m| in_default_lane(&m.name)) {
        let prep = CasePrep::new(m.source()).unwrap();
        let Ok(group) = prepare_group(&prep.source) else {
            // Refused before any attempt; GEN's record is a stop.
            for p in [256u32, 512, 1024] {
                assert_eq!(records(CHARGE, &m.name, p)[0][3], "stop", "{}", m.name);
            }
            continue;
        };
        for p in [256u32, 512, 1024] {
            let f = match p {
                256 => verification_at::<4, 8, 8>(p, 320, m, &prep, &group),
                512 => verification_at::<8, 16, 16>(p, 576, m, &prep, &group),
                _ => verification_at::<16, 16, 16>(p, 1024, m, &prep, &group),
            };
            if let Some(f) = f {
                figs.insert((m.name.clone(), p), f);
            }
            checked += 1;
        }
    }
    assert!(checked >= 330, "{checked}");
    let fig = |name: &str, p: u32| figs[&(name.to_string(), p)].clone();
    for (key, f) in &figs {
        println!("FIG {} {} {f:?}", key.0, key.1);
    }
    // R7's figures (plan §6), emulated there; K4's within the stated margins.
    for (k, w) in [
        ("k90-s40", 2681.0),
        ("k100-s40", 2.7e6),
        ("k110-s20", 2.8e9),
    ] {
        let f = fig(&format!("LEVER2-{k}"), 512);
        assert!(
            (f.estimate / w - 1.0).abs() < 0.05,
            "LEVER2-{k}: W/V {:e}",
            f.estimate
        );
        assert!(f.charge > 1.0, "LEVER2-{k}: C/allowance {:e}", f.charge);
    }
    for k in ["k90-s40", "k100-s40", "k110-s20"] {
        let f = fig(&format!("TILT-LEVER-{k}"), 512);
        // R7's figures (≥ 1.0e30 and ≥ 6.0e10), to their two figures.
        assert!(
            f.charge >= 1.0e30 && f.theta >= 5.95e10,
            "TILT-LEVER-{k}: {f:?}"
        );
    }
    for p in [256, 512] {
        let f = fig("SEEDED-COMMON", p);
        assert!(
            f.charge <= 1.0 && f.estimate > 0.25,
            "SEEDED-COMMON {p}: {f:?}"
        );
    }
    for name in ["CHARGE-SLENDER", "HH-SLENDER-m40"] {
        let f = fig(name, 256);
        assert!((1.5e-3..1.7e-3).contains(&f.charge), "{name}: {f:?}");
    }
    // θ = 28 at the 256 verification, where the charge is 5.6e-12 of its
    // allowance.
    let f = fig("THETA-STUB-COUPLED", 256);
    assert!(
        (27.0..29.0).contains(&f.theta) && (5.5e-12..5.7e-12).contains(&f.charge),
        "{f:?}"
    );
    for (name, p) in [("HH-FOOL-m40-LOADED", 256), ("HH-FOOL-m100-LOADED", 512)] {
        let f = fig(name, p);
        assert!(
            f.b_is_uc_after_three && f.charge <= 6.3e-46,
            "{name} {p}: {f:?}"
        );
    }
    for shape in ["CHAIN", "TREE", "CONT"] {
        for dir in ["AX", "ROT"] {
            let name = format!("RF-LARGE-{shape}-n00010-{dir}");
            let expected = u8::from(!(shape == "CONT" && dir == "AX"));
            assert_eq!(fig(&name, 256).shifts, expected, "{name}");
        }
    }
    let probe_charge = figs
        .iter()
        .filter(|(k, _)| k.0.starts_with("PROBE-y") && k.1 == 256)
        .map(|(_, f)| f.charge)
        .fold(0.0f64, f64::max);
    println!("probe set: worst C/allowance at 256 {probe_charge:e}");
    assert!(probe_charge <= 1e-40, "{probe_charge:e}");
}

#[test]
fn e_charge_on_rf_large_at_100_members_at_256() {
    // Plan §6: the first σ succeeds except on CONT-AX (not run); Uc is missing
    // (t ≥ 1) on CHAIN-AX, CHAIN-ROT and TREE-ROT; the charge is far inside
    // its allowance.
    for m in all_models().iter().filter(|m| !in_default_lane(&m.name)) {
        let prep = CasePrep::new(m.source()).unwrap();
        let group = prepare_group(&prep.source).unwrap();
        let f = verification_at::<4, 8, 8>(256, 320, m, &prep, &group).unwrap();
        println!("FIG {} 256 {f:?}", m.name);
        let cont_ax = m.name.contains("CONT") && m.name.ends_with("-AX");
        let no_uc = ["CHAIN-n00100-AX", "CHAIN-n00100-ROT", "TREE-n00100-ROT"]
            .iter()
            .any(|n| m.name.contains(n));
        assert_eq!(f.shifts, u8::from(!cont_ax), "{}", m.name);
        assert_eq!(f.tries, vec![u8::from(!cont_ax)], "{}", m.name);
        assert_eq!(f.uc_missing_blocks, usize::from(no_uc), "{}", m.name);
        // R7's figure, 4.5e-44, to its two figures.
        assert!(f.charge <= 4.55e-44, "{}: {f:?}", m.name);
    }
}

// ---------------------------------------------------------------- E-HEADROOM

struct At<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    values: Vec<Wide<L>>,
    hat: Vec<[f64; 2]>,
}

/// The state at p and its own ê (E from this state, §4.1.6.2 item 4).
fn state_at<const L: usize, const R: usize>(
    p: u32,
    q: u32,
    m: &models::Model,
    prep: &CasePrep,
    group: &GroupPrep,
) -> Option<At<L>>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let guard = StageGuard::unlimited();
    let _seeded = Seeded::new(&m.seeds);
    let shared = build_shared::<L, R>(p, q, &prep.source, group, guard)
        .result
        .ok()?;
    let solved = solve_case_at::<L, R>(&shared, prep, group, guard)
        .result
        .ok()?;
    let mut ctx = WideContext::<L>::new(p).unwrap();
    let mut sum = ExactWideSum::new();
    let bounded: Vec<_> = shared.members.iter().map(|m| m.bounded()).collect();
    let abar = super::super::assemble::assemble_bounded(
        &mut ctx,
        &mut sum,
        &guard,
        &prep.source,
        &group.structure,
        &bounded,
        &shared.directional,
    )
    .unwrap();
    let w: Vec<Wide<L>> = solved.u.iter().map(|v| v.abs()).collect();
    let e = formation_scale(
        &mut ctx,
        &mut sum,
        &guard,
        &prep.source,
        &prep.layout,
        &group.structure,
        &abar,
        &shared.members,
        &shared.directional,
        Some(&prep.ledger),
        &w,
        StageRounding::Nearest,
    )
    .unwrap();
    let resolution = resolution_scale(&prep.layout, &e, prep.source.body_count() as usize).ok()?;
    let hat = resolution
        .iter()
        .zip(&prep.extents)
        .map(|(r, &x)| e_hat(*r, x))
        .collect();
    Some(At {
        values: solved.recovered.values.clone(),
        hat,
    })
}

/// |q_P − q_2P| ≤ 2^8·2^-P·ê on every force and moment row, exactly; returns
/// the largest |q_P − q_2P|/(2^-P·ê).
fn headroom<const A: usize, const B: usize>(
    name: &str,
    p: u32,
    layout: &[QuantityMeta],
    lo: &At<A>,
    hi: &At<B>,
) -> f64
where
    Wide<A>: SupportedWidth,
    Wide<B>: SupportedWidth,
{
    let mut worst = 0.0f64;
    let mut ctx = WideContext::<B>::new(64 * B as u32).unwrap();
    for (index, meta) in layout.iter().enumerate() {
        let k = match meta.kind {
            Kind::Force => 0,
            Kind::Moment => 1,
            _ => continue,
        };
        let hat = lo.hat[meta.body as usize][k];
        if !hat.is_finite() {
            // ê overflows (CEIL-A and CEIL-B, whose E at 2p does not encode):
            // the bound is vacuous.
            continue;
        }
        let e = Wide::<B>::from_f64(hat).unwrap();
        let mut d = ExactWideSum::new();
        d.add_wide(&lo.values[index], false).unwrap();
        d.add_wide(&hi.values[index], true).unwrap();
        d.make_absolute();
        let mut test = d.clone();
        test.add_wide_scaled(&e, true, 1, 8 - i64::from(p)).unwrap();
        assert!(
            test.signum() <= 0,
            "{name} {p}: {:?} beyond 2^8·2^-P·ê",
            layout[index].id
        );
        let dw = d.round(&mut ctx).unwrap();
        worst = worst.max(ratio(&dw, &e.mul_pow2(-i64::from(p)).unwrap()));
    }
    worst
}

#[test]
fn e_headroom_every_state_pair_is_within_2_to_the_8_of_its_resolution() {
    // Excluded (R7 §7): the controls whose states the estimate or the charge
    // rejects (by GEN's schedule: LEVER2, CEIL5A3's LEVER2 operands,
    // SEEDED-COMMON and DIRECTIONAL-SPAN, whose state error is the solve's at a
    // condition of about 2^104), TILT-LEVER (its charge is 1e30 of its
    // allowance) and SEEDED-SOFT, whose seed is an injected error.
    let gen = gen_outcomes();
    let excluded = |n: &str| {
        n.starts_with("TILT-LEVER")
            || n.starts_with("SEEDED")
            || gen[n]
                .1
                .iter()
                .any(|t| t.contains("verification_estimate") || t.contains("charge"))
    };
    let mut pairs = 0;
    let mut worst = (0.0f64, String::new());
    for m in all_models()
        .iter()
        .filter(|m| in_default_lane(&m.name) && !excluded(&m.name))
    {
        let prep = CasePrep::new(m.source()).unwrap();
        let Ok(group) = prepare_group(&prep.source) else {
            continue;
        };
        let s128 = state_at::<4, 4>(128, 192, m, &prep, &group);
        let s256 = state_at::<4, 8>(256, 320, m, &prep, &group);
        let s512 = state_at::<8, 16>(512, 576, m, &prep, &group);
        let s1024 = state_at::<16, 16>(1024, 1024, m, &prep, &group);
        let mut note = |r: f64, p: u32| {
            pairs += 1;
            if r > worst.0 {
                worst = (r, format!("{} {p}", m.name));
            }
        };
        if let (Some(a), Some(b)) = (&s128, &s256) {
            note(headroom(&m.name, 128, &prep.layout, a, b), 128);
        }
        if let (Some(a), Some(b)) = (&s256, &s512) {
            note(headroom(&m.name, 256, &prep.layout, a, b), 256);
        }
        if let (Some(a), Some(b)) = (&s512, &s1024) {
            note(headroom(&m.name, 512, &prep.layout, a, b), 512);
        }
    }
    println!(
        "E-HEADROOM: {pairs} pairs; worst |q_P − q_2P|/(2^-P·ê) {:e} ({})",
        worst.0, worst.1
    );
    assert!(pairs >= 300, "{pairs}");
}

// ---------------------------------------------------------------- SD-G5: decide's boundaries

fn force_row() -> Vec<QuantityMeta> {
    vec![QuantityMeta {
        id: QuantityId::EndAction {
            member: 1,
            end: End::I,
            component: Component::Ux,
        },
        kind: Kind::Force,
        body: 0,
        input_derived: false,
    }]
}

fn translation_row() -> Vec<QuantityMeta> {
    vec![QuantityMeta {
        id: QuantityId::Displacement(Dof {
            node: 1,
            component: Component::Ux,
        }),
        kind: Kind::Translation,
        body: 0,
        input_derived: false,
    }]
}

/// A one-row report at P: E_fo = E_mo = ê (a body of zero extent), and the
/// row's Ŵ, C and W⁺, with θ on one block.
fn report<const L: usize>(
    precision: u32,
    e: f64,
    w: Option<Wide<L>>,
    charge: Option<Wide<L>>,
    w_plus: Option<Wide<L>>,
    theta: Wide<L>,
) -> VerificationReport<L>
where
    Wide<L>: SupportedWidth,
{
    VerificationReport {
        precision,
        q_w: super::super::verify::q_w_of(precision),
        resolution: vec![[e, e]],
        e_rows: vec![None],
        w: vec![w],
        a_s: vec![None],
        charge: vec![charge],
        w_plus: vec![w_plus],
        r_hat: Vec::new(),
        delta: Vec::new(),
        blocks: Vec::new(),
        norms: Vec::new(),
        theta: vec![Some(theta)],
        bodies: Vec::new(),
        g_max: 0,
        g_violation: None,
        uc_missing: None,
        shift_factorizations: 0,
    }
}

/// x plus one unit in the last place at `bits` bits (x > 0).
fn ulp_above<const L: usize>(x: &Wide<L>, bits: i64) -> Wide<L>
where
    Wide<L>: SupportedWidth,
{
    let mut ctx = WideContext::<L>::new(64 * L as u32).unwrap();
    let ulp = Wide::<L>::ONE.mul_pow2(x.exponent() - bits + 1).unwrap();
    ctx.add(x, &ulp).unwrap()
}

fn verdict<const L: usize>(
    layout: &[QuantityMeta],
    candidate: &Wide<L>,
    verification: &Wide<L>,
    r: &VerificationReport<L>,
) -> Option<Rejection>
where
    Wide<L>: SupportedWidth,
{
    let d = decide(
        layout,
        &[0.0],
        std::slice::from_ref(candidate),
        std::slice::from_ref(verification),
        r,
        StageGuard::unlimited(),
    );
    assert!(d.result.is_ok(), "{:?}", d.result);
    d.rejection
}

#[test]
fn sd_g5_decides_each_test_exactly_at_its_boundary_in_r7s_order() {
    let layout = force_row();
    let one = Wide::<4>::ONE;
    let zero = Wide::<4>::ZERO;
    let half = one.mul_pow2(-1).unwrap();
    let e = 2f64.powi(-4);
    let big_p = 256i64;
    let mut ctx = WideContext::<4>::new(256).unwrap();
    // (a), force kind: |Δ| = ε·M − V with M = 1 and V = ê·2^(8−P) = 2^-252.
    let v = Wide::<4>::from_f64(e).unwrap().mul_pow2(8 - big_p).unwrap();
    let delta = ctx.sub(&one.mul_pow2(-64).unwrap(), &v).unwrap();
    let candidate = ctx.add(&one, &delta).unwrap();
    let clean = report(256, e, Some(zero), Some(zero), None, zero);
    assert_eq!(verdict(&layout, &candidate, &one, &clean), None);
    let above = ulp_above(&candidate, big_p);
    assert_eq!(
        verdict(&layout, &above, &one, &clean),
        Some(Rejection::StopRule { index: 0 })
    );
    // (b): Ŵ = V/4 = ê·2^(6−P), and one ulp above.
    let quarter = Wide::<4>::from_f64(e).unwrap().mul_pow2(6 - big_p).unwrap();
    let r = report(256, e, Some(quarter), Some(zero), None, zero);
    assert_eq!(verdict(&layout, &one, &one, &r), None);
    let r = report(
        256,
        e,
        Some(ulp_above(&quarter, big_p)),
        Some(zero),
        None,
        zero,
    );
    assert_eq!(
        verdict(&layout, &one, &one, &r),
        Some(Rejection::VerificationEstimate { index: 0 })
    );
    // (c): θ = 1/2, and one ulp above; `uc` and g before the charge.
    let r = report(256, e, Some(zero), Some(zero), None, half);
    assert_eq!(verdict(&layout, &one, &one, &r), None);
    let r = report(
        256,
        e,
        Some(zero),
        Some(zero),
        None,
        ulp_above(&half, big_p),
    );
    assert_eq!(
        verdict(&layout, &one, &one, &r),
        Some(Rejection::Theta { block: 0 })
    );
    let sixty = Wide::<4>::from_f64(60.0 * e)
        .unwrap()
        .mul_pow2(-big_p)
        .unwrap();
    let over = ulp_above(&sixty, big_p);
    let mut r = report(256, e, Some(zero), Some(over), None, zero);
    r.uc_missing = Some(0);
    assert_eq!(
        verdict(&layout, &one, &one, &r),
        Some(Rejection::Uc { block: 0 })
    );
    r.uc_missing = None;
    r.g_violation = Some(7);
    assert_eq!(
        verdict(&layout, &one, &one, &r),
        Some(Rejection::GValidity { member: 7 })
    );
    // (d) at P = 256: C = 60·ê·2^-P, and one ulp above.
    let r = report(256, e, Some(zero), Some(sixty), None, zero);
    assert_eq!(verdict(&layout, &one, &one, &r), None);
    let r = report(256, e, Some(zero), Some(over), None, zero);
    assert_eq!(
        verdict(&layout, &one, &one, &r),
        Some(Rejection::Charge { index: 0 })
    );
    // An estimate failure is reported before a charge failure (R7's order).
    let r = report(
        256,
        e,
        Some(ulp_above(&quarter, big_p)),
        Some(over),
        None,
        zero,
    );
    assert_eq!(
        verdict(&layout, &one, &one, &r),
        Some(Rejection::VerificationEstimate { index: 0 })
    );
}

#[test]
fn sd_g5_charge_boundaries_at_p_256_and_p_512_and_the_translation_row() {
    // (d) for the candidate p = 256 (P = 512: 60·ê·2^-P) and p = 512
    // (P = 1024: 2^-86·M_q, M_q = 1 here, the floor Φ below it).
    let layout = force_row();
    let e = 2f64.powi(-4);
    let one8 = Wide::<8>::ONE;
    let zero8 = Wide::<8>::ZERO;
    let c512 = Wide::<8>::from_f64(60.0 * e)
        .unwrap()
        .mul_pow2(-512)
        .unwrap();
    let r = report(512, e, Some(zero8), Some(c512), None, zero8);
    assert_eq!(verdict(&layout, &one8, &one8, &r), None);
    let r = report(
        512,
        e,
        Some(zero8),
        Some(ulp_above(&c512, 512)),
        None,
        zero8,
    );
    assert_eq!(
        verdict(&layout, &one8, &one8, &r),
        Some(Rejection::Charge { index: 0 })
    );
    let one16 = Wide::<16>::ONE;
    let zero16 = Wide::<16>::ZERO;
    let c1024 = one16.mul_pow2(-86).unwrap();
    let r = report(1024, e, Some(zero16), Some(c1024), None, zero16);
    assert_eq!(verdict(&layout, &one16, &one16, &r), None);
    let r = report(
        1024,
        e,
        Some(zero16),
        Some(ulp_above(&c1024, 1024)),
        None,
        zero16,
    );
    assert_eq!(
        verdict(&layout, &one16, &one16, &r),
        Some(Rejection::Charge { index: 0 })
    );
    // A translation row at |Δ| + W⁺ = ε·M (accepted), and one 2p-ulp above.
    let layout = translation_row();
    let one = Wide::<4>::ONE;
    let zero = Wide::<4>::ZERO;
    let mut ctx = WideContext::<4>::new(256).unwrap();
    let w_plus = one.mul_pow2(-200).unwrap();
    let delta = ctx.sub(&one.mul_pow2(-64).unwrap(), &w_plus).unwrap();
    let candidate = ctx.add(&one, &delta).unwrap();
    let r = report(256, e, None, None, Some(w_plus), zero);
    assert_eq!(verdict(&layout, &candidate, &one, &r), None);
    assert_eq!(
        verdict(&layout, &ulp_above(&candidate, 256), &one, &r),
        Some(Rejection::StopRule { index: 0 })
    );
    // Without W⁺ (a block without B: rule (a) uses 0) the same candidate passes.
    let r = report(256, e, None, None, None, zero);
    assert_eq!(
        verdict(&layout, &ulp_above(&candidate, 256), &one, &r),
        None
    );
}

fn ratio_sum(num: f64, den_terms: &[f64]) -> GateRatio {
    let mut n = ExactWideSum::new();
    n.add_binary64(num, false).unwrap();
    let mut d = ExactWideSum::new();
    for &t in den_terms {
        d.add_binary64(t.abs(), t < 0.0).unwrap();
    }
    (n, d)
}

#[test]
fn sd_g5_the_gates_bounded_test_at_its_boundary_and_the_best_state() {
    // The row test |r|·(2^p − m) ≤ 64·m·d^b at p = 128, m = 4: with
    // d^b = 2^128 − 4, r = 256 is on the boundary (passes), 257 fails.
    let mut sum = ExactWideSum::new();
    let d = ratio_sum(0.0, &[2f64.powi(128), -4.0]).1;
    for (r, passes) in [(256.0, true), (-256.0, true), (257.0, false)] {
        let mut rs = ExactWideSum::new();
        rs.add_binary64(r, false).unwrap();
        let (got, _) = bounded_row(rs, &d, 4, 128, &mut sum).unwrap();
        assert_eq!(got, passes, "{r}");
    }
    // The best state is the smallest worst ratio, compared exactly, the
    // earliest on a tie, and not necessarily the last (M16's vector): state 1
    // is not eligible, states 2 and 3 tie at 2, state 4 is worse.
    let worst = vec![
        Some(Some(ratio_sum(3.0, &[1.0]))),
        None,
        Some(Some(ratio_sum(2.0, &[1.0]))),
        Some(Some(ratio_sum(4.0, &[2.0]))),
        Some(Some(ratio_sum(5.0, &[1.0]))),
    ];
    assert_eq!(best_gate_state(&worst).unwrap(), Some(2));
    // Ratios that differ below binary64 resolution are still ordered exactly.
    let worst = vec![
        Some(Some(ratio_sum(1.0, &[1.0, 2f64.powi(-200)]))),
        Some(Some(ratio_sum(1.0, &[1.0, 2f64.powi(-201)]))),
    ];
    assert_eq!(best_gate_state(&worst).unwrap(), Some(0));
    // A state whose residual vanishes is best; no eligible state gives None.
    let worst = vec![Some(Some(ratio_sum(1.0, &[1.0]))), Some(None)];
    assert_eq!(best_gate_state(&worst).unwrap(), Some(1));
    assert_eq!(best_gate_state(&[None, None]).unwrap(), None);
}

/// A cantilever along x whose y reference leans 2^-j off the chord: its y_c
/// is (0, 2^-j, 0) exactly, so its g exponent is j + 1 (R7 §4.1.6.2 item 2).
fn leaning_cantilever(j: i32) -> PrimitiveSource {
    let mut parts = SourceParts::default();
    parts.nodes = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    parts.members.push(StraightMember {
        id: 1,
        node_i: 0,
        node_j: 1,
        elastic_modulus: 2.0e11,
        shear_modulus: 8.0e10,
        area: 1.0e-2,
        second_moment_y: 1.0e-5,
        second_moment_z: 2.0e-5,
        torsion_constant: 3.0e-5,
        y_reference: [1.0, 2f64.powi(-j), 0.0],
    });
    for c in 0..6 {
        parts.constraints.push(Constraint {
            dof: Dof {
                node: 0,
                component: Component::from_index(c),
            },
            value: 0.0,
        });
    }
    parts.loads.push(NodalLoad {
        dof: Dof {
            node: 1,
            component: Component::Uy,
        },
        value: 1.0e3,
        source_id: "P".to_string(),
    });
    PrimitiveSource::new(parts).unwrap()
}

#[test]
fn sd_g5_the_g_check_passes_at_2_to_the_p_minus_16_and_fails_above() {
    // At P = 256 the limit is g ≤ 2^240: j = 239 gives g = 2^240 (in range),
    // j = 240 gives 2^241 (a violation by member 1).
    for (j, violation) in [(239, None), (240, Some(1u32))] {
        let prep = CasePrep::new(leaning_cantilever(j)).unwrap();
        let group = prepare_group(&prep.source).unwrap();
        let guard = StageGuard::unlimited();
        let shared = build_shared::<4, 8>(256, 320, &prep.source, &group, guard)
            .result
            .unwrap();
        assert_eq!(shared.members[0].g_exp, (j + 1) as u32);
        let state = solve_case_at::<4, 8>(&shared, &prep, &group, guard)
            .result
            .unwrap();
        let vs = build_verify_shared::<4, 8, 8>(&shared, &prep.source, &group, guard)
            .result
            .unwrap();
        let r = verify_state::<4, 8, 8>(&shared, &vs, &prep, &group, &state, guard)
            .result
            .unwrap();
        assert_eq!((r.g_max, r.g_violation), ((j + 1) as u32, violation), "{j}");
    }
}

#[test]
fn sd_g5_lever2s_estimate_residual_is_nonzero_where_the_assembled_one_vanishes() {
    // LEVER2-k90-s40 at 256: the gate's residual (assembled K at q) is 0 on
    // every row, while R7 item 1's contribution-level residual at q_W is not,
    // and its Ŵ is what rejects the 256 candidate.
    let m = all_models()
        .into_iter()
        .find(|m| m.name == "LEVER2-k90-s40")
        .unwrap();
    let prep = CasePrep::new(m.source()).unwrap();
    let group = prepare_group(&prep.source).unwrap();
    let guard = StageGuard::unlimited();
    let shared = build_shared::<4, 8>(256, 320, &prep.source, &group, guard)
        .result
        .unwrap();
    let state = solve_case_at::<4, 8>(&shared, &prep, &group, guard)
        .result
        .unwrap();
    assert_eq!(state.residual_worst, 0.0);
    assert_eq!(state.gate, GateTest::Coalesced);
    let vs = build_verify_shared::<4, 8, 8>(&shared, &prep.source, &group, guard)
        .result
        .unwrap();
    let r = verify_state::<4, 8, 8>(&shared, &vs, &prep, &group, &state, guard)
        .result
        .unwrap();
    assert!(r.r_hat.iter().any(|v| !v.is_zero()));
    assert!(r.w.iter().flatten().any(|v| !v.is_zero()));
}
