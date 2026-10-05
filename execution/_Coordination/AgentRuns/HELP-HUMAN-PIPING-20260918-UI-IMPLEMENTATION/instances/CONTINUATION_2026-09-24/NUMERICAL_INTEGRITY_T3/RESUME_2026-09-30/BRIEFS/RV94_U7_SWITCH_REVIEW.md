# RV94: independent review of U7, the eligibility switch-on

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate. **You wrote none of this code. Build your own oracle for eligibility from the contract, and don't rely on the implementers' oracle or tests.**

## The candidate

- **The diff:** the U7 branch `codex/piping-f2a-u7-20261004` at the head ROOT gives you, against the U7 base. The U7 base is NUM after the memory merge, merged into the U7 branch. Review the whole diff. It comprises:
  - **slice T** (I67, TS):
    - standing bound to the live native capture and the model (RV91 N-2 = RV88 U6d S-1);
    - the explicit panel gate (RV91 N-5);
    - standing compared by token, with TS's status mapping pinned (RV92 N-8);
  - **slice P:** N-6's line-neutral docs on `into_parts()` and `successor()` in PP `lib.rs`;
  - **slice F** (I66, then I67): the three flags, 07i, the pins, the D-U7-6 sentences and D-U7-4's declared difference, in one commit;
  - **slice L** (I61): the live reruns in all three languages, the PP sweeps in both builds, U5 and the tokens.
- **The implementers' accounts:**
  - `R/I67/u7_slice_t_01/`;
  - `R/I66/u7_slice_f_01/` and `R/I67/u7_slice_f_01/`;
  - `R/I61/u7_slice_a_01/` and `R/I61/u7_slice_l_01/`.
- **The plan and rulings:**
  - `R/I61/u7_scoping_01/PLAN.md`;
  - the rulings "…U7 planned and ruled", "U7 slice A returned…", and the U7 rulings after them;
  - C1:160 and :162, D2 §4.9.4, D-U6-1 and C-2.
- **G7 Pass B on the U7 head** (slice Q) is reviewed by RV89, not by you. Take its verdict as an input.

## Review, in priority order

1. **Exactly the eligible set changes.**
   - With your own contract-derived oracle, determine which statements become `numerically_eligible`.
   - Compare with the candidate's behaviour in Python, Rust and TypeScript over:
     - the corpus bases and must-pass entries;
     - every mutation;
     - U6's carrier cases;
     - the live milestone successors, with and without their invocation;
     - hostile variants you build, such as an unavailable case, a missing invocation, a mismatched invocation, a stale model in TS, and a non-live capture in TS.
   - Nothing outside the set may become eligible. Gate outcomes and classifications must not change.
2. **The three languages agree** on every input, except where a declared difference says otherwise. Check that D-U7-4 is truthfully stated and pinned.
3. **No published byte changes.** PP's registered and Stale sweeps are byte-identical to the U7 base. The milestone still publishes U1's pinned successor.
4. **Consumers.**
   - The TS export panels refuse a successor by the explicit gate.
   - TS rule checks and summaries behave as the oracle says.
   - The stress-neutral packager and D-U6-9 still refuse.
   - The runner binding stays inert.
5. **D-U7-6:** no text claims producer origin. Every stale "held until U7" claim is gone or corrected.
6. **Nothing weakened.**
   - No assertion is deleted, and no pin is loosened; each pin changes only its expected value.
   - No reader gate is relaxed.
   - Line-neutrality holds where U4's TEXT keys need it.
7. **Mutants.**
   - Rerun the implementers' mutants.
   - Add at least six of your own: each flag reverted alone; each eligibility condition dropped in each language; TS's N-2 binding removed; the N-5 gate removed; comparison by status string.
   - Report survivors.

## Host and method

- **Your copy:** build from `git archive` of the candidate in `WT/rv94/`, with targets in `WT/targets/rv94/` (plus a Stale target) and logs in `WT/scratch/rv94_u7_01/`. Delete the copies afterwards.
- **Toolchains:**
  - Cargo: the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time;
  - Python: the repository's;
  - TypeScript: vitest and tsc with an untracked `node_modules` symlink, and prebuilt WASM only as the implementers used it.
- **The memory guard** (PID 5387) must be running.
- **Never:** Git writes, installs, or native, solver-at-scale or DEC-025 jobs. Nothing goes to the system temp directory.

## Output

- **The report:** `NUM/R/REVIEW_RV94/u7_01/REVIEW.md` plus SHA256SUMS, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a section per item.
  
  Use placeholder paths only.
- **Time box:** 4 hours.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
