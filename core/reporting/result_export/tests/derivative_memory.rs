//! The derivative binding's working memory (DEL-05-02): `derivative::derive_document`
//! holds one document. Its six per-row arrays are moved into the document, not copied;
//! its derivative hash (and every other checksum it takes) is the SHA-256 of canonical
//! text written straight into the hasher, never held as a string; and its closing
//! `validate_document` hashes the document in place, without a copy less the hash
//! member. A returning copy of the arrays or of the document, or the canonical
//! string, takes a peak past its bound below.
//!
//! The allocator counts requested bytes on the measuring thread only, so the measure
//! is deterministic for a fixed input: it does not depend on the heap's history or on
//! other test threads. This binary is its own process, so its counting allocator
//! affects no other test.
use open_pipe_stress_result_export::derivative as d;
use serde_json::{json, Value};
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

/// The bound on the derivation's peak, in produced-document trees (the returned
/// document's live requested bytes, T). Measured with one document: 1.14 to 1.35 T
/// over the documents below (the readers' own working memory on top of the
/// document; the most on the retained-precision successors). With the former
/// copies (the six arrays copied by `json!`, the canonical string of the document
/// for its hash, and the validation's copy of the document with that copy's
/// canonical string): 3.14 to 3.46 T. Either returning copy alone takes it to 2.0 T
/// or more (the arrays copied: 2.03 to 2.17 T; the validation's copy: 2.00 to
/// 2.03 T). The canonical string is guarded by `DIGEST_BOUND`, since on some
/// documents the readers' work, not the hash, sets the derivation's peak.
const DERIVE_BOUND: f64 = 1.6;
/// The bound on `validate_document`'s peak over a document it does not own, in the
/// same trees. Measured 0.14 to 0.33 T in place (the readers' own work); with the
/// former copy and its canonical string, 1.30 to 1.50 T.
const VALIDATE_BOUND: f64 = 0.5;
/// The bound on `digest`'s peak over the produced document, in the same trees:
/// the streamed canonical text holds only one object's sorted keys per level and
/// one scalar's text at a time. Measured 3 to 12 kB, at most 0.004 T. With the
/// canonical string of the document built and hashed, 0.29 to 0.49 T.
const DIGEST_BOUND: f64 = 0.02;

const BASE: &str = include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json");

/// Each committed source measured: its name, file and JSON Pointer within the file.
const DOCUMENTS: [(&str, &str, &str); 10] = [
    ("preview-physics-2 invented sparse", include_str!("../../../../fixtures/results/preview_physics_invented_sparse.json"), ""),
    ("preview-physics-2 connected sparse", include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json"), ""),
    ("preview-physics-2 connected dense", include_str!("../../../../fixtures/results/preview_physics_connected_dense.json"), ""),
    ("preview-physics Timoshenko arc", include_str!("../../../../fixtures/results/preview_physics_timoshenko_arc_sparse.json"), ""),
    ("preview-physics-retained sparse", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"), "/source"),
    ("preview-physics-retained dense", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"), "/source"),
    ("physics-retained sparse", include_str!("../../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json"), "/source"),
    ("physics-retained dense", include_str!("../../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json"), "/source"),
    ("pressure-source-1 n05", include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"), ""),
    ("pressure-2 connector witness", include_str!("../../../../fixtures/reports/invented/exact_report_witness_inputs/pressure_2_connector-sparse_interactive.raw.json"), ""),
];

fn origin(source: &Value) -> Value {
    let model = source["model_ref"].as_str().unwrap();
    let carrier = d::checksum(source, "attested_headless_producer_carrier", d::reference("test_carrier", "derivative-memory")).unwrap();
    json!({"origin_id":"derivative-memory-test","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Retained producer bytes; derivative memory test only","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",source["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null})
}

#[test]
fn the_derivative_binding_holds_one_document() {
    let base: Value = serde_json::from_str(BASE).unwrap();
    let mut measured = 0;
    let mut over = Vec::new();
    for (name, text, pointer) in DOCUMENTS {
        let file: Value = serde_json::from_str(text).unwrap();
        let source = file.pointer(pointer).unwrap().clone();
        drop(file);
        // The result text as the desktop holds it: the source's compact JSON text.
        let text_bytes = serde_json::to_string(&source).unwrap().len() as f64;
        let model = json!({"project":{"id":source["model_ref"]}});
        let origin = origin(&source);
        // Warm-up: the readers' static tables are parsed once, on first use.
        d::derive_document(base.clone(), &model, &source, origin.clone(), None).unwrap_or_else(|e| panic!("{name}: {e}"));
        let (inputs, _, input_bytes) = measure(|| (base.clone(), origin.clone()));
        let (document, peak, live) = measure(|| d::derive_document(inputs.0, &model, &source, inputs.1, None));
        let document = document.unwrap_or_else(|e| panic!("{name}: {e}"));
        let tree = (live + input_bytes) as f64;
        let (verdict, validate_peak, kept) = measure(|| d::validate_document(&document, &source));
        assert_eq!(verdict, Ok(()), "{name}");
        assert_eq!(kept, 0, "{name}: the validation keeps nothing");
        let (hash, digest_peak, _) = measure(|| d::digest(&document));
        assert_eq!(hash.map(|h| h.len()), Ok(64), "{name}");
        let digest_share = digest_peak as f64 / tree;
        eprintln!("{name}: digest of the document peak {digest_peak} B = {digest_share:.4} T = {:.2} x text", digest_peak as f64 / text_bytes);
        if digest_share >= DIGEST_BOUND {
            over.push(format!("{name}: the document's digest peaks at {digest_share:.4} trees (bound {DIGEST_BOUND})"));
        }
        let (derive, validate) = ((peak + input_bytes) as f64 / tree, validate_peak as f64 / tree);
        eprintln!(
            "{name}: result text {text_bytes} B; document tree {tree} B = {:.1} x text; derive peak {} B = {derive:.3} T = {:.1} x text; validate peak {validate_peak} B = {validate:.3} T = {:.1} x text",
            tree / text_bytes,
            peak + input_bytes,
            (peak + input_bytes) as f64 / text_bytes,
            validate_peak as f64 / text_bytes,
        );
        if derive >= DERIVE_BOUND {
            over.push(format!("{name}: the derivation peaks at {derive:.3} trees (bound {DERIVE_BOUND})"));
        }
        if validate >= VALIDATE_BOUND {
            over.push(format!("{name}: the validation peaks at {validate:.3} trees (bound {VALIDATE_BOUND})"));
        }
        measured += 1;
    }
    assert_eq!(measured, DOCUMENTS.len());
    assert!(over.is_empty(), "{over:#?}");
}
