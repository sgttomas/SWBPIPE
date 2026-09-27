# RV5: independent full-diff review of slice K-D5

This is a review TASK. Read `_COMMON.md` first. This brief overrides it where they differ.

**Independence.** You must not have:
- designed D-5, R5-4 or K3a, or checked their designs (D1 and V1 are excluded);
- implemented K3a or K-D5 (I2 and I3 are excluded).

Your job is to find defects, not to confirm. Report what you find; you fix nothing.

## Candidate

- **Branch:** `codex/piping-kd5-20260926`, in `<wt>/kd5`.
- **Head:** the commit the manager names at spawn. It comes **after** S11-F (PR1000) has merged, K-D5 has merged main forward again, and I3 has finished its addendum-4 pass and RETURN addendum.
  - The earlier commits are `17f3d6e05` (the K-D5 implementation, on base `a2e804a75`) and `b6156d49d` (the merge of origin/main `b0a9a52b6`). Neither of them is the candidate.
  - If the head you are given predates the post-PR1000 merge or the addendum-4 pass, stop and tell the manager.
- **Scope:** review the complete diff from the merge base with `origin/main` to that head. Record both revisions. Every line is in scope:
  - FK `structural.rs`, `formation_check.rs` and its tests, and `retained/mod.rs` and `wide.rs`;
  - SA `structural_adapter.rs`, `kd5_tests.rs` and `kd5_models.rs`;
  - the PP call site and `tests/formation_check_runtime.rs`;
  - the nonlinear pins in `s11k_tests.rs`;
  - the records under `T3/IMPLEMENTATION/KD5/**`.
- **Write set:** `T3/REVIEW/KD5_REVIEW.md` and `T3/REVIEW/_run_records/kd5_review/**` (with their own SHA256SUMS), **in the numerics worktree** (`<wt>/numerics`).
  - Don't write in `<wt>/kd5`.
  - Builds, probes and mutations run in a scratch clone under `<scratch>`, with your own target.

## Basis

1. `T3/ROOT_SELECTION_DESIGNS.md` (the §4 hard constraints, and C2 and C5).
2. `T3/DESIGN_NUMERICS/DESIGN.md` **revision 5a.2** (`fb62ef4a…`):
   - §4.3 and §4.3.1: the trigger 2|w|/criterion > 1, the factor-2 margin, the zero-scale clause, and "nonlinear supports never selected";
   - the K-D5 row of the §6 slice table;
   - the §9 mutations (23), (26)–(28), (31) and (32).
3. `T3/DESIGN_NUMERICS/R5_4_CURVED.md` (`2c9fae78…`), with `_run_records/curved_ef.py`, and `D5_TRIGGER.md` (`f6e24a69…`) for background.
4. `T3/TASK_BRIEFS/I3_KD5_IMPLEMENTATION.md`, with addenda 1–4 (K3a's API; realization decisions B and the `selected` flag; AngleDomain near π; per-item dead_code).
5. `T3/ROOT_RULINGS_V1.md`, especially:
   - D-5, D5C-1 to D5C-5, R5-4, option (c) and the nonlinear-support ruling;
   - the S11-G rulings (DS-1: K-D5 is **expected** to stay silent on the INPLANE formation cases after S11-F; don't report that as a defect);
   - **"K-D5 mutation M31b: accepted as equivalent at the criterion"** (`c2042fd9c`).
6. `T3/REVIEW/VERIFY_R5.md` (item 16), `T3/REVIEW/D5_CHECK.md` and `T3/REVIEW/K3A_REVIEW.md`.
7. `T3/GATE/S11_EXCEPTIONS.json` (221 triples), `GATE/FORMATION_EXCEPTIONS.json` (7 triples), `T3/REFERENCES/references.json` (`c0f14201c`), and RF-ELOAD (`b6927f783`).

## What to check (at least)

1. **A complete-diff review of the actual candidate.**
   - Read every changed line at the named head. Findings must cite that revision, never an earlier one.
   - Independently re-derive, by lexer scan, the caller list of every changed function (`finish_checked_factor`, `evaluate_original_residual`, the SA solve entries, and the new check entries). Compare it with I3's `_run_records/callers.txt`.
2. **An attack on the M31b equivalence. Try to break it.**
   - **The mutant.** M31b re-forms K_t, but builds the curved element's H from the product's formula chord R(cos φ − 1, R sin φ, 0) (binary64 R, atan2, cos and sin) instead of the actual node chord. Reproduce it, and M31b0 (the same formula at p), in your scratch clone.
   - **Search the admissible product models for one where the chord substitution matters at the criterion.** Cover at least:
     - small bend angles;
     - angles near π, short of the AngleDomain refusal (for example π − 1e-6, π − 1e-10 and π − 1e-15);
     - the maximum admissible radius mismatch (the product's 1e-9 relative tolerance), with the most distorted admissible centre;
     - extreme R/L;
     - stiff-X and soft-Y combinations;
     - multi-bend chains, where chord errors could accumulate.
   - **Include inputs on both sides of the trigger:** tune a stiffness so the correct check's trigger 2|w|/criterion sits just above and just below 1. Report EF_correct, EF_M31b, and |ΔEF|/criterion for each model.
   - **A counterexample, which is BLOCKING, is an admissible model where either:**
     - **(a)** M31b does not demote, but the published value's actual error against an exact reference (computed in `Fraction` or at high precision) exceeds the criterion, which would be a silently wrong Passed value under the mutant; or
     - **(b)** |EF_M31b − EF_correct| ≥ 0.5·criterion, meaning the chord substitution alone consumes the factor-2 margin.
   - **Near-boundary flips.** Tuning the trigger to within |ΔEF| of 1 flips the demotion outcome in a band as wide as |ΔEF|. Such a flip is not by itself a counterexample. Report the largest |ΔEF|/criterion you find as a NOTE.
   - **If you find a counterexample,** give its full inputs. It becomes a required test, and M31b must then be killed before merge.
   - **What stays required whatever you find:** the implementation must still use the actual chord (R5-4 §2 step 6), and `kd5_curved_intended_element_uses_the_actual_chord` must kill M31a. Confirm both.
3. **Fail-closed and "never passes":**
   - **Every path that cannot re-form** yields `formation_check_unavailable`, the case becomes Sensitive, and no path returns an `Err` from the solve. Enumerate every such path from the source, not from I3's list:
     - any `WideError`, including AngleDomain within about 1e-19 of π and ArctangentLimit;
     - an unmatched curved slot, an explicit `CurvedBendStiffnessElement::new` slot, and a one-ulp matrix mismatch;
     - a joint with nonzero lateral stiffness;
     - any family the check cannot re-form;
     - the accumulator's truncation and allowance.
   - **No value changes:** a demotion changes the status only; every published value is bit-identical to base. `StructuralReport` is unchanged.
   - **No in-band marker** in any published value or field.
   - **Nonlinear-support cases are never selected:** `selected = built.nonlinear_supports.is_empty()`. Such a case's linear attempt, ordinary_attempt and receipt are byte-identical to base. The nonlinear loop reaches no formation check through any path, including option B's matching.
4. **The gate claims, checked against the committed run records** in `T3/IMPLEMENTATION/KD5/_run_records/gate/` (and the candidate's re-run, if I3's addendum-4 pass produced one):
   - **888 runs:** 222 cases × 2 modes × 2 entries. Recount them from the records.
   - **The trusted breach triples** equal exactly the 221 of `S11_EXCEPTIONS.json` plus the 7 of `FORMATION_EXCEPTIONS.json` (228). If S11-F's merge has emptied `S11_EXCEPTIONS.json`, then the candidate's gate must be checked against the lists **as they stand on the candidate**, and any breach outside them is a FAIL.
   - **The only standing change** against P1's main baseline is RF-SKEW-T-CANT-OFF-122-r1e-04 (checks_passed → sensitive, on both entries in both modes). Its trigger values are 4.827 dense and 2.428 sparse. It must appear nowhere as a Passed breach.
   - **The 4 timeouts at 1800 s** match P1 on main, and no timeout was raised.
   - **The provenance file** supports one continuous run of one binary, including across the reported container restart.
   - **The cost claim:** CONT-n1000 took 193 s against P1's 204 s dense.
   - **Your own re-run:** the 122 case on both entries in both modes, plus RF-SKEW 345, the RF-CHAIN r1e-04 controls and at least 20 other cases of your choice from the frozen references. Confirm they match the records.
5. **An independent re-kill of a sample of the mutations.**
   - At least (23), one of (26)–(28), **31a, 32a and 32b**, in your scratch clone, each with its patch, the command and the killing test.
   - Confirm that M31b survives, as recorded.
   - A mutant killed only by a source-text pin, where a behavioural test was expected, is a finding.
6. **The standing checks:**
   - **GEN-8 portability:** no machine paths (home, temp or tool-install) in committed records. Run `pytest tools/practitioner_harness/test_live_baseline.py -k gen8` on the candidate.
   - **dead_code per item:** no module-wide `allow(dead_code)` or `cfg_attr(…, allow(dead_code))` remains on `retained` or `wide`. Each remaining `#[allow(dead_code)]` sits on exactly one item that is unused outside tests, with a one-line reason. Check the list against actual use.
   - **PP uses only the typed entry after S11-F:** the call at the former PP:3965 is `solve_assembled_with_formation_check` (or its final name), with `&AssembledForce`, the curved sources and the `selected` flag. No `&[f64]` product entry remains, and no other PP change.
   - **The pins are folded into `s11k_tests.rs`:** I3's nonlinear pins now sit in I1's module, and each has a behavioural test that first asserts the two paths differ (the paths-differ precondition), then asserts the outcome.
     - Attempt at least three RV1-style evasions against them: an alias or helper, UFCS, comment or string text satisfying a source pin, and a route through a sibling module.
     - Report whether each is caught behaviourally.
   - **Unchanged surfaces:** option (c) is intact (the loop reaches only `_binary64` variants); DEC-046 and `benchmarks/nonlinear` are unchanged; the C2 scripts' outputs are unchanged.
7. **Disclosure and hygiene:**
   - CHANGE_RECORD states which cases can change standing and why, that no value changes, the fixture result (112 of 112 identical), the cost per curved element, and that there is no in-band marker. It records the M31b equivalence with ROOT's citation and the design-claim note (M31a versus M31b).
   - RETURN's §7 and §8 match the records.
   - The records' SHA256SUMS verify.
   - `cargo fmt` has been run on changed files.
   - `git diff --check` is clean, or the exceptions are recorded; hash-bound raw logs keep their bytes.
   - No `node_modules` links are committed.

## Running things

- Use `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0` and `--offline --locked`. Use your own `CARGO_TARGET_DIR` under `<scratch>`, and prune it when you're done.
- **One heavy cargo job at a time across T3.** Check `pgrep -x cargo` before each job.
- **Hold all cargo while a DEC-025 sweep runs:** check `pgrep -f 'python[0-9.]* .*run_evidence_sweep'`.
- **Wait patterns must not match the waiting shell.** Anchor on the interpreter, as above.
- **Keep free disk above about 8 GB.** Check it before each build, and prune your target if it falls lower.
- **The authority targets are prerequisites, never scratch.** Never delete or `git clean` `core/serialization/canonical_json/target` or `core/units/target` in any worktree. Build them in your scratch clone with the two `tools/…/build_*.py` scripts before any Python run.
- Skip no tests and raise no timeouts. Make no Git writes.

## Verdict and return

Write `T3/REVIEW/KD5_REVIEW.md` in `<wt>/numerics`, containing:
- the revisions reviewed;
- a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, resolution);
- a section per check;
- your independent caller list;
- **the M31b attack:** each model tried, its parameters, EF_correct, EF_M31b and |ΔEF|/criterion, and the conclusion;
- the gate re-check and your re-run sample;
- the mutation re-kills;
- the evasion attempts;
- what you ran;
- what you did not check.

Keep machine paths out of the review and its records: use `<wt>/…` and `<scratch>/…`.

The verdict is **PASS** (no unresolved BLOCKING findings) or **FAIL**. Send the manager a SendMessage summary with the verdict, the finding counts, the M31b result and the file's sha256.

## Addendum 1 (ROOT, 2026-09-27): M31b counterexample definition

ROOT has confirmed the definition of a counterexample used in check 2.

**BLOCKING:**
- **(a)** M31b fails to demote an admissible model whose published value has an actual error, against an exact reference, above the criterion.
- **(b)** |EF_M31b − EF_correct| ≥ 0.5·criterion.

**NOTE:**
- **Tuned near-boundary flips:** report the largest |ΔEF|/criterion found.
- **The reverse direction:** M31b demotes where the correct H would not. This is a false demotion, not a safety issue.

**Fail-closed cases.** The attack must also cover near-π bends and radius-mismatch models on which the correct check must fail closed with `formation_check_unavailable`. If M31b turns a correctly fail-closed case into a completed check that does not demote, that is **BLOCKING under (a)**, even when its EF looks small.
