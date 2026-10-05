# RV91: independent review of U6d (TypeScript carriers and standing)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You did not write this code. Don't rely on the author's tests as your oracles.** This unit was moved from RV88's queue to add review capacity (workflow §4).

**Read** `BRIEFS/U6_FANOUT_COMMON.md`, PLAN `R/I66/u6_scoping_01/PLAN.md` (§1d, §3, §4 and §6 U6d), and the rulings from "U6 plan accepted…" through "U6d (TypeScript carriers) verified and committed; S-1 ruled" and "U6b (Python carriers) verified…".

## The candidate

- **The commit:** `9555b6ffc2` on `codex/piping-f2a-carriers-ts-20261004`, from U6a's `844448112f`.
- **The author's account:** `R/I67/u6d_typescript_01/RETURN.md`.
- **ROOT's runs:** desktop Vitest 3,417/3,417, and `tsc` clean.

## Review, in priority order

1. **Existing behaviour unchanged.** Run your own sweep of existing-identity envelopes through the desktop carriers (dispatch, standing, binding and display) against base `844448112f`. Run the full Vitest suite and `tsc` on your archive.
2. **Registration and standing.**
   - Registration is bound to the exact bytes and the captured invocation; a later edit voids it.
   - Standing reads only the registration, never `numerical_quality`, and stays `needs_recompute`.
   - The 14 parity cases in `retained_precision_carrier_cases.json` agree with Rust U6a and Python U6b.
   - **I67's F1:** an unregistered invalid successor is `needs_recompute` in TS, but `unsupported` in Rust and Python. Judge it against D2 §4.7's parity, and say whether TS must match.
3. **The downgrade guards and refusal codes:** AnalysisRun receipt equality, reopen, the rule-check gate, and every T6 surface refusing (no T6 panel edited).
4. **The S-1 pin patch** keeps exact equality.
5. **New product text (F2, F6):** `N_RP_UNVALIDATED`, the per-case notices, the output refusal text, the standing text, and b rounded upward to 3 significant digits. Each must claim nothing beyond the receipt.
6. **I66's F-U6b-3:** does the TS AnalysisRun legacy builder silently drop a `retained_precision` member, as Python's v0.2 builder did? Report it; I67 aligns it in the repair round.
7. **Mutants:** re-run a sample of I67's 103, and add at least five of your own.

## Host

- **Your copy:** from `git archive 9555b6ffc2`, in `WT/rv91/`. Link REPO_ROOT's `P/node_modules`, and copy the prebuilt WASM from `WT/f2a-readers`. Never install or build, and disclose both.
- **Scratch:** `WT/scratch/rv91_u6d/`, never the system temp directory. Set `TMPDIR` there.
- **The memory guard** must be running.
- **Never:** Git writes, installs, new tooling, or native, solver or DEC-025 jobs.

## Output

- **The report:** `NUM/R/REVIEW_RV91/u6d_01/REVIEW.md`, containing a verdict, counts, findings (path:line, evidence, remedy) and SHA256SUMS.
- **Time box:** 3 h.
- **End your turn** with the verdict, the counts, one line per finding, the sha256, and anything ROOT must rule on.
