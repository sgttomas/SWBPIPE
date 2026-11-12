//! The W1 coverage repair census (DEL-04-08 Design `w1-coverage-repair.md` §3) and
//! W1-E's product-side confirmation census (`w1e-sharper-exact-coverage.md` §6.2),
//! through the actual producer: the observed ordinary run, then `retained_w1`
//! (preparation, native solve, freeze, serialization, precommit reader).
//!
//! Compiled only under the opt-in `census-probe` feature (no dev-dependency
//! enables it); every target is `#[ignore]` and re-runnable on demand:
//!
//! ```text
//! W1_CENSUS_OUT=<dir> cargo test --offline --features census-probe --lib \
//!     w1_coverage_census -- --ignored --test-threads=<n>
//! ```
//!
//! Each target writes one JSON line per input and solver mode under
//! `W1_CENSUS_OUT`, with every run's case outcomes, precommit reader outcomes and
//! the kernel probe's search and gate records. The switches are set only around
//! the retained work, after the ordinary run, so the ordinary bytes are today's.
//! Nothing here writes under `validation/references` or the fixtures. Every model
//! is invented. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use super::*;
#[path = "w1_census_driver.rs"]
mod driver;
use driver::*;
use open_pipe_stress_frame_kernel::structural::retained_api::census_probe as probe;

#[path = "../../reporting/result_export/tests/support/retained_precision_corpus.rs"]
mod storage;
#[path = "../tests/common/w1_member_load_inputs.rs"]
mod w1_member_load_inputs;

/// The census of one input in one mode: today's run and the corner-1 run (corner 2,
/// the receipt's final scales, is the production path since it landed); then, where
/// the corner-1 run has a failing verdict, the lane counterfactuals (b) and (c) with
/// corner 1 on. With `checks`, also the null lane override, which must reproduce
/// today's candidate, and the comparison of today's candidate with a committed one.
fn census(label: &str, raw: &Value, mode: PreviewSolverMode, hook: &dyn Fn(), checks: bool, expected: Option<&Value>) -> Value {
    let sw = |corner1, same_section, same_readout| probe::Switches { corner1, same_section, same_readout, lane_null: false, record: true };
    let started = std::time::Instant::now();
    let (today, today_candidate, _) = run_once(raw, mode, sw(false, false, false), hook);
    let (corner1, _, _) = run_once(raw, mode, sw(true, false, false), hook);
    let failing = corner1["records"].as_array().unwrap().iter().any(|r| r["kind"] == "gate_row" && r["passed"] == false);
    let mut out = json!({"label": label, "mode": mode.as_str(), "today": today, "corner1": corner1});
    if failing {
        out["same_section"] = run_once(raw, mode, sw(true, true, false), hook).0;
        out["same_readout"] = run_once(raw, mode, sw(true, false, true), hook).0;
    }
    if checks {
        let null = probe::Switches { lane_null: true, ..sw(false, false, false) };
        let (_, null_candidate, _) = run_once(raw, mode, null, hook);
        out["null_override_reproduces_today"] = json!(null_candidate == today_candidate);
        // Today's run without records: the probe's recording must not move a byte.
        let (_, quiet_candidate, _) = run_once(raw, mode, probe::Switches::default(), hook);
        out["records_move_nothing"] = json!(quiet_candidate == today_candidate);
        if let (Some(e), Some(c)) = (expected, today_candidate.as_ref()) {
            out["today_equals_committed"] = json!(c == e);
        }
    }
    out["seconds"] = json!(started.elapsed().as_secs_f64());
    out
}

/// One census input: its label, request, modes and (where committed) the
/// candidate the producer committed for each mode.
struct Input { label: String, raw: Value, modes: Vec<PreviewSolverMode>, expected: Vec<Option<Value>> }
fn input(label: impl Into<String>, raw: Value, modes: Vec<PreviewSolverMode>) -> Input {
    let n = modes.len();
    Input { label: label.into(), raw, modes, expected: vec![None; n] }
}

fn run_target(target: &str, inputs: Vec<Input>, checks: bool, hook: &dyn Fn(&str)) {
    let mut n = 0;
    for i in inputs.into_iter().take(limit()) {
        for (mode, expected) in i.modes.iter().zip(&i.expected) {
            let line = census(&i.label, &i.raw, *mode, &|| hook(&i.label), checks, expected.as_ref());
            println!("W1_CENSUS {target} {} {} {:.1}s today={} corner1={}", i.label, mode.as_str(), line["seconds"].as_f64().unwrap(),
                line["today"]["fallback"], line["corner1"]["fallback"]);
            write_line(target, &line);
            n += 1;
        }
    }
    println!("W1_CENSUS_DONE {target} runs={n}");
}

fn corpus_cases() -> Vec<Value> {
    let corpus = storage::expand(serde_json::from_str(include_str!("../../../fixtures/results/retained_precision_cases.json")).unwrap());
    corpus["cases"].as_array().unwrap().clone()
}
fn mode_of(name: &str) -> PreviewSolverMode {
    MODES.into_iter().find(|m| m.as_str() == name).unwrap_or_else(|| panic!("mode {name}"))
}
/// A committed document `{id, source, invocation}` as an input with its expected candidate.
fn committed(prefix: &str, doc: &Value) -> Input {
    let inv = &doc["invocation"];
    Input { label: format!("{prefix}/{}", doc["id"].as_str().unwrap()), raw: inv["request"].clone(),
        modes: vec![mode_of(inv["solver_mode"].as_str().unwrap())], expected: vec![Some(doc["source"].clone())] }
}
/// The corpus writer's fault hook for `w1_member_values_failure_*`.
fn corpus_hook(label: &str) {
    if label.starts_with("corpus/w1_member_values_failure_") {
        retained_tests_hooks::fault_case(1, crate::retained_receipt::TraceFault::ValuesCompletion);
    }
}

/// C1-a (§3.2): the committed route-L member-load inputs (the group-3 trials and
/// every corpus member-load base). The cap witnesses run as their own targets.
fn c1a_inputs() -> Vec<Input> {
    let mut v = Vec::new();
    for c in controls().into_iter().filter(|c| matches!(c.name.as_str(), "F6/partial" | "T18" | "T19/pre-K-D5" | "T19")) {
        v.push(input(format!("trial/{}", c.name), c.raw.clone(), c.modes.clone()));
    }
    for c in corpus_cases() {
        if c["id"].as_str().unwrap().starts_with("w1_member_") {
            v.push(committed("corpus", &c));
        }
    }
    v
}

#[test]
#[ignore = "W1 coverage census C1-a (run with --features census-probe --ignored)"]
fn w1_coverage_census_c1a_committed_member_loads() {
    run_target("c1a", c1a_inputs(), true, &corpus_hook);
}

// ---------------------------------------------------------------- C1-b: the generated sweep

const SWEEP: &str = "invented_w1_coverage_census_sweep_no_library_data";
/// §3.2 C1-b's sections (m): every 79th pair of §3.1's grid (D from 20 to 1,500 mm in
/// 7 mm steps, even walls 2 to 30 mm with 2t < D, D ascending then t), from the first;
/// the three D/2-power-of-two sections; and §3.1's committed sections. §3.1's last row
/// is the D/2-power-of-two family, already among the three, so 49 entries are kept. They name 48
/// distinct sections: `grid/20x2` and `committed/0.02x0.002` coincide, and both keep their sampling weight.
fn c1b_sections() -> Vec<(String, f64, f64, Option<f64>)> {
    let grid: Vec<(u32, u32)> = (20..=1500).step_by(7).flat_map(|d| (2..=30).step_by(2).filter(move |t| 2 * t < d).map(move |t| (d, t))).collect();
    assert_eq!(grid.len(), 3142);
    let mut v: Vec<(String, f64, f64, Option<f64>)> = grid.iter().step_by(79).map(|&(d, t)| (format!("grid/{d}x{t}"), f64::from(d) / 1000.0, f64::from(t) / 1000.0, None)).collect();
    assert_eq!(v.len(), 40);
    for d in [0.25, 0.5, 1.0] {
        v.push((format!("pow2/{d}"), d, 0.01, None));
    }
    for (name, d, t, mill) in [("0.2x0.01", 0.2, 0.01, None), ("0.168x0.007", 0.168, 0.007, None), ("0.1683x0.00711-0.000889", 0.1683, 0.00711, Some(0.000889)),
        ("0.114x0.006", 0.114, 0.006, None), ("0.12x0.01", 0.12, 0.01, None), ("0.02x0.002", 0.02, 0.002, None)] {
        v.push((format!("committed/{name}"), d, t, mill));
    }
    v
}
fn nodal(id: &str, node: &str, direction: &str, value: f64) -> Value {
    let moment = direction.starts_with('R');
    json!({"id": id, "category": if moment { "concentrated_moment" } else { "concentrated_force" },
        "target": {"type": "node", "node": node}, "direction": direction,
        "magnitude": {"value": value, "unit": if moment { "N*m" } else { "N" }},
        "dimension": if moment { "moment" } else { "force" }, "provenance": SWEEP})
}
fn uniform(id: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": "ss:pipe"},
        "direction": direction, "magnitude": {"value": value, "unit": "N/m"}, "dimension": "force_per_length", "provenance": SWEEP})
}
/// One loaded member `ss:pipe` from `ss:root` at (4, 0, 0) to `ss:root` + `span`, added
/// to n05's sensitive torque case, whose independent soft-spring member makes the
/// ordinary result Sensitive (stage 2 §4.4's controls).
fn loaded_member(span: [f64; 3], d: f64, t: f64, mill: Option<f64>, supports: [&[&str]; 2], loads: Vec<Value>) -> Value {
    let mut raw = n05_request(vec![n05_torques()]);
    let m = &mut raw["model"];
    m["nodes"].as_array_mut().unwrap().extend([
        json!({"id": "ss:root", "position": {"x": 4.0, "y": 0.0, "z": 0.0}, "provenance": SWEEP}),
        json!({"id": "ss:tip", "position": {"x": 4.0 + span[0], "y": span[1], "z": span[2]}, "provenance": SWEEP}),
    ]);
    let mut section = json!({"outside_diameter": {"unit": "m", "value": d}, "wall_thickness": {"unit": "m", "value": t}});
    if let Some(mill) = mill {
        section["mill_tolerance"] = json!({"unit": "m", "value": mill});
    }
    m["pipe_segments"].as_array_mut().unwrap().push(json!({"id": "ss:pipe", "from": "ss:root", "to": "ss:tip", "material": "material",
        "provenance": SWEEP, "section": section, "y_reference": {"x": 0, "y": 1, "z": 0}}));
    for (node, restraints) in ["ss:root", "ss:tip"].into_iter().zip(supports) {
        if !restraints.is_empty() {
            m["supports"].as_array_mut().unwrap().push(json!({"id": format!("{node}:support"), "family": "anchor", "node": node,
                "restraints": restraints, "provenance": SWEEP}));
        }
    }
    m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().extend(loads);
    raw
}
const PIN: &[&str] = &["UX", "UY", "UZ", "RX"];
const FIXED: &[&str] = &["UX", "UY", "UZ", "RX", "RY", "RZ"];
/// The exact annulus A and Z of (D, t), in binary64 (S5's sizing only).
fn annulus(d: f64, t: f64) -> (f64, f64) {
    let di = d - 2.0 * t;
    let a = std::f64::consts::PI * (d * d - di * di) / 4.0;
    let z = std::f64::consts::PI * (d.powi(4) - di.powi(4)) / (32.0 * d);
    (a, z)
}
/// §3.2 C1-b's five shapes on one section: (shape, request). UDL 3.5 N/m, length 2 m.
fn c1b_shapes(d: f64, t: f64, mill: Option<f64>) -> Vec<(&'static str, Value)> {
    let w = 3.5;
    let skew = [2.0 * 3.0 / 13.0, 2.0 * 4.0 / 13.0, 2.0 * 12.0 / 13.0];
    let (a, z) = annulus(d, t - mill.unwrap_or(0.0));
    // S5: N/A = M/Z at midspan, M = wL²/8 = 1.75 N·m, so b ≈ 0.5 there.
    let axial = a * (w * 4.0 / 8.0) / z;
    vec![
        ("S1", loaded_member([2.0, 0.0, 0.0], d, t, mill, [PIN, PIN], vec![uniform("ss:udl", "global_y", w)])),
        ("S2", loaded_member([2.0, 0.0, 0.0], d, t, mill, [FIXED, &[]], vec![uniform("ss:udl", "global_y", w)])),
        ("S3", loaded_member([2.0, 0.0, 0.0], d, t, mill, [FIXED, FIXED], vec![uniform("ss:udl", "global_y", w)])),
        ("S4", loaded_member(skew, d, t, mill, [PIN, PIN], vec![uniform("ss:udl:y", "global_y", w), uniform("ss:udl:z", "global_z", w)])),
        ("S5", loaded_member([2.0, 0.0, 0.0], d, t, mill, [PIN, &["UY", "UZ"]], vec![uniform("ss:udl", "global_y", w), nodal("ss:axial", "ss:tip", "global_x", axial)])),
    ]
}
fn c1b_inputs(shape: &str) -> Vec<Input> {
    c1b_sections().into_iter().map(|(name, d, t, mill)| {
        let raw = c1b_shapes(d, t, mill).into_iter().find(|(s, _)| *s == shape).unwrap().1;
        input(format!("c1b/{shape}/{name}"), raw, MODES.to_vec())
    }).collect()
}
macro_rules! c1b_targets {
    ($($name:ident: $shape:literal;)+) => {$(
        #[test]
        #[ignore = "W1 coverage census C1-b, one shape (run with --features census-probe --ignored)"]
        fn $name() {
            run_target(concat!("c1b_", $shape), c1b_inputs($shape), false, &|_| {});
        }
    )+};
}
c1b_targets! {
    w1_coverage_census_c1b_s1: "S1";
    w1_coverage_census_c1b_s2: "S2";
    w1_coverage_census_c1b_s3: "S3";
    w1_coverage_census_c1b_s4: "S4";
    w1_coverage_census_c1b_s5: "S5";
}

// ---------------------------------------------------------------- C2-a: committed W1 inputs

const FIXTURE_DOCS: [&str; 12] = [
    include_str!("../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json"),
    include_str!("../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json"),
    include_str!("../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json"),
    include_str!("../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json"),
    include_str!("../../../fixtures/results/retained_precision_l0_successor_dense_scrutiny.json"),
    include_str!("../../../fixtures/results/retained_precision_l0_successor_sparse_interactive.json"),
    include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"),
    include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),
    include_str!("../../../fixtures/results/retained_precision_o4_w1e_mixed_successor_sparse_interactive.json"),
    include_str!("../../../fixtures/results/retained_precision_o4_w1e_successor_dense_scrutiny.json"),
    include_str!("../../../fixtures/results/retained_precision_o4_w1e_successor_sparse_interactive.json"),
    include_str!("../../../fixtures/results/retained_precision_o4_w1e_unavailable_successor_sparse_interactive.json"),
];
const W_C2_DOCS: [&str; 2] = [
    include_str!("../../../fixtures/results/retained_precision_w_c2_successor_dense_scrutiny.json"),
    include_str!("../../../fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json"),
];
const CONTROL_REQUESTS: [(&str, &str); 10] = [
    ("w1_controls/multicase-dense_scrutiny", include_str!("../../../fixtures/product_preview/w1_controls/multicase-dense_scrutiny.request.json")),
    ("w1_controls/multicase-sparse_interactive", include_str!("../../../fixtures/product_preview/w1_controls/multicase-sparse_interactive.request.json")),
    ("w1_controls/n05-dense_scrutiny", include_str!("../../../fixtures/product_preview/w1_controls/n05-dense_scrutiny.request.json")),
    ("w1_controls/n05-sparse_interactive", include_str!("../../../fixtures/product_preview/w1_controls/n05-sparse_interactive.request.json")),
    ("w1_controls/n06-dense_scrutiny", include_str!("../../../fixtures/product_preview/w1_controls/n06-dense_scrutiny.request.json")),
    ("w1_controls/n06-sparse_interactive", include_str!("../../../fixtures/product_preview/w1_controls/n06-sparse_interactive.request.json")),
    ("w1_controls/rejected_stress_range/dense_scrutiny", include_str!("../../../fixtures/product_preview/w1_controls/rejected_stress_range/dense_scrutiny.request.json")),
    ("w1_controls/rejected_stress_range/sparse_interactive", include_str!("../../../fixtures/product_preview/w1_controls/rejected_stress_range/sparse_interactive.request.json")),
    ("w1_controls/ui/multicase-dense_scrutiny", include_str!("../../../fixtures/product_preview/w1_controls/ui/multicase-dense_scrutiny.request.json")),
    ("w1_controls/ui/multicase-sparse_interactive", include_str!("../../../fixtures/product_preview/w1_controls/ui/multicase-sparse_interactive.request.json")),
];
/// A control request file: a bare request, or `{request, solver_mode}`.
fn control_request(label: &str, text: &str) -> Input {
    let v: Value = serde_json::from_str(text).unwrap();
    let mode = if label.ends_with("dense_scrutiny") { MODES[1] } else { MODES[0] };
    let raw = if v.get("request").is_some() { v["request"].clone() } else { v };
    input(label, raw, vec![mode])
}
/// C2-a (§3.2): every corpus base, the successor fixtures, the W1 controls, the 54
/// B4/D1 trials (with N06 and CX-F among the controls) and the route-E successors.
/// C1-a is its own target, measured with the corner-1 switch as well.
fn c2a_inputs(group: &str) -> Vec<Input> {
    match group {
        "corpus" => corpus_cases().iter().map(|c| committed("corpus", c)).collect(),
        "fixtures" => FIXTURE_DOCS.iter().chain(&W_C2_DOCS).map(|t| committed("fixture", &serde_json::from_str(t).unwrap())).collect(),
        "controls" => {
            let mut v: Vec<Input> = CONTROL_REQUESTS.iter().map(|(l, t)| control_request(l, t)).collect();
            let controls = controls();
            for (name, mode_name, _, _) in FORMERLY_SELECTED {
                let c = controls.iter().find(|c| c.name == name).unwrap();
                v.push(input(format!("b4/{name}"), c.raw.clone(), vec![mode_of(mode_name)]));
            }
            for c in controls.iter().filter(|c| c.name == "CX-F" || c.name.starts_with("committed/n06")) {
                if !FORMERLY_SELECTED.iter().any(|f| f.0 == c.name) {
                    v.push(input(format!("control/{}", c.name), c.raw.clone(), c.modes.clone()));
                }
            }
            v
        }
        other => panic!("group {other}"),
    }
}
macro_rules! c2a_targets {
    ($($name:ident: $group:literal;)+) => {$(
        #[test]
        #[ignore = "W1 coverage census C2-a, one group (run with --features census-probe --ignored)"]
        fn $name() {
            run_target(concat!("c2a_", $group), c2a_inputs($group), true, &corpus_hook);
        }
    )+};
}
c2a_targets! {
    w1_coverage_census_c2a_corpus: "corpus";
    w1_coverage_census_c2a_fixtures: "fixtures";
    w1_coverage_census_c2a_controls: "controls";
}

// ---------------------------------------------------------------- C2-b: constructed halfways

/// The stage-2 point-hull section (D = 0.25 m) and its Ẑ, a 53-bit significand.
const ZC_BITS: u64 = 0x3f3c82e3dc7ef90c;
const ZC_WALL_BITS: u64 = 0x3f847ae147ad8ea5;
/// x = m·2^e exactly (m a signed integer).
fn split(x: f64) -> (i128, i32) {
    let bits = x.to_bits();
    let e = ((bits >> 52) & 0x7ff) as i32;
    let f = (bits & ((1u64 << 52) - 1)) as i128;
    let m = if e == 0 { f } else { f | (1i128 << 52) };
    let m = if x < 0.0 { -m } else { m };
    (m, if e == 0 { -1074 } else { e - 1075 })
}
/// Exact dyadic sums Σ c_k·x_k with c_k = n_k/2^s_k, as N·2^E.
fn dyadic(terms: &[(i128, u32, f64)]) -> (i128, i32) {
    let nonzero: Vec<(i128, u32, f64)> = terms.iter().copied().filter(|t| t.0 != 0 && t.2 != 0.0).collect();
    if nonzero.is_empty() { return (0, 0); }
    let terms = &nonzero[..];
    let e = terms.iter().map(|(_, s, x)| split(*x).1 - *s as i32).min().unwrap();
    let n = terms.iter().map(|(c, s, x)| { let (m, ex) = split(*x); c * m << (ex - *s as i32 - e) }).sum();
    (n, e)
}
/// Whether N·2^E is a binary64 halfway (a normal-range tie between neighbours).
fn halfway((n, _): (i128, i32)) -> bool {
    let a = n.unsigned_abs();
    let bits = 128 - a.leading_zeros();
    if bits <= 53 { return false; }
    let drop = bits - 53;
    a & ((1u128 << drop) - 1) == 1u128 << (drop - 1)
}
fn abs_max(a: (i128, i32), b: (i128, i32)) -> (i128, i32) {
    if a.0 == 0 { return (b.0.abs(), b.1); }
    if b.0 == 0 { return (a.0.abs(), a.1); }
    let e = a.1.min(b.1);
    let (x, y) = (a.0.abs() << (a.1 - e), b.0.abs() << (b.1 - e));
    (x.max(y), e)
}
/// §3.2 C2-b's constructed halfways on the point-hull section, length 2, each verified
/// exactly: the coupled body force or moment maximum of the loaded member's body is a
/// binary64 halfway. Simply supported (pins) with a UDL and end moments: reactions
/// |w ± (M_r + M_t)/2|, end moments |M_r|, |M_t|. Cantilever (fixed root) with a UDL,
/// tip force P and tip moment M: forces |2w + P|, |P|; moments |2w + 2P + M|, |M|.
/// The extent is 2, so fo = max(F*, M*/2) and mo = max(M*, 2F*).
fn c2b_halfways() -> Vec<(String, Value)> {
    let zc = f64::from_bits(ZC_BITS);
    let section = |raw: &mut Value| {
        let pipes = raw["model"]["pipe_segments"].as_array_mut().unwrap();
        let p = pipes.iter_mut().find(|p| p["id"] == "ss:pipe").unwrap();
        p["section"]["outside_diameter"]["value"] = json!(0.25);
        p["section"]["wall_thickness"]["value"] = json!(f64::from_bits(ZC_WALL_BITS));
    };
    let (mut ss, mut cant) = (Vec::new(), Vec::new());
    for a in 1..=8i32 {
        for b in -6..=6i32 {
            for c in -6..=6i32 {
                let (w, x, y) = (f64::from(a) * zc, f64::from(b) * zc, f64::from(c) * zc);
                if ss.len() < 16 && b != 0 && c != 0 {
                    let r1 = dyadic(&[(2, 1, w), (1, 1, x), (1, 1, y)]);
                    let r2 = dyadic(&[(2, 1, w), (-1, 1, x), (-1, 1, y)]);
                    let f = abs_max(r1, r2);
                    let m = abs_max(dyadic(&[(1, 0, x)]), dyadic(&[(1, 0, y)]));
                    let fo = abs_max(f, (m.0, m.1 - 1));
                    let mo = abs_max(m, (f.0, f.1 + 1));
                    if halfway(fo) || halfway(mo) {
                        let mut raw = loaded_member([2.0, 0.0, 0.0], 0.25, 0.01, None, [PIN, PIN],
                            vec![uniform("ss:udl", "global_y", w), nodal("ss:root:moment", "ss:root", "RZ", x), nodal("ss:tip:moment", "ss:tip", "RZ", y)]);
                        section(&mut raw);
                        ss.push((format!("c2b/halfway/ss/{a}/{b}/{c}"), raw));
                    }
                }
                if cant.len() < 16 && b != 0 {
                    let f = abs_max(dyadic(&[(2, 0, w), (1, 0, x)]), dyadic(&[(1, 0, x)]));
                    let m = abs_max(dyadic(&[(2, 0, w), (2, 0, x), (1, 0, y)]), dyadic(&[(1, 0, y)]));
                    let fo = abs_max(f, (m.0, m.1 - 1));
                    let mo = abs_max(m, (f.0, f.1 + 1));
                    if halfway(fo) || halfway(mo) {
                        let mut loads = vec![uniform("ss:udl", "global_y", w), nodal("ss:tip:force", "ss:tip", "global_y", x)];
                        if c != 0 { loads.push(nodal("ss:tip:moment", "ss:tip", "RZ", y)); }
                        let mut raw = loaded_member([2.0, 0.0, 0.0], 0.25, 0.01, None, [FIXED, &[]], loads);
                        section(&mut raw);
                        cant.push((format!("c2b/halfway/cant/{a}/{b}/{c}"), raw));
                    }
                }
            }
        }
    }
    ss.into_iter().chain(cant).collect()
}
/// §3.2 C2-b's 32 decimal controls: both shapes, lengths {1.5, 2.5, 3.7, 6.1} m and
/// forces {1.1, 2.3, 4.7, 9.9} kN on the committed 0.168 m, t = 0.007 m section
/// (cantilever tip force F; simply supported UDL of total F).
fn c2b_decimals() -> Vec<(String, Value)> {
    let mut v = Vec::new();
    for l in [1.5, 2.5, 3.7, 6.1] {
        for f in [1100.0, 2300.0, 4700.0, 9900.0] {
            v.push((format!("c2b/decimal/cant/{l}/{f}"), loaded_member([l, 0.0, 0.0], 0.168, 0.007, None, [FIXED, &[]], vec![nodal("ss:tip:force", "ss:tip", "global_y", f)])));
            v.push((format!("c2b/decimal/ss/{l}/{f}"), loaded_member([l, 0.0, 0.0], 0.168, 0.007, None, [PIN, PIN], vec![uniform("ss:udl", "global_y", f / l)])));
        }
    }
    v
}
fn c2b_inputs(group: &str) -> Vec<Input> {
    let list = match group {
        "halfway" => {
            let h = c2b_halfways();
            assert!(h.len() >= 24, "fewer than 24 confirmed halfways ({}); extend the rule", h.len());
            h
        }
        "decimal" => c2b_decimals(),
        "controls" => vec![("c2b/control/73_24_pa".into(), super::w1_domain_stage1_tests::w1_interior_publication_request(true, -2.0))],
        other => panic!("group {other}"),
    };
    list.into_iter().map(|(l, r)| input(l, r, MODES.to_vec())).collect()
}
macro_rules! c2b_targets {
    ($($name:ident: $group:literal;)+) => {$(
        #[test]
        #[ignore = "W1 coverage census C2-b, one group (run with --features census-probe --ignored)"]
        fn $name() {
            run_target(concat!("c2b_", $group), c2b_inputs($group), false, &|_| {});
        }
    )+};
}
c2b_targets! {
    w1_coverage_census_c2b_halfway: "halfway";
    w1_coverage_census_c2b_decimal: "decimal";
    w1_coverage_census_c2b_controls: "controls";
}

/// §3.2's switched-path control (revision 8): under (b) the annular lane takes the
/// admitted hull, so its loaded maximum must be driven as the admitted lane's is.
/// On the stage-2 D = 0.2 m control both lanes' searches are driven and converge.
#[test]
#[ignore = "W1 coverage census switched-path control (run with --features census-probe --ignored)"]
fn w1_coverage_census_switched_path_control() {
    let raw = c1b_shapes(0.2, 0.01, None).into_iter().find(|(s, _)| *s == "S1").unwrap().1;
    for mode in MODES {
        for (same_section, same_readout) in [(true, false), (false, true)] {
            let sw = probe::Switches { corner1: true, same_section, same_readout, lane_null: false, record: true };
            let (run, _, _) = run_once(&raw, mode, sw, &|| {});
            let searches: Vec<&Value> = run["records"].as_array().unwrap().iter()
                .filter(|r| r["kind"] == "search" && r["phase"] != "counterfactual").collect();
            assert!(searches.len() >= 4, "{mode:?}: both lanes searched at projection and gate: {searches:?}");
            assert!(searches.iter().all(|r| r["driven"] == true && r["outcome"] == "converged"), "{mode:?} ({same_section},{same_readout}): {searches:?}");
        }
    }
}

/// C1-a's cap inputs (§3.2): the cap-law input and the two 4 MiB member-cap witnesses'
/// inputs (`witness_w1_member_load_three_case_three_basis_128` and `..._dyadic`), on
/// one large-stack thread (the census measures outcomes, not stack; the witnesses
/// themselves stay the stack evidence).
#[test]
#[ignore = "W1 coverage census C1-a cap inputs (run with --features census-probe --ignored)"]
fn w1_coverage_census_c1a_cap_inputs() {
    let inputs = vec![
        input("cap/law", w1_member_load_inputs::three_basis_member_cases(crate::retained_memory::census_cap_law_input()), MODES.to_vec()),
        input("cap/witness_128", w1_member_load_inputs::narrow_three_basis_member_cases(), MODES.to_vec()),
        input("cap/witness_128_dyadic", w1_member_load_inputs::narrow_dyadic_three_basis_member_cases(), MODES.to_vec()),
    ];
    std::thread::Builder::new().stack_size(512 << 20).spawn(move || run_target("c1a_cap", inputs, true, &|_| {}))
        .unwrap().join().unwrap();
}

// ---------------------------------------------------------------- §5: the annular lane is unchanged

/// The annular lane's loaded-maximum searches of one input in one mode, from a run with
/// `switches`: (phase, row, member, driven, outcome, splits, max_live, max_depth, the
/// exact enclosure, the search's own counted work). Counterfactual searches are excluded.
fn annular_searches(raw: &Value, mode: PreviewSolverMode, corner1: bool, hook: &dyn Fn()) -> Vec<Value> {
    let sw = probe::Switches { corner1, record: true, ..Default::default() };
    let (run, _, _) = run_once(raw, mode, sw, hook);
    run["records"].as_array().unwrap().iter()
        .filter(|r| r["kind"] == "search" && r["phase"] != "counterfactual" && r["lane"] == "annular")
        .map(|r| json!([r["phase"], r["row"], r["member"], r["driven"], r["outcome"], r["splits"], r["max_live"], r["max_depth"],
            r["lower_exact"], r["upper_exact"], r["work"]]))
        .collect()
}
/// One group of committed loaded inputs: for each input and mode, every annular-lane search
/// in the corner-1 run is undriven, and wherever today's run reached the annular lane its
/// searches equal corner 1's, record for record: enclosure, nodes, depth and counted work.
/// Returns (searches checked undriven, searches compared with today's).
fn annular_lane_unchanged(inputs: Vec<Input>, hook: &dyn Fn(&str)) -> (usize, usize) {
    let (mut undriven, mut compared) = (0, 0);
    for i in inputs {
        for mode in &i.modes {
            let after = annular_searches(&i.raw, *mode, true, &|| hook(&i.label));
            assert!(after.iter().all(|s| s[3] == false), "{} {mode:?}: an annular search was driven: {after:?}", i.label);
            undriven += after.len();
            let today = annular_searches(&i.raw, *mode, false, &|| hook(&i.label));
            if !today.is_empty() {
                assert_eq!(today, after, "{} {mode:?}: the annular lane's searches moved", i.label);
                compared += today.len();
            }
            println!("W1_ANNULAR {} {} today={} corner1={}", i.label, mode.as_str(), today.len(), after.len());
        }
    }
    (undriven, compared)
}

/// W1 coverage repair §1.2 and §5: the annular lane keeps today's search on its tight z;
/// the drive-and-hull split never applies to it. Its work and enclosures are byte-identical
/// to today's on every committed loaded input: the group-3 trials, every corpus member-load
/// base (`w1_member_*`) and the two 128-term member-cap witnesses' inputs, in both modes.
///
/// Why the census build: the receipts state each proof's work summed over both lanes, so
/// the annular lane's own work is not observable in committed bytes or in the ordinary
/// build, whose corner-1 switch is always on. Under `census-probe` the switch can be off,
/// and the probe records each search's lane, exact enclosure and own counted work. Where
/// today's admitted lane exhausted first (the D = 0.2 m transverse members), today never
/// reached the annular lane; there the check is that the corner-1 run searched it
/// undriven, which is today's code path with the same inputs, since the lanes' solves
/// precede the search and the switch changes nothing else.
///
/// ```text
/// cargo test --offline --features census-probe --lib w1_annular_lane_search_is_unchanged -- --ignored
/// ```
#[test]
#[ignore = "W1 coverage repair §5, annular lane (run with --features census-probe --ignored)"]
fn w1_annular_lane_search_is_unchanged() {
    let mut inputs = c1a_inputs();
    inputs.push(input("cap/witness_128", w1_member_load_inputs::narrow_three_basis_member_cases(), MODES.to_vec()));
    inputs.push(input("cap/witness_128_dyadic", w1_member_load_inputs::narrow_dyadic_three_basis_member_cases(), MODES.to_vec()));
    let (undriven, compared) = std::thread::Builder::new().stack_size(512 << 20)
        .spawn(move || annular_lane_unchanged(inputs, &corpus_hook)).unwrap().join().unwrap();
    println!("W1_ANNULAR_DONE undriven={undriven} compared={compared}");
    assert!(undriven > 0 && compared > 0);
}
