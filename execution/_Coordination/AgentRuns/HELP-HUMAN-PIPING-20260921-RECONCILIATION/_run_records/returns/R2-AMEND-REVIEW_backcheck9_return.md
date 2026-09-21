VERDICT: FINDINGS

Backcheck 9 covers `88ad686a0..d6ef984d2`. B8-1 and B8-3 to B8-6 are closed. B8-2 is closed for the recount, but `--notes-gap` can still be gamed through other Notes markers (B9-1).

## (a) Closure

- **B8-1 — closed.**
  - F3's cutoff is now `7bee9ae41` ("Initial migrated Chirality repository", 2026-05-18).
  - Keyed CS rows keep their class, and revision pins, review states and metadata stay `STALE_REVIEW_OR_EVIDENCE`. That keeps C6(c), CP-02/03/05 and CS-01/04/05/06 intact.
  - The worker brief mirrors this.
- **B8-2 — recount confirmed, still gameable (see B9-1 and B9-2).**
  - On wave 1, 59 of 715 `ALIGNED` rows are flagged, and 5 of 5 firm errors are caught, including `DEL-07-02:SOW#CLM-018.r06`.
  - No wave 1 row has gap wording that the new stripping hides.
  - `GAP_WORDING_CHECKED:` now needs at least 25 characters.
- **B8-3 — closed.** `DEL-07-02:SOW#CLM-026` now resolves to `UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · NONE · IP_DATA;RECORD`, with `AuthorityNeeded REVIEW`, citing F8. This matches RQ-008.
- **B8-4 — closed.** F2's generalisation is labelled "AGENT (not ruled; precedence clause applies)".
- **B8-5 — closed.** `RESOLUTIONS.csv` now has 38 rows, and the `#END` count is 38.
  - Two rows are the related R-005 rows (`CONTESTED`).
  - Four are out-of-sample verifier observations (`OBSERVED`).
  - All six keys exist in the sealed ledgers, and their sealed values match what the reports describe.
  - Batch mode with `--resolutions` still passes.
- **B8-6 — closed.** Both briefs and `WAVE_PLAN.md` now say "2–4". A verifier-triggered rerun manager waits for a free slot. The queue count is corrected to 58.

## (b) New findings

**B9-1. ACTIONABLE — two other Notes markers switch off the gap check for everything after them.**
- Where: `tools/validate_ledger_v2.py`, `ESCAPES = …(GAP_WORDING_CHECKED|CANONICAL_DEPARTURE|PRETYPE_OVERRIDE):[^|]*`.
- Evidence:
  - The pattern deletes each marker and everything after it up to a `|`. Notes rarely contain `|`, so in practice that is the rest of the Notes.
  - Using the real validator with `--notes-gap` on a conforming DEL-00-05 ledger:
    - `Rule-check negative test not located.` → FAIL (correct);
    - `CANONICAL_DEPARTURE: x. Rule-check negative test not located.` → PASS;
    - `PRETYPE_OVERRIDE: x. Rule-check negative test not located.` → PASS.
  - Nothing checks that these two markers appear only on canonical-assigned or pre-typed rows. Verifiers sample only `GAP_WORDING_CHECKED:` and `OPEN_ACTION:` rows at 100%, so an escape made this way is also invisible to verification.
  - A legitimate departure note followed by a real gap sentence is hidden the same way.
- Smallest fix: strip only the `GAP_WORDING_CHECKED:` clause itself, not the other two markers or the text after them. Optionally, fail `CANONICAL_DEPARTURE:` on rows that have no CS assignment and no CP ID, and `PRETYPE_OVERRIDE:` on rows that are not pre-typed.

**B9-2. MINOR — path stripping hides some real gap wording.**
- Where: `PATH_TOKEN`, specifically the `\S*[/\\]\S*` and `` `[^`]*` `` branches.
- Evidence: in direct probes of the pattern, each of these was missed:
  - `Negative test not located/verified …`;
  - `missing/invalid inputs …`;
  - `w/o rule-check assertion`;
  - a backticked phrase such as `` `not located` ``.
- On "and/or": `partial and/or missing` is still caught, but only because "partial" is not joined to the slash. A gap word attached to any slash is removed.
- No wave 1 row is affected (0 hidden).
- Smallest fix: strip only path-shaped tokens, meaning those starting with a known root or project prefix (`projects/`, `core/`, `apps/`, `docs/`, `tools/` and so on) or ending in a file extension. Strip backticked spans only when they look like identifiers or paths.

The working tree has an uncommitted `RUN_STATE.jsonl` change that is not part of this delta.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- All 13 wave 1 ledgers pass single mode (forward, reverse and inventory). Batch with `--resolutions`: PASS, 0 findings.
- All 50 bound hashes at `d6ef984d2` match their files.
- `python3 -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py`: **11 passed**.
- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied`.
- `validate_piping_loop_receipts.py --repo-root .`: VALID (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 4, WARN 112).

Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc9/`.

END-OF-RETURN