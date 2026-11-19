//! DEL-04-07 REQ-008, part 1 (evidence only): the sparse active-set loop's heap on the
//! PRODUCT's own loop input ("product-loop").
//!
//! For each case of each model document this module forms the input exactly as the product
//! does: the same private prefix (`run_linear_static_preview_observed_inner`'s lowering,
//! validation, normalization, base materials, `build_model_for_basis`, `prepare_boundary`; per
//! case its modulus basis, pressure case, loads, thermal loads, `case_force_ledger`, spring base
//! motion, `finish_case_ledger` and motions, as `solve_prepared_load_case` does), then
//! measures, with a counting global allocator, one boundary that includes:
//! - `nonlinear_frame_input` and `loop_springs` (the caller-held input, as
//!   `append_nonlinear_support_loop_results` forms it);
//! - `solve_active_set_frame_with_mode_and_springs_assembled` in `SparseInteractive`;
//! - the returned result, held, with the exit receipt's clone inside it.
//!
//! It also reports the peak inside the call alone and the counts a bound needs, in the same
//! JSON line form as NI's `tests/sparse_loop_memory.rs` (level "product-loop"). Each case is
//! checked against the product's own call of the same document: its iteration count and every
//! support's final state code must agree (`product_agrees`). It asserts no bound. Documents
//! with load/reference states are not covered (their prefix resolves cases; refused by name).
//!
//! Counters: the loop runs on the measuring thread, so the figures are that thread's own
//! tally of requested bytes (fields prefixed `thread_`), which the self-tests check exactly.
//! The process-wide counters (prefix `process_`) are diagnostics; every record says so in
//! its `counters` field. Each region's peaks and counters are read before any report is
//! formed, and the report's own allocations are counted apart.
//!
//! Test-only and absent from every debug-assertions build: the module is
//! `cfg(all(test, not(debug_assertions)))`, so neither it nor its `#[global_allocator]` is in
//! the registered lib test binary (debug, debug assertions on) in which the AC-004 stack
//! witnesses run, nor in CI's test builds (debug assertions on). It is measured from a release
//! lib test build: `SWBPIPE_SPARSE_MEMORY_MODELS=<dir> SWBPIPE_SPARSE_MEMORY_OUT=<file>
//! cargo test --release --lib sparse_loop_memory_tests -- --test-threads 1`. Without
//! `SWBPIPE_SPARSE_MEMORY_MODELS` it returns at once. The allocator counts always but enforces
//! its safety cap only while a measurement is armed. While the file named by
//! `SWBPIPE_TIMING_WINDOW_FILE` exists, no model is solved.
use super::*;
use open_pipe_stress_frame_kernel::structural::{assemble_sparse_stiffness, SparseAssemblyOptions, StiffnessBlock};
use open_pipe_stress_nonlinear_integration::structural_adapter::{SparseAssemblyEvidence, StrictGapEvidence};
use open_pipe_stress_nonlinear_integration::{
    dense_loop_memory_estimate, NonlinearFrameSolveInput, NonlinearFrameSolveResult, NonlinearIntegrationError,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::BTreeSet;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// The counting allocator (as NI's loop-level binary's).

const CAP_BYTES: usize = 16 << 30;
static ARMED: AtomicBool = AtomicBool::new(false);
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
/// The module's tests share the global counters: one runs at a time.
pub(crate) static SERIAL: Mutex<()> = Mutex::new(());

#[cfg(target_os = "macos")]
extern "C" {
    fn malloc_size(ptr: *const std::ffi::c_void) -> usize;
}

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
    if live > CAP_BYTES && ARMED.load(SeqCst) {
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
pub(crate) struct Snapshot {
    pub(crate) live_req: usize,
    live_rnd: usize,
    pub(crate) allocs: usize,
    deallocs: usize,
    reallocs: usize,
    moved: usize,
    pub(crate) thread_live: isize,
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

pub(crate) fn reset_peaks(slot: usize) -> Snapshot {
    let now = snapshot();
    PEAK_REQ[slot].store(now.live_req, SeqCst);
    PEAK_MOVE[slot].store(now.live_req, SeqCst);
    PEAK_RND[slot].store(now.live_rnd, SeqCst);
    let _ = THREAD_PEAK.try_with(|peaks| peaks[slot].set(thread_live()));
    snapshot()
}

/// A region's end: every counter and peak, read at once and without allocating, before any
/// report is formed.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Endpoint {
    pub(crate) at: Snapshot,
    pub(crate) peak_req: usize,
    peak_rnd: usize,
    pub(crate) peak_move: usize,
    pub(crate) thread_peak: isize,
}

pub(crate) fn endpoint(slot: usize) -> Endpoint {
    Endpoint {
        thread_peak: thread_peak(slot),
        peak_req: PEAK_REQ[slot].load(SeqCst),
        peak_rnd: PEAK_RND[slot].load(SeqCst),
        peak_move: PEAK_MOVE[slot].load(SeqCst),
        at: snapshot(),
    }
}

/// What the field prefixes of each record name, carried in every record.
fn counters() -> Value {
    json!({
        "thread_": "the measuring thread's own tally of requested bytes (Layout::size); exactly self-tested; authoritative",
        "process_": "process-wide counters shared by every thread: diagnostics only, not exactly self-tested (the self-test bounds the requested peak within 64 KiB); rounded bytes are malloc_size of each block, the moving-realloc peak adds each moved block's new size to the live total, and event counts include any thread's events",
    })
}

fn measured(start: Snapshot, end: Endpoint) -> Value {
    json!({
        "thread_peak_requested": end.thread_peak - start.thread_live,
        "thread_live_requested_at_end": end.at.thread_live - start.thread_live,
        "process_peak_requested": end.peak_req - start.live_req,
        "process_peak_rounded": end.peak_rnd - start.live_rnd,
        "process_peak_requested_moving_realloc": end.peak_move - start.live_req,
        "process_live_requested_at_end": end.at.live_req as i128 - start.live_req as i128,
        "process_live_rounded_at_end": end.at.live_rnd as i128 - start.live_rnd as i128,
        "process_allocations": end.at.allocs - start.allocs,
        "process_deallocations": end.at.deallocs - start.deallocs,
        "process_reallocations": end.at.reallocs - start.reallocs,
        "process_moved_reallocations": end.at.moved - start.moved,
    })
}

// ---------------------------------------------------------------------------
// The product's prefix, in the product's order, up to each case's loop input.

/// One case's caller-side state, as `solve_prepared_load_case` holds it at the loop call.
struct PreparedCase {
    id: String,
    basis: usize,
    force: AssembledForce,
    base_motion: Option<BTreeMap<usize, f64>>,
    ground_motion: Vec<(String, f64)>,
}

struct Prepared {
    bases: Vec<(Option<String>, BuiltModel)>,
    restrained_dofs: Vec<usize>,
    springs: Vec<SpringEntry>,
    cases: Vec<PreparedCase>,
}

fn blocking_codes(diagnostics: &[Diagnostic]) -> String {
    diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect::<Vec<_>>().join(",")
}

fn check(diagnostics: &[Diagnostic]) -> Result<(), String> {
    if has_blocking(diagnostics) {
        Err(format!("blocked: {}", blocking_codes(diagnostics)))
    } else {
        Ok(())
    }
}

fn prepare(actual_request: Value, solver_mode: PreviewSolverMode) -> Result<Prepared, String> {
    if let Some(refusal) = model_fields::unknown_field_refusal(&actual_request) {
        return Err(refusal);
    }
    let (request, _capture) =
        source_receipt::CapturedInvocation::parse(actual_request, solver_mode).map_err(|error| error.0)?;
    let mut model = request.model;
    let request_materials_supplied = !request.materials.is_empty();
    let mut materials = if request.materials.is_empty() { model.materials.clone() } else { request.materials };
    let mut diagnostics = Vec::new();
    operating_conditions::lower(&mut model, &mut diagnostics);
    pressure_runtime::validate_profile(&model, &mut diagnostics);
    pressure_runtime::junction_closure_warnings(&model, &mut diagnostics);
    case_state::resolve::validate_document(&model, request_materials_supplied, &mut diagnostics);
    if imposed_motion::validate(&model, &mut diagnostics) {
        return Err(format!("blocked: {}", blocking_codes(&diagnostics)));
    }
    validate_model_inputs(&model, &materials, &mut diagnostics);
    validate_support_family_tokens(&model, &mut diagnostics);
    pf_timoshenko::append_partial_span_refusals(&model, &mut diagnostics);
    check(&diagnostics)?;
    document_members::withhold_interim_combinations(&mut model, &mut diagnostics);
    self_weight::validate_applied_self_weight(&model, &mut diagnostics);
    check(&diagnostics)?;
    resolve_shared_sections(&mut model, &mut diagnostics);
    check(&diagnostics)?;
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    check(&diagnostics)?;
    rigid_component::append_eccentric_offset_refusals(&model, &mut diagnostics);
    check(&diagnostics)?;
    if case_state::is_load_state(&model) {
        return Err("load/reference-state documents are not covered by this module".to_string());
    }
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    check(&diagnostics)?;
    let consumed_bases = pf_timoshenko::ConsumedBases::of(&model);
    let built = build_model_for_basis(&model, &materials, None, consumed_bases.base.as_ref(), &mut diagnostics);
    check(&diagnostics)?;
    let built = built.expect("build_model returns Some when no blocking diagnostics were added");
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    if !boundary.findings.is_empty() {
        return Err(format!("support findings: {}", boundary.findings.len()));
    }
    // Each case's basis, as the product's case loop selects or forms it.
    let mut bases: Vec<(Option<String>, Vec<MaterialInput>, BuiltModel)> = vec![(None, materials.clone(), built)];
    let mut case_basis = Vec::new();
    for load_case in &model.load_cases {
        let index = match modulus_basis_key(load_case, &mut diagnostics) {
            None => 0,
            Some(key) => match bases.iter().position(|(k, _, _)| k.as_ref() == Some(&key)) {
                Some(index) => index,
                None => {
                    let (basis_materials, _record) =
                        materials_for_modulus_basis_observed(&model, &materials, load_case, &mut diagnostics, None)
                            .ok_or_else(|| format!("basis refused: {}", blocking_codes(&diagnostics)))?;
                    let basis_built = build_model_for_basis(
                        &model, &basis_materials, None, consumed_bases.selected(&key), &mut diagnostics,
                    );
                    check(&diagnostics)?;
                    bases.push((Some(key), basis_materials, basis_built.expect("built without blocking")));
                    bases.len() - 1
                }
            },
        };
        check(&diagnostics)?;
        case_basis.push(index);
    }
    let mut cases = Vec::new();
    for (load_case, &basis) in model.load_cases.iter().zip(&case_basis) {
        let (_, basis_materials, basis_built) = &bases[basis];
        let exact_pressure = pressure_runtime::build_pressure_case_with_members(
            &model, basis_built, basis_materials, load_case, None, &mut diagnostics,
        );
        let loads = build_load_case_primitive_loads(&model, load_case, &mut diagnostics);
        let load_application = prepare_loads(basis_built.nodes.len(), basis_built.pipes.len(), &loads);
        if !load_application.findings.is_empty() {
            return Err(format!("load findings: {}", load_application.findings.len()));
        }
        let pipe_map = model.pipe_segments.iter().enumerate().map(|(i, p)| (p.id.as_str(), i)).collect::<HashMap<_, _>>();
        let material_map = basis_materials.iter().map(|m| (m.id.as_str(), m)).collect::<HashMap<_, _>>();
        let thermal_loads =
            build_thermal_element_loads(&model, load_case, &material_map, &pipe_map, basis_built, &mut diagnostics);
        let curved_bends_by_pipe = basis_built
            .curved_bend_elements
            .iter()
            .map(|element| (element.pipe_index, element))
            .collect::<HashMap<_, _>>();
        let mut ledger = case_force_ledger(
            &model, basis_built, &load_application, &curved_bends_by_pipe, &thermal_loads, exact_pressure.as_ref(),
            &load_case.id, &mut diagnostics,
        );
        let spring_ground = imposed_motion::case_spring_ground(&model, load_case, &boundary.springs, &mut diagnostics);
        certified_loads::spring_base_motion(&mut ledger, &boundary.springs, &spring_ground);
        let force = finish_case_ledger(ledger, basis_built.nodes.len()).map_err(|error| error.to_string())?;
        let base_motion = imposed_motion::case_prescribed(&model, load_case, &boundary.restrained_dofs, &mut diagnostics);
        let ground_motion = imposed_motion::case_ground_motion(&model, load_case);
        check(&diagnostics)?;
        cases.push(PreparedCase { id: load_case.id.clone(), basis, force, base_motion, ground_motion });
    }
    Ok(Prepared {
        bases: bases.into_iter().map(|(key, _, built)| (key, built)).collect(),
        restrained_dofs: boundary.restrained_dofs,
        springs: boundary.springs,
        cases,
    })
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

fn emit(line: &Value) {
    match OUT.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        Some(file) => writeln!(file, "{line}").unwrap(),
        None => println!("{line}"),
    }
}

fn counts(input: &NonlinearFrameSolveInput, springs: &[(usize, f64)]) -> Value {
    let blocks = input
        .curved_bend_elements
        .iter()
        .map(|e| StiffnessBlock { node_i: e.node_i, node_j: e.node_j, stiffness: e.global_stiffness })
        .collect::<Vec<_>>();
    let stiffness = assemble_sparse_stiffness(
        input.node_count, &input.elements, &input.connectors, &blocks, springs, &SparseAssemblyOptions::new(),
    )
    .unwrap();
    let evidence = SparseAssemblyEvidence::new(
        stiffness.pattern(), input.node_count, &input.elements, &input.connectors, &input.curved_bend_elements, springs,
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
        free_entries += pattern.row_range(row).filter(|&index| !restrained.contains(&pattern.column(index))).count();
    }
    let (global, free, estimate) = dense_loop_memory_estimate(input.node_count, &input.base_restrained_dofs);
    json!({
        "n": global, "f0": free, "base_restrained": restrained.len(), "z": storage.pattern_entries,
        "c": storage.contributions, "z_f0": free_entries, "nodes": input.node_count, "elements": input.elements.len(),
        "connectors": input.connectors.len(),
        "tie_rods": input.connectors.iter().map(|c| c.tie_rods().len()).sum::<usize>(),
        "curved": input.curved_bend_elements.len(), "springs": springs.len(), "k": input.nonlinear_supports.len(),
        "d_bar": input.derived_friction_normal_reactions.len(),
        "explicit_friction_normals": input.friction_normal_reactions.len(),
        "dense_estimate_bytes": estimate.map(|e| e.to_string()),
        "dense_limit_bytes": DENSE_LOOP_MEMORY_CEILING_BYTES.to_string(),
        "base_prescribed": input.base_prescribed.len(), "ground_motion": input.ground_motion.len(),
    })
}

fn state_tally(states: &[SupportStateRecord]) -> Value {
    let count = |wanted: ActiveSetState| states.iter().filter(|s| s.state == wanted).count();
    json!({"active": count(ActiveSetState::Active), "inactive": count(ActiveSetState::Inactive),
           "sticking": count(ActiveSetState::Sticking), "sliding": count(ActiveSetState::Sliding)})
}

fn iterates(result: &NonlinearFrameSolveResult) -> Value {
    let per = result
        .iterations
        .iter()
        .map(|it| {
            let e = &it.sparse_evidence;
            let (gap, reason) = match &it.strict_gap {
                StrictGapEvidence::NotApplicable => ("not_applicable", String::new()),
                StrictGapEvidence::Qualified(_) => ("qualified", String::new()),
                StrictGapEvidence::Unsupported { reason, .. } => ("unsupported", reason.chars().take(200).collect()),
            };
            json!({
                "iteration": it.iteration, "reduced_dof_count": e.reduced_dof_count,
                "active_restrained": it.active_restrained_dofs.len(),
                "ordered_profile_entry_count": e.ordered_profile_entry_count,
                "original_profile_entry_count": e.original_profile_entry_count,
                "ordered_max_half_bandwidth": e.ordered_max_half_bandwidth,
                "original_max_half_bandwidth": e.original_max_half_bandwidth,
                "evidence_status": format!("{:?}", e.status), "strict_gap": gap, "strict_gap_reason": reason,
                "changed_supports": it.active_set.changed_supports.len(),
                "states_after": state_tally(&it.active_set.states),
                "applied_sliding_forces": it.applied_sliding_friction_forces.len(),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "iterations": result.iterations.len(), "converged": result.converged, "blocked": result.is_blocked(),
        "recovery_trial": result.diagnostics.iter().any(|d| d.message.contains("recovered by one all-active trial")),
        "per_iterate": per,
    })
}

/// The product's own call of the document: per case, its iteration-count row and each
/// support's final state code (row `basis_ref` names the case).
fn product_facts(request: Value, solver_mode: PreviewSolverMode) -> BTreeMap<String, (Option<f64>, BTreeMap<String, f64>)> {
    let mut facts: BTreeMap<String, (Option<f64>, BTreeMap<String, f64>)> = BTreeMap::new();
    let Ok(envelope) = run_linear_static_preview_value_with_mode(request, solver_mode) else { return facts };
    for row in &envelope.results {
        let case = row.basis_ref.as_ref().map_or(String::new(), |b| b.ref_id.clone());
        match row.kind.as_str() {
            "nonlinear_support_active_set_iteration_count" => facts.entry(case).or_default().0 = Some(row.value),
            "nonlinear_support_active_set_state_code" => {
                facts.entry(case).or_default().1.insert(row.entity_ref.clone(), row.value);
            }
            _ => {}
        }
    }
    facts
}

fn state_code(state: ActiveSetState) -> f64 {
    match state {
        ActiveSetState::Inactive => 0.0,
        ActiveSetState::Active => 1.0,
        ActiveSetState::Sticking => 2.0,
        ActiveSetState::Sliding => 3.0,
    }
}

fn measure_document(name: &str, family: &str, sha: &str, request: Value) {
    let mode = PreviewSolverMode::SparseInteractive;
    // The product's own call is a solve too: it waits for the timing window like each case.
    wait_for_timing_window();
    let facts = product_facts(request.clone(), mode);
    let prepared = match prepare(request, mode) {
        Ok(prepared) => prepared,
        Err(reason) => {
            emit(&json!({"level": "product-loop", "model": name, "family": family, "sha256": sha,
                         "outcome": "not_prepared", "reason": reason}));
            return;
        }
    };
    for case in &prepared.cases {
        wait_for_timing_window();
        let (basis_key, built) = &prepared.bases[case.basis];
        ARMED.store(true, SeqCst);
        let outer_start = reset_peaks(0);
        let input = nonlinear_frame_input(built, &prepared.restrained_dofs, case.base_motion.as_ref(), &case.ground_motion, &case.force);
        let springs = loop_springs(&prepared.springs);
        let inner_start = reset_peaks(1);
        let started = std::time::Instant::now();
        let solved = input.as_ref().map_err(Clone::clone).and_then(|input| {
            solve_active_set_frame_with_mode_and_springs_assembled(input, &case.force, mode.nonlinear_mode(), &springs)
        });
        // Both ends are read before anything is formatted: nothing allocates between them.
        let inner_end = endpoint(1);
        let seconds = started.elapsed().as_secs_f64();
        let outer_end = endpoint(0);
        ARMED.store(false, SeqCst);
        // The report's values are formed after both ends; their bytes are counted apart
        // (`thread_report_values_live`) and excluded from the residual below.
        let report_start = thread_live();
        let outer = measured(outer_start, outer_end);
        let inner = measured(inner_start, inner_end);
        let result_live = inner_end.at.thread_live - inner_start.thread_live;
        let input_live = inner_start.thread_live - outer_start.thread_live;
        let product = facts.get(&case.id);
        let (outcome, agrees) = match &solved {
            Ok(result) => {
                let iterations = product.and_then(|(count, _)| *count) == Some(result.iterations.len() as f64);
                let states = product.is_some_and(|(_, states)| {
                    states.len() == result.final_states.len()
                        && result.final_states.iter().all(|s| states.get(&s.support_id) == Some(&state_code(s.state)))
                });
                (json!({"outcome": "solved", "loop": iterates(result)}), json!({"iterations": iterations, "final_states": states}))
            }
            Err(error) => (
                json!({"outcome": "error", "error_kind": match error {
                    NonlinearIntegrationError::DenseMemoryLimitExceeded(_) => "DenseMemoryLimitExceeded",
                    NonlinearIntegrationError::Structural(_) => "Structural",
                    NonlinearIntegrationError::FrameKernel(_) => "FrameKernel",
                    NonlinearIntegrationError::NonlinearSupport(_) => "NonlinearSupport",
                    NonlinearIntegrationError::InvalidInput { .. } => "InvalidInput",
                }, "error": error.to_string()}),
                json!({"product_has_case_rows": product.is_some()}),
            ),
        };
        let counts = input.as_ref().ok().map(|input| counts(input, &springs));
        let report_live = thread_live() - report_start;
        // Release: nothing but the drops lies between each pair of readings.
        let before_result_drop = thread_live();
        drop(solved);
        let after_result_drop = thread_live();
        drop(input);
        drop(springs);
        let after_input_drop = thread_live();
        emit(&json!({
            "level": "product-loop", "model": name, "family": family, "sha256": sha, "case": case.id,
            "counters": counters(),
            "modulus_basis": basis_key, "counts": counts, "result": outcome, "product_agrees": agrees,
            "boundary": outer, "call": inner, "thread_input_live_requested": input_live,
            "thread_result_live_requested": result_live,
            "thread_released_by_result_drop": before_result_drop - after_result_drop,
            "thread_released_by_input_and_springs_drop": after_result_drop - after_input_drop,
            "thread_report_values_live": report_live,
            "thread_live_after_drops_excluding_report_values":
                after_input_drop - outer_start.thread_live - report_live,
            "call_seconds": (seconds * 1000.0).round() / 1000.0,
        }));
        eprintln!("{name} {}: boundary peak {}", case.id, outer["thread_peak_requested"]);
    }
}

#[test]
fn product_loop_peaks_over_the_model_family() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let Some(dir) = std::env::var_os("SWBPIPE_SPARSE_MEMORY_MODELS").map(std::path::PathBuf::from) else {
        eprintln!("SWBPIPE_SPARSE_MEMORY_MODELS is not set: no model is measured");
        return;
    };
    open_out();
    let manifest: Value = serde_json::from_slice(&std::fs::read(dir.join("manifest.json")).unwrap()).unwrap();
    let only = std::env::var("SWBPIPE_SPARSE_MEMORY_ONLY").ok();
    for entry in manifest["models"].as_array().unwrap() {
        let name = entry["name"].as_str().unwrap();
        if only.as_deref().is_some_and(|only| !only.split(',').any(|o| o == name)) {
            continue;
        }
        let bytes = std::fs::read(dir.join(entry["file"].as_str().unwrap())).unwrap();
        let sha = format!("{:x}", Sha256::digest(&bytes));
        assert_eq!(sha, entry["sha256"].as_str().unwrap(), "{name}: the file's SHA-256 differs from the manifest's");
        let request: Value = serde_json::from_slice(&bytes).unwrap();
        drop(bytes);
        measure_document(name, entry["family"].as_str().unwrap(), &sha, request);
    }
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
    // measurement reads it (`endpoint`).
    assert_eq!(end.thread_peak - start.thread_live, 1 << 20);
    assert_eq!(held.thread_live - start.thread_live, 1 << 20);
    assert_eq!(end.at.thread_live, start.thread_live);
    // The global counters, which other threads (the harness, other unit tests) share: bounded
    // within 64 KiB of the block.
    let peak = end.peak_req as i128 - start.live_req as i128;
    assert!(((1 << 20) - (64 << 10)..(1 << 20) + (64 << 10)).contains(&peak), "peak {peak}");
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
