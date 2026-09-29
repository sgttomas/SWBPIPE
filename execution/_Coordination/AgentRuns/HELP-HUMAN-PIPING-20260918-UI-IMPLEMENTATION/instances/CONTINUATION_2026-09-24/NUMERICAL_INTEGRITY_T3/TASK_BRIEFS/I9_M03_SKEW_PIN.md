# I9: the skew M03 pin (tests only)

This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host") override `_COMMON.md`'s host section, and apply to you in full.

## Roles

ROOT (HELP_HUMAN) dispatches you directly as a background subagent and is your return path. Make no Git writes; ROOT commits.

## Purpose

RV7's review of K2a found that M03's element-entry floor argument holds only for **axis-aligned** members (`REVIEW/K2A_REVIEW.md` B1; `IMPLEMENTATION/K2A/RETURN_ADDENDUM_1.md` §1 and §1.4).
- `transform_roundoff` bounds each global entry through Tᵀ|K|T.
- On a skew member, the rotational-block bounds mix GJ/L with 4EI/L and 2EI/L, and the coupling block holds only 6EI/L² terms.
- So on skew members M03 **accepts** subnormal-derived 4EI/L and 2EI/L below the floor, with errors up to about 1.2e-7. 6EI/L² is the limiting coefficient.

K2a now refuses all of these at formation, by name. M03's scope on skew members is still unpinned, and later slices lean on M03: K2b's scaling, K5, and F1b's move of the product onto the pattern path.

The work graph routed **the skew M03 pin** to K1's pattern-path M03 tests, with K5 as the fallback. K1 merged without it (ROOT's miss; `IMPLEMENTATION/K1_MERGE/RECORD.md`). This slice closes that gap before K2b touches the same assembly entry.

## Base, branch and paths

- **Branch:** `codex/piping-m03-skew-pin-20260928`, from main `eb52114e9` (K1 merged). ROOT creates it in `<wt>/skewpin`.
- **Target:** `<wt>/skewpin-target`.
- **Scratch:** `<wt>/scratch/i9`.

## Write set (tests and records only; no product code)

- **New test file(s)** in `frame_kernel/tests/` (for example `m03_skew_scope.rs`), and, if the pattern path needs the adapter, an append-only addition to `nonlinear_integration/src/structural_adapter/k1_tests.rs`.
- **Records** under `T3/IMPLEMENTATION/M03_SKEW_PIN/`: CHANGE_RECORD, RETURN, `_run_records/` and SHA256SUMS.
- **Not in scope. Stop and ask** before touching any of these:
  - any product source (`FK/lib.rs`, `structural.rs`, `sparse.rs`, `transform_roundoff`, `SA`, `PP`);
  - the committed fixtures and references.

## Required tests

1. **The kernel pin, on RV7's confirmed cases** (the table in `K2A_REVIEW.md` B1; the probe is `REVIEW/_run_records/k2a/rv7_rotated_m03.rs.txt`).
   - **Inputs:** L = 2^-39 m, OD 1e-11 m, wall 1e-12 m, G = 1e-100 Pa, and E from (12E)·I = 2^-1030, 2^-1040, 2^-1045, 2^-1050, 2^-1055, plus S6a ((12E)·I = 2.5·2^-1075 exactly).
   - **Orientations:** axis x with y-ref +y; skew (1,1,1) with y-ref +z; skew (1,2,2) with y-ref +x.
   - **The local matrix:** main's pre-K2a local matrix, formed operation for operation in the test, since K2a's `local_stiffness` now refuses these formations. Use FK's own orientation and `transform_roundoff`.
   - **Assert, per case and orientation, the accepted or refused outcome in the table:**
     - axis: refused everywhere;
     - both skews: accepted from 2^-1030 to 2^-1050, refused at 2^-1055 and in S6a;
     - every refusal is `Range("arithmetic outside normal range")`.
   - **Assert the figures that make it a pin, not an observation:**
     - the formed 4EI/L and 2EI/L are subnormal-derived and below the floor (about 2^-974.585) while accepted;
     - their relative error against an exact reference (Fraction or exact integer arithmetic; do not trust binary64) matches RV7's figures: 1.1e-13, 1.16e-10, 3.73e-9 and 1.19e-7 for 2EI/L down the accepted rows;
     - 6EI/L² is the limiting coefficient.
2. **The same outcomes through both representations.** Run each case through today's dense M03 and through K1's pattern path (`SparseStiffness` or `SparseAssemblyEvidence` with the same allowances), and assert identical acceptance or refusal and identical error text.
   - This is the "pattern-path M03 tests" of the routing.
   - If the pattern path cannot carry a pre-K2a matrix without a product change, stop and report.
3. **A behavioural guard for future work.** A test that fails if M03 starts refusing these skew cases, or starts accepting the axis case, so that any later change to M03's skew scope is deliberate and visible. The accepted skew rows are pinned **as today's documented limitation, not as desired behaviour**. Say so in the test's doc comment, citing K2a's addendum §1.4.

## Product evidence (a run, not a test)

The routing also asks for "a better-conditioned skew model (G comparable to E) on main, to establish main's downstream standing", which RV7 did not probe.
- **The model:** one member, N0 anchored, N1 free; G comparable to E; the same skew orientations, at (12E)·I cases where 4EI/L and 2EI/L are subnormal-derived. State every input.
- **Run it** through both entries, in both modes, on:
  - a `git archive` of **`134eefc24`** (pre-K2a main, with K-D5 and F1a);
  - a `git archive` of **`eb52114e9`** (current main).
- **Record** for each run: the standing, the quality, and the published value against an exact reference.
- **A claim about product behaviour needs a product run.** If pre-K2a main publishes a trusted (checks_passed or numerically_eligible) value wrong by more than the 1e-9 criterion, **stop and report at once**. It would change K2a's product-reach statement.

## Mutants

Run from clean archives, with a NONE control first:
- a mutant making `transform_roundoff` use the axis-aligned per-entry bound on skew members;
- a mutant that changes the floor;
- a mutant that drops the coupling block from the bound.

Each must be killed by your tests at a behavioural assertion. Add one of your own.

## Gates (ROOT runs the PR)

- **This slice's evidence:**
  - the full suites of frame_kernel, sparse_direct and nonlinear_integration against a Mac baseline of main `eb52114e9`;
  - T9 (Mac base against Mac candidate). It is expected to be byte-identical, since there is no product change.
- **Then:** an independent review; hosted CI; and DEC-025 under the owner's Mac-sweep decision (`K1_MERGE/RECORD.md`).

## Return

- **Files:** CHANGE_RECORD and RETURN, with `_run_records/`, SHA256SUMS and placeholders only.
- **RETURN covers:** the cases and outcomes; the exact-reference errors; the representation parity; the product evidence with its standing table; the mutation table; and what was not done.
- **End your turn** with a status for ROOT: the results, the changed files, and any stop.
