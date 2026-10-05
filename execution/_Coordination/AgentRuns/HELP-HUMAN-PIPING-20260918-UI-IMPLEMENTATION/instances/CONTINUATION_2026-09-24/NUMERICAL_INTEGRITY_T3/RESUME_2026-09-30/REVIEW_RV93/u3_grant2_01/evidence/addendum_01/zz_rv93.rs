//! RV93 (independent review of U3 grant 2), scratch-only probe module. Never committed.
//! Its own oracles: process-global counters (not I61's carried tally) at the ordinary
//! run's first statement, at `solve_load_case_observed`, and inside U4's `check_late` /
//! `check_complete`; the thread each was hit on; and byte oracles built from the value
//! route's own bytes. Run alone with --test-threads=1 (the counters are global).
use super::retained_tests_hooks as hooks;
use super::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::Mutex;

pub(crate) static RUNS: AtomicUsize = AtomicUsize::new(0);
pub(crate) static SOLVES: AtomicUsize = AtomicUsize::new(0);
pub(crate) static GB: AtomicUsize = AtomicUsize::new(0);
pub(crate) static GC: AtomicUsize = AtomicUsize::new(0);
static RUN_THREADS: Mutex<Vec<std::thread::ThreadId>> = Mutex::new(Vec::new());
static GC_THREADS: Mutex<Vec<std::thread::ThreadId>> = Mutex::new(Vec::new());
pub(crate) fn hit(c: &AtomicUsize) {
    c.fetch_add(1, SeqCst);
    if std::ptr::eq(c, &GC) {
        GC_THREADS.lock().unwrap().push(std::thread::current().id());
    }
}
pub(crate) fn hit_run() {
    RUNS.fetch_add(1, SeqCst);
    RUN_THREADS.lock().unwrap().push(std::thread::current().id());
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct C { runs: usize, solves: usize, gb: usize, gc: usize }
fn reset() {
    for c in [&RUNS, &SOLVES, &GB, &GC] { c.store(0, SeqCst); }
    RUN_THREADS.lock().unwrap().clear();
    GC_THREADS.lock().unwrap().clear();
}
fn now() -> C { C { runs: RUNS.load(SeqCst), solves: SOLVES.load(SeqCst), gb: GB.load(SeqCst), gc: GC.load(SeqCst) } }
fn run_threads() -> Vec<std::thread::ThreadId> { RUN_THREADS.lock().unwrap().clone() }
fn gc_threads() -> Vec<std::thread::ThreadId> { GC_THREADS.lock().unwrap().clone() }

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const NOTICE_TEXT: &str = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.";
fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn milestone() -> Value { serde_json::from_str(MILESTONE).unwrap() }
fn fixture(rel: &str) -> Value {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures").join(rel);
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
fn value_bytes(raw: &Value, mode: PreviewSolverMode) -> Vec<u8> {
    serde_json::to_vec(&run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap()
}
fn registered() -> bool {
    // RV93's own reading of the identity: the build script's value equals the one
    // registered entry's text, read from retained_memory.rs's source (not a test constant).
    let src = include_str!("retained_memory.rs");
    let at = src.find("static REGISTERED_PROFILES").unwrap();
    let rest = &src[at..];
    let i = rest.find("identity: \"").unwrap() + "identity: \"".len();
    let ident = &rest[i..i + rest[i..].find('"').unwrap()];
    option_env!("OPS_RETAINED_BUILD_IDENTITY") == Some(ident)
}
fn publish(o: RetainedPreviewOutput) -> (bool, Vec<u8>) {
    match o.into_publication() {
        RetainedPublication::Successor(v) => (true, serde_json::to_vec(&v).unwrap()),
        RetainedPublication::Ordinary(e) => (false, serde_json::to_vec(&e).unwrap()),
    }
}
fn count_notices(b: &[u8]) -> usize {
    let v: Value = serde_json::from_slice(b).unwrap();
    v["diagnostics"].as_array().map_or(0, |d| d.iter().filter(|x| x["code"] == "RETAINED_PRECISION_UNAVAILABLE").count())
}
/// RV93's notice oracle: the published bytes are the value-route bytes with exactly the
/// notice's compact JSON inserted as the last diagnostic, nothing else changed.
fn is_plain_plus_notice(published: &[u8], plain: &[u8], case: &str, detail: Option<&str>) -> Result<(), String> {
    let mut msg = NOTICE_TEXT.to_string();
    if let Some(d) = detail { msg.push_str(&format!(" Reason: receipt_encoding; detail: {d}.")); }
    let notice = serde_json::to_string(&json!({"id": format!("diagnostic:retained-precision:{case}:unavailable")})).unwrap();
    // Field order as the product's Diagnostic struct declares it, written out by hand.
    let notice = format!("{},\"code\":\"RETAINED_PRECISION_UNAVAILABLE\",\"severity\":\"info\",\"message\":{},\"source\":\"core/product_physics\",\"affected_refs\":[{}]}}",
        &notice[..notice.len() - 1], serde_json::to_string(&msg).unwrap(), serde_json::to_string(case).unwrap());
    let p = std::str::from_utf8(published).map_err(|e| e.to_string())?;
    let at = p.find(&notice).ok_or("notice text not found")?;
    if p.matches("RETAINED_PRECISION_UNAVAILABLE").count() != 1 { return Err("not exactly one notice code".into()); }
    let (before, after) = (&p[..at], &p[at + notice.len()..]);
    // Remove the notice and its separating comma (if the plain diagnostics were non-empty).
    let rebuilt = if before.ends_with(',') { format!("{}{}", &before[..before.len() - 1], after) } else { format!("{before}{after}") };
    if rebuilt.as_bytes() != plain { return Err(format!("removing the notice does not give the plain bytes ({} vs {})", rebuilt.len(), plain.len())); }
    if !after.starts_with("]") { return Err("notice is not the last diagnostic".into()); }
    Ok(())
}
fn out_dir() -> Option<std::path::PathBuf> { std::env::var("RV93_OUT").ok().map(std::path::PathBuf::from) }

/// Review item 1: the milestone through the actual Direct entry, both modes.
#[test]
fn zz_rv93_milestone_direct() {
    assert!(registered(), "this probe is for the registered build");
    for mode in MODES {
        let raw = milestone();
        let plain = value_bytes(&raw, mode);
        reset();
        let caller = std::thread::current().id();
        let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
        let c = now();
        let report = out.admission().expect("report");
        assert_eq!(report.profile, ProfileStatus::Registered);
        assert_eq!(report.law().refusal, None);
        assert_eq!(c, C { runs: 1, solves: 1, gb: 1, gc: 1 }, "{mode:?}");
        assert!(run_threads().iter().all(|t| *t != caller), "the ordinary run ran on the reserved-stack thread");
        assert!(gc_threads().iter().all(|t| *t != caller));
        let envelope = serde_json::to_vec(out.envelope()).unwrap();
        assert_eq!(envelope, plain, "B': envelope beside the successor is the plain run");
        let (successor, bytes) = publish(out);
        assert!(successor, "{mode:?}: publishes a Successor");
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        let doc = serde_json::to_string_pretty(&json!({"id": format!("u1_milestone_{}", mode.as_str()), "source": value,
            "invocation": {"request": raw, "solver_mode": mode.as_str()}})).unwrap();
        let invocation = json!({"request": milestone(), "solver_mode": mode.as_str()});
        let rust = match open_pipe_stress_result_export::retained_precision::validate(&value, Some(&invocation)) {
            Ok(v) => format!("PASS invocation_bound={} eligible={} classes={} publication_sha256={}", v.invocation_bound, v.numerical_eligible, v.classifications.len(), v.publication_sha256),
            Err(e) => format!("FAIL {e:?}"),
        };
        println!("RV93_MILESTONE mode={} counts={c:?} published_sha={} published_len={} doc_sha={} receipt={} rust_reader: {rust}",
            mode.as_str(), sha(&bytes), bytes.len(), sha(doc.as_bytes()), value["retained_precision"]["receipt_sha256"]);
        if let Some(d) = out_dir() {
            std::fs::write(d.join(format!("u1_milestone_{}.json", mode.as_str())), &doc).unwrap();
            std::fs::write(d.join(format!("published_{}.bin", mode.as_str())), &bytes).unwrap();
        }
    }
}

/// Review item 2: every W1 fallback, each built by RV93 (hooks armed on the caller, or a
/// real input), against RV93's notice oracle and the global counters.
#[test]
fn zz_rv93_fallbacks() {
    use super::retained_receipt::TraceFault as F;
    use super::retained_wire::ReceiptCheck as K;
    assert!(registered());
    type Arm = Box<dyn Fn()>;
    let cases: Vec<(&str, Arm, &str, Option<&str>)> = vec![
        ("preparation", Box::new(hooks::fail_next_preparation), "Preparation", None),
        ("native", Box::new(hooks::withdraw_next_native_source), "Native", None),
        ("candidate_maxima", Box::new(|| hooks::fault_next_candidate(F::Maxima)), "Candidate", None),
        ("staging", Box::new(hooks::break_next_staging), "Staging(StagingFault(\"pipe_stress_extrema[]\"))", None),
        ("serializer_range", Box::new(|| hooks::fail_next_serializer(K::WorkCounterRange)), "Serializer", Some("work_counter_range")),
        ("serializer_hash", Box::new(|| hooks::fail_next_serializer(K::PublicationHashRange)), "Serializer", Some("publication_hash_range")),
        ("serializer_saturation", Box::new(|| hooks::fail_next_serializer(K::SaturationNotExcluded)), "Serializer", Some("saturation_not_excluded")),
        ("serializer_encoding", Box::new(|| hooks::fail_next_serializer(K::Encoding)), "Serializer", None),
        ("precommit_g1", Box::new(hooks::corrupt_next_precommit), "Precommit", None),
        ("precommit_g8", Box::new(hooks::rebind_next_precommit_invocation), "Precommit", None),
    ];
    for mode in MODES {
        let raw = milestone();
        let plain = value_bytes(&raw, mode);
        for (label, arm, cause, detail) in &cases {
            hooks::disarm();
            arm();
            reset();
            let caller = std::thread::current().id();
            let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
            let c = now();
            let got = format!("{:?}", out.retained().map(|r| r.as_ref().err()));
            assert!(got.contains(cause), "{label} {mode:?}: {got}");
            assert!(out.successor().is_none());
            assert_eq!(c, C { runs: 1, solves: 1, gb: 1, gc: 1 }, "{label} {mode:?}");
            assert!(gc_threads().iter().all(|t| *t != caller), "{label}: G-C on the worker");
            assert_eq!(hooks::armed_names(), Vec::<&str>::new(), "{label}: fault consumed on the worker");
            let (succ, bytes) = publish(out);
            assert!(!succ);
            assert_eq!(count_notices(&bytes), 1, "{label} {mode:?}");
            is_plain_plus_notice(&bytes, &plain, "case", *detail).unwrap_or_else(|e| panic!("{label} {mode:?}: {e}"));
            println!("RV93_FALLBACK {label} {} cause={got} counts={c:?} notice=1 sha={}", mode.as_str(), sha(&bytes));
        }
        // A fallback from a real input: rejected_stress_range (solve attempted, legacy block after).
        let rejected = fixture(&format!("product_preview/source_blocks/rejected_stress_range/{}.request.json", mode.as_str()));
        let plain_r = value_bytes(&rejected, mode);
        hooks::disarm();
        reset();
        let out = run_linear_static_preview_value_with_retained_direct(rejected.clone(), mode).unwrap();
        let c = now();
        let got = format!("{:?}", out.retained().map(|r| r.as_ref().err()));
        let case = rejected["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
        let (succ, bytes) = publish(out);
        assert!(!succ);
        assert_eq!(count_notices(&bytes), 1);
        is_plain_plus_notice(&bytes, &plain_r, &case, None).unwrap();
        println!("RV93_FALLBACK input:rejected_stress_range {} cause={got} counts={c:?} notice=1", mode.as_str());
    }
}

/// Review item 2: refusals before W1 work: exact value-route bytes, no notice.
#[test]
fn zz_rv93_no_w1_refusals() {
    assert!(registered());
    let m = milestone();
    let variant = |f: &dyn Fn(&mut Value)| { let mut v = m.clone(); f(&mut v); v };
    let inputs: Vec<(&str, Value, &str)> = vec![
        ("GA_D1.3_schema_0.3.0", variant(&|v| v["model"]["schema_version"] = json!("0.3.0")), "G-A"),
        ("GA_D1.4_components", variant(&|v| v["model"]["components"] = json!([{"id":"c1","kind":"elbow"}])), "G-A"),
        ("GA_D1.6_support_family", variant(&|v| v["model"]["supports"][0]["family"] = json!("hanger_rigid")), "G-A"),
        ("GA_D1.4_combination", variant(&|v| v["model"]["combinations"] = json!([{"id":"k","basis":"mechanics","terms":[{"load_case":"case","factor":2.0}]}])), "G-A"),
        ("GC_unattempted_category", variant(&|v| v["model"]["load_cases"][0]["primitive_loads"][0]["category"] = json!("not_a_category")), "G-C"),
        ("GC_unattempted_no_supports", variant(&|v| v["model"]["supports"] = json!([])), "G-C"),
        ("coexistence_n06", fixture("product_preview/source_blocks/n06-sparse_interactive.request.json"), "coexistence"),
        ("coexistence_torsion", fixture("product_preview/numerical_sensitive_torsion_model.json"), "coexistence"),
    ];
    for mode in MODES {
        for (label, raw, kind) in &inputs {
            let raw = if raw.get("model").is_some() { raw.clone() } else { json!({"model": raw}) };
            let plain = run_linear_static_preview_value_with_mode(raw.clone(), mode).map(|e| serde_json::to_vec(&e).unwrap());
            hooks::disarm();
            reset();
            let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode);
            let c = now();
            let (Ok(plain), Ok(out)) = (plain, out) else { println!("RV93_REFUSAL {label} {} ERR (both routes error equally?)", mode.as_str()); continue };
            let report = out.admission().map(|r| (r.profile, r.law().refusal.map(|x| x.clause().map(|c| c.id()))));
            let got = format!("{:?}", out.retained().map(|r| r.as_ref().err()));
            let (succ, bytes) = publish(out);
            assert!(!succ, "{label}");
            assert_eq!(bytes, plain, "{label} {mode:?}: exact bytes");
            assert_eq!(count_notices(&bytes), 0);
            assert_eq!(c.runs, 1, "{label}: one run");
            match *kind {
                "G-A" => { assert!(got == "None", "{label}: {got}"); assert_eq!((c.gb, c.gc), (0, 0)); }
                "G-C" => { assert!(got.contains("CompleteGate") && got.contains("OrdinarySolveNotAttempted"), "{label}: {got}"); assert_eq!(c.gc, 1); }
                _ => { assert!(got.contains("Coexistence"), "{label}: {got}"); assert_eq!(c.gc, 0); }
            }
            println!("RV93_REFUSAL {label} {} report={report:?} cause={got} counts={c:?} exact=1 notices=0", mode.as_str());
        }
        let raw = milestone();
        let plain = value_bytes(&raw, mode);
        // G-B by the hook; G-C by the hook; the stack by the override.
        for label in ["GB_hook", "GC_hook", "stack"] {
            hooks::disarm();
            match label {
                "GB_hook" => hooks::fail_next_late_gate(),
                "GC_hook" => hooks::fail_next_complete_gate(),
                _ => super::retained_memory::RESERVED_STACK_OVERRIDE.with(|c| c.set(Some(1usize << 62))),
            }
            reset();
            let caller = std::thread::current().id();
            let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
            super::retained_memory::RESERVED_STACK_OVERRIDE.with(|c| c.set(None));
            let c = now();
            let got = format!("{:?}", out.retained().map(|r| r.as_ref().err()));
            let (succ, bytes) = publish(out);
            assert!(!succ);
            assert_eq!(bytes, plain, "{label} {mode:?}");
            assert_eq!(count_notices(&bytes), 0);
            match label {
                "GB_hook" => { assert!(got.contains("LateGate"), "{got}"); assert_eq!(c, C { runs: 1, solves: 1, gb: 1, gc: 0 }); }
                "GC_hook" => { assert!(got.contains("CompleteGate") && got.contains("ObservationBytes"), "{got}"); assert_eq!(c, C { runs: 1, solves: 1, gb: 1, gc: 1 }); }
                _ => { assert!(got.contains("StackReservation"), "{got}"); assert_eq!(c, C { runs: 1, solves: 1, gb: 0, gc: 0 }); assert!(run_threads().iter().all(|t| *t == caller), "stack: on the caller"); }
            }
            println!("RV93_REFUSAL {label} {} cause={got} counts={c:?} exact=1 notices=0", mode.as_str());
        }
        hooks::disarm();
    }
}

/// Review item 3 (B'): the no-permit retained entries and the shared route each run once.
#[test]
fn zz_rv93_no_permit_runs_once() {
    let mut two = milestone();
    let mut case = two["model"]["load_cases"][0].clone();
    case["id"] = json!("case-b");
    two["model"]["load_cases"].as_array_mut().unwrap().push(case);
    for mode in MODES {
        reset();
        let _ = run_linear_static_preview_value_with_mode(two.clone(), mode).unwrap();
        let shared = now();
        reset();
        let out = run_linear_static_preview_value_with_retained_direct(two.clone(), mode).unwrap();
        let direct = now();
        assert!(out.retained().is_none());
        reset();
        let inv = json!({"request": milestone(), "solver_mode": mode.as_str()});
        let id = String::from("rv93");
        let m = milestone();
        let h = run_linear_static_preview_value_with_retained_headless(m.clone(), mode, RetainedHeadlessContext::from_borrowed_roots(&m, &inv, &id)).unwrap();
        let headless = now();
        assert!(h.retained().is_none());
        println!("RV93_NOPERMIT {} shared={shared:?} direct_refused={direct:?} headless={headless:?}", mode.as_str());
        assert_eq!(shared.runs, 1);
        assert_eq!(direct, shared, "refused Direct = shared route");
        assert_eq!((headless.runs, headless.gb, headless.gc), (1, 0, 0));
    }
}

/// Review item 2, inputs only: the in-D1 variants whose W1 work ran and fell back in RV93's
/// sweep (a 1e-300 spring; a single moment), against RV93's notice oracle.
#[test]
fn zz_rv93_input_fallbacks() {
    assert!(registered());
    let m = milestone();
    let mut tiny = m.clone();
    tiny["model"]["supports"][1]["stiffness"]["value"]["value"] = json!(1e-300);
    let mut single = m.clone();
    let l = single["model"]["load_cases"][0]["primitive_loads"][0].clone();
    single["model"]["load_cases"][0]["primitive_loads"] = json!([l]);
    for mode in MODES {
        for (label, raw) in [("tiny_spring", &tiny), ("first_load_only", &single)] {
            let plain = value_bytes(raw, mode);
            hooks::disarm();
            reset();
            let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
            let c = now();
            let got = format!("{:?}", out.retained().map(|r| r.as_ref().err()));
            let (succ, bytes) = publish(out);
            assert!(!succ);
            is_plain_plus_notice(&bytes, &plain, "case", None).unwrap_or_else(|e| panic!("{label}: {e}"));
            assert_eq!(c.runs, 1);
            println!("RV93_INPUT_FALLBACK {label} {} cause={got} counts={c:?} notice=1", mode.as_str());
        }
    }
}

// ---- RV93 addendum 01 (the final memory head): the 07h reader's F5 check at precommit ----

/// 0: off; 1: swap the first two `diagnostic_refs` of the ordinary attempt; 2: drop its last.
/// Global (the precommit runs on the reserved-stack worker), consumed once.
pub(crate) static F5_MODE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
/// Both edits keep the refs unique and resolving (checkpoint A's D6a), so before 07h the
/// reader admitted them; only F5's exact-list rule refuses. The receipt hash is recomputed
/// with the serializer's own domain hash, so G1 passes and the first failure is F5's.
fn f5_edit(source: &mut Value, how: u8) {
    let refs = source["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"].as_array_mut().unwrap();
    assert!(refs.len() >= 2);
    match how { 1 => refs.swap(0, 1), _ => { refs.pop(); } }
    let body = source["retained_precision"]["body"].clone();
    source["retained_precision"]["receipt_sha256"] = json!(super::retained_wire::domain_hash("retained_precision_receipt_mp_v2", &body).unwrap());
}
pub(crate) fn f5_tamper(successor: &mut Value) {
    let how = F5_MODE.swap(0, SeqCst);
    if how != 0 { f5_edit(successor, how); }
}

#[test]
fn zz_rv93_f5_precommit_fallback() {
    assert!(registered());
    // The reader alone, on U6's carrier fixtures (the pinned bytes): untampered admitted;
    // each F5 edit refused at G5.
    for (mode, carrier) in MODES.into_iter().zip([
        include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),
        include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")]) {
        let doc: Value = serde_json::from_str(carrier).unwrap();
        let ok = open_pipe_stress_result_export::retained_precision::validate(&doc["source"], Some(&doc["invocation"]));
        println!("RV93_F5_READER {} untampered: {}", mode.as_str(), match &ok { Ok(v) => format!("PASS classes={}", v.classifications.len()), Err(e) => format!("FAIL {e:?}") });
        assert!(ok.is_ok());
        for how in [1u8, 2] {
            let mut source = doc["source"].clone();
            f5_edit(&mut source, how);
            let r = open_pipe_stress_result_export::retained_precision::validate(&source, Some(&doc["invocation"]));
            println!("RV93_F5_READER {} edit={how}: {}", mode.as_str(), match &r { Ok(_) => "ADMITTED".to_string(), Err(e) => format!("refused {e:?}") });
            assert!(r.is_err());
        }
    }
    // The actual Direct entry: the same edits applied to the live successor at precommit.
    for mode in MODES {
        let raw = milestone();
        let plain = value_bytes(&raw, mode);
        for how in [1u8, 2] {
            hooks::disarm();
            F5_MODE.store(how, SeqCst);
            reset();
            let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
            let c = now();
            assert_eq!(F5_MODE.load(SeqCst), 0, "the edit fired");
            let got = format!("{:?}", out.retained().map(|r| r.as_ref().err()));
            assert!(got.contains("Precommit") && got.contains("\"G5\""), "{how} {mode:?}: {got}");
            assert!(out.successor().is_none());
            let (succ, bytes) = publish(out);
            assert!(!succ);
            assert_eq!(count_notices(&bytes), 1);
            is_plain_plus_notice(&bytes, &plain, "case", None).unwrap_or_else(|e| panic!("F5 {how} {mode:?}: {e}"));
            assert_eq!(c, C { runs: 1, solves: 1, gb: 1, gc: 1 });
            println!("RV93_F5_DIRECT {} edit={how} cause={got} counts={c:?} notice=1 exact_prefix=1", mode.as_str());
        }
    }
}

#[test]
fn zz_rv93_milestone_equals_u6_carriers() {
    assert!(registered());
    for (mode, carrier) in MODES.into_iter().zip([
        include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),
        include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")]) {
        let raw = milestone();
        let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
        let (succ, bytes) = publish(out);
        assert!(succ);
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        let doc = serde_json::to_string_pretty(&json!({"id": format!("u1_milestone_{}", mode.as_str()), "source": value,
            "invocation": {"request": raw, "solver_mode": mode.as_str()}})).unwrap();
        assert!(doc == carrier, "{mode:?}: the live document is U6's carrier, byte for byte");
        println!("RV93_CARRIER {} live_doc_sha={} carrier_sha={} equal=true", mode.as_str(), sha(doc.as_bytes()), sha(carrier.as_bytes()));
    }
}
