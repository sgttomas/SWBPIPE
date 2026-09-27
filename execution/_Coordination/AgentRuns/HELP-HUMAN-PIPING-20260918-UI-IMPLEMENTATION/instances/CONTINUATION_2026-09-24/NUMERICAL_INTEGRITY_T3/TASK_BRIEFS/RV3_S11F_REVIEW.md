# RV3: independent full-diff review of slice S11-F

This is a review TASK. Read `_COMMON.md` first; this brief overrides it where they differ.

You must be independent: you did not design S11, review its design, implement S11-K or S11-F, or review S11-K. Your job is to find defects, not to confirm. Report what you find; you fix nothing.

## Candidate

- **Branch:** `codex/piping-s11f-20260927`, in `<wt>/s11f`.
- **Head:** the commit the manager names at spawn, after I4's regeneration and the forward merge of origin/main.
- **Scope:** review the complete diff from the merge base with `origin/main` to that head. Record both revisions. Every line is in scope:
  - product source (`PP`, `pressure_runtime.rs`, `source_recovery.rs`, `source_receipt.rs`, FK/SA/`primitive_loads` touches);
  - tests, site tests and generators (`rf_cancel_cases.json` and its generator);
  - regenerated fixtures and derived documents;
  - the two hash-pin constants;
  - records under `T3/IMPLEMENTATION/S11F/**` (CHANGE_RECORD, RETURN, PRE_REGENERATION_REPORT, `_run_records/`).
- **Write set:** `T3/REVIEW/S11F_REVIEW.md` and `T3/REVIEW/_run_records/s11f_review/**` (with their own SHA256SUMS), **in the numerics worktree** (`<wt>/numerics`). Don't write in `<wt>/s11f`. Builds and mutations run in a scratch clone, with your own target.

## Basis

1. `T3/ROOT_SELECTION_DESIGNS.md` (the §4 hard constraints) and `T3/ROOT_SELECTION_S11.md`.
2. `T3/DESIGN_NUMERICS/S11_CONTAINMENT.md` **revision 5a.2** (sha256 `e6507587…`): §2, §4, §5, §6, **§8.2, §8.3**, and **§9 F1–F14 with the M-set**.
3. `T3/TASK_BRIEFS/I4_S11F_IMPLEMENTATION.md` and its addenda (boundary decisions; F12 and the formation list).
4. `T3/ROOT_RULINGS_V1.md`, especially:
   - the S-H/S11-F ordering;
   - "I4's F12 stop" and the amendment on formation rows;
   - **"S11-F fixture stop: conditional pre-approval of regeneration"** (`1397e8c43`);
   - the I4 boundary acceptances.
5. `T3/GATE/S11_EXCEPTIONS.json` (221 triples) and `GATE/FORMATION_EXCEPTIONS.json` (7 triples, 14 rows), and `T3/REFERENCES/references.json` (`c0f14201c`).
6. RV1's S11-K review (`T3/REVIEW/S11K_REVIEW.md`), for the carried NOTEs and its evasion method.

## What to check (at least)

1. **§8.2's write set and behaviour:**
   - the ledger at every §4.2 producer;
   - the force built only from the ledger;
   - E5, E7–E12, E15 and E16;
   - the Sensitive mapping;
   - T1's three sites;
   - the 0.4.0 prescribed motion on the typed seams;
   - removal of the `&[f64]` product entry points;
   - option (c) intact, with the loop reaching only `_binary64` variants and I1's pins unchanged and green.
   **Independently re-derive the complete caller list** of every changed function (lexer scan), and compare it with I4's.
2. **F12, literal:** after S11-F, **`GATE/S11_EXCEPTIONS.json` is empty** on both entries and both modes, over the frozen references. Re-run the gate harness yourself.
   - The expected values come from `references.json` via the committed generator (check the generator and its input hash; nothing hand-copied).
   - Any breach outside both pinned lists is a FAIL.
3. **Formation rows:** each of the 14 rows publishes **the correctly rounded net of the represented terms** and stays exactly pinned in FORMATION_EXCEPTIONS.json.
   - Check I4's base-against-candidate table: 10 rows bit-identical or better, and the 4 UDL-W1e8 rows 3% worse, disclosed.
   - Recompute at least UDL-W1e8's represented net in `Fraction`.
4. **Regeneration against ROOT's conditions,** checked on the committed bytes:
   - **A:** only Debug/diagnostic text, digests or hashes, or text-length work units change; no result value, status or diagnostic code moves; the hash pins' old → new values match the committed raws' sha256; every pinning test is green.
   - **B:** the producer is named and reproduces the committed dense file on base; only the restrained-DOF reaction residue and its magnitude move (≤ 1e-9 of the load scale), with no status change; every consumer is green, including the desktop parity test.
   - **Both:** re-run at least one producer per kind, and confirm the committed bytes equal `measurement_sha256.txt`.
5. **Line 2709:** the pre-existing 1-ulp drift in `physics_thermal_ui_mechanics_sparse.json`. Verify the explanation (last regenerated at `22452ecd1` with a hypot chain; `1792774a2` switched to `composite_support_norms` without regenerating it), and that CHANGE_RECORD discloses it **separately** from S11-F's effects.
6. **Carried items,** each resolved as the brief addenda record:
   - the sparse_direct typed sibling (none; reasoning plus behavioural site-test coverage);
   - the `global_load_vector` doc note;
   - N1 (self-assignment folds detected; counts re-baselined);
   - N5 (the audit never propagates an error, and an unaudited row makes the case Sensitive; the (1e80, −1e80, 1e-300) test; no new envelope field);
   - N6 (no rename; the PP rule's behavioural test);
   - N8 (positive doctest twins);
   - B-N1 (`::solve(` and `::solve_assembled(`, plus the structural_adapter scan);
   - SA `solve` narrowed to `pub(crate)`. Record correctly that this closes EV4 only outside `nonlinear_integration`.
7. **Every pin behavioural (RV1's lesson).** For each pin (the site tests, source pins, typed-seam pins and the N6 rule), confirm a behavioural test first asserts that the two paths differ, then asserts the outcome.
   - **Attempt RV1-style evasions:** a helper or alias hiding a fold; UFCS calls; comment or string text satisfying a source pin; restoring a fold as `s = s + v`; a fold in a sibling module.
   - Construct at least four evasion or mutation attempts, and report whether each is caught behaviourally.
8. **Mutations:**
   - Re-run a sample of §9's S11-F set: at least M1f, M1l, M1n, M2, M3, M8 and M9, at G = 1e8 and 1e80.
   - Confirm each is killed by the named test.
   - A mutant killed only by a source-text pin, where a behavioural test was expected, is a finding.
9. **Hard constraints:**
   - S-H is **not** in this slice;
   - F11's 1e80 cases run on the typed entry;
   - no in-band marker;
   - DEC-046 and `benchmarks/nonlinear` unchanged.
10. **The K-D5 boundary:** S11-F's change to `solve_preview_reduced_system` is only the switch to `solve_assembled` with `&AssembledForce`; its shape is unchanged. `finish_checked_factor` is changed only as N5 requires; SA's public surface is as ruled. Name anything K-D5's forward merge must adapt to.
11. **Disclosure and hygiene:**
    - CHANGE_RECORD covers the §8.3 items, the formation rows, the regeneration sizes, line 2709, the pre-approval citation and the carried items;
    - no machine paths (home, temp or tool-install);
    - `cargo fmt`; `git diff --check` (or the exceptions recorded);
    - the records' SHA256SUMS verify;
    - no `node_modules` links committed;
    - the authority targets untouched.

## Running things

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, and your own `CARGO_TARGET_DIR` under `<scratch>`, pruned when done.
- **Cargo priority:** you come after ROOT's sweeps; alternate by job with I3 and D1's builds if any. Check `pgrep -x cargo` and `pgrep -f 'python[0-9.]* .*run_evidence_sweep'` before each job.
- Keep free disk above about 8 GB.
- **Never delete** `core/serialization/canonical_json/target` or `core/units/target` in any worktree. Build them in your scratch clone with the two `tools/…/build_*.py` scripts before Python runs.
- Skip no tests and raise no timeouts. Make no Git writes.

## Verdict and return

Write `T3/REVIEW/S11F_REVIEW.md` in `<wt>/numerics`, containing:
- the revisions reviewed;
- a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, resolution);
- a section per check;
- your independent caller list;
- the gate re-run;
- the evasion attempts;
- what you ran;
- what you did not check.

The verdict is **PASS** (no unresolved BLOCKING findings) or **FAIL**. Send the manager a SendMessage summary with the verdict, the counts and the file's sha256.
