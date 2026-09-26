# TASK brief — T1_WP7_FINAL_REVIEW (fresh-context review of the complete frozen T1 diff)

Read `_T1_COMMON.md` first, including its **Wave 2** section. Return folder: `LSI/T1_WP7_FINAL_REVIEW/`.

You are a **fresh-context, non-author reviewer** of the whole T1 candidate. You wrote none of the reviewed bytes, and you delegate nothing. Earlier reviews exist; read them for context, but do not rely on them. This review is the independent review the merge gate requires for the actual candidate revision.

## Candidate

- **Diff.** The frozen candidate commit named in the spawn request, against `origin/main` at the merge base the spawn request names: `git diff <merge-base>..<candidate> -- projects/chirality-piping`.
- **Where to work.** Use a scratch `git archive` of the candidate, never the live worktree.

## Basis

- `LSI/T1_PLAN.md`, `CHECKPOINT_4.md`–`CHECKPOINT_7.md`, and `T1_WAVE1_RULINGS.md` §1–§16 with the ROOT addenda.
- The wire: `CP2_WIRE.md`, its addenda, `CP3_WIRE_ADDENDUM.md`, `CP4_WIRE_ADDENDUM.md` as corrected by `CP4_REVIEW_DISPOSITION.md`, and `T1_WIRE_ADDENDUM.md`.
- The T0R design: `DEFAULT_ROUTE_DESIGN/DESIGN.md` §10 and `S6_RECORD.md` §6.

## Review

1. **Product semantics.** The 0.4.0 route (`core/product_physics`):
   - resolved load/reference states;
   - support motion through the partitioned solve;
   - thermal and fit reference strain entering once;
   - material selection;
   - the joined route and its receipt;
   - the SF-1 fallback, which ROOT asked to have in scope: a joined attempt that fails its budget publishes the ordinary route, never a blocked result.
   Look for silent-wrong results: a published number that is wrong without a diagnostic. Any silent-wrong exposure is BLOCKING.
2. **Identity and standing.**
   - Every 0.4.0 envelope, blocked or solved, carries a T1 identity. No pre-0.4 envelope carries one.
   - The fresh set is identical in Rust, Python and TS.
   - The standing order is validation, then T0R's standing reason, then the T1 joined early return.
   - Joined results are never numerically eligible (§7).
3. **Readers in three languages.** Are they faithful peers? Is parity real? Do the refusals cover the tamper classes the Rust and Python suites name?
4. **Schemas and carriers.** Branch order and additivity. Do pre-existing documents still match only their old branch?
5. **Authoring.** The typed operations, the desktop inputs and the blank 0.4.0 model:
   - no defaults, library values or code values;
   - pre-0.4 gating;
   - Undo/Redo is byte-exact;
   - authored fields that cannot yet be edited pass through as authored.
6. **Persistence and migration**, native and browser.
7. **Output refusals (§12, §14).** No desktop surface lets load/reference-state result data out. The Python validation path (harness and stress-neutral packager) is untouched.
8. **Tests.**
   - Do the tests prove what they claim?
   - Are there skips, `.only`, raised timeouts, weakened criteria, or references, goldens or oracles edited to pass?
   - Include the manager's own repairs: the F1–F3 test changes (`5cf56e3de`) and the §14 panel reclassification (`64711fd94`). No non-author has reviewed these at the candidate yet.
9. **Records.**
   - No absolute machine paths.
   - The checkpoints' claims match the recorded logs.
   - The open-work routing is complete: T3, T4, T5, T6, UI-SUCCESSOR.

## Checks you may run (scratch copy only)

- Targeted cargo tests. Use the shared target named in the spawn request, with `CARGO_INCREMENTAL=0`. Check `df` first and keep more than 8 GB free.
- pytest and vitest suites.
- Your own probes and mutants.

The manager's quiet-host DEC-025 sweep, src-tauri run and VP-STATIC rerun on the candidate are recorded in `CHECKPOINT_7.md`. Read them; do not repeat full sweeps.

## Return

`LSI/T1_WP7_FINAL_REVIEW/RETURN.md`:

- the verdict: CLEAR, FINDINGS or BLOCKING;
- a findings table: severity, location, concrete failure scenario, suggested repair;
- per item 1–9, what you verified and how;
- limits.

Then `SendMessage` the manager.
