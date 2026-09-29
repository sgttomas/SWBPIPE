# KF1 change record: K4's extreme trackers in bounded memory, every result unchanged

This is the draft PR record for slice KF1 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I18, a TASK. The details are in `RETURN.md`.

> **Shipped values: T = 512 and G = 4,096** (addendum 1). The sections before addendum 1 record checkpoints A and D at T = 64 and G = 512. RV20's review (PASS) and its fixes are addendum 2.

- **Branch:** `codex/piping-kf1-20260929`, from main `8cca91701` (PR #1055).
  - Its piping tree equals `ab02ee3a6`'s, with K4 merged; the piping diff between them is empty.
- **Commits (made by ROOT):**
  - `d0566126e`: checkpoint 0, the plan;
  - the next commit: A and D, meaning the code, the tests, the site-table row and these records.
- **Size (against main `8cca91701`):**
  - `FK/src/structural/retained/adaptive.rs`: +347 −62;
  - `FK/tests/retained_k4/adaptive_tests.rs`: +3 −1;
  - `FK/tests/retained_k4/kf1_tracker_tests.rs`: new, 964 lines;
  - `FK/tests/s11_site_table.rs`: +2;
  - records under `T3/IMPLEMENTATION/KF1/`.
- **Basis:**
  - the I18 brief (`TASK_BRIEFS/I18_KF1_IMPLEMENTATION.md`);
  - the checkpoint-0 plan (`2129c337…`);
  - `ROOT_RULINGS_V1.md`'s sections "K6b: A1 accepted; K4's stop-rule memory finding", "KF1: spawn", "KF1: rulings on I18's checkpoint-0 plan" and "KF1: checkpoint A accepted; the S11 site-table row authorized".
- **Platform:** Mac (`aarch64-apple-darwin`, 128 GiB), rustc 1.97.1.

## What changes

- **`BoundedExtremeTracker` replaces K4's `ExtremeTracker`** at all seven sites:
  - the stop rule's (a), (b) and (d);
  - the pivot margin;
  - the residual gate;
  - the fallback.
- **How it works:**
  - A row counts under K4's window rule, which is unchanged.
  - At most T = 64 rows are held unevaluated. At T they are evaluated exactly (a collapse), and they are kept only as (key, outcome) entries that no entry nearer the best key matches or beats.
  - A refusal met at a collapse is recorded, not returned.
- **`TrackerSet`:** the stop rule's trackers share one set, which caps their unevaluated rows together at G = 512, counted by allocated capacity. So a model with many small bodies is bounded too.
- **`residual_rows`** gains a `ctx16` parameter.
- **The old tracker** is removed.
- **The test hook:** a thread-local override of T exists only under `#[cfg(test)]`.
- **The S11 site table** gains one declared, additive row, authorized by ROOT: `adaptive.rs` `offer`, 2 integer accumulations. No other row changes.
- **Unchanged:**
  - every file outside FK's `retained/` module, its tests and the site table;
  - GEN and its vectors;
  - `retained`'s visibility: it is private, and there is no `retained_api` on this base.

## Results

- **Equality.**
  - `finish` is bit-identical to K4's on every stream, refusals included. The invariant proof is in RETURN §3 and does not use the approximation's accuracy.
  - The differential test against a verbatim copy of K4's tracker runs 828 streams at T = 1, 2, 3, 5 and 64, in both directions. The memory bound is asserted after every row.
- **Nothing else changes.**
  - The model-level differential covers 131 controls, the six frames at 100 members and 4 combinations, each at T = ∞, 1 and 64.
  - Selections, publications, states, evidence, and attempt roles and reasons are identical under unlimited budgets.
  - Only the width-16 work moves, by exactly 17,506 LME per extra evaluation.
- **The bound:**
  - the stop rule holds at most 2.34 MB of unevaluated rows per call, against about 3.2 GB today at 10,000 members;
  - the pivot margin and each residual-gate evaluation hold 275 KB each;
  - the fallback holds 1.10 MB;
  - tables are 40 B per entry, at most 8,193 entries each; in practice one value entry, which is a practical bound only.
- **Work.**
  - The golden pins do not move.
  - At T = 64, only the six RF-LARGE frames at 100 members move, and only in the stop rule: +128 to +768 evaluations. That is +21% to +159% of the stop rule, or +2.5% to +16.2% of the case's work.
  - ROOT's A ruling states this as "+15% of the stop rule's work (about 2% of the call)". RETURN §5 gives the corrected figures.
  - The worst case is +17,506 LME per offered row, whatever T is.
  - A new pin covers two of the frames, and its T = ∞ figures were measured on main's own code.
- **The budget boundary** is accepted (ROOT's decision 3). A limit between K4's work and KF1's now ends on budget.
- **Suites:**
  - FK's full suite: 401 passed, 0 failed;
  - the S11 site table: 3 of 3;
  - `gen_k4_vectors.py --check`: 23 of 23;
  - rustfmt clean, and no warnings.
- **Mutations:** NONE passes, and 10 of 10 are killed, among them the brief's wrong-direction collapse and dropped collapsed entry.

## Limits

- **Mac only.** No timing claims.
- **The bounded-gate stage** never moved at the model level: no control's fallback tracker held two rows within the window. That site is covered by result equality.
- **The approximation lemma** (a nearer key never has a less extreme directed value) is argued, not machine-checked. It supports only the practical table bound, not the proof.

## Gates (ROOT runs the PR)

- An independent reviewer checks the equality argument, the differential test's coverage and the work pins.
- Hosted CI, with the full-SHA dispatch.
- DEC-025 with a fresh sweep target.
- GEN-8 before the records commit that goes to main.
- T9 and the both-entry gate are not run: the change is kernel only (brief, "ROOT rulings"). The reviewer re-runs the scan.

## Downstream notices

- **K6b (I16):**
  - recompute E_max from this code, for every tracker, using RETURN §4;
  - include the fallback's per-state row list, about n_f × 4.3 KB, which is not a tracker;
  - re-run W1-T3 for the stop rule's changed work before W1-T4.
- **V-K (I17):** the fault sites are clear of KF1's tracker edits, per ROOT's V-K A1 ruling.

## Addendum 1: T = 512

- **Ruling:** ROOT's "KF1: D received; T reopened and set to 512".
  - ROOT's A ruling had misread the T = 64 work as "+15% of the stop rule's work". It is +2.5% to +16.2% of the case's work.
  - On those figures ROOT set T = 512, with G = 8T = 4096. The approximation lemma stays out of the correctness basis.
- **Change against `d267a755b`:**
  - `FK/src/structural/retained/adaptive.rs` (+4 −2): `TRACKER_ROWS` = 512, and G follows as 8T;
  - `FK/tests/retained_k4/kf1_tracker_tests.rs` (+27 −17):
    - the stream differential adds T = 512;
    - the model-level differential runs at T = ∞, 1 and 512;
    - the collapse pin stays at T = 64 through the `#[cfg(test)]` hook, and now also asserts that T = 512 leaves both frames at K4's figures.
- **Work at T = 512:** 0 extra evaluations on the six RF-LARGE frames at 100 members and on HH-FOOL-m100 (at T = 64 the frames added 128 to 768). No control moves, and the golden pins do not move.
- **The per-call bound:**
  - stop rule: ≤ 4,096 unevaluated rows (17.6 MB) after every offer, and ≤ 18.7 MB during one;
  - pivot margin: 2.2 MB;
  - residual gate: 2.2 MB per evaluation;
  - fallback: 8.8 MB;
  - tables are unchanged.
- **Re-runs:**
  - KF1's tests pass 7 of 7, with the differential at 882 runs;
  - FK's full suite: 401 passed, 0 failed, with no warnings;
  - NONE passes, and KF1-M7 and KF1-M8 are killed, from clean copies of `d267a755b` plus the change.
- **Records:** RETURN addendum 1, `_run_records/t512/` and SHA256SUMS, refreshed.

## Addendum 2: RV20's review (PASS) and its fixes

- **Review:** RV20 (`T3/REVIEW/KF1_REVIEW.md`) reviewed head `1854911d1`: PASS, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs. ROOT's "KF1: rulings on RV20's review" asks for four fixes before merge.
- **Change:** `FK/tests/retained_k4/kf1_tracker_tests.rs` +48. No production code changes.
  - **RV20-1:** the shared-cap test asserts, per round, that the set's 16-limb work is at least K4's. This kills RV20-M4, the set's collapse charged to a throwaway context.
  - **N1:** RV20's order test is added as `kf1_the_stop_rules_trackers_finish_in_k4s_map_order`. It kills RV20-M5 (`RuleTest` reordered). RV20-M1 and M2 are recorded as equivalent: every reachable refusal is `Span`.
- **Records:**
  - **N3:** RETURN addendum 2 restates the memory figures.
    - The transient peak is G + T = 4,608 rows (19.8 MB) in the stop rule, because `Vec` growth briefly holds both buffers.
    - A standalone tracker peaks at 1.5T = 768 rows (3.3 MB), and a solve attempt at 2,816 rows (12.1 MB).
    - The tables are unconditionally at most one 40 B entry per row kept in a window. The one-value-entry figure is practical only.
  - **N5:** RETURN §0 and this record's head point to addendum 1: T = 512, G = 4,096.
  - N2 and N4 are recorded as ruled.
- **Re-runs:**
  - KF1's tests pass 8 of 8, with 0 warnings and rustfmt clean;
  - FK's full suite was not re-run, since only the KF1 test file changed; addendum 1's run passed 401;
  - NONE passes, and RV20-M4 and RV20-M5 are killed, from clean copies of `1854911d1` plus the test change.
- **Records:** `_run_records/rv20/`, and SHA256SUMS is refreshed.
