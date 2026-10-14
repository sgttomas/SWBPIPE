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

/// M1's kill: the v3 twins of both source families' n06, in both modes,
/// publish solved selected recovery with a valid pressure-source receipt and
/// no missing-join diagnostic (`check_control`).
#[test]
fn v3_rescued_cases_publish_pressure_source() {
    for family in [Family::Physics03, Family::LoadReference04] {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            check_control(family, "n06", mode);
        }
    }
}

/// MJ6's independent pre-join counterpart basis: the final common source at
/// qualified B7a 1b24b01, with the three reviewed join seams restored from
/// 4225e9a835. Current mechanics, H4, C5, U4a and custody changes stay on both
/// sides. All twelve immutable v2 requests in both modes use the ordinary public
/// `run_linear_static_preview_value_with_mode` with no retained profile, followed
/// by exact `serde_json::to_string`. Whole output bytes agree; no hash, counter,
/// limitation or numerical exclusion applies. These pins come only from the
/// independently captured predecessor side, not the joined producer outputs.
/// Historical pretty raw goldens are separate fixture representations.
const V2_PREJOIN_BASIS: [(Family, &str, &str, PreviewSolverMode, usize, &str); 24] = [
    (Family::Physics03, "n05", "332319ee6f47870a074a6c16fbf2f43c0171e376a0b4cd1cb7f7ad8b59254b00", PreviewSolverMode::SparseInteractive, 116628, "fbca5893809ed43ccae0d4f7e54fc56407da8473b1e47abcc76dde6c497f3856"),
    (Family::Physics03, "n05", "332319ee6f47870a074a6c16fbf2f43c0171e376a0b4cd1cb7f7ad8b59254b00", PreviewSolverMode::DenseScrutiny, 116629, "336d340bbd256344a6caf05dd8acf9a9a56e6464725e346af6276fc1473f0eb4"),
    (Family::Physics03, "n06", "5551f164b9e8ab88f04f3abce810e1e1b1ff47236bba6a296e32905f628be931", PreviewSolverMode::SparseInteractive, 109570, "327873ebd838cb90d9450ca0d756b5cdc577bf641ad378990b12086a125bc32c"),
    (Family::Physics03, "n06", "5551f164b9e8ab88f04f3abce810e1e1b1ff47236bba6a296e32905f628be931", PreviewSolverMode::DenseScrutiny, 109558, "8dc9c6a2495e6cb2be09690e9e6e81962dc9d7439dca1fe7d102aba9d508810c"),
    (Family::Physics03, "n05_unicode", "494310fccac2ff287f7da79614e243d25cb889997adf7fa6b6ff2fc39ec177e9", PreviewSolverMode::SparseInteractive, 119566, "227231ca84a459818d260d7105fc555196044539e7e482c93de38c67bfc815f4"),
    (Family::Physics03, "n05_unicode", "494310fccac2ff287f7da79614e243d25cb889997adf7fa6b6ff2fc39ec177e9", PreviewSolverMode::DenseScrutiny, 119567, "0d502bf107553325ec046477c0b20f3cace1a89375205904a403e69368fea8a0"),
    (Family::Physics03, "n05_units", "4774ffdeb00bf3a400032994f18b85b8b53b37695e88a6e15c7f6c8b3abb2775", PreviewSolverMode::SparseInteractive, 116634, "d2711a590239cb0aa1acc20e59147fb7c6120d2fc5e7e8a9730e80b2b6a65079"),
    (Family::Physics03, "n05_units", "4774ffdeb00bf3a400032994f18b85b8b53b37695e88a6e15c7f6c8b3abb2775", PreviewSolverMode::DenseScrutiny, 116635, "29226f979d7cf5e049e1dd5feb2260d8b543c76a4c0db6c7a66d745a6b5e9a6e"),
    (Family::Physics03, "fields", "7f8ff9d5e23712cedd6ca58a86a690517a8e820b9b072962ca43ad60be286002", PreviewSolverMode::SparseInteractive, 124200, "50d3b74e56adf77ef141505bbb7ce6a95b02d6c715764daa68733c7bd08ba971"),
    (Family::Physics03, "fields", "7f8ff9d5e23712cedd6ca58a86a690517a8e820b9b072962ca43ad60be286002", PreviewSolverMode::DenseScrutiny, 124381, "c2383a374b43acdf5de0688305abc52642cac83362015fcf869b692d39fb5cd0"),
    (Family::Physics03, "mixed", "a623608d33423c9bcf40a6ef5324f11576c52501953c3f0e6d08d24ddcb84abf", PreviewSolverMode::SparseInteractive, 220399, "62f98fb2df6615213b2f63cfdc3d07e1e2edf3e9242bf4a89d0e1073eb3a2300"),
    (Family::Physics03, "mixed", "a623608d33423c9bcf40a6ef5324f11576c52501953c3f0e6d08d24ddcb84abf", PreviewSolverMode::DenseScrutiny, 220393, "68d9f9a46c16a545a312142c20acb9566a7127595a6d316f25c74d25cba12fd6"),
    (Family::Physics03, "mixed_units", "065f004d03ab11324958a6b5a0c8763e063e18f9a413c9ffee336903d4a1725a", PreviewSolverMode::SparseInteractive, 220512, "26d2a90ff83c0ae17f6de2f981dc663b25ff3ca65142635e23901ad97ec9dabd"),
    (Family::Physics03, "mixed_units", "065f004d03ab11324958a6b5a0c8763e063e18f9a413c9ffee336903d4a1725a", PreviewSolverMode::DenseScrutiny, 220506, "7454c70c9d98987499ef42c9a6a55dae5670b7922d265c2057750e72bb734e63"),
    (Family::LoadReference04, "n05", "b31f44a2d3e106d9c4c139f9ec3f933a212811926d23eabae068c647b4b83ed5", PreviewSolverMode::SparseInteractive, 119228, "381d331c3862c852fbd1038a45c5ed06038f81c41095a16c24a9643538201b29"),
    (Family::LoadReference04, "n05", "b31f44a2d3e106d9c4c139f9ec3f933a212811926d23eabae068c647b4b83ed5", PreviewSolverMode::DenseScrutiny, 119225, "5db4c518229e2ae5e32bee26b2c3f1e80d6f6894ec4d018066902b35b3fc66e7"),
    (Family::LoadReference04, "n06", "d24d570ecbb8a75cdb4ea21899ccd51b294e6e4637636c4768826ba3094fe141", PreviewSolverMode::SparseInteractive, 112172, "3306c96649ea393ea8f3d2f86e1c313e879374186d39280cf52e92264fa6f961"),
    (Family::LoadReference04, "n06", "d24d570ecbb8a75cdb4ea21899ccd51b294e6e4637636c4768826ba3094fe141", PreviewSolverMode::DenseScrutiny, 112156, "8884cea721a29f254aaf4c4c491ba2435a42136fb472e99d16e64fe762952975"),
    (Family::LoadReference04, "eigen_motion", "58ce5b95d185d56964a30962b355c14595cdf831505603c6e5d455425c4ea8e9", PreviewSolverMode::SparseInteractive, 131714, "8373cca38063ae65cd06d64231cbfd26b2c27aabcdd1d4e59b537b7884640f4d"),
    (Family::LoadReference04, "eigen_motion", "58ce5b95d185d56964a30962b355c14595cdf831505603c6e5d455425c4ea8e9", PreviewSolverMode::DenseScrutiny, 131709, "bb0638d34c6b1183038237dfb11e8bea41137e769a33100ee545a70b6c326e71"),
    (Family::LoadReference04, "fields", "8c9a605758ea2e76269b8b7f2ffc5738e9c758d735189326b43335b58c2dce61", PreviewSolverMode::SparseInteractive, 127990, "6c1b645ca364fe60e38d5aa54b26aad2a28908cd9f934e37c1e355593289ba3c"),
    (Family::LoadReference04, "fields", "8c9a605758ea2e76269b8b7f2ffc5738e9c758d735189326b43335b58c2dce61", PreviewSolverMode::DenseScrutiny, 128167, "fef68c6127b9247774d99abb3952ff32f6686f4c0b5e2167b2ba4da71a99b5f1"),
    (Family::LoadReference04, "mixed", "14d2ab92aba8190c133b0c41d4dd8e7e0443b06044f48a49b3b408d6fa912ad2", PreviewSolverMode::SparseInteractive, 225090, "99a595534a81ee0f58e4273dbc4acc30258cc711fc25ce20334b01f0c38f7d6c"),
    (Family::LoadReference04, "mixed", "14d2ab92aba8190c133b0c41d4dd8e7e0443b06044f48a49b3b408d6fa912ad2", PreviewSolverMode::DenseScrutiny, 225071, "f27645b497255f901e0eb442259d7e9c57795f6528cd4afe2b11134035f3c771"),
];

/// MJ6's kill: the join changes no v2 source-publication byte. Exact producer
/// serialization of all twelve original requests x both modes equals the
/// pre-join basis; no hash, counter, limitation or numeric exclusion applies.
#[test]
fn join_preserves_every_v2_source_publication_byte() {
    let mut covered = std::collections::BTreeSet::new();
    for (family, name, request_sha256, mode, length, sha256) in V2_PREJOIN_BASIS {
        let path = project_root().join(format!(
            "fixtures/product_preview/{}/{name}.request.json",
            family.name()
        ));
        let request_bytes = std::fs::read(path).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&request_bytes)),
            request_sha256,
            "{}/{name}: the immutable original request",
            family.name()
        );
        let request: Value = serde_json::from_slice(&request_bytes).unwrap();
        assert_eq!(request, family.request(name));
        assert_eq!(
            request["model"]["pressure_contract"],
            json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"})
        );
        let produced = produce(&request, mode);
        let label = format!("{}/{name}/{}", family.name(), mode.as_str());
        assert_eq!(
            produced.source["producer"]["semantic_contract_id"],
            family.legacy_identity(),
            "{label}"
        );
        assert_eq!(produced.bytes.len(), length, "{label}: exact byte length");
        assert_eq!(
            format!("{:x}", Sha256::digest(produced.bytes.as_bytes())),
            sha256,
            "{label}: exact producer bytes"
        );
        assert!(covered.insert(label));
    }
    let mut expected = std::collections::BTreeSet::new();
    for family in [Family::Physics03, Family::LoadReference04] {
        for name in family.names() {
            for mode in [
                PreviewSolverMode::SparseInteractive,
                PreviewSolverMode::DenseScrutiny,
            ] {
                expected.insert(format!("{}/{name}/{}", family.name(), mode.as_str()));
            }
        }
    }
    assert_eq!(covered, expected, "all twelve original requests in both modes");
}
