VERDICT: FINDINGS

Backcheck 11 covers commit `cdcd14125`. The gate wave 2 verdict (MET) is right, the rates are honest, and the DEL-03-07 rerun custody is sound. One actionable finding remains: the new CP-04 grouping in batch mode accepts any tier and baseline class as its own variant.

## (a) Validator changes

- **All 44 ledgers pass single mode.** The 13 W1 ledgers pass without flags; the 31 W2 ledgers also pass `--notes-gap`. Each run used `--reverse` and `--inventory`.
- **Batch mode per wave:**

| Wave | With `--resolutions` | Without |
|---|---|---|
| W1 (13 ledgers) | PASS, 0 findings | FAIL, 1 finding |
| W2 (31 ledgers) | PASS, 0 findings | FAIL, 2 findings |

- **The previous validator on W2** (single mode) gives 0 findings, so none of the committed ledgers relied on the path-with-spaces change. Its W2 batch with resolutions gives 7 findings: 5 CP-04 `.opsproj` rows and 2 CP-03 rows. The new CP-04 split and the `RESOLVED_PAIR` exemption clear exactly those.
- **Spaces in evidence tokens:**
  - These pass: a real deliverable path with spaces, and the same path with `#L10`.
  - These still fail: `not rerun`, `NONE_FOUND (not rerun)`, and `projects/…/PRD.md see section 5`.
  - `a; b` (a space after the separator) now passes. Part D says "no spaces". Minor.

## (b) Resolutions

- **Counts.** `WAVES/W2/RESOLUTIONS.csv` has 122 rows: 36 CONTESTED, 34 FIELD, 23 RESOLVED_BY_RULE, 14 OBSERVED, 8 RESOLVED_PAIR, 6 FIRM, 1 WEAK.
- **Keys.** Every key exists in the current ledgers.
- **Spot-checks against the reports all match:**
  - FIRM: PKG-00 REQ-06-03; PKG-02's four rows (the three counted plus the worker-flagged CLM-020); PKG-03 D2.
  - PKG-04: W-1 is recorded as RESOLVED_BY_RULE, the W-2 pair as RESOLVED_PAIR, W-3 as CONTESTED, and the outside-sample items as FIELD or OBSERVED.
  - PKG-01 W3 is recorded.
- **Cause split.** It is left open as SR-1: 23 rows cite SR-1.
  - The SEMANTIC_READY sub-claims split 14 `RECORD_DRIFT` against 18 `SCOPE_REDIRECTED_BY_RULING`.
  - A separate `.s02` cause split in DEL-03-04/05/06 is recorded as CONTESTED.
  - The 9 W1 and 23 W2 RESOLVED_BY_RULE rows change only the disposition; cause, tier and layers keep their sealed values.
- **The F3 exception for "PKG-00 at SEMANTIC_READY".** This is a defensible application, but it is an interpretation. The exception names "revision pins, review states and metadata", while `SEMANTIC_READY` is a lifecycle or readiness state; the PKG-04 verifier says so explicitly. Three verifiers recommend the reading (PKG-03 W1, PKG-04 W-1 and the rerun), and it overrides 11 sealed rows where workers applied F3's origin test literally. Treat it as an Agent 0 reading of ruled text, not a ruling (B11-3).

## (c) Gate assessment

- **The rates match the reports:**

| Package | Firm false alignment |
|---|---|
| PKG-00 | 1/25 (4.0%) |
| PKG-01 | 0/50 (0%) |
| PKG-02 | 3/69 (4.3%); 5.7% if the out-of-sample worker-flagged row is added, which the assessment discloses |
| PKG-04 | 0/47 (0%) |

- **The PKG-03 post-rerun 1.5% is computed honestly.**
  - The first run was 2/68, and DEL-03-07's slice was 1 of 14 aligned normative rows.
  - The rerun verifier sampled 14 aligned normative rows at DOUBLE and found 0 errors. Replacing the slice gives 1/68 = 1.5%, as `WAVE_PLAN.md` provides.
- **Pooled:** 5/259 = 1.9%.
- **Verdict MET is right.** Every package is at ACCEPT WITH CONTESTED ROWS (PKG-03 after the rerun) and at or under 5%. Batch passes with resolutions, and all ledgers validate.

## (d) DEL-03-07 rerun

- **Byte identity.** The four files in `superseded_1/` (forward, reverse, notes, seal) are byte-identical to the first run committed in `5de73b711`. The first seal is `e339f3da…`, the new seal is `a549f6f8…`, and each matches its forward file.
- **Scoped verifier.** It was fresh, evidence-only and covered DEL-03-07 only at DOUBLE sampling. It did not read the first verifier's report or `superseded_1/`.
- **Records.** Launch, return and Agent 0 recheck events are all recorded, and both sides are committed.

## Findings

**B11-1. ACTIONABLE — CP-04 batch groups accept any tier and baseline class as a variant.**
- Where: `tools/validate_ledger_v2.py`, the `sub = …AuthorityTier…BaselineClass` key.
- Evidence:
  - The intent is the three variants defined in `CANONICAL_SITUATIONS.md`: default `LOCAL_DESIGN`/`NONE`, frozen-contract `PROJECT_BASELINE`/`FROZEN_CONTRACT`, and `.opsproj`/store `PROJECT_BASELINE`/`NONE`.
  - The code instead forms a group for every tier and baseline pair it sees. A CP-04 row with a wrong tier, for example `INVARIANT`/`NONE`, sits alone in its own group and is never compared.
  - Tier is itself a consistency field, so the split removes exactly the check that would catch it.
- Smallest fix: key on the variant only when the pair is one of the three defined pairs. Put any other pair in the default group, or flag it.

**B11-2. MINOR — the `RESOLVED_PAIR` exemption is broader than the pair.**
- Where: batch mode.
- Evidence: a `RESOLVED_PAIR` row is skipped in every group (body, CS variant and CP), not only the group of the conflict the verifier resolved. The file is controlled by Agent 0, and all 8 current uses cite a verifier.
- Smallest fix: exempt the row only from the group it was resolved in, or require a verifier `Source` for the class.

**B11-3. MINOR — the SEMANTIC_READY reading should reach the owner as an Agent 0 reading.**
- Where: the RESOLVED_BY_RULE rows in `WAVES/W1/RESOLUTIONS.csv` and `WAVES/W2/RESOLUTIONS.csv`; `W2_GATE_ASSESSMENT.md`.
- Evidence: treating a readiness state as a "review state" extends F3's exception list. It is recorded as "resolved by rule", which reads as a ruling.
- Smallest fix: label it an AGENT reading of F3's exception and list it with the canonical table at the owner checkpoint.

**B11-4. MINOR — spaces after the separator now pass.**
- Where: evidence columns.
- Evidence: `a; b` passes, although Part D says "no spaces".
- Smallest fix: reject a token that has leading or trailing whitespace before it is stripped.

## Other checks

- **Bound hashes.** All 51 entries in the latest `BOUND_INPUTS` at `cdcd14125` match their files.
- **Working tree.** It has an uncommitted `RUN_STATE.jsonl` change, which I did not review.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `python3 -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py`: **11 passed**.
- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied`.
- `validate_piping_loop_receipts.py --repo-root .`: VALID (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 4, WARN 112). No finding touches this run's files.

Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc11/`.

END-OF-RETURN