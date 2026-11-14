//! The pressure reader's working memory on a document with load combinations and a
//! combination certificate (DEL-05-02): the direct-case pass reads one filtered view
//! of the document (without the certificate, the combination record and the
//! combination rows), dropped before the combination pass, which reads the received
//! document itself. A whole copy of the parsed document anywhere in the read would
//! add at least one tree to the peak and take it past the bounds below.
//!
//! The allocator counts requested bytes on the measuring thread only, so the
//! measure is deterministic for a fixed input: it does not depend on the heap's
//! history or on other test threads. This binary is its own process, so its
//! counting allocator affects no other test.
use open_pipe_stress_result_export::{reader_verdict, semantic_contract};
use serde_json::Value;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct Counting;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static LIVE: Cell<i64> = const { Cell::new(0) };
    static PEAK: Cell<i64> = const { Cell::new(0) };
}

/// Adds `added` live bytes, records the peak, then removes `removed` (a moving
/// reallocation holds its old and new blocks together at that instant).
fn record(added: usize, removed: usize) {
    let _ = ACTIVE.try_with(|active| {
        if active.get() {
            LIVE.with(|live| {
                live.set(live.get() + added as i64);
                PEAK.with(|peak| peak.set(peak.get().max(live.get())));
                live.set(live.get() - removed as i64);
            });
        }
    });
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc(layout);
        if !p.is_null() {
            record(layout.size(), 0);
        }
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc_zeroed(layout);
        if !p.is_null() {
            record(layout.size(), 0);
        }
        p
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        record(0, layout.size());
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let p = System.realloc(ptr, layout, new_size);
        if !p.is_null() {
            record(new_size, layout.size());
        }
        p
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// `f`'s result, the peak of its live requested bytes and its live bytes at the end
/// (both counted from zero at the start, on this thread).
fn measure<R>(f: impl FnOnce() -> R) -> (R, i64, i64) {
    LIVE.with(|l| l.set(0));
    PEAK.with(|p| p.set(0));
    ACTIVE.with(|a| a.set(true));
    let result = f();
    ACTIVE.with(|a| a.set(false));
    (result, PEAK.with(Cell::get), LIVE.with(Cell::get))
}

/// The bound on the reader's peak working memory, in parsed trees (the parsed
/// document's live bytes). Measured with one view: 0.42 to 0.67 tree over the six
/// documents without load/reference states; with the two whole copies the reader
/// formerly made, 1.81 to 2.01 trees. A whole copy of the document is at least one
/// tree on top of the view.
const BOUND: f64 = 0.85;
/// A document with load/reference states: its direct-case pass also projects the
/// view once more for the base checks (`load_reference::project`, a copy of the
/// view), so two views are alive there. Measured 1.12 trees (2.01 with the two
/// whole copies).
const LOAD_REFERENCE_BOUND: f64 = 1.4;

const CERTIFICATE_CORPUS: &str = include_str!("../../../../fixtures/results/combination_certificate_corpus.json");
const COMBINATION_CORPUS: &str = include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json");

/// Every pressure-route corpus document with both a certificate and combination rows,
/// as text, with its model.
fn documents() -> Vec<(String, String, Value)> {
    let mut out = Vec::new();
    let certificate: Value = serde_json::from_str(CERTIFICATE_CORPUS).unwrap();
    for e in certificate["documents"].as_array().unwrap().iter().filter(|e| e["reader"] == "pressure") {
        out.push((e["name"].as_str().unwrap().to_string(), e["document"].to_string(), e["model"].clone()));
    }
    let combination: Value = serde_json::from_str(COMBINATION_CORPUS).unwrap();
    for case in combination["cases"].as_array().unwrap() {
        out.push((case["label"].as_str().unwrap().to_string(), case["envelope"].to_string(), case["model"].clone()));
    }
    out
}

#[test]
fn the_pressure_reader_holds_at_most_one_view_of_the_document() {
    let documents = documents();
    assert_eq!(documents.len(), 7);
    for (name, text, model) in &documents {
        let (document, _, tree) = measure(|| serde_json::from_str::<Value>(text).unwrap());
        let evidence = &document["contract_evidence"];
        assert!(evidence.get("combination_certificate").is_some() && evidence.get("exact_combinations").is_some(), "{name}");
        // Warm-up: the reader's static tables are parsed once, on first use.
        semantic_contract::validate_pressure_evidence(&document).unwrap();
        let (verdict, peak, live) = measure(|| semantic_contract::validate_pressure_evidence(&document));
        assert_eq!(verdict, Ok(()), "{name}");
        assert_eq!(live, 0, "{name}: the read keeps nothing");
        let (verdict, model_peak, _) = measure(|| reader_verdict::validate("pressure", &document, Some(model), None));
        assert_eq!(verdict["ok"], true, "{name}");
        let (pressure, with_model) = (peak as f64 / tree as f64, model_peak as f64 / tree as f64);
        eprintln!("{name}: tree {tree} B; pressure evidence peak {peak} B = {pressure:.3} T; with the model {model_peak} B = {with_model:.3} T");
        let bound = if evidence.get("load_reference_states").is_some() { LOAD_REFERENCE_BOUND } else { BOUND };
        assert!(pressure < bound, "{name}: the pressure evidence read peaks at {pressure:.3} trees (bound {bound})");
        assert!(with_model < bound, "{name}: the reader with its model peaks at {with_model:.3} trees (bound {bound})");
    }
}
