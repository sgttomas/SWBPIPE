//! The derived solves of hanger sizing (DEL-04-11 design §2 to §5
//! and §7). Each derived solve is a modified copy of the caller's solve request,
//! `{model, materials}`, sent through the product's public JSON solve entry, the
//! one the model's own cases use. The stored model is never changed, and nothing
//! here passes through the operation route.

use crate::{
    check_method, size, verification_standing, verify, Candidate, CandidateResult, Dof, FamilyPolicy, Method,
    Positions, Record, Refusal, Selected, Verification, VerificationStanding,
};
use open_pipe_stress_product_physics::source_admission::admit_source_before_assembly;
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, Diagnostic, MechanicsEnvelope, NumericalQualityStatus, PreviewSolverMode,
};
use open_pipe_stress_units::{canonical_unit, convert, unit_by_symbol, Dimension};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// The nonlinear-support state the hot-load basis is formed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightSupportState {
    /// The weight case's own converged states. The only value admitted until the
    /// solve can hold another case's states.
    OwnSolve,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestCandidate {
    pub support: String,
    pub family_policy: FamilyPolicy,
}

/// The sizing request. Every member is the person's input.
#[derive(Debug, Clone)]
pub struct SizingRequest {
    pub weight_case: String,
    pub weight_support_state: WeightSupportState,
    pub design_operating_case: String,
    pub check_operating_cases: Vec<String>,
    pub candidates: Vec<RequestCandidate>,
    pub method: Method,
    /// The solver mode the caller's own solve of these cases uses.
    pub solver_mode: PreviewSolverMode,
}

impl SizingRequest {
    /// The design operating case first, then the check cases, in request order.
    pub fn operating_cases(&self) -> Vec<&str> {
        std::iter::once(self.design_operating_case.as_str())
            .chain(self.check_operating_cases.iter().map(String::as_str))
            .collect()
    }
}

/// A candidate's support, node and DOF in the model.
#[derive(Debug, Clone, PartialEq)]
pub struct Site {
    pub support: String,
    pub node: String,
    pub dof: Dof,
}

/// A non-blocking disclosure of the run.
#[derive(Debug, Clone, PartialEq)]
pub struct Disclosure {
    pub code: &'static str,
    pub names: Vec<String>,
    pub message: String,
}

/// What the solver's integrity record of one derived solve's case states of its
/// support-state confirmation ([`support_state_confirmation`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportStateConfirmation {
    /// One of [`CONFIRMATION_OUTCOMES`].
    pub outcome: String,
    /// The solver's quoted reason, present exactly when `outcome` is
    /// `not_established_detail_quoted`; published as `null` otherwise.
    pub detail: Option<String>,
}

impl SupportStateConfirmation {
    fn of(outcome: &str, detail: Option<String>) -> Self {
        SupportStateConfirmation { outcome: outcome.to_string(), detail }
    }

    /// The published `support_state_confirmation` member: `{outcome, detail}`,
    /// `detail` null when there is none.
    pub fn to_json(&self) -> Value {
        json!({"outcome": self.outcome, "detail": self.detail})
    }
}

/// One derived solve's own record: each nonlinear support's converged state, the
/// solver's numerical-integrity standing ([`standing_token`]) and what its
/// integrity record states of the support-state confirmation.
#[derive(Debug, Clone, PartialEq)]
pub struct SolveStates {
    pub solve: String,
    pub states: BTreeMap<String, String>,
    /// One of [`SOLVER_STANDINGS`].
    pub solver_standing: &'static str,
    pub support_state_confirmation: SupportStateConfirmation,
}

/// A completed sizing run: the hanger table's values, before publication.
#[derive(Debug, Clone)]
pub struct SizingRun {
    pub sites: Vec<Site>,
    pub candidates: Vec<Candidate>,
    pub results: Vec<CandidateResult>,
    pub verifications: Vec<Option<Verification>>,
    pub standing: VerificationStanding,
    pub states: Vec<SolveStates>,
    pub disclosures: Vec<Disclosure>,
    /// The authored primitive-load categories of each named case.
    pub case_load_categories: BTreeMap<String, Vec<String>>,
}

fn refusal(code: &'static str, names: Vec<String>, message: impl Into<String>) -> Refusal {
    Refusal { code, names, message: message.into() }
}

fn parse_dof(token: &str) -> Option<Dof> {
    match token.to_ascii_uppercase().as_str() {
        "UX" => Some(Dof::Ux),
        "UY" => Some(Dof::Uy),
        "UZ" => Some(Dof::Uz),
        _ => None,
    }
}

fn axis(dof: Dof) -> &'static str {
    match dof {
        Dof::Ux => "x",
        Dof::Uy => "y",
        Dof::Uz => "z",
    }
}

fn dof_token(dof: Dof) -> &'static str {
    match dof {
        Dof::Ux => "UX",
        Dof::Uy => "UY",
        Dof::Uz => "UZ",
    }
}

fn is_present(value: Option<&Value>) -> bool {
    value.is_some_and(|v| !v.is_null())
}

fn model(request: &Value) -> &Value {
    &request["model"]
}

fn array<'a>(value: &'a Value, key: &str) -> &'a [Value] {
    value.get(key).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
}

/// Case checks: every named case exists, carries no test condition and has
/// one role. Both routes are sized; the route is the request's own.
pub fn check_cases(request_doc: &Value, request: &SizingRequest) -> Vec<Refusal> {
    let model = model(request_doc);
    let mut refusals = Vec::new();
    let cases: BTreeMap<&str, &Value> = array(model, "load_cases")
        .iter()
        .filter_map(|c| Some((c.get("id")?.as_str()?, c)))
        .collect();
    let named: Vec<&str> = std::iter::once(request.weight_case.as_str()).chain(request.operating_cases()).collect();
    let unknown: Vec<String> = named.iter().filter(|id| !cases.contains_key(*id)).map(|id| id.to_string()).collect();
    if !unknown.is_empty() {
        refusals.push(refusal("HANGER_SIZING_CASE_UNKNOWN", unknown, "each named case must be a load case of the model, not a combination"));
    }
    let tested: Vec<String> = named
        .iter()
        .filter(|id| cases.get(*id).is_some_and(|c| is_present(c.get("test_condition"))))
        .map(|id| id.to_string())
        .collect();
    if !tested.is_empty() {
        refusals.push(refusal("HANGER_SIZING_TEST_CASE_UNSUPPORTED", tested,
            "a test-condition case holds supports, which is not the state a hanger is sized in"));
    }
    let mut seen = HashSet::new();
    let repeated: Vec<String> = named.iter().filter(|id| !seen.insert(**id)).map(|id| id.to_string()).collect();
    if !repeated.is_empty() {
        refusals.push(refusal("HANGER_SIZING_CASE_ROLES_OVERLAP", repeated,
            "the weight case may not also be an operating case, and an operating case may be named once"));
    }
    refusals
}

/// Whether the request is on the exact pressure route: its model carries a
/// `pressure_contract`, the member whose presence selects the route
/// (`product_physics` `model_format::is_pressure_free`).
pub fn is_exact(request_doc: &Value) -> bool {
    is_present(model(request_doc).get("pressure_contract"))
}

/// A `support_ref` or `support_id` naming `id` anywhere in a value.
fn names_support(value: &Value, id: &str) -> bool {
    match value {
        Value::Object(map) => map.iter().any(|(key, v)| {
            (matches!(key.as_str(), "support_ref" | "support_id") && v.as_str() == Some(id)) || names_support(v, id)
        }),
        Value::Array(items) => items.iter().any(|v| names_support(v, id)),
        _ => false,
    }
}

/// Whether an analysis state refers to `id` other than through a support state's
/// own `support_ref`, which is that support's declaration of its participation
/// (REQ-002) and not a reference. A reference to `id` inside another member of a
/// support state, or anywhere else in the state, is a reference.
fn state_refers(state: &Value, id: &str) -> bool {
    let Some(map) = state.as_object() else { return names_support(state, id) };
    map.iter().any(|(key, value)| match (key.as_str(), value) {
        ("support_states", Value::Array(entries)) => entries.iter().any(|entry| match entry.as_object() {
            Some(entry) => entry.iter().any(|(k, v)| k != "support_ref" && names_support(v, id)),
            None => names_support(entry, id),
        }),
        _ => names_support(value, id),
    })
}

/// Whether a model object refers to the candidate through a member that references
/// a support: a primitive load targeted at it, another support's friction
/// normal-reaction source, or an analysis-state support reference. Text that merely
/// equals the id, such as a provenance note, is not a reference.
fn referenced(model: &Value, id: &str) -> bool {
    let targeted = array(model, "load_cases").iter().any(|case| {
        array(case, "primitive_loads").iter().any(|load| {
            let target = &load["target"];
            target["type"] == "support" && target["support"].as_str() == Some(id)
        }) || case.get("analysis_state").is_some_and(|state| state_refers(state, id))
    });
    let normal_source = array(model, "supports").iter().any(|support| {
        support["nonlinear"]["normal_reaction_source"]["support_ref"].as_str() == Some(id)
    });
    targeted || normal_source
}

/// The stiffness a support's spring takes, as the solver takes it: the top-level
/// `stiffness`, or else the hanger's (`product_physics` `support_stiffness_input`).
fn stiffness_of(support: &Value) -> &Value {
    if is_present(support.get("stiffness")) {
        &support["stiffness"]
    } else {
        &support["hanger"]["stiffness"]
    }
}

/// The DOFs a support acts on: its rigid restraints, its nonlinear DOF and its
/// stiffness DOF ([`stiffness_of`]).
fn effective_dofs(support: &Value) -> Vec<Dof> {
    let mut dofs: Vec<Dof> = array(support, "restraints").iter().filter_map(Value::as_str).filter_map(parse_dof).collect();
    for dof in [&support["nonlinear"]["dof"], &stiffness_of(support)["dof"]] {
        if let Some(dof) = dof.as_str().and_then(parse_dof) {
            dofs.push(dof);
        }
    }
    dofs
}

/// The one translational DOF a candidate acts on (REQ-002), or `None` when it acts
/// on none, on a rotation or on more than one:
/// - a single rigid restraint, when the support declares no stiffness DOF or one
///   equal to it (a stiffness on another DOF makes two);
/// - for a variable spring hanger with no rigid restraint, the DOF its stiffness
///   declares, taken as the solver takes it ([`stiffness_of`]).
///
/// Apart from that DOF, the support's family and hanger data are not read.
pub fn candidate_dof(support: &Value) -> Option<Dof> {
    let stiffness_dof = &stiffness_of(support)["dof"];
    match array(support, "restraints") {
        [only] => {
            let dof = only.as_str().and_then(parse_dof)?;
            if stiffness_dof.is_null() || stiffness_dof.as_str().and_then(parse_dof) == Some(dof) {
                Some(dof)
            } else {
                None
            }
        }
        [] if support.get("family").and_then(Value::as_str) == Some("variable_spring_hanger") => {
            stiffness_dof.as_str().and_then(parse_dof)
        }
        _ => None,
    }
}

/// Whether a support state is ordinary (REQ-002): active as a model device, with
/// no boundary motion, base motion or device reference. An empty motion list
/// imposes nothing and is ordinary.
fn ordinary_state(state: &Value) -> bool {
    let moves = |key: &str| match state.get(key) {
        None | Some(Value::Null) => false,
        Some(Value::Array(items)) => !items.is_empty(),
        Some(_) => true,
    };
    state["participation"]["kind"] == "active_model_device"
        && !moves("boundary_motion")
        && !moves("base_motion")
        && !is_present(state.get("device_reference"))
}

/// The candidates that lack, in some named case with an analysis state, exactly
/// one ordinary support state. A missing or repeated state is named here rather
/// than left to the solve: a candidate replaced by a force has no state in that
/// case's derived documents, so the operating cases' force solves would never
/// observe it.
fn moved_candidates(request_doc: &Value, request: &SizingRequest, sites: &[Site]) -> Vec<String> {
    let named: Vec<&str> = std::iter::once(request.weight_case.as_str()).chain(request.operating_cases()).collect();
    let states: Vec<&Value> = array(model(request_doc), "load_cases")
        .iter()
        .filter(|c| c.get("id").and_then(Value::as_str).is_some_and(|id| named.contains(&id)))
        .filter_map(|c| c.get("analysis_state").filter(|s| !s.is_null()))
        .collect();
    sites
        .iter()
        .filter(|site| {
            states.iter().any(|state| {
                let own: Vec<&Value> = array(state, "support_states")
                    .iter()
                    .filter(|entry| entry.get("support_ref").and_then(Value::as_str) == Some(site.support.as_str()))
                    .collect();
                !matches!(own.as_slice(), [only] if ordinary_state(only))
            })
        })
        .map(|site| site.support.clone())
        .collect()
}

/// Candidate checks.
pub fn check_candidates(request_doc: &Value, request: &SizingRequest) -> Result<Vec<Site>, Vec<Refusal>> {
    let model = model(request_doc);
    let supports = array(model, "supports");
    let mut refusals: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
    let mut flag = |code: &'static str, id: &str| refusals.entry(code).or_default().push(id.to_string());
    let mut sites = Vec::new();
    if request.candidates.is_empty() {
        flag("HANGER_SIZING_CANDIDATES_REQUIRED", "");
    }
    let mut seen = HashSet::new();
    for candidate in &request.candidates {
        let id = candidate.support.as_str();
        if !seen.insert(id) {
            flag("HANGER_SIZING_CANDIDATE_DUPLICATE", id);
            continue;
        }
        let Some(support) = supports.iter().find(|s| s.get("id").and_then(Value::as_str) == Some(id)) else {
            flag("HANGER_SIZING_CANDIDATE_UNKNOWN", id);
            continue;
        };
        // Checks that do not need the DOF run first, so that every finding is listed.
        if is_present(support.get("direction")) {
            flag("HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED", id);
        }
        let family = support.get("family").and_then(Value::as_str);
        if is_present(support.get("nonlinear")) || matches!(family, Some("anchor" | "snubber" | "nonlinear")) {
            flag("HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED", id);
        }
        if is_present(support.get("boundary_association")) || referenced(model, id) {
            flag("HANGER_SIZING_CANDIDATE_REFERENCED", id);
        }
        let Some(dof) = candidate_dof(support) else {
            flag("HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED", id);
            continue;
        };
        let node = support.get("node").and_then(Value::as_str).unwrap_or_default().to_string();
        let shared = supports.iter().any(|other| {
            other.get("id").and_then(Value::as_str) != Some(id)
                && other.get("node").and_then(Value::as_str) == Some(node.as_str())
                && effective_dofs(other).contains(&dof)
        });
        if shared {
            flag("HANGER_SIZING_CANDIDATE_DOF_SHARED", id);
        }
        sites.push(Site { support: id.to_string(), node, dof });
    }
    for id in moved_candidates(request_doc, request, &sites) {
        flag("HANGER_SIZING_CANDIDATE_MOVED", &id);
    }
    if refusals.is_empty() {
        Ok(sites)
    } else {
        Err(refusals
            .into_iter()
            .map(|(code, names)| refusal(code, names, candidate_message(code)))
            .collect())
    }
}

/// What each candidate refusal states: the rule of REQ-002 the named supports fail.
fn candidate_message(code: &str) -> &'static str {
    match code {
        "HANGER_SIZING_CANDIDATES_REQUIRED" => "name at least one support to size",
        "HANGER_SIZING_CANDIDATE_DUPLICATE" => "each support may be named only once as a candidate",
        "HANGER_SIZING_CANDIDATE_UNKNOWN" => "the named support is not a support of the model",
        "HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED" =>
            "the support has a skew direction; a candidate acts along the global X, Y or Z axis",
        "HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED" =>
            "the support is nonlinear, a snubber or an anchor; a candidate is a linear support",
        "HANGER_SIZING_CANDIDATE_REFERENCED" =>
            "another model object refers to the support: a load targeted at it, another support's friction normal-reaction source, or a case's reference to it other than its own support state",
        "HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED" =>
            "a candidate acts on exactly one translational DOF (UX, UY or UZ): its single rigid restraint, or, for a variable spring hanger with no rigid restraint, the DOF its stiffness declares; this support acts on none, on a rotation, or on more than one",
        "HANGER_SIZING_CANDIDATE_DOF_SHARED" => "another support acts on the same node and DOF as the candidate",
        "HANGER_SIZING_CANDIDATE_MOVED" =>
            "on a model with load and reference states, a candidate needs in every named case exactly one support state that is active, with no boundary motion, base motion or device reference",
        _ => "the support cannot be sized",
    }
}

/// How a candidate enters one derived solve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    /// A rigid restraint of its DOF (S_W).
    Rigid,
    /// Removed, and replaced by this force along its DOF's positive axis (S_C, S_Oi;
    /// constant and unfitted candidates in verification).
    Force(f64),
    /// A spring of rate k on its DOF with the force F_cold: F(u) = F_cold − k·u.
    Spring { k: f64, f_cold: f64 },
}

fn load_id(site: &Site) -> String {
    format!("load:hanger-sizing:{}", site.support)
}

fn force_primitive(site: &Site, value: f64) -> Value {
    json!({
        "id": load_id(site),
        "category": "concentrated_force",
        "target": {"type": "node", "node": site.node},
        "direction": format!("global_{}", axis(site.dof)),
        "magnitude": {"value": value, "unit": "N"},
        "dimension": "force",
        "provenance": "hanger sizing derived solve (DEL-04-11); not a model load"
    })
}

/// A candidate made a rigid restraint of its DOF: `family` `guide` (the product's
/// family for a rigid restraint of named translational DOFs), `restraints` exactly
/// `[dof]`, and its `hanger`, `stiffness` and `nonlinear` members removed. Every
/// other member (id, node, direction, provenance and the rest) is kept as authored.
fn make_rigid(support: &mut serde_json::Map<String, Value>, dof: Dof) {
    for key in ["hanger", "stiffness", "nonlinear"] {
        support.remove(key);
    }
    support.insert("restraints".into(), json!([dof_token(dof)]));
    support.insert("family".into(), json!("guide"));
}

/// The source with its candidates neutralised: the whole authored document, every
/// case, combination, support, state, load, reference and setting kept, and only
/// each named candidate made rigid on its admitted DOF ([`make_rigid`]). This is
/// the baseline the source is admitted on ([`check_source`]).
///
/// The candidate rules (REQ-002, [`check_candidates`]) run first, on the
/// unmodified source, and only candidates they admit are neutralised; so a
/// candidate's direction, a nonlinear, snubber or anchor candidate and a candidate
/// referred to by another object are refused before neutralisation and are never
/// hidden by it. Neutralisation removes exactly the members sizing replaces in
/// every derived solve: a candidate's hanger, stiffness and nonlinear members,
/// family and restraints. Malformed or ignored data there cannot refuse the request,
/// at the request parse or later, because the source is parsed only after they are
/// replaced. The authored request is not changed.
pub fn neutralised_source(request_doc: &Value, sites: &[Site]) -> Value {
    let mut doc = request_doc.clone();
    if let Some(Value::Array(supports)) = doc["model"].get_mut("supports") {
        for site in sites {
            let found = supports.iter_mut().find(|s| s.get("id").and_then(Value::as_str) == Some(site.support.as_str()));
            if let Some(Value::Object(support)) = found {
                make_rigid(support, site.dof);
            }
        }
    }
    doc
}

/// The derived document for one case: only that case, no combinations, and each
/// candidate in its role (REQ-003).
///
/// In a case with an analysis state (a load/reference-state case), a candidate made
/// rigid or a spring keeps its support state; a candidate replaced by a force loses
/// its support state with the support; and each added force (the replacement force,
/// and a spring's cold-load force) is entered in that case's `load_sources` at
/// factor 1, in candidate order, so that the ledger applies it. In a case without an
/// analysis state the stored primitives act and nothing of this applies.
pub fn derived_document(request_doc: &Value, case: &str, sites: &[Site], roles: &[Role]) -> Value {
    let mut doc = request_doc.clone();
    let model = doc["model"].as_object_mut().expect("solve request has a model");
    if let Some(Value::Array(cases)) = model.get_mut("load_cases") {
        cases.retain(|c| c.get("id").and_then(Value::as_str) == Some(case));
    }
    if model.contains_key("combinations") {
        model.insert("combinations".into(), json!([]));
    }
    // Pressure elongation names the operating cases it applies in. The derived
    // document keeps that choice for its one case: the case stays selected when the
    // source selects it, and otherwise the setting is removed (absent means off),
    // never left naming cases the document no longer has nor emptied, which the
    // product never writes. Every other setting is kept. The source's own setting
    // was judged before any derived document was formed (`check_source`).
    if let Some(Value::Object(settings)) = model.get_mut("model_settings") {
        let selected = settings.get("pressure_elongation").map(|setting| {
            setting["operating_load_cases"].as_array().is_some_and(|ids| ids.iter().any(|id| id.as_str() == Some(case)))
        });
        match selected {
            Some(true) => {
                settings.insert("pressure_elongation".into(), json!({"operating_load_cases": [case]}));
            }
            Some(false) => {
                settings.remove("pressure_elongation");
            }
            None => {}
        }
    }
    let mut added = Vec::new();
    let mut removed = Vec::new();
    if let Some(Value::Array(supports)) = model.get_mut("supports") {
        for (site, role) in sites.iter().zip(roles) {
            let index = supports.iter().position(|s| s.get("id").and_then(Value::as_str) == Some(site.support.as_str()));
            let Some(index) = index else { continue };
            match role {
                Role::Force(value) => {
                    supports.remove(index);
                    removed.push(site.support.as_str());
                    added.push(force_primitive(site, *value));
                }
                Role::Rigid | Role::Spring { .. } => {
                    let support = supports[index].as_object_mut().expect("support object");
                    make_rigid(support, site.dof);
                    if let Role::Spring { k, f_cold } = role {
                        support.insert("family".into(), json!("spring"));
                        support.insert("stiffness".into(), json!({"dof": dof_token(site.dof), "value": {"value": k, "unit": "N/m"}}));
                        added.push(force_primitive(site, *f_cold));
                    }
                }
            }
        }
    }
    if let Some(Value::Array(cases)) = model.get_mut("load_cases") {
        for case in cases.iter_mut() {
            let case = case.as_object_mut().expect("case object");
            if let Value::Array(loads) = case.entry("primitive_loads").or_insert(json!([])) {
                loads.extend(added.iter().cloned());
            }
            let Some(state) = case.get_mut("analysis_state").and_then(Value::as_object_mut) else { continue };
            if let Some(Value::Array(states)) = state.get_mut("support_states") {
                states.retain(|s| !s.get("support_ref").and_then(Value::as_str).is_some_and(|id| removed.contains(&id)));
            }
            if let Value::Array(sources) = state.entry("load_sources").or_insert(json!([])) {
                sources.extend(added.iter().map(|load| json!({"source_ref": load["id"], "factor": 1.0})));
            }
        }
    }
    doc
}

fn all_ids(value: &Value, out: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(id)) = map.get("id") {
                out.insert(id.clone());
            }
            map.values().for_each(|v| all_ids(v, out));
        }
        Value::Array(items) => items.iter().for_each(|v| all_ids(v, out)),
        _ => {}
    }
}

/// The source's own admission of what a derived document no longer carries,
/// judged before any derived solve and without solving (REQ-009: what the solve
/// refuses refuses the request with the solve's code). A derived document keeps one
/// case and no combinations, so a case, combination, state, reference or setting
/// that the model's own solve refuses must be judged on the source, or projection
/// would hide it.
///
/// The baseline is [`neutralised_source`]: the whole authored document with only
/// the admitted candidates made rigid. It is judged by the product's
/// `admit_source_before_assembly`: the ordinary solve's stages before assembly on
/// the complete document, and each case's primitive loads built from the case the
/// solve applies (its ledger, on a load/reference-state document). That judges
/// declarations and references in every case, not the convergence or stability of
/// a case sizing does not solve; each derived solve gets every ordinary check.
///
/// Each refusal is `HANGER_SIZING_DERIVED_SOLVE_FAILED` naming `source`, carrying
/// the product's codes and messages (or its parse refusal). It is not a solve and
/// counts against no solve bound.
fn check_source(request_doc: &Value, sites: &[Site], mode: PreviewSolverMode) -> Option<Refusal> {
    let found: Vec<String> = match admit_source_before_assembly(neutralised_source(request_doc, sites), mode) {
        Err(text) => vec![text],
        Ok(diagnostics) => diagnostics
            .iter()
            .filter(|d| d.severity == "blocking")
            .map(|d| format!("{} [{}]: {}", d.code, d.affected_refs.join(", "), d.message))
            .collect(),
    };
    (!found.is_empty()).then(|| refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED", vec!["source".to_string()], found.join("; ")))
}

/// The disclosure of a derived solve whose numerical-integrity standing is not
/// [`SOLVER_CHECKS_PASSED`] (REQ-003). Its one name is the solve's label.
pub const DERIVED_SOLVE_INTEGRITY: &str = "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY";
/// The product's own warning that a case's displacement correction was
/// unavailable (DEL-04-07 *Published displacement*), carried with the product's
/// message unchanged. Its one name is the solve's label.
pub const DISPLACEMENT_CORRECTION_UNAVAILABLE: &str = "DISPLACEMENT_CORRECTION_UNAVAILABLE";

/// The standing sizing states for a derived solve the solver's own checks found
/// nothing against: the solver's report, not a verified bound. It is the reader
/// verdict's `result_export::reader_verdict::SOLVER_CHECKS_PASSED`.
pub const SOLVER_CHECKS_PASSED: &str = "solver_checks_passed";
/// The `status` of every sized document's and reader verdict's `integrity_bound`
/// (LIM-026, release exit criterion 18).
pub const INTEGRITY_BOUND_STATUS: &str = "not_verified";
/// The `reason` of every sized document's and reader verdict's `integrity_bound`.
pub const INTEGRITY_BOUND_REASON: &str = "inverse_norm_bound_not_verified";

/// The `integrity_bound` member of every sized document's `result` and every
/// reader verdict: SWBPIPE verifies no error bound for the ordinary solve's
/// values, so the bound is stated as not verified whatever the solves' standings.
/// It is the reader verdict's `result_export::reader_verdict::integrity_bound`.
pub fn integrity_bound() -> Value {
    json!({"status": INTEGRITY_BOUND_STATUS, "reason": INTEGRITY_BOUND_REASON})
}

/// The closed token sizing states for a numerical-quality status. The four
/// non-passing statuses keep the product's own serialization
/// (`numerical_quality.status`, snake case). `ChecksPassed`, which the product
/// serializes as the solver's wire token `checks_passed`, is stated as
/// [`SOLVER_CHECKS_PASSED`]: it reports that the solver's checks found nothing,
/// and asserts no error bound, which sizing publishes as [`integrity_bound`]
/// (`not_verified`, `inverse_norm_bound_not_verified`) per LIM-026 and release
/// exit criterion 18.
pub fn standing_token(status: NumericalQualityStatus) -> &'static str {
    match status {
        NumericalQualityStatus::NotAssessed => "not_assessed",
        NumericalQualityStatus::ChecksPassed => SOLVER_CHECKS_PASSED,
        NumericalQualityStatus::Sensitive => "sensitive",
        NumericalQualityStatus::Unresolved => "unresolved",
        NumericalQualityStatus::Failed => "failed",
    }
}

/// The standings other than [`SOLVER_CHECKS_PASSED`], each disclosed by
/// [`DERIVED_SOLVE_INTEGRITY`] with [`integrity_message`].
pub const NONPASSING_STANDINGS: [&str; 4] = ["not_assessed", "sensitive", "unresolved", "failed"];

/// The closed vocabulary of every derived solve's `solver_standing`, the
/// positive per-solve statement of [`standing_token`]: [`SOLVER_CHECKS_PASSED`]
/// first, then [`NONPASSING_STANDINGS`]. A solve stands as exactly one; a
/// non-passing one is also disclosed by [`DERIVED_SOLVE_INTEGRITY`] with the same
/// token, and a passing one is not.
pub const SOLVER_STANDINGS: [&str; 5] = [SOLVER_CHECKS_PASSED, "not_assessed", "sensitive", "unresolved", "failed"];

/// The closed vocabulary of a derived solve's `support_state_confirmation.outcome`,
/// the tokens of `numerical_vocabulary::HistoricalConfirmation` that
/// `report_standing` derives from the same fixed producer text.
pub const CONFIRMATION_OUTCOMES: [&str; 5] = [
    "solver_reported_established",
    "not_attempted_reason_not_recorded",
    "not_established_detail_quoted",
    "no_confirmation_record",
    "not_applicable",
];
/// The confirmation outcome that alone carries a quoted `detail`.
pub const DETAIL_QUOTED: &str = "not_established_detail_quoted";
/// The confirmation outcome of a solve that publishes no nonlinear-support state.
pub const NOT_APPLICABLE: &str = "not_applicable";

/// The fixed producer text a case's integrity message ends with, copied from
/// `product_physics` (`NONLINEAR_CONFIRMED_EVIDENCE_LINE`,
/// `nonlinear_not_established_sentence`, the S11-G recovery-guard sentence and
/// `integrity_diagnostic_id`) and read exactly as `report_standing` reads it. The
/// copies are pinned to the producer's source by test.
pub const NONLINEAR_CONFIRMED_LINE: &str = "Converged support state: established at 128-bit precision by one formation-checked solve; each support's state has a sign margin larger than its bound, and a sliding friction force satisfies the friction law of that state within its bound; the published nodal displacements, and the support displacement rows that follow them, are the formation check's corrected displacements; the iteration's residual rows are its own.";
const NOT_ESTABLISHED_LEAD: &str = " the converged support state is not established at 128-bit precision (";
const NOT_ESTABLISHED_TAIL: &str = "); member rows are binary64 recoveries";
const NOT_RUN: &str = "confirmation attempt: not run";
const RECOVERY_LEAD: &str = " S11-G recovery guard (R-b'): load case ";
const SENSITIVE_BECAUSE: &str = " is Sensitive because ";
const INTEGRITY_DIAGNOSTIC_PREFIX: &str = "diagnostic:numerical-integrity:";

/// What the solver's integrity record of `case_id` states of the support-state
/// confirmation in one derived solve, by `report_standing`'s own rule:
/// - a solve that publishes no nonlinear-support state is `not_applicable`;
/// - otherwise the message of `diagnostic:numerical-integrity:<case_id>` (empty
///   when absent) is read at its end. The not-established sentence, its lead
///   occurring once, gives `not_attempted_reason_not_recorded` for the reason
///   "confirmation attempt: not run" and `not_established_detail_quoted` with the
///   reason quoted otherwise; the confirmed-state line, occurring once and
///   followed by nothing or by the case's S11-G recovery sentence, gives
///   `solver_reported_established`; anything else is `no_confirmation_record`.
///
/// One difference from `report_standing`: an empty or whitespace-only reason, which
/// the producer never forms, is `no_confirmation_record`, so a quoted detail is
/// never blank. `solver_reported_established` is the solver's report, not an
/// independently verified confirmation bound.
pub fn support_state_confirmation(case_id: &str, states: &BTreeMap<String, String>, diagnostics: &[Diagnostic]) -> SupportStateConfirmation {
    if states.is_empty() {
        return SupportStateConfirmation::of(NOT_APPLICABLE, None);
    }
    let id = format!("{INTEGRITY_DIAGNOSTIC_PREFIX}{case_id}");
    let message = diagnostics.iter().find(|d| d.id == id).map(|d| d.message.as_str()).unwrap_or("");
    let none = || SupportStateConfirmation::of("no_confirmation_record", None);
    if let Some(head) = message.strip_suffix(NOT_ESTABLISHED_TAIL) {
        if message.matches(NOT_ESTABLISHED_LEAD).count() == 1 {
            let Some(found) = head.rfind(NOT_ESTABLISHED_LEAD) else { return none() };
            let reason = &head[found + NOT_ESTABLISHED_LEAD.len()..];
            return if reason == NOT_RUN {
                SupportStateConfirmation::of("not_attempted_reason_not_recorded", None)
            } else if reason.trim().is_empty() {
                none()
            } else {
                SupportStateConfirmation::of(DETAIL_QUOTED, Some(reason.to_string()))
            };
        }
        return none();
    }
    let confirmed = format!(" {NONLINEAR_CONFIRMED_LINE}");
    if message.matches(confirmed.as_str()).count() == 1 {
        if let Some(found) = message.find(confirmed.as_str()) {
            let rest = &message[found + confirmed.len()..];
            let recovery = [RECOVERY_LEAD, case_id, SENSITIVE_BECAUSE].concat();
            if rest.is_empty() || rest.starts_with(recovery.as_str()) {
                return SupportStateConfirmation::of("solver_reported_established", None);
            }
        }
    }
    none()
}

/// The fixed message of a [`DERIVED_SOLVE_INTEGRITY`] disclosure: the standing's
/// closed token first, then its meaning and the unverified bound.
pub fn integrity_message(token: &str) -> String {
    format!(
        "{token}: the solver's numerical-integrity status for this derived solve is {token}, not {SOLVER_CHECKS_PASSED}; every value sizing takes from it carries that standing (integrity_bound: {INTEGRITY_BOUND_STATUS}, {INTEGRITY_BOUND_REASON})"
    )
}

/// What one derived solve's envelope gives sizing beyond its values (REQ-003): a
/// blocking diagnostic refuses the request with the solve's diagnostics, before
/// anything else is read; otherwise the solve's disclosures. A derived document
/// keeps one case, so its `numerical_quality.status`, the most severe standing of
/// its cases, is that case's `solve_quality`. A standing other than
/// [`SOLVER_CHECKS_PASSED`] is
/// disclosed with its exact token, and the product's
/// `DISPLACEMENT_CORRECTION_UNAVAILABLE` warning with its message unchanged (more
/// than one, which a one-case document does not publish, would be joined by "; "),
/// each naming the solve. Neither refuses.
fn solve_disclosures(label: &str, status: NumericalQualityStatus, diagnostics: &[Diagnostic])
    -> Result<Vec<Disclosure>, Refusal> {
    let blocking: Vec<String> = diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect();
    if !blocking.is_empty() {
        return Err(refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED", vec![label.to_string()], blocking.join("; ")));
    }
    let mut disclosures = Vec::new();
    let token = standing_token(status);
    if token != SOLVER_CHECKS_PASSED {
        disclosures.push(Disclosure {
            code: DERIVED_SOLVE_INTEGRITY,
            names: vec![label.to_string()],
            message: integrity_message(token),
        });
    }
    let unavailable: Vec<&str> = diagnostics
        .iter()
        .filter(|d| d.code == DISPLACEMENT_CORRECTION_UNAVAILABLE)
        .map(|d| d.message.as_str())
        .collect();
    if !unavailable.is_empty() {
        disclosures.push(Disclosure {
            code: DISPLACEMENT_CORRECTION_UNAVAILABLE,
            names: vec![label.to_string()],
            message: unavailable.join("; "),
        });
    }
    Ok(disclosures)
}

/// The one call site of every derived solve. Its disclosures are appended to
/// `disclosures`, in solve order.
fn solve(doc: Value, mode: PreviewSolverMode, label: &str, disclosures: &mut Vec<Disclosure>) -> Result<MechanicsEnvelope, Refusal> {
    let envelope = run_linear_static_preview_value_with_mode(doc, mode)
        .map_err(|error| refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED", vec![label.to_string()], error))?;
    disclosures.extend(solve_disclosures(label, envelope.numerical_quality.status, &envelope.diagnostics)?);
    Ok(envelope)
}

fn si(value: f64, unit: &str, dimension: Dimension) -> Option<f64> {
    let from = unit_by_symbol(unit, dimension).ok()?;
    convert(value, from, canonical_unit(dimension)?).ok()
}

fn reaction(envelope: &MechanicsEnvelope, site: &Site) -> Option<f64> {
    let component = format!("F{}", axis(site.dof));
    envelope
        .results
        .iter()
        .find(|row| {
            row.kind == "support_reaction_component_v2"
                && row.entity_ref == site.support
                && row.metadata.as_ref().is_some_and(|m| m.component == component)
        })
        .and_then(|row| si(row.value, &row.unit, Dimension::Force))
}

fn displacement(envelope: &MechanicsEnvelope, site: &Site) -> Option<f64> {
    let kind = format!("global_nodal_displacement_{}", axis(site.dof));
    envelope
        .results
        .iter()
        .find(|row| row.kind == kind && row.entity_ref == site.node)
        .and_then(|row| si(row.value, &row.unit, Dimension::Length))
}

/// One derived solve's record ([`SolveStates`]) from its envelope; `case` is the
/// one case its derived document keeps.
fn states(envelope: &MechanicsEnvelope, solve: &str, case: &str) -> SolveStates {
    let states: BTreeMap<String, String> = envelope
        .results
        .iter()
        .filter(|row| row.kind == "nonlinear_support_active_set_state_code")
        .map(|row| {
            let name = match row.value as i64 {
                0 => "inactive",
                1 => "active",
                2 => "sticking",
                3 => "sliding",
                _ => "unknown",
            };
            (row.entity_ref.clone(), name.to_string())
        })
        .collect();
    let support_state_confirmation = support_state_confirmation(case, &states, &envelope.diagnostics);
    SolveStates {
        solve: solve.to_string(),
        states,
        solver_standing: standing_token(envelope.numerical_quality.status),
        support_state_confirmation,
    }
}

fn missing(label: &str, what: &str, sites: &[&Site]) -> Refusal {
    refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED",
        std::iter::once(label.to_string()).chain(sites.iter().map(|s| s.support.clone())).collect(),
        format!("the derived solve published no {what} at the named candidates"))
}

/// Runs one derived solve and reads each candidate's displacement along its DOF.
fn positions_of(request_doc: &Value, case: &str, sites: &[Site], roles: &[Role], mode: PreviewSolverMode, label: &str,
    disclosures: &mut Vec<Disclosure>) -> Result<(Vec<f64>, SolveStates), Refusal> {
    let envelope = solve(derived_document(request_doc, case, sites, roles), mode, label, disclosures)?;
    let values: Vec<Option<f64>> = sites.iter().map(|s| displacement(&envelope, s)).collect();
    let absent: Vec<&Site> = sites.iter().zip(&values).filter(|(_, v)| v.is_none()).map(|(s, _)| s).collect();
    if !absent.is_empty() {
        return Err(missing(label, "displacement", &absent));
    }
    Ok((values.into_iter().map(Option::unwrap).collect(), states(&envelope, label, case)))
}

fn categories(request_doc: &Value, case: &str) -> Vec<String> {
    let mut out: Vec<String> = array(model(request_doc), "load_cases")
        .iter()
        .filter(|c| c.get("id").and_then(Value::as_str) == Some(case))
        .flat_map(|c| array(c, "primitive_loads").iter().filter_map(|l| l.get("category")?.as_str().map(str::to_string)))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    out.sort();
    out
}

fn state_differences(a: &SolveStates, b: &SolveStates) -> Vec<String> {
    a.states
        .iter()
        .filter(|(support, state)| b.states.get(*support).is_some_and(|other| other != *state))
        .map(|(support, _)| support.clone())
        .collect()
}

/// The whole sizing run: checks, S_W, S_C and S_Oi, the arithmetic,
/// the verification solves and the disclosures. Any refusal refuses the request.
pub fn run(request_doc: &Value, records: &[Record], request: &SizingRequest) -> Result<SizingRun, Vec<Refusal>> {
    let mode = request.solver_mode;
    let mut refusals = check_method(&request.method);
    refusals.extend(check_cases(request_doc, request));
    let sites = match check_candidates(request_doc, request) {
        Ok(sites) => sites,
        Err(found) => {
            refusals.extend(found);
            Vec::new()
        }
    };
    let mut ids = BTreeSet::new();
    all_ids(model(request_doc), &mut ids);
    let collisions: Vec<String> = sites.iter().map(load_id).filter(|id| ids.contains(id)).collect();
    if !collisions.is_empty() {
        refusals.push(refusal("HANGER_SIZING_ID_COLLISION", collisions, "a reserved derived-load id already exists in the model"));
    }
    if refusals.is_empty() {
        refusals.extend(check_source(request_doc, &sites, mode));
    }
    if !refusals.is_empty() {
        return Err(refusals);
    }

    // S_W: the weight case with every candidate rigid.
    let weight = request.weight_case.as_str();
    let rigid = vec![Role::Rigid; sites.len()];
    // Each derived solve's disclosures, in solve order (S_W, S_C, S_Oi, V_C, V_Oi).
    let mut per_solve = Vec::new();
    // The full envelope is dropped once W and the states are read, before the next
    // solve starts.
    let (w, weight_states) = {
        let envelope = solve(derived_document(request_doc, weight, &sites, &rigid), mode, "S_W", &mut per_solve).map_err(|r| vec![r])?;
        (sites.iter().map(|s| reaction(&envelope, s)).collect::<Vec<Option<f64>>>(), states(&envelope, "S_W", weight))
    };
    let absent: Vec<&Site> = sites.iter().zip(&w).filter(|(_, v)| v.is_none()).map(|(s, _)| s).collect();
    if !absent.is_empty() {
        return Err(vec![missing("S_W", "support reaction", &absent)]);
    }
    let w: Vec<f64> = w.into_iter().map(Option::unwrap).collect();
    let mut solve_states = vec![weight_states];
    let not_positive: Vec<String> = sites.iter().zip(&w).filter(|(_, w)| !(**w > 0.0)).map(|(s, _)| s.support.clone()).collect();
    if !not_positive.is_empty() {
        return Err(vec![refusal("HANGER_SIZING_LOAD_NOT_POSITIVE", not_positive,
            "the rigid reaction at each named candidate is not positive along its DOF; sizing never flips a sign, so remove the candidate and run again")]);
    }

    // S_C and S_Oi: every candidate replaced by its force W.
    let forces: Vec<Role> = w.iter().map(|w| Role::Force(*w)).collect();
    let (u_cold, cold_states) = positions_of(request_doc, weight, &sites, &forces, mode, "S_C", &mut per_solve).map_err(|r| vec![r])?;
    solve_states.push(cold_states);
    let operating = request.operating_cases();
    let mut u_operating = vec![Vec::new(); sites.len()];
    for (i, case) in operating.iter().enumerate() {
        let label = format!("S_O{}", i + 1);
        let (values, case_states) = positions_of(request_doc, case, &sites, &forces, mode, &label, &mut per_solve).map_err(|r| vec![r])?;
        solve_states.push(case_states);
        for (column, value) in u_operating.iter_mut().zip(values) {
            column.push(value);
        }
    }
    let candidates: Vec<Candidate> = sites
        .iter()
        .zip(&request.candidates)
        .zip(w.iter().zip(u_cold.iter().zip(u_operating)))
        .map(|((site, asked), (w, (u_c, u_ops)))| Candidate {
            id: site.support.clone(),
            dof: site.dof,
            family_policy: asked.family_policy,
            w_n: *w,
            positions: Positions { u_cold_m: *u_c, u_operating_m: u_ops },
            design_index: 0,
        })
        .collect();
    let results = size(&candidates, records, &request.method)?;

    // Verification: the selections in place.
    let roles: Vec<Role> = results
        .iter()
        .zip(&candidates)
        .map(|(result, candidate)| match &result.selected {
            Some(Selected::Variable { k, loads, .. }) => Role::Spring { k: *k, f_cold: loads.f_cold_n },
            _ => Role::Force(candidate.w_n),
        })
        .collect();
    let (v_cold, verified_cold_states) = positions_of(request_doc, weight, &sites, &roles, mode, "V_C", &mut per_solve).map_err(|r| vec![r])?;
    solve_states.push(verified_cold_states);
    let mut v_operating = vec![Vec::new(); sites.len()];
    for (i, case) in operating.iter().enumerate() {
        let label = format!("V_O{}", i + 1);
        let (values, case_states) = positions_of(request_doc, case, &sites, &roles, mode, &label, &mut per_solve).map_err(|r| vec![r])?;
        solve_states.push(case_states);
        for (column, value) in v_operating.iter_mut().zip(values) {
            column.push(value);
        }
    }
    let mut verifications = Vec::new();
    for (((candidate, result), u_c), u_ops) in candidates.iter().zip(&results).zip(v_cold).zip(v_operating) {
        let verified = Positions { u_cold_m: u_c, u_operating_m: u_ops };
        verifications.push(verify(candidate, result, records, &request.method, &verified).map_err(|r| vec![r])?);
    }

    // Disclosures.
    let mut disclosures = Vec::new();
    let by_label = |label: &str| solve_states.iter().find(|s| s.solve == label).expect("solve states recorded");
    let differs = state_differences(by_label("S_W"), by_label("S_O1"));
    if !differs.is_empty() {
        disclosures.push(Disclosure { code: "HANGER_SIZING_SUPPORT_STATE_DIFFERS", names: differs,
            message: "W was formed in the weight case's nonlinear-support state, not the design operating case's".into() });
    }
    let differs = state_differences(by_label("S_W"), by_label("S_C"));
    if !differs.is_empty() {
        disclosures.push(Disclosure { code: "HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS", names: differs,
            message: "replacing the candidates by their forces W found another nonlinear-support equilibrium in the weight case".into() });
    }
    // Then every derived solve's own disclosures, none dropped.
    disclosures.extend(per_solve);
    let mut case_load_categories = BTreeMap::new();
    for case in std::iter::once(weight).chain(operating.iter().copied()) {
        case_load_categories.insert(case.to_string(), categories(request_doc, case));
    }
    let standing = verification_standing(&results);
    Ok(SizingRun { sites, candidates, results, verifications, standing, states: solve_states, disclosures, case_load_categories })
}

/// The per-solve disclosure seam ([`solve_disclosures`]) on the product's own
/// envelope parts. The positive control reads the committed
/// `fixtures/results/rt_a3_s6_c1_correction_unavailable.json`, which PR 374's
/// test-only forcing hook in `product_physics` produced: it is fault-injection
/// evidence of the product's diagnostics and numerical quality on that path, not a
/// maintained sizing input that reaches the path by itself. Its diagnostics and
/// standing are converted to the product's types here, in test code only.
#[cfg(test)]
mod disclosure_tests {
    use super::*;

    const CORRECTION_UNAVAILABLE_FIXTURE: &str =
        include_str!("../../../../fixtures/results/rt_a3_s6_c1_correction_unavailable.json");

    fn status_of(token: &str) -> NumericalQualityStatus {
        match token {
            "not_assessed" => NumericalQualityStatus::NotAssessed,
            "checks_passed" => NumericalQualityStatus::ChecksPassed,
            "sensitive" => NumericalQualityStatus::Sensitive,
            "unresolved" => NumericalQualityStatus::Unresolved,
            "failed" => NumericalQualityStatus::Failed,
            other => panic!("not a numerical-quality status: {other}"),
        }
    }

    fn diagnostic(value: &Value) -> Diagnostic {
        Diagnostic {
            id: value["id"].as_str().expect("id").to_string(),
            code: value["code"].as_str().expect("code").to_string(),
            severity: value["severity"].as_str().expect("severity").to_string(),
            message: value["message"].as_str().expect("message").to_string(),
            source: value["source"].as_str().map(str::to_string),
            affected_refs: array(value, "affected_refs").iter().map(|r| r.as_str().expect("ref").to_string()).collect(),
        }
    }

    /// The fixture envelope's standing token and diagnostics, after checking the
    /// numerical-quality field contract a derived document relies on: one case,
    /// whose `solve_quality` is the document's `status`.
    fn fixture() -> (String, Vec<Diagnostic>) {
        let doc: Value = serde_json::from_str(CORRECTION_UNAVAILABLE_FIXTURE).expect("fixture JSON");
        let envelope = &doc["envelope"];
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED");
        let quality = &envelope["numerical_quality"];
        let cases = array(quality, "cases");
        assert_eq!(cases.len(), 1, "one retained case per envelope");
        assert_eq!(cases[0]["solve_quality"], quality["status"]);
        let diagnostics: Vec<Diagnostic> = array(envelope, "diagnostics").iter().map(diagnostic).collect();
        assert!(diagnostics.iter().all(|d| d.severity != "blocking"));
        (quality["status"].as_str().expect("status").to_string(), diagnostics)
    }

    #[test]
    fn the_forced_correction_failure_is_disclosed_with_its_standing_and_the_products_message_unchanged() {
        let (token, diagnostics) = fixture();
        assert_eq!(token, "sensitive");
        let warning = diagnostics.iter().find(|d| d.code == DISPLACEMENT_CORRECTION_UNAVAILABLE).expect("product warning");
        assert_eq!(warning.severity, "warning");
        // The product's message names the component and the failure kind.
        assert!(warning.message.contains("node:A:UX") && warning.message.contains("could not be certified"), "{}", warning.message);
        for label in ["S_W", "S_C", "S_O1", "S_O2", "V_C", "V_O1", "V_O2"] {
            let found = solve_disclosures(label, status_of(&token), &diagnostics).expect("not refused");
            assert_eq!(found, vec![
                Disclosure { code: DERIVED_SOLVE_INTEGRITY, names: vec![label.to_string()], message: integrity_message("sensitive") },
                Disclosure { code: DISPLACEMENT_CORRECTION_UNAVAILABLE, names: vec![label.to_string()], message: warning.message.clone() },
            ]);
            assert!(found[0].message.starts_with("sensitive: "));
        }
    }

    #[test]
    fn a_passing_solve_without_the_warning_adds_no_disclosure() {
        let (_, diagnostics) = fixture();
        let ordinary: Vec<Diagnostic> = diagnostics.into_iter().filter(|d| d.code != DISPLACEMENT_CORRECTION_UNAVAILABLE).collect();
        assert_eq!(solve_disclosures("S_O1", NumericalQualityStatus::ChecksPassed, &ordinary), Ok(Vec::new()));
        assert!(solve_disclosures("S_O1", NumericalQualityStatus::ChecksPassed, &[]).expect("not refused").is_empty());
    }

    #[test]
    fn every_standing_other_than_solver_checks_passed_is_disclosed_with_its_exact_token() {
        let (_, diagnostics) = fixture();
        let ordinary: Vec<Diagnostic> = diagnostics.into_iter().filter(|d| d.code != DISPLACEMENT_CORRECTION_UNAVAILABLE).collect();
        let all = [
            NumericalQualityStatus::NotAssessed,
            NumericalQualityStatus::ChecksPassed,
            NumericalQualityStatus::Sensitive,
            NumericalQualityStatus::Unresolved,
            NumericalQualityStatus::Failed,
        ];
        for status in all {
            let wire = serde_json::to_value(status).expect("status");
            assert_eq!(status_of(wire.as_str().expect("token")), status);
            let found = solve_disclosures("V_C", status, &ordinary).expect("not refused");
            if status == NumericalQualityStatus::ChecksPassed {
                // The product's wire token stays the solver's; sizing states it as
                // solver_checks_passed and discloses nothing for it.
                assert_eq!(wire, json!("checks_passed"));
                assert_eq!(standing_token(status), "solver_checks_passed");
                assert_eq!(standing_token(status), SOLVER_CHECKS_PASSED);
                assert!(found.is_empty());
                assert!(!NONPASSING_STANDINGS.contains(&standing_token(status)));
            } else {
                // A non-passing token is the product's own serialization of the status.
                let token = standing_token(status);
                assert_eq!(wire, json!(token));
                assert!(NONPASSING_STANDINGS.contains(&token));
                assert_eq!(found, vec![Disclosure { code: DERIVED_SOLVE_INTEGRITY, names: vec!["V_C".into()], message: integrity_message(token) }]);
                assert_eq!(found[0].message.split(':').next(), Some(token));
                assert!(found[0].message.contains("not solver_checks_passed"), "{}", found[0].message);
                assert!(found[0].message.contains("(integrity_bound: not_verified, inverse_norm_bound_not_verified)"), "{}", found[0].message);
                assert!(!found[0].message.contains("not checks_passed"), "{}", found[0].message);
            }
        }
    }

    #[test]
    fn a_blocking_diagnostic_refuses_before_any_disclosure() {
        let (token, mut diagnostics) = fixture();
        diagnostics.push(Diagnostic {
            id: "diagnostic:invented".into(),
            code: "INVENTED_BLOCKING".into(),
            severity: "blocking".into(),
            message: "an invented blocking finding".into(),
            source: None,
            affected_refs: Vec::new(),
        });
        let refused = solve_disclosures("S_O2", status_of(&token), &diagnostics).expect_err("refused");
        assert_eq!(refused.code, "HANGER_SIZING_DERIVED_SOLVE_FAILED");
        assert_eq!(refused.names, vec!["S_O2".to_string()]);
        assert_eq!(refused.message, "INVENTED_BLOCKING: an invented blocking finding");
    }

    #[test]
    fn the_solver_standings_are_the_standing_tokens() {
        let all = [
            NumericalQualityStatus::ChecksPassed,
            NumericalQualityStatus::NotAssessed,
            NumericalQualityStatus::Sensitive,
            NumericalQualityStatus::Unresolved,
            NumericalQualityStatus::Failed,
        ];
        assert_eq!(all.map(standing_token), SOLVER_STANDINGS);
        assert_eq!(SOLVER_STANDINGS[0], SOLVER_CHECKS_PASSED);
        assert_eq!(SOLVER_STANDINGS[1..], NONPASSING_STANDINGS);
        // A passing solve's positive standing adds no disclosure (AC-003).
        assert!(solve_disclosures("S_W", NumericalQualityStatus::ChecksPassed, &[]).expect("not refused").is_empty());
    }
}

/// [`support_state_confirmation`] on each of its five outcomes, and its copies of
/// the producer's fixed text pinned to `product_physics` and to `report_standing`
/// (compile reads of their sources; no dependency is added).
#[cfg(test)]
mod confirmation_tests {
    use super::*;

    const CASE: &str = "load:W";
    const PRODUCT_LIB: &str = include_str!("../../../product_physics/src/lib.rs");
    const PRODUCT_GUARD: &str = include_str!("../../../product_physics/src/formation_guard.rs");
    const REPORT_STANDING: &str = include_str!("../../../reporting/report_standing/src/report_standing.rs");
    const HISTORICAL_HEALTH: &str = include_str!("../../../reporting/report_standing/src/historical_health.rs");
    const VOCABULARY: &str = include_str!("../../../reporting/numerical_vocabulary/src/lib.rs");
    /// An invented integrity message head, as the product's report forms one.
    const HEAD: &str = "Load case load:W: numerical integrity checks passed.";

    fn integrity(case: &str, message: &str) -> Diagnostic {
        Diagnostic {
            id: format!("diagnostic:numerical-integrity:{case}"),
            code: "NUMERICAL_INTEGRITY_PASSED".into(),
            severity: "info".into(),
            message: message.into(),
            source: None,
            affected_refs: vec![case.into()],
        }
    }

    fn lifted() -> BTreeMap<String, String> {
        BTreeMap::from([("support:C-lift".to_string(), "active".to_string())])
    }

    fn outcome(message: &str) -> SupportStateConfirmation {
        support_state_confirmation(CASE, &lifted(), &[integrity(CASE, message)])
    }

    fn not_established(reason: &str) -> String {
        format!("{HEAD}{NOT_ESTABLISHED_LEAD}{reason}{NOT_ESTABLISHED_TAIL}")
    }

    #[test]
    fn each_of_the_five_outcomes() {
        let established = SupportStateConfirmation::of("solver_reported_established", None);
        let none = SupportStateConfirmation::of("no_confirmation_record", None);
        // not_applicable: no nonlinear-support state, whatever the message says.
        let confirmed = format!("{HEAD} {NONLINEAR_CONFIRMED_LINE}");
        assert_eq!(
            support_state_confirmation(CASE, &BTreeMap::new(), &[integrity(CASE, &confirmed)]),
            SupportStateConfirmation::of("not_applicable", None)
        );
        assert_eq!(support_state_confirmation(CASE, &BTreeMap::new(), &[]).outcome, "not_applicable");
        // solver_reported_established: the confirmed line alone, or followed by this
        // case's recovery sentence.
        assert_eq!(outcome(&confirmed), established);
        let recovered = format!("{confirmed}{RECOVERY_LEAD}{CASE}{SENSITIVE_BECAUSE}2 member or support group(s) ...");
        assert_eq!(outcome(&recovered), established);
        // not_attempted_reason_not_recorded: the not-established sentence, not run.
        assert_eq!(outcome(&not_established(NOT_RUN)), SupportStateConfirmation::of("not_attempted_reason_not_recorded", None));
        // not_established_detail_quoted: any other reason, quoted.
        let reason = "support support:C-lift: sign margin 1e-12 not above its bound 3e-12";
        assert_eq!(outcome(&not_established(reason)), SupportStateConfirmation::of("not_established_detail_quoted", Some(reason.into())));
        // no_confirmation_record: neither sentence, or one that does not end the message.
        assert_eq!(outcome(HEAD), none);
        assert_eq!(support_state_confirmation(CASE, &lifted(), &[]), none);
        assert_eq!(support_state_confirmation(CASE, &lifted(), &[integrity("load:O1", &confirmed)]), none);
        assert_eq!(outcome(&format!("{confirmed} trailing")), none);
        assert_eq!(outcome(&format!("{confirmed}{RECOVERY_LEAD}load:O1{SENSITIVE_BECAUSE}2")), none);
        assert_eq!(outcome(&format!("{confirmed}{confirmed}")), none);
        assert_eq!(outcome(&format!("{HEAD}{NOT_ESTABLISHED_LEAD}a{NOT_ESTABLISHED_LEAD}b{NOT_ESTABLISHED_TAIL}")), none);
        assert_eq!(outcome(&format!("{}{NOT_ESTABLISHED_TAIL}", HEAD)), none);
        // A blank reason, which the producer never forms, quotes nothing.
        assert_eq!(outcome(&not_established("")), none);
        assert_eq!(outcome(&not_established("  ")), none);
        for found in [established, none] {
            assert!(CONFIRMATION_OUTCOMES.contains(&found.outcome.as_str()));
        }
    }

    #[test]
    fn the_published_member_carries_a_null_detail_unless_one_is_quoted() {
        assert_eq!(outcome(HEAD).to_json(), json!({"outcome": "no_confirmation_record", "detail": null}));
        assert_eq!(outcome(&not_established("r")).to_json(), json!({"outcome": "not_established_detail_quoted", "detail": "r"}));
    }

    #[test]
    fn the_copied_text_is_the_producers_and_report_standings() {
        let product = [
            format!("const NONLINEAR_CONFIRMED_EVIDENCE_LINE: &str = \"{NONLINEAR_CONFIRMED_LINE}\";"),
            format!("format!(\"{}{{reason}}{NOT_ESTABLISHED_TAIL}\")", NOT_ESTABLISHED_LEAD.trim_start()),
            format!("\"{NOT_RUN}\""),
            format!("fn integrity_diagnostic_id(case_id: &str) -> String {{\n    format!(\"{INTEGRITY_DIAGNOSTIC_PREFIX}{{case_id}}\")"),
            "record.message = format!(\"{} {NONLINEAR_CONFIRMED_EVIDENCE_LINE}\", record.message);".to_string(),
        ];
        for pin in &product {
            assert!(PRODUCT_LIB.contains(pin.as_str()), "product_physics/src/lib.rs: {pin}");
        }
        let guard = [
            format!("\"{}{{case_id}}{SENSITIVE_BECAUSE}{{}}", RECOVERY_LEAD.trim_start()),
            "diagnostic.message = format!(\"{} {}\", diagnostic.message, finding.sentence);".to_string(),
        ];
        for pin in &guard {
            assert!(PRODUCT_GUARD.contains(pin.as_str()), "product_physics/src/formation_guard.rs: {pin}");
        }
        let standing = [
            format!("const NOT_ESTABLISHED_LEAD: &str = \"{NOT_ESTABLISHED_LEAD}\";"),
            format!("const NOT_ESTABLISHED_TAIL: &str = \"{NOT_ESTABLISHED_TAIL}\";"),
            format!("const NOT_RUN: &str = \"{NOT_RUN}\";"),
            format!("const RECOVERY_LEAD: &str = \"{RECOVERY_LEAD}\";"),
            format!("const SENSITIVE_BECAUSE: &str = \"{SENSITIVE_BECAUSE}\";"),
        ];
        for pin in &standing {
            assert!(REPORT_STANDING.contains(pin.as_str()), "report_standing/src/report_standing.rs: {pin}");
        }
        let line = format!("pub(crate) const NONLINEAR_CONFIRMED_LINE: &str = \"{NONLINEAR_CONFIRMED_LINE}\";");
        assert!(HISTORICAL_HEALTH.contains(line.as_str()), "report_standing/src/historical_health.rs: {line}");
        for token in CONFIRMATION_OUTCOMES {
            let pin = format!("=> \"{token}\",");
            assert!(VOCABULARY.contains(pin.as_str()), "numerical_vocabulary/src/lib.rs: {pin}");
        }
    }
}
