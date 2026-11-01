//! S2: the pressure-free (PF) resolution of a load case's `condition_use`
//! (DEL-02-01 Design `model-document-0.9.0.md` §1.5, §1.5.1, §1.7).
//!
//! [`plan`] reads the model and resolves every PF case that uses conditions into
//! the records the solve already consumes: `thermal` primitives in K from the
//! exact temperature difference, `weight` primitives copied from each set's
//! `added_weight`, and the case's wind exposure from `exposure.wind`. It never
//! mutates the model, so a later preview can show exactly what the solve
//! applies; [`ResolutionPlan::apply`] appends what the plan holds and nothing
//! else. A case the resolution cannot complete is refused by name (§1.3's
//! case-level codes), naming the case, pipe, set, key and quantity; nothing is
//! skipped and nothing is synthesized.
//!
//! Interim refusals stay (§3.8): `contents_density` (no consumer yet), a
//! `pressure` use on a pressure-free document (no carrier) and, on the exact
//! route, any use at all (S4; `super::lower` refuses those before planning).
use super::{ConditionSetInput, GlobalAxis};
use crate::case_state::temperature::canonical_decimal::{
    canonical_decimal, kelvin_numerator, round_kelvin_ratio,
};
use crate::{Diagnostic, LoadTargetInput, PreviewLoadCase, PreviewModel, PreviewPrimitiveLoad, Quantity};
use open_pipe_stress_units::{unit_by_symbol, Dimension};
use std::collections::HashMap;

/// The case uses a quantity and a pipe carries no set.
pub const OPERATING_CONDITION_SET_MISSING: &str = "OPERATING_CONDITION_SET_MISSING";
/// The pipe's set has no condition of the used key.
pub const OPERATING_CONDITION_KEY_MISSING: &str = "OPERATING_CONDITION_KEY_MISSING";
/// The used condition lacks the used quantity.
pub const OPERATING_CONDITION_VALUE_MISSING: &str = "OPERATING_CONDITION_VALUE_MISSING";
/// PF temperature use on a pipe whose set has no `installation_temperature`.
pub const OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING: &str =
    "OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING";
/// The case uses `temperature` and carries a `thermal` primitive (§1.7).
pub const LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT: &str = "LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT";
/// The case uses `wind_exposure` and authors `exposed_pipe_refs` or `exposed_spans`.
pub const LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT: &str = "LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT";
/// The case uses `wind_exposure` and has no `equivalent_static.wind`.
pub const LOAD_CASE_CONDITION_WIND_INPUT_MISSING: &str = "LOAD_CASE_CONDITION_WIND_INPUT_MISSING";
/// A `pressure` use on a pressure-free document, which has no pressure carrier.
pub const LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT: &str =
    "LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT";
/// Interim (§1.6, OC-7): a `contents_density` use; its consumer is not designed.
pub const LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED: &str = "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED";

/// The `method` of every generated record's provenance.
pub const RESOLUTION_METHOD: &str = "operating_condition_set_v1";
/// The token the desktop writes for thermal primitives (§1.5 step 1); it changes
/// neither the thermal magnitude nor its sign.
pub const THERMAL_DIRECTION: &str = "global_z";

/// The generated thermal primitive's id for (case, pipe), length-prefixed so
/// that distinct pairs never share an id.
pub fn temperature_load_id(case: &str, pipe: &str) -> String {
    format!("load:condition-temperature:{}:{case}:{}:{pipe}", case.len(), pipe.len())
}

/// The generated added-weight primitive's id for (case, pipe).
pub fn added_weight_load_id(case: &str, pipe: &str) -> String {
    format!("load:condition-added-weight:{}:{case}:{}:{pipe}", case.len(), pipe.len())
}

/// JCS (RFC 8785) text of a string: JSON string escaping as ECMAScript's
/// `JSON.stringify` writes it, which is serde_json's (short escapes for
/// `\b \t \n \f \r`, `\u00xx` in lower case for other controls, nothing else).
fn jcs_string(text: &str) -> String {
    serde_json::to_string(text).expect("a string serializes")
}

/// §1.5: the provenance of a generated thermal primitive, members in JCS order.
pub fn temperature_provenance(set: &str, key: &str) -> String {
    format!(
        "{{\"condition\":{},\"condition_set\":{},\"method\":\"{RESOLUTION_METHOD}\",\"quantity\":\"temperature\"}}",
        jcs_string(key),
        jcs_string(set)
    )
}

/// §1.5: the provenance of a generated added-weight primitive.
pub fn added_weight_provenance(set: &str) -> String {
    format!(
        "{{\"condition_set\":{},\"method\":\"{RESOLUTION_METHOD}\",\"quantity\":\"added_weight\"}}",
        jcs_string(set)
    )
}

/// Why an exact temperature difference was not formed. Document-level
/// admission refuses each of these first, so the resolution meets them only if
/// that order is broken; it then refuses by name rather than guessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DifferenceError {
    /// A unit outside the catalog's absolute temperatures.
    Unit,
    /// A non-finite value, or one outside §1.5.1's domain.
    Range,
}

/// §1.5.1: ΔT = RN(K(T) − K(T₀)) in kelvin, from each operand's canonical
/// decimal under its unit's exact affine definition; an exact zero is +0.0.
pub(crate) fn exact_temperature_difference(
    temperature: &Quantity,
    installation: &Quantity,
) -> Result<f64, DifferenceError> {
    let numerator = |quantity: &Quantity| -> Result<i128, DifferenceError> {
        let unit = unit_by_symbol(&quantity.unit, Dimension::Temperature)
            .map_err(|_| DifferenceError::Unit)?;
        let decimal = canonical_decimal(quantity.value).map_err(|_| DifferenceError::Range)?;
        kelvin_numerator(decimal, unit).ok_or(DifferenceError::Unit)
    };
    Ok(round_kelvin_ratio(numerator(temperature)? - numerator(installation)?))
}

/// One resolved case: what the solve appends to it, in §1.5's order.
#[derive(Debug, Clone)]
pub struct CaseResolution {
    /// Index of the case in `load_cases`.
    pub case_index: usize,
    pub case_id: String,
    /// Generated thermal primitives, in `pipe_segments` order.
    pub temperatures: Vec<PreviewPrimitiveLoad>,
    /// Generated added-weight primitives, in `pipe_segments` order.
    pub added_weights: Vec<PreviewPrimitiveLoad>,
    /// The resolved `equivalent_static.wind.exposed_pipe_refs`, when the case
    /// uses `wind_exposure`; possibly empty.
    pub wind_exposed_pipe_refs: Option<Vec<String>>,
}

/// The PF resolution of every case that uses conditions, computed without
/// touching the model. Cases appear in `load_cases` order.
#[derive(Debug, Clone, Default)]
pub struct ResolutionPlan {
    pub cases: Vec<CaseResolution>,
}

impl ResolutionPlan {
    /// Appends the plan to the model it was computed from: generated primitives
    /// after the authored ones (temperature, then added weight), and the case's
    /// wind exposure.
    pub(crate) fn apply(self, model: &mut PreviewModel) {
        for resolved in self.cases {
            let case = &mut model.load_cases[resolved.case_index];
            debug_assert_eq!(case.id, resolved.case_id);
            case.primitive_loads.extend(resolved.temperatures);
            case.primitive_loads.extend(resolved.added_weights);
            if let Some(refs) = resolved.wind_exposed_pipe_refs {
                let wind = case
                    .equivalent_static
                    .as_mut()
                    .and_then(|generation| generation.wind.as_mut())
                    .expect("planned only with an authored equivalent_static.wind");
                wind.exposed_pipe_refs = refs;
            }
        }
    }
}

fn case_diag(case: &str, suffix: &str, code: &'static str, message: String, refs: Vec<String>) -> Diagnostic {
    crate::diag(
        &format!("diagnostic:load-case:{}:condition:{suffix}", crate::stable_suffix(case)),
        code,
        "blocking",
        message,
        refs,
    )
}

fn axis_token(axis: GlobalAxis) -> &'static str {
    match axis {
        GlobalAxis::GlobalX => "global_x",
        GlobalAxis::GlobalY => "global_y",
        GlobalAxis::GlobalZ => "global_z",
    }
}

/// §1.5 over one PF case that uses conditions: its resolution, or every
/// case-level refusal it meets, in a fixed order (case-wide conflicts and
/// interim uses first, then each pipe in `pipe_segments` order).
fn resolve_case(
    model: &PreviewModel,
    sets: &HashMap<&str, &ConditionSetInput>,
    case_index: usize,
    case: &PreviewLoadCase,
) -> Result<Option<CaseResolution>, Vec<Diagnostic>> {
    let Some(using) = case.condition_use.as_ref() else {
        return Ok(None);
    };
    let id = case.id.as_str();
    let mut refusals = Vec::new();
    if let Some(key) = &using.pressure {
        refusals.push(case_diag(id, "pressure", LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT,
            format!("load case `{id}` uses condition `{key}` for pressure on a pressure-free document, which has no pressure carrier; pressure from conditions needs a pressure contract, so nothing is solved"),
            vec![id.to_string(), key.clone()]));
    }
    if let Some(key) = &using.contents_density {
        refusals.push(case_diag(id, "contents-density", LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED,
            format!("load case `{id}` uses condition `{key}` for contents density; the resolution of contents density is not designed yet, so nothing is solved"),
            vec![id.to_string(), key.clone()]));
    }
    if using.temperature.is_some() {
        for load in case.primitive_loads.iter().filter(|load| load.category == "thermal") {
            refusals.push(case_diag(id, &format!("temperature-conflict:{}", crate::stable_suffix(&load.id)),
                LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT,
                format!("load case `{id}` takes its temperature from conditions and also carries thermal primitive `{}`; a case takes temperature from one source only, so nothing is solved", load.id),
                vec![id.to_string(), load.id.clone()]));
        }
    }
    let wind_uses = using.wind_exposure == Some(true);
    if wind_uses {
        match case.equivalent_static.as_ref().and_then(|generation| generation.wind.as_ref()) {
            None => refusals.push(case_diag(id, "wind-input", LOAD_CASE_CONDITION_WIND_INPUT_MISSING,
                format!("load case `{id}` takes its wind exposure from conditions but has no equivalent_static.wind input; nothing is defaulted, so nothing is solved"),
                vec![id.to_string()])),
            Some(wind) => {
                for (member, authored) in [("exposed_pipe_refs", !wind.exposed_pipe_refs.is_empty()), ("exposed_spans", !wind.exposed_spans.is_empty())] {
                    if authored {
                        refusals.push(case_diag(id, &format!("wind-conflict:{member}"), LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT,
                            format!("load case `{id}` takes its wind exposure from conditions and also authors equivalent_static.wind.{member}; a case takes wind exposure from one source only, so nothing is solved"),
                            vec![id.to_string(), member.to_string()]));
                    }
                }
            }
        }
    }
    // Each set-level or keyed use the case states, for the per-pipe rules.
    let mut quantities = Vec::new();
    if using.temperature.is_some() {
        quantities.push("temperature");
    }
    if using.added_weight == Some(true) {
        quantities.push("added_weight");
    }
    if wind_uses {
        quantities.push("wind_exposure");
    }
    let mut resolved = CaseResolution {
        case_index,
        case_id: id.to_string(),
        temperatures: Vec::new(),
        added_weights: Vec::new(),
        wind_exposed_pipe_refs: wind_uses.then(Vec::new),
    };
    for pipe in &model.pipe_segments {
        let pipe_id = pipe.id.as_str();
        let Some(set_id) = pipe.condition_set_ref.as_deref() else {
            for quantity in &quantities {
                refusals.push(case_diag(id, &format!("{}:{}:set-missing", quantity.replace('_', "-"), crate::stable_suffix(pipe_id)),
                    OPERATING_CONDITION_SET_MISSING,
                    format!("load case `{id}` uses {quantity} from conditions and pipe `{pipe_id}` carries no condition set; no pipe is skipped and no value is assumed, so nothing is solved"),
                    vec![id.to_string(), pipe_id.to_string(), quantity.to_string()]));
            }
            continue;
        };
        // Document-level admission resolves every reference first; if that order
        // is ever broken, the reference is refused here rather than skipped.
        let Some(&set) = sets.get(set_id) else {
            refusals.push(case_diag(id, &format!("set-unresolved:{}", crate::stable_suffix(pipe_id)), super::OPERATING_CONDITION_SET_UNRESOLVED,
                format!("load case `{id}` uses conditions and condition_set_ref `{set_id}` of pipe `{pipe_id}` names no condition set; nothing is solved"),
                vec![id.to_string(), pipe_id.to_string(), set_id.to_string()]));
            continue;
        };
        if let Some(key) = &using.temperature {
            let at = |what: &str| format!("temperature:{}:{what}", crate::stable_suffix(pipe_id));
            let condition = set.conditions.iter().flatten().find(|condition| &condition.key == key);
            let mut temperature = None;
            match condition {
                None => refusals.push(case_diag(id, &at("key-missing"), OPERATING_CONDITION_KEY_MISSING,
                    format!("load case `{id}` uses condition `{key}` for temperature and condition set `{set_id}` of pipe `{pipe_id}` has no condition `{key}`; nothing is solved"),
                    vec![id.to_string(), pipe_id.to_string(), set_id.to_string(), key.clone(), "temperature".into()])),
                Some(condition) => match &condition.temperature {
                    None => refusals.push(case_diag(id, &at("value-missing"), OPERATING_CONDITION_VALUE_MISSING,
                        format!("load case `{id}` uses condition `{key}` for temperature and condition `{key}` of set `{set_id}` (pipe `{pipe_id}`) states no temperature; no temperature is assumed, so nothing is solved"),
                        vec![id.to_string(), pipe_id.to_string(), set_id.to_string(), key.clone(), "temperature".into()])),
                    Some(value) => temperature = Some(value),
                },
            }
            let installation = set.installation_temperature.as_ref();
            if installation.is_none() {
                refusals.push(case_diag(id, &at("installation-missing"), OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING,
                    format!("load case `{id}` uses condition `{key}` for temperature and condition set `{set_id}` of pipe `{pipe_id}` states no installation_temperature; no installation temperature is assumed, so nothing is solved"),
                    vec![id.to_string(), pipe_id.to_string(), set_id.to_string(), key.clone(), "installation_temperature".into()]));
            }
            if let (Some(temperature), Some(installation)) = (temperature, installation) {
                match exact_temperature_difference(temperature, installation) {
                    Ok(difference) => resolved.temperatures.push(PreviewPrimitiveLoad {
                        id: temperature_load_id(id, pipe_id),
                        category: "thermal".into(),
                        target: LoadTargetInput::Element { pipe: pipe_id.to_string() },
                        direction: THERMAL_DIRECTION.into(),
                        magnitude: Quantity { value: difference, unit: "K".into() },
                        dimension: "temperature_change".into(),
                        provenance: Some(temperature_provenance(set_id, key)),
                    }),
                    Err(error) => {
                        let (code, reason) = match error {
                            DifferenceError::Unit => (super::OPERATING_CONDITION_QUANTITY_INVALID, "an operand is not in an absolute-temperature unit of the units catalog"),
                            DifferenceError::Range => (super::OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED, "an operand is outside the exact-arithmetic range of absolute temperatures"),
                        };
                        refusals.push(case_diag(id, &at("difference"), code,
                            format!("load case `{id}`: the temperature difference of condition `{key}` of set `{set_id}` (pipe `{pipe_id}`) is not formed: {reason}; nothing is solved"),
                            vec![id.to_string(), pipe_id.to_string(), set_id.to_string(), key.clone(), "temperature".into()]));
                    }
                }
            }
        }
        if using.added_weight == Some(true) {
            // Absence of a set's added weight means none: no record, no zero.
            if let Some(weight) = &set.added_weight {
                resolved.added_weights.push(PreviewPrimitiveLoad {
                    id: added_weight_load_id(id, pipe_id),
                    category: "weight".into(),
                    target: LoadTargetInput::Element { pipe: pipe_id.to_string() },
                    direction: axis_token(weight.direction).into(),
                    magnitude: weight.magnitude.clone(),
                    dimension: "force_per_length".into(),
                    provenance: Some(added_weight_provenance(set_id)),
                });
            }
        }
        if let Some(refs) = resolved.wind_exposed_pipe_refs.as_mut() {
            if set.exposure.as_ref().and_then(|exposure| exposure.wind) == Some(true) {
                refs.push(pipe_id.to_string());
            }
        }
    }
    if refusals.is_empty() {
        Ok(Some(resolved))
    } else {
        Err(refusals)
    }
}

/// §1.5 over every PF case that uses conditions, in `load_cases` order: the
/// plan, or each case's refusals. The model is read, never changed. The caller
/// has judged the document-level rules and is on the pressure-free route.
pub(crate) fn plan(model: &PreviewModel) -> Result<ResolutionPlan, Vec<Diagnostic>> {
    let sets: HashMap<&str, &ConditionSetInput> =
        model.condition_sets.iter().map(|set| (set.id.as_str(), set)).collect();
    let mut planned = ResolutionPlan::default();
    let mut refusals = Vec::new();
    for (index, case) in model.load_cases.iter().enumerate() {
        match resolve_case(model, &sets, index, case) {
            Ok(Some(resolved)) => planned.cases.push(resolved),
            Ok(None) => {}
            Err(found) => refusals.extend(found),
        }
    }
    if refusals.is_empty() {
        Ok(planned)
    } else {
        Err(refusals)
    }
}

#[cfg(test)]
#[path = "resolution_tests.rs"]
mod tests;
