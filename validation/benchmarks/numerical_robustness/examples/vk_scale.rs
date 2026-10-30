//! `vk_scale`: one of V-K's scale runs per process (checkpoint B; plan §13;
//! brief Scope 12). `runner/vk_scale_runner.py` runs it in release, under K6's
//! runner, which provides the `/usr/bin/time` wrapper, the RSS watchdog and
//! the admission rule. It is observation only: it prints JSONL on stdout and
//! asserts no time or memory bound.
//!
//! Each phase prints one line:
//! - `start`: the case, the model file's sha256 against the committed one,
//!   and the heap cap;
//! - `counts`: W1's counts in O(nnz) with no solve, and the admission estimate
//!   (`envelope.rs`: complete conditional VR composition). **The backstop:** an estimate above
//!   half the heap cap is refused (exit 3), independently of the runner;
//! - `w1`: the case through `lane::run_case_with`, the same code the CI lane
//!   judges with, with `CaseLimit` and `InvocationMeter` at `u64::MAX`;
//! - `report`: the report's counts, the failures, the not-covered set against
//!   the committed list, and C9's second floor set, from S of the complete
//!   solution (`s_full`);
//! - `record`: the per-case record (`vk-case-record-v1`);
//! - `rcm`: K4's RCM against SD's on the model's free–free adjacency;
//! - `binary64`: the binary64 sparse gate's outcome class, sparse only (C4);
//! - `summary`: the heap peaks.
//!
//! Every phase line carries its elapsed time and its heap peaks (the in-place
//! and move models of K6's allocator). K6's runner records the process's load.
//!
//! Usage:
//! `vk_scale --case <id> [--model-file <path>] --heap-cap-bytes <n> [--counts-only]`,
//! or `vk_scale --noop --heap-cap-bytes <n>` for the no-op baseline run.
//!
//! Exit codes: 0 completed (an unresolved or refused outcome is an outcome,
//! not an error); 2 usage; 3 refused by the binary; 4 a check failed before
//! the solve (unknown case, model sha256, K4SRC, source refusal). A heap-cap
//! refusal aborts after the allocator's marker.
use open_pipe_stress_frame_kernel::structural::retained_api::PrimitiveSource;
use piping_numerical_robustness::cases::{load_family_described, load_large_model_described, Case};
use piping_numerical_robustness::envelope::{
    ArgumentFacts, PopulationPolicy, ReferenceKernelProfile, ReferenceVRProfile, VrEstimateContext,
    VrInvocation,
};
use piping_numerical_robustness::{floor, lane, parity, rcm, scale, sha256};
use serde_json::{json, Value};
use std::io::Write;
use std::time::Instant;

mod alloc {
    //! K6's counting, capped global allocator (`H/src/bin/k6_observe/alloc.rs`),
    //! as V-K uses it: `CURRENT` is the bytes requested and alive now. `PEAK`
    //! counts a growing `realloc` as its difference (the in-place model), and
    //! `PEAK_MOVE` counts the old and the new block together (the move model).
    //! The stage peaks restart at `stage_reset`. A request that would take
    //! `CURRENT` above `CAP` is refused: a fixed marker goes to fd 2 without
    //! allocating, and a null return makes Rust abort (SIGABRT).
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CURRENT: AtomicUsize = AtomicUsize::new(0);
    static PEAK: AtomicUsize = AtomicUsize::new(0);
    static PEAK_MOVE: AtomicUsize = AtomicUsize::new(0);
    static STAGE_PEAK: AtomicUsize = AtomicUsize::new(0);
    static STAGE_PEAK_MOVE: AtomicUsize = AtomicUsize::new(0);
    static CAP: AtomicUsize = AtomicUsize::new(usize::MAX);

    /// The marker `vk_scale_runner.py` recognizes as a heap-cap abort.
    pub const MARKER: &[u8] = b"vk_scale: heap cap refused ";

    pub struct VkAlloc;

    pub fn set_cap(bytes: usize) {
        CAP.store(bytes, Ordering::SeqCst);
    }
    pub fn cap() -> usize {
        CAP.load(Ordering::SeqCst)
    }
    pub fn current() -> usize {
        CURRENT.load(Ordering::SeqCst)
    }
    pub fn peak() -> usize {
        PEAK.load(Ordering::SeqCst)
    }
    pub fn peak_move() -> usize {
        PEAK_MOVE.load(Ordering::SeqCst)
    }
    pub fn stage_peak() -> usize {
        STAGE_PEAK.load(Ordering::SeqCst)
    }
    pub fn stage_peak_move() -> usize {
        STAGE_PEAK_MOVE.load(Ordering::SeqCst)
    }
    pub fn stage_reset() {
        let now = current();
        STAGE_PEAK.store(now, Ordering::SeqCst);
        STAGE_PEAK_MOVE.store(now, Ordering::SeqCst);
    }

    fn note_peak(level: usize) {
        PEAK.fetch_max(level, Ordering::SeqCst);
        STAGE_PEAK.fetch_max(level, Ordering::SeqCst);
        note_move(level);
    }

    fn note_move(level: usize) {
        PEAK_MOVE.fetch_max(level, Ordering::SeqCst);
        STAGE_PEAK_MOVE.fetch_max(level, Ordering::SeqCst);
    }

    fn reserve(bytes: usize) -> Option<usize> {
        let cap = cap();
        let mut now = CURRENT.load(Ordering::SeqCst);
        loop {
            let next = now.checked_add(bytes)?;
            if next > cap {
                return None;
            }
            match CURRENT.compare_exchange_weak(now, next, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return Some(next),
                Err(seen) => now = seen,
            }
        }
    }

    fn release(bytes: usize) {
        CURRENT.fetch_sub(bytes, Ordering::SeqCst);
    }

    fn put_usize(buf: &mut [u8], mut at: usize, value: usize) -> usize {
        let mut digits = [0u8; 20];
        let (mut n, mut len) = (value, 0);
        loop {
            digits[len] = b'0' + (n % 10) as u8;
            len += 1;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        for k in (0..len).rev() {
            if at < buf.len() {
                buf[at] = digits[k];
                at += 1;
            }
        }
        at
    }

    fn put_bytes(buf: &mut [u8], mut at: usize, bytes: &[u8]) -> usize {
        for &b in bytes {
            if at < buf.len() {
                buf[at] = b;
                at += 1;
            }
        }
        at
    }

    /// `vk_scale: heap cap refused <size> bytes (current <c>, cap <cap>)` to
    /// fd 2 from a stack buffer: no allocation.
    fn refuse(size: usize) {
        let mut buf = [0u8; 160];
        let mut at = put_bytes(&mut buf, 0, MARKER);
        at = put_usize(&mut buf, at, size);
        at = put_bytes(&mut buf, at, b" bytes (current ");
        at = put_usize(&mut buf, at, current());
        at = put_bytes(&mut buf, at, b", cap ");
        at = put_usize(&mut buf, at, cap());
        at = put_bytes(&mut buf, at, b")\n");
        write_stderr(&buf[..at]);
    }

    #[cfg(unix)]
    fn write_stderr(bytes: &[u8]) {
        use std::io::Write;
        use std::os::unix::io::FromRawFd;
        // SAFETY: fd 2 is the process's stderr; the `File` is forgotten, never
        // closed, and writing a byte slice does not allocate.
        let mut file = unsafe { std::fs::File::from_raw_fd(2) };
        let _ = file.write_all(bytes);
        std::mem::forget(file);
    }

    #[cfg(not(unix))]
    fn write_stderr(_bytes: &[u8]) {}

    unsafe impl GlobalAlloc for VkAlloc {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let size = layout.size();
            let Some(level) = reserve(size) else {
                refuse(size);
                return std::ptr::null_mut();
            };
            let ptr = System.alloc(layout);
            if ptr.is_null() {
                release(size);
            } else {
                note_peak(level);
            }
            ptr
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            let size = layout.size();
            let Some(level) = reserve(size) else {
                refuse(size);
                return std::ptr::null_mut();
            };
            let ptr = System.alloc_zeroed(layout);
            if ptr.is_null() {
                release(size);
            } else {
                note_peak(level);
            }
            ptr
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            System.dealloc(ptr, layout);
            release(layout.size());
        }

        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            let old = layout.size();
            if new_size > old {
                let Some(level) = reserve(new_size - old) else {
                    refuse(new_size - old);
                    return std::ptr::null_mut();
                };
                let moved = System.realloc(ptr, layout, new_size);
                if moved.is_null() {
                    release(new_size - old);
                } else {
                    note_peak(level);
                    note_move(level + old);
                }
                moved
            } else {
                let moved = System.realloc(ptr, layout, new_size);
                if !moved.is_null() {
                    release(old - new_size);
                }
                moved
            }
        }
    }
}

#[global_allocator]
static ALLOCATOR: alloc::VkAlloc = alloc::VkAlloc;

const SCHEMA: &str = "vk-scale-v1";

fn emit(mut line: Value) {
    line["schema"] = json!(SCHEMA);
    let mut out = std::io::stdout().lock();
    let _ = writeln!(out, "{line}");
    let _ = out.flush();
}

/// A phase's elapsed time and heap peaks, then a fresh stage.
struct Phase(Instant);

impl Phase {
    fn start() -> Self {
        alloc::stage_reset();
        Phase(Instant::now())
    }
    fn fields(&self) -> Value {
        json!({
            "elapsed_ns": self.0.elapsed().as_nanos() as u64,
            "heap_peak": alloc::stage_peak(),
            "heap_peak_move": alloc::stage_peak_move(),
            "heap_current": alloc::current(),
        })
    }
}

fn merge(mut a: Value, b: Value) -> Value {
    if let (Some(a), Value::Object(b)) = (a.as_object_mut(), b) {
        a.extend(b);
    }
    a
}

fn fail(code: i32, kind: &str, why: String) -> ! {
    emit(json!({ "kind": kind, "reason": why }));
    std::process::exit(code);
}

struct Args {
    case: Option<String>,
    model_file: Option<String>,
    heap_cap: Option<usize>,
    counts_only: bool,
    noop: bool,
    argument_facts: ArgumentFacts,
}

fn observed_next(
    it: &mut impl Iterator<Item = String>,
    facts: &mut ArgumentFacts,
) -> Option<String> {
    let value = it.next()?;
    facts.observe(&value);
    Some(value)
}

fn args() -> Args {
    let mut a = Args {
        case: None,
        model_file: None,
        heap_cap: None,
        counts_only: false,
        noop: false,
        argument_facts: ArgumentFacts::default(),
    };
    let mut it = std::env::args();
    let _ = observed_next(&mut it, &mut a.argument_facts);
    while let Some(arg) = observed_next(&mut it, &mut a.argument_facts) {
        match arg.as_str() {
            "--case" => a.case = observed_next(&mut it, &mut a.argument_facts),
            "--model-file" => a.model_file = observed_next(&mut it, &mut a.argument_facts),
            "--heap-cap-bytes" => {
                a.heap_cap =
                    observed_next(&mut it, &mut a.argument_facts).and_then(|v| v.parse().ok())
            }
            "--counts-only" => a.counts_only = true,
            "--noop" => a.noop = true,
            other => fail(2, "usage", format!("unknown argument {other:?}")),
        }
    }
    a
}

// The checked composition enforces the reference request width; keep the JSON
// conversion checked at the boundary as well. Errors use a bounded static reason.
fn estimate_bytes(value: u128) -> u64 {
    u64::try_from(value).unwrap_or_else(|_| fail(4, "error", "estimate byte width exceeded".into()))
}

/// S(kind) of the complete reference solution, in `floor::maxima`'s order.
fn full_maxima(case: &Case) -> Option<[f64; 4]> {
    let s = case.s_full.as_ref()?;
    let mut out = [0.0f64; 4];
    for (k, kind) in ["translation", "rotation", "force", "moment"]
        .iter()
        .enumerate()
    {
        let v: f64 = s.get(*kind)?.parse().ok()?;
        out[k] = v.abs();
    }
    Some(out)
}

fn main() {
    let a = args();
    let Some(cap) = a.heap_cap else {
        fail(2, "usage", "--heap-cap-bytes is required".into());
    };
    alloc::set_cap(cap);
    if a.noop {
        emit(json!({ "kind": "start", "noop": true, "heap_cap_bytes": cap }));
        emit(json!({
            "kind": "summary", "repeats_heap_peak": alloc::peak(),
            "repeats_heap_peak_move": alloc::peak_move(),
        }));
        return;
    }
    let Some(id) = a.case.clone() else {
        fail(2, "usage", "--case is required".into());
    };

    // ------------------------------------------------------------ load
    let phase = Phase::start();
    let (family_cases, family_facts) = load_family_described("RF-LARGE");
    let Some(case) = family_cases.into_iter().find(|c| c.id == id) else {
        fail(4, "error", format!("unknown case {id}"));
    };
    let (model, model_sha256, external_facts) = match (&case.model, &a.model_file) {
        (Some(m), None) => (m.clone(), None, None),
        (None, Some(path)) => {
            let (m, sha, facts) = load_large_model_described(std::path::Path::new(path));
            (m, Some(sha), Some(facts))
        }
        _ => fail(
            2,
            "usage",
            format!("{id}: --model-file is needed exactly when the case has no committed model"),
        ),
    };
    if let (Some(got), Some(want)) = (&model_sha256, &case.model_sha256) {
        if got != want {
            fail(
                4,
                "error",
                format!("{id}: model file sha256 {got}, committed {want}"),
            );
        }
    }
    emit(merge(
        json!({
            "kind": "start", "case": id, "members": model.members.len(),
            "heap_cap_bytes": cap, "model_sha256": model_sha256,
            "committed_model_sha256": case.model_sha256,
            "case_limit": u64::MAX, "invocation_limit": u64::MAX,
        }),
        phase.fields(),
    ));

    // ------------------------------------------------------------ counts
    let phase = Phase::start();
    let source = match PrimitiveSource::new(model.source_parts()) {
        Ok(s) => s,
        Err(e) => fail(4, "error", format!("{id}: source refused: {e:?}")),
    };
    let k4src = sha256::sha256_hex(&source.encoding());
    if k4src != case.k4src_sha256 {
        fail(
            4,
            "error",
            format!(
                "{id}: K4SRC sha256 {k4src}, the generator's {}",
                case.k4src_sha256
            ),
        );
    }
    let counts = scale::counts(&model, &source);
    let invocation = VrInvocation::actual(
        env!("CARGO_MANIFEST_DIR"),
        &id,
        a.model_file.as_deref(),
        a.argument_facts,
        a.counts_only,
    );
    let context = VrEstimateContext::capture(
        &case,
        &model,
        &source,
        &counts,
        family_facts,
        external_facts,
        invocation,
        PopulationPolicy::Exact {
            bodies: counts.bodies as u128,
            free_blocks: counts.blocks as u128,
        },
    )
    .unwrap_or_else(|_| fail(4, "error", "complete estimate context unavailable".into()));
    drop(source);
    let complete = scale::estimate(
        &context,
        &ReferenceKernelProfile::source40129_rust1971_aarch64_v1(),
        &ReferenceVRProfile::source40129_rust1971_aarch64_v1(),
    )
    .unwrap_or_else(|_| fail(4, "error", "complete estimate unavailable".into()));
    // Legacy fields use the complete moving metric. Reference arithmetic does
    // not qualify this executable; external artifact/input/launch binding remains.
    let est = complete.moving;
    emit(merge(
        json!({
            "kind": "counts", "case": id, "k4src_sha256": k4src,
            "nodes": counts.nodes, "members": counts.members, "springs": counts.springs,
            "stations": counts.stations, "constraints": counts.constraints,
            "loads": counts.loads, "dofs": counts.dofs, "free_dofs": counts.free_dofs,
            "bodies": counts.bodies, "pattern_entries": counts.pattern_entries,
            "profile_entries": counts.profile_entries, "half_bandwidth": counts.half_bandwidth,
            "blocks": counts.blocks, "rows": counts.rows,
            "source_encoding_len": counts.source_encoding_len,
            "estimate_max_bytes": estimate_bytes(est.max), "estimate_sel128_bytes": estimate_bytes(est.sel128),
            "estimate_fixed_bytes": estimate_bytes(est.fixed), "estimate_model_bytes": estimate_bytes(est.model),
            "estimate_decide_bytes": estimate_bytes(est.decide),
        }),
        phase.fields(),
    ));
    if a.counts_only {
        emit(json!({
            "kind": "summary", "case": id, "counts_only": true,
            "repeats_heap_peak": alloc::peak(), "repeats_heap_peak_move": alloc::peak_move(),
        }));
        return;
    }
    if est.max > (cap / 2) as u128 {
        emit(json!({
            "kind": "refusal", "case": id, "reason": "estimate_exceeds_half_cap",
            "estimate_max_bytes": estimate_bytes(est.max), "half_cap_bytes": cap / 2,
        }));
        std::process::exit(3);
    }

    // ------------------------------------------------------------ W1
    let phase = Phase::start();
    let mut run = lane::run_case_with(&case, &model);
    let w1 = phase.fields();
    let record = run.record.clone().unwrap_or_else(|| {
        eprintln!("{}: no exact work record: {:?}", run.id, run.failures);
        std::process::exit(1)
    });
    let storage = &record["attempts"][0]["storage"];
    let counts_match_storage = storage["pattern_entries"].as_u64()
        == Some(counts.pattern_entries as u64)
        && storage["profile_entries"].as_u64() == Some(counts.profile_entries as u64);
    emit(merge(
        json!({
            "kind": "w1", "case": id, "outcome": record["outcome"],
            "selected_precision": record["selected_precision"],
            "verification_precision": record["verification_precision"],
            "attempts": record["attempts"].as_array().map(Vec::len),
            "corrections": record["corrections"],
            "invocation_charged": record["invocation_charged"],
            "published_rows": record["published_rows"],
            "counts_match_storage": counts_match_storage,
        }),
        w1,
    ));

    // ------------------------------------------------------------ report
    let phase = Phase::start();
    let not_covered_full = full_maxima(&case).map(|m| {
        let scales = floor::scales_from(m, &model);
        floor::not_covered(&case, &model, &scales)
    });
    let t = &run.tally;
    emit(merge(
        json!({
            "kind": "report", "case": id, "rows": t.rows, "pass": t.pass,
            "pass_absolute_range": t.pass_absolute_range, "not_covered": t.not_covered,
            "structural_zero": t.structural_zero, "expected_unresolved": t.expected_unresolved,
            "fail": t.fail, "accounted": t.accounted(),
            "failures": run.failures.len(),
            "failures_head": run.failures.iter().take(50).collect::<Vec<_>>(),
            "not_covered_rows": run.not_covered,
            "not_covered_committed": case.not_covered,
            "not_covered_equal": run.not_covered == case.not_covered,
            "not_covered_rows_s_full": not_covered_full,
            "class_mismatches": run.class_mismatches,
            "controls_discriminated": run.controls.discriminated,
            "controls_non_discriminating": run.controls.non_discriminating,
            "controls_undiscriminated": run.controls.undiscriminated,
            "controls_unexpectedly_failing": run.controls.unexpectedly_failing,
        }),
        phase.fields(),
    ));
    emit(json!({ "kind": "record", "case": id, "record": record }));
    run.published.clear();
    drop(run);

    // ------------------------------------------------------------ RCM
    let phase = Phase::start();
    let adjacency = rcm::free_adjacency(&model);
    let (k4, sd) = rcm::both_orders(&adjacency);
    let equal = k4 == sd;
    drop((adjacency, k4, sd));
    emit(merge(
        json!({ "kind": "rcm", "case": id, "equal": equal }),
        phase.fields(),
    ));

    // ------------------------------------------------------------ binary64
    let phase = Phase::start();
    let b64 = parity::parity(&model, false);
    emit(merge(
        match b64 {
            Ok(p) => json!({ "kind": "binary64", "case": id, "sparse": p.sparse }),
            Err(e) => json!({ "kind": "binary64", "case": id, "error": e }),
        },
        phase.fields(),
    ));

    emit(json!({
        "kind": "summary", "case": id,
        "repeats_heap_peak": alloc::peak(), "repeats_heap_peak_move": alloc::peak_move(),
    }));
}
