//! I81 B1-0 probe (disposable archive copy only; never committed). Declared in
//! retained_memory.rs beside `law_tests` and `witness_tests`, so it can reuse their committed
//! inputs (`cap_maximal`, `attempted_examples`, `w6_input`). Every probed run goes through the
//! actual Direct entry `run_linear_static_preview_value_with_retained_direct`, counted by grant 2's
//! tally (`hooks::counted`), following I68's U8-0 method (R/I68/u8_probe_01).
//! Probe-only prints in the archive's production files (all `cfg(test)`, no control-flow change):
//! `I81_SEEDS` (lib.rs `permitted_run`, after the ordinary run; `retained_w1`, at entry),
//! `I81_W1_START` (`retained_w1`, after the notice reservation: W1 work starts),
//! `I81_PREPARATION_FAILURE`, `I81_CANDIDATE_REFUSAL`, `I81_PRECOMMIT_ERROR`/`_DUMP` (`retained_w1`),
//! `I81_NATIVE_OUTCOME` (retained_product.rs `solve_native`). Run single-threaded with --nocapture.
use crate::retained_product::{InitialSeed, OrdinarySeed, W2Seed, LegacySeed, ProductCapture};
use crate::retained_tests_hooks::{self as hooks, Counts};
use crate::{MechanicsEnvelope, PreviewSolverMode, RetainedPublication, StructuralError};
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

// ---- the seed print (called from the production prints) ------------------------------

fn tag(error: &StructuralError) -> &'static str {
    match error {
        StructuralError::InvalidInput(_) => "invalid_input",
        StructuralError::Range(_) => "range",
        StructuralError::Asymmetric { .. } => "asymmetric",
        StructuralError::NumericallyUnresolved { .. } => "numerically_unresolved",
        StructuralError::NegativeEnergy { .. } => "negative_energy",
        StructuralError::Mechanism { .. } => "mechanism",
    }
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
fn seed_json(s: &OrdinarySeed) -> Value {
    let initial = match &s.initial {
        None => json!({"kind": "none (not attempted)"}),
        Some(InitialSeed::Report { code, report_diagnostic_ref }) => json!({"kind": "report", "outcome": code, "ref": report_diagnostic_ref}),
        Some(InitialSeed::StructuralFailure { error, diagnostic_ref }) => {
            json!({"kind": "structural_failure", "tag": tag(error), "error": short(format!("{error:?}"), 300), "diagnostic_ref": diagnostic_ref})
        }
        Some(InitialSeed::FormationFailure { error }) => json!({"kind": "formation_failure", "error": short(format!("{error:?}"), 300)}),
    };
    let w2 = match &s.w2 {
        W2Seed::NotTriggered => json!({"kind": "not_triggered"}),
        W2Seed::Published { trigger, force_scale_exponent, report_diagnostic_ref } => json!({"kind": "published",
            "trigger": short(format!("{trigger:?}"), 200), "force_scale_exponent": force_scale_exponent, "report_ref": report_diagnostic_ref}),
        W2Seed::Failed { trigger, failure, diagnostic_ref } => json!({"kind": "failed",
            "trigger": short(format!("{trigger:?}"), 200), "failure": short(format!("{failure:?}"), 200), "diagnostic_ref": diagnostic_ref}),
    };
    let legacy = match &s.legacy {
        None => "none".to_owned(),
        Some(LegacySeed::NotEligible) => "not_eligible".to_owned(),
        Some(LegacySeed::NotRequired) => "not_required".to_owned(),
        Some(LegacySeed::DeclinedWithoutAttempt { .. }) => "declined_without_attempt".to_owned(),
        Some(LegacySeed::Unavailable { work, .. }) => format!("unavailable(stage={})", work.stage),
        Some(LegacySeed::ExactSelected) => "exact_selected".to_owned(),
    };
    json!({"case": s.case, "initial": initial, "w2": w2, "legacy": legacy, "recovery_demoted": s.recovery_demoted,
        "load_row_finding": s.load_row_finding.is_some(), "d5_ref": s.d5_diagnostic_ref})
}
/// The published verdicts (`numerical_quality.cases[i].solve_quality`) and the seeds, at `site`.
pub(crate) fn seeds(site: &str, observer: &ProductCapture, ordinary: &MechanicsEnvelope) {
    let env = serde_json::to_value(ordinary).unwrap();
    let quality: Vec<Value> = env["numerical_quality"]["cases"].as_array().map(|a| a.iter()
        .map(|c| json!({"ref": c["basis_ref"]["ref_id"], "solve_quality": c["solve_quality"]})).collect()).unwrap_or_default();
    let seeds: Vec<Value> = observer.ordinary.iter().map(seed_json).collect();
    println!("I81_SEEDS site={site} mechanics={} exact_selected={} quality={} seeds={}", env["status"]["mechanics"],
        ordinary.source_block_recovery.is_some(), json!(quality), json!(seeds));
}

// ---- inputs ------------------------------------------------------------------------

/// witness_tests' private helpers, copied (retained_memory_witness_tests.rs:52–73).
fn escape_every_provenance(v: &mut Value) {
    match v {
        Value::Object(m) => {
            for (k, x) in m.iter_mut() {
                match x {
                    Value::String(s) if k == "provenance" => s.push_str(" q\"b\\"),
                    _ => escape_every_provenance(x),
                }
            }
        }
        Value::Array(a) => a.iter_mut().for_each(escape_every_provenance),
        _ => {}
    }
}
fn depth_16_value() -> Value {
    let mut deep = json!(1);
    for _ in 0..14 {
        deep = json!([deep]);
    }
    deep
}
/// QUAL §4 W2's input (witness_w2_cap_maximal).
fn w2_input() -> Value {
    let mut raw = super::law_tests::cap_maximal();
    escape_every_provenance(&mut raw);
    raw["model"]["unknown_depth_witness"] = depth_16_value();
    raw
}
/// QUAL §4 W2-deep's input (witness_w2_deep_milestone_publishes).
fn w2_deep_input() -> Value {
    let mut raw = milestone();
    escape_every_provenance(&mut raw);
    raw["model"]["deep_input_witness"] = depth_16_value();
    raw
}
/// QUAL §4 W2b's input (witness_w2b_cap_maximal_solvable).
fn w2b_input() -> Value {
    let mut raw = super::law_tests::cap_maximal();
    let supports = raw["model"]["supports"].as_array_mut().unwrap();
    for (i, support) in supports.iter_mut().enumerate() {
        if i == 0 {
            support.as_object_mut().unwrap().remove("stiffness");
        } else {
            support["family"] = json!("spring");
            support["restraints"] = json!(["UY"]);
        }
    }
    raw
}
/// QUAL §4 W3's inputs, one per mode (witness_w3_exact_selected).
fn w3_input(mode: PreviewSolverMode) -> Value {
    serde_json::from_str(match mode {
        PreviewSolverMode::SparseInteractive => include_str!("../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json"),
        PreviewSolverMode::DenseScrutiny => include_str!("../../../fixtures/product_preview/source_blocks/n05-dense_scrutiny.request.json"),
    }).unwrap()
}
/// U8's committed inputs (retained_facade_tests.rs:837–894), copied; their sha256 is printed
/// and compared with I68's/RV105's recorded values.
fn u8_first_load_only() -> Value {
    let mut raw = milestone();
    let first = raw["model"]["load_cases"][0]["primitive_loads"][0].clone();
    raw["model"]["load_cases"][0]["primitive_loads"] = json!([first]);
    raw
}
fn u8_two_body_case_a() -> Value {
    let mut raw = milestone();
    let p = "invented_t3_g5_witness_input_no_library_data";
    let model = &mut raw["model"];
    model["nodes"].as_array_mut().unwrap().extend([
        json!({"id": "node:section-a", "position": {"x": 5.0, "y": 0.0, "z": 0.0}, "provenance": p}),
        json!({"id": "node:section-b", "position": {"x": 6.0, "y": 0.0, "z": 0.0}, "provenance": p}),
    ]);
    model["pipe_segments"].as_array_mut().unwrap().push(json!({"id": "pipe:source-section", "from": "node:section-a", "to": "node:section-b",
        "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
        "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": p}));
    model["materials"].as_array_mut().unwrap().push(json!({"id": "material:section", "elastic_modulus": {"value": 1.0, "unit": "Pa"},
        "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": p}));
    model["supports"].as_array_mut().unwrap().push(json!({"id": "support:section-a", "node": "node:section-a", "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));
    raw
}
fn tip_loads() -> Value {
    let (tip, p) = (f64::from_bits(0x0031fa182c40c60d), "invented_t3_g5_witness_input_no_library_data");
    json!([
        {"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": "node:section-b"}, "direction": "global_y",
            "dimension": "force", "magnitude": {"value": tip, "unit": "N"}, "provenance": p},
        {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": "node:section-b"}, "direction": "rotation_x",
            "dimension": "moment", "magnitude": {"value": tip, "unit": "N*m"}, "provenance": p}])
}
fn u8_two_body_case_b() -> Value {
    let mut raw = u8_two_body_case_a();
    raw["model"]["load_cases"][0]["primitive_loads"] = tip_loads();
    raw
}
fn u8_l0_isolated_node() -> Value {
    let mut raw = milestone();
    let p = "invented_t3_p1_detection_input_no_library_data";
    raw["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "N2", "position": {"x": 3.0, "y": 0.0, "z": 0.0}, "provenance": p}));
    raw["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "rigid:N2", "node": "N2", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));
    raw
}
/// W-C2's case C (DESIGN_v2 §1.4): case A's loads (the milestone's three moments on body 0)
/// plus case B's loads (W6's tip force and torque on body 1), as one load case.
fn case_c() -> Value {
    let mut raw = u8_two_body_case_a();
    let loads = raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap();
    loads.extend(tip_loads().as_array().unwrap().iter().cloned());
    raw
}

// ---- one counted Direct invocation, recorded (I68's `probe`) ----------------------------

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
fn probe(label: &str, raw: &Value, mode: PreviewSolverMode) {
    let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap_or("?").to_owned();
    let plain_bytes = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
    let plain_value: Value = serde_json::from_slice(&plain_bytes).unwrap();
    let codes: Vec<String> = plain_value["diagnostics"].as_array().unwrap().iter()
        .map(|d| format!("{}:{}", d["code"].as_str().unwrap_or("?"), d["severity"].as_str().unwrap_or("?"))).collect();
    let range: Vec<String> = plain_value["diagnostics"].as_array().unwrap().iter()
        .filter_map(|d| d["message"].as_str()).filter_map(|m| m.find("force_scale_exponent=").map(|i| m[i..].split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '=' || c == '-')).next().unwrap_or("").to_owned()))
        .collect();
    let verdicts: Vec<Value> = plain_value["numerical_quality"]["cases"].as_array().map(|a| a.iter()
        .map(|c| json!({"ref": c["basis_ref"]["ref_id"], "solve_quality": c["solve_quality"]})).collect()).unwrap_or_default();
    println!("I81_BEGIN {label} {} registered={} input_sha={}", mode.as_str(), registered(), sha(&serde_json::to_vec(raw).unwrap()));
    println!("I81_ORDINARY {label} {} status={} results={} plain_sha={} plain_len={} published_verdicts={} range_scaling={range:?} diagnostics={codes:?}", mode.as_str(),
        plain_value["status"], plain_value["results"].as_array().map_or(0, Vec::len), sha(&plain_bytes), plain_bytes.len(), json!(verdicts));
    hooks::disarm();
    let armed_before = hooks::armed_names();
    let input = raw.clone();
    let (result, counts) = hooks::counted(move || crate::run_linear_static_preview_value_with_retained_direct(input, mode));
    let output = match result {
        Ok(output) => output,
        Err(error) => {
            println!("I81_W1 {label} {} direct=Err({error}) counts={counts:?}", mode.as_str());
            return;
        }
    };
    let armed_after = hooks::armed_names();
    if let Some(r) = output.admission() {
        let law = r.law();
        println!("I81_ADMISSION {label} {} profile={:?} allowance={:?} refusal={:?} domain={:?} required={:?} census_complete={}",
            mode.as_str(), r.profile, r.allowance, law.refusal, law.domain, law.required, r.census_complete());
    } else {
        println!("I81_ADMISSION {label} {} report=None", mode.as_str());
    }
    let w1 = match output.retained() {
        None => "None (no permit, no W1)".to_owned(),
        Some(Ok(_)) => "Ok(successor)".to_owned(),
        Some(Err(cause)) => format!("Err({cause:?})"),
    };
    let envelope_is_plain = serde_json::to_vec(output.envelope()).unwrap() == plain_bytes;
    let (bytes, successor) = match output.into_publication() {
        RetainedPublication::Successor(value) => (serde_json::to_vec(&value).unwrap(), Some(value)),
        RetainedPublication::Ordinary(envelope) => (serde_json::to_vec(&envelope).unwrap(), None),
    };
    let n = notices(&bytes);
    let eq_notice = bytes == with_notice(&plain_bytes, &case, None);
    let eq_plain = bytes == plain_bytes;
    println!("I81_W1 {label} {} cause={w1} counts={counts:?} one_run_through_g_c={} armed_before={armed_before:?} armed_after={armed_after:?} envelope_is_plain={envelope_is_plain} published_successor={} notices={n} bytes_eq_with_notice_plain_case_none={eq_notice} bytes_eq_plain={eq_plain} published_sha={} published_len={}",
        mode.as_str(), counts == ONE_RUN_THROUGH_G_C, successor.is_some(), sha(&bytes), bytes.len());
    if let Some(value) = successor {
        let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
        let rust = match open_pipe_stress_result_export::retained_precision::validate(&value, Some(&invocation)) {
            Ok(v) => format!("PASS invocation_bound={} numerical_eligible={} classifications={}", v.invocation_bound, v.numerical_eligible, v.classifications.len()),
            Err(e) => format!("FAIL {}", short(format!("{e:?}"), 600)),
        };
        let body = &value["retained_precision"]["body"];
        println!("I81_SUCCESSOR {label} {} receipt_sha256={} rust_reader={rust} case_status={} ordinary_initial={} ordinary_w2={}", mode.as_str(),
            value["retained_precision"]["receipt_sha256"], body["cases"][0]["status"], body["ordinary_attempts"][0]["initial"], body["ordinary_attempts"][0]["w2"]);
    }
}

// ---- the probe's tests ----------------------------------------------------------------

/// Item 1: QUAL §4's witness inputs through the Direct entry (W1, W4, W7 and headroom are the
/// milestone; W5 is the dense halves; W3 runs each fixture in its own mode, as the witness does).
#[test]
fn zz_i81_item1_qual4_witness_inputs() {
    for mode in MODES {
        probe("milestone (W1, W4, W7, headroom; attempted 'milestone')", &milestone(), mode);
        probe("W2 cap_maximal", &w2_input(), mode);
        probe("W2-deep", &w2_deep_input(), mode);
        probe("W2b cap_maximal_solvable", &w2b_input(), mode);
        probe("W3 n05", &w3_input(mode), mode);
        probe("W6 force_scaled", &super::witness_tests::w6_input(), mode);
    }
}

/// Item 1: `attempted_examples()` (retained_memory_law_tests.rs), every input in both modes, as
/// `registered_g_c_declines_only_unattempted_solves` runs them.
#[test]
fn zz_i81_item1_attempted_examples() {
    for mode in MODES {
        for (label, raw) in super::law_tests::attempted_examples() {
            probe(&format!("attempted: {label}"), &raw, mode);
        }
    }
}

/// Context (U8's committed witnesses, named in DESIGN_v2 §1.3 and the brief's item 3).
#[test]
fn zz_i81_context_u8_inputs() {
    for mode in MODES {
        probe("u8 first_load_only", &u8_first_load_only(), mode);
        probe("u8 two_body_case_a", &u8_two_body_case_a(), mode);
        probe("u8 two_body_case_b (W-C1)", &u8_two_body_case_b(), mode);
        probe("u8 l0_isolated_node", &u8_l0_isolated_node(), mode);
    }
}

/// Item 3: W-C2's case C (A's loads plus B's loads), one case, both modes.
#[test]
fn zz_i81_item3_case_c() {
    for mode in MODES {
        probe("case_c", &case_c(), mode);
    }
}

/// Item 4 (re-basing): case C on the W6 stack witness's own path. `permitted_work` and `witness`
/// are copied from retained_memory_witness_tests.rs:32–50 and :74–78 (the private driver at
/// R/k = 4 MiB; no permit, G-B and G-C not consulted, as the committed witness runs).
fn permitted_work(raw: Value, mode: PreviewSolverMode) -> String {
    let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw, mode).expect("a valid request");
    let mut observer = ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    if ordinary.source_block_recovery.is_some() {
        return "ExactSelected".into();
    }
    match crate::retained_w1(observer, ordinary, &capture).1 {
        Ok(_) => "Successor".into(),
        Err(fallback) => format!("Fallback({fallback:?})"),
    }
}
#[test]
fn zz_i81_item4_case_c_on_the_w6_witness_path() {
    let stack = super::RESERVED_STACK_BYTES / super::STACK_WITNESS_DIVISOR;
    let raw = case_c();
    let plain = crate::run_linear_static_preview_value_with_mode(raw.clone(), PreviewSolverMode::SparseInteractive).unwrap();
    let scaled = plain.diagnostics.iter().any(|d| d.message.contains("range_scaling: force_scale_exponent=") && !d.message.contains("force_scale_exponent=none"));
    println!("I81_WITNESS_INPUT case_c force_scaled={scaled}");
    for mode in MODES {
        let raw = raw.clone();
        let ran = crate::on_reserved_stack(stack, crate::carry_test_hooks(move || permitted_work(raw, mode))).expect("the reserved thread spawned");
        println!("I81_WITNESS case_c {mode:?} stack={stack} ran={ran}");
    }
}
