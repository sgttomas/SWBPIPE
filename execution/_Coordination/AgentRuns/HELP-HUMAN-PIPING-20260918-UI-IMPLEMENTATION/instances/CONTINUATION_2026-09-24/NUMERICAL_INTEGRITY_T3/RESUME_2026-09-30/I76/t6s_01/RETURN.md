# I76 RETURN: T6S-1 (the dispatcher), T6S-2 (the Rust goldens) and RV95 N-5

TASK (Type 2), I76, for ROOT. 2026-10-06 UTC.

**Briefs:** `R/BRIEFS/T6S_COMMON.md` (sha256 `c6ac6c65…`) and `R/BRIEFS/I76_T6S_SCHEMA_TESTS.md` (`86eee696…`). **Basis:** I74's plan `R/I74/t6_slice_plan_01/PLAN.md` (`0350c918…`) and the RR ruling "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched" (decisions 4, 7 and 9).
**Continuation:** ROOT verified `CHECKPOINT_1.md` (`a260fc5c…`) and accepted its three choices: the test-labelled `origin_limit`, `run_hashes: []`, and diagnostics mapped per source. The goldens are unchanged since the checkpoint.

**State.** T6S-1, T6S-2 and N-5's public-API test are done. Everything is uncommitted in `WT/t6-outputs` (branch `codex/piping-t6-successor-outputs-20261005`, base main `c1bfc460fc`). No commits, pushes or other Git writes. Cargo ran only through `WT/tools/t3_cargo.sh` (with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`), with target directories `WT/targets/i76-t6s{,-base,-mut}`. Nothing went to the system temp directory. No DEC-025, native or solver-at-scale job ran.

## 1. Changed files (all within my fence)

| File | Base sha256 | Candidate sha256 | Change |
|---|---|---|---|
| `P/schemas/results.schema.yaml` | `9f2adf6a…` | `d6b5bace42ef5208ae8686dbeee88f9e942aa51b24c504ba2652bacb4dff1d13` | +2 −1,808 lines: `oneOf[2]` becomes `{"$ref": "results.v0.3.schema.yaml"}`, and the description is updated. `oneOf[0]`, `oneOf[1]`, `$defs` and the top-level keywords are byte-identical in value. |
| `P/tests/test_result_export_v0_2.py` | `dd4d2bbf…` | `9605b438d88862d442e99d94db80e48504f132c160b66ab4e36d4bc210a16c48` | +14 −1 lines, in `validator()` only: the validator gets the local schema registry (§3.2) |
| `P/tests/test_results_dispatcher_v0_3.py` | new | `08007e4ddbbb3a2ecd2ea750503376e54f73e4e6a6eef7df0959b3ace7462988` | 244 lines, 23 tests |
| `RE/tests/retained_precision_derivative_golden.rs` | new | `2706d06b561756b530a234e4c488cd341061093f822b45bfccf06e75c99fc798` | 422 lines, 3 tests (checkpoint 1) |
| `RE/tests/source_blocks.rs` | `cc9cc927…` | `00b322c51a35da25b9f552bd9965eb6a30a853dffea4e8a4cc3e4ca1513f73ee` | +88 lines, appended: one test (N-5) |
| `P/fixtures/results/retained_precision_successor_derivative_sparse_interactive.json` | new | `958df02e276538a96c4732302a46cb36a53c2298ca7e1f4deb88a3748c67c661` | golden |
| `P/fixtures/results/retained_precision_successor_derivative_dense_scrutiny.json` | new | `3f9905ad4c4bba688687714751abe965d18cea834701c32c379d81f573dcf682` | golden |

**Nothing else was written in the worktree.** The other entries in `git status` are I75's desktop files and ROOT's `node_modules` link.
- No never-touched file is changed: no `PP/` file, no D1 crate `src/`, no embedded schema, no reader or carrier, no lock file.
- The candidate copy used for the suites (§6) differs from the base archive in exactly these seven files.

## 2. T6S-2: the Rust goldens

As in `CHECKPOINT_1.md` §1–§3. The goldens, the fixed base and origin, and the regeneration command are unchanged. The final regeneration run and the compare run are in `_run_records/regen_final_test.log` and `_run_records/compare_final_test.log`.

**Mutants** (§5, G1–G7): seven of seven are killed.
- Each of the five `derivative.rs` defects is caught by at least one control that does not rely on the golden hash. The exceptions are the byte-level mutants G6 and G7, which only the pins catch, by design.
- G1 (the D-U6-2 message tail shortened) is caught by the independent tail check, as well as by the golden hash.

## 3. T6S-1: the dispatcher (CQ-7 B)

### 3.1 The schema
- **The 0.3.0 arm** is now `{"$ref": "results.v0.3.schema.yaml"}`, as the stress-neutral dispatcher does. A bare `$ref` is enough, because the version file pins `schema_version` and `result_envelope.schema_version` to `0.3.0`.
- **The description** now says that 0.1.0 and 0.2.0 are inline copies and 0.3.0 is a reference that admits exactly what the version file admits.
- **The file stays** `json.dumps(indent=2)` plus a trailing newline, as before.
- **Unchanged:** `$defs` (the v0.1 projection; `test_version_sources_and_projected_legacy_definitions` still passes) and the top-level guard (version enum, constants, `additionalProperties: false`).

### 3.2 Validator construction (`test_result_export_v0_2.py`)
- The 0.3.0 arm and the version file's own external references must now resolve locally.
- **`tests/schema_validation.py` has no function that returns a validator or a registry.** Its registry is built inside `validate_instance`, which only raises `AssertionError`. This test needs a validator object, because `errors()` returns structured error records and the `--outputs` lane writes them.
- So `validator()` builds the same registry as the helper (same resources, same retrieval URLs, no network retrieval) and passes it to `Draft202012Validator`. Nothing else in the file changed. The new test file uses the helper's `validate_instance` for every verdict.
- **For ROOT, item a in §8:** a small helper export would remove this duplication, but it lies outside my fence.

### 3.3 `test_results_dispatcher_v0_3.py` (23 tests, about 37 s)
- **Discovery:** every committed result document (an object with a string `schema_version` and an object `result_envelope`) under `fixtures`, `tests`, `core`, `examples` and `validation`, at any depth. Today there are 18:
  - 16 at 0.3.0: the 14 `load_reference*.document.json` (load-reference-1 and load-reference-source-1) and the 2 goldens;
  - 2 at 0.1.0.
  - No committed 0.2.0, preview-physics-1, physics-1 or precision-1 document exists.
- **Equivalence:** dispatcher verdict = version-file verdict for all 16 committed 0.3.0 documents, and all 16 are admitted. On the base dispatcher the 14 load-reference documents and the 2 goldens are all refused (F-U6c-2).
- **Equivalence on refusal:** 18 variants of 6 documents (the 2 goldens, one load-reference-source document, and 3 synthetic documents), each one of: an unexpected envelope member; `row_accounting` missing; the receipt removed from a successor, or added to a non-successor. Both schemas refuse all 18.
- **Coverage:** the dispatcher and the version file both admit:
  - preview-physics-1 and physics-1, as synthetic scaffolds (`test_load_reference_schema._results_document`) around the committed raw `preview_physics_connected_sparse.json` and `physics_connected_mechanics_sparse.json`;
  - the successor, through both goldens;
  - precision-1, the one identity the old arm knew.
- **The other arms:** both committed 0.1.0 documents and one synthetic, shape-only 0.2.0 instance are admitted. That instance is the first golden's envelope facts with the 0.3.0 members and every row removed; it exists because no committed 0.2.0 document does.
- **Refusals:** mixed and unknown versions (0.3/0.2, 0.3/0.1, 0.2/0.3, 0.1/0.3, 0.4/0.3, 0.3/0.4, 0.4/0.4) are refused. `test_result_export_v0_2.py`'s existing mixed-version refusals still pass, now with the registry.
- **In-file mutants**, each asserted killed:
  - the `$ref` pointed at `results.v0.2`, at `results.v0.1`, or at `analysis_run.v0.3`;
  - each arm dropped in turn (0.3.0, 0.2.0, 0.1.0).
  - A dropped arm is replaced by `{"not": {}}`, not deleted. Deleting it would shift the inline 0.2.0 arm's positional local references (`#/oneOf/1/$defs/…`), and the schema would fail as unresolvable instead of refusing.

### 3.4 Extra evidence: the Rust-captured documents
The `result_export` suites ran with `RESULTS_RUST_CONTRACT_OUTPUT_DIR` set, which captures 116 live Rust derivatives: 88 at 0.2.0, 26 precision-1 and 2 physics-1. Then `test_result_export_v0_2.py --outputs` was run on each side:
- **The captured documents are byte-identical** between the base and candidate `result_export` runs.
- **Base dispatcher:** 2 errors, the two physics-1 documents (F-U6c-2).
- **Candidate dispatcher:** 0 errors on all 116.
- **Under the candidate,** all 28 captured 0.3.0 documents get the same verdict from the dispatcher and the version file.
- Records: `t6s1/rust_captured_documents_dispatcher_errors.tsv` and `t6s1/captured_v03_equivalence.log`.

## 4. RV95 N-5: the public-API masking test

`RE/tests/source_blocks.rs::rv95_n5_the_integer_bound_is_masked_at_the_public_api`, on the synthetic source-blocks control. The control is valid with its invocation (`Ok(true)`).

**Receipt integers.** Twelve fields that `source_blocks::integer` reads are set to 2^53, and in each case:
- the body cannot be resealed: `domain_hash` returns `CHECKED-JSON-UNSAFE-INTEGER: 9007199254740992`;
- `validate(None)`, `validate(Some(invocation))` and `semantic_contract::for_source` all return `SOURCE_BLOCKS_RECEIPT_SHAPE`.

**A correction of detail to I74 §0 item 7.** Not every receipt integer read through `integer` has the schema maximum 2^53−1:
- eight do: case `work.charged`, `work.reserved_unobserved_failure`, `ordinary_attempt.quality_case_index`, `source.dof_count`, `stiffness_term_count`, `force_term_count`, `functional_count`, and an action term's `global_dof`;
- four have tighter maxima: case `work.limit` (4,000,000), and `invocation_work.limit`, `.charged` and `.publication_charged` (64,000,000).

The masking conclusion is unchanged: each one fails the shape first.

**Control at 2^53−1.** For each of the eight widest fields, resealed, the statement passes the shape and both hashes, `integer` admits the value, and a later check refuses it:
- `INVOCATION_WORK_LIMIT` for both case work fields;
- `QUALITY_CASE_BINDING`;
- `SOURCE_COUNTS` for the dof, stiffness and force counts;
- `FUNCTIONAL_COUNT`;
- `SUPPORT_ACTION_OWNER`.

In each case the refusal is never `SOURCE_BLOCKS_INTEGER`. The run log, `n5/n5_test_worktree.log`, prints each code.

**Summary counts.** Each of `node_count`, `segment_count`, `support_count` and `load_case_count` set to 2^53:
- refused with `CHECKED-JSON-UNSAFE-INTEGER: 9007199254740992` at the publication hash, in all three entries (both `validate` calls and `for_source`);
- the control at 2^53−1, resealed, reaches `summary` and fails `SOURCE_BLOCKS_SUMMARY_MODEL_COUNTS`.

**S1 is equivalent at the public API.** RV95's mutant S1 (the `<= 2^53−1` filter removed from `integer`), applied in the scratch mutation copy:
- `--test source_blocks` passes 15 of 15, including the new test;
- the whole `result_export` suite passes 176 of 176.

So S1 survives everything reachable from the public API. That is the masking this test pins.

**Two mutants show the test pins both layers:**
- N1 (the receipt schema's `work.charged` maximum removed) is killed: the error becomes `CHECKED-JSON-UNSAFE-INTEGER` instead of `RECEIPT_SHAPE`.
- N2 (the checked profile's integer bound disabled) is killed: the body then reseals.

**Left for PR-B1, per decision 9:** the direct `#[cfg(test)]` unit test of `integer` in `RE/src/source_blocks.rs`, which is the test that kills S1.

## 5. Mutants (scratch only)

**Setup.** The mutation copy is `WT/scratch/i76_t6s/mut`: the base archive plus my seven files. Each mutant edits one file, runs the named check, and restores the file byte for byte (checked by sha256). Driver: `scripts/mutants.py`. Results: `mutants/summary.jsonl`, with one log per mutant.

| Mutant | Edit | Check | Result |
|---|---|---|---|
| G1 | D-U6-2 absolute message: "…withheld from rule binding" (", and reliance" dropped) | golden test | killed (tail check; golden sha) |
| G2 | receipt not copied | golden test | killed (`RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH`) |
| G3 | b printed `{:.6e}` | golden test | killed (b round-trip `bits ≠`; golden sha) |
| G4 | `contract_evidence` not copied for the successor | golden test | killed (`SOURCE_CONTRACT_EVIDENCE_BINDING_MISMATCH`) |
| G5 | `absolute_verified` rows not disclosed (exported as values) | golden test | killed (0 ≠ 69 class disclosures; golden sha) |
| G6 | one byte of the committed sparse golden | golden test | killed (committed golden ≠ pin) |
| G7 | one byte of the pinned sparse successor | golden test | killed (PP's file pin) |
| S1 | `integer`'s bound removed | `--test source_blocks` | **survives, as expected** (15/15) |
| S1 | the same | whole `result_export` | **survives, as expected** (176/176) |
| N1 | receipt schema `work.charged` maximum removed | `--test source_blocks` | killed |
| N2 | checked-profile integer bound disabled | `--test source_blocks` | killed |
| D1 | dispatcher `$ref` → `results.v0.2.schema.yaml` | dispatcher + v0_2 tests | killed (8 tests fail) |
| D2 | dispatcher 0.3.0 arm → `{"not": {}}` | dispatcher + v0_2 tests | killed (7 tests fail) |
| D3 | the base inline 0.3.0 arm restored | dispatcher + v0_2 tests | killed (6 tests fail; precision-1 coverage passes, as F-U6c-2 predicts) |

**Notes on the mutant runs:**
- **First D pass:** my first wrapper (`run_py.sh`) returned `tail`'s exit code rather than pytest's, so it recorded D1–D3 as "survives". The pytest logs show 8, 7 and 6 failures. I reran D1–D3 with `run_py2.sh`, which differs only by a final `exit $rc`. Both passes are kept in `mutants/pass1_wrapper_exit_bug/`.
- **Not run:** dropping the registry from `test_result_export_v0_2.py`'s `validator()`. Without the registry, jsonschema falls back to retrieving `https://openpipestress.org/schemas/results.v0.3.schema.yaml` over the network, and I ran no network job. Without a registry the reference cannot resolve locally; the test only passes with it.

## 6. Suites: base `c1bfc460fc` against the candidate, test by test

**Copies.**
- **Base:** a disposable `git archive c1bfc460fc` of the whole repository in `WT/scratch/i76_t6s/base`.
- **Candidate for Python:** an APFS clone of the base, plus exactly my seven files, so I75's in-progress desktop files are excluded.
- **Candidate for `result_export`:** the worktree itself. RE does not contain I75's files.
- Both scratch copies get the same `node_modules` link as the worktree.

**`result_export`** (`cargo test --locked --offline --no-fail-fast`):
- base: 172 passed, 0 failed;
- candidate: 176 passed, 0 failed;
- the difference is the 4 added tests (3 golden and 1 N-5); none removed, none changed outcome.
- Records: `suites/result_export_{base,candidate}.log`.

**Python sweep** (`VENV/bin/python -m pytest -q -p no:cacheprovider -rA tests -n 6 --dist loadscope`, as the project's `piping-pytest` runs it):

| | passed | failed | errors | skipped | total |
|---|---|---|---|---|---|
| base | 3,424 | 52 | 64 | 32 | 3,572 |
| candidate | 3,447 | 52 | 64 | 32 | 3,595 |

- **Added:** 23, all `test_results_dispatcher_v0_3.py`, all passed. None removed, none changed outcome.
- **The 116 failures and errors are the same set in both,** and all come from the host setup, not the code:
  - **git (no metadata in archive copies):** `test_qualification_gate` (31), `test_qualification_physics_integration` (6), `test_qualification_load_reference` (1), `test_evidence_sweep` (6, partly cargo), `test_export_public_openpipestress` (3) and `test_coverage_telemetry_script` (1). `GIT_CEILING_DIRECTORIES` stops git from finding a repository above the copy.
  - **direct `cargo` refused by a PATH shim, which logs and exits 97, under the T3 rule:** `test_binary64_canonical_json_adapter` (45; its fixture builds the binary64 CLI), `test_headless_runner_contract` (20), `test_nonlinear_support_regression` (1) and `test_release_candidate_scan` (2).
  - The shim refused the same 41 cargo invocations in each run (`suites/shim_cargo_refusals.log`).
- **Setup.** The checked-JSON and units binaries the conftest needs were built through the lock from the base archive (`suites/checked_json_and_units_build.log`). `core/serialization/canonical_json` and `core/units` are unchanged between base and candidate. `TMPDIR` and `--basetemp` were under scratch, with `PYTHONDONTWRITEBYTECODE=1`.
- **Schema-related files pass on both sides,** including `test_result_export_v0_2` (2), `test_analysis_status_schema` (2, which reads the dispatcher), `test_retained_precision_schema` (54) and `test_load_reference_schema` (527).
- Records: `suites/py_{base,cand}_outcomes.tsv` (ids over 200 characters are shortened with a hash), `suites/py_*_summary.txt` and `suites/python_compare.txt`. The full 13 MB logs and the JUnit XML stay in `WT/scratch/i76_t6s/logs/`.
- **Limit:** this is not the full suite in a real checkout. ROOT's DEC-025 and the hosted CI cover the git- and cargo-dependent tests.

**Non-successor behaviour is unchanged.** My changes touch no export, packaging or reader code. In addition, the 116 Rust-captured derivatives (§3.4) are byte-identical between the base and candidate runs.

## 7. Host-rule notes

- **One Git read without `GIT_OPTIONAL_LOCKS=0`:** a single `git -C WT/t6-outputs diff --stat -- …/results.schema.yaml`. It may have refreshed the index's stat cache. Content, HEAD and refs are unchanged. Every other Git read used `GIT_OPTIONAL_LOCKS=0`.
- **D1-closure files** (`derivative.rs`, `source_blocks.rs`, `canonical_json/src/lib.rs`, `source_block_recovery.schema.json`) were edited **only** in the scratch mutation copy, and each was restored byte for byte after its mutant.
- **The `node_modules` link** in `WT/t6-outputs/P/` is untouched; I leave it for ROOT. The two `node_modules` links in my scratch copies are mine and are scratch.
- **Not written into the worktree:** no `__pycache__` or `.pytest_cache`, and no cargo `target/` (every cargo job used `WT/targets/…`).

## 8. For ROOT to rule on or carry

- **a. The registry helper** (§3.2). Optional: add `registry_validator(schema)` to `tests/schema_validation.py`, which is outside my fence. That would replace the 8 mirrored registry lines in `test_result_export_v0_2.py`'s `validator()`. Until then, both build the same registry.
- **b. I74 §0 item 7, a detail:** four of the receipt integers have maxima tighter than 2^53−1 (§4). The conclusion (S1 equivalent at the public API; the direct unit test with PR-B1) stands. PR-B1's brief may want the same field list.
- **c. Historical generation records:** `fixtures/product_preview/{preview_physics,precision}_fixture_generation.json` list `schemas/results.schema.yaml` at `9f2adf6a…` in their `source_input_files` inventories. No test checks those entries (other schemas there already differ from current main), so I changed nothing. RV101 or GEN-8 may note it.
- **d. No committed 0.2.0 result document exists.** So the dropped-0.2.0-arm mutant is killed in the default suite only by the synthetic shape-only instance. The `--outputs` lane over the Rust capture covers 88 real 0.2.0 documents when it is run.
- **e. CQ-1 parity facts for I75,** unchanged from the checkpoint: Rust `{:e}`, so no `+` and shortest digits; 69 absolute, 0 not_covered.

## 9. Records here (`R/I76/t6s_01/`)

- `CHECKPOINT_1.md`, `inputs/` (4) and `_run_records/{regen,compare}_final_test.log`, all from the checkpoint and unchanged.
- `RETURN.md` (this file).
- `_run_records/`:
  - `suites/`, `t6s1/`, `n5/` and `mutants/` (with `pass1_wrapper_exit_bug/`);
  - `scripts/` (`mutants.py`, `compare.py`, `run_py.sh`, `run_py2.sh`, `shim_cargo`).
- Paths are placeholders (`WT`, `VENV`, `<repo>`, `~`); a grep for home-directory and host-temp prefixes finds none.
- `SHA256SUMS` covers every file except itself.
