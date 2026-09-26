# T1_WP1_JOINED_READERS return: readers for `load-reference-source-1`

TASK (Type 2) return to the T1 WORKING_ITEMS manager (`a56f5deb01ca844b0`). ROOT spawned this TASK at the manager's request.

- Paths are relative to WORKING_ROOT. `LSI` is this folder's parent, and `RR` is `LSI/T1_WP1_JOINED_READERS/_run_records`.
- The brief is `LSI/TASK_BRIEFS/T1_WP1_JOINED_READERS.md`, with `_T1_COMMON.md`.
- The basis read was `CP2_WIRE_ADDENDUM_2.md` §5, `CP4_WIRE_ADDENDUM.md`, `CP4_REVIEW_DISPOSITION.md`, `T1_PLAN.md`, and the CP3/CP4 reader returns. The producer's `source_receipt.rs` and `source_receipt/composite.rs` (`case_physical_evidence`, `validate_publication`) were read only.
- The base commit is `d8f0dc4f7` (`RR/base_commit.txt`).
- No Git writes. No write outside the boundary, apart from the one case-file edit that the manager authorised (§5).
- Manager rulings applied during the run:
  - the AnalysisRun mirrors physics-source-1;
  - the write boundary was tightened (source-blocks readers out; standing functions only as ruled);
  - the explicit standing early return was restored (ruling A);
  - the case-file option 1 was taken (ruling B).

## 1. Outcome

**Done.**

- **Readers.** Both readers accept the joined envelope exclusively: identity `openpipestress.result_semantics/0.3.0/load-reference-source-1`, profile `resolved_straight_load_state_source_v1`, receipt policy `LOAD-REFERENCE-SOURCE-1`. The table is verified by sha256 (`d1628194…`), id, profile and policy.
- **Producer envelopes.** All ten committed joined raws are accepted, both raw and as transport metadata, in both languages.
- **Carriers.** 20 files: 10 canonical results 0.3 documents (Rust) and 10 AnalysisRun 0.3 records (Python). The AnalysisRun records mirror physics-source-1: they carry `source_block_recovery` and `contract_evidence` (with `load_reference_states`).
- **SF-1 fallback envelope.** Generated from the witness plus one tip-UZ load, in both modes. It stays ordinary `load-reference-1` and is accepted as such.
- **Shared cases.** 111 raw cases, 6 table cases and 14 transport cases. Parity: 0 undeclared differences, and no case accepted by one language and refused by the other.
- **Pre-existing outputs are byte-identical before and after:**
  - 204 Rust contract outputs;
  - 33 Python digest entries (AnalysisRun, stress-neutral package, standing and transport outcome for every pre-existing raw).
- **One pre-existing expectation changed, by ruling:**
  - `TABLE-id-reserved-source-successor` dispatch goes from `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` to `SOURCE_FORMULATION_BASIS_UNSUPPORTED`.
  - It is still refused. Only the code changes, and identically in both languages.
- **Mutants.** 50 mutants (25 per language), one per new check. All killed.

## 2. Design (same order and codes in both languages)

The Rust reader is `src/load_reference_source.rs`, and the Python mirror is `core/analysis_runs/load_reference_source.py`. Codes carry the prefix `SOURCE_LOAD_REFERENCE_`.

| Step | Check | Code |
|---|---|---|
| J0 | producer id and profile are the joined pair. The direct validator checks this itself, because the projection in J3 overwrites both. | `JOIN_IDENTITY` |
| J1 S1 | `carrier_evidence` absent; `source_block_recovery` present | `FOREIGN_METHOD_EVIDENCE`, `JOIN_RECEIPT_REQUIRED` |
| J1 S2–S9, S11, S12 | the unchanged load-reference-1 pre-pass (finite tree, namespace `{pressure, connector, exact_cases, load_reference_states}`, `connector == []`, trijection with the quality cases, one geometry and one mode, region materials) | existing codes |
| J1 S7 | exact cases carry the physics-source-1 `recovery_method` key | `CASE_SHAPE` |
| J1 S10 | per record: an ordinary record is unchanged; a selected record has `solve.recovery_method == retained_source_blocks_exact_v1` in either mode and `source_recovery == {status: selected, method: retained_source_blocks_exact_v1}` | `SOLVE`, `SOURCE_RECOVERY_SHAPE`, `SOURCE_RECOVERY` |
| J1 S10b | each record's method equals its exact case's `recovery_method`; at least one case is selected | `JOIN_RECOVERY_METHOD`, `JOIN_SELECTION_REQUIRED` |
| J1 S13 | a not-joined case has exactly one info NOT_JOINED (unchanged); a selected case has exactly one info `SOURCE_BLOCK_RECOVERY_SELECTED` with id `diagnostic:source-recovery:<case>:selected` and `affected_refs == [case]`; NOT_JOINED and SELECTED counts match | `NOT_JOINED_DIAGNOSTIC`, `JOIN_SELECTED_DIAGNOSTIC` |
| J2 R1 | `body.policy == LOAD-REFERENCE-SOURCE-1` | `JOIN_RECEIPT_POLICY` |
| J2 R2 | closed physics-source-1 receipt schema, with only the policy constant substituted | `JOIN_RECEIPT_SHAPE` |
| J2 R3, R4 | receipt hash; publication hash (raw only) | `JOIN_RECEIPT_HASH`, `JOIN_PUBLICATION_HASH` |
| J2 R5 | receipt cases, exact cases and records in the same case order | `JOIN_CASE_ORDER` |
| J2 R6 | receipt `selected_method` equals the exact case's `recovery_method`; receipt `requested_mode` equals the record's | `JOIN_RECOVERY_METHOD`, `JOIN_REQUESTED_MODE` |
| J2 R7 | `physical_evidence_sha256` recomputed with domain `load_reference_source_case_evidence_v1` over `{exact_case, pressure, load_reference_state}`, exactly as the producer's `case_physical_evidence` | `JOIN_PHYSICAL_CASE_HASH` |
| J3 | the unchanged physics-source-1 validator runs on a projected copy. Raw uses `physics_source::validate(_, None)`; transport uses `validate_transport_metadata`. Failures are wrapped. | `JOIN_PHYSICS_SOURCE: <inherited>` |

**The J3 projection.**
- It removes or neutralises only what J1 and J2 bound:
  - removes `load_reference_states`;
  - removes the two resolved-member keys;
  - sets `material_basis` and region `temperature_basis` to the physics constants;
  - sets producer id, profile and policy to the physics-source-1 values.
- It then re-derives the physics-source-1 case hashes, publication hash and receipt hash of the projected bytes.
- Every received hash is verified in J2 first, so the re-derivation cannot hide a change.
- The existing physics-source-1, source-blocks and physics-1 code is reused unedited. `source_blocks.rs` and `source_blocks.py` are untouched.

**The pre-pass is shared by parameter.**
- `load_reference.rs` gains `Method {LoadReference, Joined}` and `prepass(source, raw, method)`. `validate` is now `prepass(LoadReference)` plus the unchanged S14.
- Python mirrors this with `_prepass(source, raw, method)`.
- In `LoadReference` mode every branch is the prior code:
  - S13 walks the same records in the same order;
  - the transport schema check is unchanged;
  - `CASE_KEYS` is unchanged.

**Transport** does not use the frozen `schemas/load_reference_state.schema.json`, because it admits only not-joined records. Transport runs J0, J1 (without S13), J2 (without R4) and the physics-source-1 transport on the projection.

**Numerical standing.**
- A valid joined envelope is `needs_recompute` in both languages; it is never eligible.
- The reader cannot re-derive a 0.4.0 resolved case from a captured request. The physics-source-1 analogue is `actual_materials`.
- No invocation-based eligibility path was added (see §9).

## 3. Files (sha256 before → after)

### Edited

| File | Before | After |
|---|---|---|
| `core/reporting/result_export/src/load_reference.rs` | `5f9dac91…80add067` | `650ae942…4828d8b5` |
| `core/reporting/result_export/src/semantic_contract.rs` | `f51f1183…c4a61ba` | `75683560…ce6c89c9` |
| `core/reporting/result_export/src/derivative.rs` | `aca9ba57…daef6f567` | `06a02c90…0ce5b` |
| `core/reporting/result_export/src/lib.rs` | `9d5578b6…a0da5c82` | `212791d8…7297b2` |
| `core/analysis_runs/load_reference_evidence.py` | `f7d50f1c…5ba08b5a` | `eff1fb3b…c08ffe` |
| `core/analysis_runs/compatibility.py` | `ac779ba2…a78b9939` | `c71cf01d…2684db33` |
| `core/reporting/result_export/tests/fixtures/load_reference_mutations.json` | `ef3c1164…8bfc5824` | `17632db8…489ea5` (ruling B; §5) |

### New

| File | What it is |
|---|---|
| `core/reporting/result_export/src/load_reference_source.rs` | Rust joined reader |
| `core/analysis_runs/load_reference_source.py` | Python joined reader |
| `core/reporting/result_export/tests/load_reference_source_contract.rs` | Rust tests (5) |
| `tests/test_load_reference_source_readers.py` | Python tests |
| `core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json` | shared case file (generated by `RR/make_cases.py`) |
| `core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json` | SF-1 fallback request (`RR/make_fallback_request.py`) |
| `core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-{sparse_interactive,dense_scrutiny}.raw.json` | producer output of `core/product_physics/examples/physics_source_connected.rs` |
| `fixtures/results/load_reference_source_<name>_<sparse|dense>.document.json` (10) | canonical results 0.3 (Rust derivative) |
| `fixtures/results/load_reference_source_<name>_<sparse|dense>.analysis_run.json` (10) | AnalysisRun 0.3 (Python) |

- Full hashes are in `RR/final_files.sha256`.
- The tracked diff is `RR/tracked_changes.diff`.
- The reproducer is `RR/regenerate_carriers.sh`. The tests assert byte reproduction of every carrier.
- **Fallback provenance.** The build of the same producer example reproduces the committed `eigen_motion-{sparse,dense}` joined raws byte for byte (control).
- The fallback outputs carry `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` ("captured replay reservation … Exact(Budget)"), `retained_source_attempt=unavailable` and quality `sensitive`.
- `lib.rs`: the one added line is `pub mod load_reference_source;`. rustfmt also reordered the pre-existing module declarations (`derivative` before `semantic_contract`, `mod physics_evidence` after `load_reference_source`). That file was not rustfmt-clean at the base.

## 4. Hunks in `semantic_contract.rs` and the Python standing/compatibility code

### `semantic_contract.rs`

1. **Constants.** `LOAD_REFERENCE_SOURCE_ID`, `_PROFILE` and `_TABLE_SHA256` are added after `LOAD_REFERENCE_TABLE_SHA256`. They re-export the `load_reference_source` constants.
2. **Table.** `verify_load_reference_source_table(bytes)` and `load_reference_source_contract()` are added: `include_bytes!` of the pinned table, then verify.
3. **`for_source_metadata`, enumeration.** `LOAD_REFERENCE_SOURCE_ID` is added to the admitted producer ids.
4. **`for_source_metadata`, carrier.** The carrier refusal (`SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`) now covers `LOAD_REFERENCE_ID || LOAD_REFERENCE_SOURCE_ID`. The comment is reworded to the plural.
5. **`for_source_metadata`, receipt.** The receipt-downgrade exemption adds `LOAD_REFERENCE_SOURCE_ID`, because the joined envelope requires the receipt.
6. **`for_source_metadata`, profile.** `Some(LOAD_REFERENCE_SOURCE_ID) => LOAD_REFERENCE_SOURCE_PROFILE` (exclusive).
7. **`for_source_metadata`, table.** `Some(LOAD_REFERENCE_SOURCE_ID) => load_reference_source_contract()`.
8. **`for_source`.** A new arm: `Some(LOAD_REFERENCE_SOURCE_ID) => validate_load_reference_source_evidence(source)?`.
9. **Re-exports.** `pub use crate::load_reference_source::{validate_load_reference_source_evidence, validate_load_reference_source_transport_metadata}`.
10. **Standing (ruling A).** In `numerical_use_standing_with_context`, one early `return "needs_recompute"` when `producer.semantic_contract_id == LOAD_REFERENCE_SOURCE_ID`. Nothing else in any standing, Current, rule or export function changed.
    - **Rationale.** It is conservative. A reader cannot re-derive the 0.4.0 resolved case. The outcome is identical to the fall-through: an admitted joined envelope's aggregate status is never `checks_passed`, because its selected case needs a sensitive or rejected ordinary attempt.
    - **Placement.** It sits immediately after the existing `for_source` validation line, not before it. A refused joined envelope therefore stays `unsupported`, which the shared cases pin, rather than `needs_recompute`.

### `core/analysis_runs/compatibility.py`

1. **Constants.** `LOAD_REFERENCE_SOURCE_CONTRACT_ID`, `_SHA256`, `_PATH` and `LOAD_REFERENCE_SOURCE_PROFILE`.
2. **Contract sets.** `LOAD_REFERENCE_SOURCE_CONTRACT_ID` is added to the four existing contract sets:
   - `_build_analysis_run` version check;
   - `_source_contract` producer set;
   - `build_analysis_run` dispatch;
   - `validate_analysis_run_v0_3` check.
3. **`_build_analysis_run`.** It copies `source_block_recovery` and `contract_evidence` for the joined contract, mirroring physics-source-1.
4. **`_source_contract`.**
   - `joined` flag;
   - joined exempt from `SOURCE_PHYSICS_CONTRACT_MISMATCH` and from the receipt-downgrade refusal;
   - profile `LOAD_REFERENCE_SOURCE_PROFILE`;
   - a joined branch that verifies the table, then runs the raw or transport validator.
5. **`validate_analysis_run_v0_3`.** It binds `source_block_recovery` and `contract_evidence` for the joined contract, mirroring physics-source-1.
6. **Standing (ruling A).** In `numerical_use_standing`, one early `return "needs_recompute"` for the joined contract, immediately after the existing `_source_contract` try block. The rationale and placement are as for Rust. The fall-through (`contract not in {PRECISION, PHYSICS, LR}`) also gives `needs_recompute`.

`derivative.rs` adds the joined id to the `contract_evidence` copy and to the `source_block_recovery` copy and binding lists (3 hunks).

## 5. The one changed pre-existing expectation (ruling B)

- **Case.** `load_reference_mutations.json`, `TABLE-id-reserved-source-successor`: a load-reference-1 envelope relabelled with producer id `load-reference-source-1`, keeping the LR profile.
- **Old:** `"dispatch": "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"`. **New:** `"dispatch": "SOURCE_FORMULATION_BASIS_UNSUPPORTED"`, plus `"note": "D1 activated load-reference-source-1; the LR profile under the joined ID is now refused at the formulation basis"`.
- **`validator` stays `"accept"`.** The ruling said to change it too, but the direct LR validator leaves the header to dispatch, and it still accepts, unchanged. Setting it to `SOURCE_FORMULATION_BASIS_UNSUPPORTED` would assert something false.
- No other case, key, description or section changed (checked by parsed comparison). Reader code was not bent to the old code.
- **Cause.** D1 activation. The id is no longer reserved, so the producer check admits it and the exclusive profile check refuses it. Both languages change identically.

## 6. Checks

| Check | Result | Log |
|---|---|---|
| `cargo +1.97.1 test --locked --offline -j 2` (result_export) | 69/69 (baseline 64 + 5 new): 22 unit, 6 derivative, 5 load-reference, 5 load-reference-source, 10 physics, 7 physics-source, 4 precision, 10 source-blocks | `RR/final_cargo_test.log`; baseline `RR/baseline_cargo_test.log` |
| `rustfmt +stable --edition 2021 --check` on every touched Rust file | clean (rustfmt 1.8.0) | `RR/rustfmt_check.log` |
| pytest: the new file, `test_load_reference_readers.py`, `test_load_reference_schema.py`, `test_physics_source_contract.py` and the 3 stress-neutral suites (`test_physics_source_readers.py` does not exist) | 1177 passed, 1 skipped, 0 failed. The new file is 147 of these, all passed (its parity recorder ran). The skip is the load-reference-1 parity recorder (`LOAD_REFERENCE_PARITY_OUT` unset), as in CP4. | `RR/python_final.log` |
| Existing Rust outputs (`RESULTS_RUST_CONTRACT_OUTPUT_DIR`) | 204/204 files byte-identical before and after | `RR/rust_existing_outputs_{before,after}.sha256` |
| Existing Python outputs (33 entries: AnalysisRun and stress-neutral digests, standing, transport) | identical | `RR/existing_outputs_{before,after}.json`, `RR/existing_outputs_digest.py` |

- The Python count for the CP4 five-file set (baseline 957 passed, 1 skipped) is included in the combined run, together with `test_physics_source_contract.py`. The new file has 147 tests: 111 raw, 6 table and 14 transport cases, plus 16 others.
- Before ruling B, the same run gave 1176 passed, 1 skipped, 1 failed. The failure was the §5 case (`TABLE-id-reserved-source-successor`); the Rust load-reference test failed on the same case. Both pass after the ruling.

## 7. Parity (`RR/parity_summary.json`, `RR/parity/`)

- 131 case ids, 271 Rust/Python comparisons:
  - 263 exact, including every inherited `JOIN_PHYSICS_SOURCE` detail string, which also agreed exactly;
  - 8 declared language-specific (4 cases × dispatch/validator);
  - 0 undeclared;
  - 0 accepted by one language and refused by the other.
- **The declared cases** are `NUM-NONFINITE-member-E`, `NUM-NONFINITE-region-pressure`, `NUM-N2-integer-overflow-positive` and `NUM-N2-integer-first-overflow`. They are reused from `load_reference_mutations.json`. Rust refuses them at the JSON text boundary (`JSON_PARSE_REJECTED`); Python gives `SOURCE_LOAD_REFERENCE_NUMBER_INVALID`. This is the same declaration as CP3 and CP4.
- **Brief categories covered:**

| Brief category | Cases |
|---|---|
| identity, profile and table-hash tampering | `ID-*`, `PROFILE-*`, `TABLEBYTES-*` |
| namespace | `NS-*` |
| selected/not-joined inconsistency | `SEL-*` |
| diagnostics | `DIAG-*` |
| receipt policy and shape | `RCPT-*` |
| hash mismatch | `HASH-*`, `PUB-*` |
| missing, extra or swapped record | `REC-*`, `EXACT-order-swap-resealed`, `RCPT-case-order-swap-resealed` |
| every relabel in both directions, including the joined envelope under both existing labels | `RELABEL-*` |
| numeric edge cases (N3 segments, N2 negative and overflow, non-finite) | `NUM-*` |
| bindings | `BIND-*` |
| inherited physics-source | `PS-*` |
| accepted | 10 raws, 2 fallback raws, and 4 free edits (message and provenance text, the integer-zero literal) |

- **The new `reseal` op.** Both harnesses implement it. It recomputes the physical, publication and receipt hashes in that order, so that checks after the receipt can be reached. It has variants for the joined, record-less and physics domains, which show that the domain and the record binding matter.
- **Authoring corrections.** Expectations were authored from the wire and the check order. Two were wrong on the first run, and both were corrected in `make_cases.py`:
  - The fallback envelopes under the direct joined validator give `JOIN_IDENTITY`, not `JOIN_RECEIPT_REQUIRED`, because J0 comes first.
  - A resealed extremum value is not detectable in transport, because there are no raw rows. It is kept as an accepted transport case that documents the limit. A resealed endpoint-link change is used for the inherited transport refusal.
  - Neither correction exposed a reader bug.

## 8. Mutation evidence (scratch copy; the worktree was never mutated)

- **Scratch copy.** A tar copy of `core`, `fixtures`, `schemas` and `tests`, without `target` or `__pycache__`, was byte-checked against the worktree for all seven touched code files before mutating.
- **Run.** `RR/mutants.py` requires each anchor to occur exactly once, applies one mutant at a time, runs the maintained test (Rust `--test load_reference_source_contract`; Python `tests/test_load_reference_source_readers.py -x`), restores the file and byte-checks the restore.
- **Records.** Logs are in `RR/mutants/`, the summary is `RR/mutants/summary.txt`, and the pre/post hashes are `RR/mutation_{pre,post}hash.txt`.
- **Result.** 50 of 50 killed.

| Mutant | Result | First failing test (Rust panic site or Python FAILED id) |
|---|---|---|
| RS-J0-IDENTITY | KILLED | `ACCEPT-lr-fallback-sparse: joined SOURCE_LOAD_REFERENCE_JOIN_RECEIPT_REQUIRED != SOURCE_LOAD_REFERENCE_JOIN_IDENTITY` |
| RS-S1-RECEIPT-REQUIRED | KILLED | `"TRANSPORT-load-reference-relabel": SOURCE_LOAD_REFERENCE_CASE_SHAPE` |
| RS-S7-RECOVERY-METHOD-KEY | KILLED | `called 'Result::unwrap()' on an 'Err' value: "SOURCE_LOAD_REFERENCE_CASE_SHAPE"` |
| RS-S10-SELECTED-RECOVERY | KILLED | `SEL-selected-status-joined: dispatch SOURCE_LOAD_REFERENCE_JOIN_PUBLICATION_HASH != SOURCE_LOAD_REFERENCE_SOURCE_RECOVERY` |
| RS-S10-SELECTED-MODE | KILLED | `SEL-unknown-mode: dispatch SOURCE_LOAD_REFERENCE_JOIN_PUBLICATION_HASH != SOURCE_LOAD_REFERENCE_SOLVE` |
| RS-S10B-RECOVERY-METHOD | KILLED | `SEL-record-ordinary-exact-selected: dispatch SOURCE_LOAD_REFERENCE_JOIN_SELECTION_REQUIRED != SOURCE_LOAD_REFERENCE_JOIN_RECOVERY_METHOD` |
| RS-S10B-SELECTION-REQUIRED | KILLED | `"TRANSPORT-no-selection": SOURCE_LOAD_REFERENCE_JOIN_RECOVERY_METHOD` |
| RS-S13-SELECTED-HIT | KILLED | `SEL-ordinary-case-claims-selected-with-exact-case: dispatch SOURCE_LOAD_REFERENCE_NOT_JOINED_DIAGNOSTIC != SOURCE_LOAD_REFERENCE_JOIN_SELECTED_DIAGNOS` |
| RS-S13-SELECTED-COUNT | KILLED | `DIAG-selected-duplicate: dispatch SOURCE_LOAD_REFERENCE_JOIN_PUBLICATION_HASH != SOURCE_LOAD_REFERENCE_JOIN_SELECTED_DIAGNOSTIC` |
| RS-R1-POLICY | KILLED | `"TRANSPORT-policy": SOURCE_LOAD_REFERENCE_JOIN_RECEIPT_HASH` |
| RS-R2-SHAPE | KILLED | `RCPT-extra-body-key: dispatch SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE: PHYSICS_SOURCE_RECEIPT_SHAPE != SOURCE_LOAD_REFERENCE_JOIN_RECEIPT_SHAPE` |
| RS-R3-RECEIPT-HASH | KILLED | `"TRANSPORT-receipt-hash": accept` |
| RS-R4-PUBLICATION-HASH | KILLED | `ID-producer-version: joined SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE: SOURCE_BLOCKS_PRODUCER != SOURCE_LOAD_REFERENCE_JOIN_PUBLICATION_HASH` |
| RS-R5-CASE-ORDER | KILLED | `REC-order-swap-resealed: dispatch SOURCE_LOAD_REFERENCE_JOIN_PHYSICAL_CASE_HASH != SOURCE_LOAD_REFERENCE_JOIN_CASE_ORDER` |
| RS-R6-RECOVERY-METHOD | KILLED | `RCPT-selected-method-ordinary: dispatch SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE: PHYSICS_SOURCE_SOURCE_SELECTION_REQUIRED != SOURCE_LOAD_REFERENCE_J` |
| RS-R6-REQUESTED-MODE | KILLED | `SEL-selected-other-mode: dispatch accept != SOURCE_LOAD_REFERENCE_JOIN_REQUESTED_MODE` |
| RS-R7-PHYSICAL-HASH | KILLED | `"TRANSPORT-physical-hash": accept` |
| RS-R7-RECORD-BINDING | KILLED | `called 'Result::unwrap()' on an 'Err' value: "SOURCE_LOAD_REFERENCE_JOIN_PHYSICAL_CASE_HASH"` |
| RS-J3-PROJECTED-RAW | KILLED | `PUB-row-value-resealed: dispatch accept != SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE` |
| RS-J3-PROJECTED-TRANSPORT | KILLED | `"TRANSPORT-endpoint-link-resealed": accept` |
| RS-TABLE-HASH | KILLED | `assertion 'left == right' failed: "TABLEBYTES-id-edit"` |
| RS-DISPATCH-PROFILE | KILLED | `called 'Result::unwrap()' on an 'Err' value: "SOURCE_FORMULATION_BASIS_UNSUPPORTED"` |
| RS-DISPATCH-CARRIER | KILLED | `NS-carrier-evidence: dispatch SOURCE_LOAD_REFERENCE_FOREIGN_METHOD_EVIDENCE != SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| RS-DISPATCH-VALIDATE | KILLED | `assertion failed: s::for_source(&header).is_err()` |
| RS-DERIVATIVE-RECEIPT | KILLED | `called 'Result::unwrap()' on an 'Err' value: "SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH"` |
| PY-J0-IDENTITY | KILLED | `test_shared_adversarial_case[ACCEPT-lr-fallback-sparse]` |
| PY-S1-RECEIPT-REQUIRED | KILLED | `test_shared_adversarial_case[RELABEL-load-reference-as-joined]` |
| PY-S7-RECOVERY-METHOD-KEY | KILLED | `test_shared_adversarial_case[ACCEPT-n05-sparse]` |
| PY-S10-SELECTED-RECOVERY | KILLED | `test_shared_adversarial_case[SEL-selected-status-joined]` |
| PY-S10-SELECTED-MODE | KILLED | `test_shared_adversarial_case[SEL-unknown-mode]` |
| PY-S10B-RECOVERY-METHOD | KILLED | `test_shared_adversarial_case[SEL-record-ordinary-exact-selected]` |
| PY-S10B-SELECTION-REQUIRED | KILLED | `test_shared_adversarial_case[SEL-no-case-selected]` |
| PY-S13-SELECTED-HIT | KILLED | `test_shared_adversarial_case[SEL-ordinary-case-claims-selected-with-exact-case]` |
| PY-S13-SELECTED-COUNT | KILLED | `test_shared_adversarial_case[DIAG-selected-duplicate]` |
| PY-R1-POLICY | KILLED | `test_shared_adversarial_case[RCPT-policy-physics-source]` |
| PY-R2-SHAPE | KILLED | `test_shared_adversarial_case[RCPT-extra-body-key]` |
| PY-R3-RECEIPT-HASH | KILLED | `test_shared_adversarial_case[RCPT-receipt-hash]` |
| PY-R4-PUBLICATION-HASH | KILLED | `test_shared_adversarial_case[ID-producer-version]` |
| PY-R5-CASE-ORDER | KILLED | `test_shared_adversarial_case[REC-order-swap-resealed]` |
| PY-R6-RECOVERY-METHOD | KILLED | `test_shared_adversarial_case[RCPT-selected-method-ordinary]` |
| PY-R6-REQUESTED-MODE | KILLED | `test_shared_adversarial_case[SEL-selected-other-mode]` |
| PY-R7-PHYSICAL-HASH | KILLED | `test_shared_adversarial_case[HASH-physical-zero]` |
| PY-R7-RECORD-BINDING | KILLED | `test_shared_adversarial_case[ACCEPT-n05-sparse]` |
| PY-J3-PROJECTED-RAW | KILLED | `test_shared_adversarial_case[PUB-row-value-resealed]` |
| PY-J3-PROJECTED-TRANSPORT | KILLED | `test_transport_metadata_case[TRANSPORT-endpoint-link-resealed]` |
| PY-TABLE-HASH | KILLED | `test_table_bytes_are_pinned[TABLEBYTES-id-edit]` |
| PY-DISPATCH-PROFILE | KILLED | `test_shared_adversarial_case[ACCEPT-n05-sparse]` |
| PY-DISPATCH-VALIDATE | KILLED | `test_shared_adversarial_case[PROFILE-record-joined-profile]` |
| PY-ANALYSIS-RUN-EVIDENCE | KILLED | `test_shared_adversarial_case[ACCEPT-n05-sparse]` |
| PY-ANALYSIS-RUN-RECEIPT | KILLED | `test_shared_adversarial_case[ACCEPT-n05-sparse]` |

- **Bytes.** Ruling A's two standing hunks were restored after the scratch copy was made. The mutated files therefore differ from the final bytes only by those hunks. No mutant anchors on them, and the standing lines are exercised by the maintained tests: `needs_recompute` for accepted cases, `unsupported` for refused ones.

## 9. Not done, limits and design questions

1. **No invocation-based eligibility for the joined contract.** Physics-source-1 grants `numerically_eligible` only with a captured invocation, from which it re-derives E/ν/α (`actual_materials`). For 0.4.0 that would need a reader-side resolver re-derivation (thermal laws, fit, selection) in both languages, so the joined readers never grant eligibility. **Question:** is a reader-side resolved-case re-derivation wanted, or is `needs_recompute` the accepted standing for joined results?
2. **Direct-validator header scope.** J0 checks only the semantic contract id and profile. Producer component name and version are left to dispatch, as the load-reference-1 validator does. A changed header is still caught by the publication hash (`ID-producer-version`).
3. **Frozen `load_reference_state.schema.json`.** It admits only not-joined records and exact cases without `recovery_method`. I reported this to the manager; the schema TASK was ruled to add new `LoadReferenceSource*` defs. The joined transport reader does not use the frozen schema.
4. **Schema validation of the carriers.** Not done here; the schema TASK and the manager validate the 20 carriers against the new branches.
5. **Stress-neutral packager.** Not mine (manager's edit). No stress-neutral fixture was produced for the joined contract.
6. **Downstream consumers of `for_source_metadata`** (headless runner, desktop) were not exercised. They now admit joined metadata.
7. **The pre-existing `lib.rs` rustfmt drift** was normalised (module order only).
8. **Cleanup.** The scratch space and the cargo target were deleted at the end.
