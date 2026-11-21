//! DEL-05-01 REQ-008: the seismic model-check warning (issue 452 decision 7).
//!
//! On each seismic case (a case with `equivalent_static.seismic`) of a
//! document that is not a load and reference-state document, three findings
//! at warning severity, which refuse nothing and change no load:
//!
//! - [`WEIGHT_LOAD_ENTERED`]: one per load of the case in the solver's weight
//!   category (authored `weight` or `distributed_force`, an entered
//!   distributed member load included) that is not a fresh product of the
//!   self-weight plan. Applied generated self-weight, which
//!   `self_weight::validate_applied_self_weight` admits as fresh, is not
//!   named; a manual override of generated self-weight is named. A stale or
//!   modified plan primitive is refused before this check runs.
//! - [`CONDITION_WEIGHT_ADDED`]: one per added weight the case takes from an
//!   operating condition (the primitive `operating_conditions::lower`
//!   appends), which only the pressure-free route admits.
//! - [`GRAVITY_DIFFERS`]: one per case whose seismic gravity differs from the
//!   gravity at which a fresh applied component-weight primitive of a
//!   component whose force the case generates was formed. The comparison is
//!   exact, of the two magnitudes after the document's ordinary unit
//!   conversion; the finding prints both as entered. Only fresh
//!   component-weight primitives are read: an overridden component weight
//!   gives no known gravity, and a case whose component weights are all
//!   overridden has no gravity finding, which is not agreement. The
//!   primitives read are the case's own when it has any, otherwise those of
//!   every case of the document.
//!
//! None of the three is raised for a case whose seismic generation the
//! generator refuses ([`admitted`]): such a case generates no seismic load.
//!
//! Generation is not extended to entered weight loads or to a condition's
//! added weight, and the texts do not assert that the engineer failed to
//! supply forces. The findings are raised before formation, just after the
//! model's unit normalization stops blocking, in the solve and in the
//! pre-solve model checks alike: [`capture`] reads what the findings print as
//! entered and the plan's fresh primitives before normalization, and
//! [`append`] raises them after it.
use super::{diag, normalize_quantity, stable_suffix, Diagnostic, LoadTargetInput, PreviewLoadCase, PreviewModel, PreviewPrimitiveLoad, Quantity};
use crate::self_weight::{inspect_applied_self_weight, AppliedSelfWeightState, COMPONENT_WEIGHT_METHOD};
use open_pipe_stress_primitive_loads::PrimitiveLoadCategory;
use open_pipe_stress_units::Dimension;
use std::collections::HashSet;

pub(crate) const WEIGHT_LOAD_ENTERED: &str = "EQUIVALENT_STATIC_SEISMIC_WEIGHT_LOAD_ENTERED";
pub(crate) const CONDITION_WEIGHT_ADDED: &str = "EQUIVALENT_STATIC_SEISMIC_CONDITION_WEIGHT_ADDED";
pub(crate) const GRAVITY_DIFFERS: &str = "EQUIVALENT_STATIC_SEISMIC_GRAVITY_DIFFERS";

/// One fresh applied component-weight primitive: its case and index, its
/// component and the gravity request it was formed with, as entered.
struct ComponentWeight {
    case: usize,
    load_id: String,
    component: String,
    gravity: Quantity,
}

/// What the findings read before the model's units are normalized.
#[derive(Default)]
pub(crate) struct Entered {
    /// Each seismic case (by index) and its gravity as entered.
    seismic: Vec<(usize, Option<Quantity>)>,
    /// The self-weight plan's fresh primitives, (case index, primitive index).
    fresh: HashSet<(usize, usize)>,
    /// The fresh component-weight primitives, in document order.
    component_weights: Vec<ComponentWeight>,
}

fn is_seismic(case: &PreviewLoadCase) -> bool {
    case.equivalent_static.as_ref().is_some_and(|g| g.seismic.is_some())
}

/// Read, before normalization, what the findings need of the document as
/// entered. Nothing is read when no case is seismic, or on a load and
/// reference-state document, where seismic generation is refused.
pub(crate) fn capture(model: &PreviewModel) -> Entered {
    let mut entered = Entered::default();
    if crate::case_state::is_load_state(model) || !model.load_cases.iter().any(is_seismic) {
        return entered;
    }
    entered.seismic = model.load_cases.iter().enumerate().filter(|(_, case)| is_seismic(case))
        .map(|(i, case)| (i, case.equivalent_static.as_ref().and_then(|g| g.seismic.as_ref()).and_then(|s| s.gravity_acceleration.clone())))
        .collect();
    for status in inspect_applied_self_weight(model, None) {
        if status.state != AppliedSelfWeightState::Fresh {
            continue;
        }
        let Some(case) = model.load_cases.iter().position(|c| c.id == status.case_id) else { continue };
        entered.fresh.insert((case, status.primitive_index));
        let load = &model.load_cases[case].primitive_loads[status.primitive_index];
        let Some(evidence) = load.provenance.as_deref().and_then(|p| serde_json::from_str::<serde_json::Value>(p).ok()) else { continue };
        if evidence["method"] != COMPONENT_WEIGHT_METHOD {
            continue;
        }
        if let (Some(component), Some(value), Some(unit)) =
            (evidence["component_id"].as_str(), evidence["gravity"]["value"].as_f64(), evidence["gravity"]["unit"].as_str())
        {
            entered.component_weights.push(ComponentWeight {
                case,
                load_id: load.id.clone(),
                component: component.to_string(),
                gravity: Quantity { value, unit: unit.to_string() },
            });
        }
    }
    entered
}

/// The added weight a case takes from an operating condition: the primitive
/// `operating_conditions::lower` appends, by its identity and provenance.
fn condition_added(case: &PreviewLoadCase, load: &PreviewPrimitiveLoad) -> Option<String> {
    let LoadTargetInput::Element { pipe } = &load.target else { return None };
    if load.id != crate::operating_conditions::resolution::added_weight_load_id(&case.id, pipe) {
        return None;
    }
    let provenance = serde_json::from_str::<serde_json::Value>(load.provenance.as_deref()?).ok()?;
    (provenance["method"] == "operating_condition_set_v1" && provenance["quantity"] == "added_weight").then(|| pipe.clone())
}

/// A value as entered, with its unit.
fn shown(q: &Quantity) -> String {
    format!("{} {}", q.value, q.unit)
}

/// The magnitude of a self-weight gravity request (signed along its axis), as
/// entered, with its unit.
fn shown_applied(q: &Quantity) -> String {
    format!("{} {}", q.value.abs(), q.unit)
}

/// The magnitude of a self-weight gravity request in m/s² by the document's
/// ordinary conversion, or `None` when it does not convert to a finite value.
fn magnitude(q: &Quantity) -> Option<f64> {
    let mut converted = Quantity { value: q.value.abs(), unit: q.unit.clone() };
    let mut scratch = Vec::new();
    normalize_quantity(&mut converted, Dimension::Acceleration, "diagnostic:equivalent-static:seismic:gravity", vec![], &mut scratch);
    (scratch.is_empty() && converted.value.is_finite()).then_some(converted.value)
}

/// Raise the findings on the normalized `model`, from what [`capture`] read
/// before normalization.
pub(crate) fn append(model: &PreviewModel, entered: &Entered, diagnostics: &mut Vec<Diagnostic>) {
    for (case_index, gravity) in &entered.seismic {
        let case = &model.load_cases[*case_index];
        // One gate for all three findings: a case whose seismic generation
        // the generator refuses generates no seismic load, so none of them is
        // raised for it.
        let Some(plan) = admitted(model, case) else { continue };
        for (index, load) in case.primitive_loads.iter().enumerate() {
            if !matches!(crate::parse_category(&load.category), Ok(PrimitiveLoadCategory::Weight)) {
                continue;
            }
            if let Some(pipe) = condition_added(case, load) {
                diagnostics.push(diag(
                    &format!("diagnostic:equivalent-static:{}:seismic:condition-weight:{}", stable_suffix(&case.id), stable_suffix(&load.id)),
                    CONDITION_WEIGHT_ADDED,
                    "warning",
                    format!(
                        "seismic case {}: the added weight {} that its operating condition gives member {pipe} acts in the case; static-equivalent seismic generation forms its loads from the model's own mass and geometry and is not extended to a condition's added weight",
                        case.id, load.id
                    ),
                    vec![case.id.clone(), load.id.clone(), pipe],
                ));
            } else if !entered.fresh.contains(&(*case_index, index)) {
                diagnostics.push(diag(
                    &format!("diagnostic:equivalent-static:{}:seismic:weight-load:{}", stable_suffix(&case.id), stable_suffix(&load.id)),
                    WEIGHT_LOAD_ENTERED,
                    "warning",
                    format!(
                        "seismic case {}: the entered weight load {} acts in the case as authored; static-equivalent seismic generation forms its loads from the model's own mass and geometry and is not extended to entered weight loads",
                        case.id, load.id
                    ),
                    vec![case.id.clone(), load.id.clone()],
                ));
            }
        }
        if let Some(finding) = gravity_finding(model, entered, *case_index, gravity.as_ref(), &plan) {
            diagnostics.push(finding);
        }
    }
}

/// The generator's own admission of the case's seismic generation: the
/// component plan when [`crate::append_seismic_generated_loads`] generates
/// the case's seismic loads with no refusal (a present, finite, strictly
/// positive gravity; at least one entered g-factor, every entered one finite;
/// no component-plan refusal; every unreplaced member's mass; the
/// generator's own checks) and no generated id collides with an authored one
/// (`refuse_generated_id_collisions`); otherwise `None`. The generator is
/// asked, so the gate cannot drift from it.
fn admitted<'a>(model: &'a PreviewModel, case: &PreviewLoadCase) -> Option<crate::seismic_component::Plan<'a>> {
    let seismic = case.equivalent_static.as_ref()?.seismic.as_ref()?;
    let mut loads = Vec::new();
    let mut refusals = Vec::new();
    let plan = crate::append_seismic_generated_loads(model, case, seismic, &mut loads, &mut refusals)?;
    crate::refuse_generated_id_collisions(case, &loads, &mut refusals);
    refusals.is_empty().then_some(plan)
}

fn gravity_finding(model: &PreviewModel, entered: &Entered, case_index: usize, gravity: Option<&Quantity>, plan: &crate::seismic_component::Plan) -> Option<Diagnostic> {
    let case = &model.load_cases[case_index];
    let entered_gravity = gravity?;
    // The normalized seismic gravity (normalize_model_units converted it),
    // finite and strictly positive since the case is admitted; it is not a
    // signed request, so no magnitude is taken.
    let seismic = case.equivalent_static.as_ref()?.seismic.as_ref()?.gravity_acceleration.as_ref()?.value;
    let generated: HashSet<&str> = plan.items.iter().map(|item| item.component.id.as_str()).collect();
    let applies = |w: &&ComponentWeight| generated.contains(w.component.as_str());
    let own: Vec<&ComponentWeight> = entered.component_weights.iter().filter(|w| w.case == case_index).filter(applies).collect();
    let read = if own.is_empty() { entered.component_weights.iter().filter(applies).collect() } else { own };
    let mut differing: Vec<(f64, &ComponentWeight)> = Vec::new();
    for w in read {
        let Some(applied) = magnitude(&w.gravity) else { continue };
        if applied != seismic && !differing.iter().any(|(m, _)| *m == applied) {
            differing.push((applied, w));
        }
    }
    if differing.is_empty() {
        return None;
    }
    let mut components: Vec<&str> = Vec::new();
    for (_, w) in &differing {
        if !components.contains(&w.component.as_str()) {
            components.push(&w.component);
        }
    }
    let applied: Vec<String> = differing.iter().map(|(_, w)| shown_applied(&w.gravity)).collect();
    let mut refs = vec![case.id.clone()];
    refs.extend(components.iter().map(|c| c.to_string()));
    refs.extend(differing.iter().map(|(_, w)| w.load_id.clone()));
    Some(diag(
        &format!("diagnostic:equivalent-static:{}:seismic:gravity", stable_suffix(&case.id)),
        GRAVITY_DIFFERS,
        "warning",
        format!(
            "seismic case {} states gravity {}, and the applied self-weight of {} {}, whose seismic force the case generates, was formed at {}; the component's seismic force is its g-factor times its entered weight at either gravity, while the members' seismic loads use the case's gravity",
            case.id,
            shown(entered_gravity),
            if components.len() == 1 { "component" } else { "components" },
            components.join(", "),
            applied.join(", ")
        ),
        refs,
    ))
}
