VERDICT: PASS

Backcheck 12 covers `cdcd14125..f250882c1`. B11-1 to B11-4 are all closed, and I found no new blocking or actionable defect. One residual is minor, noted below.

## Closure

**B11-1 — closed.**
- `CP04_VARIANTS` maps only the three defined tier and baseline pairs. Any other pair now lands in the default group.
- I changed one CP-04 row that has no recorded resolution (`DEL-01-02:SOW`) to `INVARIANT`/`IP_DATA` in a scratch copy of the 31 W2 ledgers. Batch mode with `--resolutions` flags it: `same pattern CP-04/STALE_REVI… 'AuthorityTier': 'INVARIANT' … differs from the majority` → FAIL.
- My first attempt used `DEL-00-03:AB`, which is a verifier-sourced RESOLVED_PAIR row, so its recorded values replaced the edit and the probe passed. That is correct behaviour: a recorded resolution always supplies the values compared.

**B11-2 — closed.**
- The exemption now requires a `*_VERIFICATION.md` source and applies only in body and pattern groups; CS variant groups still compare every row.
- I changed the source of the `DEL-04-01:SOW#CLM-021` RESOLVED_PAIR row to "Agent 0 judgment". The CP-03 conflict then reappears: FAIL, 1 finding.

**B11-3 — closed.**
- The 9 W1 and 23 W2 SEMANTIC_READY rows are now class `AGENT_READING`. Each keeps its sealed cause and cites cluster SR-1.
- No `RESOLVED_BY_RULE` remains in either resolutions file or in the gate assessment.
- The assessment calls this "an Agent 0 reading of the F3 exception, not a ruling". It notes that the reading overrides 11 sealed rows and sends it to the owner with the canonical table.
- Row counts: W1 has 47 (38 + 9) and W2 has 122.

**B11-4 — closed.**
- Evidence columns that fail:
  - `a; b` (space after the separator);
  - `NONE_FOUND ` (trailing space);
  - free text;
  - a path followed by text.
- Evidence columns that pass: deliverable paths containing spaces, with or without `#Lnn`.
- `ContextRefs` is unaffected: `PR #803; path` still passes.

## Residual (MINOR, no action needed)

- CP-04 variants are keyed on tier and baseline class only. A row that wrongly uses another defined variant cannot be caught mechanically. An example is `PROJECT_BASELINE`/`NONE` on plain rename residue that names no `.opsproj` or store identifier. Verifier sampling covers that case.

## Validation

- **All 44 ledgers pass single mode.** The 13 W1 ledgers pass without flags. The 31 W2 ledgers pass with `--notes-gap`. Each run used `--reverse` and `--inventory`.
- **Batch mode per wave with `--resolutions` passes:**

| Wave | Ledgers | Result |
|---|---|---|
| W1 | 13 | PASS, 0 findings |
| W2 | 31 | PASS, 0 findings |

- **Bound hashes.** All 52 entries in the latest `BOUND_INPUTS` at `f250882c1` match their files.
- **Working tree.** It holds work outside this delta, which I did not review: an uncommitted `RUN_STATE.jsonl` change and untracked W3 manager launch messages.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `python3 -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py`: **11 passed**.
- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied`.
- `validate_piping_loop_receipts.py --repo-root .`: VALID (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 4, WARN 112).

Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc12/`.

END-OF-RETURN