# I1 — implement slice S11-K (exact load sums, kernel side)

Implementation TASK. Read `_COMMON.md` first; this brief overrides it where they differ (writes, builds).

## Purpose

Implement slice S11-K of the selected S11 containment. S11-K is T1-disjoint and lands as its own full-gate PR after T1 merges. Its job: exact, correctly rounded load sums in the kernel and in the straight-pipe, curved-bend and combination recovery paths; the exact prescribed-motion right-hand side (KS1–KS3); the ledger and typed-force seams added beside today's entry points; and the kernel-side tests and mutations.

It must never publish a silently wrong value, strip a load or feature, skip a test or raise a timeout.

## Governing basis (read in this order)

1. [ROOT_SELECTION_S11.md](../ROOT_SELECTION_S11.md): the selection and its conditions R3-1 to R3-4.
2. [DESIGN_NUMERICS/S11_CONTAINMENT.md](../DESIGN_NUMERICS/S11_CONTAINMENT.md) **revision 3** (`d6575c25e`, sha256 `561c7200…`): §4 (the rule), §4.6 (KS1–KS3), §5.3 (the invariant), §8.1 (S11-K write set and live effect), §8.3 (disclosure and the fixture stop rule), §9 (K1–K12 and the mutation table).
3. The S11 rulings in [ROOT_RULINGS_V1.md](../ROOT_RULINGS_V1.md): D-S11-1 (+0.0 everywhere; the zero witness only at `FK/structural.rs:554`), D-S11-2, D-S11-3, D-S11-4, S11B-1, and the verbatim no-interim ruling.
4. V1's reviews for context: [REVIEW/S11_CHECK.md](../REVIEW/S11_CHECK.md), [S11_BACKCHECK.md](../REVIEW/S11_BACKCHECK.md) and [S11_BACKCHECK_R3.md](../REVIEW/S11_BACKCHECK_R3.md).

D1 is writing a narrow revision 4 of S11 that carries the R3 fixes. **Build to revision 3 plus the R3 items in `ROOT_SELECTION_S11.md`.** If revision 4 or V1's check of it changes anything, the manager sends you the delta, and it is applied before the PR merges.

## Base, worktree and branch

- The work happens in a dedicated worktree, `<s11k-worktree>`, on branch `codex/piping-s11k-20260926`. It is created from the T3 branch and then merged with T1's head `e43412a9b`, so the fixture diffs against T1's committed fixtures are known during development. ROOT gives the path.
- You make no Git writes. The manager commits your work on that branch, and merges T1 updates or main (after T1 merges) as they arrive.
- Never read, build in or write to T1's worktree or branch.

## Write set (from S11 §8.1)

- `P/core/solver/frame_kernel/src/exact_sum.rs` and `load_ledger.rs` (new), and `frame_kernel/src/lib.rs` (their module declarations; `reduce_system*`'s exact right-hand side KS2; typed entry points beside today's `&[f64]` ones).
- `P/core/solver/frame_kernel/src/structural.rs`:
  - `rounded()` replaced by the correctly rounded function at `:369-377`, `:406-414` and `:554` (with the -0.0 witness at `:554` only);
  - the audit taking exact per-DOF force terms;
  - KS1 in `prepare_structural` (`:603-606`);
  - KS3, the exact numerator in `evaluate_original_residual` (`:697-760`) on prescribed-coupled rows;
  - a typed `StructuralSystem` constructor.
- `P/core/solver/frame_kernel/src/structural/exact_boundary.rs:361`, `:387`.
- `P/core/solver/nonlinear_integration/src/structural_adapter.rs`: `AssemblyEvidence::with_force_terms`, and a typed `StructuralAssembly::solve` beside today's.
- `P/core/solver/nonlinear_integration/src/lib.rs`: typed variants of the entry points at `:2015`, `:2045` and `:2242`, **signatures only**.
- `P/core/loads/primitive_loads/src/lib.rs`: re-exports of the ledger types.
- `P/core/solver/straight_pipe/src/lib.rs`: E1 to E4, E6, and `equivalent_nodal_load_terms_with_spans`.
- `P/core/solver/curved_bend/src/lib.rs`: `arc_section_resultant_terms`.
- `P/core/loads/load_case_algebra/src/lib.rs` and its `Cargo.toml` (E13, plus a path dependency on `frame_kernel`), and the lockfiles that dependency change requires. Report every lockfile you change.
- `P/core/product_physics/src/pressure_sum.rs` (the wrapper only).
- Tests in those crates, and the site-test constant table (R3-3) where it lives in S11-K's crates.
- Records: `T3/IMPLEMENTATION/S11K/**` in `<s11k-worktree>`.

**Out of scope (S11-F, after T1's merge):** `P/core/product_physics/src/lib.rs`, `source_recovery.rs`, `source_receipt*`, `pressure_runtime.rs`, and every other `PP` producer. `PP` must compile unchanged against S11-K. The new typed entry points sit beside the old ones.

**Arithmetic dependency.** None new. The accumulator is in-repo (`exact_sum.rs`), generalized from `pressure_sum::exact_sum`: 68 limbs, quantum 2^-2148 once products are admitted, rounding at the binary64 subnormal quantum without the copy-bits shortcut, and +0.0 for an exact zero and for an underflowed nonzero net (S11 §4.1).

## The R3 conditions, as they apply to S11-K

- **R3-1.** The "bit-identical" claim holds only for all-zero prescribed values. T1's fixtures with nonzero support motion (`load_reference/connected*`, `load_reference_source/eigen_motion*` and their derived results documents) are expected to diff. Measure and report each diff's size.
- **R3-2.** K4 includes an axial-effect case (thermal or thrust, (G, n, −G)) so that M1b (E2) and the axial half of E3 are killed. F8 belongs to S11-F, not to you.
- **R3-3.** The §2.5 table becomes the site test's constant: every floating-point compound assignment or sum/fold in the named modules appears in it, as an E-site or as an explicit non-load exemption. Implement the part of that test that covers S11-K's crates (`FK`, `SA`, `nonlinear_integration` signatures, `SP`, `CB`, `load_case_algebra`). Allow-list the `nonlinear_integration:1333` unit-force influence solves as T5's. The `PP` part comes with S11-F.
- **R3-4.** KS1 and KS3 scale before rounding, or you record a proof that double rounding cannot occur there. The M1 labels match the E-sites.

## Tests and mutations

- **Tests.** Implement K1 to K12 from S11 §9, with the precondition rule: every test meant to kill a fold mutation first asserts, inside the test, that the binary64 fold differs from the correctly rounded net. Expected values come from `Fraction`-style exact references written in the test, or from the frozen T3 references. They never come from the code under test.
- **Mutations.** Run every mutation in S11 §9 whose site is in S11-K's write set: M1 for E1–E4, E6 and E13; M6; M7; M10; M11; M12; M13; M14; M15. The required kill set is G = 1e8 and 1e80, one mutant per E-site. Record each mutant's patch, the command, and the killing test. A survivor is a defect to report, never a reason to edit the test's expectation.
- **Existing suites.** Run the full test suites of every crate you touch, and of every crate that depends on them by path. Find the dependents from the `Cargo.toml` files. They include at least `product_physics`, `sparse_direct`, `nonlinear_integration`, `straight_pipe`, `curved_bend`, `primitive_loads`, `load_case_algebra`, `runner/headless` and `reporting/result_export`. Also run `apps/desktop/src-tauri`, which builds on this host. No test is skipped, filtered out or given a longer timeout. A test that fails on the base too is recorded as a pre-existing failure, with its base result.
- **Fixture diff (S11 §8.3).** Run every committed request under `P/fixtures/**` through the base (`e43412a9b`) and the candidate, in both modes. Attach the diff and a summary by output kind. Apply the stop rule:
  - the pre-registered T1 support-motion fixtures are expected diffs, and are reported with their sizes;
  - **any other committed byte change stops the work**, and is reported to the manager with its site and reason before anything is regenerated;
  - nothing is regenerated without ROOT's decision, and then only by the actual producer;
  - frozen references and historical raws never change.

## Build rules

- `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`, `CARGO_TARGET_DIR=<t3-target>`, `--offline --locked` (after any intended lockfile change).
- **Run one heavy cargo job at a time across T3.** Before every cargo build or test, check `pgrep -x cargo`, and wait while any other cargo process runs. T1's final review and P1 may be using the host.
- Keep free disk above about 8 GB. Prune only your own output.
- Run `cargo fmt` on the files you change, and `git diff --check` in `<s11k-worktree>`.

## Disclosure

Draft the PR-body change record (S11 §8.3) in `T3/IMPLEMENTATION/S11K/CHANGE_RECORD.md`. It states:
- which quantities change bits and why;
- the bound in the non-cancelling case (at most one rounding of the gross);
- that no case changes status unless it was absorbing a load;
- the measured fixture diff;
- that there is no in-band marker.

Follow `.agents/skills/chirality-change/SKILL.md` for the PR record format.

## Return

`T3/IMPLEMENTATION/S11K/RETURN.md` in `<s11k-worktree>`, containing:
- the files changed, with line counts;
- how each §8.1 item and each R3 condition was met;
- the test results per crate, with counts and pre-existing failures;
- the mutation table (mutant, result, killing test);
- the fixture diff summary, and any stop-rule report;
- lockfile changes;
- the toolchain;
- what was not done.

Put logs under `T3/IMPLEMENTATION/S11K/_run_records/`, with `SHA256SUMS`, and no machine paths. Send the manager a SendMessage summary when done. Also message the manager at once if the stop rule triggers, or if a design item cannot be implemented as specified. Don't improvise a different design.
