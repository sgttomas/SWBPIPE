//! End-to-end rule-check orchestration for user-owned rule packs (completion
//! plan C4 — "end-to-end rule checks on authored models").
//!
//! This crate wires three existing engine crates into one runnable path:
//! [`open_pipe_stress_completeness_checker`] (are the required inputs present),
//! [`open_pipe_stress_rule_pack_document`] (decode the frozen-grammar formula
//! AST), and [`open_pipe_stress_expression_evaluator`] (compute the formula and
//! the acceptability comparison). It produces, per check, one of the three
//! automatic rule-check statuses (`RULE_INPUTS_INCOMPLETE`,
//! `USER_RULE_CHECKED`, `USER_RULE_FAILED`) and aggregates them worst-of.
//!
//! It does not parse text, execute host-language code, access files, access the
//! network, spawn processes, load plugins, store private data, embed protected
//! standards content, or emit professional/certification/sealing/authentication/
//! approval/code-compliance claims. It is pure in-memory orchestration over a
//! caller-supplied rule-pack `serde_json::Value` and caller-resolved values.
//!
//! ## Acceptability composition (C4-defined; the schema deliberately leaves it
//! open — see the DEL-06-01 `TP-C2-SCHEMA-001` run record, §"Residuals and
//! hand-offs", and PRD §12.5 ratio-vs-allowable shape)
//!
//! - A check whose formula evaluates to a **boolean** uses that boolean
//!   directly as the acceptability predicate.
//! - A check whose formula evaluates to a **quantity** (e.g. a ratio) is
//!   compared against the single user-supplied limit named by the check's
//!   `value_slot_refs`, via a synthesized `Compare(formula, <relation>, limit)`
//!   evaluated through the same evaluator (so unit/dimension safety is enforced
//!   in one place). The relation is the check's optional `acceptability_relation`
//!   member (`TP-C4-ACCEPTREL-001`; one of the four ordering relations
//!   less_than / less_than_or_equal / greater_than / greater_than_or_equal),
//!   defaulting to `less_than_or_equal` when absent (backward compatible). An
//!   explicit but unrecognized token blocks the check rather than defaulting
//!   silently. The member is an additive PROPOSAL awaiting human ratification
//!   (precedent `DEC-038` / `library_value_ref`); equality acceptance is a
//!   deliberate non-goal of the member.
//!
//! ## Input binding (C4-defined minimal mechanism)
//!
//! The schema carries no selector tying a `solver_result` required input to a
//! solved result row, so the binding is **caller-supplied**: solver values via
//! [`SolverResultBinding`] (the caller extracts `{value, unit}` for an
//! `input_id` from a solved mechanics envelope), user values and limit slots
//! via [`SuppliedValueBinding`]. `private_library_value` inputs are resolved via
//! [`LibraryValueBinding`] (C3 rule-pack ↔ library reference wiring): the caller
//! reads `{value, unit}` from the referenced private library record's value slot
//! (the rule pack's `library_value_ref` member) in the local store. A
//! `private_library_value` input with no resolved library binding stays
//! unsupplied (never a silent pass) with a recorded note. The library value is
//! resolved at run time and never embedded in the rule pack (IP boundary).

use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;

use open_pipe_stress_completeness_checker::{
    check_completeness, AutomaticAnalysisStatus, CompletenessCheckInput,
    Dimension as CompletenessDimension, FindingSeverity, QuantityIntent, Readiness, RequiredFor,
    RequiredInput, SourceKind, SuppliedInput,
};
pub use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_expression_evaluator::{
    evaluate, BindingSource, ComparisonOperator, Dimension as EvalDimension, EvaluationInput,
    EvaluationValue, Expression, Quantity, VariableBinding,
};
use open_pipe_stress_rule_pack_document::{decode_dimension, decode_expression, encode_dimension};

/// Document kind tag on a serialized run result.
pub const DOCUMENT_KIND: &str = "openpipestress.rule_check.run";

/// Boundary notice carried on every run result. No professional/code-compliance
/// claim is made anywhere in this crate.
pub const PROFESSIONAL_BOUNDARY_NOTICE: &str =
    "Software rule-check evidence only; not a professional, certification, sealing, \
     authentication, approval, or code-compliance claim. Human review remains required.";

/// One solver value the caller resolved from a solved mechanics envelope for a
/// `solver_result` required input.
#[derive(Debug, Clone)]
pub struct SolverResultBinding {
    pub input_id: String,
    pub result_id: String,
    pub value: f64,
    pub unit: String,
}

/// One user-entered value for a `user_supplied_rule_value` required input or a
/// `value_slot` limit. `dimension` is the snake_case dimension token (e.g.
/// `"dimensionless"`, `"stress"`).
#[derive(Debug, Clone)]
pub struct SuppliedValueBinding {
    pub ref_id: String,
    pub value: f64,
    pub unit: String,
    pub dimension: String,
}

/// One value the caller resolved from a saved private library for a
/// `private_library_value` required input (C3 rule-pack ↔ library reference
/// wiring). The caller (the desktop command) reads `{value, unit}` from the
/// referenced library record's value slot in the local private-library store;
/// the `library_*`/`record_id`/`slot_id` fields are carried for the outcome's
/// provenance note only. The value is never embedded in the rule pack — it is
/// resolved at run time and stays in the private library (IP boundary).
#[derive(Debug, Clone)]
pub struct LibraryValueBinding {
    pub input_id: String,
    pub value: f64,
    pub unit: String,
    pub library_kind: String,
    pub library_id: String,
    pub record_id: String,
    pub slot_id: String,
}

/// Inputs to a rule-check run.
#[derive(Debug, Clone)]
pub struct RuleCheckRunInput<'a> {
    /// The full rule-pack document. The caller is expected to have already run
    /// document validation; the runner surfaces structural findings (missing
    /// references) but does not re-validate the checksum or lifecycle.
    pub rule_pack_document: &'a Value,
    pub solver_results: Vec<SolverResultBinding>,
    pub supplied_values: Vec<SuppliedValueBinding>,
    /// Values resolved from saved private libraries for `private_library_value`
    /// required inputs (caller-resolved from the local store).
    pub library_values: Vec<LibraryValueBinding>,
    /// Current analysis statuses from the solve (e.g. `[MechanicsSolved]`).
    pub current_statuses: Vec<AnalysisStatus>,
}

/// The three automatic rule-check statuses this runner may emit. The wider
/// status vocabulary (`MODEL_INCOMPLETE`, `MECHANICS_SOLVED`,
/// `HUMAN_REVIEW_REQUIRED`) and the external `HUMAN_APPROVED_FOR_PROJECT`
/// record are intentionally not producible here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RuleCheckStatus {
    #[serde(rename = "RULE_INPUTS_INCOMPLETE")]
    RuleInputsIncomplete,
    #[serde(rename = "USER_RULE_CHECKED")]
    UserRuleChecked,
    #[serde(rename = "USER_RULE_FAILED")]
    UserRuleFailed,
}

impl RuleCheckStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            RuleCheckStatus::RuleInputsIncomplete => "RULE_INPUTS_INCOMPLETE",
            RuleCheckStatus::UserRuleChecked => "USER_RULE_CHECKED",
            RuleCheckStatus::UserRuleFailed => "USER_RULE_FAILED",
        }
    }

    /// Worst-of precedence: failed (2) > incomplete (1) > checked (0).
    fn severity_rank(self) -> u8 {
        match self {
            RuleCheckStatus::UserRuleChecked => 0,
            RuleCheckStatus::RuleInputsIncomplete => 1,
            RuleCheckStatus::UserRuleFailed => 2,
        }
    }
}

/// A computed or limit quantity surfaced in a check outcome.
#[derive(Debug, Clone, Serialize)]
pub struct ComputedQuantity {
    pub value: f64,
    pub dimension: String,
    pub unit_ref: String,
}

/// What a single rule-pack input resolved to for a check.
#[derive(Debug, Clone, Serialize)]
pub struct BoundInput {
    pub input_id: String,
    pub source_kind: String,
    pub supplied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// A finding surfaced from completeness or evaluation, traced to its engine.
#[derive(Debug, Clone, Serialize)]
pub struct RunFinding {
    pub code: String,
    pub severity: String,
    pub subject_id: String,
    pub message: String,
}

/// The outcome of one `check_definition`.
#[derive(Debug, Clone, Serialize)]
pub struct CheckOutcome {
    pub check_id: String,
    pub status: RuleCheckStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_value: Option<ComputedQuantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_value: Option<ComputedQuantity>,
    /// The relation that governed the result: one of the four ordering relations
    /// for a quantity-vs-limit check (`less_than`, `less_than_or_equal`,
    /// `greater_than`, `greater_than_or_equal`; the check's `acceptability_relation`,
    /// defaulting to `less_than_or_equal` when absent), `"formula_predicate"`
    /// (boolean formula), or `"none"` (blocked before acceptability).
    pub acceptability_relation: String,
    pub bound_inputs: Vec<BoundInput>,
    pub completeness_findings: Vec<RunFinding>,
    pub evaluator_findings: Vec<RunFinding>,
    pub diagnostic_codes: Vec<String>,
}

/// The result of a rule-check run over all of a pack's checks.
#[derive(Debug, Clone, Serialize)]
pub struct RuleCheckRunResult {
    pub document_kind: String,
    pub rule_pack_id: String,
    pub grammar_version: String,
    pub aggregate_status: RuleCheckStatus,
    pub checks: Vec<CheckOutcome>,
    pub professional_boundary_notice: String,
}

/// Run every `check_definition` in the rule pack against the caller-resolved
/// values and return per-check outcomes plus a worst-of aggregate.
pub fn run_rule_checks(input: &RuleCheckRunInput) -> RuleCheckRunResult {
    let doc = input.rule_pack_document;
    let rule_pack_id = doc
        .pointer("/metadata/rule_pack_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let grammar_version = doc
        .pointer("/grammar_version")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let ctx = RunContext {
        required_index: index_by(doc, "required_inputs", "input_id"),
        formula_index: index_by(doc, "formula_declarations", "formula_id"),
        solver_by_input: input
            .solver_results
            .iter()
            .map(|b| (b.input_id.as_str(), b))
            .collect(),
        supplied_by_ref: input
            .supplied_values
            .iter()
            .map(|b| (b.ref_id.as_str(), b))
            .collect(),
        library_by_input: input
            .library_values
            .iter()
            .map(|b| (b.input_id.as_str(), b))
            .collect(),
        current_statuses: &input.current_statuses,
        grammar_version: grammar_version.as_str(),
    };

    let checks: Vec<CheckOutcome> = doc
        .get("check_definitions")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().map(|c| run_one_check(&ctx, c)).collect())
        .unwrap_or_default();

    let aggregate_status = if checks.is_empty() {
        RuleCheckStatus::RuleInputsIncomplete
    } else {
        checks
            .iter()
            .map(|c| c.status)
            .reduce(|acc, next| {
                if next.severity_rank() > acc.severity_rank() {
                    next
                } else {
                    acc
                }
            })
            .unwrap_or(RuleCheckStatus::RuleInputsIncomplete)
    };

    RuleCheckRunResult {
        document_kind: DOCUMENT_KIND.to_string(),
        rule_pack_id,
        grammar_version,
        aggregate_status,
        checks,
        professional_boundary_notice: PROFESSIONAL_BOUNDARY_NOTICE.to_string(),
    }
}

struct RunContext<'a> {
    required_index: HashMap<&'a str, &'a Value>,
    formula_index: HashMap<&'a str, &'a Value>,
    solver_by_input: HashMap<&'a str, &'a SolverResultBinding>,
    supplied_by_ref: HashMap<&'a str, &'a SuppliedValueBinding>,
    library_by_input: HashMap<&'a str, &'a LibraryValueBinding>,
    current_statuses: &'a [AnalysisStatus],
    grammar_version: &'a str,
}

/// A value resolved for a required input, ready to bind into the evaluator.
struct ResolvedValue {
    value: f64,
    unit: String,
    eval_dimension: Option<EvalDimension>,
    binding_source: BindingSource,
}

fn run_one_check(ctx: &RunContext, check: &Value) -> CheckOutcome {
    let check_id = check
        .pointer("/check_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let result_statuses: Vec<String> = check
        .get("result_statuses")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let diagnostic_policy = check.get("diagnostic_policy");

    let mut completeness_findings: Vec<RunFinding> = Vec::new();
    let mut evaluator_findings: Vec<RunFinding> = Vec::new();
    let mut diagnostic_codes: Vec<String> = Vec::new();
    let mut bound_inputs: Vec<BoundInput> = Vec::new();

    // --- Resolve the check's required inputs ---------------------------------
    let mut comp_required: Vec<RequiredInput> = Vec::new();
    let mut comp_supplied: Vec<SuppliedInput> = Vec::new();
    let mut resolved: HashMap<String, ResolvedValue> = HashMap::new();

    let input_refs = check
        .get("required_input_refs")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    for ref_obj in &input_refs {
        let ref_id = ref_obj
            .pointer("/ref_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if ref_id.is_empty() {
            continue;
        }
        let Some(req) = ctx.required_index.get(ref_id) else {
            completeness_findings.push(RunFinding {
                code: "INVALID_REFERENCE".to_string(),
                severity: "blocking".to_string(),
                subject_id: ref_id.to_string(),
                message: "check references an undeclared required input".to_string(),
            });
            continue;
        };

        let source_token = req
            .pointer("/source_kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let dimension_token = req
            .pointer("/quantity_intent/dimension")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let unit_ref = req
            .pointer("/quantity_intent/unit_ref")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let unit_required = req
            .pointer("/quantity_intent/unit_required")
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let dimension_check_required = req
            .pointer("/quantity_intent/dimension_check_required")
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let provenance_required = req
            .pointer("/provenance_required")
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let redistribution_status_required = req
            .pointer("/redistribution_status_required")
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let name = req
            .pointer("/name")
            .and_then(Value::as_str)
            .unwrap_or(ref_id)
            .to_string();
        let required_for_token = req
            .pointer("/required_for")
            .and_then(Value::as_str)
            .unwrap_or("rule_check");

        let source_kind = source_kind_from_token(source_token);
        let completeness_dimension = completeness_dimension_from_token(dimension_token);
        let eval_dimension = decode_dimension(dimension_token);

        comp_required.push(RequiredInput {
            input_id: ref_id.to_string(),
            name,
            source_kind,
            quantity_intent: QuantityIntent {
                dimension: completeness_dimension.unwrap_or(CompletenessDimension::Tbd),
                unit_ref: unit_ref.clone(),
                unit_required,
                dimension_check_required,
            },
            required_for: required_for_from_token(required_for_token),
            provenance_required,
            redistribution_status_required,
        });

        // Resolve the supplied value per source kind.
        let (value, unit, result_id, binding_source, note) = match source_kind {
            SourceKind::SolverResult => match ctx.solver_by_input.get(ref_id) {
                Some(b) => (
                    Some(b.value),
                    Some(b.unit.clone()),
                    Some(b.result_id.clone()),
                    BindingSource::SolverResultField,
                    None,
                ),
                None => (None, None, None, BindingSource::SolverResultField, None),
            },
            SourceKind::PrivateLibraryValue => match ctx.library_by_input.get(ref_id) {
                Some(b) => (
                    Some(b.value),
                    Some(b.unit.clone()),
                    None,
                    BindingSource::RulePackRequiredInput,
                    Some(format!(
                        "resolved from private library {}:{} record {} slot {} (value stays in the \
                         private library; never embedded in the rule pack)",
                        b.library_kind, b.library_id, b.record_id, b.slot_id
                    )),
                ),
                None => (
                    None,
                    None,
                    None,
                    BindingSource::RulePackRequiredInput,
                    Some(
                        "private_library_value input has no resolved library reference (missing \
                         library_value_ref, or the referenced library/record/slot was not found in \
                         the local private-library store); treated as unsupplied"
                            .to_string(),
                    ),
                ),
            },
            SourceKind::UserSuppliedRuleValue => match ctx.supplied_by_ref.get(ref_id) {
                Some(b) => (
                    Some(b.value),
                    Some(b.unit.clone()),
                    None,
                    BindingSource::UserSuppliedValue,
                    None,
                ),
                None => (None, None, None, BindingSource::UserSuppliedValue, None),
            },
            _ => match ctx.supplied_by_ref.get(ref_id) {
                Some(b) => (
                    Some(b.value),
                    Some(b.unit.clone()),
                    None,
                    BindingSource::RulePackRequiredInput,
                    None,
                ),
                None => (None, None, None, BindingSource::RulePackRequiredInput, None),
            },
        };

        bound_inputs.push(BoundInput {
            input_id: ref_id.to_string(),
            source_kind: source_token.to_string(),
            supplied: value.is_some(),
            value,
            unit: unit.clone(),
            result_id,
            note,
        });

        // A supplied value with a known dimension passes the completeness gates
        // via the private-reviewed constructor; otherwise the input is left
        // unsupplied so completeness blocks (never a silent pass).
        if let (Some(_), Some(u), Some(cd)) = (value, unit.clone(), completeness_dimension) {
            comp_supplied.push(SuppliedInput::private_reviewed(ref_id.to_string(), cd, u));
        }

        if let (Some(v), Some(u)) = (value, unit) {
            resolved.insert(
                ref_id.to_string(),
                ResolvedValue {
                    value: v,
                    unit: u,
                    eval_dimension,
                    binding_source,
                },
            );
        }
    }

    // --- Completeness -------------------------------------------------------
    let completeness = check_completeness(&CompletenessCheckInput {
        required_inputs: comp_required,
        supplied_inputs: comp_supplied,
        current_statuses: ctx
            .current_statuses
            .iter()
            .map(|s| to_automatic_status(*s))
            .collect(),
    });
    for f in &completeness.findings {
        completeness_findings.push(RunFinding {
            code: format!("{:?}", f.code),
            severity: severity_str(f.severity),
            subject_id: f.input_id.clone(),
            message: f.message.clone(),
        });
    }

    if completeness.readiness == Readiness::Blocked
        || completeness_findings
            .iter()
            .any(|f| f.severity == "blocking")
    {
        push_diagnostic(&mut diagnostic_codes, diagnostic_policy, "missing_input");
        let status = enforce_declared(
            RuleCheckStatus::RuleInputsIncomplete,
            &result_statuses,
            &mut completeness_findings,
        );
        return CheckOutcome {
            check_id,
            status,
            computed_value: None,
            limit_value: None,
            acceptability_relation: "none".to_string(),
            bound_inputs,
            completeness_findings,
            evaluator_findings,
            diagnostic_codes,
        };
    }

    // --- Decode + evaluate the formula --------------------------------------
    let formula_ref_id = check
        .pointer("/formula_ref/ref_id")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let Some(formula) = ctx.formula_index.get(formula_ref_id) else {
        evaluator_findings.push(RunFinding {
            code: "INVALID_REFERENCE".to_string(),
            severity: "blocking".to_string(),
            subject_id: formula_ref_id.to_string(),
            message: "check references an undeclared formula".to_string(),
        });
        return blocked_after_completeness(
            check_id,
            bound_inputs,
            completeness_findings,
            evaluator_findings,
            &mut diagnostic_codes,
            diagnostic_policy,
            &result_statuses,
        );
    };
    let Some(ast) = formula.pointer("/declaration_payload/expression_ast") else {
        evaluator_findings.push(RunFinding {
            code: "INVALID_REFERENCE".to_string(),
            severity: "blocking".to_string(),
            subject_id: formula_ref_id.to_string(),
            message: "formula declaration carries no expression_ast payload".to_string(),
        });
        return blocked_after_completeness(
            check_id,
            bound_inputs,
            completeness_findings,
            evaluator_findings,
            &mut diagnostic_codes,
            diagnostic_policy,
            &result_statuses,
        );
    };
    let expression = match decode_expression(ast) {
        Ok(expr) => expr,
        Err(err) => {
            evaluator_findings.push(RunFinding {
                code: "DECODE_ERROR".to_string(),
                severity: "blocking".to_string(),
                subject_id: formula_ref_id.to_string(),
                message: format!("{err:?}"),
            });
            return blocked_after_completeness(
                check_id,
                bound_inputs,
                completeness_findings,
                evaluator_findings,
                &mut diagnostic_codes,
                diagnostic_policy,
                &result_statuses,
            );
        }
    };

    // Bind the formula's variable references to the resolved values.
    let formula_input_refs = formula
        .get("input_refs")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut bindings: Vec<VariableBinding> = Vec::new();
    let mut required_variable_ids: Vec<String> = Vec::new();
    for fr in &formula_input_refs {
        let id = fr
            .pointer("/ref_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if id.is_empty() {
            continue;
        }
        required_variable_ids.push(id.to_string());
        match resolved.get(id) {
            Some(rv) => match rv.eval_dimension {
                Some(dim) => match Quantity::new(rv.value, dim, rv.unit.clone()) {
                    Ok(q) => bindings.push(VariableBinding::new(id, rv.binding_source, q)),
                    Err(_) => bindings.push(VariableBinding::missing(id, rv.binding_source)),
                },
                None => bindings.push(VariableBinding::missing(id, rv.binding_source)),
            },
            None => bindings.push(VariableBinding::missing(
                id,
                BindingSource::RulePackRequiredInput,
            )),
        }
    }

    let formula_eval = evaluate(&EvaluationInput {
        expression,
        bindings,
        required_variable_ids,
        statuses: ctx.current_statuses.to_vec(),
        declared_grammar_version: ctx.grammar_version.to_string(),
    });
    for f in &formula_eval.findings {
        evaluator_findings.push(RunFinding {
            code: format!("{:?}", f.code),
            severity: "blocking".to_string(),
            subject_id: f.subject_id.clone(),
            message: f.message.clone(),
        });
    }
    if !formula_eval.findings.is_empty() {
        return blocked_after_completeness(
            check_id,
            bound_inputs,
            completeness_findings,
            evaluator_findings,
            &mut diagnostic_codes,
            diagnostic_policy,
            &result_statuses,
        );
    }

    // --- Acceptability ------------------------------------------------------
    match formula_eval.value {
        Some(EvaluationValue::Boolean(passed)) => {
            let status = if passed {
                RuleCheckStatus::UserRuleChecked
            } else {
                RuleCheckStatus::UserRuleFailed
            };
            let status = enforce_declared(status, &result_statuses, &mut evaluator_findings);
            CheckOutcome {
                check_id,
                status,
                computed_value: None,
                limit_value: None,
                acceptability_relation: "formula_predicate".to_string(),
                bound_inputs,
                completeness_findings,
                evaluator_findings,
                diagnostic_codes,
            }
        }
        Some(EvaluationValue::Quantity(formula_quantity)) => {
            // The top-level acceptability relation (computed <relation> limit).
            // Absent => less_than_or_equal (backward compatible); an explicit but
            // unrecognized token blocks the check rather than defaulting silently.
            let acceptability_operator = match resolve_acceptability_relation(check) {
                Ok(operator) => operator,
                Err(token) => {
                    evaluator_findings.push(RunFinding {
                        code: "RULE_EVALUATOR_ERROR".to_string(),
                        severity: "blocking".to_string(),
                        subject_id: check_id.clone(),
                        message: format!(
                            "check declares an unsupported acceptability_relation '{token}'; \
                             expected one of less_than, less_than_or_equal, greater_than, \
                             greater_than_or_equal"
                        ),
                    });
                    push_diagnostic(&mut diagnostic_codes, diagnostic_policy, "evaluator_error");
                    let status = enforce_declared(
                        RuleCheckStatus::RuleInputsIncomplete,
                        &result_statuses,
                        &mut evaluator_findings,
                    );
                    return CheckOutcome {
                        check_id,
                        status,
                        computed_value: Some(quantity_to_computed(&formula_quantity)),
                        limit_value: None,
                        acceptability_relation: "none".to_string(),
                        bound_inputs,
                        completeness_findings,
                        evaluator_findings,
                        diagnostic_codes,
                    };
                }
            };
            let acceptability_label =
                acceptability_relation_label(&acceptability_operator).to_string();

            let computed_value = Some(quantity_to_computed(&formula_quantity));
            let limit = resolve_limit(check, ctx);
            let Some((limit_value, limit_unit, limit_dimension)) = limit else {
                evaluator_findings.push(RunFinding {
                    code: "RULE_INPUT_MISSING".to_string(),
                    severity: "blocking".to_string(),
                    subject_id: check_id.clone(),
                    message: "no user-supplied value-slot limit available for this check"
                        .to_string(),
                });
                push_diagnostic(&mut diagnostic_codes, diagnostic_policy, "missing_input");
                let status = enforce_declared(
                    RuleCheckStatus::RuleInputsIncomplete,
                    &result_statuses,
                    &mut evaluator_findings,
                );
                return CheckOutcome {
                    check_id,
                    status,
                    computed_value,
                    limit_value: None,
                    acceptability_relation: acceptability_label,
                    bound_inputs,
                    completeness_findings,
                    evaluator_findings,
                    diagnostic_codes,
                };
            };

            let limit_quantity = match limit_dimension
                .and_then(|d| Quantity::new(limit_value, d, limit_unit.clone()).ok())
            {
                Some(q) => q,
                None => {
                    evaluator_findings.push(RunFinding {
                        code: "RULE_EVALUATOR_ERROR".to_string(),
                        severity: "blocking".to_string(),
                        subject_id: check_id.clone(),
                        message: "value-slot limit has missing or unknown unit/dimension metadata"
                            .to_string(),
                    });
                    push_diagnostic(&mut diagnostic_codes, diagnostic_policy, "evaluator_error");
                    let status = enforce_declared(
                        RuleCheckStatus::RuleInputsIncomplete,
                        &result_statuses,
                        &mut evaluator_findings,
                    );
                    return CheckOutcome {
                        check_id,
                        status,
                        computed_value,
                        limit_value: None,
                        acceptability_relation: acceptability_label,
                        bound_inputs,
                        completeness_findings,
                        evaluator_findings,
                        diagnostic_codes,
                    };
                }
            };
            let limit_computed = Some(quantity_to_computed(&limit_quantity));

            let comparison = evaluate(&EvaluationInput {
                expression: Expression::Compare {
                    operator: acceptability_operator,
                    left: Box::new(Expression::Literal(formula_quantity)),
                    right: Box::new(Expression::Literal(limit_quantity)),
                },
                bindings: Vec::new(),
                required_variable_ids: Vec::new(),
                statuses: ctx.current_statuses.to_vec(),
                declared_grammar_version: ctx.grammar_version.to_string(),
            });
            for f in &comparison.findings {
                evaluator_findings.push(RunFinding {
                    code: format!("{:?}", f.code),
                    severity: "blocking".to_string(),
                    subject_id: f.subject_id.clone(),
                    message: f.message.clone(),
                });
            }

            let status = match comparison.value {
                Some(EvaluationValue::Boolean(true)) => RuleCheckStatus::UserRuleChecked,
                Some(EvaluationValue::Boolean(false)) => RuleCheckStatus::UserRuleFailed,
                _ => {
                    push_diagnostic(&mut diagnostic_codes, diagnostic_policy, "evaluator_error");
                    RuleCheckStatus::RuleInputsIncomplete
                }
            };
            let status = enforce_declared(status, &result_statuses, &mut evaluator_findings);
            CheckOutcome {
                check_id,
                status,
                computed_value,
                limit_value: limit_computed,
                acceptability_relation: acceptability_label,
                bound_inputs,
                completeness_findings,
                evaluator_findings,
                diagnostic_codes,
            }
        }
        None => blocked_after_completeness(
            check_id,
            bound_inputs,
            completeness_findings,
            evaluator_findings,
            &mut diagnostic_codes,
            diagnostic_policy,
            &result_statuses,
        ),
    }
}

/// Resolve a check's governing limit from the first `value_slot_refs` entry,
/// taking the numeric value from the caller's supplied-value map (keyed by the
/// slot id). Returns `(value, unit, eval_dimension)`.
fn resolve_limit(check: &Value, ctx: &RunContext) -> Option<(f64, String, Option<EvalDimension>)> {
    let slot_refs = check.get("value_slot_refs").and_then(Value::as_array)?;
    for slot_ref in slot_refs {
        let Some(slot_id) = slot_ref.pointer("/ref_id").and_then(Value::as_str) else {
            continue;
        };
        if let Some(binding) = ctx.supplied_by_ref.get(slot_id) {
            return Some((
                binding.value,
                binding.unit.clone(),
                decode_dimension(&binding.dimension),
            ));
        }
    }
    None
}

/// Resolve a check's acceptability relation — the top-level pass/fail comparison
/// of the computed formula quantity against the user-supplied value-slot limit
/// (`computed <relation> limit`) — to a frozen-grammar comparison operator.
///
/// An absent member resolves to `LessThanOrEqual`, preserving the historical
/// hard-coded behaviour for packs authored before the additive
/// `acceptability_relation` member existed (the sole documented default; the
/// editor authors the member explicitly, so new packs never rely on it). An
/// explicit but unrecognized token is never silently defaulted — `Err(token)`
/// is returned so the caller blocks the check (no-silent-defaults, CONTRACT).
/// The member is restricted to the four ordering relations; equality acceptance
/// is a deliberate non-goal (a boolean-predicate formula can express equality).
fn resolve_acceptability_relation(check: &Value) -> Result<ComparisonOperator, String> {
    match check
        .pointer("/acceptability_relation")
        .and_then(Value::as_str)
    {
        None => Ok(ComparisonOperator::LessThanOrEqual),
        Some(token) => match token.trim() {
            "" => Ok(ComparisonOperator::LessThanOrEqual),
            "less_than" => Ok(ComparisonOperator::LessThan),
            "less_than_or_equal" => Ok(ComparisonOperator::LessThanOrEqual),
            "greater_than" => Ok(ComparisonOperator::GreaterThan),
            "greater_than_or_equal" => Ok(ComparisonOperator::GreaterThanOrEqual),
            other => Err(other.to_string()),
        },
    }
}

/// Canonical string label for an acceptability relation operator, reported on
/// the check outcome so the GUI shows the relation that governed the result.
fn acceptability_relation_label(operator: &ComparisonOperator) -> &'static str {
    match operator {
        ComparisonOperator::LessThan => "less_than",
        ComparisonOperator::LessThanOrEqual => "less_than_or_equal",
        ComparisonOperator::GreaterThan => "greater_than",
        ComparisonOperator::GreaterThanOrEqual => "greater_than_or_equal",
        ComparisonOperator::Equal => "equal",
        ComparisonOperator::NotEqual => "not_equal",
    }
}

fn quantity_to_computed(quantity: &Quantity) -> ComputedQuantity {
    ComputedQuantity {
        value: quantity.value,
        dimension: encode_dimension(quantity.dimension).to_string(),
        unit_ref: quantity.unit_ref.clone(),
    }
}

#[allow(clippy::too_many_arguments)]
fn blocked_after_completeness(
    check_id: String,
    bound_inputs: Vec<BoundInput>,
    completeness_findings: Vec<RunFinding>,
    mut evaluator_findings: Vec<RunFinding>,
    diagnostic_codes: &mut Vec<String>,
    diagnostic_policy: Option<&Value>,
    result_statuses: &[String],
) -> CheckOutcome {
    push_diagnostic(diagnostic_codes, diagnostic_policy, "evaluator_error");
    let status = enforce_declared(
        RuleCheckStatus::RuleInputsIncomplete,
        result_statuses,
        &mut evaluator_findings,
    );
    CheckOutcome {
        check_id,
        status,
        computed_value: None,
        limit_value: None,
        acceptability_relation: "none".to_string(),
        bound_inputs,
        completeness_findings,
        evaluator_findings,
        diagnostic_codes: diagnostic_codes.clone(),
    }
}

/// Keep an emitted status inside the check's declared `result_statuses`
/// envelope. If a status the pack did not declare would be emitted, downgrade
/// to `RULE_INPUTS_INCOMPLETE` and record why.
fn enforce_declared(
    status: RuleCheckStatus,
    result_statuses: &[String],
    findings: &mut Vec<RunFinding>,
) -> RuleCheckStatus {
    if result_statuses.is_empty() || result_statuses.iter().any(|s| s == status.as_str()) {
        return status;
    }
    findings.push(RunFinding {
        code: "STATUS_NOT_DECLARED".to_string(),
        severity: "blocking".to_string(),
        subject_id: status.as_str().to_string(),
        message: "computed status is not in the check's declared result_statuses".to_string(),
    });
    RuleCheckStatus::RuleInputsIncomplete
}

fn push_diagnostic(codes: &mut Vec<String>, policy: Option<&Value>, key: &str) {
    if let Some(code) = policy
        .and_then(|p| p.get(key))
        .and_then(Value::as_str)
        .filter(|c| !c.is_empty())
    {
        if !codes.iter().any(|c| c == code) {
            codes.push(code.to_string());
        }
    }
}

fn index_by<'a>(doc: &'a Value, array_key: &str, id_key: &str) -> HashMap<&'a str, &'a Value> {
    let mut map = HashMap::new();
    if let Some(arr) = doc.get(array_key).and_then(Value::as_array) {
        for item in arr {
            if let Some(id) = item.get(id_key).and_then(Value::as_str) {
                map.insert(id, item);
            }
        }
    }
    map
}

fn source_kind_from_token(token: &str) -> SourceKind {
    match token {
        "solver_result" => SourceKind::SolverResult,
        "model_input" => SourceKind::ModelInput,
        "user_supplied_rule_value" => SourceKind::UserSuppliedRuleValue,
        "private_library_value" => SourceKind::PrivateLibraryValue,
        "owner_design_basis" => SourceKind::OwnerDesignBasis,
        _ => SourceKind::Tbd,
    }
}

fn required_for_from_token(token: &str) -> RequiredFor {
    match token {
        "rule_check" => RequiredFor::RuleCheck,
        "reporting" => RequiredFor::Reporting,
        "diagnostic" => RequiredFor::Diagnostic,
        _ => RequiredFor::Tbd,
    }
}

fn severity_str(severity: FindingSeverity) -> String {
    match severity {
        FindingSeverity::Info => "info",
        FindingSeverity::Warning => "warning",
        FindingSeverity::Blocking => "blocking",
    }
    .to_string()
}

fn to_automatic_status(status: AnalysisStatus) -> AutomaticAnalysisStatus {
    match status {
        AnalysisStatus::ModelIncomplete => AutomaticAnalysisStatus::ModelIncomplete,
        AnalysisStatus::MechanicsSolved => AutomaticAnalysisStatus::MechanicsSolved,
        AnalysisStatus::RuleInputsIncomplete => AutomaticAnalysisStatus::RuleInputsIncomplete,
        AnalysisStatus::UserRuleChecked => AutomaticAnalysisStatus::UserRuleChecked,
        AnalysisStatus::UserRuleFailed => AutomaticAnalysisStatus::UserRuleFailed,
        AnalysisStatus::HumanReviewRequired => AutomaticAnalysisStatus::HumanReviewRequired,
        AnalysisStatus::HumanApprovedForProject => AutomaticAnalysisStatus::HumanReviewRequired,
    }
}

/// Map a snake_case dimension token to the completeness checker's dimension
/// enum by routing through the document codec's evaluator-dimension decode
/// (single source of truth for the token vocabulary).
fn completeness_dimension_from_token(token: &str) -> Option<CompletenessDimension> {
    decode_dimension(token).map(eval_dimension_to_completeness)
}

fn eval_dimension_to_completeness(dimension: EvalDimension) -> CompletenessDimension {
    match dimension {
        EvalDimension::Dimensionless => CompletenessDimension::Dimensionless,
        EvalDimension::Length => CompletenessDimension::Length,
        EvalDimension::Mass => CompletenessDimension::Mass,
        EvalDimension::Time => CompletenessDimension::Time,
        EvalDimension::Temperature => CompletenessDimension::Temperature,
        EvalDimension::TemperatureInterval => CompletenessDimension::TemperatureInterval,
        EvalDimension::Angle => CompletenessDimension::Angle,
        EvalDimension::Rotation => CompletenessDimension::Rotation,
        EvalDimension::Force => CompletenessDimension::Force,
        EvalDimension::Moment => CompletenessDimension::Moment,
        EvalDimension::Pressure => CompletenessDimension::Pressure,
        EvalDimension::Stress => CompletenessDimension::Stress,
        EvalDimension::Area => CompletenessDimension::Area,
        EvalDimension::Volume => CompletenessDimension::Volume,
        EvalDimension::Density => CompletenessDimension::Density,
        EvalDimension::LinearStiffness => CompletenessDimension::LinearStiffness,
        EvalDimension::RotationalStiffness => CompletenessDimension::RotationalStiffness,
        EvalDimension::Displacement => CompletenessDimension::Displacement,
        EvalDimension::Velocity => CompletenessDimension::Velocity,
        EvalDimension::Acceleration => CompletenessDimension::Acceleration,
        EvalDimension::ThermalConductivity => CompletenessDimension::ThermalConductivity,
        EvalDimension::SpecificHeat => CompletenessDimension::SpecificHeat,
        EvalDimension::ThermalExpansionCoefficient => {
            CompletenessDimension::ThermalExpansionCoefficient
        }
        EvalDimension::SecondMomentArea => CompletenessDimension::SecondMomentArea,
        EvalDimension::SectionModulus => CompletenessDimension::SectionModulus,
        EvalDimension::MassPerLength => CompletenessDimension::MassPerLength,
        EvalDimension::VolumePerLength => CompletenessDimension::VolumePerLength,
        EvalDimension::Slope => CompletenessDimension::Slope,
        EvalDimension::Tbd => CompletenessDimension::Tbd,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A minimal invented rule pack: one ratio check `actual / limit` against a
    /// ratio limit value slot. All values are invented non-engineering content.
    fn demo_pack() -> Value {
        json!({
            "grammar_version": "1.0.0",
            "metadata": { "rule_pack_id": "test_demo_pack" },
            "required_inputs": [
                {
                    "input_id": "actual",
                    "name": "actual",
                    "source_kind": "solver_result",
                    "required_for": "rule_check",
                    "provenance_required": true,
                    "redistribution_status_required": true,
                    "quantity_intent": {
                        "dimension": "stress", "unit_ref": "demo_unit",
                        "unit_required": true, "dimension_check_required": true
                    }
                },
                {
                    "input_id": "limit",
                    "name": "limit",
                    "source_kind": "user_supplied_rule_value",
                    "required_for": "rule_check",
                    "provenance_required": true,
                    "redistribution_status_required": true,
                    "quantity_intent": {
                        "dimension": "stress", "unit_ref": "demo_unit",
                        "unit_required": true, "dimension_check_required": true
                    }
                }
            ],
            "formula_declarations": [
                {
                    "formula_id": "ratio",
                    "declaration_payload": {
                        "expression_ast": {
                            "node": "binary", "operator": "divide",
                            "left": { "node": "variable_ref", "variable_id": "actual" },
                            "right": { "node": "variable_ref", "variable_id": "limit" }
                        }
                    },
                    "input_refs": [
                        { "ref_id": "actual", "ref_type": "required_input" },
                        { "ref_id": "limit", "ref_type": "required_input" }
                    ]
                }
            ],
            "value_slots": [
                {
                    "slot_id": "ratio_limit", "slot_kind": "ratio_limit",
                    "quantity_intent": {
                        "dimension": "dimensionless", "unit_ref": "ratio",
                        "unit_required": true, "dimension_check_required": true
                    }
                }
            ],
            "check_definitions": [
                {
                    "check_id": "ratio_check",
                    "required_input_refs": [
                        { "ref_id": "actual", "ref_type": "required_input" },
                        { "ref_id": "limit", "ref_type": "required_input" }
                    ],
                    "value_slot_refs": [ { "ref_id": "ratio_limit", "ref_type": "value_slot" } ],
                    "formula_ref": { "ref_id": "ratio", "ref_type": "formula" },
                    "result_statuses": [
                        "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"
                    ],
                    "diagnostic_policy": {
                        "missing_input": "RULE_INPUT_MISSING",
                        "evaluator_error": "RULE_EVALUATOR_ERROR"
                    }
                }
            ]
        })
    }

    fn solver(input_id: &str, value: f64) -> SolverResultBinding {
        SolverResultBinding {
            input_id: input_id.to_string(),
            result_id: format!("result:stress:{input_id}"),
            value,
            unit: "demo_unit".to_string(),
        }
    }

    fn supplied(ref_id: &str, value: f64, unit: &str, dimension: &str) -> SuppliedValueBinding {
        SuppliedValueBinding {
            ref_id: ref_id.to_string(),
            value,
            unit: unit.to_string(),
            dimension: dimension.to_string(),
        }
    }

    fn run(
        pack: &Value,
        solver_results: Vec<SolverResultBinding>,
        supplied_values: Vec<SuppliedValueBinding>,
    ) -> RuleCheckRunResult {
        run_full(pack, solver_results, supplied_values, Vec::new())
    }

    fn run_full(
        pack: &Value,
        solver_results: Vec<SolverResultBinding>,
        supplied_values: Vec<SuppliedValueBinding>,
        library_values: Vec<LibraryValueBinding>,
    ) -> RuleCheckRunResult {
        run_rule_checks(&RuleCheckRunInput {
            rule_pack_document: pack,
            solver_results,
            supplied_values,
            library_values,
            current_statuses: vec![AnalysisStatus::MechanicsSolved],
        })
    }

    fn library(input_id: &str, value: f64, unit: &str) -> LibraryValueBinding {
        LibraryValueBinding {
            input_id: input_id.to_string(),
            value,
            unit: unit.to_string(),
            library_kind: "material".to_string(),
            library_id: "lib:demo".to_string(),
            record_id: "mat:demo".to_string(),
            slot_id: "allow:demo".to_string(),
        }
    }

    #[test]
    fn passing_check_yields_user_rule_checked() {
        let pack = demo_pack();
        let result = run(
            &pack,
            vec![solver("actual", 50.0)],
            vec![
                supplied("limit", 100.0, "demo_unit", "stress"),
                supplied("ratio_limit", 1.0, "ratio", "dimensionless"),
            ],
        );
        assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleChecked);
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::UserRuleChecked);
        assert_eq!(check.acceptability_relation, "less_than_or_equal");
        let computed = check.computed_value.as_ref().expect("ratio computed");
        assert_eq!(computed.value, 0.5);
        assert_eq!(computed.dimension, "dimensionless");
        assert_eq!(computed.unit_ref, "ratio");
        assert!(check.evaluator_findings.is_empty());
        assert!(check.completeness_findings.is_empty());
    }

    #[test]
    fn failing_check_yields_user_rule_failed() {
        let pack = demo_pack();
        let result = run(
            &pack,
            vec![solver("actual", 150.0)],
            vec![
                supplied("limit", 100.0, "demo_unit", "stress"),
                supplied("ratio_limit", 1.0, "ratio", "dimensionless"),
            ],
        );
        assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleFailed);
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::UserRuleFailed);
        assert_eq!(check.computed_value.as_ref().expect("ratio").value, 1.5);
    }

    #[test]
    fn missing_solver_input_blocks_without_evaluating() {
        let pack = demo_pack();
        // No solver binding for `actual`.
        let result = run(
            &pack,
            vec![],
            vec![
                supplied("limit", 100.0, "demo_unit", "stress"),
                supplied("ratio_limit", 1.0, "ratio", "dimensionless"),
            ],
        );
        assert_eq!(
            result.aggregate_status,
            RuleCheckStatus::RuleInputsIncomplete
        );
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
        // Formula must not have been evaluated.
        assert!(check.computed_value.is_none());
        assert_eq!(check.acceptability_relation, "none");
        assert!(check
            .diagnostic_codes
            .iter()
            .any(|c| c == "RULE_INPUT_MISSING"));
        assert!(check
            .bound_inputs
            .iter()
            .any(|b| b.input_id == "actual" && !b.supplied));
    }

    #[test]
    fn missing_limit_slot_blocks_as_incomplete() {
        let pack = demo_pack();
        let result = run(
            &pack,
            vec![solver("actual", 50.0)],
            vec![supplied("limit", 100.0, "demo_unit", "stress")],
        );
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
        // Formula evaluated (ratio computed) but no limit to compare against.
        assert_eq!(check.computed_value.as_ref().expect("ratio").value, 0.5);
        assert!(check.limit_value.is_none());
    }

    #[test]
    fn unit_mismatch_between_ratio_and_limit_blocks() {
        let pack = demo_pack();
        // Limit slot declared with a non-ratio unit/dimension -> comparison
        // against the dimensionless ratio is a blocking evaluator finding.
        let result = run(
            &pack,
            vec![solver("actual", 50.0)],
            vec![
                supplied("limit", 100.0, "demo_unit", "stress"),
                supplied("ratio_limit", 1.0, "demo_unit", "stress"),
            ],
        );
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
        assert!(check
            .diagnostic_codes
            .iter()
            .any(|c| c == "RULE_EVALUATOR_ERROR"));
    }

    #[test]
    fn status_outside_declared_result_statuses_is_downgraded() {
        let mut pack = demo_pack();
        // Declare only the incomplete status; a computed PASS must downgrade.
        pack["check_definitions"][0]["result_statuses"] = json!(["RULE_INPUTS_INCOMPLETE"]);
        let result = run(
            &pack,
            vec![solver("actual", 50.0)],
            vec![
                supplied("limit", 100.0, "demo_unit", "stress"),
                supplied("ratio_limit", 1.0, "ratio", "dimensionless"),
            ],
        );
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
        assert!(check
            .evaluator_findings
            .iter()
            .any(|f| f.code == "STATUS_NOT_DECLARED"));
    }

    #[test]
    fn aggregate_is_worst_of_across_checks() {
        let mut pack = demo_pack();
        // Second value slot with a stricter limit, and a second check that uses
        // it so the same ratio (0.5) passes the first check and fails the
        // second — a genuine pass+fail mix.
        pack["value_slots"].as_array_mut().unwrap().push(json!({
            "slot_id": "ratio_limit_strict", "slot_kind": "ratio_limit",
            "quantity_intent": {
                "dimension": "dimensionless", "unit_ref": "ratio",
                "unit_required": true, "dimension_check_required": true
            }
        }));
        let mut second = pack["check_definitions"][0].clone();
        second["check_id"] = json!("ratio_check_strict");
        second["value_slot_refs"] =
            json!([{ "ref_id": "ratio_limit_strict", "ref_type": "value_slot" }]);
        pack["check_definitions"]
            .as_array_mut()
            .unwrap()
            .push(second);

        let result = run(
            &pack,
            vec![solver("actual", 50.0)],
            vec![
                supplied("limit", 100.0, "demo_unit", "stress"),
                supplied("ratio_limit", 1.0, "ratio", "dimensionless"),
                supplied("ratio_limit_strict", 0.25, "ratio", "dimensionless"),
            ],
        );
        assert_eq!(result.checks.len(), 2);
        assert_eq!(result.checks[0].status, RuleCheckStatus::UserRuleChecked);
        assert_eq!(result.checks[1].status, RuleCheckStatus::UserRuleFailed);
        assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleFailed);
    }

    #[test]
    fn private_library_value_is_unsupplied_not_silently_passed() {
        let mut pack = demo_pack();
        pack["required_inputs"][1]["source_kind"] = json!("private_library_value");
        // Provide a value under the id anyway; it must be ignored for a
        // library-sourced input in this slice.
        let result = run(
            &pack,
            vec![solver("actual", 50.0)],
            vec![
                supplied("limit", 100.0, "demo_unit", "stress"),
                supplied("ratio_limit", 1.0, "ratio", "dimensionless"),
            ],
        );
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::RuleInputsIncomplete);
        let limit_binding = check
            .bound_inputs
            .iter()
            .find(|b| b.input_id == "limit")
            .expect("limit bound input present");
        assert!(!limit_binding.supplied);
        assert!(limit_binding.note.is_some());
    }

    #[test]
    fn private_library_value_resolves_from_library_binding_and_passes() {
        let mut pack = demo_pack();
        // `limit` is now sourced from a private library; its value is resolved
        // by the caller (the desktop command reads the library record's slot)
        // and passed as a LibraryValueBinding. ratio_limit stays user-supplied.
        pack["required_inputs"][1]["source_kind"] = json!("private_library_value");
        let result = run_full(
            &pack,
            vec![solver("actual", 50.0)],
            vec![supplied("ratio_limit", 1.0, "ratio", "dimensionless")],
            vec![library("limit", 100.0, "demo_unit")],
        );
        assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleChecked);
        let check = &result.checks[0];
        assert_eq!(check.status, RuleCheckStatus::UserRuleChecked);
        assert_eq!(check.computed_value.as_ref().expect("ratio").value, 0.5);
        let limit_binding = check
            .bound_inputs
            .iter()
            .find(|b| b.input_id == "limit")
            .expect("limit bound input present");
        assert!(limit_binding.supplied);
        let note = limit_binding.note.as_ref().expect("library note");
        assert!(note.contains("private library"));
        assert!(note.contains("lib:demo"));
        assert!(note.contains("never embedded"));
    }

    #[test]
    fn private_library_value_with_failing_library_value_reports_failure() {
        let mut pack = demo_pack();
        pack["required_inputs"][1]["source_kind"] = json!("private_library_value");
        // actual/limit = 50/40 = 1.25 > ratio_limit 1.0 -> fail.
        let result = run_full(
            &pack,
            vec![solver("actual", 50.0)],
            vec![supplied("ratio_limit", 1.0, "ratio", "dimensionless")],
            vec![library("limit", 40.0, "demo_unit")],
        );
        assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleFailed);
    }

    #[test]
    fn emits_only_the_automatic_status_vocabulary() {
        let pack = demo_pack();
        let result = run(
            &pack,
            vec![solver("actual", 50.0)],
            vec![
                supplied("limit", 100.0, "demo_unit", "stress"),
                supplied("ratio_limit", 1.0, "ratio", "dimensionless"),
            ],
        );
        let serialized = serde_json::to_string(&result).expect("serializes");
        for forbidden in [
            "HUMAN_APPROVED_FOR_PROJECT",
            "Certified",
            "Sealed",
            "Approved",
            "code_compliant",
        ] {
            assert!(
                !serialized.contains(forbidden),
                "output must not contain {forbidden}"
            );
        }
        assert!(serialized.contains("USER_RULE_CHECKED"));
    }
}
