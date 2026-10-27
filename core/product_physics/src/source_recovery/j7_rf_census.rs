//! J7 RF census (b7-plan, "The J7 RF census"): the maintained consuming check.
//!
//! Every frozen T3 R1 case in D1's source family runs, in both solver modes, as a
//! genuine request under `3.0.0/exact_pressure_v3` with explicitly empty
//! `pressure_regions`. Inputs are `fixtures/results/j7_rf_census_inputs.json`
//! (`tools/fixtures/j7_rf_census_inputs.py` over the frozen references, authored
//! as 0.3.0); B7b re-authors each copy once, at run time, to the current 0.8.0
//! document (`shear_model` Euler-Bernoulli, an explicit zero corrosion allowance
//! on every pipe section) and changes nothing else. Each case/mode's public
//! outcome (identity, quality, blocking findings, source attempt, and the digests
//! of its published rows and diagnostics, of its contract evidence, numerical
//! quality, summary and status, and of the whole envelope), the direct method's selection, stage,
//! error, work and response digest, and the typed first cause are pinned in
//! `fixtures/results/j7_rf_census_expected.json`. Live, for every case and mode:
//! - the direct method call (the committed `j7_t4_consequences.rs` input sequence
//!   over the actual normalized/built/boundary/load-owned source) is recorded;
//! - any public decline has exactly the direct method's typed failure;
//! - exactly PIN-OFF-345-r1e-08/-12 publish `pressure-source-1`, in both modes,
//!   with the congruence level, Rust reader acceptance, exact J2 base bytes and
//!   every published source row bit-equal to its direct projection;
//! - all 33 directly selected cases (31 method-only and 2 published) meet
//!   `|obs - exp| <= 1e-9 * max(|exp|, scale)` exactly on every reference row.
//! The other first causes are 84 blocks above order 4, 5 upfront descriptor-unit
//! Budget declines, 4 checked ranges, 29 HASH-001 refusals and 6 axis gates.
//! The five rotated LARGE/INVARIANCE models reach the unchanged 16,384-unit
//! precheck before their order-above-4 blocks (owner's RF correction).
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
const J2: &str = include_str!("../../../../fixtures/results/pressure_source_reader_corpus.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const FAMILIES: [&str; 10] = ["RF-CHAIN", "RF-SKEW", "RF-WEAK", "RF-LARGE", "RF-INVARIANCE", "RF-RANGE", "RF-ZERO", "RF-FINITE", "RF-MECH", "RF-CANCEL"];
const PRESSURE_1: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
/// Every source-recovery identity (the current one and the two retired v2 ones).
const SOURCE_IDENTITIES: [&str; 3] = [
    "openpipestress.result_semantics/0.3.0/pressure-source-1",
    "openpipestress.result_semantics/0.3.0/physics-source-1",
    "openpipestress.result_semantics/0.3.0/load-reference-source-1",
];
const PUBLISHED: [&str; 2] = ["RF-SKEW-T-PIN-OFF-345-r1e-08", "RF-SKEW-T-PIN-OFF-345-r1e-12"];

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
pub(crate) fn inputs() -> &'static Value {
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
pub(super) struct Owned {
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
    pub(super) fn actual(raw: &Value, mode: PreviewSolverMode) -> Result<Self, String> {
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
    pub(super) fn input(&self) -> Input<'_> {
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
            cause["class"] = json!(if run.public.as_ref().unwrap()["producer"]["semantic_contract_id"] == SOURCE_IDENTITIES[0] {
                "selected_published"
            } else {
                "selected_method_only"
            });
            cause
        }
        _ => {
            let error = method["error"].as_str().unwrap();
            let class = if error.starts_with("Exact(UnsupportedBlock { order: ") {
                let order = error.strip_prefix("Exact(UnsupportedBlock { order: ").unwrap().strip_suffix(" })").unwrap().parse::<usize>().unwrap();
                assert!(order > 4, "an admitted block was refused: {error}");
                "exact_block_order_above_4"
            } else if error == "Unsupported(\"actual transform is not a block rotation\")" {
                "transform_not_block_rotation"
            } else if error == "Exact(Budget)" {
                "budget"
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
                "outcome": if env["producer"]["semantic_contract_id"] == SOURCE_IDENTITIES[0] { "source_published" } else if !blocking.is_empty() && rows == 0 { "refused_blocking" } else { "ordinary_published" },
                "identity": env["producer"]["semantic_contract_id"],
                "numerical_quality": env["numerical_quality"]["status"],
                "blocking": blocking,
                "source_attempt": if diags.iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_SELECTED") { "selected" } else if diags.iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE") { "declined" } else { "not_attempted" },
                "results_sha256": member_digest(&env["results"]),
                "diagnostics_sha256": member_digest(&env["diagnostics"]),
                // B7b F2: the whole published envelope, and the members a change
                // would most likely touch named separately so a difference is located.
                "envelope_sha256": member_digest(env),
                "contract_evidence_sha256": member_digest(&env["contract_evidence"]),
                "numerical_quality_sha256": member_digest(&env["numerical_quality"]),
                "summary_sha256": member_digest(&env["summary"]),
                "status_sha256": member_digest(&env["status"]),
            })
        }
    }
}

/// One case's request: the committed input's copy, re-authored once to the
/// current 0.8.0 exact document (B7b); the committed inputs stay 0.3.0.
pub(crate) fn current_request(case: &Value) -> Value {
    let input = &case["request"];
    assert_eq!(input["model"]["pressure_contract"], json!({"version": "3.0.0", "mode": "exact_pressure_v3"}));
    let current = crate::b7b_admission_tests::reauthor_current_request(input.clone());
    assert_eq!(current["model"]["schema_version"], crate::MODEL_SCHEMA_VERSION);
    current
}

/// RF design §6: bind all four public envelopes to genuine J2 bases and to
/// their direct method, and read them with their captured invocation. Receipt
/// qualification and both ledgers establish that finalization fitted; the
/// implementation probe separately measures its two reservations (§2.9).
fn check_public_source(id: &str, request: &Value, mode: PreviewSolverMode, env: &Value, direct: &SelectedSourceRecovery) {
    use open_pipe_stress_result_export::physics_source as ps;
    let at = format!("{id} {}", mode.as_str());
    assert_eq!(env["producer"]["semantic_contract_id"], SOURCE_IDENTITIES[0], "{at}");
    let body = &env["source_block_recovery"]["body"];
    assert_eq!(body["status"], "qualified", "{at}");
    let cases = body["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 1, "{at}");
    let case = &cases[0];
    assert_eq!(case["outcome"], "qualified", "{at}");
    assert_eq!(case["source"]["level"], "complete_identified_declared_member_congruence", "{at}");
    let orders: Vec<_> = case["source"]["free_blocks"].as_array().unwrap().iter().map(|b| b.as_array().unwrap().len()).collect();
    assert_eq!(orders, [4, 2], "{at}");
    let invocation = json!({"request":request,"solver_mode":mode.as_str()});
    assert_eq!(ps::validate_in(env, Some(&invocation), ps::SourceContract::PressureV3), Ok(true), "{at}");
    assert_eq!(ps::validate_transport_metadata_in(env, ps::SourceContract::PressureV3), Ok(()), "{at}");
    static CORPUS: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    let corpus = CORPUS.get_or_init(|| corpus_storage::expand(serde_json::from_str(J2).unwrap()));
    let base_id = format!("rf_source_method:{id}:{}:genuine", mode.as_str());
    let bases: Vec<_> = corpus["entries"].as_array().unwrap().iter().filter(|e| e["id"] == base_id).collect();
    assert_eq!(bases.len(), 1, "{at}: one genuine J2 base");
    assert_eq!(bases[0]["invocation"], invocation, "{at}: same captured request");
    assert_eq!(serde_json::to_vec(&bases[0]["source"]).unwrap(), serde_json::to_vec(env).unwrap(), "{at}: exact J2 envelope bytes");
    let projections = direct.retained().projections();
    let mut rows = 0;
    for p in case["projections"].as_array().unwrap() {
        let fid = p["functional_id"].as_str().unwrap();
        let index: usize = fid.rsplit(':').next().unwrap().parse().unwrap();
        let value = projections[index].value();
        assert_eq!(p["value_bits"], format!("{:016x}", value.to_bits()), "{at}: {fid}");
        if let Some(rid) = p["result_id"].as_str() {
            let row = env["results"].as_array().unwrap().iter().find(|r| r["id"] == rid).unwrap();
            assert_eq!(row["value"].as_f64().unwrap().to_bits(), value.to_bits(), "{at}: row {rid}");
            rows += 1;
        }
    }
    assert_eq!(rows, 60, "{at}: all published source rows");
    let work = &case["work"];
    let charged = work["charged"].as_u64().unwrap();
    assert_eq!(work["reserved_unobserved_failure"], 0, "{at}");
    assert_eq!(work["rejected_reservation"]["amount"], 0, "{at}");
    assert!(charged <= work["limit"].as_u64().unwrap() && work["limit"].as_u64().unwrap() <= 8_000_000, "{at}");
    let total = &body["invocation_work"];
    assert_eq!(total["charged"].as_u64().unwrap(), charged + total["publication_charged"].as_u64().unwrap(), "{at}");
    assert!(total["charged"].as_u64().unwrap() <= total["limit"].as_u64().unwrap() && total["limit"].as_u64().unwrap() <= 64_000_000, "{at}");
}

/// One case and mode: public bindings and decline-text checks; returns the pinned entry.
fn census_entry(case: &Value, mode: PreviewSolverMode) -> Value {
    let id = case["id"].as_str().unwrap();
    let request = current_request(case);
    let v3 = run(&request, mode);
    let at = format!("{id} {}", mode.as_str());
    if let Ok(env) = &v3.public {
        if PUBLISHED.contains(&id) {
            check_public_source(id, &request, mode, env, v3.method.as_ref().unwrap().as_ref().unwrap());
        } else {
            assert!(env.get("source_block_recovery").is_none(), "{at}: source publication");
            assert!(!SOURCE_IDENTITIES.iter().any(|s| env["producer"]["semantic_contract_id"] == *s), "{at}: source identity");
            assert_eq!(env["producer"]["semantic_contract_id"], PRESSURE_1, "{at}");
        }
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
        if PUBLISHED.contains(&id) {
            assert_eq!(e["first_cause"]["class"], "selected_published");
            assert_eq!(e["public"]["identity"], SOURCE_IDENTITIES[0]);
            assert_eq!(e["public"]["source_attempt"], "selected");
            assert_eq!(e["method_only_comparison"], json!({"rows":28,"pass":28}));
        } else {
            assert!(!SOURCE_IDENTITIES.iter().any(|s| e["public"]["identity"] == *s));
        }
        assert!(e["public"].get("twin_identity").is_none(), "B7b: no v2 twin is run");
    }
    assert_eq!(per_case.keys().map(|k| k.to_string()).collect::<BTreeSet<_>>(), family);
    assert!(per_case.values().all(|c| c.len() == 1), "a case's first cause is mode-independent");
    let counts: BTreeMap<_, _> = classes.iter().map(|(k, v)| (k.as_str(), v.len())).collect();
    assert_eq!(counts, BTreeMap::from([
        ("exact_block_order_above_4", 84), ("budget", 5), ("checked_value_range", 4),
        ("hash001_unsafe_integral_float", 29), ("ordinary_axis_gate", 6), ("selected_method_only", 31),
        ("selected_published", 2),
    ]));
    let mut method_only = BTreeSet::new();
    for load in ["F", "M"] {
        for exponent in [5, 6, 7, 8] {
            for pattern in ["GnG", "GGn", "nGG"] {
                method_only.insert(format!("RF-CANCEL-{load}-G1e{exponent}-{pattern}"));
            }
        }
        for companion in ["ORTHO", "INPLANE"] {
            method_only.insert(format!("RF-CANCEL-{load}-G1e8-GnG-{companion}"));
        }
    }
    assert_eq!(method_only.len(), 28);
    method_only.extend(["RF-ZERO-TINY", "RF-SKEW-T-PIN-OFF-345-r1e-04", "RF-MECH-LINE345-RX-COMPANION"].map(String::from));
    assert_eq!(classes["selected_method_only"], method_only);
    assert_eq!(classes["selected_published"], BTreeSet::from(PUBLISHED.map(String::from)));
    assert_eq!(classes["budget"], BTreeSet::from([
        "RF-LARGE-CHAIN-n00010-ROT", "RF-LARGE-TREE-n00010-ROT", "RF-LARGE-CONT-n00010-ROT",
        "RF-INVARIANCE-CONT4-ROT-Q3", "RF-INVARIANCE-CONT4-ROT-Q9",
    ].map(String::from)));
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
    assert_ne!(changed["envelope_sha256"], base["envelope_sha256"]);
}

/// B7b F2: a change confined to contract evidence, the numerical-quality detail
/// (status unchanged), the summary or the status, or to any other envelope member,
/// is flagged by the pinned digests although rows and diagnostics are unchanged.
#[test]
fn j7_rf_census_public_digests_flag_envelope_member_changes() {
    let a = json!({"producer": {"semantic_contract_id": PRESSURE_1, "component_version": "1"},
                   "contract_evidence": {"pressure_contract": {"version": "3.0.0"}},
                   "numerical_quality": {"status": "checks_passed", "cases": [{"id": "c", "status": "checks_passed"}]},
                   "summary": {"node_count": 2}, "status": {"mechanics": "preview_complete"},
                   "results": [{"value": 1.0}], "diagnostics": []});
    let base = public_value(&Ok(a.clone()));
    let unchanged = ["outcome", "identity", "numerical_quality", "blocking", "source_attempt", "results_sha256", "diagnostics_sha256"];
    for (pointer, value, member) in [
        ("/contract_evidence/pressure_contract/version", json!("2.0.0"), Some("contract_evidence_sha256")),
        ("/numerical_quality/cases/0/status", json!("checks_failed"), Some("numerical_quality_sha256")),
        ("/summary/node_count", json!(3), Some("summary_sha256")),
        ("/status/mechanics", json!("preview_refused"), Some("status_sha256")),
        ("/producer/component_version", json!("2"), None),
    ] {
        let mut env = a.clone();
        *env.pointer_mut(pointer).unwrap() = value;
        let changed = public_value(&Ok(env));
        for key in unchanged { assert_eq!(changed[key], base[key], "{pointer}: {key}"); }
        assert_ne!(changed["envelope_sha256"], base["envelope_sha256"], "{pointer}");
        for key in ["contract_evidence_sha256", "numerical_quality_sha256", "summary_sha256", "status_sha256"] {
            if Some(key) == member { assert_ne!(changed[key], base[key], "{pointer}"); } else { assert_eq!(changed[key], base[key], "{pointer}: {key}"); }
        }
    }
    // An absent contract-evidence member has its own stable digest (of `null`).
    let mut env = a;
    env.as_object_mut().unwrap().remove("contract_evidence");
    let changed = public_value(&Ok(env));
    assert_ne!(changed["contract_evidence_sha256"], base["contract_evidence_sha256"]);
    assert_eq!(changed["contract_evidence_sha256"], json!(member_digest(&Value::Null)));
}

/// RF source-method design §5.4's maintained control of the metered charging
/// rule, over the census's exact work: every in-family case's direct exact
/// attempt in both modes (the public route's attempts are the same typed
/// declines, work included, which the family tests assert), and, for every
/// selected attempt, the functional replay the receipt performs on the
/// selection's own ledger. Through the frame kernel's dev-only operation
/// counter (`exact-operation-count`, enabled by this crate's dev-dependency;
/// thread-local, so everything runs on this thread): no `Work::mul` or
/// `Work::add` call is charged less than the operations it executed, at least
/// one metered multiplication ran, and each attempt and each replay charged at
/// least the operations counted during it. The cases that make no attempt are
/// exactly the pinned HASH-001 and axis-gate first causes.
#[test]
fn j7_rf_census_metered_charges_cover_counted_operations() {
    use open_pipe_stress_frame_kernel::structural::exact_boundary::exact_operation_count as count;
    count::reset();
    let no_attempt: BTreeSet<&str> = expected()["entries"].as_array().unwrap().iter()
        .filter(|e| matches!(e["first_cause"]["class"].as_str(), Some("hash001_unsafe_integral_float" | "ordinary_axis_gate")))
        .map(|e| e["id"].as_str().unwrap()).collect();
    let (mut attempts, mut replays, mut skipped) = (0usize, 0usize, 0usize);
    let (mut attempt_charged, mut attempt_counted, mut replay_charged, mut replay_counted) = (0u64, 0u64, 0u64, 0u64);
    for case in inputs()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let request = current_request(case);
        for mode in MODES {
            let at = format!("{id} {}", mode.as_str());
            let Ok(owned) = Owned::actual(&request, mode) else {
                assert!(no_attempt.contains(id), "{at}: an in-family case without an attempt");
                skipped += 1;
                continue;
            };
            assert!(!no_attempt.contains(id), "{at}: a pinned no-attempt case reached the method");
            let before = count::operations();
            let result = solve_ordinary(owned.input(), exact::Limits { operations: PHYSICS_SOURCE_WORK_LIMIT, ..Default::default() }, ForceScale::UNSCALED);
            let counted = count::operations() - before;
            let charged = match &result { Ok(m) => m.summary().work.charged, Err(f) => f.work.charged } as u64;
            assert!(charged >= counted, "{at}: attempt charged {charged} < {counted} counted operations");
            attempts += 1;
            attempt_charged += charged;
            attempt_counted += counted;
            if let Ok(mut m) = result {
                let (before, ledger) = (count::operations(), m.summary().work.charged);
                m.replay_against(owned.input()).unwrap_or_else(|f| panic!("{at}: replay on the selection's ledger: {f:?}"));
                let counted = count::operations() - before;
                let charged = (m.summary().work.charged - ledger) as u64;
                assert!(charged >= counted, "{at}: replay charged {charged} < {counted} counted operations");
                replays += 1;
                replay_charged += charged;
                replay_counted += counted;
            }
        }
    }
    let totals = count::totals();
    eprintln!("j7_rf_census metered control: attempts={attempts} replays={replays} no_attempt={skipped} \
               attempt_charged={attempt_charged} attempt_counted={attempt_counted} \
               replay_charged={replay_charged} replay_counted={replay_counted} totals={totals:?}");
    assert_eq!(attempts + skipped, 2 * inputs()["cases"].as_array().unwrap().len());
    assert_eq!(skipped, 2 * no_attempt.len());
    assert!(replays > 0, "no selected attempt was replayed");
    assert_eq!(totals.shortfalls(), 0, "a Work call was charged less than it executed: {totals:?}");
    assert!(totals.mul.calls > 0 && totals.pairs > 0, "no metered multiplication ran: {totals:?}");
    assert!(totals.mul.min_slack.is_some_and(|s| s >= 0) && totals.add.min_slack.is_some_and(|s| s >= 0));
    assert!(totals.min_pair_slack.is_some_and(|s| s >= 0), "{totals:?}");
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

/// T4-U4b §4.3 (S4) capability: every census case the pinned census selects (the two published
/// RF-SKEW cases and every method-only selection, both modes) still selects with the same class
/// under the unchanged 8,000,000 case limit. Each case's new charge is computed and printed, not
/// assumed: the direct method's work, and for a published case its publication's case work (the
/// direct work plus the finalization and wrapper terms the U4b rows add).
#[test]
fn u4b_every_selected_census_case_still_selects_with_its_new_charge() {
    let mut selected = 0;
    let mut largest = 0u64;
    for pinned in expected()["entries"].as_array().unwrap() {
        let class = pinned["first_cause"]["class"].as_str().unwrap();
        if !class.starts_with("selected") {
            continue;
        }
        selected += 1;
        let id = pinned["id"].as_str().unwrap();
        let mode = MODES.into_iter().find(|m| pinned["mode"] == m.as_str()).unwrap();
        let case = inputs()["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap();
        let run = run(&current_request(case), mode);
        let cause = first_cause(&run);
        assert_eq!(cause["class"], class, "{id} {}: same selection class", mode.as_str());
        let direct = cause["work"]["charged"].as_u64().unwrap();
        assert!(direct <= 8_000_000 && cause["work"]["limit"] == 8_000_000, "{id}");
        let mut line = format!("U4B_J7_SELECTED {id} {} class={class} direct_charged={direct} (pinned {})", mode.as_str(), pinned["first_cause"]["work"]["charged"]);
        largest = largest.max(direct);
        if class == "selected_published" {
            let env = run.public.as_ref().unwrap();
            let case = &env["source_block_recovery"]["body"]["cases"][0];
            let charged = case["work"]["charged"].as_u64().unwrap();
            assert!(charged <= case["work"]["limit"].as_u64().unwrap() && case["work"]["limit"].as_u64().unwrap() <= 8_000_000, "{id}");
            largest = largest.max(charged);
            line += &format!(" publication_case_charged={charged}");
        }
        println!("{line}");
    }
    assert_eq!(selected, 66, "the census's 62 method-only and 4 published selections");
    println!("U4B_J7_SELECTED largest per-case charge {largest} of 8000000");
}

// R-A's gating test (DEL-04-08 Design `source-work-nondyadic-inputs.md` §3.2).
#[path = "ra_gating_tests.rs"]
pub(crate) mod ra_gating;
