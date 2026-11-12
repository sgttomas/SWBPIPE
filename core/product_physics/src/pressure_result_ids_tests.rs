//! Pressure evidence `result_ids` from one pass over the case's rows answer as the
//! replaced per-entry scan, kept here verbatim as the oracle.
use super::*;
use serde_json::{json, Value};

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
