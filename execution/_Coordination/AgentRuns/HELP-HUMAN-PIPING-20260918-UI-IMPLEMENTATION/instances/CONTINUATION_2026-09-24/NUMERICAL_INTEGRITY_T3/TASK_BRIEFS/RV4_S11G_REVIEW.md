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

## Addendum 1 (2026-09-27): the I5 rulings

**Basis.** Add `T3/ROOT_RULINGS_V1.md` "S11-G implementation: I5 rulings" to the basis. Where they differ, it takes precedence over the brief and the design.

**Also check:**
1. **The B = T0 = 0 erratum.**
   - The first clause is `B > 0 && B ≥ T0`, not revision 2.1's `B ≥ T0`.
   - The boundary tests exist and pass:
     - B = T0 = 0 with A = 0 does not fire (T6 stays CHECKS_PASSED);
     - B = T0 = 0 with A_net ≠ 0 fires;
     - B = T0 = 0 with |A_se| > 12·Tf fires;
     - B > 0 with B = T0 fires.
   - Re-run the mutation that restores `B ≥ T0` and confirm that T6 kills it.
   - Confirm that D21-1's exact second test and the A_se third test are unchanged.
   - Confirm that CHANGE_RECORD records the deviation.
2. **The reader-window condition (ruling 1).**
   - The load-row finding enters at `append_integrity_report` on both call sites. R-b′ amends the same integrity diagnostic after the recovery loop, and only while it is still CHECKS_PASSED.
   - Independently enumerate every reader of the diagnostic code, the envelope's `solve_quality` and the case standing between the append and R-b′'s amendment. Cover Sensitive containment and withholding, receipt building, export and qualification gating, and headless digests. **A reader in that window that sees CHECKS_PASSED for a case R-b′ later demotes is BLOCKING.**
   - Confirm that the R-b′-only test and the both-guards test exist, and that each asserts the final envelope and every downstream view.
3. **The receipt residual (ruling 3).**
   - The multi-case captured refusal (`SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`) is accepted as a disclosed, fail-closed availability residual, owned by T3's composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item.
   - Confirm that it fails closed on **every reachable path**. **Any path that publishes a case value is BLOCKING.**
   - Confirm that the characterization test is labelled as a known residual, not desired behaviour, and that it asserts:
     - refusal with the finalization code;
     - no published case value;
     - single-case captured invocations demoted and not refused.
   - Confirm that I5 recorded the reach per slice (through S11-F's and K-D5's demotions as well as S11-G's) as facts, not presumptions.
   - Confirm that CHANGE_RECORD and the PR-body text disclose it.

## Addendum 2 (2026-09-27): revision 2.2 and the D22-1 condition

**Basis.** The design is now **S11G_GUARD revision 2.2** (`680fecdd…`, `3c80158e9`) plus D1's D22-1 erratum (§0.2), as selected in `ROOT_RULINGS_V1.md` "Selection: S11-G revision 2.2". Read it with V1's delta-2.2 section of `REVIEW/S11G_CHECK.md` (`ec0efce7…`). Where they differ, 2.2 replaces 2.1's routing gate and Addendum 1's receipt-residual scope.

**Also check:**
1. **G-1 to G-3.**
   - The routing gate is removed.
   - A load-row finding routes like a Sensitive verdict, with `OrdinaryAttempt`'s outcome sensitive (a constructor argument only; `wire()` and the finalize bodies are unchanged; no receipt contract change).
   - `decline_formation()` is present, including V1's 0.4.0 subnormal eigen-load corner.
2. **D22-1, the binding condition.** A Passed, guard-fired case that main would not attempt records a **zero-work** formation decline, not a charged attempt. Check:
   - the invocation ledger equals main's on such a case;
   - the result_export and desktop readers accept the entry under WORK_LEDGER and FAILURE_CATEGORY;
   - already-Sensitive guard-fired cases keep the real attempt;
   - re-run the mutation that restores the charged attempt, and confirm it is killed.
3. **Path 2 is gone.** Re-run T18 on I5's path-2 model: no refusal, case B SENSITIVE, and bytes equal to main's.
4. **The residual after 2.2** is only path 1's R-b′ variant. It fails closed: pre-0.4 captured returns `Err` with no envelope, and 0.4.0 republishes. T20 is labelled pre-0.4 only. T19 and T20 may use V1's single-basis construction (N1).
5. **The tests and mutations of 2.2:** T1's restated diagnostic set, T10/T10b restated, T18–T21, M19–M22, M7 withdrawn, and the D22-1 tests. Also the forecast: zero committed bytes, and UDL-W1e8 captured gaining one info diagnostic.

## Addendum 3 (ROOT, 2026-09-27): T20 and T6a

- **T20** is a characterization of the actual behaviour on I5's constructions: published, no receipt, case B demoted, not refused. The R-b′ path-1 residual is disclosed as "not demonstrated reachable". **Attempt a construction independently:** a selected case A beside a Passed case B that R-b′ demotes. If you find one that refuses, confirm that it fails closed (pre-0.4 captured `Err` with no envelope; 0.4.0 republished). **It is BLOCKING only if it publishes a value.** Report your constructions and their outcomes either way.
- **T6a's pressure run.** Check I5's resolution:
  - either the straight-thrust `RoundedProduct` family is exercised end to end through a solve path that reaches it, including a legacy replay;
  - or its unreachability is evidenced with file:line, a unit-level test covers the family's bound, and the deviation is disclosed in CHANGE_RECORD.

  Independently attempt to reach the family through any entry.
