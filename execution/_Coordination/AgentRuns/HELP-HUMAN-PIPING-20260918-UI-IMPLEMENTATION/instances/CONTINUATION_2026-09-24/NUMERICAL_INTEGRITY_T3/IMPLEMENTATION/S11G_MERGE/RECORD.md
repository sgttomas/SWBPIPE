# S11-G merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1003. ROOT (HELP_HUMAN) merged it on 2026-09-27 as `b24b3d5360a4809d7c584c1780a39955fa810dcd`, under the owner's standing Git authorization.
- **Candidate head:** `e6f45d30ff1593d9032146fe32528cafccda6b2a`, on branch `codex/piping-s11g-20260927`. It consists of:
  - I5's S11-G implementation `b62e40d4d` (S11G_GUARD revision 2.2 plus the D22-1 erratum), on main `72d5ff864` (forward merge `3d844fea4`);
  - the formation-list emptying `759dccf35`, cherry-picked from numerics `37bdc2edd` so that it lands in the same merge;
  - the RV4 repair `6d6d31923`;
  - the records-only restoration of the committed mutation driver, `e6f45d30f`.

## Gates

- **RV4's independent complete-diff review** of `759dccf35` (`REVIEW/S11G_REVIEW.md`, commit `1724f2ad5`): **PASS**, with 0 BLOCKING, 4 SHOULD-FIX and 10 NOTE findings.
  - **S1:** the R-b′ path-1 residual is reachable (RV4's construction C1) and fails closed.
  - **S2–S4:** RV-M1, RV-M2 and RV-M3 survived.

  RV4 confirmed:
  - G-1 to G-3 and D22-1 (the invocation budget equals main's, and both readers accept the zero-work entry);
  - the `B > 0 && B ≥ T0` erratum and the reader window;
  - T18 byte-identical to base;
  - all 14 formation rows going from CHECKS_PASSED to SENSITIVE with no value change;
  - the FORMATION_EXCEPTIONS emptying, reproduced by the generator;
  - T9 (218 of 218 identical);
  - T6a unreachable.
- **RV4's delta check** of `759dccf35..e6f45d30f` (the delta section of `REVIEW/S11G_REVIEW.md`, commit `d706f6e2f`): **PASS**, with 0 BLOCKING, 0 SHOULD-FIX and 2 NOTE findings.
  - T20 now pins C1.
  - RV-M1, M2, M3, M6 and M10 are each killed by a behavioural assertion.
  - Every hash-bound file from `b62e40d4d` is byte-unchanged.
  - The delta is tests, records and layout only.
- **The DEC-025 sandboxed sweep,** on `759dccf35`: `dec025/SWEEP_20260927T113957Z_759dccf35305.json`, overall **pass**, `working_tree_dirty: false`. All four surfaces passed:
  - the cargo crate sweep;
  - pytest: 3023 passed, 32 skipped;
  - desktop vitest: 134 files and 2822/2822 tests;
  - the production build.

  The run window was 11:39:54Z–12:18:39Z, exit 0, recorded in `dec025/meta.txt`; the full log is `dec025/sweep.log`. Machine paths are replaced with `<WORKTREE>`, `<wt>`, `<VENV>` and `<scratch>`. The source_recovery.rs `stage` dead_code warning in the log is also in the S11-F sweep log, so it predates S11-G.
  - **Why the sweep on `759dccf35` stands for the merged head `e6f45d30f`:** RV4's delta check confirmed that the change after `759dccf35` is tests, records and layout only. Product_physics `lib.rs` is token-identical to `b62e40d4d` apart from 10 rustfmt trailing commas, its rustfmt count equals base's, and no other product file changed. This follows PR1002's precedent, with one difference: unlike PR1002, this delta changed `lib.rs` bytes (layout only). It therefore stands on RV4's token-level check plus the hosted Numerical cargo suite being green on `e6f45d30f`.
- **Hosted CI** and the **surface-4 dual-viewport dispatch**, with full-SHA target_base `72d5ff864`: green on `e6f45d30f`, and the PR-event E2E run on `e6f45d30f` is green. An earlier red E2E run on `6d6d31923` was cancelled by the push to `e6f45d30f` and is superseded.

## What main now carries

- `GATE/S11_EXCEPTIONS.json` and `GATE/FORMATION_EXCEPTIONS.json` are both empty. Any Passed breach over the frozen references now fails the gate.
- **The one disclosed residual:** R-b′'s path 1 (ruling 3 and its C1 note). It is reachable, and fails closed: a pre-0.4 captured invocation returns `Err` with no envelope, 0.4.0 republishes, and the typed entry publishes Sensitive. The composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item owns it, with C1 as its test case, and closes it before T3 closes.
- **Other disclosures, in S11-G's CHANGE_RECORD:**
  - the desktop reader coverage gap (no committed fixture carries a non-qualified receipt entry);
  - T6a and the curved pressure families, unreachable on fresh solves, with unit tests kept;
  - the growth of the receipt's publication reservation;
  - R-b′ silent below a moment scale of 2^-988.

## Not run for this merge

The native macOS witnesses do not apply, because S11-G changes no native path. I5 ran the src-tauri suite (114/114) on the S11-G tree; it runs neither in the sweep nor in CI.
