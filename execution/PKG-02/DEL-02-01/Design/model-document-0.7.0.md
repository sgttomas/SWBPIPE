# Model document 0.7.0: one canonical document and its schema (W-13)

**Basis:** `origin/main` at `9d76659e02` (2026-10-23). This is revision 3, after
two review rounds, each by an independent Claude reviewer and Codex. Design before
implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings this design applies

- **OD-4 (owner, 2026-10-23, "I approve OD-1 through OD-5 as you recommended"):**
  the desktop's model document is canonical. `schemas/model.schema.yaml` is
  rewritten to describe it. The missing slots are added: tags, notes, unresolved
  assumptions, rule-pack references and structured provenance. They arrive in a
  new document version under the one-version rule.
- **One current version per format (owner, 2026-10-10):** "nothing needs to be
  preserved for backwards compatibility … one current version per format". 0.7.0
  becomes the only version the product reads and writes. 0.6.0 is refused by name
  with the re-author diagnostic, like 0.1.0–0.5.0, and nothing is migrated.
- **OD-1:** the design-knowledge layer (PKG-13: constraint evaluation, candidate
  generation, knowledge entry) is deferred to v0.2. FR-MOD-002 still requires
  constraints and equipment interfaces to be typed entities of the model. This
  design adds them as records with no evaluation (§3.6), so FR-MOD-002 is not
  narrowed.
- **HELP_HUMAN (2026-10-23), second answer:** "Scope addition: keep them in."
  Typed lines, constraints (the ConstraintRecord subset) and equipment
  interfaces go in as data slots with no evaluation. That keeps FR-MOD-002
  (Must) met rather than narrowed, and it does not touch OD-3's deferral of
  design-knowledge evaluation.

Requirements: FR-MOD-001, -002, -004, -005, -006 (PRD §11.1). FR-MOD-003 holds
unchanged: no new slot renames or re-keys an entity.

## 1. Where things stand

- **The product document.** It is the `openpipestress.product_preview.model`
  document, version 0.6.0.
- **Rust decoding.** `PreviewModel` (`core/product_physics/src/lib.rs:209`,
  wire form `:249`) decodes it.
  - The top level and the node, pipe, support, material and component records
    ignore unknown keys.
  - The load/reference-state group (`case_state/input.rs:1-77`) and the pressure
    contract (`pressure_runtime.rs:57,65,79`) are `deny_unknown_fields`.
  - Section `provenance` is a `serde_json::Value` (`lib.rs:402`), so it may be a
    string or an object. Material, support and component provenance are
    optional strings.
- **Applier.** The operation applier (`core/model_operations/operation_applier`)
  edits the document as a JSON value.
- **Desktop.** The desktop shape check (`modelDocumentShape.ts`) requires members
  the decoder ignores: `data_boundary`, `project.name`/`description`, string
  labels and `diagnostics`. The desktop also writes members the decoder ignores,
  e.g. `components[].completeness`.
- **The schema before S1a.** `schemas/model.schema.yaml` described an older
  shape (`project.models[]`, `elements`, `traceability_links`), which the
  bundled demo failed with 11 errors. Since S1a (#1554) it describes the 0.6.0
  document and the old shape is `schemas/physical_transform_model.schema.yaml`.
  The consumers of the old shape at the design basis:
  - **Validate old-shape fixtures:** `test_model_schema.py`,
    `test_physical_to_analytical_transform.py`,
    `test_invented_example_models.py` and `test_project_persistence_service.py`.
  - **Old-shape code:** the headless transform
    (`core/model_transform/physical_to_analytical`), which only tests call.
  - **`$ref`:** `project_persistence.schema.yaml:333`.
  - **Reads enums and `$defs`:** `test_component_section_schema.py:189` and
    `schemaSlotEmission.test.tsx:339-401`.
  - **Bind the path as a const:** `handoff_package.schema.json:354,593`,
    `native_json_export.schema.json:465` and `native_json/package.py:374`.
  - **Pointer strings:** `primitive_loads/src/lib.rs:366-368`, which is a
    D3-pinned file, and `user_loads/src/lib.rs:1324`. The
    `ModelResultValue` binding has live consumers (`user_loads:342`,
    `load_case_algebra:226`).
- **Committed documents.** 576 documents carry the `document_kind`:
  - 290 are 0.6.0 and 286 are retired versions (frozen references and refusal
    controls);
  - 4 accept-case applier documents (contract corpus cases 76–78) are partial
    and have no `project` or `nodes`.

## 2. Version 0.7.0

0.7.0 is 0.6.0 plus the optional members in §3. A 0.6.0 document whose version
string alone is changed is a valid 0.7.0 document with the same meaning. None
of the new members is read by the solve.

**The solve decodes the whole document.** There is no projection to solve
fields.
- **Strict solve inputs.** `PreviewModelWire`, `PreviewLoadCase`,
  `PreviewPrimitiveLoad` and `PreviewCombination` refuse unknown keys
  (`deny_unknown_fields`). Every member a document may carry is declared on them,
  including the desktop-only `data_boundary` and `diagnostics` on the root and
  `kind`, `label` and `status` on load cases.
- **0.7.0 declares,** as accepted and unread:
  - top-level `lines`, `unresolved_assumptions`, `constraints`,
    `equipment_interfaces` and `rule_pack_refs`;
  - `tags` and `notes` on load cases and combinations.
- **Test.** A 0.7.0 document carrying every new member solves end to end, with
  results equal to the same document without them.

**The standard for every regenerated artifact** (precedent `f468da4651`): the
version token plus every binding digest that follows from it, written by the
maintained writers. No physical value, status or reader outcome moves. Each
regeneration is checked by comparing the numerical leaves of old and new
artifacts separately from the expected hash and byte changes. This is S1b's
review evidence.

**Code.**
- `MODEL_SCHEMA_VERSION = "0.7.0"`. `RETIRED_MODEL_SCHEMA_VERSIONS` gains
  `0.6.0`, and the re-author diagnostic names 0.7.0.
- The same change goes to:
  - the desktop Rust evaluator;
  - the TypeScript evaluator and constants (`projectService.ts:55,437`,
    `loadStateAuthoring.tsx:17`);
  - the applier's version checks (`bend_authoring.rs:465`);
  - the Python verifiers and qualification tools;
  - the readers that accept only the current version (`retained_precision.rs`,
    `physics_source.rs` and their Python twins);
  - the schema enum.
- **Literal version gates.** These production gates compare the literal
  `"0.6.0"` and are routed through the shared constant:
  - desktop: `componentIntent.ts:86,136`, `ConnectorDraftFields.tsx:60`,
    `JointPressureModelEditor.tsx:8`, `PressureAuthoringPanel.tsx:164`,
    `pressureContract.ts:32,48`;
  - applier: `joint_pressure_authoring.rs:34,132,198`, `lib.rs:2691`,
    `pressure_authoring.rs:149`.
- **Guard test.** A new test fails on any literal model version outside the
  constant definitions and the retired lists, in desktop and applier production
  code. The test covers string literals in messages and names as well as
  comparisons.
- **Texts and emitted data that name the version** take the constant:
  - diagnostics: `case_state/resolve.rs:79`, `joint.rs:838`,
    `pressure_authoring.rs:285`, `model_document_version.rs:76`,
    `projectService.ts:76`, `geometry_operations.rs:665`,
    `joint_pressure_authoring.rs:39`, applier `lib.rs:2693,2782,5161`;
  - the blank project name "Blank Local Load/reference Model 0.6.0"
    (`projectService.ts:447`).
- **Packaged self-test.** The production self-test embeds a 0.6.0 model
  (`apps/desktop/src-tauri/src/lib.rs:4482`, run from `main.rs:8`) and moves to
  0.7.0.

**Frozen references.**
- SHA-bound frozen requests under `validation/**` keep their bytes. Their
  consuming tests check the recorded SHA, then re-author the copy. The shared
  helpers (`tests/common/current_model.rs`, its lib twin,
  `qualification_physics.reauthor_model`) re-author 0.6.0 and 0.5.0 by version
  alone.
- **The exception** is the precedent's token-edited witness input,
  `tp_runner_015`, with its generator and the validation-manual line
  (`353ca256a3`). It is edited the same way, by token.
- `validation/references/o4_retained_source/declared_problem/twins.py:220,240`
  and the t4_u5b builder (`core/product_physics/tests/common/t4_u5b.rs`) follow
  the current version.

**Regenerated by maintained writers.** Hand edits are not allowed. This covers:
- the bundled demo, product-preview fixtures and their generation records
  (`demo_fixture_generation.json` and `preview_physics_fixture_generation.json`;
  these also pin the schema's SHA, at lines 531 and 1072);
- the ui-foundation fixtures and their `fixture-manifest.json`;
- `core/runner/headless/tests/t4_u5b_requests`;
- the applier contract corpus (`applied_model_canonical_sha256` and its claimed
  hashes);
- the qualification packages and their MANIFEST;
- the reader corpora;
- result envelopes, whose `model_hash` comes from the request checksum
  (`result_envelope_binding.rs:379`);
- persistence binding hashes and test literals.

**Retained and source successors (the D3 chain).** These embed a captured 0.6.0
model, and the readers accept only the current version:
- the 16 `fixtures/results/retained_precision_*successor*` files;
- `retained_precision_cases.json`;
- the two `failed_ordinary` fixtures.

Invocation hashes cover the raw request (`source_receipt.rs:127`). DEF-E's
hash enters preparation payloads (`retained_wire.rs:1508`), and successor
documents and receipt hashes are pinned (`test_retained_precision_b3.py:656`).
The 0.5.0→0.6.0 epoch needed four steps, and S1b repeats them:
1. **Price.** `tools/retained_pricing` pins the full-file SHA of
   `model_format.rs`, `lib.rs`, `exact_admission.rs`, `primitive_loads/src/lib.rs`,
   `physics_source.rs`, `retained_precision.rs`, `semantic_contract.rs` and others
   (`base/b7a_domain_model.py:74`). Moving any of them stops the tool with exit
   4.
2. **Register.** `retained_memory.rs:1204` pins the reviewed inputs
   `prepared_exact_v1`, `pressure_source_1` and `pressure_2`. Admission reads
   Stale until the profile is re-registered and the registration is rebound.
3. **Regenerate the successors publicly** on the registered build, with the
   maintained writers.
4. **Update the synthetic-bases writer.** It hard-codes the version step
   (`tools/fixtures/retained_precision_synthetic_bases.py:11,362`).

The reviewed inputs are edited by token only, 0.6.0 → 0.7.0, with no other
wording change. They are:
- DEF-E `scope.model_format`;
- `supported_profile_limitations[0]` of the pressure-source-1 table;
- the pressure-2 admitted-families sentence
  (`semantic_contract_v0_3_pressure_2.json:2187`).

The physics-retained-1 `accuracy_classification.scope` sentence is TBD-002 and
stays untouched. The re-pricing gets independent review.

**`native-smoke/expected.json` does not change.** Both reviews confirmed this:
it pins no version or hash, `run.mjs:157` hashes the file it opened, and the
newer-version case uses 9.0.0. A native smoke run on the S1b head confirms it,
and the counts are reported.

## 3. New members

All new members are optional, and absence means none. The product never writes
an empty default the user did not author. Clearing a member removes it. The
members are model data: editing one changes the model hash and invalidates the
current solve, as any edit does.

The new members **may not appear** on the load/reference-state group
(reference configurations, analysis states, expansion laws) or the pressure
contract. Those are `deny_unknown_fields` solve inputs (§1), and W-13 leaves them
unchanged.

### 3.1 Tags and notes (FR-MOD-001, FR-MOD-004)

`tags` and `notes` are allowed on:
- `project`;
- nodes, pipe segments, sections, materials, supports, components;
- load cases and combinations;
- the new entities in §3.2, §3.3 and §3.6.

```json
"tags": ["tie-in", "rev-B"],
"notes": "Free text the user owns."
```

- `tags`: unique, non-empty, trimmed strings, at most 64 characters each,
  ordered as authored. No vocabulary is enforced (FR-MOD-004).
- `notes`: one non-empty string.
- State-level and run-level tags and notes belong to the state and run records
  (W-12).

### 3.2 Lines (FR-MOD-002)

```json
"lines": [{ "id": "line:L-100", "label": "Pump discharge", "pipe_refs": ["pipe:P-100"],
            "tags": [], "notes": "…", "provenance": "user_entered" }]
```

- `pipe_refs` holds unique ids of existing pipes, and a pipe belongs to at most
  one line.
- Grouping only: the solve does not read lines.
- A line may be left with empty `pipe_refs`; the user deletes it explicitly.
- **Pipe edits.**
  - Deleting a pipe removes it from its line in the same operation.
  - A pipe created by split joins the source pipe's line.
  - A bend arc joins the line of the incoming pipe `p_in` (`bend_authoring.rs:631`).
  - A copied pipe (`geometry_operations.rs:534-541`) joins no line, since the
    copy is a new run.

### 3.3 Unresolved assumptions and open issues (FR-MOD-006)

The slot is `unresolved_assumptions`, the name used by OD-4, PRD §15.1 and the
state, operation and handoff schemas. Its vocabulary is
`model_state.schema.json` `AssumptionRecord`'s, so W-12 and W-6 map it without a
third vocabulary:

```json
"unresolved_assumptions": [{
  "assumption_id": "assumption:A-001", "kind": "assumption" | "open_issue",
  "statement": "Nozzle N-100 treated as anchor pending vendor loads.",
  "status": "unresolved" | "resolved" | "rejected",
  "affected_refs": ["support:S-100"], "resolution_note": "…",
  "tags": [], "notes": "…" }]
```

- **Levels.** An empty `affected_refs` is project level. Otherwise the record
  names lines, components, supports or any entity id. This covers FR-MOD-006's
  project, line, component and support levels.
- **State and run levels.** These are W-12's: assumptions recorded on a saved
  state or a recorded run, held in their label rows (W-12 §2, §3). Copying the
  model's assumptions into a state record does not by itself discharge them.
- **Resolution.** `resolution_note` is required when `status` is `resolved` or
  `rejected`.
- **Deleted entities.** An assumption keeps naming an entity that is later
  deleted. The document validator reports `MODEL-ASSUMPTION-SUBJECT-MISSING`
  (warning).
- **Deletion is never blocked.** The applier's generic delete scan treats any
  `*_refs` key as a reference (`lib.rs:3807-3837`). It therefore skips
  `unresolved_assumptions`, so naming an entity in an assumption never blocks
  that entity's deletion (§6).
- **Adapters.** Neither the state schema nor the handoff schema has entity types
  in its closed `Reference.object_type`. Their `AssumptionRecord`s are closed,
  and the handoff status vocabulary differs. Each consumer therefore defines its
  own mapping and lists what it drops: W-12 §2 and W-6 §3 do.
  - The shared rule: an entity id becomes the model-qualified reference
    `{object_type: "Model", ref: "<project id>#<entity id>"}`. A project-level
    record names the project.

### 3.4 Rule-pack references

```json
"rule_pack_refs": [{ "rule_pack_id": "…", "name": "…", "rule_pack_version": "…",
  "checksum": <the pack's own ComputedRulePackChecksum record>,
  "redistribution_status": "private_only" | "public_permissive" | "unknown" | "protected_suspected",
  "source_notice": "…" }]
```

- **The checksum record is the pack's own stamped record**
  (`core/rules/rule_pack_document/src/lib.rs:742-790`): canonicalization `"JCS"`,
  basis `rfc8785_jcs_sha256_excluding_checksums`, bound to the grammar version.
  A reference is checked by comparing it with the stored pack's stamped
  checksum.
- **Identity only.** Rule content never enters the document (PRD §20.3).
  Rule-pack names and notices can be private. The model document is the user's
  private data, and every export route treats `private_only` references under
  the export policy (W-6). Issue bundles and public contexts redact them.
- **UI.** References are added and removed from the Rule Pack Manager ("Use with
  this model"). A reference whose pack is no longer in the local store reports
  `MODEL-RULE-PACK-REF-UNRESOLVED` (warning). A reference never makes a check
  pass.

### 3.5 Structured provenance (FR-MOD-005, PRD §20.2)

- **Existing `provenance` keeps its current forms:** a string on nodes, pipes,
  materials, supports, components, load cases and combinations, and a string or
  object on sections (the decoder's `Value`). The schema describes these forms
  exactly.
- **New `provenance_record`.** An optional `provenance_record` object is added
  to materials, sections, components and supports. A component's record covers
  its SIF and flexibility modifiers. Rules are covered by rule packs (§3.4), and
  library records already carry the full record.

```json
"provenance_record": {
  "source_name": "…", "source_location": "…", "source_license": "…",
  "contributor": "…", "contributor_certification": "…",
  "redistribution_status": "public_permissive" | "private_only" | "unknown" | "protected_suspected",
  "review_status": "pending" | "accepted" | "rejected" | "quarantined",
  "reviewed_by": "…", "reviewed_on": "YYYY-MM-DD", "verification_note": "…",
  "library_ref": { "library_kind": "material" | "section" | "component" | "hanger",
                   "library_id": "…", "library_version": "…", "record_id": "…" } }
```

`library_ref` records the local library and record an entity was inserted from.
- It matches the store's key `(project_id, library_kind, library_id)`
  (`src-tauri/src/lib.rs:875-885`).
- The desktop sets it when it inserts a library record (S2).
- It is the source of the handoff manifest's `library_refs` (W-6b).
- `library_version` = "sha256:" + the SHA-256 of the inserted record's RFC 8785
  (JCS) form.
- **Warnings.** The store keeps one document per library id (`:3768`). Both
  warnings need the store, so they run at desktop open and load, not in the
  applier validator:
  - `MODEL-LIBRARY-REF-UNRESOLVED` when the store has no such library or record;
  - `MODEL-LIBRARY-REF-VERSION-CHANGED` when the stored record's hash differs
    from `library_version`.

  The recorded version is never silently updated.
- **E/ν route** (W-17 design, DEL-03-01; HELP_HUMAN ruling, 2026-10-23). An
  isotropic material record with E and ν but no G derives G = E / (2(1+ν)). The
  `provenance_record` records that G was derived and from what. A stored G is
  used and never overwritten. This is numerical meaning, so it gets targeted
  regression tests and independent scrutiny.
- The library payload never enters the document.

- **Field names and enums** are the library import's
  (`REQUIRED_PROVENANCE_FIELDS`, `library_import_document/src/lib.rs:25`).
- **Required fields.** Only `source_name` is required on a private model
  (FR-MOD-005 "can record"). Public data records need all seven §20.2 fields.
- **Committed public fixtures.** A pytest fails if any committed model document
  carrying `provenance_record` lacks one of the seven or carries a non-public
  disposition. Committed fixtures stay invented (F-PIP-1).
- **Disposition policy.** The library import's full policy
  (`library_import_document/src/lib.rs:345-410`) is applied on every
  public or downstream export context (W-6):
  - `protected_suspected` or `quarantined`: quarantine;
  - `rejected`: blocked;
  - `private_only`, `unknown` or missing: blocked for public use.

  In the model itself, a quarantine disposition raises a warning on the entity.
- **UI wording.** `review_status` describes source-data review only. Its UI
  label reads "Source review", checked against `docs/claims_registry.md`. It
  never presents professional acceptance (F-PIP-2).

### 3.6 Constraints and equipment interfaces (FR-MOD-002)

These are typed records with no evaluation. The solve does not read them.
Constraint checking, design-knowledge entry and candidate generation are PKG-13
and v0.2 (OD-1).

**Scope and boundary (HELP_HUMAN, 2026-10-23: "keep them in").** These records
go beyond the five slots OD-4 enumerates. They are data slots only, there so
that FR-MOD-002 (Must) is met rather than narrowed.
- Nothing evaluates them: no constraint checking, clearance, routing or
  equipment-interface consistency.
- Design-knowledge entry and evaluation (FR-KNOW-001, PKG-13) remain deferred
  by OD-1/OD-3.
- The UI says the records are not checked.

**Constraint vocabulary.** It shares field names and enums with `schemas/constraint.schema.json`
`ConstraintRecord`, so v0.2 extends the vocabulary instead of replacing it:
- the same names: `constraint_id`, `constraint_kind`, `name`, `state`,
  `target_refs`;
- `constraint_kind` excludes `TBD`, `equipment_interface` (that is
  `equipment_interfaces`) and `missing_required_data` (that is an open issue,
  §3.3);
- `state` uses `ConstraintRecord`'s enum.

```json
"constraints": [{ "constraint_id": "constraint:K-001", "name": "…",
  "constraint_kind": "clearance", "state": "proposed",
  "target_refs": ["pipe:P-100"], "statement": "…", "tags": [], "notes": "…", "provenance": "…" }],
"equipment_interfaces": [{ "id": "equipment-interface:E-001", "label": "Pump P-1 discharge nozzle",
  "node_ref": "node:N-100", "equipment_label": "…", "tags": [], "notes": "…", "provenance": "…" }]
```

`target_refs` and `node_ref` follow §3.3's rule for deleted entities: they
never block deletion, and a missing entity is reported as a warning.

### 3.7 Not added

- **Coordinate system.** The document's frame is the implicit global frame,
  stated in the schema description.
- **Storage policy** belongs to the local store, **report settings** to the
  report package, and **result/report records** to the result and report
  formats.
- Recording this reading in DEL-02-01 ScopeOfWork CLM-006 is commitment text.
  It goes to HELP_HUMAN as old → new, with the reasoning that no FR-MOD
  requirement is narrowed (§7).
- **Rigid elements (DEC-045).** Components already carry `geometry.rigid_*` and
  `modifiers.stiffness_scaling_user_value`; W-13 adds no rigid fields.

### 3.8 Model settings (node-proximity tolerance)

- **Ruling.** The owner ruled (2026-10-23, relayed by HELP_HUMAN) that the
  near-duplicate node check uses a fixed 5 mm tolerance "and give the user the
  means to change it". HELP_HUMAN approved this design the same day, adding that
  the unit must be a length unit of the units catalog. The setting lives in the
  model document, so the desktop and the headless runner warn identically.
- **Member.** An optional top-level `model_settings`: a closed record of
  model-level analysis and modeling settings, every member optional. Its one
  member is `node_proximity_tolerance`, a length quantity `{value, unit}`.
- **Default rule.** An absent `model_settings`, or an absent member, means
  5 mm. The product never writes the default into a document the user did not
  author. An authored `null` is not absence: the validator refuses it as
  `MODEL-SLOT-SHAPE` and the solve wire refuses it, as for `model_settings`
  itself.
- **Admission.** The value must be finite and greater than zero, and the unit a
  length unit of the units catalog; converted to metres, the value must still be
  finite and greater than zero (a value too small to represent in metres is
  `MODEL-SETTING-TOLERANCE-NOT-POSITIVE`). Each refusal is named:
  `MODEL-SETTING-TOLERANCE-NOT-FINITE`, `MODEL-SETTING-TOLERANCE-NOT-POSITIVE`
  and `MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH` (a non-length or unknown unit);
  a malformed record is `MODEL-SLOT-SHAPE`. The document validator (§5.3) and
  product_physics' accessor give the same codes and messages.
- **Readers.** product_physics declares the record on the solve wire
  (`model_settings.rs`, a closed `ModelSettings`) and exposes the effective
  tolerance in metres (`PreviewModel::node_proximity_tolerance_m`). The
  topology check (DEL-04-06 Design `model-topology-checks.md`) reads it through
  that accessor, and the solve refuses a malformed value with the accessor's
  code before the check runs (`validation::validate_model_settings`). The desktop editing
  control comes with the S2 authoring surface; the desktop already round-trips
  the whole document.

## 4. The schema

`schemas/model.schema.yaml` is rewritten (same path and `$id`, strict JSON
syntax, JSON Schema 2020-12).

- **The root is the format.** It requires what `PreviewModelWire` requires:
  `schema_version`, `document_kind`, `project`, `analysis_status`, `nodes`,
  `pipe_segments` and `supports`.
- **`$defs/DesktopDocument`** is the desktop's open and save profile. It is the
  root plus every member `modelDocumentShape.ts` requires (`data_boundary`,
  `project.name`, `project.description`, `project.units`, the required string
  members and the required arrays, including `diagnostics`).
  - Documents the desktop writes must satisfy it. Headless solve requests need
    only the root.
  - A document is desktop-profile when the desktop wrote it: blank and saved
    projects, the bundled demo, e2e fixtures, the emission goldens (§5.2), and
    any fixture that `modelDocumentShape.ts` admits in an existing test.
- **What "exactly" means.** The persisted contract is the union of:
  - every member any reader decodes (the Rust wire types, including the closed
    case-state and pressure types);
  - every member any writer emits (desktop TypeScript, applier, generators).

  Desktop-only members the decoder ignores are part of the contract and are
  preserved: `data_boundary`, `diagnostics`, `project.name`/`description`,
  `components[].completeness`.
- **Stray members.** A member is stray only when every reader ignores it and no
  writer emits it. A stray member is a finding. It is removed only from non-frozen
  fixtures that writers regenerate. Refusal controls keep their form.
- **Every record is closed** (`additionalProperties: false`). It carries types,
  `{value, unit}` quantities and the known enums, including the alias spellings
  the readers accept.
- **Ids are non-blank strings** (as implemented in S1a). The readers refuse only
  blank ids, and deliberate Unicode-id fixtures exist. The applier's ASCII
  pattern is its rule for ids it generates.
- **Two exceptions to closed records.** `data_boundary` is an open string map,
  as the desktop types it (`Record<string, string>`), and its known keys are
  listed. Section `provenance` may be a string or an object.
- **The pressure contract's members are required** in the schema: a present
  contract is complete. Rust decodes them as optional and refuses an incomplete
  contract at admission. Incomplete-contract fixtures are refusal controls.
- **`$defs/Model`** (the root) and **`$defs/Element`** (the pipe segment) name
  REQ-02-01-02's families explicitly.
- **The load/reference group** `$ref`s `load_reference_state.schema.json`,
  unchanged.
- **Result and report records by reference** (REQ-02-01-02: "define or
  reference"). `$defs/Result` `$ref`s the result-envelope definition of
  `results.v0.3.schema.yaml`. `$defs/Report` `$ref`s the report-section records
  of `report_sections.schema.yaml`. The root does not use them, because the
  model document carries no results or reports. They make the schema's family
  coverage explicit, and the conformance test resolves both references.
- **Structural only.** Physical validity and admission remain the solver's.
  Cross-references are the document validator's (§5.3).

**Old shape.**
- The old schema moves by `git mv` to
  `schemas/physical_transform_model.schema.yaml`, with a new `$id` and a
  description that names it the headless transform's reference format
  (FR-MOD-007, DEL-13-04).
- The transform tests and `test_invented_example_models.py` (with
  `examples/models/invented/*`) validate against the moved file directly. Their
  old-shape assertions move into `test_physical_transform_model_schema.py`.
- The shared validation registry (`tests/schema_validation.py:75-124`) already
  loads every schema file. The hand-made registries in two tests are updated.

**Persistence tests are separated.**
- `project_persistence.schema.yaml` `ModelPayload` keeps
  `$ref: model.schema.yaml`, now the canonical document.
- The canonical persistence fixtures and tests embed 0.7.0 documents.
- The transform-fed persistence test (`test_project_persistence_service.py`,
  around :289-345 and :647) stops wrapping transform output in a canonical
  envelope. It validates the transform output against the moved schema and
  keeps its own persistence assertions on a canonical payload. There is no
  adapter between the two shapes.

**Pointer strings.**
- The `primitive_loads` bindings' referents are the desktop load case and its
  `primitive_loads[]` record. The old schema's `LoadCase` (`load_type` +
  `loads[]`) and `LoadRecord` (`load_record_type`) are different records.
- The new schema therefore defines `$defs/LoadCase` (the desktop load case) and
  `$defs/LoadRecord` (the desktop primitive load). The strings at
  `primitive_loads/src/lib.rs:366-367`, which is a D3-pinned file, then keep
  valid text unchanged.
- `ModelResultValue` (`#/$defs/Result/values`, broken today) is repointed to
  its actual referent, the result-value definition in the results schema the
  consumers use (`user_loads:342`, `load_case_algebra:226`,
  `stress_recovery/src/lib.rs:757`). This also happens in S1b. All of these
  consumers' tests follow.

**Path consts.**
- `handoff_package.schema.json`'s `model_schema` const keeps naming
  `schemas/model.schema.yaml`, which is correct after W-6b. Its tests
  (`test_handoff_package_schema.py:173,234`, `test_handoff_export_workflow.py:186,210`)
  are unchanged until then.
- `native_json_export.schema.json:465` and `native_json/package.py:374` bind an
  entity-view payload, PKG-17 (v0.2). They stay unchanged and are recorded as a
  known PKG-17 gap.

**Tests that read the old defs.**
- `test_component_section_schema.py:189` checks the canonical component `kind`
  enum against `component.schema.yaml`'s families.
- `schemaSlotEmission.test.tsx:339-401` (TP-PMM-GUIEMIT-001) is replaced by the
  emission validation in §5.2, which keeps the TP-PMM-GUIEMIT-001 test id.

## 5. Keeping the schema exact and the document valid

There is no runtime JSON Schema: no library is in the Rust or desktop
dependency set, and none is added.

1. **Conformance pytest, by rule:**
   - committed documents at the current version validate against the root;
   - desktop-profile documents (§4) also validate against `DesktopDocument`;
   - SHA-bound frozen requests validate after their SHA check and in-memory
     re-authoring;
   - retired-version documents are excluded by rule, as refusal controls or
     frozen inputs;
   - partial applier operation fixtures (no `project`) are excluded by rule;
   - documents embedded in requests and envelopes are found by `document_kind`;
   - the bundled demo is named explicitly;
   - any other exclusion needs an explicit entry with its reason.
2. **Desktop emission golden.**
   - A vitest drives each authoring path: blank project and blank
     load/reference project; node, pipe, support, load and combination
     creation; and every §3 slot.
   - It runs under a fixed clock and writes
     `fixtures/model_document/desktop_emitted.json`, which has no version in
     its name. An explicit update mode regenerates it, and the vitest fails when
     a fresh emission differs.
   - The pytest validates every entry against `DesktopDocument`. It keeps the
     TP-PMM-GUIEMIT-001 id.
   - A Rust applier test writes a golden for one operation of each new kind,
     and the pytest validates it.
3. **One document validator.** A manual Rust validator in the applier crate,
   exposed to wasm, checks:
   - tag form, unique ids within each new collection, line membership (each
     pipe exists and belongs to one line);
   - assumption status and resolution consistency;
   - constraint kinds, rule-pack checksum form, provenance enums;
   - missing-subject warnings.

   It runs at file open (`read_model_document_file` and the webview open path),
   at project load, and on every operation the applier applies. A malformed
   externally authored slot is refused at open with a named diagnostic and its
   JSON path, like today's `MODEL-FILE-SHAPE`. It is never repaired.
4. **Lossless round trip (FR-MOD-001).** A document carrying every new member
   goes through four paths, and each must return the same model hash and equal
   content:
   - open → save → reopen through the SQLite store;
   - file open → W-6a save → reopen;
   - the Python persistence envelope;
   - undo/redo of each new operation.
5. **Negative tests** for every validator rule, at open and at apply.
6. **Strict-JSON check of the schema** (no duplicate keys, no NaN), like
   `test_load_reference_schema.py:67`.

## 6. Authoring (applier and desktop)

**Object types and change kinds.** `collection_for` (`lib.rs:867-878`) gains:
- `Line`, `Assumption`, `Constraint` and `EquipmentInterface`, over their
  collections;
- `Project`, as a singleton target for project tags and notes.

The new operations are identity-preserving, with explicit create and delete and
no whole-array replacement:
- `create_line`, `update_line`, `delete_line`;
- `create_assumption`, `update_assumption`, `delete_assumption`;
- `create_constraint`, `update_constraint`, `delete_constraint`;
- `create_equipment_interface`, `update_equipment_interface`,
  `delete_equipment_interface`;
- `add_rule_pack_ref`, `remove_rule_pack_ref`;
- annotations: (any entity, `tags`), (any entity, `notes`) and
  (Material | Section | Component | Support, `provenance_record`), carried on
  each family's existing change kind. That is `set_field` for most families and
  `update_load` / `update_support` for loads and supports (`lib.rs:1404-1411,
  2390`).

`model_operation.schema.json` (closed `object_type` and `change_kind` enums,
imported at `App.tsx:6,498`) is not extended. The applier's existing kinds are
already outside it.

`update_*` never changes an `id`, which keeps FR-MOD-003. `FieldKind` gains a
string-list kind for tags. Clearing a field removes the member; the `TBD`
sentinel (`lib.rs:250-253`) is not used for these slots.

**Geometry and reference scans.**
- **Record shape.** `record_shape` (`geometry_operations.rs:27-40,128-157`)
  accepts `tags` and `notes` on nodes and pipes, so split, transform and copy
  work on annotated records. Nodes and pipes carry no `provenance_record`
  (§3.5).
- **Attachment scans.** The scans (`geometry_operations.rs:233-303`,
  `bend_authoring.rs:583-606`) and the generic delete scan
  (`lib.rs:3807-3837`) skip `tags`, `notes` and `provenance_record`, and do not
  scan `unresolved_assumptions`, `constraints`, `equipment_interfaces` or
  `lines`. These are handled explicitly:
  - deleting a pipe removes it from its line;
  - deleting a node named by an equipment interface, or any entity named by an
    assumption or constraint, is never blocked and leaves a missing-subject
    warning.
- **Split and bend insertion** (`geometry_operations.rs:388-393`,
  `bend_authoring.rs:631-636`) clone the source pipe.
  - The new pipe inherits the source's `tags` and `notes`.
  - It joins lines as §3.2 says.
  - It is not added to assumptions or constraints that name the source; the
    source id survives.
- **Applier tests** cover each case.

**Desktop.**
- The property inspector gets Tags and Notes for the selected entity and for
  the project.
- Materials, sections, components and supports get a Provenance record section.
- The model tree gets a Lines section: create a line from the selected pipes,
  rename it, remove it.
- An Assumptions and open issues panel lets the user add, edit, resolve and
  reject, and filter by status and subject.
- A Constraints and interfaces list lets the user add and edit records only;
  there is no evaluation.
- The Rule Pack Manager gets "Use with this model" and "Remove from model".
- New diagnostics appear in the diagnostics list.
- **Known gap closed in S2 (found in the S1a review).** The schema accepts
  `support.nonlinear: null` as linear, as the applier and solver read it. The
  desktop's `SupportConfigurationForm.tsx` (~39, ~119, ~250) treats a null as
  present: it shows "Include nonlinear behavior" checked and then requires a
  behaviour. S2 makes the form read null as absent.

## 7. Owner-reserved text (exact old → new to HELP_HUMAN at the slice that needs it)

1. **Version token 0.6.0 → 0.7.0.** These record OD-4, so HELP_HUMAN approves
   them (HELP_HUMAN, 2026-10-23). Item 2 went to the owner and is approved.
   - DEL-02-05 ScopeOfWork CLM-003, lines 33, 35, 37 and 100, and CLM-013
     line 178.
     - Line 33's "defined by the T4-U4b/U5b epoch" becomes a pointer to this
       design.
     - Line 34's retired list gains `0.5.0` and `0.6.0`.
   - DEL-04-08 ScopeOfWork REQ-001 (line 108), REQ-002 (line 122) and the
     TBD-002 status text (line 262), with TBD-002's held physics-retained
     sentence unchanged.
   - DEL-03-01 REQ-03-01-007 (line 145).
2. **DEL-02-01 ScopeOfWork CLM-006, "project/model containers".** Record that
   the document's frame is the implicit global frame, that storage policy is
   the store's, and that report settings and result/report records belong to
   their own formats.
   - **Reasoning:** no FR-MOD requirement names a coordinate-system, storage,
     report-settings or result slot in the model. FR-MOD-001's "warnings" are
     the document's `diagnostics` plus the validator's. FR-MOD-002 is met in
     full by §3.2 and §3.6.
     - New line 79 also drops "application identity" and "provenance summary".
       The application is identified by `document_kind`
       (`openpipestress.product_preview.model`), not a separate field.
       Provenance is carried per record (§3.5), and no FR-MOD or PRD §20.2
       clause requires a root-level summary. The report's provenance summary
       stays with the Report row (CLM-012, line 177).
   - **This narrows no FR-MOD requirement.** It is still commitment text, so it
     went to the owner through HELP_HUMAN as exact old → new. **Approved:**
     owner, 2026-10-23: "CLM‑006/CLM‑012, option A as recommended." The new
     text below is applied in the ScopeOfWork.
   - **Exact text.** These are DEL-02-01 ScopeOfWork CLM-006 lines 79–83; lines
     77, 84 and 86 are unchanged.
     - **Old 79:** `- document/root metadata: schema version, application/schema identity, provenance summary, and migration/compatibility status where persisted;`
     - **New 79:** ``- document/root metadata: schema version, document identity (`document_kind`, which also identifies the application) and the data-boundary statement; provenance is carried per record; no migration or compatibility status is persisted, because one model-document version is current (DEL-02-05 CLM-003);``
     - **Old 80:** `- project/model containers: units, coordinate system, storage policy hooks, rule-pack references, report settings, and model object collections;`
     - **New 80:** `- project/model containers: units, rule-pack references and model object collections; the coordinate system is the implicit global frame stated by the schema, storage policy belongs to the local project store (DEL-02-05), and report settings belong to the report package;`
     - **Old 81:** `- model objects: node coordinates and degrees of freedom, element endpoints/material/section references, components, supports, loads, load cases, and combinations;`
     - **New 81:** `- model objects: node coordinates; pipe-segment endpoints and material/section references; sections, materials, components and supports (degrees of freedom are expressed by support restraints); loads, load cases and combinations; lines, constraints and equipment interfaces as typed records without evaluation; and unresolved assumptions and open issues;`
     - **Old 82:** `- engineering data records: unit-bearing quantities, provenance records, redistribution status, source quality, and review status;`
     - **New 82:** `- engineering data records: unit-bearing quantities, provenance strings and structured provenance records with redistribution status and review status (source quality is recorded as review status and verification note), tags and notes;`
     - **Old 83:** `- result/report records: result envelopes, warnings/diagnostics, input manifests, hashes/checksums, and professional-boundary notices;`
     - **New 83:** `- warnings/diagnostics carried by the model document and its professional-boundary statement; result envelopes, input manifests and report records belong to the result, analysis-run and report formats, and model hashes are computed over the canonical document (RFC 8785 JCS) rather than stored in it;`
   - **The same reading in the CLM-012 crosswalk** (lines 169, 170, 176 and 177,
     "Expected schema treatment" column only; the other columns are unchanged).
     REQ-02-01-02 (line 148, "define or reference") needs no edit, because the
     schema references Result and Report (§4).
     - **Old 169 (Project):** `Define a root/project record with stable identity, units, storage/privacy posture, rule-pack references, report settings, and persistence hooks where applicable.`
     - **New 169:** `Define a root/project record with stable identity, units and rule-pack references; storage/privacy posture and persistence hooks belong to the local project store (DEL-02-05), and report settings to the report package.`
     - **Old 170 (Model):** `Define the model container and collections for nodes, elements, components, supports, loads, libraries, results, and report-facing references.`
     - **New 170:** `Define the model collections for nodes, pipe segments (elements), sections, materials, components, supports, loads, lines, constraints, equipment interfaces and assumptions; library records are referenced per entity (provenance_record.library_ref); results and report-facing records are referenced by schema definition (Result, Report), not carried in the document.`
     - **Old 176 (Result):** `Define result-facing records compatible with mechanical outputs, warning classes, diagnostics, affected-object references, provenance, and authority/status boundaries.`
     - **New 176:** `Reference, from $defs/Result, the result-facing records defined by the results schema, compatible with mechanical outputs, warning classes, diagnostics, affected-object references, provenance, and authority/status boundaries.`
     - **Old 177 (Report):** `Define report-facing records for input manifest, model hash/checksum references, warnings, assumptions, provenance summary, limitations, and professional-boundary notices.`
     - **New 177:** `Reference, from $defs/Report, the report-facing records defined by the report schemas, covering input manifest, model hash/checksum references, warnings, assumptions, provenance summary, limitations, and professional-boundary notices.`
3. **Reconciliation pointers.** These are reconciliation, not commitment
   changes:
   - DEL-02-01 line 88 (current implementation);
   - `deliverable.yaml` conditions naming 0.6.0 or `b7-plan` "Current model
     format": DEL-02-05:128, DEL-10-05:48, DEL-12-01:98, DEL-04-07:7;
   - DEL-05-02 ScopeOfWork :78 and DEL-05-03 :124, if they are current-state
     text; if either is a commitment, it is listed under item 1;
   - `b7-plan.md` "Current model format", `RELEASE_NOTES_v0.1.0_DRAFT.md:53`,
     `persistence_contract.md` and `GATE_USAGE.md`.
4. **Scope addition: decided.** HELP_HUMAN kept the §3.2 and §3.6 records in on
   2026-10-23, as data slots with no evaluation (§0, §3.6).

## 8. Slices

1. **S1a: the canonical schema for the current document.** No D3, and no file
   in the D3 pinned set (§2).
   - Scope:
     - the rewritten `model.schema.yaml` at version 0.6.0, without the §3
       members;
     - the moved old schema, with the transform and persistence test
       separation;
     - the conformance pytest (§5.1) and the strict-JSON check (§5.6).
   - The replaced persistence fixture
     (`fixtures/persistence/invented_persisted_preview_project.json`), which
     carries an old-shape payload.
   - The repaired `schemaSlotEmission.test.tsx` (TP-PMM-GUIEMIT-001): its
     assertions are rewritten against the new defs in S1a, and S2 extends it
     with the golden.
   - The generation records' schema SHA pins
     (`demo_fixture_generation.json:531,1072` and its twin) were **not
     refreshed** in S1a.
     - Their writer (`generate_product_preview_mechanics.mjs:123`) would also
       re-pin about 13 core source files that changed on main since its last
       run.
     - No test reads the schema pins; `test_preview_physics_consumer_contract.py`
       checks only the output and input-model SHAs.
     - The pins refresh at the writer's next run by its owner.
   - Checks: `piping-pytest` and `desktop-test` compared with main; dual review.
2. **S1b: version 0.7.0 and the §3 members in the format.**
   - Scope:
     - schema members and enum;
     - the version ripple and the D3 chain (§2), with exclusive runs through
       HELP_HUMAN;
     - the document validator (§5.3) at open and load;
     - any pointer-string repoint;
     - the reserved token edits once HELP_HUMAN approves.
   - Checks: affected Rust crates, `piping-pytest`, `desktop-test` and the
     native smoke compared with main; D3 `generate --check`; the
     `tools/hosted-ci-routing.json` digest refresh; numerical-leaf comparison of
     the regenerated artifacts; independent review of the re-pricing; dual
     review.
3. **S2: authoring** (§6, §5.2, §5.4, §5.5).
   - **Constraint:** S2 touches no D3-pinned file. If it must, it carries its own
     D3.
   - Checks: applier tests, `desktop-test`, `piping-pytest`, the native smoke
     where open/save changes, and dual review.
4. **Order.** S1a (#1554), then S1b, then S2. The rigid-element unit's fields,
   if any, join S1b.

## 9. Reconciliation (per slice)

- **DEL-02-01:** schema, Design pointer, `deliverable.yaml` code paths and checks.
- **DEL-02-05:** CLM-003 and CLM-013 via §7.
- **DEL-04-08 / DEL-04-07:** version mentions and Design.
- **DEL-03-01:** REQ-03-01-007 via §7.
- **DEL-07-02:** inspector slots.
- **DEL-13-01:** FR-MOD-006 pointer.
- **DEL-13-04:** the moved transform schema.
- The completeness-map rows FR-MOD-001/002/004/005/006 are for the
  reconciliation owner.
