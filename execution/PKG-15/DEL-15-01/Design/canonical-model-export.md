# Canonical model export and handoff manifest (W-6)

**Basis:** `origin/main` at `9d76659e02` (2026-10-23), after model document 0.7.0
(DEL-02-01 Design `model-document-0.7.0.md`). This is revision 3, after two
review rounds, each by an independent Claude reviewer and Codex. Design before
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

**Design reading.**
- **W-6a** saves the model document as its own canonical file. It needs no
  ruling.
- **W-6b** writes a **directory package**, one of DEC-028's two forms ("zip/
  directory"). It contains the canonical model member and a schema-conformant
  handoff manifest, which carries the member inventory and per-member sha256
  hashes. It is published atomically by a single directory rename.
  - The handoff schema is unchanged: `physical_package_container` stays `"TBD"`,
    and `private_payload_embedded` stays `false`.
  - The zip/`.opsproj` form and the PKG-17 package-manifest roles stay with
    OI-015-OPEN.
- **HELP_HUMAN, 2026-10-23: "accepted as the design."** The directory package
  applies an existing ruling (DEC-028), and the schema's container stays TBD.
  - Whether it meets DEL-15-01 R12 and V-03 is established by its verification
    evidence (§4). Acceptance stays with the owner at validation, as for
    everything else.
  - Nothing here claims acceptance.

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
  - Python `core/security/local_first_storage/route_control.py:17-39`, whose
    unregistered routes fail closed at `:86-95`;
  - the new Rust command's own evidence admission, which re-checks the
    screening and local-first evidence like `lib.rs:64-86`.
- **Tests.** Route tests are added to
  `tests/security/test_redaction_export_controls.py` and the vitest twin.
- **Disposition screening.** The TypeScript screener blocks
  `protected_suspected`, `quarantined` and `rejected` dispositions in every
  context, `local_private` included (`redactionExportControls.ts:435-440,556-571`).
  W-6a states this and tests it:
  - a model carrying such a `provenance_record`, rule-pack reference or library
    disposition is refused with the screener's code, naming the records;
  - every other disposition, including `private_only` and `unknown`, saves.

  The screening explicitly covers `provenance_record`, `rule_pack_refs[]` and the
  resolved `library_ref` dispositions. It does not rely on the generic payload
  projection, which passes those through unchanged.
- **Rust repeats the content checks (implementation review, PR #1588).** The
  evidence alone does not admit a save. The command re-establishes the
  conditions on the exact text it admits:
  - the webview shape check, ported to Rust (`model_document_shape.rs`) and
    kept equal by a shared invented corpus
    (`fixtures/model_document/desktop_shape_cases.json`);
  - the open path's adoption preconditions, which the shape check now states
    explicitly (DEL-07-11 Design `model-document-open-and-native-smoke.md`,
    "Shape check"). Every shape-clean corpus case saves and re-opens through the
    open path, and the adoption mutation test covers the demo's string members;
  - disposition screening at any depth: every `provenance_record`, every
    `rule_pack_refs[]` entry, and every `library_ref`, resolved against the
    local library store. Identities are trimmed as `open_local_library` does (Unicode White_Space, as Rust's `trim`; the webview uses `trimLikeRust`),
    and every matching record is screened. The webview finds the
    references with `libraryRefSites` (`services/libraryImportService.ts`),
    the walk the open-time store check `check_model_library_refs` also uses;
    the native walk is pinned to it by
    `fixtures/model_document/library_ref_sites.json`, and the identity and
    record-matching rules (`library_identity`, `matching_library_records`)
    are one Rust implementation shared with that check (DEL-03-01
    `apply-library-records.md` §2.7). The verdict is a port of the
    library import's disposition rules, including the hanger conflict rule.
    A parity test (`disposition_verdicts_match_the_library_import`) pins it
    to the public import API. It covers four kinds, library and record level,
    own and provenance redistribution and review values, and the blocking
    findings only;
  - a store read error refuses (`MODEL-DOC-SAVE-LIBRARY-STORE-UNAVAILABLE`). A
    library or record the store does not hold saves with a
    `MODEL-LIBRARY-REF-UNRESOLVED` warning; the entity's own record is still
    screened;
  - the screening evidence carries the payload's sha256, and Rust refuses
    `MODEL-DOC-SAVE-EVIDENCE-UNBOUND` unless it equals the admitted text's.
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

**Dependency.** It needs W-12's state store, because `model_state_refs`
has `minItems: 1` and PRD §15.1 names a `handoff_basis` state kind. It does not
wait for comparison.

**Output.** A directory package `<name>.swbhandoff/`, at a location the user
chooses:

| Member | Content |
|---|---|
| `model/model.json` | The W-6a canonical model document bytes, with W-6a's admission and screening |
| `handoff_package.json` | A `schemas/handoff_package.schema.json` instance (below), compact JCS |

**Publication.** The package is published atomically:
1. Both members are written into a staging directory
   `<name>.swbhandoff.staging-<random>` beside the target and fsynced.
2. One `rename` publishes the staging directory. This is the single commit
   point.
3. **An existing target is replaced by an atomic swap.** macOS
   `renamex_np(staging, target, RENAME_SWAP)` exchanges the two directories in
   one step, so there is no interval without a target. The swapped-out old
   package is then deleted.
   - If the volume does not support the swap, the command refuses with
     `HANDOFF-REPLACE-UNSUPPORTED` and asks for a new name.
   - A leftover `*.staging-*` from a crash is ignored, and is reported for the
     user to delete.
4. **Failure tests.** A failure injected between the member writes, or before
   the swap, leaves the previous package intact.

**Privacy.** The manifest binds `model/model.json` by sha256 in `checksums` and
does not embed it.
- `private_payload_embedded: false` is true of the manifest. "Private payload"
  here means model content, library content and rule content.
- The manifest's ids, names and assumption statements are private project
  metadata, classified `private_project_data`.

**Basis state.**
- If the current model hash equals a saved state of the project, that state is
  the basis.
- Otherwise the user confirms a new state, kind `handoff_basis`, default name
  "Handoff <date>". It is written through W-12's state store.
- **Order.** The state is committed first, and then the package is published.
  A filesystem rename cannot join a database transaction. If publication
  fails, the state remains; it is harmless and reusable.

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
| `model_hash` | `algorithm: sha256`, `canonicalization: JCS` (the schema's label for RFC 8785; the desktop's `rfc8785_jcs` maps to it), `payload_scope: model_payload`, `payload_ref` `{Model, <project id>}`, value = sha256 of the `model/model.json` member |
| `units_manifest` | Derived from `project.units` by a fixed table (below); `units_schema`, `dimension_basis` and `unit_system_ref` take the schema's values for the units catalogue; `hash_refs` [JCS hash of the derived block]; refused with `HANDOFF-UNIT-UNDERIVABLE` if a unit cannot be derived |
| `entity_ids` | Sorted ids. Nodes, `component_ids`, `element_ids` (pipe segments), `load_case_ids` (cases and combinations), `model_state_ids` [basis]; `station_ids`, `result_ids`, `analysis_run_ids` are `[]`. The families `EntityIdManifest` has no slot for go in `mapping_id_refs` as model-qualified references `{object_type: "Model", ref: "<project id>#<entity id>"}`: supports, materials, sections, lines, constraints, equipment interfaces. FR-HAND-002's entity ids are thus complete |
| `model_state_refs` | [basis state] |
| `analysis_run_refs`, `result_export_refs` | `[]` in v0.1. `AnalysisRunRef`'s binding const names `analysis_run.schema.json` (`schema:67,362`) while recorded runs are v0.3 records, so including them needs a contract edit. This is recorded as open (§6) |
| `library_refs` | One per distinct `provenance_record.library_ref` (W-13 §3.5): `library_ref` `{Library, <library_id>}`, `library_kind`, `version` (the recorded `library_version`). `checksum`: sha256 JCS over the reference metadata `{library_kind, library_id, library_version, document_sha256}`, where `document_sha256` is the JCS sha256 of the stored `local_libraries.document_json`; `payload_scope: library_reference_metadata`. `private_payload_redacted: true`; classifications come from the library's disposition.
- `library_kind` mapping: `material`, `section` and `component` copy across.
  The store's `hanger` kind has no handoff enum value, so it maps to `TBD`, and
  `source_notice` says "hanger library". That is recorded as an open contract
  gap of the handoff schema.
- `source_notice` comes from the library document's metadata source notice, or
  its provenance `source_name` when there is none.
- `provenance` is the library's provenance mapped to the eight handoff fields
  (as the `provenance` row maps them). |
| `rule_pack_refs` | The document's `rule_pack_refs`, mapped: `rule_pack_version → version`. `checksum`: sha256 JCS over the reference metadata `{rule_pack_id, version, stamped_checksum_value, basis, grammar_version}`, copied from the pack's stamped `ComputedRulePackChecksum`; `payload_scope: rule_pack_reference_metadata`, `payload_ref` `{RulePack, <id>}`. This hash truthfully covers the reference metadata, and through it the pack's own checksum. The stamped record itself cannot be a handoff `Checksum`, because its fields and scope differ (`rule_pack_document/src/lib.rs:742-751`). Also `source_notice`, `redistribution_status`, `review_classification` (from the pack's lifecycle status, by fixed table), `privacy_classification: private_rule_pack_data` unless public, `private_payload_redacted: true`, `provenance` |
| `unresolved_assumptions` | The document's records with `status: unresolved`: `assumption_id`, `statement`, `status: unresolved`, `affected_refs` (below), `provenance` |
| `warnings`, `diagnostics` | The document validator's and the solve-independent model diagnostics, mapped to the handoff `Diagnostic` (`code`, `class` by fixed table from code family, `severity`, `source`, `affected_object`, `message`, `remediation`, `provenance`) |
| `unsupported_behavior_flags`, `target_mapping_metadata` | `reserved_metadata_only`, per DEL-15-02. Nothing target-specific is claimed |
| `checksums` | The member inventory: (1) `model/model.json`, sha256 of its bytes, `canonicalization: JCS` (the bytes are JCS), `payload_scope: model_payload`; (2) the export hash, sha256 of the JCS of the instance's `handoff_package_manifest` object with its `checksums` member removed, `payload_scope: handoff_package_manifest`. The verifier is `verify_handoff_package` (Rust, in the desktop crate; a Python twin under `tests/verifiers/`). It recomputes both and is run by the tamper witnesses: one byte changed in each member must fail |
| `privacy` | `local_only: true`, `telemetry_allowed: false`, `private_payload_embedded: false`, and the other required flags as they actually hold |
| `redistribution_classification`, `review_classification` | `private_only` / `unreviewed` unless every included record is public, in which case it is the least permissive of them |
| `provenance` | A product-generated handoff `Provenance` with all eight required fields: `source_name` "SWBPIPE desktop", `source_location` (the package name), `source_license` "user_private_project_data", `contributor` (the local user label), `contributor_attestation` (W-13's `contributor_certification` maps here wherever it is copied), `redistribution_status: private_only`, `review_classification: unreviewed`, `privacy_classification: private_project_data` |
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

**Reference mapping.** The handoff `Reference` is `{object_type, ref}`, and
`object_type` has no entity types. Assumption `affected_refs` therefore map
like this:
- the project id maps to `{Project, <id>}`;
- assumption ids map to `{Assumption, <id>}`;
- every other entity id, including a deleted subject, maps to the
  model-qualified `{Model, "<project id>#<entity id>"}`.

**What the handoff assumption record drops.** It is closed:
- W-13's `kind` (assumption or open issue) is folded into the statement prefix
  "Open issue: …";
- `tags`, `notes` and `resolution_note` are dropped;
- only `unresolved` records are included, with handoff status `unresolved`.

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

0. **Directory package: decided as the design** (HELP_HUMAN, 2026-10-23; §0).
   - `physical_package_container` stays `"TBD"`.
   - The zip/`.opsproj` form stays with OI-015-OPEN.
   - Conformance to R12 and V-03 is shown by the W-6b verification evidence.
     Acceptance is the owner's at validation.
1. **DEL-15-01 delivery commitment :2** reads "Phase H program (gate v0.2 R6)".
   The exact old → new text records OD-1: model export moves into v0.1, and
   target mapping (DEL-15-02) and further formats stay v0.2.
2. **Open, for the owner when wanted:**
   - archiving the model file and manifest into one DEC-028 package. This needs
     a container ruling (OI-015-OPEN) and `.opsproj` member roles under its one
     package version;
   - referencing v0.3 analysis runs from the manifest. This needs an edit to
     `AnalysisRunRef`'s binding const.

   The design reads neither as needed for FR-HAND-001/002. Whether the
   requirements are met is shown by evidence and accepted by the owner.
