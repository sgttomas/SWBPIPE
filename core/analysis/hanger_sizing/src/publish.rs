//! Publication of a sizing result (DEL-04-11 `Work/hanger-sizing.md` §8.3, slice
//! 1d-α): the closed document `openpipestress.hanger_sizing.result` 1.0.0
//! (`schemas/hanger_sizing_result.schema.json`) and the headless pipeline that
//! forms it from one input text (§8.3.2): admission, shape, library import
//! boundary, request, the engine's run, then publication.
//!
//! Every hash is the SHA-256 of the `openpipestress_jcs_ijson_v1` canonical
//! text. The hashes identify the inputs; they authenticate no origin and claim
//! no input manifest (P4).

use crate::admission::{admit, in_profile, push_key, AdmissionFinding};
use crate::derived::{self, derived_document, RequestCandidate, Role, SizingRequest, SizingRun, WeightSupportState};
use crate::{
    library, Dof, FamilyPolicy, LoadDesign, Method, Outcome, Refusal, Selected, SelectionRule, Test,
    VariationReference, VerificationStanding,
};
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_library_import_document::{validate_library_import, IntendedVisibility, LibraryKind};
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

pub const DOCUMENT_KIND: &str = "openpipestress.hanger_sizing.result";
pub const SCHEMA_VERSION: &str = "1.0.0";
pub const SEMANTIC_CONTRACT_ID: &str = "openpipestress.result_semantics/hanger-sizing-1";
pub const ROUTE: &str = "pressure_free";
pub const CANONICAL_PROFILE: &str = "openpipestress_jcs_ijson_v1";

pub const LIMITATIONS: [&str; 4] = [
    "Hanger sizing selects records of the user's own library by the user's own tests on the stated cases; it judges no hanger's suitability for a service and determines no code compliance.",
    "Sized means the sizing algorithm completed. A candidate may have no fitting record, and the verification may be conditional; both are stated in the result.",
    "Loads and positions come from derived solves of the bound solve request and inherit every premise of those solves; nonlinear supports take each case's own converged state.",
    "Hanger sizing is published on the pressure-free route only.",
];

pub const BASIS_STATEMENT: [&str; 2] = [
    "Every position is measured from the drawn geometry, taken as the installed position at which each spring is set; F_cold is that setting and F_C the cold-condition load.",
    "Quantities are SI: forces in N, lengths in m, rates in N/m; variations are ratios.",
];

/// The headless verb's own input limit (§8.3.2); not an engineering value.
pub const MAX_INPUT_BYTES: usize = 64 * 1024 * 1024;

pub const INPUT_TOO_LARGE: &str = "HANGER_SIZING_INPUT_TOO_LARGE";
pub const INPUT_NOT_ADMITTED: &str = "HANGER_SIZING_INPUT_NOT_ADMITTED";
pub const INPUT_SHAPE_INVALID: &str = "HANGER_SIZING_INPUT_SHAPE_INVALID";
pub const LIBRARY_NOT_ADMITTED: &str = "HANGER_SIZING_LIBRARY_NOT_ADMITTED";
pub const REQUEST_INVALID: &str = "HANGER_SIZING_REQUEST_INVALID";
pub const RESULT_NOT_PUBLISHABLE: &str = "HANGER_SIZING_RESULT_NOT_PUBLISHABLE";
pub const DERIVED_DOCUMENT_NOT_HASHABLE: &str = "HANGER_SIZING_DERIVED_DOCUMENT_NOT_HASHABLE";

/// The refusal codes the design names (§2, §3, §4, §8.3), which a refused
/// document's `refusals[].code` takes. The design's two disclosure codes and its
/// verification warning are not refusals and are not listed.
pub const REFUSAL_CODES: [&str; 40] = [
    "HANGER_SIZING_CANDIDATES_REQUIRED",
    "HANGER_SIZING_CANDIDATE_DOF_SHARED",
    "HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED",
    "HANGER_SIZING_CANDIDATE_DUPLICATE",
    "HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED",
    "HANGER_SIZING_CANDIDATE_MOVED",
    "HANGER_SIZING_CANDIDATE_REFERENCED",
    "HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED",
    "HANGER_SIZING_CANDIDATE_UNKNOWN",
    "HANGER_SIZING_CASE_ROLES_OVERLAP",
    "HANGER_SIZING_CASE_UNKNOWN",
    "HANGER_SIZING_CHECK_OPERATING_CASES_REQUIRED",
    "HANGER_SIZING_DERIVED_DOCUMENT_NOT_HASHABLE",
    "HANGER_SIZING_DERIVED_SOLVE_FAILED",
    "HANGER_SIZING_DESIGN_OPERATING_CASE_REQUIRED",
    "HANGER_SIZING_EXACT_ROUTE_PENDING",
    "HANGER_SIZING_FAMILY_POLICY_REQUIRED",
    "HANGER_SIZING_ID_COLLISION",
    "HANGER_SIZING_INPUT_NOT_ADMITTED",
    "HANGER_SIZING_INPUT_SHAPE_INVALID",
    "HANGER_SIZING_INPUT_TOO_LARGE",
    "HANGER_SIZING_LIBRARY_NOT_ADMITTED",
    "HANGER_SIZING_LIBRARY_RECORD_DUPLICATE",
    "HANGER_SIZING_LIBRARY_REQUIRED",
    "HANGER_SIZING_LIBRARY_UNAVAILABLE",
    "HANGER_SIZING_LOAD_DESIGN_REQUIRED",
    "HANGER_SIZING_LOAD_NOT_POSITIVE",
    "HANGER_SIZING_REQUEST_INVALID",
    "HANGER_SIZING_RESULT_NOT_PUBLISHABLE",
    "HANGER_SIZING_SELECTION_RULE_REQUIRED",
    "HANGER_SIZING_SOLVER_MODE_REQUIRED",
    "HANGER_SIZING_TEST_CASE_UNSUPPORTED",
    "HANGER_SIZING_TRAVEL_ALLOWANCE_INVALID",
    "HANGER_SIZING_TRAVEL_ALLOWANCE_REQUIRED",
    "HANGER_SIZING_VARIATION_LIMIT_INVALID",
    "HANGER_SIZING_VARIATION_LIMIT_REQUIRED",
    "HANGER_SIZING_VARIATION_REFERENCE_REQUIRED",
    "HANGER_SIZING_WEIGHT_CASE_REQUIRED",
    "HANGER_SIZING_WEIGHT_SUPPORT_STATE_REQUIRED",
    "HANGER_SIZING_WEIGHT_SUPPORT_STATE_UNSUPPORTED",
];

/// The model document's primitive-load categories (`schemas/model.schema.yaml`,
/// `LoadCategory`).
pub const LOAD_CATEGORIES: [&str; 11] = [
    "weight",
    "distributed_force",
    "pressure",
    "thermal",
    "hydrotest",
    "wind",
    "seismic",
    "occasional",
    "concentrated_force",
    "concentrated_moment",
    "imposed_displacement",
];

pub const STATE_CODES: [&str; 4] = ["inactive", "active", "sticking", "sliding"];

/// The SHA-256 (lowercase hex) of a value's checked canonical text, or `None`
/// when the value is outside the profile (or nested past the parse depth).
pub fn checked_sha256(value: &Value) -> Option<String> {
    let text = serde_json::to_string(value).ok()?;
    let canonical = canonical_json_checked_v1_text(&text).ok()?;
    Some(Sha256::digest(canonical.as_bytes()).iter().map(|b| format!("{b:02x}")).collect())
}

fn refusal(code: &'static str, names: Vec<String>, message: impl Into<String>) -> Refusal {
    Refusal { code, names, message: message.into() }
}

/// The outcome of the headless pipeline: the document it formed.
#[derive(Debug, Clone)]
pub enum Published {
    Sized(Value),
    Refused(Value),
}

impl Published {
    pub fn document(&self) -> &Value {
        match self {
            Published::Sized(doc) | Published::Refused(doc) => doc,
        }
    }
    pub fn into_document(self) -> Value {
        match self {
            Published::Sized(doc) | Published::Refused(doc) => doc,
        }
    }
    pub fn is_sized(&self) -> bool {
        matches!(self, Published::Sized(_))
    }
}

/// What a refused document states of its inputs (§8.3.3).
#[derive(Debug, Clone, Default)]
pub struct RefusedInputs {
    pub sizing_request: Option<Value>,
    pub solve_request_sha256: Option<String>,
    pub library_id: Option<String>,
    pub library_sha256: Option<String>,
}

fn identity() -> Map<String, Value> {
    let mut doc = Map::new();
    doc.insert("document_kind".into(), json!(DOCUMENT_KIND));
    doc.insert("schema_version".into(), json!(SCHEMA_VERSION));
    doc.insert("semantic_contract_id".into(), json!(SEMANTIC_CONTRACT_ID));
    doc.insert("route".into(), json!(ROUTE));
    doc.insert("limitations".into(), json!(LIMITATIONS));
    doc.insert("basis_statement".into(), json!(BASIS_STATEMENT));
    doc
}

fn inputs_member(sizing: Option<&Value>, solve_sha: Option<&str>, library_id: Option<&str>, library_sha: Option<&str>) -> Value {
    let sizing_sha = sizing.and_then(checked_sha256);
    json!({
        "canonical_profile": CANONICAL_PROFILE,
        "sizing_request": sizing.cloned().unwrap_or(Value::Null),
        "sizing_request_sha256": sizing_sha,
        "solve_request": {"sha256": solve_sha},
        "library": {"library_id": library_id, "sha256": library_sha},
    })
}

/// The refused document (§8.3.3). Refusals keep the engine's order.
pub fn refused_document(inputs: &RefusedInputs, refusals: &[Refusal]) -> Value {
    let mut doc = identity();
    doc.insert("status".into(), json!("refused"));
    doc.insert(
        "inputs".into(),
        inputs_member(
            inputs.sizing_request.as_ref(),
            inputs.solve_request_sha256.as_deref(),
            inputs.library_id.as_deref(),
            inputs.library_sha256.as_deref(),
        ),
    );
    let refusals: Vec<Value> = refusals
        .iter()
        .map(|r| {
            // The schema requires a non-empty message; a refusal without one
            // carries its code as its message.
            let message = if r.message.is_empty() { r.code.to_string() } else { r.message.clone() };
            json!({"code": r.code, "names": r.names, "message": message})
        })
        .collect();
    doc.insert("refusals".into(), Value::Array(refusals));
    Value::Object(doc)
}

const SIZING_KEYS: [&str; 7] = [
    "weight_case",
    "weight_support_state",
    "design_operating_case",
    "check_operating_cases",
    "candidates",
    "method",
    "solver_mode",
];
const METHOD_KEYS: [&str; 5] = ["load_design", "variation_reference", "variation_limit", "travel_allowance_m", "selection_rule"];
const CANDIDATE_KEYS: [&str; 2] = ["support", "family_policy"];

fn member_path(root: &str, key: &str) -> String {
    let mut path = root.to_string();
    push_key(&mut path, key);
    path
}

/// The paths of an object's missing and foreign members against a closed key set.
fn closed_findings(map: &Map<String, Value>, root: &str, keys: &[&str], out: &mut Vec<String>) {
    for key in keys {
        if !map.contains_key(*key) {
            out.push(member_path(root, key));
        }
    }
    for key in map.keys() {
        if !keys.contains(&key.as_str()) {
            out.push(member_path(root, key));
        }
    }
}

fn token<'v>(value: Option<&'v Value>, allowed: &[&str]) -> Option<&'v str> {
    value.and_then(Value::as_str).filter(|v| allowed.contains(v))
}

/// Reads a sizing request that must match `$defs.SizingRequest` exactly
/// (§8.3.4), or returns the path of every member that does not. The value
/// checks (a positive limit, roles, duplicates) are the engine's.
pub fn parse_sizing_request(value: &Value, root: &str) -> Result<SizingRequest, Vec<String>> {
    let mut bad = Vec::new();
    let Some(map) = value.as_object() else { return Err(vec![root.to_string()]) };
    closed_findings(map, root, &SIZING_KEYS, &mut bad);
    let at = |key: &str| member_path(root, key);
    let string = |key: &str, bad: &mut Vec<String>| -> String {
        match map.get(key) {
            Some(Value::String(s)) => s.clone(),
            Some(_) => {
                bad.push(at(key));
                String::new()
            }
            None => String::new(),
        }
    };
    let weight_case = string("weight_case", &mut bad);
    let design_operating_case = string("design_operating_case", &mut bad);
    if map.contains_key("weight_support_state") && token(map.get("weight_support_state"), &["own_solve"]).is_none() {
        bad.push(at("weight_support_state"));
    }
    let mut check_operating_cases = Vec::new();
    match map.get("check_operating_cases") {
        Some(Value::Array(items)) => {
            for (i, item) in items.iter().enumerate() {
                match item.as_str() {
                    Some(s) => check_operating_cases.push(s.to_string()),
                    None => bad.push(format!("{}[{i}]", at("check_operating_cases"))),
                }
            }
        }
        Some(_) => bad.push(at("check_operating_cases")),
        None => {}
    }
    let mut candidates = Vec::new();
    match map.get("candidates") {
        Some(Value::Array(items)) if !items.is_empty() => {
            for (i, item) in items.iter().enumerate() {
                let path = format!("{}[{i}]", at("candidates"));
                let Some(candidate) = item.as_object() else {
                    bad.push(path);
                    continue;
                };
                closed_findings(candidate, &path, &CANDIDATE_KEYS, &mut bad);
                let support = match candidate.get("support") {
                    Some(Value::String(s)) => Some(s.clone()),
                    Some(_) => {
                        bad.push(member_path(&path, "support"));
                        None
                    }
                    None => None,
                };
                let policy = match token(candidate.get("family_policy"), &["variable", "constant", "variable_else_constant"]) {
                    Some("variable") => Some(FamilyPolicy::Variable),
                    Some("constant") => Some(FamilyPolicy::Constant),
                    Some(_) => Some(FamilyPolicy::VariableElseConstant),
                    None => {
                        if candidate.contains_key("family_policy") {
                            bad.push(member_path(&path, "family_policy"));
                        }
                        None
                    }
                };
                if let (Some(support), Some(family_policy)) = (support, policy) {
                    candidates.push(RequestCandidate { support, family_policy });
                }
            }
        }
        Some(_) => bad.push(at("candidates")),
        None => {}
    }
    let mut method = None;
    match map.get("method") {
        Some(Value::Object(m)) => {
            let root = at("method");
            closed_findings(m, &root, &METHOD_KEYS, &mut bad);
            let mpath = |key: &str| member_path(&root, key);
            let load_design = match token(m.get("load_design"), &["hot", "cold"]) {
                Some("hot") => Some(LoadDesign::Hot),
                Some(_) => Some(LoadDesign::Cold),
                None => None,
            };
            let variation_reference = match token(m.get("variation_reference"), &["operating_load", "design_load"]) {
                Some("operating_load") => Some(VariationReference::OperatingLoad),
                Some(_) => Some(VariationReference::DesignLoad),
                None => None,
            };
            let selection_rule = match token(m.get("selection_rule"), &["smallest_capacity", "least_variation"]) {
                Some("smallest_capacity") => Some(SelectionRule::SmallestCapacity),
                Some(_) => Some(SelectionRule::LeastVariation),
                None => None,
            };
            for (key, ok) in [
                ("load_design", load_design.is_some()),
                ("variation_reference", variation_reference.is_some()),
                ("selection_rule", selection_rule.is_some()),
            ] {
                if m.contains_key(key) && !ok {
                    bad.push(mpath(key));
                }
            }
            let number = |key: &str, bad: &mut Vec<String>| match m.get(key) {
                Some(v) if v.is_number() => v.as_f64(),
                Some(_) => {
                    bad.push(mpath(key));
                    None
                }
                None => None,
            };
            let variation_limit = number("variation_limit", &mut bad);
            let travel_allowance_m = number("travel_allowance_m", &mut bad);
            if let (Some(load_design), Some(variation_reference), Some(variation_limit), Some(travel_allowance_m), Some(selection_rule)) =
                (load_design, variation_reference, variation_limit, travel_allowance_m, selection_rule)
            {
                method = Some(Method { load_design, variation_reference, variation_limit, travel_allowance_m, selection_rule });
            }
        }
        Some(_) => bad.push(at("method")),
        None => {}
    }
    let solver_mode = map.get("solver_mode").and_then(Value::as_str).and_then(PreviewSolverMode::from_wire);
    if map.contains_key("solver_mode") && solver_mode.is_none() {
        bad.push(at("solver_mode"));
    }
    if !bad.is_empty() {
        return Err(bad);
    }
    Ok(SizingRequest {
        weight_case,
        weight_support_state: WeightSupportState::OwnSolve,
        design_operating_case,
        check_operating_cases,
        candidates,
        method: method.expect("a complete method"),
        solver_mode: solver_mode.expect("a solver mode"),
    })
}

/// The input's shape (§8.3.2): exactly `{request: {model, materials: […]}, sizing, library}`.
fn shape_findings(input: &Value) -> Vec<String> {
    let mut bad = Vec::new();
    let Some(map) = input.as_object() else { return vec!["$".into()] };
    closed_findings(map, "$", &["request", "sizing", "library"], &mut bad);
    match map.get("request") {
        Some(Value::Object(request)) => {
            closed_findings(request, "$.request", &["model", "materials"], &mut bad);
            if request.get("model").is_some_and(|m| !m.is_object()) {
                bad.push("$.request.model".into());
            }
            if request.get("materials").is_some_and(|m| !m.is_array()) {
                bad.push("$.request.materials".into());
            }
        }
        Some(_) => bad.push("$.request".into()),
        None => {}
    }
    bad
}

/// The solves in publication order: `S_W`, `S_C`, `S_O1` … `S_On`, `V_C`,
/// `V_O1` … `V_On`, each with its case (§8.3.5).
pub fn solve_plan(request: &SizingRequest) -> Vec<(String, String)> {
    let weight = request.weight_case.clone();
    let operating = request.operating_cases();
    let mut plan = vec![("S_W".to_string(), weight.clone()), ("S_C".to_string(), weight.clone())];
    plan.extend(operating.iter().enumerate().map(|(i, case)| (format!("S_O{}", i + 1), case.to_string())));
    plan.push(("V_C".to_string(), weight));
    plan.extend(operating.iter().enumerate().map(|(i, case)| (format!("V_O{}", i + 1), case.to_string())));
    plan
}

/// The candidates' roles in a solve, by its label's family: rigid in `S_W`, the
/// forces W in `S_C` and `S_Oi`, the verification roles in `V_*`.
pub fn roles_for(label: &str, w: &[f64], verification: &[Role]) -> Vec<Role> {
    if label == "S_W" {
        vec![Role::Rigid; w.len()]
    } else if label.starts_with('S') {
        w.iter().map(|w| Role::Force(*w)).collect()
    } else {
        verification.to_vec()
    }
}

fn dof_token(dof: Dof) -> &'static str {
    match dof {
        Dof::Ux => "UX",
        Dof::Uy => "UY",
        Dof::Uz => "UZ",
    }
}

fn policy_token(policy: FamilyPolicy) -> &'static str {
    match policy {
        FamilyPolicy::Variable => "variable",
        FamilyPolicy::Constant => "constant",
        FamilyPolicy::VariableElseConstant => "variable_else_constant",
    }
}

fn outcome_token(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Selected => "selected",
        Outcome::SelectedByFallback => "selected_by_fallback",
        Outcome::NoRecordFits => "no_record_fits",
    }
}

/// Collects every number the document would publish, refusing one outside the
/// profile by its owner (candidate or solve) and member (§8.3.1).
struct Numbers {
    refusals: Vec<Refusal>,
}

impl Numbers {
    fn num(&mut self, owner: &str, member: &str, value: f64) -> Value {
        if in_profile(value) {
            json!(value)
        } else {
            self.refusals.push(refusal(RESULT_NOT_PUBLISHABLE, vec![owner.to_string(), member.to_string()],
                "a value the engine would publish is not finite or is outside the openpipestress_jcs_ijson_v1 profile"));
            Value::Null
        }
    }
}

fn counts(failures: &Option<std::collections::BTreeMap<Test, usize>>) -> Value {
    match failures {
        None => Value::Null,
        Some(map) => Value::Object(map.iter().map(|(test, n)| (test.name().to_string(), json!(n))).collect()),
    }
}

/// The index of the largest value, ties to the earlier.
pub fn argmax(values: impl Iterator<Item = f64>) -> usize {
    let mut best: Option<(usize, f64)> = None;
    for (i, v) in values.enumerate() {
        if best.map_or(true, |(_, b)| v > b) {
            best = Some((i, v));
        }
    }
    best.map_or(0, |(i, _)| i)
}

/// The record object of `hanger_records` whose `hanger_id` is `id`.
pub fn record_object<'a>(library_doc: &'a Value, id: &str) -> Vec<&'a Value> {
    library_doc
        .get("hanger_records")
        .and_then(Value::as_array)
        .map(|records| records.iter().filter(|r| r.get("hanger_id").and_then(Value::as_str) == Some(id)).collect())
        .unwrap_or_default()
}

fn uses_condition_set(request_doc: &Value, case: &str) -> bool {
    request_doc["model"]["load_cases"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|c| c.get("id").and_then(Value::as_str) == Some(case))
        .and_then(|c| c.get("condition_use"))
        .is_some_and(|v| !v.is_null())
}

/// The sized document (§8.3.5) from a completed run and its bound inputs, or
/// the publication refusals (`HANGER_SIZING_RESULT_NOT_PUBLISHABLE`,
/// `HANGER_SIZING_DERIVED_DOCUMENT_NOT_HASHABLE`), every finding listed.
pub fn sized_document(
    request_doc: &Value,
    sizing: &Value,
    library_doc: &Value,
    request: &SizingRequest,
    run: &SizingRun,
) -> Result<Value, Vec<Refusal>> {
    let records = library::records(library_doc);
    let operating = request.operating_cases();
    let mut numbers = Numbers { refusals: Vec::new() };
    let mut hash_refusals = Vec::new();

    let mut cases = Vec::new();
    for (i, case) in std::iter::once(request.weight_case.as_str()).chain(operating.iter().copied()).enumerate() {
        cases.push(json!({
            "case": case,
            "role": if i == 0 { "weight" } else { "operating" },
            "load_categories": run.case_load_categories.get(case).cloned().unwrap_or_default(),
            "uses_condition_set": uses_condition_set(request_doc, case),
        }));
    }

    let mut candidates = Vec::new();
    let mut verification_roles = Vec::new();
    for (index, ((site, candidate), result)) in run.sites.iter().zip(&run.candidates).zip(&run.results).enumerate() {
        let owner = site.support.as_str();
        let verification = &run.verifications[index];
        let w = numbers.num(owner, "w_n", candidate.w_n);
        let u_cold = numbers.num(owner, "u_cold_m", candidate.positions.u_cold_m);
        let positions: Vec<Value> = operating
            .iter()
            .zip(&candidate.positions.u_operating_m)
            .enumerate()
            .map(|(i, (case, u))| json!({"case": case, "u_m": numbers.num(owner, &format!("operating[{i}].u_m"), *u)}))
            .collect();
        let span = json!([numbers.num(owner, "span_m[0]", result.span.min_m), numbers.num(owner, "span_m[1]", result.span.max_m)]);
        let required = numbers.num(owner, "required_travel_m", result.required_travel_m);
        let travel_case = operating[argmax(candidate.positions.u_operating_m.iter().map(|u| (u - candidate.positions.u_cold_m).abs()))];
        let (selected, governing, assumption, role) = match &result.selected {
            None => (
                Value::Null,
                Value::Null,
                json!({"role": "force", "force_n": numbers.num(owner, "verification_assumption.force_n", candidate.w_n)}),
                Role::Force(candidate.w_n),
            ),
            Some(selection) => {
                let record = &records[selection.record_index()];
                let objects = record_object(library_doc, &record.hanger_id);
                let record_sha = objects.first().and_then(|o| checked_sha256(o));
                match selection {
                    Selected::Variable { k, loads, .. } => {
                        let entries: Vec<Value> = operating
                            .iter()
                            .enumerate()
                            .map(|(i, case)| json!({
                                "case": case,
                                "f_n": numbers.num(owner, &format!("selected.operating[{i}].f_n"), loads.f_operating_n[i]),
                                "variation": numbers.num(owner, &format!("selected.operating[{i}].variation"), loads.variation[i]),
                            }))
                            .collect();
                        let variation_case = operating[argmax(loads.variation.iter().copied())];
                        (
                            json!({
                                "family": "variable_spring",
                                "hanger_id": record.hanger_id,
                                "record_sha256": record_sha,
                                "k_n_per_m": numbers.num(owner, "selected.k_n_per_m", *k),
                                "f_cold_n": numbers.num(owner, "selected.f_cold_n", loads.f_cold_n),
                                "f_cold_condition_n": numbers.num(owner, "selected.f_cold_condition_n", loads.f_cold_condition_n),
                                "envelope_n": [numbers.num(owner, "selected.envelope_n[0]", loads.envelope_n.0),
                                               numbers.num(owner, "selected.envelope_n[1]", loads.envelope_n.1)],
                                "operating": entries,
                            }),
                            json!({"travel_case": travel_case, "variation_case": variation_case}),
                            json!({
                                "role": "spring",
                                "k_n_per_m": numbers.num(owner, "verification_assumption.k_n_per_m", *k),
                                "f_cold_n": numbers.num(owner, "verification_assumption.f_cold_n", loads.f_cold_n),
                            }),
                            Role::Spring { k: *k, f_cold: loads.f_cold_n },
                        )
                    }
                    Selected::Constant { load_n, .. } => (
                        json!({
                            "family": "constant_effort",
                            "hanger_id": record.hanger_id,
                            "record_sha256": record_sha,
                            "load_n": numbers.num(owner, "selected.load_n", *load_n),
                        }),
                        json!({"travel_case": travel_case, "variation_case": null}),
                        json!({"role": "force", "force_n": numbers.num(owner, "verification_assumption.force_n", candidate.w_n)}),
                        Role::Force(candidate.w_n),
                    ),
                }
            }
        };
        verification_roles.push(role);
        let verification = match verification {
            None => Value::Null,
            Some(v) => {
                let failed: Vec<&str> = v.failed_tests.iter().map(|t| t.name()).collect();
                let span = json!([numbers.num(owner, "verification.span_m[0]", v.span.min_m),
                                  numbers.num(owner, "verification.span_m[1]", v.span.max_m)]);
                let u_cold = numbers.num(owner, "verification.u_cold_m", v.positions.u_cold_m);
                let required = numbers.num(owner, "verification.required_travel_m", v.required_travel_m);
                match &v.variable {
                    Some(variable) => {
                        let entries: Vec<Value> = operating
                            .iter()
                            .enumerate()
                            .map(|(i, case)| json!({
                                "case": case,
                                "u_m": numbers.num(owner, &format!("verification.operating[{i}].u_m"), v.positions.u_operating_m[i]),
                                "f_n": numbers.num(owner, &format!("verification.operating[{i}].f_n"), variable.f_operating_n[i]),
                                "variation": variable.variation[i]
                                    .map_or(Value::Null, |x| numbers.num(owner, &format!("verification.operating[{i}].variation"), x)),
                            }))
                            .collect();
                        json!({
                            "family": "variable_spring",
                            "u_cold_m": u_cold,
                            "operating": entries,
                            "span_m": span,
                            "required_travel_m": required,
                            "t_pass": v.t_pass,
                            "f_cold_condition_n": numbers.num(owner, "verification.f_cold_condition_n", variable.f_cold_condition_n),
                            "departure_n": numbers.num(owner, "verification.departure_n", variable.departure_n),
                            "failed_tests": failed,
                        })
                    }
                    None => {
                        let entries: Vec<Value> = operating
                            .iter()
                            .enumerate()
                            .map(|(i, case)| json!({
                                "case": case,
                                "u_m": numbers.num(owner, &format!("verification.operating[{i}].u_m"), v.positions.u_operating_m[i]),
                            }))
                            .collect();
                        json!({
                            "family": "constant_effort",
                            "u_cold_m": u_cold,
                            "operating": entries,
                            "span_m": span,
                            "required_travel_m": required,
                            "t_pass": v.t_pass,
                            "failed_tests": failed,
                        })
                    }
                }
            }
        };
        candidates.push(json!({
            "support": owner,
            "node": site.node,
            "dof": dof_token(site.dof),
            "family_policy": policy_token(candidate.family_policy),
            "outcome": outcome_token(result.outcome),
            "w_n": w,
            "u_cold_m": u_cold,
            "operating": positions,
            "span_m": span,
            "required_travel_m": required,
            "failure_counts": {"variable": counts(&result.variable_failures), "constant": counts(&result.constant_failures)},
            "selected": selected,
            "governing": governing,
            "verification_assumption": assumption,
            "verification": verification,
        }));
    }

    let w: Vec<f64> = run.candidates.iter().map(|c| c.w_n).collect();
    let mut solves = Vec::new();
    for ((label, case), states) in solve_plan(request).into_iter().zip(&run.states) {
        debug_assert_eq!(label, states.solve);
        let roles = roles_for(&label, &w, &verification_roles);
        let hash = checked_sha256(&derived_document(request_doc, &case, &run.sites, &roles));
        if hash.is_none() {
            hash_refusals.push(refusal(DERIVED_DOCUMENT_NOT_HASHABLE, vec![label.clone()],
                "the derived document cannot be put in openpipestress_jcs_ijson_v1 canonical form"));
        }
        let mut map = Map::new();
        for (support, state) in &states.states {
            if !STATE_CODES.contains(&state.as_str()) {
                numbers.refusals.push(refusal(RESULT_NOT_PUBLISHABLE, vec![label.clone(), format!("states.{support}")],
                    "a nonlinear support's state code is not one of inactive, active, sticking or sliding"));
            }
            map.insert(support.clone(), json!(state));
        }
        solves.push(json!({"label": label, "case": case, "derived_document_sha256": hash, "states": map}));
    }

    let disclosures: Vec<Value> = run
        .disclosures
        .iter()
        .map(|d| json!({"code": d.code, "names": d.names, "message": d.message}))
        .collect();

    let mut refusals = numbers.refusals;
    refusals.extend(hash_refusals);
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let library_id = library_doc["hanger_library"]["library_id"].as_str();
    let mut doc = identity();
    doc.insert("status".into(), json!("sized"));
    doc.insert(
        "inputs".into(),
        inputs_member(Some(sizing), checked_sha256(request_doc).as_deref(), library_id, checked_sha256(library_doc).as_deref()),
    );
    doc.insert(
        "result".into(),
        json!({
            "verification_standing": match run.standing {
                VerificationStanding::Unconditional => "unconditional",
                VerificationStanding::Conditional => "conditional",
            },
            "cases": cases,
            "candidates": candidates,
            "solves": solves,
            "disclosures": disclosures,
        }),
    );
    Ok(Value::Object(doc))
}

/// The headless pipeline (§8.3.1, §8.3.2) on an input text the caller has read
/// and found within [`MAX_INPUT_BYTES`]: always a document, sized or refused.
pub fn size_hangers(text: &str) -> Published {
    match admit(text) {
        Ok(input) => size_admitted(&input),
        Err(finding) => not_admitted(finding),
    }
}

/// The refused document of an input the admission layer refused (§8.3.1).
pub(crate) fn not_admitted(finding: AdmissionFinding) -> Published {
    Published::Refused(refused_document(
        &RefusedInputs::default(),
        &[refusal(INPUT_NOT_ADMITTED, vec![finding.kind.as_str().to_string(), finding.path],
            "the input is outside the openpipestress_jcs_ijson_v1 profile; the finding names its kind and path")],
    ))
}

/// The pipeline after admission: shape, library import boundary, request, the
/// engine's run, then publication. [`size_hangers`] and the desktop entry
/// (`crate::desktop`) share it, so that an input is admitted once and published
/// the same way by both.
pub(crate) fn size_admitted(input: &Value) -> Published {
    let mut inputs = RefusedInputs {
        sizing_request: None,
        solve_request_sha256: input.get("request").and_then(checked_sha256),
        library_id: input.get("library").and_then(|l| l["hanger_library"]["library_id"].as_str()).map(str::to_string),
        library_sha256: input.get("library").and_then(checked_sha256),
    };
    let shape = shape_findings(input);
    if !shape.is_empty() {
        return Published::Refused(refused_document(&inputs, &[refusal(INPUT_SHAPE_INVALID, shape,
            "the input must be exactly {\"request\": {\"model\", \"materials\"}, \"sizing\", \"library\"}")]));
    }
    let (request_doc, sizing, library_doc) = (&input["request"], &input["sizing"], &input["library"]);
    let mut refusals = Vec::new();
    let import = validate_library_import(library_doc, LibraryKind::Hanger, IntendedVisibility::Private);
    if !import.accepted {
        let findings: Vec<String> = import.findings.iter().map(|f| format!("{} at {}", f.code, f.path)).collect();
        refusals.push(refusal(LIBRARY_NOT_ADMITTED, Vec::new(),
            format!("the library is not accepted at the import boundary ({}): {}", import.outcome, findings.join("; "))));
    } else if inputs.library_id.is_none() {
        refusals.push(refusal(LIBRARY_NOT_ADMITTED, Vec::new(), "the library carries no hanger_library.library_id string"));
    }
    let request = match parse_sizing_request(sizing, "$.sizing") {
        Ok(request) => {
            inputs.sizing_request = Some(sizing.clone());
            Some(request)
        }
        Err(paths) => {
            refusals.push(refusal(REQUEST_INVALID, paths, "the sizing request does not match $defs.SizingRequest exactly"));
            None
        }
    };
    if !refusals.is_empty() {
        return Published::Refused(refused_document(&inputs, &refusals));
    }
    let request = request.expect("an admitted request");
    let records = library::records(library_doc);
    let run = match derived::run(request_doc, &records, &request) {
        Ok(run) => run,
        Err(refusals) => return Published::Refused(refused_document(&inputs, &refusals)),
    };
    match sized_document(request_doc, sizing, library_doc, &request, &run) {
        Ok(doc) => Published::Sized(doc),
        Err(refusals) => Published::Refused(refused_document(&inputs, &refusals)),
    }
}
