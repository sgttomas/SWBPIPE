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

/// The committed pre-join basis for MJ6's kill: for each of the twelve original
/// v2 requests (its exact request-file SHA-256) in both modes, the byte length and
/// SHA-256 of the public producer's exact serialization (`produce`:
/// `serde_json::to_string` of `run_linear_static_preview_value_with_mode`),
/// captured at the join's parent `4225e9a835` (pre-join source) through that API.
/// The historical pretty `fixtures/product_preview/{physics,load_reference}_source/*.raw.json`
/// goldens are older captures and are not this basis.
const V2_PREJOIN_BASIS: [(Family, &str, &str, PreviewSolverMode, usize, &str); 24] = [
    (Family::Physics03, "n05", "332319ee6f47870a074a6c16fbf2f43c0171e376a0b4cd1cb7f7ad8b59254b00", PreviewSolverMode::SparseInteractive, 116682, "c674a625af9d168474c3c16f3f5ad1f951063b9855a60f27350c592672742045"),
    (Family::Physics03, "n05", "332319ee6f47870a074a6c16fbf2f43c0171e376a0b4cd1cb7f7ad8b59254b00", PreviewSolverMode::DenseScrutiny, 117939, "26792d2b53aee0a3d0ade8c86f4cca0c0e441c7970733c8123f318b8d8dcf3e7"),
    (Family::Physics03, "n06", "5551f164b9e8ab88f04f3abce810e1e1b1ff47236bba6a296e32905f628be931", PreviewSolverMode::SparseInteractive, 109624, "c4572399e16f8b1d0705f62d47790b8c8923264b681948fb14f9bb1a16271e5b"),
    (Family::Physics03, "n06", "5551f164b9e8ab88f04f3abce810e1e1b1ff47236bba6a296e32905f628be931", PreviewSolverMode::DenseScrutiny, 109612, "84577446d1a3d9ed41d140c07132ccb201a70831c0bb9cffefcf211287291a91"),
    (Family::Physics03, "n05_unicode", "494310fccac2ff287f7da79614e243d25cb889997adf7fa6b6ff2fc39ec177e9", PreviewSolverMode::SparseInteractive, 119620, "10088231ae7b63d47cd7ba314d96fca5a11cfe694dfaa701428ecf02471d8aef"),
    (Family::Physics03, "n05_unicode", "494310fccac2ff287f7da79614e243d25cb889997adf7fa6b6ff2fc39ec177e9", PreviewSolverMode::DenseScrutiny, 120887, "ea6169a164b9143e88e6453a039c8d1954ea0afdca7ce136393d7a42ddf85382"),
    (Family::Physics03, "n05_units", "4774ffdeb00bf3a400032994f18b85b8b53b37695e88a6e15c7f6c8b3abb2775", PreviewSolverMode::SparseInteractive, 116688, "a39b2d92da3c924c2d317813822907176720bd6debf2028844c1882f17195963"),
    (Family::Physics03, "n05_units", "4774ffdeb00bf3a400032994f18b85b8b53b37695e88a6e15c7f6c8b3abb2775", PreviewSolverMode::DenseScrutiny, 117945, "ddffff9120363d0398efab59f22c26503c8ba69f5527d760ddb01fa1164c62a2"),
    (Family::Physics03, "fields", "7f8ff9d5e23712cedd6ca58a86a690517a8e820b9b072962ca43ad60be286002", PreviewSolverMode::SparseInteractive, 124254, "185f9c981e0a308d8ae9e37aa2125ce3f8d0d547b3d8c27220f33df643cb0c6b"),
    (Family::Physics03, "fields", "7f8ff9d5e23712cedd6ca58a86a690517a8e820b9b072962ca43ad60be286002", PreviewSolverMode::DenseScrutiny, 125691, "15afd7017824c21c4941a47f1d380e8c4a490baebe858f87ebafec23bc4522ea"),
    (Family::Physics03, "mixed", "a623608d33423c9bcf40a6ef5324f11576c52501953c3f0e6d08d24ddcb84abf", PreviewSolverMode::SparseInteractive, 220755, "a16a4cda514fd3420c7bd6d0e64b9d6b8ebac73a8d092fce96432fb20dacfe7b"),
    (Family::Physics03, "mixed", "a623608d33423c9bcf40a6ef5324f11576c52501953c3f0e6d08d24ddcb84abf", PreviewSolverMode::DenseScrutiny, 223272, "899c011eb3a7e784ab052de1af7c17da40fcf85c2da67d0d2c77ecabad5875bd"),
    (Family::Physics03, "mixed_units", "065f004d03ab11324958a6b5a0c8763e063e18f9a413c9ffee336903d4a1725a", PreviewSolverMode::SparseInteractive, 220868, "e87743afd27c0bf80be39c999057595625781ffec5ef01edf719bd86d8e47181"),
    (Family::Physics03, "mixed_units", "065f004d03ab11324958a6b5a0c8763e063e18f9a413c9ffee336903d4a1725a", PreviewSolverMode::DenseScrutiny, 223385, "4509212829da703721abe22dcefd192e0509a60c0fbe06093e1c3d736fe1303e"),
    (Family::LoadReference04, "n05", "b31f44a2d3e106d9c4c139f9ec3f933a212811926d23eabae068c647b4b83ed5", PreviewSolverMode::SparseInteractive, 119282, "d67d61ea254a1e5266300d1fd726e4334a2ce5cf77e61f445db1304873df2b69"),
    (Family::LoadReference04, "n05", "b31f44a2d3e106d9c4c139f9ec3f933a212811926d23eabae068c647b4b83ed5", PreviewSolverMode::DenseScrutiny, 120535, "adb021aa925b151124b252dc3e3a5f96a36d31d56b3898d12f38bf036002168a"),
    (Family::LoadReference04, "n06", "d24d570ecbb8a75cdb4ea21899ccd51b294e6e4637636c4768826ba3094fe141", PreviewSolverMode::SparseInteractive, 112226, "8ecfae9a87229394fe148da169c148a651b9e7556fea3a63d6a70a385d1f4820"),
    (Family::LoadReference04, "n06", "d24d570ecbb8a75cdb4ea21899ccd51b294e6e4637636c4768826ba3094fe141", PreviewSolverMode::DenseScrutiny, 112210, "620ec97747ea3e236c58eef074f7cb0134e494df56d0bb6f50268737928497ed"),
    (Family::LoadReference04, "eigen_motion", "58ce5b95d185d56964a30962b355c14595cdf831505603c6e5d455425c4ea8e9", PreviewSolverMode::SparseInteractive, 131768, "0935a4037bbb74b92875cbf31f21a51d57e7565a341509b3e1b5860f40cb694f"),
    (Family::LoadReference04, "eigen_motion", "58ce5b95d185d56964a30962b355c14595cdf831505603c6e5d455425c4ea8e9", PreviewSolverMode::DenseScrutiny, 133024, "96c5e67bb412ccbb41e46cd17e353889e9f4fcb00398ea6de43b8a58b7d3f466"),
    (Family::LoadReference04, "fields", "8c9a605758ea2e76269b8b7f2ffc5738e9c758d735189326b43335b58c2dce61", PreviewSolverMode::SparseInteractive, 128044, "45165ed9ff3f556fbf13d03cdc609d9bf48ff97f1e209404f235ce5bb9f4a076"),
    (Family::LoadReference04, "fields", "8c9a605758ea2e76269b8b7f2ffc5738e9c758d735189326b43335b58c2dce61", PreviewSolverMode::DenseScrutiny, 129477, "ed1d1b1a31f0141eef3847dc446f94b33fd9f76b3b05042e694ab7c70df42bf8"),
    (Family::LoadReference04, "mixed", "14d2ab92aba8190c133b0c41d4dd8e7e0443b06044f48a49b3b408d6fa912ad2", PreviewSolverMode::SparseInteractive, 225446, "8962e35fc2a069b9d92b9e512cb7c7d74f6166579f6d0a7638819eea789a42cc"),
    (Family::LoadReference04, "mixed", "14d2ab92aba8190c133b0c41d4dd8e7e0443b06044f48a49b3b408d6fa912ad2", PreviewSolverMode::DenseScrutiny, 227950, "dcc1744c07bd175ec4c9d0a2f03195ace054ffd21a98fda2482dc9fb0899640d"),
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
