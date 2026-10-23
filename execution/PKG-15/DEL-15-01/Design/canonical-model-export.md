# Canonical model export and handoff manifest (W-6)

**Basis:** `origin/main` at `9d76659e02` (2026-10-23), after model document 0.7.0
(DEL-02-01 Design `model-document-0.7.0.md`). Design for review before
implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings and requirements

- **OD-1 (owner, 2026-10-23):** canonical model export is in v0.1. Handoff
  (PKG-15) stays in for model export. Further export formats (PKG-17) are v0.2.
- **OD-4 (owner, 2026-10-23):** the desktop's model document is canonical.
- **FR-HAND-001 (Must):** "User can export canonical model data for the
  downstream modeling engine."
- **FR-HAND-002 (Must):** "Export includes model hash, units, entity IDs,
  library references, rule references, warnings, and unresolved assumptions."
- **PRD §16.1:** the package holds the canonical internal model file, a units
  manifest, an entity ID manifest, a rule-pack reference manifest, the model
  hash and the export hash.
- **DEC-028 (2026-06-11):** a multi-member archive with a manifest and
  per-member JCS hashes. The owner closed the compatibility window: one package
  version.
- **DEC-057:** the extension is `.opsproj`.
- **DEL-15-01 R01–R13.**
- **PRD §20 data boundary.**

## 1. Where things stand

The completeness map records the gap, with evidence:
- There is no model save-to-file. The only native writers are
  `save_report_package` (a Save dialog, `.opsproj`, `atomic_save_bytes`;
  `apps/desktop/src-tauri/src/lib.rs:180-236`) and `save_local_result_json`
  (Downloads only).
- The handoff download (`HandoffPanel.tsx:209-283`) is a preview document:
  - it does not conform to `schemas/handoff_package.schema.json`;
  - it has no model hash and hard-codes `rule_pack_refs: []`;
  - it is offered only with a solved pressure-free result.
- There is no producer of a schema manifest. `core/handoff/exporter/workflow.py`
  consumes an already-built package.

## 2. Two commands, two slices

### W-6a: Save Model Document… (FR-HAND-001, FR-MOD-001)

**What it is.** A File menu command, Save Model Document…, available whenever
a model is open. It does not need a solve or a result, and it works on either
route.
- **New Rust command `save_model_document_file`.** It follows the
  `save_report_package` pattern: a Rust-side `blocking_save_file` dialog with
  the filter "SWBPIPE model document" (`json`), then `atomic_save_bytes`.
- **The saved bytes are the canonical JSON of the session's model document:**
  RFC 8785 JCS via `core/serialization/canonical_json`, the same canonical form
  the session's model hash covers. One trailing newline is added.
  - **Consequence:** sha256 of the file without the newline equals the
    session's model hash value.
  - Saving twice gives identical bytes. Re-opening the file
    (`open_model_document_file`) gives the same model hash.
  - Readability is traded for that identity. The file is one line.
- **Refusals before writing:**
  - the version is not current;
  - the desktop-profile shape check fails (`check_model_document_for_persist`);
  - the bytes exceed the 64 MiB open cap
    (`model_document_file.rs:23`), so a saved file always re-opens.
- **Data boundary.** A model document is the user's private project data. Saving
  it to a path the user picks in the dialog is the `local_private` context
  with explicit intent; the dialog is the intent. It gets a new governed route
  `DOTH-MODEL-DOC-005` in `GOVERNED_LOCAL_FIRST_ROUTE_IDS` and the route
  registry. Nothing is redacted: the file is the model, exact (FR-MOD-001
  "without loss"). No network, no telemetry, no default path inside the
  repository (F-PIP-1).
- **Outcome.** `Saved{file_name, byte_count, model_hash}` or
  `Refused{diagnostics}`. A cancel is not an error. The session's model, hash
  and solve standing are unchanged.

### W-6b: Save Handoff Package… (FR-HAND-001/002, PRD §16.1)

**What it is.** A File menu command, Save Handoff Package…. It writes one
`.opsproj` archive, a DEC-028 multi-member archive built by the
report-package crate's deterministic zip (`build_deterministic_zip`, made
`pub`). The members are:

| Member | Content |
|---|---|
| `model/model.json` | The canonical model document bytes, exactly as in W-6a |
| `manifest/handoff_package.json` | A `schemas/handoff_package.schema.json` instance (below) |

**Basis state.** `model_state_refs` has `minItems: 1`, and PRD §15.1 lists
`handoff_basis` as a state kind. Saving a package therefore references a
saved model state (W-12):
- If the current model's hash equals a saved state of this project, that state
  is the basis.
- Otherwise the command first creates a state, kind `handoff_basis`, with a name
  the user confirms (default "Handoff <date>").

W-6b therefore lands after W-12's state store. Its order is W-13 S1 → W-6a →
W-12 → W-6b.

**Manifest content.** Every field comes from the document, the state or the run.
None is hard-coded to look complete.

| Field | Source |
|---|---|
| `package_identity` | `package_kind: canonical_handoff_manifest`, `package_schema_version` (the schema's current), `created_at` (local time), `created_by: "SWBPIPE desktop <version>"` |
| `model_basis` | `model_ref` (project id), `model_schema: schemas/model.schema.yaml`, `model_kind: canonical_domain_model`, `basis_state_ref`, `hash_refs` |
| `model_hash` | sha256, `canonicalization: "JCS"` (the schema's label for RFC 8785), `payload_scope: model_payload`, the session hash value |
| `units_manifest` | `project.units` |
| `entity_ids` | sorted ids per family from the document; `result_ids`/`analysis_run_ids` only when a run is included |
| `analysis_run_refs`, `result_export_refs` | The basis state's recorded runs the user ticks, by id and hash; may be empty |
| `rule_pack_refs` | The document's `rule_pack_refs` (W-13 §3.4), with `private_payload_redacted: true`; no rule content |
| `library_refs` | The local libraries the document's materials, sections and components were imported from, by identity, version and checksum; no payload |
| `unresolved_assumptions` | The document's `assumptions` with `status: open` |
| `warnings`, `diagnostics` | The current document validator's warnings and diagnostics, by code and affected refs |
| `unsupported_behavior_flags`, `target_mapping_metadata` | `reserved_metadata_only`, as the schema reserves them for DEL-15-02; nothing target-specific is claimed |
| `checksums` | One per member: sha256 of the member bytes, with `canonicalization: "JCS"` for the model member and the JCS of the manifest-without-checksums for the manifest; this is the export hash (§16.1) |
| `privacy` | `local_only: true`, `telemetry_allowed: false`, `private_payload_embedded`: true when the model member is present (the model is the user's private data), and the schema's other required flags as they actually hold |
| `professional_boundary` | The schema's fixed boundary statement; Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081) |

**Data boundary.** This is the `downstream_tool` context, and it gets a new
governed route `DOTH-HANDOFF-PACKAGE-006`.
- The existing redaction controls classify the manifest's items. Rule-pack and
  library payloads are never included.
- The model member is the user's own model. The save proceeds only with the
  explicit local-private intent control ("Include the model document — your
  private project data"), as `ControlledExportLink` does for `local_private`.
  Without it, the command refuses rather than writing a package that lacks the
  model.
- **The command refuses** while any entity's `provenance_record` has
  `redistribution_status: protected_suspected` or `review_status: quarantined`.
  The diagnostic names the records.
- Exact-route models are exported like pressure-free ones. The package carries
  the model, not results. Run results are referenced by hash only, so ruling
  10A (exact results refused for reports) is not engaged.

**Contract gaps the schema leaves open.** These are implementation choices
within DEL-15-01, not commitment changes:
- `handoff_contract_status.physical_package_container` is `"TBD"` in the
  schema. W-6b sets it to the DEC-028 archive. If the schema's `const` forbids
  that, the schema is edited to the DEC-028 value; that records an existing
  ruling.
- The top-level wrapper members (`deliverable_id`, `package_id`, `scope_item`,
  `objectives`) are filled with their schema constants.

### The preview panels

- `HandoffPanel` keeps its preview role. Its download is relabelled "Preview
  handoff summary (not a package)", and it gains a pointer to File › Save
  Handoff Package….
- The hard-coded `rule_pack_refs: []` is replaced by the document's references.
- `NativePackagePanel` is unchanged; it is PKG-17 and deferred.

## 3. Verification

- **Rust:**
  - W-6a: the round trip (save → open → equal model hash and identical bytes),
    the refusals, and cancel.
  - W-6b: the deterministic archive, member checksums that match the bytes,
    the basis-state reuse and creation, and the quarantine refusal.
  - Persistence regressions where W-6b creates a state.
- **pytest:**
  - The emitted manifest of an invented fixture validates against
    `handoff_package.schema.json`.
  - Its `model/model.json` validates against `model.schema.yaml`
    (`DesktopDocument`).
  - `core/handoff/exporter/workflow.py` accepts the manifest as its
    `handoff_package` input, which is the existing consumer's contract.
- **vitest** for the menu wiring and the intent control.
- **The native smoke** gains a step: Save Model Document… on the demo, then
  re-open it and get an equal hash. This changes `native-smoke/run.mjs`, and any
  new observation in `expected.json` goes to HELP_HUMAN as an exact change with
  its cause.
- **Review:** an independent Claude reviewer and Codex on each slice (the save
  path is persistence plus a private-data write).

## 4. Reconciliation

- **DEL-15-01:** ScopeOfWork pointers for R03–R07, R10 and R12; the
  `deliverable.yaml` code and check paths; and delivery commitment :1 (contract
  reconciliation), which W-6b addresses.
- **DEL-07-12 / DEL-17-03:** pointers only, since the preview panels stay.
- **DEL-02-05:** the save path is an outward file write of the document.
- The completeness-map rows FR-HAND-001/002 are for the reconciliation owner.
- DEL-15-01 delivery commitment :2 reads "Phase H program (gate v0.2 R6)". OD-1
  places model export in v0.1, so any text that only records that ruling goes to
  HELP_HUMAN as exact old → new.
