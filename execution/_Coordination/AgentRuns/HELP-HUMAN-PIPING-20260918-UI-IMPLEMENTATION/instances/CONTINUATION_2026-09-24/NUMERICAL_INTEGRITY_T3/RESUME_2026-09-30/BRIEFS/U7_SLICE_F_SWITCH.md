# U7 slice F: the atomic eligibility switch (I66 for Python and Rust, then I67 for TypeScript)

TASKs (Type 2) dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. Neither of you delegates. The two parts share one worktree **sequentially**: I66 first, then I67. ROOT commits once, after both, so that **all three flags move in one commit** (D-U7-5).

## Basis

- **The plan:** `R/I61/u7_scoping_01/PLAN.md`.
- **The rulings:** "RV93 on U3 grant 2: PASS; RV89 confirms the final basis re-qualified; U7 planned and ruled" (D-U7-1 to D-U7-6), and "U7 slice A returned…".
- **Slice A's records:** `R/I61/u7_slice_a_01/`. They hold the inventory with every pin's post-U7 value, the stdlib oracle (`oracle_post_u7.json`), the 07i patch (`u7_07i_expectations.patch`) and the order of work (RETURN §3).
- **Slice T** (I67, TS standing binding, the explicit panel gate and the token pin) and **slice P** (N-6's docs) are committed on the U7 branch before F starts.
- **The worktree:** WT/f2a-u7, branch `codex/piping-f2a-u7-20261004`, after ROOT merges NUM (with the memory branch) into it. ROOT gives the exact head at dispatch. Leave the work uncommitted.

## Part 1: I66 (Python and Rust, the shared fixtures, PP's pin)

1. **Apply 07i:** `patch -p1` from `projects/chirality-piping`. Check the staged hashes: 07i `1e53ea9c…`, case file `f20a7db0…`.
2. **Flip the flags:**
   - Python, `retained_precision.py:30`;
   - Rust, `retained_precision.rs:4269`. The Rust edit must be **line-neutral**, so U4's TEXT line keys hold. Read QUALIFICATION_G7's rule lines first.
3. **Rewrite the stale comments and docstrings** in Python and Rust with slice A's proposed text. The Rust ones stay line-neutral (`retained_precision.rs:4267–4268`, `semantic_contract.rs:581–582`).
4. **Update every Python and Rust eligibility pin** to its oracle value, per slice A §1, including PP `retained_wire_tests.rs:122`. **No assertion may be deleted or weakened.** A pin changes only its expected value.
5. **The D-U7-6 sentences:**
   - the case file's scope (applied by the patch);
   - the note.
   
   The Rust and Python scope assertions require the new sentence.
6. **Fix any fixture or comment text the switch makes false,** including the corpus bases' `qualification` strings if they become false. List each.

## Part 2: I67 (TypeScript), after I66 returns

1. **Flip the TS flag,** `retainedPrecision.ts:97`, and update the stale comments.
2. **Update every TS pin** to its oracle value, per slice A §1. No deletion or weakening.
3. **Add the D-U7-6 sentence** to `retainedPrecisionStanding.ts`'s `tail`, and the TS scope assertion.
4. **Add D-U7-4's declared-difference entry** (drafted in slice T) to the case file's `declared_differences`. Update the three languages' declared-difference consumers to require it.

## Controls (each failure is a stop)

1. **The oracle diff.** Across the corpus, the must-pass entries, U6's 20 carrier cases and the live milestone, eligibility, the standing token and the withheld counts change **exactly** on the oracle's set: 13 bases, 13 must-pass entries, the 2 milestone `…:invocation` cases and the live successor with its invocation. Nothing else changes.
2. **Gate outcomes and classifications are unchanged** for every input. So are the derivative, row binding, transports, the packager and D-U6-9.
3. **No published byte changes.** PP's registered and Stale sweeps are byte-identical to `u3_grant2_02`'s.
4. **Mutants,** none killed only by a compile error:
   - each flag reverted alone;
   - each eligibility condition dropped;
   - the D-U7-6 assertion removed;
   - the D-U7-4 entry removed.
5. **Suites:**
   - Python: the 24-file sweep plus the three retained suites;
   - result_export;
   - PP, registered and Stale;
   - runner/headless;
   - vitest and tsc.
   
   Compare every one with the U7 base head, and explain every outcome change by the oracle.

## Host

- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time, in your own target dirs. Never the default target of WT/f2a-memory.
- **The memory guard** (PID 5387) must be running.
- **Never:** Git writes, installs, or native, solver-at-scale or DEC-025 jobs. Nothing goes to the system temp directory.
- **node_modules:** an untracked symlink only.
- **Records:**
  - I66: `NUM/R/I66/u7_slice_f_01/`;
  - I67: `NUM/R/I67/u7_slice_f_01/`.
  
  Each has RETURN.md and SHA256SUMS, with placeholder paths only.

## Budget and return

I66: 3 h. I67: 2 h. Each returns once, with the changed files and their hashes, the control results and anything ROOT must rule on.
