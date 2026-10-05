# I66: U6a, the end-to-end carrier slice, with D-U6-1

I66 continues as a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), with its existing context. ROOT is the return path, and I66 does not delegate. **I66 owns U6 through repairs.** This is its first implementation grant, under its own plan, `R/I66/u6_scoping_01/PLAN.md` (sha256 `8742d105…`), as ruled in RR "U6 plan accepted: D-U6-1 to D-U6-9".

## Purpose (workflow §1: prove the path before multiplying work)

Carry the milestone successor through one real carrier and back out, verified, before the Python, schema, TypeScript and reader-round units fan out. The slice is your plan's U6a: Rust dispatch, the downgrade guard, the fresh set, standing, the binding refusal and `classification_summary`. The receipt goes through `derive_document` and `validate_document` and comes out byte-equal and revalidated, with standing `needs_recompute`. Add D-U6-1 (the Python reader's public entry) and the shared fixture step (D-U6-5).

## Where to work

- **Worktree:** `WT/f2a-carriers`, branch `codex/piping-f2a-carriers-20261004`, from NUM `7e4f5a51dd`. NUM includes U1, U2 and U3 grant 1. U3 grants 1b and 1c, with the public carrier `RetainedPublication`, are on `codex/piping-f2a-facade-20261004`; U6a does not depend on them.
- **Records:** `NUM/R/I66/u6a_slice_01/`. **Targets:** `WT/targets/i66-u6a/`.
- **Write fence:**
  - `P/core/reporting/result_export/src/`: `derivative.rs` and the dispatch, standing and binding files your plan §1a names for U6a, plus their tests;
  - `P/core/analysis_runs/retained_precision.py`: the public entry only (D-U6-1), plus its test;
  - the fixture directory your plan names for the byte-identical successor copies (D-U6-5);
  - `P/core/reporting/result_export/tests/`.
  
  Nothing else. Schemas belong to U6c, TypeScript to U6d, and the rest of the reader round to U6e.

## Rulings that bind this grant

- **D-U6-1:** the Python reader runs every gate, and the completeness flag gates eligibility only, as Rust and TS do. Eligibility stays off. **Check that no input changes its accept/refuse outcome except by now reaching the gates it previously short-circuited.** Run the full 07e corpus and the real milestone receipts through it.
- **D-U6-2:** the derivative discloses `absolute_verified` and `not_covered` rows as `disclosed`, with reason codes `retained_precision_absolute_verified` and `retained_precision_not_covered`. Implement the Rust side.
  - The schema reason codes are U6c's, so until U6c lands, test against the schema in a lane copy and state that.
  - **If the slice cannot be validated without the schema change,** pull that one schema delta into this grant and say so in RETURN.
- **D-U6-5:** byte-identical copies of PP's pinned successor files (sparse `ac6986b0…`, dense `6cd1d249…`), checked by sha256 in every test.
- **D-U6-6:** the successor joins the Current-admission sets. **Standing, not freshness, gates reliance,** and standing stays `needs_recompute`.
- **D-U6-9:** the legacy 0.1.0 AnalysisRun wrapper refuses sources carrying `retained_precision`. Do it here if it sits in your Rust fence; otherwise in U6b.
- **The milestone limits** (RR "RV86 on U5"): carriers must not represent the sharper stop-rule bound, or truth-enclosure by the extrema intervals, as claims.

## Controls (each failure is a stop)

1. **Every existing document identity** is accepted, refused and classified exactly as at base, with identical bytes out of `derive_document`. Use your own sweep over the existing fixtures.
2. **The slice:** the milestone successor in both modes, through `derive_document` and `validate_document` and back out. The receipt is byte-equal and revalidates in all three readers. Standing stays `needs_recompute`, and eligibility stays false.
3. **The downgrade guard:** a successor offered as a base `preview-physics-1` document is refused (F-5). Every receipt mutation and binding mismatch is refused with its ruled code.
4. **Nothing weakened:** no existing check removed or narrowed, and no reader eligibility flag changed.
5. **Mutants** for each new branch, none killed only by a compile error.

## Host

- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time. Python lanes as the reader rounds used them.
- **Scratch** in `WT/scratch/i66_u6a_slice_01/`, never the system temp directory. The memory guard must be running.
- **Never:** Git writes, installs, new tooling, or native, solver-at-scale or DEC-025 jobs.
- **Other TASKs are working;** don't touch their files. They are I61 in `WT/f2a-facade`, I65 on U4 G4, and RV85.

## Budget and return

**Budget: 7 h.** Return once, with RETURN.md (changed-file hashes, controls, mutants, findings), SHA256SUMS and a concise status. If a ruling is needed (a contract reading, or a write outside the fence), stop and report it; continue with the work it does not affect.
