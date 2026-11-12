//! A pressurized member drops its replaced rows from `member_rows_from` on, not
//! over the whole case: the narrower compaction gives the vector the whole-case
//! retain gave, on synthetic rows that pin its bounds and on real solved case
//! rows of straight and arc members in both solver modes. The reference is the
//! replaced whole retain, kept here verbatim.
use super::*;
use crate::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::Value;

/// The replaced whole-case retain.
fn whole_retain(results: &mut Vec<ResultItem>, pipe_id: &str) {
    results.retain(|r| {
        r.entity_ref != pipe_id
            || !matches!(
                r.kind.as_str(),
                "element_local_axial_force"
                    | "element_local_axial_normal_stress"
                    | "pipe_section_pressure_hoop_stress"
                    | "pipe_section_pressure_longitudinal_stress"
            )
    });
}

fn row(id: &str, kind: &str, entity: &str) -> ResultItem {
    ResultItem {
        id: id.into(),
        kind: kind.into(),
        value: 0.0,
        unit: "N".into(),
        entity_ref: entity.into(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: None,
        component_body: None,
    }
}

fn ids(rows: &[ResultItem]) -> Vec<&str> {
    rows.iter().map(|r| r.id.as_str()).collect()
}

/// Rows before `from` are never read, even one the member would replace (the
/// product never has one there); every replaced row from `from` on is dropped,
/// the first one at `from` included; the rest keep their order.
#[test]
fn the_compaction_drops_exactly_the_members_replaced_rows_from_its_start() {
    let rows = || {
        vec![
            row("a", "element_local_axial_force", "pipe:q"),
            row("b", "pipe_wall_axial_force_v2", "pipe:p"),
            row("c", "element_local_axial_force", "pipe:p"),
            // from = 3
            row("d", "element_local_axial_force", "pipe:p"),
            row("e", "element_local_shear_force_y", "pipe:p"),
            row("f", "pipe_section_pressure_hoop_stress", "pipe:q"),
            row("g", "element_local_axial_normal_stress", "pipe:p"),
            row("h", "pipe_section_pressure_hoop_stress", "pipe:p"),
            row("i", "element_local_bending_moment_y", "pipe:p"),
            row("j", "pipe_section_pressure_longitudinal_stress", "pipe:p"),
        ]
    };
    let mut results = rows();
    drop_replaced_rows(&mut results, 3, "pipe:p");
    assert_eq!(ids(&results), ["a", "b", "c", "e", "f", "i"]);
    // From the start it is the whole retain.
    let (mut narrow, mut whole) = (rows(), rows());
    drop_replaced_rows(&mut narrow, 0, "pipe:p");
    whole_retain(&mut whole, "pipe:p");
    assert_eq!(ids(&narrow), ids(&whole));
    // At or past the end nothing is read or dropped.
    for from in [10, 11] {
        let mut results = rows();
        drop_replaced_rows(&mut results, from, "pipe:p");
        assert_eq!(ids(&results), ids(&rows()));
    }
}

const I7_DOCUMENTS: &str = include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");

fn solve(id: &str, mode: PreviewSolverMode) -> crate::MechanicsEnvelope {
    let documents: Value = serde_json::from_str(I7_DOCUMENTS).unwrap();
    let request = crate::b7b_admission_tests::reauthor_current_request(documents["sketches"][id]["0.3.0"]["document"].clone());
    let request = crate::pressure_runtime::with_pressurised_cases_named(&request);
    run_linear_static_preview_value_with_mode(request, mode).unwrap_or_else(|e| panic!("{id}: {e}"))
}

/// On real case rows the narrower compaction and the whole retain agree: for every
/// pressurized member of every case, the case's rows before the member's first row
/// (what `member_rows_from` marks) followed by the member's own rows and a fresh set
/// of its elastic end rows give the same vector; and no published row of a case is
/// one its pressurized members replace.
#[test]
fn on_real_case_rows_the_narrower_compaction_is_the_whole_retain() {
    let (mut arcs, mut straights) = (0, 0);
    for id in ["U2-L-ANCH-PTW-K2", "MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K2", "U2-U-ANCH-ALL-K2"] {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let envelope = solve(id, mode);
            let evidence = envelope.contract_evidence.as_ref().expect("contract evidence");
            let regions = evidence["pressure"].as_array().expect("pressure evidence");
            assert!(!regions.is_empty(), "{id}: a pressurized case");
            for region in regions {
                let case = region["load_case_id"].as_str().unwrap();
                let case_rows: Vec<ResultItem> = envelope
                    .results
                    .iter()
                    .filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case))
                    .cloned()
                    .collect();
                for member in region["member_pipe_ids"].as_array().unwrap().iter().map(|m| m.as_str().unwrap()) {
                    assert!(!case_rows.iter().any(|r| replaced(r, member)), "{id} {case} {member}: a replaced row is published");
                    let from = case_rows.iter().position(|r| r.entity_ref == member).expect("the member's rows");
                    let own: Vec<ResultItem> = case_rows[from..].iter().filter(|r| r.entity_ref == member).cloned().collect();
                    if own.iter().any(|r| r.kind == "pipe_toroidal_membrane_hoop_stress_v1") {
                        arcs += 1;
                    } else {
                        straights += 1;
                    }
                    let mut rows = case_rows[..from].to_vec();
                    crate::append_element_force_results(&mut rows, member, &[1.0; 12]);
                    rows.extend(own);
                    assert!(rows[from..].iter().any(|r| replaced(r, member)), "the tail carries rows to drop");
                    let (mut narrow, mut whole) = (rows.clone(), rows);
                    drop_replaced_rows(&mut narrow, from, member);
                    whole_retain(&mut whole, member);
                    assert_eq!(ids(&narrow), ids(&whole), "{id} {case} {member}");
                }
            }
        }
    }
    assert!(arcs > 0 && straights > 0, "arcs {arcs}, straights {straights}");
}
