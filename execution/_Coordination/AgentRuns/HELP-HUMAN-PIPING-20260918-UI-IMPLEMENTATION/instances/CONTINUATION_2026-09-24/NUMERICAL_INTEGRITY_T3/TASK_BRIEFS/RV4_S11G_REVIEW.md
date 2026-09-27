# RV4: independent full-diff review of slice S11-G

This is a review TASK. Read `_COMMON.md` first; this brief overrides it where they differ.

You must be independent: you did not design S11-G (D1), check its design (V1), or implement it (I5). Your job is to find defects, not to confirm. Report what you find; you fix nothing.

## Candidate

- **Branch:** I5's S11-G branch, in `<wt>/s11g`, at the head the manager names at spawn.
- **Scope:** review the complete diff from the merge base with `origin/main`. Every line is in scope: product source, tests, site tests and records under `T3/IMPLEMENTATION/S11G/**`. Check also the GATE change on the T3 branch that empties `FORMATION_EXCEPTIONS.json`.
- **Write set:** `T3/REVIEW/S11G_REVIEW.md` and `T3/REVIEW/_run_records/s11g_review/**` (with their own SHA256SUMS), in `<wt>/numerics`. Build in a scratch clone.

## Basis

1. `T3/ROOT_RULINGS_V1.md`: every S11-G section, ending with "Selection: the S11-G design".
2. `T3/DESIGN_NUMERICS/S11G_GUARD.md` **revision 2.1** (`7c052c9e…`): §3, §4 (R-b′), §7, §8 and §9.
3. `T3/REVIEW/S11G_CHECK.md` (V1's checks, counterexamples and NOTEs).
4. `T3/TASK_BRIEFS/I5_S11G_IMPLEMENTATION.md`.

## What to check (at least)

1. **Design conformance:**
   - the formed/input tagging at every push site (T8's site table complete);
   - per-family records;
   - two exact accumulators per row (A_net and A_se);
   - the exact decision, including D21-1 (±12B and ∓12T0 added into the accumulator copy, with no rounded comparison) and D21-2 (γ2·|value| + |k|·2^-1074);
   - the RD(10^-9) or exact-rational threshold;
   - the FMA underflow and overflow conditions;
   - body S\*, with the floor only on the self-equilibrated part;
   - R-b′'s clauses;
   - the no-op rule;
   - `source_eligible`;
   - no new field, code or `Err`.
2. **The specific attack (ROOT): try to hide a real formation defect past both accumulators.** Construct inputs (product requests where possible) in which a genuine net formation defect above the criterion is published Passed. For example:
   - defects split so they cancel within A_net;
   - a formed term mis-tagged as self-equilibrated or as input;
   - a family whose defect is computed from a different operand than the product uses;
   - underflow and overflow edges;
   - a defect routed into A_se and hidden by the floor;
   - a recovery row just under R-b′'s floor or ratio clauses.
   Report every attempt and whether it is caught.
3. **R-b′ is load-bearing** (the only INPLANE catch after S11-F):
   - Re-run M14 (signed sums and |Tu|), M13 and T11/T17 yourself.
   - Construct at least three further mutants against `bending_formation_bound` and R-b′'s clauses (for example: a dropped γ16, B computed without Σ|K|, a wrong row for q, the floor applied to B instead of q), and confirm each is killed.
4. **All mutations M1–M18:** re-run a sample that includes M1, M2, M5, M10, M12, M17 and M18, and confirm the named killing tests. A mutant killed only by a source pin, where a behavioural test was expected, is a finding.
5. **Every pin behavioural,** with a paths-differ precondition (RV1's lesson), including T10/T10b's routing pin.
6. **The gate (T16):** re-run it yourself.
   - All 14 formation rows are published non-Passed on both entries and both modes.
   - The S11 list is empty; there is no breach outside the lists.
   - FORMATION_EXCEPTIONS.json is emptied by its generator, and only then.
7. **Zero committed-byte diff:** run every committed request through base and candidate, in both modes, yourself or by re-running I5's harness.
   - The S11-F formation-row pins are bit-identical.
   - No legitimate row is demoted: the 221, RF-SKEW, UDL-W1e5, probe A, the collinear and pressure runs, and every committed fixture.
8. **Disclosure:** CHANGE_RECORD states the CannotBound availability loss, the DN-4 residual (routed to W1/F2), the formation list emptied, and no in-band marker.
9. **The K-D5 boundary:** SA, FK `structural.rs`, `solve_preview_reduced_system` and CB are untouched.
10. **Hygiene:**
    - no machine paths (GEN-8 regex);
    - `cargo fmt`; `git diff --check` (or the exceptions recorded);
    - the records' SHA256SUMS verify;
    - the authority targets untouched.

## Running things

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, and your own target under `<scratch>`.
- One heavy cargo job at a time. Hold during sweeps (`pgrep -x cargo`, `pgrep -f 'python[0-9.]* .*run_evidence_sweep'`).
- Keep free disk above about 8 GB. Never delete the authority targets.
- No Git writes. Skip no tests and raise no timeouts.

## Verdict and return

Write `T3/REVIEW/S11G_REVIEW.md`, containing:
- the revisions reviewed;
- a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, resolution);
- a section per check;
- **the attack log** (every attempt to hide a defect, and its outcome);
- the mutation table;
- the gate re-run;
- what you ran;
- what you did not check.

The verdict is **PASS** or **FAIL**. Send the manager a SendMessage summary with the verdict, the counts and the sha256.
