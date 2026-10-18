//! Genuine pressure-source consuming controls, produced through the public
//! captured-request API. No result is relabelled, projected or resealed.
//! The n06 equilibrium controls and protected criterion are the existing ones
//! in physics_source_runtime.rs; this is not the reserved desktop G10 witness.
//!
//! B7b deleted the v2 exact contract (`2.0.0/exact_straight_pressure_v2`) and
//! its physics-source-1 and load-reference-source-1 publications. The twelve
//! committed source controls are now the current 0.6.0 documents under
//! `fixtures/product_preview/pressure_source/` (v3, explicit zero corrosion
//! allowance, Euler-Bernoulli), each re-authored once from its deleted v2
//! original. The two families stay distinct: without and with the optional
//! load/reference-state group.
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_product_physics::result_export::{
    physics_source as ps, reader_verdict as rv, semantic_contract as sc,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "common/current_model.rs"]
mod current_model;

#[derive(Clone, Copy)]
enum Family {
    /// Straight source controls without the load/reference-state group.
    Physics,
    /// Source controls carrying the load/reference-state group.
    LoadReference,
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
    fn request_path(self, name: &str) -> String {
        assert!(self.names().contains(&name));
        format!("fixtures/product_preview/pressure_source/{}/{name}.request.json", self.name())
    }
    /// The committed current request: 0.6.0 with the current exact contract.
    fn request(self, name: &str) -> Value {
        let request = fixture(&self.request_path(name));
        current_model::assert_current_exact_request(&request, &self.request_path(name));
        request
    }
    fn resolved(self) -> bool {
        matches!(self, Self::LoadReference)
    }
}

fn project_root() -> std::path::PathBuf {
    current_model::project_root()
}
/// A committed JSON fixture, by project-relative path. Kept here, its only
/// caller, so the runtime read stays out of `common/current_model.rs`, which
/// other crates compile through `#[path]`.
fn fixture(path: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(project_root().join(path)).unwrap())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}
struct Control {
    family: Family,
    name: String,
    mode: PreviewSolverMode,
    invocation: Value,
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
fn capture_evidence(family: Family, name: &str, mode: PreviewSolverMode, joined: &Produced) {
    // No default writes and no permanent test dependency on captured outputs.
    // A coordinator may explicitly request exact invented-control evidence.
    if let Some(directory) = std::env::var_os("SWBPIPE_PRESSURE_JOIN_EVIDENCE_DIR") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join(format!("{}-{name}-{}.raw.json", family.name(), mode.as_str()));
        std::fs::write(&path, joined.bytes.as_bytes()).unwrap();
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
    basis_pair(found).unwrap_or_else(|| panic!("{entity}/{kind}/{component:?}"))
}
/// D-5 (DEL-05-03 Design §2.1, §2.5): a stress row is published on the nominal
/// section and immediately followed by its reduced twin. The source family
/// admits zero mill tolerance and zero allowance only, so the twins are
/// bit-equal and the nominal row stands for both. `None` unless exactly one
/// basis-free row or one such pair.
fn basis_pair(rows: Vec<&Value>) -> Option<&Value> {
    let basis = |r: &Value| r["metadata"]["basis"].as_str().unwrap_or("").to_owned();
    match rows.as_slice() {
        [one] => (!basis(one).contains("; section_basis=")).then_some(*one),
        [nominal, reduced] => {
            let stem = basis(nominal).strip_suffix("; section_basis=nominal_section")?.to_owned();
            let reduced_stem = basis(reduced).strip_suffix("; section_basis=reduced_section")?.to_owned();
            let id = |r: &Value, a: &str, b: &str| {
                let id = r["id"].as_str().unwrap_or("");
                id.strip_suffix(a).or_else(|| id.strip_suffix(b)).map(str::to_owned)
            };
            let same_id = id(nominal, ":nominal-section", ":nominal_section")
                .is_some_and(|n| id(reduced, ":reduced-section", ":reduced_section") == Some(n));
            let same_value = nominal["value"].as_f64().map(f64::to_bits) == reduced["value"].as_f64().map(f64::to_bits);
            (stem == reduced_stem && same_id && same_value).then_some(*nominal)
        }
        _ => None,
    }
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
    // D-5: the nominal row of the maximum's twin pair names its basis.
    assert_eq!(
        maximum["metadata"]["basis"],
        "retained_source_endpoint_normal_max_v1; section_basis=nominal_section"
    );
    assert_eq!(maximum["value"].as_f64().unwrap(), 0.0);
    let material = &source["contract_evidence"]["exact_cases"][0]["pipe_materials"][0];
    assert_eq!(material["nu"].as_f64().unwrap(), 0.25);
    assert_eq!(material["G_pa"].as_f64().unwrap(), 80e9);
}

fn check_control(family: Family, name: &str, mode: PreviewSolverMode) -> Control {
    let original = family.request(name);
    let snapshot = original.clone();
    let produced = produce(&original, mode);
    capture_evidence(family, name, mode, &produced);
    let source = &produced.source;
    let invocation = json!({"request":original,"solver_mode":mode.as_str()});
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
    // The expected context is the caller's (here the captured invocation's),
    // never the received profile: a genuine source whose header names a
    // retired v2 identity is refused by name at header dispatch, and by the
    // caller-selected current reader on its identity, raw and transported.
    for retired in sc::RETIRED_EXACT_V2_IDS {
        let mut relabelled = source.clone();
        relabelled["producer"]["semantic_contract_id"] = json!(retired);
        let refused = rv::dispatch(&relabelled, Some(&invocation["request"]["model"]), Some(&invocation));
        assert_eq!(refused["ok"], false, "{refused}");
        assert_eq!(refused["error"]["code"], sc::SOURCE_EXACT_PRESSURE_V2_RETIRED, "{refused}");
        assert_eq!(
            ps::validate_in(&relabelled, Some(&invocation), ps::SourceContract::PressureV3),
            Err("PHYSICS_SOURCE_IDENTITY".into())
        );
        let mut transported_relabelled = transported.clone();
        transported_relabelled["producer"]["semantic_contract_id"] = json!(retired);
        assert_eq!(
            ps::validate_transport_metadata_in(&transported_relabelled, ps::SourceContract::PressureV3),
            Err("PHYSICS_SOURCE_IDENTITY".into())
        );
    }
    Control {
        family,
        name: name.to_owned(),
        mode,
        invocation,
        joined: produced,
    }
}

/// M1's kill: the current n06 controls of both source families, in both modes,
/// publish solved selected recovery with a valid pressure-source receipt and
/// no missing-join diagnostic (`check_control`).
#[test]
fn v3_rescued_cases_publish_pressure_source() {
    for family in [Family::Physics, Family::LoadReference] {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            check_control(family, "n06", mode);
        }
    }
}

/// MJ6's current basis (B7b): the twelve committed current requests (each the
/// once re-authored twin of a deleted v2 original) in both modes, through the
/// ordinary public `run_linear_static_preview_value_with_mode` with no retained
/// profile, followed by exact `serde_json::to_string`. The pins are the
/// regenerated committed raw fixtures `pressure_source/{family}/{name}-{mode}.raw.json`
/// (B7b's regeneration evidence records that against the pre-B7b v3 twins
/// and the v2 originals only the declared value classes moved). No hash,
/// counter, limitation or numerical exclusion applies. Re-pinned after finding
/// (b)'s limitation-7 text: each publication is 129 bytes longer, and against
/// the previous pins only limitation 7 and the receipt and publication hashes
/// that bind it changed. Re-pinned for T4-U6 (D-5, DEL-05-03 Design §2.6):
/// every source publication gains the reduced stress twins, basis tokens and
/// id segments, per-basis headlines, extrema and coverage, the D-5
/// `pipe_sections` shape and the revised limitations 0 and 7; against the
/// previous pins every earlier row value is bit-identical and each reduced
/// twin is bit-equal to its nominal row (the sections coincide here).
/// Re-pinned for the RF metered charging rule (DEL-05-02
/// rf-source-method-extension.md §5): against the previous pins exactly six
/// leaves of each publication changed, all work-derived: the case, invocation
/// and publication charges, the selected summary's work, and the receipt and
/// publication hashes that bind them. Re-pinned for model 0.6.0 (DEL-04-07
/// Design `u4b-u5b-implementation.md` §1.4): against the previous pins exactly
/// four leaves of each publication changed, all digests of the captured 0.6.0
/// invocation: the invocation value, the normalized-source and functional-plan
/// hashes and the receipt hash; every length is unchanged.
const CURRENT_SOURCE_BASIS: [(Family, &str, &str, PreviewSolverMode, usize, &str); 24] = [
    (Family::Physics, "n05", "90c1edab9cbf5c7caa2d2072cea6484be1681eabac8c8f48d053342dcc7f4efb", PreviewSolverMode::SparseInteractive, 188523, "0a2645ed2fafd7f0653c297b893456c62b9e2357ef0de2426519f6d4b5ea5735"),
    (Family::Physics, "n05", "90c1edab9cbf5c7caa2d2072cea6484be1681eabac8c8f48d053342dcc7f4efb", PreviewSolverMode::DenseScrutiny, 188524, "9c65ee8f582ed402b4fe3b1bf5a94d6fe4f9ccaef680586a8ad8f45ea6ac8b64"),
    (Family::Physics, "n06", "d9e3ede6542ce6d73832d6205d45bed185986295ea192357b2d884048cc3a217", PreviewSolverMode::SparseInteractive, 181730, "d5aff210ac39d17ccdfe5025267a924f65790543293d16a2c58d9201c71f5ec3"),
    (Family::Physics, "n06", "d9e3ede6542ce6d73832d6205d45bed185986295ea192357b2d884048cc3a217", PreviewSolverMode::DenseScrutiny, 181718, "a2ca490d28c9cbd1283b62fe219d2eb0c1206e1233d5fabf426181a59028f800"),
    (Family::Physics, "n05_unicode", "49d235584b3dcad4ab887320e44e470c78379aca8459a4ff3a0c84a013511b3d", PreviewSolverMode::SparseInteractive, 190372, "5af41dcfe5773495fa439dee6f683d1e8c2f9b730180f506e6dd1ba6e228bed3"),
    (Family::Physics, "n05_unicode", "49d235584b3dcad4ab887320e44e470c78379aca8459a4ff3a0c84a013511b3d", PreviewSolverMode::DenseScrutiny, 190373, "d450f6229d3f50cdc0f314331d55c69fa6f2cbce98cc5c983bcea0121874fcd7"),
    (Family::Physics, "n05_units", "e5f0564cb7d977a509db9b5bae3f51d5351c23e1199c3b7d31aac9e78992f23b", PreviewSolverMode::SparseInteractive, 188529, "8eb4e5f802b0d8a7ed73022efa70354fa82cda31ac8951cdc1349e01bf810afd"),
    (Family::Physics, "n05_units", "e5f0564cb7d977a509db9b5bae3f51d5351c23e1199c3b7d31aac9e78992f23b", PreviewSolverMode::DenseScrutiny, 188530, "5dc085669d772e46641513e336e2cb359962646e3175a5c5f5fb041f97fbe559"),
    (Family::Physics, "fields", "b0b8ce054f06382dc15916b436a6a8bad704135884878aa047120eb0071d8852", PreviewSolverMode::SparseInteractive, 197933, "1e04991faa0a169626099ceac5f12e37e36c6d0b5ed515cedc203e495a1e7aeb"),
    (Family::Physics, "fields", "b0b8ce054f06382dc15916b436a6a8bad704135884878aa047120eb0071d8852", PreviewSolverMode::DenseScrutiny, 198114, "91dd19519c60027e73b1002d248f745a37605e649f303e7fd222014500847497"),
    (Family::Physics, "mixed", "0da7828c35c78d5f3fa25b5336d448c0c72ee83b469b908376e2bb09194f3349", PreviewSolverMode::SparseInteractive, 369632, "af5161d5ad8819b6c7c20af0f7da733123ac67e102449282a1bcc4f9d12cb6de"),
    (Family::Physics, "mixed", "0da7828c35c78d5f3fa25b5336d448c0c72ee83b469b908376e2bb09194f3349", PreviewSolverMode::DenseScrutiny, 369626, "0ce597ef7696c6353c1840a43431f67a213e2796443e868f3c1e5ffc9b138509"),
    (Family::Physics, "mixed_units", "f8dc9ffafcf0d3854498ac3456b30264524589432dbe29af90a0ba3dc7c523f5", PreviewSolverMode::SparseInteractive, 369745, "fd04a1765ea7ec945c7a33f4bd33262ab18a233181c8b760b8bf738ed176055b"),
    (Family::Physics, "mixed_units", "f8dc9ffafcf0d3854498ac3456b30264524589432dbe29af90a0ba3dc7c523f5", PreviewSolverMode::DenseScrutiny, 369739, "704e39cf2e57c0316e836e1ef85c414cf81891ba0101c603e180b69ff9e02354"),
    (Family::LoadReference, "n05", "2d0223c1125e5eaefc7f87f8d5f5f2688ce223952a090b4b42fa92c90252a699", PreviewSolverMode::SparseInteractive, 191819, "0f981b2aa30450b84c815bbc742cd9c2ff0f10a6a8036d5f09f01bf2ba31e76e"),
    (Family::LoadReference, "n05", "2d0223c1125e5eaefc7f87f8d5f5f2688ce223952a090b4b42fa92c90252a699", PreviewSolverMode::DenseScrutiny, 191816, "b6e2a4836ac5f062627ef25136cf509ba64f8243ea53ac8eaf57de0cdacc9eaf"),
    (Family::LoadReference, "n06", "2a8226694ec44a4a07fa1e89b3bc4ff2e77e75098071bb5a0a118e5c7c48a55d", PreviewSolverMode::SparseInteractive, 185028, "180ee329a07094749ccbfb0e802e19ecbd8c092333d059e3fc22eb91310e0328"),
    (Family::LoadReference, "n06", "2a8226694ec44a4a07fa1e89b3bc4ff2e77e75098071bb5a0a118e5c7c48a55d", PreviewSolverMode::DenseScrutiny, 185012, "a7d0f751aa74cb92f6de788cb44c021588ff7e0a7e6afee98e82b208ca8b16aa"),
    (Family::LoadReference, "eigen_motion", "79820f837ed1be649af20304adea15402180d2422b983de16a673e53759e0382", PreviewSolverMode::SparseInteractive, 202864, "58060b193373989b50fc2cca53d29ccad5b97a7bb3d9bb576167172161c2e96f"),
    (Family::LoadReference, "eigen_motion", "79820f837ed1be649af20304adea15402180d2422b983de16a673e53759e0382", PreviewSolverMode::DenseScrutiny, 202859, "fbf226cb56a7ed77b50fe6eb7567350eada26154c5edce462452385381b1059b"),
    (Family::LoadReference, "fields", "5479d2422d6700418cf019e3629c16974b85fba7789a8ddb8f20e0ed5939ff82", PreviewSolverMode::SparseInteractive, 202418, "f0dd8e58aefd6b38e3d75825858e0baf24ecaa74753f451cfb77f13471249acf"),
    (Family::LoadReference, "fields", "5479d2422d6700418cf019e3629c16974b85fba7789a8ddb8f20e0ed5939ff82", PreviewSolverMode::DenseScrutiny, 202595, "e0729db50575e33e3285ae5a0d876a49a329f6f282bf190a79715e957db9e134"),
    (Family::LoadReference, "mixed", "9209e39bbd5f8976e59b8989915d4baf67934111537db23cacff170dbf6755db", PreviewSolverMode::SparseInteractive, 375019, "6dc0ad63293ecc51694c514516dec77547813a09b106bc5e9522107bc7d2244f"),
    (Family::LoadReference, "mixed", "9209e39bbd5f8976e59b8989915d4baf67934111537db23cacff170dbf6755db", PreviewSolverMode::DenseScrutiny, 375000, "7df0a056fd604298d791a281d4ee9c4ea6f96d6aa9d5fb1dca0b3fcd6d51f5af"),
];

/// MJ6's kill (B7b meaning): changing the established source work limit
/// (8,000,000, `PHYSICS_SOURCE_WORK_LIMIT`) or any other published byte of a
/// current source publication fails here. Exact producer serialization of all
/// twelve current requests x both modes equals the pinned current basis and
/// the committed raw fixture; no hash, counter, limitation or numeric exclusion
/// applies. Its pre-B7b meaning (the join preserves v2 bytes) lapsed with v2.
#[test]
fn join_preserves_every_source_publication_byte() {
    let mut covered = std::collections::BTreeSet::new();
    for (family, name, request_sha256, mode, length, sha256) in CURRENT_SOURCE_BASIS {
        let request_bytes = std::fs::read(project_root().join(family.request_path(name))).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&request_bytes)),
            request_sha256,
            "{}/{name}: the committed current request",
            family.name()
        );
        let request: Value = serde_json::from_slice(&request_bytes).unwrap();
        assert_eq!(request, family.request(name));
        let produced = produce(&request, mode);
        let label = format!("{}/{name}/{}", family.name(), mode.as_str());
        assert_eq!(
            produced.source["producer"]["semantic_contract_id"],
            ps::PRESSURE_CONTRACT_ID,
            "{label}"
        );
        for case in produced.source["source_block_recovery"]["body"]["cases"].as_array().unwrap() {
            if case["selected_method"] == "retained_source_blocks_exact_v1" {
                assert_eq!(case["work"]["limit"], json!(8_000_000), "{label}: the established work limit");
            }
        }
        assert_eq!(produced.bytes.len(), length, "{label}: exact byte length");
        assert_eq!(
            format!("{:x}", Sha256::digest(produced.bytes.as_bytes())),
            sha256,
            "{label}: exact producer bytes"
        );
        let committed = std::fs::read(project_root().join(format!(
            "fixtures/product_preview/pressure_source/{}/{name}-{}.raw.json",
            family.name(),
            mode.as_str()
        )))
        .unwrap();
        assert_eq!(format!("{:x}", Sha256::digest(&committed)), sha256, "{label}: committed raw");
        assert!(covered.insert(label));
    }
    let mut expected = std::collections::BTreeSet::new();
    for family in [Family::Physics, Family::LoadReference] {
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
