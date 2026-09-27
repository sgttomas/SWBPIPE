# I3R: resume K-D5's RV5 repair (replacement for I3 after the container restart)

This is an implementation TASK that continues I3's K-D5 work. I3 (ad7a67f6b8e67c081) was lost in a container restart; its state is on disk. Read `_COMMON.md` first, then `I3_KD5_IMPLEMENTATION.md` with its addenda 1–4, and `T3/IMPLEMENTATION/KD5/RETURN.md` and `CHANGE_RECORD.md` on the kd5 branch. Make no Git writes and no index operations (no checkout, restore, reset or stash). The manager commits.

## Where things stand

- **Branch and head:** `codex/piping-kd5-20260926` in `<wt>/kd5`, at `2409de83e`. The history is: K-D5 `17f3d6e05`, then forward merges, then addendum 4 `a89fde17b`, then the S11-G merge `fbc9661a4`, then the T20 comment `2409de83e`.
- **The combined-tree evidence is complete and green:**
  - suites 24/24 on `fbc9661a4`;
  - T9 against main `b24b3d536` 112/112 byte-identical;
  - the gate union PASS: 888 runs, 0 trusted breaches against main's empty lists, and the 4 dense timeouts timed out on a quiet host;
  - only 122 differs from main.

  The records are in `<scratch>/kd5-i3/gate3/`. RV5's recount is in `<wt>/numerics` `T3/REVIEW/_run_records/kd5_review/gate/rv5_gate_recount.txt`.
- **The timing comparison:** `<scratch>/kd5-i3/timing/` (timing.jsonl, timing.log). It was started by I3 as PID 8017: main probe `12811c32…` against candidate probe `39791d93…`, interleaved, on CHAIN-n01000-AX and TREE-n01000-ROT dense. **If it's still running, let it finish; don't restart it.** Read its result from timing.jsonl.
- **RV5 (the K-D5 reviewer) has provisionally NOT PASSED K-D5 on RV5-B1:** the M31b chord-only mutant survives, and an admissible counterexample exists. **ROOT withdraws the M31b equivalence (`c2042fd9c`)** once RV5's Rust run confirms. M31b and M31b0 must be killed by required tests.

## I3's uncommitted drafts (your starting point)

These are in `<wt>/kd5`, uncommitted. A byte snapshot is in `<scratch>/kd5-i3-drafts-snapshot/`, with sha256 values in the manager's log.
- `projects/chirality-piping/core/solver/nonlinear_integration/src/structural_adapter/kd5_tests.rs`:
  - `kd5_admissible_centre_mismatch_demotes_where_the_product_chord_hides_the_error` (CPLANAR_60 and CSKEW_30_N122, both modes);
  - `kd5_large_coordinate_pp_route_elbow_does_not_demote` (PP_UTM_2).
- `.../structural_adapter/kd5_models.rs`: 3 appended exact-reference models. The existing constants are byte-identical.
- `projects/chirality-piping/core/product_physics/tests/formation_check_runtime.rs`:
  - `kd5_very_large_coordinate_pp_route_elbow_demotes_on_both_entries` (X 5e6, Y 3.5e6, φ 5°, R 0.3);
  - `kd5_large_coordinate_pp_route_elbow_is_published_accurately_and_not_demoted` (X 5e5);
  - the vacuous receipt-loop fix (the nonlinear never-selected test now asserts `source_block_recovery` is null).
- **The exact-reference generator:** `<scratch>/kd5-i3/refs2/kd5_models.py` (Fraction inputs; D1's objective curved_int), with its outputs.
- **The callers,** regenerated on the candidate: `<scratch>/kd5-i3/callers2/`. 113 sites, 39 non-test. SA `solve_assembled_with_formation_check` is the only non-test builder, and PP:4399 is the only product call.
- **The clean mutation runner:** `<scratch>/kd5-i3/mut/run_mutants_clean.sh`. It extracts with `tar -m`, removes the target before each mutant, runs a NONE control, and runs FK kd5 + NI kd5 + PP formation_check_runtime per mutant. The mutation patcher is `mut/mutate.py`. The old logs in `mut/` are from the **stale-build** run.

Read every draft in full before building. They are yours to finish, not to trust.

## What remains (one repair commit, tests and records only; no product code)

1. **Build and run the drafted tests** on the tree (`2409de83e` plus the drafts). ROOT's required tests are:
   - **the M31b and M31b0 kills:** the 60° planar and 30° skew cases with exact references, on both entries and in both modes. Each patch fails at a **behavioural** assertion, not a precondition;
   - **the product-level demotion test** (X 5e6, Y 3.5e6, φ 5°, R 0.3): it publishes SENSITIVE on both entries and in both modes, and **applying the M31b patch makes it publish CHECKS_PASSED.** Show this by applying the patch;
   - **the product-level control** (PP_UTM, X 5e5): not demoted, with the published error below half the criterion, on both entries and in both modes.
2. **The clean mutation re-run of the whole K-D5 set:** M23, M26, M27, M28, M31a, M31b, M31b0, M32a and M32b, plus the NONE control, with the clean runner. Record the corrected kill table. **Disclose the stale-build issue:** the earlier runner reused an old frame_kernel artefact, because tar preserves mtimes, so M32a and M32b ran against an FK carrying M31b0.
3. **The callers:** replace `_run_records/callers.txt` with the regenerated list, and correct RETURN §5 (it describes phase 1).
4. **A RETURN addendum for the combined tree,** covering:
   - the union gate verdict and its records;
   - the main-against-candidate attribution (only 122 differs);
   - **the 34 S11-G-attributed moves,** accepted by ROOT as S11-G's forecast demotions (the 7 former FORMATION cases; LFRAME×4 and WEAK-W-3D/AX-rho1e-08 as R-b′'s disclosed false demotions);
   - the T20 comment commit;
   - the timing result. If S11-G costs about 15% on dense 1000-member solves, record it as an S11-G performance finding. K-D5's own cost is well under 1 s per case;
   - the M31b equivalence withdrawn (once confirmed), with the earlier claim kept and marked **superseded**, in RETURN §7 and CHANGE_RECORD;
   - **the findings:**
     - the gate corpus lacks realized curved bends, so curved-bend formation integrity is evidenced by unit and product tests only (routed to the gate-corpus owner);
     - the product's curved element carries formation error above the criterion at coordinates ≳ 2e6 m (UTM northing scale), which K-D5 correctly demotes (routed to T4/W1c). The earlier ~5e5 m figure is superseded as a repr-input artefact;
   - **ROOT's fact:** no committed fixture or gate case changes standing through this effect. Give your own evidence: no gate request realizes a curved bend, and the only bend-realizing solve models are the arc_model tests at 1.2 m or less;
   - **the NOTEs:**
     - the vacuous loop, fixed;
     - the 4 empty nohup logs: record them, and don't commit empty logs as evidence;
     - the conservative S\* (free DOFs only).
5. **Records:** copy the combined-tree evidence, sanitized (`<wt>`, `<scratch>`, no machine paths), into `T3/IMPLEMENTATION/KD5/_run_records/combined/`. Refresh SHA256SUMS. **Never rewrite committed, hash-bound evidence:** add new files instead. Run `cargo fmt` on the changed test files only. Run GEN-8 (`pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, from the repo root).

## Running things

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, `CARGO_TARGET_DIR=<wt>/kd5-target`.
- **The cargo token is the manager's.** Do no cargo until the manager says your slot has started. The order is: the timing comparison, then RV5's Rust confirmation, then you.
- One heavy job at a time. Hold cargo while a DEC-025 sweep runs (`pgrep -f 'python[0-9.]* .*run_evidence_sweep'`). Wait patterns must not match the waiting shell.
- Keep free disk above about 8 GB. The authority targets in kd5 are prerequisites, never scratch.
- Skip no tests and raise no timeouts.

## Return

Send the manager "clean point" with the repair's file list, the test results, the M31b/M31b0 kill evidence (the assertion sites), the clean mutation table, and the RETURN addendum's section list. Report at once if anything cannot be done as specified.
