//! RV88 (U6a review): extra probes, candidate lane only. Records to $RV88_EXTRA_OUT.
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_result_export::{derivative as d, retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};
use std::fmt::Write as _;
use std::time::Instant;

const SPARSE: &str = include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json");

fn derive(raw: &Value) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap();
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv88"))?;
    let origin = json!({"origin_id":"rv88","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV88","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base: Value = serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap();
    base["result_envelope"]["envelope_id"] = json!("envelope:rv88");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
fn rv88_extra() {
    let mut log = String::new();
    let doc: Value = serde_json::from_str(SPARSE).unwrap();
    let (src, inv) = (doc["source"].clone(), doc["invocation"].clone());
    let der = derive(&src).unwrap();
    // (1) The headless runner's derivative-metadata check (runner lib.rs:617-621 pattern).
    let mut meta = der["result_envelope"].clone();
    meta["schema_version"] = json!("0.2.0");
    let r = s::for_source_metadata(&meta).map(|(t, v)| (t["semantic_contract_id"].clone(), v));
    let bound = meta["semantic_contract_ref"] == json!({"ref_type":"semantic_contract","ref_id":meta["producer"]["semantic_contract_id"]});
    writeln!(log, "runner_metadata_check\t{r:?}\tref_bound={bound}").unwrap();
    // (2) Canonical round trip of the derivative, then validate against the raw source.
    let rt: Value = serde_json::from_str(&canonical_json(&der)).unwrap();
    writeln!(log, "canonical_roundtrip_validate\t{:?}", d::validate_document(&rt, &src)).unwrap();
    // (3) An integral-float receipt member (D25/D32: the same number), hash-consistent.
    let mut f = src.clone();
    f["retained_precision"]["body"]["receipt_version"] = json!(1.0);
    writeln!(log, "integral_float_reader\t{:?}", rp::validate(&f, Some(&inv)).map(|v| v.publication_sha256)).unwrap();
    match derive(&f) {
        Ok(fd) => {
            writeln!(log, "integral_float_derive\tok\tvalidate_in_memory={:?}", d::validate_document(&fd, &f)).unwrap();
            let frt: Value = serde_json::from_str(&canonical_json(&fd)).unwrap();
            writeln!(log, "integral_float_roundtrip_validate\t{:?}", d::validate_document(&frt, &f)).unwrap();
        }
        Err(e) => writeln!(log, "integral_float_derive\terr:{e}").unwrap(),
    }
    // (4) Cost (debug build): one validation, and binding every row.
    let t = Instant::now();
    for _ in 0..3 {
        rp::validate(&src, None).unwrap();
    }
    writeln!(log, "validate_ms_debug\t{}", t.elapsed().as_millis() / 3).unwrap();
    let rows = src["results"].as_array().unwrap();
    let t = Instant::now();
    let refused = rows.iter().filter(|r| s::rule_binding_refusal(&src, r).is_some()).count();
    writeln!(log, "binding_all_rows_ms_debug\t{}\trows={}\trefused={refused}", t.elapsed().as_millis(), rows.len()).unwrap();
    let t = Instant::now();
    let _ = derive(&src).unwrap();
    writeln!(log, "derive_ms_debug\t{}", t.elapsed().as_millis()).unwrap();
    // (5) Headline rows of the milestone and their classes.
    for key in ["max_displacement", "max_open_formula_stress", "max_support_reaction"] {
        let id = &src["summary"][key]["result_ref"];
        if let Some(id) = id.as_str() {
            let row = rows.iter().find(|r| r["id"] == id).unwrap();
            writeln!(log, "headline\t{key}\t{id}\t{:?}", s::rule_binding_refusal(&src, row)).unwrap();
        }
    }
    // (6) The classification summary (compared with the Python reader's class counts).
    writeln!(log, "summary_with_invocation\t{}", json!(s::classification_summary(&src, Some(&inv)))).unwrap();
    writeln!(log, "summary_without_invocation\t{}", json!(s::classification_summary(&src, None))).unwrap();
    let dense: Value = serde_json::from_str(include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")).unwrap();
    writeln!(log, "summary_dense_with_invocation\t{}", json!(s::classification_summary(&dense["source"], Some(&dense["invocation"])))).unwrap();
    if let Ok(p) = std::env::var("RV88_EXTRA_OUT") {
        std::fs::write(p, &log).unwrap();
    }
}
