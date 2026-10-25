//! One outcome per check and basis (rule-pack format 0.5.0; DEL-06-01 Design
//! `rule-pack-0.5.0.md` §2.6-§2.8 and §5 "Evaluation"). An invented two-case
//! model with combinations: load cases `load:A`, `load:B`, a superposition
//! `combination:AB`, a subtraction `combination:A-minus-B` and a range envelope
//! `combination:range`, each a basis the caller resolved. Category names,
//! values and limits are invented; nothing here is a professional or
//! code-compliance claim.

use open_pipe_stress_expression_evaluator::AnalysisStatus;
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, BasisRef, CheckOutcome, RefusedSolverResult,
    RuleCheckRunInput, RuleCheckRunResult, RuleCheckStatus, SolverResultBinding, SolverResultBound,
    SuppliedValueBinding, UnavailableBasis, RULE_INTERVAL_ALL_PASS,
};
use serde_json::{json, Value};

const CASES: [(&str, &str, &str); 5] = [
    ("rlc:a", "load_case", "load:A"),
    ("rlc:b", "load_case", "load:B"),
    ("rlc:ab", "combination", "combination:AB"),
    ("rlc:a-minus-b", "combination", "combination:A-minus-B"),
    ("rlc:range", "combination", "combination:range"),
];

fn basis(id: &str) -> BasisRef {
    let (_, kind, ref_id) = CASES.iter().find(|(_, _, ref_id)| *ref_id == id).unwrap();
    BasisRef::new(*kind, *ref_id)
}

fn reference(id: &str, ref_type: &str) -> Value {
    json!({ "ref_id": id, "ref_type": ref_type })
}

fn input_declaration(id: &str, source_kind: &str) -> Value {
    let mut input = json!({
        "input_id": id, "name": id, "source_kind": source_kind, "required_for": "rule_check",
        "provenance_required": true, "redistribution_status_required": true,
        "quantity_intent": { "dimension": "stress", "unit_ref": "MPa",
                             "unit_required": true, "dimension_check_required": true }
    });
    if source_kind == "solver_result" {
        input["solver_result_ref"] =
            json!({ "result_kind": "invented_stress", "entity_ref": "pipe:P-1" });
    }
    input
}

fn check(id: &str, category: &str, formula: &str, inputs: &[&str], slot: Option<&str>) -> Value {
    let mut check = json!({
        "check_id": id,
        "category_ref": reference(category, "stress_category"),
        "required_input_refs": inputs.iter().map(|i| reference(i, "required_input")).collect::<Vec<_>>(),
        "formula_ref": reference(formula, "formula"),
        "result_statuses": ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"],
        "diagnostic_policy": { "missing_input": "RULE_INPUT_MISSING", "evaluator_error": "RULE_EVALUATOR_ERROR" }
    });
    if let Some(slot) = slot {
        check["value_slot_refs"] = json!([reference(slot, "value_slot")]);
    }
    check
}

/// `mappings`: (category, required load case ids in order). Checks:
/// `ratio_check` (actual / limit <= 1, category A), `fixed_check` (a formula
/// with no solver input, category B) when category B is mapped.
fn pack(mappings: &[(&str, &[&str])]) -> Value {
    let mut checks = vec![check(
        "ratio_check",
        "cat:A",
        "ratio",
        &["actual", "limit"],
        Some("ratio_limit"),
    )];
    if mappings.iter().any(|(category, _)| *category == "cat:B") {
        checks.push(check("fixed_check", "cat:B", "fixed", &["limit"], None));
    }
    json!({
        "schema_version": "0.5.0",
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": "invented_bases_pack", "schema_version": "0.5.0" },
        "required_load_cases": CASES.iter().map(|(id, kind, ref_id)| json!({
            "required_load_case_id": id, "name": id, "basis_ref": { "ref_type": kind, "ref_id": ref_id }
        })).collect::<Vec<_>>(),
        "stress_categories": mappings.iter().map(|(category, _)| json!({
            "category_id": category, "name": format!("Invented category {category}")
        })).collect::<Vec<_>>(),
        "load_case_mappings": mappings.iter().map(|(category, cases)| json!({
            "mapping_id": format!("map:{category}"),
            "category_ref": reference(category, "stress_category"),
            "required_load_case_refs": cases.iter().map(|id| reference(id, "required_load_case")).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "required_inputs": [input_declaration("actual", "solver_result"), input_declaration("limit", "user_supplied_rule_value")],
        "formula_declarations": [
            { "formula_id": "ratio",
              "declaration_payload": { "expression_ast": { "node": "binary", "operator": "divide",
                  "left": { "node": "variable_ref", "variable_id": "actual" },
                  "right": { "node": "variable_ref", "variable_id": "limit" } } },
              "input_refs": [reference("actual", "required_input"), reference("limit", "required_input")] },
            { "formula_id": "fixed",
              "declaration_payload": { "expression_ast": { "node": "compare", "operator": "less_than_or_equal",
                  "left": { "node": "variable_ref", "variable_id": "limit" },
                  "right": { "node": "literal", "quantity": { "value": 500.0, "dimension": "stress", "unit_ref": "MPa" } } } },
              "input_refs": [reference("limit", "required_input")] }
        ],
        "value_slots": [{ "slot_id": "ratio_limit", "slot_kind": "ratio_limit",
            "quantity_intent": { "dimension": "dimensionless", "unit_ref": "ratio",
                                 "unit_required": true, "dimension_check_required": true } }],
        "check_definitions": checks
    })
}

/// The caller-resolved row value of `actual` on each basis (MPa): `load:A` 50,
/// `load:B` 150, the superposition 200, the subtraction A - B = -100, and the
/// range envelope's upper end 150.
fn solver(id: &str, value: f64) -> SolverResultBinding {
    SolverResultBinding {
        input_id: "actual".to_string(),
        basis_ref: basis(id),
        result_id: format!("result:invented:{id}"),
        value,
        unit: "MPa".to_string(),
    }
}

fn all_rows() -> Vec<SolverResultBinding> {
    vec![
        solver("load:A", 50.0),
        solver("load:B", 150.0),
        solver("combination:AB", 200.0),
        solver("combination:A-minus-B", -100.0),
        solver("combination:range", 150.0),
    ]
}

fn supplied() -> Vec<SuppliedValueBinding> {
    vec![
        SuppliedValueBinding {
            ref_id: "limit".into(),
            value: 100.0,
            unit: "MPa".into(),
            dimension: "stress".into(),
        },
        SuppliedValueBinding {
            ref_id: "ratio_limit".into(),
            value: 1.0,
            unit: "ratio".into(),
            dimension: "dimensionless".into(),
        },
    ]
}

fn input<'a>(
    document: &'a Value,
    solver_results: Vec<SolverResultBinding>,
    refused: Vec<RefusedSolverResult>,
    unavailable: Vec<UnavailableBasis>,
) -> RuleCheckRunInput<'a> {
    RuleCheckRunInput {
        rule_pack_document: document,
        solver_results,
        refused_solver_results: refused,
        unavailable_bases: unavailable,
        supplied_values: supplied(),
        library_values: Vec::new(),
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

/// Outcome identity: (check_id, basis_ref.ref_type, basis_ref.ref_id).
fn identity(outcome: &CheckOutcome) -> (String, String, String) {
    let basis = outcome.basis_ref.as_ref().expect("a basis");
    (
        outcome.check_id.clone(),
        basis.ref_type.clone(),
        basis.ref_id.clone(),
    )
}

fn outcome<'a>(result: &'a RuleCheckRunResult, check_id: &str, basis_id: &str) -> &'a CheckOutcome {
    let found: Vec<&CheckOutcome> = result
        .checks
        .iter()
        .filter(|o| {
            o.check_id == check_id && o.basis_ref.as_ref().is_some_and(|b| b.ref_id == basis_id)
        })
        .collect();
    assert_eq!(found.len(), 1, "{check_id} on {basis_id}");
    found[0]
}

fn ratio(outcome: &CheckOutcome) -> Option<f64> {
    outcome.computed_value.as_ref().map(|value| value.value)
}

#[test]
fn a_check_mapped_to_several_bases_is_evaluated_on_each_with_its_own_row() {
    let document = pack(&[(
        "cat:A",
        &["rlc:a", "rlc:b", "rlc:ab", "rlc:a-minus-b", "rlc:range"],
    )]);
    let result = run_rule_checks(&input(&document, all_rows(), vec![], vec![]));
    let identities: Vec<_> = result.checks.iter().map(identity).collect();
    assert_eq!(
        identities
            .iter()
            .map(|(_, _, id)| id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "load:A",
            "load:B",
            "combination:AB",
            "combination:A-minus-B",
            "combination:range"
        ],
        "mapping order"
    );
    for outcome in &result.checks {
        assert_eq!(outcome.category_ref, "cat:A");
        assert_eq!(outcome.check_id, "ratio_check");
    }
    let expected = [
        ("load:A", 0.5, RuleCheckStatus::UserRuleChecked),
        ("load:B", 1.5, RuleCheckStatus::UserRuleFailed),
        ("combination:AB", 2.0, RuleCheckStatus::UserRuleFailed),
        // Subtraction: A - B = -100 MPa, ratio -1.0 <= 1.0.
        (
            "combination:A-minus-B",
            -1.0,
            RuleCheckStatus::UserRuleChecked,
        ),
        // Range envelope: the caller bound its row (upper end 150 MPa).
        ("combination:range", 1.5, RuleCheckStatus::UserRuleFailed),
    ];
    for (id, value, status) in expected {
        let o = outcome(&result, "ratio_check", id);
        assert_eq!(ratio(o), Some(value), "{id}");
        assert_eq!(o.status, status, "{id}");
        let bound = o
            .bound_inputs
            .iter()
            .find(|b| b.input_id == "actual")
            .unwrap();
        assert_eq!(
            bound.result_id.as_deref(),
            Some(format!("result:invented:{id}").as_str()),
            "{id}"
        );
    }
    // The check fails if any basis fails; the aggregate rule is unchanged.
    assert_eq!(result.aggregate_status, RuleCheckStatus::UserRuleFailed);
    // Counts count outcomes: one check failing on three bases is three failed.
    let counts = result.status_counts();
    assert_eq!(
        (counts.failed, counts.checked, counts.incomplete),
        (3, 2, 0)
    );
}

#[test]
fn a_missing_or_unsolved_basis_is_incomplete_and_the_other_bases_are_still_reported() {
    let document = pack(&[("cat:A", &["rlc:a", "rlc:b", "rlc:ab"])]);
    for code in [
        "RULE_REQUIRED_LOAD_CASE_MISSING",
        "RULE_REQUIRED_LOAD_CASE_UNSOLVED",
    ] {
        // A row the caller still passed for load:B is never bound there.
        let unavailable = vec![UnavailableBasis {
            basis_ref: basis("load:B"),
            required_load_case_id: "rlc:b".to_string(),
            code: code.to_string(),
            message: "invented: load:B is not available".to_string(),
        }];
        let result = run_rule_checks(&input(&document, all_rows(), vec![], unavailable));
        assert_eq!(result.checks.len(), 3);
        let b = outcome(&result, "ratio_check", "load:B");
        assert_eq!(b.status, RuleCheckStatus::RuleInputsIncomplete, "{code}");
        assert_eq!(ratio(b), None, "{code}: never evaluated");
        assert_eq!(b.acceptability_relation, "none");
        assert_eq!(b.completeness_findings[0].code, code);
        assert_eq!(b.completeness_findings[0].subject_id, "rlc:b");
        assert!(b
            .bound_inputs
            .iter()
            .any(|i| i.input_id == "actual" && !i.supplied));
        // load:A passes and the combination fails, both still reported.
        assert_eq!(
            outcome(&result, "ratio_check", "load:A").status,
            RuleCheckStatus::UserRuleChecked
        );
        assert_eq!(
            outcome(&result, "ratio_check", "combination:AB").status,
            RuleCheckStatus::UserRuleFailed
        );
        // Incomplete ranks above failed; the failure stays counted.
        assert_eq!(
            result.aggregate_status,
            RuleCheckStatus::RuleInputsIncomplete
        );
        let counts = result.status_counts();
        assert_eq!(
            (counts.failed, counts.checked, counts.incomplete),
            (1, 1, 1)
        );
    }
}

#[test]
fn a_missing_or_ambiguous_row_is_incomplete_on_its_basis_and_never_a_pass() {
    let document = pack(&[("cat:A", &["rlc:a", "rlc:b"])]);
    for reason in [
        "RULE_SOLVER_RESULT_ROW_MISSING",
        "RULE_SOLVER_RESULT_ROW_AMBIGUOUS",
    ] {
        let refused = vec![RefusedSolverResult {
            input_id: "actual".to_string(),
            basis_ref: basis("load:A"),
            reason: reason.to_string(),
            message: Some(format!("invented: {reason}")),
        }];
        // Even a binding for load:A cannot rescue the refused input there.
        let result = run_rule_checks(&input(&document, all_rows(), refused, vec![]));
        let a = outcome(&result, "ratio_check", "load:A");
        assert_eq!(a.status, RuleCheckStatus::RuleInputsIncomplete, "{reason}");
        assert_eq!(ratio(a), None);
        assert!(a
            .completeness_findings
            .iter()
            .any(|f| f.code == reason && f.message == format!("invented: {reason}")));
        let b = outcome(&result, "ratio_check", "load:B");
        assert_eq!(
            b.status,
            RuleCheckStatus::UserRuleFailed,
            "{reason}: load:B unaffected"
        );
        assert_eq!(ratio(b), Some(1.5));
    }
}

#[test]
fn a_refused_retained_row_is_refused_on_its_basis_only() {
    let document = pack(&[("cat:A", &["rlc:a", "rlc:b"])]);
    let refused = vec![RefusedSolverResult {
        input_id: "actual".to_string(),
        basis_ref: basis("load:B"),
        reason: "RULE_QUANTITY_NOT_COVERED".to_string(),
        message: None,
    }];
    let result = run_rule_checks(&input(&document, all_rows(), refused, vec![]));
    let b = outcome(&result, "ratio_check", "load:B");
    assert_eq!(b.status, RuleCheckStatus::RuleInputsIncomplete);
    let bound = b
        .bound_inputs
        .iter()
        .find(|i| i.input_id == "actual")
        .unwrap();
    assert!(!bound.supplied);
    assert_eq!(
        bound.note.as_deref(),
        Some("refused: RULE_QUANTITY_NOT_COVERED")
    );
    assert_eq!(
        outcome(&result, "ratio_check", "load:A").status,
        RuleCheckStatus::UserRuleChecked
    );
}

#[test]
fn a_bound_applies_to_its_own_basis_only_and_a_bound_without_basis_is_refused() {
    let document = pack(&[("cat:A", &["rlc:a", "rlc:b"])]);
    let rows = input(&document, all_rows(), vec![], vec![]);
    let plain = run_rule_checks(&rows);
    let bounded = run_rule_checks_with_bounds(
        &rows,
        &[SolverResultBound {
            input_id: "actual".into(),
            basis_ref: Some(basis("load:A")),
            absolute_bound: 1.0,
        }],
    );
    let a = outcome(&bounded, "ratio_check", "load:A");
    assert_eq!(a.status, RuleCheckStatus::UserRuleChecked);
    assert!(a
        .evaluator_findings
        .iter()
        .any(|f| f.code == RULE_INTERVAL_ALL_PASS));
    // load:B is byte-identical to the run without bounds.
    assert_eq!(
        serde_json::to_string(outcome(&bounded, "ratio_check", "load:B")).unwrap(),
        serde_json::to_string(outcome(&plain, "ratio_check", "load:B")).unwrap()
    );
    // A bound naming no basis refuses its input on every basis.
    let refused = run_rule_checks_with_bounds(
        &rows,
        &[SolverResultBound {
            input_id: "actual".into(),
            basis_ref: None,
            absolute_bound: 1.0,
        }],
    );
    for id in ["load:A", "load:B"] {
        let o = outcome(&refused, "ratio_check", id);
        assert_eq!(o.status, RuleCheckStatus::RuleInputsIncomplete, "{id}");
        assert!(o
            .completeness_findings
            .iter()
            .any(|f| f.code == "RULE_EVALUATOR_ERROR" && f.subject_id == "actual"));
    }
}

#[test]
fn outcome_identity_is_stable_across_reruns_and_reordered_inputs() {
    let document = pack(&[
        ("cat:A", &["rlc:a", "rlc:b", "rlc:ab"]),
        ("cat:B", &["rlc:b", "rlc:range"]),
    ]);
    let first = run_rule_checks(&input(&document, all_rows(), vec![], vec![]));
    let mut reversed = all_rows();
    reversed.reverse();
    let second = run_rule_checks(&input(&document, reversed, vec![], vec![]));
    assert_eq!(
        serde_json::to_string(&first).unwrap(),
        serde_json::to_string(&second).unwrap()
    );
    let identities: Vec<_> = first.checks.iter().map(identity).collect();
    let mut unique = identities.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(
        unique.len(),
        identities.len(),
        "identities are unique: {identities:?}"
    );
    // check_id repeats once per basis; the basis tells the outcomes apart.
    assert_eq!(
        identities
            .iter()
            .filter(|(check, _, _)| check == "ratio_check")
            .count(),
        3
    );
}

/// A check with no solver inputs is evaluated on each of its bases; its
/// outcomes differ only by basis. A load case shared by two categories feeds
/// both of their checks.
#[test]
fn a_check_without_solver_inputs_and_a_load_case_shared_by_two_categories() {
    let document = pack(&[
        ("cat:A", &["rlc:a", "rlc:b"]),
        ("cat:B", &["rlc:b", "rlc:range"]),
    ]);
    let result = run_rule_checks(&input(&document, all_rows(), vec![], vec![]));
    let order: Vec<_> = result.checks.iter().map(identity).collect();
    assert_eq!(
        order
            .iter()
            .map(|(c, _, b)| format!("{c}@{b}"))
            .collect::<Vec<_>>(),
        vec![
            "ratio_check@load:A",
            "ratio_check@load:B",
            "fixed_check@load:B",
            "fixed_check@combination:range"
        ],
        "check order, then mapping order"
    );
    let shared_a = outcome(&result, "ratio_check", "load:B");
    let shared_b = outcome(&result, "fixed_check", "load:B");
    assert_eq!(
        (
            shared_a.category_ref.as_str(),
            shared_b.category_ref.as_str()
        ),
        ("cat:A", "cat:B")
    );
    let on_b = outcome(&result, "fixed_check", "load:B");
    let on_range = outcome(&result, "fixed_check", "combination:range");
    assert_eq!(on_b.status, RuleCheckStatus::UserRuleChecked);
    let strip = |o: &CheckOutcome| {
        let mut value = serde_json::to_value(o).unwrap();
        value.as_object_mut().unwrap().remove("basis_ref");
        value
    };
    assert_eq!(
        strip(on_b),
        strip(on_range),
        "outcomes differ only by basis"
    );
    assert_ne!(on_b.basis_ref, on_range.basis_ref);
}
