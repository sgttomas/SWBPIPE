//! T6S-2 (I76): Rust goldens of the successor's result-export derivative, the
//! reference bytes for the desktop's TypeScript derivative (T6S-4, byte parity).
//! B3b-T (I101; I93 PLAN decision 21): the same goldens for the exact successor
//! (`physics-retained-1`), derived from lane P's pinned m3x successors with the
//! same base and origin builders (`retained_precision_exact_successor_derivative_{mode}.json`).
//!
//! Each golden is Rust `derivative::derive_document` applied to one of PP's
//! pinned milestone successors, with one fixed desktop-shaped base and origin:
//! the base and origin that `buildCurrentResultExport`
//! (`apps/desktop/src/features/result-export/resultExportAdapter.ts`) builds,
//! with fixed `test:` stand-ins for the AnalysisRun and InputManifest values it
//! reads (no AnalysisRun or manifest is built here). The model is the pinned
//! request's model, and no request is passed (the desktop passes none).
//!
//! A golden file is the document's canonical JSON text (`canonical_json`,
//! openpipestress_jcs_ijson_v1, no trailing newline), so its file sha256 is the
//! document's canonical digest. The goldens are test-built derivatives of
//! producer test bytes: not product exports, not native Current evidence, and
//! not producer-attested (the origin says so).
//!
//! Regeneration (from `core/reporting/result_export`; `<P>` is
//! `projects/chirality-piping`): run this test with `I76_T6S2_OUT` naming the
//! output folder, e.g.
//! `I76_T6S2_OUT=<P>/fixtures/results cargo test --locked --offline --test retained_precision_derivative_golden`,
//! which writes the goldens and prints their sha256; then set `GOLDEN_SHA256`
//! and `EXACT_GOLDEN_SHA256` to the printed values and rerun without the variable.
//! Without the variable the test compares the live derivative with the committed
//! files by sha256.
//! `I76_T6S2_INPUTS_OUT=<folder>` also writes each mode's exact base and
//! origin (canonical JSON) there (`exact_`-prefixed for the exact successor's).
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_result_export::{
    derivative as d, retained_precision as rp, semantic_contract as s, source_blocks,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const SPARSE: &str = include_str!(
    "../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"
);
const DENSE: &str = include_str!(
    "../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"
);
/// PP's pins of the two successors (retained_facade_tests.rs `PINNED`):
/// mode, file text, file sha256, receipt sha256.
/// Model 0.9.0 re-pin (S1; DEL-02-01 Design `model-document-0.9.0.md` §3.6): read from the regenerated producer bytes on
/// the registered build; only the version token and the hashes binding it moved (no numeric leaf, class or status).
/// RT-A4 D3 re-pin (DEL-04-08 §8.46): the regenerated carriers move only `formulation_basis.limitations[6]` (§10.2) and the
/// digests binding it; each restated to the former sentence and resealed (Python 07e) reproduces its former pin.
const PINNED: [(&str, &str, &str, &str); 2] = [
    (
        "sparse_interactive",
        SPARSE,
        "49dfe6ebab459bbfdb68a541080313199cc24f85e649f1bbfb005c5be155a567",
        "8ce5b023d43eeb764bb779e44cc088d11616e14332d0e9f2898c9acd9b665137",
    ),
    (
        "dense_scrutiny",
        DENSE,
        "d1c8edd1efcaf0986d2625714d76dbca81275e2fa32acaf9f4ade798dba48b11",
        "c20407e71fd6d9b4d2089123351ea949deb7ffcf2b14df61a451355f888d445a",
    ),
];
/// The committed goldens' file sha256 (= the documents' canonical digests).
/// Model 0.9.0 re-pin (S1; DEL-02-01 Design `model-document-0.9.0.md` §3.6): read from the regenerated producer bytes on
/// the registered build; only the version token and the hashes binding it moved (no numeric leaf, class or status).
/// RT-A4 D3 re-pin (DEL-04-08 §8.46): the regenerated goldens move only `limitations[6]` (§10.2), the two seals,
/// `received_carrier_checksum` and `derivative_hash`, each equal to its recomputation (the RT-A3 CE guard's derived_pairs).
const GOLDEN_SHA256: [(&str, &str); 2] = [
    ("sparse_interactive", "f8f30a04084333e3686fd9765cbe8a9fe82ece183fc751966e531cdc1f0b2dc3"),
    ("dense_scrutiny", "fb28270f66a28c82c40a01e6ec6b775daed6fc3a766fde9b11d39ac4a26036fe"),
];
/// The milestone's validated class counts (as retained_precision_carriers.rs
/// `CLASSES`): relative, absolute, input-derived, non-quantity and not-covered (T4-U4b: the member's 15
/// derived σ⁺, σ⁻ and τ_v,max rows at five stations; W-7: its 12 global end rows).
const CLASSES: [(&str, [usize; 5]); 2] = [
    ("sparse_interactive", [25, 69, 3, 1, 15 + 12]),
    ("dense_scrutiny", [25, 69, 3, 1, 15 + 12]),
];
/// B3b-T: lane P's m3x exact successors (PP retained_facade_tests.rs `EXACT_PINNED`):
/// mode, file text, file sha256 (the fixture document's), receipt sha256.
/// O4 (W1-E): re-pinned with the exact goldens after DEF-E's revision (only DEF-E-bound hashes moved).
/// O4 limitation re-pin (§3.3, §4.9): the base limitation [7] and the hashes binding it moved.
/// W-15 (DEL-03-05): re-pinned for the pressure-2 identity limitation and the hashes binding it.
/// W-16 slice b (DEL-05-02): re-pinned again for the pressure-2 identity limitation and the hashes binding it.
/// RT-A3 increment 1: re-pinned for the pressure-2 identity limitation (identity bytes only).
/// Model 0.9.0 re-pin (S1; DEL-02-01 Design `model-document-0.9.0.md` §3.6): read from the regenerated producer bytes on
/// the registered build; only the version token and the hashes binding it moved (no numeric leaf, class or status).
/// RT-A3 S6 (§8.59): S1's nonlinear clause restated; PP `EXACT_PINNED`'s S6 re-pin (restating it to main 394f4c623's
/// and resealing with the Python 07e rule reproduces main's document and receipt).
/// Pressure elongation (RT-L2): PP `EXACT_PINNED`'s re-pin; the successors name the pressure-3 identity and state the
/// setting (restating them and resealing by the 07e rule reproduces the former documents; no numeric leaf moved).
const EXACT_PINNED: [(&str, &str, &str, &str); 2] = [
    (
        "sparse_interactive",
        include_str!("../../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json"),
        "40b2b1991dfc86f602a8c74f53c2e2d07bf935fcbcded5d374e9e0e702d46b08",
        "eccf7619075504b3dc3fd9d2bcbac700ce489da62e29b7876a0a897d5d42c0ee",
    ),
    (
        "dense_scrutiny",
        include_str!("../../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json"),
        "d425e5a228b01079f1a9bee790fd66157f7d94f345eafab1a5961aa5b0069402",
        "71c11cd36323869a680349bf7ea9393a919b90eb0a1fe78fd43c4a498957a776",
    ),
];
/// The committed exact goldens' file sha256.
/// Model 0.9.0 re-pin (S1; DEL-02-01 Design `model-document-0.9.0.md` §3.6): read from the regenerated producer bytes on
/// the registered build; only the version token and the hashes binding it moved (no numeric leaf, class or status).
/// Pressure elongation (RT-L2): the exact successors they derive from name the pressure-3 identity and state the setting;
/// only that text and the digests binding it moved (was 6f943339.../c39d21a8...).
const EXACT_GOLDEN_SHA256: [(&str, &str); 2] = [
    // W-15 (DEL-03-05) on W-7: the exact successors restate the pressure-2 identity limitation.
    // W-16 slice b (DEL-05-02): and again (was 9c4756e9.../579878c8...).
    // RT-A3 increment 1: re-pinned for the pressure-2 identity limitation (identity bytes only) (was e15c27cd.../e0c14d0e...).
    // RT-A3 increment 3: the identity and the support-attribution limitation (S7) restated; the exact successors
    // they derive from are proven by reseal-to-base (DEL-04-08 §8.44).
    // RT-A3 S6 (§8.59): S1's nonlinear clause restated in the exact successors they derive from (proven by
    // reseal-to-base); only that text and the digests binding it moved (was 4e0e0cb8.../f75a417e...).
    ("sparse_interactive", "1e8d4ae92756130865544b6945447a198a941a45fff357029d4e2fe94512493a"),
    ("dense_scrutiny", "bcf05a0dea9e6fdf6d265845f6a916803f50a88f8461a069af2a9a266a16f9cc"),
];
/// The m3x successors' validated class counts (relative, absolute, input-derived,
/// non-quantity, not-covered). T4-U6 (D-5 §2.5): each of the member's 21
/// reduced stress twins takes its nominal row's class (5 relative, 16 absolute;
/// the receipt's own lists stay at 25 and 69). T4-U4b (U4B-RP): the 30 derived
/// σ⁺, σ⁻ and τ_v,max rows (five stations, both bases) are not covered; so are W-7's 12 global end rows.
const EXACT_CLASSES: [(&str, [usize; 5]); 2] = [
    ("sparse_interactive", [25 + 5, 69 + 16, 3, 1, 30 + 12]),
    ("dense_scrutiny", [25 + 5, 69 + 16, 3, 1, 30 + 12]),
];
/// One set of goldens: the pinned successors, their identity, the goldens' file
/// prefix and sha256, and the class counts; `label` prefixes the mode in messages
/// and in `I76_T6S2_INPUTS_OUT`'s file names.
struct GoldenSet {
    label: &'static str,
    identity: &'static str,
    pinned: [(&'static str, &'static str, &'static str, &'static str); 2],
    prefix: &'static str,
    golden_sha256: [(&'static str, &'static str); 2],
    classes: [(&'static str, [usize; 5]); 2],
}
const T6S2: GoldenSet = GoldenSet {
    label: "",
    identity: s::PREVIEW_PHYSICS_RETAINED_ID,
    pinned: PINNED,
    prefix: "retained_precision_successor_derivative",
    golden_sha256: GOLDEN_SHA256,
    classes: CLASSES,
};
const B3B_EXACT: GoldenSet = GoldenSet {
    label: "exact_",
    identity: s::PHYSICS_RETAINED_ID,
    pinned: EXACT_PINNED,
    prefix: "retained_precision_exact_successor_derivative",
    golden_sha256: EXACT_GOLDEN_SHA256,
    classes: EXACT_CLASSES,
};
const OUT: &str = "I76_T6S2_OUT";
/// Optional: a folder for the exact base and origin of each mode (canonical JSON).
const INPUTS_OUT: &str = "I76_T6S2_INPUTS_OUT";

// ---- The fixed desktop-shaped base and origin -----------------------------------
// Fixed stand-ins for what `buildCurrentResultExport` reads from the AnalysisRun
// and the InputManifest; every other value is the builder's own constant or its
// own function of the model and the result.
/// `inputManifest.manifest_ref.ref`.
const MANIFEST_REF: &str = "test:t6s-golden-reference-only-manifest";
/// `inputManifest.manifest.solver_basis.solver_build_ref`.
const SOLVER_BUILD_REF: &str = "test:t6s-golden-pinned-producer-test-bytes-not-native-attestation";
/// The builder's `origin_limit` claims a qualified Current carrier; a test-built
/// golden must not, so this one text replaces it.
const ORIGIN_LIMIT: &str = "Test-built desktop-shaped origin (T6S golden): pinned producer test bytes, not a qualified Current received carrier; independent authentic original producer bytes unavailable; dimension absence is not producer attestation";

/// resultExportAdapter.ts `derivativeProvenance`.
fn derivative_provenance() -> Value {
    json!({"source_name":"local qualified result derivative","source_location":"apps/desktop/src/features/result-export/resultExportAdapter.ts","source_license":"project-local","contributor":"SWBPIPE","contributor_certification":"local desktop session; human review required","redistribution_status":"private_only","review_status":"pending"})
}
/// The builder's diagnostic mapping (`documentDiagnostic`): Rust's own
/// (`core/runner/headless/src/result_envelope_binding.rs`, `export_mechanics_diagnostic`)
/// with the desktop derivative's provenance.
fn desktop_diagnostic(x: &Value) -> Value {
    let code = x["code"].as_str().unwrap();
    let severity = x["severity"].as_str().unwrap();
    let class = if code.contains("RULE") {
        if severity == "blocking" { "RULE_CHECK_BLOCKING" } else { "ASSUMPTION_WARNING" }
    } else if code.contains("UNIT") {
        "UNIT_WARNING"
    } else if code.contains("PROVENANCE") {
        "PROVENANCE_WARNING"
    } else if severity != "info" && severity != "warning" {
        "SOLVE_BLOCKING"
    } else if code.starts_with("NONLINEAR") {
        "NONLINEAR_WARNING"
    } else {
        "ASSUMPTION_WARNING"
    };
    let source = if x["source"].is_null() { json!("core/product_physics") } else { x["source"].clone() };
    json!({
        "code": x["code"],
        "class": class,
        "severity": if severity == "info" || severity == "warning" { json!(severity) } else { json!("blocking") },
        "source": {"ref_type":"solver_component","ref_id":source},
        "affected_object": {"ref_type":"preview_diagnostic","ref_id":x["id"].as_str().expect("a source diagnostic carries its id")},
        "message": x["message"],
        "remediation": "Review the carried solve diagnostic before relying on this exported result envelope.",
        "provenance": derivative_provenance(),
    })
}
/// The builder's base. The AnalysisRun stand-in: `run_id` is the result's; its
/// load basis is `modelLoadBasisRefs(model)`; its statuses are the analysis
/// builder's sorted [HUMAN_REVIEW_REQUIRED, mechanics, rule_check]; its
/// boundary is the analysis builder's; it has no hashes (no record is built).
fn desktop_base(source: &Value, model: &Value) -> Value {
    let run = source["run_id"].as_str().unwrap();
    let project = model["project"]["id"].as_str().unwrap();
    let mut load_basis: Vec<Value> = model["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| d::reference("LoadCase", c["id"].as_str().unwrap()))
        .collect();
    load_basis.extend(
        model["combinations"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|c| d::reference("Combination", c["id"].as_str().unwrap())),
    );
    let mut statuses = vec![
        "HUMAN_REVIEW_REQUIRED".to_string(),
        source["status"]["mechanics"].as_str().unwrap().to_string(),
        source["status"]["rule_check"].as_str().unwrap().to_string(),
    ];
    statuses.sort();
    statuses.dedup();
    let diagnostics: Vec<Value> = source["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(desktop_diagnostic)
        .collect();
    json!({
        "schema_version": s::RESULT_EXPORT_VERSION,
        "deliverable_id": "DEL-08-04",
        "package_id": "PKG-08",
        "scope_item": "SOW-046",
        "objectives": ["OBJ-007", "OBJ-009"],
        "export_format_status": {"baseline_format":"schema_first_json_result_envelope","additional_formats":"TBD","public_transport_protocol":"TBD","local_fea_package_format":"TBD","external_adapter_formats":"TBD"},
        "result_envelope": {
            "schema_version": s::RESULT_EXPORT_VERSION,
            "envelope_id": format!("result-envelope:{run}"),
            "model_ref": d::reference("model_payload", project),
            "run_ref": d::reference("analysis_run", run),
            "solver_version": {
                "solver_name": source["producer"]["component_name"],
                "solver_version": source["producer"]["component_version"],
                "solver_build_ref": SOLVER_BUILD_REF,
            },
            "unit_system_ref": d::reference("unit_system", &format!("{project}:units")),
            "load_basis_refs": load_basis,
            "result_sets": [{"set_id":format!("result-set:{run}:mechanics"),"set_type":"mechanics","basis_ref":d::reference("analysis_run", run),"values":[]}],
            "diagnostics": diagnostics,
            "provenance": derivative_provenance(),
            "reproducibility": {"model_hash":null,"run_hashes":[],"audit_manifest_ref":d::reference("audit_manifest", MANIFEST_REF),"deterministic_ordering":true},
            "analysis_status": statuses,
            "professional_boundary": {"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},
            "downstream_use": {"review":true,"regression_comparison":true,"report_consumption":true,"headless_automation":true,"governed_downstream_tooling":true,"additional_export_formats":"TBD"},
        }
    })
}
/// The builder's origin for a dimension-absent carrier (every successor row
/// lacks `dimension`), with the test `ORIGIN_LIMIT`.
fn desktop_origin(source: &Value, model: &Value) -> Value {
    let run = source["run_id"].as_str().unwrap();
    json!({
        "origin_id": "source-origin:current-received",
        "origin_class": "received_current_dimension_absent",
        "qualification_ref": d::reference("current_manifest", MANIFEST_REF),
        "authentic_producer_available": false,
        "received_carrier_checksum": d::checksum(source, "received_current_dimension_absent_carrier", d::reference("received_current_carrier", run)).unwrap(),
        "original_producer_checksum": null,
        "origin_limit": ORIGIN_LIMIT,
        "actual_model_ref": d::reference("model_payload", model["project"]["id"].as_str().unwrap()),
        "mechanics_run_ref": d::reference("mechanics_run", run),
        "request_model_ref": null,
        "request_run_ref": null,
        "request_alias_disclosure": null,
    })
}

// ---- Inputs and derivation ---------------------------------------------------------
struct Milestone {
    /// The set's label and the mode, for messages and file names.
    mode: String,
    set: &'static GoldenSet,
    pinned_mode: &'static str,
    source: Value,
    model: Value,
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn milestones(set: &'static GoldenSet) -> Vec<Milestone> {
    set.pinned
        .iter()
        .map(|(pinned_mode, text, file_sha, receipt_sha)| {
            let mode = format!("{}{pinned_mode}", set.label);
            assert_eq!(sha(text.as_bytes()), *file_sha, "{mode}: PP's pinned successor bytes");
            let doc: Value = serde_json::from_str(text).unwrap();
            assert_eq!(doc["source"]["retained_precision"]["receipt_sha256"], *receipt_sha);
            assert_eq!(doc["invocation"]["solver_mode"], *pinned_mode);
            assert_eq!(doc["source"]["producer"]["semantic_contract_id"], set.identity);
            assert!(
                doc["source"]["results"].as_array().unwrap().iter().all(|r| r.get("dimension").is_none()),
                "{mode}: a dimension-absent carrier"
            );
            Milestone {
                mode,
                set,
                pinned_mode,
                source: doc["source"].clone(),
                model: doc["invocation"]["request"]["model"].clone(),
            }
        })
        .collect()
}
fn derive(source: &Value, model: &Value) -> Result<Value, String> {
    d::derive_document(desktop_base(source, model), model, source, desktop_origin(source, model), None)
}
fn golden_name(m: &Milestone) -> String {
    format!("{}_{}.json", m.set.prefix, m.pinned_mode)
}
fn committed(m: &Milestone) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/results")
        .join(golden_name(m))
}
fn pinned_golden(m: &Milestone) -> &'static str {
    m.set.golden_sha256.iter().find(|(mode, _)| *mode == m.pinned_mode).unwrap().1
}
/// Recompute the publication and receipt hashes after an edit, so the edited
/// statement is hash-consistent (as retained_precision_carriers.rs `rehash`).
fn rehash(mut raw: Value) -> Value {
    let mut public = raw.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    raw["retained_precision"]["body"]["publication_sha256"] =
        json!(source_blocks::domain_hash("retained_precision_publication_mp_v2", &public).unwrap());
    let body = raw["retained_precision"]["body"].clone();
    raw["retained_precision"]["receipt_sha256"] =
        json!(source_blocks::domain_hash("retained_precision_receipt_mp_v2", &body).unwrap());
    raw
}

/// CQ-5: no `request` or `solver_mode` member anywhere in the document.
fn no_invocation(v: &Value) -> bool {
    match v {
        Value::Object(o) => o
            .iter()
            .all(|(k, c)| k != "request" && k != "solver_mode" && no_invocation(c)),
        Value::Array(a) => a.iter().all(no_invocation),
        _ => true,
    }
}

// ---- The goldens -------------------------------------------------------------------
/// The live derivative equals the committed golden, by sha256 (or, with
/// `I76_T6S2_OUT`, is written there).
#[test]
fn t6s2_goldens_are_the_live_successor_derivative() {
    goldens_are_the_live_derivative(&T6S2);
}
/// B3b-T: the exact successor's goldens, as T6S-2's.
#[test]
fn b3b_exact_goldens_are_the_live_successor_derivative() {
    goldens_are_the_live_derivative(&B3B_EXACT);
}
fn goldens_are_the_live_derivative(set: &'static GoldenSet) {
    let out = std::env::var(OUT).ok().map(PathBuf::from);
    let inputs = std::env::var(INPUTS_OUT).ok().map(PathBuf::from);
    for m in milestones(set) {
        if let Some(dir) = &inputs {
            // The exact base and origin, for reproduction in TypeScript.
            for (name, value) in [("base", desktop_base(&m.source, &m.model)), ("origin", desktop_origin(&m.source, &m.model))] {
                std::fs::write(dir.join(format!("{}.{name}.json", m.mode)), canonical_json(&value)).unwrap();
            }
        }
        let doc = derive(&m.source, &m.model).unwrap_or_else(|e| panic!("{}: {e}", m.mode));
        d::validate_document(&doc, &m.source).unwrap();
        let text = canonical_json(&doc);
        let live = sha(text.as_bytes());
        assert_eq!(d::digest(&doc).unwrap(), live, "{}: the file sha256 is the canonical digest", m.mode);
        // The derivation is deterministic.
        assert_eq!(canonical_json(&derive(&m.source, &m.model).unwrap()), text, "{}", m.mode);
        if let Some(dir) = &out {
            std::fs::write(dir.join(golden_name(&m)), &text).unwrap();
            eprintln!("{OUT}: wrote {} sha256 {live}", golden_name(&m));
            continue;
        }
        let file = std::fs::read(committed(&m))
            .unwrap_or_else(|e| panic!("{}: the committed golden {}: {e}", m.mode, golden_name(&m)));
        assert_eq!(sha(&file), pinned_golden(&m), "{}: the committed golden is the pinned one", m.mode);
        assert_eq!(live, pinned_golden(&m), "{}: the live derivative is the committed golden", m.mode);
    }
}

/// Controls on the golden: the desktop shape, the receipt whole,
/// `contract_evidence`, and exactly one D-U6-2 disclosure per `absolute_verified`
/// or `not_covered` row, with its code and message.
#[test]
fn t6s2_golden_carries_the_receipt_evidence_and_class_disclosures() {
    golden_carries_the_receipt_evidence_and_class_disclosures(&T6S2);
}
/// B3b-T: the same controls on the exact successor's goldens (physics-1's evidence
/// and XTABLE's identity).
#[test]
fn b3b_exact_golden_carries_the_receipt_evidence_and_class_disclosures() {
    golden_carries_the_receipt_evidence_and_class_disclosures(&B3B_EXACT);
}
fn golden_carries_the_receipt_evidence_and_class_disclosures(set: &'static GoldenSet) {
    for m in milestones(set) {
        let doc = derive(&m.source, &m.model).unwrap();
        let e = &doc["result_envelope"];
        assert_eq!((doc["schema_version"].as_str(), e["schema_version"].as_str()), (Some("0.3.0"), Some("0.3.0")));
        // D2 4.9.7: the receipt travels whole; the evidence and base statements as received.
        assert_eq!(e["retained_precision"], m.source["retained_precision"], "{}", m.mode);
        assert!(e["contract_evidence"].is_object(), "{}", m.mode);
        assert_eq!(e["contract_evidence"], m.source["contract_evidence"], "{}", m.mode);
        for key in ["producer", "numerical_quality", "formulation_basis"] {
            assert_eq!(e[key], m.source[key], "{} {key}", m.mode);
        }
        assert_eq!(e["semantic_contract_ref"], d::reference("semantic_contract", m.set.identity));
        assert!(e.get("source_block_recovery").is_none());
        // The desktop origin: no producer attestation, no request, no invocation (D-U7-6, CQ-5).
        let origins = e["reproducibility"]["source_origin_bindings"].as_array().unwrap();
        assert_eq!(origins.len(), 1);
        assert_eq!(origins[0], desktop_origin(&m.source, &m.model));
        assert_eq!(origins[0]["authentic_producer_available"], false);
        assert!(origins[0]["original_producer_checksum"].is_null());
        assert_eq!(e["reproducibility"]["raw_source_hashes"], json!([]));
        assert!(e["reproducibility"]["request_hash"].is_null());
        // The receipt's own `body.invocation` is its hash statement, not the invocation.
        assert!(no_invocation(&doc), "{}: no {{request, solver_mode}} in the export", m.mode);
        assert_eq!(e["reproducibility"]["model_hash"], d::checksum(&m.model, "model_payload", d::reference("model_payload", m.source["model_ref"].as_str().unwrap())).unwrap());
        // The classes come from the accepted reader without an invocation.
        let validation = rp::validate(&m.source, None).unwrap();
        let classes = s::retained_row_classes(&m.source).unwrap().unwrap();
        let mut counts = [0usize; 5];
        for c in &validation.classifications {
            counts[match c.class {
                rp::AccuracyClass::RelativeVerified => 0,
                rp::AccuracyClass::AbsoluteVerified { .. } => 1,
                rp::AccuracyClass::InputDerived => 2,
                rp::AccuracyClass::NonQuantity => 3,
                rp::AccuracyClass::NotCovered => 4,
            }] += 1;
        }
        let expected = m.set.classes.iter().find(|(mode, _)| *mode == m.pinned_mode).unwrap().1;
        assert_eq!(counts, expected, "{}", m.mode);
        let rows = m.source["results"].as_array().unwrap();
        let disclosures = e["row_disclosures"].as_array().unwrap();
        let claimed = |x: &&Value| {
            x["reason_code"] == d::RETAINED_ABSOLUTE_VERIFIED || x["reason_code"] == d::RETAINED_NOT_COVERED
        };
        let mut classed = 0;
        for (i, row) in rows.iter().enumerate() {
            let id = row["id"].as_str().unwrap();
            let kind = row["kind"].as_str().unwrap();
            let account = &e["row_accounting"][i];
            let mine: Vec<&Value> = disclosures.iter().filter(|x| x["source_result_id"] == id).collect();
            let Some((code, message)) =
                d::class_disclosure(kind, row["unit"].as_str().unwrap(), classes.get(id))
            else {
                assert!(mine.iter().all(|x| !claimed(x)), "{} {id}: no class code without a class", m.mode);
                continue;
            };
            classed += 1;
            assert_eq!(mine.len(), 1, "{} {id}: exactly one disclosure", m.mode);
            let x = mine[0];
            assert_eq!((x["reason_code"].as_str(), x["message"].as_str()), (Some(code), Some(message.as_str())), "{} {id}", m.mode);
            assert_eq!((&x["source_value"], &x["source_unit"]), (&row["value"], &row["unit"]), "{} {id}: retained, not valued", m.mode);
            assert_eq!(account["disposition"], "disclosed", "{} {id}", m.mode);
            assert_eq!(account["target_ref"], d::reference("row_disclosure", id));
            assert!(
                e["unit_preservation_witnesses"].as_array().unwrap().iter().all(|w| w["source_row_index"] != i),
                "{} {id}: no value witness for a withheld row",
                m.mode
            );
            // D-U6-2's text, checked independently of class_disclosure's format.
            let tail = "; source value/unit and annotation retained; withheld from rule binding and reliance";
            assert!(message.starts_with(&format!("{kind}: {code}; ")) && message.ends_with(tail), "{message}");
            match classes.get(id) {
                Some(rp::AccuracyClass::AbsoluteVerified { bound_bits }) => {
                    // CQ-1: b is Rust's `{:e}` (shortest round-trip digits, no `+`),
                    // in the reader's SI unit, followed by its binary64 bits.
                    let after = message.split("absolute bound b = ").nth(1).unwrap();
                    let (bound, rest) = after.split_once(' ').unwrap();
                    assert_eq!(bound.parse::<f64>().unwrap().to_bits(), *bound_bits, "{message}");
                    assert!(!bound.contains('+'), "{message}");
                    assert!(rest.contains(&format!("(binary64 {bound_bits:016x}), below the relative accuracy floor")), "{message}");
                }
                Some(rp::AccuracyClass::NotCovered) => {
                    assert_eq!(message, format!("{kind}: {code}; no verified accuracy for this quantity kind{tail}"));
                }
                other => panic!("{} {id}: {other:?}", m.mode),
            }
        }
        assert_eq!(classed, expected[1] + expected[4], "{}: one disclosure per absolute_verified or not_covered row", m.mode);
        assert_eq!(disclosures.iter().filter(claimed).count(), classed, "{}", m.mode);
    }
}

/// Controls on the pin: an edited pinned successor changes the golden's hash.
/// A hash-consistent edit (one diagnostic's message, resealed) is still derived,
/// to a document with another sha256; an unresealed edit is refused.
#[test]
fn t6s2_a_mutated_pinned_successor_changes_the_golden() {
    a_mutated_pinned_successor_changes_the_golden(&T6S2);
}
/// B3b-T: the same controls on the exact successor's pin.
#[test]
fn b3b_a_mutated_pinned_exact_successor_changes_the_golden() {
    a_mutated_pinned_successor_changes_the_golden(&B3B_EXACT);
}
fn a_mutated_pinned_successor_changes_the_golden(set: &'static GoldenSet) {
    for m in milestones(set) {
        let golden = sha(canonical_json(&derive(&m.source, &m.model).unwrap()).as_bytes());
        let mut edited = m.source.clone();
        let index = edited["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .position(|x| x["code"] == "RULE_CHECK_INPUTS_MISSING")
            .unwrap();
        let message = format!("{} (edited)", edited["diagnostics"][index]["message"].as_str().unwrap());
        edited["diagnostics"][index]["message"] = json!(message);
        assert!(derive(&edited, &m.model).is_err(), "{}: an unresealed edit is refused", m.mode);
        let resealed = rehash(edited);
        let doc = derive(&resealed, &m.model).unwrap_or_else(|e| panic!("{}: {e}", m.mode));
        let changed = sha(canonical_json(&doc).as_bytes());
        assert_ne!(changed, golden, "{}: the edit reaches the golden", m.mode);
        // A one-ulp edit of a relative_verified row's value, resealed, is refused
        // by the reader: no golden is derived from it.
        let classes = s::retained_row_classes(&m.source).unwrap().unwrap();
        let mut value = m.source.clone();
        let row = value["results"]
            .as_array()
            .unwrap()
            .iter()
            .position(|r| {
                classes.get(r["id"].as_str().unwrap()) == Some(&rp::AccuracyClass::RelativeVerified)
                    && r["value"].as_f64().is_some_and(|v| v != 0.0)
            })
            .unwrap();
        let v = value["results"][row]["value"].as_f64().unwrap();
        value["results"][row]["value"] = json!(f64::from_bits(v.to_bits() + 1));
        let error = derive(&rehash(value), &m.model).unwrap_err();
        eprintln!("{}: a resealed one-ulp edit of row {row} is refused: {error}", m.mode);
    }
}
