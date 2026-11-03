//! The state case's post-lowering binding (Agent 0 decision D-L, adopted in
//! place of an observation hook in the operating-condition lowering).
//!
//! The modal entry lowers the model once, through the solve's own
//! `operating_conditions::lower`; it never plans, judges or resolves a second
//! time. [`ModalCaseContext::capture`] keeps access to the named case as it
//! was authored, before lowering. [`ModalCaseContext::bind`] then proves, from
//! the lowered model and authored facts alone, that the lowered case is that
//! case with exactly S2's documented output appended
//! (`operating_conditions::resolution`):
//! 1. the same case index and id;
//! 2. the complete authored primitive-load prefix, equal by value (every
//!    field, magnitudes by their bits), and every other member of the case
//!    unchanged except the two lowering is documented to change: the cleared
//!    `condition_use` (its authored presence and value are recorded here) and
//!    the wind exposure `ResolutionPlan::apply` sets;
//! 3. the generated suffix: every temperature record, then every added-weight
//!    record, each in `pipe_segments` order, each with S2's identity,
//!    provenance, category, target, direction and dimension. Which pipes carry
//!    a record follows from authored presence facts only (the uses the case
//!    states, each pipe's `condition_set_ref`, the set's condition key and its
//!    `added_weight`); no temperature difference or magnitude is computed
//!    here. A generated temperature magnitude is S2's output, bound by its
//!    identity and provenance; an added-weight magnitude is S2's verbatim copy
//!    of the set's authored value and is compared with it;
//! 4. the wind exposure: absent, unstated, authored (possibly empty) or
//!    resolved (possibly empty), the resolved list being exactly the pipes
//!    whose set marks `exposure.wind`, in pipe order.
//!
//! Missing or contradictory evidence refuses (`MODAL_STATE_CASE_UNAVAILABLE`)
//! with the reason; there is no fallback to a second resolution or to a
//! prefix-only match. A condition's added weight is an action, never mass.
use super::MODAL_STATE_CASE_UNAVAILABLE;
use crate::operating_conditions::resolution::{
    added_weight_load_id, added_weight_provenance, temperature_load_id, temperature_provenance, THERMAL_DIRECTION,
};
use crate::operating_conditions::{ConditionSetInput, ConditionUseInput, GlobalAxis};
use crate::{diag, stable_suffix, Diagnostic, LoadTargetInput, PreviewLoadCase, PreviewModel, PreviewPrimitiveLoad, Quantity};
use std::ops::Range;

/// The wind exposure of the state case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WindPresence {
    /// The case has no `equivalent_static.wind`.
    Absent,
    /// A wind input whose `exposed_pipe_refs` is neither authored nor resolved.
    Unstated,
    /// Authored `exposed_pipe_refs`, possibly empty.
    Authored(Vec<String>),
    /// Resolved from the pipes' sets by the case's `wind_exposure` use, possibly empty.
    Resolved(Vec<String>),
}

/// The bound state case: its position and the ranges of its lowered
/// primitive loads. Generated added weight is an action, never mass.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BoundCase {
    pub(crate) case_index: usize,
    pub(crate) case_id: String,
    pub(crate) authored: Range<usize>,
    pub(crate) generated_temperatures: Range<usize>,
    pub(crate) generated_added_weights: Range<usize>,
    pub(crate) wind: WindPresence,
    /// Whether the case authored a `condition_use`.
    pub(crate) condition_use_authored: bool,
}

/// The authored state case, captured before lowering.
#[derive(Debug, Clone)]
pub(crate) struct ModalCaseContext<'a> {
    original: &'a PreviewModel,
    case_index: usize,
    /// The case's authored `condition_use`: its presence and value.
    condition_use: Option<ConditionUseInput>,
    /// The case's authored wind exposure.
    authored_wind: WindPresence,
}

fn unavailable(case: &str, reason: String) -> Diagnostic {
    diag(
        &format!("diagnostic:modal:state-case:{}:binding", stable_suffix(case)),
        MODAL_STATE_CASE_UNAVAILABLE,
        "blocking",
        format!("state case `{case}` cannot be bound to the lowered model: {reason}; nothing is formed"),
        vec!["state_case".to_string(), case.to_string()],
    )
}

fn wind_of(case: &PreviewLoadCase) -> WindPresence {
    match case.equivalent_static.as_ref().and_then(|generation| generation.wind.as_ref()) {
        None => WindPresence::Absent,
        Some(wind) if wind.authors_exposed_pipe_refs() => WindPresence::Authored(wind.exposed_pipe_refs.clone()),
        Some(_) => WindPresence::Unstated,
    }
}

fn target_eq(a: &LoadTargetInput, b: &LoadTargetInput) -> bool {
    match (a, b) {
        (LoadTargetInput::Node { node: x }, LoadTargetInput::Node { node: y }) => x == y,
        (LoadTargetInput::Element { pipe: x }, LoadTargetInput::Element { pipe: y }) => x == y,
        (LoadTargetInput::Support { support: s, dof: d }, LoadTargetInput::Support { support: t, dof: e }) => {
            s == t && d == e
        }
        _ => false,
    }
}

fn quantity_eq(a: &Quantity, b: &Quantity) -> bool {
    a.value.to_bits() == b.value.to_bits() && a.unit == b.unit
}

/// The first field in which two primitive loads differ, compared exactly
/// (the magnitude by its bits), or `None` when they are equal by value.
pub(crate) fn primitive_load_difference(a: &PreviewPrimitiveLoad, b: &PreviewPrimitiveLoad) -> Option<&'static str> {
    if a.id != b.id {
        Some("id")
    } else if a.category != b.category {
        Some("category")
    } else if !target_eq(&a.target, &b.target) {
        Some("target")
    } else if a.direction != b.direction {
        Some("direction")
    } else if a.magnitude.value.to_bits() != b.magnitude.value.to_bits() {
        Some("magnitude.value")
    } else if a.magnitude.unit != b.magnitude.unit {
        Some("magnitude.unit")
    } else if a.dimension != b.dimension {
        Some("dimension")
    } else if a.provenance != b.provenance {
        Some("provenance")
    } else {
        None
    }
}

/// The case with the members lowering is documented to change neutralized:
/// its primitive loads (compared separately), `condition_use` and the wind
/// exposure list and its authored flag. Every remaining member is rendered by
/// its complete `Debug` form, which distinguishes every two values of each
/// member except NaN payloads (`f64`'s `Debug` is its shortest round-trip
/// form, signed zeros included), so the rendering compares the rest of the
/// case exactly without adding a derive to a shared type.
fn rest_of_case(case: &PreviewLoadCase) -> String {
    let mut rest = case.clone();
    rest.primitive_loads.clear();
    rest.condition_use = None;
    if let Some(wind) = rest.equivalent_static.as_mut().and_then(|generation| generation.wind.as_mut()) {
        wind.exposed_pipe_refs.clear();
        wind.exposed_pipe_refs_authored = false;
    }
    format!("{rest:?}")
}

/// What S2 generates for one pipe, from authored facts only.
struct Expected<'a> {
    id: String,
    category: &'static str,
    pipe: &'a str,
    direction: &'static str,
    dimension: &'static str,
    provenance: String,
    /// The generated unit, where S2 states one (`K`); `None` where the unit is
    /// the copied authored quantity's own.
    unit: Option<&'static str>,
    /// The authored value S2 copies verbatim (added weight); `None` for a
    /// temperature difference, which is S2's output and is not recomputed.
    copied: Option<&'a Quantity>,
}

fn axis_token(axis: GlobalAxis) -> &'static str {
    match axis {
        GlobalAxis::GlobalX => "global_x",
        GlobalAxis::GlobalY => "global_y",
        GlobalAxis::GlobalZ => "global_z",
    }
}

fn expected_difference(load: &PreviewPrimitiveLoad, expected: &Expected) -> Option<&'static str> {
    if load.id != expected.id {
        Some("id")
    } else if load.category != expected.category {
        Some("category")
    } else if !matches!(&load.target, LoadTargetInput::Element { pipe } if pipe == expected.pipe) {
        Some("target")
    } else if load.direction != expected.direction {
        Some("direction")
    } else if load.dimension != expected.dimension {
        Some("dimension")
    } else if load.provenance.as_deref() != Some(expected.provenance.as_str()) {
        Some("provenance")
    } else if expected.unit.is_some_and(|unit| load.magnitude.unit != unit) {
        Some("magnitude.unit")
    } else if expected.copied.is_some_and(|authored| !quantity_eq(&load.magnitude, authored)) {
        Some("magnitude (S2 copies the set's added_weight verbatim)")
    } else {
        None
    }
}

impl<'a> ModalCaseContext<'a> {
    /// Captures the authored case named `state_case`, before lowering.
    pub(crate) fn capture(original: &'a PreviewModel, state_case: &str) -> Result<Self, Diagnostic> {
        let mut matching = original.load_cases.iter().enumerate().filter(|(_, case)| case.id == state_case);
        let (Some((case_index, case)), None) = (matching.next(), matching.next()) else {
            return Err(unavailable(state_case, "it does not name exactly one load case of the authored model".into()));
        };
        Ok(Self {
            original,
            case_index,
            condition_use: case.condition_use.clone(),
            authored_wind: wind_of(case),
        })
    }

    fn case(&self) -> &'a PreviewLoadCase {
        &self.original.load_cases[self.case_index]
    }

    /// The authored set `pipe` refers to, or the contradiction that refuses.
    fn set_of(&self, pipe: &crate::PreviewPipe, uses: &str) -> Result<&'a ConditionSetInput, String> {
        let Some(reference) = pipe.condition_set_ref.as_deref() else {
            return Err(format!("the case uses {uses} and pipe `{}` carries no condition set, which the resolution refuses; the lowered model is not its output", pipe.id));
        };
        let mut sets = self.original.condition_sets.iter().filter(|set| set.id == reference);
        match (sets.next(), sets.next()) {
            (Some(set), None) => Ok(set),
            _ => Err(format!("pipe `{}` refers to condition set `{reference}`, which does not name exactly one authored set", pipe.id)),
        }
    }

    /// The generated suffix S2 documents for this case: temperature records,
    /// then added-weight records, from authored presence facts.
    fn expected_suffix(&self) -> Result<(Vec<Expected<'a>>, Vec<Expected<'a>>), String> {
        let (mut temperatures, mut added_weights) = (Vec::new(), Vec::new());
        let Some(using) = self.condition_use.as_ref() else {
            return Ok((temperatures, added_weights));
        };
        let case = self.case().id.as_str();
        if let Some(key) = &using.pressure {
            return Err(format!("the case uses condition `{key}` for pressure, which the pressure-free resolution refuses"));
        }
        if let Some(key) = &using.contents_density {
            return Err(format!("the case uses condition `{key}` for contents density, which the resolution refuses until its consumer lands"));
        }
        for pipe in &self.original.pipe_segments {
            if let Some(key) = &using.temperature {
                let set = self.set_of(pipe, "temperature")?;
                let stated = set.conditions.iter().flatten().any(|condition| &condition.key == key && condition.temperature.is_some());
                if !stated || set.installation_temperature.is_none() {
                    return Err(format!("condition set `{}` of pipe `{}` does not state both condition `{key}`'s temperature and an installation_temperature, which the resolution refuses", set.id, pipe.id));
                }
                temperatures.push(Expected {
                    id: temperature_load_id(case, &pipe.id),
                    category: "thermal",
                    pipe: &pipe.id,
                    direction: THERMAL_DIRECTION,
                    dimension: "temperature_change",
                    provenance: temperature_provenance(&set.id, key),
                    unit: Some("K"),
                    copied: None,
                });
            }
            if using.added_weight == Some(true) {
                let set = self.set_of(pipe, "added_weight")?;
                if let Some(weight) = &set.added_weight {
                    added_weights.push(Expected {
                        id: added_weight_load_id(case, &pipe.id),
                        category: "weight",
                        pipe: &pipe.id,
                        direction: axis_token(weight.direction),
                        dimension: "force_per_length",
                        provenance: added_weight_provenance(&set.id),
                        unit: None,
                        copied: Some(&weight.magnitude),
                    });
                }
            }
        }
        Ok((temperatures, added_weights))
    }

    /// The wind exposure the lowered case must carry.
    fn expected_wind(&self) -> Result<WindPresence, String> {
        let uses = self.condition_use.as_ref().and_then(|using| using.wind_exposure) == Some(true);
        if !uses {
            return Ok(self.authored_wind.clone());
        }
        if self.authored_wind != WindPresence::Unstated {
            return Err("the case uses wind_exposure but its authored wind input is absent or authors exposed_pipe_refs, which the resolution refuses".into());
        }
        let mut refs = Vec::new();
        for pipe in &self.original.pipe_segments {
            let set = self.set_of(pipe, "wind_exposure")?;
            if set.exposure.as_ref().and_then(|exposure| exposure.wind) == Some(true) {
                refs.push(pipe.id.clone());
            }
        }
        Ok(WindPresence::Resolved(refs))
    }

    /// Binds the captured case to `lowered`, the model after the one lowering
    /// (module docs). `lowered` must not yet have passed a stage that rewrites
    /// a primitive load (unit normalization does), so the prefix compares bit
    /// for bit.
    pub(crate) fn bind(&self, lowered: &PreviewModel) -> Result<BoundCase, Diagnostic> {
        let original = self.case();
        let id = original.id.as_str();
        let refuse = |reason: String| unavailable(id, reason);
        // Evidence that the one lowering ran, and that it kept the case's place.
        if lowered.load_cases.len() != self.original.load_cases.len() {
            return Err(refuse(format!("the lowered model has {} load cases, the authored one {}", lowered.load_cases.len(), self.original.load_cases.len())));
        }
        let case = &lowered.load_cases[self.case_index];
        if case.id != original.id {
            return Err(refuse(format!("load case {} of the lowered model is `{}`, not the state case", self.case_index, case.id)));
        }
        if case.condition_use.is_some() || !lowered.condition_sets.is_empty() || lowered.pipe_segments.iter().any(|pipe| pipe.condition_set_ref.is_some()) {
            return Err(refuse("the model given as lowered still carries operating-condition members, so the lowering has not run on it".into()));
        }
        let lowered_pipes: Vec<&str> = lowered.pipe_segments.iter().map(|pipe| pipe.id.as_str()).collect();
        let authored_pipes: Vec<&str> = self.original.pipe_segments.iter().map(|pipe| pipe.id.as_str()).collect();
        if lowered_pipes != authored_pipes {
            return Err(refuse("the lowered model's pipe segments are not the authored ones in authored order".into()));
        }
        // 2. The authored prefix, by value; the rest of the case unchanged.
        let authored = original.primitive_loads.len();
        if case.primitive_loads.len() < authored {
            return Err(refuse(format!("the lowered case has {} primitive loads, fewer than its {authored} authored ones", case.primitive_loads.len())));
        }
        for (index, (a, b)) in original.primitive_loads.iter().zip(&case.primitive_loads).enumerate() {
            if let Some(field) = primitive_load_difference(a, b) {
                return Err(refuse(format!("authored primitive load {index} (`{}`) differs in its {field} after lowering", a.id)));
            }
        }
        if rest_of_case(original) != rest_of_case(case) {
            return Err(refuse("a member of the case other than its primitive loads, condition_use and wind exposure changed in lowering".into()));
        }
        // 3. The generated suffix, exactly.
        let (temperatures, added_weights) = self.expected_suffix().map_err(&refuse)?;
        let suffix = &case.primitive_loads[authored..];
        let expected: Vec<&Expected> = temperatures.iter().chain(&added_weights).collect();
        if suffix.len() != expected.len() {
            return Err(refuse(format!(
                "the lowered case appends {} records after its authored loads; the resolution documents {} ({} temperature, {} added weight)",
                suffix.len(),
                expected.len(),
                temperatures.len(),
                added_weights.len()
            )));
        }
        for (offset, (load, wanted)) in suffix.iter().zip(&expected).enumerate() {
            if let Some(field) = expected_difference(load, wanted) {
                return Err(refuse(format!(
                    "generated record {} (`{}`) is not the resolution's record `{}` for pipe `{}`: its {field} differs",
                    authored + offset,
                    load.id,
                    wanted.id,
                    wanted.pipe
                )));
            }
        }
        // 4. The wind exposure.
        let wind = self.expected_wind().map_err(&refuse)?;
        let lowered_wind = case.equivalent_static.as_ref().and_then(|generation| generation.wind.as_ref());
        let agrees = match (&wind, lowered_wind) {
            (WindPresence::Absent, None) => true,
            (WindPresence::Unstated, Some(lowered)) => !lowered.authors_exposed_pipe_refs(),
            (WindPresence::Authored(refs), Some(lowered)) => {
                lowered.authors_exposed_pipe_refs() && &lowered.exposed_pipe_refs == refs
            }
            (WindPresence::Resolved(refs), Some(lowered)) => {
                lowered.exposed_pipe_refs_authored && &lowered.exposed_pipe_refs == refs
            }
            _ => false,
        };
        if !agrees {
            return Err(refuse(format!("the lowered wind exposure is not the expected {wind:?}")));
        }
        let temperature_end = authored + temperatures.len();
        Ok(BoundCase {
            case_index: self.case_index,
            case_id: id.to_string(),
            authored: 0..authored,
            generated_temperatures: authored..temperature_end,
            generated_added_weights: temperature_end..temperature_end + added_weights.len(),
            wind,
            condition_use_authored: self.condition_use.is_some(),
        })
    }
}
