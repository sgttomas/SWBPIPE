# TASK brief — T1_WP1_JOINED_READERS (readers for `load-reference-source-1`)

Read `_T1_COMMON.md` first. Return folder: `LSI/T1_WP1_JOINED_READERS/`.

## Assignment

Make both evidence readers accept the joined envelope `openpipestress.result_semantics/0.3.0/load-reference-source-1` exclusively. The Rust reader is `core/reporting/result_export`; the Python reader is `core/analysis_runs`. The joined contract is `CP2_WIRE_ADDENDUM_2.md` §5, with the CP4 corrections. The pinned table is `fixtures/results/semantic_contract_v0_3_load_reference_source_1.json` (`d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337`); verify its identity, profile and hash, never infer them.

Model the work on the existing readers:

- the physics-source-1 readers (`result_export/src/physics_source.rs`, `analysis_runs/physics_source.py`), for receipt and source-block validation;
- the load-reference-1 readers (`result_export/src/load_reference.rs`, `analysis_runs/load_reference_evidence.py`), for `contract_evidence.load_reference_states`.

**Required behaviour** (both languages, same order and same codes wherever the two can be made to agree):

- **Dispatch and profile.** The contract dispatches only with profile `resolved_straight_load_state_source_v1`. `source_block_recovery` is required, with receipt policy `LOAD-REFERENCE-SOURCE-1`.
- **Evidence namespace.** `contract_evidence` has exactly `{pressure, connector, exact_cases, load_reference_states}`. `connector` is `[]`. There is one `load_reference_states` record per case, in case order.
- **Per-case consistency** (ADDENDUM_2 §5.3). A `selected` case has method `retained_source_blocks_exact_v1` and recovery method `retained_source_blocks_exact_v1`. It carries `SOURCE_BLOCK_RECOVERY_SELECTED` and no NOT_JOINED record. Every other case is ordinary `checks_passed`, `not_joined`, with its ordinary method. At least one case is selected.
- **Receipt.** Each receipt case's `physical_evidence_sha256` is recomputed over domain `load_reference_source_case_evidence_v1` over `{exact_case, pressure, load_reference_state}`, exactly as the producer does. The receipt's `contract_evidence` binding and envelope identity follow the physics-source-1 checks.
- **Relabels refused.** Refuse a load-reference-1 envelope relabelled as joined, a physics-source-1 envelope relabelled as joined, and the reverse of each. Both existing readers must also still refuse a joined envelope that is presented under their own labels. No pre-existing accept/refuse outcome may change: rerun and compare the existing Rust outputs and Python digests byte for byte, as CP3_READERS did.
- **Derivative and AnalysisRun.**
  - `derivative` carries the joined envelope.
  - `compatibility.py` registers it.
  - AnalysisRun 0.3 records mirror physics-source-1.
- **Carriers.** Produce canonical results 0.3 documents and AnalysisRun 0.3 records from the ten committed raws in `fixtures/product_preview/load_reference_source/*-{sparse_interactive,dense_scrutiny}.raw.json`. Put them in `fixtures/results/load_reference_source_<name>_<mode>.{document,analysis_run}.json`, with a reproducer script in your `_run_records`.
- **Fallback envelopes.** The SF-1 fallback envelopes are ordinary `load-reference-1` and must stay accepted as such. Add one to the load-reference-1 cases: generate it from the committed witness plus the extra tip-UZ load, as in `core/product_physics/src/source_receipt/load_state_fallback_tests.rs`, using `core/product_physics/examples/physics_source_connected.rs`.
- **Shared adversarial cases.** Add a new shared case file `core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json`, read by both suites. It covers:
  - identity, profile and table-hash tampering;
  - namespace keys;
  - selected/not-joined inconsistency;
  - the receipt policy;
  - a physical-evidence hash mismatch;
  - a missing or extra record;
  - a record-order swap;
  - each relabel;
  - numeric edge cases reused from `load_reference_mutations.json`.
  Report parity as CP3_READERS and CP4_READERS did: zero undeclared differences, and no case accepted by one language and refused by the other.

The stress-neutral packager is **not** yours; the manager makes that edit.

## Write boundary

- `core/reporting/result_export/**`, but not its existing `tests/fixtures/load_reference_mutations.json` except to read it.
- `core/analysis_runs/**`.
- `fixtures/results/load_reference_source_*` (new files only).
- `tests/test_load_reference_source_readers.py` (new).
- Your return folder.

## Checks

- result_export: `cargo test`. Baseline 64 plus yours.
- rustfmt clean on the Rust files you touch.
- pytest over your new file plus `tests/test_load_reference_readers.py`, `tests/test_load_reference_schema.py`, `tests/test_physics_source_readers.py` (if present) and the three stress-neutral suites. The CP4 baseline for the five named files is 957 passed, 1 skipped.
- Byte comparison of pre-existing reader outputs.
- Mutants, one per new check, in both languages.
