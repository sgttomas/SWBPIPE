//! I86 B1-SW probe (disposable archive copy only; never committed). Declared in
//! retained_memory.rs beside `law_tests` and `witness_tests`, as I81's B1-0 probe was
//! (R/I81/b1_probe_01), so it can reuse their committed inputs (`cap_maximal`) and the
//! census (`assess`, `cap_rows`). Every probed run goes through the actual Direct entry
//! `run_linear_static_preview_value_with_retained_direct`, counted by grant 2's tally, and
//! through a copy of the S1 witnesses' private driver at R/k = 4 MiB (the "witness twin").
//! Inputs are read from JSON files (env `I86_FILES`), so new variants need no rebuild.
//! Probe-only prints in the archive's production files (all `cfg(test)`, no control-flow
//! change): I81's `I86_SEEDS`, `I86_W1_START`, `I86_PREPARATION_FAILURE`,
//! `I86_CANDIDATE_REFUSAL`, `I86_PRECOMMIT_ERROR`/`_DUMP`, `I86_NATIVE_OUTCOME`, plus the
//! phase marks `I86_MARK` (`retained_w1` and `permitted_run`). Run single-threaded with
//! --nocapture. Every test is `#[ignore]` and run explicitly.
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
fn milestone() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
fn quiet() -> bool {
    std::env::var_os("I86_QUIET").is_some()
}
fn now_us() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
}
/// A W1 phase boundary (called from the production prints).
pub(crate) fn mark(phase: &str) {
    println!("I86_MARK {phase} t_us={}", now_us());
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
/// The published verdicts and the seeds, at `site` (skipped under `I86_QUIET`).
pub(crate) fn seeds(site: &str, observer: &ProductCapture, ordinary: &MechanicsEnvelope) {
    if quiet() {
        return;
    }
    let env = serde_json::to_value(ordinary).unwrap();
    let quality: Vec<Value> = env["numerical_quality"]["cases"].as_array().map(|a| a.iter()
        .map(|c| json!({"ref": c["basis_ref"]["ref_id"], "solve_quality": c["solve_quality"]})).collect()).unwrap_or_default();
    let seeds: Vec<Value> = observer.ordinary.iter().map(seed_json).collect();
    println!("I86_SEEDS site={site} mechanics={} exact_selected={} quality={} seeds={}", env["status"]["mechanics"],
        ordinary.source_block_recovery.is_some(), json!(quality), json!(seeds));
}

// ---- committed inputs (copied from the witnesses, as I81 did) ---------------------------

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

// ---- the census (D1.2–D1.11 rows), the ordinary route, Direct, and the witness twin -----

fn mode_name(mode: PreviewSolverMode) -> &'static str {
    mode.as_str()
}
/// D1's census over a request (parsed once in sparse mode; the census does not read the
/// mode): the domain clause, the refusal, the required bytes, and the cap rows that matter.
fn census(label: &str, raw: &Value) {
    use super::CapFact as K;
    let (request, capture) = match crate::source_receipt::CapturedInvocation::parse(raw.clone(), PreviewSolverMode::SparseInteractive) {
        Ok(x) => x,
        Err(e) => {
            println!("I86_CENSUS {label} parse=Err({})", short(format!("{e:?}"), 400));
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
    let first_over_excluding_case_rows = rows.iter().find(|r| r.observed > r.cap && r.fact != K::LoadCasesCapacity).map(|r| format!("{:?}={}/{}", r.fact, r.observed, r.cap));
    println!("I86_CENSUS {label} input_sha={} profile={:?} census_complete={} domain={:?} refusal={:?} required={:?} loads_per_case={cases:?} loads_total={total} \
              over={over:?} first_over_excluding_LoadCasesCapacity={first_over_excluding_case_rows:?} at_cap={at_cap:?} rows={key_rows:?}",
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
/// The phase a W1 outcome reached (PLAN_v2 §3.5's mapping).
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
/// The ordinary value route alone: the published verdicts, the report's reciprocal condition
/// estimate, K-D5's evidence line, W2's range scaling, and the diagnostic codes (counted).
fn ordinary(label: &str, raw: &Value, mode: PreviewSolverMode) -> Option<Vec<u8>> {
    let t = Instant::now();
    let envelope = match crate::run_linear_static_preview_value_with_mode(raw.clone(), mode) {
        Ok(e) => e,
        Err(e) => {
            println!("I86_ORDINARY {label} {} Err({})", mode_name(mode), short(e, 400));
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
    println!("I86_ORDINARY {label} {} status={} results={} plain_sha={} plain_len={} published_verdicts={} rcond={rcond:?} refinement_attempts={refine:?} assembly={amplification:?} \
              d5={d5:?} range_scaling={range:?} diagnostics={counts:?} ms={ms}",
        mode_name(mode), v["status"], v["results"].as_array().map_or(0, Vec::len), sha(&plain_bytes), plain_bytes.len(), json!(verdicts));
    Some(plain_bytes)
}
/// One counted Direct invocation, recorded (I68's and I81's `probe`).
fn probe(label: &str, raw: &Value, mode: PreviewSolverMode) {
    let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap_or("?").to_owned();
    println!("I86_BEGIN {label} {} registered={} input_sha={}", mode_name(mode), registered(), sha(&serde_json::to_vec(raw).unwrap()));
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
            println!("I86_W1 {label} {} direct=Err({error}) counts={counts:?}", mode_name(mode));
            return;
        }
    };
    let armed_after = hooks::armed_names();
    if let Some(r) = output.admission() {
        let law = r.law();
        println!("I86_ADMISSION {label} {} profile={:?} allowance={:?} refusal={:?} domain={:?} required={:?} census_complete={}",
            mode_name(mode), r.profile, r.allowance, law.refusal, law.domain, law.required, r.census_complete());
    } else {
        println!("I86_ADMISSION {label} {} report=None", mode_name(mode));
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
    println!("I86_W1 {label} {} cause={w1} phase={phase} counts={counts:?} one_run_through_g_c={} armed_before={armed_before:?} armed_after={armed_after:?} \
              envelope_is_plain={envelope_is_plain} published_successor={} notices={n} bytes_eq_with_notice_plain_case_none={eq_notice} bytes_eq_plain={eq_plain} \
              published_sha={} published_len={} direct_ms={ms}",
        mode_name(mode), counts == ONE_RUN_THROUGH_G_C, successor.is_some(), sha(&bytes), bytes.len());
    if let Some(value) = successor {
        let invocation = json!({"request": raw, "solver_mode": mode_name(mode)});
        let rust = match open_pipe_stress_result_export::retained_precision::validate(&value, Some(&invocation)) {
            Ok(v) => format!("PASS invocation_bound={} numerical_eligible={} classifications={}", v.invocation_bound, v.numerical_eligible, v.classifications.len()),
            Err(e) => format!("FAIL {}", short(format!("{e:?}"), 600)),
        };
        let body = &value["retained_precision"]["body"];
        println!("I86_SUCCESSOR {label} {} receipt_sha256={} rust_reader={rust} case_status={} ordinary_initial={} ordinary_w2={}", mode_name(mode),
            value["retained_precision"]["receipt_sha256"], body["cases"][0]["status"], body["ordinary_attempts"][0]["initial"], body["ordinary_attempts"][0]["w2"]);
        if let Ok(d) = std::env::var("I86_OUT") {
            let path = std::path::Path::new(&d).join(format!("successor_{label}_{}.json", mode_name(mode)));
            std::fs::write(&path, &bytes).unwrap();
            println!("I86_SUCCESSOR_FILE {label} {} {} sha={}", mode_name(mode), path.file_name().unwrap().to_string_lossy(), sha(&bytes));
        }
    }
}
/// The S1 witnesses' private driver (retained_memory_witness_tests.rs `permitted_work`,
/// copied as I81 did): no permit, G-B and G-C not consulted.
fn witness_work(raw: Value, mode: PreviewSolverMode) -> String {
    let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw, mode).expect("a valid request");
    let mut observer = ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    mark("witness_ordinary_done");
    if ordinary.source_block_recovery.is_some() {
        return "ExactSelected".into();
    }
    if let Some(error) = &observer.error {
        println!("I86_WITNESS_CAPTURE_ERROR {error:?}");
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
    println!("I86_WITNESS {label} {} stack={stack} ran={ran} furthest_phase={} ms={}", mode_name(mode), phase_of(&ran), t.elapsed().as_millis());
    ran
}

// ---- inputs from files ------------------------------------------------------------------

fn modes_from_env() -> Vec<PreviewSolverMode> {
    match std::env::var("I86_MODES").ok().as_deref() {
        Some("sparse_interactive") => vec![PreviewSolverMode::SparseInteractive],
        Some("dense_scrutiny") => vec![PreviewSolverMode::DenseScrutiny],
        _ => MODES.to_vec(),
    }
}
fn mode_from_env() -> PreviewSolverMode {
    match std::env::var("I86_MODE").expect("I86_MODE").as_str() {
        "sparse_interactive" => PreviewSolverMode::SparseInteractive,
        "dense_scrutiny" => PreviewSolverMode::DenseScrutiny,
        other => panic!("unknown mode {other}"),
    }
}
fn files_from_env(var: &str) -> Vec<(String, Value)> {
    let list = std::env::var(var).unwrap_or_else(|_| panic!("{var}"));
    list.split(':').filter(|p| !p.is_empty()).map(|p| {
        let path = std::path::Path::new(p);
        let label = path.file_stem().unwrap().to_string_lossy().into_owned();
        let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{p}: {e}"));
        println!("I86_FILE {label} file_sha={}", sha(&bytes));
        (label, serde_json::from_slice(&bytes).unwrap())
    }).collect()
}

// ---- the probe's tests ------------------------------------------------------------------

/// Writes the committed inputs (built by the committed helpers) to `I86_OUT`, so the
/// records' generator builds its variants on the exact committed Values.
#[test]
#[ignore]
fn zz_i86_dump_builtin() {
    let out = std::env::var("I86_OUT").expect("I86_OUT");
    for (name, value) in [("cap_maximal", super::law_tests::cap_maximal()), ("w2", w2_input()), ("w2b", w2b_input()), ("milestone", milestone())] {
        let bytes = serde_json::to_vec_pretty(&value).unwrap();
        std::fs::write(std::path::Path::new(&out).join(format!("builtin_{name}.json")), &bytes).unwrap();
        println!("I86_BUILTIN {name} value_sha={} file_sha={}", sha(&serde_json::to_vec(&value).unwrap()), sha(&bytes));
    }
}

/// Controls: the milestone, W2 and W2b (as the committed witnesses build them) through
/// the census, the ordinary route, Direct and the witness twin, both modes.
#[test]
#[ignore]
fn zz_i86_builtin_controls() {
    for (label, raw) in [("milestone", milestone()), ("committed_w2", w2_input()), ("committed_w2b", w2b_input())] {
        census(label, &raw);
        for mode in modes_from_env() {
            probe(label, &raw, mode);
            witness_twin(label, &raw, mode);
        }
    }
}

/// Every input in `I86_FILES`: the census, then per mode the ordinary route, Direct and
/// (unless `I86_NO_WITNESS`) the witness twin.
#[test]
#[ignore]
fn zz_i86_files() {
    for (label, raw) in files_from_env("I86_FILES") {
        census(&label, &raw);
        for mode in modes_from_env() {
            probe(&label, &raw, mode);
            if std::env::var_os("I86_NO_WITNESS").is_none() {
                witness_twin(&label, &raw, mode);
            }
        }
    }
}

/// The census alone (for the assembled three-case request, which D1.4 refuses today).
#[test]
#[ignore]
fn zz_i86_assess() {
    for (label, raw) in files_from_env("I86_FILES") {
        census(&label, &raw);
    }
}

/// One run, one mode, one path, for `/usr/bin/time -l` (item 4): `I86_FILE`, `I86_MODE`,
/// `I86_PATH` = `direct` (the Direct entry), `witness` (the twin) or `value` (the ordinary
/// value route only). Run with `I86_QUIET=1`, so no seed print serializes the envelope.
#[test]
#[ignore]
fn zz_i86_once() {
    let (label, raw) = files_from_env("I86_FILE").remove(0);
    let mode = mode_from_env();
    let path = std::env::var("I86_PATH").expect("I86_PATH");
    mark("call_start");
    let t = Instant::now();
    let outcome = match path.as_str() {
        "direct" => {
            let output = crate::run_linear_static_preview_value_with_retained_direct(raw, mode).expect("direct");
            match output.retained() {
                None => "None (no permit, no W1)".to_owned(),
                Some(Ok(_)) => "Ok(successor)".to_owned(),
                Some(Err(cause)) => format!("Err({cause:?})"),
            }
        }
        "witness" => {
            let stack = witness_stack();
            crate::on_reserved_stack(stack, crate::carry_test_hooks(move || witness_work(raw, mode))).expect("spawned")
        }
        "value" => {
            let envelope = crate::run_linear_static_preview_value_with_mode(raw, mode).expect("value");
            format!("value status={}", serde_json::to_value(&envelope.status).unwrap())
        }
        other => panic!("unknown path {other}"),
    };
    let ms = t.elapsed().as_millis();
    mark("call_end");
    println!("I86_ONCE {label} {} path={path} outcome={outcome} phase={} elapsed_ms={ms}", mode_name(mode), phase_of(&outcome));
}
