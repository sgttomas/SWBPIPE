//! J7 RF census (b7-plan, "The J7 RF census"): the maintained consuming check.
//!
//! Every frozen T3 R1 case in D1's source family runs, in both solver modes, as a
//! genuine request under `3.0.0/exact_pressure_v3` with explicitly empty
//! `pressure_regions`. Inputs are `fixtures/results/j7_rf_census_inputs.json`
//! (`tools/fixtures/j7_rf_census_inputs.py` over the frozen references, authored
//! as 0.3.0); B7b re-authors each copy once, at run time, to the current 0.5.0
//! document (`shear_model` Euler-Bernoulli, an explicit zero corrosion allowance
//! on every pipe section) and changes nothing else. Each case/mode's public
//! outcome (identity, quality, blocking findings, source attempt, and the digests
//! of its published rows and diagnostics), the direct method's selection, stage,
//! error, work and response digest, and the typed first cause are pinned in
//! `fixtures/results/j7_rf_census_expected.json`. Live, for every case and mode:
//! - the direct method call (the committed `j7_t4_consequences.rs` input sequence
//!   over the actual normalized/built/boundary/load-owned source) is recorded;
//! - any public source attempt declines with exactly that typed failure;
//! - no envelope publishes a source-recovery identity or `source_block_recovery`;
//! - a directly selected case (only RF-ZERO-TINY) meets
//!   `|obs - exp| <= 1e-9 * max(|exp|, scale)` exactly on every reference row.
//! B7b deleted the v2 twin (`2.0.0/exact_straight_pressure_v2`) that this census
//! compared against D5's closed exclusions before v2's deletion.
//! The 216-case classification is recomputed here from the frozen file.
//! Regenerate the pin only for a deliberate change:
//! `SWBPIPE_J7_RF_CENSUS_WRITE=1 cargo test --lib j7_rf_census_expected_write`.
use super::*;
use crate::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
#[allow(dead_code)]
#[path = "../../../../validation/benchmarks/numerical_robustness/src/exact.rs"]
mod r1_exact;
#[path = "../../../reporting/result_export/tests/support/retained_precision_corpus.rs"]
mod corpus_storage;
use r1_exact::Exact;

const REFERENCES: &[u8] = include_bytes!("../../../../validation/references/t3_r1/REFERENCES/references.json");
const REFERENCES_SHA256: &str = "7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9";
const INPUTS: &str = include_str!("../../../../fixtures/results/j7_rf_census_inputs.json");
const EXPECTED: &str = include_str!("../../../../fixtures/results/j7_rf_census_expected.json");
const EXPECTED_PATH: &str = "../../fixtures/results/j7_rf_census_expected.json";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const FAMILIES: [&str; 10] = ["RF-CHAIN", "RF-SKEW", "RF-WEAK", "RF-LARGE", "RF-INVARIANCE", "RF-RANGE", "RF-ZERO", "RF-FINITE", "RF-MECH", "RF-CANCEL"];
const PRESSURE_1: &str = "openpipestress.result_semantics/0.3.0/pressure-1";
/// Every source-recovery identity (the current one and the two retired v2
/// ones): none may be published by a frozen RF case.
const SOURCE_IDENTITIES: [&str; 3] = [
    "openpipestress.result_semantics/0.3.0/pressure-source-1",
    "openpipestress.result_semantics/0.3.0/physics-source-1",
    "openpipestress.result_semantics/0.3.0/load-reference-source-1",
];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn references() -> &'static Value {
    static VALUE: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    VALUE.get_or_init(|| {
        assert_eq!(sha(REFERENCES), REFERENCES_SHA256, "frozen references changed");
        serde_json::from_slice(REFERENCES).unwrap()
    })
}
fn inputs() -> &'static Value {
    static VALUE: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    VALUE.get_or_init(|| {
        let inputs = corpus_storage::expand(serde_json::from_str(INPUTS).unwrap());
        assert_eq!(inputs["artifact"], "j7-rf-census-inputs");
        assert_eq!(inputs["reference_pins"]["references.json"], REFERENCES_SHA256);
        inputs
    })
}
fn expected() -> &'static Value {
    static VALUE: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    VALUE.get_or_init(|| serde_json::from_str(EXPECTED).unwrap())
}

/// The actual normalized/built/boundary/load-owned method input (the committed
/// `j7_t4_consequences.rs` sequence).
struct Owned {
    model: PreviewModel,
    built: BuiltModel,
    stiffness: Vec<Vec<f64>>,
    force: AssembledForce,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    springs: Vec<SpringEntry>,
    loads: open_pipe_stress_primitive_loads::LoadApplication,
}
impl Owned {
    fn actual(raw: &Value, mode: PreviewSolverMode) -> Result<Self, String> {
        let (request, _captured) = source_receipt::CapturedInvocation::parse(raw.clone(), mode)
            .map_err(|e| format!("captured parsing: {e:?}"))?;
        let mut model = request.model;
        let mut materials = model.materials.clone();
        let mut diagnostics = Vec::new();
        pressure_runtime::validate_profile(&model, &mut diagnostics);
        validate_model_inputs(&model, &materials, &mut diagnostics);
        validate_support_family_tokens(&model, &mut diagnostics);
        resolve_shared_sections(&mut model, &mut diagnostics);
        normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
        let blocking = |d: &[Diagnostic]| d.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect::<Vec<_>>();
        if has_blocking(&diagnostics) {
            return Err(format!("normalization/admission: {:?}", blocking(&diagnostics)));
        }
        let built = build_model_for_members(&model, &materials, None, &mut diagnostics)
            .ok_or_else(|| format!("model build: {:?}", blocking(&diagnostics)))?;
        if has_blocking(&diagnostics) {
            return Err(format!("model build: {:?}", blocking(&diagnostics)));
        }
        model.materials = materials;
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        if !boundary.findings.is_empty() {
            return Err(format!("actual boundary: {:?}", boundary.findings));
        }
        let mut stiffness =
            assemble_global_stiffness_with_connectors(built.nodes.len(), &built.frame_elements, &built.connectors)
                .map_err(|e| format!("actual formation: {e:?}"))?;
        let springs = boundary.springs;
        for spring in &springs {
            let d = spring.node_dof.global_index();
            stiffness[d][d] += spring.stiffness.value;
        }
        let primitives = build_load_case_primitive_loads(&model, &model.load_cases[0], &mut diagnostics);
        let loads = prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
        if has_blocking(&diagnostics) || loads.is_blocked() {
            return Err(format!("actual loads: {:?}", blocking(&diagnostics)));
        }
        let force = nodal_and_eigen_case_force(&loads, &[], &built).map_err(|e| format!("actual ledger: {e:?}"))?;
        let prescribed: Vec<_> = boundary.restrained_dofs.into_iter().map(|d| (d, 0.0)).collect();
        let free = (0..force.len()).filter(|d| !prescribed.iter().any(|(p, _)| p == d)).collect();
        Ok(Self { model, built, stiffness, force, free, prescribed, springs, loads })
    }
    fn input(&self) -> Input<'_> {
        Input {
            model: &self.model,
            built: &self.built,
            stiffness: &self.stiffness,
            force: &self.force,
            free: &self.free,
            prescribed: &self.prescribed,
            spring_entries: &self.springs,
            load_case: &self.model.load_cases[0],
            load_application: &self.loads,
            thermal_loads: &[],
            load_state: None,
        }
    }
    /// Member axial (EA/L) or torsional (GJ/L) coefficient of the actual frame.
    fn coefficient(&self, member: &str, torsion: bool) -> f64 {
        let i = self.model.pipe_segments.iter().position(|m| m.id == member).unwrap();
        let local = self.built.frame_elements[i].local_stiffness().unwrap();
        if torsion { local[3][3] } else { local[0][0] }
    }
}

/// The unchanged predicate on one direct-method quantity, decided exactly. Method
/// values are SI; `ref_exp` is the reference unit's decimal exponent (mm, N*mm).
fn method_row_holds(key: &str, m: &SelectedSourceRecovery, owned: &Owned, expected: &Exact, scale: &Exact, ref_exp: i64) -> bool {
    let comp = |c: &str| ["UX", "UY", "UZ", "RX", "RY", "RZ"].iter().position(|d| *d == c)
        .or_else(|| ["FX", "FY", "FZ", "MX", "MY", "MZ"].iter().position(|d| *d == c)).unwrap();
    let (e, s) = (expected.scaled(0, ref_exp), scale.scaled(0, ref_exp));
    let scalar = |v: f64| v.is_finite() && r1_exact::predicate(&Exact::from_f64(v), &e, &s);
    let parts: Vec<_> = key.split('.').collect();
    let member = |id: &str| m.members().iter().find(|x| x.member_id == id).unwrap();
    match parts.as_slice() {
        [kind, node, dof] if matches!(*kind, "u" | "th") => {
            let i = owned.model.nodes.iter().position(|n| n.id == *node).unwrap();
            scalar(m.displacements()[i * 6 + comp(dof)])
        }
        ["R", node, dof] => scalar(m.support_actions().iter().find(|s| s.support_id == format!("rigid:{node}")).unwrap().values[comp(dof)]),
        ["S", node, ordinal, c] => scalar(m.support_actions().iter().find(|s| s.support_id == format!("spring:{node}:{ordinal}")).unwrap().values[comp(c)]),
        ["N", id] => scalar(member(id).end_forces[6]),
        ["T", id] => scalar(member(id).end_forces[9]),
        [kind @ ("tw" | "ext"), id] => {
            // Derived relative motion: action / actual member coefficient, decided
            // exactly as |a - e k| <= t k (k > 0); not itself a published quantity.
            let torsion = *kind == "tw";
            let a = member(id).end_forces[if torsion { 9 } else { 6 }];
            let k = owned.coefficient(id, torsion);
            let kk = Exact::from_f64(k);
            a.is_finite() && k > 0.0
                && Exact::from_f64(a).sub(&e.mul(&kk)).abs().cmp(&r1_exact::tolerance(&e, &s).mul(&kk)) != std::cmp::Ordering::Greater
        }
        ["Mb", id, site] => {
            let x = member(id);
            let (y, z) = match *site {
                "i" => (x.end_forces[4], x.end_forces[5]),
                "j" => (x.end_forces[10], x.end_forces[11]),
                _ => (x.sections[2][4], x.sections[2][5]),
            };
            y.is_finite() && z.is_finite() && r1_exact::magnitude_predicate(y, z, &e, &s)
        }
        _ => panic!("unmapped reference key {key}"),
    }
}
/// (rows judged, rows passing) over the case's reference basis.
fn method_comparison(refcase: &Value, m: &SelectedSourceRecovery, owned: &Owned) -> (usize, usize) {
    let rows = if refcase["basis"] == "represented" { &refcase["expected_represented"] } else { &refcase["expected"] };
    let rows = rows.as_array().unwrap();
    let pass = rows.iter().filter(|r| {
        let class = r[2].as_str().unwrap();
        let kind = class.split('@').next().unwrap();
        let scale = if refcase["family"] == "RF-CANCEL" { r[3].as_str().unwrap() } else { refcase["scales"][class]["value"].as_str().unwrap() };
        let ref_exp = match kind {
            "translation" | "extension" if refcase["units"]["length"] == "mm" => -3,
            "moment" if refcase["units"]["moment"] == "N*mm" => -3,
            _ => 0,
        };
        method_row_holds(r[0].as_str().unwrap(), m, owned,
            &Exact::parse_decimal(r[1].as_str().unwrap()).unwrap(), &Exact::parse_decimal(scale).unwrap(), ref_exp)
    }).count();
    (rows.len(), pass)
}
fn work_json(w: &exact::WorkReport) -> Value {
    json!({"charged": w.charged, "rejected": w.rejected.exact().ok(), "limit": w.limit})
}
fn response_digest(m: &SelectedSourceRecovery) -> String {
    let mut h = Sha256::new();
    let mut put = |v: f64| h.update(v.to_bits().to_le_bytes());
    m.displacements().iter().copied().for_each(&mut put);
    m.reactions().iter().copied().for_each(&mut put);
    m.published_nodal_components().iter().copied().for_each(&mut put);
    for x in m.members() {
        x.end_forces.iter().copied().for_each(&mut put);
        x.sections.iter().flatten().copied().for_each(&mut put);
    }
    for s in m.support_actions() {
        s.values.iter().copied().for_each(&mut put);
    }
    format!("{:x}", h.finalize())
}
/// The digest of a published member's exact serialization (rows or diagnostics).
fn member_digest(value: &Value) -> String {
    sha(serde_json::to_string(value).unwrap().as_bytes())
}

/// One contract's actual run: public envelope (or refusal) and the direct method call.
struct Run {
    public: Result<Value, String>,
    owned: Result<Owned, String>,
    method: Option<Result<SelectedSourceRecovery, RecoveryFailure>>,
}
fn run(raw: &Value, mode: PreviewSolverMode) -> Run {
    let public = run_linear_static_preview_value_with_mode(raw.clone(), mode).map(|e| serde_json::to_value(e).unwrap());
    let owned = Owned::actual(raw, mode);
    let method = owned.as_ref().ok().map(|o| {
        solve_ordinary(o.input(), exact::Limits { operations: PHYSICS_SOURCE_WORK_LIMIT, ..Default::default() }, ForceScale::UNSCALED)
    });
    Run { public, owned, method }
}
fn method_value(run: &Run) -> Value {
    match &run.method {
        None => json!({"outcome": "input_not_ready", "error": run.owned.as_ref().err()}),
        Some(Ok(m)) => {
            let s = m.summary();
            json!({"outcome": "selected", "work": work_json(&s.work), "blocks": s.blocks, "largest_block": s.largest_block,
                   "functional_count": s.functional_count, "dof_projection_count": s.dof_projection_count,
                   "response_digest": response_digest(m)})
        }
        Some(Err(f)) => json!({"outcome": "declined", "stage": f.stage, "helper_stage": format!("{:?}", f.helper_stage),
                               "error": format!("{:?}", f.error), "work": work_json(&f.work)}),
    }
}
/// The pinned typed first cause of one case/mode.
fn first_cause(run: &Run) -> Value {
    let method = method_value(run);
    if let Err(e) = &run.public {
        assert!(e.starts_with("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT: "), "unclassified public refusal {e}");
        assert_eq!(method["error"], json!(format!("captured parsing: ReceiptError({e:?}, None)")));
        return json!({"class": "hash001_unsafe_integral_float", "stage": "captured request (checked JSON, before solve)", "error": e});
    }
    match method["outcome"].as_str().unwrap() {
        "input_not_ready" => {
            let error = method["error"].as_str().unwrap();
            assert!(error.starts_with("model build: [\"PIPE_ELEMENT_INPUT_INVALID\""), "unclassified input refusal {error}");
            json!({"class": "ordinary_axis_gate", "stage": "model build (before solve)", "error": error})
        }
        "selected" => {
            let mut cause = method.clone();
            cause.as_object_mut().unwrap().remove("outcome");
            cause["class"] = json!("selected_method_only");
            cause
        }
        _ => {
            let error = method["error"].as_str().unwrap();
            let class = if error.starts_with("Exact(UnsupportedBlock { order: ") {
                "exact_block_order_above_2"
            } else if error == "Unsupported(\"actual transform is not a signed permutation\")" {
                "transform_not_signed_permutation"
            } else if error == "Exact(Arithmetic(Range(\"arithmetic outside normal range\")))" {
                "checked_value_range"
            } else {
                panic!("unclassified source-method decline {error}")
            };
            let mut cause = method.clone();
            cause.as_object_mut().unwrap().remove("outcome");
            cause["class"] = json!(class);
            cause
        }
    }
}
fn public_value(public: &Result<Value, String>) -> Value {
    match public {
        Err(e) => json!({"outcome": "refused_before_envelope", "error": e}),
        Ok(env) => {
            let diags = env["diagnostics"].as_array().unwrap();
            let blocking: BTreeSet<String> = diags.iter().filter(|d| d["severity"] == "blocking")
                .map(|d| format!("{}: {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap())).collect();
            let rows = env["results"].as_array().map_or(0, Vec::len);
            json!({
                "outcome": if !blocking.is_empty() && rows == 0 { "refused_blocking" } else { "ordinary_published" },
                "identity": env["producer"]["semantic_contract_id"],
                "numerical_quality": env["numerical_quality"]["status"],
                "blocking": blocking,
                "source_attempt": if diags.iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE") { "declined" } else { "not_attempted" },
                "results_sha256": member_digest(&env["results"]),
                "diagnostics_sha256": member_digest(&env["diagnostics"]),
            })
        }
    }
}

/// One case's request: the committed input's copy, re-authored once to the
/// current 0.5.0 exact document (B7b); the committed inputs stay 0.3.0.
fn current_request(case: &Value) -> Value {
    let input = &case["request"];
    assert_eq!(input["model"]["pressure_contract"], json!({"version": "3.0.0", "mode": "exact_pressure_v3"}));
    let current = crate::b7b_admission_tests::reauthor_current_request(input.clone());
    assert_eq!(current["model"]["schema_version"], crate::MODEL_SCHEMA_VERSION);
    current
}

/// One case and mode: decline-text and no-publication checks; returns the pinned entry.
fn census_entry(case: &Value, mode: PreviewSolverMode) -> Value {
    let id = case["id"].as_str().unwrap();
    let v3 = run(&current_request(case), mode);
    let at = format!("{id} {}", mode.as_str());
    // No frozen RF case publishes a source-recovery identity.
    if let Ok(env) = &v3.public {
        assert!(env.get("source_block_recovery").is_none(), "{at}: source publication");
        assert!(!SOURCE_IDENTITIES.iter().any(|s| env["producer"]["semantic_contract_id"] == *s), "{at}: source identity");
        assert_eq!(env["producer"]["semantic_contract_id"], PRESSURE_1, "{at}");
    }
    // A public attempt declines with exactly the direct method's typed failure.
    let declines: Vec<String> = v3.public.as_ref().map(|env| env["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE").map(|d| d["message"].as_str().unwrap().to_owned()).collect())
        .unwrap_or_default();
    match (&v3.method, declines.as_slice()) {
        (_, []) => {}
        (Some(Err(f)), [text]) => assert_eq!(*text, format!("The bounded retained-source method did not produce a selected response: {f:?}"), "{at}"),
        (_, other) => panic!("{at}: public attempt without a matching direct decline: {other:?}"),
    }
    let mut entry = json!({"id": id, "family": case["family"], "mode": mode.as_str(), "public": public_value(&v3.public),
        "first_cause": first_cause(&v3)});
    // A directly selected case meets the frozen criterion on every reference row.
    if let (Some(Ok(m3)), Ok(o3)) = (&v3.method, &v3.owned) {
        let refcase = &references()["cases"][id];
        let (n3, p3) = method_comparison(refcase, m3, o3);
        assert_eq!(p3, n3, "{at}: direct-method rows outside the frozen criterion");
        entry["method_only_comparison"] = json!({"rows": n3, "pass": p3});
    }
    entry
}

fn family_entries(family: &str) -> Vec<Value> {
    let cases: Vec<_> = inputs()["cases"].as_array().unwrap().iter().filter(|c| c["family"] == family).collect();
    assert!(!cases.is_empty(), "{family}: no source-family cases");
    cases.into_iter().flat_map(|case| MODES.map(|mode| census_entry(case, mode))).collect()
}
fn check_family(family: &str) {
    let actual = family_entries(family);
    if std::env::var("SWBPIPE_J7_RF_CENSUS_WRITE").as_deref() == Ok("1") {
        return;
    }
    let pinned: Vec<_> = expected()["entries"].as_array().unwrap().iter().filter(|e| e["family"] == family).collect();
    assert_eq!(actual.len(), pinned.len(), "{family}: entry count");
    for (a, p) in actual.iter().zip(pinned) {
        assert_eq!(a, p, "{family}: census entry differs from its pinned first cause/work");
    }
}

#[test]
fn j7_rf_census_rf_chain() { check_family("RF-CHAIN") }
#[test]
fn j7_rf_census_rf_skew() { check_family("RF-SKEW") }
#[test]
fn j7_rf_census_rf_weak() { check_family("RF-WEAK") }
#[test]
fn j7_rf_census_rf_large() { check_family("RF-LARGE") }
#[test]
fn j7_rf_census_rf_invariance() { check_family("RF-INVARIANCE") }
#[test]
fn j7_rf_census_rf_range() { check_family("RF-RANGE") }
#[test]
fn j7_rf_census_rf_zero() { check_family("RF-ZERO") }
#[test]
fn j7_rf_census_rf_finite() { check_family("RF-FINITE") }
#[test]
fn j7_rf_census_rf_mech_companion() { check_family("RF-MECH") }
#[test]
fn j7_rf_census_rf_cancel() { check_family("RF-CANCEL") }

/// The 216-case classification, recomputed from the frozen file; the pinned census
/// covers exactly the 161 source-family cases and states the design's first causes.
#[test]
fn j7_rf_census_classification_and_pin() {
    let refs = references();
    let mut reasons: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    let mut family = BTreeSet::new();
    for (id, case) in refs["cases"].as_object().unwrap() {
        let model = &case["model"];
        let nodes = model["nodes_m"].as_object().map(|n| n.len()).unwrap_or_else(|| {
            let text = model["generator"].as_str().unwrap();
            text.split(" has ").nth(1).unwrap().split(' ').next().unwrap().parse().unwrap()
        });
        let directional = model["supports"].as_object().unwrap().values()
            .flat_map(|s| s["springs"].as_array().into_iter().flatten())
            .any(|s| s["direction"].as_array().unwrap().iter().filter(|x| !Exact::parse_decimal(x.as_str().unwrap()).unwrap().is_zero()).count() != 1);
        assert_eq!(directional, case["flags"]["needs_directional_spring"] == true, "{id}");
        let mut why = Vec::new();
        if case["family"] == "RF-MECH" && case.get("expected").is_none() { why.push("must_refuse_mechanism"); }
        if 6 * nodes > 256 { why.push("dofs_exceed_256"); }
        if model.get("member_uniform_loads_N_per_m_global").is_some() { why.push("non_nodal_load"); }
        if directional { why.push("directional_spring_not_authorable"); }
        if why.is_empty() { family.insert(id.clone()); }
        for w in why { reasons.entry(w).or_default().insert(id.clone()); }
    }
    assert_eq!(refs["cases"].as_object().unwrap().len(), 216);
    assert_eq!(family.len(), 161);
    let counts: BTreeMap<_, _> = reasons.iter().map(|(k, v)| (*k, v.len())).collect();
    assert_eq!(counts, BTreeMap::from([("must_refuse_mechanism", 8), ("dofs_exceed_256", 25), ("directional_spring_not_authorable", 22), ("non_nodal_load", 3)]));
    // The three large mechanisms are the only overlap: 8 + 22 + 22 + 3 = 55 outside.
    assert_eq!(reasons["dofs_exceed_256"].intersection(&reasons["must_refuse_mechanism"]).count(), 3);
    assert_eq!(reasons.values().flatten().collect::<BTreeSet<_>>().len(), 55);
    assert_eq!(reasons["non_nodal_load"], BTreeSet::from(["RF-CANCEL-UDL-W1e5", "RF-CANCEL-UDL-W1e8", "RF-CANCEL-UDL-W1e80"].map(String::from)));
    // The committed generator's classification and requests agree.
    let fixture = inputs()["classification"].as_array().unwrap();
    assert_eq!(fixture.len(), 216);
    for c in fixture {
        let id = c["id"].as_str().unwrap();
        let why: BTreeSet<&str> = c["reasons"].as_array().unwrap().iter().map(|r| r.as_str().unwrap()).collect();
        let actual: BTreeSet<&str> = reasons.iter().filter(|(_, ids)| ids.contains(id)).map(|(k, _)| *k).collect();
        assert_eq!(why, actual, "{id}");
    }
    let run_ids: BTreeSet<String> = inputs()["cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap().to_owned()).collect();
    assert_eq!(run_ids, family);
    assert!(inputs()["cases"].as_array().unwrap().iter().all(|c| FAMILIES.contains(&c["family"].as_str().unwrap())));
    // The pin: two modes per case, one first-cause class per case, the design's counts.
    let pin = expected();
    assert_eq!(pin["inputs_sha256"], sha(INPUTS.as_bytes()), "regenerate the pin after an input change");
    assert_eq!(pin["references_sha256"], REFERENCES_SHA256);
    let entries = pin["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 322);
    let mut classes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut per_case: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for e in entries {
        let id = e["id"].as_str().unwrap();
        per_case.entry(id).or_default().insert(e["first_cause"]["class"].as_str().unwrap());
        classes.entry(e["first_cause"]["class"].as_str().unwrap().into()).or_default().insert(id.into());
        assert!(!SOURCE_IDENTITIES.iter().any(|s| e["public"]["identity"] == *s));
        assert!(e["public"].get("twin_identity").is_none(), "B7b: no v2 twin is run");
    }
    assert_eq!(per_case.keys().map(|k| k.to_string()).collect::<BTreeSet<_>>(), family);
    assert!(per_case.values().all(|c| c.len() == 1), "a case's first cause is mode-independent");
    let counts: BTreeMap<_, _> = classes.iter().map(|(k, v)| (k.as_str(), v.len())).collect();
    assert_eq!(counts, BTreeMap::from([
        ("exact_block_order_above_2", 94), ("transform_not_signed_permutation", 27), ("checked_value_range", 4),
        ("hash001_unsafe_integral_float", 29), ("ordinary_axis_gate", 6), ("selected_method_only", 1),
    ]));
    assert_eq!(classes["selected_method_only"], BTreeSet::from(["RF-ZERO-TINY".to_owned()]));
    assert_eq!(classes["checked_value_range"], BTreeSet::from(["RF-RANGE-CHAIN-E-1000", "RF-RANGE-CONT-E-1000", "RF-RANGE-SKEW-E-1000", "RF-RANGE-THIN-B"].map(String::from)));
    let tiny: Vec<_> = entries.iter().filter(|e| e["id"] == "RF-ZERO-TINY").collect();
    assert_eq!(tiny.len(), 2);
    for e in tiny {
        assert_eq!(e["public"]["source_attempt"], "not_attempted");
        assert_eq!(e["public"]["numerical_quality"], "checks_passed");
        assert_eq!(e["method_only_comparison"], json!({"rows": 25, "pass": 25}));
    }
}

/// The pinned public digests name a changed published row value or an added
/// diagnostic (they replace the deleted v2 twin's leaf comparison).
#[test]
fn j7_rf_census_public_digests_flag_row_and_diagnostic_changes() {
    let a = json!({"producer": {"semantic_contract_id": PRESSURE_1}, "numerical_quality": {"status": "checks_passed"},
                   "results": [{"value": 1.0}], "diagnostics": []});
    let base = public_value(&Ok(a.clone()));
    let mut row = a.clone();
    row["results"][0]["value"] = json!(f64::from_bits(1.0f64.to_bits() + 1));
    let changed = public_value(&Ok(row));
    assert_ne!(changed["results_sha256"], base["results_sha256"]);
    assert_eq!(changed["diagnostics_sha256"], base["diagnostics_sha256"]);
    let mut diagnostic = a;
    diagnostic["diagnostics"] = json!([{"code": "X", "severity": "warning", "message": "x"}]);
    let changed = public_value(&Ok(diagnostic));
    assert_eq!(changed["results_sha256"], base["results_sha256"]);
    assert_ne!(changed["diagnostics_sha256"], base["diagnostics_sha256"]);
}

/// Regeneration only (`SWBPIPE_J7_RF_CENSUS_WRITE=1`); otherwise a no-op whose
/// counterpart checks are the family tests above.
#[test]
fn j7_rf_census_expected_write() {
    if std::env::var("SWBPIPE_J7_RF_CENSUS_WRITE").as_deref() != Ok("1") {
        return;
    }
    let entries: Vec<Value> = FAMILIES.iter().flat_map(|f| family_entries(f)).collect();
    let lines: Vec<String> = entries.iter().map(|e| serde_json::to_string(e).unwrap()).collect();
    let text = format!(
        "{{\"artifact\":\"j7-rf-census-expected\",\"schema_version\":\"1.0.0\",\"inputs_sha256\":\"{}\",\"references_sha256\":\"{}\",\"entries\":[\n{}\n]}}\n",
        sha(INPUTS.as_bytes()), REFERENCES_SHA256, lines.join(",\n")
    );
    std::fs::write(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(EXPECTED_PATH), text).unwrap();
}
