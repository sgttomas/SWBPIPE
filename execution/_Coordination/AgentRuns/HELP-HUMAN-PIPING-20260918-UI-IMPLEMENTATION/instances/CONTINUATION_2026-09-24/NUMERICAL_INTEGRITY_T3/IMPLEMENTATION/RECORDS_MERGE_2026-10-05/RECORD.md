# T3 records merged to main — PR #1084 (ROOT, 2026-10-05 UTC)

**PR [#1084](https://github.com/sgttomas/chirality/pull/1084)** was squash-merged at `2026-10-05T13:34:37Z` as `f506f3e2dedf136361fbfac285374201b490e023`, a single parent on main `09574ed9a5`. Its execution records and portability policy equal the gated head R3 `5758e1c3df70a51b80a2a94d0118c5c2fad7456f` byte for byte.

**Main moved after the gates.** Between the gated base `e916ad1789` and the merge, main took #1086 and #1087 (`09574ed9a5`), which touch only `projects/chirality-app-v4`.
- **No overlap:** neither touches the PR's paths or any piping file, so the piping DEC-025 and the PR checks carry over.
- **GEN-8,** which scans the whole repository, passed on a local, unpushed combination of R3 with that main (`_run_records/gen8_combined.txt`).
- **The owner** said "merge what's ready".

**Neither earlier PR head is an ancestor of main:** `dfa5e2dc44` (with the unredacted listings) and `59b72619fe`.

**The merge:**
- **The command:** `gh pr merge --squash --match-head-commit` R3, with an explicit subject and body (RV96 N-8). Immediately before, ROOT confirmed main was still `09574ed9a5`, the head whose changes were checked above. Auto-merge stayed off.
- **The merge facts** are in `_run_records/`.

## What reached main

- **`projects/chirality-piping/execution/`,** byte-identical to the T3 integration branch at `cc44bce7f3`. That covers:
  - the rulings;
  - the TASK and review records;
  - the F2a D1 merge record;
  - I61's U8 and F2a-breadth plan;
  - the prepared dispatch briefs (I68–I74, RV97–RV99);
  - the host-cleanup record;
  - the handoff `HANDOFF_2026-10-05_TO_NEXT_ROOT.md` and its prompt.
- **`projects/chirality-piping/validation/portability_policy.json`:** 255 owner-approved, hash-bound entries, appended. 210 historical run records are EVIDENCE; 45 as-issued briefs, plans and the 2026-10-03 handoff are CONTROL. With them, GEN-8 passes without editing sealed records.
- **13 redacted files,** by owner decision: 12 whole-host process listings and one diff log quoting them. `IMPLEMENTATION/HANDOFF_2026-10-05/_run_records/REDACTIONS.json` gives their original and redacted hashes.
- **The squash keeps the unredacted originals out of main's history.** They remain in the integration branch's and the PR branch's history on origin.

## Gates at R3

| Gate | Result | Evidence |
|---|---|---|
| **Independent review (RV96)** | FAIL at the first head, with B-1 (GEN-8) and S-1 (process listings). FAIL at R2, with B-2 (the merge method) and S-2 (one more redaction). Each was remedied and ruled. **PASS at R3** (0/0/1; N-8, the squash message, is applied) | `R/REVIEW_RV96/records_01/` (REVIEW, ADDENDUM_01, ADDENDUM_02) |
| **GEN-8** | 1 passed, 10 deselected, on R3. Also on the integration branch at `cc44bce7f3` | `_run_records/gen8_R3.txt` |
| **Hosted CI on R3** | governance-harness 37258661309 (1,156 passed), Harness Pre-merge 37258661233, pec-tests 37258661340 and Piping Desktop E2E 37258661311 all succeeded | `_run_records/CI_RUNS.jsonl` |
| **The full-SHA dispatch** | Run 37258656686 on R3 (`target_base` = main): **success** (every job, the Numerical cargo suite and Desktop E2E included) | `_run_records/CI_RUNS.jsonl` |
| **DEC-025 on R3 (Mac)** | 03:18:01–03:50:59Z, with the recorded driver and a fresh target. All 40 manifests are identical to F′'s run, which has the same piping source. Against main's Mac baseline, the differences are the added tests only, and the failing set is main's (PP `t13`, runner's two `load_reference` tests). pytest 3,540 passed and 32 skipped; vitest 3,552; both builds exit 0 | `dec025/` |

**Superseded runs, not counted:** DEC-025 on R2, stopped at 28 of 40 manifests (in `<WT>/scratch/u9_dec025/R2_stopped_superseded/`), and the first head's and R2's CI runs.

## Rulings

On the integration branch:
- "Handoff prepared; main absorbed; host cleanup; a records-only PR next";
- "Records PR #1084: GEN-8 flags historical records; the owner approves their portability registration";
- "Records PR #1084: redactions, GEN-8 repair and re-cut";
- "RV96 addendum 01 at R2: the merge method and one more redaction";
- the merge ruling that follows this record.

## Lessons for later records PRs

1. **Run GEN-8 on the candidate before opening the PR.** Write living documents (rulings, current state, work graph, handoffs, briefs) without machine-absolute paths, using `<repo>/…` or `WT/…`. Never hash-bind a living document.
2. **Screen run records for whole-host data before publishing,** such as process listings with app names or session IDs. Capture only the processes the check needs.
3. **If a PR's history contains material the owner decided not to publish,** squash, or re-cut a single commit. A merge commit would carry it into main's history.
