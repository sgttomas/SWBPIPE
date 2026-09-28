# RV8: independent full-diff review of slice K1

**Renumbered (ROOT, 2026-09-28).** This brief was first committed as `RV7_K1_REVIEW.md` (`88c3a3125`). The cloud session's T3 manager had already assigned RV7 to K2a's reviewer, so K1's reviewer is RV8. The reviewer's working paths keep the names it was spawned with: `<wt>/rv7-target` and `<wt>/scratch/rv7`.

This is a review TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. This brief overrides `_COMMON.md` where they differ. In particular, the Mac host rules in `I8R_K1_RESUME.md` ("The Mac host") apply to you in full.

**Independence.** You must not have drafted, implemented or tested K1 (I8 and I8R are excluded), or designed W3 or checked its design.

Your job is to find defects, not to confirm. Report what you find; you fix nothing.

## Roles

ROOT (HELP_HUMAN) dispatches you directly and is your return path. You are a background subagent of ROOT's session. Make no Git writes of any kind.

## Candidate

- **Branch:** `codex/piping-k1-20260928`. ROOT names the head at spawn.
- **Base:** main `134eefc24`, which is the merge base. The branch has four commits:
  - (a) the K1 slice;
  - (b) the `formation_check.rs` site-table rows;
  - (c) the KERNEL-list hunk;
  - (d) the records.
- **History.** It was reshaped at the clean point. The pre-reshape history, which contains I8's WIP `d08b0efc7`, is kept on `codex/piping-k1-wip-20260928`.
  - The reshaped head's code tree equals the tested tree `19925122b`.
  - Its whole tree equals the pre-reshape snapshot `3513fd8ab`.
  - Verify both claims.
- **Scope:** the complete diff from the base to the head. Every line is in scope, commit by commit.
- **Build only from `git archive <exact commit>`**, into your scratch. Never build in `<wt>/k1`.
- **Write set:** `T3/REVIEW/K1_REVIEW.md` and `T3/REVIEW/_run_records/k1_review/**` (with their own SHA256SUMS), in the numerics worktree `<wt>/numerics`. Don't write anywhere in `<wt>/k1`.
- **K2a is not merged yet.** K1's PR cannot merge before K2a's. After K2a merges, ROOT merges main into K1, I8R lands the K2a-interaction tests, and you (or a fresh reviewer) do a delta check. This review covers the pre-K2a head.

## Basis

1. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (`fb62ef4a…`):
   - §4.8 (W3), all of it;
   - the K1, K2b, K5, K6 and F1 rows of §6;
   - §7.1, the frozen references;
   - §7.3, mutations 8 and 10.
2. `T3/TASK_BRIEFS/I8_K1_IMPLEMENTATION.md` with addenda 1–4, and `I8R_K1_RESUME.md`.
3. `T3/ROOT_RULINGS_V1.md`:
   - the S11-K option (c) rulings;
   - the K-D5 sections;
   - the F1 split;
   - "K2a product reach", corrections 1–3;
   - the four K1 sections dated 2026-09-28: spawn and no gate; the S11 site table; the source-pin extension; the KERNEL list and handoff.
4. `T3/PLATFORM_CALIBRATION_MAC/RECORD.md`, for why T9 on this Mac is a Mac-only comparison, and the three Mac-main platform failures.
5. The candidate's `T3/IMPLEMENTATION/K1/` (CHANGE_RECORD, RETURN, `_run_records/`, and the WIP artefacts).

## What to check (at least)

1. **A complete-diff review** of the named head, citing that revision.
   - Re-derive by lexer scan, independently, the callers of every new or refactored entry. That includes:
     - `solve_structural_dense*`, `solve_structural_sparse*` and `factor_structural_ldlt`;
     - `negative_pair_witness` and `verify_negative_direction`;
     - the SA solve entries.
   - Compare the result with `_run_records/callers.txt`. Confirm that no product caller changes, and that the one product call is still PP's dense `solve_preview_reduced_system` path.
2. **Dense behaviour is unchanged.** `FK/structural.rs` was refactored so that its gate stages are generic over two private traits. Every dense call must stay byte-identical.
   - Don't rely only on the existing tests. Build a differential probe against base and candidate, both on this Mac. Compare `Debug` of `StructuralReport` and `StructuralSolution`, and the errors, through the public dense and today's sparse entries. Use:
     - the N01–N09 and R01–R07 references;
     - NP-B and NP-D;
     - product-shaped models with frames, user elements, realized curved bends, springs and prescribed motion.
3. **Try to break the bitwise parity claims.** The coalesced pattern values must equal the dense assembly entry for entry, bit for bit, and the pattern path's adjacency, RCM order and profile must equal today's.
   - Look for models where accumulation order could differ:
     - nodes with three or more skew members;
     - mixed frames, users, blocks and springs at one node;
     - springs on prescribed DOFs;
     - duplicate connectivity;
     - two modulus bases.
   - Look for explicit-zero edge cases: exact cancellation, −0.0 against +0.0, and an entry that is zero on one basis only.
   - Report any divergence, with its inputs. A parity failure is BLOCKING.
4. **Test the O(nnz) claims.** The contribution audit, residual, rcond norm and negative witness must visit pattern entries only.
   - Read the code.
   - Check that the pair-count test is not vacuous: its model's dense pair count must be much larger than its stored pairs.
   - Confirm that no dense n×n allocation remains on the pattern path. `to_dense` and `dense_symmetry_view` are dense-scrutiny views only.
   - **Don't allocate dense at 10,000 members yourself.**
5. **The pin extension** (`s11k_tests.rs`), against the original intent of the pins (S11-K option (c), K-D5's pins).
   - Try at least three evasions:
     - an alias or helper;
     - UFCS;
     - comment or string text that satisfies a source pin;
     - a route through a sibling module or a new impl.
   - Report whether each is caught, and whether behaviourally or by text.
   - Confirm ROOT's three required mutants are killed. Confirm the original mutants (`KD5-M32a`, `KD5-M32b`, `KD5-E4`, and `S11K-RV-OPT1`, `-OPT3`, `-OPT4`, `-PUB`) keep their original kill sets.
6. **The site table and the KERNEL list,** under ROOT's five conditions.
   - Commits (b) and (c) are separate, and each is additive.
   - The dispositions are right. Is `sparse_audit_contributions`'s `|rhs|` norm really a perturbation-estimate norm, as I8 claims?
   - `formation_check.rs` has no binary64 load, force or RHS fold.
7. **The reviewer items in RETURN §6,** the test-fixture changes made at checkpoint A. For each, decide whether it corrects a wrong fixture or weakens a test. Specifically:
   - the O(n⁴) dense witness cross-check, reduced to a 6-member chain;
   - the C3-detect case moved to G = 1e12 on an unsettled chain;
   - the added formation allowances, plus the bare parity checks;
   - the assembly-refusal fixture;
   - witness case 2.

   A change that weakens a required test is SHOULD-FIX or BLOCKING, by impact.
8. **The mutation table** (RETURN §10, `_run_records/mutations/`).
   - Re-kill a sample independently, in your own clean archives: at least `K1-M8`, `K1-ORDER`, `K1-KFC`, `K1-DENSEPAIRS`, `K1-LABEL-ORDER`, one pin mutant and one original.
   - Add at least two mutants of your own aimed at gaps you suspect: for example a curved-block-only value error, a spring-on-prescribed-DOF error, or a formation-check parity break.
   - A mutant killed only by a source-text pin, where a behavioural test was expected, is a finding.
   - ROOT's ruling on K1-LABEL: it is not demonstrated, and K1-LABEL-ORDER is mutation 10's demonstrated form. Check that the table states this truthfully.
9. **The F1b interface** (RETURN §12). Check it against the brief's list:
   - the pattern builder per modulus basis;
   - the sparse `AssemblyEvidence`;
   - the plain and formation-checked pattern solves;
   - reduction with prescribed displacements, and partition maps;
   - reactions from sparse rows;
   - the dense view;
   - storage counts;
   - an options struct that lets K2b add b without touching callers.

   Are the signatures exact at the head?
10. **Records and hygiene.**
    - SHA256SUMS verify.
    - GEN-8 passes on the head (`pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, with `<VENV>`).
    - There are no machine paths and no model identifiers.
    - T9 is stated as Mac-only.
    - The suites claim is exact: the three Mac-main platform failures, and nothing else.
    - The sanitization is disclosed. The record logs had trailing whitespace stripped at commit, and the WIP `.patch` files keep their patch-syntax spaces.
    - "Gate not run" cites ROOT's ruling.
    - "Not done" is honest.
11. **The invalid-input differences** (`WIP_STATE.md` §2.6: the sparse side refuses where dense `+=` would panic, and so on). Decide whether each is acceptable, and whether any can reach a product path.

## Running things

- **The Mac host rules** in `I8R_K1_RESUME.md` apply to you in full:
  - no swap;
  - `-j 8` at most and `RUST_TEST_THREADS=4`;
  - at most two cargo jobs of yours at once, and three mutants at `-j 4`;
  - no dense materialization at 10,000 members;
  - the memory guard at `<wt>/guard/memguard.log`.
- **Your own paths:** target `<wt>/rv7-target`, scratch `<wt>/scratch/rv7`. Delete your mutant targets after use.
- **I8R may be idle or doing small record work.** ROOT tells you if another heavy job starts.
- Use `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0` and `--offline --locked`. Skip no tests and raise no timeouts.

## Verdict and return

Write `T3/REVIEW/K1_REVIEW.md` in `<wt>/numerics`, containing:
- the revisions reviewed;
- a findings table (ID, severity BLOCKING / SHOULD-FIX / NOTE, site, evidence, resolution);
- a section per check;
- your independent caller list;
- the dense differential;
- the parity attacks, with each model tried;
- the mutation re-kills and your own mutants;
- the evasion attempts;
- what you ran;
- what you did not check.

Keep machine paths out: use `<wt>/…` and `<scratch>/…`.

The verdict is **PASS** (no unresolved BLOCKING findings) or **FAIL**. End your turn with a summary for ROOT: the verdict, the finding counts, and the file's sha256.
