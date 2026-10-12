//! First four genuine pressure-source consuming controls, produced through the
//! public captured-request API. No result is relabelled, projected or resealed.
//! The n06 equilibrium controls and protected criterion are the existing ones
//! in physics_source_runtime.rs; this is not the reserved desktop G10 witness.
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_product_physics::result_export::{
    physics_source as ps, reader_verdict as rv, semantic_contract as sc,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy)]
enum Family {
    Physics03,
    LoadReference04,
}
impl Family {
    fn name(self) -> &'static str {
        if self.resolved() {
            "load_reference_source"
        } else {
            "physics_source"
        }
    }
    fn names(self) -> &'static [&'static str] {
        if self.resolved() {
            &["n05", "n06", "eigen_motion", "fields", "mixed"]
        } else {
            &[
                "n05",
                "n06",
                "n05_unicode",
                "n05_units",
                "fields",
                "mixed",
                "mixed_units",
            ]
        }
    }
    fn request(self, name: &str) -> Value {
        assert!(self.names().contains(&name));
        let path = project_root().join(format!(
            "fixtures/product_preview/{}/{name}.request.json",
            self.name()
        ));
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
    fn resolved(self) -> bool {
        matches!(self, Self::LoadReference04)
    }
    fn legacy_identity(self) -> &'static str {
        if self.resolved() {
            sc::LOAD_REFERENCE_SOURCE_ID
        } else {
            sc::PHYSICS_SOURCE_ID
        }
    }
}

fn project_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
struct Control {
    family: Family,
    name: String,
    mode: PreviewSolverMode,
    original: Value,
    invocation: Value,
    legacy: Produced,
    joined: Produced,
}
#[path = "pressure_source_join/consequences.rs"]
mod consequences;
// CI1282's shared strict test-fixture DAG decoder, adopted by the parent.
#[path = "../../reporting/result_export/tests/support/retained_precision_corpus.rs"]
mod corpus_storage;

struct Produced {
    bytes: String,
    source: Value,
}
fn produce(request: &Value, mode: PreviewSolverMode) -> Produced {
    let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
    // This is the public producer's exact serialization, as the native IPC sends it.
    let bytes = serde_json::to_string(&envelope).unwrap();
    let source = serde_json::from_str(&bytes).unwrap();
    Produced { bytes, source }
}
fn capture_evidence(
    family: Family,
    name: &str,
    mode: PreviewSolverMode,
    legacy: &Produced,
    joined: &Produced,
) {
    // No default writes and no permanent test dependency on captured outputs.
    // A coordinator may explicitly request exact invented-control evidence.
    if let Some(directory) = std::env::var_os("SWBPIPE_PRESSURE_JOIN_EVIDENCE_DIR") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        let version = if family.resolved() { "0_4" } else { "0_3" };
        for (contract, produced) in [("v2", legacy), ("v3", joined)] {
            let path = directory.join(format!(
                "{name}-{version}-{}-{contract}.raw.json",
                mode.as_str()
            ));
            std::fs::write(&path, produced.bytes.as_bytes()).unwrap();
        }
    }
}
fn domain_commitment(domain: &str, payload: &Value) -> String {
    // Construct the specified commitment independently of the producer's policy
    // selector/case builder, using the declared canonical JSON hash authority.
    let text = serde_json::to_string(&json!({"domain":domain,"payload":payload})).unwrap();
    let canonical = canonical_json_checked_v1_text(&text).unwrap();
    format!("{:x}", Sha256::digest(canonical.as_bytes()))
}
fn row<'a>(source: &'a Value, entity: &str, kind: &str, component: Option<&str>) -> &'a Value {
    let found: Vec<_> = source["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| {
            row["entity_ref"] == entity
                && row["kind"] == kind
                && row["basis_ref"]["ref_id"] == "case"
                && component.is_none_or(|value| row["metadata"]["component"] == value)
        })
        .collect();
    assert_eq!(found.len(), 1, "{entity}/{kind}/{component:?}");
    found[0]
}
fn relative(actual: f64, expected: f64) {
    if expected == 0.0 {
        assert_eq!(actual, 0.0);
    } else {
        assert!(
            ((actual - expected) / expected).abs() <= 1e-9,
            "{actual:e} vs {expected:e}"
        );
    }
}
fn existing_n06_equilibrium(source: &Value, request: &Value) {
    let torque = request["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"]
        .as_f64()
        .unwrap();
    let spring = request["model"]["supports"][1]["stiffness"]["value"]["value"]
        .as_f64()
        .unwrap();
    let j = std::f64::consts::PI * (0.1_f64.powi(4) - 0.09_f64.powi(4)) / 2.0;
    relative(
        row(source, "independent-root", "global_nodal_rotation_x", None)["value"]
            .as_f64()
            .unwrap(),
        torque / spring,
    );
    relative(
        row(source, "independent-tip", "global_nodal_rotation_x", None)["value"]
            .as_f64()
            .unwrap(),
        torque / spring + torque * 2.0 / (80e9 * j),
    );
    relative(
        row(
            source,
            "independent-spring",
            "support_reaction_component_v2",
            Some("Mx"),
        )["value"]
            .as_f64()
            .unwrap(),
        -torque,
    );
    relative(
        row(
            source,
            "independent-spring",
            "support_reaction_moment_magnitude_v2",
            Some("moment_magnitude"),
        )["value"]
            .as_f64()
            .unwrap(),
        torque.abs(),
    );
    let maximum = row(
        source,
        "independent-member",
        "pipe_elastic_normal_stress_maximum_v2",
        None,
    );
    assert_eq!(
        maximum["metadata"]["basis"],
        "retained_source_endpoint_normal_max_v1"
    );
    assert_eq!(maximum["value"].as_f64().unwrap(), 0.0);
    let material = &source["contract_evidence"]["exact_cases"][0]["pipe_materials"][0];
    assert_eq!(material["nu"].as_f64().unwrap(), 0.25);
    assert_eq!(material["G_pa"].as_f64().unwrap(), 80e9);
}

fn check_control(family: Family, name: &str, mode: PreviewSolverMode) -> Control {
    let original = family.request(name);
    let snapshot = original.clone();
    assert_eq!(
        original["model"]["pressure_contract"],
        json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"})
    );
    let legacy = produce(&original, mode);
    assert_eq!(original, snapshot);
    assert_eq!(
        legacy.source["producer"]["semantic_contract_id"],
        family.legacy_identity()
    );
    if name == "n06" {
        existing_n06_equilibrium(&legacy.source, &original);
    }
    // Keep exact current T4 v2 bytes available for the later byte-basis check.
    // Historical goldens are not rewritten or treated as an oracle for declared T4 changes.

    let mut twin = original.clone();
    twin["model"]["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3"});
    let mut restored = twin.clone();
    restored["model"]["pressure_contract"] = original["model"]["pressure_contract"].clone();
    assert_eq!(
        restored, original,
        "The only authored twin change is the pressure contract"
    );
    let produced = produce(&twin, mode);
    capture_evidence(family, name, mode, &legacy, &produced);
    let source = &produced.source;
    let invocation = json!({"request":twin,"solver_mode":mode.as_str()});
    assert_eq!(original, snapshot, "Original requests remain untouched");
    assert_eq!(
        source["status"]["mechanics"], "MECHANICS_SOLVED",
        "{}",
        source["diagnostics"]
    );
    assert_eq!(
        source["producer"]["semantic_contract_id"],
        ps::PRESSURE_CONTRACT_ID
    );
    assert_eq!(
        source["formulation_basis"]["profile_id"],
        "exact_pressure_v3"
    );
    let body = &source["source_block_recovery"]["body"];
    assert_eq!(body["policy"], "PRESSURE-SOURCE-1");
    assert_eq!(body["status"], "qualified");
    let selected_ids: Vec<_> = body["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["selected_method"] == "retained_source_blocks_exact_v1")
        .map(|case| &case["basis_ref"]["ref_id"])
        .collect();
    assert!(source["diagnostics"].as_array().unwrap().iter().all(|d| {
        d["code"] != "EXACT_PRESSURE_V3_RETAINED_SOURCE_NOT_JOINED"
            && !(d["code"] == "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"
                && d["affected_refs"]
                    .as_array()
                    .is_some_and(|refs| refs.iter().any(|r| selected_ids.contains(&r))))
    }));
    let evidence = &source["contract_evidence"];
    assert_eq!(
        evidence.get("load_reference_states").is_some(),
        family.resolved()
    );
    let case_count = original["model"]["load_cases"].as_array().unwrap().len();
    assert_eq!(body["cases"].as_array().unwrap().len(), case_count);
    assert_eq!(
        evidence["exact_cases"].as_array().unwrap().len(),
        case_count
    );
    let selected_count = body["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["selected_method"] == "retained_source_blocks_exact_v1")
        .count();
    assert!(selected_count > 0);
    assert_eq!(
        source["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_SELECTED")
            .count(),
        selected_count
    );
    for (record, exact) in body["cases"]
        .as_array()
        .unwrap()
        .iter()
        .zip(evidence["exact_cases"].as_array().unwrap())
    {
        assert_eq!(record["basis_ref"]["ref_id"], exact["load_case_id"]);
        assert_eq!(record["selected_method"], exact["recovery_method"]);
        assert_eq!(exact["profile_mode"], "exact_pressure_v3");
        let pressure: Vec<_> = evidence["pressure"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|p| p["load_case_id"] == exact["load_case_id"])
            .collect();
        if record["selected_method"] == "retained_source_blocks_exact_v1" {
            assert!(pressure.is_empty());
        }
        let mut payload = json!({"exact_case":exact,"pressure":pressure});
        if family.resolved() {
            let state = evidence["load_reference_states"]
                .as_array()
                .unwrap()
                .iter()
                .find(|s| s["load_case_id"] == exact["load_case_id"])
                .unwrap();
            payload["load_reference_state"] = state.clone();
        }
        assert_eq!(
            record["physical_evidence_sha256"],
            domain_commitment("pressure_source_case_evidence_v1", &payload)
        );
    }
    if name == "n06" {
        existing_n06_equilibrium(source, &invocation["request"]);
    }
    let eligible = !family.resolved();
    assert_eq!(
        ps::validate_in(source, Some(&invocation), ps::SourceContract::PressureV3),
        Ok(eligible)
    );
    assert_eq!(
        ps::validate_in(source, None, ps::SourceContract::PressureV3),
        Ok(false)
    );
    assert_eq!(
        ps::validate_transport_metadata_in(source, ps::SourceContract::PressureV3),
        Ok(())
    );
    let validation = rv::validate("pressure-source", source, None, Some(&invocation));
    assert_eq!(validation["ok"], true, "{validation}");
    assert_eq!(validation["eligible"], eligible);
    let transported = json!({"schema_version":source["schema_version"],"producer":source["producer"],
        "numerical_quality":source["numerical_quality"],"formulation_basis":source["formulation_basis"],
        "contract_evidence":source["contract_evidence"],"source_block_recovery":source["source_block_recovery"]});
    assert_eq!(
        ps::validate_transport_metadata_in(&transported, ps::SourceContract::PressureV3),
        Ok(())
    );
    assert_eq!(
        rv::transport("pressure-source", &transported),
        json!({"ok":true})
    );
    let bound = rv::dispatch(
        source,
        Some(&invocation["request"]["model"]),
        Some(&invocation),
    );
    assert_eq!(bound["ok"], true, "{bound}");
    assert_eq!(bound["route"], "pressure_source");
    assert_eq!(
        bound["binding"],
        json!({"id":ps::PRESSURE_CONTRACT_ID,"sha256":sc::PRESSURE_SOURCE_TABLE_SHA256})
    );
    assert_eq!(bound["standing"]["eligible"], eligible);
    if family.resolved() {
        assert_eq!(bound["standing"]["token"], "needs_recompute");
        assert_eq!(
            bound["standing"]["findings"],
            json!([rv::LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE])
        );
    } else {
        assert_eq!(bound["standing"]["token"], "numerically_eligible");
        assert_eq!(bound["standing"]["findings"], json!([]));
    }
    // Genuine sources, explicitly caller-selected opposite readers; no relabelling.
    assert_eq!(
        ps::validate_in(source, None, ps::SourceContract::PhysicsV2),
        Err(sc::V3_READ_AS_V2.into())
    );
    assert_eq!(
        ps::validate_transport_metadata_in(source, ps::SourceContract::PhysicsV2),
        Err(sc::V3_READ_AS_V2.into())
    );
    assert_eq!(
        ps::validate_in(&legacy.source, None, ps::SourceContract::PressureV3),
        Err(sc::V2_READ_AS_V3.into())
    );
    assert_eq!(
        ps::validate_transport_metadata_in(&legacy.source, ps::SourceContract::PressureV3),
        Err(sc::V2_READ_AS_V3.into())
    );
    assert_eq!(
        ps::validate_transport_metadata_in(&transported, ps::SourceContract::PhysicsV2),
        Err(sc::V3_READ_AS_V2.into())
    );
    Control {
        family,
        name: name.to_owned(),
        mode,
        original,
        invocation,
        legacy,
        joined: produced,
    }
}

#[test]
fn v3_rescued_physics_n06_sparse_publishes_pressure_source() {
    check_control(
        Family::Physics03,
        "n06",
        PreviewSolverMode::SparseInteractive,
    );
}
#[test]
fn v3_rescued_physics_n06_dense_publishes_pressure_source() {
    check_control(Family::Physics03, "n06", PreviewSolverMode::DenseScrutiny);
}
#[test]
fn v3_rescued_load_state_n06_sparse_publishes_pressure_source() {
    check_control(
        Family::LoadReference04,
        "n06",
        PreviewSolverMode::SparseInteractive,
    );
}
#[test]
fn v3_rescued_load_state_n06_dense_publishes_pressure_source() {
    check_control(
        Family::LoadReference04,
        "n06",
        PreviewSolverMode::DenseScrutiny,
    );
}
