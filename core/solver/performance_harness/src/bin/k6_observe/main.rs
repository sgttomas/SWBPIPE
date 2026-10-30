//! `k6_observe`: one K6 kernel observation per process (T3 D1 §4.8; K6 plan
//! §2). Observation only: it prints JSONL on stdout and asserts no time or
//! memory bound. See `H/README.md` ("K6") for the schema.
//!
//! Exit codes: 0 completed (an M03 refusal is an outcome, not an error);
//! 2 usage; 3 refused by the binary; 4 internal error. A heap-cap refusal
//! aborts (SIGABRT) after the allocator's marker.

mod alloc;
mod w1;

use open_pipe_stress_frame_kernel::assemble_global_stiffness;
use open_pipe_stress_frame_kernel::structural::retained_api::AttemptRecord;
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, SparseAssemblyOptions, StructuralError, StructuralSolution,
};
use open_pipe_stress_solver_performance_harness::k6::counts::{
    admission_estimate_bytes, compute, f1b_estimate_bytes, parse_counts_line, K6Counts, SizeFacts,
};
use open_pipe_stress_solver_performance_harness::k6::models::{
    extra_model_ids, model_described, sealed_model_ids, K6Model, ModelOrigin,
};
use open_pipe_stress_solver_performance_harness::k6::parity::bitwise_k;
use open_pipe_stress_solver_performance_harness::k6::staged::{
    entry, lane_id, lane_lu, outcome_class, recovery, setup, short_error, staged_solve, Stage,
    StageObserver,
};
use open_pipe_stress_solver_performance_harness::k6::w1::adapter as w1_adapter;
use open_pipe_stress_solver_performance_harness::k6::w1::counts::{
    compute_described as w1_compute, storage_bytes, wide_bytes, LIMBS_PER_ENTRY,
};
use open_pipe_stress_solver_performance_harness::k6::w1::envelope::EnvelopeError;
use open_pipe_stress_solver_performance_harness::k6::w1::h_envelope::{
    self, HComposedEstimate, HLaunch, HModelFacts, HSourceFacts, ReferenceHProfile,
};
use open_pipe_stress_solver_performance_harness::k6::w1::staged::{
    attempts_of, prefix_limits, w1_solve, w1_source, W1Limits,
};
use open_pipe_stress_solver_performance_harness::k6::{
    canonical, debug_digest, Fnv64, Mode, N2_REFUSAL_MEMBERS,
};
use std::fmt::Display;
use std::io::Write;
use std::time::Instant;

#[global_allocator]
static ALLOCATOR: alloc::K6Alloc = alloc::K6Alloc;

const SCHEMA: &str = "k6-observe-v1";
/// `--allow-over-estimate` is accepted only under a heap cap this small.
const OVER_ESTIMATE_CAP_LIMIT: usize = 512 * 1024 * 1024;
/// Bitwise K is recorded up to this member count (K6 brief Scope 7).
const BITWISE_K_MEMBERS: usize = 1000;

// ------------------------------------------------------------------ output

/// One JSON object, keys in insertion order.
struct Line(String);

impl Line {
    fn new(kind: &str) -> Self {
        let mut line = Line(String::from("{"));
        line.key("kind");
        line.quoted(kind);
        line
    }
    fn key(&mut self, key: &str) {
        if self.0.len() > 1 {
            self.0.push(',');
        }
        self.quoted(key);
        self.0.push(':');
    }
    fn quoted(&mut self, text: &str) {
        self.0.push('"');
        for c in text.chars() {
            match c {
                '"' => self.0.push_str("\\\""),
                '\\' => self.0.push_str("\\\\"),
                '\n' => self.0.push_str("\\n"),
                '\r' => self.0.push_str("\\r"),
                '\t' => self.0.push_str("\\t"),
                c if (c as u32) < 0x20 => self.0.push_str(&format!("\\u{:04x}", c as u32)),
                c => self.0.push(c),
            }
        }
        self.0.push('"');
    }
    fn s(mut self, key: &str, value: &str) -> Self {
        self.key(key);
        self.quoted(value);
        self
    }
    fn n(mut self, key: &str, value: impl Display) -> Self {
        self.key(key);
        self.0.push_str(&value.to_string());
        self
    }
    fn b(mut self, key: &str, value: bool) -> Self {
        self.key(key);
        self.0.push_str(if value { "true" } else { "false" });
        self
    }
    fn opt_s(self, key: &str, value: Option<&str>) -> Self {
        match value {
            Some(v) => self.s(key, v),
            None => self.null(key),
        }
    }
    fn opt_n(self, key: &str, value: Option<impl Display>) -> Self {
        match value {
            Some(v) => self.n(key, v),
            None => self.null(key),
        }
    }
    fn null(mut self, key: &str) -> Self {
        self.key(key);
        self.0.push_str("null");
        self
    }
    fn hex(self, key: &str, value: u64) -> Self {
        self.s(key, &format!("{value:016x}"))
    }
    /// A binary64 in exponent form (the shortest round-trip digits), `null`
    /// when absent, and a string for a non-finite value (not a JSON number).
    fn f(mut self, key: &str, value: Option<f64>) -> Self {
        match value {
            None => self.null(key),
            Some(x) if x.is_finite() => {
                self.key(key);
                self.0.push_str(&format!("{x:e}"));
                self
            }
            Some(x) => self.s(key, &format!("{x}")),
        }
    }
    fn emit(mut self) {
        self.0.push('}');
        self.0.push('\n');
        let mut out = std::io::stdout().lock();
        let _ = out.write_all(self.0.as_bytes());
        let _ = out.flush();
    }
}

// ------------------------------------------------------------------ arguments

struct Args {
    model: Option<String>,
    model_file: Option<String>,
    mode: Option<Mode>,
    heap_cap: Option<usize>,
    repeats: usize,
    entry_repeats: Option<usize>,
    time_budget_s: Option<u64>,
    first_repeat_limit_s: u64,
    dump_solution: Option<String>,
    dump_pattern: Option<String>,
    counts_file: Option<String>,
    allow_over_estimate: bool,
    emit_model: bool,
    counts_only: bool,
    noop: bool,
    list_models: bool,
    // K6b (the `w1a` mode).
    case_limit: u64,
    invocation_limit: u64,
    w1_prefixes: bool,
    dump_published: Option<String>,
    emit_source: bool,
}

fn usage(message: &str) -> ! {
    eprintln!("k6_observe: {message}");
    eprintln!(
        "usage: k6_observe (--model <id> | --model-file <path>) --mode sparse|dense|lane-id|lane-lu \
         --heap-cap-bytes <n> [--repeats 5] [--entry-repeats <k>] [--time-budget-s <s>] \
         [--first-repeat-limit-s 600] [--counts-file <path>] [--dump-solution <path>] [--allow-over-estimate]\n       \
         k6_observe --emit-model --model <id>\n       \
         k6_observe --counts-only (--model <id> | --model-file <path>) --heap-cap-bytes <n> [--dump-pattern <path>]\n       \
         k6_observe --noop --heap-cap-bytes <n>\n       \
         k6_observe --list-models\n       \
         k6_observe (--model <id> | --model-file <path>) --mode w1a --heap-cap-bytes <n> [--repeats 5] \
         [--case-limit <u64>] [--invocation-limit <u64>] [--w1-prefixes] [--dump-published <path>] \
         [--counts-file <path>] [--time-budget-s <s>] [--first-repeat-limit-s 600]\n       \
         k6_observe --emit-source (--model <id> | --model-file <path>)"
    );
    std::process::exit(2)
}

fn parse_args() -> Args {
    let mut args = Args {
        model: None,
        model_file: None,
        mode: None,
        heap_cap: None,
        repeats: 5,
        entry_repeats: None,
        time_budget_s: None,
        first_repeat_limit_s: 600,
        dump_solution: None,
        dump_pattern: None,
        counts_file: None,
        allow_over_estimate: false,
        emit_model: false,
        counts_only: false,
        noop: false,
        list_models: false,
        case_limit: u64::MAX,
        invocation_limit: u64::MAX,
        w1_prefixes: false,
        dump_published: None,
        emit_source: false,
    };
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        let mut value = || {
            it.next()
                .unwrap_or_else(|| usage(&format!("{flag} needs a value")))
        };
        let number = |text: String| -> u64 {
            text.parse()
                .unwrap_or_else(|_| usage(&format!("not a number: {text}")))
        };
        match flag.as_str() {
            "--model" => args.model = Some(value()),
            "--model-file" => args.model_file = Some(value()),
            "--mode" => {
                let text = value();
                args.mode = Some(
                    Mode::parse(&text).unwrap_or_else(|| usage(&format!("unknown mode {text}"))),
                );
            }
            "--heap-cap-bytes" => args.heap_cap = Some(number(value()) as usize),
            "--repeats" => args.repeats = number(value()) as usize,
            "--entry-repeats" => args.entry_repeats = Some(number(value()) as usize),
            "--time-budget-s" => args.time_budget_s = Some(number(value())),
            "--first-repeat-limit-s" => args.first_repeat_limit_s = number(value()),
            "--dump-solution" => args.dump_solution = Some(value()),
            "--dump-pattern" => args.dump_pattern = Some(value()),
            "--counts-file" => args.counts_file = Some(value()),
            "--allow-over-estimate" => args.allow_over_estimate = true,
            "--emit-model" => args.emit_model = true,
            "--counts-only" => args.counts_only = true,
            "--noop" => args.noop = true,
            "--list-models" => args.list_models = true,
            "--case-limit" => args.case_limit = number(value()),
            "--invocation-limit" => args.invocation_limit = number(value()),
            "--w1-prefixes" => args.w1_prefixes = true,
            "--dump-published" => args.dump_published = Some(value()),
            "--emit-source" => args.emit_source = true,
            other => usage(&format!("unknown argument {other}")),
        }
    }
    args
}

fn load_model(args: &Args) -> (K6Model, ModelOrigin) {
    match (&args.model, &args.model_file) {
        (Some(id), None) => model_described(id).unwrap_or_else(|e| usage(&e)),
        (None, Some(path)) => {
            let text = std::fs::read_to_string(path)
                .unwrap_or_else(|e| usage(&format!("cannot read the model file: {e}")));
            canonical::parse_described(&text).unwrap_or_else(|e| usage(&e))
        }
        _ => usage("give exactly one of --model and --model-file"),
    }
}

fn compose_h(
    model: &K6Model,
    origin: ModelOrigin,
    counts: &K6Counts,
    captured: Option<Result<HSourceFacts, EnvelopeError>>,
    args: &Args,
) -> Option<HComposedEstimate> {
    counts.w1.as_ref().map(|w| {
        let source = captured
            .unwrap_or_else(|| HSourceFacts::from_counts(model, w))
            .unwrap_or_else(|e| usage(&format!("W1 source facts unavailable: {e:?}")));
        let facts = HModelFacts::capture(model, origin)
            .unwrap_or_else(|e| usage(&format!("W1 model facts unavailable: {e:?}")));
        h_envelope::estimate(
            facts,
            source,
            HLaunch {
                retained_arguments: [
                    args.model.as_deref(),
                    args.model_file.as_deref(),
                    args.counts_file.as_deref(),
                    args.dump_solution.as_deref(),
                    args.dump_pattern.as_deref(),
                    args.dump_published.as_deref(),
                ],
                repeats: args.repeats,
                prefixes: args.w1_prefixes,
            },
            &ReferenceHProfile::source40129_rust1971_aarch64_v1(),
        )
        .unwrap_or_else(|e| usage(&format!("W1 envelope unavailable: {e:?}")))
    })
}

// ------------------------------------------------------------------ observer

struct Observer {
    repeat: usize,
    started: Option<Instant>,
    current_begin: usize,
    calls_begin: u64,
    /// The largest stage peaks seen (the repeats' peak, without the counts
    /// and parity phases).
    max_stage_peak: usize,
    max_stage_peak_move: usize,
}

impl StageObserver for Observer {
    fn begin(&mut self, stage: Stage) {
        Line::new("stage_begin")
            .n("repeat", self.repeat)
            .s("stage", stage.as_str())
            .n("heap_current", alloc::current())
            .emit();
        alloc::stage_reset();
        self.current_begin = alloc::current();
        self.calls_begin = alloc::calls();
        self.started = Some(Instant::now());
    }
    fn end(&mut self, stage: Stage, ok: bool, error: Option<String>) {
        let elapsed = self
            .started
            .take()
            .map(|t| t.elapsed().as_nanos())
            .unwrap_or(0);
        let (peak, peak_move, current) = (
            alloc::stage_peak(),
            alloc::stage_peak_move(),
            alloc::current(),
        );
        let calls = alloc::calls() - self.calls_begin;
        self.max_stage_peak = self.max_stage_peak.max(peak);
        self.max_stage_peak_move = self.max_stage_peak_move.max(peak_move);
        Line::new("stage")
            .n("repeat", self.repeat)
            .s("stage", stage.as_str())
            .b("ok", ok)
            .n("elapsed_ns", elapsed)
            .n("heap_current_begin", self.current_begin)
            .n("heap_current_end", current)
            .n("heap_peak", peak)
            .n("heap_peak_move", peak_move)
            .n("alloc_calls", calls)
            .opt_s("error", error.as_deref())
            .emit();
    }
}

// ------------------------------------------------------------------ phases

fn refusal(reason: &str, estimate: Option<u128>) -> ! {
    Line::new("refusal")
        .s("reason", reason)
        .opt_n("estimate_adm_bytes", estimate)
        .n("heap_cap_bytes", alloc::cap())
        .n("heap_peak_so_far", alloc::peak())
        .n("heap_peak_move_so_far", alloc::peak_move())
        .emit();
    std::process::exit(3)
}

/// The model's canonical length and FNV-1a digest, streamed.
fn canonical_digest(model: &K6Model) -> (u64, u64) {
    let mut digest = Fnv64::new();
    canonical::write_canonical(model, &mut digest);
    (digest.len(), digest.finish())
}

fn counts_line(
    model: &K6Model,
    mode: Option<Mode>,
    c: &K6Counts,
    source: &str,
    phase: Option<(u128, usize, usize)>,
    w1_error: Option<&str>,
    composed: Option<&HComposedEstimate>,
) {
    let sizes = SizeFacts::of_this_build();
    let (canonical_len, canonical_fnv) = canonical_digest(model);
    let mut line = Line::new("counts")
        .s("counts_source", source)
        .s("model", &model.id)
        .s("family", model.family.as_str())
        .n("nodes", c.nodes)
        .n("members", c.members)
        .n("dofs", c.dofs)
        .n("free_dofs", c.free_dofs)
        .n("restrained_dofs", c.restrained_dofs)
        .n("pattern_entries", c.pattern_entries)
        .n("lower_entries", c.lower_entries)
        .n("free_entries", c.free_entries)
        .n("free_lower_entries", c.free_lower_entries)
        .n("free_lower_nonzero", c.free_lower_nonzero)
        .n("rcm_profile_entries", c.rcm_profile_entries)
        .n("rcm_half_bandwidth", c.rcm_half_bandwidth)
        .n("identity_profile_entries", c.identity_profile_entries)
        .n("identity_half_bandwidth", c.identity_half_bandwidth)
        .n("lane_entries", c.lane_entries)
        .n("lane_off_diagonal", c.lane_off_diagonal)
        .n("contributions", c.contributions)
        .n("dense_entries", c.dense_entries);
    for m in [Mode::Sparse, Mode::Dense, Mode::LaneId, Mode::LaneLu] {
        line = line
            .opt_n(
                &format!("estimate_f1b_bytes_{}", m.as_str().replace('-', "_")),
                f1b_estimate_bytes(m, c),
            )
            .n(
                &format!("estimate_adm_bytes_{}", m.as_str().replace('-', "_")),
                admission_estimate_bytes(m, c, &sizes, None).expect("non-W1 mode"),
            );
    }
    // K6b: W1's counts, storage and estimate (plan §3.4, §3.5), when the
    // counts carry them.
    // Under w1_source_ok=false the typed SourceRefused result emits positive
    // source-window max/sel128 and caller-only fixed. Its zero kernel fields
    // mean unexecuted phases, never unavailable proof or a successful kernel.
    if let Some(w) = &c.w1 {
        let e = &composed
            .expect("W1 fields require complete H composition")
            .legacy;
        line = line
            .null("estimate_f1b_bytes_w1a")
            .n("estimate_adm_bytes_w1a", e.max)
            .n("estimate_w1_sel128_bytes", e.sel128)
            .n("estimate_w1_fixed_bytes", e.fixed)
            .n("estimate_w1_decide_bytes", e.decide);
        for (k, &(p, _)) in LIMBS_PER_ENTRY.iter().enumerate() {
            line = line
                .n(&format!("estimate_w1_shared_{p}"), e.shared[k])
                .n(&format!("estimate_w1_state_{p}"), e.state[k])
                .n(&format!("estimate_w1_solve_{p}"), e.solve[k]);
        }
        for (k, p) in [256, 512, 1024].iter().enumerate() {
            line = line
                .n(&format!("estimate_w1_verify_{p}"), e.verify[k])
                .n(&format!("estimate_w1_pass_{p}"), e.pass[k]);
        }
        line = line
            .b("w1_source_ok", w.source_ok)
            .opt_s("w1_source_error", w1_error)
            .n("w1_nodes", w.nodes)
            .n("w1_members", w.members)
            .n("w1_stations", w.stations)
            .n("w1_constraints", w.constraints)
            .n("w1_loads", w.loads)
            .n("w1_dofs", w.dofs)
            .n("w1_free_dofs", w.free_dofs)
            .n("w1_bodies", w.bodies)
            .n("w1_pattern_entries", w.pattern_entries)
            .n("w1_profile_entries", w.profile_entries)
            .n("w1_half_bandwidth", w.half_bandwidth)
            .n("w1_blocks", w.blocks)
            .n("w1_rows", w.rows);
        for &(p, limbs) in &LIMBS_PER_ENTRY {
            line = line
                .n(&format!("w1_limbs_per_entry_{p}"), limbs)
                .n(&format!("w1_storage_bytes_{p}"), storage_bytes(w, p));
        }
        for limbs in [4u128, 8, 16] {
            line = line.n(&format!("w1_wide_bytes_{limbs}"), wide_bytes(limbs));
        }
        line = line
            .n("w1_source_encoding_len", w.source_encoding_len)
            .hex("w1_source_encoding_fnv64", w.source_encoding_fnv64);
    }
    line.opt_s("mode", mode.map(Mode::as_str))
        .n("size_stiffness_contribution", sizes.stiffness_contribution)
        .n("size_contribution_rounding", sizes.contribution_rounding)
        .n("size_residual_row", sizes.residual_row)
        .n("size_pivot_evidence", sizes.pivot_evidence)
        .n("size_frame_element", sizes.frame_element)
        .n("size_symmetric_matrix_entry", sizes.symmetric_matrix_entry)
        .n("model_canonical_len", canonical_len)
        .hex("model_canonical_fnv64", canonical_fnv)
        .opt_n("phase_elapsed_ns", phase.map(|p| p.0))
        .opt_n("phase_peak_heap", phase.map(|p| p.1))
        .opt_n("phase_peak_heap_move", phase.map(|p| p.2))
        .emit();
}

fn solution_digest(result: &Result<StructuralSolution, StructuralError>) -> (u64, u64) {
    debug_digest(result)
}

fn parity(item: &str, repeat: usize, equal: bool) -> Line {
    Line::new("parity")
        .s("item", item)
        .n("repeat", repeat)
        .b("equal", equal)
}

fn write_solution(path: &str, values: &[f64]) {
    let file = std::fs::File::create(path).unwrap_or_else(|e| {
        eprintln!("k6_observe: cannot write the solution dump: {e}");
        std::process::exit(4)
    });
    let mut out = std::io::BufWriter::new(file);
    for v in values {
        let _ = writeln!(out, "{:016x}", v.to_bits());
    }
    let _ = out.flush();
}

fn main() {
    let args = parse_args();
    if args.list_models {
        for id in sealed_model_ids() {
            println!("sealed {id}");
        }
        for id in extra_model_ids() {
            println!("extra {id}");
        }
        return;
    }
    if args.emit_model {
        let (model, _) = load_model(&args);
        print!("{}", canonical::serialize(&model));
        return;
    }
    if args.emit_source {
        // K6b (Q6): the adapter's K4SRC bytes, in hex, for the independent check.
        let (model, _) = load_model(&args);
        match w1_adapter::source(&model) {
            Ok(source) => {
                let hex: String = source
                    .encoding()
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect();
                println!("{hex}");
                return;
            }
            Err(e) => {
                eprintln!("k6_observe: the source is refused: {e:?}");
                std::process::exit(3)
            }
        }
    }
    let cap = args
        .heap_cap
        .unwrap_or_else(|| usage("--heap-cap-bytes is required"));
    if args.allow_over_estimate && cap > OVER_ESTIMATE_CAP_LIMIT {
        usage("--allow-over-estimate is accepted only with a heap cap of at most 512 MiB");
    }
    alloc::set_cap(cap);
    if args.noop {
        // The process baseline (ROOT's ruling at the A1 stop, Q2): the same
        // binary, the same allocator, no model and no stage.
        Line::new("start")
            .s("schema", SCHEMA)
            .n("pid", std::process::id())
            .b("noop", true)
            .n("heap_cap_bytes", cap)
            .emit();
        Line::new("summary")
            .n("repeats_completed", 0)
            .null("stop_reason")
            .n("heap_peak", alloc::peak())
            .n("heap_peak_move", alloc::peak_move())
            .emit();
        return;
    }
    let mode = if args.counts_only {
        args.mode
    } else {
        Some(args.mode.unwrap_or_else(|| usage("--mode is required")))
    };
    let entry_repeats = args.entry_repeats.unwrap_or(args.repeats);
    let start = Line::new("start")
        .s("schema", SCHEMA)
        .n("pid", std::process::id())
        .opt_s("model", args.model.as_deref())
        .b("model_from_file", args.model_file.is_some())
        .opt_s("mode", mode.map(Mode::as_str))
        .b("counts_only", args.counts_only)
        .n("repeats", args.repeats)
        .n("entry_repeats", entry_repeats)
        .n("heap_cap_bytes", cap)
        .opt_n("time_budget_s", args.time_budget_s)
        .n("first_repeat_limit_s", args.first_repeat_limit_s)
        .b("allow_over_estimate", args.allow_over_estimate);
    let start = if mode == Some(Mode::W1a) {
        start
            .n("case_limit", args.case_limit)
            .n("invocation_limit", args.invocation_limit)
            .b("w1_prefixes", args.w1_prefixes)
            .b("dump_published", args.dump_published.is_some())
    } else {
        start
    };
    start.emit();
    let (model, model_origin) = load_model(&args);

    // The refusals by name (host rule; ROOT's rulings Q12 and on RV16-N4),
    // before any count or n² allocation.
    if let Some(mode) = mode {
        if !args.counts_only {
            if mode.materializes_n2() && model.member_count() >= N2_REFUSAL_MEMBERS {
                refusal("n2_mode_at_or_above_10000_members", None);
            }
            if mode == Mode::LaneId && model.is_cont_n10000() {
                refusal("cont_n10000_identity_lane", None);
            }
        }
    }

    let frames = model.frames().unwrap_or_else(|e| {
        eprintln!("k6_observe: frame formation failed: {e:?}");
        std::process::exit(4)
    });

    // The counts: from the counts-only record (`--counts-file`, so the
    // observed process carries no counts phase), or computed here in O(nnz)
    // memory, with no n² array and no profile values.
    let (counts, phase, h_estimate) = match &args.counts_file {
        Some(path) if !args.counts_only => {
            let text = std::fs::read_to_string(path)
                .unwrap_or_else(|e| usage(&format!("cannot read the counts file: {e}")));
            let (_, canonical_fnv) = canonical_digest(&model);
            let found = text
                .lines()
                .filter_map(parse_counts_line)
                .find(|(id, _, _)| *id == model.id)
                .unwrap_or_else(|| {
                    usage(&format!(
                        "no counts line for {} in the counts file",
                        model.id
                    ))
                });
            if found.2 != canonical_fnv {
                usage("the counts line's model digest differs from this model's canonical bytes");
            }
            if mode == Some(Mode::W1a) && found.1.w1.is_none() {
                usage("the counts line carries no W1 counts (run --counts-only on this binary)");
            }
            let estimate = compose_h(&model, model_origin, &found.1, None, &args);
            counts_line(
                &model,
                mode,
                &found.1,
                "file",
                None,
                None,
                estimate.as_ref(),
            );
            (found.1, None, estimate)
        }
        _ => {
            alloc::stage_reset();
            let started = Instant::now();
            let mut file = args.dump_pattern.as_ref().map(|path| {
                std::io::BufWriter::new(std::fs::File::create(path).unwrap_or_else(|e| {
                    eprintln!("k6_observe: cannot write the pattern dump: {e}");
                    std::process::exit(4)
                }))
            });
            let sink = file.as_mut().map(|f| f as &mut dyn Write);
            let mut counts = compute(&model, &frames, sink).unwrap_or_else(|e| {
                eprintln!("k6_observe: counts failed: {e}");
                std::process::exit(4)
            });
            // K6b: W1's counts in the counts-only record and the w1a mode.
            let mut w1_error = None;
            let mut captured = None;
            if args.counts_only || mode == Some(Mode::W1a) {
                let (w1, error, source_facts) = w1_compute(&model);
                captured = Some(source_facts);
                counts.w1 = Some(w1);
                w1_error = error;
            }
            if let Some(f) = file.as_mut() {
                let _ = f.flush();
            }
            drop(file);
            let phase = (
                started.elapsed().as_nanos(),
                alloc::stage_peak(),
                alloc::stage_peak_move(),
            );
            let estimate = compose_h(&model, model_origin, &counts, captured, &args);
            counts_line(
                &model,
                mode,
                &counts,
                "computed",
                Some(phase),
                w1_error.as_deref(),
                estimate.as_ref(),
            );
            (counts, Some(phase), estimate)
        }
    };
    let sizes = SizeFacts::of_this_build();

    let Some(mode) = mode.filter(|_| !args.counts_only) else {
        Line::new("summary")
            .n("repeats_completed", 0)
            .null("stop_reason")
            .n("heap_peak", alloc::peak())
            .n("heap_peak_move", alloc::peak_move())
            .emit();
        return;
    };

    let estimate = admission_estimate_bytes(
        mode,
        &counts,
        &sizes,
        h_estimate.as_ref().map(|h| &h.legacy),
    )
    .unwrap_or_else(|e| usage(&format!("admission estimate unavailable: {e:?}")));
    if estimate > (cap as u128) / 2 && !args.allow_over_estimate {
        refusal("estimate_exceeds_half_cap", Some(estimate));
    }

    let mut observer = Observer {
        repeat: 0,
        started: None,
        current_begin: 0,
        calls_begin: 0,
        max_stage_peak: 0,
        max_stage_peak_move: 0,
    };
    let run_started = Instant::now();
    let mut repeats_completed = 0;
    let mut stop_reason: Option<&str> = None;
    let mut first_digest: Option<(u64, u64)> = None;
    let mut determinism = true;
    let mut repeat0_solution: Option<Vec<f64>> = None;
    let mut internal_error = false;
    // K6b: the limits (ROOT's Q3 ruling: u64::MAX, recorded in `start`), the
    // last full call's attempts (for the prefixes) and a source refusal.
    let w1_limits = W1Limits {
        case: args.case_limit,
        invocation: args.invocation_limit,
    };
    let mut w1_last_attempts: Option<Vec<AttemptRecord>> = None;
    let mut source_refused = false;
    for repeat in 0..args.repeats {
        observer.repeat = repeat;
        let repeat_started = Instant::now();
        let with_entries = repeat < entry_repeats;
        let digest = match mode {
            Mode::Sparse | Mode::Dense => {
                let setup_result = setup(&model, &frames, &mut observer);
                match setup_result {
                    Err(error) => {
                        Line::new("outcome")
                            .n("repeat", repeat)
                            .s("class", "SetupError")
                            .s("failed_stage", error.stage.as_str())
                            .s("detail", &error.detail)
                            .emit();
                        internal_error = true;
                        (0, 0)
                    }
                    Ok(setup) => {
                        let solve = staged_solve(
                            &setup,
                            model.node_count(),
                            &frames,
                            mode,
                            true,
                            &mut observer,
                        );
                        let digest = solution_digest(&solve.solution);
                        let mut recovered = None;
                        if let Ok(solution) = &solve.solution {
                            recovered = Some(recovery(
                                &setup,
                                &frames,
                                &solution.displacements,
                                &mut observer,
                            ));
                            if repeat == 0 && args.dump_solution.is_some() {
                                repeat0_solution = Some(solution.displacements.clone());
                            }
                        }
                        Line::new("outcome")
                            .n("repeat", repeat)
                            .s("class", outcome_class(&solve.solution))
                            .opt_s("failed_stage", solve.failed_stage.map(Stage::as_str))
                            .opt_s(
                                "factor_error",
                                solve.factor_error.as_ref().map(short_error).as_deref(),
                            )
                            .opt_s(
                                "error",
                                solve.solution.as_ref().err().map(short_error).as_deref(),
                            )
                            .b(
                                "formation_demoted",
                                solve
                                    .solution
                                    .as_ref()
                                    .is_ok_and(|s| s.formation_check.is_some()),
                            )
                            .b(
                                "load_fidelity_flagged",
                                solve
                                    .solution
                                    .as_ref()
                                    .is_ok_and(|s| s.load_fidelity.is_some()),
                            )
                            .opt_n(
                                "contribution_rounding_rows",
                                solve
                                    .solution
                                    .as_ref()
                                    .ok()
                                    .map(|s| s.report.contribution_rounding.len()),
                            )
                            .opt_n("recovery_ok", recovered.map(|r| r.0))
                            .opt_s(
                                "recovery_digest",
                                recovered.map(|r| format!("{:016x}", r.1)).as_deref(),
                            )
                            .n("solution_debug_len", digest.0)
                            .hex("solution_debug_fnv64", digest.1)
                            .emit();
                        if let Some((profile, bandwidth)) = solve.ordering_counts {
                            parity("rcm_count_equals_ordering", repeat, {
                                profile as u128 == counts.rcm_profile_entries
                                    && bandwidth == counts.rcm_half_bandwidth
                            })
                            .n("ordering_profile_entries", profile)
                            .n("ordering_half_bandwidth", bandwidth)
                            .emit();
                        }
                        if with_entries {
                            let checked = entry(&setup, mode, true, &mut observer);
                            let checked_digest = solution_digest(&checked);
                            parity("staged_vs_entry_checked", repeat, checked_digest == digest)
                                .s("entry_class", outcome_class(&checked))
                                .n("entry_debug_len", checked_digest.0)
                                .hex("entry_debug_fnv64", checked_digest.1)
                                .emit();
                            drop(checked);
                            let plain = entry(&setup, mode, false, &mut observer);
                            let plain_digest = solution_digest(&plain);
                            parity(
                                "entry_plain_vs_entry_checked",
                                repeat,
                                plain_digest == checked_digest,
                            )
                            .s("entry_plain_class", outcome_class(&plain))
                            .n("entry_plain_debug_len", plain_digest.0)
                            .hex("entry_plain_debug_fnv64", plain_digest.1)
                            .emit();
                        }
                        digest
                    }
                }
            }
            Mode::LaneId | Mode::LaneLu => {
                let lane = if mode == Mode::LaneId {
                    lane_id(&model, &frames, &mut observer)
                } else {
                    lane_lu(&model, &frames, &mut observer)
                };
                match lane {
                    Err(error) => {
                        Line::new("outcome")
                            .n("repeat", repeat)
                            .s("class", "SetupError")
                            .s("failed_stage", error.stage.as_str())
                            .s("detail", &error.detail)
                            .emit();
                        internal_error = true;
                        (0, 0)
                    }
                    Ok(lane) => {
                        let mut line = Line::new("outcome")
                            .n("repeat", repeat)
                            .s("class", if lane.ok { "LaneSolved" } else { "LaneError" })
                            .opt_s("error", lane.error.as_deref())
                            .n("solution_len", lane.solution_len)
                            .hex("solution_fnv64", lane.solution_digest);
                        if let Some((original, ordered, original_bw, ordered_bw, entries)) =
                            lane.profile
                        {
                            line = line
                                .n("original_profile_entries", original)
                                .n("ordered_profile_entries", ordered)
                                .n("original_half_bandwidth", original_bw)
                                .n("ordered_half_bandwidth", ordered_bw)
                                .n("lane_entries", entries);
                        }
                        line.emit();
                        if let Some((original, _, original_bw, _, entries)) = lane.profile {
                            parity("lane_identity_profile_equals_count", repeat, {
                                original as u128 == counts.identity_profile_entries
                                    && original_bw == counts.identity_half_bandwidth
                                    && entries == counts.lane_entries
                            })
                            .emit();
                        }
                        (lane.solution_len as u64, lane.solution_digest)
                    }
                }
            }
            Mode::W1a => {
                let w1_counts = counts.w1.as_ref().expect("the w1a mode carries W1 counts");
                match w1_source(&model, &mut observer) {
                    Err(error) => {
                        Line::new("outcome")
                            .n("repeat", repeat)
                            .s("class", "SourceRefused")
                            .s("reason", &format!("{error:?}"))
                            .emit();
                        source_refused = true;
                        (0, 0)
                    }
                    Ok(source) => {
                        let solve = w1_solve(source, w1_limits, Stage::W1Solve, &mut observer);
                        w1::outcome_line(repeat, &solve).unwrap_or_else(work_failure);
                        w1::attempt_lines(repeat, attempts_of(&solve.outcome))
                            .unwrap_or_else(work_failure);
                        if repeat == 0 {
                            w1::parity_lines(repeat, w1_counts, &solve)
                                .unwrap_or_else(work_failure);
                            if let Some(path) = &args.dump_published {
                                w1::dump_rows(path, &model.id, &solve);
                            }
                        }
                        w1_last_attempts = Some(attempts_of(&solve.outcome).to_vec());
                        w1::repeat_digest(&solve).unwrap_or_else(work_failure)
                    }
                }
            }
        };
        repeats_completed += 1;
        match first_digest {
            None => first_digest = Some(digest),
            Some(first) => determinism &= first == digest,
        }
        if internal_error {
            stop_reason = Some("setup_error");
            break;
        }
        if source_refused {
            stop_reason = Some("source_refused");
            break;
        }
        let repeat_elapsed = repeat_started.elapsed();
        if repeat == 0 && repeat_elapsed.as_secs() >= args.first_repeat_limit_s {
            stop_reason = Some("first_repeat_over_limit");
            break;
        }
        if let Some(budget) = args.time_budget_s {
            let projected = run_started.elapsed() + repeat_elapsed;
            if repeat + 1 < args.repeats && projected.as_secs() >= budget {
                stop_reason = Some("time_budget");
                break;
            }
        }
    }
    if repeats_completed > 1 {
        parity("repeat_determinism", repeats_completed - 1, determinism).emit();
    }

    // Bitwise K (dense process only, after the repeats, up to 1,000 members).
    let mut parity_peak = None;
    if mode == Mode::Dense && model.member_count() <= BITWISE_K_MEMBERS {
        alloc::stage_reset();
        let sparse = assemble_sparse_stiffness(
            model.node_count(),
            &frames,
            &[],
            &[],
            &[],
            &SparseAssemblyOptions::new(),
        );
        let dense = assemble_global_stiffness(model.node_count(), &frames);
        match (sparse, dense) {
            (Ok(sparse), Ok(dense)) => {
                let result = bitwise_k(&sparse, &dense);
                parity("bitwise_k", 0, result.equal())
                    .n("stored_entries", result.stored_entries)
                    .n("stored_mismatches", result.stored_mismatches)
                    .n("unstored_nonzero_bits", result.unstored_nonzero_bits)
                    .emit();
            }
            (sparse, dense) => {
                parity("bitwise_k", 0, false)
                    .opt_s("error", sparse.err().map(|e| format!("{e:?}")).as_deref())
                    .opt_s(
                        "dense_error",
                        dense.err().map(|e| format!("{e:?}")).as_deref(),
                    )
                    .emit();
            }
        }
        parity_peak = Some((alloc::stage_peak(), alloc::stage_peak_move()));
    }
    if let (Some(path), Some(values)) = (&args.dump_solution, &repeat0_solution) {
        write_solution(path, values);
    }

    // K6b (Q1(c)): the budget-truncated prefixes, after the timed repeats and
    // outside the repeats' peak. Each prefix re-runs `solve_case` under the
    // case work charged through one segment of the last full call.
    let mut prefix_peak = None;
    if mode == Mode::W1a && args.w1_prefixes {
        if let Some(full) = &w1_last_attempts {
            let saved = (observer.max_stage_peak, observer.max_stage_peak_move);
            observer.repeat = repeats_completed;
            let (mut peak, mut peak_move) = (0, 0);
            for (j, (label, limit)) in prefix_limits(full)
                .unwrap_or_else(work_failure)
                .iter()
                .enumerate()
            {
                let Ok(source) = w1_adapter::source(&model) else {
                    break;
                };
                let limits = W1Limits {
                    case: *limit,
                    invocation: args.invocation_limit,
                };
                let stage = Stage::W1Prefix(u8::try_from(j + 1).unwrap_or(u8::MAX));
                let solve = w1_solve(source, limits, stage, &mut observer);
                peak = peak.max(alloc::stage_peak());
                peak_move = peak_move.max(alloc::stage_peak_move());
                w1::prefix_line(j + 1, label, *limit, full, &solve).unwrap_or_else(work_failure);
            }
            observer.max_stage_peak = saved.0;
            observer.max_stage_peak_move = saved.1;
            prefix_peak = Some((peak, peak_move));
        }
    }
    let summary = Line::new("summary")
        .n("repeats_completed", repeats_completed)
        .opt_s("stop_reason", stop_reason)
        .n("repeats_heap_peak", observer.max_stage_peak)
        .n("repeats_heap_peak_move", observer.max_stage_peak_move)
        .opt_n("counts_phase_heap_peak", phase.map(|p| p.1))
        .opt_n("counts_phase_heap_peak_move", phase.map(|p| p.2))
        .opt_n("parity_phase_heap_peak", parity_peak.map(|p| p.0))
        .opt_n("parity_phase_heap_peak_move", parity_peak.map(|p| p.1))
        .n("heap_peak", alloc::peak())
        .n("heap_peak_move", alloc::peak_move())
        .n("elapsed_ns", run_started.elapsed().as_nanos());
    let summary = if mode == Mode::W1a {
        summary
            .opt_n("prefix_phase_heap_peak", prefix_peak.map(|p| p.0))
            .opt_n("prefix_phase_heap_peak_move", prefix_peak.map(|p| p.1))
    } else {
        summary
    };
    summary.emit();
    if internal_error {
        std::process::exit(4);
    }
}

fn work_failure<T>(fault: open_pipe_stress_frame_kernel::structural::retained_api::WorkFault) -> T {
    eprintln!("k6_observe: retained work accounting {fault}");
    std::process::exit(4)
}
