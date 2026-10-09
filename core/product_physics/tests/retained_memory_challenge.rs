//! U4 G5 part 2, re-qualified by B1 SQ (PLAN_v2 §3.5, with RV107's A1-S-1 and A1-N-9): the
//! allocation challenge. A challenge only, not a proof and not a production guard.
//!
//! This binary's own global allocator records the peak of live requested heap bytes (the
//! f1b_sparse_pattern_memory.rs pattern), like-for-like with E_mov,max. Each entry point runs one
//! input in one mode, through one route, and is meant to run as its own process
//! (`<binary> <name> --exact --ignored --test-threads=1 --nocapture`; RSS_TIME.md's runs):
//! - `direct_*`: the public retained Direct entry (G-A's census and law, the parse, the ordinary
//!   run, and the W1 phases when the registered build permits the call);
//! - `ordinary_*`: the ordinary value route alone (the control for W1's increment);
//! - `process_floor`: no product call.
//!
//! The bound (A1-S-1): a run that did W1 work, which the public surface shows as a successor or an
//! N1 notice, is bounded by E_mov,max, the maximum over every phase; any other run (no permit,
//! `NoTriggeredCase`, or the ordinary route) by the in-build W1 phase. Both without R. The
//! furthest W1 phase a run reached is not visible here (`W1Fallback` is `pub(crate)`); it comes
//! from the witness-driver run of the same input, mode and build (`witness_tests`), and RSS_TIME.md
//! records it beside this peak. The bounds below are the generated profile's values; a lib test
//! (`challenge_bounds_are_the_profile`) reads this file and checks they equal the in-build
//! evaluation. The one default test runs the cheap inputs (milestone, W-C2) in both modes.
//!
//! The counting allocator does SeqCst atomics on every allocation, which slows these dev/test
//! timings; release times come from the witness tests, without it (A1-N-9).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, run_linear_static_preview_value_with_retained_direct, PreviewSolverMode};
use serde_json::Value;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

#[path = "common/b1_sq_inputs.rs"]
mod b1_sq_inputs;
use b1_sq_inputs as inputs;

/// The profile's in-build W1 phase (requested + moving, without R), sparse and dense, in the
/// pinned record's build (the lib test `challenge_bounds_are_the_profile` checks it there).
const W1_PHASE_BYTES: [u64; 2] = [5_069_320_590, 5_128_451_934];
/// The profile's in-build maximum over every phase (E_mov,max, without R; W3 in both modes),
/// the bound once a run has done W1 work (a registered build, G6).
const MAX_PHASE_BYTES: [u64; 2] = [9_733_566_502, 9_792_697_846];
/// The abort cap: above E_mov,max + R at M and within T3's host allowance (PLAN_v2 §3.5).
const CAP_BYTES: usize = 16 << 30;
/// The N1 notice's fixed text (lib.rs `RETAINED_UNAVAILABLE_NOTICE`), as the base publication shows it.
const N1_NOTICE: &str = "Retained-precision recovery is unavailable for this load case.";

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
/// One measured span at a time in this process.
static SPAN: Mutex<()> = Mutex::new(());
struct Counting;
impl Counting {
    fn grow(size: usize) {
        let now = CURRENT.fetch_add(size, Ordering::SeqCst) + size;
        if now > CAP_BYTES {
            std::process::abort();
        }
        PEAK.fetch_max(now, Ordering::SeqCst);
    }
}
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Self::grow(layout.size());
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        Self::grow(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        CURRENT.fetch_sub(layout.size(), Ordering::SeqCst);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            Self::grow(new_size - layout.size());
        } else {
            CURRENT.fetch_sub(layout.size() - new_size, Ordering::SeqCst);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}
#[global_allocator]
static GLOBAL: Counting = Counting;

#[derive(Clone, Copy, Debug)]
enum Route {
    Direct,
    Ordinary,
}
const SPARSE: PreviewSolverMode = PreviewSolverMode::SparseInteractive;
const DENSE: PreviewSolverMode = PreviewSolverMode::DenseScrutiny;

/// One measured run: the input is built before the span; the peak is the span's live requested
/// bytes above its start. Prints the outcome, the peak, its bound and the call's wall time.
fn measure(label: &str, raw: Value, mode: PreviewSolverMode, route: Route) {
    let _span = SPAN.lock().unwrap_or_else(|poison| poison.into_inner());
    let m = match mode {
        PreviewSolverMode::SparseInteractive => 0,
        _ => 1,
    };
    let base = CURRENT.load(Ordering::SeqCst);
    PEAK.store(base, Ordering::SeqCst);
    let start = std::time::Instant::now();
    let (outcome, w1_work, envelope_rows) = match route {
        Route::Direct => {
            let output = run_linear_static_preview_value_with_retained_direct(raw, mode).unwrap();
            let elapsed = start.elapsed();
            let notices = output.envelope().diagnostics.iter().filter(|d| d.message.starts_with(N1_NOTICE)).count();
            let successor = output.successor().map(|s| serde_json::to_vec(s).unwrap().len());
            let rows = output.envelope().results.len();
            drop(output);
            println!("I104_SQ_CHALLENGE_TIME {label} {mode:?} {route:?} call_ms={:.1}", elapsed.as_secs_f64() * 1e3);
            let outcome = match (successor, notices) {
                (Some(bytes), 0) => format!("successor({bytes} B)"),
                (Some(bytes), n) => format!("successor({bytes} B)+notices({n})"),
                (None, 0) => "no_w1_work".to_owned(),
                (None, n) => format!("notices({n})"),
            };
            (outcome, successor.is_some() || notices > 0, rows)
        }
        Route::Ordinary => {
            let envelope = run_linear_static_preview_value_with_mode(raw, mode).unwrap();
            let elapsed = start.elapsed();
            let rows = envelope.results.len();
            drop(envelope);
            println!("I104_SQ_CHALLENGE_TIME {label} {mode:?} {route:?} call_ms={:.1}", elapsed.as_secs_f64() * 1e3);
            ("ordinary_route".to_owned(), false, rows)
        }
    };
    let peak = (PEAK.load(Ordering::SeqCst) - base) as u64;
    let bound = if w1_work { MAX_PHASE_BYTES[m] } else { W1_PHASE_BYTES[m] };
    println!("I104_SQ_CHALLENGE {label} {mode:?} {route:?} outcome={outcome} rows={envelope_rows} peak_bytes={peak} bound={} bound_bytes={bound} ratio={:.6}",
        if w1_work { "E_mov_max" } else { "W1_phase" }, peak as f64 / bound as f64);
    assert!(peak <= bound, "{label} {mode:?} {route:?}: measured peak {peak} above the profile's bound {bound}");
}

/// The default suite's challenge: the milestone (its peak still within bound) and W-C2, both modes,
/// through the Direct entry, in one process.
#[test]
fn retained_direct_peak_is_within_the_profiles_bounds() {
    for (label, raw) in [("milestone", inputs::milestone()), ("w_c2", inputs::w_c2())] {
        for mode in [SPARSE, DENSE] {
            measure(label, raw.clone(), mode, Route::Direct);
        }
    }
}

/// The process floor: no product call.
#[test]
#[ignore]
fn process_floor() {
    let _span = SPAN.lock().unwrap_or_else(|poison| poison.into_inner());
    println!("I104_SQ_CHALLENGE process_floor live_bytes={}", CURRENT.load(Ordering::SeqCst));
}

macro_rules! entries {
    ($($name:ident: $input:expr, $label:literal, [$($mode:ident => $value:expr),+];)+) => {
        $(
            mod $name {
                use super::*;
                $(
                    mod $mode {
                        use super::*;
                        #[test]
                        #[ignore]
                        fn direct() {
                            measure($label, $input, $value, Route::Direct);
                        }
                        #[test]
                        #[ignore]
                        fn ordinary() {
                            measure($label, $input, $value, Route::Ordinary);
                        }
                    }
                )+
            }
        )+
    };
}
// One entry point per input, mode and route (A1-N-9), e.g. `c1::dense::direct`. The inputs are the
// witnesses' own (`witness_tests`), so each run's furthest phase is the witness's outcome.
entries! {
    milestone: inputs::milestone(), "milestone", [sparse => SPARSE, dense => DENSE];
    w_c2: inputs::w_c2(), "w_c2", [sparse => SPARSE, dense => DENSE];
    w_c2_ac: inputs::w_c2_ac(), "w_c2_ac", [sparse => SPARSE];
    w2: inputs::w2(), "w2_cap_maximal", [sparse => SPARSE, dense => DENSE];
    b2_k1e3: inputs::b2_k1e3(), "b2_k1e3", [sparse => SPARSE, dense => DENSE];
    c1: inputs::c1(), "c1", [sparse => SPARSE, dense => DENSE];
    i3_three_case: inputs::i3_three_case(), "i3_three_case", [sparse => SPARSE, dense => DENSE];
}
