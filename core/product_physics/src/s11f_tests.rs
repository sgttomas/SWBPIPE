//! S11-F product tests (S11_CONTAINMENT revision 5a.2, section 9: F1-F14).
//!
//! Every test that is meant to kill a fold mutation first asserts, inside the
//! test, that the binary64 path differs from the exact one (S11B-6, extended
//! by RV1's lesson to every pin), and only then asserts the outcome. Inputs
//! are invented; no material, component, catalogue or code-rule data is used.
//! Each property is stated where it is authored.
use super::*;
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::load_ledger::ForceTerm;
use serde_json::{json, Value};
use std::collections::BTreeSet;

const INVENTED: &str = "invented_t3_s11f_test_input_not_library_data";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

// ------------------------------------------------------------------ entries

#[derive(Clone, Copy, Debug, PartialEq)]
enum Entry {
    /// `run_linear_static_preview_value_with_mode` (the captured entry).
    Captured,
    /// `run_linear_static_preview_with_mode`, the historical typed entry that
    /// headless `run_preview_in_memory_mode` calls (`runner/headless/src/
    /// lib.rs:804-809`, a one-line delegation).
    Typed,
}

fn run(
    entry: Entry,
    request: &Value,
    mode: PreviewSolverMode,
) -> Result<MechanicsEnvelope, String> {
    match entry {
        Entry::Captured => run_linear_static_preview_value_with_mode(request.clone(), mode),
        Entry::Typed => {
            let typed: LinearStaticPreviewRequest =
                serde_json::from_value(request.clone()).map_err(|e| format!("DTO: {e}"))?;
            Ok(run_linear_static_preview_with_mode(typed, mode))
        }
    }
}

fn solved(entry: Entry, request: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let envelope = run(entry, request, mode).unwrap_or_else(|e| panic!("{entry:?} {mode:?}: {e}"));
    let blocking = envelope
        .diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect::<Vec<_>>();
    assert!(
        blocking.is_empty(),
        "{entry:?} {mode:?} blocked: {blocking:?}"
    );
    envelope
}

fn codes(envelope: &MechanicsEnvelope) -> BTreeSet<String> {
    envelope
        .diagnostics
        .iter()
        .map(|d| d.code.clone())
        .collect()
}

/// The case's M03 integrity code (`..._CHECKS_PASSED` or `..._SENSITIVE`).
fn integrity_code(envelope: &MechanicsEnvelope, case: &str) -> String {
    envelope
        .diagnostics
        .iter()
        .find(|d| d.id == integrity_diagnostic_id(case))
        .map(|d| d.code.clone())
        .unwrap_or_default()
}

fn case_rows<'a>(
    envelope: &'a MechanicsEnvelope,
    case: &str,
) -> impl Iterator<Item = &'a ResultItem> + 'a {
    let case = case.to_string();
    envelope
        .results
        .iter()
        .filter(move |r| r.basis_ref.as_ref().is_some_and(|b| b.ref_id == case))
}

fn row_by_id<'a>(envelope: &'a MechanicsEnvelope, case: &str, id: &str) -> Option<&'a ResultItem> {
    case_rows(envelope, case).find(|r| r.id == id)
}

fn support_row<'a>(
    envelope: &'a MechanicsEnvelope,
    case: &str,
    support: &str,
    component: &str,
) -> Option<&'a ResultItem> {
    case_rows(envelope, case).find(|r| {
        r.kind == "support_reaction_component_v2"
            && r.entity_ref == support
            && r.metadata
                .as_ref()
                .is_some_and(|m| m.component == component)
    })
}

fn element_row<'a>(
    envelope: &'a MechanicsEnvelope,
    case: &str,
    member: &str,
    component: &str,
    location: &str,
) -> Option<&'a ResultItem> {
    case_rows(envelope, case).find(|r| {
        r.kind.starts_with("element_local_")
            && r.entity_ref == member
            && r.metadata
                .as_ref()
                .is_some_and(|m| m.component == component && m.location == location)
    })
}

fn element_value(
    envelope: &MechanicsEnvelope,
    case: &str,
    member: &str,
    component: &str,
    location: &str,
) -> f64 {
    element_row(envelope, case, member, component, location)
        .unwrap_or_else(|| panic!("{member} {component} {location} published"))
        .value
}

/// Every published value of a case, keyed by id and bit pattern, for exact
/// comparisons between two invocations.
fn value_bits(envelope: &MechanicsEnvelope, case: &str) -> BTreeMap<String, u64> {
    case_rows(envelope, case)
        .map(|r| (r.id.clone(), r.value.to_bits()))
        .collect()
}

// --------------------------------------------------------------- arithmetic

fn exact_net(values: &[f64]) -> f64 {
    let mut accumulator = ExactAccumulator::new();
    for &v in values {
        accumulator.add(v).unwrap();
    }
    accumulator.round().unwrap()
}

fn fold(values: &[f64]) -> f64 {
    values.iter().fold(0.0, |s, v| s + v)
}

/// `|obs - exp| <= 1e-9 * max(|exp|, scale)`, evaluated exactly: the exact
/// sign of `tolerance - |obs - exp|` from the kernel's exact accumulator, with
/// `1e-9 * max(|exp|, scale)` itself rounded down (so the check never accepts
/// a breach).
fn within(obs: f64, exp: f64, scale: f64) -> bool {
    let tolerance = {
        let t = 1e-9 * exp.abs().max(scale);
        // Round the tolerance toward zero by one ulp so a pass is exact.
        if t > 0.0 {
            f64::from_bits(t.to_bits() - 1)
        } else {
            0.0
        }
    };
    let mut diff = ExactAccumulator::new();
    diff.add(obs).unwrap();
    diff.add(-exp).unwrap();
    let mut slack = ExactAccumulator::new();
    slack.add(tolerance).unwrap();
    if diff.signum() >= 0 {
        slack.add(-obs).unwrap();
        slack.add(exp).unwrap();
    } else {
        slack.add(obs).unwrap();
        slack.add(-exp).unwrap();
    }
    slack.signum() >= 0
}

/// `lo <= a^2 + b^2 <= hi`, exactly.
fn hypot_squared_within(a: f64, b: f64, lo: f64, hi: f64) -> bool {
    let mut low = ExactAccumulator::new();
    low.add_product(a, a).unwrap();
    low.add_product(b, b).unwrap();
    low.add(-lo).unwrap();
    let mut high = ExactAccumulator::new();
    high.add(hi).unwrap();
    high.add_product(-a, a).unwrap();
    high.add_product(-b, b).unwrap();
    low.signum() >= 0 && high.signum() >= 0
}

// ------------------------------------------------- the case ledger (tests)

/// The case's ledger terms, from the product's own producer sequence
/// (`case_force_ledger`), for a pre-0.4 request: the preconditions fold these
/// terms in push order.
fn case_ledger(request: &Value, case_index: usize) -> (Vec<ForceTerm>, AssembledForce) {
    let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).expect("model builds");
    let load_case = &model.load_cases[case_index];
    let loads = build_load_case_primitive_loads(&model, load_case, &mut diagnostics);
    let application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
    let bends = built
        .curved_bend_elements
        .iter()
        .map(|e| (e.pipe_index, e))
        .collect::<HashMap<_, _>>();
    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), i))
        .collect::<HashMap<_, _>>();
    let material_map = materials
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect::<HashMap<_, _>>();
    let thermal = build_thermal_element_loads(
        &model,
        load_case,
        &material_map,
        &pipe_map,
        &built.sections,
        &mut diagnostics,
    );
    let thrust = build_pressure_thrust_loads(&model, load_case, &pipe_map, &built.sections);
    let ledger = case_force_ledger(
        &model,
        &built,
        &application,
        &bends,
        &thrust,
        &thermal,
        None,
        &load_case.id,
        &mut diagnostics,
    );
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    let terms = ledger.terms().to_vec();
    (
        terms,
        ledger.finish(built.nodes.len() * DOF_PER_NODE).unwrap(),
    )
}

/// DOFs where the binary64 fold of the case's ledger terms, in the
/// producer's push order, differs from the correctly rounded net.
fn fold_differs(terms: &[ForceTerm], force: &AssembledForce) -> Vec<usize> {
    let mut folded = vec![0.0; force.len()];
    for term in terms {
        folded[term.dof] += match term.kind {
            open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(v) => v,
            open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Product(a, b) => a * b,
        };
    }
    (0..force.len())
        .filter(|&dof| folded[dof].to_bits() != force.values()[dof].to_bits())
        .collect()
}

// ------------------------------------------- F1, F11, F12: RF-CANCEL, both entries

fn rf_cancel_data() -> Value {
    serde_json::from_str(include_str!("../tests/fixtures/s11f/rf_cancel_cases.json")).unwrap()
}

fn hex_f64(v: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap())
}

/// Checks every published row of one run against its acceptance interval.
/// Returns (breaches, checked keys).
/// Returns the breaching keys, the checked keys, and each checked key's
/// published binary64 values (value; support; pair end i, end j; hypot y, z).
fn check_rf_rows(
    case: &Value,
    envelope: &MechanicsEnvelope,
) -> (Vec<String>, BTreeSet<String>, BTreeMap<String, Vec<f64>>) {
    let mut breaches = Vec::new();
    let mut checked = BTreeSet::new();
    let mut published = BTreeMap::new();
    for row in case["rows"].as_array().unwrap() {
        let key = row["key"].as_str().unwrap().to_string();
        let check = row["check"].as_str().unwrap();
        if check == "not_published" {
            continue;
        }
        let (lo, hi) = (hex_f64(&row["lo_bits"]), hex_f64(&row["hi_bits"]));
        let inside = |x: f64| lo <= x && x <= hi;
        let ok = match check {
            "value" => {
                let Some(r) = row_by_id(envelope, "case", row["id"].as_str().unwrap()) else {
                    continue;
                };
                assert_eq!(r.unit, row["unit"].as_str().unwrap(), "{key}");
                published.insert(key.clone(), vec![r.value]);
                inside(r.value)
            }
            "support" => {
                let Some(r) = support_row(
                    envelope,
                    "case",
                    row["support"].as_str().unwrap(),
                    row["component"].as_str().unwrap(),
                ) else {
                    continue;
                };
                published.insert(key.clone(), vec![r.value]);
                inside(r.value)
            }
            "pair" => {
                let member = row["member"].as_str().unwrap();
                let component = row["component"].as_str().unwrap();
                let (Some(i), Some(j)) = (
                    element_row(envelope, "case", member, component, "end_i"),
                    element_row(envelope, "case", member, component, "end_j"),
                ) else {
                    continue;
                };
                published.insert(key.clone(), vec![i.value, j.value]);
                inside(-i.value) && inside(j.value)
            }
            "hypot" => {
                let member = row["member"].as_str().unwrap();
                let location = row["location"].as_str().unwrap();
                let (Some(y), Some(z)) = (
                    element_row(envelope, "case", member, "bending_moment_y", location),
                    element_row(envelope, "case", member, "bending_moment_z", location),
                ) else {
                    continue;
                };
                published.insert(key.clone(), vec![y.value, z.value]);
                hypot_squared_within(y.value, z.value, lo, hi)
            }
            other => panic!("unknown check {other}"),
        };
        checked.insert(key.clone());
        if !ok {
            breaches.push(key);
        }
    }
    (breaches, checked, published)
}

/// The 14 formation rows' published binary64 values after S11-F, exactly
/// pinned (ROOT's amended formation-class condition, 2026-09-27): (entry,
/// case, key, mode, values), values as `check_rf_rows` returns them.
const FORMATION_PINS: &[(&str, &str, &str, &str, &[f64])] = &[
    (
        "captured",
        "RF-CANCEL-UDL-W1e8",
        "th.S1.RZ",
        "dense_scrutiny",
        &[2.2754051790793294e-08],
    ),
    (
        "captured",
        "RF-CANCEL-UDL-W1e8",
        "th.S1.RZ",
        "sparse_interactive",
        &[2.2754051790793294e-08],
    ),
    (
        "typed",
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "Mb.M1.j",
        "dense_scrutiny",
        &[0.0, 1.0000036354540498e-08],
    ),
    (
        "typed",
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "Mb.M1.j",
        "sparse_interactive",
        &[0.0, 1.0000022143685783e-08],
    ),
    (
        "typed",
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "Mb.M2.i",
        "dense_scrutiny",
        &[0.0, -9.999993721976352e-09],
    ),
    (
        "typed",
        "RF-CANCEL-F-G1e80-GnG-INPLANE",
        "Mb.M2.i",
        "sparse_interactive",
        &[0.0, -1.0000043459967856e-08],
    ),
    (
        "typed",
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "Mb.M2.i",
        "dense_scrutiny",
        &[0.0, -1.000005767082257e-08],
    ),
    (
        "typed",
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "Mb.M2.i",
        "sparse_interactive",
        &[0.0, -1.0000043459967856e-08],
    ),
    (
        "typed",
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "Mb.M2.j",
        "dense_scrutiny",
        &[0.0, 9.999993721976352e-09],
    ),
    (
        "typed",
        "RF-CANCEL-M-G1e80-GnG-INPLANE",
        "Mb.M2.j",
        "sparse_interactive",
        &[0.0, 9.999993721976352e-09],
    ),
    (
        "typed",
        "RF-CANCEL-UDL-W1e8",
        "th.S1.RZ",
        "dense_scrutiny",
        &[2.2754051790793294e-08],
    ),
    (
        "typed",
        "RF-CANCEL-UDL-W1e8",
        "th.S1.RZ",
        "sparse_interactive",
        &[2.2754051790793294e-08],
    ),
    (
        "typed",
        "RF-CANCEL-UDL-W1e80",
        "th.S1.RZ",
        "dense_scrutiny",
        &[1.2184480799204924e+57],
    ),
    (
        "typed",
        "RF-CANCEL-UDL-W1e80",
        "th.S1.RZ",
        "sparse_interactive",
        &[1.2184480799204922e+57],
    ),
];

/// F1, F11 and F12. Every RF-CANCEL case of R1's frozen references runs
/// through the captured entry and the historical typed entry, in both modes.
/// Every published value meets R1's unchanged predicate with the binding
/// net-governed scale, except the 14 pinned formation rows (ROOT, option
/// (c), `GATE/FORMATION_EXCEPTIONS.json`, owned by S11-G and then F2/F3):
/// so the S11 list `GATE/S11_EXCEPTIONS.json` (221 triples as re-pinned) is
/// empty after S11-F, on both entries and both modes, and a breach outside
/// both lists fails. S-H is not in this slice (ROOT `b6fe1eb75`): the
/// captured entry still refuses |x| >= 2^53 at capture, so the G = 1e80
/// cases (F, M, ORTHO, INPLANE, UDL-W1e80) run through the typed entry and
/// the captured entry is asserted to refuse them.
#[test]
fn f1_f11_f12_rf_cancel_cases_meet_the_binding_predicate_on_both_entries() {
    let data = rf_cancel_data();
    assert_eq!(
        data["inputs"]["references_json_sha256"],
        json!("7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9")
    );
    assert_eq!(
        data["inputs"]["s11_exceptions_json_sha256"],
        json!("1d8979f656c91686be43900664f249f95758c834446cbf27c211671739a427e3")
    );
    assert_eq!(
        data["inputs"]["formation_exceptions_json_sha256"],
        json!("454bbc24bcbe5e1b97272b4e6d1251ff78b0c98c1e9894876c1f260464a43b5c")
    );
    let formation: BTreeSet<(String, String, String, String)> = data["formation_exceptions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r[0].as_str().unwrap().to_string(),
                r[1].as_str().unwrap().to_string(),
                r[2].as_str().unwrap().to_string(),
                r[3].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(formation.len(), 14);
    let mut checked: BTreeMap<(String, String, String), BTreeSet<String>> = BTreeMap::new();
    let mut breaches = BTreeSet::new();
    let mut discriminating_orders = 0;
    let mut formation_published = BTreeMap::new();
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let request = &case["request"];
        // Precondition (S11B-6): in every order whose binary64 fold R1's
        // negative control says discriminates, the fold of the case's actual
        // ledger terms, in the producer's push order, differs from the
        // correctly rounded net. (UDL-W1e80 is the exception: there the
        // formed fixed-end terms themselves differ by 2.6e64, which the fold
        // and the exact sum both keep; it is a formation row.)
        let (terms, force) = case_ledger(request, 0);
        let differs = fold_differs(&terms, &force);
        let order = id.rsplit('-').find(|s| ["GnG", "nGG", "GGn"].contains(s));
        let authored_nc = match order {
            Some(order) => format!("NC-FLOAT-SUM-{order}"),
            None => "NC-FLOAT-SUM-A-node-B".to_string(),
        };
        let nc = case["negative_controls"]
            .as_array()
            .unwrap()
            .iter()
            .find(|nc| nc["id"] == json!(authored_nc))
            .unwrap_or_else(|| panic!("{id}: {authored_nc}"));
        if (nc["discriminates"] == json!(true) || id.contains("UDL")) && id != "RF-CANCEL-UDL-W1e80"
        {
            assert!(
                !differs.is_empty(),
                "{id}: precondition, the ledger fold equals the net"
            );
            discriminating_orders += 1;
        }
        for entry in [Entry::Captured, Entry::Typed] {
            let entry_name = if entry == Entry::Captured {
                "captured"
            } else {
                "typed"
            };
            for mode in MODES {
                if entry == Entry::Captured && case["captured_refused_at_capture"] == json!(true) {
                    let refused = run(entry, request, mode);
                    assert!(
                        refused.is_err(),
                        "{id} {mode:?}: capture refuses |x| >= 2^53 until S-H lands"
                    );
                    continue;
                }
                let envelope = solved(entry, request, mode);
                let (bad, keys, values) = check_rf_rows(case, &envelope);
                for (key, values) in values {
                    let row = (
                        entry_name.to_string(),
                        id.to_string(),
                        key,
                        mode.as_str().to_string(),
                    );
                    if formation.contains(&row) {
                        formation_published.insert(row, values);
                    }
                }
                for key in bad {
                    breaches.insert((
                        entry_name.to_string(),
                        id.to_string(),
                        key,
                        mode.as_str().to_string(),
                    ));
                }
                checked
                    .entry((
                        entry_name.to_string(),
                        id.to_string(),
                        mode.as_str().to_string(),
                    ))
                    .or_default()
                    .extend(keys);
            }
        }
    }
    assert!(discriminating_orders >= 20, "{discriminating_orders}");
    // A breach outside the formation list is a gate failure.
    let outside: Vec<_> = breaches.difference(&formation).collect();
    assert!(
        outside.is_empty(),
        "breaches outside both lists: {outside:?}"
    );
    // The formation list's residual is reported, not repaired here (S11-G).
    let formation_residual: Vec<_> = breaches.intersection(&formation).collect();
    eprintln!(
        "F12 formation-list residual ({} of 14 rows): {formation_residual:?}",
        formation_residual.len()
    );
    // ROOT's amended formation-class condition (2026-09-27): each formation
    // row publishes the correctly rounded net of its represented terms and
    // stays exactly pinned. The pins are the S11-F values measured against
    // base in `IMPLEMENTATION/S11F/_run_records/formation_rows/` (the four
    // UDL-W1e8 rows are 3% worse than base; the other ten are bit-identical
    // or better), so any change to a formation row's published bits fails.
    let pinned: BTreeMap<(String, String, String, String), Vec<f64>> = FORMATION_PINS
        .iter()
        .map(|(entry, case, key, mode, values)| {
            (
                (
                    entry.to_string(),
                    case.to_string(),
                    key.to_string(),
                    mode.to_string(),
                ),
                values.to_vec(),
            )
        })
        .collect();
    assert_eq!(pinned.keys().cloned().collect::<BTreeSet<_>>(), formation);
    for (row, expected) in &pinned {
        let observed = formation_published
            .get(row)
            .unwrap_or_else(|| panic!("formation row {row:?} was not published"));
        assert_eq!(
            observed.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            "formation row {row:?} moved from its pinned bits"
        );
    }
    // F12: every pinned S11 triple is published on its entry in both modes,
    // checked, and inside its interval: the S11 list is empty after S11-F.
    let triples = data["exceptions"].as_array().unwrap();
    assert_eq!(triples.len(), 87 + 134);
    let mut counted = BTreeMap::new();
    for triple in triples {
        let (entry, case, key) = (
            triple[0].as_str().unwrap().to_string(),
            triple[1].as_str().unwrap().to_string(),
            triple[2].as_str().unwrap().to_string(),
        );
        *counted.entry(entry.clone()).or_insert(0) += 1;
        for mode in MODES {
            let slot = (entry.clone(), case.clone(), mode.as_str().to_string());
            assert!(
                checked.get(&slot).is_some_and(|k| k.contains(&key)),
                "S11 exception {entry} {case} {key} {mode:?} was not published and checked"
            );
            assert!(
                !breaches.contains(&(
                    entry.clone(),
                    case.clone(),
                    key.clone(),
                    mode.as_str().to_string()
                )),
                "S11 exception {entry} {case} {key} {mode:?} remains a breach"
            );
        }
    }
    assert_eq!(counted.get("captured"), Some(&87));
    assert_eq!(counted.get("typed"), Some(&134));
}

// ------------------------------------------- F14 and F2: P1's probe A

/// P1's S11-PROBE-A product model (DETECTION/scripts/gen.py.txt
/// `supplementary_cases`): one 2 m cantilever along x, OD 0.2 m, wall
/// 0.01 m, E = 200 GPa, G = 80 GPa, anchored at the root, with three uniform
/// global-y loads (G, 0.3, -G) N/m in that authored order.
fn probe_a(g: f64, order: [f64; 3]) -> Value {
    let loads = order
        .iter()
        .enumerate()
        .map(|(i, &factor)| {
            let value = if factor == 0.0 { 0.3 } else { factor * g };
            json!({"id": format!("udl:{i}"), "category": "distributed_force",
                "target": {"type": "element", "pipe": "pipe"}, "direction": "global_y",
                "magnitude": {"value": value, "unit": "N/m"}, "dimension": "force_per_length",
                "provenance": INVENTED})
        })
        .collect::<Vec<_>>();
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:t3-s11f:probe-a", "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [
            {"id": "root", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": INVENTED},
            {"id": "tip", "position": {"x": 2.0, "y": 0.0, "z": 0.0}, "provenance": INVENTED}],
        "pipe_segments": [{"id": "pipe", "from": "root", "to": "tip", "material": "material",
            "y_reference": {"x": 0, "y": 1, "z": 0},
            "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
            "provenance": INVENTED}],
        "materials": [{"id": "material", "elastic_modulus": {"value": 200e9, "unit": "Pa"},
            "shear_modulus": {"value": 80e9, "unit": "Pa"}, "provenance": INVENTED}],
        "supports": [{"id": "anchor", "node": "root", "family": "anchor",
            "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": INVENTED}],
        "load_cases": [{"id": "case", "label": "probe A", "kind": "primitive_user_load",
            "primitive_loads": loads, "provenance": INVENTED}],
        "combinations": []}, "materials": []})
}

const GNG: [f64; 3] = [1.0, 0.0, -1.0];
const NGG: [f64; 3] = [0.0, 1.0, -1.0];

/// The exact net w = 0.3 N/m (the binary64 value of the authored 0.3), L = 2
/// m, and EI from the product's section: E = 200 GPa, I = pi/64 (OD^4 -
/// ID^4) with OD = 0.2 m, ID = 0.18 m, so EI = 1719500 pi N*m^2.
struct ProbeAExpected {
    tip_uy: f64,
    tip_rz: f64,
    root_uy: f64,
    root_rz: f64,
    moment_i: f64,
    shear_i: f64,
}

fn probe_a_expected() -> ProbeAExpected {
    let w = 0.3_f64;
    let l = 2.0_f64;
    let ei = 1_719_500.0 * std::f64::consts::PI;
    ProbeAExpected {
        tip_uy: w * l.powi(4) / (8.0 * ei),
        tip_rz: w * l.powi(3) / (6.0 * ei),
        root_uy: -w * l,
        root_rz: -w * l * l / 2.0,
        moment_i: w * l * l / 2.0,
        shear_i: w * l,
    }
}

/// Net-governed scale |exp| (P1's `max(|exp|, 0)`), plus the few-ulp formation
/// of the expected value itself, which is far below 1e-9.
fn assert_probe_a(envelope: &MechanicsEnvelope, label: &str) {
    let e = probe_a_expected();
    let value = |id: &str| {
        row_by_id(envelope, "case", id)
            .unwrap_or_else(|| panic!("{label}: {id}"))
            .value
    };
    // Translations are published in mm.
    assert!(
        within(value("result:disp:tip:uy"), e.tip_uy * 1000.0, 0.0),
        "{label}: tip UY"
    );
    assert!(
        within(value("result:disp:tip:rz"), e.tip_rz, 0.0),
        "{label}: tip RZ"
    );
    let my = element_value(envelope, "case", "pipe", "bending_moment_y", "end_i");
    let mz = element_value(envelope, "case", "pipe", "bending_moment_z", "end_i");
    assert!(
        within(my.hypot(mz), e.moment_i, 0.0),
        "{label}: Mb.i {my} {mz}"
    );
    let vy = element_value(envelope, "case", "pipe", "shear_force_y", "end_i");
    let vz = element_value(envelope, "case", "pipe", "shear_force_z", "end_i");
    assert!(
        within(vy.hypot(vz), e.shear_i, 0.0),
        "{label}: Vb.i {vy} {vz}"
    );
    if let Some(r) = support_row(envelope, "case", "anchor", "Fy") {
        assert!(within(r.value, e.root_uy, 0.0), "{label}: R.root.UY");
    }
    if let Some(r) = support_row(envelope, "case", "anchor", "Mz") {
        assert!(within(r.value, e.root_rz, 0.0), "{label}: R.root.RZ");
    }
}

/// F14: P1's S11-PROBE-A cases (G >= 1e7) as product tests, through both
/// entries and both modes, with exact expected nets derived here.
#[test]
fn f14_p1_probe_a_cases_publish_the_exact_net_answer_on_both_entries() {
    for g in [1e7, 1e8] {
        // Precondition: the binary64 fold of (G, 0.3, -G) differs from the
        // correctly rounded net and would breach the net-governed 1e-9.
        let folded = fold(&[g, 0.3, -g]);
        assert_eq!(exact_net(&[g, 0.3, -g]), 0.3);
        assert!(
            !within(folded, 0.3, 0.0),
            "G={g}: the fold {folded} does not breach"
        );
        let request = probe_a(g, GNG);
        let (terms, force) = case_ledger(&request, 0);
        assert!(
            !fold_differs(&terms, &force).is_empty(),
            "G={g}: ledger fold precondition"
        );
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let envelope = solved(entry, &request, mode);
                assert_probe_a(&envelope, &format!("F14 G={g} {entry:?} {mode:?}"));
            }
        }
    }
}

/// The C3-detect verdict on the same model (S11 section 5.3): the ledger's
/// terms folded in push order form the force vector, and `FK`'s audit entry
/// point compares it with the identified terms.
fn c3_detect_flags(request: &Value) -> bool {
    use open_pipe_stress_frame_kernel::structural::{
        solve_structural_dense_with_force_terms, StructuralSystem,
    };
    let (terms, force) = case_ledger(request, 0);
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = parsed.model;
    let mut materials = model.materials.clone();
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).unwrap();
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let stiffness = assemble_case_stiffness(&built, &boundary.springs).unwrap();
    let mut folded = vec![0.0; force.len()];
    for term in &terms {
        if let open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(v) = term.kind {
            folded[term.dof] += v;
        }
    }
    let prescribed = boundary
        .restrained_dofs
        .iter()
        .map(|&d| (d, 0.0))
        .collect::<Vec<_>>();
    let free = (0..folded.len())
        .filter(|d| !boundary.restrained_dofs.contains(d))
        .collect::<Vec<_>>();
    let system = StructuralSystem {
        stiffness: &stiffness,
        force: &folded,
        free_dofs: &free,
        prescribed: &prescribed,
        contributions: None,
        symmetry: None,
    };
    let solution = solve_structural_dense_with_force_terms(&system, &terms).unwrap();
    solution.load_fidelity.is_some()
}

/// F2: probe A through the product at the required kill set G = 1e8 and
/// 1e80, in both authored orders (G, n, -G) and (n, G, -G). End forces,
/// stations and extrema: every published value equals the product's answer
/// for the single net load 0.3 N/m bit for bit (E5, E4/E6, E7 and E12 sum each
/// load's term exactly), and the closed forms hold within 1e-9. The invariant:
/// C3-detect on the same model flags, and the product publishes Passed with
/// no `LOAD_CONTRIBUTION_ABSORBED`. G = 1e80 runs on the typed entry (S-H is
/// not in this slice).
#[test]
fn f2_probe_a_through_the_product_g1e8() {
    f2_probe_a(1e8);
}

#[test]
fn f2_probe_a_through_the_product_g1e80() {
    f2_probe_a(1e80);
}

fn f2_probe_a(g: f64) {
    {
        for order in [GNG, NGG] {
            let request = probe_a(g, order);
            let authored: Vec<f64> = order
                .iter()
                .map(|&f| if f == 0.0 { 0.3 } else { f * g })
                .collect();
            // Precondition: the fold differs from the net, in the authored
            // order and in the ledger's push order.
            assert_ne!(fold(&authored).to_bits(), exact_net(&authored).to_bits());
            let (terms, force) = case_ledger(&request, 0);
            assert!(
                !fold_differs(&terms, &force).is_empty(),
                "G={g}: ledger fold precondition"
            );
            assert!(
                c3_detect_flags(&request),
                "G={g} {order:?}: C3-detect must flag"
            );
            let net = probe_a(1.0, [0.0, 0.0, 0.0]);
            let net = {
                let mut r = net;
                let loads = r["model"]["load_cases"][0]["primitive_loads"]
                    .as_array()
                    .unwrap()[..1]
                    .to_vec();
                r["model"]["load_cases"][0]["primitive_loads"] = json!(loads);
                r
            };
            let entries: &[Entry] = if g < 9.0e15 {
                &[Entry::Captured, Entry::Typed]
            } else {
                &[Entry::Typed]
            };
            for &entry in entries {
                for mode in MODES {
                    let label = format!("F2 G={g} {order:?} {entry:?} {mode:?}");
                    let envelope = solved(entry, &request, mode);
                    assert_eq!(
                        integrity_code(&envelope, "case"),
                        "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                        "{label}"
                    );
                    assert!(
                        !codes(&envelope).contains("LOAD_CONTRIBUTION_ABSORBED"),
                        "{label}"
                    );
                    assert_probe_a(&envelope, &label);
                    let e = probe_a_expected();
                    for (location, fraction) in
                        [("quarter_1", 0.25), ("midspan", 0.5), ("quarter_3", 0.75)]
                    {
                        let rest = 1.0 - fraction;
                        let my =
                            element_value(&envelope, "case", "pipe", "bending_moment_y", location);
                        let mz =
                            element_value(&envelope, "case", "pipe", "bending_moment_z", location);
                        assert!(
                            within(my.hypot(mz), e.moment_i * rest * rest, 0.0),
                            "{label}: M at {location}"
                        );
                        let vy =
                            element_value(&envelope, "case", "pipe", "shear_force_y", location);
                        let vz =
                            element_value(&envelope, "case", "pipe", "shear_force_z", location);
                        assert!(
                            within(vy.hypot(vz), e.shear_i * rest, 0.0),
                            "{label}: V at {location}"
                        );
                    }
                    let reference = solved(entry, &net, mode);
                    assert_eq!(
                        value_bits(&envelope, "case"),
                        value_bits(&reference, "case"),
                        "{label}: rows differ from the net-load answer"
                    );
                }
            }
        }
    }
    f2_probe_a_simply_supported(g);
}

/// Probe A's member simply supported (root: UX, UY, UZ, RX; tip: UY, UZ), so
/// that the span's elastic stress maximum is interior (w L^2 / 8 at midspan)
/// and depends on the span intensity itself (E7), not only on the end
/// resultants: on the cantilever the maximum sits at the root and E7's
/// intensity does not reach the published value.
fn probe_a_simply_supported(g: f64, order: [f64; 3]) -> Value {
    let mut request = probe_a(g, order);
    request["model"]["supports"] = json!([
        support("pin", "root", "anchor", &["UX", "UY", "UZ", "RX"]),
        support("roller", "tip", "anchor", &["UY", "UZ"])
    ]);
    request
}

fn f2_probe_a_simply_supported(g: f64) {
    let maximum_id = "result:elastic-maximum:4:case:4:pipe";
    for order in [GNG, NGG] {
        let request = probe_a_simply_supported(g, order);
        let authored: Vec<f64> = order
            .iter()
            .map(|&f| if f == 0.0 { 0.3 } else { f * g })
            .collect();
        // Precondition: E7's binary64 fold of the span's intensities, in the
        // authored order, differs from their correctly rounded net.
        assert_ne!(fold(&authored).to_bits(), exact_net(&authored).to_bits());
        let net = {
            let mut r = probe_a_simply_supported(1.0, [0.0, 0.0, 0.0]);
            let loads = r["model"]["load_cases"][0]["primitive_loads"]
                .as_array()
                .unwrap()[..1]
                .to_vec();
            r["model"]["load_cases"][0]["primitive_loads"] = json!(loads);
            r
        };
        let entries: &[Entry] = if g < 9.0e15 {
            &[Entry::Captured, Entry::Typed]
        } else {
            &[Entry::Typed]
        };
        for &entry in entries {
            for mode in MODES {
                let label = format!("F2-SS G={g} {order:?} {entry:?} {mode:?}");
                let envelope = solved(entry, &request, mode);
                assert_eq!(
                    integrity_code(&envelope, "case"),
                    "NUMERICAL_INTEGRITY_CHECKS_PASSED",
                    "{label}"
                );
                assert!(
                    !codes(&envelope).contains("LOAD_CONTRIBUTION_ABSORBED"),
                    "{label}"
                );
                // Exact net answers: M(L/2) = w L^2 / 8 = 0.15 (0.3 / 2
                // exactly), quarter-span shear w L / 4 = 0.15.
                let my = element_value(&envelope, "case", "pipe", "bending_moment_y", "midspan");
                let mz = element_value(&envelope, "case", "pipe", "bending_moment_z", "midspan");
                assert!(
                    within(my.hypot(mz), 0.3 / 2.0, 0.0),
                    "{label}: midspan moment"
                );
                let vy = element_value(&envelope, "case", "pipe", "shear_force_y", "quarter_1");
                let vz = element_value(&envelope, "case", "pipe", "shear_force_z", "quarter_1");
                assert!(
                    within(vy.hypot(vz), 0.3 / 2.0, 0.0),
                    "{label}: quarter-span shear"
                );
                let reference = solved(entry, &net, mode);
                let bits = value_bits(&envelope, "case");
                // The interior maximum is published, so E7 reaches a row.
                assert!(bits.contains_key(maximum_id), "{label}: {bits:?}");
                assert_eq!(
                    bits,
                    value_bits(&reference, "case"),
                    "{label}: rows differ from the net-load answer"
                );
            }
        }
    }
}

// ---------------------------------------------------------------- builders

fn preview_model(id: &str) -> Value {
    json!({
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:t3-s11f:{id}"), "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [], "pipe_segments": [], "materials": [], "supports": [], "components": [],
        "load_cases": [{"id": "case", "label": id, "kind": "primitive_user_load", "primitive_loads": [],
            "provenance": INVENTED}],
        "combinations": []})
}

fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id": id, "position": {"x": x, "y": y, "z": z}, "provenance": INVENTED})
}

fn pipe(id: &str, from: &str, to: &str, od: f64, wall: f64) -> Value {
    json!({"id": id, "from": from, "to": to, "material": "material", "y_reference": {"x": 0, "y": 1, "z": 0},
        "section": {"outside_diameter": {"value": od, "unit": "m"}, "wall_thickness": {"value": wall, "unit": "m"}},
        "provenance": INVENTED})
}

/// Invented steel-like material: E = 200 GPa, G = 80 GPa, alpha = 1.2e-5 /degC.
fn material() -> Value {
    json!({"id": "material", "elastic_modulus": {"value": 200e9, "unit": "Pa"},
        "shear_modulus": {"value": 80e9, "unit": "Pa"},
        "thermal_expansion_coefficient": {"value": 1.2e-5, "unit": "1/degC"}, "provenance": INVENTED})
}

fn support(id: &str, node: &str, family: &str, restraints: &[&str]) -> Value {
    json!({"id": id, "node": node, "family": family, "restraints": restraints, "provenance": INVENTED})
}

fn nodal(id: &str, node: &str, direction: &str, value: f64) -> Value {
    let moment = direction.starts_with('R');
    json!({"id": id, "category": if moment { "concentrated_moment" } else { "concentrated_force" },
        "target": {"type": "node", "node": node}, "direction": direction,
        "magnitude": {"value": value, "unit": if moment { "N*m" } else { "N" }},
        "dimension": if moment { "moment" } else { "force" }, "provenance": INVENTED})
}

fn uniform(id: &str, pipe: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": pipe},
        "direction": direction, "magnitude": {"value": value, "unit": "N/m"},
        "dimension": "force_per_length", "provenance": INVENTED})
}

fn thermal(id: &str, pipe: &str, delta_t: f64) -> Value {
    json!({"id": id, "category": "thermal", "target": {"type": "element", "pipe": pipe},
        "direction": "global_x", "magnitude": {"value": delta_t, "unit": "degC"},
        "dimension": "temperature_interval", "provenance": INVENTED})
}

fn pressure(id: &str, pipe: &str, value: f64) -> Value {
    json!({"id": id, "category": "pressure", "target": {"type": "element", "pipe": pipe},
        "direction": "global_x", "magnitude": {"value": value, "unit": "Pa"},
        "dimension": "pressure", "provenance": INVENTED})
}

fn request_of(model: Value) -> Value {
    json!({"model": model, "materials": []})
}

// ------------------------------------------------- F3: the realistic thermal case

/// Hot colinear run: nodes at x = 0, 10 and 20 m, OD 1.0 m, wall 0.05 m
/// (A = pi (0.5^2 - 0.45^2) m^2), anchored at both ends; both members carry
/// the same temperature interval, so the interior node receives the two
/// thermal pair terms +P and -P (P = E A alpha dT, about 4.1e7 N) after a
/// 1.3 N co-axial nodal load. Expected: u_x(mid) = 1.3 L / (2 E A).
const HOT_DELTA_T: f64 = 114.0;
const HOT_OD: f64 = 1.0;
const HOT_WALL: f64 = 0.05;
const HOT_L: f64 = 10.0;

fn hot_run() -> Value {
    let mut model = preview_model("hot-run");
    model["nodes"] = json!([
        node("n0", 0.0, 0.0, 0.0),
        node("n1", HOT_L, 0.0, 0.0),
        node("n2", 2.0 * HOT_L, 0.0, 0.0)
    ]);
    model["pipe_segments"] = json!([
        pipe("p1", "n0", "n1", HOT_OD, HOT_WALL),
        pipe("p2", "n1", "n2", HOT_OD, HOT_WALL)
    ]);
    model["materials"] = json!([material()]);
    let all = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
    model["supports"] = json!([
        support("a0", "n0", "anchor", &all),
        support("a2", "n2", "anchor", &all)
    ]);
    model["load_cases"][0]["primitive_loads"] = json!([
        nodal("nodal", "n1", "global_x", 1.3),
        thermal("hot:1", "p1", HOT_DELTA_T),
        thermal("hot:2", "p2", HOT_DELTA_T)
    ]);
    request_of(model)
}

fn hot_area() -> f64 {
    let ri = 0.5 * HOT_OD - HOT_WALL;
    std::f64::consts::PI * (0.25 * HOT_OD * HOT_OD - ri * ri)
}

/// F3 (ordinary route): the realistic breach of S11 section 2.3, repaired.
#[test]
fn f3_realistic_hot_run_publishes_the_exact_net_displacement() {
    let request = hot_run();
    let (terms, force) = case_ledger(&request, 0);
    // Precondition: the interior node's UX terms fold (1.3, +P, -P) to a
    // value that differs from 1.3 by more than 1e-9 relative.
    let dof = DOF_PER_NODE;
    let values: Vec<f64> = terms
        .iter()
        .filter(|t| t.dof == dof)
        .map(|t| match t.kind {
            open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(v) => v,
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(values.len(), 3);
    assert!(values[1].abs() > 3.0e7, "{values:?}");
    assert_eq!(force.values()[dof], 1.3);
    assert!(
        !within(fold(&values), 1.3, 0.0),
        "precondition: fold {} does not breach",
        fold(&values)
    );
    let expected_mm = 1.3 * HOT_L / (2.0 * 200e9 * hot_area()) * 1000.0;
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let envelope = solved(entry, &request, mode);
            let ux = row_by_id(&envelope, "case", "result:disp:n1:ux")
                .expect("n1 ux")
                .value;
            assert!(
                within(ux, expected_mm, 0.0),
                "F3 {entry:?} {mode:?}: {ux} vs {expected_mm}"
            );
            assert_eq!(
                integrity_code(&envelope, "case"),
                "NUMERICAL_INTEGRITY_CHECKS_PASSED"
            );
        }
    }
}

// ------------------------------------ F4 and F6: retained source, pre-0.4

/// N05's committed model (a 2 m member, OD 0.2 m, wall 0.01 m, E = 200 GPa,
/// G = 80 GPa, anchored except RX, which a soft spring k = 1e-4 N*m/rad
/// holds; Sensitive on the ordinary route), with its tip torque replaced by
/// the given RX contributions in that order.
fn n05_with_tip_torques(torques: &[f64]) -> Value {
    let mut model: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/numerical_sensitive_torsion_model.json"
    ))
    .unwrap();
    model["load_cases"][0]["primitive_loads"] = json!(torques
        .iter()
        .enumerate()
        .map(|(i, &t)| nodal(&format!("tip:{i}"), "tip", "RX", t))
        .collect::<Vec<_>>());
    request_of(model)
}

/// F4: an N05-class Sensitive case with (1e8, 0.3, -1e8) N*m at the tip (on
/// N05's own torque DOF; a transverse tip force makes this model's pre-0.4
/// replay exceed its work budget whatever the load, see RETURN.md) is
/// selected by the retained-source route, and its answer equals the answer
/// for the single exact net term, bit for bit (the retained exact solve and
/// the ledger both see the same exact net).
#[test]
fn f4_retained_source_selects_the_cancelling_case_and_matches_the_exact_net() {
    let request = n05_with_tip_torques(&[1e8, 0.3, -1e8]);
    let (terms, force) = case_ledger(&request, 0);
    assert!(
        !fold_differs(&terms, &force).is_empty(),
        "precondition: ledger fold equals net"
    );
    let net = n05_with_tip_torques(&[0.3]);
    for mode in MODES {
        let envelope = solved(Entry::Captured, &request, mode);
        assert!(
            codes(&envelope).contains("SOURCE_BLOCK_RECOVERY_SELECTED"),
            "F4 {mode:?}: not selected: {:?}",
            codes(&envelope)
        );
        let reference = solved(Entry::Captured, &net, mode);
        assert!(codes(&reference).contains("SOURCE_BLOCK_RECOVERY_SELECTED"));
        assert_eq!(
            value_bits(&envelope, "case"),
            value_bits(&reference, "case"),
            "F4 {mode:?}"
        );
    }
}

/// F6: a multi-case pre-0.4 exact-route invocation. Case A (the F4 loads) is
/// selected; case B carries the cancelling loads and an element load, so it
/// is outside retained scope. No `Err` (SOURCE_BLOCKS_FINALIZATION_FAILED),
/// no blocked envelope, and case A's receipt finalizes.
#[test]
fn f6_multi_case_pre_0_4_invocation_never_errs_on_cancelling_loads() {
    let mut request = n05_with_tip_torques(&[1e8, 0.3, -1e8]);
    let mut case_b = request["model"]["load_cases"][0].clone();
    case_b["id"] = json!("case-b");
    for load in case_b["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = json!(format!("b:{}", load["id"].as_str().unwrap()));
    }
    case_b["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(uniform("udl:b", "pipe", "global_y", 10.0));
    request["model"]["load_cases"]
        .as_array_mut()
        .unwrap()
        .push(case_b);
    let (terms, force) = case_ledger(&request, 1);
    assert!(
        !fold_differs(&terms, &force).is_empty(),
        "precondition for case B"
    );
    for mode in MODES {
        let envelope = run(Entry::Captured, &request, mode)
            .unwrap_or_else(|e| panic!("F6 {mode:?}: the invocation errs: {e}"));
        let blocking = envelope
            .diagnostics
            .iter()
            .filter(|d| d.severity == "blocking")
            .map(|d| format!("{}: {}", d.code, d.message))
            .collect::<Vec<_>>();
        assert!(blocking.is_empty(), "F6 {mode:?}: blocked {blocking:?}");
        assert!(
            envelope.source_block_recovery.is_some(),
            "F6 {mode:?}: no receipt"
        );
        assert!(envelope
            .diagnostics
            .iter()
            .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED"
                && d.affected_refs.contains(&"case".to_string())));
    }
}

// ---------------------------------------------- 0.4.0 builders (F3, F5, F9)

fn load_state_witness() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/load_reference_source/eigen_motion.request.json"
    ))
    .unwrap()
}

/// The 0.4.0 variants built from the committed eigen/motion witness.
#[derive(Clone, Copy)]
enum LoadState {
    /// A colinear hot run: root (0), mid (L) and tip (2L) along x, OD 1.0 m,
    /// wall 0.05 m, the witness's pair (E = 200 GPa, nu = 0.25), both members
    /// with the explicit thermal strain `strain` (fit strain 0) and a 1.3 N UX
    /// load at mid. Root and tip are fully anchored. With `soft_tip`, the
    /// model also carries a separate N05-class member (a 2 m member at
    /// y = 5 m, OD 0.2 m, wall 0.01 m, anchored except RX, which an invented
    /// soft spring k = 1e-4 N*m/rad holds, tip torque 1e-8 N*m), so the
    /// ordinary route is Sensitive and the retained join is attempted, with
    /// every free block within the retained method's supported order.
    HotRun { strain: f64, soft_tip: bool },
    /// V1's probe P: root (0), mid (3 m) and tip (6 m), OD 1.0 m, wall 0.05 m,
    /// zero thermal strain, root fully anchored, tip held in UY, UZ, RY and RZ,
    /// both ends settled by `g` (m) in UY, and `moment` N*m about z at mid.
    Settle { g: f64, moment: f64 },
}

const SOFT_TIP_K: f64 = 1.0e-4;

fn load_state_request(variant: LoadState) -> Value {
    let mut request = load_state_witness();
    let model = &mut request["model"];
    let (span, strain) = match variant {
        LoadState::HotRun { strain, .. } => (HOT_L, strain),
        LoadState::Settle { .. } => (3.0, 0.0),
    };
    model["nodes"] = json!([
        node("root", 0.0, 0.0, 0.0),
        node("mid", span, 0.0, 0.0),
        node("tip", 2.0 * span, 0.0, 0.0)
    ]);
    let mut m1 = model["pipe_segments"][0].clone();
    m1["id"] = json!("m1");
    m1["from"] = json!("root");
    m1["to"] = json!("mid");
    m1["section"] = json!({"outside_diameter": {"value": HOT_OD, "unit": "m"},
        "wall_thickness": {"value": HOT_WALL, "unit": "m"}});
    let mut m2 = m1.clone();
    m2["id"] = json!("m2");
    m2["from"] = json!("mid");
    m2["to"] = json!("tip");
    let mut members = vec![m1.clone(), m2];
    if let LoadState::HotRun { soft_tip: true, .. } = variant {
        let mut nodes = model["nodes"].as_array().unwrap().clone();
        nodes.push(node("n05-root", 0.0, 5.0, 0.0));
        nodes.push(node("n05-tip", 2.0, 5.0, 0.0));
        model["nodes"] = json!(nodes);
        let mut m05 = m1;
        m05["id"] = json!("m05");
        m05["from"] = json!("n05-root");
        m05["to"] = json!("n05-tip");
        m05["section"] = json!({"outside_diameter": {"value": 0.2, "unit": "m"},
            "wall_thickness": {"value": 0.01, "unit": "m"}});
        members.push(m05);
    }
    let member_ids: Vec<String> = members
        .iter()
        .map(|m| m["id"].as_str().unwrap().to_string())
        .collect();
    model["pipe_segments"] = json!(members);
    let all = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
    let (supports, loads, motion) = match variant {
        LoadState::HotRun {
            soft_tip: false, ..
        } => (
            vec![
                support("anchor", "root", "anchor", &all),
                support("stop", "tip", "anchor", &all),
            ],
            vec![nodal("nodal", "mid", "UX", 1.3)],
            None,
        ),
        LoadState::HotRun { soft_tip: true, .. } => (
            vec![
                support("anchor", "root", "anchor", &all),
                support("stop", "tip", "anchor", &all),
                support(
                    "n05-anchor",
                    "n05-root",
                    "anchor",
                    &["UX", "UY", "UZ", "RY", "RZ"],
                ),
                json!({"id": "spring", "node": "n05-root", "family": "spring", "restraints": ["RX"],
                    "stiffness": {"dof": "RX", "value": {"value": SOFT_TIP_K, "unit": "N*m/rad"}},
                    "provenance": INVENTED}),
            ],
            vec![
                nodal("nodal", "mid", "UX", 1.3),
                nodal("torque", "n05-tip", "RX", 1.0e-8),
            ],
            None,
        ),
        LoadState::Settle { g, moment } => (
            vec![
                support("anchor", "root", "anchor", &all),
                support("stop", "tip", "anchor", &["UY", "UZ", "RY", "RZ"]),
            ],
            vec![nodal("moment", "mid", "RZ", moment)],
            Some(g),
        ),
    };
    let support_ids: Vec<String> = supports
        .iter()
        .map(|s| s["id"].as_str().unwrap().to_string())
        .collect();
    model["supports"] = json!(supports);
    let reference = &mut model["reference_configurations"][0]["member_references"];
    let mut template = reference[0].clone();
    template["fit"]["strain"]["value"] = json!(0.0);
    *reference = json!(member_ids
        .iter()
        .map(|id| {
            let mut r = template.clone();
            r["pipe_ref"] = json!(id);
            r
        })
        .collect::<Vec<_>>());
    let case = &mut model["load_cases"][0];
    case["primitive_loads"] = json!(loads);
    let state = &mut case["analysis_state"];
    let template = state["element_states"][0].clone();
    // A zero thermal strain gives no eigen load (thermal_state is required).
    state["element_states"] = json!(member_ids
        .iter()
        .map(|id| {
            let mut e = template.clone();
            e["pipe_ref"] = json!(id);
            e["thermal_state"]["strain"]["value"] = json!(if id == "m05" { 0.0 } else { strain });
            e
        })
        .collect::<Vec<_>>());
    let support_states = support_ids
        .iter()
        .map(|id| {
            let mut entry =
                json!({"support_ref": id, "participation": {"kind": "active_model_device"}});
            if let (Some(g), true) = (motion, id == "anchor" || id == "stop") {
                entry["boundary_motion"] = json!([{"dof": "UY", "value": {"value": g, "unit": "m"},
                    "meaning": "absolute_reference_displacement"}]);
            }
            entry
        })
        .collect::<Vec<_>>();
    state["support_states"] = json!(support_states);
    state["load_sources"] = json!(loads
        .iter()
        .map(|l| json!({"source_ref": l["id"], "factor": 1.0}))
        .collect::<Vec<_>>());
    request
}

/// The eigen axial load E A eps, as the product forms it.
fn hot_eigen_axial(strain: f64) -> f64 {
    2.0e11 * hot_area() * strain
}

/// Strain giving P about 4.1e7 N on the hot section.
const HOT_EIGEN_STRAIN: f64 = 1.37e-3;

/// F3 (0.4.0 eigen route): the same hot run authored as a resolved case; the
/// eigen pairs (+P, -P) meet the 1.3 N load at the interior node.
#[test]
fn f3_realistic_hot_run_on_the_0_4_0_eigen_route() {
    let request = load_state_request(LoadState::HotRun {
        strain: HOT_EIGEN_STRAIN,
        soft_tip: false,
    });
    let p = hot_eigen_axial(HOT_EIGEN_STRAIN);
    assert!(p > 4.0e7 && p < 4.2e7, "{p}");
    assert!(
        !within(fold(&[1.3, p, -p]), 1.3, 0.0),
        "precondition: fold does not breach"
    );
    let expected_mm = 1.3 * HOT_L / (2.0 * 2.0e11 * hot_area()) * 1000.0;
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let envelope = solved(entry, &request, mode);
            let ux = row_by_id(&envelope, "case:join", "result:disp:mid:ux")
                .expect("mid ux")
                .value;
            assert!(
                within(ux, expected_mm, 0.0),
                "F3 0.4.0 {entry:?} {mode:?}: {ux} vs {expected_mm}"
            );
        }
    }
}

/// F5 (V1's 0.4.0 test, restated, S11B-6): the producer pushes nodal loads
/// first and then the eigen pairs, so the unfavourable order (1.3, +N, -N)
/// arises at the interior node of two colinear hot members (N about 4.1e7 N).
/// The N05-class appendix makes the ordinary route Sensitive, so the
/// retained join is attempted. It is selected, finalization succeeds (the
/// replay builds the same ledger), and the published mid displacement is the
/// exact net's 1.3 L / (2 E A), within 1e-9.
#[test]
fn f5_eigen_join_is_selected_and_publishes_the_exact_net() {
    let request = load_state_request(LoadState::HotRun {
        strain: HOT_EIGEN_STRAIN,
        soft_tip: true,
    });
    let p = hot_eigen_axial(HOT_EIGEN_STRAIN);
    assert_ne!(
        fold(&[1.3, p, -p]),
        exact_net(&[1.3, p, -p]),
        "precondition"
    );
    assert!(
        !within(fold(&[1.3, p, -p]), 1.3, 0.0),
        "precondition: the fold breaches"
    );
    let expected_mm = 1.3 * HOT_L / (2.0 * 2.0e11 * hot_area()) * 1000.0;
    for mode in MODES {
        let envelope = solved(Entry::Captured, &request, mode);
        assert!(
            codes(&envelope).contains("SOURCE_BLOCK_RECOVERY_SELECTED"),
            "F5 {mode:?}: the join is not selected: {:?}",
            codes(&envelope)
        );
        assert!(
            envelope.source_block_recovery.is_some(),
            "F5 {mode:?}: finalization failed"
        );
        let ux = row_by_id(&envelope, "case:join", "result:disp:mid:ux")
            .expect("mid ux")
            .value;
        assert!(
            within(ux, expected_mm, 0.0),
            "F5 {mode:?}: {ux} vs {expected_mm}"
        );
    }
}

/// F9 (S11B-1): V1's probe P authored on T1's 0.4.0 route: two 3 m spans,
/// both ends settled by g = 0.05 and 0.20 m (a rigid translation), end
/// rotations held, 0.0137 N*m at mid. The mid RZ row's two prescribed
/// couplings are exact negatives (+-6 E I / L^2, checked in the product's
/// stiffness by `n6_*`'s model) and its coupling to the mid UY is exactly 0,
/// so the exact reduced right-hand side (KS1) is m and the mid rotation is
/// m L / (8 E I). A binary64 fold of m - K_ra g - K_rc g loses bits of m
/// against the gross 6 E I g / L^2 (about 1.1e8 N*m). The published mid
/// rotation is within 1e-9 of m L / (8 E I) (net-governed, scale |exp|).
///
/// Not asserted, and recorded as a limit (W1, formed-term and displacement
/// representation accuracy): the member end moments, and any rotation that
/// couples to a displacement carrying the rigid offset g, whose binary64
/// recovery (ulp(g) times the gross coupling, about 1e-8 N*m) cannot meet
/// 1e-9 of m on any binary64 route.
#[test]
fn f9_support_motion_on_the_0_4_0_route_is_exact() {
    const M: f64 = 0.0137;
    const L: f64 = 3.0;
    let ri = 0.5 * HOT_OD - HOT_WALL;
    let i = std::f64::consts::PI / 4.0 * ((0.5 * HOT_OD).powi(4) - ri.powi(4));
    let ei = 2.0e11 * i;
    let expected = M * L / (8.0 * ei);
    for g in [0.05, 0.20] {
        let request = load_state_request(LoadState::Settle { g, moment: M });
        // Precondition: the binary64 fold of m - K_ra g - K_rc g (the
        // couplings +-6EI/L^2) differs from m by more than 1e-9 m.
        let coupling = 6.0 * ei / (L * L) * g;
        let folded = (M + coupling) - coupling;
        assert!(!within(folded, M, 0.0), "precondition: fold {folded}");
        for mode in MODES {
            for entry in [Entry::Captured, Entry::Typed] {
                let envelope = solved(entry, &request, mode);
                let rz = row_by_id(&envelope, "case:join", "result:disp:mid:rz")
                    .expect("mid rz")
                    .value;
                assert!(
                    within(rz, expected, 0.0),
                    "F9 g={g} {entry:?} {mode:?}: {rz:e} vs {expected:e}"
                );
            }
        }
    }
}

// ---------------------------------------------------- F8: curved bends

/// A single 90-degree macro-realized bend (chord 2 m along x, R = sqrt(2) m,
/// SIF 1.15, flexibility 2.0, as the lib tests' `curved_bend_span_request`),
/// OD 0.168 m, wall 0.007 m, anchored at the root, the tip held in UX and UY,
/// with the given loads on the span.
fn curved_bend(loads: Vec<Value>) -> Value {
    let mut model = preview_model("curved");
    model["nodes"] = json!([node("n0", 0.0, 0.0, 0.0), node("n1", 2.0, 0.0, 0.0)]);
    model["pipe_segments"] = json!([pipe("bend", "n0", "n1", 0.168, 0.007)]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([
        support("a0", "n0", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"]),
        support("g1", "n1", "guide", &["UX", "UY"])
    ]);
    model["components"] = json!([{
        "id": "component:bend", "label": "Invented bend", "kind": "bend", "node": "n1",
        "geometry": {"bend_pipe_ref": "bend", "bend_radius": {"value": std::f64::consts::SQRT_2, "unit": "m"},
            "bend_angle": {"value": std::f64::consts::PI / 2.0, "unit": "rad"},
            "bend_plane_orientation": "global_xy_preview",
            "bend_geometry_source_reference": "invented_user_entered_preview_geometry"},
        "modifiers": {"sif_user_value": {"value": 1.15, "unit": "none"},
            "flexibility_factor_user_value": {"value": 2.0, "unit": "none"},
            "source_reference": "invented_user_entered_preview_no_code_table"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element",
            "rule_check_consumption": "user_rule_pack_inputs_only"},
        "completeness": [{"finding_id": "finding:bend:geometry", "status": "complete",
            "diagnostic_code": "BEND_GEOMETRY_INCOMPLETE", "missing_field_kinds": []}],
        "provenance": INVENTED}]);
    model["load_cases"][0]["primitive_loads"] = json!(loads);
    request_of(model)
}

/// F8 (R3-2): a bend carrying three uniform loads (G, 0.3, -G) N/m, G = 1e8
/// and 1e80, in that authored order, and a thermal case with three
/// cancelling temperature intervals (T, t, -T), so the strains are
/// (eps, eps_n, -eps) and E8's pre-summed strain would be exercised. The
/// recovery (end forces and arc stations) equals the answer for the net load
/// alone, bit for bit: the ledger nets and E8-E11's per-load exact sums see
/// the same exact net.
#[test]
fn f8_curved_bend_uniform_g1e8() {
    f8_curved_uniform(1e8);
}

#[test]
fn f8_curved_bend_uniform_g1e80() {
    f8_curved_uniform(1e80);
}

fn f8_curved_uniform(g: f64) {
    {
        let loads = vec![
            uniform("w:0", "bend", "global_z", g),
            uniform("w:1", "bend", "global_z", 0.3),
            uniform("w:2", "bend", "global_z", -g),
        ];
        assert_ne!(
            fold(&[g, 0.3, -g]),
            0.3,
            "precondition: the intensity fold equals the net"
        );
        let request = curved_bend(loads);
        let (terms, force) = case_ledger(&request, 0);
        assert!(
            !fold_differs(&terms, &force).is_empty(),
            "G={g}: ledger fold precondition"
        );
        let net = curved_bend(vec![uniform("w:1", "bend", "global_z", 0.3)]);
        let entries: &[Entry] = if g < 9.0e15 {
            &[Entry::Captured, Entry::Typed]
        } else {
            &[Entry::Typed]
        };
        for &entry in entries {
            for mode in MODES {
                let envelope = solved(entry, &request, mode);
                let reference = solved(entry, &net, mode);
                assert_eq!(
                    value_bits(&envelope, "case"),
                    value_bits(&reference, "case"),
                    "F8 uniform G={g} {entry:?} {mode:?}"
                );
            }
        }
    }
}

/// F8, thermal part: three cancelling temperature intervals (T, t, -T) on the
/// bend (E8's pre-summed strain), compared with the net interval alone.
#[test]
fn f8_curved_bend_thermal_cancelling_strains() {
    // Thermal: alpha = 1.2e-5 /degC; T = 1e6 degC, t = 0.3 degC.
    let (big, small) = (1.0e6_f64, 0.3_f64);
    let eps = 1.2e-5 * big;
    let eps_n = 1.2e-5 * small;
    assert_eq!((-1.2e-5 * big), -eps);
    assert_ne!(
        fold(&[eps, eps_n, -eps]),
        eps_n,
        "precondition: the strain fold equals the net"
    );
    let request = curved_bend(vec![
        thermal("t:0", "bend", big),
        thermal("t:1", "bend", small),
        thermal("t:2", "bend", -big),
    ]);
    let net = curved_bend(vec![thermal("t:1", "bend", small)]);
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let envelope = solved(entry, &request, mode);
            let reference = solved(entry, &net, mode);
            assert_eq!(
                value_bits(&envelope, "case"),
                value_bits(&reference, "case"),
                "F8 thermal {entry:?} {mode:?}"
            );
        }
    }
}

/// F8, the curved thermal producer (M5's behavioural backing): each thermal
/// load pushes its load-proportional term as the exact product
/// `K_rc * fl(eps * chord)`, never the rounded product. Two nearly cancelling
/// temperature intervals (T, -T') are searched deterministically until the
/// precondition holds at a bend DOF: the correctly rounded sum of the exact
/// products differs from the correctly rounded sum of the rounded products
/// (the two paths differ). Then the case force there equals the former.
#[test]
fn f8_curved_bend_thermal_terms_are_exact_products() {
    use open_pipe_stress_frame_kernel::load_ledger::ForceTermKind;
    let big = 1.0e6_f64;
    for k in 1..=256 {
        let request = curved_bend(vec![
            thermal("t:0", "bend", big),
            thermal("t:1", "bend", -(big - k as f64 / 64.0)),
        ]);
        let (terms, force) = case_ledger(&request, 0);
        let mut differing = Vec::new();
        for dof in 0..force.len() {
            let at: Vec<&ForceTerm> = terms.iter().filter(|t| t.dof == dof).collect();
            let thermal: Vec<&&ForceTerm> =
                at.iter().filter(|t| t.source.starts_with("t:")).collect();
            if thermal.len() < 2 {
                continue;
            }
            let (mut exact, mut rounded) = (ExactAccumulator::new(), ExactAccumulator::new());
            for term in &at {
                match term.kind {
                    ForceTermKind::Term(v) => {
                        exact.add(v).unwrap();
                        rounded.add(v).unwrap();
                    }
                    ForceTermKind::Product(a, b) => {
                        exact.add_product(a, b).unwrap();
                        rounded.add(a * b).unwrap();
                    }
                }
            }
            // The thermal terms are exact products, one per load.
            assert!(
                thermal
                    .iter()
                    .all(|t| matches!(t.kind, ForceTermKind::Product(_, _))),
                "dof {dof}: a curved thermal term was pushed pre-rounded"
            );
            let (exact, rounded) = (exact.round().unwrap(), rounded.round().unwrap());
            assert_eq!(
                force.values()[dof].to_bits(),
                exact.to_bits(),
                "dof {dof}: the force is the correctly rounded exact-product net"
            );
            if exact.to_bits() != rounded.to_bits() {
                differing.push(dof);
            }
        }
        if !differing.is_empty() {
            // Precondition met at `differing`; the assertions above held at
            // every bend DOF of this request.
            return;
        }
    }
    panic!("precondition: no searched (T, -T') pair separates exact and rounded products");
}

// ---------------------------------------------------- F10: E15 and E16

/// A 3 m pipe (OD 0.168 m, wall 0.007 m) anchored at its root, carrying an
/// expansion joint (solver consumption `mechanics_geometry_and_user_flexibility`,
/// effective area 0.018 m^2, axial user stiffness 3.2e6 N/m only, so no
/// lateral-stiffness refusal) and the given pressure loads.
fn pressure_joint(loads: Vec<Value>) -> Value {
    let mut model = preview_model("pressure");
    model["nodes"] = json!([node("n0", 0.0, 0.0, 0.0), node("n1", 3.0, 0.0, 0.0)]);
    model["pipe_segments"] = json!([pipe("p", "n0", "n1", 0.168, 0.007)]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([support(
        "a0",
        "n0",
        "anchor",
        &["UX", "UY", "UZ", "RX", "RY", "RZ"]
    )]);
    model["components"] = json!([{
        "id": "component:joint", "label": "Invented expansion joint", "kind": "expansion_joint", "node": "n1",
        "geometry": {"expansion_joint_pipe_ref": "p", "effective_area": {"value": 0.018, "unit": "m^2"},
            "movement_limit": {"value": 0.045, "unit": "m"},
            "hardware_reference": "invented_user_entered_tie_rod_limit_metadata_no_catalog",
            "manufacturer_reference": "invented_user_entered_manufacturer_reference_no_catalog",
            "pressure_thrust_reference": "load_side_pressure_thrust_user_review_required",
            "expansion_joint_source_reference": "invented_user_entered_expansion_joint_preview_geometry"},
        "modifiers": {"axial_stiffness_user_value": {"value": 3200000, "unit": "N/m"},
            "source_reference": "invented_user_entered_expansion_joint_stiffness_no_catalog_or_code_table"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_and_user_flexibility",
            "rule_check_consumption": "user_rule_pack_inputs_only"},
        "completeness": [{"finding_id": "finding:joint", "status": "complete",
            "diagnostic_code": "EXPANSION_JOINT_STIFFNESS_DATA_MISSING", "missing_field_kinds": []}],
        "provenance": INVENTED}]);
    model["load_cases"][0]["primitive_loads"] = json!(loads);
    request_of(model)
}

/// F10 (S11B-2): pressure loads (P, 0.3 Pa, -P), P = 1e8 and 1e80 Pa, on a
/// pipe with an expansion joint carrying the matching thrust loads. The
/// published hoop and longitudinal stresses (E15) and
/// `expansion_joint_pressure_thrust_load_review` (E16) equal the values for
/// the net pressure 0.3 Pa alone, bit for bit.
///
/// E15 and E16 are unreachable from a fresh product solve: a nonzero legacy
/// pressure load is refused (`PRESSURE_MODEL_REAUTHOR_REQUIRED`,
/// `pressure_runtime.rs`), so they are exercised inside the test-only
/// `historical_pressure_reference::with_scope` (manager/ROOT, 2026-09-27).
#[test]
fn f10_pressure_and_joint_thrust_g1e8() {
    f10_pressure(1e8);
}

#[test]
fn f10_pressure_and_joint_thrust_g1e80() {
    f10_pressure(1e80);
}

fn f10_pressure(p: f64) {
    {
        assert_ne!(fold(&[p, 0.3, -p]), 0.3, "precondition: pressure fold");
        let area = 0.018;
        assert_ne!(
            fold(&[p * area, 0.3 * area, -p * area]),
            0.3 * area,
            "precondition: thrust fold"
        );
        let request = pressure_joint(vec![
            pressure("p:0", "p", p),
            pressure("p:1", "p", 0.3),
            pressure("p:2", "p", -p),
        ]);
        let net = pressure_joint(vec![pressure("p:1", "p", 0.3)]);
        let entries: &[Entry] = if p < 9.0e15 {
            &[Entry::Captured, Entry::Typed]
        } else {
            &[Entry::Typed]
        };
        for &entry in entries {
            for mode in MODES {
                let envelope = crate::historical_pressure_reference::with_scope(|| {
                    solved(entry, &request, mode)
                });
                let reference =
                    crate::historical_pressure_reference::with_scope(|| solved(entry, &net, mode));
                let thrust = case_rows(&envelope, "case")
                    .find(|r| r.kind == "expansion_joint_pressure_thrust_load_review")
                    .unwrap_or_else(|| panic!("F10 P={p}: no thrust review row"));
                assert_eq!(thrust.value, 0.3 * area, "F10 P={p} {entry:?} {mode:?}");
                let mut rows = value_bits(&envelope, "case");
                let mut reference_rows = value_bits(&reference, "case");
                // The review row's id and its source list name the loads; the
                // value is compared above.
                rows.retain(|id, _| !id.starts_with("result:pressure-thrust"));
                reference_rows.retain(|id, _| !id.starts_with("result:pressure-thrust"));
                assert_eq!(rows, reference_rows, "F10 P={p} {entry:?} {mode:?}");
            }
        }
    }
}

// ------------------------------------------------ RV1-N6 behavioural backing

/// RV1-N6 (kept name, ROOT/manager condition): the product's residual rows on
/// a prescribed-coupled row are the exact-numerator residual
/// (`evaluate_assembled_original_residual`), never the public binary64
/// `evaluate_original_residual`, which the site test forbids in the product.
/// The product's own linear seam `solve_preview_reduced_system` solves a
/// probe-P model (two 3 m members, OD 1.0 m, wall 0.05 m, root pinned about
/// z, tip roller, both ends settled by g in UY, 0.0137 N*m at mid). The two
/// residual evaluations first differ at the solution (precondition).
#[test]
fn n6_product_residual_rows_use_the_exact_numerator() {
    use open_pipe_stress_frame_kernel::structural::{
        evaluate_assembled_original_residual, evaluate_original_residual, StructuralSystem,
    };
    let mut model = preview_model("n6");
    model["nodes"] = json!([
        node("a", 0.0, 0.0, 0.0),
        node("b", 3.0, 0.0, 0.0),
        node("c", 6.0, 0.0, 0.0)
    ]);
    model["pipe_segments"] = json!([
        pipe("m1", "a", "b", HOT_OD, HOT_WALL),
        pipe("m2", "b", "c", HOT_OD, HOT_WALL)
    ]);
    model["materials"] = json!([material()]);
    model["supports"] = json!([
        support("sa", "a", "anchor", &["UX", "UY", "UZ", "RX", "RY"]),
        support("sc", "c", "guide", &["UY", "UZ"])
    ]);
    model["load_cases"][0]["primitive_loads"] = json!([nodal("m", "b", "RZ", 0.0137)]);
    let request = request_of(model);
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = parsed.model;
    let mut materials = model.materials.clone();
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).unwrap();
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let stiffness = assemble_case_stiffness(&built, &boundary.springs).unwrap();
    let (_, force) = case_ledger(&request, 0);
    let b_rz = DOF_PER_NODE + RZ;
    let mut checked = 0;
    for g in [0.05, 0.20] {
        let prescribed = boundary
            .restrained_dofs
            .iter()
            .map(|&d| (d, if d % DOF_PER_NODE == UY { g } else { 0.0 }))
            .collect::<Vec<_>>();
        let values = prescribed.iter().map(|&(_, v)| v).collect::<Vec<_>>();
        let reduced = reduce_assembled_system_with_prescribed_displacements(
            &stiffness,
            &force,
            &boundary.restrained_dofs,
            &values,
        )
        .unwrap();
        let observation = legacy_observation_force(
            &force,
            &stiffness,
            &boundary.restrained_dofs,
            &prescribed,
            true,
        );
        for mode in MODES {
            let mut preliminary = Vec::new();
            let solve = solve_preview_reduced_system(
                mode,
                &stiffness,
                reduced.force.values(),
                &built,
                &boundary.springs,
                &force,
                &observation,
                &prescribed,
                &model.load_cases[0],
                &mut preliminary,
            )
            .unwrap();
            let mut u = vec![0.0; force.len()];
            for (k, &dof) in reduced.free_dofs.iter().enumerate() {
                u[dof] = solve.solution[k];
            }
            for &(dof, v) in &prescribed {
                u[dof] = v;
            }
            let system = StructuralSystem::assembled(
                &stiffness,
                &force,
                &reduced.free_dofs,
                &prescribed,
                None,
                None,
            );
            let exact = evaluate_assembled_original_residual(&system, &u).unwrap();
            let legacy = evaluate_original_residual(system.system(), &u).unwrap();
            let pick = |rows: &[open_pipe_stress_frame_kernel::structural::ResidualRow]| {
                rows.iter()
                    .find(|r| r.global_dof == b_rz)
                    .unwrap()
                    .normalized_residual
            };
            // Precondition: the two numerators differ on the coupled row.
            assert_ne!(
                pick(&exact).to_bits(),
                pick(&legacy).to_bits(),
                "N6 g={g} {mode:?}: precondition"
            );
            assert_eq!(
                pick(&solve.structural_report.residual_rows).to_bits(),
                pick(&exact).to_bits(),
                "N6 g={g} {mode:?}: the product row is not the exact numerator"
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 4);
}
