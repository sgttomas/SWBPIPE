# I66 return: U6c, the successor carrier schemas (closes U6a's F2)

I66 is a TASK (Type 2) under ROOT, the owner of U6, working to `BRIEFS/U6_FANOUT_COMMON.md`, PLAN §6 U6c and RR "U6a verified and committed; fan-out". It did not delegate.

**Verdict: U6c is done.** The F2 block is lifted:
- the AnalysisRun and stress-neutral schemas carry explicit successor branches (C1:162; D2 §4.9.6; RV78-S1);
- `results.v0.3` admits D-U6-2's two `RowDisclosure` codes in the successor branch only;
- U6a's actual Rust derivatives now validate against `results.v0.3`.

Every control passes, and all 26 schema mutants are killed.

**One stop to report (§ Stop).** Three existing pin tests pin branch counts and positions. They have been failing **since the reader fan-in** (`c15e64b756`), at the base of this grant too. U6c would extend the same mismatch to the two new branches, though earlier assertions currently mask it. Fixing them means writing outside my fence. So I have **not** done it. A tested patch is ready for your ruling.

## Basis, host and fence

- **Worktree:** `WT/f2a-carriers`, on `844448112f`, uncommitted. No Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **When:** 2026-10-04, about 08:34Z to 09:40Z, with the memory guard (PID 5387) running.
- **Python:** `REPO_ROOT/projects/chirality-piping/.venv`, with the I52 CLIs set and `PYTHONDONTWRITEBYTECODE=1`.
- **Not run:** no Cargo, npm, install, native or DEC-025 job.
- **Scratch:** `WT/scratch/i66_u6c_schemas/` holds the base archive of `844448112f` (`basefull`, all of P except execution records), plus the `mut` and `pinlane` lanes.
- **The fence held: 4 files.** The three schema files were changed by one reproducible script, `_run_records/apply_u6c.py`. It keeps each file's exact JSON style, verified by a byte-exact round trip before writing.

| File | sha256 | Change |
|---|---|---|
| P/schemas/results.v0.3.schema.yaml | `eb21b496…` | Both codes are added to `RowDisclosure.reason_code`. Each of the 7 non-successor branches gains, in its existing `allOf`, a clause refusing those two codes on `row_disclosures`. The successor branch is unchanged. |
| P/schemas/analysis_run.v0.3.schema.json | `04cf09ca…` | `SemanticContract` gains the successor id, the sha `c74742ce…` and its oneOf pair. `AnalysisRun.retained_precision` points (`$ref`) to the closed receipt schema. Every existing branch refuses `retained_precision`. A new 8th branch, the preview-physics-1 branch with the successor's identity and sha, **requires** the receipt and refuses `source_block_recovery` and `contract_evidence`. |
| P/schemas/stress_neutral_export.v0.3.schema.json | `6355640a…` | The successor joins the producer, contract-ref, contract and profile enums. A new top-level `retained_precision` points to the closed receipt schema. Every existing branch refuses it. A new 8th branch, the preview-physics-1 branch with the successor's identity, sha and profile, requires `contract_evidence`, `source_annotations` and `retained_precision`, and refuses `source_block_recovery`. **This is shape only:** the packager still refuses the successor (T6), as tested. |
| P/tests/test_retained_precision_schema.py | `65aff0f4…` | 12 tests before, 37 now: 25 new and the 12 existing unchanged. |

## The tests

All use the repository's `validate_instance`.
- **RV78-N2 (Y0):** the derivative shape is checked for every successor statement on hand: the 15 corpus cases and both milestones, which are byte-checked against PP's pins.
- **RV78-N2 (Y1–Y11):** each is a **discriminating pair**: the unmodified document validates, and the single change is refused.
  - Y9 and Y10 now run for every statement: the projection to the base identity validates, and the same projection with a receipt is refused. RV78's single Y9 probe could have been refused for another reason.
- **D-U6-2:**
  - the enum and the exact per-branch clause, checked structurally;
  - the successor with each class code, or an ordinary code, is valid;
  - the projected base with a class code is refused, and with an ordinary code is valid;
  - an unknown code is refused.
- **AnalysisRun, in both modes:**
  - the base preview record built by `build_analysis_run` from the reader's own projection is valid;
  - the successor-shaped record with the real receipt is valid, through the dispatcher and the exact-version schema.
  - **Refused:**
    - no receipt;
    - receipt policy v1;
    - an unknown receipt member;
    - a shape-valid foreign `source_block_recovery` (a source-blocks receipt) or `contract_evidence` (physics-source evidence);
    - a successor id with the preview sha;
    - a row ref with the preview identity;
    - a receipt on the preview record.
  - The physics-source branch, which had no `not` before, refuses a receipt.
  - A structural check of all 8 branches.
- **Stress-neutral, in both modes:** the base preview package from the real packager is valid, and the successor-shaped package is valid.
  - **Refused:** no receipt, receipt policy v1, the preview profile, the preview sha, a contract-ref mismatch, `source_block_recovery`, no `contract_evidence`, and a receipt on the preview package.
  - **The packager itself still refuses the successor.** A structural check covers all 8 branches and the enums.
- **The new tests fail against base:** 7 of the new tests fail on the base schemas, with the test file run in the base lane. The RV78-N2 pins pass there, as they should; they pin already-accepted behaviour.

## Controls

**1. Existing identities: unchanged.**
- **Document sweep** (`doc_sweep.py`; `docsweep_{base,cand}.tsv`): every committed results, AnalysisRun or stress-neutral document under P/fixtures and P/tests (49: 16, 17 and 16) gets the same verdict and first error under base and candidate schemas, 35 valid and 14 invalid. **0 differences.**
- **Suites** (`sweep_test_files.txt`, 24 files that touch these schemas, excluding the cargo-building headless-runner test):
  - **Base** (full archive of `844448112f`): 1,726 passed, 22 failed.
    - 19 of the failures are archive-lane artefacts: the handoff tests read execution-record fixtures the archive excludes, and they pass in the worktree.
    - The other 3 are the pin tests (§ Stop).
  - **Candidate:** 1,745 passed and 3 failed, the same 3.
  - **No test that passes at base fails in the candidate, and no new failure appears** (`suites_compare.txt`).
- **The retained schema test file:** 37/37 pass.

**2. F2 closed** (`lane_u6a_derivatives.log`).
- U6a's actual Rust derivatives, from the slice's lane output with 69 class disclosures each, **validate against the candidate `results.v0.3`**.
- The same derivative relabelled to preview-physics-1 with its receipt dropped is refused, because the base branch does not admit the class codes.
- **A finding, not U6c's:** the self-contained dispatcher `results.schema.yaml` inlines an old v0.3 that knows only precision-1. So it refuses every fresh identity, physics-1 and preview-physics-1 included, at base too. Every existing v0.3 test validates against `results.v0.3.schema.yaml` directly, and so do mine (F-U6c-2).

**3. Refusals:**
- every non-successor branch, in all three schemas, refuses a receipt or a class code;
- the successor branches refuse a missing receipt, an invalid receipt (`$ref`), foreign members and mismatched identity, sha or profile.

**4. Nothing weakened.**
- The diff removes no constraint. The removed lines are trailing-comma rewrites in the enum lists.
- No existing test was changed. No reader file and no flag was touched.

**5. Mutants: all 26 are killed** (`mutants.py`, `mutants_final.json`, `mutants_run.log`).
- They cover 4 in `results`, 11 in AnalysisRun and 11 in stress-neutral: dropped codes, per-branch code admission, open receipt refs, dropped requirements and exclusions, swapped sha, profile and enums, and per-branch receipt admission.
- **The first round left two survivors:** A08 and A09, where the successor admitted `source_block_recovery` or `contract_evidence`. The probes used shape-invalid values, so they were refused anyway. They now use shape-valid foreign members, and both mutants are killed.

## Stop: three existing pin tests (outside the fence; ruling needed)

**Which tests:**
- `tests/test_load_reference_schema.py::test_carrier_branches_are_appended_after_the_existing_methods` (T1);
- `tests/test_load_reference_source_schema.py::test_joined_branches_are_appended_with_pinned_identities` (T1);
- `tests/test_source_block_schema_contract.py::test_actual_composite_maximum_metadata_has_a_method_scoped_canonical_route` (T0R/T3).

**Why they fail:**
- **They already fail at `844448112f`, and at NUM since the reader fan-in `c15e64b756`.** That round appended the successor to `results.v0.3`, and these tests pin exactly 7 branches, or "T1's entries are last". The run ROOT verified covered only the retained contract tests, so they went unnoticed.
- **U6c would extend the same pins** to the 8th AnalysisRun and stress-neutral branches, and to the new `not` clause on load-reference-1's AnalysisRun branch. The earlier failing assertions currently mask this.

**The proposed patch** (`_run_records/proposed_pin_tests.diff`, 78 lines, not applied):
- append the successor to the pinned identity order;
- add `{"required": ["retained_precision"]}` to load-reference-1's expected `not`;
- change T1's `[-1]` "last" pins to `[-2:] == [T1's entry, the successor's entry]`;
- set the branch count to 8, with the successor mapped to `PreviewPhysicsResultSet`.

It removes nothing, and every T1 assertion still binds at its identity. **In a lane with the candidate schemas, those three files pass: 787 passed, 11 skipped** (`pinlane.summary`).

**Proposed ruling:** extend U6c's fence to these three test files for this patch alone, with RV88 reviewing it as a pin update rather than a weakening.

## Findings

- **F-U6c-1 (the stop above).** The pin tests have been red since the reader fan-in. Recommend that ROOT's acceptance runs include the 24-file suite listed in `sweep_test_files.txt`.
- **F-U6c-2 (existing behaviour, outside the fence).** `schemas/results.schema.yaml`, the "self-contained strict version dispatcher", has a v0.3 branch that knows only precision-1. It therefore refuses physics-1, preview-physics-1, both load-reference identities and the successor. It has done this since T0R, and every v0.3 test bypasses it. Confirmed at base, where preview-physics-1 and physics-1 documents are refused by the dispatcher and valid under `results.v0.3` (`dispatcher_check.log`). Proposed: route it to its owner (results-document dispatch; T6).
- **F-U6c-3 (scope).** The stress-neutral successor branch admits the **transport shape** only. The Python packager refuses the successor (`SN-SOURCE-METHOD-UNSUPPORTED`, or the source-contract refusal before U6b), so T6's refusal stands, and is tested. Whether T6 later emits successor packages, with the class disclosures in its rows, is T6's question.

## Records and next

- **Records:** `_run_records/` holds the generator script, mutants, sweeps, suite outcomes and comparison, the lane logs, the proposed pin patch and the changed-file hashes, all with placeholder paths. SHA256SUMS covers this folder.
- **Next:** U6b, the Python carriers including D-U6-9, follows in the same worktree, as directed. It does not depend on the pin ruling.
