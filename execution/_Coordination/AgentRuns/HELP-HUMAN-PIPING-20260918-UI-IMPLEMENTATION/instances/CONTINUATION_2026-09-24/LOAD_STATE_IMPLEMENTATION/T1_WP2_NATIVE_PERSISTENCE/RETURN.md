# RETURN — T1_WP2_NATIVE_PERSISTENCE

- **Role:** TASK (Type 2), reporting to the T1 WORKING_ITEMS manager (load/reference states).
- **Brief:** `TASK_BRIEFS/T1_WP2_NATIVE_PERSISTENCE.md` and `_T1_COMMON.md` (Wave 2).
- **Checkout:** the load-state worktree, branch `codex/piping-load-states-20260925`, HEAD `6810b67ff`. `fe3e2dbbc` is its parent; the only commit between them adds the WP3 brief. No Git writes.
- **Paths:** WORKING_ROOT-relative. Records are in `_run_records/`, with machine paths replaced by `<REPO>`, `<SCRATCH>`, `<SHARED_TARGET>` and `<CARGO_HOME>`.
- **Status:** completed execution. This is not acceptance.

## Files changed

| File | sha256 before | sha256 after |
|---|---|---|
| `apps/desktop/src-tauri/src/model_document_migration.rs` | `2a716bf0e39a09dfc033ce526655cea897ff9701f437a026b9610ff696ab42d7` | `acdd8c925cc219474c4186968ba35a8a7ded328e992606c2bc2025d814754131` |
| `apps/desktop/src-tauri/src/lib.rs` (tests only) | `913173e3edbc533c135e33d2d556ab2ed72a953655c3dad715ea4fd915147cb4` | `8f9281c3184a5c407bf371582d8a601b03617c7b71e68168b10a38a54ce7f6bd` |
| `schemas/model.schema.yaml` | `a47b00e0…01b6c` | unchanged |
| `schemas/project_persistence.schema.yaml` | `babb93fc…b4164` | unchanged |

## What each change does

### `model_document_migration.rs`

- **New constant.** `LOAD_REFERENCE_MODEL_SCHEMA_VERSION = "0.4.0"`.
- **New branch in `evaluate_model_document`.** It is a copy of the 0.3.0 branch. A 0.4.0 document gets:
  - status `current`, with source and target `0.4.0`;
  - `stored_document_current`;
  - no applied migrations;
  - `migrated_document: None`, so the caller keeps the stored or incoming `Value` unchanged.

  The document is never rewritten, re-stamped, down-migrated or upgraded, and nothing is injected into it.
- **Other versions are unchanged.**
  - 0.4.1, 0.5.0 and the rest stay `newer_than_supported`.
  - The only text change is in the refusal detail, which now names "0.3.0 or 0.4.0". The existing `"no down-migration"` pin still holds.
  - 0.3.0, 0.2.0 and the 0.1.0→0.2.0 no-op behave exactly as before.
- **Three new unit tests.** They use the committed 0.4.0 producer fixture `fixtures/product_preview/load_reference/connected.request.json` (invented data):
  - `load_reference_model_is_retained_without_migration_or_invented_state` checks:
    - the status fields;
    - no migrated document;
    - the input `Value` and its serialized bytes are unchanged;
    - a bare 0.4.0 draft gains no keys;
    - the answer is the same on the test chain.
  - `load_reference_neighbours_stay_refused_without_down_migration`: 0.4.1, 0.5.0, 0.3.1 and 1.0.0 are refused.
  - `earlier_documents_open_unchanged_and_carried_load_reference_keys_are_not_normalized`:
    - 0.2.0 and 0.3.0 documents stay current and gain no 0.4.0 key;
    - 0.1.0 walks only the no-op and gains no 0.4.0 key;
    - the fixture's 0.4.0 records, re-stamped as 0.3.0 or 0.2.0, are retained as authored: not stripped and not re-stamped to 0.4.0;
    - re-stamped as 0.1.0, those records survive the no-op byte-for-byte, and only `schema_version` changes (existing DEC-033 behaviour).

### `lib.rs` (a new test helper and two new tests in `mod tests`; no production code changed)

The native persistence and IPC path already carries the model, `mechanics_result` and the other envelope fields as untyped `serde_json::Value`: `SaveLocalProjectRequest`, `prepare_model_document_for_persist`, `upsert_project`, `load_project` and `open_local_project`. So the migration gate was the only refusal point, and there is no whole-model serde error for records the native side does not model. These tests prove that on the real functions.

- `load_reference_model_and_saved_result_round_trip_native_store_byte_exactly`:
  - **Setup:** a file-backed SQLite store, the 0.4.0 fixture model, and the committed `connected-sparse_interactive.raw.json` `load-reference-1` raw result as the saved `mechanics_result`.
  - **Cycle, run twice:** save through `prepare_project_persistence_tuple` + `upsert_project`, close, reopen, `load_project`, then the `open_local_project` evaluation step. The second save passes back the open-time status, as the UI does.
  - **Every cycle asserts:**
    - the stored `model_json` bytes equal the serialized input exactly;
    - the stored `mechanics_result_json` bytes are exact;
    - the ledger is `[]`, with no transition and no rehash;
    - the received model and envelope hashes are kept;
    - the reopened model `Value`, its bytes and its payload hash are unchanged;
    - the raw result hash (`model_payload_hash` of the saved result) is unchanged;
    - the `contract_evidence.load_reference_states` bytes are unchanged.
  - **Then** the reopened document solves (SparseInteractive) on `load-reference-1` and reproduces the saved `load_reference_states` evidence exactly.
- `earlier_documents_persist_unchanged_and_carried_load_reference_keys_block_on_solve`:
  - The committed 0.3.0 `exact_pressure_authoring_model.json` and a 0.2.0 copy of it (without `pressure_contract`) save and reopen with identical bytes. Each keeps its own target version and gains no 0.4.0 key.
  - The 0.4.0 fixture re-stamped as 0.3.0 is saved and reopened exactly as authored.
  - Solving it in both modes gives a blocking `LOAD_STATE_CONTRACT_VERSION_MISMATCH`, and it is not `MECHANICS_SOLVED`.

### Portable schemas: not changed

The brief's condition is not met. `schemas/model.schema.yaml` is the canonical domain model: at the root it allows only `schema_version` and `project`, with `project.models[]` and `unit_system`, and `additionalProperties: false`. `project_persistence.schema.yaml` `ModelPayload` delegates to it. Neither validates the product-preview model document (`document_kind: openpipestress.product_preview.model`) that the native side persists.

Checked with jsonschema: the committed 0.3.0 product document `exact_pressure_authoring_model.json` already fails `model.schema.yaml` with 11 errors, and the 0.4.0 fixture fails with 12, all structural. 0.3.0 has no coverage there either, so adding 0.4.0 branches would add coverage for a document these schemas do not govern. Both files and their tests are untouched.

## Checks

| Check | Result |
|---|---|
| Scratch crate: `model_document_migration.rs` included verbatim through `#[path]`; serde 1.0.228 and serde_json 1.0.149 with `float_roundtrip`, pinned to the src-tauri lock; nothing stubbed. `cargo +1.97.1 test --locked --offline -j 2`, shared target, `CARGO_INCREMENTAL=0` | 14 passed, 0 failed: the 11 existing tests and 3 new ones (`scratch_harness_test.log`; harness `make_harness.sh`, lock `scratch_harness.Cargo.lock`) |
| **Real `apps/desktop/src-tauri`**, full suite. `cargo +1.97.1 test --locked --offline -j 2`, own target under scratch, `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_{DEV,TEST}_DEBUG=0` (no debuginfo, to stay near the disk floor; debug assertions and overflow checks stay on) | lib: 114 passed, 0 failed, 0 ignored (T0R baseline 109, plus 3 migration and 2 persistence tests); main: 0 tests (`src_tauri_build.log`, `src_tauri_test.log`). The only warning in the desktop crate is the existing unused `qualify_rule_mechanics`. |
| `pytest -p no:cacheprovider tests/test_model_schema.py tests/test_persistence_schema.py tests/test_load_reference_schema.py` (venv, `PYTHONDONTWRITEBYTECODE=1`) | 534 passed, 0 skipped (`schema_pytest.log`) |

**Timing note.** The src-tauri run was on migration-module bytes `a6ea7a43…`. After it, one doc-comment line was reworded ("…blocks on solve." became "…when it solves."), giving the final `acdd8c92…`. The scratch harness and the mutation run were repeated on the final bytes. The src-tauri target had already been deleted for disk, so src-tauri was not re-run on the comment-only change.

## Mutation evidence (scratch copy only; `mutants.py`, `mutants.log`)

Each mutant ran on a partial scratch copy: the module plus the one fixture it includes, at the same relative paths. The baseline passes (14/14). All 8 mutants were killed and none survived:

| Mutant | Guard | Killed by |
|---|---|---|
| M1 remove the 0.4.0 acceptance branch | acceptance | `load_reference_model_is_retained…` |
| M2 0.4.0 returns a document with `reference_configurations` dropped | byte-exact retention | same |
| M3 0.4.0 down-migrated to 0.3.0 | retention / no down-migration | same |
| M4 0.4.0 draft gets an injected `reference_configurations: []` | no injection | same |
| M5 the 0.1.0→0.2.0 no-op injects `reference_configurations: []` | no injection (pre-0.4) | `earlier_documents_open_unchanged…`, `published_chain_migrates_0_1_0…` |
| M6 the no-op strips carried 0.4.0 keys | no silent normalization | `earlier_documents_open_unchanged…` |
| M7 0.3.0 carrying 0.4.0 keys is up-stamped to 0.4.0 | no upgrade (D3) | same |
| M8 accept any 0.4.x | acceptance is exact | `load_reference_neighbours_stay_refused…` |

No mutants were run against the `lib.rs` persistence tests. The brief requires scratch-crate mutants only, and a scratch copy of src-tauri would have needed a second full Tauri build near the disk floor.

## Not done or open

1. **Deviation from the brief's evidence limits.** The brief assumed `src-tauri` could not build here. WebKitGTK is now installed, and the manager directed a real run, so `src-tauri` **was** built and its whole suite run on this Linux host. The two `lib.rs` persistence tests could only run in the real crate: the scratch crate cannot link `lib.rs`. The three migration tests ran in both. This is Linux evidence, not the owner's-Mac native witness 1. The macOS app was not built or run.
2. **TS browser mirror (not my write set).** `apps/desktop/src/services/projectService.ts` `migrateModelDocumentLocal` recognises only 0.2.0 plus the 0.1.0 chain. It reports both 0.3.0 and 0.4.0 as `newer_than_supported` in the browser preview; the 0.3.0 case predates this task. If the browser path should mirror the native 0.4.0 acceptance, T1_WP2_DESKTOP_READERS needs to add a 0.4.0 (and presumably 0.3.0) `current` branch with `target_schema_version` equal to the document version. No TS IPC type change is required: the native `model_document_migration` shape is unchanged, and only the `target_schema_version` value can now be `"0.4.0"`.
3. **"Reopen must not show a result as Current when its reference basis changed"** (`T1_PLAN.md` WP2) is not in this brief. Natively the saved result and model are stored and returned byte-exactly, so standing stays with the TS readers and standing logic.
4. `unit_round_trip_summary` (the native project-summary unit signature) does not enumerate the new 0.4.0 quantities, such as expansion-law coefficients and `boundary_motion`. It is informational and does not block, and I left it unchanged as out of scope. Tell me if the summary should list them.

## Design questions

None blocking.

## Manager integration note

- The manager applied `rustfmt` to `model_document_migration.rs` (whitespace-only line wrapping in the new tests; the base file was rustfmt-clean), giving `f3dcb380…`. The scratch harness was re-run on these bytes with the recorded lock: 14 passed.
- The real `src-tauri` suite last ran on `a6ea7a43…`. The later changes (one doc-comment rewording by the author, then rustfmt by the manager) are non-semantic. The full `src-tauri` suite re-runs on the candidate in WP7.
