//! DEL-04-07 REQ-008, part 1 (evidence only): the sparse active-set loop's heap,
//! measured by a counting global allocator over a model family.
//!
//! **Translated input: same counts, not the product's input; a cross-check.** The
//! witness on the product's own loop input is the product crate's in-crate
//! `src/sparse_loop_memory_tests.rs` (level "product-loop"); this binary translates
//! each document itself and keeps the product's topology and counts, not its loads.
//!
//! This binary asserts no memory bound. For each model it measures, in one
//! process and one test at a time, the peak live heap over one boundary that
//! includes:
//! - constructing the caller-held `NonlinearFrameSolveInput` from the translated
//!   model, as the product's `nonlinear_frame_input` does (clones of the elements,
//!   connectors and supports, each curved slot through
//!   `CurvedBendStiffnessElement::from_macro_element`, the force copy);
//! - the call `solve_active_set_frame_with_mode_and_springs_assembled` in
//!   `SparseInteractive`, as the product makes it;
//! - the returned result, held, with the exit receipt's clone inside it.
//!
//! It also reports the peak inside the call alone, and the counts a bound needs:
//! n, f0, the base-restrained DOFs, z and c (`SparseAssemblyEvidence::storage_counts`
//! of the loop's own assembly, re-formed after the boundary), z_f0 (the stored
//! entries whose row and column are both base-free), k, the derived-normal
//! friction supports, the iterations and each iterate's profile counts.
//!
//! Heap figures are requested bytes (`Layout::size`) and, on macOS, the
//! allocator's `malloc_size` of each block. Peaks are increments over the live
//! heap at the boundary's start. A realloc moves live bytes by its size change;
//! `process_peak_requested_moving_realloc` also counts a moved block's old and new
//! copies together for the instant of the copy. The counters are static atomics and a
//! const thread-local tally: their bookkeeping allocates nothing.
//!
//! Counters: the loop runs on the measuring thread, so the figures are that thread's own
//! tally of requested bytes (fields prefixed `thread_`), which the self-tests check exactly.
//! The process-wide counters (prefix `process_`: rounded bytes, the moving-realloc peak,
//! event counts) are diagnostics; every record says so in its `counters` field. Each
//! boundary's peaks and counters are read before any report is formed, and the report's
//! own allocations are counted apart.
//!
//! Models: the documents written by `sparse_loop_memory/generate_models.py`,
//! translated here (the product's model build is private to the product crate),
//! and one family generated in Rust (connectors with tie rods). The translation
//! keeps the product's topology, node order, restraints, springs, supports and
//! states, so n, f0, z, c, k and the profiles are the product's; its loads are
//! lumped (weight and thermal axial ends; pressure is not applied), so the
//! iterates' states and counts can differ from the product's solve of the same
//! document. The product crate's `sparse_admission_memory_twin.rs` measures the
//! whole product call on the same documents.
//!
//! Run (release, one test at a time; the counters are global):
//! `SWBPIPE_SPARSE_MEMORY_MODELS=<dir> SWBPIPE_SPARSE_MEMORY_OUT=<file.jsonl>
//!  cargo test --release --test sparse_loop_memory -- --test-threads 1 --nocapture`.
//! Without `SWBPIPE_SPARSE_MEMORY_MODELS` the model tests return at once. While
//! the file named by `SWBPIPE_TIMING_WINDOW_FILE` exists, no model is solved.
use open_pipe_stress_curved_bend::CurvedBendMacroElement;
use open_pipe_stress_frame_kernel::connector::{
    ConnectorAttachment, ObjectiveConnector, ScaledWorkMatrix,
};
use open_pipe_stress_frame_kernel::load_ledger::{AssembledForce, LoadLedger};
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, SparseAssemblyOptions, StiffnessBlock,
};
use open_pipe_stress_frame_kernel::tie_rod::TieRodInput;
use open_pipe_stress_frame_kernel::{
    FrameDof, FrameElement, FrameNode, FrameSection, DOF_PER_NODE,
};
use open_pipe_stress_nonlinear_integration::structural_adapter::{
    SparseAssemblyEvidence, StrictGapEvidence,
};
use open_pipe_stress_nonlinear_integration::{
    dense_loop_memory_estimate, solve_active_set_frame_with_mode_and_springs_assembled,
    ConvergenceControl, ConvergencePolicyStatus, CurvedBendStiffnessElement,
    DerivedFrictionNormalReaction, FrictionNormalReaction, LinearSolveMode,
    NonlinearFrameSolveInput, NonlinearFrameSolveResult, NonlinearIntegrationError,
    DENSE_LOOP_MEMORY_CEILING_BYTES,
};
use open_pipe_stress_nonlinear_supports::{
    ActivationSense, ActiveSetState, GapDirection, NonlinearSupport, SupportStateRecord,
};
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// The counting allocator.

/// A safety net, not a measurement: a run whose requested live heap passes it
/// aborts instead of exhausting the host.
const CAP_BYTES: usize = 16 << 30;

static LIVE_REQ: AtomicUsize = AtomicUsize::new(0);
static LIVE_RND: AtomicUsize = AtomicUsize::new(0);
/// Peaks: [outer boundary, inner call].
static PEAK_REQ: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)];
static PEAK_RND: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)];
static PEAK_MOVE: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)];
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static DEALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);
static MOVED_REALLOCS: AtomicUsize = AtomicUsize::new(0);
static SERIAL: Mutex<()> = Mutex::new(());

#[cfg(target_os = "macos")]
extern "C" {
    fn malloc_size(ptr: *const std::ffi::c_void) -> usize;
}

/// The allocator's usable size of a live block (macOS), else the request.
fn rounded(ptr: *mut u8, requested: usize) -> usize {
    #[cfg(target_os = "macos")]
    {
        let _ = requested;
        unsafe { malloc_size(ptr as *const std::ffi::c_void) }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = ptr;
        requested
    }
}

// The calling thread's own requested live bytes and its peaks (outer, inner): the measuring
// thread's share, isolated from every other thread's allocations. Const-initialised and
// without a destructor, so the allocator may read it at any time.
thread_local! {
    static THREAD_LIVE: std::cell::Cell<isize> = const { std::cell::Cell::new(0) };
    static THREAD_PEAK: [std::cell::Cell<isize>; 2] = const { [std::cell::Cell::new(0), std::cell::Cell::new(0)] };
}

fn thread_add(delta: isize) {
    let _ = THREAD_LIVE.try_with(|live| {
        let now = live.get() + delta;
        live.set(now);
        let _ = THREAD_PEAK.try_with(|peaks| {
            for peak in peaks {
                if now > peak.get() {
                    peak.set(now);
                }
            }
        });
    });
}

fn thread_live() -> isize {
    THREAD_LIVE.try_with(|live| live.get()).unwrap_or(0)
}

fn thread_peak(slot: usize) -> isize {
    THREAD_PEAK.try_with(|peaks| peaks[slot].get()).unwrap_or(0)
}

fn raise(peaks: &[AtomicUsize; 2], value: usize) {
    peaks[0].fetch_max(value, SeqCst);
    peaks[1].fetch_max(value, SeqCst);
}

fn grow(requested: usize, rnd: usize) {
    let live = LIVE_REQ.fetch_add(requested, SeqCst) + requested;
    if live > CAP_BYTES {
        std::process::abort();
    }
    raise(&PEAK_REQ, live);
    raise(&PEAK_MOVE, live);
    let live_rnd = LIVE_RND.fetch_add(rnd, SeqCst) + rnd;
    raise(&PEAK_RND, live_rnd);
    thread_add(requested as isize);
}

fn shrink(requested: usize, rnd: usize) {
    LIVE_REQ.fetch_sub(requested, SeqCst);
    LIVE_RND.fetch_sub(rnd, SeqCst);
    thread_add(-(requested as isize));
}


struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            ALLOCS.fetch_add(1, SeqCst);
            grow(layout.size(), rounded(ptr, layout.size()));
        }
        ptr
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            ALLOCS.fetch_add(1, SeqCst);
            grow(layout.size(), rounded(ptr, layout.size()));
        }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let rnd = rounded(ptr, layout.size());
        unsafe { System.dealloc(ptr, layout) };
        DEALLOCS.fetch_add(1, SeqCst);
        shrink(layout.size(), rnd);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_rnd = rounded(ptr, layout.size());
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
        if new_ptr.is_null() {
            return new_ptr;
        }
        REALLOCS.fetch_add(1, SeqCst);
        if new_ptr != ptr {
            // For the instant of the copy both blocks were live.
            MOVED_REALLOCS.fetch_add(1, SeqCst);
            raise(&PEAK_MOVE, LIVE_REQ.load(SeqCst) + new_size);
        }
        let new_rnd = rounded(new_ptr, new_size);
        shrink(layout.size(), old_rnd);
        grow(new_size, new_rnd);
        new_ptr
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[derive(Debug, Clone, Copy)]
struct Snapshot {
    live_req: usize,
    live_rnd: usize,
    allocs: usize,
    deallocs: usize,
    reallocs: usize,
    moved: usize,
    thread_live: isize,
}

fn snapshot() -> Snapshot {
    Snapshot {
        live_req: LIVE_REQ.load(SeqCst),
        live_rnd: LIVE_RND.load(SeqCst),
        allocs: ALLOCS.load(SeqCst),
        deallocs: DEALLOCS.load(SeqCst),
        reallocs: REALLOCS.load(SeqCst),
        moved: MOVED_REALLOCS.load(SeqCst),
        thread_live: thread_live(),
    }
}

/// Resets the peaks of `slot` (0 outer, 1 inner) to the live heap now.
fn reset_peaks(slot: usize) -> Snapshot {
    let now = snapshot();
    PEAK_REQ[slot].store(now.live_req, SeqCst);
    PEAK_MOVE[slot].store(now.live_req, SeqCst);
    PEAK_RND[slot].store(now.live_rnd, SeqCst);
    let _ = THREAD_PEAK.try_with(|peaks| peaks[slot].set(thread_live()));
    snapshot()
}

/// A boundary's end: every counter and peak of `slot`, read at once and without
/// allocating, before any report is formed.
#[derive(Debug, Clone, Copy)]
struct Endpoint {
    at: Snapshot,
    peak_req: usize,
    peak_rnd: usize,
    peak_move: usize,
    thread_peak: isize,
}

fn endpoint(slot: usize) -> Endpoint {
    Endpoint {
        thread_peak: thread_peak(slot),
        peak_req: PEAK_REQ[slot].load(SeqCst),
        peak_rnd: PEAK_RND[slot].load(SeqCst),
        peak_move: PEAK_MOVE[slot].load(SeqCst),
        at: snapshot(),
    }
}

/// What the field prefixes of each record name, carried in every record.
const COUNTERS: &str = "\"counters\":{\"thread_\":\"the measuring thread's own tally of requested bytes (Layout::size); exactly self-tested; authoritative\",\"process_\":\"process-wide counters shared by every thread: diagnostics only, not exactly self-tested (the self-test bounds the requested peak within 64 KiB); rounded bytes are malloc_size of each block, the moving-realloc peak adds each moved block's new size to the live total, and event counts include any thread's events\"}";

/// One boundary's figures, increments over its start.
fn measured(start: Snapshot, end: Endpoint) -> String {
    format!(
        "{{\"thread_peak_requested\":{},\"thread_live_requested_at_end\":{},\"process_peak_requested\":{},\"process_peak_rounded\":{},\"process_peak_requested_moving_realloc\":{},\"process_live_requested_at_end\":{},\"process_live_rounded_at_end\":{},\"process_allocations\":{},\"process_deallocations\":{},\"process_reallocations\":{},\"process_moved_reallocations\":{}}}",
        end.thread_peak - start.thread_live,
        end.at.thread_live - start.thread_live,
        end.peak_req - start.live_req,
        end.peak_rnd - start.live_rnd,
        end.peak_move - start.live_req,
        end.at.live_req as i128 - start.live_req as i128,
        end.at.live_rnd as i128 - start.live_rnd as i128,
        end.at.allocs - start.allocs,
        end.at.deallocs - start.deallocs,
        end.at.reallocs - start.reallocs,
        end.at.moved - start.moved,
    )
}

// ---------------------------------------------------------------------------
// A small JSON reader (this crate has no JSON dependency).

#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(#[allow(dead_code)] bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(members) => members.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    fn at(&self, key: &str) -> &Json {
        self.get(key).unwrap_or_else(|| panic!("missing key {key}"))
    }
    fn str(&self) -> &str {
        match self {
            Json::Str(s) => s,
            other => panic!("not a string: {other:?}"),
        }
    }
    fn num(&self) -> f64 {
        match self {
            Json::Num(v) => *v,
            other => panic!("not a number: {other:?}"),
        }
    }
    fn arr(&self) -> &[Json] {
        match self {
            Json::Arr(v) => v,
            other => panic!("not an array: {other:?}"),
        }
    }
    fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn parse(text: &[u8]) -> Json {
        let mut reader = Reader { bytes: text, at: 0 };
        let value = reader.value();
        reader.ws();
        assert_eq!(reader.at, reader.bytes.len(), "trailing JSON");
        value
    }
    fn ws(&mut self) {
        while self.at < self.bytes.len() && self.bytes[self.at].is_ascii_whitespace() {
            self.at += 1;
        }
    }
    fn eat(&mut self, byte: u8) {
        self.ws();
        assert_eq!(self.bytes[self.at], byte, "JSON at {}", self.at);
        self.at += 1;
    }
    fn value(&mut self) -> Json {
        self.ws();
        match self.bytes[self.at] {
            b'{' => {
                self.at += 1;
                let mut members = Vec::new();
                self.ws();
                if self.bytes[self.at] == b'}' {
                    self.at += 1;
                    return Json::Obj(members);
                }
                loop {
                    self.ws();
                    let key = self.string();
                    self.eat(b':');
                    members.push((key, self.value()));
                    self.ws();
                    let next = self.bytes[self.at];
                    self.at += 1;
                    if next == b'}' {
                        return Json::Obj(members);
                    }
                    assert_eq!(next, b',');
                }
            }
            b'[' => {
                self.at += 1;
                let mut items = Vec::new();
                self.ws();
                if self.bytes[self.at] == b']' {
                    self.at += 1;
                    return Json::Arr(items);
                }
                loop {
                    items.push(self.value());
                    self.ws();
                    let next = self.bytes[self.at];
                    self.at += 1;
                    if next == b']' {
                        return Json::Arr(items);
                    }
                    assert_eq!(next, b',');
                }
            }
            b'"' => Json::Str(self.string()),
            b't' => {
                self.at += 4;
                Json::Bool(true)
            }
            b'f' => {
                self.at += 5;
                Json::Bool(false)
            }
            b'n' => {
                self.at += 4;
                Json::Null
            }
            _ => {
                let start = self.at;
                while self.at < self.bytes.len()
                    && matches!(self.bytes[self.at], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                {
                    self.at += 1;
                }
                let text = std::str::from_utf8(&self.bytes[start..self.at]).unwrap();
                Json::Num(text.parse().unwrap_or_else(|_| panic!("number {text}")))
            }
        }
    }
    fn string(&mut self) -> String {
        assert_eq!(self.bytes[self.at], b'"');
        self.at += 1;
        let mut out = Vec::new();
        loop {
            let byte = self.bytes[self.at];
            self.at += 1;
            match byte {
                b'"' => return String::from_utf8(out).unwrap(),
                b'\\' => {
                    let escape = self.bytes[self.at];
                    self.at += 1;
                    match escape {
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'u' => {
                            let hex = std::str::from_utf8(&self.bytes[self.at..self.at + 4]).unwrap();
                            self.at += 4;
                            let code = u32::from_str_radix(hex, 16).unwrap();
                            let ch = char::from_u32(code).unwrap_or('\u{fffd}');
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        other => out.push(other),
                    }
                }
                other => out.push(other),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SHA-256 (FIPS 180-4), for the model hashes.

fn sha256(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let mut message = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bits.to_be_bytes());
    for block in message.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([block[4 * i], block[4 * i + 1], block[4 * i + 2], block[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    h.iter().map(|word| format!("{word:08x}")).collect()
}

// ---------------------------------------------------------------------------
// The translated model: what the product's build holds, from which the
// caller-held input is formed inside the boundary.

struct Model {
    node_count: usize,
    elements: Vec<FrameElement>,
    connectors: Vec<ObjectiveConnector>,
    curved: Vec<(String, CurvedBendMacroElement)>,
    restrained: Vec<usize>,
    springs: Vec<(usize, f64)>,
    supports: Vec<NonlinearSupport>,
    states: Vec<SupportStateRecord>,
    friction_normals: Vec<FrictionNormalReaction>,
    derived: Vec<DerivedFrictionNormalReaction>,
    force: AssembledForce,
    /// Primitive loads the translation did not apply (pressure, others).
    ignored_loads: usize,
}

fn dof_index(token: &str) -> FrameDof {
    match token {
        "UX" => FrameDof::Ux,
        "UY" => FrameDof::Uy,
        "UZ" => FrameDof::Uz,
        "RX" => FrameDof::Rx,
        "RY" => FrameDof::Ry,
        "RZ" => FrameDof::Rz,
        other => panic!("dof {other}"),
    }
}

fn dof_offset(token: &str) -> usize {
    ["UX", "UY", "UZ", "RX", "RY", "RZ"]
        .iter()
        .position(|t| *t == token)
        .unwrap_or_else(|| panic!("dof {token}"))
}

fn length_m(q: &Json) -> f64 {
    let v = q.at("value").num();
    match q.at("unit").str() {
        "m" => v,
        "mm" => v / 1000.0,
        other => panic!("length unit {other}"),
    }
}

fn modulus_pa(q: &Json) -> f64 {
    let v = q.at("value").num();
    match q.at("unit").str() {
        "Pa" => v,
        "MPa" => v * 1e6,
        "GPa" => v * 1e9,
        other => panic!("modulus unit {other}"),
    }
}

fn force_n(q: &Json) -> f64 {
    let v = q.at("value").num();
    match q.at("unit").str() {
        "N" | "N/m" | "N*m" | "N*m/rad" => v,
        "kN" | "kN/m" | "kN*m" => v * 1000.0,
        other => panic!("force unit {other}"),
    }
}

fn unit(v: [f64; 3]) -> [f64; 3] {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / n, v[1] / n, v[2] / n]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// What a thermal load reads of a member: unscaled E, metal area and alpha.
struct Section {
    e: f64,
    area: f64,
    alpha: f64,
}

/// The product document translated into what the loop consumes. Panics on a
/// document this translation does not cover (it is a measurement fixture).
fn translate(request: &Json) -> Model {
    let model = request.at("model");
    let exact = model.get("pressure_contract").is_some_and(|c| !c.is_null());
    let mut node_map = HashMap::new();
    let mut nodes = Vec::new();
    for (index, node) in model.at("nodes").arr().iter().enumerate() {
        node_map.insert(node.at("id").str().to_string(), index);
        let p = node.at("position");
        nodes.push(FrameNode::new(index, [p.at("x").num(), p.at("y").num(), p.at("z").num()]).unwrap());
    }
    let n = nodes.len() * DOF_PER_NODE;
    let mut materials = HashMap::new();
    for material in model.at("materials").arr() {
        let e = modulus_pa(material.at("elastic_modulus"));
        let g = match material.get("shear_modulus") {
            Some(g) => modulus_pa(g),
            None => e / (2.0 * (1.0 + material.at("poisson_ratio").at("value").num())),
        };
        let alpha = material
            .get("thermal_expansion_coefficient")
            .or_else(|| material.get("temperature_points").map(|p| p.arr()[0].at("thermal_expansion_coefficient")))
            .map_or(0.0, |a| a.at("value").num());
        materials.insert(material.at("id").str().to_string(), (e, g, alpha));
    }
    let components = model.get("components").map_or(&[][..], |c| c.arr());
    let mut bends = HashMap::new();
    let mut scales = HashMap::new();
    for component in components {
        let Some(geometry) = component.get("geometry") else { continue };
        if let Some(pipe) = geometry.get("bend_pipe_ref") {
            let factor = component.at("modifiers").at("flexibility_factor_user_value").at("value").num();
            bends.insert(pipe.str().to_string(), (component.at("id").str().to_string(), length_m(geometry.at("bend_radius")), factor));
        }
        if let (Some(pipe), Some(scale)) = (
            geometry.get("rigid_pipe_ref"),
            component.get("modifiers").and_then(|m| m.get("stiffness_scaling_user_value")),
        ) {
            scales.insert(pipe.str().to_string(), scale.at("value").num());
        }
    }
    let mut elements = Vec::new();
    let mut curved = Vec::new();
    let mut pipe_ends = HashMap::new();
    let mut sections = HashMap::new();
    for pipe in model.at("pipe_segments").arr() {
        let id = pipe.at("id").str().to_string();
        let (i, j) = (node_map[pipe.at("from").str()], node_map[pipe.at("to").str()]);
        let (e, g, alpha) = materials[pipe.at("material").str()];
        let s = pipe.at("section");
        let od = length_m(s.at("outside_diameter"));
        let nominal = length_m(s.at("wall_thickness"));
        let wall = match (exact, s.get("mill_tolerance")) {
            (false, Some(m)) => nominal - length_m(m),
            _ => nominal,
        };
        let inner = od - 2.0 * wall;
        let area = std::f64::consts::PI * (od.powi(2) - inner.powi(2)) / 4.0;
        let second_moment = std::f64::consts::PI * (od.powi(4) - inner.powi(4)) / 64.0;
        let scale = scales.get(&id).copied().unwrap_or(1.0);
        let y = pipe.at("y_reference");
        let y_ref = [y.at("x").num(), y.at("y").num(), y.at("z").num()];
        if let Some((component, radius, factor)) = bends.get(&id) {
            let element = CurvedBendMacroElement::new(
                nodes[i], nodes[j], *radius, y_ref, e * scale, g * scale, area, second_moment,
                2.0 * second_moment, *factor, *factor,
            )
            .unwrap_or_else(|error| panic!("bend {component}: {error:?}"));
            curved.push((component.clone(), element));
        } else {
            let section = FrameSection::new(e * scale, g * scale, area, second_moment, second_moment, 2.0 * second_moment).unwrap();
            elements.push(FrameElement::new(nodes[i], nodes[j], section, y_ref).unwrap());
        }
        pipe_ends.insert(id.clone(), (i, j));
        sections.insert(id, Section { e, area, alpha });
    }

    let supports_json = model.at("supports").arr();
    let by_id = supports_json
        .iter()
        .map(|s| (s.at("id").str().to_string(), s))
        .collect::<HashMap<_, _>>();
    let mut restrained = BTreeSet::new();
    let mut springs = Vec::new();
    let mut supports = Vec::new();
    let mut states = Vec::new();
    let mut friction_normals = Vec::new();
    let mut derived = Vec::new();
    for support in supports_json {
        let id = support.at("id").str();
        let node = node_map[support.at("node").str()];
        let family = support.at("family").str();
        if support.get("direction").is_some() {
            panic!("skew support {id}: not translated (the product refuses skew with a nonlinear support)");
        }
        match family {
            "anchor" | "guide" | "line_stop" | "vertical_support" | "restraint" => {
                for token in support.at("restraints").arr() {
                    restrained.insert(node * DOF_PER_NODE + dof_offset(token.str()));
                }
            }
            "spring" => {
                let stiffness = support.at("stiffness");
                springs.push((node * DOF_PER_NODE + dof_offset(stiffness.at("dof").str()), force_n(stiffness.at("value"))));
            }
            "nonlinear" => {
                let spec = support.at("nonlinear");
                let dof = dof_index(spec.at("dof").str());
                let sense = |field: &str| match spec.at(field).str() {
                    "positive_reaction" => ActivationSense::PositiveReaction,
                    "negative_reaction" => ActivationSense::NegativeReaction,
                    other => panic!("activation sense {other}"),
                };
                let state = match spec.at("initial_state").str() {
                    "active" => ActiveSetState::Active,
                    "inactive" => ActiveSetState::Inactive,
                    "sticking" => ActiveSetState::Sticking,
                    "sliding" => ActiveSetState::Sliding,
                    other => panic!("state {other}"),
                };
                let made = match spec.at("behavior").str() {
                    "one_way" => NonlinearSupport::one_way(id, node, dof, sense("active_when")),
                    "lift_off" => NonlinearSupport::lift_off(id, node, dof, sense("contact_when")),
                    "gap" => {
                        let direction = match spec.at("closes_when").str() {
                            "positive_displacement" => GapDirection::PositiveDisplacement,
                            "negative_displacement" => GapDirection::NegativeDisplacement,
                            other => panic!("gap direction {other}"),
                        };
                        NonlinearSupport::gap(id, node, dof, length_m(spec.at("gap")), direction).unwrap()
                    }
                    "friction" => {
                        if let Some(normal) = spec.get("normal_reaction") {
                            friction_normals.push(FrictionNormalReaction::new(id, force_n(normal)).unwrap());
                        } else {
                            let source = spec.at("normal_reaction_source");
                            let reference = source.at("support_ref").str();
                            let source_node = node_map[by_id[reference].at("node").str()];
                            derived.push(
                                DerivedFrictionNormalReaction::from_support_reaction(
                                    id, source_node, dof_index(source.at("dof").str()), reference,
                                )
                                .unwrap(),
                            );
                        }
                        NonlinearSupport::friction(id, node, dof, spec.at("friction_coefficient").at("value").num()).unwrap()
                    }
                    other => panic!("behavior {other}"),
                };
                supports.push(made);
                states.push(SupportStateRecord::new(id, state));
            }
            other => panic!("support family {other} is not translated"),
        }
    }

    let cases = model.at("load_cases").arr();
    assert_eq!(cases.len(), 1, "the loop-level translation takes single-case documents");
    let mut force = vec![0.0; n];
    let mut ignored_loads = 0;
    let axis = |direction: &str| match direction {
        "global_x" => 0,
        "global_y" => 1,
        "global_z" => 2,
        "rotation_x" => 3,
        "rotation_y" => 4,
        "rotation_z" => 5,
        other => panic!("direction {other}"),
    };
    for load in cases[0].at("primitive_loads").arr() {
        let target = load.at("target");
        let dimension = load.at("dimension").str();
        match (load.at("category").str(), target.at("type").str(), dimension) {
            (_, "node", "force" | "moment") => {
                let node = node_map[target.at("node").str()];
                force[node * DOF_PER_NODE + axis(load.at("direction").str())] += force_n(load.at("magnitude"));
            }
            ("weight", "element", "force_per_length") => {
                let (i, j) = pipe_ends[target.at("pipe").str()];
                let (a, b) = (nodes[i].coordinates, nodes[j].coordinates);
                let length = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2) + (b[2] - a[2]).powi(2)).sqrt();
                let half = 0.5 * force_n(load.at("magnitude")) * length;
                let k = axis(load.at("direction").str());
                force[i * DOF_PER_NODE + k] += half;
                force[j * DOF_PER_NODE + k] += half;
            }
            ("thermal", "element", "temperature_interval") => {
                let pipe = target.at("pipe").str();
                let (i, j) = pipe_ends[pipe];
                let section = &sections[pipe];
                let axial = section.e * section.area * section.alpha * load.at("magnitude").at("value").num();
                let (a, b) = (nodes[i].coordinates, nodes[j].coordinates);
                let e = unit([b[0] - a[0], b[1] - a[1], b[2] - a[2]]);
                for k in 0..3 {
                    force[i * DOF_PER_NODE + k] -= axial * e[k];
                    force[j * DOF_PER_NODE + k] += axial * e[k];
                }
            }
            _ => ignored_loads += 1,
        }
    }
    let mut ledger = LoadLedger::new();
    for (dof, value) in force.iter().enumerate() {
        if *value != 0.0 {
            ledger.push("translated", dof, *value);
        }
    }
    Model {
        node_count: nodes.len(),
        elements,
        connectors: Vec::new(),
        curved,
        restrained: restrained.into_iter().collect(),
        springs,
        supports,
        states,
        friction_normals,
        derived,
        force: ledger.finish(n).unwrap(),
        ignored_loads,
    }
}

/// Rust-generated family: a chain along x anchored at both ends whose every
/// `every`-th span is an objective connector with two tie rods (no frame
/// element on that span), a lift-off rest at every fourth node, weight and a
/// lateral load.
fn tie_rod_chain(count: usize, every: usize) -> Model {
    let nodes = (0..count)
        .map(|i| FrameNode::new(i, [1.5 * i as f64, 0.0, 0.0]).unwrap())
        .collect::<Vec<_>>();
    let section = FrameSection::new(2.0e11, 8.0e10, 7.38e-3, 6.2e-5, 6.2e-5, 1.24e-4).unwrap();
    let mut elements = Vec::new();
    let mut connectors = Vec::new();
    for i in 1..count {
        if i % every == 0 {
            let x = unit([1.0, 0.0, 0.0]);
            let y = unit(cross([0.0, 0.0, 1.0], x));
            let z = cross(x, y);
            let mut h = [0.0; 21];
            for (index, value) in [(0, 5e8), (6, 4e8), (11, 4e8), (15, 2e6), (18, 1e6), (20, 1e6)] {
                h[index] = value;
            }
            let connector = ObjectiveConnector::new(
                nodes[i - 1],
                nodes[i],
                ConnectorAttachment::global([0.0; 3]),
                ConnectorAttachment::global([0.0; 3]),
                [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]],
                ScaledWorkMatrix { upper_triangle: h, translation_scale: 1.0 },
                [0.0; 6],
            )
            .unwrap()
            .with_tie_rods(&[
                TieRodInput { offset_i: [0.0, 0.2, 0.0], offset_j: [0.0, 0.2, 0.0], stiffness: 1.0e8, reference_extension: 0.0 },
                TieRodInput { offset_i: [0.0, -0.2, 0.0], offset_j: [0.0, -0.2, 0.0], stiffness: 1.0e8, reference_extension: 0.0 },
            ])
            .unwrap();
            connectors.push(connector);
        } else {
            elements.push(FrameElement::new(nodes[i - 1], nodes[i], section, [0.0, 0.0, 1.0]).unwrap());
        }
    }
    let mut restrained = (0..6).collect::<Vec<_>>();
    restrained.extend((count - 1) * 6..count * 6);
    let mut supports = Vec::new();
    let mut states = Vec::new();
    for i in (4..count - 1).step_by(4) {
        let id = format!("support:LO{i}");
        supports.push(NonlinearSupport::lift_off(&id, i, FrameDof::Uz, ActivationSense::PositiveReaction));
        states.push(SupportStateRecord::new(&id, ActiveSetState::Active));
    }
    let mut ledger = LoadLedger::new();
    for i in 0..count {
        ledger.push("weight", i * 6 + 2, -1600.0);
        ledger.push("lateral", i * 6 + 1, if i % 2 == 0 { 250.0 } else { -250.0 });
    }
    Model {
        node_count: count,
        elements,
        connectors,
        curved: Vec::new(),
        restrained,
        springs: Vec::new(),
        supports,
        states,
        friction_normals: Vec::new(),
        derived: Vec::new(),
        force: ledger.finish(count * 6).unwrap(),
        ignored_loads: 0,
    }
}

fn convergence() -> ConvergenceControl {
    // The product's control (DEC-046 product preview policy): count tolerance 0, cap 4.
    ConvergenceControl::new(
        "DEC-046-CV-B-product-preview-active-set-count-v1",
        ConvergencePolicyStatus::Accepted,
        0.0,
        0.0,
        4,
    )
    .unwrap()
}

/// The caller-held input, formed as the product's `nonlinear_frame_input` forms it.
fn caller_input(model: &Model) -> NonlinearFrameSolveInput {
    let mut curved = Vec::with_capacity(model.curved.len());
    for (id, element) in &model.curved {
        curved.push(CurvedBendStiffnessElement::from_macro_element(id.clone(), element).unwrap());
    }
    NonlinearFrameSolveInput {
        node_count: model.node_count,
        elements: model.elements.clone(),
        connectors: model.connectors.clone(),
        curved_bend_elements: curved,
        force: model.force.values().to_vec(),
        base_restrained_dofs: model.restrained.to_vec(),
        base_prescribed: Vec::new(),
        ground_motion: Vec::new(),
        nonlinear_supports: model.supports.clone(),
        initial_states: model.states.clone(),
        friction_normal_reactions: model.friction_normals.clone(),
        derived_friction_normal_reactions: model.derived.clone(),
        convergence: convergence(),
    }
}

// ---------------------------------------------------------------------------
// The measurement.

fn wait_for_timing_window() {
    let Some(path) = std::env::var_os("SWBPIPE_TIMING_WINDOW_FILE") else { return };
    while std::path::Path::new(&path).exists() {
        eprintln!("timing window open: waiting 60 s");
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

/// The JSON-lines output: the file named by `SWBPIPE_SPARSE_MEMORY_OUT`, created once per
/// run (else stdout).
static OUT: Mutex<Option<std::fs::File>> = Mutex::new(None);

fn open_out() {
    if let Some(path) = std::env::var_os("SWBPIPE_SPARSE_MEMORY_OUT") {
        *OUT.lock().unwrap_or_else(|e| e.into_inner()) = Some(std::fs::File::create(path).unwrap());
    }
}

fn emit(line: &str) {
    match OUT.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        Some(file) => writeln!(file, "{line}").unwrap(),
        None => println!("{line}"),
    }
}

fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn opt(value: Option<usize>) -> String {
    value.map_or("null".to_string(), |v| v.to_string())
}

/// The model's counts, from a re-formed loop assembly (after the boundary).
fn counts(model: &Model, input: &NonlinearFrameSolveInput) -> String {
    let blocks = input
        .curved_bend_elements
        .iter()
        .map(|e| StiffnessBlock { node_i: e.node_i, node_j: e.node_j, stiffness: e.global_stiffness })
        .collect::<Vec<_>>();
    let stiffness = assemble_sparse_stiffness(
        input.node_count, &input.elements, &input.connectors, &blocks, &model.springs, &SparseAssemblyOptions::new(),
    )
    .unwrap();
    let evidence = SparseAssemblyEvidence::new(
        stiffness.pattern(), input.node_count, &input.elements, &input.connectors, &input.curved_bend_elements,
        &model.springs,
    )
    .unwrap();
    let storage = evidence.storage_counts();
    let pattern = stiffness.pattern();
    let restrained = input.base_restrained_dofs.iter().copied().collect::<BTreeSet<_>>();
    let mut free_entries = 0usize;
    for row in 0..pattern.dimension() {
        if restrained.contains(&row) {
            continue;
        }
        for index in pattern.row_range(row) {
            if !restrained.contains(&pattern.column(index)) {
                free_entries += 1;
            }
        }
    }
    let (global, free, estimate) = dense_loop_memory_estimate(input.node_count, &input.base_restrained_dofs);
    let tie_rods = input.connectors.iter().map(|c| c.tie_rods().len()).sum::<usize>();
    format!(
        "\"n\":{},\"f0\":{},\"base_restrained\":{},\"z\":{},\"c\":{},\"z_f0\":{},\"nodes\":{},\"elements\":{},\"connectors\":{},\"tie_rods\":{},\"curved\":{},\"springs\":{},\"k\":{},\"d_bar\":{},\"explicit_friction_normals\":{},\"dense_estimate_bytes\":{},\"dense_limit_bytes\":{},\"ignored_loads\":{}",
        opt(global),
        opt(free),
        restrained.len(),
        storage.pattern_entries,
        storage.contributions,
        free_entries,
        input.node_count,
        input.elements.len(),
        input.connectors.len(),
        tie_rods,
        input.curved_bend_elements.len(),
        model.springs.len(),
        input.nonlinear_supports.len(),
        input.derived_friction_normal_reactions.len(),
        input.friction_normal_reactions.len(),
        estimate.map_or("null".to_string(), |e| e.to_string()),
        DENSE_LOOP_MEMORY_CEILING_BYTES,
        model.ignored_loads,
    )
}

/// Counts of active, inactive, sticking and sliding states.
fn state_tally(states: &[SupportStateRecord]) -> String {
    let count = |wanted: ActiveSetState| states.iter().filter(|s| s.state == wanted).count();
    format!(
        "{{\"active\":{},\"inactive\":{},\"sticking\":{},\"sliding\":{}}}",
        count(ActiveSetState::Active),
        count(ActiveSetState::Inactive),
        count(ActiveSetState::Sticking),
        count(ActiveSetState::Sliding),
    )
}

fn iterates(result: &NonlinearFrameSolveResult) -> String {
    let rows = result
        .iterations
        .iter()
        .map(|it| {
            let e = &it.sparse_evidence;
            let (gap, reason) = match &it.strict_gap {
                StrictGapEvidence::NotApplicable => ("not_applicable", String::new()),
                StrictGapEvidence::Qualified(_) => ("qualified", String::new()),
                StrictGapEvidence::Unsupported { reason, .. } => {
                    ("unsupported", reason.chars().take(200).collect::<String>())
                }
            };
            format!(
                "{{\"iteration\":{},\"reduced_dof_count\":{},\"active_restrained\":{},\"ordered_profile_entry_count\":{},\"original_profile_entry_count\":{},\"ordered_max_half_bandwidth\":{},\"original_max_half_bandwidth\":{},\"evidence_status\":{},\"strict_gap\":\"{gap}\",\"strict_gap_reason\":{},\"changed_supports\":{},\"states_after\":{},\"applied_sliding_forces\":{}}}",
                it.iteration,
                e.reduced_dof_count,
                it.active_restrained_dofs.len(),
                opt(e.ordered_profile_entry_count),
                opt(e.original_profile_entry_count),
                opt(e.ordered_max_half_bandwidth),
                opt(e.original_max_half_bandwidth),
                json_string(&format!("{:?}", e.status)),
                json_string(&reason),
                it.active_set.changed_supports.len(),
                state_tally(&it.active_set.states),
                it.applied_sliding_friction_forces.len(),
            )
        })
        .collect::<Vec<_>>();
    let recovered = result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("recovered by one all-active trial"));
    format!(
        "\"iterations\":{},\"converged\":{},\"blocked\":{},\"recovery_trial\":{},\"per_iterate\":[{}]",
        result.iterations.len(),
        result.converged,
        result.is_blocked(),
        recovered,
        rows.join(","),
    )
}

/// Measures one model: the boundary (input, call, held result) and the call.
fn measure(name: &str, family: &str, sha: &str, model: &Model) -> Result<(), String> {
    wait_for_timing_window();
    let outer_start = reset_peaks(0);
    let input = caller_input(model);
    let inner_start = reset_peaks(1);
    let started = std::time::Instant::now();
    let solved = solve_active_set_frame_with_mode_and_springs_assembled(
        &input,
        &model.force,
        LinearSolveMode::SparseInteractive,
        &model.springs,
    );
    // Both ends are read before anything is formatted: nothing allocates between them.
    let inner_end = endpoint(1);
    let seconds = started.elapsed().as_secs_f64();
    let outer_end = endpoint(0);
    // The report's values are formed after both ends; their bytes are counted apart
    // (`thread_report_values_live`) and excluded from the residual below.
    let report_start = thread_live();
    let outer = measured(outer_start, outer_end);
    let inner = measured(inner_start, inner_end);
    // The held result's and input's shares of the end-of-boundary live heap.
    let result_live = inner_end.at.thread_live - inner_start.thread_live;
    let input_live = inner_start.thread_live - outer_start.thread_live;
    let outcome = match &solved {
        Ok(result) => format!("\"outcome\":\"solved\",{}", iterates(result)),
        Err(error) => format!(
            "\"outcome\":\"error\",\"error_kind\":{},\"error\":{}",
            json_string(match error {
                NonlinearIntegrationError::DenseMemoryLimitExceeded(_) => "DenseMemoryLimitExceeded",
                NonlinearIntegrationError::Structural(_) => "Structural",
                NonlinearIntegrationError::FrameKernel(_) => "FrameKernel",
                NonlinearIntegrationError::NonlinearSupport(_) => "NonlinearSupport",
                NonlinearIntegrationError::InvalidInput { .. } => "InvalidInput",
            }),
            json_string(&error.to_string()),
        ),
    };
    let counts = counts(model, &input);
    let report_live = thread_live() - report_start;
    // Release: nothing but the drops lies between each pair of readings. The springs belong
    // to the translated model, formed before the boundary, and are not in it.
    let before_result_drop = thread_live();
    drop(solved);
    let after_result_drop = thread_live();
    drop(input);
    let after_input_drop = thread_live();
    let line = format!(
        "{{\"level\":\"loop\",\"model\":{},\"family\":{},\"sha256\":\"{sha}\",{COUNTERS},{counts},{outcome},\"boundary\":{outer},\"call\":{inner},\"thread_input_live_requested\":{input_live},\"thread_result_live_requested\":{result_live},\"thread_released_by_result_drop\":{},\"thread_released_by_input_drop\":{},\"thread_report_values_live\":{report_live},\"thread_live_after_drops_excluding_report_values\":{},\"call_seconds\":{seconds:.3}}}",
        json_string(name),
        json_string(family),
        before_result_drop - after_result_drop,
        after_result_drop - after_input_drop,
        after_input_drop - outer_start.thread_live - report_live,
    );
    emit(&line);
    eprintln!("{name}: boundary {outer}");
    Ok(())
}

fn manifest() -> Option<(std::path::PathBuf, Json)> {
    let dir = std::path::PathBuf::from(std::env::var_os("SWBPIPE_SPARSE_MEMORY_MODELS")?);
    let text = std::fs::read(dir.join("manifest.json")).expect("manifest.json in SWBPIPE_SPARSE_MEMORY_MODELS");
    Some((dir, Reader::parse(&text)))
}

/// The manifest's models at `level` (name, family, SHA-256, path), each file's
/// hash checked against the manifest; parsed one at a time by the caller.
fn models(level: &str) -> Vec<(String, String, String, std::path::PathBuf)> {
    let Some((dir, manifest)) = manifest() else { return Vec::new() };
    let only = std::env::var("SWBPIPE_SPARSE_MEMORY_ONLY").ok();
    let mut out = Vec::new();
    for entry in manifest.at("models").arr() {
        if !entry.at("levels").arr().iter().any(|l| l.str() == level) {
            continue;
        }
        let name = entry.at("name").str().to_string();
        if only.as_deref().is_some_and(|only| !only.split(',').any(|o| o == name)) {
            continue;
        }
        let path = dir.join(entry.at("file").str());
        let sha = sha256(&std::fs::read(&path).unwrap());
        assert_eq!(sha, entry.at("sha256").str(), "{name}: the file's SHA-256 differs from the manifest's");
        out.push((name, entry.at("family").str().to_string(), sha, path));
    }
    out
}

#[test]
fn sha256_matches_the_standard_vectors() {
    assert_eq!(sha256(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(sha256(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

#[test]
fn the_counter_counts_a_known_allocation() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let start = reset_peaks(0);
    let block = vec![0u8; 1 << 20];
    let held = snapshot();
    drop(block);
    let end = endpoint(0);
    // Exact on this thread's own tally, which no other thread touches, read as the
    // measurement reads it (`endpoint`): the counter counts the block's requested size at its
    // allocation, in the peak, and at its release.
    assert_eq!(end.thread_peak - start.thread_live, 1 << 20);
    assert_eq!(held.thread_live - start.thread_live, 1 << 20);
    assert_eq!(end.at.thread_live, start.thread_live);
    // The global counters, which the harness's other threads share (a few bytes may be
    // allocated or freed there at the same time): bounded within 64 KiB.
    let near = |value: i128| ((1 << 20) - (64 << 10)..(1 << 20) + (64 << 10)).contains(&value);
    let peak = end.peak_req as i128 - start.live_req as i128;
    assert!(near(peak), "peak {peak}");
    assert!(near(held.live_req as i128 - start.live_req as i128));
    assert!((end.at.live_req as i128 - start.live_req as i128).abs() < 64 << 10);
    assert!(near(end.peak_rnd as i128 - start.live_rnd as i128));
}

#[test]
fn the_thread_tally_counts_a_reallocation() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let start = reset_peaks(0);
    let mut block: Vec<u8> = Vec::with_capacity(1 << 19);
    block.reserve_exact(1 << 20);
    let grown = snapshot();
    let capacity = block.capacity();
    drop(block);
    let end = endpoint(0);
    // A reallocation leaves the new size live and the old one released; the peak is the new size.
    assert_eq!(grown.thread_live - start.thread_live, capacity as isize);
    assert_eq!(end.thread_peak - start.thread_live, capacity as isize);
    assert_eq!(end.at.thread_live, start.thread_live);
    assert!(grown.reallocs > start.reallocs);
}

/// Every manifest model marked `loop`, then the Rust-generated tie-rod chains, then the
/// loop entry's guard on the models marked `refusal` (just above the limit): the refusal
/// comes before the loop's assembly, and its heap is reported. One test, so that one run
/// writes one output file.
#[test]
fn sparse_loop_peaks_over_the_model_family() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    if std::env::var_os("SWBPIPE_SPARSE_MEMORY_MODELS").is_none() {
        eprintln!("SWBPIPE_SPARSE_MEMORY_MODELS is not set: no model is measured");
        return;
    }
    open_out();
    let only = std::env::var("SWBPIPE_SPARSE_MEMORY_ONLY").ok();
    for (name, family, sha, path) in models("loop") {
        let model = translate(&Reader::parse(&std::fs::read(path).unwrap()));
        measure(&name, &family, &sha, &model).unwrap();
    }
    for (count, every) in [(400usize, 5usize), (1200, 3)] {
        let name = format!("tie-rods-{count}-every-{every}");
        if only.as_deref().is_some_and(|only| !only.split(',').any(|o| o == name)) {
            continue;
        }
        let model = tie_rod_chain(count, every);
        // The generated input's identity: the SHA-256 of its `Debug` text.
        let sha = sha256(format!("{:?}", caller_input(&model)).as_bytes());
        measure(&name, "connectors with tie rods (Rust-generated)", &sha, &model).unwrap();
    }
    for (name, family, sha, path) in models("refusal") {
        let model = translate(&Reader::parse(&std::fs::read(path).unwrap()));
        measure(&name, &family, &sha, &model).unwrap();
    }
}
