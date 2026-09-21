VERDICT: FINDINGS

Backcheck 8 reviewed commit `88ad686a0` on `claude/piping-recon-r2-w1-20260921`. Three actionable findings and three minor ones remain. The most important is that Part F's origin test (F3) does not match the ruled remedy text.

What checks out:
- **Ruling record.** The two owner blocks in `W1_GATE_RULING.md` match their stated hashes: 209 bytes (`1d9297db…`) and 361 bytes (`858bc341…`). The assessment's SHA-256 is `0440ef3b…`, as stated.
- **Bound hashes.** All 50 entries in the latest `BOUND_INPUTS` event at `88ad686a0` match their files.
- **Validator without the new flags.** On all 13 wave 1 ledgers, single mode (forward, reverse and inventory) gives the same result as the previous version: PASS, 0 findings.
- **Batch mode.** Over the 13 ledgers it fails with 1 finding (the DEL-07-06 R04 body conflict). With `--resolutions` it passes.
- **`RESOLUTIONS.csv`.** It has 32 rows, and every item in both verification reports is there:
  - PKG-07: 4 firm; 1 resolved pair; 4 R-005 rows left contested; 6 weak (W6–W11); 11 field.
  - PKG-16: 1 firm; 3 weak; 2 field.
- **Resolution values.** Each matches the verifier's value, or applies F3, F7 or F8 as intended. The four DEL-07-03 R-005 rows are left contested, with empty values and a candidate reading for R4. Every key exists in the sealed ledgers, and the ledgers' current values match the reports' "row has" values.
- **Briefs.** Sealing still comes before the reverse pass. `--notes-gap` is required before sealing. Verifier sampling is correct:
  - rows carrying `GAP_WORDING_CHECKED:` or `OPEN_ACTION:`: 100%;
  - rows carrying `PRODUCT_CALLER: NONE`: 25% (doubled in gate waves);
  - F5 checks on overlapping `NOT_MINE` answers: at least 20%;
  - the ISSUED DEL-01-01: 100% (existing rule).
- **Gate wave 2 plan.** It has 31 deliverables. Worker budgets are 2+1+2+3+2 = 10, so live agents are 1 + 5 + 10 = 16. Verifiers take manager slots as those free up. The gate conditions are unchanged and judged per package.

## Findings

**B8-1. ACTIONABLE — F3 uses a later cutoff than the ruled R-3, and conflicts with C6(c) and the keyed canonical rows.**
- Where: `CONVENTIONS.md` F3; `R2-WORKER_brief.md` ("before 2026-07-14").
- Evidence:
  - R-3, adopted as written, says: "Text first present at the initial migration … is `STALE_SETUP_SPECIFICATION` … `STALE_REVIEW_OR_EVIDENCE` is only for text first declared later". The initial migration is `7bee9ae41`, "Initial migrated Chirality repository", dated 2026-05-18. The verifier's W6–W11 rows use exactly that commit.
  - F3 instead sets the cutoff at the Scope of Work migration, 2026-07-14. So text first declared between 05-18 and 07-14 becomes setup-era under F3 but stays review-class under R-3. For example, the D-41 R5 T7 "current declaration" blocks were added in `d2073a0c1` on 2026-07-12.
  - C6(c) puts "later declarations, review states, revision pins and metadata" under `STALE_REVIEW_OR_EVIDENCE`, and CP-02, CP-03 and CP-05 do the same.
  - The keyed rows CS-01, CS-04, CS-05 and CS-06-DRIFT are canonically `STALE_REVIEW_OR_EVIDENCE` for setup-era `_CONTEXT.md` pins, and the validator enforces that. F3's absolute wording contradicts all of this.
- Smallest fix:
  - Word F3 as "first present at the initial migration `7bee9ae41` (2026-05-18)".
  - Add: "Keyed CS rows keep their assigned class. Revision pins, review states and metadata stay `STALE_REVIEW_OR_EVIDENCE` under C6(c)."
  - Mirror both in the worker brief.

**B8-2. ACTIONABLE — `--notes-gap` is mostly noise, and it misses common gap wording.**
- Where: `tools/validate_ledger_v2.py`, `GAP_WORDING` and `GAP_COLUMNS`.
- Evidence:
  - Under Part D, evidence columns hold only path tokens, so any match there comes from a path.
  - On wave 1, 43 of the 95 flagged `ALIGNED` rows are flagged only by paths. Examples are `features/missing-data/…` and `test_missing_data_warning_ux.py`. PKG-04's nonlinear `gap` element code will flag heavily in wave 2.
  - The pattern misses "gaps" (plural) and "no … test". So it missed firm F3, `DEL-07-02:SOW#CLM-018.r06`, whose note reads "Solve-required gaps are covered; no rule-check-specific negative test".
  - `GAP_WORDING_CHECKED:` accepts any 10 characters. With this much noise, that escape becomes routine.
- Smallest fix:
  - Scan only `Notes` and `RemainingWork`, or strip path tokens first.
  - Use `gaps?\b` and something like `\bno\b[^.;]{0,40}\btests?\b`.
  - Rerun the wave 1 count and record it.

**B8-3. ACTIONABLE — the resolution for `DEL-07-02:SOW#CLM-026` uses a different tier from the row that carries the same gap.**
- Where: `WAVES/W1/RESOLUTIONS.csv`.
- Evidence:
  - The resolution gives `UNKNOWN · EVIDENCE_NOT_LOCATED · LOCAL_DESIGN · RECORD` and leaves `AuthorityNeeded` at `NO`.
  - The unmet element is the protected-content review, the same gap the row says is "carried on RQ-008". RQ-008 is `INVARIANT · IP_DATA`, and its field correction sets `AuthorityNeeded REVIEW`.
  - Under F8 (tier of the gap), a gap that touches the IP boundary's subject is `INVARIANT`. The verifier's value predates F8.
- Smallest fix: set `INVARIANT`, layers `IP_DATA;RECORD` and `AuthorityNeeded REVIEW`, or record why F8 does not apply.

**B8-4. MINOR — F2's last sentence goes beyond R-2 and is labelled as ruled text.**
- Where: `CONVENTIONS.md` F2.
- Evidence: the sentence making a Remaining item whose runtime observation predates later code changes `UNKNOWN · EVIDENCE_NOT_LOCATED` generalises one verifier resolution (body `1840ad3a…`). R-2 does not contain it.
- Smallest fix: label it AGENT under the precedence clause.

**B8-5. MINOR — some verifier observations are not recorded for R3.**
- Where: `WAVES/W1/RESOLUTIONS.csv`.
- Evidence:
  - Rows that rest on the R-005 reading but were not counted: DEL-07-03 `CLM-018` and `CLM-012/DEL-07-03-R-002`.
  - Out-of-sample observations:
    - `DEL-07-08:SOW#CLM-004.r04` is `ALIGNED` on "TBD";
    - `DEL-16-03:SOW#CLM-011` is `ALIGNED` although it declares persistence TBD. Both are likely F1 cases;
    - `DEL-16-02:SOW#CLM-010/REQ-16-02-002` has field inconsistencies.
  - Under R-6, only counted disagreements are recorded, so R3 would not see these.
- Smallest fix: add them with a class such as `OBSERVED` or `CONTESTED`.

**B8-6. MINOR — the wave plan disagrees with the briefs on group size, and one cap case is unstated.**
- Where: `WAVE_PLAN.md`, gate wave 2; the worker and manager briefs.
- Evidence:
  - Two gate-wave-2 groups hold 2 deliverables (under Direction 6), but both briefs still say "3–4".
  - The plan does not say how a verifier-triggered rerun fits within the full cap of 16.
- Smallest fix: say "2–4" in both briefs, and state that a rerun manager waits for a free slot.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `python3 -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py`: **11 passed** (34.0s).
- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied`.
- `validate_piping_loop_receipts.py --repo-root .`: VALID, frozen through Receipt-44 (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 4, WARN 112). No finding touches this run's files.
- `--notes-gap` on the wave 1 ledgers, for information only (these ledgers predate Part F): 12 of 13 fail, with 3–33 findings each. DEL-07-09 passes.

The working tree has an uncommitted `RUN_STATE.jsonl` change, which is not part of the commit I reviewed. Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc8/`.

END-OF-RETURN