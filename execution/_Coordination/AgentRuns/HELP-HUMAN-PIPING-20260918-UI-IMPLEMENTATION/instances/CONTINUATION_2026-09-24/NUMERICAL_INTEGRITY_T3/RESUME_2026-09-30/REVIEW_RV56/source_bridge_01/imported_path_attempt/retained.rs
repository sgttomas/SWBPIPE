//! W1's retained-precision structural method (T3 D1 §4.1, `FK/structural/retained/`).
//!
//! Slice K3a adds only the arithmetic the D-5 formation check (K-D5) needs:
//! `wide::Wide<2>` with correctly rounded + − × ÷ √ at a runtime precision
//! p ≤ 128, the exact lift from binary64, the exact split into at most three
//! binary64 terms for `ExactAccumulator::add_product`, the included-angle
//! arctangent, and the work counter. Its first caller is K-D5's formation
//! check (`structural/formation_check.rs`).
//!
//! Slice K3 adds, in `wide/multi.rs` beside K3a's unchanged `Wide<2>` code:
//! `Wide<L>` at L = 4, 8 and 16 with correctly rounded + − × ÷ √ at a runtime
//! precision p ≤ 64L (`WideContext<L>`), the rounded conversion to binary64
//! with its outcome at every width (L = 2 included), exact widening, rounded
//! narrowing, TwoSum and TwoProduct, the integer constructor, and per-width
//! work counts (`AttemptWork`). Its first caller is K4.

// K-D5 uses `Wide2`, `WideArith` (add, sub, mul, div, sqrt, included_angle),
// `from_f64`, `split_binary64`/`add_product_to`, `mul_pow2`, `neg`, `abs`,
// `cmp_value`, `is_zero` and `WideError`. The 12 items of K3a's tested surface
// with no non-test caller (13 before K3, whose `multi` constructs
// `NotNormalized`) carry their own `#[allow(dead_code)]` in `wide.rs`,
// each with its reason (test-only, or the slice that will call it). K3's
// `wide/multi.rs` items follow the same rule (K4 API).
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/wide.rs"] pub(crate) mod wide;

// K4 (the W1a kernel method, T3 D1 §4.1; ROOT's K4 rulings): the correctly
// rounded exact multi-term sum, the source, the ledger at p, formation and
// assembly, the factor and its screens, recovery, combinations and the adaptive
// schedule. K4 has no product caller (ROOT's K4 ruling Q1): F2a wires W1. Each
// entry point without a non-test caller carries its own `#[allow(dead_code)]`
// naming its consumer (F2a API or V-K API); the helpers they reach are live.
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/REVIEW_RV56/source_bridge_01/imported/adaptive.rs"] pub(crate) mod adaptive;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/assemble.rs"] pub(crate) mod assemble;
// D1 revision 5a.3 (R7 §4.1.6.1 item 6a, §4.1.6.2, §4.1.6.3): directed wide
// rounding, the certified inverse-norm bounds and the verification.
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/bound.rs"] pub(crate) mod bound;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/combine.rs"] pub(crate) mod combine;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/directed.rs"] pub(crate) mod directed;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/factor.rs"] pub(crate) mod factor;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/ledger.rs"] pub(crate) mod ledger;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/recover.rs"] pub(crate) mod recover;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/product_certificate.rs"] mod product_certificate;
// V-K's seeded faults (T3 D1 §4.10, §7.3): compiled only for tests and under
// the `mutation-controls` feature, which only numerical_robustness's mutation
// run enables; inactive unless `FK_SEEDED_FAULT` names a fault.
#[cfg(any(test, feature = "mutation-controls"))]
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/seeded.rs"] pub(crate) mod seeded;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/source.rs"] pub(crate) mod source;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/verify.rs"] pub(crate) mod verify;
#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/wide_sum.rs"] pub(crate) mod wide_sum;

#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/work.rs"] pub(crate) mod work;

#[path="/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/origins.rs"] pub(crate) mod origins;
