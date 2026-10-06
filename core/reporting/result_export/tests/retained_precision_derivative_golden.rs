//! T6S-2 (I76): Rust goldens of the successor's result-export derivative, the
//! reference bytes for the desktop's TypeScript derivative (T6S-4, byte parity).
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
//! which writes both goldens and prints their sha256; then set `GOLDEN_SHA256`
//! to the printed values and rerun without the variable. Without the variable
//! the test compares the live derivative with the committed files by sha256.
//! `I76_T6S2_INPUTS_OUT=<folder>` also writes each mode's exact base and
//! origin (canonical JSON) there.
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
const PINNED: [(&str, &str, &str, &str); 2] = [
    (
        "sparse_interactive",
        SPARSE,
        "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc",
        "efc1a39bbe83840df6bd0761c932b8020285d3b8c005ba8fd0b45ba10d667494",
    ),
    (
        "dense_scrutiny",
        DENSE,
        "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5",
        "3e26499f17caff8f5fc0d46406bbe54acf43e8cb16e761784aa5074413b0ac4a",
    ),
];
/// The committed goldens' file sha256 (= the documents' canonical digests).
const GOLDEN_SHA256: [(&str, &str); 2] = [
    ("sparse_interactive", "958df02e276538a96c4732302a46cb36a53c2298ca7e1f4deb88a3748c67c661"),
    ("dense_scrutiny", "3f9905ad4c4bba688687714751abe965d18cea834701c32c379d81f573dcf682"),
];
/// The milestone's validated class counts (as retained_precision_carriers.rs
/// `CLASSES`): relative, absolute, input-derived, non-quantity. No validated
/// statement has a `not_covered` row.
const CLASSES: [(&str, [usize; 4]); 2] = [
    ("sparse_interactive", [25, 69, 3, 1]),
    ("dense_scrutiny", [25, 69, 3, 2]),
];
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
/// The builder's diagnostic mapping (`result.diagnostics.map(...)`).
fn desktop_diagnostic(x: &Value, model_ref: &Value) -> Value {
    let or = |v: &Value, fallback: Value| if v.is_null() { fallback } else { v.clone() };
    json!({
        "code": x["code"],
        "class": "ASSUMPTION_WARNING",
        "severity": if x["severity"] == "error" { json!("blocking") } else { x["severity"].clone() },
        "source": {"ref_type":"source","ref_id":or(&x["source"], json!("local_preview"))},
        "affected_object": {"ref_type":"preview_entity","ref_id":or(&x["affected_refs"][0], model_ref.clone())},
        "message": x["message"],
        "remediation": "Review source model and preview limitations.",
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
        .map(|x| desktop_diagnostic(x, &source["model_ref"]))
        .collect();
    json!({
        "schema_version": "0.2.0",
        "deliverable_id": "DEL-08-04",
        "package_id": "PKG-08",
        "scope_item": "SOW-046",
        "objectives": ["OBJ-007", "OBJ-009"],
        "export_format_status": {"baseline_format":"schema_first_json_result_envelope","additional_formats":"TBD","public_transport_protocol":"TBD","local_fea_package_format":"TBD","external_adapter_formats":"TBD"},
        "result_envelope": {
            "schema_version": "0.2.0",
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
    mode: &'static str,
    source: Value,
    model: Value,
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn milestones() -> Vec<Milestone> {
    PINNED
        .iter()
        .map(|(mode, text, file_sha, receipt_sha)| {
            assert_eq!(sha(text.as_bytes()), *file_sha, "{mode}: PP's pinned successor bytes");
            let doc: Value = serde_json::from_str(text).unwrap();
            assert_eq!(doc["source"]["retained_precision"]["receipt_sha256"], *receipt_sha);
            assert_eq!(doc["invocation"]["solver_mode"], *mode);
            assert_eq!(doc["source"]["producer"]["semantic_contract_id"], s::PREVIEW_PHYSICS_RETAINED_ID);
            assert!(
                doc["source"]["results"].as_array().unwrap().iter().all(|r| r.get("dimension").is_none()),
                "{mode}: a dimension-absent carrier"
            );
            Milestone {
                mode,
                source: doc["source"].clone(),
                model: doc["invocation"]["request"]["model"].clone(),
            }
        })
        .collect()
}
fn derive(source: &Value, model: &Value) -> Result<Value, String> {
    d::derive_document(desktop_base(source, model), model, source, desktop_origin(source, model), None)
}
fn golden_name(mode: &str) -> String {
    format!("retained_precision_successor_derivative_{mode}.json")
}
fn committed(mode: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/results")
        .join(golden_name(mode))
}
fn pinned_golden(mode: &str) -> &'static str {
    GOLDEN_SHA256.iter().find(|(m, _)| *m == mode).unwrap().1
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
    let out = std::env::var(OUT).ok().map(PathBuf::from);
    let inputs = std::env::var(INPUTS_OUT).ok().map(PathBuf::from);
    for m in milestones() {
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
            std::fs::write(dir.join(golden_name(m.mode)), &text).unwrap();
            eprintln!("{OUT}: wrote {} sha256 {live}", golden_name(m.mode));
            continue;
        }
        let file = std::fs::read(committed(m.mode))
            .unwrap_or_else(|e| panic!("{}: the committed golden {}: {e}", m.mode, golden_name(m.mode)));
        assert_eq!(sha(&file), pinned_golden(m.mode), "{}: the committed golden is the pinned one", m.mode);
        assert_eq!(live, pinned_golden(m.mode), "{}: the live derivative is the committed golden", m.mode);
    }
}

/// Controls on the golden: the desktop shape, the receipt whole,
/// `contract_evidence`, and exactly one D-U6-2 disclosure per `absolute_verified`
/// or `not_covered` row, with its code and message.
#[test]
fn t6s2_golden_carries_the_receipt_evidence_and_class_disclosures() {
    for m in milestones() {
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
        assert_eq!(e["semantic_contract_ref"], d::reference("semantic_contract", s::PREVIEW_PHYSICS_RETAINED_ID));
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
        assert_eq!(e["reproducibility"]["model_hash"], d::legacy_checksum(&m.model, d::reference("model_payload", m.source["model_ref"].as_str().unwrap())).unwrap());
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
        let expected = CLASSES.iter().find(|(mode, _)| *mode == m.mode).unwrap().1;
        assert_eq!(counts, [expected[0], expected[1], expected[2], expected[3], 0], "{}", m.mode);
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
        assert_eq!(classed, expected[1], "{}: one disclosure per absolute_verified or not_covered row", m.mode);
        assert_eq!(disclosures.iter().filter(claimed).count(), classed, "{}", m.mode);
    }
}

/// Controls on the pin: an edited pinned successor changes the golden's hash.
/// A hash-consistent edit (one diagnostic's message, resealed) is still derived,
/// to a document with another sha256; an unresealed edit is refused.
#[test]
fn t6s2_a_mutated_pinned_successor_changes_the_golden() {
    for m in milestones() {
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
