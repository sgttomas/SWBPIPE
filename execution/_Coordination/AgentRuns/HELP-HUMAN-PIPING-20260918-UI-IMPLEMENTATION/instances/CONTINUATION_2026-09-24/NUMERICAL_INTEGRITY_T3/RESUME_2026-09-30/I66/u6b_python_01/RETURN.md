# I66 return: U6b, the Python carriers (D-U6-9 included)

I66 is a TASK (Type 2) under ROOT and the owner of U6. This unit follows `BRIEFS/U6_FANOUT_COMMON.md`, PLAN §6 U6b and §1b, and ROOT's message "Start U6b, the Python carriers". It did not delegate.

**Verdict: U6b is done, with one stop to report.**
- The F2a successor `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` now passes through every Python carrier in §1b:
  - dispatch, the fresh set and standing;
  - binding refusal and the classification summary;
  - AnalysisRun build and validate;
  - the 0.1.0 wrapper refusal (D-U6-9).
- The new test file passes: 21 tests, run against both milestones.
- The 14 shared parity scenarios agree with the expectations file that Rust U6a and TS U6d read.
- The packager refusal still holds (T6). No reader file and no flag was touched. Standing stays `needs_recompute` until U7.

**The stop (§ Stop).** D-U6-6 adds the successor to `FRESH_CONTRACT_IDS`. One existing test pins that set exactly: `tests/test_preview_physics_consumer_contract.py::test_table_identity_and_registry`. It is outside my fence, so I have **not** changed it. A tested 3-line patch is ready for your ruling. In the 24-file sweep it is the only test that differs from base.

## Basis, host and fence

- **Worktree:** `WT/f2a-carriers`, at `cb03315779`, uncommitted. No Git writes. Git reads used `GIT_OPTIONAL_LOCKS=0`. The base lane was made with `git archive` (a read).
- **When:** 2026-10-04, from 10:01Z to about 10:40Z. The memory guard (PID 5387) was checked before every run.
- **Python:** `REPO_ROOT/projects/chirality-piping/.venv`, with the I52 checked-JSON and units CLIs set and `PYTHONDONTWRITEBYTECODE=1`. Runs used `_run_records/run_py.sh`.
- **Not run:** Cargo, npm, installs, new tooling, native jobs, solver jobs and DEC-025.
- **Scratch:** `WT/scratch/i66_u6b_python/` holds:
  - `base`, an archive of `cb03315779` P without execution records, except the PKG-15 handoff fixtures that two sweep files read;
  - `mut`, the mutant lane;
  - `pinlane`, the lane for the proposed pin patch.
- **The fence held: 3 files** (`git status`).

| File | sha256 | Change |
|---|---|---|
| P/core/analysis_runs/compatibility.py | `6f66f9c860a18883a797c138fdba13a9ef2ffa9fef9e99bd84036ea3fb993b9d` | +160/−5. See below. |
| P/core/analysis_runs/records.py | `e3b867d7080e1f5cb648755246b96140efdcd182ad2e853ce505b2df0e1d36be` | +4: the D-U6-9 guard. |
| P/tests/test_retained_precision_carriers.py (new) | `39e09be5912b711c401e10aca5e8576cd0cc54f8135d1c60aff490a1fdbe4cf0` | 21 tests. |

**compatibility.py, by §1b row:**
- **Dispatch.** `_source_contract` sends the successor id to the accepted reader, `validate_retained_precision` (G0–G8, after D-U6-1). A reader failure raises its first code. A G7 failure keeps the base validator's own text, as in Rust. It returns `(id, c74742ce…, the retained table path)`.
- **Downgrade guards (F-5).**
  - Any other identity carrying a `retained_precision` member gets `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`. That includes a null member. The check applies to mappings only, as Rust's `source.get` does.
  - In the raw path, any row carrying the W1 method token gets the same code. As in Rust, the header-only transport path does not read rows.
- **Transport.** `check_receipt=False` on the successor is refused with `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`. The Python reader has no transport validator (F-U6b-2).
- **Fresh and current sets.** The id joins `FRESH_CONTRACT_IDS` (D-U6-6). The new `CURRENT_RECORD_CONTRACT_IDS` replaces the three inline 7-id allow-lists in `_build_analysis_run`, `build_analysis_run` and `validate_analysis_run_v0_3`. It holds the same 7 ids plus the successor.
- **Standing.** The successor branch comes first in `numerical_use_standing`. It validates once, using `source_block_context` as the invocation. Any reader error gives `unsupported`. Otherwise `_retained_standing_from` applies PLAN §3 rule 2:
  - the validation is invocation-bound and `numerical_eligible`;
  - the requested refs equal `body.cases[].basis_ref` in order;
  - the source is `MECHANICS_SOLVED`;
  - every case is `selected`, or `not_required` and meets the base ordinary-eligibility predicate (F-7).

  If any of these fails, the standing is `needs_recompute`. `numerical_quality` never contributes. Because eligibility is held, the result today is `needs_recompute` with or without an invocation.
- **Binding.** `rule_binding_refusal` on a successor uses only the reader's validated class:
  - absolute → `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`;
  - not covered → `RULE_QUANTITY_NOT_COVERED`;
  - a refused statement → `RULE_QUANTITY_NOT_COVERED` for every row (F5, as in Rust);
  - a row with no classification → `None`, as in Rust.
- **Summary.** `classification_summary(source, invocation)` mirrors Rust's field for field, including `withheld` and `interval_bindable: 0`. It returns `[]` for any other identity or a refused statement.
- **AnalysisRun.**
  - Build copies the complete receipt into `analysis_run.retained_precision` (C1:162).
  - Validate requires exact equality with the source's receipt, else `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`. A receipt on any other identity's record gets `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
  - The explicit 0.2 constructor's forbidden-member list gains `retained_precision`, which gets `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN` (F-U6b-3).

**records.py (D-U6-9).** `build_preview_analysis_run_envelope` refuses any source carrying a `retained_precision` member with `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`. The receipt is never hashed and dropped.

## The tests (`tests/test_retained_precision_carriers.py`)

Every milestone input is checked against the D-U6-5 sha256 pins and receipt hashes before use.

- **Dispatch, in both modes:**
  - the exact tuple, and the table bytes against the reader's `TABLE_HASH`;
  - fresh membership, and `standing_reason` is None;
  - a bad receipt gives `RETAINED_PRECISION_RECEIPT_MISMATCH`;
  - a hash-consistent G7 unit edit gives the reader's detail text;
  - transport is refused.
- **Standing, in both modes:**
  - `needs_recompute` with and without the invocation;
  - an edited row gives `unsupported`, with and without the invocation (the I67 F1 case below);
  - a foreign solver mode gives `unsupported`;
  - a hash-consistent `checks_passed` quality claim is refused by the reader (D6b) and reads `unsupported`.
- **Rule 2 through the seam**, with eligibility set in the validation dict and no flag touched:
  - The positive cases give `numerically_eligible`: a selected case, and a `not_required` case whose quality meets the predicate.
  - Each conjunct, when it fails, gives `needs_recompute`:
    - invocation-bound and eligibility;
    - empty, other and extra refs;
    - mechanics status;
    - each of the seven predicate fields;
    - duplicate evidence ids and short quality;
    - case status `unavailable`, other or None, even with qualifying quality.
- **The 14 shared parity scenarios** from `fixtures/results/retained_precision_carrier_cases.json`. Each checks the expected standing and the expected raw-dispatch result: `ok` or the exact first error.
- **AnalysisRun, in both modes:**
  - Build leaves the source unchanged.
  - The record carries the receipt byte-equal, with the successor contract on the reproducibility block and on every row ref, and no `contract_evidence` or `source_block_recovery`.
  - `verify_analysis_run_record` returns `match`, validate passes, and the record is valid under `schemas/analysis_run.schema.json`.
  - **Back out:** the record's receipt reauthenticates the raw publication with an identical validation.
  - **Refused with `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`:** a dropped receipt, an altered body, and an altered `receipt_sha256`.
  - **A record checked against an edited source** fails at the reader first.
- **Downgrades, in both modes:**
  - **Refused with `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`:**
    - relabelled with the receipt kept (by dispatch and by build; its standing is `unsupported`);
    - relabelled with only the tokens kept;
    - the projected base with `retained_precision: null`.
  - The token check in the transport path is header-only (Rust parity).
  - The projected base builds and validates. With the receipt pasted into its record, it gets `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
  - The explicit 0.2 constructor refuses a receipt.
- **Binding codes, in both modes:**
  - every row matches its validated class;
  - the headline displacement binds, and the headline stress gets `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`;
  - an edited source refuses every row with NOT_COVERED;
  - an unclassified row and a row without an id give None;
  - the base projection binds every row;
  - the class map covers all five classes and None.
- **Summary, in both modes:**
  - the counts are 25/69/3/1 (sparse) and 25/69/3/2 (dense), with `withheld` 97, with and without the invocation;
  - an edited source or a base projection gives `[]`.
  - **Through the seam after U7:** only absolute rows are withheld when the invocation matches, and the held formula applies without an invocation or with another one. A fabricated `not_covered` class is counted and withheld both ways. A classification under another case is not counted.
- **Stress-neutral pin (T6), in both modes:**
  - the packager build on the successor gives `SN-SOURCE-METHOD-UNSUPPORTED`;
  - the base projection still packages;
  - that package relabelled to the successor, with the receipt, fails validation with `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (F-U6b-2).
- **records.py (D-U6-9), in both modes:**
  - the successor is refused;
  - the wrapper's own fixture (`fixtures/product_preview/invented_mechanics_result.json`) still builds;
  - the same fixture carrying the real receipt, or `null`, is refused.
- **R-2 noticed envelope, in both modes.** I appended the reserved unavailable notice to the base projection. It uses U3 grant 1b's exact id, code, severity, message, source and affected refs. The result:
  - it dispatches as preview-physics-1 with the same tuple as the plain base;
  - its standing equals the plain base's;
  - its AnalysisRun builds and validates, carries the notice as a source annotation, and has no receipt;
  - no row is refused.

## Controls

**1. The 24-file sweep** (`sweep_test_files.txt`, plus `test_retained_precision_schema.py`, as at U6c; `sweep.compare.txt`):
- **Base** (archive of `cb03315779`): **1,785 passed, 30 skipped, 0 failed.** This matches ROOT's count.
- **Candidate:** 1,805 passed, 30 skipped, **1 failed**. The 21 extra are the new tests.
- **The only test that passes at base and fails in the candidate is the fresh-set pin (§ Stop).** Nothing else changed, and no test was removed.
- The sweep ran on the final code bytes. The new test file had three assertions added after the sweep started. Its final bytes pass 21/21 (`new_tests_final.summary`) and were used for the mutants.

**2. The other Python consumers of the changed carriers** (`extra_test_files.txt`: product-preview service, both load-reference readers, source-blocks validation, comparison report sections; `extra.compare.txt`): 428 passed and 2 skipped at both base and candidate, with **0 differences**.

**3. The document sweep** (`doc_sweep.py`; `docsweep_compare.txt`). It covers every committed JSON under P/fixtures and P/tests that has a `results` list: 62 documents. For each, it records 11 carrier outcomes, under base and candidate:
- dispatch and transport;
- standing with no refs and with the quality refs;
- `standing_reason`, binding and the summary;
- AnalysisRun build and validate;
- the 0.2 constructor and the 0.1.0 wrapper.

The result:
- **On the 60 non-successor documents, nothing differs.** The only change is the new `classification_summary`, which is absent at base and returns `[]` in the candidate.
- **All other differences are on the two successor milestones.** At base they fail closed (`SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, `unsupported`). In the candidate they are admitted through the reader, with `needs_recompute`, class-based binding, the summary, and a record that builds and validates. The 0.1.0 wrapper now refuses them with the receipt code.

**4. The new tests discriminate** (`base_with_new_tests.summary`). The final test file was run against the base code, with the successor id given as a literal. **19 of 21 fail.** The 2 that pass are the R-2 tests, which pin unchanged base behaviour.

**5. Nothing weakened.**
- The diff removes no check. The removed lines are the three inline allow-lists, now the named set with the same 7 ids plus the successor.
- No existing test was changed.
- `retained_precision.py`, the flags and the T6 packager are untouched.

**6. Mutants: 60, of which 59 are killed and 1 is equivalent** (`mutants.py`, `mutants_final.json`).
- Every mutant compiled and applied. Each kill is a failing test, and all 59 first fail in the new file; `test_analysis_run_compatibility.py` ran with it.
- **What they cover:**
  - **Dispatch (13):** G7 text, transport admission, the reader skipped, sha and path, member and token guards and their gating, and `_is_retained`.
  - **Sets and routers (5).**
  - **Standing (12)**, including S12: the successor routed to the generic quality branches, which is PLAN §3 rule 3's mutant. It is killed by the foreign-mode case.
  - **The `not_required` predicate (10).**
  - **Binding (5).**
  - **Summary (6).**
  - **AnalysisRun (6).**
  - **records.py (3).**
- **Equivalent: S04** adds the successor to the generic quality-branch set. That branch is unreachable for the successor, because the successor branch returns first. S01 removes the successor branch and S12 does both; both are killed.
- **History** (`mutants_run1.json`, `mutants_run2.json`). The first round had three survivors:
  - **A04** (equality checked on the body only) is now killed by an altered-`receipt_sha256` case.
  - **D12** (the token guard in the transport path) is now killed by the header-only parity assertion.
  - **S04** is the equivalent mutant above.

## Stop: the fresh-set pin (outside the fence; ruling needed)

- **The test:** `tests/test_preview_physics_consumer_contract.py::test_table_identity_and_registry`, which comes from T0R and T1.
- **What it does:** it asserts that `FRESH_CONTRACT_IDS` equals exactly the six pre-U6 ids. Under D-U6-6, the successor joins that set. It passes at base and fails in the candidate (`cand_sweep_failure.txt`).
- **The Rust counterpart:** `precision_1_is_readable_but_never_fresh` was updated inside U6a's fence for the same reason.
- **The proposed patch** (`_run_records/proposed_pin_test.diff`, not applied) changes 3 lines:
  - import `PREVIEW_PHYSICS_RETAINED_CONTRACT_ID`;
  - add it to the expected set;
  - add one comment citing D-U6-6.

  It removes nothing, and every existing assertion still binds. With the candidate code and the patch, the file passes 140/140 (`pinlane.summary`).
- **Proposed ruling:** extend U6b's fence to this one file for this patch alone, with RV88 reviewing it as a pin update rather than a weakening.

## Findings

- **F-U6b-1:** the stop above.
- **F-U6b-2 (Python transport gap; a deviation from the §1b dispatch row).**
  - PLAN §1b says "else the transport validator". The Python reader has none; Rust's `validate_transport_metadata` and TS's equivalent exist. So U6b refuses a transported successor, failing closed, rather than admitting an unchecked one.
  - Python's only `check_receipt=False` caller is the T6 stress-neutral packager, which refuses the successor anyway. So the outcome is unchanged today.
  - **A consequence:** a successor-shaped package fails validation at `_transport_contract` with `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, not `SN-SOURCE-METHOD-UNSUPPORTED`. Both codes are pinned. If a Python transport validator lands, that pin moves to `SN-SOURCE-METHOD-UNSUPPORTED`.
  - **Proposed:** route a Python `validate_transport_metadata` to U6e's reader round, or to wider F2a before T6 admits successor packages.
- **F-U6b-3 (a judgement within the fence, disclosed for RV88 and TS parity).**
  - The explicit 0.2 constructor (`build_analysis_run_v0_2`) skips `_source_contract`. It would otherwise accept a 0.1.0/0.2.0 source carrying `retained_precision` and drop it.
  - I added the member to its existing forbidden list, which gives `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN`, in the spirit of D-U6-9 and §4 ("never silently dropped").
  - No existing identity carries the member (F-5), and the document sweep shows no change.
  - U6d may want the same rule in TS's legacy constructor, if one exists.
- **F-U6b-4 (plan text).** PLAN §1b says the 0.1.0 wrapper "accepts any source". In fact it already refused both milestones at base, with `ANALYSIS_RUN_RESULT_DIMENSION_UNDECLARED`, because preview-physics-1 maxima declare no source dimension. The D-U6-9 guard now fires first, with the receipt code. It also covers inputs the wrapper does admit, tested with the wrapper's own fixture.
- **I67 F1 (noted, unchanged).** Python reads `unsupported` for an invalid successor without an invocation, and `needs_recompute` for a valid one, as the plan specifies. Both are pinned. RV88 and U6f are judging whether TS must match.
- **Limits:**
  - The fixtures are single-case producer test outputs (D-U6-5), not native Current evidence.
  - Eligibility is held. So `numerically_eligible`, the post-U7 summary and the per-case partition are tested through the seam, not end to end. U7 or U9 reruns these tests on live output (PLAN §6).

## Records

- `_run_records/` holds:
  - the runner, the mutant, sweep, comparison and document-sweep scripts;
  - the sweep, extra-suite and document-sweep outcomes with their comparisons;
  - the mutant results;
  - the discrimination run;
  - the candidate diff and changed-file hashes;
  - the proposed pin patch and its lane result.

  All use placeholder paths (`WT`, `REPO_ROOT`, `P`).
- SHA256SUMS covers this folder.
