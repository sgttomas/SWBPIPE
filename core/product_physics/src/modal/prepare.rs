//! The modal entry's preparation: the static entry's pre-formation sequence
//! (`run_linear_static_preview_observed_inner`), calling the same crate
//! functions in the same order and stopping where it stops, as
//! `validation/model_checks.rs` does; then the state case's binding, the
//! modal interim refusals, the build and the restraint pre-checks. It forms
//! no stiffness and no mass. The static entry is not changed.
//!
//! The operating-condition lowering runs once. The binding's evidence is taken
//! on the model the lowering returned, before unit normalization rewrites any
//! magnitude (`normalize_model_units` converts every primitive load), so the
//! authored prefix compares bit for bit; its verdict is applied after the
//! static admission of the state case, where the sequence names it, and the
//! case's place is checked again on the normalized model.
use super::context::{BoundCase, ModalCaseContext};
use super::request::{ModalPreviewRequest, RequestInputs};
use super::{MODAL_NONLINEAR_SUPPORT_INTERIM_UNSUPPORTED, MODAL_STATE_CASE_UNAVAILABLE, MODAL_TIMOSHENKO_MASS_UNSUPPORTED};
use crate::*;
use open_pipe_stress_linear_supports::BoundaryPreparation;

/// Proof that a [`Prepared`] came from [`prepare_modal`]: it has no constructor
/// outside this module, so nothing that takes a `Prepared` (mass assembly, the
/// weight-basis inspection) is reachable before preparation has passed.
#[derive(Debug)]
pub(crate) struct Admitted(());

/// A request that passed every check the static solve makes before it forms
/// a stiffness, the state case's binding and the modal interim refusals.
pub(crate) struct Prepared<'a> {
    /// The request's authored model, unchanged.
    pub(crate) original: &'a PreviewModel,
    pub(crate) inputs: RequestInputs,
    /// The lowered, validated and normalized model.
    pub(crate) model: PreviewModel,
    pub(crate) materials: Vec<MaterialInput>,
    pub(crate) built: BuiltModel,
    pub(crate) boundary: BoundaryPreparation,
    pub(crate) bound_case: Option<BoundCase>,
    /// Whether the model is on the exact route (its mass basis is then
    /// `EXACT_MASS_METHOD`).
    pub(crate) exact: bool,
    /// The non-blocking diagnostics the sequence published.
    pub(crate) diagnostics: Vec<Diagnostic>,
    _admitted: Admitted,
}

fn modal_stop(mut diagnostics: Vec<Diagnostic>, state_case: Option<&str>, from: usize) -> Vec<Diagnostic> {
    // A static refusal of the state case refuses the modal request by name,
    // carrying the static diagnostics that refused it.
    if let Some(case) = state_case {
        let carried = diagnostics[from..]
            .iter()
            .filter(|d| d.severity == "blocking" && d.affected_refs.iter().any(|r| r == case))
            .count();
        if carried > 0 {
            diagnostics.push(state_case_unavailable(case, carried));
        }
    }
    diagnostics
}

/// The modal refusal of a state case the static path refused. The static
/// diagnostics themselves are returned with it, unchanged; this message counts
/// them rather than repeating their texts (DEL-04-08 §8.43: a repeated static
/// message has no source-bound TEXT size).
fn state_case_unavailable(case: &str, carried: usize) -> Diagnostic {
    diag(
        &format!("diagnostic:modal:state-case:{}:static-refusal", stable_suffix(case)),
        MODAL_STATE_CASE_UNAVAILABLE,
        "blocking",
        format!("state case `{case}` is refused by the static path's own admission ({carried} blocking diagnostic(s), returned with this one), so it defines no modal model"),
        vec!["state_case".to_string(), case.to_string()],
    )
}

/// The modal refusal of a state case whose binding cannot be applied, for a
/// fixed reason.
fn state_case_unbound(case: &str, reason: &'static str) -> Diagnostic {
    diag(
        &format!("diagnostic:modal:state-case:{}:unbound", stable_suffix(case)),
        MODAL_STATE_CASE_UNAVAILABLE,
        "blocking",
        format!("state case `{case}` defines no modal model: {reason}"),
        vec!["state_case".to_string(), case.to_string()],
    )
}

/// The id of [`request_basis_case`]'s internal case. It never leaves this
/// module: [`attribute_request_basis`] renames it in every diagnostic the static
/// resolver emits for it.
const REQUEST_BASIS_CASE: &str = "modal:request-modulus-basis";

/// Attributes the static resolver's diagnostics about the internal request
/// case to the modal request itself, so the internal id reaches no output.
fn attribute_request_basis(found: &mut [Diagnostic]) {
    let suffix = stable_suffix(REQUEST_BASIS_CASE);
    for finding in found {
        finding.id = finding.id.replace(REQUEST_BASIS_CASE, "modal-request").replace(&suffix, "modal-request");
        finding.message = finding.message.replace(REQUEST_BASIS_CASE, "the modal request");
        for reference in &mut finding.affected_refs {
            if reference == REQUEST_BASIS_CASE {
                *reference = "modal_request".to_string();
            }
        }
    }
}

/// A load case carrying only the request's modulus basis, so that the static
/// resolver (`materials_for_modulus_basis`) is reused unchanged on a plain
/// document; it is never solved, stored or published.
fn request_basis_case(req: &ModalPreviewRequest, diagnostics: &mut Vec<Diagnostic>) -> Option<PreviewLoadCase> {
    if req.modulus_basis_ref.is_none() && req.modulus_basis_temperature.is_none() {
        return None;
    }
    let mut case: PreviewLoadCase = serde_json::from_value(serde_json::json!({"id": REQUEST_BASIS_CASE}))
        .expect("a case with only an id decodes");
    case.modulus_basis_ref = req.modulus_basis_ref.clone();
    case.modulus_basis_temperature = req.modulus_basis_temperature.clone();
    if let Some(temperature) = &mut case.modulus_basis_temperature {
        // As `normalize_model_units` normalizes a case's solve temperature.
        normalize_quantity(
            temperature,
            Dimension::Temperature,
            "diagnostic:unit-conversion:modal-request:modulus-basis-temperature",
            vec!["modulus_basis_temperature".to_string()],
            diagnostics,
        );
    }
    Some(case)
}

/// The static entry's pre-formation sequence on the request (module docs).
pub(crate) fn prepare_modal(req: &ModalPreviewRequest) -> Result<Prepared<'_>, Vec<Diagnostic>> {
    let inputs = req.validate_modal_request()?;
    let state_case = inputs.state_case.as_deref();
    let context = match state_case {
        Some(case) => Some(ModalCaseContext::capture_state_case(&req.model, case).map_err(|d| vec![d])?),
        None => None,
    };
    let mut model = req.model.clone();
    let request_materials_supplied = !req.materials.is_empty();
    let mut materials = if req.materials.is_empty() { model.materials.clone() } else { req.materials.clone() };
    let mut diagnostics = Vec::new();

    operating_conditions::lower(&mut model, &mut diagnostics);
    // The binding's evidence, on the lowered model before any stage rewrites a
    // magnitude (module docs). A refused lowering stops below with its own
    // diagnostics, so no evidence is taken from it.
    let binding = match &context {
        Some(context) if !has_blocking(&diagnostics) => Some(context.bind_state_case(&model)),
        _ => None,
    };
    pressure_runtime::validate_profile(&model, &mut diagnostics);
    pressure_runtime::junction_closure_warnings(&model, &mut diagnostics);
    case_state::resolve::validate_document(&model, request_materials_supplied, &mut diagnostics);
    if imposed_motion::validate(&model, &mut diagnostics) {
        return Err(modal_stop(diagnostics, state_case, 0));
    }
    validate_model_inputs(&model, &materials, &mut diagnostics);
    validate_support_family_tokens(&model, &mut diagnostics);
    pf_timoshenko::append_partial_span_refusals(&model, &mut diagnostics);
    if model.document_kind != "openpipestress.product_preview.model" {
        diagnostics.push(diag(
            "diagnostic:physics:document-kind",
            "PREVIEW_DOCUMENT_KIND_INVALID",
            "blocking",
            "physics adapter requires an openpipestress.product_preview.model document",
            vec!["model".to_string()],
        ));
    }
    if model.schema_version.is_empty() {
        diagnostics.push(diag(
            "diagnostic:physics:schema-version",
            "PREVIEW_SCHEMA_VERSION_MISSING",
            "blocking",
            "preview model requires an explicit schema version",
            vec!["model".to_string()],
        ));
    }
    if has_blocking(&diagnostics) {
        return Err(modal_stop(diagnostics, state_case, 0));
    }
    document_members::withhold_interim_combinations(&mut model, &mut diagnostics);
    self_weight::validate_applied_self_weight(&model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Err(modal_stop(diagnostics, state_case, 0));
    }
    resolve_shared_sections(&mut model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Err(modal_stop(diagnostics, state_case, 0));
    }
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Err(modal_stop(diagnostics, state_case, 0));
    }
    rigid_component::append_eccentric_offset_refusals(&model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Err(modal_stop(diagnostics, state_case, 0));
    }
    let load_state = case_state::is_load_state(&model);
    if !load_state {
        pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    }
    if has_blocking(&diagnostics) {
        return Err(modal_stop(diagnostics, state_case, 0));
    }
    let resolution_start = diagnostics.len();
    let resolved_cases = if load_state {
        let resolved = model
            .load_cases
            .iter()
            .filter_map(|case| case_state::resolve::resolve_case(&model, &materials, case, &mut diagnostics))
            .collect::<Vec<_>>();
        if has_blocking(&diagnostics) || resolved.len() != model.load_cases.len() {
            return Err(stop_resolution(diagnostics, state_case, resolution_start));
        }
        case_state::resolve::refuse_resolved_span_strain(&model, &resolved, &mut diagnostics);
        case_state::resolve::refuse_all_reference_states(&resolved, &mut diagnostics);
        if has_blocking(&diagnostics) {
            return Err(stop_resolution(diagnostics, state_case, resolution_start));
        }
        Some(resolved)
    } else {
        None
    };

    // The state case's binding, applied here (module docs).
    let bound_case = match (state_case, binding) {
        (None, _) => None,
        (Some(case), None) => {
            diagnostics.push(state_case_unbound(case, "no binding evidence was taken from the lowered model"));
            return Err(diagnostics);
        }
        (Some(_), Some(Err(refusal))) => {
            diagnostics.push(refusal);
            return Err(diagnostics);
        }
        (Some(case), Some(Ok(bound))) => {
            // No stage since the binding reorders, adds or removes a case or a
            // primitive load.
            let kept = model.load_cases.get(bound.case_index).is_some_and(|lowered| {
                lowered.id == bound.case_id && lowered.primitive_loads.len() == bound.generated_added_weights.end
            });
            if !kept {
                diagnostics.push(state_case_unbound(case, "the case's place changed after it was bound"));
                return Err(diagnostics);
            }
            Some(bound)
        }
    };

    // The modal interim refusals, each by name.
    if pf_timoshenko::selects(&model) || pressure_runtime::selects_timoshenko(&model) {
        diagnostics.push(diag(
            "diagnostic:modal:timoshenko-mass",
            MODAL_TIMOSHENKO_MASS_UNSUPPORTED,
            "blocking",
            "the model selects Timoshenko straight members (timoshenko_annulus_energy_v1); modal analysis forms an Euler-Bernoulli consistent mass, and pairing it with shear-flexible stiffness would be inconsistent, so nothing is formed (a Timoshenko consistent mass is a later slice, M4)",
            vec!["shear_model".to_string()],
        ));
        return Err(diagnostics);
    }
    let nonlinear: Vec<String> =
        model.supports.iter().filter(|support| support.nonlinear.is_some()).map(|support| support.id.clone()).collect();
    if !nonlinear.is_empty() {
        let mut refs = nonlinear.clone();
        refs.extend(state_case.map(str::to_string));
        diagnostics.push(diag(
            "diagnostic:modal:nonlinear-support-interim",
            MODAL_NONLINEAR_SUPPORT_INTERIM_UNSUPPORTED,
            "blocking",
            format!("the model has nonlinear supports ({}); their linearization at the named state case's same-run solved state is not built yet, so nothing is formed", nonlinear.join(", ")),
            refs,
        ));
        return Err(diagnostics);
    }

    // The materials and member pairs the build takes: the state case's
    // resolved pairs on a load/reference-state document; otherwise the named
    // case's modulus basis, or the request's, through the static resolver.
    let basis_case = match (&bound_case, &resolved_cases) {
        (_, Some(_)) => None,
        (Some(bound), None) => Some(model.load_cases[bound.case_index].clone()),
        (None, None) => request_basis_case(req, &mut diagnostics),
    };
    if has_blocking(&diagnostics) {
        return Err(diagnostics);
    }
    if let Some(case) = &basis_case {
        let before = diagnostics.len();
        let resolved = if modulus_basis_key(case, &mut diagnostics).is_some() {
            materials_for_modulus_basis(&model, &materials, case, &mut diagnostics).map(|(basis, _record)| basis)
        } else {
            Some(materials.clone())
        };
        if bound_case.is_none() {
            attribute_request_basis(&mut diagnostics[before..]);
        }
        match resolved {
            Some(basis_materials) if !has_blocking(&diagnostics) => materials = basis_materials,
            _ => return Err(modal_stop(diagnostics, state_case, before)),
        }
    }
    let pairs = match (&resolved_cases, &bound_case) {
        (Some(resolved), Some(bound)) => Some(&resolved[bound.case_index].pairs),
        _ => None,
    };
    let built = build_model_for_members(&model, &materials, pairs, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return Err(diagnostics);
    }
    let Some(built) = built else {
        return Err(diagnostics);
    };
    append_constant_effort_consumption_diagnostics(&model, &mut diagnostics);
    topology::append_topology_diagnostics(&model, &built, &mut diagnostics);

    // The restraint pre-checks, as the static entry states them.
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let potential_contact_dofs = eligible_contact_dofs(
        built.nodes.len(),
        &boundary.restrained_dofs,
        &built.nonlinear_supports,
        &built.nonlinear_initial_states,
    );
    if boundary.restrained_dofs.is_empty() && boundary.springs.is_empty() && potential_contact_dofs.is_none() {
        diagnostics.push(diag(
            "diagnostic:physics:no-restraints",
            "SUPPORT_INPUT_MISSING",
            "blocking",
            "linear-static preview requires explicit support restraints or springs; no automatic boundary conditions are applied",
            vec!["supports".to_string()],
        ));
    }
    let ground_dofs = boundary
        .restrained_dofs
        .iter()
        .copied()
        .chain(
            boundary
                .springs
                .iter()
                .filter(|spring| spring.stiffness.value > 0.0)
                .map(|spring| spring.node_dof.global_index()),
        )
        .chain(potential_contact_dofs.iter().flatten().copied())
        .collect::<HashSet<_>>();
    if !ground_dofs.is_empty() && ground_dofs.len() < DOF_PER_NODE {
        let (restrained, missing) = support_restraint_summary(&boundary.restrained_dofs);
        diagnostics.push(diag("diagnostic:physics:under-restrained", "SOLVER_SYSTEM_BLOCKED", "blocking",
            format!("fewer than six independent ground constraints including positive springs: the six rigid-body modes of a connected structure cannot all be removed; directly restrained global DOF classes: {restrained}; global DOF classes with no direct restraint: {missing} (not a rigid-body mode analysis; separated restraints can resist rotations); support contributions: {}", support_contribution_summary(&model)),
            vec!["supports".to_string()]));
    }
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
        return Err(diagnostics);
    }
    let exact = pressure_runtime::is_exact(&model);
    Ok(Prepared {
        original: &req.model,
        inputs,
        model,
        materials,
        built,
        boundary,
        bound_case,
        exact,
        diagnostics,
        _admitted: Admitted(()),
    })
}

/// A refusal in the load-state resolution or its two guards: the static path
/// refuses every case there, the state case included, so every blocking
/// diagnostic of the step is carried.
fn stop_resolution(mut diagnostics: Vec<Diagnostic>, state_case: Option<&str>, from: usize) -> Vec<Diagnostic> {
    if let Some(case) = state_case {
        let carried = diagnostics[from..].iter().filter(|d| d.severity == "blocking").count();
        if carried > 0 {
            diagnostics.push(state_case_unavailable(case, carried));
        }
    }
    diagnostics
}

#[cfg(test)]
pub(crate) mod tests_support {
    use super::*;

    /// Test only: an admitted preparation whose lowered model is replaced, to
    /// corrupt a formed-mass input while the authored model (and so its
    /// self-weight basis) stays unchanged.
    pub(crate) fn with_model(prepared: Prepared<'_>, model: PreviewModel) -> Prepared<'_> {
        Prepared { model, ..prepared }
    }
}
