//! No-solve admission of a complete model document before assembly.
//!
//! A caller that solves only a projection of a document (one case, no
//! combinations, as hanger sizing's derived solves do: DEL-04-11 REQ-003, REQ-009)
//! needs what the ordinary solve would refuse in the cases, combinations, states
//! and references the projection drops. [`admit_source_before_assembly`] runs, on
//! the complete document, the ordinary solve's own stages up to assembly, in the
//! ordinary order and with the ordinary early returns, and then builds each case's
//! primitive loads from the case the ordinary solve applies. It copies no rule:
//! every stage is the ordinary function itself.
//!
//! Stages, as `run_linear_static_preview_observed_inner` calls them:
//!
//! 1. the JSON entry's request admission: the closed-record refusal
//!    (`model_fields::unknown_field_refusal`) and the request parse
//!    (`source_receipt::CapturedInvocation::parse`: its encoding check, digest and
//!    decode). A refusal here is returned as `Err` with the same text the ordinary
//!    JSON entry returns;
//! 2. the lowering (`operating_conditions::lower`), the profile check and the
//!    junction-closure warnings (`pressure_runtime`), the load/reference-state
//!    document check (`case_state::resolve::validate_document`), the named
//!    imposed-motion checks (`imposed_motion::validate`, with its early return),
//!    generic validation (`validation::validate_model_inputs`: ids, references,
//!    provenance, settings), support family tokens and the partial-span refusals;
//! 3. after the first blocking return: interim combination withholding, applied
//!    self-weight validation, shared sections, unit normalisation and the eccentric
//!    offset refusals, each with the ordinary return;
//! 4. on a pressure-free document without load states, the base material
//!    resolution; on a load/reference-state document, every case's resolution
//!    (`case_state::resolve::resolve_case`), the resolved span-strain refusals and
//!    the all-reference-state refusal, with the ordinary return;
//! 5. for every case, the ordinary primitive-load construction
//!    (`build_load_case_primitive_loads`, which judges each load's category,
//!    direction, dimension, magnitude and node or pipe target, and appends the
//!    equivalent-static generated loads). It is given the case the ordinary solve
//!    applies: on a load/reference-state document the resolved `effective_case`,
//!    whose primitives are exactly the case's ledger, so a stored primitive the
//!    ledger excludes is never judged here, as it is never applied there; otherwise
//!    the case as authored.
//!
//! Not judged here, by design:
//!
//! - assembly and everything after it: the built model, topology and support
//!   boundary checks, stiffness formation, the resource guards, the exact-route
//!   pressure case of each case (`pressure_runtime::build_pressure_case_with_members`)
//!   and the load application's index checks (`prepare_loads`), which need the
//!   built model; and the solve, its convergence and stability. A case the caller
//!   does not solve is therefore admitted on its declarations and references only,
//!   never on its convergence or stability. A case the caller solves gets every
//!   ordinary check in that solve;
//! - the entry's two inline namespace checks (document kind, schema version). They
//!   read whole-document members that a projection does not change, so the
//!   caller's first ordinary solve judges them exactly; they are not copied here.
//!
//! Computation reused, and its cost: one request parse with its canonical
//! encoding check and digest, the lowering and every validation pass over the whole
//! document, the unit normalisation of a working copy, every case's state
//! resolution (member pairs and prescribed values, no matrix) and every case's
//! primitive-load list. No model is built or assembled, no matrix is formed, no
//! solve or retained-precision entry runs, and nothing is observed or captured.
//! Peak memory is a few copies of the document's typed form.
//!
//! The diagnostics are those the ordinary solve's same stages publish, in their
//! order. For a document with one fault reached by these stages, the blocking
//! codes and affected ids equal the ordinary solve's of the same document. For a
//! document with several faults no first error is promised: the ordinary solve
//! interleaves assembly and case solving with step 5, and stops at the first
//! blocking case.

use crate::{case_state, has_blocking, Diagnostic, PreviewSolverMode};

/// Admits a complete solve request `{model, materials}` without assembling or
/// solving it: `Err` with the ordinary JSON entry's refusal text when the request is
/// not admitted at parse, otherwise every diagnostic the ordinary stages before
/// assembly publish (the module documentation lists them). The document is
/// admitted when none is blocking.
pub fn admit_source_before_assembly(
    actual_request: serde_json::Value,
    solver_mode: PreviewSolverMode,
) -> Result<Vec<Diagnostic>, String> {
    if let Some(refusal) = crate::model_fields::unknown_field_refusal(&actual_request) {
        return Err(refusal);
    }
    let (request, _capture) =
        crate::source_receipt::CapturedInvocation::parse(actual_request, solver_mode).map_err(|error| error.0)?;
    let mut model = request.model;
    let request_materials_supplied = !request.materials.is_empty();
    let mut materials = if request.materials.is_empty() { model.materials.clone() } else { request.materials };
    let mut diagnostics = Vec::new();

    crate::operating_conditions::lower(&mut model, &mut diagnostics);
    crate::pressure_runtime::validate_profile(&model, &mut diagnostics);
    crate::pressure_runtime::junction_closure_warnings(&model, &mut diagnostics);
    case_state::resolve::validate_document(&model, request_materials_supplied, &mut diagnostics);
    if crate::imposed_motion::validate(&model, &mut diagnostics) {
        return Ok(diagnostics);
    }
    crate::validation::validate_model_inputs(&model, &materials, &mut diagnostics);
    crate::validate_support_family_tokens(&model, &mut diagnostics);
    crate::pf_timoshenko::append_partial_span_refusals(&model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Ok(diagnostics);
    }
    crate::document_members::withhold_interim_combinations(&mut model, &mut diagnostics);
    crate::self_weight::validate_applied_self_weight(&model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Ok(diagnostics);
    }
    crate::resolve_shared_sections(&mut model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Ok(diagnostics);
    }
    crate::normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Ok(diagnostics);
    }
    crate::rigid_component::append_eccentric_offset_refusals(&model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Ok(diagnostics);
    }
    let load_state = case_state::is_load_state(&model);
    if !load_state {
        crate::pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    }
    if has_blocking(&diagnostics) {
        return Ok(diagnostics);
    }
    let resolved = if load_state {
        let resolved = model
            .load_cases
            .iter()
            .filter_map(|case| case_state::resolve::resolve_case(&model, &materials, case, &mut diagnostics))
            .collect::<Vec<_>>();
        if has_blocking(&diagnostics) || resolved.len() != model.load_cases.len() {
            return Ok(diagnostics);
        }
        case_state::resolve::refuse_resolved_span_strain(&model, &resolved, &mut diagnostics);
        case_state::resolve::refuse_all_reference_states(&resolved, &mut diagnostics);
        if has_blocking(&diagnostics) {
            return Ok(diagnostics);
        }
        Some(resolved)
    } else {
        None
    };
    for (index, case) in model.load_cases.iter().enumerate() {
        let applied = resolved.as_ref().map_or(case, |cases| &cases[index].effective_case);
        crate::build_load_case_primitive_loads(&model, applied, &mut diagnostics);
    }
    Ok(diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn request() -> Value {
        let model: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap();
        json!({"model": model, "materials": []})
    }

    fn blocking(diagnostics: &[Diagnostic]) -> Vec<(String, Vec<String>)> {
        diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| (d.code.clone(), d.affected_refs.clone())).collect()
    }

    fn ordinary(request: &Value) -> Result<Vec<(String, Vec<String>)>, String> {
        crate::run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::DenseScrutiny)
            .map(|envelope| blocking(&envelope.diagnostics))
    }

    #[test]
    fn an_admitted_document_has_no_blocking_diagnostic_here_or_in_the_ordinary_solve() {
        let request = request();
        assert_eq!(ordinary(&request), Ok(Vec::new()));
        let admitted = admit_source_before_assembly(request, PreviewSolverMode::DenseScrutiny).unwrap();
        assert_eq!(blocking(&admitted), Vec::new());
    }

    #[test]
    fn a_load_target_unknown_in_the_second_case_is_refused_with_the_ordinary_code_and_ids() {
        let mut request = request();
        let loads = request["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap();
        let load = loads.iter_mut().find(|l| l["target"]["type"] == "node").unwrap();
        load["target"]["node"] = json!("node:not-in-model");
        let expected = ordinary(&request).unwrap();
        assert!(expected.iter().any(|(code, _)| code == "LOAD_NODE_UNKNOWN"), "{expected:?}");
        let admitted = admit_source_before_assembly(request, PreviewSolverMode::DenseScrutiny).unwrap();
        assert_eq!(blocking(&admitted), expected);
    }

    #[test]
    fn a_request_refused_at_parse_gives_the_ordinary_entrys_text() {
        let mut unknown = request();
        unknown["model"]["supports"][0]["not_a_member"] = json!(true);
        let mut malformed = request();
        malformed["model"]["load_cases"][0]["primitive_loads"] = json!("not a list");
        for request in [unknown, malformed] {
            let expected = ordinary(&request).unwrap_err();
            assert_eq!(admit_source_before_assembly(request, PreviewSolverMode::DenseScrutiny).unwrap_err(), expected);
        }
    }
}
