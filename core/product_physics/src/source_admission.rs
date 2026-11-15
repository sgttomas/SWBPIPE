//! No-solve admission of a complete model document before global assembly.
//!
//! A caller that solves only a projection of a document (one case, no
//! combinations, as hanger sizing's derived solves do: DEL-04-11 REQ-003, REQ-009)
//! needs what the ordinary solve would refuse in the cases, combinations, states
//! and references the projection drops. [`admit_source_before_assembly`] runs, on
//! the complete document, the ordinary solve's own stages before its global
//! stiffness assembly, in the ordinary order and with the ordinary early returns:
//! the document-level stages, the local built model and boundary, and each case's
//! local preparation. It copies no rule: every stage is the ordinary function
//! itself. The local texts are the wrappings of the boundary's and the load
//! application's findings (`SUPPORT_INPUT_INVALID`, `LOAD_INPUT_INVALID`), which
//! the ordinary solve writes inline, with the same code, severity, message and
//! affected ids, and the prescribed-value list the skew refusal reads, formed as
//! the ordinary solve forms it.
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
//! 5. the local built model (`build_model_for_basis`, on a load/reference-state
//!    document with the first case's member pairs, and the PF shear basis
//!    `pf_timoshenko::ConsumedBases` consumes) and its boundary
//!    (`prepare_boundary`), with the ordinary return; the boundary's findings
//!    refuse as `SUPPORT_INPUT_INVALID`, written as the ordinary solve writes them;
//! 6. each case in order, with the ordinary return after each case:
//!    - on a load/reference-state document, each case after the first built on its
//!      own member pairs (`build_model_for_members`), as the ordinary solve builds
//!      it; otherwise the case's modulus-basis selector (`modulus_basis_key`, read
//!      only here, as ordinarily) and, for a basis key not yet resolved, its
//!      material set (`materials_for_modulus_basis`: on the exact route
//!      `pressure_material::resolve_case`, otherwise the exact-point or
//!      interpolated selection with its derived shear modulus) and its built model
//!      (`build_model_for_basis` with that basis's PF shear consumer). A basis is
//!      resolved and built once per key and kept, as the ordinary solve keeps its
//!      basis states; a later case with the same key reuses both, and every check
//!      below still runs for that case;
//!    - the exact-route pressure case (`pressure_runtime::build_pressure_case_with_members`:
//!      its profile pass, the region traversal, references, topology, geometry,
//!      material coverage and formed loads), its product dropped;
//!    - the ordinary primitive-load construction (`build_load_case_primitive_loads`),
//!      given the case the ordinary solve applies: on a load/reference-state
//!      document the resolved `effective_case`, whose primitives are exactly the
//!      case's ledger, so a stored primitive the ledger excludes is never judged
//!      here, as it is never applied there; otherwise the case as authored;
//!    - the load application (`prepare_loads`) with the built node and pipe counts,
//!      its findings written as the ordinary solve writes them
//!      (`LOAD_INPUT_INVALID`, the load and the case), with the ordinary return;
//!    - on a case without a resolved state, its thermal element loads
//!      (`build_thermal_element_loads`) from its basis's materials, so a selected
//!      basis without the thermal expansion a thermal load needs is refused;
//!    - the uniform element loads' equivalents (`add_uniform_element_loads`) into a
//!      scratch force ledger that is never finished, where a partial extent on a
//!      realized arc is refused;
//!    - the case's motion: moved spring bases (`load_state_spring_ground`, or
//!      `imposed_motion::case_spring_ground`) and, without a resolved state, its
//!      support-targeted primitives' prescribed map (`imposed_motion::case_prescribed`),
//!      with the ordinary return on their refusals;
//!    - on a model with skew stops, their prescribed-motion refusal
//!      (`skew::prescribed_motion_refusal`) on the prescribed values the ordinary
//!      solve forms (each restrained DOF's resolved or support-targeted motion, or
//!      zero).
//!
//! Early stops and passing paths. Every return before the end follows a blocking
//! diagnostic, so a pass means every case was prepared through every applicable
//! step above. Inside the helpers, a step is skipped without a diagnostic only
//! where it does not apply or where an earlier check guarantees it: the pressure
//! case on a document without a pressure contract; the modulus basis on a case
//! without a selector or with resolved cases; the thermal element loads on a
//! resolved case (whose eigenloads read built sections that every admitted build
//! forms) and for loads that are not thermal; the skew refusal on a model without
//! skew stops. A material selection or a built model is never absent without a
//! blocking diagnostic (the selection helpers return none only after one, and the
//! build's absence without one is the ordinary solve's own invariant).
//!
//! Not judged here, by design:
//!
//! - global stiffness assembly and everything after it: stiffness formation, the
//!   resource guards, the finished force vector and its range, the reduction and
//!   the solve, its convergence and stability. A case the caller does not solve is
//!   therefore admitted on its declarations, references and local preparation,
//!   never on its convergence or stability. A case the caller solves gets every
//!   ordinary check in that solve. The connector reference loads and constant-effort
//!   loads that the ordinary force ledger also forms are not formed here;
//! - the ordinary solve's inline whole-model checks after the build (topology
//!   warnings, no restraint at all, fewer than six ground constraints) and the
//!   entry's two inline namespace checks (document kind, schema version). They read
//!   members a projection does not change, and the derived solves' supports are the
//!   neutralised ones, so the caller's first ordinary solve judges them exactly;
//!   they are not copied here;
//! - the held-combination solves, which judge a combination's operands with their
//!   own diagnostics and publish an outcome per combination; no such outcome is
//!   formed or promoted here.
//!
//! Computation and allocation. The caller's request `Value` is moved in. The parse
//! writes its compact and canonical texts (two strings of the document's encoded
//! size), hashes a payload that holds a clone of the request `Value`, and decodes a
//! further clone into the typed request; the capture keeps the moved request and is
//! dropped after the parse. Then: one material list (the request's, or a clone of
//! the model's), the lowering, validation and normalisation of the typed model in
//! place; on a load/reference-state document one resolved case per case (a record
//! per member, the selected pairs, the prescribed values, an effective case holding
//! the ledgered primitives and its evidence `Value`), held until the end; the base
//! built model (nodes, members with their sections, local connector and curved-bend
//! data with each bend's local stiffness, supports and the skew restraint basis)
//! and its boundary, held until the end; per selected basis key, its material list
//! and built model, held until the end; on a load/reference-state document, each
//! later case's own built model, dropped after the case; per case, the pressure
//! case (its 6N-entry load arrays, terms and evidence), the primitive-load list,
//! the load application, the thermal loads, the scratch ledger of uniform-load
//! equivalents and the motion maps, each dropped within the case. No global
//! stiffness is assembled, no force vector finished, no matrix factorised, and no
//! solve, observer or retained-precision entry runs.
//!
//! The diagnostics are those the ordinary solve's same stages publish, in their
//! order, warnings included. For a document with one fault reached by these stages,
//! the blocking codes and affected ids equal the ordinary solve's of the same
//! document, which refuses the whole document. For a document with several faults
//! no first error is promised: the ordinary solve assembles and solves each case
//! between the preparations of step 6.

use crate::{case_state, diag, has_blocking, Diagnostic, PreviewSolverMode};
use std::collections::HashMap;

/// Admits a complete solve request `{model, materials}` without assembling its
/// global stiffness, factorising or solving it: `Err` with the ordinary JSON entry's
/// refusal text when the request is not admitted at parse, otherwise every
/// diagnostic the ordinary stages before global assembly and each case's local
/// preparation publish (the module documentation lists them). The document is
/// admitted when none is blocking; it then passed every applicable stage for every
/// case and every selected basis.
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
    // The local built model and boundary, as the ordinary solve forms them before
    // its first stiffness assembly: the base build (on a load/reference-state
    // document with the first case's member pairs), and the boundary prepared
    // from it, which every case uses. The boundary's findings refuse here as the
    // ordinary solve refuses them (`SUPPORT_INPUT_INVALID`, written as it writes
    // them), so no case is prepared on a boundary it refuses and no pass follows.
    // The ordinary solve's inline whole-model checks after the build (topology
    // warnings, the absence of any restraint, the six-ground-constraint count) are
    // not repeated: they read the built supports, which are the same in every
    // derived solve (each derived solve makes the candidates rigid as the
    // neutralisation does), so the first derived solve judges them exactly.
    let consumed_bases = crate::pf_timoshenko::ConsumedBases::of(&model);
    let base_built = crate::build_model_for_basis(
        &model,
        &materials,
        resolved.as_ref().map(|cases| &cases[0].pairs),
        consumed_bases.base.as_ref(),
        &mut diagnostics,
    );
    if has_blocking(&diagnostics) {
        return Ok(diagnostics);
    }
    let base_built = base_built.expect("build_model returns Some when no blocking diagnostics were added");
    let boundary = crate::prepare_boundary(base_built.nodes.len(), &base_built.supports);
    for finding in &boundary.findings {
        diagnostics.push(diag(
            &format!("diagnostic:support:{}", finding.support_id),
            "SUPPORT_INPUT_INVALID",
            "blocking",
            finding.message.clone(),
            vec![finding.support_id.clone()],
        ));
    }
    if has_blocking(&diagnostics) {
        return Ok(diagnostics);
    }
    // Each selected modulus basis's materials and built model, once per basis key,
    // kept as the ordinary solve keeps its basis states.
    let mut bases: Vec<(String, Vec<crate::MaterialInput>, crate::BuiltModel)> = Vec::new();
    for (index, case) in model.load_cases.iter().enumerate() {
        match &resolved {
            Some(cases) => {
                // Each resolved case after the first is built on its own member pairs.
                let state = &cases[index];
                let own_built;
                let built = if index == 0 {
                    &base_built
                } else {
                    own_built = crate::build_model_for_members(&model, &materials, Some(&state.pairs), &mut diagnostics);
                    if has_blocking(&diagnostics) {
                        return Ok(diagnostics);
                    }
                    own_built.as_ref().expect("build_model returns Some when no blocking diagnostics were added")
                };
                prepare_case(&model, built, &materials, &state.effective_case, Some(state), &boundary, &mut diagnostics);
            }
            None => {
                let position = match crate::modulus_basis_key(case, &mut diagnostics) {
                    None if has_blocking(&diagnostics) => return Ok(diagnostics),
                    None => None,
                    Some(key) => match bases.iter().position(|(known, _, _)| *known == key) {
                        Some(position) => Some(position),
                        None => {
                            let Some((basis_materials, _record)) =
                                crate::materials_for_modulus_basis(&model, &materials, case, &mut diagnostics)
                            else {
                                return Ok(diagnostics);
                            };
                            let basis_built = crate::build_model_for_basis(
                                &model,
                                &basis_materials,
                                None,
                                consumed_bases.selected(&key),
                                &mut diagnostics,
                            );
                            if has_blocking(&diagnostics) {
                                return Ok(diagnostics);
                            }
                            let basis_built = basis_built.expect("build_model returns Some when no blocking diagnostics were added");
                            bases.push((key, basis_materials, basis_built));
                            Some(bases.len() - 1)
                        }
                    },
                };
                let (case_materials, built) = match position {
                    Some(position) => (bases[position].1.as_slice(), &bases[position].2),
                    None => (materials.as_slice(), &base_built),
                };
                prepare_case(&model, built, case_materials, case, None, &boundary, &mut diagnostics);
            }
        }
        // The ordinary solve returns the blocked document after a case with any
        // blocking diagnostic.
        if has_blocking(&diagnostics) {
            return Ok(diagnostics);
        }
    }
    Ok(diagnostics)
}

/// One case's preparation, as `solve_load_case_observed` and
/// `solve_prepared_load_case` run it on a built model before the case's force
/// vector is finished, reduced or solved, with the ordinary early returns. `case`
/// is the case the solve applies (a resolved case's `effective_case`), and
/// `materials` its basis's. Every product here is dropped when the case ends.
fn prepare_case(
    model: &crate::PreviewModel,
    built: &crate::BuiltModel,
    materials: &[crate::MaterialInput],
    case: &crate::PreviewLoadCase,
    state: Option<&case_state::resolve::ResolvedCase>,
    boundary: &open_pipe_stress_linear_supports::BoundaryPreparation,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // The exact-route pressure case: its profile pass, region traversal,
    // references, topology, geometry, material coverage and formed loads. Its
    // result is not used here.
    drop(crate::pressure_runtime::build_pressure_case_with_members(
        model,
        built,
        materials,
        case,
        state.map(|state| &state.pairs),
        diagnostics,
    ));
    let loads = crate::build_load_case_primitive_loads(model, case, diagnostics);
    let application = crate::prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
    for finding in &application.findings {
        diagnostics.push(diag(
            &format!("diagnostic:load:{}", finding.load_id),
            "LOAD_INPUT_INVALID",
            "blocking",
            finding.message.clone(),
            vec![finding.load_id.clone(), case.id.clone()],
        ));
    }
    if has_blocking(diagnostics) {
        return;
    }
    // A case without a resolved state forms its thermal element loads from its
    // basis's materials (the selected basis's thermal expansion included).
    if state.is_none() {
        let pipe_map = model.pipe_segments.iter().enumerate().map(|(i, p)| (p.id.as_str(), i)).collect::<HashMap<_, _>>();
        let material_map = materials.iter().map(|m| (m.id.as_str(), m)).collect::<HashMap<_, _>>();
        drop(crate::build_thermal_element_loads(model, case, &material_map, &pipe_map, built, diagnostics));
    }
    // The uniform element loads' equivalents, into a scratch ledger that is
    // never finished: this is where a partial extent on a realized arc is refused.
    let curved_bends_by_pipe =
        built.curved_bend_elements.iter().map(|element| (element.pipe_index, element)).collect::<HashMap<_, _>>();
    let mut scratch = crate::LoadLedger::new();
    crate::add_uniform_element_loads(
        &mut scratch,
        model,
        &application.element_uniform_loads,
        &built.pipes,
        &curved_bends_by_pipe,
        &case.id,
        diagnostics,
    );
    drop(scratch);
    // The case's motion: moved spring bases and, without a resolved state, its
    // support-targeted primitives' prescribed map, with the ordinary return on
    // their refusals.
    let before = diagnostics.len();
    drop(match state {
        Some(state) => crate::load_state_spring_ground(state, &boundary.springs, &case.id, diagnostics),
        None => crate::imposed_motion::case_spring_ground(model, case, &boundary.springs, diagnostics),
    });
    let pf_motion = match state {
        Some(_) => None,
        None => crate::imposed_motion::case_prescribed(model, case, &boundary.restrained_dofs, diagnostics),
    };
    if has_blocking(&diagnostics[before..]) {
        return;
    }
    // The skew stops' prescribed-motion refusal, on the prescribed values the
    // ordinary solve forms: each restrained DOF's resolved or support-targeted
    // motion, or zero.
    if let Some(basis) = built.skew.as_ref() {
        let prescribed: Vec<(usize, f64)> = boundary
            .restrained_dofs
            .iter()
            .map(|&dof| {
                let motion = state.map(|state| &state.prescribed).or(pf_motion.as_ref());
                (dof, motion.and_then(|prescribed| prescribed.get(&dof).copied()).unwrap_or(0.0))
            })
            .collect();
        if let Some(refusal) = crate::skew::prescribed_motion_refusal(basis, &prescribed, &case.id) {
            diagnostics.push(refusal);
        }
    }
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

    /// The demo model reshaped as the product's own curved-bend span test model
    /// (`lib.rs` `curved_bend_span_request`): one pipe realized as a quarter arc of
    /// radius √2 m on a 2 m chord, anchored at its start, with a tip-force case and
    /// a second case of equivalent-static wind on the arc span exposed over
    /// `exposure` (`None`: the whole span).
    fn curved_span_request(exposure: Option<(f64, f64)>) -> Value {
        let mut request = request();
        let model = &mut request["model"];
        model["nodes"].as_array_mut().unwrap().truncate(2);
        model["nodes"][0]["id"] = json!("node:N-100");
        model["nodes"][0]["position"] = json!({"x": 0.0, "y": 0.0, "z": 0.0});
        model["nodes"][1]["id"] = json!("node:N-110");
        model["nodes"][1]["position"] = json!({"x": 2.0, "y": 0.0, "z": 0.0});
        model["pipe_segments"].as_array_mut().unwrap().truncate(1);
        let pipe = &mut model["pipe_segments"][0];
        pipe["id"] = json!("pipe:P-100");
        pipe["from"] = json!("node:N-100");
        pipe["to"] = json!("node:N-110");
        pipe["y_reference"] = json!({"x": 0.0, "y": 1.0, "z": 0.0});
        model["supports"].as_array_mut().unwrap().truncate(1);
        model["supports"][0]["id"] = json!("support:S-100");
        model["supports"][0]["node"] = json!("node:N-100");
        model["supports"][0]["restraints"] = json!(["UX", "UY", "UZ", "RX", "RY", "RZ"]);
        model["components"].as_array_mut().unwrap().truncate(1);
        let component = &mut model["components"][0];
        component["node"] = json!("node:N-110");
        component["geometry"]["bend_pipe_ref"] = json!("pipe:P-100");
        component["geometry"]["bend_radius"] = json!({"value": std::f64::consts::SQRT_2, "unit": "m"});
        component["geometry"]["bend_angle"] = json!({"value": std::f64::consts::PI / 2.0, "unit": "rad"});
        component["modifiers"]["sif_user_value"] = json!({"value": 1.15, "unit": "none"});
        component["modifiers"]["flexibility_factor_user_value"] = json!({"value": 2.0, "unit": "none"});
        component["mechanics_interface"]["solver_consumption"] = json!(crate::DEC_070_CURVED_BEND_SOLVER_CONSUMPTION);
        model["load_cases"].as_array_mut().unwrap().truncate(1);
        let mut tip = model["load_cases"][0].clone();
        tip["id"] = json!("load:L-100");
        tip["primitive_loads"] = json!([{"id": "load:L-100-Y", "category": "occasional",
            "target": {"type": "node", "node": "node:N-110"}, "direction": "global_y",
            "magnitude": {"value": 350.0, "unit": "N"}, "dimension": "force", "provenance": "invented_example_user_input"}]);
        let mut wind = tip.clone();
        wind["id"] = json!("load:L-200");
        wind["primitive_loads"] = json!([]);
        let (exposed_pipe_refs, exposed_spans) = match exposure {
            None => (json!(["pipe:P-100"]), json!([])),
            Some((start, end)) => (json!([]), json!([{"pipe_ref": "pipe:P-100",
                "start_fraction": {"value": start, "unit": "1"}, "end_fraction": {"value": end, "unit": "1"}}])),
        };
        wind["equivalent_static"] = json!({"wind": {"pressure": {"value": 480.0, "unit": "Pa"},
            "shape_factor": {"value": 0.7, "unit": "1"}, "direction": "global_y",
            "exposed_pipe_refs": exposed_pipe_refs, "exposed_spans": exposed_spans},
            "provenance": "invented_example_user_input"});
        model["load_cases"] = json!([tip, wind]);
        model["combinations"] = json!([]);
        request
    }

    #[test]
    fn a_partial_extent_on_a_realized_arc_in_the_second_case_is_refused_as_the_ordinary_solve_refuses_it() {
        // Control: the whole span exposed is admitted here and solved ordinarily.
        let whole = curved_span_request(None);
        assert_eq!(ordinary(&whole), Ok(Vec::new()));
        assert_eq!(blocking(&admit_source_before_assembly(whole, PreviewSolverMode::DenseScrutiny).unwrap()), Vec::new());
        // A partial extent: the ordinary solve refuses the whole document, with no result.
        let partial = curved_span_request(Some((0.2, 0.7)));
        let envelope = crate::run_linear_static_preview_value_with_mode(partial.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
        let expected = blocking(&envelope.diagnostics);
        assert!(envelope.results.is_empty(), "{expected:?}");
        assert!(matches!(expected.as_slice(), [(code, refs)] if code == "LOAD_INPUT_INVALID"
            && refs.iter().any(|r| r == "pipe:P-100") && refs.iter().any(|r| r == "load:L-200")), "{expected:?}");
        let admitted = admit_source_before_assembly(partial, PreviewSolverMode::DenseScrutiny).unwrap();
        assert_eq!(blocking(&admitted), expected);
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
