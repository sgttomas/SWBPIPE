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
/// RT-A3 increment 3 (D-2): the load/reference-state publications restate the
/// shared support limitation, 109 bytes longer each; the physics publications are
/// unchanged (test_rt_a3_constant_effort_text_only_envelope_diff).
/// RT-A3 increment 3 review repair: every publication restates the support-attribution
/// limitation (S7) for a consumed constant-effort support, 67 bytes longer each; nothing
/// else moved but the hashes binding it (the same preservation test).
/// RT-L2 (DEL-04-07): every publication states one
/// pressure-elongation statement, 156 bytes longer each without a pressurised case; the
/// mixed requests name their pressurised case, so they state the inclusion statement and
/// the per-case label, their request digests move, and the receipt's request-bound work
/// charges and the hashes chained from the request move with them (324 bytes in all).
const CURRENT_SOURCE_BASIS: [(Family, &str, &str, PreviewSolverMode, usize, &str); 24] = [
    (Family::Physics, "n05", "d12f7a404ad3e8d916cdaa86958efdbc8d86c7c16736ae9b11ac1085dcd90d28", PreviewSolverMode::SparseInteractive, 189210, "dc2f6ad71a773a56d6b8251e012722a4320e5ae6492d65f1f34eba8367df4595"),
    (Family::Physics, "n05", "d12f7a404ad3e8d916cdaa86958efdbc8d86c7c16736ae9b11ac1085dcd90d28", PreviewSolverMode::DenseScrutiny, 189211, "c80653960fc1fcf39d49a94bd048777c25e8474d93bf61846b90fe401bc6c79c"),
    (Family::Physics, "n06", "4a10d62774d21a21514890fc7a573b8386c4bcaa4d80959a6414699d81f0ae2b", PreviewSolverMode::SparseInteractive, 182416, "df01761f46da055cc9bdc64ee56a313375bd8cef27e1beffbedfa991741f6571"),
    (Family::Physics, "n06", "4a10d62774d21a21514890fc7a573b8386c4bcaa4d80959a6414699d81f0ae2b", PreviewSolverMode::DenseScrutiny, 182404, "e2565eacf657ea0e9fef27be9a0a9a4d526b5268149c5110266fcb0b80e88e74"),
    (Family::Physics, "n05_unicode", "304f559b5b5ef8866ff5ef009e38347aef3f2d49f8dc799a8af28e5edc4cafa7", PreviewSolverMode::SparseInteractive, 191059, "c31df3ef36484e3d63319690d0a3d54279809ef1db82b186d2aeb6daedd10f93"),
    (Family::Physics, "n05_unicode", "304f559b5b5ef8866ff5ef009e38347aef3f2d49f8dc799a8af28e5edc4cafa7", PreviewSolverMode::DenseScrutiny, 191060, "b7645b7b3fdf50a5d1062b6ea0b0acdd7b16ef89c32e0d17157021e4a6a376f4"),
    (Family::Physics, "n05_units", "489a47cc4aa32f5c27eaf627cd34857094e5296047fbf89579e7db63418f7447", PreviewSolverMode::SparseInteractive, 189216, "e908e4e85e1cd4b3355c99b71c3bfa9714ef6a2fc3803bcbba7f31589f22c40e"),
    (Family::Physics, "n05_units", "489a47cc4aa32f5c27eaf627cd34857094e5296047fbf89579e7db63418f7447", PreviewSolverMode::DenseScrutiny, 189217, "c69418874d0f9e2ef2d89116b38eb733efd7070330e871c339a94db7f9e66e47"),
    (Family::Physics, "fields", "1cd5959a4c3c832d25ad18e62e8d2ead77a20ff93bc0ac0c02373c33606eb58e", PreviewSolverMode::SparseInteractive, 198620, "812cb802f6a78e435328d4b89c8ebb9bc2d07583c8d0eddf6243f4e9e8866472"),
    (Family::Physics, "fields", "1cd5959a4c3c832d25ad18e62e8d2ead77a20ff93bc0ac0c02373c33606eb58e", PreviewSolverMode::DenseScrutiny, 198801, "89891ca0216b3bcf7d92ff7d3a3c0fa8c91668536ca53042f7c6d4db64c65914"),
    (Family::Physics, "mixed", "5a18bece8c0a601404b782fc89d280a7d290bd3133f879146d68315720815aa6", PreviewSolverMode::SparseInteractive, 370487, "8e7ec95c671a2ba3b1da6757acaac6d7647be8219c3ac3ebe10cbd1d19325b5f"),
    (Family::Physics, "mixed", "5a18bece8c0a601404b782fc89d280a7d290bd3133f879146d68315720815aa6", PreviewSolverMode::DenseScrutiny, 370481, "284085d7f5670b1a2caf7da0d709c5e3acba4a4f1671a3a15a70bf196d498a11"),
    (Family::Physics, "mixed_units", "4280a1b703216170898ee9c6b6e1e1c529f4afe03f2c52d5a7a1c74c83c37dbd", PreviewSolverMode::SparseInteractive, 370600, "2a4f7586a11c1b92638d0f7b904928380f5d98e02922edaabf5e3f8d16703559"),
    (Family::Physics, "mixed_units", "4280a1b703216170898ee9c6b6e1e1c529f4afe03f2c52d5a7a1c74c83c37dbd", PreviewSolverMode::DenseScrutiny, 370594, "9d5f98dd7683004f7138d15000a28de4fa03157b9709bc3c6814c3c08fb0ae1c"),
    (Family::LoadReference, "n05", "9bb4453b6251a37d7027608a26fe07373e0b2aaf39b8a0cba147bfe01815c067", PreviewSolverMode::SparseInteractive, 193227, "ae4c3613e22514702f9529f76fa0d99fac639896d30173e74c488ce30f1944cd"),
    (Family::LoadReference, "n05", "9bb4453b6251a37d7027608a26fe07373e0b2aaf39b8a0cba147bfe01815c067", PreviewSolverMode::DenseScrutiny, 193224, "af26c424b11e4c7fda10d06437f5dc9308eabd70e2432328d3eebcc0d7176a40"),
    (Family::LoadReference, "n06", "e976f055488185b7d9e5cb1ccc59c8bdb503bfe115e83ef430b40a6581847a6c", PreviewSolverMode::SparseInteractive, 186436, "d0fe5db9ca988d4e4191f9a712e41b73ebbbd793b787e2531f6aefb7e2fe22be"),
    (Family::LoadReference, "n06", "e976f055488185b7d9e5cb1ccc59c8bdb503bfe115e83ef430b40a6581847a6c", PreviewSolverMode::DenseScrutiny, 186420, "942f49406d6011c29bae94e32a286845bce5ae6d22e6a17bf69b774c23806c8e"),
    (Family::LoadReference, "eigen_motion", "e90e8b223ceb79dcc4144f85e30f9d8489dcb79baaf0a07ffbb981fc6dc2d1ca", PreviewSolverMode::SparseInteractive, 204272, "9c61c7739226ee488599b24f3aa4301da828b38aff391cdd968c894cbe3eedfc"),
    (Family::LoadReference, "eigen_motion", "e90e8b223ceb79dcc4144f85e30f9d8489dcb79baaf0a07ffbb981fc6dc2d1ca", PreviewSolverMode::DenseScrutiny, 204267, "05c3366ca806948abd774c6f57d7c5b115ae221e599ccd5bed0733942a1b03e8"),
    (Family::LoadReference, "fields", "947fb8ef33732123514db9c53ac03f91045bd82988b8b2bdb5f33b3f9387fbeb", PreviewSolverMode::SparseInteractive, 203826, "a46afe731a57fd4c3a09c0f8c1d00f9184ce42c1c77a93fcf8c6d20571c05919"),
    (Family::LoadReference, "fields", "947fb8ef33732123514db9c53ac03f91045bd82988b8b2bdb5f33b3f9387fbeb", PreviewSolverMode::DenseScrutiny, 204003, "a0aa546bd80498ea1ea5fd2e39a31fab8d16608a501a282682aa8fa24cff85fd"),
    (Family::LoadReference, "mixed", "f037e7832e81bfb6c42723fcef2affbe7676c83077d953e1eb1724f1d4bfa895", PreviewSolverMode::SparseInteractive, 376595, "27e3b100f4216859b1a2fd734e2f538843427f50f2d36520235a262309058bd4"),
    (Family::LoadReference, "mixed", "f037e7832e81bfb6c42723fcef2affbe7676c83077d953e1eb1724f1d4bfa895", PreviewSolverMode::DenseScrutiny, 376576, "e6ec582d81b3d14c4a77c26a5d105d70c0117ce45a7b8483e2ef49b1065d4468"),
];
// RT-A3 S6 (§3.7): the ten load/reference-state publications restate S13 and S15 (the uniform ordinary load-state
// motion and history limitations; +465 B each) and re-seal; every other byte is unchanged (the S6 text-only guard).

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
