//! The hydrostatic test case (DEL-05-01 REQ-014, AC-010; DEL-04-07 REQ-011,
//! AC-012).
//!
//! A load case that declares `test_condition` is a hydrostatic test. It is
//! solved on the exact pressure route only; on the pressure-free route it is
//! refused by name and never solved as an ordinary case. Its pressure is the
//! case's own authored pressure (its `pressure_regions`). Its contents density
//! is that of the condition its `condition_use.contents_density` selects, from
//! the set each weighted pipe carries: it enters the case's generated
//! self-weight once, in place of the section's own contents, through the
//! self-weight calculation's own mass function
//! ([`crate::self_weight::mass_with_contents`]); no second weight load is
//! generated. With `held_supports: springs_and_constant_effort` each spring,
//! variable spring and constant-effort support is replaced on the degree of
//! freedom it acts on by a rigid restraint at zero displacement from the
//! authored geometry, with neither its stiffness nor its force acting, and its
//! reaction is published under its own identity; with `none` the supports act
//! as in any case.
//!
//! [`judge`] reads the model only: it gives every refusal by name and the
//! contents resolution, which the solve applies after the applied self-weight
//! has been checked against its model ([`apply_contents`]). Nothing is
//! defaulted: a test case selects its contents, and a replacement that cannot
//! be formed is refused with no partial result.
use crate::operating_conditions::{resolution, HeldSupports, TestConditionInput};
use crate::self_weight::{self, GENERATED_METHOD, LEGACY_METHOD, MANUAL_OVERRIDE_METHOD};
use crate::{diag, stable_suffix, Diagnostic, LoadTargetInput, PreviewLoadCase, PreviewModel, PreviewSupport, Quantity};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// A test case on a document without a pressure contract (the pressure-free
/// route). The test is defined on the exact route only; it is refused, never
/// solved as an ordinary case.
pub const LOAD_CASE_TEST_CONDITION_PRESSURE_FREE: &str = "LOAD_CASE_TEST_CONDITION_PRESSURE_FREE";
/// A test case that selects no contents density (`condition_use.contents_density`).
pub const LOAD_CASE_TEST_CONTENTS_UNSELECTED: &str = "LOAD_CASE_TEST_CONTENTS_UNSELECTED";
/// The selected test contents have no generated self-weight to enter, or the
/// test case carries a self-weight load they cannot enter (a manual override, a
/// legacy-method load, or one generated for another case).
pub const LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID: &str = "LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID";
/// Two held supports act on one degree of freedom of one node.
pub const LOAD_CASE_TEST_HELD_SUPPORT_DUPLICATE: &str = "LOAD_CASE_TEST_HELD_SUPPORT_DUPLICATE";
/// A held support acts on a degree of freedom another support already restrains.
pub const LOAD_CASE_TEST_HELD_SUPPORT_RESTRAINED: &str = "LOAD_CASE_TEST_HELD_SUPPORT_RESTRAINED";
/// A held support's acted degree of freedom or position cannot be determined.
pub const LOAD_CASE_TEST_HELD_SUPPORT_UNFORMED: &str = "LOAD_CASE_TEST_HELD_SUPPORT_UNFORMED";
/// A movement is prescribed at a held support in the test case.
pub const LOAD_CASE_TEST_HELD_SUPPORT_MOTION: &str = "LOAD_CASE_TEST_HELD_SUPPORT_MOTION";
/// A combination that has a hydrostatic test case among its operands,
/// directly or through another combination (LIM-003).
pub const COMBINATION_HYDROSTATIC_TEST_WITHHELD: &str = "COMBINATION_HYDROSTATIC_TEST_WITHHELD";

/// The `method` of the contents resolution named in a resolved load's evidence.
pub const CONTENTS_METHOD: &str = "hydrostatic_test_contents_in_self_weight/v1";

/// One generated self-weight load of a test case with its test contents.
#[derive(Debug, Clone)]
pub(crate) struct ContentsResolution {
    pub(crate) case_index: usize,
    pub(crate) case_id: String,
    pub(crate) load_id: String,
    pub(crate) pipe_id: String,
    pub(crate) condition_set_ref: String,
    pub(crate) condition_key: String,
    pub(crate) contents_density: Quantity,
    pub(crate) mass_kg_per_m: f64,
    pub(crate) intensity_n_per_m: f64,
}

/// One support a test case holds: its identity, node and acted DOF token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HeldSupport {
    pub(crate) support_id: String,
    pub(crate) node: String,
    pub(crate) dof: crate::FrameDof,
}

fn case_diag(case: &str, suffix: &str, code: &'static str, message: String, refs: Vec<String>) -> Diagnostic {
    diag(&format!("diagnostic:load-case:{}:test:{suffix}", stable_suffix(case)), code, "blocking", message, refs)
}

/// Whether a support is of a family a test case holds: a spring, a variable
/// spring hanger or a constant-effort support, linear (no `nonlinear` member).
pub(crate) fn is_holdable(support: &PreviewSupport) -> bool {
    support.nonlinear.is_none()
        && support.family.as_deref() != Some("snubber")
        && (support.family.as_deref() == Some("spring")
            || crate::is_variable_spring_hanger(support)
            || crate::is_constant_effort_support(support))
}

/// The degree of freedom a holdable support acts on, as the solve consumes it:
/// a spring's or variable spring's stiffness DOF (its first restraint when it
/// states none, as the assembly does), a constant-effort support's single
/// translational restraint.
fn acted_dof(support: &PreviewSupport) -> Result<crate::FrameDof, String> {
    if crate::document_members::is_skewed_record(support) {
        return Err("its line of action is skewed, so no global degree of freedom carries the replacement".into());
    }
    if crate::is_constant_effort_support(support) {
        let mut translational = Vec::new();
        for raw in &support.restraints {
            let dof = crate::parse_dof(raw).map_err(|_| format!("its restraint `{raw}` is not a frame degree of freedom"))?;
            if dof.is_translational() {
                translational.push(dof);
            }
        }
        return match translational.as_slice() {
            [dof] => Ok(*dof),
            [] => Err("it declares no translational restraint, so the degree of freedom it acts on is not entered".into()),
            many => Err(format!("it declares {} translational restraints where it acts on exactly one", many.len())),
        };
    }
    let token = crate::support_stiffness_input(support).map(|input| input.dof.clone()).or_else(|| support.restraints.first().cloned());
    let Some(token) = token else {
        return Err("it states neither a stiffness degree of freedom nor a restraint".into());
    };
    crate::parse_dof(&token).map_err(|_| format!("its degree of freedom `{token}` is not a frame degree of freedom"))
}

/// The supports test case `case` holding `springs_and_constant_effort` holds on
/// a model whose axis-aligned directions are lowered, in `supports` order, with
/// a refusal by name for every reason the replacement cannot be formed. The
/// held set is the document's; the refusals name the case.
pub(crate) fn held_set(model: &PreviewModel, case: &str) -> (Vec<HeldSupport>, Vec<Diagnostic>) {
    let mut held = Vec::new();
    let mut refusals = Vec::new();
    let lead = |message: String| format!("hydrostatic test case `{case}`: {message}; nothing is solved");
    let named = |refs: Vec<String>| std::iter::once(case.to_string()).chain(refs).collect::<Vec<_>>();
    let nodes: HashSet<&str> = model.nodes.iter().map(|node| node.id.as_str()).collect();
    for support in model.supports.iter().filter(|support| is_holdable(support)) {
        if !nodes.contains(support.node.as_str()) {
            refusals.push(case_diag(case, &format!("unformed:{}", stable_suffix(&support.id)), LOAD_CASE_TEST_HELD_SUPPORT_UNFORMED,
                lead(format!("held support `{}` names node `{}`, which the model does not have, so its position from the authored geometry is not determined", support.id, support.node)),
                named(vec![support.id.clone(), support.node.clone()])));
            continue;
        }
        match acted_dof(support) {
            Ok(dof) => held.push(HeldSupport { support_id: support.id.clone(), node: support.node.clone(), dof }),
            Err(reason) => refusals.push(case_diag(case, &format!("unformed:{}", stable_suffix(&support.id)), LOAD_CASE_TEST_HELD_SUPPORT_UNFORMED,
                lead(format!("held support `{}` cannot be replaced by a rigid restraint: {reason}", support.id)),
                named(vec![support.id.clone()]))),
        }
    }
    let mut by_dof: BTreeMap<(String, usize), Vec<&HeldSupport>> = BTreeMap::new();
    for support in &held {
        by_dof.entry((support.node.clone(), crate::dof_index(support.dof))).or_default().push(support);
    }
    for ((node, _), group) in &by_dof {
        if group.len() > 1 {
            let ids: Vec<String> = group.iter().map(|support| support.support_id.clone()).collect();
            refusals.push(case_diag(case, &format!("duplicate:{}", stable_suffix(&ids.join(","))), LOAD_CASE_TEST_HELD_SUPPORT_DUPLICATE,
                lead(format!("held supports {} all act on {} at node `{node}`; as rigid restraints on one degree of freedom the split of the reaction between them is not determined, so the replacement cannot be formed",
                    ids.iter().map(|id| format!("`{id}`")).collect::<Vec<_>>().join(", "), crate::dof_name(group[0].dof))),
                named(ids.into_iter().chain([node.clone()]).collect())));
        }
    }
    for support in &held {
        for other in model.supports.iter().filter(|other| other.id != support.support_id && other.node == support.node && !is_holdable(other)) {
            let rigid = other.family.as_deref() != Some("snubber")
                && other.restraints.iter().any(|raw| crate::parse_dof(raw).ok() == Some(support.dof));
            let nonlinear = other.nonlinear.as_ref().is_some_and(|nonlinear| crate::parse_dof(&nonlinear.dof).ok() == Some(support.dof));
            if rigid || nonlinear {
                refusals.push(case_diag(case, &format!("restrained:{}:{}", stable_suffix(&support.support_id), stable_suffix(&other.id)), LOAD_CASE_TEST_HELD_SUPPORT_RESTRAINED,
                    lead(format!("held support `{}` acts on {} at node `{}`, which support `{}` already restrains; the replacement would duplicate that restraint and its reaction under `{}` is not determined, so it cannot be formed",
                        support.support_id, crate::dof_name(support.dof), support.node, other.id, support.support_id)),
                    named(vec![support.support_id.clone(), other.id.clone()])));
            }
        }
    }
    (held, refusals)
}

/// The test condition of a case, when it declares one.
pub(crate) fn test_condition(case: &PreviewLoadCase) -> Option<&TestConditionInput> {
    case.test_condition.as_ref()
}

/// Whether the case holds supports.
pub(crate) fn holds_supports(case: &PreviewLoadCase) -> bool {
    test_condition(case).is_some_and(|test| test.held_supports == HeldSupports::SpringsAndConstantEffort)
}

/// On a document with a pressure contract, a case's `condition_use` is admitted
/// only on a test case and only as a contents selection.
pub(crate) fn admits_exact_use(case: &PreviewLoadCase) -> bool {
    let Some(using) = case.condition_use.as_ref() else { return true };
    case.test_condition.is_some()
        && using.contents_density.is_some()
        && using.temperature.is_none()
        && using.pressure.is_none()
        && using.added_weight.is_none()
        && using.wind_exposure.is_none()
}

fn evidence(load: &crate::PreviewPrimitiveLoad) -> Option<Value> {
    load.provenance.as_deref().and_then(|text| serde_json::from_str::<Value>(text).ok())
}

fn generated_id(case: &str, pipe: &str) -> String {
    format!("load:self-weight:{}:{}:{}:{}", case.len(), case, pipe.len(), pipe)
}

/// Every refusal of the hydrostatic test cases of `model` (read only, before
/// the lowering), appended to `diagnostics`, and the contents resolution of
/// each admitted test case. On the pressure-free route each test case is
/// refused by name and nothing is resolved.
pub(crate) fn judge(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) -> Vec<ContentsResolution> {
    let exact = model.pressure_contract.is_some();
    let mut resolved = Vec::new();
    if !model.load_cases.iter().any(|case| case.test_condition.is_some()) {
        return resolved;
    }
    if !exact {
        for case in model.load_cases.iter().filter(|case| case.test_condition.is_some()) {
            diagnostics.push(case_diag(&case.id, "pressure-free", LOAD_CASE_TEST_CONDITION_PRESSURE_FREE,
                format!("load case `{}` is a hydrostatic test on a document without a pressure contract; the test is solved on the exact pressure route only and is never solved as an ordinary case, so nothing is solved", case.id),
                vec![case.id.clone()]));
        }
        return resolved;
    }
    // The held set reads DOF tokens as the solve will: after the lowering of
    // exactly axis-aligned directions.
    let lowered = model.load_cases.iter().any(holds_supports).then(|| {
        let mut lowered = model.clone();
        crate::document_members::lower_axis_aligned_directions(&mut lowered);
        lowered
    });
    let sets: HashMap<&str, &crate::operating_conditions::ConditionSetInput> =
        model.condition_sets.iter().map(|set| (set.id.as_str(), set)).collect();
    for (case_index, case) in model.load_cases.iter().enumerate() {
        let Some(test) = test_condition(case) else { continue };
        let id = case.id.as_str();
        if test.held_supports == HeldSupports::SpringsAndConstantEffort {
            let (held, refusals) = held_set(lowered.as_ref().expect("formed when a case holds supports"), id);
            diagnostics.extend(refusals);
            let held_ids: HashSet<&str> = held.iter().map(|support| support.support_id.as_str()).collect();
            for load in &case.primitive_loads {
                if let LoadTargetInput::Support { support, dof } = &load.target {
                    if held_ids.contains(support.as_str()) {
                        diagnostics.push(case_diag(id, &format!("motion:{}", stable_suffix(&load.id)), LOAD_CASE_TEST_HELD_SUPPORT_MOTION,
                            format!("hydrostatic test case `{id}` prescribes movement `{}` on {dof} of held support `{support}`; a held support is a rigid restraint at zero displacement from the authored geometry, so the movement contradicts it and nothing is solved", load.id),
                            vec![id.to_string(), support.clone(), load.id.clone()]));
                    }
                }
            }
            if let Some(state) = case.analysis_state.value() {
                for support_state in &state.support_states {
                    let moved = support_state.boundary_motion.as_ref().is_some_and(|m| !m.is_empty())
                        || support_state.base_motion.as_ref().is_some_and(|m| !m.is_empty());
                    if moved && held_ids.contains(support_state.support_ref.as_str()) {
                        diagnostics.push(case_diag(id, &format!("motion:{}", stable_suffix(&support_state.support_ref)), LOAD_CASE_TEST_HELD_SUPPORT_MOTION,
                            format!("hydrostatic test case `{id}` states boundary or base motion for held support `{}`; a held support is a rigid restraint at zero displacement from the authored geometry, so the motion contradicts it and nothing is solved", support_state.support_ref),
                            vec![id.to_string(), support_state.support_ref.clone()]));
                    }
                }
            }
        }
        // Contents: selected, and entered through the case's generated self-weight.
        let Some(key) = case.condition_use.as_ref().and_then(|using| using.contents_density.clone()) else {
            diagnostics.push(case_diag(id, "contents-unselected", LOAD_CASE_TEST_CONTENTS_UNSELECTED,
                format!("hydrostatic test case `{id}` selects no contents density; a test case takes its contents from the condition its condition_use.contents_density names and none is assumed, so nothing is solved"),
                vec![id.to_string()]));
            continue;
        };
        let mut entering = Vec::new();
        for load in &case.primitive_loads {
            let Some(found) = evidence(load) else { continue };
            let method = found["method"].as_str().unwrap_or_default();
            if method == MANUAL_OVERRIDE_METHOD || method == LEGACY_METHOD {
                diagnostics.push(case_diag(id, &format!("contents-self-weight:{}", stable_suffix(&load.id)), LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID,
                    format!("hydrostatic test case `{id}` carries self-weight load `{}` by method `{method}`, whose magnitude the test contents cannot enter; generate the case's self-weight again so that the contents enter once, so nothing is solved", load.id),
                    vec![id.to_string(), load.id.clone()]));
                continue;
            }
            if method != GENERATED_METHOD {
                continue;
            }
            let pipe = match &load.target {
                LoadTargetInput::Element { pipe } => pipe.clone(),
                _ => String::new(),
            };
            if pipe.is_empty() || load.id != generated_id(id, &pipe) {
                diagnostics.push(case_diag(id, &format!("contents-self-weight:{}", stable_suffix(&load.id)), LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID,
                    format!("hydrostatic test case `{id}` carries self-weight load `{}`, which was generated for another case or target; the test contents enter only this case's own generated self-weight, so nothing is solved", load.id),
                    vec![id.to_string(), load.id.clone()]));
                continue;
            }
            entering.push((load, pipe, found));
        }
        if entering.is_empty() {
            diagnostics.push(case_diag(id, "contents-self-weight", LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID,
                format!("hydrostatic test case `{id}` selects contents `{key}` but carries no generated self-weight for them to enter; contents are a mass input of the self-weight calculation and are never a load of their own, so nothing is solved"),
                vec![id.to_string(), key.clone()]));
            continue;
        }
        for (load, pipe_id, found) in entering {
            let at = |what: &str| format!("contents:{}:{what}", stable_suffix(&pipe_id));
            let refs = |extra: &[&str]| std::iter::once(id.to_string()).chain([pipe_id.clone()]).chain(extra.iter().map(|text| text.to_string())).collect::<Vec<_>>();
            let Some(pipe) = model.pipe_segments.iter().find(|pipe| pipe.id == pipe_id) else { continue };
            let Some(set_id) = pipe.condition_set_ref.as_deref() else {
                diagnostics.push(case_diag(id, &at("set-missing"), resolution::OPERATING_CONDITION_SET_MISSING,
                    format!("hydrostatic test case `{id}` uses contents_density from conditions and pipe `{pipe_id}` carries no condition set; no value is assumed, so nothing is solved"),
                    refs(&["contents_density"])));
                continue;
            };
            let Some(set) = sets.get(set_id) else { continue };
            let Some(condition) = set.conditions.iter().flatten().find(|condition| condition.key == key) else {
                diagnostics.push(case_diag(id, &at("key-missing"), resolution::OPERATING_CONDITION_KEY_MISSING,
                    format!("hydrostatic test case `{id}` uses condition `{key}` for contents_density and condition set `{set_id}` of pipe `{pipe_id}` has no condition `{key}`; nothing is solved"),
                    refs(&[set_id, &key, "contents_density"])));
                continue;
            };
            let Some(density) = condition.contents_density.as_ref() else {
                diagnostics.push(case_diag(id, &at("value-missing"), resolution::OPERATING_CONDITION_VALUE_MISSING,
                    format!("hydrostatic test case `{id}` uses condition `{key}` for contents_density and condition `{key}` of set `{set_id}` (pipe `{pipe_id}`) states no contents_density; none is assumed, so nothing is solved"),
                    refs(&[set_id, &key, "contents_density"])));
                continue;
            };
            let acceleration = found["normalized_acceleration_m_per_s2"].as_f64();
            let mass = self_weight::mass_with_contents(model, &pipe_id, density);
            match (mass, acceleration) {
                (Ok(mass), Some(acceleration)) if (mass * acceleration).is_finite() && mass * acceleration != 0.0 => resolved.push(ContentsResolution {
                    case_index,
                    case_id: id.to_string(),
                    load_id: load.id.clone(),
                    pipe_id: pipe_id.clone(),
                    condition_set_ref: set_id.to_string(),
                    condition_key: key.clone(),
                    contents_density: density.clone(),
                    mass_kg_per_m: mass,
                    intensity_n_per_m: mass * acceleration,
                }),
                (mass, _) => {
                    let reason = match mass {
                        Err(found) => found.iter().map(|d| format!("{}: {}", d.code, d.message)).collect::<Vec<_>>().join("; "),
                        Ok(_) => "its generation evidence states no finite gravity, or the weight per length is outside the representable range".into(),
                    };
                    diagnostics.push(case_diag(id, &at("mass"), LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID,
                        format!("hydrostatic test case `{id}`: the self-weight of pipe `{pipe_id}` with contents `{key}` cannot be formed ({reason}), so nothing is solved"),
                        refs(&[&load.id])));
                }
            }
        }
    }
    combination_refusals(model, diagnostics);
    resolved
}

/// LIM-003: every combination that reaches a hydrostatic test case through its
/// operands, directly or through other combinations, is withheld by name.
fn combination_refusals(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    let tests: HashSet<&str> = model.load_cases.iter().filter(|case| case.test_condition.is_some()).map(|case| case.id.as_str()).collect();
    if tests.is_empty() {
        return;
    }
    let operands = |combination: &crate::PreviewCombination| -> Vec<String> {
        let mut found: Vec<String> = Vec::new();
        for term in &combination.terms {
            found.extend(term.combination.iter().cloned());
            if term.authored_load_case() {
                found.push(term.load_case.clone());
            }
        }
        found.extend(combination.minuend_id.iter().cloned());
        found.extend(combination.subtrahend_id.iter().cloned());
        found.extend(combination.operand_ids.iter().flatten().cloned());
        found
    };
    let by_id: HashMap<&str, &crate::PreviewCombination> = model.combinations.iter().map(|c| (c.id.as_str(), c)).collect();
    for combination in &model.combinations {
        let mut reached = BTreeSet::new();
        let mut seen = HashSet::new();
        let mut stack = vec![combination.id.clone()];
        while let Some(current) = stack.pop() {
            if !seen.insert(current.clone()) {
                continue;
            }
            let Some(&node) = by_id.get(current.as_str()) else { continue };
            for operand in operands(node) {
                if tests.contains(operand.as_str()) {
                    reached.insert(operand);
                } else if by_id.contains_key(operand.as_str()) {
                    stack.push(operand);
                }
            }
        }
        if !reached.is_empty() {
            let named = reached.iter().map(|id| format!("`{id}`")).collect::<Vec<_>>().join(", ");
            diagnostics.push(diag(
                &format!("diagnostic:combination:{}:hydrostatic-test", stable_suffix(&combination.id)),
                COMBINATION_HYDROSTATIC_TEST_WITHHELD,
                "blocking",
                format!("combination `{}` has hydrostatic test case {named} among its operands; v1.0 does not define what a combination with a test case means, so the combination is withheld and nothing is solved; report the test case on its own", combination.id),
                std::iter::once(combination.id.clone()).chain(reached).collect(),
            ));
        }
    }
}

/// Applies a contents resolution to the model the solve reads, after the
/// applied self-weight has been checked against the model it came from: each
/// resolved load takes the self-weight calculation's magnitude with the test
/// contents, and its evidence names the set, the key, the density and the mass.
pub(crate) fn apply_contents(model: &mut PreviewModel, resolved: &[ContentsResolution]) {
    for item in resolved {
        let Some(case) = model.load_cases.get_mut(item.case_index).filter(|case| case.id == item.case_id) else { continue };
        let Some(load) = case.primitive_loads.iter_mut().find(|load| load.id == item.load_id) else { continue };
        let mut found = evidence(load).unwrap_or_else(|| json!({}));
        found["test_contents"] = contents_record(item);
        found["generated_payload"]["magnitude"]["value"] = json!(item.intensity_n_per_m);
        load.magnitude = Quantity { value: item.intensity_n_per_m, unit: "N/m".into() };
        load.provenance = Some(found.to_string());
    }
}

fn contents_record(item: &ContentsResolution) -> Value {
    json!({
        "method": CONTENTS_METHOD,
        "load_id": item.load_id,
        "pipe_id": item.pipe_id,
        "condition_set_ref": item.condition_set_ref,
        "condition_key": item.condition_key,
        "contents_density": {"value": item.contents_density.value, "unit": item.contents_density.unit},
        "mass_kg_per_m": item.mass_kg_per_m,
        "applied_n_per_m": item.intensity_n_per_m,
    })
}

/// The result record's mark of a test case (DEL-05-01 REQ-014): the test, the
/// held choice, the supports held (sorted; empty when nothing is held) and the
/// selected contents with the resolved self-weight each pipe received.
pub(crate) fn mark(case: &PreviewLoadCase, held: &[HeldSupport], resolved: &[ContentsResolution]) -> Option<Value> {
    let test = test_condition(case)?;
    let mut refs: Vec<String> = if holds_supports(case) { held.iter().map(|support| support.support_id.clone()).collect() } else { Vec::new() };
    refs.sort();
    Some(json!({
        "kind": "hydrostatic",
        "held_supports": match test.held_supports { HeldSupports::SpringsAndConstantEffort => "springs_and_constant_effort", HeldSupports::None => "none" },
        "held_support_refs": refs,
        "contents_condition": case.condition_use.as_ref().and_then(|using| using.contents_density.clone()),
        "contents_resolution": resolved.iter().filter(|item| item.case_id == case.id).map(contents_record).collect::<Vec<_>>(),
    }))
}

/// The model a held test case is assembled, solved, recovered and published
/// on: each held support becomes a rigid support of the same identity and node
/// restraining only its acted DOF, with no stiffness, hanger or force.
pub(crate) fn held_model(model: &PreviewModel, held: &[HeldSupport]) -> PreviewModel {
    let mut replaced = model.clone();
    for support in &mut replaced.supports {
        let Some(item) = held.iter().find(|item| item.support_id == support.id) else { continue };
        // No family is claimed for the replacement: it is the held support's
        // own rigid restraint, published under its own identity.
        support.family = None;
        support.restraints = vec![crate::dof_name(item.dof).to_string()];
        support.stiffness = None;
        support.hanger = None;
    }
    replaced
}
