//! I68 U8-0 probe (disposable archive copy only; never committed). Every run goes through
//! the actual Direct entry `run_linear_static_preview_value_with_retained_direct`, counted
//! by grant 2's tally (`hooks::counted`). Inputs are derived here from the milestone
//! request and from W6's input (written inline: `w6_input()` is `pub(super)` in I65's file).
//! Probe-only prints in the archive's production files: `I68_NATIVE_OUTCOME`
//! (retained_product.rs `solve_native`), `I68_PREPARATION_FAILURE` and
//! `I68_CANDIDATE_REFUSAL` (lib.rs `retained_w1`). Run single-threaded with --nocapture.
use super::retained_tests_hooks::{self as hooks, Counts};
use super::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const REGISTERED_IDENTITY: &str = "v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0";
const ONE_RUN_THROUGH_G_C: Counts = Counts { runs: 1, complete_gates: 1 };

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn registered() -> bool {
    option_env!("OPS_RETAINED_BUILD_IDENTITY") == Some(REGISTERED_IDENTITY)
}
fn milestone() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
fn plain(mode: PreviewSolverMode, raw: &Value) -> Vec<u8> {
    serde_json::to_vec(&run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap()
}
/// R-2's notice bytes, as the facade tests pin them (retained_facade_tests.rs:41-53).
fn notice_json(case: &str, detail: Option<&str>) -> String {
    let reason = detail.map_or(String::new(), |d| format!(" Reason: receipt_encoding; detail: {d}."));
    format!(r#"{{"id":"diagnostic:retained-precision:{case}:unavailable","code":"RETAINED_PRECISION_UNAVAILABLE","severity":"info","message":"Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.{reason}","source":"core/product_physics","affected_refs":["{case}"]}}"#)
}
fn with_notice(plain: &[u8], case: &str, detail: Option<&str>) -> Vec<u8> {
    let text = std::str::from_utf8(plain).unwrap();
    let (head, tail) = text.split_once(r#""diagnostics":["#).unwrap();
    let close = tail.find("],\"professional_boundary\"").unwrap();
    let (items, rest) = tail.split_at(close);
    let sep = if items.is_empty() { "" } else { "," };
    format!(r#"{head}"diagnostics":[{items}{sep}{}{rest}"#, notice_json(case, detail)).into_bytes()
}
fn notices(bytes: &[u8]) -> usize {
    let value: Value = serde_json::from_slice(bytes).unwrap();
    value["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == "RETAINED_PRECISION_UNAVAILABLE").count()
}
fn out_dir() -> Option<std::path::PathBuf> {
    std::env::var("I68_OUT").ok().map(std::path::PathBuf::from)
}

/// W6's input (retained_memory_witness_tests.rs:181-199), the same JSON content.
fn w6_input() -> Value {
    let (a, b) = ("node:section-a", "node:section-b");
    let tip = f64::from_bits(0x0031fa182c40c60d);
    let p = "invented_t3_g5_witness_input_no_library_data";
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:section-oracle", "units": {"length": "m", "force": "N"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "nodes": [{"id": a, "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": p},
                  {"id": b, "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": p}],
        "pipe_segments": [{"id": "pipe:source-section", "from": a, "to": b, "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": p}],
        "materials": [{"id": "material:section", "elastic_modulus": {"value": 1.0, "unit": "Pa"}, "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": p}],
        "supports": [{"id": "support:section-a", "node": a, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}],
        "load_cases": [{"id": "case:source-section", "provenance": p, "primitive_loads": [
            {"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": b}, "direction": "global_y", "dimension": "force", "magnitude": {"value": tip, "unit": "N"}, "provenance": p},
            {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": b}, "direction": "rotation_x", "dimension": "moment", "magnitude": {"value": tip, "unit": "N*m"}, "provenance": p}]}],
        "combinations": []}, "materials": []})
}

/// Item 1: RV93's two real inputs (zz_rv93.rs:292-315).
fn first_load_only() -> Value {
    let mut v = milestone();
    let l = v["model"]["load_cases"][0]["primitive_loads"][0].clone();
    v["model"]["load_cases"][0]["primitive_loads"] = json!([l]);
    v
}
fn tiny_spring() -> Value {
    let mut v = milestone();
    v["model"]["supports"][1]["stiffness"]["value"]["value"] = json!(1e-300);
    v
}
/// Item 3: the milestone plus one node no member references, with one rigid support on it
/// restraining all six DOFs (the milestone's rigid-support shape: no family).
fn l0_isolated_node() -> Value {
    let mut v = milestone();
    let p = "invented_t3_p1_detection_input_no_library_data";
    v["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "N2", "position": {"x": 3.0, "y": 0.0, "z": 0.0}, "provenance": p}));
    v["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "rigid:N2", "node": "N2", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));
    v
}
/// Item 4, two-body pair: the milestone body plus PHYS-R4's cantilever (W6's body, moved to
/// x = 5..6 so no node coincides with the milestone's N0). Case A loads body 0 (the
/// milestone's three moments); case B loads body 1 (W6's tip force and torque).
fn two_body(case_b: bool) -> Value {
    let mut v = milestone();
    let w6 = w6_input();
    let m = &mut v["model"];
    for mut node in w6["model"]["nodes"].as_array().unwrap().iter().cloned() {
        let x = node["position"]["x"].as_f64().unwrap();
        node["position"]["x"] = json!(x + 5.0);
        m["nodes"].as_array_mut().unwrap().push(node);
    }
    m["pipe_segments"].as_array_mut().unwrap().push(w6["model"]["pipe_segments"][0].clone());
    m["materials"].as_array_mut().unwrap().push(w6["model"]["materials"][0].clone());
    m["supports"].as_array_mut().unwrap().push(w6["model"]["supports"][0].clone());
    if case_b {
        m["load_cases"][0]["primitive_loads"] = w6["model"]["load_cases"][0]["primitive_loads"].clone();
    }
    v
}
/// Item 4, fallback one-body pair: PHYS-R4's x-aligned cantilever (W6's model). Case A is
/// axial only (the tip force along global_x); case B transverse (the tip force along
/// global_y only). W6 itself is case B plus the tip torque.
fn one_body(case_b: bool) -> Value {
    let mut v = w6_input();
    let loads = v["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap();
    loads.truncate(1);
    if !case_b {
        loads[0]["id"] = json!("load:tip-x");
        loads[0]["direction"] = json!("global_x");
    }
    v
}

fn short(text: String, max: usize) -> String {
    if text.len() <= max {
        return text;
    }
    let mut end = max;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...[{} bytes]", &text[..end], text.len())
}

/// One counted Direct invocation, recorded.
fn probe(label: &str, raw: &Value, mode: PreviewSolverMode) -> Option<Value> {
    let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
    let plain_bytes = plain(mode, raw);
    let plain_value: Value = serde_json::from_slice(&plain_bytes).unwrap();
    let codes: Vec<String> = plain_value["diagnostics"].as_array().unwrap().iter()
        .map(|d| format!("{}:{}", d["code"].as_str().unwrap_or("?"), d["severity"].as_str().unwrap_or("?"))).collect();
    println!("I68_BEGIN {label} {} registered={} input_sha={}", mode.as_str(), registered(), sha(&serde_json::to_vec(raw).unwrap()));
    println!("I68_ORDINARY {label} {} status={} results={} plain_sha={} plain_len={} diagnostics={:?}", mode.as_str(),
        plain_value["status"], plain_value["results"].as_array().map_or(0, Vec::len), sha(&plain_bytes), plain_bytes.len(), codes);
    hooks::disarm();
    let armed_before = hooks::armed_names();
    let input = raw.clone();
    let (output, counts) = hooks::counted(move || run_linear_static_preview_value_with_retained_direct(input, mode).unwrap());
    let armed_after = hooks::armed_names();
    let report = output.admission().copied();
    if let Some(r) = &report {
        let law = r.law();
        println!("I68_ADMISSION {label} {} profile={:?} allowance={:?} refusal={:?} refusal_clause={:?} domain={:?} required={:?} census_complete={}",
            mode.as_str(), r.profile, r.allowance, law.refusal, law.refusal.and_then(|x| x.clause()), law.domain, law.required, r.census_complete());
        println!("I68_ADMISSION_FACTS {label} {} typed={:?} raw={:?}", mode.as_str(), r.typed, r.raw);
    } else {
        println!("I68_ADMISSION {label} {} report=None", mode.as_str());
    }
    let w1 = match output.retained() {
        None => "None (no W1)".to_owned(),
        Some(Ok(_)) => "Ok(successor)".to_owned(),
        Some(Err(cause)) => format!("Err({cause:?})"),
    };
    let envelope_is_plain = serde_json::to_vec(output.envelope()).unwrap() == plain_bytes;
    let mut successor = None;
    let bytes = match output.into_publication() {
        RetainedPublication::Successor(value) => {
            let b = serde_json::to_vec(&value).unwrap();
            successor = Some(value);
            b
        }
        RetainedPublication::Ordinary(envelope) => serde_json::to_vec(&envelope).unwrap(),
    };
    let n = notices(&bytes);
    let eq_notice = bytes == with_notice(&plain_bytes, &case, None);
    let eq_plain = bytes == plain_bytes;
    println!("I68_W1 {label} {} cause={w1} counts={counts:?} one_run_through_g_c={} armed_before={armed_before:?} armed_after={armed_after:?} envelope_is_plain={envelope_is_plain} published_successor={} notices={n} bytes_eq_with_notice_plain_case_none={eq_notice} bytes_eq_plain={eq_plain} published_sha={} published_len={}",
        mode.as_str(), counts == ONE_RUN_THROUGH_G_C, successor.is_some(), sha(&bytes), bytes.len());
    successor.map(|value| {
        let doc = json!({"id": format!("i68_{label}_{}", mode.as_str()), "source": value, "invocation": {"request": raw, "solver_mode": mode.as_str()}});
        let text = serde_json::to_string_pretty(&doc).unwrap();
        let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
        let rust = match open_pipe_stress_result_export::retained_precision::validate(&value, Some(&invocation)) {
            Ok(v) => format!("PASS invocation_bound={} numerical_eligible={} classifications={} publication_sha256={}", v.invocation_bound, v.numerical_eligible, v.classifications.len(), v.publication_sha256),
            Err(e) => format!("FAIL {}", short(format!("{e:?}"), 600)),
        };
        println!("I68_SUCCESSOR {label} {} doc_sha={} doc_len={} receipt_sha256={} rust_reader={rust}", mode.as_str(), sha(text.as_bytes()), text.len(),
            value["retained_precision"]["receipt_sha256"]);
        if let Some(dir) = out_dir() {
            std::fs::write(dir.join(format!("{label}_{}.json", mode.as_str())), &text).unwrap();
        }
        value
    })
}

#[test]
fn zz_i68_item1_real_input_fallbacks() {
    for mode in MODES {
        for (label, raw) in [("first_load_only", first_load_only()), ("tiny_spring", tiny_spring())] {
            probe(label, &raw, mode);
        }
    }
}

#[test]
fn zz_i68_item2_w6() {
    for mode in MODES {
        probe("w6", &w6_input(), mode);
    }
}

#[test]
fn zz_i68_item3_l0() {
    for mode in MODES {
        probe("l0_isolated_node", &l0_isolated_node(), mode);
    }
}

#[test]
fn zz_i68_item4_pairs() {
    for mode in MODES {
        for (label, raw) in [("two_body_a", two_body(false)), ("two_body_b", two_body(true)), ("one_body_a_axial", one_body(false)), ("one_body_b_transverse", one_body(true))] {
            probe(label, &raw, mode);
        }
    }
}

/// Control: the milestone through the same probe (its pinned successor).
#[test]
fn zz_i68_control_milestone() {
    for mode in MODES {
        probe("milestone", &milestone(), mode);
    }
}

/// The dense two-body case A successor the precommit refused (dumped by the probe-only
/// precommit print): the Rust reader again, directly, on the same bytes and invocation.
#[test]
fn zz_i68_reader_on_dump() {
    let Ok(path) = std::env::var("I68_DUMP") else { println!("I68_DUMP unset"); return };
    let doc: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    let r = open_pipe_stress_result_export::retained_precision::validate(&doc["source"], Some(&doc["invocation"]));
    println!("I68_READER_DUMP bound {}", match &r { Ok(v) => format!("PASS eligible={} classifications={}", v.numerical_eligible, v.classifications.len()), Err(e) => format!("FAIL {e:?}") });
    let r = open_pipe_stress_result_export::retained_precision::validate(&doc["source"], None);
    println!("I68_READER_DUMP unbound {}", match &r { Ok(v) => format!("PASS eligible={} classifications={}", v.numerical_eligible, v.classifications.len()), Err(e) => format!("FAIL {e:?}") });
}
