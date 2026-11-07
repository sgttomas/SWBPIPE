//! RT-A6 H1: the one contents resolver of an explicitly selected load case
//! (DEL-05-01 Work `hydrostatic-test.md` §4, §4.1).
//!
//! A case's `condition_use.contents_density` names a condition key; each pipe's
//! `condition_set_ref` names the set whose condition of that key states the
//! contents density. [`CaseContentsContext::capture`] takes an immutable,
//! owned snapshot of one authored case's selection, by exact case id, **before**
//! any consumer clears, copies or renames cases (the self-weight inspection
//! clears `load_cases` on its clone; the modal weight basis uses a scratch case
//! id). [`case_contents_density`] then resolves a pipe's density from that
//! snapshot: the set, the key and an explicit finite nonnegative density in a
//! density unit of the units catalog, normalized by the one units authority the
//! self-weight generator also uses. Zero is an authored value, not absence.
//! Missing or contradictory inputs refuse by name; nothing is assumed or
//! defaulted, and a case with no contents use has no context at all (the
//! unchanged legacy section-density branch).
//!
//! Contents density is mass input, not an action. The solve consumes a case's
//! selection only through self-weight generated from it ([`crate::self_weight`]),
//! one mass computation per pipe. A valid selection without generated contents
//! weight gives a nonblocking notice: no force or gravity is inferred. Invalid
//! selections and unconnected seismic mass remain blocking ([`contents_use_findings`]). [`RetainedContents`] is the lowering's contents-specific retention:
//! after `lower` clears the operating-condition members, it restores only the
//! contents selection (each using case's key, the sets' contents values and the
//! pipes' set references) so the post-lowering self-weight validation resolves
//! the same selection from the same authored values.
use super::{
    ConditionSetInput, ConditionUseInput, OperatingConditionInput, OPERATING_CONDITION_KEY_DUPLICATE,
    OPERATING_CONDITION_QUANTITY_INVALID, OPERATING_CONDITION_SET_UNRESOLVED,
};
use super::resolution::{
    LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED, OPERATING_CONDITION_KEY_MISSING, OPERATING_CONDITION_SET_MISSING,
    OPERATING_CONDITION_VALUE_MISSING,
};
use crate::{Diagnostic, PreviewLoadCase, PreviewModel, Quantity};
use open_pipe_stress_units::{unit_by_symbol, Dimension};

/// The named source case of an explicit contents selection does not exist, or
/// states no `condition_use.contents_density`.
pub const CASE_CONTENTS_CONTEXT_MISSING: &str = "CASE_CONTENTS_CONTEXT_MISSING";
/// The named source case, a pipe or a set does not resolve exactly once, or the
/// explicit selection contradicts another stated input.
pub const CASE_CONTENTS_CONTEXT_CONFLICT: &str = "CASE_CONTENTS_CONTEXT_CONFLICT";
/// The provenance `method` of a resolved contents selection.
pub const CONTENTS_SOURCE_METHOD: &str = "case_condition_contents_density/v1";
/// Valid selected contents have not been applied as weight in this case.
pub const CASE_CONTENTS_WEIGHT_NOT_APPLIED: &str = "CASE_CONTENTS_WEIGHT_NOT_APPLIED";

fn contents_diag(case: &str, suffix: &str, code: &str, message: String, refs: Vec<String>) -> Diagnostic {
    crate::diag(
        &format!("diagnostic:load-case:{}:contents:{suffix}", crate::stable_suffix(case)),
        code,
        "blocking",
        message,
        refs,
    )
}

/// One authored case's contents selection, captured before any consumer
/// changes the case list. Owned: later edits of a clone cannot reach it.
#[derive(Debug, Clone)]
pub struct CaseContentsContext {
    case_id: String,
    key: String,
    /// Every pipe's id and `condition_set_ref`, in `pipe_segments` order.
    pipes: Vec<(String, Option<String>)>,
    /// The document's condition sets as authored.
    sets: Vec<ConditionSetInput>,
}

/// One pipe's resolved contents: the selecting case, its key, the set that
/// states the value, the authored quantity and its normalized value.
#[derive(Debug, Clone)]
pub struct ResolvedContents {
    pub case_id: String,
    pub key: String,
    pub condition_set: String,
    pub authored: Quantity,
    /// The density in the canonical unit (kg/m^3).
    pub normalized: f64,
}

impl CaseContentsContext {
    /// The exact-id capture of `case_id`'s selection (§4.1 step 1). No case of
    /// that exact id is `CASE_CONTENTS_CONTEXT_MISSING`, more than one is
    /// `CASE_CONTENTS_CONTEXT_CONFLICT`; the id is never trimmed or matched
    /// loosely. `Ok(None)`: the case states no contents use.
    pub fn capture(model: &PreviewModel, case_id: &str) -> Result<Option<Self>, Vec<Diagnostic>> {
        let mut matching = model.load_cases.iter().enumerate().filter(|(_, case)| case.id == case_id);
        match (matching.next(), matching.next()) {
            (None, _) => Err(vec![contents_diag(case_id, "source-missing", CASE_CONTENTS_CONTEXT_MISSING,
                format!("the contents source case `{case_id}` names no load case of the model; no contents are assumed, so nothing is generated"),
                vec![case_id.to_string()])]),
            (Some(_), Some(_)) => Err(vec![contents_diag(case_id, "source-conflict", CASE_CONTENTS_CONTEXT_CONFLICT,
                format!("the contents source case `{case_id}` names more than one load case; the selection is not taken from either"),
                vec![case_id.to_string()])]),
            (Some((index, _)), None) => {
                let context = Self::of_case(model, index);
                if context.as_ref().is_some_and(|context| context.key.trim().is_empty()) {
                    return Err(vec![contents_diag(case_id, "key-invalid", CASE_CONTENTS_CONTEXT_CONFLICT,
                        "the source case's contents key must be nonblank and is never trimmed into another key".into(),
                        vec![case_id.to_string()])]);
                }
                Ok(context)
            },
        }
    }

    /// The selection of the case at `index`, which the caller is reading in
    /// place; `None` when it states no contents use.
    fn of_case(model: &PreviewModel, index: usize) -> Option<Self> {
        let case = &model.load_cases[index];
        let key = case.condition_use.as_ref()?.contents_density.clone()?;
        Some(Self {
            case_id: case.id.clone(),
            key,
            pipes: model.pipe_segments.iter().map(|pipe| (pipe.id.clone(), pipe.condition_set_ref.clone())).collect(),
            sets: model.condition_sets.clone(),
        })
    }

    /// The selection of a case read in place (the inspection's own loop): no
    /// context when it states no contents use, so a legacy case is never
    /// looked up; otherwise the exact-id capture, which also refuses a
    /// duplicated case id.
    pub(crate) fn of_case_in_place(model: &PreviewModel, case: &PreviewLoadCase) -> Result<Option<Self>, Vec<Diagnostic>> {
        if case.condition_use.as_ref().and_then(|using| using.contents_density.as_ref()).is_none() {
            return Ok(None);
        }
        Self::capture(model, &case.id)
    }

    /// The case whose authored use supplied this selection.
    pub fn case_id(&self) -> &str {
        &self.case_id
    }

    /// The selected condition key.
    pub fn key(&self) -> &str {
        &self.key
    }
}

/// The density `context` selects for `pipe` (§4): its set, the selected key's
/// condition in that set and the condition's explicit density. Every refusal
/// names the case, pipe, set and key.
pub fn case_contents_density(context: &CaseContentsContext, pipe: &str) -> Result<ResolvedContents, Vec<Diagnostic>> {
    let case = context.case_id.as_str();
    let key = context.key.as_str();
    let at = |what: &str| format!("{}:{what}", crate::stable_suffix(pipe));
    let refs = |set: &str| vec![case.to_string(), pipe.to_string(), set.to_string(), key.to_string(), "contents_density".to_string()];
    let mut pipes = context.pipes.iter().filter(|(id, _)| id == pipe);
    let reference = match (pipes.next(), pipes.next()) {
        (Some((_, reference)), None) => reference,
        (None, _) => return Err(vec![contents_diag(case, &at("pipe-missing"), CASE_CONTENTS_CONTEXT_MISSING,
            format!("load case `{case}` selects contents `{key}` and pipe `{pipe}` is not a pipe of the captured model; nothing is generated"),
            vec![case.to_string(), pipe.to_string()])]),
        _ => return Err(vec![contents_diag(case, &at("pipe-conflict"), CASE_CONTENTS_CONTEXT_CONFLICT,
            format!("load case `{case}` selects contents `{key}` and pipe `{pipe}` names more than one pipe; nothing is generated"),
            vec![case.to_string(), pipe.to_string()])]),
    };
    let Some(set_id) = reference.as_deref() else {
        return Err(vec![contents_diag(case, &at("set-missing"), OPERATING_CONDITION_SET_MISSING,
            format!("load case `{case}` uses condition `{key}` for contents density and pipe `{pipe}` carries no condition set; no contents density is assumed, so nothing is generated"),
            vec![case.to_string(), pipe.to_string(), "contents_density".to_string()])]);
    };
    let mut sets = context.sets.iter().filter(|set| set.id == set_id);
    let set = match (sets.next(), sets.next()) {
        (Some(set), None) => set,
        (None, _) => return Err(vec![contents_diag(case, &at("set-unresolved"), OPERATING_CONDITION_SET_UNRESOLVED,
            format!("load case `{case}` uses condition `{key}` for contents density and condition_set_ref `{set_id}` of pipe `{pipe}` names no condition set; nothing is generated"),
            refs(set_id))]),
        _ => return Err(vec![contents_diag(case, &at("set-conflict"), CASE_CONTENTS_CONTEXT_CONFLICT,
            format!("load case `{case}` uses condition `{key}` for contents density and condition_set_ref `{set_id}` of pipe `{pipe}` names more than one condition set; nothing is generated"),
            refs(set_id))]),
    };
    let mut conditions = set.conditions.iter().flatten().filter(|condition: &&OperatingConditionInput| condition.key == key);
    let condition = match (conditions.next(), conditions.next()) {
        (Some(condition), None) => condition,
        (None, _) => return Err(vec![contents_diag(case, &at("key-missing"), OPERATING_CONDITION_KEY_MISSING,
            format!("load case `{case}` uses condition `{key}` for contents density and condition set `{set_id}` of pipe `{pipe}` has no condition `{key}`; nothing is generated"),
            refs(set_id))]),
        _ => return Err(vec![contents_diag(case, &at("key-conflict"), OPERATING_CONDITION_KEY_DUPLICATE,
            format!("load case `{case}` uses condition `{key}` for contents density and condition set `{set_id}` of pipe `{pipe}` states condition `{key}` more than once; nothing is generated"),
            refs(set_id))]),
    };
    let Some(authored) = condition.contents_density.as_ref() else {
        return Err(vec![contents_diag(case, &at("value-missing"), OPERATING_CONDITION_VALUE_MISSING,
            format!("load case `{case}` uses condition `{key}` for contents density and condition `{key}` of set `{set_id}` (pipe `{pipe}`) states no contents_density; no density is assumed, so nothing is generated"),
            refs(set_id))]);
    };
    let invalid = |reason: String| {
        vec![contents_diag(case, &at("value-invalid"), OPERATING_CONDITION_QUANTITY_INVALID,
            format!("load case `{case}` uses condition `{key}` for contents density and conditions[`{key}`].contents_density of set `{set_id}` (pipe `{pipe}`) is not a contents density: {reason}; nothing is generated"),
            refs(set_id))]
    };
    if unit_by_symbol(&authored.unit, Dimension::Density).is_err() {
        return Err(invalid(format!("its unit `{}` is not a density unit of the units catalog", authored.unit)));
    }
    if !authored.value.is_finite() {
        return Err(invalid(format!("its value {} is not finite", authored.value)));
    }
    if authored.value < 0.0 {
        return Err(invalid(format!("{} {} is negative", authored.value, authored.unit)));
    }
    let mut normalized = authored.clone();
    let mut found = Vec::new();
    crate::normalize_quantity(&mut normalized, Dimension::Density,
        &format!("diagnostic:load-case:{}:contents:{}", crate::stable_suffix(case), at("normalize")), refs(set_id), &mut found);
    if !found.is_empty() {
        return Err(found);
    }
    if !normalized.value.is_finite() || normalized.value < 0.0 || (authored.value != 0.0 && normalized.value == 0.0) {
        return Err(invalid(format!("{} {} does not normalize to a finite nonnegative density", authored.value, authored.unit)));
    }
    Ok(ResolvedContents {
        case_id: case.to_string(),
        key: key.to_string(),
        condition_set: set_id.to_string(),
        authored: authored.clone(),
        normalized: normalized.value,
    })
}

/// Whether one generated-record provenance binds a contents selection: a
/// generated distributed self-weight with a `contents_source`, or a manual
/// override whose original generation carried one.
pub(crate) fn binds_contents(provenance: Option<&str>) -> bool {
    let Some(record) = provenance.and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok()) else {
        return false;
    };
    let generated = |record: &serde_json::Value| {
        record["method"] == crate::self_weight::GENERATED_METHOD && record["contents_source"].is_object()
    };
    generated(&record)
        || (record["method"] == crate::self_weight::MANUAL_OVERRIDE_METHOD && generated(&record["original_generation_provenance"]))
}

/// Judge the authored contents selection before reporting a nonblocking
/// absence of weight. A notice must never hide an unresolved set/key/value,
/// and seismic mass still has no connected contents consumer in H1.
pub(crate) fn contents_use_findings(model: &PreviewModel, case: &PreviewLoadCase) -> Vec<Diagnostic> {
    let Some(key) = case.condition_use.as_ref().and_then(|using| using.contents_density.as_ref()) else {
        return Vec::new();
    };
    let context = match CaseContentsContext::capture(model, &case.id) {
        Ok(Some(context)) => context,
        Ok(None) => return vec![contents_diag(&case.id, "selection-missing", CASE_CONTENTS_CONTEXT_MISSING,
            "the explicit contents selection has no authoritative context".into(), vec![case.id.clone()])],
        Err(found) => return found,
    };
    let mut found = Vec::new();
    for pipe in &model.pipe_segments {
        if let Err(refusals) = case_contents_density(&context, &pipe.id) {
            found.extend(refusals);
        }
    }
    if crate::has_blocking(&found) {
        return found;
    }
    let id = case.id.as_str();
    if case.equivalent_static.as_ref().is_some_and(|generation| generation.seismic.is_some()) {
        found.push(crate::diag(
            &format!("diagnostic:load-case:{}:condition:contents-density-seismic", crate::stable_suffix(id)),
            LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED,
            "blocking",
            format!("load case `{id}` uses condition `{key}` for contents density and also generates equivalent-static seismic loads, whose mass is the sections' own and does not consume the selection; nothing is solved"),
            vec![id.to_string(), key.clone()],
        ));
    } else if !case.primitive_loads.iter().any(|load| binds_contents(load.provenance.as_deref())) {
        found.push(crate::diag(
            &format!("diagnostic:load-case:{}:condition:contents-density", crate::stable_suffix(id)),
            CASE_CONTENTS_WEIGHT_NOT_APPLIED,
            "warning",
            format!("load case `{id}` selects condition `{key}` for contents density, but that density has NOT been applied as weight in this case. No force is generated and no gravity is assumed; generate self-weight with explicit gravity when weight is wanted. The source selection can remain authored."),
            vec![id.to_string(), key.clone()],
        ));
    }
    found
}

/// The contents selection the lowering keeps (module docs): taken before it
/// clears the operating-condition members, restored after. Only a document
/// in which some case uses contents keeps anything.
pub(crate) struct RetainedContents {
    uses: Vec<(usize, String)>,
    sets: Vec<ConditionSetInput>,
    pipes: Vec<Option<String>>,
}

impl RetainedContents {
    pub(crate) fn take(model: &PreviewModel) -> Option<Self> {
        let uses: Vec<(usize, String)> = model
            .load_cases
            .iter()
            .enumerate()
            .filter_map(|(index, case)| Some((index, case.condition_use.as_ref()?.contents_density.clone()?)))
            .collect();
        if uses.is_empty() {
            return None;
        }
        let sets = model
            .condition_sets
            .iter()
            .map(|set| {
                let mut kept = set.clone();
                kept.label = None;
                kept.installation_temperature = None;
                kept.design = None;
                kept.peak = None;
                kept.added_weight = None;
                kept.exposure = None;
                kept.conditions = set.conditions.as_ref().map(|conditions| {
                    conditions
                        .iter()
                        .map(|condition| OperatingConditionInput {
                            key: condition.key.clone(),
                            label: None,
                            temperature: None,
                            pressure: None,
                            contents_density: condition.contents_density.clone(),
                        })
                        .collect()
                });
                kept
            })
            .collect();
        Some(Self { uses, sets, pipes: model.pipe_segments.iter().map(|pipe| pipe.condition_set_ref.clone()).collect() })
    }

    pub(crate) fn restore(self, model: &mut PreviewModel) {
        model.condition_sets = self.sets;
        for (pipe, reference) in model.pipe_segments.iter_mut().zip(self.pipes) {
            pipe.condition_set_ref = reference;
        }
        for (index, key) in self.uses {
            model.load_cases[index].condition_use = Some(ConditionUseInput {
                temperature: None,
                pressure: None,
                contents_density: Some(key),
                added_weight: None,
                wind_exposure: None,
            });
        }
    }
}
