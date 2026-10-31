//! Genuine pressure-source consuming controls, produced through the public
//! captured-request API. No result is relabelled, projected or resealed.
//! The n06 equilibrium controls and protected criterion are the existing ones
//! in physics_source_runtime.rs; this is not the reserved desktop G10 witness.
//!
//! B7b deleted the v2 exact contract (`2.0.0/exact_straight_pressure_v2`) and
//! its physics-source-1 and load-reference-source-1 publications. The twelve
//! committed source controls are now the current 0.9.0 documents under
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
    /// The committed current request: 0.9.0 with the current exact contract.
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
/// Re-pinned for the RF metered charging rule (DEL-04-08
/// rf-source-method-extension.md §5): against the previous pins exactly six
/// leaves of each publication changed, all work-derived: the case, invocation
/// and publication charges, the selected summary's work, and the receipt and
/// publication hashes that bind them. Re-pinned for model 0.6.0 (DEL-04-07
/// Design `u4b-u5b-implementation.md` §1.4): against the previous pins exactly
/// four leaves of each publication changed, all digests of the captured 0.7.0
/// invocation: the invocation value, the normalized-source and functional-plan
/// hashes and the receipt hash; every length is unchanged. Re-pinned for
/// T4-U4b S4 (DEL-04-07 Design `u4b-u5b-implementation.md` §4.3): every case
/// of a source publication publishes its straight members' σ⁺, σ⁻ and τ_v,max
/// (fifteen rows per member on each basis, formed from its published rows),
/// treated `derived_not_qualified` on the selected case and
/// `ordinary_physics_checked` on an ordinary-method case. Against the previous
/// pins every earlier row and treatment is unchanged and exactly five other
/// leaves of each publication changed: the selected case's, the invocation's
/// and the publication's charges (the new rows' work) and the receipt and
/// publication hashes; every length grows by the new rows and treatments.
/// Re-pinned for O4's limitation text (DEL-04-08 Design
/// `o4-source-extension.md` §3.3, §4.9): against the previous pins exactly
/// four leaves of each publication changed: limitations [0] and [7] and the
/// receipt and publication hashes that bind them; every work counter, number
/// and row is unchanged and every length is 163 bytes shorter
/// (`o4_text_only_envelope_diff`). Re-pinned for R-A (DEL-04-08 Design
/// `source-work-nondyadic-inputs.md` §4): against the previous pins exactly
/// five leaves of each publication changed, all counted work: the selected
/// case's and the invocation's charges, the selected diagnostic's embedded
/// method work, and the receipt and publication hashes that bind them.
/// `publication_charged`, every value, row and limitation are unchanged;
/// physics n06's two lengths are one byte shorter (its invocation charge
/// loses a digit) and every other length is unchanged
/// (`tests/test_ra_work_only_envelope_diff.py`). Re-pinned for W-16 slice c
/// (DEL-05-01 Design `exact-equivalent-static.md` §4): the ten load/reference
/// publications state the refusal of generated equivalent-static loads in
/// their applied-loads limitation, 147 bytes longer each; the physics
/// publications are unchanged. Re-pinned for W-16 slice b (DEL-05-02 Design
/// `exact-combinations.md` §7): every publication restates the
/// pressure-source-1 identity limitation, 32 bytes shorter each; nothing else
/// moved but the hashes binding it. Merged with model document 0.8.0 (W-15
/// slice c, on main): its re-authored requests move the request digests and the
/// publication hashes; against main's pins the physics lengths are 32 bytes
/// shorter and the load/reference lengths 115 bytes longer (147 - 32).
const CURRENT_SOURCE_BASIS: [(Family, &str, &str, PreviewSolverMode, usize, &str); 24] = [
    (Family::Physics, "n05", "b19f05c4e18b26ab7fa9deb185f35178ee665d410821a70fbd30b52225d05a93", PreviewSolverMode::SparseInteractive, 188967, "c3c03c86e25ad6752c51a9c6c03490f0d55f0d9abb08dadc243ecd5be467cacc"),
    (Family::Physics, "n05", "b19f05c4e18b26ab7fa9deb185f35178ee665d410821a70fbd30b52225d05a93", PreviewSolverMode::DenseScrutiny, 188968, "9b25fb904a92183cd2ffaebd9e3c03b867f89b7076a804c8d7a74c502cc199d1"),
    (Family::Physics, "n06", "f0e76211c0d0f40ae339b92f30a2f58e4152c6a75ae0e74a5210ebb1e77904bc", PreviewSolverMode::SparseInteractive, 182173, "f85fa18e2ae55d5dcea3f28ec041b034a356419dea5cee10fea83c27d7173048"),
    (Family::Physics, "n06", "f0e76211c0d0f40ae339b92f30a2f58e4152c6a75ae0e74a5210ebb1e77904bc", PreviewSolverMode::DenseScrutiny, 182161, "ba11ef236dfd89f1e2ebd82527f464eb5afaa191fc934614a2b107aefc758b48"),
    (Family::Physics, "n05_unicode", "dbd29e78e25fd409cf0aeb2e595ecb6b24318090f7f0c1b279c9d8c4c0259a0d", PreviewSolverMode::SparseInteractive, 190816, "175f95eee756b59dcf120cd401a5f9ff54dee44ad9ec73a35cdbedf82155f9f8"),
    (Family::Physics, "n05_unicode", "dbd29e78e25fd409cf0aeb2e595ecb6b24318090f7f0c1b279c9d8c4c0259a0d", PreviewSolverMode::DenseScrutiny, 190817, "130c06fc79f62f56861257174b0b5f51878ffd98c8b9e2ae54fd7d00562d9f24"),
    (Family::Physics, "n05_units", "451451e9ae4dce61508a4b3f7e465f38b707dfdfd3ddf1d7f251fa5d4988ff80", PreviewSolverMode::SparseInteractive, 188973, "62ff4a07532fef74d43fb1acd7487867c2ed8435731a079e8d0fc2a862947737"),
    (Family::Physics, "n05_units", "451451e9ae4dce61508a4b3f7e465f38b707dfdfd3ddf1d7f251fa5d4988ff80", PreviewSolverMode::DenseScrutiny, 188974, "02f9a39857c81b0f00019883c13c70bd9c40a90161edcd2fb2cb6fd105cf63ed"),
    (Family::Physics, "fields", "22f5085e243dcbb73cb6732b115780afcabc645a655e8dbb470a59d898600093", PreviewSolverMode::SparseInteractive, 198377, "f24736ccbec433bd629f9069c28fb27f0ad373ada4158091349060859ffaee73"),
    (Family::Physics, "fields", "22f5085e243dcbb73cb6732b115780afcabc645a655e8dbb470a59d898600093", PreviewSolverMode::DenseScrutiny, 198558, "a261f8e39711eb84e55848fb259b1725cb66448e05e46e09f9bc026de8a30ad4"),
    (Family::Physics, "mixed", "b3bd568c2cacc8591c90219da1a73754ed1f495f872471372d66bc58c4479e44", PreviewSolverMode::SparseInteractive, 370076, "5c38dc652710e07159b8a8c3feee4c9aaa6b0918be96a6afc24f92f2a725c2be"),
    (Family::Physics, "mixed", "b3bd568c2cacc8591c90219da1a73754ed1f495f872471372d66bc58c4479e44", PreviewSolverMode::DenseScrutiny, 370070, "ac0369624ade64d50f0fd04b1c1794125603f67fda2533ba0709fdaa8406fb62"),
    (Family::Physics, "mixed_units", "bc92b30390eccbb20421ace77e8b8348968e702a7601ef62b8080f0310be31c9", PreviewSolverMode::SparseInteractive, 370189, "1e1f982d28b18c4acedfb4af1b6742c65fc168584c4d872c3ecf513f24a56284"),
    (Family::Physics, "mixed_units", "bc92b30390eccbb20421ace77e8b8348968e702a7601ef62b8080f0310be31c9", PreviewSolverMode::DenseScrutiny, 370183, "429d5c8a2cb2d7f64485ec3e44d2e310ab6fb63afba892145e2f792868b584ca"),
    (Family::LoadReference, "n05", "bcb96a3f13d6a51e3634fa1b68dde72ba1dcd5fd6b53356111f746092e9d44b1", PreviewSolverMode::SparseInteractive, 192410, "2a4e0c0e36a45e0915c8df9a49b09f3c2d4c5ac2d6bd4b60529cd3bad73414fe"),
    (Family::LoadReference, "n05", "bcb96a3f13d6a51e3634fa1b68dde72ba1dcd5fd6b53356111f746092e9d44b1", PreviewSolverMode::DenseScrutiny, 192407, "478ea8649fa24696e09342c31ee85570f1a7eccaa0c3af70c2db14313002db4d"),
    (Family::LoadReference, "n06", "4b41b45a72c44f4b970b083f71bbf788616122a4a05ca5e9a4a018eb69aae677", PreviewSolverMode::SparseInteractive, 185619, "77b1874ecf5a1220cf06e86025583310ea0bbec1e2a4dc51eaa65d6c166e9cf2"),
    (Family::LoadReference, "n06", "4b41b45a72c44f4b970b083f71bbf788616122a4a05ca5e9a4a018eb69aae677", PreviewSolverMode::DenseScrutiny, 185603, "6cb1ea5a93e6cc1f75c666a3c6c85e8faca326566c53a1b3e81356afec8fceed"),
    (Family::LoadReference, "eigen_motion", "26ecd51a47ba09a28ef0a3165ad16862d7b6ae212179a780cc222d1a0bd6388f", PreviewSolverMode::SparseInteractive, 203455, "c5be2a8cbe3f9552ac414dccb05e70da4cb30e3fc3e5ce296a8b4ca500d79124"),
    (Family::LoadReference, "eigen_motion", "26ecd51a47ba09a28ef0a3165ad16862d7b6ae212179a780cc222d1a0bd6388f", PreviewSolverMode::DenseScrutiny, 203450, "3b52127c84eb3ecdd2f546405a0bb937a2c67dabd20fdb504a6a97b4ac12b203"),
    (Family::LoadReference, "fields", "e08ed5cb01ec3b94fedd3e441f666c5eb257f5b2018cce1e7db9536ef71bf218", PreviewSolverMode::SparseInteractive, 203009, "cf5e3611af302efe7a1a879e95f3000368d6052b65393b9d0e1fd0781f33be49"),
    (Family::LoadReference, "fields", "e08ed5cb01ec3b94fedd3e441f666c5eb257f5b2018cce1e7db9536ef71bf218", PreviewSolverMode::DenseScrutiny, 203186, "695534bbc8654d8b0f6afb718e3d320a79a91b57a3b6f36ec980ef85e68dfbc0"),
    (Family::LoadReference, "mixed", "be1971fd0a2a2dc50a102ba5a511a949c4494bd82a2fcda24af889fc1e225397", PreviewSolverMode::SparseInteractive, 375610, "fd7c020f8401b9176e2f8af37b5b27c762e597ca416d25279705d7fa2612d75e"),
    (Family::LoadReference, "mixed", "be1971fd0a2a2dc50a102ba5a511a949c4494bd82a2fcda24af889fc1e225397", PreviewSolverMode::DenseScrutiny, 375591, "15ed9d274de123760dd9b9cad8dced7cdded39405d8f4dcaa1f1804c3841a7b7"),
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
