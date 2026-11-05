//! §10.5 at the product (DEL-04-03 Work `skew-restraints.md` §10.5, §15 step 7): on the
//! references' guided cantilever, every skewed record's certified K-D5 record and every
//! axis record at its node are available (no recovery failure), the skewed record
//! carries its along-direction group, and the certified rows enclose the references'
//! exact values. Model inputs restate `tests/skew_restraints_references.rs`.
use crate::tests::KD5_RECOVERY_PROBE;
use crate::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const PROV: &str = "skew_restraints invented benchmark input; not library, component, material-catalog or code data";

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// `(id, family, token, direction)` records at B (a zero direction: none); a spring
/// has k = 2^20 N/m.
fn document(records: &[(&str, &str, &str, [f64; 3])]) -> Value {
    let mut supports = vec![
        json!({"id": "anchor_A", "node": "A", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}),
        json!({"id": "rot_B", "node": "B", "family": "anchor", "restraints": ["RX", "RY", "RZ"], "provenance": PROV}),
    ];
    for &(id, family, token, d) in records {
        let mut s = json!({"id": id, "node": "B", "family": family, "restraints": [token], "provenance": PROV});
        if d != [0.0; 3] {
            s["direction"] = json!({"x": d[0], "y": d[1], "z": d[2]});
        }
        if family == "spring" {
            s["stiffness"] = json!({"dof": token, "value": qv(1_048_576.0, "N/m")});
        }
        supports.push(s);
    }
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:skew", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "B", "position": {"x": 4.0, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:AB", "from": "A", "to": "B", "section": {"outside_diameter": qv(0.206, "m"), "wall_thickness": qv(0.011, "m")},
            "material": "material:skew", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV}],
        "supports": supports, "components": [],
        "materials": [{"id": "material:skew", "elastic_modulus": qv(171_798_691_840.0, "Pa"), "shear_modulus": qv(68_719_476_736.0, "Pa"), "provenance": PROV}],
        "load_cases": [{"id": "case:skew", "primitive_loads": [{"id": "load:y", "category": "concentrated_force", "target": {"type": "node", "node": "B"},
            "direction": "global_y", "magnitude": qv(-1024.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV}], "combinations": []},
        "materials": []})
}

#[test]
fn skewed_records_publish_available_certified_records() {
    let tiny = 2.0_f64.powi(-600);
    let cases: [(&str, Vec<(&str, &str, &str, [f64; 3])>); 5] = [
        ("P_R1_stop", vec![("stop_B", "line_stop", "UY", [3.0, 4.0, 0.0])]),
        ("P_S1_spring", vec![("spring_B", "spring", "UY", [3.0, 4.0, 0.0])]),
        ("P_RS_stop_and_spring", vec![("stop_B", "line_stop", "UY", [3.0, 4.0, 0.0]), ("spring_B", "spring", "UY", [0.0, 3.0, 4.0])]),
        ("P_R4_guide_plus_stop", vec![("guide_uz_B", "line_stop", "UZ", [0.0; 3]), ("stop_B", "line_stop", "UY", [3.0, 4.0, 5.0])]),
        ("P_R4_tiny_restricted_direction", vec![("ux_B", "line_stop", "UX", [0.0; 3]), ("uz_B", "line_stop", "UZ", [0.0; 3]), ("stop_B", "line_stop", "UY", [1.0, tiny, 0.0])]),
    ];
    for (name, records) in cases {
        let wire = document(&records);
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
            let output = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
            let probe = KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{name} {mode:?}: {:?}", output.diagnostics);
            for id in records.iter().map(|r| r.0).chain(["rot_B"]) {
                let record = probe.iter().find(|r| r.member == id).unwrap_or_else(|| panic!("{name} {mode:?}: no certified record for {id}"));
                for group in &record.groups {
                    assert!(group.bound.is_ok(), "{name} {mode:?}: {id} {}: {:?}", group.kind, group.bound);
                }
                let skewed = records.iter().any(|r| r.0 == id && r.3 != [0.0; 3]);
                let along = record.groups.iter().filter(|g| g.kind == "along_direction").count();
                assert_eq!(along, usize::from(skewed), "{name} {mode:?}: {id} along-direction groups");
                if skewed {
                    // The published along row is the certified value, within its bound.
                    let group = record.groups.iter().find(|g| g.kind == "along_direction").unwrap();
                    let row = output.results.iter()
                        .find(|r| r.entity_ref == id && r.kind == "support_reaction_along_direction_v1").unwrap();
                    assert_eq!(row.value.abs().to_bits(), group.q.to_bits(), "{name} {mode:?}: {id} published = certified");
                }
            }
        }
    }
}
