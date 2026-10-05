# RV88 review of U6c (`cb03315779`): the successor carrier schemas, with the branch-order pin repair

RV88, the standing independent U6 reviewer, is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), working to ROOT's U6c message, `BRIEFS/U6_FANOUT_COMMON.md` and PLAN §6 U6c. ROOT is the return path, and RV88 did not delegate.

RV88 did not write this code and did not use I66's tests as oracles. Every control below is RV88's own script or probe. I66's tests and mutants were rerun only as items to report.

## Verdict: PASS, with 0 BLOCKING, 0 SHOULD-FIX and 4 NOTE findings

- **The two `RowDisclosure` codes are admitted only in the successor branch.**
  - Both of U6a's real Rust successor derivatives go from invalid to valid under `results.v0.3`.
  - On every other valid document, a class code is refused (11/11 each).
  - The 7 non-successor branches each carry the same refusing clause; the successor carries none.
- **The AnalysisRun and stress-neutral successor branches require the receipt.** Every existing branch refuses `retained_precision`, null included. Shown on real records and packages built by the lane's own Python builders, then relabelled by RV88's generic transformation, with single-change negatives.
- **Y0–Y11:** RV88 ran its own **discriminating** versions over all 17 successor statements and both Rust derivatives. Every single change is refused, and every control is valid. I66's own Y1–Y8 and Y11 run on one statement only, and two of them are not discriminating (N-1).
- **No existing document's verdict changes.** RV88's sweep of 53 documents (with each injected probe) and the 24-file schema suite sweep confirm it: no verdict changes apart from the two successor derivatives, which are the intended fix; in the suite sweep, no test that passes at base fails, and the 3 pin tests go from failing to passing.
- **The pin patch** follows D2 §4.9.6 (T3's branch appended after T1's, at index 7). It removes no assertion, and only tightens each pin.
- **Mutants:**
  - I66's sample: I66's suite kills all 13 sampled (every 2nd of its 26), none by an error; RV88's scripts also kill 9 of them.
  - RV88's own 7: both suites kill 5 (W01, W03, W04, W05, W07). W02 is equivalent, and was replaced by W07. **W06 survives both** (N-4).

## Basis and host

- **The copies:** `cb03315779` (candidate) and `844448112f` (base) were copied by `git archive`, excluding the records tree. They went to `WT/rv88/c_{cand,base}`, with a mutant copy in `WT/rv88/c_mut`.
- **The candidate's files match I66's record.** The 7 changed files equal I66's `changed_files_sha256.txt` and `pin_files_sha256.txt`.
- **Python:** `REPO_ROOT/projects/chirality-piping/.venv` with `PYTHONDONTWRITEBYTECODE=1`. The checked-JSON CLI (`--features checked-cli`) and the units CLI (`--features cli`) were built `--release` from the U6c archive. The memory guard ran throughout.
- **When:** 2026-10-04, about 10:15Z to 11:05Z.
- **Not run:** no native, solver-at-scale or DEC-025 job, and no install.

## 1. The `RowDisclosure` codes (D-U6-2)

**The diff, read by RV88:**
- the enum gains the two codes;
- each of the 7 non-successor `ResultEnvelope` branches gains one `allOf` clause, `row_disclosures.items.reason_code: {not: {enum: [both codes]}}`;
- the successor branch is unchanged;
- the only removed line is the old enum's last element, rewritten with a comma.

**RV88's probes** (`rv88_schema_sweep.py` → `docsweep_c_{base,cand}.tsv`):

| Document | Base | Candidate |
|---|---|---|
| U6a's Rust successor derivative (69 class disclosures), sparse and dense | invalid (`'retained_precision_absolute_verified' is not one of…`) | **valid** |
| U6a's Rust projection derivative, both modes | valid | valid |
| A successor derivative with a class code on a plain disclosure, or an unknown code | — | class codes valid; unknown refused |
| Every other valid results document (14 load-reference documents plus the 2 projection derivatives) with a class code on its first disclosure | refused | refused (11/11 per code where disclosures exist) |

## 2. The AnalysisRun and stress-neutral successor branches

**Existing branches refuse a receipt.**
- **Committed documents (RV88's sweep):** every exact-valid AnalysisRun document (14) and stress-neutral document (14) refuses an injected receipt, in both lanes.
- **Freshly built records (`rv88_successor_shapes.py` → `shapes_c_{base,cand}.tsv`):** the lane's `build_analysis_run` built 6 records, for physics-source-1, source-blocks-1, physics-1, preview-physics-1, load-reference-source-1 and load-reference-1. Each is valid plain and refused with a receipt or a null member.

**The successor branches.** RV88 built a preview-physics-1 AnalysisRun record (`build_analysis_run`) and stress-neutral package (`build_stress_neutral_export_package_v0_3`) from each milestone's reader projection. It then relabelled them generically: every occurrence of the preview id and table sha was replaced by the successor's (and, for the profile variant, the profile too), and the real receipt was added.

| Probe (both modes) | AnalysisRun, candidate | Stress-neutral, candidate | Base |
|---|---|---|---|
| Positive | valid (either profile) | valid with the successor profile; refused with the preview profile | all refused |
| No receipt / null receipt | refused | refused | refused |
| Receipt with an extra member / policy v1 | refused | refused | refused |
| The preview sha under the successor id | refused | refused | refused |
| A receipt on the preview record or package | refused | refused | refused |
| **The other mode's (valid) receipt** | **valid** | **valid** | refused |

The last row is by design: the schemas check shape only, and cannot bind a receipt to its publication. See N-2.

## 3. RV78-N2's probes (Y0–Y11)

RV88 ran its own versions (`rv88_y_probes.py` → `yprobes_c_{base,cand}.tsv`) over 19 documents: the 15 corpus bases, both milestones, and U6a's two Rust successor derivatives. The pairs are discriminating:
- **Y4** uses a **shape-valid** foreign `source_block_recovery`, from source-blocks n05;
- **Y11** adds the W1 token to a **real** derivative value row.

| Probe | Candidate |
|---|---|
| Y0 control | valid 19/19 (at base the 2 Rust derivatives are invalid: the class codes) |
| Y1 receipt policy v1, Y2 unknown receipt member, Y3 empty body, Y4 shape-valid `source_block_recovery`, Y5 preview profile, Y6 limitations changed, Y7 contract ref, Y8 no `contract_evidence`, missing receipt, null receipt | refused 19/19 each |
| Y11, the token on a real value row | refused 2/2. The 17 scaffold documents have no value rows. |
| Y10, the projection on the base branch | valid 17/17 |
| Y9, the projection plus the receipt on the base branch | refused 17/17 |

## 4. Existing documents and suites

**RV88's document sweep validates every carrier document twice:** against the version dispatcher and against the exact v0.3 schema. The results dispatcher refuses every v0.3 identity except precision-1 (I66 F-U6c-2), so a dispatcher-only sweep cannot see a change in `results.v0.3`. The 53 documents are 20 results documents (including U6a's 4 Rust derivatives), 17 AnalysisRun and 16 stress-neutral documents.
- **Dispatcher verdicts:** identical, 35 valid and 18 invalid in both lanes.
- **Exact-schema verdicts:** identical, except the 2 successor derivatives, which go from invalid to valid (the D-U6-2 codes; F2 closed).

**The 24-file schema suite sweep** (`sweep_test_files.txt`, plus `test_retained_precision_schema.py`) was run in both archives (`suite_compare.txt`):

| Lane | Passed | Failed | Skipped |
|---|---|---|---|
| Base `844448112f` | 1,738 | 22 | 30 |
| Candidate `cb03315779` | 1,766 | 19 | 30 |

- **Failing to passing:** exactly the 3 pin tests:
  - `test_carrier_branches_are_appended_after_the_existing_methods`;
  - `test_joined_branches_are_appended_with_pinned_identities`;
  - `test_actual_composite_maximum_metadata_has_a_method_scoped_canonical_route`.
- **Passing to failing:** none.
- **New:** 25, all passing.
- **The 19 failures that remain,** in `test_handoff_export_workflow.py` and `test_handoff_package_schema.py`, are identical in both lanes. They are archive-lane artefacts: those tests read execution-record fixtures, which the archive excludes, as I66 recorded. ROOT's in-worktree run (1,785 passed, 0 failed) is 1,766 + 19, which agrees.

## 5. The pin patch (3 test files, applied as ruled)

- **Each list or count gains the successor at the end,** after T1's load-reference-source-1. This is D2 §4.9.6 ("appended after T1's branches (index 7 and on)"). C1:162 requires explicit successor branches in all three schemas, and the patch pins exactly those.
- **The pins, one by one:**
  - `test_load_reference_schema.py:606–611` keeps an exact identity list. It gains the successor, and load-reference-1's AnalysisRun `not` gains `{"required": ["retained_precision"]}`, which is U6c's own new clause, pinned exactly.
  - `test_load_reference_source_schema.py:62–69, 302–311`: `CARRIER_ORDER` gains the successor. Each `[-1] == T1` pin becomes `[-2:] == [T1, successor]`, which is stricter.
  - `test_source_block_schema_contract.py:377–388`: the count goes 7 → 8, and the successor's branch is pinned to `PreviewPhysicsResultSet`, where it would otherwise default to `ResultSet`. That pin is stricter.
- **The removed lines are only the replaced pins,** and each replacement asserts at least as much.

## 6. Mutants (`rv88_schema_mutants.py` → `mutants.json`)

Each mutant changes one constraint in `WT/rv88/c_mut`. It then runs I66's `test_retained_precision_schema.py` and RV88's three scripts. **A kill by RV88's tests** means that a script's output differs from the unmutated candidate's; the control matches exactly.

| Mutant | I66's suite | RV88's scripts |
|---|---|---|
| R01_codes_not_in_enum | killed | killed (docsweep, yprobes) |
| R03_precision_admits_codes | killed | survived |
| A01_no_receipt_property | killed | killed (docsweep, shapes) |
| A03_successor_sha_preview | killed | killed (shapes) |
| A05_contract_id_enum_missing | killed | killed (shapes) |
| A07_physics_source_admits_receipt | killed | killed (shapes) |
| A09_successor_admits_evidence | killed | survived |
| A11_row_identity_open | killed | survived |
| S02_profile_enum_missing | killed | killed (shapes) |
| S04_successor_profile_preview | killed | killed (shapes) |
| S06_successor_admits_sbr | killed | survived |
| S08_successor_sha_preview | killed | killed (shapes) |
| S10_ref_enum_missing | killed | killed (shapes) |
| W01_results_successor_receipt_not_required | killed | killed (docsweep, yprobes) |
| W02_results_preview_admits_receipt | survived | survived. equivalent: the preview branch refuses a receipt through its allOf[0] not-required clause, so adding a property definition changes no verdict; replaced by W07 |
| W03_results_load_reference_admits_codes | killed | killed (docsweep) |
| W04_analysis_run_lrs_admits_receipt | killed | killed (docsweep, shapes) |
| W05_stress_neutral_lr_admits_receipt | killed | killed (docsweep) |
| W06_stress_neutral_successor_annotations_optional | survived | survived. test gap: a successor stress-neutral package without source_annotations is valid under the mutant (checked directly) and refused by the candidate |
| W07_results_preview_drops_receipt_refusal | killed | killed (docsweep, yprobes). run separately after the batch (same lane and oracles) |

## Findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| **N-1** | NOTE | P/tests/test_retained_precision_schema.py:162–191 | Only Y0, Y9 and Y10 run over all 17 statements. Y1–Y8 and Y11 run on corpus case 0 alone; ROOT's summary ("run as pairs over all 17") overstates this. Two of them also cannot discriminate: **Y4** sets `source_block_recovery: {}` and **Y11** replaces the value rows with `{"result_id": "x", "recovery_method": …}`. Both would be refused for shape reasons, whatever the clause under test does. RV88's discriminating forms (shape-valid foreign receipt; a token on a real value row), over all 19 documents, are all refused (§3). | Optional, at U6f: adopt the shape-valid Y4 and a real-row Y11, and loop Y1–Y11 over `successors()`. |
| **N-2** | NOTE (for U6b) | P/schemas/analysis_run.v0.3.schema.json, stress_neutral_export.v0.3.schema.json | The successor AnalysisRun and stress-neutral shapes accept a valid receipt from **another publication** (the other mode's). The schemas cannot bind a receipt to the raw source. TS's `validateAnalysisRunV03` refuses it (U6d §3); Rust's derivative validator refuses it (U6a). | U6b's Python `validate_analysis_run` must compare the copy with the source's receipt exactly (PLAN §1b: `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`), and RV88 will test it. Schema validity is never receipt binding. |
| **N-3** | NOTE (confirms I66 F-U6c-2) | P/schemas/results.schema.yaml | The results dispatcher refuses all 18 v0.3 documents of the newer identities in both lanes, while `results.v0.3` accepts them. This is pre-existing, and already routed to T6. Any consumer that validates through the dispatcher refuses every successor derivative. | As routed (T6). Until then, U6f and U7 should validate successor documents against `results.v0.3` directly, as every test does. |
| **N-4** | NOTE (test gap) | P/schemas/stress_neutral_export.v0.3.schema.json (successor branch `required`); P/tests/test_retained_precision_schema.py | Mutant W06 removes `source_annotations` from the successor stress-neutral branch's `required`, and survives I66's suite and RV88's scripts. RV88 checked directly: a successor package without `source_annotations` is refused by the candidate and accepted by the mutant. The requirement is correct but unpinned. (I66's S09 pins `contract_evidence` the same way.) | Add one negative, "successor package without `source_annotations`", to `test_stress_neutral_successor…` at U6f or with U6b. |

## For ROOT to rule

Nothing blocks. N-2 is a test obligation for U6b (RV88 checks it there). N-1 and N-4 are optional test additions for U6f.

## Records

Everything is in `_run_records/`, with placeholder paths only. **The scripts:**
- `rv88_schema_sweep.py`, `rv88_successor_shapes.py`, `rv88_y_probes.py` and `rv88_schema_mutants.py`;
- `run_suite.sh`.

**The outputs:**
- `docsweep_c_{base,cand}.tsv`, `shapes_c_{base,cand}.tsv` and `yprobes_c_{base,cand}.tsv`;
- `suite_compare.txt`;
- `mutants.json`.

SHA256SUMS covers this folder.
