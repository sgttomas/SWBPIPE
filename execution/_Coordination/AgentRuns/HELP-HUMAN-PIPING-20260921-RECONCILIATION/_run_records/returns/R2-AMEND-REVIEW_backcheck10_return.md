VERDICT: PASS

Backcheck 10 covers `d6ef984d2..8f7a9762f` (HEAD `8f7a9762f53b75da407a22e7c9a47a1f27e6a0a7`): B9-1 and B9-2 are closed, and I found no new blocking or actionable defect. There are three minor residuals, none of which affects wave 1.

## Closure

**B9-1 — closed.**
- The escape now removes only a trailing `GAP_WORDING_CHECKED:` clause.
- I reran my synthetic DEL-00-05 ledger with `--notes-gap`:
  - `CANONICAL_DEPARTURE: x. Rule-check negative test not located.` now **FAIL**s (it passed before);
  - `PRETYPE_OVERRIDE: x. …` now **FAIL**s;
  - the plain gap sentence still **FAIL**s;
  - `…not located. GAP_WORDING_CHECKED: <25+ chars>` passes, as intended.
- F4 and the worker brief now say the escape must be the last clause, and that other markers do not exempt later text.

**B9-2 — closed.** Direct probes of the new pattern now catch the wording that was hidden before:

| Probe | Caught as |
|---|---|
| `not located/verified` | "not located" |
| `missing/invalid` | "missing" |
| `` `not located` `` (backticked phrase) | "not located" |
| `partial and/or missing` | "partial" |
| `docs/missing and partial things` | "partial" (only the path is stripped) |

Path-shaped tokens are still stripped: root-prefixed paths such as `…/missing-data/…` and `core/solver/gap_element.rs`, `*.py::symbol`, and `*.test.tsx`.

**Wave 1 recount:** 59 of 715 `ALIGNED` rows are flagged and 5 of 5 firm rows are caught, matching your count. No wave 1 row has gap wording that the stripping hides.

## Residuals (MINOR, no action needed)

- **An escape placed first still hides later text.** A `GAP_WORDING_CHECKED:` clause written first in Notes hides whatever follows it; the last-clause rule is not enforced mechanically. Verifiers sample every row carrying the marker at 100%, so these rows stay visible.
- **Bare identifiers can be false positives.** An identifier written unquoted in prose, such as `BEND_RULE_INPUT_MISSING`, matches "missing". Backticking it avoids this.
- **Unlisted shorthand is not caught.** "w/o" is not in the wording list; this is a vocabulary gap, not a stripping problem.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- All 13 wave 1 ledgers pass single mode (forward, reverse and inventory). Batch with `--resolutions`: PASS, 0 findings.
- All 50 bound hashes at `8f7a9762f` match their files.
- `python3 -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py`: **11 passed**.
- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied`.
- `validate_piping_loop_receipts.py --repo-root .`: VALID (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 4, WARN 112).

The working tree also holds work outside this delta, which I did not review: an uncommitted `RUN_STATE.jsonl` change and untracked W2 manager launch messages in `_run_records/launches/`. Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc10/`.

END-OF-RETURN