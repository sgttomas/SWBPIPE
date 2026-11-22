//! On-demand model checks (DEL-04-06 Design `on-demand-model-checks.md`): the
//! solve's own input checks, run without forming a stiffness or solving.
//!
//! [`LinearStaticPreviewRequest::pre_formation_checks_value`] takes the request the desktop's solve takes
//! (`{"model": …, "materials": []}`) and runs, in the solve's order and with the
//! solve's stops, the checks that `run_linear_static_preview_observed_inner`
//! runs before it forms the first stiffness:
//! - the request's closed-record and parse checks;
//! - input validation (profile, load-state document, imposed displacements,
//!   model inputs, support families, PF Timoshenko partial-span wind refusals,
//!   document kind and schema version);
//! - applied self weight, shared sections, unit normalization, seismic
//!   model-check warnings, eccentric-offset refusals and the base material pair;
//! - the per-case load-state resolution;
//! - the model build (element formation inputs);
//! - constant-effort consumption, the W-8 topology checks
//!   (`topology::append_topology_diagnostics`);
//! - the restraint pre-checks (`check_ground_restraints` and invalid support
//!   input), run on the document's system when some case is solved on it and on
//!   the replaced system (`hydrostatic::all_cases_held` /
//!   `hydrostatic::held_model`) when every case holds supports.
//!
//! It adds no rule and changes no number. Each check is the function the solve
//! calls, with the same arguments; the only copies are the diagnostics the
//! solve builds inline (document kind, schema version and
//! `SUPPORT_INPUT_INVALID` from `boundary.findings`). Where the solve
//! stops at a blocking diagnostic, this stops too, so every finding listed is
//! one the solve itself would publish before it forms a stiffness.
//! `tests/model_checks_runtime.rs` holds the two in step: the findings equal the
//! leading diagnostics of the solve's envelope for each model in its corpus.
//! The retained-memory observer and the dense-scrutiny size guard are solve
//! work, not input checks, and do not run here. Nothing here writes to the
//! model; the caller's request is consumed.
use crate::*;

impl LinearStaticPreviewRequest {
    /// The solve's pre-formation diagnostics for `actual_request` (the
    /// request the desktop's solve takes), or the solve's own refusal text when
    /// the request cannot be read (a model record that is not an object or has
    /// an undeclared key, or a request that does not parse).
    pub fn pre_formation_checks_value(actual_request: serde_json::Value) -> Result<Vec<Diagnostic>, String> {
        if let Some(refusal) = model_fields::unknown_field_refusal(&actual_request) {
            return Err(refusal);
        }
        // The solve's parse; no check before stiffness formation reads the solver mode.
        let (request, _capture) =
            source_receipt::CapturedInvocation::parse(actual_request, PreviewSolverMode::default())
                .map_err(|error| error.0)?;
        Ok(pre_formation_checks(request))
    }
}

fn pre_formation_checks(request: LinearStaticPreviewRequest) -> Vec<Diagnostic> {
    let mut model = request.model;
    let request_materials_supplied = !request.materials.is_empty();
    let mut materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = Vec::new();

    // Model document 0.9.0: the solve's lowering, first, as in the solve.
    model.declared_test_contents = crate::hydrostatic::declared_contents(&model);
    let condition_added = crate::operating_conditions::lower(&mut model, &mut diagnostics);
    pressure_runtime::validate_profile(&model, &mut diagnostics);
    case_state::resolve::validate_document(&model, request_materials_supplied, &mut diagnostics);
    if imposed_motion::validate(&model, &mut diagnostics) {
        return diagnostics;
    }
    validate_model_inputs(&model, &materials, &mut diagnostics);
    validate_support_family_tokens(&model, &mut diagnostics);
    crate::pf_timoshenko::append_partial_span_refusals(&model, &mut diagnostics);
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
        return diagnostics;
    }
    crate::document_members::withhold_interim_combinations(&mut model, &mut diagnostics);

    self_weight::validate_applied_self_weight(&model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return diagnostics;
    }
    // DEL-05-01 REQ-008, as in the solve.
    let seismic_entered = crate::seismic_warning::capture(&model, &condition_added);
    let test_contents = std::mem::take(&mut model.test_contents);
    crate::hydrostatic::apply_contents(&mut model, &test_contents);
    resolve_shared_sections(&mut model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return diagnostics;
    }
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return diagnostics;
    }
    let load_state = case_state::is_load_state(&model);
    if !load_state {
        crate::seismic_warning::append(&model, &materials, None, &seismic_entered, &mut diagnostics);
    }
    crate::rigid_component::append_eccentric_offset_refusals(&model, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return diagnostics;
    }
    if !load_state {
        pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    }
    if has_blocking(&diagnostics) {
        return diagnostics;
    }
    let resolved_cases = if load_state {
        let resolved = model
            .load_cases
            .iter()
            .filter_map(|case| case_state::resolve::resolve_case(&model, &materials, case, &mut diagnostics))
            .collect::<Vec<_>>();
        if has_blocking(&diagnostics) || resolved.len() != model.load_cases.len() {
            return diagnostics;
        }
        case_state::resolve::refuse_resolved_span_strain(&model, &resolved, &mut diagnostics);
        case_state::resolve::refuse_all_reference_states(&resolved, &mut diagnostics);
        if has_blocking(&diagnostics) {
            return diagnostics;
        }
        crate::seismic_warning::append(&model, &materials, Some(&resolved), &seismic_entered, &mut diagnostics);
        Some(resolved)
    } else {
        None
    };
    let built = build_model_for_members(
        &model,
        &materials,
        resolved_cases.as_ref().map(|cases| &cases[0].pairs),
        &mut diagnostics,
    );
    if has_blocking(&diagnostics) {
        return diagnostics;
    }
    let Some(built) = built else {
        return diagnostics;
    };
    append_constant_effort_consumption_diagnostics(&model, &mut diagnostics);
    topology::append_topology_diagnostics(&model, &built, &mut diagnostics);

    // The restraint pre-checks, sharing the solve's own decision: the document's
    // system when some case is solved on it, and otherwise the replaced system a
    // held test case is solved on (DEL-04-07 REQ-011, DEL-04-06 REQ-013).
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let held_all = crate::hydrostatic::all_cases_held(&model);
    if held_all.is_none() {
        check_ground_restraints(&model, &built, &boundary, &mut diagnostics);
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
    if let Some(held) = held_all.filter(|_| !has_blocking(&diagnostics)) {
        let replaced = crate::hydrostatic::held_model(&model, &held);
        let replaced_built = build_model_for_members(
            &replaced,
            &materials,
            resolved_cases.as_ref().map(|cases| &cases[0].pairs),
            &mut diagnostics,
        );
        if has_blocking(&diagnostics) {
            return diagnostics;
        }
        let Some(replaced_built) = replaced_built else {
            return diagnostics;
        };
        let replaced_boundary = prepare_boundary(replaced_built.nodes.len(), &replaced_built.supports);
        if !replaced_boundary.findings.is_empty() {
            for finding in &replaced_boundary.findings {
                diagnostics.push(diag(
                    &format!("diagnostic:support:{}", finding.support_id),
                    "SUPPORT_INPUT_INVALID",
                    "blocking",
                    finding.message.clone(),
                    vec![finding.support_id.clone()],
                ));
            }
            return diagnostics;
        }
        check_ground_restraints(&replaced, &replaced_built, &replaced_boundary, &mut diagnostics);
    }
    diagnostics
}
