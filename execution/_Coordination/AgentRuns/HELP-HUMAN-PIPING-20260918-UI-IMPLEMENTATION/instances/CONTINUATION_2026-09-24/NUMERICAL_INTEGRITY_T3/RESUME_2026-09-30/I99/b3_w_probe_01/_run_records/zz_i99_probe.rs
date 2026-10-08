//! I99 B3-W probe (disposable archive copy only; never committed). Declared in
//! retained_memory.rs beside `law_tests` and `witness_tests`, as I81's and I86's probes
//! were (R/I81/b1_probe_01, R/I86/b1_w_probe_01), whose `probe()`, census, seed print and
//! witness twin it reuses (renamed I99). Inputs are JSON files (env `I99_FILES`).
//! Per input and mode it records:
//! - the ordinary value route (`run_linear_static_preview_value_with_mode`, which is
//!   `ordinary_dispatch`: the exact route's 8,000,000 per-case exact-block budget):
//!   the published verdicts, physics-source-1's selection (`source_block_recovery`, per
//!   case), and the rows published per case and kind;
//! - one counted Direct invocation (`run_linear_static_preview_value_with_retained_direct`):
//!   the admission report, the W1 cause, the byte equalities (I81's and I86's `probe`);
//! - the witness twin: the S1 witnesses' private driver (`permitted_work`, copied), which
//!   uses `SourceRecoveryBudget::default()` (4,000,000, `permitted_run`'s budget) and skips
//!   G-A, G-B and G-C, with the seeds and the capture error printed after its ordinary run;
//! - P-2's budget note: `run_linear_static_preview_captured` at 4,000,000 and at
//!   8,000,000 per case, compared with each other and with the value route.
//! Probe-only prints in the archive's production files (all `cfg(test)`, no control-flow
//! change) are I86's, renamed: `I99_SEEDS` (permitted_run), `I99_W1_START`,
//! `I99_PREPARATION_FAILURE`, `I99_CANDIDATE_REFUSAL`, `I99_PRECOMMIT_ERROR`/`_DUMP`,
//! `I99_NATIVE_OUTCOME`. Run single-threaded with --nocapture. Every test is `#[ignore]`.
use crate::retained_product::{InitialSeed, LegacySeed, OrdinarySeed, ProductCapture, W2Seed};
use crate::retained_tests_hooks::{self as hooks, Counts};
use crate::{MechanicsEnvelope, PreviewSolverMode, RetainedPublication, StructuralError};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

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
fn out_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("I99_OUT").map(std::path::PathBuf::from)
}
fn save(name: &str, bytes: &[u8]) {
    if let Some(d) = out_dir() {
        std::fs::write(d.join(name), bytes).unwrap();
    }
}

// ---- the seed print (I81's and I86's) -----------------------------------------------------

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
        Some(LegacySeed::DeclinedWithoutAttempt { work, .. }) => format!("declined_without_attempt(stage={})", work.stage),
        Some(LegacySeed::Unavailable { work, .. }) => format!("unavailable(stage={}, charged={}, limit={})", work.stage, work.charged, work.limit),
        Some(LegacySeed::ExactSelected) => "exact_selected".to_owned(),
    };
    json!({"case": s.case, "initial": initial, "w2": w2, "legacy": legacy, "recovery_demoted": s.recovery_demoted,
        "load_row_finding": s.load_row_finding.is_some(), "d5_ref": s.d5_diagnostic_ref})
}
/// The published verdicts and the seeds, at `site`.
pub(crate) fn seeds(site: &str, observer: &ProductCapture, ordinary: &MechanicsEnvelope) {
    let env = serde_json::to_value(ordinary).unwrap();
    let quality: Vec<Value> = env["numerical_quality"]["cases"].as_array().map(|a| a.iter()
        .map(|c| json!({"ref": c["basis_ref"]["ref_id"], "solve_quality": c["solve_quality"]})).collect()).unwrap_or_default();
    let seeds: Vec<Value> = observer.ordinary.iter().map(seed_json).collect();
    println!("I99_SEEDS site={site} mechanics={} exact_selected={} quality={} seeds={}", env["status"]["mechanics"],
        ordinary.source_block_recovery.is_some(), json!(quality), json!(seeds));
}

// ---- what the exact route publishes ------------------------------------------------------

fn token_after(text: &str, key: &str, stop: &[char]) -> Option<String> {
    text.find(key).map(|i| text[i + key.len()..].split(|c: char| stop.contains(&c)).next().unwrap_or("").to_owned())
}
/// The facts B3-W records from one published envelope (any route).
fn facts(v: &Value) -> Value {
    let verdicts: Vec<Value> = v["numerical_quality"]["cases"].as_array().map(|a| a.iter()
        .map(|c| json!({"ref": c["basis_ref"]["ref_id"], "solve_quality": c["solve_quality"], "structural_status": c["structural_status"]})).collect()).unwrap_or_default();
    let sbr = &v["source_block_recovery"];
    let selection = if sbr.is_null() {
        json!(null)
    } else {
        let body = &sbr["body"];
        let cases: Vec<Value> = body["cases"].as_array().map(|a| a.iter().map(|c| json!({
            "ref": c["basis_ref"]["ref_id"], "selected_method": c["selected_method"], "outcome": c["outcome"],
            "work": c["work"], "failure": c["failure"], "rows": c["rows"].as_array().map_or(0, Vec::len)})).collect()).unwrap_or_default();
        json!({"policy": body["policy"], "status": body["status"], "invocation_work": body["invocation_work"], "cases": cases,
            "receipt_sha256": sbr["receipt_sha256"]})
    };
    // Rows per case and kind.
    let mut rows: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    for r in v["results"].as_array().into_iter().flatten() {
        let case = r["basis_ref"]["ref_id"].as_str().unwrap_or("<none>").to_owned();
        let kind = r["kind"].as_str().unwrap_or("?").to_owned();
        *rows.entry(case).or_default().entry(kind).or_default() += 1;
    }
    let totals: BTreeMap<String, usize> = rows.iter().map(|(c, k)| (c.clone(), k.values().sum())).collect();
    let mut codes: BTreeMap<String, usize> = BTreeMap::new();
    let (mut rcond, mut d5, mut unavailable) = (vec![], vec![], vec![]);
    for d in v["diagnostics"].as_array().into_iter().flatten() {
        let code = d["code"].as_str().unwrap_or("?");
        *codes.entry(format!("{code}:{}", d["severity"].as_str().unwrap_or("?"))).or_default() += 1;
        let message = d["message"].as_str().unwrap_or("");
        let refs = d["affected_refs"].clone();
        if code.starts_with("NUMERICAL_INTEGRITY") {
            rcond.push(json!({"refs": refs, "rcond": token_after(message, "reciprocal_condition_estimate: ", &[',', ' ', '}'])}));
            d5.push(json!({"refs": d["affected_refs"], "d5": message.find("formation_check: reason=").map(|i| short(message[i..].to_owned(), 300))}));
        }
        if code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE" || code == "SOURCE_BLOCK_RECOVERY_SELECTED" {
            unavailable.push(json!({"code": code, "refs": d["affected_refs"], "message": short(message.to_owned(), 400)}));
        }
    }
    let ce = &v["contract_evidence"];
    let evidence = if ce.is_null() {
        json!(null)
    } else {
        json!({"keys": ce.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()), "pressure": ce["pressure"].as_array().map(Vec::len),
            "connector": ce["connector"].as_array().map(Vec::len), "exact_cases": ce["exact_cases"].as_array().map(|a| a.iter().map(|c| json!({
                "load_case_id": c["load_case_id"], "keys": c.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()),
                "pipe_sections": c["pipe_sections"], "pipe_materials": c["pipe_materials"]})).collect::<Vec<_>>())})
    };
    json!({"contract": v["producer"]["semantic_contract_id"], "profile": v["formulation_basis"]["profile_id"], "status": v["status"],
        "verdicts": verdicts, "exact_block": selection, "rows_per_case": totals, "rows_by_kind": rows, "diagnostics": codes,
        "rcond": rcond, "d5": d5, "source_block_diagnostics": unavailable, "contract_evidence": evidence})
}

// ---- the census, the ordinary route, Direct, and the witness twin (I86's) ---------------

fn mode_name(mode: PreviewSolverMode) -> &'static str {
    mode.as_str()
}
/// D1's census and admission over a request (I86's, shortened to the admission facts).
fn census(label: &str, raw: &Value) {
    let (request, capture) = match crate::source_receipt::CapturedInvocation::parse(raw.clone(), PreviewSolverMode::SparseInteractive) {
        Ok(x) => x,
        Err(e) => {
            println!("I99_CENSUS {label} parse=Err({})", short(format!("{e:?}"), 400));
            return;
        }
    };
    let report = super::assess(&capture, &request, super::Entry::Direct);
    let law = report.law();
    println!("I99_CENSUS {label} input_sha={} profile={:?} census_complete={} domain={:?} refusal={:?} required={:?}",
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
/// The ordinary value route (the exact route's 8M budget): the facts above, and the bytes.
fn ordinary(label: &str, raw: &Value, mode: PreviewSolverMode) -> Option<Vec<u8>> {
    let envelope = match crate::run_linear_static_preview_value_with_mode(raw.clone(), mode) {
        Ok(e) => e,
        Err(e) => {
            println!("I99_ORDINARY {label} {} Err({})", mode_name(mode), short(e, 400));
            return None;
        }
    };
    let plain_bytes = serde_json::to_vec(&envelope).unwrap();
    let v: Value = serde_json::from_slice(&plain_bytes).unwrap();
    println!("I99_ORDINARY {label} {} results={} plain_sha={} plain_len={} facts={}", mode_name(mode),
        v["results"].as_array().map_or(0, Vec::len), sha(&plain_bytes), plain_bytes.len(), facts(&v));
    save(&format!("plain_{label}_{}.json", mode_name(mode)), &plain_bytes);
    Some(plain_bytes)
}
/// One counted Direct invocation, recorded (I68's, I81's and I86's `probe`).
fn probe(label: &str, raw: &Value, mode: PreviewSolverMode, plain_bytes: &[u8]) {
    let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap_or("?").to_owned();
    hooks::disarm();
    let armed_before = hooks::armed_names();
    let input = raw.clone();
    let (result, counts) = hooks::counted(move || crate::run_linear_static_preview_value_with_retained_direct(input, mode));
    let output = match result {
        Ok(output) => output,
        Err(error) => {
            println!("I99_W1 {label} {} direct=Err({error}) counts={counts:?}", mode_name(mode));
            return;
        }
    };
    let armed_after = hooks::armed_names();
    if let Some(r) = output.admission() {
        let law = r.law();
        println!("I99_ADMISSION {label} {} profile={:?} allowance={:?} refusal={:?} domain={:?} required={:?} census_complete={}",
            mode_name(mode), r.profile, r.allowance, law.refusal, law.domain, law.required, r.census_complete());
    } else {
        println!("I99_ADMISSION {label} {} report=None", mode_name(mode));
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
    let eq_notice = bytes == with_notice(plain_bytes, &case, None);
    let eq_plain = bytes == plain_bytes;
    println!("I99_W1 {label} {} cause={w1} phase={phase} counts={counts:?} one_run_through_g_c={} armed_before={armed_before:?} armed_after={armed_after:?} \
              envelope_is_plain={envelope_is_plain} published_successor={} notices={n} bytes_eq_with_notice_plain_case_none={eq_notice} bytes_eq_plain={eq_plain} \
              published_sha={} published_len={}",
        mode_name(mode), counts == ONE_RUN_THROUGH_G_C, successor.is_some(), sha(&bytes), bytes.len());
    if let Some(value) = successor {
        let invocation = json!({"request": raw, "solver_mode": mode_name(mode)});
        let rust = match open_pipe_stress_result_export::retained_precision::validate(&value, Some(&invocation)) {
            Ok(v) => format!("PASS invocation_bound={} numerical_eligible={} classifications={}", v.invocation_bound, v.numerical_eligible, v.classifications.len()),
            Err(e) => format!("FAIL {}", short(format!("{e:?}"), 600)),
        };
        let body = &value["retained_precision"]["body"];
        println!("I99_SUCCESSOR {label} {} receipt_sha256={} rust_reader={rust} case_status={} ordinary_initial={} ordinary_w2={}", mode_name(mode),
            value["retained_precision"]["receipt_sha256"], body["cases"][0]["status"], body["ordinary_attempts"][0]["initial"], body["ordinary_attempts"][0]["w2"]);
        save(&format!("successor_{label}_{}.json", mode_name(mode)), &bytes);
    }
}
/// The S1 witnesses' private driver (retained_memory_witness_tests.rs `permitted_work`,
/// copied as I81 and I86 did): no permit, G-A, G-B and G-C not consulted, the default
/// (4,000,000 per case) exact-block budget. The seeds and the capture error are printed
/// after its ordinary run, before W1 (so a coexistence run also shows them).
fn witness_work(label: String, raw: Value, mode: PreviewSolverMode) -> String {
    let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw, mode).expect("a valid request");
    let mut observer = ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    let bytes = serde_json::to_vec(&ordinary).unwrap();
    println!("I99_DRIVER_ORDINARY {label} {} sha={} len={} exact_selected={}", mode_name(mode), sha(&bytes), bytes.len(), ordinary.source_block_recovery.is_some());
    seeds("witness_driver", &observer, &ordinary);
    if let Some(error) = &observer.error {
        println!("I99_WITNESS_CAPTURE_ERROR {label} {} {error:?}", mode_name(mode));
    }
    if ordinary.source_block_recovery.is_some() {
        return "ExactSelected".into();
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
    let owned = label.to_owned();
    hooks::disarm();
    let ran = crate::on_reserved_stack(stack, crate::carry_test_hooks(move || witness_work(owned, raw, mode))).expect("the reserved thread spawned");
    println!("I99_WITNESS {label} {} stack={stack} ran={ran} furthest_phase={}", mode_name(mode), phase_of(&ran));
    ran
}
/// P-2 (B3-D §5): the same captured ordinary run at 4,000,000 (`SourceRecoveryBudget::
/// default()`, `permitted_run`'s) and at `PHYSICS_SOURCE_WORK_LIMIT` (8,000,000,
/// `ordinary_dispatch`'s exact-route value), each compared with the value route's bytes.
fn budgets(label: &str, raw: &Value, mode: PreviewSolverMode, plain_bytes: &[u8]) {
    let mut lines = vec![];
    let mut all = vec![];
    for limit in [None, Some(crate::PHYSICS_SOURCE_WORK_LIMIT)] {
        let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode).expect("a valid request");
        let mut budget = crate::SourceRecoveryBudget::default();
        if let Some(limit) = limit {
            budget.per_case_limit = limit;
        }
        let per_case = budget.per_case_limit;
        let envelope = crate::run_linear_static_preview_captured(request, mode, Some(&capture), &mut budget);
        let bytes = serde_json::to_vec(&envelope).unwrap();
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        let f = facts(&v);
        lines.push(format!("per_case_limit={per_case} sha={} len={} eq_value_route={} contract={} exact_block={}", sha(&bytes), bytes.len(),
            bytes == plain_bytes, f["contract"], f["exact_block"]));
        save(&format!("budget_{per_case}_{label}_{}.json", mode_name(mode)), &bytes);
        all.push(bytes);
    }
    println!("I99_BUDGET {label} {} same_bytes_4M_8M={} [{}]", mode_name(mode), all[0] == all[1], lines.join("] ["));
}

// ---- inputs from files ------------------------------------------------------------------

fn modes_from_env() -> Vec<PreviewSolverMode> {
    match std::env::var("I99_MODES").ok().as_deref() {
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
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        println!("I99_FILE {label} file_sha={} value_sha={}", sha(&bytes), sha(&serde_json::to_vec(&value).unwrap()));
        (label, value)
    }).collect()
}
fn run_one(label: &str, raw: &Value) {
    census(label, raw);
    for mode in modes_from_env() {
        println!("I99_BEGIN {label} {} registered={} input_sha={}", mode_name(mode), registered(), sha(&serde_json::to_vec(raw).unwrap()));
        let Some(plain) = ordinary(label, raw, mode) else { continue };
        probe(label, raw, mode, &plain);
        witness_twin(label, raw, mode);
        budgets(label, raw, mode, &plain);
    }
}

// ---- the probe's tests ------------------------------------------------------------------

/// Writes the committed milestone Value (the generator's base), as I86's dump did.
#[test]
#[ignore]
fn zz_i99_dump_builtin() {
    let out = out_dir().expect("I99_OUT");
    let value = milestone();
    let bytes = serde_json::to_vec_pretty(&value).unwrap();
    std::fs::write(out.join("builtin_milestone.json"), &bytes).unwrap();
    println!("I99_BUILTIN milestone value_sha={} file_sha={}", sha(&serde_json::to_vec(&value).unwrap()), sha(&bytes));
}

/// Control: the committed 0.1.0 milestone, through every path (reproduces I81's and I86's lines).
#[test]
#[ignore]
fn zz_i99_controls() {
    run_one("milestone", &milestone());
}

/// Every input in `I99_FILES`, through every path, both modes (unless `I99_MODES`).
#[test]
#[ignore]
fn zz_i99_files() {
    for (label, raw) in files_from_env("I99_FILES") {
        run_one(&label, &raw);
    }
}
