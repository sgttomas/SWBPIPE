//! DEL-05-01 REQ-008: the seismic model-check warning (issue 452 decision 7).
//!
//! On each seismic case (a case with `equivalent_static.seismic`) — on plain
//! documents against the case's primitive loads, and on load and
//! reference-state documents against each case's resolved load sources
//! (`effective_case`) and applied component-weight primitives — three findings
//! at warning severity, which refuse nothing and change no load:
//!
//! - [`WEIGHT_LOAD_ENTERED`]: one per case that has any load in the solver's
//!   weight category (authored `weight` or `distributed_force`, an entered
//!   distributed member load included) that is not a fresh product of the
//!   self-weight plan. Applied generated self-weight, which
//!   `self_weight::validate_applied_self_weight` admits as fresh, is exempt;
//!   a manual override of generated self-weight is covered. A stale or
//!   modified plan primitive is refused before this check runs.
//! - [`CONDITION_WEIGHT_ADDED`]: one per case that takes any added weight from
//!   an operating condition: exactly the primitives
//!   `operating_conditions::lower` reports it appended, which only the
//!   pressure-free route admits. An authored load is an entered weight load
//!   whatever its id or provenance.
//!
//!   Each of these two states how many loads it covers and names them in the
//!   case's load order, at most [`MAX_NAMED_LOADS`] (10) of them, reporting
//!   the remainder as "and N more". Its affected references are the case and
//!   the named loads (for an added weight, each named load and then its
//!   member, each once), so a case with thousands of entered loads raises one
//!   finding of bounded size.
//! - [`GRAVITY_DIFFERS`]: one per case whose seismic gravity differs from the
//!   gravity at which a fresh applied component-weight primitive of a
//!   component whose force the case generates was formed. The comparison is
//!   exact, of the two magnitudes after the document's ordinary unit
//!   conversion; the finding prints both as entered. Only fresh
//!   component-weight primitives are read: an overridden component weight
//!   gives no known gravity. So where no fresh applicable component weight
//!   exists anywhere in the document for a generated component, there is no
//!   gravity finding, which is not agreement; a fresh applicable component
//!   weight elsewhere in the document is still compared. The primitives read
//!   are the case's own when it has any, otherwise those of every case of
//!   the document.
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
use super::{diag, normalize_quantity, Diagnostic, LoadTargetInput, PreviewLoadCase, PreviewModel, PreviewPrimitiveLoad, Quantity};
use crate::self_weight::{inspect_applied_self_weight, AppliedSelfWeightState, COMPONENT_WEIGHT_METHOD};
use open_pipe_stress_primitive_loads::PrimitiveLoadCategory;
use open_pipe_stress_units::Dimension;
use std::collections::HashSet;

pub(crate) const WEIGHT_LOAD_ENTERED: &str = "EQUIVALENT_STATIC_SEISMIC_WEIGHT_LOAD_ENTERED";
pub(crate) const CONDITION_WEIGHT_ADDED: &str = "EQUIVALENT_STATIC_SEISMIC_CONDITION_WEIGHT_ADDED";
pub(crate) const GRAVITY_DIFFERS: &str = "EQUIVALENT_STATIC_SEISMIC_GRAVITY_DIFFERS";

/// The most loads a weight or condition finding names; the rest are counted.
pub(crate) const MAX_NAMED_LOADS: usize = 10;

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
    /// The self-weight plan's fresh primitives by identity, (case index, load id),
    /// for load and reference-state effective cases whose indices follow `load_sources`.
    fresh_ids: HashSet<(usize, String)>,
    /// The fresh component-weight primitives, in document order.
    component_weights: Vec<ComponentWeight>,
    /// The added weights the lowering appended, (case index, load id).
    condition_added: HashSet<(usize, String)>,
}

fn is_seismic(case: &PreviewLoadCase) -> bool {
    case.equivalent_static.as_ref().is_some_and(|g| g.seismic.is_some())
}

/// Read, before normalization, what the findings need of the document as
/// entered, with `condition_added`, the added weights
/// `operating_conditions::lower` appended. Nothing is read when no case is
/// seismic.
pub(crate) fn capture(model: &PreviewModel, condition_added: &[(usize, String)]) -> Entered {
    let mut entered = Entered::default();
    if !model.load_cases.iter().any(is_seismic) {
        return entered;
    }
    entered.seismic = model.load_cases.iter().enumerate().filter(|(_, case)| is_seismic(case))
        .map(|(i, case)| (i, case.equivalent_static.as_ref().and_then(|g| g.seismic.as_ref()).and_then(|s| s.gravity_acceleration.clone())))
        .collect();
    entered.condition_added = condition_added.iter().cloned().collect();
    for status in inspect_applied_self_weight(model, None) {
        if status.state != AppliedSelfWeightState::Fresh {
            continue;
        }
        let Some(case) = model.load_cases.iter().position(|c| c.id == status.case_id) else { continue };
        entered.fresh.insert((case, status.primitive_index));
        let load = &model.load_cases[case].primitive_loads[status.primitive_index];
        entered.fresh_ids.insert((case, load.id.clone()));
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

/// The member of an added weight the lowering appended to the case at
/// `case_index`, or `None` for any other load.
fn condition_added(entered: &Entered, case_index: usize, load: &PreviewPrimitiveLoad) -> Option<String> {
    let LoadTargetInput::Element { pipe } = &load.target else { return None };
    entered.condition_added.contains(&(case_index, load.id.clone())).then(|| pipe.clone())
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

/// A finding's id: the case's id as entered and the finding's own `part`
/// (`weight-load`, `condition-weight` or `gravity`; each finding is raised at
/// most once per case). Case ids are unique in a document, and the case id
/// ends where the fixed tail `:seismic:model-check:` + `part` begins, so two
/// findings never share an id. Every other
/// `diagnostic:equivalent-static:` id is a fixed pattern of colon-free parts
/// with no `model-check` where these have it, so none equals one of these.
/// (`stable_suffix` would map `case:s` and `case-s` to one id.)
fn id(case: &str, part: &str) -> String {
    format!("diagnostic:equivalent-static:{case}:seismic:model-check:{part}")
}

fn plural(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}
fn verb(n: usize) -> &'static str {
    if n == 1 { "s" } else { "" }
}
/// The named loads, then the count of those beyond [`MAX_NAMED_LOADS`].
fn listed(named: Vec<String>, total: usize) -> String {
    let mut text = named.join(", ");
    if total > MAX_NAMED_LOADS {
        text.push_str(&format!(" and {} more", total - MAX_NAMED_LOADS));
    }
    text
}

/// Raise the findings on the normalized `model`, from what [`capture`] read
/// before normalization. On a load and reference-state document,
/// `resolved_cases` provides each case's effective case over `load_sources`.
pub(crate) fn append(
    model: &PreviewModel,
    materials: &[crate::MaterialInput],
    resolved_cases: Option<&[crate::case_state::resolve::ResolvedCase]>,
    entered: &Entered,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (case_index, gravity) in &entered.seismic {
        let resolved = resolved_cases.map(|cases| &cases[*case_index]);
        let case = resolved.map_or(&model.load_cases[*case_index], |state| &state.effective_case);
        // One gate for all three findings: a case whose seismic generation
        // the generator refuses generates no seismic load, so none of them is
        // raised for it.
        let Some(plan) = admitted(model, materials, case, resolved) else { continue };
        let mut weights: Vec<&str> = Vec::new();
        let mut conditions: Vec<(&str, String)> = Vec::new();
        for (index, load) in case.primitive_loads.iter().enumerate() {
            if !matches!(crate::parse_category(&load.category), Ok(PrimitiveLoadCategory::Weight)) {
                continue;
            }
            let is_fresh = if resolved.is_some() {
                entered.fresh_ids.contains(&(*case_index, load.id.clone()))
            } else {
                entered.fresh.contains(&(*case_index, index))
            };
            if let Some(pipe) = condition_added(entered, *case_index, load) {
                conditions.push((&load.id, pipe));
            } else if !is_fresh {
                weights.push(&load.id);
            }
        }
        if !conditions.is_empty() {
            let named: Vec<String> = conditions.iter().take(MAX_NAMED_LOADS).map(|(load, pipe)| format!("{load} (member {pipe})")).collect();
            let mut refs = vec![case.id.clone()];
            for (load, pipe) in conditions.iter().take(MAX_NAMED_LOADS) {
                for r in [load.to_string(), pipe.clone()] {
                    if !refs.contains(&r) {
                        refs.push(r);
                    }
                }
            }
            diagnostics.push(diag(
                &id(&case.id, "condition-weight"),
                CONDITION_WEIGHT_ADDED,
                "warning",
                format!(
                    "seismic case {}: {} added weight{} that its operating condition gives act{} in the case: {}; static-equivalent seismic generation forms its loads from the model's own mass and geometry and is not extended to a condition's added weight",
                    case.id,
                    conditions.len(),
                    plural(conditions.len()),
                    verb(conditions.len()),
                    listed(named, conditions.len())
                ),
                refs,
            ));
        }
        if !weights.is_empty() {
            let named: Vec<String> = weights.iter().take(MAX_NAMED_LOADS).map(|load| load.to_string()).collect();
            let mut refs = vec![case.id.clone()];
            refs.extend(named.iter().cloned());
            let message = if resolved.is_some() {
                format!(
                    "seismic case {}: {} entered weight load{} {} included in the case's resolved load sources: {}; static-equivalent seismic generation forms its loads from the model's own mass and geometry and is not extended to entered weight loads",
                    case.id,
                    weights.len(),
                    plural(weights.len()),
                    if weights.len() == 1 { "is" } else { "are" },
                    listed(named, weights.len())
                )
            } else {
                format!(
                    "seismic case {}: {} entered weight load{} act{} in the case as authored: {}; static-equivalent seismic generation forms its loads from the model's own mass and geometry and is not extended to entered weight loads",
                    case.id,
                    weights.len(),
                    plural(weights.len()),
                    verb(weights.len()),
                    listed(named, weights.len())
                )
            };
            diagnostics.push(diag(
                &id(&case.id, "weight-load"),
                WEIGHT_LOAD_ENTERED,
                "warning",
                message,
                refs,
            ));
        }
        if let Some(finding) = gravity_finding(model, resolved_cases, entered, *case_index, gravity.as_ref(), &plan) {
            diagnostics.push(finding);
        }
    }
}

/// The generator's own admission of the case's generation: the component plan
/// when [`crate::append_equivalent_static_generated_loads`] admits both seismic
/// and any accompanying wind generation on the case with no refusal and no
/// generated id collision (`refuse_generated_id_collisions`,
/// `refuse_load_state_generated_collisions`, and
/// `refuse_document_generated_collisions`); otherwise `None`. The case's
/// complete generation gate is asked so warnings agree with `solve_prepared_load_case`.
fn admitted<'a>(
    model: &'a PreviewModel,
    materials: &[crate::MaterialInput],
    case: &PreviewLoadCase,
    resolved: Option<&crate::case_state::resolve::ResolvedCase>,
) -> Option<crate::seismic_component::Plan<'a>> {
    case.equivalent_static.as_ref()?.seismic.as_ref()?;
    let mut loads = Vec::new();
    let mut refusals = Vec::new();
    let plan = crate::append_equivalent_static_generated_loads(model, case, &mut loads, &mut refusals)?;
    if let Some(state) = resolved {
        crate::refuse_load_state_generated_collisions(model, case, state, &loads, &mut refusals);
    }
    crate::refuse_document_generated_collisions(model, materials, case, &loads, &mut refusals);
    refusals.is_empty().then_some(plan)
}

fn gravity_finding(
    model: &PreviewModel,
    resolved_cases: Option<&[crate::case_state::resolve::ResolvedCase]>,
    entered: &Entered,
    case_index: usize,
    gravity: Option<&Quantity>,
    plan: &crate::seismic_component::Plan,
) -> Option<Diagnostic> {
    let case = resolved_cases.map_or(&model.load_cases[case_index], |cases| &cases[case_index].effective_case);
    let entered_gravity = gravity?;
    // The normalized seismic gravity (normalize_model_units converted it),
    // finite and strictly positive since the case is admitted; it is not a
    // signed request, so no magnitude is taken.
    let seismic = case.equivalent_static.as_ref()?.seismic.as_ref()?.gravity_acceleration.as_ref()?.value;
    let generated: HashSet<&str> = plan.items.iter().map(|item| item.component.id.as_str()).collect();
    let applied_in_case = |w: &&ComponentWeight| {
        resolved_cases.is_none_or(|cases| cases[w.case].effective_case.primitive_loads.iter().any(|l| l.id == w.load_id))
    };
    let applies = |w: &&ComponentWeight| generated.contains(w.component.as_str()) && applied_in_case(w);
    let own: Vec<&ComponentWeight> = entered.component_weights.iter().filter(|w| w.case == case_index).filter(applies).collect();
    let read = if own.is_empty() { entered.component_weights.iter().filter(applies).collect() } else { own };
    // Every differing weight is named, with its component and load; only the
    // magnitudes shown are distinct (each as first entered).
    let mut differing: Vec<(f64, &ComponentWeight)> = Vec::new();
    for w in read {
        let Some(applied) = magnitude(&w.gravity) else { continue };
        if applied != seismic {
            differing.push((applied, w));
        }
    }
    if differing.is_empty() {
        return None;
    }
    let mut components: Vec<&str> = Vec::new();
    let mut magnitudes: Vec<(f64, String)> = Vec::new();
    for (applied, w) in &differing {
        if !components.contains(&w.component.as_str()) {
            components.push(&w.component);
        }
        if !magnitudes.iter().any(|(m, _)| m == applied) {
            magnitudes.push((*applied, shown_applied(&w.gravity)));
        }
    }
    let applied: Vec<String> = magnitudes.into_iter().map(|(_, shown)| shown).collect();
    let mut refs = vec![case.id.clone()];
    refs.extend(components.iter().map(|c| c.to_string()));
    refs.extend(differing.iter().map(|(_, w)| w.load_id.clone()));
    Some(diag(
        &id(&case.id, "gravity"),
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
