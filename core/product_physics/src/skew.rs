//! R-6/R-7 S-K1 product path (DEL-04-03 Work `skew-restraints.md` §2.2, §6 to §9, P-2).
//!
//! A skewed record (an explicit `line_stop` or `spring` on one translational DOF with a
//! `direction` of two or more nonzero components, admitted by `document_members`'
//! `skew_admission`) never becomes a `LinearSupport`: its DOF token states only that it is
//! translational (§6.4). `build_basis` forms the model's one `RestraintBasis` beside
//! `prepare_boundary` (codes 6, 7 and 8, per node, ascending), the basis's skew spring
//! blocks join the assembly (`node_blocks`), and the case solve runs on coordinates
//! (`solve`), is lifted, and is attributed (`attribute`). A model without a skewed record
//! builds no basis, and every path is today's (§6.5).
use super::*;
use crate::document_members::{
    is_skewed_record, skew_code, SUPPORT_CONSTRAINT_REDUNDANT, SUPPORT_DIRECTION_NODE_UNCONNECTED,
    SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED,
};
use open_pipe_stress_frame_kernel::restraint_basis::{BasisError, PublishedValue, RestraintBasis, SkewCode, StopReaction};
use open_pipe_stress_linear_supports::{
    prepare_skew_boundary, FindingCode, SkewFamily, SkewSupport,
};

/// The skewed records of a lowered model, in document order.
pub(crate) fn skew_supports(model: &PreviewModel) -> Vec<SkewSupport> {
    model
        .supports
        .iter()
        .filter(|support| is_skewed_record(support))
        .filter_map(|support| {
            let Some(document_members::DirectionInput::Vector(direction)) = support.direction else {
                return None;
            };
            let node_index = model.nodes.iter().position(|node| node.id == support.node)?;
            let (family, stiffness) = match support.family.as_deref() {
                Some("spring") => (
                    SkewFamily::Spring,
                    support_stiffness_input(support).and_then(|input| {
                        SupportQuantity::positive(input.value.value, QuantityDimension::TranslationalStiffness).ok()
                    }),
                ),
                _ => (SkewFamily::LineStop, None),
            };
            Some(SkewSupport { support_id: support.id.clone(), family, node_index, direction, stiffness })
        })
        .collect()
}

/// The model's basis (§3.2) beside `prepare_boundary`, or `None` without a skewed record.
/// Boundary preparation is per node, ascending, the first that applies of §2.2 items 6
/// (a node with no element), 7 and 8; every node is reported.
#[allow(clippy::too_many_arguments)]
pub(crate) fn build_basis(
    model: &PreviewModel,
    node_count: usize,
    supports: &[LinearSupport],
    frames: &[FrameElement],
    connectors: &[open_pipe_stress_frame_kernel::connector::ObjectiveConnector],
    curved: &[CurvedBendMacroBuild],
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<RestraintBasis> {
    let skew = skew_supports(model);
    if skew.is_empty() {
        return None;
    }
    let mut connected = vec![false; node_count];
    let mut mark = |node: usize| {
        if let Some(slot) = connected.get_mut(node) {
            *slot = true;
        }
    };
    for e in frames {
        mark(e.node_i.index);
        mark(e.node_j.index);
    }
    for c in connectors {
        mark(c.node_i().index);
        mark(c.node_j().index);
    }
    for c in curved {
        mark(c.node_i);
        mark(c.node_j);
    }
    let mut found: Vec<(usize, Diagnostic)> = Vec::new();
    let (unconnected, admitted): (Vec<_>, Vec<_>) =
        skew.into_iter().partition(|s| !connected.get(s.node_index).copied().unwrap_or(false));
    for support in &unconnected {
        found.push((support.node_index, diag(
            &format!("diagnostic:support:{}:direction-node-unconnected", stable_suffix(&support.support_id)),
            SUPPORT_DIRECTION_NODE_UNCONNECTED,
            "blocking",
            format!("support `{}` has a skewed direction at a node with no element; its node's translations are not in the stiffness, so nothing is solved", support.support_id),
            vec![support.support_id.clone()],
        )));
    }
    let boundary = prepare_boundary(node_count, supports);
    let basis = match prepare_skew_boundary(node_count, &boundary, &admitted) {
        Ok(basis) => Some(basis),
        Err(findings) => {
            for finding in findings {
                let node = admitted
                    .iter()
                    .find(|s| s.support_id == finding.support_id)
                    .map_or(usize::MAX, |s| s.node_index);
                let code = match finding.code {
                    FindingCode::SkewStopsPerNode => SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED,
                    FindingCode::ConstraintRedundant => SUPPORT_CONSTRAINT_REDUNDANT,
                    FindingCode::SkewDirectionRange => skew_code(SkewCode::RangeUnsupported),
                    _ => "SUPPORT_INPUT_INVALID",
                };
                found.push((node, diag(
                    &format!("diagnostic:support:{}:{}", stable_suffix(&finding.support_id), code.to_ascii_lowercase().replace('_', "-")),
                    code,
                    "blocking",
                    format!("support `{}`: {}", finding.support_id, finding.message),
                    vec![finding.support_id.clone()],
                )));
            }
            None
        }
    };
    found.sort_by_key(|(node, _)| *node);
    let refused = !found.is_empty();
    diagnostics.extend(found.into_iter().map(|(_, d)| d));
    if refused {
        return None;
    }
    basis
}

/// The skew spring blocks for the assembly (§6.1), in `springs()` order; empty (no
/// allocation) without a basis.
pub(crate) fn node_blocks(basis: Option<&RestraintBasis>) -> Vec<(usize, [[f64; 3]; 3])> {
    match basis {
        None => Vec::new(),
        Some(basis) => (0..basis.springs().len()).map(|s| (basis.springs()[s].node, basis.spring_block(s))).collect(),
    }
}

/// §9: the skewed rows the ground count adds (stops and positive skewed springs).
pub(crate) fn ground_rows(basis: Option<&RestraintBasis>) -> usize {
    basis.map_or(0, |basis| basis.stops().len() + basis.springs().iter().filter(|s| s.stiffness > 0.0).count())
}

/// The physical DOFs that move in a skew case: every DOF that is not an axis restraint
/// (the pivots included), ascending.
pub(crate) fn physical_free(basis: &RestraintBasis) -> Vec<usize> {
    (0..basis.dimension()).filter(|dof| basis.axis_restrained().binary_search(dof).is_err()).collect()
}

/// §2.2 item 9 before reduction: a nonzero prescribed translation at a stop's node.
pub(crate) fn prescribed_motion_refusal(basis: &RestraintBasis, prescribed: &[(usize, f64)], case_id: &str) -> Option<Diagnostic> {
    let dofs = basis.judge_prescribed(prescribed);
    (!dofs.is_empty()).then(|| diag(
        &format!("diagnostic:load-case:{}:skew-node-prescribed-motion", stable_suffix(case_id)),
        skew_code(SkewCode::NodePrescribedMotion),
        "blocking",
        format!("load case `{case_id}` prescribes a nonzero translation on an axis restraint at a skewed stop's node (global DOF {dofs:?}); the affine reduction is not built yet, so nothing is solved"),
        vec![case_id.to_string()],
    ))
}

/// §2.2 item 10: a skew case whose ordinary formation left binary64 (W2 not attempted).
pub(crate) fn force_scaled_refusal(case_id: &str, detail: &str) -> Diagnostic {
    diag(
        &format!("diagnostic:load-case:{}:skew-force-scaled", stable_suffix(case_id)),
        skew_code(SkewCode::ForceScaledUnsupported),
        "blocking",
        format!("load case `{case_id}`: a skew model's ordinary formation left the binary64 range ({detail}); force scaling with skewed restraints is not built yet, so nothing is solved"),
        vec![case_id.to_string()],
    )
}

/// The static path's coordinate solve (§6.2, §6.3): NI's linear skew entry with its
/// assembly and symmetry evidence on K′ and `restrained_coordinates`. The case solve
/// (`solve`) and modal's singularity evidence (D-1 C1, on a zero force in
/// `DenseScrutiny`) both run it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn solve_coordinates(
    mode: LinearSolveMode,
    stiffness: &SparseStiffness,
    built: &BuiltModel,
    curved: &[CurvedBendStiffnessElement],
    springs: &[(usize, f64)],
    global_force: &AssembledForce,
    prescribed: &[(usize, f64)],
    basis: &RestraintBasis,
) -> Result<open_pipe_stress_frame_kernel::structural::StructuralSolution, StructuralError> {
    let assembly = SparseAssemblyEvidence::new_with_restraint_basis(
        stiffness.pattern(),
        built.nodes.len(),
        &built.frame_elements,
        &built.connectors,
        curved,
        springs,
        basis,
    )?
    .with_frame_members(&frame_member_identities(built));
    let curved_sources = built.curved_bend_elements.iter().map(|e| e.macro_element).collect::<Vec<_>>();
    assembly.solve_skew_with_formation_check(stiffness, global_force, prescribed, mode, &curved_sources)
}

/// The case solve on coordinates (§6.2, §6.3), through NI's linear skew entry, lifted
/// (§10.4): the physical free values (`physical_free` order) and the physical member bound.
#[allow(clippy::too_many_arguments)]
pub(crate) fn solve(
    solver_mode: PreviewSolverMode,
    stiffness: &SparseStiffness,
    built: &BuiltModel,
    spring_entries: &[SpringEntry],
    global_force: &AssembledForce,
    prescribed: &[(usize, f64)],
    basis: &RestraintBasis,
) -> Result<PreviewLinearSolve, StructuralError> {
    let curved = built
        .curved_bend_elements
        .iter()
        .map(|e| CurvedBendStiffnessElement::from_macro_element(e.component_id.clone(), &e.macro_element))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StructuralError::InvalidInput("curved formation evidence"))?;
    let springs = spring_entries
        .iter()
        .map(|e| (e.node_dof.global_index(), e.stiffness.value))
        .collect::<Vec<_>>();
    let mode = match solver_mode {
        PreviewSolverMode::DenseScrutiny => LinearSolveMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive => LinearSolveMode::SparseInteractive,
    };
    let checked = solve_coordinates(mode, stiffness, built, &curved, &springs, global_force, prescribed, basis)?;
    let lift_error = |_| StructuralError::InvalidInput("restraint basis lift");
    let (u, member_bound) = match &checked.member_bound {
        Some(bound) => {
            let lifted = basis.lift_member_bound(&checked.displacements, bound).map_err(lift_error)?;
            (lifted.u, Some(lifted.bound))
        }
        None => (basis.lift(&checked.displacements).map_err(lift_error)?, None),
    };
    Ok(PreviewLinearSolve {
        solution: physical_free(basis).iter().map(|&i| u[i]).collect(),
        structural_report: checked.report,
        load_fidelity: checked.load_fidelity,
        formation_check: checked.formation_check,
        member_bound,
        solution_basis: match solver_mode {
            PreviewSolverMode::DenseScrutiny => "dense_structural_integrity_primary",
            PreviewSolverMode::SparseInteractive => "sparse_structural_integrity_primary",
        },
        dense_fallback_message: None,
    })
}

/// A case's attributed skew reactions (§7.2).
pub(crate) struct SkewReactions {
    /// Global support-on-pipe force and along-direction value, by skewed record id.
    pub records: HashMap<String, StopReaction>,
}

/// §7.2 from the physical rows r = K u − f (`reactions`, each one exact sum rounded
/// once): each constrained node's stop and axis records by `attribute`, each skewed
/// spring by `spring_reaction`. The axis records' raw rows at a constrained node are
/// replaced by their attributed components. An out-of-range published value is §2.2
/// item 11, named.
pub(crate) fn attribute(
    basis: &RestraintBasis,
    displacements: &[f64],
    reactions: &mut [f64],
    case_id: &str,
) -> Result<SkewReactions, Diagnostic> {
    let range = |support: &str, value: PublishedValue| diag(
        &format!("diagnostic:load-case:{}:{}:skew-reaction-range", stable_suffix(case_id), stable_suffix(support)),
        skew_code(SkewCode::ReactionRange),
        "blocking",
        format!("load case `{case_id}`: the published {value:?} of support `{support}` is outside the binary64 range; it is not published, and nothing of the case is published"),
        vec![case_id.to_string(), support.to_string()],
    );
    let internal = |support: &str, error: &BasisError| diag(
        &format!("diagnostic:load-case:{}:{}:skew-attribution", stable_suffix(case_id), stable_suffix(support)),
        "SOLVER_SYSTEM_BLOCKED",
        "blocking",
        format!("load case `{case_id}`: the attribution of support `{support}` failed ({error:?}); nothing of the case is published"),
        vec![case_id.to_string(), support.to_string()],
    );
    let mut records = HashMap::new();
    for node in basis.nodes() {
        let base = node.node * DOF_PER_NODE;
        let r = [reactions[base], reactions[base + 1], reactions[base + 2]];
        let stop = &basis.stops()[node.stop];
        let attributed = basis.attribute(node.node, r).map_err(|error| match error {
            BasisError::ReactionRange { value, .. } => range(&stop.support_id, value),
            other => internal(&stop.support_id, &other),
        })?;
        for (dof, value) in attributed.axis {
            reactions[dof] = value;
        }
        records.insert(stop.support_id.clone(), attributed.stop);
    }
    for (s, spring) in basis.springs().iter().enumerate() {
        let reaction = basis.spring_reaction(s, displacements).map_err(|error| match error {
            BasisError::ReactionRange { value, .. } => range(&spring.support_id, value),
            other => internal(&spring.support_id, &other),
        })?;
        records.insert(spring.support_id.clone(), reaction);
    }
    Ok(SkewReactions { records })
}

/// §7.1: a skewed record's along-direction row (N, positive along the authored +d).
pub(crate) fn along_direction_row(case_id: &str, support_id: &str, value: f64, basis_text: &str) -> ResultItem {
    ResultItem {
        component_body: None,
        id: format!("result:support-along-direction-v1:{}", exact_source_identity(&[case_id, support_id])),
        kind: SUPPORT_ALONG_DIRECTION_KIND.into(),
        value,
        unit: "N".into(),
        entity_ref: support_id.to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            basis: basis_text.into(),
            component: "along_direction".into(),
            coordinate_system: "support_direction".into(),
            location: "node".into(),
            sign_convention: "support_on_pipe_positive_along_authored_direction".into(),
        }),
    }
}

/// §7.1's row kind.
pub(crate) const SUPPORT_ALONG_DIRECTION_KIND: &str = "support_reaction_along_direction_v1";

/// Whether a row kind's id already names its load case, so that a later case's row is
/// published under it unqualified and is combined by its support group, never by base id:
/// the `_v2` kinds and §7.1's along-direction kind, whose agreed id
/// (`result:support-along-direction-v1:` + the exact source identity of case and record)
/// names the case although the kind ends in `_v1`. Before this predicate the along row
/// of a second case was qualified (`result:loadcase:…`), and no combination over two
/// cases published one (its case-named base id matches in no other case).
pub(crate) fn case_named_id(kind: &str) -> bool {
    kind.ends_with("_v2") || kind == SUPPORT_ALONG_DIRECTION_KIND
}

/// A case refused by a skew code before anything is published.
pub(crate) fn blocked_case(case_id: &str) -> LoadCaseSolve {
    LoadCaseSolve {
        load_state_evidence: None,
        exact_case_evidence: None,
        pressure_evidence: Vec::new(),
        source_case: None,
        source_selected: false,
        load_case_id: case_id.to_string(),
        results: Vec::new(),
        max_displacement: None,
        max_stress: None,
        max_stress_reduced: None,
        component_stress_modifier_count: 0,
        preview: None,
        withheld_summaries: Vec::new(),
        held_status_source: None,
        nonlinear_state: None,
    }
}

#[cfg(test)]
#[path = "skew_tests.rs"]
mod tests;
