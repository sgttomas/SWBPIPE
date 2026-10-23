# Model document 0.7.0: one canonical document and its schema (W-13)

**Basis:** `origin/main` at `9d76659e02` (2026-10-23). Design for review before
implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings this design applies

- **OD-4 (owner, 2026-10-23, "I approve OD-1 through OD-5 as you recommended"):**
  the desktop's model document is canonical. `schemas/model.schema.yaml` is
  rewritten to describe it, and the missing slots are added: tags, notes,
  unresolved assumptions, rule-pack references and structured provenance. The
  slots arrive in a new document version under the one-version rule.
- **One current version per format (owner, 2026-10-10):** "nothing needs to be
  preserved for backwards compatibility … one current version per format". 0.7.0
  becomes the only version the product reads and writes. 0.6.0 is refused by name
  with the re-author diagnostic, like 0.1.0–0.5.0, and nothing is migrated.
- **OD-1:** constraints and equipment interfaces belong to design knowledge
  (PKG-13, PRD §13.1), which is deferred to v0.2. They get no slot here (§3.6).

Requirements: FR-MOD-001, -002, -004, -005, -006 (PRD §11.1). FR-MOD-003 is
unchanged and must keep holding: the new slots never rename or re-key an entity.

## 1. Where things stand

- **The product document.** The product document is `document_kind`
  `openpipestress.product_preview.model`, version 0.6.0. It is decoded by
  `PreviewModel` (`core/product_physics/src/lib.rs:209`). Serde is not
  `deny_unknown_fields`, so unknown keys pass through the solve unread. The
  document is edited as a JSON value by the operation applier
  (`core/model_operations/operation_applier`). Its version rule lives in
  `model_format.rs`, `model_document_version.rs` (desktop) and
  `projectService.ts:55`. The desktop open-time shape check is
  `modelDocumentShape.ts`; it requires `data_boundary`, `project.description`
  and string `label`/`provenance` members that the Rust decoder does not.
- **The current schema.** `schemas/model.schema.yaml` describes an older,
  unrelated shape (`project.models[]`, `elements`, `traceability_links`). The
  bundled demo fails it with 11 errors. Its live consumers are:
  - Python tests that validate old-shape fixtures: `test_model_schema.py`,
    `test_physical_to_analytical_transform.py`, `test_invented_example_models.py`
    and `test_project_persistence_service.py`.
  - The headless physical-to-analytical transform
    (`core/model_transform/physical_to_analytical`), called only by tests.
  - `project_persistence.schema.yaml:333` (`ModelPayload` `$ref`).
  - `test_component_section_schema.py:189`, which compares enums.
  - `schemaSlotEmission.test.tsx:339-401`, which reads `$defs` to check the
    names the desktop emits.
  - Path strings only: `primitive_loads/src/lib.rs:366-368`, `user_loads`, the
    handoff, native-JSON and persistence constants.
- **Optional members in today's documents.** All 576 committed documents carry
  `schema_version`, `document_kind` and `project`. The optional members are
  `data_boundary` (166), `diagnostics` (164), `pressure_contract` (347),
  `reference_configurations` (146) and top-level `sections` (8). The
  load/reference-state group is already described by
  `schemas/load_reference_state.schema.json`.

## 2. Version 0.7.0

0.7.0 is 0.6.0 plus the optional members in §3. A 0.6.0 document whose version
string alone is changed is a valid 0.7.0 document with the same meaning. Behaviour
still keys on what the document carries, never on the version string:
pressure-freedom is the absence of `pressure_contract`, and the load/reference
group is selected by presence. None of the new members is read by the solve, so
no numerical path changes.

Version handling follows the S1 precedent (`b2e695a555`):
- `MODEL_SCHEMA_VERSION = "0.7.0"`. `RETIRED_MODEL_SCHEMA_VERSIONS` gains
  `0.6.0`, and the re-author diagnostic names 0.7.0. The same change goes to the
  desktop Rust evaluator, the TypeScript evaluator and constants
  (`projectService.ts:55,437`, `loadStateAuthoring.tsx:17`), the applier's
  current-version checks (`bend_authoring.rs:465` and the others found by
  search), the Python verifiers and qualification tools, and the version enum
  in the schema.
- **Frozen references are not rewritten.** The shared re-author helpers
  (`tests/common/current_model.rs`, its lib twin, and
  `qualification_physics.reauthor_model`) re-author a 0.6.0 or 0.5.0 copy by its
  version string alone. SHA-bound frozen requests under `validation/**` keep
  their bytes and are re-authored in memory after their SHA check.
- **Committed non-frozen documents are regenerated.** Their `schema_version`
  becomes 0.7.0 and nothing else changes. This covers fixtures, the bundled
  demo, e2e fixtures and test literals. Where a generator exists
  (`generate_product_preview_mechanics.mjs`, the ui-foundation
  `generate-fixtures.mjs`, the qualification package generators), the generator
  is updated and re-run; the bytes are not hand-edited.
- **Generated reader corpora** record `document_schema_version` (10 files under
  `fixtures/results/`). They are regenerated by their producing tests.
- **Reviewed inputs that name the version** are edited by token only,
  0.6.0 → 0.7.0, with no other wording change. They are:
  - DEF-E `scope.model_format`;
  - `supported_profile_limitations[0]` of the pressure-source-1 table;
  - the pressure-2 table's admitted-families sentence
    (`semantic_contract_v0_3_pressure_2.json:2187`);
  - any others found by search.

  Their meaning hashes and every pin follow the actual bytes. **This needs a D3
  re-registration** (`tools/retained_pricing` `run`, `compare`, `generate --check`)
  on the exclusive lane, under the owner's M = 12 GiB. The physics-retained-1
  `accuracy_classification.scope` sentence is TBD-002 and stays untouched,
  as at S1.
- **Owner text.** DEL-02-05 ScopeOfWork CLM-003 names 0.6.0 as the current
  version in four places. The edit is the version token only and records OD-4.
  Exact old → new text goes to HELP_HUMAN (§7).
- **`native-smoke/expected.json` does not change.** It pins no version or
  hash: `run.mjs` computes the model hash from each file it opens, and the
  version string changes no observation (rows, probe, standing). This is to be
  confirmed by a native smoke run on the slice head.

## 3. New members

All new members are optional. Absence means none, and the product never
writes an empty default the user did not author. They are model data: editing one
changes the model hash and invalidates the current solve, as any edit does
(FR-CMP-002 ties results to the exact model state).

### 3.1 Tags and notes (FR-MOD-001, FR-MOD-004)

`tags` and `notes` are allowed on `project` and on every entity record:
- nodes, pipe segments, sections, materials, supports, components;
- load cases, combinations, reference configurations;
- lines and assumptions (§3.2, §3.3).

```json
"tags": ["tie-in", "rev-B"],
"notes": "Free text the user owns."
```

- `tags` is an array of unique, non-empty, trimmed strings of at most 64
  characters each. Order is preserved as authored. No vocabulary is enforced
  (FR-MOD-004: "flexible naming conventions without enforced prover status").
- `notes` is one non-empty string.
- State-level and run-level tags and notes belong to the state and run records
  (W-12), not the model document.

### 3.2 Lines (FR-MOD-002)

`lines` is a top-level array of typed line entities:

```json
{ "id": "line:L-100", "label": "Pump discharge", "pipe_refs": ["pipe:P-100", "pipe:P-110"],
  "tags": [], "notes": "…", "provenance": "user_entered" }
```

- `pipe_refs` holds unique ids of existing pipe segments. A pipe belongs to at
  most one line.
- A line is grouping only: it has no mechanical meaning and the solve does not
  read it.
- When a pipe is deleted, its id is removed from every line in the same
  operation. An emptied line is kept, and the user deletes it explicitly.

### 3.3 Assumptions and open issues (FR-MOD-006)

`assumptions` is a top-level array. PRD §11.1 and §15.1 use "unresolved
assumptions"; the record carries a status, so one collection holds the history:

```json
{ "id": "assumption:A-001", "kind": "assumption" | "open_issue",
  "statement": "Nozzle N-100 treated as anchor pending vendor loads.",
  "status": "open" | "resolved" | "withdrawn",
  "subject_refs": ["support:S-100"], "resolution_note": "…",
  "tags": [], "notes": "…" }
```

- **Levels.** An empty `subject_refs` is project level. Otherwise the record
  names lines, components, supports or any other entity id. This covers
  FR-MOD-006's project, line, component and support levels; state and run levels
  are W-12.
- **Resolution.** `resolution_note` is required when `status` is `resolved` or
  `withdrawn`.
- **Deleted subjects.** A subject that no longer exists is kept, not removed. It
  is reported as a warning diagnostic `MODEL-ASSUMPTION-SUBJECT-MISSING`, so an
  assumption's history survives entity deletion.
- **Downstream.** The handoff manifest's `unresolved_assumptions` (W-6) is the
  open subset.

### 3.4 Rule-pack references

`rule_pack_refs` is a top-level array. It names the private rule packs this
model is checked against, by identity only. Rule content never enters the model
document (PRD §20.3; DEL-02-05's rule-pack slot):

```json
{ "rule_pack_id": "rulepack:…", "name": "…", "rule_pack_version": "…",
  "checksum": { "algorithm": "sha256", "canonicalization": "rfc8785_jcs", "value": "sha256:<64 hex>" },
  "redistribution_status": "private_only" | "public_permissive" | "unknown" | "protected_suspected",
  "source_notice": "…" }
```

- The checksum is computed over the stored rule pack with the hash service the
  desktop already uses.
- The reference is added or removed from the Rule Pack Manager ("Use with this
  model"). Removing a rule pack from the local store leaves the reference in
  place, reported by `MODEL-RULE-PACK-REF-UNRESOLVED`.
- A rule-check run reads the references (W-12 and W-6 consume them). Their
  presence never makes a check pass.

### 3.5 Structured provenance (FR-MOD-005, PRD §20.2)

`provenance` stays the required short string on every record; it is unchanged.
An optional `provenance_record` object is added to materials, sections,
components and supports. A component's record covers its SIF and flexibility
modifiers, which already carry `modifiers.source_reference`. Allowables and
rules are covered by rule packs and their references (§3.4). Library records
already carry the full record.

```json
"provenance_record": {
  "source_name": "…", "source_location": "…", "source_license": "…",
  "contributor": "…", "contributor_certification": "…",
  "redistribution_status": "public_permissive" | "private_only" | "unknown" | "protected_suspected",
  "review_status": "pending" | "accepted" | "rejected" | "quarantined",
  "reviewed_by": "…", "reviewed_on": "YYYY-MM-DD", "verification_note": "…" }
```

- The field names and enums are the library import's
  (`REQUIRED_PROVENANCE_FIELDS`, `library_import_document/src/lib.rs:25`).
- Only `source_name` is required: a private model "can record" these
  (FR-MOD-005), while public data records need all seven §20.2 fields. Public
  completeness is enforced where public data is produced (library import,
  fixture checks), not on private models.
- A record with `redistribution_status: protected_suspected` or
  `review_status: quarantined` raises the existing quarantine diagnostic
  semantics on that entity (warning on the model). The export routes (W-6)
  refuse a public context while such a record is present.
- `review_status` describes source-data review only. It is not, and the UI
  never presents it as, professional acceptance (F-PIP-2).

### 3.6 Not added

- Constraints and equipment interfaces (FR-MOD-002): design knowledge, PKG-13,
  deferred by OD-1. FR-MOD-002 stays partial for those two families.
- Coordinate-system declaration, storage-policy hooks and report settings
  (DEL-02-01 ScopeOfWork, "project/model containers"):
  - The document's frame is the implicit global frame, documented in the
    schema's description.
  - Storage policy belongs to the local store.
  - Report settings belong to the report package.

  Recording this reading in the DEL-02-01 ScopeOfWork changes commitment text,
  so it goes to HELP_HUMAN as old → new (§7) rather than being edited here.
- Rigid elements (DEC-045): components already carry `geometry.rigid_*` and
  `modifiers.stiffness_scaling_user_value`, so W-13 adds no rigid fields. If the
  rigid-element unit needs one, its design comes here through HELP_HUMAN before
  the schema lands.

## 4. The schema

`schemas/model.schema.yaml` is rewritten (same path and `$id`, strict JSON
syntax, JSON Schema 2020-12) to describe the 0.7.0 document:

- **Root is the format.** It requires what the product's decoder requires:
  `schema_version` (enum `["0.7.0"]`), `document_kind` (const), `project`,
  `analysis_status`, `nodes`, `pipe_segments` and `supports`. Every other member
  is optional.
- **`$defs/DesktopDocument`** is the desktop's open and save profile: root plus
  `data_boundary`, `project.name`, `project.description`, `project.units`, and
  the string members `modelDocumentShape.ts` requires. Documents the desktop
  writes satisfy it. Headless solve requests need only the root.
- **Every record is closed** (`additionalProperties: false`), with types, units
  as `{value, unit}` quantities, the id pattern and the known enums: support
  families, component kinds, load categories, combination modes, nonlinear
  behaviours and pressure-contract members.
- **The load/reference group** `$ref`s `load_reference_state.schema.json`
  rather than restating it.
- **The schema stays structural.** Physical validity and solve admission remain
  the solver's named diagnostics. Cross-references (pipe ids exist, line
  membership is unique) belong to the applier and the document validator, not
  to JSON Schema.
- **Source of truth for the content:** the Rust wire types (`PreviewModelWire`
  and the nested `Deserialize` types), the applier's writers, and the
  TypeScript `PreviewModel` type, cross-checked against every committed
  document. Where the decoder accepts a member that no committed document and
  no writer uses, it is still described: "exactly" means the format the product
  reads. Where a committed document carries a member the decoder ignores, that
  is a finding. The stray member is removed from the fixture, unless the
  fixture is a deliberate refusal control.

**The old shape moves.** It goes by `git mv` to
`schemas/physical_transform_model.schema.yaml`, with a new `$id` and title.
Its description says it is the headless physical-to-analytical transform's
reference format (FR-MOD-007, DEL-13-04), not the model document.
- **Repointed to the moved file:** `test_physical_to_analytical_transform.py`,
  `test_model_schema.py`'s old-shape assertions (moved into a
  `test_physical_transform_model_schema.py`), `test_invented_example_models.py`
  with `examples/models/invented/*`, and the transform-fed parts of
  `test_project_persistence_service.py`.
- **Kept on the canonical schema:** `project_persistence.schema.yaml`
  `ModelPayload` keeps `$ref: model.schema.yaml`, now the canonical document. The
  persistence fixture `fixtures/persistence/invented_persisted_preview_project.json`
  and the persistence tests embed a 0.7.0 document instead of the old shape.
  W-12 uses this envelope as reference behaviour.
- **Enum checks:** `test_component_section_schema.py:189` checks the canonical
  component `kind` enum against `component.schema.yaml` `ComponentType` where
  both name the same families.
- **Cited pointers:** the `#/$defs/LoadCase` and `#/$defs/LoadRecord` strings in
  `primitive_loads` and `user_loads` are repointed to the new defs (`LoadCase`,
  `PrimitiveLoad`). `Result/values` is already broken today and is removed.
- **The desktop name-mirror test** (`schemaSlotEmission.test.tsx:339-401`) is
  replaced by direct validation of the emitted document (§5).

## 5. Keeping the schema exact

The product does not validate against the schema at runtime: no JSON Schema
library is in the Rust or desktop dependency set, and adding one is not
warranted. Exactness is kept by tests instead:

1. **Committed positive documents.** A pytest validates every committed
   positive model document against the root, and every desktop-profile
   document against `DesktopDocument`. Positive documents are found by
   `document_kind`, including those embedded in requests and envelopes. The
   bundled demo is named explicitly. Deliberate refusal controls are excluded by
   an explicit list that gives each one's reason.
2. **Desktop emission.** A desktop vitest drives each authoring path and writes
   the resulting documents to a generated, committed golden file
   (`fixtures/model_document/desktop_emitted_0.7.0.json`). The paths are: blank
   project, blank load/reference project, node/pipe/support/load/combination
   creation, tags, notes, lines, assumptions, provenance records and rule-pack
   references. The vitest fails if a fresh emission differs from the golden; the
   pytest validates the golden against `DesktopDocument`.
3. **Applier emission.** A Rust applier test does the same for one operation of
   each `set_field` slot kind and each create/delete kind, writing a golden
   that the pytest validates.
4. **A strict-JSON check** of the schema itself (no duplicate keys, no NaN),
   like `test_load_reference_schema.py:67`.

## 6. Authoring

**Applier.** The new slots go through the existing `set_field` route. The
`(object_type, field_path)` allowlist (`operation_applier/src/lib.rs:1403`)
gains these pairs:
- (every entity type, `tags`) and (every entity type, `notes`);
- (`Material` | `Section` | `Component` | `Support`, `provenance_record`);
- (`Project`, `tags` | `notes`);
- (`Model`, `lines` | `assumptions` | `rule_pack_refs`).

Each value is validated before it is applied, with named blocking diagnostics:
- tag rules;
- line pipe references exist and are unique across lines;
- assumption status/resolution consistency and id pattern;
- rule-pack checksum form;
- provenance enums.

Pipe deletion (`delete_pipe_run`) also removes the pipe from lines. All of these
edits are validated, diffed and undoable like every other operation, and none
can change an entity id (FR-MOD-003).

**Desktop.**
- The property inspector gains Tags and Notes for the selected entity and for
  the project.
- Materials, sections, components and supports get a collapsible Provenance
  record section.
- A Lines section in the model tree supports creating a line from selected pipes,
  renaming it and removing it.
- An Assumptions and open issues panel supports adding, editing, resolving and
  withdrawing, and filtering by status and subject. A subject is chosen from
  the current selection.
- The Rule Pack Manager gets "Use with this model" and "Remove from model".
- New diagnostics appear in the existing diagnostics list.
- No new menu item; save-to-file is W-6.

## 7. Owner-reserved text (exact old → new, for HELP_HUMAN)

To be sent with the exact lines once the implementation slice is ready:

1. **DEL-02-05 ScopeOfWork CLM-003, lines 33, 35 and 37.** Version token
   `0.6.0` → `0.7.0`. The `RETIRED_MODEL_SCHEMA_VERSIONS` sentence gains
   `0.6.0`. Records OD-4.
2. **DEL-02-01 ScopeOfWork.** The current-implementation paragraph (line 88) is
   reconciliation and is edited here. The "project/model containers"
   obligation (line 80) needs the §3.6 reading recorded, which is a change to
   commitment text, so it goes to HELP_HUMAN.

## 8. Slices and checks

- **S1: format and schema.** Version 0.7.0 everywhere (§2); regenerated
  fixtures; the reviewed-input token edits with D3 re-registration; the
  rewritten schema and the moved old schema; the tests in §5.1 and §5.4.
  Checks:
  - `desktop-test`, `piping-pytest`, the affected Rust crates (`product_physics`,
    `operation_applier`, `result_export`, `runner/headless`, desktop
    `src-tauri`) and the native smoke, each compared with main;
  - the `tools/hosted-ci-routing.json` digest refresh for the `.rs` changes;
  - D3 `generate --check`;
  - review: an independent Claude reviewer and Codex.
- **S2: authoring.** Applier slots, desktop UI, and the emission goldens in §5.2
  and §5.3. Checks: `desktop-test`, `piping-pytest`, applier tests, and the
  native smoke where the open/save path changes; same review.
- **Ordering.** S1 changes every model document in the repository, so W-4 (the
  native journey test) should author 0.7.0 documents. HELP_HUMAN agrees the
  order: S1 lands first, or W-4 lands first and S1 regenerates its fixture.

## 9. Reconciliation (per slice)

- **DEL-02-01** (schema, Design pointer, `deliverable.yaml` code paths and the
  new tests).
- **DEL-02-05** (CLM-003 via §7).
- **DEL-04-08 / DEL-04-07** Design version mentions.
- **DEL-07-02** (inspector slots).
- **DEL-13-01** (FR-MOD-006 pointer).
- The completeness-map rows FR-MOD-001/002/004/005/006 are for the
  reconciliation owner.
