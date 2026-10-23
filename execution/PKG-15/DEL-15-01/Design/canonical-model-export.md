# Canonical model export and handoff manifest (W-6)

**Basis:** `origin/main` at `9d76659e02` (2026-10-23), after model document 0.7.0
(DEL-02-01 Design `model-document-0.7.0.md`). This is revision 2, after two
review rounds (an independent Claude reviewer and Codex). Design before
implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings and requirements

- **Rulings.**
  - **OD-1 (owner, 2026-10-23):** canonical model export is in v0.1. Handoff
    (PKG-15) stays in for model export; further formats (PKG-17) are v0.2.
  - **OD-4:** the desktop's model document is canonical.
- **Requirements.**
  - **FR-HAND-001 (Must):** "User can export canonical model data for the
    downstream modeling engine."
  - **FR-HAND-002 (Must):** "Export includes model hash, units, entity IDs,
    library references, rule references, warnings, and unresolved assumptions."
- **DEL-15-01 commitments.**
  - R01–R13 apply.
  - The package container is unresolved: V-03 "container … not invented"
    (ScopeOfWork :184, :271, :334) and OI-015-OPEN, "Human ruling (TBD)" (:374).
  - The handoff schema says packages are produced "without embedding private
    payloads" (`private_payload_embedded` is `const: false`,
    `handoff_package.schema.json:749`).
- **PRD §20 data boundary.**

This design meets FR-HAND-001/002 **without inventing a container and without
changing the handoff schema**:
- the model document is saved as its own canonical file (W-6a);
- the handoff manifest is a separate schema instance that binds that file by
  sha256 and embeds no private payload (W-6b);
- archiving the pair into a DEC-028 package stays the open container question
  (OI-015-OPEN, DEC-028 package roles), which is the owner's.

## 1. Where things stand

- **Native writers.** The only native file writers are:
  - `save_report_package` (`apps/desktop/src-tauri/src/lib.rs:180-236`): a Save
    dialog, `.opsproj`, an intent control, and a Rust re-check of screening and
    local-first evidence (`:64-86, :211-217`);
  - `save_local_result_json` (Downloads only; admission at
    `native_result_download.rs:93-110`, which accepts two named routes).
- **The atomic writer** `atomic_save_bytes` admits `.opsproj` paths only
  (`atomic_report_package_save.rs:183,321`).
- **The handoff download** (`HandoffPanel.tsx:209-283`) is a preview document. It
  does not conform to the schema, has no model hash, and hard-codes
  `rule_pack_refs: []`.
- **No producer** of a schema manifest exists. `core/handoff/exporter/workflow.py`
  consumes one.

## 2. W-6a: Save Model Document… (FR-HAND-001, FR-MOD-001)

**Availability.** The File menu command is available whenever a model is open,
on either route. It needs no solve and no result.

**Bytes.**
- The saved bytes are the unchecked RFC 8785 JCS text of the session's model
  document. This is the serializer the session's model hash uses:
  `computeModelHash` passes `JSON.stringify(model)` to wasm
  `canonicalSha256Hex`, which calls `core/serialization/canonical_json`
  `canonical_json` (`hashService.ts:100-118`; `operation_applier/src/lib.rs:1142`).
- There is **no trailing newline**, so sha256(file) equals the model hash value.
- The checked-v1 profiles are not used: they refuse some models the session
  hashes.

**Hash verification.**
- The webview sends the canonical text and its claimed hash.
- Rust re-parses the text, re-canonicalizes it, and refuses
  (`MODEL-DOC-SAVE-HASH-MISMATCH`) unless the result equals the text and its
  sha256 equals the claimed hash.
- Rust returns the sha256 of the bytes it wrote.

**Admission before writing.**
- The version must be current.
- The open-time checks must pass: the shape check `modelDocumentShape.ts` (run
  in the webview, as at open) and the W-13 document validator (Rust, §5.3 of
  W-13). `check_model_document_for_persist` checks only the version, so it is
  not sufficient alone.
- The 64 MiB open cap (`model_document_file.rs:23`) applies.
- Every saved file re-opens.

**Data boundary.**
- **Context.** A model document is the user's private project data. The save is
  the `local_private` context with an explicit intent control: "Save my model
  document, including private project data, to a file I choose". The control
  follows the report-save pattern; the dialog alone is not the intent.
- **Payload binding.** The save is `lossless` with an exact canonical payload.
- **Route.** A new governed route, `DOTH-MODEL-DOC-007`, is registered in three
  places:
  - the TypeScript `GOVERNED_LOCAL_FIRST_ROUTE_IDS`
    (`redactionExportControls.ts:135-147`);
  - Python `core/security/redaction/route_control.py:17-39`;
  - the new Rust command's own evidence admission, which re-checks the
    screening and local-first evidence like `lib.rs:64-86`.
- **Tests.** Route tests are added to
  `tests/security/test_redaction_export_controls.py` and the vitest twin.
- **F-PIP-1.** No network and no telemetry. The default path is the user's
  documents folder, never the repository.

**Writer.** An extension-neutral atomic writer is extracted from
`atomic_save_bytes`, with explicit admission for each format. The report
writer keeps its `.opsproj` checks unchanged. Model saves admit `.json`.

**Outcome.**
- `Saved{file_name, byte_count, model_hash}` or `Refused{diagnostics}`.
- Cancel is not an error.
- The session's model, hash and solve standing are unchanged.

## 3. W-6b: Save Handoff… (FR-HAND-001/002, PRD §16.1)

**Ordering.** It lands after W-12's state store, because `model_state_refs`
has `minItems: 1` and PRD §15.1 names a `handoff_basis` state kind. It does not
wait for comparison.

**Output.** In a folder the user chooses, the command writes two files, by the
W-6a writer and admission:

| File | Content |
|---|---|
| `<name>.model.json` | The W-6a canonical model document bytes |
| `<name>.handoff.json` | A `schemas/handoff_package.schema.json` instance (below), compact JCS |

The manifest binds the model file by sha256 and does not embed it.
`private_payload_embedded` is `false`, which is true of the manifest. Archiving
the two files into one package is left to OI-015-OPEN.

**Basis state.**
- If the current model hash equals a saved state of the project, that state is
  the basis.
- Otherwise the user confirms a new state, kind `handoff_basis`, default name
  "Handoff <date>". It is written through W-12's state store in the same
  transaction as the save's bookkeeping.

**Data boundary.** Both files are written under `local_private` with explicit
intent and a lossless binding. The manifest carries private ids, assumption
statements and rule-pack names, which `downstream_tool` would redact
(`core/security/redaction/controls.py:398,417-422`).
- **Route.** A new governed route, `DOTH-HANDOFF-PACKAGE-008`, is registered in
  the same three places as W-6a.
- **Downstream use** is the user's own later act, outside the product (PRD §20.4).
- **Disposition refusals.** The command refuses while a `provenance_record`
  carries a quarantine or rejected disposition: `protected_suspected`,
  `quarantined` or `rejected`, with the library import's codes
  (`library_import_document/src/lib.rs:345-410`). The diagnostic names the
  records.
- **Private material passes.** `private_only` and `unknown` dispositions are
  allowed in this local-private save. The manifest records them as they are and
  claims nothing public.

**Manifest adapters.** Every required field is listed, with its source. Nothing
is hard-coded to look complete.

| Field | Source / rule |
|---|---|
| top-level `schema_version`, `deliverable_id`, `package_id`, `scope_item`, `objectives`, `handoff_contract_status` | The schema's constants; `physical_package_container` stays `"TBD"` |
| `package_identity` | `handoff_package_id: handoff:<project token>:<model hash prefix>`, `manifest_id: manifest:<same>`, `package_schema_version` (current), `created_at` (local ISO time), `created_by: "SWBPIPE desktop <version>"`, `package_label` (user-entered, default the project name), `package_kind: canonical_handoff_manifest` |
| `model_basis` | `model_ref` `{Model, <project id>}`, `model_schema: schemas/model.schema.yaml`, `model_kind: canonical_domain_model`, `basis_state_ref` `{ModelState, <state id>}`, `source_refs: []`, `hash_refs` [the model hash], `provenance` (product-generated record, below) |
| `model_hash` | `algorithm: sha256`, `canonicalization: JCS` (the schema's label for RFC 8785; the desktop's `rfc8785_jcs` maps to it), `payload_scope: model_payload`, `payload_ref` `{Model, <project id>}`, value = sha256 of `<name>.model.json` |
| `units_manifest` | Derived from `project.units` by a fixed table (below); `units_schema`, `dimension_basis` and `unit_system_ref` take the schema's values for the units catalogue; `hash_refs` [JCS hash of the derived block]; refused with `HANDOFF-UNIT-UNDERIVABLE` if a unit cannot be derived |
| `entity_ids` | Sorted ids. Nodes, `component_ids`, `element_ids` (pipe segments), `load_case_ids` (cases and combinations), `model_state_ids` [basis]. `station_ids`, `result_ids`, `analysis_run_ids` and `mapping_id_refs` are `[]` |
| `model_state_refs` | [basis state] |
| `analysis_run_refs`, `result_export_refs` | `[]` in v0.1. `AnalysisRunRef`'s binding const names `analysis_run.schema.json` (`schema:67,362`) while recorded runs are v0.3 records, so including them needs a contract edit. This is recorded as open (§6) |
| `library_refs` | One per distinct `provenance_record.library_ref` (W-13 §3.5), resolved against the project's local library store for version and checksum. `private_payload_redacted: true`. Classifications come from the library's disposition |
| `rule_pack_refs` | The document's `rule_pack_refs`, mapped: `rule_pack_version → version`; `checksum` (the pack's stamped `JCS` record); `source_notice`; `redistribution_status`; `review_classification` (from the pack's lifecycle status, by fixed table); `privacy_classification: private_rule_pack_data` unless public; `private_payload_redacted: true`; `provenance` |
| `unresolved_assumptions` | The document's records with `status: unresolved`: `assumption_id`, `statement`, `status: unresolved`, `affected_refs` (below), `provenance` |
| `warnings`, `diagnostics` | The document validator's and the solve-independent model diagnostics, mapped to the handoff `Diagnostic` (`code`, `class` by fixed table from code family, `severity`, `source`, `affected_object`, `message`, `remediation`, `provenance`) |
| `unsupported_behavior_flags`, `target_mapping_metadata` | `reserved_metadata_only`, per DEL-15-02. Nothing target-specific is claimed |
| `checksums` | (1) the model file: sha256 of its bytes, `canonicalization: JCS` (the bytes are JCS), `payload_scope: model_payload`. (2) the manifest's export hash: sha256 of the JCS of the manifest with `checksums` removed, `payload_scope: handoff_package_manifest`. A tamper witness changes one byte of each file and must fail its check |
| `privacy` | `local_only: true`, `telemetry_allowed: false`, `private_payload_embedded: false`, and the other required flags as they actually hold |
| `redistribution_classification`, `review_classification` | `private_only` / `unreviewed` unless every included record is public, in which case it is the least permissive of them |
| `provenance` | A product-generated provenance record: source name "SWBPIPE desktop", location = the model file name, contributor = the local user label, review `unreviewed` |
| `professional_boundary` | The schema's fixed statement. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081) |

**Units derivation table.** Units come from `project.units`, with each unit's
identifier taken from the units catalogue.

| Manifest field | Source |
|---|---|
| `coordinate_unit`, `displacement_unit` | `length` |
| `rotation_unit` | `angle` |
| `force_unit` | `force` |
| `stress_unit` | `stress` |
| `temperature_unit` | `temperature` |
| `moment_unit` | `moment` when present; otherwise the catalogue's force×length unit for the pair, or a refusal if the catalogue has none |

**Reference mapping.** The handoff `Reference.object_type` vocabulary has no
entity types. Assumption `affected_refs` therefore map like this:
- the project id maps to `Project`;
- assumption ids map to `Assumption`;
- every other entity id maps to `{object_type: "EntityIdManifest", id}`, meaning
  an id listed in this manifest's `entity_ids`.

The vocabulary is not extended here.

**The preview panels.**
- `HandoffPanel` stays a preview. Its download is relabelled "Preview handoff
  summary (not a handoff manifest)" and it points to File › Save Handoff….
- Its hard-coded `rule_pack_refs: []` takes the document's references.
- `NativePackagePanel` is PKG-17 and unchanged.

## 4. Verification

- **Rust, W-6a:**
  - save → open gives identical bytes and an equal model hash;
  - the hash-mismatch refusal, the shape, validator and version refusals, the
    size cap, and cancel;
  - the extension-neutral writer, with the report writer's `.opsproj` checks
    unchanged.
- **Rust, W-6b:**
  - basis-state reuse and creation, written transactionally;
  - the disposition refusals;
  - the tamper witnesses.
- **pytest:**
  - one fully populated invented witness manifest validates against
    `handoff_package.schema.json`;
  - its model file validates against `model.schema.yaml` (`DesktopDocument`);
  - `workflow.py` given that manifest emits no `EXP-HANDOFF-*` or
    `EXP-PRIVACY-*` diagnostic. `EXP-TARGET-MAPPING-MISSING` is expected,
    because DEL-15-02 has no target-mapping contract (`workflow.py:144-154`);
  - route-registration tests in Python and TypeScript.
- **vitest:** menu wiring, the intent controls and the adapters.
- **Native smoke:** gains Save Model Document… on the demo, then re-open with
  an equal hash. Any new `expected.json` observation goes to HELP_HUMAN as an
  exact change with its cause.
- **Review:** an independent Claude reviewer and Codex on each slice.

## 5. Reconciliation

- **DEL-15-01:** the ScopeOfWork pointers for R03–R07, R10 and R12, and
  `deliverable.yaml` code and check paths.
  - Delivery commitment :1 asks for units entries and diagnostics coverage
    (:386). It is addressed by the units table, the diagnostic mapping and the
    witness.
- **DEL-07-12 / DEL-17-03:** pointers only.
- **The completeness-map rows** FR-HAND-001/002 are for the reconciliation
  owner.

## 6. Owner-reserved and open items (to HELP_HUMAN)

1. **DEL-15-01 delivery commitment :2** reads "Phase H program (gate v0.2 R6)".
   The exact old → new text records OD-1: model export moves into v0.1, and
   target mapping (DEL-15-02) and further formats stay v0.2.
2. **Open, for the owner when wanted:**
   - archiving the model file and manifest into one DEC-028 package. This needs
     a container ruling (OI-015-OPEN) and `.opsproj` member roles under its one
     package version;
   - referencing v0.3 analysis runs from the manifest. This needs an edit to
     `AnalysisRunRef`'s binding const.

   Neither is needed for FR-HAND-001/002 as designed.
