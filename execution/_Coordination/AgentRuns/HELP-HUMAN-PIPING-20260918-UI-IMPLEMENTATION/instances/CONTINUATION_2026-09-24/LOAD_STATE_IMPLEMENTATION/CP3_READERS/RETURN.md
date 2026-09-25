# CP3_READERS return: load-reference-1 readers

TASK (Type 2) return to the session-2 WORKING_ITEMS load-state manager
(`a3675abb28ada0834`). ROOT (HELP_HUMAN) spawned this TASK. Paths are
WORKING_ROOT-relative; `LSI` is this folder's parent, and `RR` is
`LSI/CP3_READERS/_run_records`. Machine paths appear only under `RR/`.

The brief is `LSI/CP3_READERS/BRIEF.md` (sha256 `d9ab20b4…c174883`, verified).
All eight frozen inputs were verified.

The manager regenerated the raw envelopes (message text only). The new hashes
are verified, and the relayed claim was checked independently: substituting the
CP2 NOT_JOINED message back into each file reproduces the frozen hash exactly,
with two occurrences per file. Everything below uses the new bytes:

| Raw envelope | New sha256 |
|---|---|
| connected-sparse_interactive | `915965a4…9cb74c` |
| connected-dense_scrutiny | `3824035c…dfb2bf` |
| pressure-sparse_interactive | `71be3e4f…e47f134` |
| pressure-dense_scrutiny | `b46eeb8a…f1d0707` |

The table (`44bc41c0…`), both requests and `schemas/load_reference_state.schema.json`
(`640fd447…`, frozen and relayed) are unchanged.

## Outcome

Partial completion. The Rust and Python readers are done and in parity.
Canonical results 0.3 and AnalysisRun 0.3 outputs exist for all four inputs.

**Blocked:** the stress-neutral 0.3 outputs.
- ROOT extended my boundary, at the manager's request, to
  `core/handoff/stress_neutral/package_v0_3.py`. The permitted change was
  additive only: import the ID and add it to `SUPPORTED_METHODS` and
  `PHYSICAL_METHODS`.
- The host permission classifier refused that write ("Modify Shared
  Resources"). It then also refused a hash read of the file.
- The file was not changed and I pursued no other route. It stayed at sha256
  `db128dab…a967ea`, identical to HEAD, recorded in
  `RR/stress_neutral_package_before.sha256`.
- The needed change is still those three additive edits. It needs the owner's
  permission, or another writer.
- Until then, `build_stress_neutral_export_package_v0_3` rejects load-reference-1
  (its `SUPPORTED_METHODS` does not contain it). No stress-neutral fixture was
  produced.

## New API

**Rust `core/reporting/result_export`**
- `semantic_contract::` constants and table functions:
  - `LOAD_REFERENCE_ID`, `LOAD_REFERENCE_PROFILE`, `LOAD_REFERENCE_TABLE_SHA256`;
  - `load_reference_contract()`: `include_bytes`, then sha256, id and profile checked, beside `physics_contract()`;
  - `verify_load_reference_table(bytes)`.
- `semantic_contract::` re-exports of the validators:
  - `validate_load_reference_evidence`;
  - `validate_load_reference_transport_metadata`.
- New `load_reference.rs` (`pub mod load_reference`) with the validator and
  `transport_schema()`. The transport schema is pinned at `640fd447…`.
- `for_source_metadata` accepts load-reference-1 with profile
  `resolved_straight_load_state_v1` only.
  - No other contract accepts that profile; each keeps its own.
  - `carrier_evidence` on load-reference-1 gives `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, the same code as Python.
  - A source receipt gives the existing `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN`.
- `for_source` dispatches load-reference-1 to the new validator.
  - The physics-1 and physics-source-1 arms now reject `contract_evidence.load_reference_states` first with `SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN`.
  - Those inputs were already rejected by the closed physics namespace, so the accept set is unchanged.
- `derivative::derive_document` copies `contract_evidence` verbatim for
  load-reference-1, as for physics-1. The `source_block_recovery` downgrade rule is
  unchanged.

**Python `core/analysis_runs`**
- `compatibility.py` registers:
  - `LOAD_REFERENCE_CONTRACT_ID`, `LOAD_REFERENCE_CONTRACT_SHA256`, `_LOAD_REFERENCE_CONTRACT_PATH`, `LOAD_REFERENCE_PROFILE`.
- The same bindings as Rust hold in `_source_contract`, in both the raw path and the `check_receipt=False` transport path.
- The ID is added to the build and validate sets for the AnalysisRun 0.3 record.
  The record mirrors physics-1: no `contract_evidence` and no `source_block_recovery`.
  The manager confirmed that rule.
- `numerical_use_standing` takes the physics-1 path, as Rust already does.
- New `load_reference_evidence.py`:
  - `validate_load_reference_evidence`;
  - `validate_load_reference_transport_metadata`;
  - `verify_load_reference_table`, `load_reference_table`;
  - `transport_schema`;
  - `LoadReferenceError`.

## Validator design

The pre-pass runs one check order, identical in both languages, and raises
identical error strings `SOURCE_LOAD_REFERENCE_<CHECK>`. It walks arrays in
document order.

| Step | Check |
|---|---|
| S1 | foreign namespaces |
| S2 | finite tree |
| S3–S5 | closed namespace, `connector == []` |
| S6 | numerical case identities |
| S7 | exact-case shape, `material_basis` constant, the 11-key resolved `pipe_materials`, section IDs |
| S8 | record shape and uniqueness |
| S9 | trijection between `numerical_quality.cases`, `exact_cases` and `load_reference_states` when solved; empty namespace when unsolved |
| S10 | per record, below |
| S11 | one projection sha and one requested mode per envelope |
| S12 | region shape; `temperature_basis == "resolved_member_state"`; region material equals the case material minus the two resolved keys, and so equals the member pair |
| S13 | exactly one `info` NOT_JOINED diagnostic per case, by its ID `diagnostic:load-state:<case-suffix>:source-recovery-not-joined` and `affected_refs == [case]`, with no extras; message text is not validated |
| S14 | unchanged physics-1 validator on a projected copy |

S10 checks each record in this order:
1. Contract, profile, geometry, history, solve (mode bound to recovery method, fixed texts), source_recovery.
2. The 34-key members, including:
   - E/nu range and G within 2 ulp;
   - selection-kind rules against the consumed points;
   - reference basis and installation temperature;
   - `stretch == 1 + strain`, exactly in binary64;
   - law and datum presence per thermal definition;
   - indices and segments;
   - fit kind and input: `fit_strain == Δ/L` or `== strain`.
3. Support components: DOF, `global_dof mod 6`, unit, law, node consistency.
4. Contributions: closed shape per `owner_kind`, including the producer's `value` on support_state and `factor: null` on pressure_region.
   - A stored primitive must have a finite nonzero factor and `applied == authored × factor`.
5. Uniqueness of `source_id`; excluded IDs disjoint from included ones.
6. Members form a bijection with `pipe_sections`.
7. `exact_cases.pipe_materials` equals the member E/nu/G, `material_id`, selection kind and `resolved_eigenstrain == total_eigenstrain`, with `thermal_consumed false` and `alpha null`. Provenance must be non-empty text.
8. Contribution ledgers are bijections with their owners: members (value and refs), support components (value) and the case's pressure regions.

S14 details:
- The projection removes `load_reference_states` and the two resolved keys, and
  neutralizes `material_basis` and region `temperature_basis`, all of which the
  pre-pass already bound.
- It reuses the rows, extrema, regions, RHS and headline checks without editing them.
- Failures are wrapped as `SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE: <inherited>`.
  Parity is compared by that leading code, because the inherited Rust and Python
  detail texts already differed before this TASK.

Transport mode:
- first checks shape against `$defs/LoadReferenceContractEvidence` of the frozen schema (hash-pinned, read-only);
- then runs S3–S12;
- then physics-1 transport on the projection;
- skips diagnostics and status.

## Files (sha256)

| File | sha256 | Kind |
|---|---|---|
| `core/reporting/result_export/src/load_reference.rs` | `8148c981…0587` | new |
| `core/reporting/result_export/src/semantic_contract.rs` | `4e7e1a4b…afdcc` | edited; HEAD `a04750cc…` |
| `core/reporting/result_export/src/derivative.rs` | `aca9ba57…f567` | edited; HEAD `c3e5c91e…` |
| `core/reporting/result_export/src/lib.rs` | `9d5578b6…5c82` | edited, one line; HEAD `04f85b53…` |
| `core/reporting/result_export/tests/load_reference_contract.rs` | `23add807…c3ac` | new |
| `core/reporting/result_export/tests/fixtures/load_reference_mutations.json` | `f4ffc827…7168` | new, shared cases |
| `core/analysis_runs/compatibility.py` | `ac779ba2…b9939` | edited; HEAD `5e5083a1…` |
| `core/analysis_runs/load_reference_evidence.py` | `7427bd1d…c8452` | new |
| `tests/test_load_reference_readers.py` | `7405292b…6ff0` | new |
| `fixtures/results/load_reference_connected_sparse.document.json` | `8faf0049…30cf` | canonical results 0.3 (Rust) |
| `fixtures/results/load_reference_connected_dense.document.json` | `2a5806ee…ef86` | canonical results 0.3 |
| `fixtures/results/load_reference_pressure_sparse.document.json` | `4fc66a9e…e237` | canonical results 0.3 |
| `fixtures/results/load_reference_pressure_dense.document.json` | `04c97991…9127` | canonical results 0.3 |
| `fixtures/results/load_reference_connected_sparse.analysis_run.json` | `27e6d3d6…e4` | AnalysisRun 0.3 (Python) |
| `fixtures/results/load_reference_connected_dense.analysis_run.json` | `d235bb21…c714d` | AnalysisRun 0.3 |
| `fixtures/results/load_reference_pressure_sparse.analysis_run.json` | `f8f49fd8…ddb` | AnalysisRun 0.3 |
| `fixtures/results/load_reference_pressure_dense.analysis_run.json` | `d392484d…f1da` | AnalysisRun 0.3 |

Full hashes are in `RR/final_files.sha256`, and the tracked-file diff is in
`RR/tracked_changes.diff`.

How the carriers are built:
- They are compact JSON plus a newline, about 1.1–1.5 MB per document and 0.3–0.4 MB per record.
- The inputs are:
  - base `fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json`;
  - model from the matching request;
  - origin: `attested_headless_producer` test binding;
  - manifest ref `manifest:load-reference-<name>` and hash `1…1`.
- They are regenerated mechanically by `RR/regenerate_carriers.sh`.
- The tests assert byte-for-byte reproduction.

## Commands and logs

- **Environment:** `RR/environment.json`.
  - Python 3.11.15 from the session virtual environment recorded in `RR/environment.json`, jsonschema 4.26.0, pytest 9.1.1.
  - `cargo +1.97.1 … --locked --offline -j 1` with a per-crate `CARGO_TARGET_DIR` outside the repository (recorded in `RR/environment.json`), deleted at the end.
  - The pre-existing checked-JSON and units binaries were supplied through `OPENPIPESTRESS_*_BIN`, so conftest never invoked Cargo outside result_export.
- **Baseline before edits:** `cargo test` in result_export gave 59/59 pass (`RR/baseline_cargo_test.log`).
- **Final:** `cargo test` (all result_export) gave 64/64 pass: 22 unit, 6 derivative, 5 new, 10 physics, 7 physics-source, 4 precision, 10 source-blocks (`RR/final_cargo_test.log`).
- **Python, final:** the new file plus the pre-existing set gave 630 passed, 3 skipped and 2 failed (`RR/python_final_after.log`). The pre-existing set is `test_analysis_run_*.py`, `test_physics_consumer_contract.py`, `test_physics_source_contract.py` and `test_stress_neutral_*.py`, listed in `RR/python_preexisting_files.txt`.
- **Python, HEAD code:** the same pre-existing set in a read-only HEAD mirror gave 417 passed, 3 skipped and the same 2 failed (`RR/python_preexisting_before_head_mirror.log`).
- The new file alone passes 213/213 (`RR/python_load_reference_readers.log`, pre-regeneration). The final run includes it.
- **The two failures exist at HEAD and are unchanged by this TASK:**
  - The tests are `tests/test_stress_neutral_physics_source.py::test_method_namespace_removal_substitution_and_relabel_are_rejected[physics]` and `[source_blocks]`.
  - Original failure text: `ValueError: SN-CSV-PROFILE-MISMATCH` at `core/handoff/stress_neutral/package_v0_3.py:98`, raised from `rehash_packet(packet)` after the relabel to `…/unknown-method`, via `_csv_policy`.
  - The same failure appeared before the attempted package edit (`RR/stress_neutral_tests_before_package_edit.log`: 189 passed, 2 failed) and at HEAD code.

## Existing outputs are byte-identical

- **Rust:** all 204 output files that the existing derivative, physics and
  precision tests emit through `RESULTS_RUST_CONTRACT_OUTPUT_DIR` are identical
  between a HEAD-code mirror build and the new code.
  - Records: `RR/rust_existing_outputs_{before_head,after}.sha256`, `RR/head_mirror_cargo_test.log`, `RR/after_cargo_outputs.log`.
- **Python:** AnalysisRun 0.3 records and stress-neutral 0.3 packages for all 29
  pre-existing sources are identical between HEAD code and the new code.
  - The sources are physics, precision, source-blocks UI, physics-source and the precision `prepare()`.
  - Records: `RR/existing_outputs_{before_head,after}.json`, script `RR/existing_outputs_digest.py`.
- No existing table, fixture, test, oracle or criterion was edited. The
  precision-1, physics-1, source-blocks-1 and physics-source-1 dispatch accept
  sets are unchanged.

## Shared mutation parity (Rust and Python)

- **Case file:** `core/reporting/result_export/tests/fixtures/load_reference_mutations.json`, generated by `RR/make_mutations.py`.
  - The expectations were authored from the wire and the check order, not captured from either reader.
  - Both suites assert every expectation.
- **Outcome logs:** `RR/parity/{rust_outcomes,rust_table_transport_outcomes,python_outcomes}.json`, joined in `RR/parity_summary.json`.
- **Counts:** 201 case IDs (186 raw, 4 table-byte, 11 transport); 387 comparisons.
  - 358 exact;
  - 20 same inherited leading code;
  - 9 declared language-specific;
  - 0 undeclared differences;
  - no case accepted by one language and rejected by the other.
- **Accepted, 18 (both):**
  - `ACCEPT-{connected,pressure}-{sparse,dense}`;
  - `ACCEPT-member-order`, `ACCEPT-contribution-order`, `ACCEPT-support-component-order`, `ACCEPT-record-order`;
  - `ACCEPT-record-provenance-text`, `ACCEPT-not-joined-message-text`;
  - `ACCEPT-source-block-unavailable-info`, an info `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic, tolerated;
  - `ACCEPT-unsolved-empty-namespace`;
  - `TABLEBYTES-pinned`;
  - `TRANSPORT-ACCEPT-{4 sources}`;
  - `TRANSPORT-no-diagnostic-binding`.
- **Rejected, 183 (both).** The mapping from case to code is in
  `RR/parity_summary.json`. By brief category:

| Brief category | Cases | Codes |
|---|---|---|
| unknown key, each level | `UNKNOWN-*` (21) | `EVIDENCE_SHAPE`, `RECORD_SHAPE`, `REFERENCE_GEOMETRY_SHAPE`, `HISTORY_SHAPE`, `SOLVE_SHAPE`, `SOURCE_RECOVERY_SHAPE`, `MEMBER_SHAPE`, `MATERIAL_POINT_SHAPE`, `FIT_BINDING`, `SUPPORT_COMPONENT_SHAPE`, `CONTRIBUTION_SHAPE` (4 kinds), `EXCLUDED_SHAPE`, `CASE_SHAPE`, `MATERIAL_SHAPE`, `REGION_SHAPE`, `REGION_MATERIAL_SHAPE`, `OVERRIDE_SHAPE`, `LAW_SEGMENT_SHAPE` |
| unknown key, inherited | 2 | `PHYSICS_EVIDENCE` |
| missing key | `MISSING-*` (20) | the same shape codes |
| forged E/nu/G and resolved facts | `FORGE-*` (16) | `MEMBER_MATERIAL_BINDING`, `REGION_MATERIAL_BINDING`, `MEMBER_G_BINDING`, `MEMBER_MATERIAL_RANGE`, `STRING_INVALID` (null provenance) |
| cross-case swap | `SWAP-*` (8) | `MEMBER_MATERIAL_BINDING`, `REGION_MATERIAL_BINDING`, `MEMBER_CONTRIBUTION_BINDING`, `NOT_JOINED_DIAGNOSTIC`, inherited `PHYSICS_EVIDENCE` |
| missing or altered not-joined diagnostic | `NOTJOINED-*` (5) | `NOT_JOINED_DIAGNOSTIC` |
| `source_recovery` changed | `RECOVERY-*` (3) | `SOURCE_RECOVERY` |
| source receipt | `RECEIPT-present*` | dispatch `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN`; direct validator `FOREIGN_METHOD_EVIDENCE` |
| receipt inside evidence | `RECEIPT-inside-evidence` | `EVIDENCE_SHAPE` |
| carrier evidence | `CARRIER-present` | dispatch `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| wrong profile | `PROFILE-*` | `SOURCE_FORMULATION_BASIS_UNSUPPORTED` (the direct validator leaves the header to dispatch), `RECORD_PROFILE`, inherited `PHYSICS_EVIDENCE` for `profile_mode` |
| wrong table ID | `TABLE-id-*`, `TABLE-producer-version` | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (load-reference-2, load-reference-source-1, version), `SOURCE_FORMULATION_BASIS_UNSUPPORTED`, `SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN` |
| wrong table hash | `TABLEBYTES-*` (3) | `TABLE_HASH` |
| contract mix-up | `CONTRACT-*` | `EVIDENCE_SHAPE`, `SOURCE_FORMULATION_BASIS_UNSUPPORTED`, `RECORD_CONTRACT` |
| `load_reference_states` on physics-1 | `PHYSICS1-*` (2) | dispatch `SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN` |
| duplicate `source_id` and other duplicates | `DUPLICATE-*` (8) | `CONTRIBUTION_DUPLICATE`, `EXCLUDED_OVERLAP`, `MEMBER_DUPLICATE`, `SUPPORT_COMPONENT_DUPLICATE`, `RECORD_DUPLICATE`, `CASE_DUPLICATE` |
| zero factor | `FACTOR-*` (6) | `CONTRIBUTION_FACTOR` (0, −0.0, null, string, non-null pressure-region factor), `CONTRIBUTION_APPLIED_MAGNITUDE` |
| non-finite number | `NONFINITE-*` (4) | see below |
| other closure | `CLOSED-*` (53) | see `RR/parity_summary.json` |
| inherited physics-1 | `INHERITED-*` (6) | `PHYSICS_EVIDENCE` |
| transport | `TRANSPORT-*` (6 rejections) | `TRANSPORT_SHAPE`, `MEMBER_MATERIAL_BINDING`, `REGION_MATERIAL_BINDING`, `FOREIGN_METHOD_EVIDENCE` |

For `PHYSICS1-*`, the direct validator gives `MATERIAL_BASIS`.

The 9 declared language-specific comparisons (5 cases) are both rejected, with
different codes:

| Cases | Rust | Python | Reason |
|---|---|---|---|
| `NONFINITE-*` (4 cases, dispatch and validator) | JSON text rejected by `serde_json` (`1e400`, `NaN`) | `SOURCE_LOAD_REFERENCE_NUMBER_INVALID` | A non-finite number cannot exist in a Rust `serde_json::Value` (`Number::from_f64(inf)` is `None`), so Rust rejects at the text boundary. |
| `TABLE-id-precision-1` | `SOURCE_FORMULATION_BASIS_UNSUPPORTED` | `SOURCE_PHYSICS_CONTRACT_MISMATCH` | The precision-1 header check order predates this TASK and was left unchanged. |

## Ambiguities, limits and carry-forward

1. **Stress-neutral gap** (see Outcome). The in-memory alternative was not used.
2. The producer behaviours confirmed by the manager are enforced:
   - support_state `value` equals `prescribed_value`;
   - pressure_region `factor: null`;
   - `pipe_materials` carry `material_selection_kind` and `resolved_eigenstrain`, with `thermal_consumed false` and `alpha null`;
   - region `temperature_basis` is `"resolved_member_state"`;
   - provenance must be non-empty text. The producer does not block null material provenance, so such an envelope is rejected with `STRING_INVALID`.
3. **Stricter bindings I derived from the producer code**, beyond the listed
   bindings. They hold on all four envelopes; please review:
   - `stretch == 1 + strain` in binary64 (the producer uses an exact sum rounded half-even);
   - `fit_strain == length_change_m / reference_length_m`, and `== strain`;
   - `applied_magnitude == authored × factor`;
   - member contribution `consumed_input_refs == ["<reference_configuration_id>:<pipe>", "<case>:element_state:<pipe>"]`;
   - one projection sha and one requested mode per envelope;
   - `solve.boundary` and `solve.eigenload` fixed texts;
   - `retained_G_ignored` consistent with the consumed points.
4. `total_eigenstrain` is not recomputed from fit and thermal strain, because
   that needs exact 4-term rounding. It is bound to `resolved_eigenstrain` and to
   the member contribution `value`.
5. An unsolved envelope must carry an empty namespace (`ACCEPT-unsolved-empty-namespace`
   is a transform, not producer output). NOT_JOINED diagnostics are bound only
   when solved.
6. **Downstream consumers not exercised here:**
   - `core/runner/headless` calls `for_source_metadata` and now admits load-reference-1 metadata;
   - desktop and native consumers of result_export are untested (no native builds allowed).
7. The carriers were not validated against the schema TASK's carrier schemas,
   which were not frozen to me. The manager validates them at integration.
8. `rustfmt` is not installed for 1.97.1, so no formatting pass was run.
9. The HEAD mirrors used for before/after evidence were read-only `git archive`
   extracts in the session scratchpad, and were deleted. The branch head moved to
   `d92ca3b7` during the run, but that commit touched none of these paths.
10. No Git writes, browser, native builds, library population or code rules.
    The schemas, `core/product_physics`, the tables and the raw fixtures were
    only read. The cargo target was deleted.
