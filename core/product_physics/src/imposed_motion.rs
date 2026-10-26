//! Imposed displacement on the pressure-free route, stage 1 (W-16 slice a;
//! DEL-04-03 Design `pf-imposed-displacement.md`; owner, 2026-10-23, "Row 2 I
//! approve option A").
//!
//! A primitive load with `category: imposed_displacement` and target
//! `{type: "support", support, dof}` becomes one prescribed value g on the
//! global DOF of its support's node. The case's solve then runs the
//! partitioned reduction the exact route already uses for
//! `boundary_motion`: K_ff u_f = f_f - K_fc g_c, u_c = g_c, R_c = (K u - f)_c.
//! The motion is never also applied as a force, and no load/reference-state
//! case is fabricated. A case with no motion primitive keeps the `None` path.
//!
//! Stage 2 (DEL-04-03 Design `pf-nonlinear-imposed-motion.md`). R-a: in a
//! model with nonlinear supports the same map reaches the nonlinear active-set
//! loop as base prescribed values on the rigid anchors and restraints. R-b: a
//! primitive on a nonlinear support's own DOF moves that support's ground by
//! g_s (`case_ground_motion`); its state is measured on u_d - g_s. Still
//! refused, and open: the base of a linear spring, a spring or constant-effort
//! device on the nonlinear record itself, and the rigid restraints of a mixed
//! nonlinear record (`IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID`).
//!
//! The exact route refuses every support-targeted primitive by name; its form
//! is `analysis_state.support_states[].boundary_motion`.
use crate::{
    diag, dof_index, exact_source_identity, has_blocking, is_constant_effort_support,
    is_variable_spring_hanger, normalize_quantity, parse_dof, parse_gap_direction, pressure_runtime,
    Diagnostic, LoadTargetInput, PreviewLoadCase, PreviewModel, PreviewPrimitiveLoad, PreviewSupport,
    DOF_PER_NODE,
};
use open_pipe_stress_nonlinear_integration::exact_gap_closing_position;
use open_pipe_stress_nonlinear_supports::GapDirection;
use open_pipe_stress_linear_supports::FrameDof;
use open_pipe_stress_units::{unit_by_symbol, Dimension};
use std::collections::{BTreeMap, HashMap};

const CATEGORY: &str = "imposed_displacement";

pub(crate) const EXACT_UNSUPPORTED: &str = "EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED";
pub(crate) const SUPPORT_UNKNOWN: &str = "IMPOSED_DISPLACEMENT_SUPPORT_UNKNOWN";
pub(crate) const SUPPORT_NOT_RIGID: &str = "IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID";
pub(crate) const DOF_UNRESTRAINED: &str = "IMPOSED_DISPLACEMENT_DOF_UNRESTRAINED";
pub(crate) const DOF_MISMATCH: &str = "IMPOSED_DISPLACEMENT_DOF_MISMATCH";
pub(crate) const DOF_DUPLICATE: &str = "IMPOSED_DISPLACEMENT_DOF_DUPLICATE";
pub(crate) const COINCIDENT_RESTRAINT: &str = "IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT";
pub(crate) const DIMENSION_INVALID: &str = "IMPOSED_DISPLACEMENT_DIMENSION_INVALID";
pub(crate) const NONLINEAR_DOF_MISMATCH: &str = "IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH";
pub(crate) const GAP_CLOSURE_INEXACT: &str = "IMPOSED_DISPLACEMENT_GAP_CLOSURE_INEXACT";
pub(crate) const TARGET_INVALID: &str = "IMPOSED_DISPLACEMENT_TARGET_INVALID";

pub(crate) fn is_support_target(load: &PreviewPrimitiveLoad) -> bool {
    matches!(load.target, LoadTargetInput::Support { .. })
}

/// The exact resolver's rigid-support rule (`case_state/resolve.rs`, `is_spring`
/// and the `rigid` test of its boundary-motion loop): not a spring or variable
/// spring hanger, not a nonlinear device and not a constant-effort device.
fn is_rigid(support: &PreviewSupport) -> bool {
    let spring = support.family.as_deref() == Some("spring") || is_variable_spring_hanger(support);
    !spring && support.nonlinear.is_none() && !is_constant_effort_support(support)
}

/// R-b: a nonlinear record that is not also a spring, a variable spring
/// hanger or a constant-effort device; a primitive on its own DOF moves its
/// ground.
fn moves_ground(support: &PreviewSupport) -> bool {
    let spring = support.family.as_deref() == Some("spring") || is_variable_spring_hanger(support);
    support.nonlinear.is_some() && !spring && !is_constant_effort_support(support)
}

/// The nonlinear DOF a record acts on, when its token parses.
fn nonlinear_dof(support: &PreviewSupport) -> Option<FrameDof> {
    support.nonlinear.as_ref().and_then(|nonlinear| parse_dof(&nonlinear.dof).ok())
}

/// R-b, §3.2: a gap's closing position from the operands normalized
/// independently by the solve's own path (clearance as a length, the motion as
/// a displacement); `None` when either cannot be normalized, which generic
/// validation refuses. `Some(false)` when c + sigma*g_s is not finite and exact
/// in binary64.
fn gap_closure_exact(support: &PreviewSupport, load: &PreviewPrimitiveLoad) -> Option<bool> {
    let nonlinear = support.nonlinear.as_ref()?;
    if nonlinear.behavior != "gap" {
        return None;
    }
    let sigma = match parse_gap_direction(nonlinear.closes_when.as_deref()?).ok()? {
        GapDirection::PositiveDisplacement => 1.0,
        GapDirection::NegativeDisplacement => -1.0,
    };
    let mut scratch = Vec::new();
    let mut clearance = nonlinear.gap.clone()?;
    normalize_quantity(&mut clearance, Dimension::Length, "", Vec::new(), &mut scratch);
    let mut ground = load.magnitude.clone();
    normalize_quantity(&mut ground, Dimension::Displacement, "", Vec::new(), &mut scratch);
    if !scratch.is_empty() || !clearance.value.is_finite() || !ground.value.is_finite() {
        return None;
    }
    Some(exact_gap_closing_position(clearance.value, sigma, ground.value).is_some())
}

fn restrains(support: &PreviewSupport, dof: FrameDof) -> usize {
    support.restraints.iter().filter(|token| parse_dof(token).ok() == Some(dof)).count()
}

fn expected_dimension(dof: FrameDof) -> (&'static str, Dimension) {
    if dof.is_translational() {
        ("displacement", Dimension::Displacement)
    } else {
        ("rotation", Dimension::Rotation)
    }
}

fn refuse(
    diagnostics: &mut Vec<Diagnostic>,
    case: &PreviewLoadCase,
    load: &PreviewPrimitiveLoad,
    code: &str,
    message: String,
    support: Option<&str>,
) {
    let mut refs = vec![load.id.clone(), case.id.clone()];
    refs.extend(support.map(str::to_string));
    diagnostics.push(diag(
        &format!(
            "diagnostic:imposed-displacement:{}",
            exact_source_identity(&[&case.id, &load.id, code])
        ),
        code,
        "blocking",
        message,
        refs,
    ));
}

/// The named checks, ahead of generic validation (and so ahead of the unit
/// checks and of boundary preparation's repeated-rigid-DOF refusal). Returns
/// whether they added a blocking diagnostic; the caller then withholds the
/// whole document (`results: []`), as for any blocking input.
pub(crate) fn validate(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) -> bool {
    let mut found = Vec::new();
    if pressure_runtime::is_exact(model) {
        // Every authored primitive, before any effective-case selection: an
        // unlisted primitive of a load/reference-state case cannot escape.
        for case in &model.load_cases {
            for load in case.primitive_loads.iter().filter(|load| is_support_target(load)) {
                let LoadTargetInput::Support { support, .. } = &load.target else { continue };
                refuse(&mut found, case, load, EXACT_UNSUPPORTED,
                    format!("load {} in {} targets support {support}; the exact route takes support and anchor motion only as analysis_state.support_states[].boundary_motion, so a support-targeted primitive load is refused and nothing is solved", load.id, case.id),
                    Some(support));
            }
        }
    } else {
        validate_pressure_free(model, &mut found);
    }
    let blocking = has_blocking(&found);
    diagnostics.extend(found);
    blocking
}

fn validate_pressure_free(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for case in &model.load_cases {
        let mut prescribed: HashMap<(&str, usize), &str> = HashMap::new();
        for load in &case.primitive_loads {
            let LoadTargetInput::Support { support: support_id, dof } = &load.target else {
                if load.category == CATEGORY {
                    refuse(diagnostics, case, load, TARGET_INVALID,
                        format!("load {} in {} is an imposed_displacement on a node or element target; an imposed displacement targets a support ({{type: \"support\", support, dof}})", load.id, case.id),
                        None);
                }
                continue;
            };
            if load.category != CATEGORY {
                refuse(diagnostics, case, load, TARGET_INVALID,
                    format!("load {} in {} targets support {support_id} with category {}; a support target carries only an imposed_displacement", load.id, case.id, load.category),
                    Some(support_id));
                continue;
            }
            let Some(support) = model.supports.iter().find(|support| &support.id == support_id) else {
                refuse(diagnostics, case, load, SUPPORT_UNKNOWN,
                    format!("load {} in {} imposes a displacement on support {support_id}, which is not in the model", load.id, case.id),
                    Some(support_id));
                continue;
            };
            let dof = match (parse_dof(dof), parse_dof(&load.direction)) {
                (Ok(target), Ok(direction)) if target == direction => target,
                _ => {
                    refuse(diagnostics, case, load, DOF_MISMATCH,
                        format!("load {} in {} names target DOF {dof} and direction {}; both must name the same frame DOF (UX, UY, UZ, RX, RY or RZ)", load.id, case.id, load.direction),
                        Some(support_id));
                    continue;
                }
            };
            if support.nonlinear.is_some() {
                validate_ground_motion(model, case, load, support, dof, &mut prescribed, diagnostics);
                continue;
            }
            if !is_rigid(support) {
                refuse(diagnostics, case, load, SUPPORT_NOT_RIGID,
                    format!("load {} in {} imposes a displacement on support {support_id}, which is a spring, a variable spring hanger or a constant-effort device; an imposed displacement applies only to a rigid restraint or to a nonlinear support's own DOF", load.id, case.id),
                    Some(support_id));
                continue;
            }
            if restrains(support, dof) == 0 {
                refuse(diagnostics, case, load, DOF_UNRESTRAINED,
                    format!("load {} in {} imposes {} on support {support_id}, which does not restrain that DOF; an imposed displacement applies only to a DOF the support already restrains rigidly", load.id, case.id, dof.as_str().to_ascii_uppercase()),
                    Some(support_id));
                continue;
            }
            let name = dof.as_str().to_ascii_uppercase();
            if !dimension_valid(case, load, dof, support_id, diagnostics) {
                continue;
            }
            let holders = model
                .supports
                .iter()
                .filter(|other| other.node == support.node && is_rigid(other))
                .map(|other| restrains(other, dof))
                .sum::<usize>();
            if holders > 1 {
                refuse(diagnostics, case, load, COINCIDENT_RESTRAINT,
                    format!("load {} in {} imposes {name} on support {support_id}, but another rigid restraint also holds {name} at node {}; the prescribed DOF must be held by exactly one rigid restraint", load.id, case.id, support.node),
                    Some(support_id));
                continue;
            }
            if let Some(first) = prescribed.insert((support.node.as_str(), dof_index(dof)), load.id.as_str()) {
                refuse(diagnostics, case, load, DOF_DUPLICATE,
                    format!("loads {first} and {} in {} both prescribe {name} at node {}; one global DOF takes one prescribed value, even when the values are equal", load.id, case.id, support.node),
                    Some(support_id));
            }
        }
    }
}

/// The named dimension check: a translation needs dimension displacement with
/// a length unit, a rotation dimension rotation with an angle unit, and the
/// magnitude must be finite. Refuses and returns false otherwise.
fn dimension_valid(
    case: &PreviewLoadCase,
    load: &PreviewPrimitiveLoad,
    dof: FrameDof,
    support_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> bool {
    let name = dof.as_str().to_ascii_uppercase();
    let (dimension_token, dimension) = expected_dimension(dof);
    let unit = unit_by_symbol(&load.magnitude.unit, dimension);
    if load.dimension != dimension_token || unit.is_err() || !load.magnitude.value.is_finite() {
        refuse(diagnostics, case, load, DIMENSION_INVALID,
            format!("load {} in {} imposes {name}: a translation needs dimension displacement with a length unit and a rotation needs dimension rotation with an angle unit, and the magnitude must be finite (got dimension {}, unit {}, value {})",
                load.id, case.id, load.dimension, load.magnitude.unit, load.magnitude.value),
            Some(support_id));
        return false;
    }
    true
}

/// R-b (§3.2): a primitive on a nonlinear record. Dimensional checks first,
/// then, on the canonical binary64 operands: the record's own DOF, a spring or
/// constant-effort device on the record, the rigid restraints of a mixed
/// record (open), the exact gap closing position, another nonlinear support on
/// the same node DOF, and the duplicate.
fn validate_ground_motion<'m>(
    model: &'m PreviewModel,
    case: &PreviewLoadCase,
    load: &'m PreviewPrimitiveLoad,
    support: &'m PreviewSupport,
    dof: FrameDof,
    prescribed: &mut HashMap<(&'m str, usize), &'m str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let support_id = support.id.as_str();
    let name = dof.as_str().to_ascii_uppercase();
    if !dimension_valid(case, load, dof, support_id, diagnostics) {
        return;
    }
    let own = nonlinear_dof(support);
    if own != Some(dof) {
        if restrains(support, dof) > 0 {
            refuse(diagnostics, case, load, SUPPORT_NOT_RIGID,
                format!("load {} in {} imposes {name} on a rigid restraint of nonlinear record {support_id}; the rigid restraints of a mixed nonlinear record do not move (open), only the nonlinear support's own DOF does", load.id, case.id),
                Some(support_id));
        } else {
            refuse(diagnostics, case, load, NONLINEAR_DOF_MISMATCH,
                format!("load {} in {} imposes {name} on nonlinear support {support_id}, which acts on {}; a moving ground applies only to the nonlinear support's own DOF", load.id, case.id,
                    support.nonlinear.as_ref().map_or("", |nonlinear| nonlinear.dof.as_str())),
                Some(support_id));
        }
        return;
    }
    if !moves_ground(support) {
        refuse(diagnostics, case, load, SUPPORT_NOT_RIGID,
            format!("load {} in {} imposes {name} on nonlinear record {support_id}, which also carries a spring, a variable spring hanger or a constant-effort device; moving its ground would also move that device's base, so the request is refused", load.id, case.id),
            Some(support_id));
        return;
    }
    if gap_closure_exact(support, load) == Some(false) {
        refuse(diagnostics, case, load, GAP_CLOSURE_INEXACT,
            format!("load {} in {} moves the ground of gap {support_id}; its closing position c + sigma*g_s, formed from the normalized clearance and motion, is not finite and exact in binary64, and a rounded closing position is never used", load.id, case.id),
            Some(support_id));
        return;
    }
    let shared = model
        .supports
        .iter()
        .any(|other| other.id != support.id && other.node == support.node && nonlinear_dof(other) == Some(dof));
    if shared {
        refuse(diagnostics, case, load, COINCIDENT_RESTRAINT,
            format!("load {} in {} moves the ground of nonlinear support {support_id} in {name}, but another nonlinear support also acts on {name} at node {}; a moving ground must belong to exactly one support", load.id, case.id, support.node),
            Some(support_id));
        return;
    }
    if let Some(first) = prescribed.insert((support.node.as_str(), dof_index(dof)), load.id.as_str()) {
        refuse(diagnostics, case, load, DOF_DUPLICATE,
            format!("loads {first} and {} in {} both prescribe {name} at node {}; one global DOF takes one prescribed value, even when the values are equal", load.id, case.id, support.node),
            Some(support_id));
    }
}

/// R-b: the case's ground offsets g_s (m or rad), by nonlinear support id, from
/// its validated primitives on nonlinear records; empty when there are none.
/// Called after unit normalization.
pub(crate) fn case_ground_motion(model: &PreviewModel, case: &PreviewLoadCase) -> Vec<(String, f64)> {
    case.primitive_loads
        .iter()
        .filter_map(|load| {
            let LoadTargetInput::Support { support, .. } = &load.target else { return None };
            model
                .supports
                .iter()
                .find(|record| &record.id == support && record.nonlinear.is_some())
                .map(|record| (record.id.clone(), load.magnitude.value))
        })
        .collect()
}

/// The case's prescribed map, global DOF to its normalized value (m or rad),
/// from its validated motion primitives; `None` when the case has none, which
/// keeps the existing path. Called after unit normalization. An insertion that
/// would overwrite, or a value off the prepared rigid boundary, is a blocking
/// finding (unreachable after `validate`); nothing is silently replaced.
pub(crate) fn case_prescribed(
    model: &PreviewModel,
    case: &PreviewLoadCase,
    restrained_dofs: &[usize],
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<BTreeMap<usize, f64>> {
    let node_index = model
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let mut prescribed = BTreeMap::new();
    let mut any = false;
    for load in &case.primitive_loads {
        let LoadTargetInput::Support { support, dof } = &load.target else { continue };
        // R-b: a primitive on a nonlinear record moves that support's ground
        // (`case_ground_motion`); it is not a rigid boundary value.
        if model.supports.iter().any(|record| &record.id == support && record.nonlinear.is_some()) {
            continue;
        }
        any = true;
        let global = model
            .supports
            .iter()
            .find(|record| &record.id == support)
            .and_then(|record| node_index.get(record.node.as_str()))
            .zip(parse_dof(dof).ok())
            .map(|(&node, dof)| node * DOF_PER_NODE + dof_index(dof));
        let code = match global {
            None => Some(SUPPORT_UNKNOWN),
            Some(global) if !restrained_dofs.contains(&global) => Some(DOF_UNRESTRAINED),
            Some(global) if prescribed.contains_key(&global) => Some(DOF_DUPLICATE),
            Some(global) => {
                prescribed.insert(global, load.magnitude.value);
                None
            }
        };
        if let Some(code) = code {
            refuse(diagnostics, case, load, code,
                format!("load {} in {} does not address one prepared rigid boundary DOF of support {support}", load.id, case.id),
                Some(support));
        }
    }
    any.then_some(prescribed)
}
