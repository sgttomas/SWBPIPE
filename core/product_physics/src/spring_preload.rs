//! Variable spring preload (DEL-04-03 REQ-014; DEL-04-11 REQ-014).
//!
//! A variable spring hanger is a spring of its entered rate on its stiffness
//! DOF plus its entered `cold_load` as a constant force along the positive
//! axis of that DOF at its node, in every load case that applies it. It then
//! carries F_cold - k*(u - g), u its DOF's displacement from the drawn
//! geometry and g its base's displacement (zero unless the case moves it).
//!
//! [`dispositions`] is the one per-case decision: which springs apply their
//! cold load in a case and, for each that does not, why. Every site reads it:
//! the case ledger ([`add_loads`]), the published support action
//! ([`with_cold_load`], and the certified row in `certified_publication`),
//! the applied-force rows and not-applied diagnostics ([`append_results`]),
//! the combination gates ([`fixed_load_source`]), W2's admission and the
//! source replay's refusal. The stiffness is unchanged (`build_model`).
use super::*;

/// One spring that applies its cold load in a case.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Preload {
    pub node_index: usize,
    pub dof: FrameDof,
    pub cold_load_newtons: f64,
}

impl Preload {
    pub(crate) fn global_dof(&self) -> usize {
        self.node_index * DOF_PER_NODE + dof_index(self.dof)
    }
}

/// Why a variable spring hanger does not apply its cold load in a case. No
/// reason depends on the case yet; the case is the decision's input so that a
/// case-dependent reason (a held spring) is decided here and nowhere else.
/// A direction is not among them: every directed record that is a variable
/// spring hanger is refused before any solve (`document_members::supports_findings`).
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NotApplied {
    /// The record carries a `nonlinear` member: its nonlinear law governs.
    NonlinearRecord,
    /// No entered `hanger.cold_load`.
    ColdLoadMissing,
    /// The entered cold load is not a finite positive force.
    ColdLoadNotPositive,
    /// No stiffness DOF can be read from the record.
    StiffnessDofUnreadable,
    /// The stiffness DOF is rotational, where a force cannot act.
    RotationalDof(FrameDof),
    /// The support node is not in the model.
    UnknownNode,
}

impl NotApplied {
    /// A fixed token per reason, lower snake case, with no payload: for a
    /// caller that names the reason by code (not yet used in any published text).
    pub(crate) fn token(&self) -> &'static str {
        match self {
            Self::NonlinearRecord => "nonlinear_record",
            Self::ColdLoadMissing => "cold_load_missing",
            Self::ColdLoadNotPositive => "cold_load_not_positive",
            Self::StiffnessDofUnreadable => "stiffness_dof_unreadable",
            Self::RotationalDof(_) => "rotational_dof",
            Self::UnknownNode => "unknown_node",
        }
    }

    pub(crate) fn reason(&self) -> String {
        match self {
            Self::NonlinearRecord => "the record carries a nonlinear member, whose own law governs its DOF".into(),
            Self::ColdLoadMissing => "no hanger.cold_load is entered".into(),
            Self::ColdLoadNotPositive => "the entered hanger.cold_load is not a finite positive force".into(),
            Self::StiffnessDofUnreadable => "no stiffness DOF can be read from the record".into(),
            Self::RotationalDof(dof) => format!("its stiffness DOF {} is rotational, and the cold load is a force", dof_name(*dof)),
            Self::UnknownNode => "the support node is not present in the model".into(),
        }
    }
}

/// The stiffness DOF as `build_model` chooses it, without its `UZ` fallback.
fn stiffness_dof(support: &PreviewSupport) -> Option<FrameDof> {
    support_stiffness_input(support)
        .and_then(|input| parse_dof(&input.dof).ok())
        .or_else(|| support.restraints.first().and_then(|token| parse_dof(token).ok()))
}

fn decide(model: &PreviewModel, support: &PreviewSupport) -> Result<Preload, NotApplied> {
    if support.nonlinear.is_some() {
        return Err(NotApplied::NonlinearRecord);
    }
    let cold = support
        .hanger
        .as_ref()
        .and_then(|hanger| hanger.cold_load.as_ref())
        .ok_or(NotApplied::ColdLoadMissing)?;
    if !(cold.value.is_finite() && cold.value > 0.0) {
        return Err(NotApplied::ColdLoadNotPositive);
    }
    let dof = stiffness_dof(support).ok_or(NotApplied::StiffnessDofUnreadable)?;
    if !dof.is_translational() {
        return Err(NotApplied::RotationalDof(dof));
    }
    let node_index = node_index(model, &support.node).ok_or(NotApplied::UnknownNode)?;
    Ok(Preload { node_index, dof, cold_load_newtons: cold.value })
}

/// The one per-case decision: every variable spring hanger of the model, in
/// document order, with its preload in `case` or the reason it has none.
pub(crate) fn dispositions<'a>(
    model: &'a PreviewModel,
    _case: &PreviewLoadCase,
) -> Vec<(&'a PreviewSupport, Result<Preload, NotApplied>)> {
    model
        .supports
        .iter()
        .filter(|support| is_variable_spring_hanger(support))
        .map(|support| (support, decide(model, support)))
        .collect()
}

/// The spring's preload in `case`, if it applies one.
pub(crate) fn applied(model: &PreviewModel, case: &PreviewLoadCase, support_id: &str) -> Option<Preload> {
    dispositions(model, case)
        .into_iter()
        .find(|(support, _)| support.id == support_id)
        .and_then(|(_, disposition)| disposition.ok())
}

/// Whether any spring applies its cold load in any of the model's cases: a
/// fixed-load source for the combination gates' fixed-load count rule.
pub(crate) fn fixed_load_source(model: &PreviewModel) -> bool {
    model
        .load_cases
        .iter()
        .any(|case| dispositions(model, case).iter().any(|(_, disposition)| disposition.is_ok()))
}

/// The case ledger's preload terms, one per applying spring, beside the
/// constant-effort seam (S11 section 4.2: one term per application).
pub(crate) fn add_loads(ledger: &mut LoadLedger, model: &PreviewModel, case: &PreviewLoadCase) {
    for (support, disposition) in dispositions(model, case) {
        if let Ok(preload) = disposition {
            ledger.push(&support.id, preload.global_dof(), preload.cold_load_newtons);
        }
    }
}

/// The captured source replay's case force (S11 section 4.5): the live
/// producers `nodal_and_eigen_case_force` uses, then the case's preload terms,
/// so that the replay's ledger agrees bit for bit with the live case ledger.
pub(crate) fn replay_case_force(
    application: &LoadApplication,
    eigen: &[ThermalElementLoad],
    built: &BuiltModel,
    model: &PreviewModel,
    case: &PreviewLoadCase,
) -> Result<AssembledForce, open_pipe_stress_frame_kernel::load_ledger::LedgerError> {
    let mut ledger = LoadLedger::new();
    push_nodal_loads(&mut ledger, application, built.nodes.len());
    add_thermal_equivalent_loads(&mut ledger, eigen, &built.pipes, &HashMap::new());
    add_loads(&mut ledger, model, case);
    ledger.finish(built.nodes.len() * DOF_PER_NODE)
}

/// The spring's published action on its DOF: the cold load plus the spring
/// term -k*(u - g) the caller formed, where the case applies the cold load.
pub(crate) fn with_cold_load(model: &PreviewModel, case: &PreviewLoadCase, support_id: &str, spring_term: f64) -> f64 {
    match applied(model, case, support_id) {
        Some(preload) => preload.cold_load_newtons + spring_term,
        None => spring_term,
    }
}

/// W2's admission (F1b, ROOT Q3): a case with a preload term is outside the
/// declared subset, named as its own family.
pub(crate) const W2_FAMILY: &str = "variable_spring_hanger_preload";

pub(crate) const APPLIED_LOAD_KIND: &str = "variable_spring_hanger_applied_load";
/// The `combination_derivation` member stating that a spring applies its cold
/// load in the solve (a fixed-load source beside `consuming_constant_effort`);
/// present, and `true`, only where [`fixed_load_source`] holds.
pub(crate) const COLD_LOAD_FACT: &str = "applying_variable_spring_cold_load";
pub(crate) const NOT_APPLIED: &str = "SPRING_HANGER_COLD_LOAD_NOT_APPLIED";
pub(crate) const INSTALLED_DIFFERS: &str = "SPRING_HANGER_INSTALLED_LOAD_DIFFERS_FROM_COLD_LOAD";
const APPLIED_SIGN_CONVENTION: &str = "positive value is the user-entered cold load of the variable spring hanger, applied as a constant force along the positive axis of its translational stiffness DOF in this load case; the spring keeps its entered rate, and its published support force is this cold load minus the rate times the displacement relative to its base; no catalog or default value is supplied";

fn text_or<'a>(value: Option<&'a str>, missing: &'a str) -> &'a str {
    value.filter(|value| !value.trim().is_empty()).unwrap_or(missing)
}

/// Per case: one applied-force row per applying spring, and one non-blocking
/// warning per spring whose cold load the case does not apply.
pub(crate) fn append_results(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    model: &PreviewModel,
    case: &PreviewLoadCase,
) {
    for (support, disposition) in dispositions(model, case) {
        let suffix = stable_suffix(&support.id);
        match disposition {
            Ok(preload) => {
                let hanger = support.hanger.as_ref();
                let source = text_or(hanger.and_then(|h| h.source_reference.as_deref()), "source_reference_missing");
                let manufacturer = text_or(hanger.and_then(|h| h.manufacturer_reference.as_deref()), "manufacturer_reference_missing");
                let review = text_or(hanger.and_then(|h| h.load_side_review_reference.as_deref()), "load_side_review_reference_missing");
                let dof = dof_name(preload.dof);
                results.push(ResultItem {
                    component_body: None,
                    id: format!("result:spring-hanger:{suffix}:applied-load"),
                    kind: APPLIED_LOAD_KIND.to_string(),
                    value: preload.cold_load_newtons,
                    unit: "N".to_string(),
                    entity_ref: support.id.clone(),
                    basis_ref: None,
                    source_result_refs: Vec::new(),
                    metadata: Some(ResultMetadata {
                        component: APPLIED_LOAD_KIND.to_string(),
                        coordinate_system: "global".to_string(),
                        location: format!("{}:{dof} applied load", support.node),
                        basis: format!("support_family=variable_spring_hanger;consumed_dof={dof};source={source};manufacturer={manufacturer};load_side_review={review};mechanics_consumption=assembled_solve_cold_load;dec_ref=DEC-049"),
                        sign_convention: APPLIED_SIGN_CONVENTION.to_string(),
                    }),
                });
            }
            Err(reason) => diagnostics.push(diag(
                &format!("diagnostic:spring-hanger:{suffix}:cold-load-not-applied:{}", stable_suffix(&case.id)),
                NOT_APPLIED,
                "warning",
                format!("variable spring hanger {}: its cold load is not applied in load case {}, because {}; the spring acts with its rate only, and no applied force is published for it in this case", support.id, case.id, reason.reason()),
                vec![support.id.clone(), case.id.clone()],
            )),
        }
    }
}

/// The authored `hanger.mechanics_consumption` key's word when the document has none.
const AUTHORED_MISSING: &str = "mechanics_consumption_missing";

/// What the solve did with a spring's cold load over the model's cases (the review rows
/// are written once per document, the decision per case): `Ok(None)` applied in every
/// case, `Ok(Some(cases))` applied except in `cases` (with the first such case's
/// reason), `Err(reason)` applied in none. Today no reason depends on the case, so
/// every case agrees.
fn cold_load_outcome(model: &PreviewModel, support: &PreviewSupport) -> Result<Option<(Vec<String>, NotApplied)>, NotApplied> {
    let decisions: Vec<(&str, Result<Preload, NotApplied>)> = if model.load_cases.is_empty() {
        vec![("", decide(model, support))]
    } else {
        model
            .load_cases
            .iter()
            .map(|case| {
                let found = dispositions(model, case).into_iter().find(|(s, _)| s.id == support.id).map(|(_, d)| d);
                (case.id.as_str(), found.unwrap_or_else(|| decide(model, support)))
            })
            .collect()
    };
    let refused: Vec<(&str, NotApplied)> = decisions.into_iter().filter_map(|(case, d)| d.err().map(|r| (case, r))).collect();
    let total = model.load_cases.len().max(1);
    match refused.first() {
        None => Ok(None),
        Some((_, reason)) if refused.len() == total => Err(reason.clone()),
        Some((_, reason)) => Ok(Some((refused.iter().map(|(case, _)| case.to_string()).collect(), reason.clone()))),
    }
}

/// A variable spring hanger review row's basis (DEL-04-11 REQ-014): the author's
/// `mechanics_consumption` under `authored_mechanics_consumption`, verbatim, and under
/// `mechanics_consumption` what the solve consumed, in the product's own words, from
/// [`dispositions`]. `field` is `None` for the stiffness row.
pub(crate) fn review_basis(
    model: &PreviewModel,
    support: &PreviewSupport,
    field: Option<&str>,
    source: &str,
    manufacturer: &str,
    load_side_review: &str,
) -> String {
    let authored = text_or(support.hanger.as_ref().and_then(|h| h.mechanics_consumption.as_deref()), AUTHORED_MISSING);
    match field {
        None => {
            let dof = support_stiffness_input(support).map_or("", |input| input.dof.as_str());
            format!("support_family=variable_spring_hanger;user_entered_dof={dof};source={source};manufacturer={manufacturer};authored_mechanics_consumption={authored};mechanics_consumption=assembled_solve_spring_rate;dec_ref=DEC-049")
        }
        Some(field) => {
            let consumed = match field {
                "cold-load" => match cold_load_outcome(model, support) {
                    Ok(None) => "assembled_solve_cold_load".to_string(),
                    Ok(Some((cases, _))) => format!("assembled_solve_cold_load_except_cases:{}", cases.join(",")),
                    Err(reason) => format!("cold_load_not_applied_{}", reason.token()),
                },
                _ => "review_only".to_string(),
            };
            format!("support_family=variable_spring_hanger;field={field};source={source};manufacturer={manufacturer};load_side_review={load_side_review};authored_mechanics_consumption={authored};mechanics_consumption={consumed};dec_ref=DEC-049")
        }
    }
}

/// A variable spring hanger load review row's sign convention: the cold load's row
/// says what the solve did with it; every other row keeps the review sentence.
pub(crate) fn review_sign_convention(model: &PreviewModel, support: &PreviewSupport, field: &str) -> String {
    if field != "cold-load" {
        return "positive value is user-entered variable spring hanger input evidence; load-side and travel review remain human-reviewed preview metadata".to_string();
    }
    match cold_load_outcome(model, support) {
        Ok(None) => "positive value is the user-entered variable spring hanger cold load, applied in the solve as a constant force along the positive axis of its stiffness DOF in every load case".to_string(),
        Ok(Some((cases, reason))) => format!("positive value is the user-entered variable spring hanger cold load, applied in the solve as a constant force along the positive axis of its stiffness DOF except in load cases {}, where it is not applied because {}", cases.join(", "), reason.reason()),
        Err(reason) => format!("positive value is the user-entered variable spring hanger cold load, not applied in the solve because {}; the spring acts with its rate only", reason.reason()),
    }
}

/// Once per solve, on normalized values: a warning for each variable spring
/// hanger whose entered installed load differs from its cold load. The hot
/// load is review data and warns nothing.
pub(crate) fn append_model_diagnostics(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for support in model.supports.iter().filter(|support| is_variable_spring_hanger(support)) {
        let Some(hanger) = support.hanger.as_ref() else { continue };
        let (Some(installed), Some(cold)) = (hanger.installed_load.as_ref(), hanger.cold_load.as_ref()) else { continue };
        if installed.value != cold.value || installed.unit != cold.unit {
            diagnostics.push(diag(
                &format!("diagnostic:spring-hanger:{}:installed-differs-from-cold", stable_suffix(&support.id)),
                INSTALLED_DIFFERS,
                "warning",
                format!("variable spring hanger {}: its installed load ({} {}) differs from its cold load ({} {}); the solve applies the cold load, and the installed load is review data only", support.id, installed.value, installed.unit, cold.value, cold.unit),
                vec![support.id.clone(), "hanger.installed_load".to_string(), "hanger.cold_load".to_string()],
            ));
        }
    }
}

/// The message of a fixed-load gate code, naming its actual cause. A model
/// whose only fixed-load source is a constant-effort support keeps the
/// existing text byte for byte.
pub(crate) fn fixed_load_gate_message(code: &str, constant_effort: bool, springs: bool, existing: &str) -> String {
    if !springs {
        return existing.to_string();
    }
    let (source, acts, each) = if constant_effort {
        ("a constant-effort support's force and a variable spring hanger's cold load", "act", "each")
    } else {
        ("a variable spring hanger's cold load", "acts", "it")
    };
    match code {
        preview_physics::CONSTANT_EFFORT_UNSIGNED_OPERAND => format!("Combination withheld: {source} {acts} in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count {each} once per term with its sign discarded. The difference of two solved cases is available as a result state subtraction."),
        preview_physics::CONSTANT_EFFORT_OPERAND_MIXED => format!("Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts {source} cannot be decided."),
        _ => format!("Combination withheld: {source} would be counted by the sum of factors instead of once. The difference of two solved cases is available as a result state subtraction."),
    }
}

#[cfg(test)]
#[path = "spring_preload_tests.rs"]
pub(crate) mod tests;
