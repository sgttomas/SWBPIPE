# I21: implement slice K6c (W1's E_max as an upper bound on every phase of the post-KF3 kernel; W1-T4 re-run post-KF3; VR's estimate deduplicated)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration", `I8R_K1_RESUME.md:24-50`) override `_COMMON.md`'s host section, and apply to you in full.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Paths and bases

- `P/`, `T3/`, `FK`, `H` (`P/core/solver/performance_harness`), `K4R` (`FK/src/structural/retained/`) and `K4T` are as in `I16_K6B_IMPLEMENTATION.md` and `I19_KF3_IMPLEMENTATION.md`. `VR` is `P/validation/benchmarks/numerical_robustness`.
- **The base is main after KF3's merge.** ROOT records the actual base at spawn. T3's records are read in `<wt>/numerics`.
- **Branch** `codex/piping-k6c-<date>`, in `<wt>/k6c`.

## Purpose

ROOT's W1 limits rest on K6b's E_max being an upper bound on W1's heap in every phase, derived from the code ("K6b: A1 accepted"; the ruling on RV22-2). KF3's scale run found that it is not ("KF3: checkpoint B accepted; two findings for D", KF3-B2; "KF3: D accepted; KF3-B1 and KF3-B2 routed; PR to review"):
- **The measurement:** on RF-LARGE-TREE-n10000-AX and -ROT, the heap peaks inside `nl_pass` on the shifted factor in the 1024 verification (`K4R/bound.rs:1040` at KF3's head). The peak exceeds K6b's final E_max by 19.5 MB and 19.4 MB. **[Correction (ROOT, 2026-09-30, RV25-S1): this comparison is not like for like. It uses `k6_observe`'s fixed term against `vk_scale`'s measured peak. On `vk_scale`'s own fixed term, K6b's final formula bounds both peaks, by 7.0 and 7.5 MB (KF3 RETURN §13). The finding rests on the code derivation below (a net under-count of 10,799,688 B at the 1024 shift), not on a measured excess. So E_max is not yet *shown* to bound that phase; it has not been shown to fail it either. [Correction (ROOT, 2026-09-30, RV25-D1): the finding is stronger than this bracket says. By construction, K6b's formula does not bound this phase: taken from the code, the phase needs 3,021,565,490 B (AX) against E_max 3,010,765,802 B (KF3 RETURN §13, :440-445). Only a *measured* excess is unshown; the `vk_scale` peak stays inside E_max through slack elsewhere, not through this term.] On K6c's base (main), the `nl_pass` call in `shift_schedule` is at `K4R/bound.rs:1059`.]**
- **I19's derivation** (`T3/IMPLEMENTATION/KF3/RETURN.md` §13; `_run_records/d/b2_emax.{py,txt}`):
  - H's pass term leaves out `nl_pass`'s `at`, `bt` and `ct` (3·n_f·w);
  - it counts `work`, which is freed before `nl_pass` runs;
  - `OPTION_EXTRA` (`H/src/k6/w1/counts.rs:259`) over-counts 8 B per entry, since `Option<Wide<L>>` is the same size as `Wide<L>` on rustc 1.97.1;
  - net, the 1024 shift is under-counted by 10,799,688 B.
- **Why no earlier run caught it:** the allocations predate KF3. KF3 made the path reachable: amendment A2 turns a refused Uc into a shifted factorization where the attempt used to stop.
- **Not traced:** whether `uc_bounds`' transients overlap H's 214.7 MB transient term.
- **VR's copy:** VR carries a stale port of E_max (`VR/src/scale.rs`, from K6b's `082990c8d`, before RV22). It is 153 MB below K6b's final formula, and V-K's runner admits with it.
- **W1-T4 is still pre-KF3:** K6b's figures at 10,000 members predate KF3 (K6b RETURN addendum 1, reserved). KF3's B shows three more frames selected, and the TREE frames escalating to 1024, which gives the first measured 512 and 1024 memory and work.

## Scope

1. **Re-derive E_max against the post-KF3 kernel, phase by phase** (checkpoint 0, with no code).
   - For every phase K6b's `estimate` models, and any phase KF3 made reachable (the shifted factorization at every precision, and each refusal's per-block slots), list what is alive at the phase's peak, with file:line in K4R at the base.
   - Include the overlap question (`uc_bounds`' `c` with `at`, `bt` and `ct`, against the transient term).
   - State the corrected formula and its change per phase, at 10, 100, 1,000 and 10,000 members.
   - **The bar:** E_max is at least the measured heap peak on every recorded W1 run, KF3's B included, and the derivation shows why for runs not measured. **Compare like for like:** use the fixed term of the binary that measured the peak (`k6_observe` or `vk_scale`), and say how that fixed term is derived (RV25-S1).
2. **Implement the corrected estimate** in `H/src/k6/w1/counts.rs`.
   - Regenerate `H/observations/k6b/counts.jsonl`, and keep RV22-3's tests binding the committed lines to the code.
   - Add a test that fails on each omitted term I19 found, and one that asserts the `Option` size claim, if it can be asserted from H. Otherwise, show where it is asserted.
3. **Deduplicate VR's estimate.** VR takes E_max from H's `estimate`, or from one shared definition, rather than its own port.
   - Choose the mechanism at checkpoint 0: a dependency, a shared module, or a generated table.
   - Say which committed VR records carry the estimate, and whether VR's byte-for-byte tests move. Only the estimate's fields may change, and VR's suite and kill matrix must pass.
   - Re-check every admission decision V-K's runner made with the stale port (V-K's B and KF3's B) against the corrected estimate, and list any that would change.
4. **W1-T4 post-KF3** (K6b's reserved addendum 1), in a slot ROOT grants.
   - Run K6b's W1-T4 schedule (b3's six RF-LARGE 10,000-member models, both passes, the repeats), with its prefixes, on a release binary from a `git archive` of your candidate.
   - Record, per model: outcome, precision, work by stage and precision, heap, footprint, RSS, time with its load, and heap against the corrected E_max.
   - Compare outcomes with KF3's B model by model.
   - The TREE frames' 512 and 1024 measurements answer K6b RETURN §13.5's "measured 512 and 1024 memory and work".
5. **The small items:**
   - the stale doc comment at `H/src/bin/k6_observe/w1.rs:6-10` ("zero on completed builds"; KF3's NOTE);
   - RV22's C-N1: a test of a stop inside the solve (`into_solve_128`), if it fits H's existing test structure. Otherwise, say why not.
6. **Nothing else changes.** No FK edit. K6's four modes, and the W1 records of every model below 10,000 members, are unchanged except for the estimate's fields.

## Write set

- `H/src/k6/w1/counts.rs`, and `H/src/bin/k6_observe/w1.rs` (the doc comment, and the estimate's use if needed).
- `H/tests/k6b_*.rs` and `H/runner/**`, as the estimate and the re-run need.
- `H/observations/k6b/**`.
- `VR/src/scale.rs`, `VR/examples/vk_scale.rs` and `VR/Cargo.toml`, as the chosen mechanism needs, with VR's tests and committed records only where the estimate's fields appear.
- `T3/IMPLEMENTATION/K6C/`. K6b's committed records are hash-bound: don't edit them. K6C/'s RETURN serves as K6b's addendum 1.

Anything else is a stop, including FK and K4R.

## Required tests

- **The estimate:**
  - a test per corrected term, each killed by its removal;
  - the committed `counts.jsonl` lines recomputed from the code (RV22-3's test, kept);
  - a check that the corrected E_max is at least the recorded heap peak on every KF3 B and W1-T4 row, run from the committed records, not in CI if heavy.
- **VR:** its suite (47 tests or more) and its kill matrix, with the dedup in place.
- H's suite (`--all-targets`, with `k6_alloc`) and the runner's suite.
- **Mutants:** each of I19's three terms reverted (`at`/`bt`/`ct` dropped, `work` counted, `OPTION_EXTRA` back to 8); VR back on its own port; the transient overlap dropped (if you add one). All killed, and the NONE control passes.

## Gates (ROOT runs the PR)

- **An independent reviewer,** directed to the phase-by-phase derivation, the dedup, and W1-T4's records.
- Hosted CI with the full-SHA dispatch.
- DEC-025 with a fresh sweep target.
- GEN-8.
- **Harness and validation only:** T9 and the both-entry gate are not run. No product crate is touched.

## Checkpoints

- **0: the derivation and plan,** with no code: the phase table with file:line, the corrected formula, the dedup mechanism, the W1-T4 schedule, and the tests and mutants.
- **A:** the estimate, the dedup, the tests and the suites.
- **B:** W1-T4 post-KF3, in ROOT's slot.
- **D:** RETURN (K6b's addendum 1), CHANGE_RECORD, `_run_records/` and SHA256SUMS.

**Stop and report** on any recorded heap above the corrected E_max, any change to a W1 outcome or published row against KF3's B, an edit outside the write set, or a surviving mutant.

## Host

- One cargo job at `-j 4`, `RUST_TEST_THREADS=2`.
- Your own target, `<wt>/k6c-target`.
- The memory guard running.
- No building during a timed slot ROOT grants to another slice.
