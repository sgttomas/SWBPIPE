//! I98 B2-W probe (disposable archive copy only; never committed). I86's B1-SW probe module
//! (R/I86/b1_w_probe_01/_run_records/zz_i86_probe.rs) renamed I86 -> I98, declared in
//! retained_memory.rs beside `law_tests` and `witness_tests` as I81's and I86's were, plus:
//! - `bypass`: W-CB1's probe-only path (RV114 N-7). The Direct path's permitted run with G-A
//!   and G-B bypassed: G-A's census and refusal are recorded (the over-cap fact), G-B's
//!   refusal is recorded from a separate observed run with a probe-built permit, then the
//!   permitted run proper runs with an observer that holds no permit (G-B not consulted), G-C
//!   is evaluated with a probe-built permit and recorded, and W1 runs on the reserved stack R
//!   whatever G-C says;
//! - `I98_DUMP_PLAIN`: the plain envelope written to `I98_OUT` (for the R-7 row count);
//! - `zz_i98_dump_builtin`: U8's committed helpers (retained_facade_tests.rs), copied verbatim
//!   as I81 copied them, written to files, with I81's case C.
//! Every probed run goes through the actual Direct entry
//! `run_linear_static_preview_value_with_retained_direct`, counted by grant 2's tally, and
//! through a copy of the S1 witnesses' private driver at R/16 = 4 MiB (the "witness twin").
//! Every test is `#[ignore]` and run explicitly, single-threaded, with --nocapture.
use crate::retained_product::{InitialSeed, LegacySeed, OrdinarySeed, ProductCapture, W2Seed};
use crate::retained_tests_hooks::{self as hooks, Counts};
use crate::{MechanicsEnvelope, PreviewSolverMode, RetainedPublication, StructuralError};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

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
fn raw() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
fn quiet() -> bool {
    std::env::var_os("I98_QUIET").is_some()
}
fn now_us() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
}
/// A W1 phase boundary (called from the production prints).
pub(crate) fn mark(phase: &str) {
    println!("I98_MARK {phase} t_us={}", now_us());
}

// ---- the seed print (called from the production prints; I81's) -----------------------

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
/// The published verdicts and the seeds, at `site` (skipped under `I98_QUIET`).
pub(crate) fn seeds(site: &str, observer: &ProductCapture, ordinary: &MechanicsEnvelope) {
    if quiet() {
        return;
    }
    let env = serde_json::to_value(ordinary).unwrap();
    let quality: Vec<Value> = env["numerical_quality"]["cases"].as_array().map(|a| a.iter()
        .map(|c| json!({"ref": c["basis_ref"]["ref_id"], "solve_quality": c["solve_quality"]})).collect()).unwrap_or_default();
    let seeds: Vec<Value> = observer.ordinary.iter().map(seed_json).collect();
    println!("I98_SEEDS site={site} mechanics={} exact_selected={} quality={} seeds={}", env["status"]["mechanics"],
        ordinary.source_block_recovery.is_some(), json!(quality), json!(seeds));
}

// ---- U8's committed inputs (retained_facade_tests.rs:837–894, copied verbatim as I81 did) --

fn u8_two_body_case_a() -> Value {
    let mut raw = raw();
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
    let mut raw = raw();
    let p = "invented_t3_p1_detection_input_no_library_data";
    raw["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "N2", "position": {"x": 3.0, "y": 0.0, "z": 0.0}, "provenance": p}));
    raw["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "rigid:N2", "node": "N2", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));
    raw
}
/// W-C2's case C (I81's `case_c`): case A's loads plus case B's loads, as one load case.
fn case_c() -> Value {
    let mut raw = u8_two_body_case_a();
    let loads = raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap();
    loads.extend(tip_loads().as_array().unwrap().iter().cloned());
    raw
}

// ---- the census (D1.2–D1.11 rows), the ordinary route, Direct, and the witness twin -----

fn mode_name(mode: PreviewSolverMode) -> &'static str {
    mode.as_str()
}
/// D1's census over a request (I86's): the domain clause, the refusal, the required bytes,
/// and the cap rows that matter.
fn census(label: &str, raw: &Value) {
    use super::CapFact as K;
    let (request, capture) = match crate::source_receipt::CapturedInvocation::parse(raw.clone(), PreviewSolverMode::SparseInteractive) {
        Ok(x) => x,
        Err(e) => {
            println!("I98_CENSUS {label} parse=Err({})", short(format!("{e:?}"), 400));
            return;
        }
    };
    let report = super::assess(&capture, &request, super::Entry::Direct);
    let law = report.law();
    let facts = super::DomainFacts {
        raw: &report.raw,
        raw_text: &law.raw_text,
        typed: &report.typed,
        nested: &law.nested,
        headless: None,
        digest: &report.captured_digest,
    };
    let rows = super::cap_rows(&facts);
    let over: Vec<String> = rows.iter().filter(|r| r.observed > r.cap).map(|r| format!("{:?}={}/{}", r.fact, r.observed, r.cap)).collect();
    let at_cap: Vec<String> = rows.iter().filter(|r| r.cap > 0 && r.observed == r.cap).map(|r| format!("{:?}={}/{}", r.fact, r.observed, r.cap)).collect();
    let keys = [
        K::Nodes, K::NodesCapacity, K::Members, K::MembersCapacity, K::Supports, K::SupportsCapacity, K::Restraints, K::Springs, K::Loads,
        K::LoadsCapacity, K::ModelMaterials, K::RequestMaterials, K::TemperaturePoints, K::LoadCasesCapacity, K::TypedTextBytes, K::RawTextBytes,
        K::RawKeyTextBytes, K::RawValues, K::RawDepth, K::RawStringBytes, K::RawKeyBytes, K::RawArrayCapacity, K::RawStringCapacity,
        K::RawKeyCapacity, K::DigestCapacity, K::ControlBytes,
    ];
    let key_rows: Vec<String> = keys.iter().map(|k| {
        let r = rows.iter().find(|r| r.fact == *k).unwrap();
        format!("{:?}={}/{}", r.fact, r.observed, r.cap)
    }).collect();
    let cases = raw["model"]["load_cases"].as_array().map(|a| a.iter().map(|c| c["primitive_loads"].as_array().map_or(0, Vec::len)).collect::<Vec<_>>()).unwrap_or_default();
    let total: usize = cases.iter().sum();
    let combinations = raw["model"]["combinations"].as_array().map_or(0, Vec::len);
    println!("I98_CENSUS {label} input_sha={} profile={:?} census_complete={} domain={:?} refusal={:?} required={:?} loads_per_case={cases:?} loads_total={total} \
              combinations={combinations} over={over:?} at_cap={at_cap:?} rows={key_rows:?}",
        sha(&serde_json::to_vec(raw).unwrap()), report.profile, report.census_complete(), law.domain, law.refusal, law.required);
}

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
/// The token after `key` in `text`, up to the first delimiter.
fn token_after(text: &str, key: &str, stop: &[char]) -> Option<String> {
    text.find(key).map(|i| text[i + key.len()..].split(|c: char| stop.contains(&c)).next().unwrap_or("").to_owned())
}
/// The phase a W1 outcome reached (PLAN_v2 §3.5's mapping, as I86 used it).
fn phase_of(outcome: &str) -> &'static str {
    let o = outcome.trim_start_matches("Err(").trim_start_matches("Fallback(");
    if outcome.contains("Successor") || outcome.contains("successor") {
        "E_mov,max (successor)"
    } else if o.starts_with("Preparation") || o.starts_with("Native") || o.starts_with("Candidate") {
        "W2"
    } else if o.starts_with("Staging") || o.starts_with("Serializer") {
        "W3"
    } else if o.starts_with("Precommit") {
        "W4"
    } else {
        "none (no W1 phase work)"
    }
}
/// The ordinary value route alone (I86's): the published verdicts, the report's reciprocal
/// condition estimate, K-D5's evidence line, W2's range scaling, and the diagnostic codes.
/// With `I98_DUMP_PLAIN` and `I98_OUT`, the plain bytes are also written to a file.
fn ordinary(label: &str, raw: &Value, mode: PreviewSolverMode) -> Option<Vec<u8>> {
    let t = Instant::now();
    let envelope = match crate::run_linear_static_preview_value_with_mode(raw.clone(), mode) {
        Ok(e) => e,
        Err(e) => {
            println!("I98_ORDINARY {label} {} Err({})", mode_name(mode), short(e, 400));
            return None;
        }
    };
    let ms = t.elapsed().as_millis();
    let plain_bytes = serde_json::to_vec(&envelope).unwrap();
    let v: Value = serde_json::from_slice(&plain_bytes).unwrap();
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let (mut rcond, mut refine, mut d5, mut range) = (vec![], vec![], vec![], vec![]);
    let mut amplification = vec![];
    for d in v["diagnostics"].as_array().unwrap() {
        let code = d["code"].as_str().unwrap_or("?");
        *counts.entry(format!("{code}:{}", d["severity"].as_str().unwrap_or("?"))).or_default() += 1;
        let message = d["message"].as_str().unwrap_or("");
        if code.starts_with("NUMERICAL_INTEGRITY") {
            rcond.push(token_after(message, "reciprocal_condition_estimate: ", &[',', ' ', '}']).unwrap_or_else(|| "-".into()));
            refine.push(token_after(message, "refinement_attempts: ", &[',', ' ', '}']).unwrap_or_else(|| "-".into()));
            amplification.push(format!("rel={} amp={} load={}",
                token_after(message, "assembly_relative_perturbation_estimate: ", &[',', ' ', '}']).unwrap_or_else(|| "-".into()),
                token_after(message, "assembly_amplification_estimate: ", &[',', ' ', '}']).unwrap_or_else(|| "-".into()),
                token_after(message, "assembly_load_perturbation_estimate: ", &[',', ' ', '}']).unwrap_or_else(|| "-".into())));
            d5.push(message.find("formation_check: reason=").map(|i| short(message[i..].to_owned(), 300)).unwrap_or_else(|| "none".into()));
        }
        if let Some(r) = token_after(message, "range_scaling: force_scale_exponent=", &[' ', ';', ',', '.']) {
            range.push(r);
        }
    }
    let verdicts: Vec<Value> = v["numerical_quality"]["cases"].as_array().map(|a| a.iter()
        .map(|c| json!({"ref": c["basis_ref"]["ref_id"], "solve_quality": c["solve_quality"]})).collect()).unwrap_or_default();
    println!("I98_ORDINARY {label} {} status={} results={} plain_sha={} plain_len={} published_verdicts={} rcond={rcond:?} refinement_attempts={refine:?} assembly={amplification:?} \
              d5={d5:?} range_scaling={range:?} diagnostics={counts:?} ms={ms}",
        mode_name(mode), v["status"], v["results"].as_array().map_or(0, Vec::len), sha(&plain_bytes), plain_bytes.len(), json!(verdicts));
    if std::env::var_os("I98_DUMP_PLAIN").is_some() {
        if let Ok(d) = std::env::var("I98_OUT") {
            let path = std::path::Path::new(&d).join(format!("plain_{label}_{}.json", mode_name(mode)));
            std::fs::write(&path, &plain_bytes).unwrap();
            println!("I98_PLAIN_FILE {label} {} {} sha={}", mode_name(mode), path.file_name().unwrap().to_string_lossy(), sha(&plain_bytes));
        }
    }
    Some(plain_bytes)
}
fn rust_reader(value: &Value, raw: &Value, mode: PreviewSolverMode) -> String {
    let invocation = json!({"request": raw, "solver_mode": mode_name(mode)});
    match open_pipe_stress_result_export::retained_precision::validate(value, Some(&invocation)) {
        Ok(v) => format!("PASS invocation_bound={} numerical_eligible={} classifications={}", v.invocation_bound, v.numerical_eligible, v.classifications.len()),
        Err(e) => format!("FAIL {}", short(format!("{e:?}"), 600)),
    }
}
/// One counted Direct invocation, recorded (I68's, I81's and I86's `probe`).
fn probe(label: &str, raw: &Value, mode: PreviewSolverMode) {
    let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap_or("?").to_owned();
    println!("I98_BEGIN {label} {} registered={} input_sha={}", mode_name(mode), registered(), sha(&serde_json::to_vec(raw).unwrap()));
    let Some(plain_bytes) = ordinary(label, raw, mode) else { return };
    hooks::disarm();
    let armed_before = hooks::armed_names();
    let input = raw.clone();
    let t = Instant::now();
    let (result, counts) = hooks::counted(move || crate::run_linear_static_preview_value_with_retained_direct(input, mode));
    let ms = t.elapsed().as_millis();
    let output = match result {
        Ok(output) => output,
        Err(error) => {
            println!("I98_W1 {label} {} direct=Err({error}) counts={counts:?}", mode_name(mode));
            return;
        }
    };
    let armed_after = hooks::armed_names();
    if let Some(r) = output.admission() {
        let law = r.law();
        println!("I98_ADMISSION {label} {} profile={:?} allowance={:?} refusal={:?} domain={:?} required={:?} census_complete={}",
            mode_name(mode), r.profile, r.allowance, law.refusal, law.domain, law.required, r.census_complete());
    } else {
        println!("I98_ADMISSION {label} {} report=None", mode_name(mode));
    }
    let w1 = match output.retained() {
        None => "None (no permit, no W1)".to_owned(),
        Some(Ok(_)) => "Ok(successor)".to_owned(),
        Some(Err(cause)) => format!("Err({cause:?})"),
    };
    let phase = if w1.starts_with("None") { "none (no W1 phase work)" } else { phase_of(&w1) };
    let envelope_is_plain = serde_json::to_vec(output.envelope()).unwrap() == plain_bytes;
    let (bytes, successor) = match output.into_publication() {
        RetainedPublication::Successor(value) => (serde_json::to_vec(&value).unwrap(), Some(value)),
        RetainedPublication::Ordinary(envelope) => (serde_json::to_vec(&envelope).unwrap(), None),
    };
    let n = notices(&bytes);
    let eq_notice = bytes == with_notice(&plain_bytes, &case, None);
    let eq_plain = bytes == plain_bytes;
    println!("I98_W1 {label} {} cause={w1} phase={phase} counts={counts:?} one_run_through_g_c={} armed_before={armed_before:?} armed_after={armed_after:?} \
              envelope_is_plain={envelope_is_plain} published_successor={} notices={n} bytes_eq_with_notice_plain_case_none={eq_notice} bytes_eq_plain={eq_plain} \
              published_sha={} published_len={} direct_ms={ms}",
        mode_name(mode), counts == ONE_RUN_THROUGH_G_C, successor.is_some(), sha(&bytes), bytes.len());
    if let Some(value) = successor {
        let rust = rust_reader(&value, raw, mode);
        let body = &value["retained_precision"]["body"];
        println!("I98_SUCCESSOR {label} {} receipt_sha256={} rust_reader={rust} case_status={} ordinary_initial={} ordinary_w2={}", mode_name(mode),
            value["retained_precision"]["receipt_sha256"], body["cases"][0]["status"], body["ordinary_attempts"][0]["initial"], body["ordinary_attempts"][0]["w2"]);
        if let Ok(d) = std::env::var("I98_OUT") {
            let path = std::path::Path::new(&d).join(format!("successor_{label}_{}.json", mode_name(mode)));
            std::fs::write(&path, &bytes).unwrap();
            println!("I98_SUCCESSOR_FILE {label} {} {} sha={}", mode_name(mode), path.file_name().unwrap().to_string_lossy(), sha(&bytes));
        }
    }
}
/// The S1 witnesses' private driver (retained_memory_witness_tests.rs `permitted_work`,
/// copied as I81 and I86 did): no permit, G-A, G-B and G-C not consulted.
fn witness_work(raw: Value, mode: PreviewSolverMode) -> String {
    let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw, mode).expect("a valid request");
    let mut observer = ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    mark("witness_ordinary_done");
    if ordinary.source_block_recovery.is_some() {
        return "ExactSelected".into();
    }
    if let Some(error) = &observer.error {
        println!("I98_WITNESS_CAPTURE_ERROR {error:?}");
    }
    match crate::retained_w1(observer, ordinary, &capture).1 {
        Ok(_) => "Successor".into(),
        Err(fallback) => format!("Fallback({fallback:?})"),
    }
}
fn witness_stack() -> usize {
    super::RESERVED_STACK_BYTES / super::STACK_WITNESS_DIVISOR
}
/// The witness twin: the private driver on its own thread at R/k = 4 MiB.
fn witness_twin(label: &str, raw: &Value, mode: PreviewSolverMode) -> String {
    let stack = witness_stack();
    let raw = raw.clone();
    hooks::disarm();
    let t = Instant::now();
    let ran = crate::on_reserved_stack(stack, crate::carry_test_hooks(move || witness_work(raw, mode))).expect("the reserved thread spawned");
    println!("I98_WITNESS {label} {} stack={stack} ran={ran} furthest_phase={} ms={}", mode_name(mode), phase_of(&ran), t.elapsed().as_millis());
    ran
}

// ---- W-CB1's bypass (RV114 N-7; probe-only) ----------------------------------------------

/// A permit built by the probe, never by G-A (the archive only). `CapturePermit` holds only
/// its registered profile; the probe takes the one registered entry.
fn probe_permit() -> super::CapturePermit {
    super::CapturePermit { _profile: &super::REGISTERED_PROFILES[0] }
}
/// The Direct path's permitted run (lib.rs `permitted_dispatch` and `permitted_run`) with G-A
/// and G-B bypassed, G-C evaluated and recorded, and W1 run whatever G-C says, on the
/// reserved stack R. G-A's and G-B's refusals are recorded first (the over-cap facts).
fn bypass(label: &str, raw: &Value, mode: PreviewSolverMode) -> String {
    let m = mode_name(mode);
    // G-A, recorded.
    let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode).expect("a valid request");
    let report = super::assess(&capture, &request, super::Entry::Direct);
    let law = report.law();
    println!("I98_BYPASS_GA {label} {m} profile={:?} refusal={:?} domain={:?} required={:?} census_complete={}",
        report.profile, law.refusal, law.domain, law.required, report.census_complete());
    let stack = probe_permit().reserved_stack_bytes();
    // G-B, recorded: one observed ordinary run whose observer holds a probe-built permit.
    let gb = crate::on_reserved_stack(stack, crate::carry_test_hooks(move || {
        let mut observer = ProductCapture::permitted_probe(probe_permit());
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        format!("late_refusal={:?} exact_selected={} capture_error={:?}", observer.late_refusal(), ordinary.source_block_recovery.is_some(),
            observer.error.as_ref().map(|e| short(format!("{e:?}"), 300)))
    })).expect("the reserved thread spawned");
    println!("I98_BYPASS_GB {label} {m} stack={stack} {gb}");
    // The permitted run proper, with G-B bypassed (an observer without a permit).
    let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode).expect("a valid request");
    let (label_owned, raw_owned) = (label.to_owned(), raw.clone());
    hooks::disarm();
    let t = Instant::now();
    let ran = crate::on_reserved_stack(stack, crate::carry_test_hooks(move || {
        let label = label_owned;
        let mut observer = ProductCapture::prepared_probe();
        let mut budget = crate::SourceRecoveryBudget::default();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut budget, Some(&mut observer));
        mark("bypass_ordinary_done");
        if ordinary.source_block_recovery.is_some() {
            return "ExactSelected (G-C not consulted)".to_owned();
        }
        let gc = probe_permit().check_complete(&super::CompleteFacts { ordinary: &ordinary, capture: &observer });
        println!("I98_BYPASS_GC {label} {m} complete={gc:?}");
        if let Some(error) = &observer.error {
            println!("I98_BYPASS_CAPTURE_ERROR {label} {m} {}", short(format!("{error:?}"), 600));
        }
        match crate::retained_w1(observer, ordinary, &capture).1 {
            Ok(successor) => {
                let value = successor.value();
                let body = &value["retained_precision"]["body"];
                println!("I98_BYPASS_SUCCESSOR {label} {m} sha={} len={} receipt_sha256={} rust_reader={} case_status={}", sha(&serde_json::to_vec(value).unwrap()),
                    serde_json::to_vec(value).unwrap().len(), value["retained_precision"]["receipt_sha256"], rust_reader(value, &raw_owned, mode), body["cases"][0]["status"]);
                "Successor".to_owned()
            }
            Err(fallback) => format!("Fallback({fallback:?})"),
        }
    })).expect("the reserved thread spawned");
    println!("I98_BYPASS {label} {m} stack={stack} ran={ran} furthest_phase={} ms={}", phase_of(&ran), t.elapsed().as_millis());
    ran
}

// ---- inputs from files ------------------------------------------------------------------

fn modes_from_env() -> Vec<PreviewSolverMode> {
    match std::env::var("I98_MODES").ok().as_deref() {
        Some("sparse_interactive") => vec![PreviewSolverMode::SparseInteractive],
        Some("dense_scrutiny") => vec![PreviewSolverMode::DenseScrutiny],
        _ => MODES.to_vec(),
    }
}
fn files_from_env(var: &str) -> Vec<(String, Value)> {
    let list = std::env::var(var).unwrap_or_else(|_| panic!("{var}"));
    list.split(':').filter(|p| !p.is_empty()).map(|p| {
        let path = std::path::Path::new(p);
        let label = path.file_stem().unwrap().to_string_lossy().into_owned();
        let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{p}: {e}"));
        println!("I98_FILE {label} file_sha={}", sha(&bytes));
        (label, serde_json::from_slice(&bytes).unwrap())
    }).collect()
}

// ---- the probe's tests ------------------------------------------------------------------

/// Writes the committed inputs (built by U8's committed helpers, copied) and I81's case C to
/// `I98_OUT`, so the records' generator builds its variants on the exact committed Values.
#[test]
#[ignore]
fn zz_i98_dump_builtin() {
    let out = std::env::var("I98_OUT").expect("I98_OUT");
    for (name, value) in [("milestone", raw()), ("two_body_case_a", u8_two_body_case_a()), ("two_body_case_b", u8_two_body_case_b()),
        ("l0_isolated_node", u8_l0_isolated_node()), ("case_c", case_c())] {
        let bytes = serde_json::to_vec_pretty(&value).unwrap();
        std::fs::write(std::path::Path::new(&out).join(format!("builtin_{name}.json")), &bytes).unwrap();
        println!("I98_BUILTIN {name} value_sha={} file_sha={}", sha(&serde_json::to_vec(&value).unwrap()), sha(&bytes));
    }
}

/// Every input in `I98_FILES`: the census, then per mode the ordinary route, Direct,
/// (unless `I98_NO_WITNESS`) the witness twin, and (with `I98_BYPASS`) the bypass.
#[test]
#[ignore]
fn zz_i98_files() {
    for (label, raw) in files_from_env("I98_FILES") {
        census(&label, &raw);
        for mode in modes_from_env() {
            probe(&label, &raw, mode);
            if std::env::var_os("I98_NO_WITNESS").is_none() {
                witness_twin(&label, &raw, mode);
            }
            if std::env::var_os("I98_BYPASS").is_some() {
                bypass(&label, &raw, mode);
            }
        }
    }
}

/// The ordinary route alone (with `I98_DUMP_PLAIN`, the plain bytes written), plus the census.
#[test]
#[ignore]
fn zz_i98_plain() {
    for (label, raw) in files_from_env("I98_FILES") {
        census(&label, &raw);
        for mode in modes_from_env() {
            println!("I98_BEGIN {label} {} registered={} input_sha={}", mode_name(mode), registered(), sha(&serde_json::to_vec(&raw).unwrap()));
            ordinary(&label, &raw, mode);
        }
    }
}
