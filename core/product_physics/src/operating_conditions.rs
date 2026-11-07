//! Model document 0.9.0's operating-condition sets (DEL-02-01 Design
//! `model-document-0.9.0.md` §1): the closed records of `condition_sets[]`, a
//! load case's `condition_use` and its `test_condition` (§2.4), the
//! document-level admission rules of §1.3, and the lowering entry of §1.5.
//!
//! A set is a source of values; a case's use of it resolves into the per-case
//! records the solve already consumes. On the pressure-free route S2 resolves
//! temperature, added weight and wind exposure ([`resolution`]); a pressure use
//! stays refused by name there, and on a document with a pressure contract every
//! use other than contents density is refused
//! (`LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED`, until S4). A contents-density
//! use (RT-A6 H1, [`contents`]) is mass input on either route: it is admitted
//! when generated self-weight of the same case consumes it, and refused by name
//! (`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`) when none does. Every set and
//! reference is judged whether or not a case uses it, so a document's
//! admissibility does not depend on which cases use what.
//!
//! [`lower`] runs first in the solve. It applies the document-level rules of
//! every 0.9.0 member ([`crate::document_members`]), the interim refusals, and
//! then removes the members no later stage reads: `condition_sets`,
//! `condition_set_ref`, `condition_use`, `spectra`, and an exactly axis-aligned
//! support `direction` (its DOF token set to that axis, §2.1). Only when a case
//! uses contents does it keep the contents selection
//! ([`contents::RetainedContents`]) for the post-lowering self-weight
//! validation. Everything after it is today's pipeline applied to the lowered
//! model.
use crate::case_state::temperature::canonical_decimal::{canonical_decimal, kelvin_numerator, CanonicalError};
use crate::document_members::{Finding, MemberFindings};
use crate::{Diagnostic, PreviewModel, PreviewPrimitiveLoad, Quantity};
use open_pipe_stress_units::{convert, unit_by_symbol, Dimension, UnitId};
use serde::Deserialize;
use std::collections::HashSet;

pub mod contents;
pub mod resolution;

/// Two conditions in one set share a key.
pub const OPERATING_CONDITION_KEY_DUPLICATE: &str = "OPERATING_CONDITION_KEY_DUPLICATE";
/// A condition-set value is non-finite, in a unit of the wrong dimension or outside the
/// catalog, an absolute temperature below zero kelvin, or a negative contents density.
pub const OPERATING_CONDITION_QUANTITY_INVALID: &str = "OPERATING_CONDITION_QUANTITY_INVALID";
/// An absolute temperature of a set outside the exact-arithmetic range of §1.5.1.
pub const OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED: &str = "OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED";
/// A set's provenance is blank.
pub const OPERATING_CONDITION_PROVENANCE_MISSING: &str = "OPERATING_CONDITION_PROVENANCE_MISSING";
/// A pipe's `condition_set_ref` names no set.
pub const OPERATING_CONDITION_SET_UNRESOLVED: &str = "OPERATING_CONDITION_SET_UNRESOLVED";
/// An empty `condition_use`, or `false` in a `true`-only member.
pub const LOAD_CASE_CONDITION_USE_EMPTY: &str = "LOAD_CASE_CONDITION_USE_EMPTY";
/// Interim refusal of S1, removed from the solve by S2 (the PF resolution). The
/// applier still refuses authoring a `condition_use` with it until S3a.
pub const LOAD_CASE_CONDITION_USE_UNSUPPORTED: &str = "LOAD_CASE_CONDITION_USE_UNSUPPORTED";
/// Interim until S4: any `condition_use` on a document with `pressure_contract`.
pub const LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED: &str = "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED";
/// Interim until RT-A6's build (DEL-05-01 Design `hydrostatic-test.md` §2).
pub const LOAD_CASE_TEST_CONDITION_UNSUPPORTED: &str = "LOAD_CASE_TEST_CONDITION_UNSUPPORTED";

/// The entity name of a condition set in `EMPTY_ID` and `DUPLICATE_ID`.
pub const CONDITION_SET_ENTITY: &str = "condition-set";

/// One operating-condition set (closed). An authored `null` is refused for every
/// optional member: absence means none.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionSetInput {
    pub id: String,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub label: Option<String>,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub conditions: Option<Vec<OperatingConditionInput>>,
    #[serde(default, deserialize_with = "crate::model_fields::closed_present_quantity")]
    pub installation_temperature: Option<Quantity>,
    #[serde(default, deserialize_with = "crate::deserialize_present_object")]
    pub design: Option<ConditionValuesInput>,
    #[serde(default, deserialize_with = "crate::deserialize_present_object")]
    pub peak: Option<ConditionValuesInput>,
    #[serde(default, deserialize_with = "crate::deserialize_present_object")]
    pub added_weight: Option<AddedWeightInput>,
    #[serde(default, deserialize_with = "crate::deserialize_present_object")]
    pub exposure: Option<ConditionExposureInput>,
    pub provenance: String,
    #[serde(default)]
    pub tags: serde::de::IgnoredAny,
    #[serde(default)]
    pub notes: serde::de::IgnoredAny,
}

/// One keyed condition of a set (closed).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatingConditionInput {
    pub key: String,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub label: Option<String>,
    #[serde(default, deserialize_with = "crate::model_fields::closed_present_quantity")]
    pub temperature: Option<Quantity>,
    #[serde(default, deserialize_with = "crate::model_fields::closed_present_quantity")]
    pub pressure: Option<Quantity>,
    #[serde(default, deserialize_with = "crate::model_fields::closed_present_quantity")]
    pub contents_density: Option<Quantity>,
}

/// Design or peak values (closed); read by no solve in 0.9.0.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionValuesInput {
    #[serde(default, deserialize_with = "crate::model_fields::closed_present_quantity")]
    pub temperature: Option<Quantity>,
    #[serde(default, deserialize_with = "crate::model_fields::closed_present_quantity")]
    pub pressure: Option<Quantity>,
}

/// A set's uniform added weight (closed): what a user types into a weight primitive.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddedWeightInput {
    #[serde(deserialize_with = "crate::model_fields::closed_quantity")]
    pub magnitude: Quantity,
    pub direction: GlobalAxis,
}

/// The entered direction of an added weight; the model has no vertical axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlobalAxis {
    GlobalX,
    GlobalY,
    GlobalZ,
}

/// Exposure marks (closed); absent means not marked.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionExposureInput {
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub wind: Option<bool>,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub snow_ice: Option<bool>,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub wave: Option<bool>,
}

/// A load case's use of its pipes' sets (closed): condition keys, and the
/// set-level uses, which admit only `true`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionUseInput {
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub temperature: Option<String>,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub pressure: Option<String>,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub contents_density: Option<String>,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub added_weight: Option<bool>,
    #[serde(default, deserialize_with = "crate::deserialize_present_value")]
    pub wind_exposure: Option<bool>,
}

/// A load case's hydrostatic test condition (closed; DEL-05-01 Design
/// `hydrostatic-test.md` §2). Both members are entered; nothing is defaulted.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestConditionInput {
    pub kind: TestConditionKind,
    pub held_supports: HeldSupports,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestConditionKind {
    Hydrostatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeldSupports {
    SpringsAndConstantEffort,
    None,
}

/// What a set's quantity is, for its admission rule.
#[derive(Clone, Copy)]
enum ValueKind {
    AbsoluteTemperature,
    Pressure,
    ContentsDensity,
    ForcePerLength,
}

impl ValueKind {
    fn dimension(self) -> Dimension {
        match self {
            ValueKind::AbsoluteTemperature => Dimension::Temperature,
            ValueKind::Pressure => Dimension::Pressure,
            ValueKind::ContentsDensity => Dimension::Density,
            ValueKind::ForcePerLength => Dimension::ForcePerLength,
        }
    }

    fn what(self) -> &'static str {
        match self {
            ValueKind::AbsoluteTemperature => "an absolute temperature",
            ValueKind::Pressure => "a pressure",
            ValueKind::ContentsDensity => "a contents density",
            ValueKind::ForcePerLength => "a force per length",
        }
    }
}

/// Whether an absolute temperature lies below zero kelvin: exactly, from its
/// canonical decimal, inside the exact domain; by the binary64 conversion outside
/// it, where the sign is not in doubt.
fn below_zero_kelvin(value: f64, unit: UnitId) -> Result<bool, CanonicalError> {
    match canonical_decimal(value) {
        Ok(decimal) => Ok(kelvin_numerator(decimal, unit).is_some_and(|numerator| numerator < 0)),
        Err(CanonicalError::RangeUnsupported) => {
            let kelvin = convert(value, unit, UnitId::Kelvin).unwrap_or(f64::NAN);
            if kelvin < 0.0 {
                Ok(true)
            } else {
                Err(CanonicalError::RangeUnsupported)
            }
        }
        Err(error) => Err(error),
    }
}

/// §1.3's quantity rules, with normalization first (§1.5.1 "One implementation"):
/// a unit outside the catalog or of the wrong dimension, a non-finite value or one
/// below zero kelvin is `OPERATING_CONDITION_QUANTITY_INVALID` whatever its
/// magnitude; an otherwise valid absolute temperature outside the exact domain is
/// `OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED`.
fn quantity(quantity: &Quantity, kind: ValueKind, path: &str, refs: &[String], found: &mut MemberFindings) {
    let invalid = |reason: String| Finding::new(OPERATING_CONDITION_QUANTITY_INVALID, path, format!("{path} is not {}: {reason}.", kind.what()), refs.to_vec());
    let Ok(unit) = unit_by_symbol(&quantity.unit, kind.dimension()) else {
        found.push(invalid(format!("its unit `{}` is not a {} unit of the units catalog", quantity.unit, kind.dimension().as_str())));
        return;
    };
    if !quantity.value.is_finite() {
        found.push(invalid(format!("its value {} is not finite", quantity.value)));
        return;
    }
    match kind {
        ValueKind::AbsoluteTemperature => match below_zero_kelvin(quantity.value, unit) {
            Ok(true) => found.push(invalid(format!("{} {} is below zero kelvin", quantity.value, quantity.unit))),
            Ok(false) => {}
            Err(_) => found.push(Finding::new(
                OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED,
                path,
                format!("{path} is outside the exact-arithmetic range of absolute temperatures (|value| <= 1e6 in its authored unit, canonical decimal exponent >= -24)."),
                refs.to_vec(),
            )),
        },
        ValueKind::ContentsDensity if quantity.value < 0.0 => {
            found.push(invalid(format!("{} {} is negative", quantity.value, quantity.unit)));
        }
        _ => {}
    }
}

fn values(values: &ConditionValuesInput, path: &str, refs: &[String], found: &mut MemberFindings) {
    if let Some(temperature) = &values.temperature {
        quantity(temperature, ValueKind::AbsoluteTemperature, &format!("{path}.temperature"), refs, found);
    }
    if let Some(pressure) = &values.pressure {
        quantity(pressure, ValueKind::Pressure, &format!("{path}.pressure"), refs, found);
    }
}

/// §1.3's document-level rules over the sets, the pipes' references and the
/// cases' uses, in document order. `pipes` and `cases` are `(id, member)` in
/// collection order.
pub(crate) fn findings(
    sets: &[ConditionSetInput],
    pipes: &[(&str, Option<&str>)],
    cases: &[(&str, Option<&ConditionUseInput>)],
    found: &mut MemberFindings,
) {
    let mut seen = HashSet::new();
    let mut reported = HashSet::new();
    for (index, set) in sets.iter().enumerate() {
        let at = format!("condition_sets[{index}]");
        let refs = vec![set.id.clone()];
        if set.id.trim().is_empty() {
            found.push(Finding::empty_id(CONDITION_SET_ENTITY, &format!("{at}.id")));
        } else if !seen.insert(set.id.as_str()) && reported.insert(set.id.as_str()) {
            found.push(Finding::duplicate_id(CONDITION_SET_ENTITY, &set.id, &format!("{at}.id")));
        }
        if set.provenance.trim().is_empty() {
            let path = format!("{at}.provenance");
            found.push(Finding::new(OPERATING_CONDITION_PROVENANCE_MISSING, &path,
                format!("{path} is blank; a condition set states the source of its values."), refs.clone()));
        }
        let mut keys = HashSet::new();
        for (k, condition) in set.conditions.iter().flatten().enumerate() {
            let path = format!("{at}.conditions[{k}]");
            if !keys.insert(condition.key.as_str()) {
                found.push(Finding::new(OPERATING_CONDITION_KEY_DUPLICATE, &format!("{path}.key"),
                    format!("{path}.key `{}` repeats a key of condition set `{}`; keys are unique within a set.", condition.key, set.id),
                    vec![set.id.clone(), condition.key.clone()]));
            }
            if let Some(temperature) = &condition.temperature {
                quantity(temperature, ValueKind::AbsoluteTemperature, &format!("{path}.temperature"), &refs, found);
            }
            if let Some(pressure) = &condition.pressure {
                quantity(pressure, ValueKind::Pressure, &format!("{path}.pressure"), &refs, found);
            }
            if let Some(density) = &condition.contents_density {
                quantity(density, ValueKind::ContentsDensity, &format!("{path}.contents_density"), &refs, found);
            }
        }
        if let Some(temperature) = &set.installation_temperature {
            quantity(temperature, ValueKind::AbsoluteTemperature, &format!("{at}.installation_temperature"), &refs, found);
        }
        if let Some(design) = &set.design {
            values(design, &format!("{at}.design"), &refs, found);
        }
        if let Some(peak) = &set.peak {
            values(peak, &format!("{at}.peak"), &refs, found);
        }
        if let Some(weight) = &set.added_weight {
            quantity(&weight.magnitude, ValueKind::ForcePerLength, &format!("{at}.added_weight.magnitude"), &refs, found);
        }
    }
    let set_ids: HashSet<&str> = sets.iter().map(|set| set.id.as_str()).collect();
    for (index, (pipe, reference)) in pipes.iter().enumerate() {
        let Some(reference) = reference else { continue };
        if !set_ids.contains(reference) {
            let path = format!("pipe_segments[{index}].condition_set_ref");
            found.push(Finding::new(OPERATING_CONDITION_SET_UNRESOLVED, &path,
                format!("{path} `{reference}` of pipe `{pipe}` names no condition set."),
                vec![pipe.to_string(), reference.to_string()]));
        }
    }
    for (index, (case, using)) in cases.iter().enumerate() {
        let Some(using) = using else { continue };
        let path = format!("load_cases[{index}].condition_use");
        let refs = vec![case.to_string()];
        let members = [
            using.temperature.is_some(),
            using.pressure.is_some(),
            using.contents_density.is_some(),
            using.added_weight.is_some(),
            using.wind_exposure.is_some(),
        ];
        if !members.contains(&true) {
            found.push(Finding::new(LOAD_CASE_CONDITION_USE_EMPTY, &path,
                format!("{path} of load case `{case}` is empty; it names at least one use, and absence means none."), refs.clone()));
        }
        for (member, value) in [("added_weight", using.added_weight), ("wind_exposure", using.wind_exposure)] {
            if value == Some(false) {
                let at = format!("{path}.{member}");
                found.push(Finding::new(LOAD_CASE_CONDITION_USE_EMPTY, &at,
                    format!("{at} of load case `{case}` is false; it admits only true, and absence means not used."), refs.clone()));
            }
        }
    }
}

/// What the lowering decides about a model, without changing it: the
/// diagnostics it adds, whether a document-level rule refused the document, and
/// the PF resolution plan, present only when the lowering refuses nothing (so a
/// refused document never carries part of the generated records). [`lower`]
/// (the solve and its pre-formation checks) and [`preview_operating_conditions`]
/// (the read-only preview) both take this one judgement, so they cannot differ.
pub(crate) struct Judgement {
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) document_refused: bool,
    pub(crate) plan: Option<resolution::ResolutionPlan>,
}

/// §1.3 and §1.5 on a model it reads only: the document-level rules of every
/// 0.9.0 member, the PF resolution of each case's `condition_use` (S2,
/// [`resolution`]) and the remaining interim refusals.
pub(crate) fn judge(model: &PreviewModel) -> Judgement {
    let found = crate::document_members::model_findings(model);
    if !found.is_empty() {
        return Judgement {
            diagnostics: found.into_vec().into_iter().map(|finding| finding.into_diagnostic("blocking")).collect(),
            document_refused: true,
            plan: None,
        };
    }
    let mut diagnostics = Vec::new();
    let exact = model.pressure_contract.is_some();
    // The PF resolution (§1.5), read-only; the exact route's is S4's.
    let planned = if exact {
        None
    } else {
        match resolution::plan(model) {
            Ok(planned) => Some(planned),
            Err(refusals) => {
                diagnostics.extend(refusals);
                None
            }
        }
    };
    for case in &model.load_cases {
        if let (true, Some(using)) = (exact, case.condition_use.as_ref()) {
            // RT-A6 H1: contents density alone is admitted on the exact route
            // (its consumer is the generated self-weight, as on PF); every
            // other use waits for S4.
            let other = using.temperature.is_some() || using.pressure.is_some() || using.added_weight.is_some() || using.wind_exposure.is_some();
            if other {
                diagnostics.push(crate::diag(&format!("diagnostic:load-case:{}:condition-use", crate::stable_suffix(&case.id)),
                    LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED, "blocking",
                    format!("load case `{}` uses operating conditions on a document with a pressure contract; the exact route's condition resolution is not built yet, so nothing is solved", case.id),
                    vec![case.id.clone()]));
            } else if let Some(refusal) = contents::unconsumed_refusal(case) {
                diagnostics.push(refusal);
            }
        }
        if case.test_condition.is_some() {
            diagnostics.push(crate::diag(
                &format!("diagnostic:load-case:{}:test-condition", crate::stable_suffix(&case.id)),
                LOAD_CASE_TEST_CONDITION_UNSUPPORTED,
                "blocking",
                format!("load case `{}` is a hydrostatic test condition; the test-case mechanics (held supports, test contents and pressure) are not built yet, so nothing is solved", case.id),
                vec![case.id.clone()],
            ));
        }
    }
    crate::document_members::interim_support_refusals(model, &mut diagnostics);
    crate::document_members::interim_exact_combination_refusals(model, &mut diagnostics);
    let plan = planned.filter(|_| !crate::has_blocking(&diagnostics));
    Judgement { diagnostics, document_refused: false, plan }
}

/// §1.5: the lowering. It runs first in the solve, before `validate_profile`:
/// it takes [`judge`]'s diagnostics, applies its plan when there is one, and then
/// removes every member no later stage reads. A document refused by a
/// document-level rule keeps its members; nothing after the refusal solves it.
pub(crate) fn lower(model: &mut PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    let judged = judge(model);
    diagnostics.extend(judged.diagnostics);
    if judged.document_refused {
        return;
    }
    if let Some(planned) = judged.plan {
        planned.apply(model);
    }
    // RT-A6 H1: the contents selection, taken before the clear (module docs).
    let retained = contents::RetainedContents::take(model);
    // The members no later stage reads.
    model.condition_sets.clear();
    model.spectra.clear();
    for pipe in &mut model.pipe_segments {
        pipe.condition_set_ref = None;
    }
    for case in &mut model.load_cases {
        case.condition_use = None;
    }
    // Only the contents selection returns, for the self-weight validation that
    // follows; a document with no contents use keeps nothing.
    if let Some(retained) = retained {
        retained.restore(model);
    }
    crate::document_members::lower_axis_aligned_directions(model);
}

/// The `document_kind` of [`preview_operating_conditions`]'s report.
pub const OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND: &str = "openpipestress.operating_condition_preview";

fn generated_record(load: &PreviewPrimitiveLoad) -> serde_json::Value {
    let target = match &load.target {
        crate::LoadTargetInput::Element { pipe } => serde_json::json!({"type": "element", "pipe": pipe}),
        // The resolution generates element records only.
        other => serde_json::json!(format!("{other:?}")),
    };
    serde_json::json!({
        "id": load.id, "category": load.category, "target": target, "direction": load.direction,
        "magnitude": {"value": load.magnitude.value, "unit": load.magnitude.unit},
        "dimension": load.dimension, "provenance": load.provenance,
    })
}

/// The read-only preview of a model document's operating-condition resolution
/// (DEL-02-01 Design `model-document-0.9.0.md` §4.2): the document is read
/// through the solve's own request parse, judged by the lowering's own
/// [`judge`], and reported without changing it. With `resolved: true` the
/// cases list exactly the records the solve's lowering appends, in its order
/// (generated thermal, then added-weight records, and the resolved wind-exposed
/// pipe ids). A valid document with no condition use resolves to a no-op on
/// either route. The solve's namespace checks run before judgement here, so
/// retired/unsupported versions, wrong document kinds and invalid pressure
/// contracts cannot report a successful resolution. A refused preview lists no
/// generated records and names its diagnostics, or the request parse's text in
/// `refusal`. Later physical validation and formation do not run; `resolved`
/// reports condition resolution, never that the model can solve.
pub fn preview_operating_conditions(model_document: &serde_json::Value) -> serde_json::Value {
    let request = serde_json::json!({"model": model_document, "materials": []});
    let refusal = |text: String| serde_json::json!({
        "document_kind": OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND, "resolved": false, "refusal": text,
        "cases": [], "diagnostics": [],
    });
    if let Some(text) = crate::model_fields::unknown_field_refusal(&request) {
        return refusal(text);
    }
    let parsed = match crate::source_receipt::CapturedInvocation::parse(request, crate::PreviewSolverMode::default()) {
        Ok((parsed, _)) => parsed,
        Err(error) => return refusal(error.0),
    };
    // Parsing establishes the wire shape, not the current model namespace.
    // Reuse the solve's version/pressure-contract checks; this does not move
    // their position in the solve or change any existing mechanics result.
    let mut namespace = Vec::new();
    crate::pressure_runtime::validate_profile(&parsed.model, &mut namespace);
    if parsed.model.document_kind != "openpipestress.product_preview.model" {
        namespace.push(crate::diag(
            "diagnostic:physics:document-kind", "PREVIEW_DOCUMENT_KIND_INVALID", "blocking",
            "physics adapter requires an openpipestress.product_preview.model document",
            vec!["model".to_string()],
        ));
    }
    if parsed.model.schema_version.is_empty() {
        namespace.push(crate::diag(
            "diagnostic:physics:schema-version", "PREVIEW_SCHEMA_VERSION_MISSING", "blocking",
            "preview model requires an explicit schema version", vec!["model".to_string()],
        ));
    }
    if crate::has_blocking(&namespace) {
        return serde_json::json!({
            "document_kind": OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND,
            "resolved": false, "cases": [], "diagnostics": namespace,
        });
    }
    let judged = judge(&parsed.model);
    let cases: Vec<serde_json::Value> = judged.plan.iter().flat_map(|plan| plan.cases.iter()).map(|case| serde_json::json!({
        "case_id": case.case_id,
        "generated_primitive_loads": case.temperatures.iter().chain(&case.added_weights).map(generated_record).collect::<Vec<_>>(),
        "wind_exposed_pipe_refs": case.wind_exposed_pipe_refs,
    })).collect();
    serde_json::json!({
        "document_kind": OPERATING_CONDITION_PREVIEW_DOCUMENT_KIND,
        "resolved": !judged.document_refused && !crate::has_blocking(&judged.diagnostics),
        "cases": cases,
        "diagnostics": judged.diagnostics,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(value: f64, unit: &str) -> Quantity {
        Quantity { value, unit: unit.into() }
    }

    fn codes(set: ConditionSetInput) -> Vec<&'static str> {
        let mut found = MemberFindings::default();
        findings(&[set], &[], &[], &mut found);
        found.into_vec().into_iter().map(|finding| finding.code).collect()
    }

    fn set_with(temperature: Quantity) -> ConditionSetInput {
        let mut set: ConditionSetInput = serde_json::from_value(serde_json::json!({
            "id": "cs", "provenance": "user",
            "conditions": [{"key": "OPE", "temperature": {"value": 0.0, "unit": "K"}}],
        }))
        .unwrap();
        // Set after the decode: a non-finite value has no JSON form.
        set.conditions.as_mut().unwrap()[0].temperature = Some(temperature);
        set
    }

    #[test]
    fn temperatures_take_normalization_first_then_the_exact_range() {
        assert!(codes(set_with(q(350.1, "degC"))).is_empty());
        assert!(codes(set_with(q(1.0e6, "K"))).is_empty());
        assert!(codes(set_with(q(1e-24, "K"))).is_empty());
        assert_eq!(codes(set_with(q(1.5e-24, "K"))), [OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED]);
        assert_eq!(codes(set_with(q(1.000001e6, "K"))), [OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED]);
        assert_eq!(codes(set_with(q(1e-310, "K"))), [OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED]);
        // Below zero kelvin is invalid whatever its magnitude, in or out of the range.
        assert_eq!(codes(set_with(q(-273.16, "degC"))), [OPERATING_CONDITION_QUANTITY_INVALID]);
        assert_eq!(codes(set_with(q(-2.0e6, "degC"))), [OPERATING_CONDITION_QUANTITY_INVALID]);
        assert_eq!(codes(set_with(q(-1.5e-24, "K"))), [OPERATING_CONDITION_QUANTITY_INVALID]);
        assert!(codes(set_with(q(-273.15, "degC"))).is_empty(), "exactly zero kelvin");
        assert!(codes(set_with(q(-459.67, "degF"))).is_empty(), "exactly zero kelvin");
        assert_eq!(codes(set_with(q(20.0, "K/s"))), [OPERATING_CONDITION_QUANTITY_INVALID]);
        assert_eq!(codes(set_with(q(20.0, "MPa"))), [OPERATING_CONDITION_QUANTITY_INVALID]);
        assert_eq!(codes(set_with(q(f64::NAN, "K"))), [OPERATING_CONDITION_QUANTITY_INVALID]);
    }
}
