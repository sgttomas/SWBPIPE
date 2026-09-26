# Checkpoint-4 review dispositions and repairs

- **Author:** the session-3 T1 WORKING_ITEMS manager.
- **Review:** `REVIEW_CHECKPOINT_4/RETURN.md`, an independent review of `14a74b793`. Verdict: FINDINGS, with one should-fix (SF-1R) and four notes (N-1 to N-4). Nothing is blocking.
- **Rulings:** ROOT's rulings, received by message on 2026-09-26, are quoted in substance below.
- **Scope:** everything applies to 0.4.0 invocations only. Pre-0.4 bytes and meanings are unchanged.
- **Records:** earlier records are not rewritten. This record corrects them. It also corrects the pertinent wording in `CHECKPOINT_4.md` §1 and `CP4_WIRE_ADDENDUM.md` §1.1–1.2, and ROOT's selection text in `REVIEW_CHECKPOINT_3/ROOT_DISPOSITION.md`.

## SF-1R — the replay reservation is a screen, not a guarantee

**ROOT ruling (option b).** The replay reservation is a screen. It turns most budget cliffs into a direct "unavailable" at selection. The guarantee is the fallback: no 0.4.0 invocation loses its ordinary results.

ROOT's earlier wording in `ROOT_DISPOSITION.md` ("so a case is selected only when replay can complete") overstated the guarantee. This ruling supersedes it. `ROOT_DISPOSITION.md` is not rewritten.

**Corrections:**

- **`CHECKPOINT_4.md` §1, "Replay is therefore bounded by the live charge."** This is not true in general. On the committed `fields` witness, replay costs 3,196,145 against a live charge of 3,196,032, which is 113 more (review). The four `eigen_motion` measurements were one witness.
- **`CP4_WIRE_ADDENDUM.md` §1.1.**
  - The statement that replay's measured charge is at most the live charge is corrected in the same way.
  - "The reservation covers the replay only. It does not guarantee the later finalization stages" is also corrected. Derived recipes and the finalization reservation are charged *before* replay, in the same ledger (432k–549k in the review's measurements). So a case can pass the screen (`c ≤ L − c`) and still fail captured replay.
  - Example: the committed witness plus a 1e-6 N·m tip RY moment. Here c = 3,992,928 at the public 8M limit. The §1.2 fallback then publishes the ordinary route, with the attempt declined as "invocation join withheld … captured source replay …".
- **The two code comments** (`lib.rs` in `solve_load_case`, and `SelectedSourceRecovery::reserve_captured_replay`) now describe a screen.

**Code: comments only.** The screen arithmetic is unchanged.

**New test (public limit, both modes):** `a_join_that_passes_the_replay_screen_but_cannot_finalize_falls_back_at_the_public_limit`. It uses the RY input and asserts:
- the ordinary publication, bit-identical to the typed route;
- the withheld cause naming captured replay;
- no screen refusal.

## N-1 — SF-1 mutant record ran on pre-rustfmt bytes

The committed `_run_records/session3/cp4_sf1_mutations.{py,log}` ran on bytes from before the rustfmt edits. That was the working tree before commit `6235f6b43`. The anchors of M3, M4 and M5 occur 0 times in the committed `lib.rs`, so that log does not evidence the committed bytes. This was not disclosed. It is the same class as the CP3 N-4 finding.

- The reviewer re-anchored all six mutants to `14a74b793` and reran them. All six are killed (`REVIEW_CHECKPOINT_4/_run_records/mutations.log`).
- After the repairs below, `_run_records/session3/cp4r_sf1_mutations.{py,log}` supersedes the earlier run. It is anchored to the committed bytes, and every anchor must occur exactly once. Its first run of M1 used an invalid anchor that did not compile. The corrected M1 is `cp4r_sf1_mutations_M1_rerun.log`.
- It also adds mutants for the two repairs below. All eight are killed.

## N-2 — resource bound of the fallback

**ROOT's preference.** If the fallback can charge the same ledger at low risk, prefer that.

**Done.** The republication now continues the first run's ledger (`SourceRecoveryBudget::withholding_load_state_join` copies every charge and count). Previously it used a fresh ledger with the full limits.

**The declared resource bound**, in the terms of `CORRECTNESS_DESIGN/COMPOSITE_ENGINE/RESOURCE_POLICY.md`: for a captured 0.4.0 invocation, the per-invocation limit of 64,000,000 work units bounds *all* retained-source work, *including* the fallback republication. Every successful, failed, reserved and publication charge of the first run stays charged. Nothing is refunded or reset.

Consequences:
- The per-case limit (8,000,000) still applies to each attempt, as `min(per-case, remaining invocation)`.
- When the first run has exhausted the invocation limit, the republication's attempts are refused for budget. Each such case publishes its ordinary response with `retained_source_attempt=unavailable`.
- The resource policy's figures, its protected 1e-9 criterion and the pre-0.4 methods are unchanged.

**Not done.** The ordinary solve itself runs twice when the fallback runs: once in each run. That work is outside the retained-source ledger, as every ordinary solve is. Reusing the first run's ordinary rows was not attempted, because the first run publishes selected projections for a selected case, not ordinary rows. So reuse would need a second result path, and the risk is not low.

**Tests:**
- `a_selected_case_whose_own_finalization_fails_after_the_reservation_falls_back` now asserts `attempts == 2`, `charged == failed_charged` and `charged ≤ invocation_limit`.
- New: `a_fallback_after_invocation_limit_exhaustion_stays_within_the_invocation_limit`. Ten selectable copies exhaust 64M in the first run. The test asserts the ordinary publication for all ten cases and total `charged ≤ 64,000,000`.
- Mutant `N2-M8` resets the fallback ledger, and it is killed.

## N-3 — the republication reports a case's own refusal

In the republication, the screen is now checked before the withheld branch. A case that fails its own screen reports "captured replay reservation", not "invocation join withheld". `CP4_WIRE_ADDENDUM.md` §1.2, "any successful attempt is declined with stage 'invocation join withheld'", now reads: any attempt that passes its own screen is declined so.

- **New test (both modes):** `a_republication_reports_a_case_s_own_replay_screen_refusal`, using the reviewer's mixed-reservation input.
- **Mutant:** `N3-M7` is killed.

## N-4 — record inaccuracies

- **`CHECKPOINT_4.md` §3.** The path edit in `ad6da6880` also changed `TASK_BRIEFS/CP4_REVIEW.md`, which had not yet been dispatched. The substitution also left awkward text there: "the venv `the session DEC-025 venv (location given in the spawn request)`". The reviewer received the concrete locations in its spawn request.
- **`_run_records/session3/cp4_integrated_python_named.log`** was produced by this command, run from WORKING_ROOT with the session venv:

  ```
  PYTHONDONTWRITEBYTECODE=1 python -m pytest -q -p no:cacheprovider \
    tests/test_load_reference_readers.py tests/test_load_reference_schema.py \
    tests/test_stress_neutral_export_package.py tests/test_stress_neutral_physics_source.py \
    tests/test_stress_neutral_precision.py
  ```

  This is the same five-file set that the reviewer reproduced (957 passed, 1 skipped).

## Checks after the repairs

See `CHECKPOINT_4.md` §5 for the counts and logs.

## Not changed

- The screen arithmetic.
- The published identities.
- `load-reference-source-1`, which is still reserved and inactive.
- Every pre-0.4 path.
- The inherited physics-source-1 composite `Err` (T3).
- Any ordinary-route summary, maxima or reaction publication (reserved for T0R).
