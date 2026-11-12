//! Pressure evidence `result_ids` from one pass over the case's rows answer as the
//! replaced per-entry scan, kept here verbatim as the oracle.
use super::*;
use serde_json::{json, Value};
use crate::{run_linear_static_preview_value_with_mode, PreviewSolverMode};

/// The replaced scan: for each entry, its region's members, then every row.
fn scan(evidence: &mut [Value], members: &[(&str, &str)], results: &[ResultItem]) {
    for evidence in evidence {
        let region_id = evidence.get("region_id").and_then(|id| id.as_str()).unwrap_or("");
        let members = members.iter().filter(|(_, region)| *region == region_id).map(|(id, _)| *id).collect::<HashSet<_>>();
        evidence["result_ids"] = serde_json::json!(results
            .iter()
            .filter(|r| members.contains(r.entity_ref.as_str()) && r.kind.starts_with("pipe_") && r.kind.ends_with("_v2"))
            .map(|r| &r.id)
            .collect::<Vec<_>>());
    }
}

fn row(id: &str, kind: &str, entity: &str) -> ResultItem {
    ResultItem { id: id.into(), kind: kind.into(), value: 0.0, unit: "N".into(), entity_ref: entity.into(), basis_ref: None,
        source_result_refs: Vec::new(), metadata: None, component_body: None }
}

/// Every entry gets `result_ids`: one naming no region reads "" (empty unless a
/// member's region is ""), two naming one region each get its list, identifiers
/// are listed in row order (not sorted), and a row of another entity whose
/// identifier equals a member's is listed whenever its kind matches.
#[test]
fn result_ids_answer_as_the_per_entry_scan() {
    let results = vec![
        row("z:3", "pipe_wall_axial_force_v2", "pipe:b"),
        row("a:1", "pipe_wall_axial_force_v2", "pipe:a"),
        row("m:2", "pipe_lame_hoop_stress_v2", "pipe:b"),
        row("b:0", "element_local_shear_force_y", "pipe:a"),
        row("c:9", "pipe_toroidal_membrane_hoop_stress_v1", "pipe:a"),
        row("d:8", "support_reaction_force_magnitude_v2", "pipe:a"),
        // Another entity named as pipe:a, with a matching kind.
        row("e:7", "pipe_effective_axial_force_v2", "pipe:a"),
        row("f:6", "pipe_wall_axial_force_v2", "pipe:c"),
        row("g:5", "pipe_wall_axial_force_v2", "pipe:x"),
    ];
    let members = [("pipe:a", "region:A"), ("pipe:b", "region:A"), ("pipe:c", "")];
    let entries = || vec![json!({"region_id": "region:A"}), json!({"note": "no region"}), json!({"region_id": "region:A"}),
        json!({"region_id": "region:Z"}), json!({"region_id": 3})];
    let (mut indexed, mut scanned) = (entries(), entries());
    attach_pressure_result_ids(&mut indexed, members.iter().copied(), &results);
    scan(&mut scanned, &members, &results);
    assert_eq!(indexed, scanned);
    assert_eq!(indexed[0]["result_ids"], json!(["z:3", "a:1", "m:2", "e:7"]), "row order, collision included");
    assert_eq!(indexed[2]["result_ids"], indexed[0]["result_ids"], "two entries of one region");
    assert_eq!(indexed[1]["result_ids"], json!(["f:6"]), "no region reads \"\"");
    assert_eq!(indexed[3]["result_ids"], json!([]));
    assert_eq!(indexed[4]["result_ids"], json!(["f:6"]));
    // With no member in region "", an entry naming no region gets an empty list.
    let mut none = vec![json!({})];
    attach_pressure_result_ids(&mut none, members[..2].iter().copied(), &results);
    assert_eq!(none[0]["result_ids"], json!([]));
}

/// Outside the product's validation (a pipe in two regions, or one identifier
/// listed twice in a region), the helper still gives exactly what the replaced
/// scan gives: a row is listed under every region naming its pipe.
#[test]
fn a_repeated_element_identifier_gives_the_replaced_scans_lists() {
    let results = vec![
        row("a:1", "pipe_wall_axial_force_v2", "pipe:a"),
        row("b:2", "pipe_wall_axial_force_v2", "pipe:b"),
        row("a:3", "pipe_lame_hoop_stress_v2", "pipe:a"),
        row("c:4", "pipe_wall_axial_force_v2", "pipe:c"),
    ];
    let entries = || vec![json!({"region_id": "region:A"}), json!({"region_id": "region:B"}), json!({})];
    for members in [
        vec![("pipe:a", "region:A"), ("pipe:b", "region:B"), ("pipe:a", "region:B")],
        vec![("pipe:a", "region:B"), ("pipe:c", "region:A"), ("pipe:a", "region:A")],
        vec![("pipe:a", "region:A"), ("pipe:a", "region:A"), ("pipe:b", "region:B")],
    ] {
        let (mut indexed, mut scanned) = (entries(), entries());
        attach_pressure_result_ids(&mut indexed, members.iter().copied(), &results);
        scan(&mut scanned, &members, &results);
        assert_eq!(indexed, scanned, "{members:?}");
    }
    let mut both = entries();
    attach_pressure_result_ids(&mut both, [("pipe:a", "region:A"), ("pipe:b", "region:B"), ("pipe:a", "region:B")].into_iter(), &results);
    assert_eq!(both[0]["result_ids"], json!(["a:1", "a:3"]));
    assert_eq!(both[1]["result_ids"], json!(["a:1", "b:2", "a:3"]), "listed under both regions");
}

const I7_DOCUMENTS: &str = include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");

/// Through the solve (`run_linear_static_preview_value_with_mode`, which builds
/// the members from the case's pipe states): each entry of each case lists
/// exactly that case's pipe_*_v2 rows of the members the entry names, in row
/// order, on straight and arc members in both solver modes.
#[test]
fn solved_pressure_evidence_lists_each_regions_rows_in_row_order() {
    let documents: Value = serde_json::from_str(I7_DOCUMENTS).unwrap();
    let mut listed = 0;
    for id in ["U2-L-ANCH-PTW-K2", "MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K2", "U2-U-ANCH-ALL-K2"] {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let request = crate::b7b_admission_tests::reauthor_current_request(documents["sketches"][id]["0.3.0"]["document"].clone());
            let request = crate::pressure_runtime::with_pressurised_cases_named(&request);
            let envelope = run_linear_static_preview_value_with_mode(request, mode).unwrap_or_else(|e| panic!("{id}: {e}"));
            let entries = envelope.contract_evidence.as_ref().unwrap()["pressure"].as_array().unwrap().clone();
            assert!(!entries.is_empty(), "{id}");
            for entry in &entries {
                let case = entry["load_case_id"].as_str().unwrap();
                let members: HashSet<&str> = entry["member_pipe_ids"].as_array().unwrap().iter().map(|m| m.as_str().unwrap()).collect();
                let expected: Vec<&str> = envelope.results.iter()
                    .filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case))
                    .filter(|r| members.contains(r.entity_ref.as_str()) && r.kind.starts_with("pipe_") && r.kind.ends_with("_v2"))
                    .map(|r| r.id.as_str())
                    .collect();
                assert!(!expected.is_empty(), "{id} {case}: the entry has rows to list");
                assert_eq!(entry["result_ids"], json!(expected), "{id} {case}");
                listed += expected.len();
            }
        }
    }
    assert!(listed > 0);
}
