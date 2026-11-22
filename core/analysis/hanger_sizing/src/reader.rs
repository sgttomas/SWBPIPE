//! The Rust product reader of a hanger-sizing result (DEL-04-11
//! design §8.3.7). It lives in this crate because a reader in
//! `result_export` would form a dependency cycle through `product_physics` (§13).
//!
//! Each standing is reported as passed, failed (with the first difference) or
//! not checked:
//! - `closure`: the closed schema, the variant and discriminator rules, the
//!   cross-field invariants and the fixed texts;
//! - `binding`: the inputs' hashes, for each input supplied;
//! - `arithmetic_consistent_with_published_positions`: the §8.3.6 arithmetic
//!   recomputed from the published W, positions and states with the request and
//!   the library. A document whose positions were invented consistently passes it;
//! - `derivation`: every derived document rebuilt from the solve request, the
//!   sizing request and the published roles, compared by hash;
//! - `reproduction`: the whole sizing re-run from the bound inputs through this
//!   build's engine, compared member by member, numbers as binary64. Claimed only
//!   for the same engine build on the same platform. It is the only standing that
//!   ties positions and states to the bound model.
//!
//! No standing states that mechanics are verified. When `closure` fails, nothing
//! else is checked. [`read_document_with`] can skip `reproduction` explicitly
//! ([`ReadOptions`]); [`read`] and [`read_document`] run every standing.

use crate::admission::admit;
use crate::derived::{self, derived_document, Role, SizingRequest};
use crate::publish::{
    self, checked_sha256, parse_sizing_request, record_object, roles_for, route_of, sized_document, solve_plan,
    BASIS_STATEMENT, CANONICAL_PROFILE, DOCUMENT_KIND, LIMITATIONS, LOAD_CATEGORIES, REFUSAL_CODES, ROUTES,
    SCHEMA_VERSION, SEMANTIC_CONTRACT_ID, STATE_CODES,
};
use crate::{check_request, library, size_candidate, verify, Candidate, Dof, FamilyPolicy, Outcome, Positions, Selected};
use serde_json::{json, Map, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Passed,
    Failed,
    NotChecked,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Passed => "passed",
            Status::Failed => "failed",
            Status::NotChecked => "not_checked",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Standing {
    pub status: Status,
    /// The first difference found, for a failed standing.
    pub difference: Option<String>,
}

impl Standing {
    fn passed() -> Self {
        Standing { status: Status::Passed, difference: None }
    }
    fn not_checked() -> Self {
        Standing { status: Status::NotChecked, difference: None }
    }
    fn of(result: Result<(), String>) -> Self {
        match result {
            Ok(()) => Standing::passed(),
            Err(difference) => Standing { status: Status::Failed, difference: Some(difference) },
        }
    }
    fn to_json(&self) -> Value {
        json!({"status": self.status.as_str(), "difference": self.difference})
    }
}

/// The inputs the caller supplies beside the document.
#[derive(Debug, Clone, Copy, Default)]
pub struct Supplied<'a> {
    /// The solve request `{model, materials}`.
    pub solve_request: Option<&'a Value>,
    /// The hanger library document.
    pub library: Option<&'a Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Verdict {
    pub closure: Standing,
    pub binding: Standing,
    /// The inputs `binding` could not check because they were not supplied.
    pub binding_not_checked: Vec<&'static str>,
    pub arithmetic: Standing,
    pub derivation: Standing,
    pub reproduction: Standing,
}

impl Verdict {
    fn closure_failed(difference: String) -> Self {
        Verdict {
            closure: Standing { status: Status::Failed, difference: Some(difference) },
            binding: Standing::not_checked(),
            binding_not_checked: Vec::new(),
            arithmetic: Standing::not_checked(),
            derivation: Standing::not_checked(),
            reproduction: Standing::not_checked(),
        }
    }

    pub fn to_json(&self) -> Value {
        let mut binding = self.binding.to_json();
        binding["not_checked"] = json!(self.binding_not_checked);
        json!({
            "closure": self.closure.to_json(),
            "binding": binding,
            "arithmetic_consistent_with_published_positions": self.arithmetic.to_json(),
            "derivation": self.derivation.to_json(),
            "reproduction": self.reproduction.to_json(),
            "integrity_bound": derived::integrity_bound(),
        })
    }
}

/// Reads a document text: admitted by the same rule as the headless input
/// (§8.3.1), then read.
pub fn read(text: &str, supplied: Supplied<'_>) -> Verdict {
    match admit(text) {
        Ok(doc) => read_document(&doc, supplied),
        Err(finding) => Verdict::closure_failed(format!("{}: not admitted ({})", finding.path, finding.kind.as_str())),
    }
}

/// Which of the optional standings a read runs. A standing a read does not run
/// is reported as `not_checked`, never as passed. There is no default: a caller
/// names its choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadOptions {
    /// Whether `reproduction` re-runs the sizing (all 2n + 3 solves) from the
    /// bound inputs. When false it is `not_checked`.
    pub reproduction: bool,
}

impl ReadOptions {
    /// Every standing the supplied inputs allow: what [`read`] and
    /// [`read_document`] run.
    pub const FULL: ReadOptions = ReadOptions { reproduction: true };
    /// Every standing but `reproduction`, which is `not_checked`.
    pub const WITHOUT_REPRODUCTION: ReadOptions = ReadOptions { reproduction: false };
}

/// Reads an admitted document with every standing ([`ReadOptions::FULL`]).
pub fn read_document(doc: &Value, supplied: Supplied<'_>) -> Verdict {
    read_document_with(doc, supplied, ReadOptions::FULL)
}

/// Reads an admitted document, running the optional standings `options` names.
pub fn read_document_with(doc: &Value, supplied: Supplied<'_>, options: ReadOptions) -> Verdict {
    if let Err(difference) = closure(doc) {
        return Verdict::closure_failed(difference);
    }
    let sized = doc["status"] == "sized";
    let (binding, binding_not_checked, library_bound, solve_bound) = binding(doc, supplied, sized);
    let request = parse_sizing_request(&doc["inputs"]["sizing_request"], "$.inputs.sizing_request").ok();
    let (arithmetic, derivation, reproduction) = match (sized, request) {
        (true, Some(request)) => (
            match supplied.library.filter(|_| library_bound) {
                Some(library) => Standing::of(arithmetic(doc, &request, library)),
                None => Standing::not_checked(),
            },
            match supplied.solve_request.filter(|_| solve_bound) {
                Some(solve) => Standing::of(derivation(doc, &request, solve)),
                None => Standing::not_checked(),
            },
            match (supplied.solve_request.filter(|_| solve_bound), supplied.library.filter(|_| library_bound)) {
                (Some(solve), Some(library)) if options.reproduction => Standing::of(reproduction(doc, &request, solve, library)),
                _ => Standing::not_checked(),
            },
        ),
        _ => (Standing::not_checked(), Standing::not_checked(), Standing::not_checked()),
    };
    Verdict { closure: Standing::passed(), binding, binding_not_checked, arithmetic, derivation, reproduction }
}

// --- closure ------------------------------------------------------------------

type R<T> = Result<T, String>;

const SIZED_KEYS: [&str; 9] = [
    "document_kind", "schema_version", "semantic_contract_id", "route", "limitations", "basis_statement", "status", "inputs", "result",
];
const REFUSED_KEYS: [&str; 9] = [
    "document_kind", "schema_version", "semantic_contract_id", "route", "limitations", "basis_statement", "status", "inputs", "refusals",
];
const INPUT_KEYS: [&str; 5] = ["canonical_profile", "sizing_request", "sizing_request_sha256", "solve_request", "library"];
const RESULT_KEYS: [&str; 6] = ["verification_standing", "integrity_bound", "cases", "candidates", "solves", "disclosures"];
const CANDIDATE_KEYS: [&str; 15] = [
    "support", "node", "dof", "family_policy", "outcome", "w_n", "u_cold_m", "operating", "span_m", "required_travel_m",
    "failure_counts", "selected", "governing", "verification_assumption", "verification",
];
const SOLVE_KEYS: [&str; 6] = ["label", "case", "derived_document_sha256", "states", "solver_standing", "support_state_confirmation"];
const VARIABLE_TESTS: [&str; 6] = ["E1", "E2", "V0", "L", "T", "V"];
const CONSTANT_TESTS: [&str; 3] = ["E1", "L", "T"];
/// The two state disclosures, each at most once in a document.
const DISCLOSURE_CODES: [&str; 2] = ["HANGER_SIZING_SUPPORT_STATE_DIFFERS", "HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS"];
/// The per-solve disclosures, each at most once per solve label of the document's
/// plan, naming exactly that label.
const SOLVE_DISCLOSURE_CODES: [&str; 2] = [derived::DERIVED_SOLVE_INTEGRITY, derived::DISPLACEMENT_CORRECTION_UNAVAILABLE];

fn at(path: &str, key: &str) -> String {
    format!("{path}.{key}")
}

fn obj<'v>(value: &'v Value, path: &str, keys: &[&str]) -> R<&'v Map<String, Value>> {
    let map = value.as_object().ok_or_else(|| format!("{path}: not an object"))?;
    if let Some(key) = keys.iter().find(|k| !map.contains_key(**k)) {
        return Err(format!("{}: missing", at(path, key)));
    }
    if let Some(key) = map.keys().find(|k| !keys.contains(&k.as_str())) {
        return Err(format!("{}: not a member of the closed object", at(path, key)));
    }
    Ok(map)
}

fn string<'v>(value: &'v Value, path: &str) -> R<&'v str> {
    value.as_str().ok_or_else(|| format!("{path}: not a string"))
}

fn number(value: &Value, path: &str) -> R<f64> {
    value.as_f64().filter(|_| value.is_number()).ok_or_else(|| format!("{path}: not a number"))
}

fn boolean(value: &Value, path: &str) -> R<bool> {
    value.as_bool().ok_or_else(|| format!("{path}: not a boolean"))
}

fn array<'v>(value: &'v Value, path: &str) -> R<&'v [Value]> {
    value.as_array().map(Vec::as_slice).ok_or_else(|| format!("{path}: not an array"))
}

fn one_of<'v>(value: &'v Value, path: &str, allowed: &[&str]) -> R<&'v str> {
    let text = string(value, path)?;
    if allowed.contains(&text) {
        Ok(text)
    } else {
        Err(format!("{path}: not one of {allowed:?}"))
    }
}

fn constant(value: &Value, path: &str, expected: &str) -> R<()> {
    if value.as_str() == Some(expected) {
        Ok(())
    } else {
        Err(format!("{path}: not {expected:?}"))
    }
}

fn hash<'v>(value: &'v Value, path: &str) -> R<&'v str> {
    let text = string(value, path)?;
    if text.len() == 64 && text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        Ok(text)
    } else {
        Err(format!("{path}: not a lowercase SHA-256 hex digest"))
    }
}

fn nullable_hash<'v>(value: &'v Value, path: &str) -> R<Option<&'v str>> {
    if value.is_null() {
        Ok(None)
    } else {
        hash(value, path).map(Some)
    }
}

/// A non-negative integer (a JSON number with an integral value).
fn count(value: &Value, path: &str) -> R<f64> {
    let n = number(value, path)?;
    if n >= 0.0 && n.fract() == 0.0 {
        Ok(n)
    } else {
        Err(format!("{path}: not a non-negative integer"))
    }
}

fn texts(value: &Value, path: &str, expected: &[&str]) -> R<()> {
    let items = array(value, path)?;
    if items.len() != expected.len() || items.iter().zip(expected).any(|(item, text)| item.as_str() != Some(*text)) {
        return Err(format!("{path}: not the fixed text"));
    }
    Ok(())
}

fn equal(a: f64, b: f64, path: &str, what: &str) -> R<()> {
    if a == b {
        Ok(())
    } else {
        Err(format!("{path}: {what} ({a} against {b})"))
    }
}

/// The fixed members. A sized document states the route its solves ran on; a
/// refused one states its solve request's route, or null when no model was read.
fn identity(top: &Map<String, Value>, sized: bool) -> R<()> {
    constant(&top["document_kind"], "$.document_kind", DOCUMENT_KIND)?;
    constant(&top["schema_version"], "$.schema_version", SCHEMA_VERSION)?;
    constant(&top["semantic_contract_id"], "$.semantic_contract_id", SEMANTIC_CONTRACT_ID)?;
    if sized || !top["route"].is_null() {
        one_of(&top["route"], "$.route", &ROUTES)?;
    }
    texts(&top["limitations"], "$.limitations", &LIMITATIONS)?;
    texts(&top["basis_statement"], "$.basis_statement", &BASIS_STATEMENT)
}

fn sizing_request(value: &Value, path: &str) -> R<SizingRequest> {
    parse_sizing_request(value, path).map_err(|paths| format!("{}: does not match $defs.SizingRequest", paths[0]))
}

/// The closed schema, the variant rules, the cross-field invariants and the fixed texts.
pub fn closure(doc: &Value) -> R<()> {
    match doc.get("status").and_then(Value::as_str) {
        Some("sized") => closure_sized(doc),
        Some("refused") => closure_refused(doc),
        _ => Err("$.status: neither \"sized\" nor \"refused\"".into()),
    }
}

fn closure_refused(doc: &Value) -> R<()> {
    let top = obj(doc, "$", &REFUSED_KEYS)?;
    identity(top, false)?;
    let inputs = obj(&top["inputs"], "$.inputs", &INPUT_KEYS)?;
    constant(&inputs["canonical_profile"], "$.inputs.canonical_profile", CANONICAL_PROFILE)?;
    let sizing = &inputs["sizing_request"];
    if !sizing.is_null() {
        sizing_request(sizing, "$.inputs.sizing_request")?;
    }
    let sizing_sha = nullable_hash(&inputs["sizing_request_sha256"], "$.inputs.sizing_request_sha256")?;
    if sizing.is_null() != sizing_sha.is_none() {
        return Err("$.inputs.sizing_request_sha256: null exactly when sizing_request is null".into());
    }
    let solve = obj(&inputs["solve_request"], "$.inputs.solve_request", &["sha256"])?;
    nullable_hash(&solve["sha256"], "$.inputs.solve_request.sha256")?;
    let library = obj(&inputs["library"], "$.inputs.library", &["library_id", "sha256"])?;
    if !library["library_id"].is_null() {
        string(&library["library_id"], "$.inputs.library.library_id")?;
    }
    nullable_hash(&library["sha256"], "$.inputs.library.sha256")?;
    let refusals = array(&top["refusals"], "$.refusals")?;
    if refusals.is_empty() {
        return Err("$.refusals: empty".into());
    }
    for (i, refusal) in refusals.iter().enumerate() {
        let path = format!("$.refusals[{i}]");
        let refusal = obj(refusal, &path, &["code", "names", "message"])?;
        one_of(&refusal["code"], &at(&path, "code"), &REFUSAL_CODES)?;
        for (j, name) in array(&refusal["names"], &at(&path, "names"))?.iter().enumerate() {
            string(name, &format!("{path}.names[{j}]"))?;
        }
        if string(&refusal["message"], &at(&path, "message"))?.is_empty() {
            return Err(format!("{path}.message: empty"));
        }
    }
    Ok(())
}

fn closure_sized(doc: &Value) -> R<()> {
    let top = obj(doc, "$", &SIZED_KEYS)?;
    identity(top, true)?;
    let inputs = obj(&top["inputs"], "$.inputs", &INPUT_KEYS)?;
    constant(&inputs["canonical_profile"], "$.inputs.canonical_profile", CANONICAL_PROFILE)?;
    let request = sizing_request(&inputs["sizing_request"], "$.inputs.sizing_request")?;
    hash(&inputs["sizing_request_sha256"], "$.inputs.sizing_request_sha256")?;
    let solve = obj(&inputs["solve_request"], "$.inputs.solve_request", &["sha256"])?;
    hash(&solve["sha256"], "$.inputs.solve_request.sha256")?;
    let library = obj(&inputs["library"], "$.inputs.library", &["library_id", "sha256"])?;
    string(&library["library_id"], "$.inputs.library.library_id")?;
    hash(&library["sha256"], "$.inputs.library.sha256")?;

    let result = obj(&top["result"], "$.result", &RESULT_KEYS)?;
    let standing = one_of(&result["verification_standing"], "$.result.verification_standing", &["unconditional", "conditional"])?;
    let bound = obj(&result["integrity_bound"], "$.result.integrity_bound", &["status", "reason"])?;
    constant(&bound["status"], "$.result.integrity_bound.status", derived::INTEGRITY_BOUND_STATUS)?;
    constant(&bound["reason"], "$.result.integrity_bound.reason", derived::INTEGRITY_BOUND_REASON)?;
    let operating = request.operating_cases();
    let named: Vec<&str> = std::iter::once(request.weight_case.as_str()).chain(operating.iter().copied()).collect();

    let cases = array(&result["cases"], "$.result.cases")?;
    if cases.len() != named.len() {
        return Err(format!("$.result.cases: {} entries for {} named cases", cases.len(), named.len()));
    }
    for (i, (case, name)) in cases.iter().zip(&named).enumerate() {
        let path = format!("$.result.cases[{i}]");
        let case = obj(case, &path, &["case", "role", "load_categories", "uses_condition_set"])?;
        if string(&case["case"], &at(&path, "case"))? != *name {
            return Err(format!("{path}.case: not the request's case {name:?}"));
        }
        let role = one_of(&case["role"], &at(&path, "role"), &["weight", "operating"])?;
        if role != if i == 0 { "weight" } else { "operating" } {
            return Err(format!("{path}.role: not the case's role in the request"));
        }
        let categories = array(&case["load_categories"], &at(&path, "load_categories"))?;
        let mut previous: Option<&str> = None;
        for (j, category) in categories.iter().enumerate() {
            let category = one_of(category, &format!("{path}.load_categories[{j}]"), &LOAD_CATEGORIES)?;
            if previous.is_some_and(|p| p.as_bytes() >= category.as_bytes()) {
                return Err(format!("{path}.load_categories: not sorted and unique"));
            }
            previous = Some(category);
        }
        boolean(&case["uses_condition_set"], &at(&path, "uses_condition_set"))?;
    }
    let mut unique = named.clone();
    unique.sort_unstable();
    unique.dedup();
    if unique.len() != named.len() {
        return Err("$.result.cases: case ids not unique".into());
    }

    let candidates = array(&result["candidates"], "$.result.candidates")?;
    if candidates.len() != request.candidates.len() {
        return Err(format!("$.result.candidates: {} entries for {} requested", candidates.len(), request.candidates.len()));
    }
    let mut supports = Vec::new();
    let mut no_fit = false;
    for (i, (candidate, asked)) in candidates.iter().zip(&request.candidates).enumerate() {
        let path = format!("$.result.candidates[{i}]");
        let candidate = obj(candidate, &path, &CANDIDATE_KEYS)?;
        let support = string(&candidate["support"], &at(&path, "support"))?;
        if support != asked.support {
            return Err(format!("{path}.support: not the request's candidate {:?}", asked.support));
        }
        supports.push(support);
        no_fit |= closure_candidate(candidate, &path, asked.family_policy, &operating)?;
    }
    let mut unique = supports.clone();
    unique.sort_unstable();
    unique.dedup();
    if unique.len() != supports.len() {
        return Err("$.result.candidates: support not unique".into());
    }
    if (standing == "conditional") != no_fit {
        return Err("$.result.verification_standing: not conditional exactly when some candidate has no fit".into());
    }

    let plan = solve_plan(&request);
    let solves = array(&result["solves"], "$.result.solves")?;
    if solves.len() != plan.len() {
        return Err(format!("$.result.solves: {} entries, expected {}", solves.len(), plan.len()));
    }
    let mut standings: Vec<&str> = Vec::new();
    let mut confirmations: Vec<&'static str> = Vec::new();
    for (i, (solve, (label, case))) in solves.iter().zip(&plan).enumerate() {
        let path = format!("$.result.solves[{i}]");
        let solve = obj(solve, &path, &SOLVE_KEYS)?;
        if string(&solve["label"], &at(&path, "label"))? != label {
            return Err(format!("{path}.label: not {label:?}"));
        }
        if string(&solve["case"], &at(&path, "case"))? != case {
            return Err(format!("{path}.case: not the case of {label}"));
        }
        hash(&solve["derived_document_sha256"], &at(&path, "derived_document_sha256"))?;
        let states = solve["states"].as_object().ok_or_else(|| format!("{path}.states: not an object"))?;
        for (support, state) in states {
            if support.is_empty() {
                return Err(format!("{path}.states: an empty support id"));
            }
            one_of(state, &format!("{path}.states.{support}"), &STATE_CODES)?;
        }
        standings.push(one_of(&solve["solver_standing"], &at(&path, "solver_standing"), &derived::SOLVER_STANDINGS)?);
        confirmations.push(closure_confirmation(&solve["support_state_confirmation"], &at(&path, "support_state_confirmation"), states.is_empty())?);
    }

    let disclosures = array(&result["disclosures"], "$.result.disclosures")?;
    let mut codes = Vec::new();
    let mut solve_pairs: Vec<(&str, &str)> = Vec::new();
    // Each integrity disclosure's solve label, standing token and index.
    let mut integrity: Vec<(&str, &str, usize)> = Vec::new();
    // Each displacement-correction-unavailable disclosure's solve label and index.
    let mut correction_unavailable: Vec<(&str, usize)> = Vec::new();
    let all_codes: Vec<&str> = DISCLOSURE_CODES.iter().chain(&SOLVE_DISCLOSURE_CODES).copied().collect();
    for (i, disclosure) in disclosures.iter().enumerate() {
        let path = format!("$.result.disclosures[{i}]");
        let disclosure = obj(disclosure, &path, &["code", "names", "message"])?;
        let code = one_of(&disclosure["code"], &at(&path, "code"), &all_codes)?;
        if SOLVE_DISCLOSURE_CODES.contains(&code) {
            // A per-solve disclosure names one solve of this document's plan, once per
            // code; the integrity disclosure's message is fixed by its standing token.
            let names = array(&disclosure["names"], &at(&path, "names"))?;
            let [name] = names else {
                return Err(format!("{path}.names: not exactly one solve label"));
            };
            let label = string(name, &format!("{path}.names[0]"))?;
            if !plan.iter().any(|(planned, _)| planned == label) {
                return Err(format!("{path}.names[0]: not a solve label of this document"));
            }
            if solve_pairs.contains(&(code, label)) {
                return Err(format!("{path}: repeated for solve {label}"));
            }
            solve_pairs.push((code, label));
            let message = string(&disclosure["message"], &at(&path, "message"))?;
            if code == derived::DERIVED_SOLVE_INTEGRITY {
                let token = message.split(':').next().unwrap_or_default();
                if !derived::NONPASSING_STANDINGS.contains(&token) || message != derived::integrity_message(token) {
                    return Err(format!("{path}.message: not the integrity message of a standing other than solver_checks_passed"));
                }
                integrity.push((label, token, i));
            } else if message.is_empty() {
                return Err(format!("{path}.message: empty"));
            } else {
                correction_unavailable.push((label, i));
            }
            continue;
        }
        if codes.contains(&code) {
            return Err(format!("{path}.code: repeated"));
        }
        codes.push(code);
        let names = array(&disclosure["names"], &at(&path, "names"))?;
        if names.is_empty() {
            return Err(format!("{path}.names: empty"));
        }
        let mut previous: Option<&str> = None;
        for (j, name) in names.iter().enumerate() {
            let name = string(name, &format!("{path}.names[{j}]"))?;
            if previous.is_some_and(|p| p.as_bytes() >= name.as_bytes()) {
                return Err(format!("{path}.names: not unique and sorted bytewise"));
            }
            previous = Some(name);
        }
        string(&disclosure["message"], &at(&path, "message"))?;
    }
    // The integrity disclosures and the solves' standings are one statement: a solve
    // other than solver_checks_passed has exactly its own disclosure with its token,
    // and a passing solve has none.
    for (i, ((label, _), standing)) in plan.iter().zip(&standings).enumerate() {
        match integrity.iter().find(|entry| entry.0 == label.as_str()) {
            Some((_, token, j)) if token != standing => {
                return Err(format!("$.result.disclosures[{j}]: the integrity disclosure of {label} states {token}, not its solver_standing {standing}"));
            }
            None if *standing != derived::SOLVER_CHECKS_PASSED => {
                return Err(format!("$.result.solves[{i}].solver_standing: {standing} without its {} disclosure", derived::DERIVED_SOLVE_INTEGRITY));
            }
            _ => {}
        }
    }
    // The producer withdraws a case's confirmed state when its displacement
    // correction fails, so a solve with that disclosure never reports it established.
    for (i, ((label, _), outcome)) in plan.iter().zip(&confirmations).enumerate() {
        if *outcome != derived::SOLVER_REPORTED_ESTABLISHED {
            continue;
        }
        if let Some((_, j)) = correction_unavailable.iter().find(|entry| entry.0 == label.as_str()) {
            return Err(format!(
                "$.result.solves[{i}].support_state_confirmation.outcome: {} beside the {} disclosure of {label} at $.result.disclosures[{j}]",
                derived::SOLVER_REPORTED_ESTABLISHED,
                derived::DISPLACEMENT_CORRECTION_UNAVAILABLE
            ));
        }
    }
    Ok(())
}

/// A solve's `support_state_confirmation`: a closed `{outcome, detail}`, the
/// outcome `not_applicable` exactly when the solve publishes no nonlinear-support
/// state, and `detail` a non-blank string ([`derived::is_blank_detail`]) exactly
/// when the outcome is `not_established_detail_quoted`, null otherwise. Returns the
/// outcome's closed token.
fn closure_confirmation(value: &Value, path: &str, stateless: bool) -> R<&'static str> {
    let confirmation = obj(value, path, &["outcome", "detail"])?;
    let text = one_of(&confirmation["outcome"], &at(path, "outcome"), &derived::CONFIRMATION_OUTCOMES)?;
    let outcome = derived::CONFIRMATION_OUTCOMES.into_iter().find(|token| *token == text).unwrap_or_default();
    if stateless != (outcome == derived::NOT_APPLICABLE) {
        return Err(format!("{path}.outcome: not {:?} exactly when the solve publishes no nonlinear-support state", derived::NOT_APPLICABLE));
    }
    let detail = &confirmation["detail"];
    if outcome == derived::DETAIL_QUOTED {
        if derived::is_blank_detail(string(detail, &at(path, "detail"))?) {
            return Err(format!("{path}.detail: blank"));
        }
    } else if !detail.is_null() {
        return Err(format!("{path}.detail: not null for the outcome {outcome:?}"));
    }
    Ok(outcome)
}

/// A per-case list: one entry per operating case, its `case` ids in order.
fn per_case<'v>(value: &'v Value, path: &str, operating: &[&str], keys: &[&str]) -> R<Vec<&'v Map<String, Value>>> {
    let items = array(value, path)?;
    if items.len() != operating.len() {
        return Err(format!("{path}: {} entries for {} operating cases", items.len(), operating.len()));
    }
    let mut out = Vec::new();
    for (i, (item, case)) in items.iter().zip(operating).enumerate() {
        let item_path = format!("{path}[{i}]");
        let item = obj(item, &item_path, keys)?;
        if string(&item["case"], &at(&item_path, "case"))? != *case {
            return Err(format!("{item_path}.case: not the operating case {case:?}"));
        }
        out.push(item);
    }
    Ok(out)
}

fn span(value: &Value, path: &str) -> R<()> {
    let items = array(value, path)?;
    if items.len() != 2 {
        return Err(format!("{path}: not a pair"));
    }
    let (min, max) = (number(&items[0], &format!("{path}[0]"))?, number(&items[1], &format!("{path}[1]"))?);
    if min > max {
        return Err(format!("{path}: min above max"));
    }
    Ok(())
}

fn failure_counts(value: &Value, path: &str, tests: &[&str]) -> R<()> {
    if value.is_null() {
        return Ok(());
    }
    let map = obj(value, path, tests)?;
    for test in tests {
        count(&map[*test], &at(path, test))?;
    }
    Ok(())
}

fn failed_tests(value: &Value, path: &str, order: &[&str]) -> R<()> {
    let mut last: Option<usize> = None;
    for (i, item) in array(value, path)?.iter().enumerate() {
        let name = one_of(item, &format!("{path}[{i}]"), order)?;
        let index = order.iter().position(|t| *t == name).expect("listed");
        if last.is_some_and(|l| l >= index) {
            return Err(format!("{path}: not an ordered subset of {order:?}"));
        }
        last = Some(index);
    }
    Ok(())
}

/// One candidate's closure; returns whether its outcome is `no_record_fits`.
fn closure_candidate(c: &Map<String, Value>, path: &str, policy: FamilyPolicy, operating: &[&str]) -> R<bool> {
    string(&c["node"], &at(path, "node"))?;
    one_of(&c["dof"], &at(path, "dof"), &["UX", "UY", "UZ"])?;
    let policy_token = match policy {
        FamilyPolicy::Variable => "variable",
        FamilyPolicy::Constant => "constant",
        FamilyPolicy::VariableElseConstant => "variable_else_constant",
    };
    constant(&c["family_policy"], &at(path, "family_policy"), policy_token)?;
    let outcome = one_of(&c["outcome"], &at(path, "outcome"), &["selected", "selected_by_fallback", "no_record_fits"])?;
    let w = number(&c["w_n"], &at(path, "w_n"))?;
    number(&c["u_cold_m"], &at(path, "u_cold_m"))?;
    for (i, entry) in per_case(&c["operating"], &at(path, "operating"), operating, &["case", "u_m"])?.iter().enumerate() {
        number(&entry["u_m"], &format!("{path}.operating[{i}].u_m"))?;
    }
    span(&c["span_m"], &at(path, "span_m"))?;
    number(&c["required_travel_m"], &at(path, "required_travel_m"))?;
    let counts = obj(&c["failure_counts"], &at(path, "failure_counts"), &["variable", "constant"])?;
    failure_counts(&counts["variable"], &format!("{path}.failure_counts.variable"), &VARIABLE_TESTS)?;
    failure_counts(&counts["constant"], &format!("{path}.failure_counts.constant"), &CONSTANT_TESTS)?;

    let no_fit = outcome == "no_record_fits";
    let assumption_path = at(path, "verification_assumption");
    let assumption_role = c["verification_assumption"].get("role").and_then(Value::as_str);
    if no_fit {
        for key in ["selected", "governing", "verification"] {
            if !c[key].is_null() {
                return Err(format!("{}: not null for no fit", at(path, key)));
            }
        }
        let assumption = obj(&c["verification_assumption"], &assumption_path, &["role", "force_n"])?;
        constant(&assumption["role"], &at(&assumption_path, "role"), "force")?;
        equal(number(&assumption["force_n"], &at(&assumption_path, "force_n"))?, w, &assumption_path, "force_n is not w_n")?;
        return Ok(true);
    }

    let selected_path = at(path, "selected");
    let family = c["selected"].get("family").and_then(Value::as_str);
    let governing_path = at(path, "governing");
    let governing = obj(&c["governing"], &governing_path, &["travel_case", "variation_case"])?;
    one_of(&governing["travel_case"], &at(&governing_path, "travel_case"), operating)?;
    let verification_path = at(path, "verification");
    match family {
        Some("variable_spring") => {
            let s = obj(
                &c["selected"],
                &selected_path,
                &["family", "hanger_id", "record_sha256", "k_n_per_m", "f_cold_n", "f_cold_condition_n", "envelope_n", "operating"],
            )?;
            string(&s["hanger_id"], &at(&selected_path, "hanger_id"))?;
            hash(&s["record_sha256"], &at(&selected_path, "record_sha256"))?;
            let k = number(&s["k_n_per_m"], &at(&selected_path, "k_n_per_m"))?;
            let f_cold = number(&s["f_cold_n"], &at(&selected_path, "f_cold_n"))?;
            number(&s["f_cold_condition_n"], &at(&selected_path, "f_cold_condition_n"))?;
            let envelope = array(&s["envelope_n"], &at(&selected_path, "envelope_n"))?;
            if envelope.len() != 2 {
                return Err(format!("{selected_path}.envelope_n: not a pair"));
            }
            for (i, end) in envelope.iter().enumerate() {
                number(end, &format!("{selected_path}.envelope_n[{i}]"))?;
            }
            let entries = per_case(&s["operating"], &at(&selected_path, "operating"), operating, &["case", "f_n", "variation"])?;
            for (i, entry) in entries.iter().enumerate() {
                number(&entry["f_n"], &format!("{selected_path}.operating[{i}].f_n"))?;
                number(&entry["variation"], &format!("{selected_path}.operating[{i}].variation"))?;
            }
            one_of(&governing["variation_case"], &at(&governing_path, "variation_case"), operating)?;
            if assumption_role != Some("spring") {
                return Err(format!("{assumption_path}.role: not spring for a variable selection"));
            }
            let assumption = obj(&c["verification_assumption"], &assumption_path, &["role", "k_n_per_m", "f_cold_n"])?;
            equal(number(&assumption["k_n_per_m"], &at(&assumption_path, "k_n_per_m"))?, k, &assumption_path, "k_n_per_m is not the selection's")?;
            equal(number(&assumption["f_cold_n"], &at(&assumption_path, "f_cold_n"))?, f_cold, &assumption_path, "f_cold_n is not the selection's")?;
            let v = obj(
                &c["verification"],
                &verification_path,
                &["family", "u_cold_m", "operating", "span_m", "required_travel_m", "t_pass", "f_cold_condition_n", "departure_n", "failed_tests"],
            )?;
            constant(&v["family"], &at(&verification_path, "family"), "variable_spring")?;
            number(&v["u_cold_m"], &at(&verification_path, "u_cold_m"))?;
            let entries = per_case(&v["operating"], &at(&verification_path, "operating"), operating, &["case", "u_m", "f_n", "variation"])?;
            for (i, entry) in entries.iter().enumerate() {
                let entry_path = format!("{verification_path}.operating[{i}]");
                number(&entry["u_m"], &at(&entry_path, "u_m"))?;
                number(&entry["f_n"], &at(&entry_path, "f_n"))?;
                if !entry["variation"].is_null() {
                    number(&entry["variation"], &at(&entry_path, "variation"))?;
                }
            }
            span(&v["span_m"], &at(&verification_path, "span_m"))?;
            number(&v["required_travel_m"], &at(&verification_path, "required_travel_m"))?;
            boolean(&v["t_pass"], &at(&verification_path, "t_pass"))?;
            number(&v["f_cold_condition_n"], &at(&verification_path, "f_cold_condition_n"))?;
            number(&v["departure_n"], &at(&verification_path, "departure_n"))?;
            failed_tests(&v["failed_tests"], &at(&verification_path, "failed_tests"), &["V0", "L", "T", "V"])?;
        }
        Some("constant_effort") => {
            let s = obj(&c["selected"], &selected_path, &["family", "hanger_id", "record_sha256", "load_n"])?;
            string(&s["hanger_id"], &at(&selected_path, "hanger_id"))?;
            hash(&s["record_sha256"], &at(&selected_path, "record_sha256"))?;
            equal(number(&s["load_n"], &at(&selected_path, "load_n"))?, w, &selected_path, "load_n is not w_n")?;
            if !governing["variation_case"].is_null() {
                return Err(format!("{governing_path}.variation_case: not null for a constant selection"));
            }
            if assumption_role != Some("force") {
                return Err(format!("{assumption_path}.role: not force for a constant selection"));
            }
            let assumption = obj(&c["verification_assumption"], &assumption_path, &["role", "force_n"])?;
            equal(number(&assumption["force_n"], &at(&assumption_path, "force_n"))?, w, &assumption_path, "force_n is not w_n")?;
            let v = obj(
                &c["verification"],
                &verification_path,
                &["family", "u_cold_m", "operating", "span_m", "required_travel_m", "t_pass", "failed_tests"],
            )?;
            constant(&v["family"], &at(&verification_path, "family"), "constant_effort")?;
            number(&v["u_cold_m"], &at(&verification_path, "u_cold_m"))?;
            let entries = per_case(&v["operating"], &at(&verification_path, "operating"), operating, &["case", "u_m"])?;
            for (i, entry) in entries.iter().enumerate() {
                number(&entry["u_m"], &format!("{verification_path}.operating[{i}].u_m"))?;
            }
            span(&v["span_m"], &at(&verification_path, "span_m"))?;
            number(&v["required_travel_m"], &at(&verification_path, "required_travel_m"))?;
            boolean(&v["t_pass"], &at(&verification_path, "t_pass"))?;
            failed_tests(&v["failed_tests"], &at(&verification_path, "failed_tests"), &["T"])?;
        }
        _ => return Err(format!("{selected_path}: not a variable or constant selection for outcome {outcome}")),
    }
    Ok(false)
}

// --- binding ------------------------------------------------------------------

fn hash_matches(value: &Value, published: &Value, path: &str) -> R<()> {
    match checked_sha256(value) {
        Some(digest) if published.as_str() == Some(digest.as_str()) => Ok(()),
        Some(_) => Err(format!("{path}: does not match the supplied input's hash")),
        None => Err(format!("{path}: the supplied input cannot be hashed under the profile")),
    }
}

fn note(result: R<()>, first: &mut Option<String>) {
    if let Err(difference) = result {
        first.get_or_insert(difference);
    }
}

/// The binding standing, the inputs not checked, and whether the supplied
/// library and solve request are each hash-matched.
fn binding(doc: &Value, supplied: Supplied<'_>, sized: bool) -> (Standing, Vec<&'static str>, bool, bool) {
    let inputs = &doc["inputs"];
    let mut checked = false;
    let mut first: Option<String> = None;
    let mut not_checked = Vec::new();
    if !inputs["sizing_request"].is_null() {
        checked = true;
        note(hash_matches(&inputs["sizing_request"], &inputs["sizing_request_sha256"], "$.inputs.sizing_request_sha256"), &mut first);
    }
    let mut library_bound = false;
    match supplied.library {
        Some(library) if sized => {
            checked = true;
            let hashed = hash_matches(library, &inputs["library"]["sha256"], "$.inputs.library.sha256");
            let id = if library["hanger_library"]["library_id"] == inputs["library"]["library_id"] {
                Ok(())
            } else {
                Err("$.inputs.library.library_id: not the supplied library's".to_string())
            };
            library_bound = hashed.is_ok() && id.is_ok();
            note(hashed, &mut first);
            note(id, &mut first);
            for (i, candidate) in doc["result"]["candidates"].as_array().into_iter().flatten().enumerate() {
                let selected = &candidate["selected"];
                if selected.is_null() {
                    continue;
                }
                let id = selected["hanger_id"].as_str().unwrap_or_default();
                let path = format!("$.result.candidates[{i}].selected");
                match record_object(library, id).as_slice() {
                    [record] => note(hash_matches(record, &selected["record_sha256"], &format!("{path}.record_sha256")), &mut first),
                    [] => note(Err(format!("{path}.hanger_id: no record of the supplied library")), &mut first),
                    _ => note(Err(format!("{path}.hanger_id: not unique in the supplied library")), &mut first),
                }
            }
        }
        Some(library) => {
            if !inputs["library"]["sha256"].is_null() {
                checked = true;
                note(hash_matches(library, &inputs["library"]["sha256"], "$.inputs.library.sha256"), &mut first);
            }
        }
        None => not_checked.push("library"),
    }
    let mut solve_bound = false;
    match supplied.solve_request {
        Some(solve) => {
            if !inputs["solve_request"]["sha256"].is_null() {
                checked = true;
                let result = hash_matches(solve, &inputs["solve_request"]["sha256"], "$.inputs.solve_request.sha256");
                solve_bound = result.is_ok();
                note(result, &mut first);
                if solve_bound && doc["route"].as_str() != route_of(solve) {
                    note(Err("$.route: not the supplied solve request's route".to_string()), &mut first);
                }
            }
        }
        None => not_checked.push("solve_request"),
    }
    let standing = match first {
        Some(difference) => Standing { status: Status::Failed, difference: Some(difference) },
        None if checked => Standing::passed(),
        None => Standing::not_checked(),
    };
    (standing, not_checked, library_bound, solve_bound)
}

// --- arithmetic ---------------------------------------------------------------

fn f(value: &Value) -> f64 {
    value.as_f64().expect("closure checked the number")
}

fn parse_dof(token: &str) -> Dof {
    match token {
        "UX" => Dof::Ux,
        "UY" => Dof::Uy,
        _ => Dof::Uz,
    }
}

fn published_candidates(doc: &Value, request: &SizingRequest) -> Vec<Candidate> {
    doc["result"]["candidates"]
        .as_array()
        .expect("closure")
        .iter()
        .zip(&request.candidates)
        .map(|(c, asked)| Candidate {
            id: c["support"].as_str().expect("closure").to_string(),
            dof: parse_dof(c["dof"].as_str().expect("closure")),
            family_policy: asked.family_policy,
            w_n: f(&c["w_n"]),
            positions: Positions {
                u_cold_m: f(&c["u_cold_m"]),
                u_operating_m: c["operating"].as_array().expect("closure").iter().map(|e| f(&e["u_m"])).collect(),
            },
            design_index: 0,
        })
        .collect()
}

fn outcome_token(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Selected => "selected",
        Outcome::SelectedByFallback => "selected_by_fallback",
        Outcome::NoRecordFits => "no_record_fits",
    }
}

fn compare_counts(published: &Value, expected: &Option<std::collections::BTreeMap<crate::Test, usize>>, path: &str) -> R<()> {
    match expected {
        None if published.is_null() => Ok(()),
        None => Err(format!("{path}: published for a family that was not tested")),
        Some(_) if published.is_null() => Err(format!("{path}: null for a family that was tested")),
        Some(map) => {
            for (test, n) in map {
                equal(f(&published[test.name()]), *n as f64, &at(path, test.name()), "count differs")?;
            }
            Ok(())
        }
    }
}

/// The §8.3.6 arithmetic recomputed from the published W and positions with
/// the request and the library, compared by numeric equality.
fn arithmetic(doc: &Value, request: &SizingRequest, library_doc: &Value) -> R<()> {
    let method = &request.method;
    let records = library::records(library_doc);
    let operating = request.operating_cases();
    let candidates = published_candidates(doc, request);
    let refusals = check_request(&candidates, &records, method);
    if !refusals.is_empty() {
        let codes: Vec<&str> = refusals.iter().map(|r| r.code).collect();
        return Err(format!("$.result: the engine refuses these published inputs ({})", codes.join(", ")));
    }
    for (i, (candidate, c)) in candidates.iter().zip(doc["result"]["candidates"].as_array().expect("closure")).enumerate() {
        let path = format!("$.result.candidates[{i}]");
        let expected = size_candidate(candidate, &records, method);
        if c["outcome"] != outcome_token(expected.outcome) {
            return Err(format!("{path}.outcome: recomputed {}", outcome_token(expected.outcome)));
        }
        equal(f(&c["span_m"][0]), expected.span.min_m, &format!("{path}.span_m[0]"), "differs")?;
        equal(f(&c["span_m"][1]), expected.span.max_m, &format!("{path}.span_m[1]"), "differs")?;
        equal(f(&c["required_travel_m"]), expected.required_travel_m, &at(&path, "required_travel_m"), "differs")?;
        compare_counts(&c["failure_counts"]["variable"], &expected.variable_failures, &format!("{path}.failure_counts.variable"))?;
        compare_counts(&c["failure_counts"]["constant"], &expected.constant_failures, &format!("{path}.failure_counts.constant"))?;
        let travel = operating[publish::argmax(candidate.positions.u_operating_m.iter().map(|u| (u - candidate.positions.u_cold_m).abs()))];
        let selected_path = at(&path, "selected");
        let s = &c["selected"];
        match &expected.selected {
            None => {}
            Some(selection) => {
                let record = &records[selection.record_index()];
                if s["hanger_id"] != record.hanger_id.as_str() {
                    return Err(format!("{selected_path}.hanger_id: recomputed {:?}", record.hanger_id));
                }
                if c["governing"]["travel_case"] != travel {
                    return Err(format!("{path}.governing.travel_case: recomputed {travel:?}"));
                }
                match selection {
                    Selected::Variable { k, loads, .. } => {
                        if s["family"] != "variable_spring" {
                            return Err(format!("{selected_path}.family: recomputed variable_spring"));
                        }
                        equal(f(&s["k_n_per_m"]), *k, &at(&selected_path, "k_n_per_m"), "differs")?;
                        equal(f(&s["f_cold_n"]), loads.f_cold_n, &at(&selected_path, "f_cold_n"), "differs")?;
                        equal(f(&s["f_cold_condition_n"]), loads.f_cold_condition_n, &at(&selected_path, "f_cold_condition_n"), "differs")?;
                        equal(f(&s["envelope_n"][0]), loads.envelope_n.0, &format!("{selected_path}.envelope_n[0]"), "differs")?;
                        equal(f(&s["envelope_n"][1]), loads.envelope_n.1, &format!("{selected_path}.envelope_n[1]"), "differs")?;
                        for (j, entry) in s["operating"].as_array().expect("closure").iter().enumerate() {
                            equal(f(&entry["f_n"]), loads.f_operating_n[j], &format!("{selected_path}.operating[{j}].f_n"), "differs")?;
                            equal(f(&entry["variation"]), loads.variation[j], &format!("{selected_path}.operating[{j}].variation"), "differs")?;
                        }
                        let variation = operating[publish::argmax(loads.variation.iter().copied())];
                        if c["governing"]["variation_case"] != variation {
                            return Err(format!("{path}.governing.variation_case: recomputed {variation:?}"));
                        }
                    }
                    Selected::Constant { load_n, .. } => {
                        if s["family"] != "constant_effort" {
                            return Err(format!("{selected_path}.family: recomputed constant_effort"));
                        }
                        equal(f(&s["load_n"]), *load_n, &at(&selected_path, "load_n"), "differs")?;
                    }
                }
                let v = &c["verification"];
                let verified = Positions {
                    u_cold_m: f(&v["u_cold_m"]),
                    u_operating_m: v["operating"].as_array().expect("closure").iter().map(|e| f(&e["u_m"])).collect(),
                };
                let recomputed = verify(candidate, &expected, &records, method, &verified)
                    .map_err(|r| format!("{path}.verification: {}", r.code))?
                    .expect("a selection is verified");
                let vpath = at(&path, "verification");
                equal(f(&v["span_m"][0]), recomputed.span.min_m, &format!("{vpath}.span_m[0]"), "differs")?;
                equal(f(&v["span_m"][1]), recomputed.span.max_m, &format!("{vpath}.span_m[1]"), "differs")?;
                equal(f(&v["required_travel_m"]), recomputed.required_travel_m, &at(&vpath, "required_travel_m"), "differs")?;
                if v["t_pass"] != recomputed.t_pass {
                    return Err(format!("{vpath}.t_pass: recomputed {}", recomputed.t_pass));
                }
                let failed: Vec<&str> = recomputed.failed_tests.iter().map(|t| t.name()).collect();
                if v["failed_tests"] != json!(failed) {
                    return Err(format!("{vpath}.failed_tests: recomputed {failed:?}"));
                }
                if let Some(variable) = &recomputed.variable {
                    equal(f(&v["f_cold_condition_n"]), variable.f_cold_condition_n, &at(&vpath, "f_cold_condition_n"), "differs")?;
                    equal(f(&v["departure_n"]), variable.departure_n, &at(&vpath, "departure_n"), "differs")?;
                    for (j, entry) in v["operating"].as_array().expect("closure").iter().enumerate() {
                        let entry_path = format!("{vpath}.operating[{j}]");
                        equal(f(&entry["f_n"]), variable.f_operating_n[j], &at(&entry_path, "f_n"), "differs")?;
                        match variable.variation[j] {
                            None if entry["variation"].is_null() => {}
                            None => return Err(format!("{entry_path}.variation: recomputed null")),
                            Some(_) if entry["variation"].is_null() => return Err(format!("{entry_path}.variation: recomputed a ratio")),
                            Some(x) => equal(f(&entry["variation"]), x, &at(&entry_path, "variation"), "differs")?,
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

// --- derivation and reproduction ---------------------------------------------

fn verification_roles(doc: &Value) -> Vec<Role> {
    doc["result"]["candidates"]
        .as_array()
        .expect("closure")
        .iter()
        .map(|c| {
            let a = &c["verification_assumption"];
            if a["role"] == "spring" {
                Role::Spring { k: f(&a["k_n_per_m"]), f_cold: f(&a["f_cold_n"]) }
            } else {
                Role::Force(f(&a["force_n"]))
            }
        })
        .collect()
}

/// Each derived document rebuilt from the solve request, the sizing request
/// and the published roles, compared by hash.
fn derivation(doc: &Value, request: &SizingRequest, solve_request: &Value) -> R<()> {
    let sites = derived::check_candidates(solve_request, request).map_err(|refusals| {
        let codes: Vec<&str> = refusals.iter().map(|r| r.code).collect();
        format!("$.inputs.sizing_request.candidates: refused against the solve request ({})", codes.join(", "))
    })?;
    let w: Vec<f64> = doc["result"]["candidates"].as_array().expect("closure").iter().map(|c| f(&c["w_n"])).collect();
    let roles = verification_roles(doc);
    for (i, ((label, case), solve)) in solve_plan(request).iter().zip(doc["result"]["solves"].as_array().expect("closure")).enumerate() {
        let rebuilt = checked_sha256(&derived_document(solve_request, case, &sites, &roles_for(label, &w, &roles)));
        if solve["derived_document_sha256"].as_str() != rebuilt.as_deref() {
            return Err(format!("$.result.solves[{i}].derived_document_sha256: the rebuilt {label} document hashes otherwise"));
        }
    }
    Ok(())
}

/// Numeric-aware equality: numbers as binary64 (−0 equal to 0), all else exactly.
fn same(a: &Value, b: &Value, path: &str) -> R<()> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => equal(x.as_f64().unwrap_or(f64::NAN), y.as_f64().unwrap_or(f64::NAN), path, "differs"),
        (Value::Object(x), Value::Object(y)) => {
            if x.len() != y.len() || x.keys().any(|k| !y.contains_key(k)) {
                return Err(format!("{path}: members differ"));
            }
            x.iter().try_for_each(|(k, v)| same(v, &y[k], &at(path, k)))
        }
        (Value::Array(x), Value::Array(y)) => {
            if x.len() != y.len() {
                return Err(format!("{path}: lengths differ"));
            }
            x.iter().zip(y).enumerate().try_for_each(|(i, (u, v))| same(u, v, &format!("{path}[{i}]")))
        }
        _ if a == b => Ok(()),
        _ => Err(format!("{path}: differs")),
    }
}

/// The sizing re-run from the bound inputs through this build's engine and
/// compared with the whole published document.
fn reproduction(doc: &Value, request: &SizingRequest, solve_request: &Value, library_doc: &Value) -> R<()> {
    let records = library::records(library_doc);
    let run = derived::run(solve_request, &records, request).map_err(|refusals| {
        let codes: Vec<&str> = refusals.iter().map(|r| r.code).collect();
        format!("$: the bound inputs are refused on re-run ({})", codes.join(", "))
    })?;
    let again = sized_document(solve_request, &doc["inputs"]["sizing_request"], library_doc, request, &run).map_err(|refusals| {
        let codes: Vec<&str> = refusals.iter().map(|r| r.code).collect();
        format!("$: the re-run is not publishable ({})", codes.join(", "))
    })?;
    same(doc, &again, "$")
}

/// The closure of the per-solve disclosures on the committed two-span document
/// (labels S_W, S_C, S_O1, S_O2, V_C, V_O1, V_O2), changed in memory only.
#[cfg(test)]
mod disclosure_closure_tests {
    use super::*;

    const TWO_SPAN: &str = include_str!("../../../../fixtures/results/hanger_sizing_result_two_span.json");
    /// The product's warning on the forced-failure fixture, unchanged.
    const PRODUCT_WARNING: &str = "Load case load:VACUUM: the corrected displacement of node:A:UX could not be certified (forced by test); no published result of this case relies on the member-relative formation check.";

    fn with(disclosures: Value) -> Value {
        let mut doc: Value = serde_json::from_str(TWO_SPAN).expect("fixture JSON");
        assert_eq!(doc["result"]["disclosures"], json!([]));
        doc["result"]["disclosures"] = disclosures;
        doc
    }

    fn integrity(label: &str, token: &str) -> Value {
        json!({"code": derived::DERIVED_SOLVE_INTEGRITY, "names": [label], "message": derived::integrity_message(token)})
    }

    fn unavailable(label: &str) -> Value {
        json!({"code": derived::DISPLACEMENT_CORRECTION_UNAVAILABLE, "names": [label], "message": PRODUCT_WARNING})
    }

    fn state(code: &str) -> Value {
        json!({"code": code, "names": ["support:L"], "message": "invented"})
    }

    fn refused(disclosures: Value, reason: &str) {
        let error = closure(&with(disclosures)).expect_err("closure refuses");
        assert!(error.contains(reason), "{error}");
    }

    /// The solve labelled `label` in the two-span plan.
    fn solve_mut<'d>(doc: &'d mut Value, label: &str) -> &'d mut Value {
        doc["result"]["solves"].as_array_mut().expect("solves").iter_mut().find(|s| s["label"] == label).expect("label")
    }

    /// [`with`], each integrity disclosure's solve standing as its token and each
    /// displacement-correction-unavailable solve's confirmation withdrawn, as the
    /// producer withdraws it, to `no_confirmation_record`.
    fn with_standings(disclosures: Value) -> Value {
        let mut doc = with(disclosures.clone());
        for disclosure in disclosures.as_array().expect("disclosures") {
            let label = disclosure["names"][0].as_str().expect("label");
            if disclosure["code"] == derived::DERIVED_SOLVE_INTEGRITY {
                let token = disclosure["message"].as_str().expect("message").split(':').next().expect("token").to_string();
                solve_mut(&mut doc, label)["solver_standing"] = json!(token);
            } else if disclosure["code"] == derived::DISPLACEMENT_CORRECTION_UNAVAILABLE {
                solve_mut(&mut doc, label)["support_state_confirmation"] = json!({"outcome": "no_confirmation_record", "detail": null});
            }
        }
        doc
    }

    fn refused_doc(doc: &Value, reason: &str) {
        let error = closure(doc).expect_err("closure refuses");
        assert!(error.contains(reason), "{error}");
    }

    #[test]
    fn every_committed_solve_states_its_standing_and_confirmation() {
        let doc: Value = serde_json::from_str(TWO_SPAN).expect("fixture JSON");
        for solve in doc["result"]["solves"].as_array().expect("solves") {
            assert_eq!(solve["solver_standing"], derived::SOLVER_CHECKS_PASSED, "{solve}");
            assert_eq!(solve["support_state_confirmation"], json!({"outcome": "solver_reported_established", "detail": null}), "{solve}");
        }
    }

    #[test]
    fn the_solver_standing_and_the_integrity_disclosures_are_one_statement() {
        // A non-passing standing without its disclosure.
        let mut doc = with(json!([]));
        solve_mut(&mut doc, "S_O1")["solver_standing"] = json!("sensitive");
        refused_doc(&doc, "$.result.solves[2].solver_standing: sensitive without its HANGER_SIZING_DERIVED_SOLVE_INTEGRITY disclosure");
        // A disclosure beside a passing standing, and one with another token.
        refused(json!([integrity("S_O1", "sensitive")]), "$.result.disclosures[0]: the integrity disclosure of S_O1 states sensitive, not its solver_standing solver_checks_passed");
        let mut doc = with_standings(json!([integrity("S_O1", "sensitive")]));
        solve_mut(&mut doc, "S_O1")["solver_standing"] = json!("failed");
        refused_doc(&doc, "the integrity disclosure of S_O1 states sensitive, not its solver_standing failed");
        // Tokens outside the closed vocabulary, the solver's wire token among them.
        for token in [json!("checks_passed"), json!("verified"), json!("SOLVER_CHECKS_PASSED"), json!(null), json!(1)] {
            let mut doc = with(json!([]));
            solve_mut(&mut doc, "V_C")["solver_standing"] = token.clone();
            let error = closure(&doc).expect_err("closure refuses");
            assert!(error.starts_with("$.result.solves[4].solver_standing: not"), "{token}: {error}");
        }
        let mut doc = with(json!([]));
        solve_mut(&mut doc, "S_W").as_object_mut().expect("solve").remove("solver_standing");
        refused_doc(&doc, "$.result.solves[0].solver_standing: missing");
    }

    #[test]
    fn the_support_state_confirmation_is_closed() {
        let changed = |change: &dyn Fn(&mut Value)| {
            let mut doc = with(json!([]));
            change(solve_mut(&mut doc, "S_C"));
            doc
        };
        let path = "$.result.solves[1].support_state_confirmation";
        refused_doc(&changed(&|s| { s.as_object_mut().expect("solve").remove("support_state_confirmation"); }), &format!("{path}: missing"));
        refused_doc(&changed(&|s| s["support_state_confirmation"] = json!("solver_reported_established")), &format!("{path}: not an object"));
        refused_doc(&changed(&|s| s["support_state_confirmation"]["outcome"] = json!("verified")), &format!("{path}.outcome: not one of"));
        refused_doc(&changed(&|s| s["support_state_confirmation"]["extra"] = json!(1)), &format!("{path}.extra: not a member of the closed object"));
        refused_doc(&changed(&|s| { s["support_state_confirmation"].as_object_mut().expect("c").remove("detail"); }), &format!("{path}.detail: missing"));
        // not_applicable exactly when the solve has no nonlinear-support state.
        refused_doc(&changed(&|s| s["support_state_confirmation"]["outcome"] = json!("not_applicable")),
            &format!("{path}.outcome: not \"not_applicable\" exactly when"));
        refused_doc(&changed(&|s| s["states"] = json!({})), &format!("{path}.outcome: not \"not_applicable\" exactly when"));
        let stateless = changed(&|s| {
            s["states"] = json!({});
            s["support_state_confirmation"] = json!({"outcome": "not_applicable", "detail": null});
        });
        assert_eq!(closure(&stateless), Ok(()));
        // A detail exactly for a quoted reason, never blank.
        refused_doc(&changed(&|s| s["support_state_confirmation"]["detail"] = json!("invented")), &format!("{path}.detail: not null"));
        let quoted = |detail: Value| changed(&move |s| s["support_state_confirmation"] = json!({"outcome": "not_established_detail_quoted", "detail": detail.clone()}));
        assert_eq!(closure(&quoted(json!("support support:C-lift: margin below its bound"))), Ok(()));
        refused_doc(&quoted(json!(null)), &format!("{path}.detail: not a string"));
        refused_doc(&quoted(json!("")), &format!("{path}.detail: blank"));
        refused_doc(&quoted(json!(" \t")), &format!("{path}.detail: blank"));
        for blank in ["  ", "\u{0085}", "\u{FEFF}", "\u{0085}\u{FEFF}"] {
            refused_doc(&quoted(json!(blank)), &format!("{path}.detail: blank"));
        }
        assert_eq!(closure(&quoted(json!("support:α"))), Ok(()));
        for outcome in ["solver_reported_established", "not_attempted_reason_not_recorded", "no_confirmation_record"] {
            assert_eq!(closure(&changed(&|s| s["support_state_confirmation"]["outcome"] = json!(outcome))), Ok(()), "{outcome}");
        }
    }

    #[test]
    fn both_new_codes_on_different_valid_labels_with_the_state_warnings_are_closed() {
        let doc = with_standings(json!([
            state("HANGER_SIZING_SUPPORT_STATE_DIFFERS"),
            state("HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS"),
            integrity("S_W", "not_assessed"),
            integrity("S_O1", "sensitive"),
            unavailable("S_O1"),
            integrity("V_O2", "unresolved"),
            unavailable("V_O2"),
            integrity("V_C", "failed"),
        ]));
        assert_eq!(closure(&doc), Ok(()));
        let verdict = read_document_with(&doc, Supplied::default(), ReadOptions::WITHOUT_REPRODUCTION);
        assert_eq!(verdict.closure.status, Status::Passed);
    }

    #[test]
    fn an_established_confirmation_beside_an_unavailable_correction_is_refused() {
        // The producer withdraws the confirmed state when the correction fails: a
        // solve with that disclosure never reports its state established.
        let mut doc = with_standings(json!([integrity("S_O1", "sensitive"), unavailable("S_O1")]));
        assert_eq!(closure(&doc), Ok(()));
        solve_mut(&mut doc, "S_O1")["support_state_confirmation"] = json!({"outcome": derived::SOLVER_REPORTED_ESTABLISHED, "detail": null});
        refused_doc(&doc, "$.result.solves[2].support_state_confirmation.outcome: solver_reported_established beside the DISPLACEMENT_CORRECTION_UNAVAILABLE disclosure of S_O1 at $.result.disclosures[1]");
        // Another solve's established confirmation is unaffected.
        assert_eq!(solve_mut(&mut doc, "S_O2")["support_state_confirmation"]["outcome"], derived::SOLVER_REPORTED_ESTABLISHED);
        solve_mut(&mut doc, "S_O1")["support_state_confirmation"] = json!({"outcome": "no_confirmation_record", "detail": null});
        assert_eq!(closure(&doc), Ok(()));
    }

    #[test]
    fn unknown_codes_invalid_labels_repeats_and_malformed_members_are_refused() {
        refused(json!([{"code": "HANGER_SIZING_UNKNOWN", "names": ["S_W"], "message": "x"}]), "code: not one of");
        refused(json!([{"code": "NUMERICAL_INTEGRITY_SENSITIVE", "names": ["S_W"], "message": "x"}]), "code: not one of");
        refused(json!([integrity("S_O3", "sensitive")]), "names[0]: not a solve label of this document");
        refused(json!([unavailable("V_O3")]), "names[0]: not a solve label of this document");
        refused(json!([integrity("source", "sensitive")]), "not a solve label");
        refused(json!([integrity("S_O1", "sensitive"), integrity("S_O1", "failed")]), "repeated for solve S_O1");
        refused(json!([unavailable("V_C"), unavailable("V_C")]), "repeated for solve V_C");
        refused(json!([state("HANGER_SIZING_SUPPORT_STATE_DIFFERS"), state("HANGER_SIZING_SUPPORT_STATE_DIFFERS")]), "code: repeated");
        refused(json!([{"code": derived::DERIVED_SOLVE_INTEGRITY, "names": ["S_W", "S_C"], "message": derived::integrity_message("sensitive")}]),
            "not exactly one solve label");
        refused(json!([{"code": derived::DISPLACEMENT_CORRECTION_UNAVAILABLE, "names": [], "message": PRODUCT_WARNING}]), "not exactly one solve label");
        refused(json!([{"code": derived::DERIVED_SOLVE_INTEGRITY, "names": [7], "message": derived::integrity_message("sensitive")}]), "not a string");
        refused(json!([integrity("S_W", "checks_passed")]), "not the integrity message");
        refused(json!([integrity("S_W", "solver_checks_passed")]), "not the integrity message");
        // The message PR 272 first published, before the bound was stated as not verified.
        refused(json!([{"code": derived::DERIVED_SOLVE_INTEGRITY, "names": ["S_W"],
            "message": "sensitive: the numerical-integrity standing of this derived solve is sensitive, not checks_passed; every value sizing takes from it carries that standing"}]),
            "not the integrity message of a standing other than solver_checks_passed");
        refused(json!([integrity("S_W", "SENSITIVE")]), "not the integrity message");
        refused(json!([{"code": derived::DERIVED_SOLVE_INTEGRITY, "names": ["S_W"], "message": "sensitive"}]), "not the integrity message");
        refused(json!([{"code": derived::DISPLACEMENT_CORRECTION_UNAVAILABLE, "names": ["S_W"], "message": ""}]), "message: empty");
        refused(json!([{"code": derived::DISPLACEMENT_CORRECTION_UNAVAILABLE, "names": ["S_W"], "message": PRODUCT_WARNING, "extra": 1}]),
            "not a member of the closed object");
    }

    #[test]
    fn the_integrity_bound_is_closed_and_fixed() {
        let doc: Value = serde_json::from_str(TWO_SPAN).expect("fixture JSON");
        assert_eq!(doc["result"]["integrity_bound"], derived::integrity_bound());
        assert_eq!(closure(&doc), Ok(()));
        let verdict = read_document_with(&doc, Supplied::default(), ReadOptions::WITHOUT_REPRODUCTION).to_json();
        assert_eq!(verdict["integrity_bound"], json!({"status": "not_verified", "reason": "inverse_norm_bound_not_verified"}));
        let refused_bound = |change: &dyn Fn(&mut Value), reason: &str| {
            let mut changed = doc.clone();
            change(&mut changed);
            let error = closure(&changed).expect_err("closure refuses");
            assert!(error.contains(reason), "{error}");
        };
        refused_bound(&|d| { d["result"].as_object_mut().expect("result").remove("integrity_bound"); }, "$.result.integrity_bound: missing");
        refused_bound(&|d| d["result"]["integrity_bound"]["status"] = json!("verified"), "$.result.integrity_bound.status: not");
        refused_bound(&|d| d["result"]["integrity_bound"]["reason"] = json!("invented_reason"), "$.result.integrity_bound.reason: not");
        refused_bound(&|d| d["result"]["integrity_bound"]["bound"] = json!(1.0), "$.result.integrity_bound.bound: not a member of the closed object");
        refused_bound(&|d| d["result"]["integrity_bound"] = json!("not_verified"), "$.result.integrity_bound: not an object");
    }
}
