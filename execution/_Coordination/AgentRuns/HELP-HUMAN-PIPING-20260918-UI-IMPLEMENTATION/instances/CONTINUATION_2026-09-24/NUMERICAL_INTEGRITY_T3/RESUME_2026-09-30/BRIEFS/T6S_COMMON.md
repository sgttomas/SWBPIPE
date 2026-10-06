# T6S: what every T6 successor-output slice assignment shares (I75, I76, RV101)

Each assignment is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and none delegates.

**You are a fresh instance.** Your ID names a records folder and a role. It carries no memory of earlier sessions, so read the basis below before acting.

## What the slice is

The owner pulled a narrow T6 slice forward (RR "I61's U8 plan ruled; the owner pulls T6's successor-output slice forward; …", decision 12): the desktop result export and stress-neutral export of a successor, replacing the explicit N-5 panel refusal deliberately; the `results.schema.yaml` v0.3 dispatcher; and RV95 N-5's bound test. It closes public activation's checklist item 4 (`T3/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` §4) before B8.

## The basis

- **The plan:** `R/I74/t6_slice_plan_01/PLAN.md` (sha256 `0350c918…`). Its §1 is the scope, §2 the fence, §3 the contract questions, §4 the slices, §4.3 the breadth consistency rules. Code is cited at NUM `b1e2d7741e`, whose maintained source equals main `c1bfc460fc`.
- **The ruling:** RR "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched". Its table rules I74's decisions 1–14. Decision 10 (G10) is the owner's and does not change your work.
- **The contract:** D2 (`T3/DESIGN_STANDING/DESIGN.md`) §4.9.4, §4.9.7 and §4.9.9; D-U6-2 (the class disclosures); D-U7-4 (TS standing needs the live native capture); D-U7-6 (no producer-origin claim). Rust `RE/src/derivative.rs` is the reference form of the successor derivative.
- **The pinned successors:** `P/fixtures/results/retained_precision_milestone_successor_{sparse_interactive,dense_scrutiny}.json`. The corpus `P/fixtures/results/retained_precision_cases.json` supplies the synthetic two-case bases, read by id.

## The working tree (set up by ROOT)

- **The branch** `codex/piping-t6-successor-outputs-20261005`, cut from main `c1bfc460fc`, in worktree `WT/t6-outputs`. I75 and I76 share it, on disjoint files.
- **Leave work uncommitted.** ROOT commits.

## The fence (PLAN §2; the ruled form)

**May be written,** by the owner named:
- **I75:** `DT/features/results/loadReferenceOutputAvailability.ts`, a new `DT/features/results/outputPolicy.ts`, a new `DT/features/results/retainedPrecisionDisclosure.ts`, `DT/features/result-export/ResultExportPanel.tsx`, `DT/features/result-export/resultExportAdapter.ts`, `DT/features/stress-neutral/StressNeutralExportPanel.tsx`, `DT/features/results/retainedPrecisionOutputRefusal.test.tsx` (the two panels' expectations and the reworded text only), and new `DT/features/result-export/retainedPrecisionResultExport.test.tsx` and `DT/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx`. Other existing desktop tests only where the reworded refusal text (decision 12) changes an expected string, and nothing else in them.
- **I76:** `P/schemas/results.schema.yaml`, `P/tests/test_result_export_v0_2.py` (validator construction only), a new `P/tests/test_results_dispatcher_v0_3.py`, a new `RE/tests/retained_precision_derivative_golden.rs`, `RE/tests/source_blocks.rs` (N-5's public-API test only), and the new goldens `P/fixtures/results/retained_precision_successor_derivative_{sparse_interactive,dense_scrutiny}.json`.

**Never touched** (PLAN §2.2): every file under `PP/`; every `src/` of a D1 crate (Pass B's `crate_dirs`), including RE's reader, `derivative.rs`, `semantic_contract.rs` and `source_blocks.rs`; every file D1 code embeds (the 14 reviewed inputs, `results.v0.3.schema.yaml`, `load_reference_state.schema.json`, `units.schema.yaml`, every `semantic_contract_*.json`); the readers and carriers (`DT/features/results/retainedPrecision.ts`, `retainedPrecisionStanding.ts`, `knownSemanticLimitations.ts`, `P/core/analysis_runs/*.py`, `P/core/handoff/stress_neutral/*.py`); `P/apps/desktop/src-tauri/**` and `P/tools/ci/e2e_plan.py`; U8's and S-I1's files; the carrier case file; the stress-neutral and AnalysisRun schemas; every lock file.

## Host rules

- **Git:** no Git writes. Reads use `GIT_OPTIONAL_LOCKS=0`.
- **Cargo:** every command through `WT/tools/t3_cargo.sh <args>` (a host-wide lock; other TASKs share it), with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, and your own target directory `WT/targets/<your-id>-t6s/`. Waits can be long; never kill a job waiting for the lock. No DEC-025, native or solver-at-scale jobs.
- **The memory guard** (`WT/guard/memguard.sh`) must be running; `t3_cargo.sh` refuses otherwise.
- **Node:** the existing `node_modules` through an untracked symlink at `WT/t6-outputs/projects/chirality-piping/node_modules` → `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/projects/chirality-piping/node_modules` (ROOT gives the absolute path at dispatch). Only one of I75 and I76 creates it; check first. The last to return removes it, or leaves it for ROOT and says so. No installs.
- **Python:** the VENV at `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/projects/chirality-piping/.venv`.
- **Scratch** under `WT/scratch/<your-id>_t6s/`; never the system temp directory or a worktree's own folder.
- **Records** in `NUM/R/<your-id>/t6s_01/`: RETURN.md (or REVIEW.md) with SHA256SUMS, placeholder paths only (`WT`, `NUM`, `P`, `DT`, `RE`), the changed files' hashes, and no whole-host data.

## Binding rules

- **Nothing weakened.** No assertion is deleted or loosened. A refusal expectation changes only where the ruled scope deliberately admits a successor (the two panels) or rewords its text (decision 12).
- **Existing behaviour holds** for every non-successor route: ordinary result exports and stress-neutral packages are byte-identical to the base for every committed fixture.
- **No schema change** except the dispatcher (decision 7). No new dependency.
- **Stops:** a write outside your fence; a needed change to a never-touched file; an existing byte changing for a non-successor route; a check that would be weakened; a contract reading beyond the ruling. Report the stop and continue the unaffected work.
