# TASK brief — CP4_JOIN_TESTS (checkpoint-3 review N-1)

- **Role:** TASK (Type 2). You do not delegate.
- **Manager:** the T1 WORKING_ITEMS manager (load/reference states) that requested this TASK. Report to it by `SendMessage`, never only as final text.
- **Checkout:** `/home/user/wt/loadstate`, branch `codex/piping-load-states-20260925`, at the manager's commit `a819a2b1f` or later. Paths are relative to WORKING_ROOT = `projects/chirality-piping/`. `LSI` = `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`.
- **No Git writes.** Do not stage, commit, push or change branches.
- You did not write the join; keep it that way. You write tests only.

## Basis (read first)

- `LSI/REVIEW_CHECKPOINT_3/RETURN.md`, finding N-1 and §2–3 (probes P1–P12, mutants K1–K12).
- `LSI/REVIEW_CHECKPOINT_3/ROOT_DISPOSITION.md`, item N-1: "add committed tests equivalent to the reviewer's probes P1–P12, so mutants K5 and K7–K10 are killed by maintained tests."
- The reviewer's probe source `LSI/REVIEW_CHECKPOINT_3/_run_records/review3_probes.rs.txt` and mutant harness `review3_mutations.py` (anchors verbatim). Many probes only print; your tests must assert.
- `LSI/CP2_WIRE_ADDENDUM_2.md` §5 (join rules) and `LSI/CHECKPOINT_3.md` §1.1.
- **SF-1 has landed** (commit `6235f6b43`, see `core/product_physics/src/source_receipt/load_state_fallback_tests.rs`). Probes P3, P9 and P12 are already pinned there; do not duplicate them. Where a probe's expected outcome depended on the old "blocked or `Err`" behaviour, assert the current SF-1 behaviour instead: a selected join that cannot finalize republishes every case on its ordinary route under `load-reference-1`, and selection first reserves the captured replay's work.

## Assignment

Write maintained tests in the placeholder module `core/product_physics/src/source_receipt/load_state_join_tests.rs` (declared `#[cfg(test)]` in `source_receipt.rs`; it can use `super::*`, as `load_state_tests.rs` does). Cover, with real assertions:

1. **P1** a selected case whose member pair differs from the base material: qualified receipt, and the published member pair and axial force match the closed form `N = E·A·(δ/L − ε*)` with the selected E.
2. **P2** two selected cases with different pairs and motions: both qualify; per-case closed-form tip displacements.
3. **P4** a zero-valued pressure region: not joined (`load-reference-1`, `retained_source_attempt=unavailable`).
4. **P5** post-capture perturbations refused by captured replay: strain split with the same total, resolver evidence only, an explicit zero motion, an effective primitive magnitude, ν only. These must kill **K5** and **K10**, and help kill **K7**.
5. **P6** a primitive load whose ID collides with `load_state_eigenstrain:<len>:<pipe>`: refused. Design the test so that it kills **K8** (collision check removed). The reviewer found that P6 alone does not; find an observable that does (for example the specific refusal stage or message in the attempt diagnostic, or an in-module check on the source commitment), without relying on message text that the wire says readers must not validate — tests may assert producer diagnostics.
6. **P7** the `mixed` joined witness: the ordinary pressure case's record is bound. Design a test that kills **K9** (ordinary-case record binding removed in a joined envelope). This likely needs an in-module test that tampers with that record between product and receipt finalization.
7. **P11** joined rows against the ordinary route: `eigen_motion`, the exact-point pair, factor 2.5 with an excluded 7 kN source, and a parallel spring on a moving rigid DOF, both modes, at the reviewer's tolerances or tighter where a closed form exists. Also assert the parallel-spring closed forms (spring −k·g = −1 N, anchor +1 N).
8. **Motion ownership (K7)**: a motion that is not owned by exactly one rigid DOF is refused, killing **K7**.

All inputs must be invented and marked as such, as in the existing witnesses. Do not use any material or component library value or code rule, and add no defaults. Never loosen an existing tolerance or assertion; never edit an existing test.

**Kill demonstration.** Run K5, K7, K8, K9 and K10 (the reviewer's anchors) against your module and show each is killed by at least one of your tests; also show K1–K4 and K6 remain killed. Use a scratch copy of `core/` plus `fixtures/` (the reviewer's method: `review3_mutations.py <scratch core/product_physics> <target>`), never in place. Restore and sha256-verify is not needed on a scratch copy, but record the scratch-copy prehash against the candidate. If a mutant cannot be killed without a production-code change, stop on that mutant, say exactly why, and propose the smallest change; do not make it.

## Write boundary (exact)

- `core/product_physics/src/source_receipt/load_state_join_tests.rs`
- your return folder `LSI/CP4_JOIN_TESTS/` (RETURN.md; scripts and logs under `LSI/CP4_JOIN_TESTS/_run_records/`).
- scratch copies and cargo targets under `/home/user/wt/ls-join-scratch/` (create; delete when done).

Nothing else. Another TASK is editing the result_export and analysis_runs readers concurrently; do not touch them.

## Checks to run

Use `CARGO_TARGET_DIR=/home/user/wt/ls-join-target` for the live checkout (delete when done), `-j 2`, toolchain `+1.97.1`, `--locked --offline`. Disk is limited (about 8 GB free): keep at most one scratch copy and one extra target at a time.

1. `cargo test --lib source_receipt::load_state_join_tests` — all pass, both modes where relevant.
2. The whole crate once at the end: `cargo test` in `core/product_physics` — baseline 407 passed, 1 ignored, plus your tests.
3. `rustfmt +stable --edition 2021 --check src/source_receipt/load_state_join_tests.rs` — clean.
4. The mutation runs above.
5. `git status --short` shows only your test file and return folder.

## Return

`LSI/CP4_JOIN_TESTS/RETURN.md`, then a `SendMessage` to the manager with a short summary and that path. Include: the test names and what each pins (with the probe it replaces); the file's sha256; check commands and counts; a mutant table (K1–K12, killed by which test, or survived and why); and any design question. Keep absolute machine paths out of RETURN.md.
