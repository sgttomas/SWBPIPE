//! The derived solves of hanger sizing (DEL-04-11 `Work/hanger-sizing.md` §2 to §5
//! and §7). Each derived solve is a modified copy of the caller's solve request,
//! `{model, materials}`, sent through the product's public JSON solve entry, the
//! one the model's own cases use. The stored model is never changed, and nothing
//! here passes through the operation route.

use crate::{
    check_method, size, verification_standing, verify, Candidate, CandidateResult, Dof, FamilyPolicy, Method,
    Positions, Record, Refusal, Selected, Verification, VerificationStanding,
};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};
use open_pipe_stress_units::{canonical_unit, convert, unit_by_symbol, Dimension};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// The nonlinear-support state the hot-load basis is formed in (Design §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightSupportState {
    /// The weight case's own converged states. The only value admitted until the
    /// solve can hold another case's states (Design §14, O-1).
    OwnSolve,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestCandidate {
    pub support: String,
    pub family_policy: FamilyPolicy,
}

/// The sizing request (Design §2). Every member is the person's input.
#[derive(Debug, Clone)]
pub struct SizingRequest {
    pub weight_case: String,
    pub weight_support_state: WeightSupportState,
    pub design_operating_case: String,
    pub check_operating_cases: Vec<String>,
    pub candidates: Vec<RequestCandidate>,
    pub method: Method,
    /// The solver mode the caller's own solve of these cases uses (Design §4.1).
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

/// A non-blocking disclosure of the run (Design §5).
#[derive(Debug, Clone, PartialEq)]
pub struct Disclosure {
    pub code: &'static str,
    pub names: Vec<String>,
    pub message: String,
}

/// Each nonlinear support's converged state in one derived solve.
#[derive(Debug, Clone, PartialEq)]
pub struct SolveStates {
    pub solve: String,
    pub states: BTreeMap<String, String>,
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

/// Case checks (Design §2): every named case exists, carries no test condition and
/// has one role; the exact route waits for increment 4.
pub fn check_cases(request_doc: &Value, request: &SizingRequest) -> Vec<Refusal> {
    let model = model(request_doc);
    let mut refusals = Vec::new();
    if is_present(model.get("pressure_contract")) {
        refusals.push(refusal("HANGER_SIZING_EXACT_ROUTE_PENDING", Vec::new(),
            "hanger sizing on the exact pressure route arrives with increment 4 (Design §13); this slice sizes on the pressure-free route"));
    }
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

fn mentions(value: &Value, id: &str) -> bool {
    match value {
        Value::String(s) => s == id,
        Value::Array(items) => items.iter().any(|v| mentions(v, id)),
        Value::Object(map) => map.values().any(|v| mentions(v, id)),
        _ => false,
    }
}

/// Whether any model object other than the candidate itself refers to it. The
/// stored diagnostics and status are records about the model, not model objects.
fn referenced(model: &Value, id: &str) -> bool {
    let Some(object) = model.as_object() else { return false };
    object.iter().any(|(key, value)| match key.as_str() {
        "diagnostics" | "analysis_status" | "project" | "data_boundary" => false,
        "supports" => value.as_array().into_iter().flatten().any(|s| s.get("id").and_then(Value::as_str) != Some(id) && mentions(s, id)),
        _ => mentions(value, id),
    })
}

/// Candidate checks (Design §3).
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
        let restraints: Vec<&str> = array(support, "restraints").iter().filter_map(Value::as_str).collect();
        let dof = match restraints.as_slice() {
            [only] => parse_dof(only),
            _ => None,
        };
        let Some(dof) = dof else {
            flag("HANGER_SIZING_CANDIDATE_DOF_UNSUPPORTED", id);
            continue;
        };
        if is_present(support.get("direction")) {
            flag("HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED", id);
        }
        let family = support.get("family").and_then(Value::as_str);
        if is_present(support.get("nonlinear")) || matches!(family, Some("anchor" | "snubber" | "nonlinear")) {
            flag("HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED", id);
        }
        let node = support.get("node").and_then(Value::as_str).unwrap_or_default().to_string();
        let shared = supports.iter().any(|other| {
            other.get("id").and_then(Value::as_str) != Some(id)
                && other.get("node").and_then(Value::as_str) == Some(node.as_str())
                && array(other, "restraints").iter().filter_map(Value::as_str).any(|r| parse_dof(r) == Some(dof))
        });
        if shared {
            flag("HANGER_SIZING_CANDIDATE_DOF_SHARED", id);
        }
        if is_present(support.get("boundary_association")) || referenced(model, id) {
            flag("HANGER_SIZING_CANDIDATE_REFERENCED", id);
        }
        sites.push(Site { support: id.to_string(), node, dof });
    }
    if refusals.is_empty() {
        Ok(sites)
    } else {
        Err(refusals
            .into_iter()
            .map(|(code, names)| refusal(code, names, "the candidate does not meet the sizing rules of Design §3"))
            .collect())
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
    /// A spring of rate k on its DOF with the force F_cold: F(u) = F_cold − k·u (§7).
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

/// The derived document for one case (Design §4.1): only that case, no
/// combinations, and each candidate in its role.
pub fn derived_document(request_doc: &Value, case: &str, sites: &[Site], roles: &[Role]) -> Value {
    let mut doc = request_doc.clone();
    let model = doc["model"].as_object_mut().expect("solve request has a model");
    if let Some(Value::Array(cases)) = model.get_mut("load_cases") {
        cases.retain(|c| c.get("id").and_then(Value::as_str) == Some(case));
    }
    if model.contains_key("combinations") {
        model.insert("combinations".into(), json!([]));
    }
    let mut added = Vec::new();
    if let Some(Value::Array(supports)) = model.get_mut("supports") {
        for (site, role) in sites.iter().zip(roles) {
            let index = supports.iter().position(|s| s.get("id").and_then(Value::as_str) == Some(site.support.as_str()));
            let Some(index) = index else { continue };
            match role {
                Role::Force(value) => {
                    supports.remove(index);
                    added.push(force_primitive(site, *value));
                }
                Role::Rigid | Role::Spring { .. } => {
                    let support = supports[index].as_object_mut().expect("support object");
                    for key in ["hanger", "stiffness", "nonlinear"] {
                        support.remove(key);
                    }
                    support.insert("restraints".into(), json!([dof_token(site.dof)]));
                    if let Role::Spring { k, f_cold } = role {
                        support.insert("family".into(), json!("spring"));
                        support.insert("stiffness".into(), json!({"dof": dof_token(site.dof), "value": {"value": k, "unit": "N/m"}}));
                        added.push(force_primitive(site, *f_cold));
                    } else {
                        // `guide` is the product's family for a rigid restraint of
                        // named translational DOFs (Design §4.1).
                        support.insert("family".into(), json!("guide"));
                    }
                }
            }
        }
    }
    if let Some(Value::Array(cases)) = model.get_mut("load_cases") {
        for case in cases.iter_mut() {
            let loads = case.as_object_mut().expect("case object").entry("primitive_loads").or_insert(json!([]));
            if let Value::Array(loads) = loads {
                loads.extend(added.iter().cloned());
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

/// The one call site of every derived solve (Design §10).
fn solve(doc: Value, mode: PreviewSolverMode, label: &str) -> Result<MechanicsEnvelope, Refusal> {
    let envelope = run_linear_static_preview_value_with_mode(doc, mode)
        .map_err(|error| refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED", vec![label.to_string()], error))?;
    let blocking: Vec<String> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect();
    if blocking.is_empty() {
        Ok(envelope)
    } else {
        Err(refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED", vec![label.to_string()], blocking.join("; ")))
    }
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

fn states(envelope: &MechanicsEnvelope, solve: &str) -> SolveStates {
    let states = envelope
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
    SolveStates { solve: solve.to_string(), states }
}

fn missing(label: &str, what: &str, sites: &[&Site]) -> Refusal {
    refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED",
        std::iter::once(label.to_string()).chain(sites.iter().map(|s| s.support.clone())).collect(),
        format!("the derived solve published no {what} at the named candidates"))
}

/// Runs one derived solve and reads each candidate's displacement along its DOF.
fn positions_of(request_doc: &Value, case: &str, sites: &[Site], roles: &[Role], mode: PreviewSolverMode, label: &str)
    -> Result<(Vec<f64>, SolveStates), Refusal> {
    let envelope = solve(derived_document(request_doc, case, sites, roles), mode, label)?;
    let values: Vec<Option<f64>> = sites.iter().map(|s| displacement(&envelope, s)).collect();
    let absent: Vec<&Site> = sites.iter().zip(&values).filter(|(_, v)| v.is_none()).map(|(s, _)| s).collect();
    if !absent.is_empty() {
        return Err(missing(label, "displacement", &absent));
    }
    Ok((values.into_iter().map(Option::unwrap).collect(), states(&envelope, label)))
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

/// The whole sizing run (Design §1): checks, S_W, S_C and S_Oi, the arithmetic,
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
    if !refusals.is_empty() {
        return Err(refusals);
    }

    // S_W: the weight case with every candidate rigid.
    let weight = request.weight_case.as_str();
    let rigid = vec![Role::Rigid; sites.len()];
    let envelope = solve(derived_document(request_doc, weight, &sites, &rigid), mode, "S_W").map_err(|r| vec![r])?;
    let w: Vec<Option<f64>> = sites.iter().map(|s| reaction(&envelope, s)).collect();
    let absent: Vec<&Site> = sites.iter().zip(&w).filter(|(_, v)| v.is_none()).map(|(s, _)| s).collect();
    if !absent.is_empty() {
        return Err(vec![missing("S_W", "support reaction", &absent)]);
    }
    let w: Vec<f64> = w.into_iter().map(Option::unwrap).collect();
    let mut solve_states = vec![states(&envelope, "S_W")];
    let not_positive: Vec<String> = sites.iter().zip(&w).filter(|(_, w)| !(**w > 0.0)).map(|(s, _)| s.support.clone()).collect();
    if !not_positive.is_empty() {
        return Err(vec![refusal("HANGER_SIZING_LOAD_NOT_POSITIVE", not_positive,
            "the rigid reaction at each named candidate is not positive along its DOF; sizing never flips a sign, so remove the candidate and run again")]);
    }

    // S_C and S_Oi: every candidate replaced by its force W.
    let forces: Vec<Role> = w.iter().map(|w| Role::Force(*w)).collect();
    let (u_cold, cold_states) = positions_of(request_doc, weight, &sites, &forces, mode, "S_C").map_err(|r| vec![r])?;
    solve_states.push(cold_states);
    let operating = request.operating_cases();
    let mut u_operating = vec![Vec::new(); sites.len()];
    for (i, case) in operating.iter().enumerate() {
        let label = format!("S_O{}", i + 1);
        let (values, case_states) = positions_of(request_doc, case, &sites, &forces, mode, &label).map_err(|r| vec![r])?;
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

    // Verification: the selections in place (§7).
    let roles: Vec<Role> = results
        .iter()
        .zip(&candidates)
        .map(|(result, candidate)| match &result.selected {
            Some(Selected::Variable { k, loads, .. }) => Role::Spring { k: *k, f_cold: loads.f_cold_n },
            _ => Role::Force(candidate.w_n),
        })
        .collect();
    let (v_cold, verified_cold_states) = positions_of(request_doc, weight, &sites, &roles, mode, "V_C").map_err(|r| vec![r])?;
    solve_states.push(verified_cold_states);
    let mut v_operating = vec![Vec::new(); sites.len()];
    for (i, case) in operating.iter().enumerate() {
        let label = format!("V_O{}", i + 1);
        let (values, case_states) = positions_of(request_doc, case, &sites, &roles, mode, &label).map_err(|r| vec![r])?;
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

    // Disclosures (§5).
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
    let mut case_load_categories = BTreeMap::new();
    for case in std::iter::once(weight).chain(operating.iter().copied()) {
        case_load_categories.insert(case.to_string(), categories(request_doc, case));
    }
    let standing = verification_standing(&results);
    Ok(SizingRun { sites, candidates, results, verifications, standing, states: solve_states, disclosures, case_load_categories })
}
