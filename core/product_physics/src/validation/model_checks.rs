//! On-demand model checks (DEL-04-06 Design `on-demand-model-checks.md`): the
//! solve's own input checks, run without forming a stiffness or solving.
//!
//! Both entries take the request the desktop's solve takes
//! (`{"model": …, "materials": []}`) and parse it as the solve does.
//!
//! - [`LinearStaticPreviewRequest::pre_formation_checks_value`] runs
//!   `crate::pre_formation`, the one function that holds every step
//!   `run_linear_static_preview_observed_inner` takes before it forms the
//!   first stiffness: lowering, validation, self weight, shared sections, unit
//!   normalization, eccentric offsets, the base material pair, the per-case
//!   load-state resolution, the model build, the topology checks and the
//!   restraint pre-checks. The solve calls the same function, so its findings
//!   are the solve's leading diagnostics, and where it stops the solve stops.
//! - [`LinearStaticPreviewRequest::solve_input_checks_value`] adds, when
//!   nothing before formation blocks, the refusals the solve's load generation
//!   (`build_load_case_primitive_loads`, with its equivalent-static generation)
//!   raises for every load case, in load-case order. The generation's one
//!   statement that refuses nothing, the disclosure that an authored category
//!   is applied under a preview category (`LOAD_CATEGORY_PREVIEW_MAPPED`, a
//!   warning on each such load), is the solve's to publish and is not listed. The solve runs that
//!   generation for one case at a time after it has formed that case's
//!   stiffness, and stops at the first case that refuses; here it runs for
//!   every case, on the model `pre_formation` leaves (a load-state case on its
//!   resolved effective case, as the solve does), and its loads are dropped.
//!   So a missing solve input (a pipe's density for a seismic case, say) is
//!   reported, on each entity that lacks it, without a solve.
//!
//! Where a finding blocks, the findings are those the solve's blocked envelope
//! publishes: on the preview route (a model neither exact nor load-state)
//! `blocked_envelope` strips every `result:` ref, which can no longer resolve
//! in a document with no rows, and drops a non-blocking diagnostic that named
//! one ([`crate::preview_physics::sanitize_blocked_diagnostics`]); the checks
//! apply the same step on the same lowered model.
//!
//! No diagnostic is built here: each finding is one the solve's own code
//! states. The retained-memory observer, the dense-scrutiny size guard, the
//! modulus-basis resolution and the pressure preparation of each case, and the
//! load application (`prepare_loads`) are solve work after formation and do not
//! run here. Nothing here writes to the model; the caller's request is
//! consumed. `tests/model_checks_runtime.rs` holds the checks and the solve in
//! step over its corpus.
use crate::*;

/// `fixtures/diagnostics/solve_input_missing_codes.json`, the one list of the
/// codes that mean a solve input is missing, which the desktop reads too.
pub const SOLVE_INPUT_MISSING_CODES_JSON: &str = include_str!("../../../../fixtures/diagnostics/solve_input_missing_codes.json");

/// The members of [`SOLVE_INPUT_MISSING_CODES_JSON`], in its order.
pub fn solve_input_missing_codes() -> Vec<String> {
    let list: serde_json::Value = serde_json::from_str(SOLVE_INPUT_MISSING_CODES_JSON).expect("the committed list is JSON");
    list["codes"].as_array().expect("the list has codes").iter().map(|code| code.as_str().expect("a code is a string").to_string()).collect()
}

/// The on-demand findings, in their two parts. [`SolveInputChecks::findings`]
/// lists them as the desktop shows them: the findings before formation, then
/// the load generation's.
#[derive(Debug, Clone)]
pub struct SolveInputChecks {
    /// The solve's diagnostics before it forms a stiffness, in its order.
    pub before_formation: Vec<Diagnostic>,
    /// When nothing before formation blocks, the blocking diagnostics of the
    /// solve's load generation for every load case, in load-case order;
    /// otherwise none.
    pub load_generation: Vec<Diagnostic>,
}

impl SolveInputChecks {
    /// Both parts, the findings before formation first.
    pub fn findings(self) -> Vec<Diagnostic> {
        let mut findings = self.before_formation;
        findings.extend(self.load_generation);
        findings
    }
}

impl LinearStaticPreviewRequest {
    /// The solve's pre-formation diagnostics for `actual_request` (the
    /// request the desktop's solve takes), or the solve's own refusal text when
    /// the request cannot be read (a model record that is not an object or has
    /// an undeclared key, or a request that does not parse).
    pub fn pre_formation_checks_value(actual_request: serde_json::Value) -> Result<Vec<Diagnostic>, String> {
        let request = checked_request(actual_request)?;
        Ok(match run_pre_formation(request) {
            PreFormation::Blocked(model, mut diagnostics) => {
                as_blocked_envelope_publishes(&model, &mut diagnostics);
                diagnostics
            }
            PreFormation::Formable(_, diagnostics) => diagnostics,
        })
    }

    /// The pre-formation diagnostics and, when none of them blocks, every load
    /// case's load-generation diagnostics; refused as
    /// [`Self::pre_formation_checks_value`] refuses.
    pub fn solve_input_checks_value(actual_request: serde_json::Value) -> Result<SolveInputChecks, String> {
        let request = checked_request(actual_request)?;
        Ok(match run_pre_formation(request) {
            PreFormation::Blocked(model, mut before_formation) => {
                as_blocked_envelope_publishes(&model, &mut before_formation);
                SolveInputChecks { before_formation, load_generation: Vec::new() }
            }
            PreFormation::Formable(formable, mut before_formation) => {
                let mut load_generation = load_generation_findings(&formable);
                if !load_generation.is_empty() {
                    // The solve stops at the first refusing case and publishes
                    // its blocked envelope over both parts. The step keeps or
                    // drops, and strips, each diagnostic by itself, so applying
                    // it to each part gives that one list, split where it was.
                    as_blocked_envelope_publishes(&formable.model, &mut before_formation);
                    as_blocked_envelope_publishes(&formable.model, &mut load_generation);
                }
                SolveInputChecks { before_formation, load_generation }
            }
        })
    }
}

/// The solve's closed-record check and parse, with its refusal text.
fn checked_request(actual_request: serde_json::Value) -> Result<LinearStaticPreviewRequest, String> {
    if let Some(refusal) = model_fields::unknown_field_refusal(&actual_request) {
        return Err(refusal);
    }
    // The solve's parse; no check before stiffness formation reads the solver mode.
    let (request, _capture) = source_receipt::CapturedInvocation::parse(actual_request, PreviewSolverMode::default())
        .map_err(|error| error.0)?;
    Ok(request)
}

/// `pre_formation` on the request's model and materials, as the solve calls it,
/// with no observer.
fn run_pre_formation(request: LinearStaticPreviewRequest) -> PreFormation {
    let model = request.model;
    let request_materials_supplied = !request.materials.is_empty();
    let materials = if request.materials.is_empty() { model.materials.clone() } else { request.materials };
    pre_formation(model, materials, request_materials_supplied, &mut |_, _| {})
}

/// The step `blocked_envelope` applies to a blocked document's diagnostics on
/// the preview route, decided on the same lowered model; on the exact and
/// load-state routes the diagnostics stand as they are.
fn as_blocked_envelope_publishes(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    if !(crate::pressure_runtime::is_exact(model) || crate::case_state::is_load_state(model)) {
        crate::preview_physics::sanitize_blocked_diagnostics(diagnostics);
    }
}

/// The solve's load generation for every case, its loads dropped, and its refusals kept.
fn load_generation_findings(formable: &Formable) -> Vec<Diagnostic> {
    let mut findings = Vec::new();
    for (index, load_case) in formable.model.load_cases.iter().enumerate() {
        // As `solve_load_case_observed`: a resolved case's loads are its effective case's.
        let load_case = formable.resolved_cases.as_ref().map_or(load_case, |cases| &cases[index].effective_case);
        drop(build_load_case_primitive_loads(&formable.model, load_case, &mut findings));
    }
    findings.retain(|finding| finding.severity == "blocking");
    findings
}
