//! No silent combination row drops (Agent 0, 2026-10-23), at the one combination
//! reader. The rows are the actual solved case rows of the bundled demo with its
//! linear supports only (so its `mechanics` combination publishes), handed to
//! `append_combination_results` exactly as the solve hands them; each probe
//! removes or invalidates one source to reach a withholding branch that a valid
//! request does not otherwise reach once duplicate terms block before solving.
use super::*;
use crate::{
    qualified_combination_result_id, qualified_load_case_result_id,
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use std::collections::HashMap;

const COMBINATION: &str = "combination:C-OPER-ALT";
const FIRST: &str = "load:L-100";
const SECOND: &str = "load:L-200";

struct Fixture {
    model: PreviewModel,
    case_rows: Vec<ResultItem>,
    rows_by_base_id: HashMap<String, HashMap<String, ResultItem>>,
    published: Vec<ResultItem>,
}

fn fixture(mode: PreviewSolverMode) -> Fixture {
    let mut model: serde_json::Value =
        serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap();
    model["supports"]
        .as_array_mut()
        .unwrap()
        .retain(|support| matches!(support["family"].as_str(), Some("anchor" | "guide")));
    let request = serde_json::json!({"model": model, "materials": []});
    let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
    assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED");
    let of = |kind: &str| -> Vec<ResultItem> {
        envelope.results.iter().filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == kind)).cloned().collect()
    };
    let case_rows = of("load_case");
    let published = of("combination");
    assert!(!published.is_empty());
    // Staged as the solve stages them: a row's base id is its first-case id; the
    // second case's row carries it qualified (its `_v2` rows keep their own
    // case-specific ids, so they never pair across cases).
    let base_of: HashMap<String, String> = case_rows
        .iter()
        .filter(|r| r.basis_ref.as_ref().unwrap().ref_id == FIRST && !r.kind.ends_with("_v2"))
        .map(|r| (qualified_load_case_result_id(SECOND, &r.id), r.id.clone()))
        .collect();
    let mut rows_by_base_id: HashMap<String, HashMap<String, ResultItem>> = HashMap::new();
    for row in &case_rows {
        let case = row.basis_ref.as_ref().unwrap().ref_id.clone();
        let base = if case == SECOND && !row.kind.ends_with("_v2") { base_of[&row.id].clone() } else { row.id.clone() };
        rows_by_base_id.entry(base).or_default().insert(case, row.clone());
    }
    let model: PreviewModel = serde_json::from_value(request["model"].clone()).unwrap();
    Fixture { model, case_rows, rows_by_base_id, published }
}

fn rendered() -> Rendered {
    Rendered { preview_cases: Vec::new(), nonlinear: false, consuming_constant_effort: false }
}

/// The combination rows and the combination-reader diagnostics for `fixture`.
fn combine(fixture: &Fixture) -> (Vec<ResultItem>, Vec<Diagnostic>) {
    let mut results = fixture.case_rows.clone();
    let mut diagnostics = Vec::new();
    append_combination_results(&fixture.model, &rendered(), &fixture.rows_by_base_id, &mut results, &mut diagnostics);
    (results.split_off(fixture.case_rows.len()), diagnostics)
}

fn bytes(rows: &[ResultItem]) -> Vec<u8> {
    serde_json::to_vec(rows).unwrap()
}

fn withheld(diagnostics: &[Diagnostic]) -> Vec<&Diagnostic> {
    diagnostics.iter().filter(|d| d.code == "LOAD_COMBINATION_ROW_WITHHELD").collect()
}

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

/// Control: the reader reproduces the published combination rows byte for byte
/// and adds no withheld or missing-source diagnostic.
#[test]
fn the_reader_reproduces_the_published_combination_rows() {
    for mode in MODES {
        let fixture = fixture(mode);
        let (rows, diagnostics) = combine(&fixture);
        assert_eq!(bytes(&rows), bytes(&fixture.published), "{mode:?}");
        let codes: Vec<&str> = diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, ["COMBINATION_STRESS_MAXIMUM_UNAVAILABLE", "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE"], "{mode:?}");
    }
}

/// A source row missing in the first (reference) operand is reported like one
/// missing in any other operand.
#[test]
fn a_row_missing_in_the_first_operand_is_reported() {
    for mode in MODES {
        let fixture = fixture(mode);
        let base = fixture.rows_by_base_id.iter().find(|(_, rows)| rows.get(FIRST).is_some_and(|r| r.kind == "element_local_axial_force")).map(|(id, _)| id.clone()).unwrap();
        for (removed, kept) in [(FIRST, SECOND), (SECOND, FIRST)] {
            let mut probe = Fixture { model: fixture.model.clone(), case_rows: fixture.case_rows.clone(), rows_by_base_id: fixture.rows_by_base_id.clone(), published: fixture.published.clone() };
            probe.rows_by_base_id.get_mut(&base).unwrap().remove(removed);
            assert!(probe.rows_by_base_id[&base].contains_key(kept));
            let (rows, diagnostics) = combine(&probe);
            let combined = qualified_combination_result_id(COMBINATION, &base);
            assert!(fixture.published.iter().any(|r| r.id == combined), "{mode:?}: the control publishes {combined}");
            assert!(!rows.iter().any(|r| r.id == combined), "{mode:?}/{removed}");
            let missing: Vec<&Diagnostic> = diagnostics.iter().filter(|d| d.code == "LOAD_COMBINATION_SOURCE_RESULT_MISSING").collect();
            assert_eq!(missing.len(), 1, "{mode:?}/{removed}: {diagnostics:?}");
            assert!(missing[0].message.contains(&base), "{mode:?}/{removed}: {}", missing[0].message);
            assert_eq!(missing[0].affected_refs, vec![COMBINATION.to_string()]);
        }
    }
}

/// An ordinary row the load-case algebra refuses is withheld by name.
#[test]
fn an_algebra_refused_row_is_withheld_by_name() {
    for mode in MODES {
        let mut fixture = fixture(mode);
        let base = fixture.rows_by_base_id.iter().find(|(_, rows)| rows.get(FIRST).is_some_and(|r| r.kind == "element_local_bending_moment_y")).map(|(id, _)| id.clone()).unwrap();
        fixture.rows_by_base_id.get_mut(&base).unwrap().get_mut(SECOND).unwrap().value = f64::NAN;
        let (rows, diagnostics) = combine(&fixture);
        assert_eq!(rows.len(), fixture.published.len() - 1, "{mode:?}");
        let named = withheld(&diagnostics);
        assert_eq!(named.len(), 1, "{mode:?}: {diagnostics:?}");
        assert_eq!(named[0].severity, "warning");
        assert_eq!(named[0].affected_refs, vec![COMBINATION.to_string()]);
        assert!(named[0].message.contains(COMBINATION) && named[0].message.contains(&base), "{}", named[0].message);
    }
}

/// A support group publishes whole or not at all, and a withheld group is named.
#[test]
fn a_support_group_is_never_published_partially() {
    for mode in MODES {
        let mut fixture = fixture(mode);
        let support = "support:S-100";
        let my = fixture
            .case_rows
            .iter_mut()
            .find(|r| {
                r.entity_ref == support
                    && r.basis_ref.as_ref().unwrap().ref_id == SECOND
                    && r.kind.starts_with("support_reaction_")
                    && r.metadata.as_ref().is_some_and(|m| m.component == "My")
            })
            .unwrap();
        my.value = f64::NAN;
        let (rows, diagnostics) = combine(&fixture);
        let support_rows = |rows: &[ResultItem]| rows.iter().filter(|r| r.kind.starts_with("support_reaction_") && r.entity_ref == support).count();
        assert_eq!(support_rows(&fixture.published), 8, "{mode:?}");
        assert_eq!(support_rows(&rows), 0, "{mode:?}: no partial support group");
        assert_eq!(rows.len(), fixture.published.len() - 8, "{mode:?}");
        let named = withheld(&diagnostics);
        assert_eq!(named.len(), 1, "{mode:?}: {diagnostics:?}");
        assert!(named[0].message.contains(support), "{}", named[0].message);
    }
}

/// The withheld list is bounded: eight rows are named, the rest counted.
#[test]
fn the_withheld_list_is_bounded() {
    for mode in MODES {
        let mut fixture = fixture(mode);
        let mut bases: Vec<String> = fixture.rows_by_base_id.iter().filter(|(_, rows)| rows.get(FIRST).is_some_and(|r| r.kind == "element_local_axial_force")).map(|(id, _)| id.clone()).collect();
        bases.sort();
        assert!(bases.len() > 8);
        for base in &bases {
            fixture.rows_by_base_id.get_mut(base).unwrap().get_mut(SECOND).unwrap().value = f64::NAN;
        }
        let (_, diagnostics) = combine(&fixture);
        let named = withheld(&diagnostics);
        assert_eq!(named.len(), 1, "{mode:?}");
        let listed = bases.iter().filter(|base| named[0].message.contains(base.as_str())).count();
        assert_eq!(listed, 8, "{}", named[0].message);
        assert!(named[0].message.contains(&format!("and {} more", bases.len() - 8)), "{}", named[0].message);
    }
}
