//! O4 M3-O4 (DEL-04-08 Design `o4-source-extension.md` §5): the product's route-E (W1-E,
//! `physics-retained-1`) publications judged against the frozen, refuted references
//! `validation/references/o4_retained_source/` (both stiffness readouts, exact).
//!
//! Binding (the reference README's "Binding"): each request-agnostic case document becomes one
//! model 0.8.0 product request (`judged_request`); each source twin is the in-memory
//! re-authoring of its pinned J2 or RF request's load case. Each request runs through the actual
//! W1 private driver (`retained_w1` after the single observed ordinary run, as the facade runs it
//! after G-C), in both solver modes. W1 attempts a case only when its ordinary verdict is not
//! `checks_passed` and no retained-source composite was selected, so most documents publish no
//! successor. The "augmented" binding adds W1's own test-path trigger (W1 stage 2 §4.4; the W1
//! coverage repair §3.2): an independent, disconnected soft-spring body (`m3x`'s published O4
//! control A geometry, re-authored in the document's material, shear model and units), which makes
//! the case Sensitive and refuses the source route's block search. The trigger is a separate body:
//! the judged document's exact rows are unchanged by it, and only those rows are judged.
//!
//! Judging (§5 "Acceptance"): every published row of every selected W1-E case that the references
//! cover is judged against BOTH readouts. A `relative_verified` row must lie within 1e-9·|value| of
//! the exact value (SI and published-unit decimal forms) and within the kernel's sharper allowance
//! (DEF-E `acceptance.final`: max(|x|,S)(2^-64 + 2^-85) + 2^-53|x| + 2^-1074); an
//! `absolute_verified` row within its published bound; an admitted-readout interval (bending
//! stress and maxima over hull(Z, I/r_o)) is met at its nearest point. The comparison is carried in
//! double-double arithmetic on the references' 40-digit decimals (about 1e-31 relative), so a
//! verdict closer than 1e-6 of a tolerance to its boundary is reported as undecided, never passed.
//! `not_covered` rows are counted separately. Expected values come only from the reference files.
//!
//! Every judged breach fails the test; per design §5 such a breach on a published row stops the
//! epoch and returns to HELP_HUMAN. Records are written under `O4_M3_OUT` when it is set.
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Every model is invented.
use super::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[path = "../../reporting/result_export/tests/support/retained_precision_corpus.rs"]
mod storage;

const SUMS: &str = include_str!("../../../validation/references/o4_retained_source/SHA256SUMS");
const CASES: &str = include_str!("../../../validation/references/o4_retained_source/cases.json");
const ORACLE: &str = include_str!("../../../validation/references/o4_retained_source/oracle.json");
const TWIN_CASES: &str = include_str!("../../../validation/references/o4_retained_source/source_twins_cases.json");
const TWIN_ORACLE: &str = include_str!("../../../validation/references/o4_retained_source/source_twins_oracle.json");
const J2: [(&str, &str); 11] = [
    ("fixtures/product_preview/pressure_source/physics_source/n05.request.json", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json")),
    ("fixtures/product_preview/pressure_source/physics_source/n06.request.json", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n06.request.json")),
    ("fixtures/product_preview/pressure_source/physics_source/n05_unicode.request.json", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05_unicode.request.json")),
    ("fixtures/product_preview/pressure_source/physics_source/n05_units.request.json", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05_units.request.json")),
    ("fixtures/product_preview/pressure_source/physics_source/fields.request.json", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/fields.request.json")),
    ("fixtures/product_preview/pressure_source/physics_source/mixed.request.json", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json")),
    ("fixtures/product_preview/pressure_source/physics_source/mixed_units.request.json", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/mixed_units.request.json")),
    ("fixtures/product_preview/pressure_source/load_reference_source/n05.request.json", include_str!("../../../fixtures/product_preview/pressure_source/load_reference_source/n05.request.json")),
    ("fixtures/product_preview/pressure_source/load_reference_source/n06.request.json", include_str!("../../../fixtures/product_preview/pressure_source/load_reference_source/n06.request.json")),
    ("fixtures/product_preview/pressure_source/load_reference_source/fields.request.json", include_str!("../../../fixtures/product_preview/pressure_source/load_reference_source/fields.request.json")),
    ("fixtures/product_preview/pressure_source/load_reference_source/mixed.request.json", include_str!("../../../fixtures/product_preview/pressure_source/load_reference_source/mixed.request.json")),
];
const RF_INPUTS: &str = include_str!("../../../fixtures/results/j7_rf_census_inputs.json");
const RF_INPUTS_PATH: &str = "fixtures/results/j7_rf_census_inputs.json";

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
/// The owner-adopted per-invocation default, 12 GiB (G-A's D1 verdict reads only the domain).
const LIMIT: RetainedByteLimit = RetainedByteLimit::new(12_884_901_888);
const PROVENANCE: &str = "invented_o4_m3_judging_no_library_data";

fn sha(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }

/// The model version the frozen unit pinned J2's committed requests at (model 0.6.0). Model 0.7.0
/// (DEL-02-01 Design `model-document-0.7.0.md` §2) re-authored those committed requests by their
/// version token alone, so a request binds to its frozen pin with that one token restored: the
/// frozen files are not edited, and any other byte change still fails the pin.
const J2_FROZEN_MODEL_VERSION: &str = "0.6.0";
fn j2_as_frozen(path: &str, text: &str) -> String {
    let current = format!("\"schema_version\": \"{}\"", crate::MODEL_SCHEMA_VERSION);
    assert_eq!(text.matches(&current).count(), 1, "{path}: one current model version token");
    text.replace(&current, &format!("\"schema_version\": \"{J2_FROZEN_MODEL_VERSION}\""))
}

/// The frozen files, each bound by the unit's SHA256SUMS; `cases_sha256` binds each oracle to its cases.
struct Frozen { cases: Value, oracle: Value, twins: Value, twin_oracle: Value }
fn frozen() -> &'static Frozen {
    static F: OnceLock<Frozen> = OnceLock::new();
    F.get_or_init(|| {
        for (name, text) in [("cases.json", CASES), ("oracle.json", ORACLE), ("source_twins_cases.json", TWIN_CASES), ("source_twins_oracle.json", TWIN_ORACLE)] {
            let line = SUMS.lines().find(|l| l.ends_with(&format!("  {name}"))).unwrap_or_else(|| panic!("{name} in SHA256SUMS"));
            assert_eq!(sha(text.as_bytes()), line.split_whitespace().next().unwrap(), "{name}: the frozen bytes");
        }
        let f = Frozen { cases: serde_json::from_str(CASES).unwrap(), oracle: serde_json::from_str(ORACLE).unwrap(),
            twins: serde_json::from_str(TWIN_CASES).unwrap(), twin_oracle: serde_json::from_str(TWIN_ORACLE).unwrap() };
        assert_eq!(f.oracle["cases_sha256"], json!(sha(CASES.as_bytes())));
        assert_eq!(f.twin_oracle["cases_sha256"], json!(sha(TWIN_CASES.as_bytes())));
        for (path, text) in J2 {
            assert_eq!(f.twins["inputs"][path], json!(sha(j2_as_frozen(path, text).as_bytes())), "{path}: the pinned J2 input");
        }
        assert_eq!(f.twins["inputs"][RF_INPUTS_PATH], json!(sha(RF_INPUTS.as_bytes())), "the pinned RF census inputs");
        f
    })
}

// ------------------------------------------------------------------ binding: requests

fn length(value: f64) -> Value { json!({"unit": "m", "value": value}) }
fn direction(kind: &str, axis: &str) -> &'static str {
    match (kind, axis) {
        ("force", "x") => "global_x", ("force", "y") => "global_y", ("force", "z") => "global_z",
        ("moment", "x") => "rotation_x", ("moment", "y") => "rotation_y", ("moment", "z") => "rotation_z",
        other => panic!("load direction {other:?}"),
    }
}

/// A case document (`cases.json` `document`) as one model 0.8.0 product request (README "Binding" 1).
fn judged_request(doc: &Value, project_id: &str) -> Value {
    let mat = &doc["material"];
    let loads: Vec<Value> = doc["loads"].as_array().unwrap().iter().map(|l| {
        let kind = l["kind"].as_str().unwrap();
        json!({"category": format!("concentrated_{kind}"), "dimension": kind, "direction": direction(kind, l["direction"].as_str().unwrap()),
            "id": l["id"], "magnitude": {"unit": l["unit"], "value": l["value"]}, "provenance": PROVENANCE,
            "target": {"node": l["node"], "type": "node"}})
    }).collect();
    let supports: Vec<Value> = doc["supports"].as_array().unwrap().iter().map(|s| match s["kind"].as_str().unwrap() {
        "rigid" => json!({"family": "anchor", "id": s["id"], "node": s["node"], "provenance": PROVENANCE, "restraints": s["restraints"]}),
        "spring" => json!({"family": "spring", "id": s["id"], "node": s["node"], "provenance": PROVENANCE, "restraints": [s["dof"]],
            "stiffness": {"dof": s["dof"], "value": {"unit": s["unit"], "value": s["stiffness"]}}}),
        other => panic!("support kind {other}"),
    }).collect();
    json!({"materials": [], "model": {
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "professional_acceptance": "not_provided",
            "rule_check": "not_performed_user_rule_inputs_missing"},
        "combinations": [], "components": [], "diagnostics": [],
        "data_boundary": {"private_data_policy": "no_private_project_data",
            "professional_boundary": "technical_preview_requires_human_engineering_review",
            "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
            "public_examples_policy": "invented_or_cleared_data_only"},
        "document_kind": "openpipestress.product_preview.model",
        "load_cases": [{"id": "case", "kind": "primitive_user_load", "label": project_id, "pressure_regions": doc["pressure_regions"],
            "primitive_loads": loads, "provenance": PROVENANCE, "status": "user_entered"}],
        "materials": [{"constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": {"unit": "Pa", "value": mat["elastic_modulus_Pa"]}, "id": mat["id"], "label": "Invented material",
            "poisson_ratio": {"unit": "1", "value": mat["poisson_ratio"]}, "provenance": PROVENANCE}],
        "nodes": doc["nodes"].as_array().unwrap().iter().map(|n| json!({"id": n["id"], "label": "Invented node", "provenance": PROVENANCE,
            "position": {"x": n["xyz_m"][0], "y": n["xyz_m"][1], "z": n["xyz_m"][2]}})).collect::<Vec<_>>(),
        "pipe_segments": doc["members"].as_array().unwrap().iter().map(|m| json!({"from": m["from"], "id": m["id"], "label": "Invented member",
            "material": mat["id"], "provenance": PROVENANCE, "to": m["to"],
            "section": {"outside_diameter": length(m["outside_diameter_m"].as_f64().unwrap()), "wall_thickness": length(m["wall_thickness_m"].as_f64().unwrap()),
                "mill_tolerance": length(m["mill_tolerance_m"].as_f64().unwrap()), "corrosion_allowance": length(m["corrosion_allowance_m"].as_f64().unwrap())},
            "y_reference": {"x": m["y_reference"][0], "y": m["y_reference"][1], "z": m["y_reference"][2]}})).collect::<Vec<_>>(),
        "pressure_contract": {"mode": "exact_pressure_v3", "shear_model": doc["shear_model"], "version": "3.0.0"},
        "project": {"description": "Invented O4 M3-O4 judged document; no external project or library data.", "id": project_id, "name": project_id,
            "units": {"angle": "rad", "force": "N", "length": "m", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "schema_version": "0.8.0",
        "supports": supports,
    }})
}

const THEORY: [(&str, &str); 2] = [("eb", "euler_bernoulli"), ("timo", "timoshenko_annulus_energy_v1")];
fn theory_token(theory: &str) -> &'static str { THEORY.iter().find(|(t, _)| *t == theory).unwrap().1 }
fn to_m(unit: &str) -> f64 { match unit { "m" => 1.0, "mm" => 0.001, "cm" => 0.01, other => panic!("length unit {other}") } }

/// The source twin's in-memory re-authoring of its pinned request (README "Binding" 4; the
/// generator's rule): the shear model, and the class's mill tolerance and allowance in the section's
/// own unit, each asserted equal to the frozen twin document's value. An RF census request (model
/// 0.3.0, no shear model, no allowances) is re-authored to model 0.8.0 with both explicit.
fn twin_request(twin: &Value) -> Value {
    let file = twin["source"]["file"].as_str().unwrap();
    let mut raw: Value = if file == RF_INPUTS_PATH {
        let census = storage::expand(serde_json::from_str(RF_INPUTS).unwrap());
        let name = twin["design"].as_str().unwrap().split('|').next().unwrap();
        census["cases"].as_array().unwrap().iter().find(|c| c["id"] == name).unwrap_or_else(|| panic!("{name}"))["request"].clone()
    } else {
        serde_json::from_str(J2.iter().find(|(p, _)| *p == file).unwrap().1).unwrap()
    };
    let (variant, theory) = (twin["variant"].as_str().unwrap(), twin["theory"].as_str().unwrap());
    let rf = file == RF_INPUTS_PATH;
    let model = &mut raw["model"];
    if rf {
        model["schema_version"] = json!("0.8.0");
        model["pressure_contract"] = json!({"mode": "exact_pressure_v3", "shear_model": theory_token(theory), "version": "3.0.0"});
    } else if variant == "zero" && theory == "eb" {
        return raw;
    } else {
        model["pressure_contract"]["shear_model"] = json!(theory_token(theory));
    }
    let members = twin["document"]["members"].as_array().unwrap();
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        let pipe_id = pipe["id"].clone();
        let section = &mut pipe["section"];
        let unit = section["wall_thickness"]["unit"].as_str().unwrap().to_owned();
        let t_m = section["wall_thickness"]["value"].as_f64().unwrap() * to_m(&unit);
        let doc = members.iter().find(|m| m["id"] == pipe_id).unwrap_or_else(|| panic!("{pipe_id}"));
        let (cm, ca) = (doc["mill_tolerance_m"].as_f64().unwrap(), doc["corrosion_allowance_m"].as_f64().unwrap());
        // The generator's class rule, re-derived here and held to the frozen document.
        let want_cm = if matches!(variant, "mill" | "both") { 0.125 * t_m } else { 0.0 };
        let want_ca = if matches!(variant, "ca" | "both") { if t_m < 0.006 { 0.25 * t_m } else { 0.0015 } } else { 0.0 };
        assert_eq!((cm.to_bits(), ca.to_bits()), (want_cm.to_bits(), want_ca.to_bits()), "{}: allowances", twin["id"]);
        if rf || variant != "zero" {
            let scale = to_m(&unit);
            assert_eq!(((cm / scale) * scale).to_bits(), cm.to_bits(), "{}: exact in {unit}", twin["id"]);
            section["mill_tolerance"] = json!({"unit": unit, "value": cm / scale});
            section["corrosion_allowance"] = json!({"unit": unit, "value": ca / scale});
        }
    }
    raw
}

/// W1's test-path trigger (see the module comment): `m3x`'s O4 control A body (N0 translations
/// rigid, RX 144 N·m/rad and RY/RZ 1e6 N·m/rad springs; one member to (2, 4, 4) m, D = 0.4 m,
/// t = 0.02 m; three moments at N1), disconnected, offset by (-64, -64, -64) m, in the document's
/// material, shear model and length unit, with zero allowances. Its ids carry the prefix `o4trig:`.
fn augment(raw: &Value) -> Value {
    let mut raw = raw.clone();
    let m = &mut raw["model"];
    let unit = m["project"]["units"]["length"].as_str().unwrap_or("m").to_owned();
    let s = 1.0 / to_m(&unit);
    let material = m["materials"][0]["id"].clone();
    let p = PROVENANCE;
    let len = |v: f64| json!({"unit": "m", "value": v});
    m["nodes"].as_array_mut().unwrap().extend([
        json!({"id": "o4trig:N0", "label": "Invented trigger node", "provenance": p, "position": {"x": -64.0 * s, "y": -64.0 * s, "z": -64.0 * s}}),
        json!({"id": "o4trig:N1", "label": "Invented trigger node", "provenance": p, "position": {"x": -62.0 * s, "y": -60.0 * s, "z": -60.0 * s}}),
    ]);
    m["pipe_segments"].as_array_mut().unwrap().push(json!({"id": "o4trig:M1", "from": "o4trig:N0", "to": "o4trig:N1", "label": "Invented trigger member",
        "material": material, "provenance": p, "y_reference": {"x": 1, "y": 0, "z": 0},
        "section": {"outside_diameter": len(0.4), "wall_thickness": len(0.02), "mill_tolerance": len(0.0), "corrosion_allowance": len(0.0)}}));
    m["supports"].as_array_mut().unwrap().extend([
        json!({"family": "anchor", "id": "o4trig:rigid", "node": "o4trig:N0", "provenance": p, "restraints": ["UX", "UY", "UZ"]}),
        json!({"family": "spring", "id": "o4trig:rx", "node": "o4trig:N0", "provenance": p, "restraints": ["RX"], "stiffness": {"dof": "RX", "value": {"unit": "N*m/rad", "value": 144.0}}}),
        json!({"family": "spring", "id": "o4trig:ry", "node": "o4trig:N0", "provenance": p, "restraints": ["RY"], "stiffness": {"dof": "RY", "value": {"unit": "N*m/rad", "value": 1.0e6}}}),
        json!({"family": "spring", "id": "o4trig:rz", "node": "o4trig:N0", "provenance": p, "restraints": ["RZ"], "stiffness": {"dof": "RZ", "value": {"unit": "N*m/rad", "value": 1.0e6}}}),
    ]);
    for (i, (dir, v)) in [("RX", 0.0048), ("RY", 0.0096), ("RZ", 0.0096)].into_iter().enumerate() {
        m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(json!({"id": format!("o4trig:load:{i}"), "category": "concentrated_moment",
            "target": {"type": "node", "node": "o4trig:N1"}, "direction": dir, "magnitude": {"value": v, "unit": "N*m"}, "dimension": "moment", "provenance": p}));
    }
    raw
}

// ------------------------------------------------------------------ the private driver

/// One invocation through the actual W1 body: the observed ordinary run, then `retained_w1`.
struct Run {
    domain: Option<String>,
    quality: BTreeMap<String, String>,
    source_selected: bool,
    outcome: String,
    successor: Option<Value>,
    published: bool,
    causes: Option<Value>,
}

fn run(raw: &Value, mode: PreviewSolverMode) -> Run {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).expect("the request parses");
    let domain = retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, LIMIT).law().domain.as_ref().map(|r| format!("{r:?}"));
    let route = w1_route(&request.model).unwrap_or_default();
    let mut observer = retained_product::ProductCapture::prepared_probe_on(route);
    let mut ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut w1_budget(route), Some(&mut observer));
    ordinary.diagnostics.shrink_to_fit();
    let quality = ordinary.numerical_quality.cases.iter()
        .map(|c| (c.basis_ref.ref_id.clone(), format!("{:?}", c.solve_quality))).collect();
    let source_selected = ordinary.source_block_recovery.is_some();
    let ((_, retained), _, captured) = retained_tests_hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
    let (outcome, published) = match &retained {
        Ok(_) => ("published".to_owned(), true),
        Err(f) => (format!("{f:?}"), false),
    };
    let successor = match retained { Ok(s) => Some(s.value().clone()), Err(_) => captured };
    let causes = (!published && matches!(outcome.as_str(), "Preparation" | "Native" | "Candidate")).then(|| w1_causes(raw, mode));
    Run { domain, quality, source_selected, outcome, successor, published, causes }
}

/// The per-case first refusal of a W1 fallback before precommit (the census's stepped stages,
/// `b4_w1_census_loss_causes`): evidence only.
fn w1_causes(raw: &Value, mode: PreviewSolverMode) -> Value {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
    let route = w1_route(&request.model).unwrap_or_default();
    let mut observer = retained_product::ProductCapture::prepared_probe_on(route);
    let mut ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut w1_budget(route), Some(&mut observer));
    ordinary.diagnostics.shrink_to_fit();
    let Some(cases) = w1_case_ids(&capture) else { return json!("domain") };
    let triggers: Vec<CaseTrigger> = case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()).collect();
    let attempted = cases.attempted(triggers.iter().copied());
    let universe = row_universe(raw, &serde_json::to_value(&ordinary).unwrap());
    match observer.prepare_cases(ordinary, cases.ids().len(), attempted.requests()) {
        Err(failure) => json!({"custody": format!("{:?}", failure.error)}),
        Ok(mut prepared) => {
            prepared.native();
            prepared.freeze();
            let ends: Vec<Value> = (0..prepared.attempts.len()).map(|i| {
                let request = prepared.attempts[i].request;
                let end = match &prepared.attempts[i].end {
                    retained_product::AttemptEnd::Preparation => format!("Preparation({})", prepared.capture.with_case(request, |c| format!("{:?}", c.error))),
                    retained_product::AttemptEnd::Prepared => "Prepared".into(),
                    retained_product::AttemptEnd::Native => format!("Native({:?})", prepared.attempts[i].trace.native_error),
                    retained_product::AttemptEnd::Selected => "Selected".into(),
                    retained_product::AttemptEnd::Frozen(_) => "Frozen".into(),
                    retained_product::AttemptEnd::Candidate(refused) => format!("Candidate({:?})", refused.error),
                };
                // The failing row's identity: its index in the receipt row universe.
                let failing = end.split("Predicate { row: ").nth(1).and_then(|t| t.split(',').next()).and_then(|n| n.parse::<usize>().ok())
                    .and_then(|i| universe.get(i).cloned());
                let alpha = end.split("alpha_hi: ").nth(1).map(|t| t.chars().take(160).collect::<String>());
                json!({"request": request, "end": end.split(", work:").next().unwrap().chars().take(2000).collect::<String>(),
                    "failing_row": failing, "alpha_hi": alpha})
            }).collect();
            json!(ends)
        }
    }
}

/// The receipt row universe of case `case` in published order (the reader's `rows_for`): the
/// ordinary rows less the overlaid reduced twins of members whose two walls are bit-equal.
fn row_universe(raw: &Value, ordinary: &Value) -> Vec<Value> {
    let distinct: Vec<&str> = raw["model"]["pipe_segments"].as_array().unwrap().iter().filter(|p| {
        let s = &p["section"];
        let v = |k: &str| s.get(k).map_or(0.0, |q| q["value"].as_f64().unwrap() * to_m(q["unit"].as_str().unwrap()));
        let t = v("wall_thickness");
        ((t - v("mill_tolerance")) - v("corrosion_allowance")).to_bits() != t.to_bits()
    }).map(|p| p["id"].as_str().unwrap()).collect();
    ordinary["results"].as_array().unwrap().iter().filter(|r| r["basis_ref"]["ref_id"] == "case").filter(|r| {
        let reduced = !retained_product::derived_kind(r["kind"].as_str().unwrap())
            && r["metadata"]["basis"].as_str().is_some_and(|b| b.ends_with("; section_basis=reduced_section"));
        !reduced || distinct.contains(&r["entity_ref"].as_str().unwrap())
    }).map(|r| json!({"id": r["id"], "kind": r["kind"], "entity": r["entity_ref"], "location": r["metadata"]["location"], "value": r["value"]})).collect()
}

// ------------------------------------------------------------------ double-double arithmetic

/// A double-double: hi + lo, |lo| ≤ ulp(hi)/2 (about 106 bits). Used only to compare the product's
/// binary64 rows with the references' 40-digit decimals; every verdict keeps a 1e-6 margin.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Dd { hi: f64, lo: f64 }
fn two_sum(a: f64, b: f64) -> (f64, f64) { let s = a + b; let bb = s - a; (s, (a - (s - bb)) + (b - bb)) }
fn quick_two_sum(a: f64, b: f64) -> (f64, f64) { let s = a + b; (s, b - (s - a)) }
impl Dd {
    const ZERO: Dd = Dd { hi: 0.0, lo: 0.0 };
    fn from(x: f64) -> Dd { Dd { hi: x, lo: 0.0 } }
    fn add(self, o: Dd) -> Dd {
        let (s, e) = two_sum(self.hi, o.hi);
        let (t, f) = two_sum(self.lo, o.lo);
        let (s, e) = quick_two_sum(s, e + t);
        let (hi, lo) = quick_two_sum(s, e + f);
        Dd { hi, lo }
    }
    fn neg(self) -> Dd { Dd { hi: -self.hi, lo: -self.lo } }
    fn sub(self, o: Dd) -> Dd { self.add(o.neg()) }
    fn mul(self, o: Dd) -> Dd {
        let p = self.hi * o.hi;
        let e = self.hi.mul_add(o.hi, -p) + (self.hi * o.lo + self.lo * o.hi);
        let (hi, lo) = quick_two_sum(p, e);
        Dd { hi, lo }
    }
    fn div(self, o: Dd) -> Dd {
        let q1 = self.hi / o.hi;
        let r = self.sub(o.mul(Dd::from(q1)));
        let q2 = r.hi / o.hi;
        let r = r.sub(o.mul(Dd::from(q2)));
        let q3 = r.hi / o.hi;
        let (hi, lo) = quick_two_sum(q1, q2);
        Dd { hi, lo }.add(Dd::from(q3))
    }
    fn sqrt(self) -> Dd {
        if self.hi <= 0.0 { assert!(self.hi == 0.0 && self.lo == 0.0, "sqrt of a negative"); return Dd::ZERO; }
        let x = self.hi.sqrt();
        let r = self.sub(Dd::from(x).mul(Dd::from(x)));
        Dd::from(x).add(Dd::from(r.hi / (2.0 * x)))
    }
    fn abs(self) -> Dd { if self.hi < 0.0 || (self.hi == 0.0 && self.lo < 0.0) { self.neg() } else { self } }
    fn f(self) -> f64 { self.hi + self.lo }
    fn lt(self, o: Dd) -> bool { let d = self.sub(o); d.hi < 0.0 || (d.hi == 0.0 && d.lo < 0.0) }
    fn pow10(n: i32) -> Dd {
        let mut r = Dd::from(1.0);
        let mut k = n.unsigned_abs();
        while k > 0 { let step = k.min(22); r = r.mul(Dd::from(10f64.powi(step as i32))); k -= step; }
        if n < 0 { Dd::from(1.0).div(r) } else { r }
    }
    /// A reference decimal ("0", or "d.ddd…e±x" with up to 40 digits).
    fn parse(text: &str) -> Dd {
        let (neg, body) = match text.strip_prefix('-') { Some(b) => (true, b), None => (false, text) };
        let (mant, exp) = match body.split_once(['e', 'E']) { Some((m, e)) => (m, e.parse::<i32>().unwrap()), None => (body, 0) };
        let (int, frac) = mant.split_once('.').unwrap_or((mant, ""));
        let digits: String = format!("{int}{frac}");
        let mut acc = Dd::ZERO;
        for chunk in digits.as_bytes().chunks(15) {
            let c: u64 = std::str::from_utf8(chunk).unwrap().parse().unwrap();
            acc = acc.mul(Dd::from(10f64.powi(chunk.len() as i32))).add(Dd::from(c as f64));
        }
        let v = acc.mul(Dd::pow10(exp - frac.len() as i32));
        if neg { v.neg() } else { v }
    }
}

// ------------------------------------------------------------------ judging

/// A reference value of one readout: a point, or the admitted readout's represented interval.
#[derive(Clone, Copy, Debug)]
enum Exact { Point(Dd), Interval(Dd, Dd) }
impl Exact {
    fn of(v: &Value) -> Exact {
        if let Some(a) = v.as_array() { return Exact::Point(Dd::parse(a[0].as_str().unwrap())); }
        Exact::Interval(Dd::parse(v["lower"][0].as_str().unwrap()), Dd::parse(v["upper"][0].as_str().unwrap()))
    }
    fn scaled(self, k: Dd, divide: bool) -> Exact {
        let s = |d: Dd| if divide { d.div(k) } else { d.mul(k) };
        match self { Exact::Point(p) => Exact::Point(s(p)), Exact::Interval(a, b) => Exact::Interval(s(a), s(b)) }
    }
    /// The distance from x to the value or interval.
    fn distance(self, x: f64) -> Dd {
        let x = Dd::from(x);
        match self {
            Exact::Point(p) => x.sub(p).abs(),
            Exact::Interval(a, b) => if x.lt(a) { a.sub(x) } else if b.lt(x) { x.sub(b) } else { Dd::ZERO },
        }
    }
    fn is_zero(self) -> bool { matches!(self, Exact::Point(p) if p.hi == 0.0 && p.lo == 0.0) }
    fn mid(self) -> f64 { match self { Exact::Point(p) => p.f(), Exact::Interval(a, b) => 0.5 * (a.f() + b.f()) } }
}

/// The reference key a product row maps to (README "Rows"), or None for a row the references do not cover.
fn reference(row: &Value, readout: &Value) -> Option<Exact> {
    let kind = row["kind"].as_str()?;
    let entity = row["entity_ref"].as_str()?;
    let meta = &row["metadata"];
    let location = meta["location"].as_str().unwrap_or("");
    let basis = meta["basis"].as_str().and_then(|b| b.rsplit_once("section_basis=")).map(|(_, b)| b);
    let get = |path: &[&str]| -> Option<Exact> {
        let mut v = readout;
        for p in path { v = v.get(*p)?; }
        Some(Exact::of(v))
    };
    let end_or_station = |end: &str, station: &str| -> Option<Exact> {
        if matches!(location, "end_i" | "end_j") { get(&["member_end_force", entity, location, end]) }
        else { get(&["section_action", entity, location, station]) }
    };
    match kind {
        "global_nodal_displacement_x" => get(&["nodal", entity, "ux"]),
        "global_nodal_displacement_y" => get(&["nodal", entity, "uy"]),
        "global_nodal_displacement_z" => get(&["nodal", entity, "uz"]),
        "global_nodal_rotation_x" => get(&["nodal", entity, "rx"]),
        "global_nodal_rotation_y" => get(&["nodal", entity, "ry"]),
        "global_nodal_rotation_z" => get(&["nodal", entity, "rz"]),
        "displacement_magnitude" => readout.get("nodal")?.get(entity).and_then(|n| {
            let mut sum = Dd::ZERO;
            for c in ["ux", "uy", "uz"] { let Exact::Point(p) = Exact::of(n.get(c)?) else { return None }; sum = sum.add(p.mul(p)); }
            Some(Exact::Point(sum.sqrt()))
        }),
        "element_local_axial_force" => end_or_station("Fx", "N"),
        "element_local_shear_force_y" => end_or_station("Fy", "Vy"),
        "element_local_shear_force_z" => end_or_station("Fz", "Vz"),
        "element_local_torsional_moment" => end_or_station("Mx", "T"),
        "element_local_bending_moment_y" => end_or_station("My", "My"),
        "element_local_bending_moment_z" => end_or_station("Mz", "Mz"),
        "element_local_axial_normal_stress" => get(&["stress", entity, location, basis?, "axial"]),
        "element_local_bending_normal_stress_y" => get(&["stress", entity, location, basis?, "bending_y"]),
        "element_local_bending_normal_stress_z" => get(&["stress", entity, location, basis?, "bending_z"]),
        "element_local_torsional_shear_stress" => get(&["stress", entity, location, basis?, "torsion"]),
        "pipe_elastic_normal_stress_maximum_v2" => get(&["circular_maximum", entity, basis?]),
        "support_reaction_component_v2" => get(&["support_reaction", entity, meta["component"].as_str()?]),
        "support_reaction_force_magnitude_v2" => { let s = readout.get("support_reaction")?.get(entity)?; magnitude(s, ["Fx", "Fy", "Fz"]) }
        "support_reaction_moment_magnitude_v2" => { let s = readout.get("support_reaction")?.get(entity)?; magnitude(s, ["Mx", "My", "Mz"]) }
        _ => None,
    }
}
fn magnitude(v: &Value, names: [&str; 3]) -> Option<Exact> {
    let mut sum = Dd::ZERO;
    for n in names { let Exact::Point(p) = Exact::of(v.get(n)?) else { return None }; sum = sum.add(p.mul(p)); }
    Some(Exact::Point(sum.sqrt()))
}
/// SI value → published unit: (factor, divide).
fn unit_factor(unit: &str) -> (Dd, bool) {
    match unit { "mm" => (Dd::from(1000.0), false), "MPa" => (Dd::from(1.0e6), true), _ => (Dd::from(1.0), false) }
}
/// Rows whose exact values the references derive rather than state (norms of stated components).
fn derived_reference(kind: &str) -> bool {
    matches!(kind, "displacement_magnitude" | "support_reaction_force_magnitude_v2" | "support_reaction_moment_magnitude_v2")
}

/// One judged row against one readout.
#[derive(Default, Clone)]
struct Tally { rows: usize, worst: f64, worst_row: String, worst_sharper: f64, worst_raw: f64 }
impl Tally {
    fn note(&mut self, m: f64, row: &str) { self.rows += 1; if m > self.worst || self.worst_row.is_empty() { self.worst = self.worst.max(m); self.worst_row = row.to_owned(); } }
}

#[derive(Default)]
struct Judgement {
    tallies: BTreeMap<String, Tally>,
    not_covered: Vec<String>,
    input_derived: usize,
    unmapped: Vec<String>,
    /// Covered rows (any class but `not_covered` and `non_quantity`) and readout comparisons made.
    covered: usize,
    comparisons: usize,
    breaches: Vec<Value>,
    undecided: Vec<Value>,
    rows: Vec<Value>,
}

const UNDECIDED: f64 = 1e-6;

/// Comparison coverage of one selected successor: every covered row owned by the document is
/// classified and compared with BOTH readouts, and at least one comparison is made. A row the
/// references cannot place fails here rather than passing unjudged (repair round 1).
fn coverage_failures(label: &str, j: &Judgement) -> Vec<String> {
    let mut failures: Vec<String> = j.unmapped.iter().map(|u| format!("{label}: not compared: {u}")).collect();
    if j.comparisons == 0 {
        failures.push(format!("{label}: a selected successor yielded no reference comparison"));
    }
    if j.comparisons != 2 * j.covered {
        failures.push(format!("{label}: {} comparisons for {} covered rows (both readouts each)", j.comparisons, j.covered));
    }
    failures
}

/// Judges one selected case of a successor against both readouts of its reference case. `owned`
/// limits the judged rows to the document's own entities (the trigger's rows are not referenced).
fn judge(label: &str, successor: &Value, invocation: &Value, reference_case: &Value, case_id: &str, record: bool) -> Judgement {
    use open_pipe_stress_result_export::retained_precision::{validate, AccuracyClass};
    let verdict = validate(successor, Some(invocation)).unwrap_or_else(|e| panic!("{label}: the reader validates the published successor: {e:?}"));
    assert!(verdict.invocation_bound && verdict.numerical_eligible, "{label}: eligible");
    let classes: BTreeMap<&str, _> = verdict.classifications.iter().map(|c| (c.result_id.as_str(), c)).collect();
    let mut j = Judgement::default();
    let readouts = &reference_case["readouts"];
    for row in successor["results"].as_array().unwrap() {
        if row["basis_ref"]["ref_id"] != case_id { continue; }
        let id = row["id"].as_str().unwrap();
        let entity = row["entity_ref"].as_str().unwrap_or("");
        if entity.starts_with("o4trig:") { continue; }
        let Some(class) = classes.get(id) else { j.unmapped.push(format!("{id} (no class)")); continue };
        let kind = row["kind"].as_str().unwrap();
        match class.class {
            AccuracyClass::NonQuantity => continue,
            AccuracyClass::NotCovered => { j.not_covered.push(id.to_owned()); continue; }
            _ => {}
        }
        j.covered += 1;
        let n = f64::from_bits(class.normalized_bits);
        let y = row["value"].as_f64().unwrap();
        let unit = row["unit"].as_str().unwrap();
        let mut entry = json!({"id": id, "kind": kind, "unit": unit, "value": y, "n": n, "class": format!("{:?}", class.class),
            "entity": entity, "location": row["metadata"]["location"], "basis": row["metadata"]["basis"], "component": row["metadata"]["component"],
            "scale": class.scale_bits.map(f64::from_bits)});
        for readout in ["annular", "admitted"] {
            let Some(exact) = reference(row, &readouts[readout]) else { j.unmapped.push(format!("{id} ({readout})")); continue };
            j.comparisons += 1;
            let h = exact.distance(n).f();
            let (k, div) = unit_factor(unit);
            let h_raw = exact.scaled(k, div).distance(y).f();
            let key_class;
            let (pass, margin, undecided) = match class.class {
                AccuracyClass::InputDerived => {
                    key_class = "input_derived";
                    j.input_derived += 1;
                    (n == 0.0 && exact.is_zero(), if h == 0.0 { 0.0 } else { f64::INFINITY }, false)
                }
                AccuracyClass::AbsoluteVerified { bound_bits } => {
                    key_class = "absolute_verified";
                    let b = f64::from_bits(bound_bits);
                    let m = if b > 0.0 { h / b } else if h == 0.0 { 0.0 } else { f64::INFINITY };
                    (m <= 1.0, m, (m - 1.0).abs() < UNDECIDED)
                }
                AccuracyClass::RelativeVerified => {
                    key_class = "relative_verified";
                    let s = class.scale_bits.map(f64::from_bits).unwrap_or(0.0);
                    let tol = 1e-9 * n.abs();
                    let m = h / tol;
                    let m_raw = h_raw / (1e-9 * y.abs());
                    let allowance = n.abs().max(s) * (2f64.powi(-64) + 2f64.powi(-85)) + 2f64.powi(-53) * n.abs() + f64::from_bits(1);
                    let m_sharp = h / allowance;
                    let t = j.tallies.entry(format!("relative_verified/{readout}")).or_default();
                    t.worst_sharper = t.worst_sharper.max(m_sharp);
                    t.worst_raw = t.worst_raw.max(m_raw);
                    let worst = m.max(m_raw).max(m_sharp);
                    (worst <= 1.0, m, (m - 1.0).abs() < UNDECIDED || (m_raw - 1.0).abs() < UNDECIDED || (m_sharp - 1.0).abs() < UNDECIDED)
                }
                _ => unreachable!(),
            };
            let tag = if derived_reference(kind) { format!("{key_class}/{readout}/derived_norm") } else { format!("{key_class}/{readout}") };
            j.tallies.entry(tag).or_default().note(margin, &format!("{label}:{id}"));
            entry[readout] = json!({"exact_mid": exact.mid(), "distance": h, "distance_raw": h_raw, "margin": margin});
            if undecided {
                j.undecided.push(json!({"label": label, "id": id, "readout": readout, "margin": margin}));
            } else if !pass {
                j.breaches.push(json!({"label": label, "id": id, "kind": kind, "readout": readout, "class": format!("{:?}", class.class),
                    "value": y, "n": n, "exact": exact.mid(), "distance": h, "margin": margin}));
            }
        }
        if record { j.rows.push(entry); }
    }
    j
}

// ------------------------------------------------------------------ the premise census (receipt facts)

fn hexf(v: &Value) -> Option<f64> { v.as_str().and_then(|s| u64::from_str_radix(s, 16).ok()).map(f64::from_bits) }

/// R7's native attempts, the selected and verification precisions, the p = 512 floor, θ, B_b and
/// the charge and estimate shares per body, from a selected case's receipt.
fn premise(successor: &Value, case_index: usize) -> Value {
    let body = &successor["retained_precision"]["body"];
    let case = &body["cases"][case_index];
    let sel = &case["selection"];
    let attempts: Vec<Value> = case["run"]["attempts"].as_array().map(|a| a.iter().map(|x| json!({
        "precision": x["precision"], "outcome": x["outcome"], "verification": x["verification"], "case_charge": x["case_charge"]})).collect()).unwrap_or_default();
    let per = |key: &str| -> Vec<Value> {
        sel[key].as_array().map(|a| a.iter().map(|e| {
            let mut e = e.clone();
            for f in ["value", "force", "moment", "translation", "rotation"] {
                if let Some(v) = hexf(&e[f]) { e[f] = json!(v); }
            }
            e
        }).collect()).unwrap_or_default()
    };
    json!({"status": case["status"], "precision": sel["precision"], "verification_precision": sel["verification_precision"],
        "floor": sel["floor"], "floor_active": !sel["floor"].is_null(), "theta": per("theta"), "certified_bound": per("certified_bound"),
        "verification_charge": per("verification_charge"), "verification_estimate": per("verification_estimate"),
        "body_scales": per("body_scales"), "attempts": attempts, "case_charge": case["run"]["case_charge"],
        "rcond": hexf(&sel["rcond"]), "pivot_margin_min": hexf(&sel["pivot_margin_min"])})
}

// ------------------------------------------------------------------ the binding test body

/// The judged documents of one set: (id, request, reference case, theory, variant, design).
struct Doc { id: String, design: String, theory: String, variant: String, raw: Value, reference: &'static Value, twin: bool }

fn judged_docs() -> Vec<Doc> {
    let f = frozen();
    f.cases["cases"].as_array().unwrap().iter().map(|c| {
        let id = c["id"].as_str().unwrap().to_owned();
        Doc { raw: judged_request(&c["document"], &format!("invented:o4-m3:{id}")), reference: &f.oracle["cases"][&id], design: c["design"].as_str().unwrap().into(),
            theory: c["theory"].as_str().unwrap().into(), variant: c["variant"].as_str().unwrap().into(), id, twin: false }
    }).collect()
}
fn twin_docs() -> Vec<Doc> {
    let f = frozen();
    f.twins["cases"].as_array().unwrap().iter().map(|c| {
        let id = c["id"].as_str().unwrap().to_owned();
        Doc { raw: twin_request(c), reference: &f.twin_oracle["cases"][&id], design: c["design"].as_str().unwrap().into(),
            theory: c["theory"].as_str().unwrap().into(), variant: c["variant"].as_str().unwrap().into(), id, twin: true }
    }).collect()
}

/// The augmented request of a document: the frozen case document plus the trigger body.
fn augmented(doc: &Doc) -> Option<Value> {
    let f = frozen();
    let source = if doc.twin { &f.twins } else { &f.cases };
    let c = source["cases"].as_array().unwrap().iter().find(|c| c["id"] == doc.id.as_str()).unwrap();
    let d = &c["document"];
    let over = |k: &str| d[k].as_array().unwrap().len() + if k == "supports" { 4 } else if k == "nodes" { 2 } else { 1 };
    // D1's caps: 32 nodes, 32 members, 32 supports.
    if over("nodes") > 32 || over("members") > 32 || over("supports") > 32 { return None; }
    Some(augment(&judged_request(d, &format!("invented:o4-m3-aug:{}", doc.id))))
}

#[derive(Default)]
struct Summary { runs: usize, published: usize, breaches: Vec<Value>, undecided: Vec<Value>, incomplete: Vec<String>, tallies: BTreeMap<String, Tally> }

/// Runs one document in both modes, plain and (optionally) augmented, judging every published
/// successor; writes one record per run under `O4_M3_OUT`.
fn bind(doc: &Doc, augmented_too: bool, summary: &mut Summary) {
    let out = std::env::var_os("O4_M3_OUT").map(std::path::PathBuf::from);
    let mut variants = vec![("plain", doc.raw.clone())];
    if augmented_too {
        match augmented(doc) {
            Some(raw) => variants.push(("augmented", raw)),
            None => if let Some(dir) = &out {
                std::fs::write(dir.join(format!("{}+augmented+skipped.json", doc.id.replace('|', "+"))),
                    serde_json::to_vec(&json!({"id": doc.id, "binding": "augmented", "skipped": "beyond D1's caps with the trigger body"})).unwrap()).unwrap();
            },
        }
    }
    for (binding, raw) in variants {
        for mode in MODES {
            let label = format!("{}|{binding}|{}", doc.id, mode.as_str());
            let r = run(&raw, mode);
            summary.runs += 1;
            let mut record = json!({"id": doc.id, "design": doc.design, "theory": doc.theory, "variant": doc.variant, "twin": doc.twin,
                "binding": binding, "mode": mode.as_str(), "domain": r.domain, "quality": r.quality, "source_selected": r.source_selected,
                "outcome": r.outcome, "published": r.published, "causes": r.causes});
            if r.published {
                let successor = r.successor.as_ref().unwrap();
                summary.published += 1;
                let body = &successor["retained_precision"]["body"];
                let index = body["cases"].as_array().unwrap().iter().position(|c| c["basis_ref"]["ref_id"] == "case").expect("case");
                let status = body["cases"][index]["status"].as_str().unwrap().to_owned();
                record["case_status"] = json!(status);
                record["premise"] = premise(successor, index);
                if status == "selected" {
                    let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
                    let j = judge(&label, successor, &invocation, doc.reference, "case", out.is_some());
                    for (k, t) in &j.tallies {
                        let s = summary.tallies.entry(k.clone()).or_default();
                        s.rows += t.rows;
                        if t.worst > s.worst || s.worst_row.is_empty() { s.worst = s.worst.max(t.worst); s.worst_row = t.worst_row.clone(); }
                        s.worst_sharper = s.worst_sharper.max(t.worst_sharper);
                        s.worst_raw = s.worst_raw.max(t.worst_raw);
                    }
                    record["judged"] = json!({"tallies": j.tallies.iter().map(|(k, t)| (k.clone(), json!({"rows": t.rows, "worst": t.worst, "worst_row": t.worst_row,
                        "worst_sharper": t.worst_sharper, "worst_raw": t.worst_raw}))).collect::<BTreeMap<_, _>>(),
                        "not_covered": j.not_covered.len(), "input_derived": j.input_derived, "covered": j.covered, "comparisons": j.comparisons,
                        "unmapped": j.unmapped, "breaches": j.breaches, "undecided": j.undecided, "rows": j.rows});
                    summary.incomplete.extend(coverage_failures(&label, &j));
                    summary.breaches.extend(j.breaches);
                    summary.undecided.extend(j.undecided);
                }
            }
            if let Some(dir) = &out {
                std::fs::write(dir.join(format!("{}.json", label.replace('|', "+"))), serde_json::to_vec(&record).unwrap()).unwrap();
            }
            println!("O4_M3 {label} domain={:?} quality={:?} source_selected={} w1={}", r.domain, r.quality, r.source_selected, r.outcome);
        }
    }
}

fn finish(name: &str, summary: Summary, expect_published: Option<usize>) {
    for (k, t) in &summary.tallies {
        println!("O4_M3_TALLY {name} {k} rows={} worst={:.3e} ({}) sharper={:.3e} raw={:.3e}", t.rows, t.worst, t.worst_row, t.worst_sharper, t.worst_raw);
    }
    println!("O4_M3_SUMMARY {name} runs={} published={} breaches={} undecided={} incomplete={}", summary.runs, summary.published, summary.breaches.len(),
        summary.undecided.len(), summary.incomplete.len());
    assert!(summary.incomplete.is_empty(), "{name}: published rows not compared with both readouts: {:#?}", summary.incomplete);
    assert!(summary.breaches.is_empty(), "{name}: judged breaches on published rows (design §5: stop and return to HELP_HUMAN): {:#?}", summary.breaches);
    assert!(summary.undecided.is_empty(), "{name}: verdicts within 1e-6 of their boundary need the exact judge: {:#?}", summary.undecided);
    if let Some(n) = expect_published { assert_eq!(summary.published, n, "{name}: published W1-E successors"); }
}

fn bind_designs(name: &str, designs: &[&str], twins: bool, augmented_too: bool) -> Summary {
    let docs = if twins { twin_docs() } else { judged_docs() };
    let mut summary = Summary::default();
    let mut matched = 0;
    // `O4_M3_ONLY=eb|zero` restricts a run to one theory and allowance class (the main-versus-head
    // comparison of the Euler-Bernoulli zero-allowance harness twins).
    let only = std::env::var("O4_M3_ONLY").ok();
    for doc in docs.iter().filter(|d| designs.iter().any(|p| d.design.starts_with(p)))
        .filter(|d| only.as_deref().is_none_or(|o| o == format!("{}|{}", d.theory, d.variant))) {
        matched += 1;
        bind(doc, augmented_too, &mut summary);
    }
    assert!(matched > 0, "{name}: no document matched {designs:?}");
    summary
}

/// The fast subset (un-ignored): GUIDED (Euler-Bernoulli and Timoshenko, zero, both and mixed
/// allowances) and J2 n05's eight source twins, plain and augmented, both modes. Today these give
/// at least 20 published W1-E successors (GUIDED zero in both theories and every n05 twin when
/// augmented), judged against both readouts; the long groups below cover the rest.
#[test]
fn o4_m3_w1e_fast_subset_judged_against_both_readouts() {
    let mut summary = bind_designs("fast_guided", &["GUIDED"], false, true);
    let twins = bind_designs("fast_n05_twins", &["J2-physics_source-n05|"], true, true);
    summary.runs += twins.runs;
    summary.published += twins.published;
    summary.breaches.extend(twins.breaches);
    summary.undecided.extend(twins.undecided);
    summary.incomplete.extend(twins.incomplete);
    for (k, t) in twins.tallies { summary.tallies.entry(k).or_default().rows += t.rows; }
    let published = summary.published;
    finish("fast_subset", summary, None);
    assert!(published >= 20, "fast subset: {published} published W1-E successors (coverage fell)");
}

/// AC-006 for the sharper-allowance decline (DEL-04-08 Design `w1e-sharper-exact-coverage.md`
/// §6.1, chosen from the census of §6.3): `SOFT-RUN-r1e-08|zero|eb`, a sensitive judged document,
/// reaches the product's final gate on its own trigger in both modes and is refused there with
/// `Cause::Predicate` on SharperExact. Its actual Direct entry then publishes the ordinary bytes
/// with the one `RETAINED_PRECISION_UNAVAILABLE` notice in the registered build, and the ordinary
/// bytes alone in any other build.
#[test]
fn o4_w1e_sharper_allowance_refusal_keeps_the_ordinary_result() {
    let doc = judged_docs().into_iter().find(|d| d.id == "SOFT-RUN-r1e-08|zero|eb").unwrap();
    for mode in MODES {
        let run = run(&doc.raw, mode);
        assert!(run.domain.is_none() && !run.source_selected, "{mode:?}: inside D1, W1's route");
        assert_eq!(run.outcome, "Candidate", "{mode:?}");
        let causes = run.causes.expect("the per-case causes");
        let ends = causes.as_array().unwrap();
        assert_eq!(ends.len(), 1, "{mode:?}: one attempted case");
        let end = ends[0]["end"].as_str().unwrap();
        assert!(end.starts_with("Candidate(") && end.contains("Predicate {") && end.contains("SharperExact"),
            "{mode:?}: refused at the final gate on SharperExact: {end}");
        let plain: Value = serde_json::to_value(run_linear_static_preview_value_with_mode(doc.raw.clone(), mode).unwrap()).unwrap();
        let output = run_linear_static_preview_value_with_retained_direct(doc.raw.clone(), mode, LIMIT).unwrap();
        let mut published = match output.into_publication() {
            RetainedPublication::Successor(_) => panic!("{mode:?}: no successor"),
            RetainedPublication::Ordinary(envelope) => serde_json::to_value(envelope).unwrap(),
        };
        if super::retained_facade_tests::registered() {
            let diagnostics = published["diagnostics"].as_array_mut().unwrap();
            let notice = diagnostics.pop().unwrap();
            assert_eq!(notice["code"], "RETAINED_PRECISION_UNAVAILABLE", "{mode:?}: the notice follows the ordinary diagnostics");
            assert!(diagnostics.iter().all(|d| d["code"] != "RETAINED_PRECISION_UNAVAILABLE"), "{mode:?}: exactly one notice");
        }
        assert_eq!(published, plain, "{mode:?}: the ordinary result");
    }
}

/// Repair round 1 (Codex's probe, kept as a permanent negative control): on a real selected
/// successor (GUIDED zero, Euler-Bernoulli, augmented, sparse), the judge must refuse a reference
/// with both readouts removed (every row not compared, and no comparison at all), with one readout
/// removed, and with one reference value corrupted (a breach); and pass the true reference.
#[test]
fn o4_m3_judge_refuses_missing_or_corrupt_references() {
    let doc = judged_docs().into_iter().find(|d| d.id == "GUIDED|zero|eb").unwrap();
    let raw = augmented(&doc).unwrap();
    let mode = PreviewSolverMode::SparseInteractive;
    let r = run(&raw, mode);
    assert!(r.published, "the control publishes: {}", r.outcome);
    let successor = r.successor.unwrap();
    let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
    let judged = |reference: &Value| {
        let j = judge("negative-control", &successor, &invocation, reference, "case", false);
        (coverage_failures("negative-control", &j), j.breaches.len(), j.comparisons, j.covered)
    };
    let (failures, breaches, comparisons, covered) = judged(doc.reference);
    assert!(failures.is_empty() && breaches == 0 && covered > 0 && comparisons == 2 * covered, "true reference: {failures:?} {breaches}");
    let mut none = doc.reference.clone();
    none["readouts"]["annular"] = json!({});
    none["readouts"]["admitted"] = json!({});
    let (failures, _, comparisons, covered) = judged(&none);
    assert_eq!(comparisons, 0);
    assert!(failures.iter().any(|f| f.contains("no reference comparison")), "zero comparisons refused");
    assert_eq!(failures.iter().filter(|f| f.contains("not compared")).count(), 2 * covered, "every row, both readouts");
    let mut one = doc.reference.clone();
    one["readouts"]["admitted"] = json!({});
    let (failures, _, comparisons, covered) = judged(&one);
    assert_eq!(comparisons, covered);
    assert!(!failures.is_empty(), "one readout missing refused");
    let mut corrupt = doc.reference.clone();
    corrupt["readouts"]["annular"]["nodal"]["N1"]["uy"] = json!(["1", 1.0]);
    let (failures, breaches, _, _) = judged(&corrupt);
    assert!(failures.is_empty() && breaches > 0, "a corrupted reference value is a breach");
}

/// The augmentation's trigger body publishes alone in the judged set's materials and both shear
/// models, so a fallback of an augmented document is the document's own (its failing row is named
/// in the records). With nu = 1/8 or 1/32 the trigger alone can withdraw at G5b (printed only).
#[test]
fn o4_m3_trigger_body_publishes_alone() {
    for (e, nu, required) in [(2.0e11, 0.25, true), (2.0e11, 0.3, true), (1.93e11, 0.31, true), (2.0e11, 0.125, false), (2.0e11, 0.03125, false)] {
        for theory in ["euler_bernoulli", "timoshenko_annulus_energy_v1"] {
            let doc = json!({"shear_model": theory, "material": {"id": "steel", "elastic_modulus_Pa": e, "poisson_ratio": nu},
                "nodes": [], "members": [], "supports": [], "loads": [], "pressure_regions": []});
            let raw = augment(&judged_request(&doc, "invented:o4-m3-trigger"));
            for mode in MODES {
                let r = run(&raw, mode);
                println!("O4_M3_TRIGGER e={e} nu={nu} {theory} {} quality={:?} w1={}", mode.as_str(), r.quality, r.outcome);
                if required { assert!(r.published && r.domain.is_none(), "trigger alone, E={e} nu={nu} {theory} {mode:?}: {}", r.outcome); }
            }
        }
    }
}

macro_rules! judged {
    ($($name:ident: [$($design:literal),+], $twins:expr;)+) => {$(
        /// M3-O4 binding and judging of one group of the frozen judged set or source twins, plain
        /// and augmented, both modes (long: `#[ignore]`).
        #[test]
        #[ignore = "long: the M3-O4 binding and judging of one design group (run with --ignored)"]
        fn $name() {
            let summary = bind_designs(stringify!($name), &[$($design),+], $twins, true);
            finish(stringify!($name), summary, None);
        }
    )+};
}
judged! {
    o4_m3_cantilevers: ["CANT-"], false;
    o4_m3_phi2: ["PROP-PHI2-"], false;
    o4_m3_fixed_runs: ["FF-"], false;
    o4_m3_lines: ["LLINE-", "ZLINE"], false;
    o4_m3_portal_guided_beam: ["UPORTAL", "GUIDED", "CONT-BEAM"], false;
    o4_m3_skew: ["SKEW345-", "SKEW122-"], false;
    o4_m3_rflike: ["RFLIKE-"], false;
    o4_m3_soft_run: ["SOFT-RUN-"], false;
    o4_m3_soft_torsion: ["SOFT-TORSION-"], false;
    o4_m3_contrast_short: ["CONTRAST-STIFF", "SHORT-MEMBER"], false;
    o4_m3_caps: ["CAPS-LOOP-32"], false;
    o4_m3_twins_j2_physics: ["J2-physics_source-"], true;
    o4_m3_twins_j2_load_reference: ["J2-load_reference_source-"], true;
    o4_m3_twins_rf: ["RF-"], true;
}

/// The binding's own checks: every frozen file is bound by SHA-256, every document builds and
/// parses as a product request, the source twins re-author their pinned requests with the frozen
/// allowances, and the double-double reader reproduces each reference value's binary64.
#[test]
fn o4_m3_binding_reproduces_the_frozen_documents() {
    let f = frozen();
    let (docs, twins) = (judged_docs(), twin_docs());
    assert_eq!((docs.len(), twins.len()), (172, 104));
    for d in docs.iter().chain(&twins) {
        assert!(d.reference.is_object(), "{}: its reference case", d.id);
        for mode in MODES { source_receipt::CapturedInvocation::parse(d.raw.clone(), mode).unwrap_or_else(|e| panic!("{}: {e:?}", d.id)); }
        if let Some(a) = augmented(d) { source_receipt::CapturedInvocation::parse(a, MODES[0]).unwrap_or_else(|e| panic!("{} augmented: {e:?}", d.id)); }
    }
    let mut checked = 0;
    for oracle in [&f.oracle, &f.twin_oracle] {
        for (_, case) in oracle["cases"].as_object().unwrap() {
            let mut stack = vec![&case["readouts"]];
            while let Some(v) = stack.pop() {
                match v {
                    Value::Array(a) if a.len() == 2 && a[0].is_string() && a[1].is_number() => {
                        assert_eq!(Dd::parse(a[0].as_str().unwrap()).f().to_bits(), a[1].as_f64().unwrap().to_bits(), "{a:?}");
                        checked += 1;
                    }
                    Value::Array(a) => stack.extend(a),
                    Value::Object(o) => stack.extend(o.values()),
                    _ => {}
                }
            }
        }
    }
    println!("O4_M3_BINDING documents={} twins={} reference_values_checked={checked}", docs.len(), twins.len());
    assert_eq!(checked, 111_636 + 24_944, "every written readout value");
}

// ------------------------------------------------------------------ Agent 0's census: the source work limit

/// Significant bits of a binary64's significand (53 less its trailing zero bits; 0 for zero).
fn significant_bits(x: f64) -> u32 {
    if x == 0.0 { return 0; }
    let bits = x.to_bits();
    let (e, m) = ((bits >> 52) & 0x7ff, bits & ((1u64 << 52) - 1));
    let m = if e == 0 { m } else { m | (1u64 << 52) };
    64 - m.leading_zeros() - m.trailing_zeros()
}
/// x rounded to `bits` significant bits (a short, dyadic significand).
fn dyadic(x: f64, bits: i32) -> f64 {
    if x == 0.0 { return 0.0; }
    let e = x.abs().log2().floor() as i32;
    let q = 2f64.powi(e - bits + 1);
    (x / q).round() * q
}
/// x rounded to `digits` significant decimal digits (an ordinary authored decimal).
fn decimal(x: f64, digits: usize) -> f64 { format!("{:.*e}", digits - 1, x).parse().unwrap() }
fn annular_phi(d: f64, t: f64, nu: f64, l: f64) -> f64 {
    let ro = d / 2.0;
    let m = (ro - t) / ro;
    let dk = (8.0 * nu * nu + 14.0 * nu + 7.0) * (1.0 - m * m).powi(2) + 48.0 * m * m * (1.0 + nu).powi(2);
    ro * ro * dk / ((1.0 + nu) * (1.0 + m * m) * l * l)
}

/// One generated census document (README-agnostic form) and its inputs' description.
fn census_document(layout: &str, section: (f64, f64), length: f64, ratio: f64, theory: &str, style: &str, allowance: &str) -> (Value, Value) {
    let dyad = style == "dyadic";
    let (mut d, mut t) = section;
    let (mut e, mut nu) = (2.03e11, 0.3);
    let mut l = length;
    if dyad { d = dyadic(d, 6); t = dyadic(t, 6); e = 1.5 * 2f64.powi(37); nu = 0.3125; l = dyadic(l, 8); }
    let ro = d / 2.0;
    let ri = ro - t;
    let g = e / (2.0 * (1.0 + nu));
    let j = std::f64::consts::PI / 2.0 * (ro.powi(4) - ri.powi(4));
    let k_exact = ratio * g * j / l;
    let k = if dyad { 2f64.powi(k_exact.log2().round() as i32) } else { decimal(k_exact, 3) };
    let (m1, m2) = if dyad { (2f64.powi((k.log2() - 4.0) as i32), 2f64.powi((k.log2() - 3.0) as i32)) } else { (decimal(0.0617 * k, 3), decimal(0.1234 * k, 3)) };
    let (cm, ca) = if allowance == "both" { (0.125 * t, if t < 0.006 { 0.25 * t } else { 0.0015 }) } else { (0.0, 0.0) };
    let member = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "outside_diameter_m": d, "wall_thickness_m": t,
        "mill_tolerance_m": cm, "corrosion_allowance_m": ca, "y_reference": [0.0, 0.0, 1.0]});
    let rigid = |id: &str, node: &str, r: &[&str]| json!({"id": id, "node": node, "kind": "rigid", "restraints": r});
    let spring = |id: &str, node: &str, dof: &str| json!({"id": id, "node": node, "kind": "spring", "dof": dof, "stiffness": k, "unit": "N*m/rad"});
    let moment = |id: &str, node: &str, axis: &str, v: f64| json!({"id": id, "node": node, "kind": "moment", "direction": axis, "value": v, "unit": "N*m"});
    let node = |id: &str, x: f64, y: f64, z: f64| json!({"id": id, "xyz_m": [x, y, z]});
    let (nodes, members, supports, loads, member_count) = match layout {
        // RF PIN-OFF-345's layout: one (3,4,0) member, translations pinned at both ends, a soft RX spring at N0.
        "rf345" => (vec![node("N0", 0.0, 0.0, 0.0), node("N1", 0.6 * l, 0.8 * l, 0.0)], vec![member("M1", "N0", "N1")],
            vec![rigid("rigid:N0", "N0", &["UX", "UY", "UZ"]), spring("spring:N0", "N0", "RX"), rigid("rigid:N1", "N1", &["UX", "UY", "UZ"])],
            vec![moment("l0", "N1", "x", m1), moment("l1", "N1", "y", m2)], 1),
        // The declared-problem oracle's Z-hinge on a (1,2,2) member.
        "zhinge122" => (vec![node("N0", 0.0, 0.0, 0.0), node("N1", l / 3.0, 2.0 * l / 3.0, 2.0 * l / 3.0)], vec![member("M1", "N0", "N1")],
            vec![rigid("rigid:N0", "N0", &["UX", "UY", "UZ", "RX", "RY"]), spring("spring:N0", "N0", "RZ"), rigid("rigid:N1", "N1", &["UZ", "RX", "RY"])],
            vec![moment("l0", "N1", "z", m1)], 1),
        // A two-member run along (3,4,0): translations pinned at the ends and the middle, a soft RX spring at N0.
        "run2" => (vec![node("N0", 0.0, 0.0, 0.0), node("N1", 0.6 * l, 0.8 * l, 0.0), node("N2", 1.2 * l, 1.6 * l, 0.0)],
            vec![member("M1", "N0", "N1"), member("M2", "N1", "N2")],
            vec![rigid("rigid:N0", "N0", &["UX", "UY", "UZ"]), spring("spring:N0", "N0", "RX"), rigid("rigid:N1", "N1", &["UX", "UY", "UZ"]), rigid("rigid:N2", "N2", &["UX", "UY", "UZ"])],
            vec![moment("l0", "N2", "x", m1), moment("l1", "N2", "y", m2)], 2),
        // A planar L frame, (3,4,0) then (-4,3,0), translations pinned at the three nodes, a soft RX spring at N0.
        "lframe" => (vec![node("N0", 0.0, 0.0, 0.0), node("N1", 0.6 * l, 0.8 * l, 0.0), node("N2", 0.6 * l - 0.8 * l, 0.8 * l + 0.6 * l, 0.0)],
            vec![member("M1", "N0", "N1"), member("M2", "N1", "N2")],
            vec![rigid("rigid:N0", "N0", &["UX", "UY", "UZ"]), spring("spring:N0", "N0", "RX"), rigid("rigid:N1", "N1", &["UX", "UY", "UZ"]), rigid("rigid:N2", "N2", &["UX", "UY", "UZ"])],
            vec![moment("l0", "N2", "x", m1), moment("l1", "N2", "y", m2)], 2),
        other => panic!("layout {other}"),
    };
    let doc = json!({"shear_model": theory_token(theory), "material": {"id": "steel", "elastic_modulus_Pa": e, "poisson_ratio": nu},
        "nodes": nodes, "members": members, "supports": supports, "loads": loads, "pressure_regions": []});
    let inputs = [d, t, e, nu, k, m1, m2, l, 0.6 * l, 0.8 * l];
    let info = json!({"layout": layout, "D": d, "t": t, "L": l, "E": e, "nu": nu, "k": k, "ratio": ratio, "theory": theory, "style": style,
        "allowance": allowance, "members": member_count, "phi_annular": annular_phi(d, t, nu, l),
        "input_bits": inputs.iter().map(|x| significant_bits(*x)).sum::<u32>(), "max_input_bits": inputs.iter().map(|x| significant_bits(*x)).max()});
    (doc, info)
}

/// The ordinary exact run with the source route (captured), and what it charged.
fn source_outcome(raw: &Value, mode: PreviewSolverMode) -> Value {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).expect("parses");
    let route = w1_route(&request.model).unwrap_or_default();
    let mut budget = w1_budget(route);
    let envelope = run_linear_static_preview_observed(request, mode, Some(&capture), &mut budget, None);
    let quality: Vec<String> = envelope.numerical_quality.cases.iter().map(|c| format!("{:?}", c.solve_quality)).collect();
    let receipt = envelope.source_block_recovery.as_ref();
    let selected_charge = receipt.and_then(|r| r["body"]["cases"].as_array()).and_then(|c| c.first()).map(|c| c["work"]["charged"].clone());
    let diagnostics: Vec<Value> = envelope.diagnostics.iter().filter(|d| d.code.starts_with("SOURCE_BLOCK_RECOVERY") || d.code.contains("INTEGRITY"))
        .map(|d| json!({"code": d.code, "message": d.message.chars().take(1500).collect::<String>()})).collect();
    json!({"mode": mode.as_str(), "quality": quality, "selected": receipt.is_some(), "selected_case_charge": selected_charge,
        "invocation_work": receipt.map(|r| r["body"]["invocation_work"].clone()),
        "budget": {"charged": budget.charged, "failed_charged": budget.failed_charged, "publication_charged": budget.publication_charged,
            "rejected": budget.rejected, "attempts": budget.attempts, "per_case_limit": budget.per_case_limit},
        "diagnostics": diagnostics})
}

fn census_sweep(name: &str, layouts: &[&str]) {
    // Ordinary authored decimal sections (invented; no standards table).
    let sections = [(0.168, 0.007), (0.219, 0.0082), (0.324, 0.0095), (0.114, 0.006), (0.273, 0.0093)];
    let out = std::env::var_os("O4_CENSUS_OUT").map(std::path::PathBuf::from);
    let mut rows = Vec::new();
    for layout in layouts {
        for &section in &sections {
            // phi = 2's length, rounded to an authored millimetre; and two ordinary lengths.
            let nu = 0.3;
            let lstar = (annular_phi(section.0, section.1, nu, 1.0) / 2.0).sqrt();
            for (lname, length) in [("near_phi2_mm", decimal_mm(lstar)), ("short", 0.75), ("long", 2.5)] {
                for ratio in [1e-8, 1e-12] {
                    for theory in ["eb", "timo"] {
                        for style in ["decimal", "dyadic"] {
                            for allowance in ["zero", "both"] {
                                let (doc, info) = census_document(layout, section, length, ratio, theory, style, allowance);
                                let raw = judged_request(&doc, &format!("invented:o4-census:{layout}:{lname}"));
                                let mode = PreviewSolverMode::SparseInteractive;
                                let outcome = source_outcome(&raw, mode);
                                println!("O4_CENSUS {name} {layout} {section:?} {lname} r={ratio:e} {theory} {style} {allowance} selected={} charge={}",
                                    outcome["selected"], outcome["selected_case_charge"]);
                                rows.push(json!({"length_class": lname, "info": info, "outcome": outcome}));
                            }
                        }
                    }
                }
            }
        }
    }
    if let Some(dir) = out { std::fs::write(dir.join(format!("{name}.json")), serde_json::to_vec(&rows).unwrap()).unwrap(); }
}
fn decimal_mm(x: f64) -> f64 { (x * 1000.0).round() / 1000.0 }

/// Agent 0's census (measurement only; the 8,000,000 source work limit is unchanged): the source
/// route's case charge, selection and fallback on representative sensitive layouts with ordinary
/// decimal inputs and their dyadic-rounded twins, both shear models (long: `#[ignore]`).
#[test]
#[ignore = "census: source work limit on single members (run with --ignored; O4_CENSUS_OUT)"]
fn o4_census_source_work_single_members() { census_sweep("single_members", &["rf345", "zhinge122"]); }
#[test]
#[ignore = "census: source work limit on small frames (run with --ignored; O4_CENSUS_OUT)"]
fn o4_census_source_work_small_frames() { census_sweep("small_frames", &["run2", "lframe"]); }

// W1-E's product-side confirmation census (DEL-04-08 Design `w1e-sharper-exact-coverage.md`
// §6.2): opt-in, `--features census-probe` only.
#[cfg(feature = "census-probe")]
#[path = "w1e_census_tests.rs"]
mod w1e_census_tests;

/// R-A's gating test on the Z-hinge inputs (DEL-04-08 Design `source-work-nondyadic-inputs.md`
/// §3.2): over a subset of O4's single-member census sweep (both sections' extremes, every
/// length, both shear models, both styles), every row equals the long evaluation's bit for bit,
/// and the coverage assertion holds: all 22 negated rows of the member take B1 (components 0–3
/// at every station, 4 and 5 at the i end), exact-zero rows included, and no other row does.
#[test]
fn ra_gating_zhinge_negated_rows_take_b1() {
    use crate::source_recovery::j7_rf_census::ra_gating::gate;
    use open_pipe_stress_frame_kernel::structural::exact_boundary::exact_operation_count::RowPath;
    let mut gated = 0;
    for section in [(0.168, 0.007), (0.114, 0.006)] {
        let lstar = (annular_phi(section.0, section.1, 0.3, 1.0) / 2.0).sqrt();
        for length in [decimal_mm(lstar), 0.75, 2.5] {
            for theory in ["eb", "timo"] {
                for style in ["decimal", "dyadic"] {
                    let (doc, _) = census_document("zhinge122", section, length, 1e-8, theory, style, "both");
                    let raw = judged_request(&doc, "invented:ra-gating:zhinge122");
                    let at = format!("{section:?} L={length} {theory} {style}");
                    let gate = gate(&raw, PreviewSolverMode::SparseInteractive).unwrap_or_else(|| panic!("{at}: no plan")).unwrap_or_else(|e| panic!("{at}: {e}"));
                    assert!(gate.mismatches.is_empty(), "{at}: rows {:?} differ from the long evaluation", gate.mismatches);
                    let b1: Vec<&str> = gate.paths.iter().zip(&gate.kinds).filter(|(p, _)| **p == RowPath::Negated).map(|(_, k)| k.as_str()).collect();
                    let expected: Vec<String> = [0.0, 0.25, 0.5, 0.75, 1.0].iter().flat_map(|s| (0..6).filter(move |&c| c < 4 || *s == 0.0).map(move |c| format!("section:{c}:{s}"))).collect();
                    let mut got = b1.clone();
                    got.sort_unstable();
                    let mut want: Vec<&str> = expected.iter().map(String::as_str).collect();
                    want.sort_unstable();
                    assert_eq!(got, want, "{at}: the 22 negated rows take B1, and only they do");
                    assert!(gate.b2_skipped() > 0, "{at}: B2 skips the zero-prescribed recipes");
                    gated += 1;
                }
            }
        }
    }
    assert_eq!(gated, 24);
}
