//! No-solve admission of a complete model document before assembly.
//!
//! A caller that solves only a projection of a document (one case, no
//! combinations, as hanger sizing's derived solves do: DEL-04-11 REQ-003, REQ-009)
//! needs what the ordinary solve would refuse in the cases, combinations, states
//! and references the projection drops. [`admit_source_before_assembly`] runs, on
//! the complete document, the ordinary solve's own stages before assembly, in the
//! ordinary order and with the ordinary early returns, and then each case's
//! assembly-independent preparation. It copies no rule: every stage is the
//! ordinary function itself. The one local text is the wrapping of the load
//! application's findings as `LOAD_INPUT_INVALID`, which the ordinary solve also
//! writes inline (`solve_prepared_load_case`), with the same code, severity,
//! message and affected ids.
//!
//! Stages, as `run_linear_static_preview_observed_inner` calls them:
//!
//! 1. the JSON entry's request admission: the closed-record refusal
//!    (`model_fields::unknown_field_refusal`) and the request parse
//!    (`source_receipt::CapturedInvocation::parse`). A refusal here is returned as
//!    `Err` with the same text the ordinary JSON entry returns. The parse forms its
//!    capture (the raw request, its canonical length and its digest) as it always
//!    does; no observer or retained entry receives it, and it is dropped at once;
//! 2. the lowering (`operating_conditions::lower`), the profile check and the
//!    junction-closure warnings (`pressure_runtime`), the load/reference-state
//!    document check (`case_state::resolve::validate_document`), the named
//!    imposed-motion checks (`imposed_motion::validate`, with its early return),
//!    generic validation (`validation::validate_model_inputs`: ids, references,
//!    provenance, settings), support family tokens and the partial-span refusals;
//! 3. after the first blocking return: interim combination withholding, applied
//!    self-weight validation, shared sections, unit normalisation and the eccentric
//!    offset refusals, each with the ordinary return;
//! 4. on every document without load states, the base material resolution
//!    (`pressure_material::resolve_base`, which acts only on the exact route); on a
//!    load/reference-state document, every case's resolution
//!    (`case_state::resolve::resolve_case`), the resolved span-strain refusals and
//!    the all-reference-state refusal, with the ordinary return;
//! 5. each case in order, with the ordinary return after each case:
//!    - on a document without load states only (the ordinary solve reads no
//!      selector when cases are resolved), the case's modulus-basis selector
//!      (`modulus_basis_key`) and, for a basis key not yet resolved, its material
//!      set (`materials_for_modulus_basis`: on the exact route
//!      `pressure_material::resolve_case`, otherwise the exact-point or
//!      interpolated selection with its derived shear modulus). As in the
//!      ordinary solve, one basis key is resolved once, so its findings appear once;
//!    - the ordinary primitive-load construction (`build_load_case_primitive_loads`,
//!      which judges each load's category, direction, dimension, magnitude and node
//!      or pipe target, and appends the equivalent-static generated loads), given
//!      the case the ordinary solve applies: on a load/reference-state document the
//!      resolved `effective_case`, whose primitives are exactly the case's ledger,
//!      so a stored primitive the ledger excludes is never judged here, as it is
//!      never applied there; otherwise the case as authored;
//!    - the load application (`prepare_loads`) with the model's node and pipe
//!      counts. On a document the build admits these equal the built counts the
//!      ordinary solve passes: the build forms one node per model node and one
//!      element per pipe segment, in model order, and omits one only with a
//!      blocking diagnostic (curved and connector-replaced spans leave the frame
//!      elements, not the pipes). The node and pipe indices of each load are model
//!      positions in both.
//!
//! Not judged here, by design:
//!
//! - assembly and everything after it: the built model, topology and support
//!   boundary checks, stiffness formation, the resource guards, the checks inside
//!   each case's built preparation (among them the exact-route pressure case,
//!   `pressure_runtime::build_pressure_case_with_members`, the thermal element
//!   loads and the skew prescribed-motion refusal), and the solve, its convergence
//!   and stability. A case the caller does not solve is therefore admitted on its
//!   declarations, references and assembly-independent preparation only, never on
//!   its convergence or stability. A case the caller solves gets every ordinary
//!   check in that solve;
//! - the entry's two inline namespace checks (document kind, schema version). They
//!   read whole-document members that a projection does not change, so the
//!   caller's first ordinary solve judges them exactly; they are not copied here.
//!
//! Computation and allocation. The caller's request `Value` is moved in. The parse
//! writes its compact and canonical texts (two strings of the document's encoded
//! size), hashes a payload that holds a clone of the request `Value`, and decodes a
//! further clone into the typed request; the capture keeps the moved request and is
//! dropped after the parse. Then: one material list (the request's, or a clone of
//! the model's), the lowering, validation and normalisation of the typed model in
//! place; on a load/reference-state document one resolved case per case (a record
//! per member, the selected pairs, the prescribed values, an effective case holding
//! the ledgered primitives and its evidence `Value`), held until the end; per basis
//! key, one resolved material list (interpolation included), dropped at once; per
//! case, its primitive-load list and load application, dropped before the next
//! case. No model is built or assembled, no matrix is formed, and no solve, observer
//! or retained-precision entry runs.
//!
//! The diagnostics are those the ordinary solve's same stages publish, in their
//! order. For a document with one fault reached by these stages, the blocking
//! codes and affected ids equal the ordinary solve's of the same document. For a
//! document with several faults no first error is promised: the ordinary solve
//! interleaves assembly and each case's built preparation and solve with step 5.

use crate::{case_state, diag, has_blocking, Diagnostic, PreviewSolverMode};
use std::collections::HashSet;

/// Admits a complete solve request `{model, materials}` without assembling or
/// solving it: `Err` with the ordinary JSON entry's refusal text when the request is
/// not admitted at parse, otherwise every diagnostic the ordinary stages before
/// assembly and each case's assembly-independent preparation publish (the module
/// documentation lists them). The document is admitted when none is blocking.
///
/// The request is judged exactly as given. A caller whose every solve replaces some
/// members passes the document with those members replaced as its solves replace
/// them, and nothing else changed, so that data it never solves cannot refuse the
/// request and nothing it does solve is hidden. Hanger sizing (DEL-04-11 REQ-002,
/// REQ-009) passes the whole authored request (every case, combination, support,
/// state, load, reference and setting) after running its candidate rules on the
/// unmodified request, with only each candidate those rules admit changed:
/// `family` set to `guide`, `restraints` set to its one admitted DOF, and its
/// `hanger`, `stiffness` and `nonlinear` members removed. Its id, node, direction,
/// provenance and every other member, and every state, motion and reference that
/// names it, are kept. A candidate the rules refuse (a direction, a nonlinear,
/// snubber or anchor family, a reference to it) is refused before this call and
/// never reaches it. The authored request is not changed and remains what the
/// sizing result binds.
pub fn admit_source_before_assembly(
    actual_request: serde_json::Value,
    solver_mode: PreviewSolverMode,
) -> Result<Vec<Diagnostic>, String> {
    if let Some(refusal) = crate::model_fields::unknown_field_refusal(&actual_request) {
        return Err(refusal);
    }
    let (request, capture) =
        crate::source_receipt::CapturedInvocation::parse(actual_request, solver_mode).map_err(|error| error.0)?;
    drop(capture);
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
    let mut resolved_bases: HashSet<String> = HashSet::new();
    for (index, case) in model.load_cases.iter().enumerate() {
        let applied = match &resolved {
            Some(cases) => &cases[index].effective_case,
            None => {
                match crate::modulus_basis_key(case, &mut diagnostics) {
                    Some(key) if !resolved_bases.contains(&key) => {
                        // The ordinary solve returns here when the selection fails, and
                        // after building its basis on any blocking diagnostic.
                        let selected = crate::materials_for_modulus_basis(&model, &materials, case, &mut diagnostics);
                        if selected.is_none() || has_blocking(&diagnostics) {
                            return Ok(diagnostics);
                        }
                        resolved_bases.insert(key);
                    }
                    Some(_) => {}
                    None if has_blocking(&diagnostics) => return Ok(diagnostics),
                    None => {}
                }
                case
            }
        };
        let loads = crate::build_load_case_primitive_loads(&model, applied, &mut diagnostics);
        let application = crate::prepare_loads(model.nodes.len(), model.pipe_segments.len(), &loads);
        for finding in &application.findings {
            diagnostics.push(diag(
                &format!("diagnostic:load:{}", finding.load_id),
                "LOAD_INPUT_INVALID",
                "blocking",
                finding.message.clone(),
                vec![finding.load_id.clone(), applied.id.clone()],
            ));
        }
        if has_blocking(&diagnostics) {
            return Ok(diagnostics);
        }
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

    /// The load application's counts: on an admitted document the ordinary build
    /// forms one node per model node and one pipe element per pipe segment.
    #[test]
    fn the_built_counts_of_an_admitted_document_are_the_model_counts() {
        let request = request();
        let (parsed, _capture) = crate::source_receipt::CapturedInvocation::parse(request, PreviewSolverMode::DenseScrutiny).unwrap();
        let mut model = parsed.model;
        let mut materials = if parsed.materials.is_empty() { model.materials.clone() } else { parsed.materials };
        let mut diagnostics = Vec::new();
        crate::operating_conditions::lower(&mut model, &mut diagnostics);
        crate::resolve_shared_sections(&mut model, &mut diagnostics);
        crate::normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        let built = crate::build_model(&model, &materials, &mut diagnostics).expect("the demo model builds");
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        assert_eq!(built.nodes.len(), model.nodes.len());
        assert_eq!(built.pipes.len(), model.pipe_segments.len());
        for (element, pipe) in built.pipes.iter().zip(&model.pipe_segments) {
            assert_eq!(element.element_id, pipe.id);
        }
    }
}
