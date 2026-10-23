# Model states, run history and comparison in the desktop (W-12)

**Basis:** `origin/main` at `9d76659e02` (2026-10-23), after model document 0.7.0
(DEL-02-01 Design `model-document-0.7.0.md`). Design for review before
implementation. Persistence changes get targeted regression tests and dual review.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings and scope

- **OD-1 (owner, 2026-10-23, "I approve OD-1 through OD-5 as you recommended"):**
  model states, run history, and state and run comparison are in v0.1. This
  settles D-11/DEC-062's "W7 — Forward horizon v0.2" placement for PKG-14.
- **Scope is the PRD §23 MVP:** "named immutable model states; analysis runs
  tied to states; basic model-state comparison; basic analysis-run comparison".
  That means FR-CMP-001, -002, -003, -004 and -006 (all Must).
- **Out of this unit:** the Should items FR-CMP-005/007 (mapping editor, manual
  mappings), -008 (tolerance profiles), -009 (graphical diffs) and -010
  (comparison export). PRD §23 places these in R3, and they stay open as
  DEL-14-05 commitments. Comparison results are therefore
  `not_tolerance_checked`, and nothing is classified as within or outside a
  tolerance.
- **Reference behaviour (PKG-14 headless):**
  - `core/comparison/model_state/engine.py` (`compare_model_states`): stable id
    first, then added, removed, changed or unchanged by canonical field diff.
  - `core/comparison/analysis_run/engine.py` (`derive_exact_result_id_mappings`,
    raw deltas with unit checks).
  - `schemas/model_state.schema.json`, `analysis_run.v0.3.schema.json`.
  - `core/project_persistence/service.py` `run_history`.

## 1. Where things stand

- **Store.** The desktop store (`openpipestress-projects.sqlite3`, migrations to
  v11, `apps/desktop/src-tauri/src/lib.rs:743,896`) keeps one row per project,
  with one `model_json`, one `mechanics_result_json` and one
  `analysis_run_json`. The upsert overwrites them (`:1129`); there is no
  history.
- **Run-to-model link.** The run's `model_state_ref` is synthetic,
  `state:<model_ref>:preview` (`analysisRunCompatibility.ts:164`, checked at
  `:237`, re-derived in `HistoricalRunContext.tsx`). No state record exists.
- **Comparison.** Comparison is within one run: one load case against one
  combination, with hard-coded ids (`previewService.ts:654`,
  `ComparisonPanel.tsx`).
- **Session.** Any edit clears the current run (`workspaceSession.ts:617-642`).

## 2. Model states (FR-CMP-001)

**A state** is a named, immutable snapshot of the model document. The snapshot
is the canonical model document and its model hash, which is the session's
`computeModelHash`, RFC 8785 JCS.

**Record.** The state record is a `model_state.schema.json` `ModelStateRecord`:
- `state_id`: `state:<project-local token>`;
- `state_name`;
- `state_kind`: `design`, `pre_solve`, `post_solve`, `comparison_basis` or
  `handoff_basis`;
- `created_at`;
- `model_ref`: the project id;
- `parent_state_refs`: the state the working model was opened from, if any;
- `tags`, `notes`, `unresolved_assumptions` (the document's open assumptions)
  and `warnings` (the document validator's);
- `hashes`: `model_payload` with `canonicalization: "JCS"`, the schema's label,
  plus `model_state_record`;
- `immutability_policy`, `professional_boundary` and `provenance`.

**Immutability.**
- The snapshot (`model_json`, `model_hash`) and the record are never updated.
  SQLite `BEFORE UPDATE` triggers on those columns raise, and the Rust API has
  no update path.
- Only the labels (`state_name`, `tags`, `notes`) can be edited, and they live
  in a separate label row. FR-MOD-004 says users label states with flexible
  naming, and FR-CMP-001 says "tag … without mutation" of the snapshot.
- The record hash covers the snapshot and the immutable fields, not the labels.
- A state is deleted only by the user, and only when no recorded run references
  it. Deleting the project deletes its states and runs.

**Commands.**
- **Save Model State…:** a name (required), tags and notes. If the project row
  does not exist yet, the existing Save Local Project flow runs first, with its
  confirmation rules.
- **Equal hashes.** Saving a state whose model hash equals an existing state of
  the project does not create a duplicate. It names the existing state and
  offers to add the new name as a label.
- **Open State.** Loads the snapshot into the session as the working model,
  behind the existing unsaved-changes guard. The working model's
  `parent_state_ref` becomes that state. Editing then produces a new working
  model; the state is unchanged.

**Store migration v12** adds `model_states` and `model_state_labels`, keyed by
`project_id` with foreign keys, plus the triggers. Existing rows are untouched.

## 3. Runs tied to states (FR-CMP-002)

A recorded run is a `analysis_run.v0.3` record plus its mechanics result. Its
`model_state_ref` is a real state id: the synthetic `state:<model_ref>:preview`
is retired.

- **Record Run** is available when a solved result exists for the current model.
  - The run's state is the project state whose model hash equals the result's
    model hash.
  - If there is none, the user is asked to name a new state, kind `post_solve`,
    and the state and run are written in one transaction.
- The run record carries what the product already carries: solver version,
  build identity, settings, load cases and combinations, diagnostics and
  result hashes.
- It also carries the document's `rule_pack_refs` (W-13 §3.4) and its library
  references. These are no longer the hard-coded `[]` at
  `analysisRunCompatibility.ts:167`.
- Results attach to the immutable run, not to the mutable working state
  (DEL-14-02 R004).
- **Migration v12 also adds `analysis_runs`:**
  - `run_id` PK, `project_id`, `state_id` (FK, NOT NULL), `created_at`;
  - `run_json`, `run_hash`, `result_json`, `result_hash`;
  - update triggers that raise.
- **The current-run slot.** The existing `mechanics_result_json` and
  `analysis_run_json` columns stay the project's current-run slot.
  `HistoricalRunContext` continues to reopen it, re-checked against the real
  state id.
- **Stored synthetic refs.** A current-run slot saved before this change carries
  the synthetic ref. On reopen it reports the existing
  `HISTORICAL_MODEL_STATE_REF_MISMATCH` finding and is not usable for overlays.
  Nothing is migrated (owner, 2026-10-10).
- **Size.** The mechanics result can be large. Recording uses the same
  persistence memory guard as Save Local Project (W-5, #1547), and the History
  panel lists runs without loading their results.
- **Run history panel.** States are listed with name, tags, kind, created time,
  short hash and run count. Runs are listed under their state with solver,
  cases, standing and created time.

## 4. Comparison (FR-CMP-003, -004, -006)

Both comparisons run in the desktop's TypeScript session, like today's
`buildPreviewComparison`. They are deterministic and use stable ids
automatically. There are no manual mappings in this unit.

### 4.1 State comparison

- **Entities.** The two snapshots are flattened to entity lists. Each family
  (nodes, pipe segments, sections, materials, supports, components, load cases,
  combinations, reference configurations, lines, assumptions, rule-pack
  references) contributes `{family, id, record}`.
  - **Model-level members** (`project`, `pressure_contract`, `data_boundary`,
    `analysis_status`) are compared as one `model` entity.
  - **Primitive loads** are entities under their load case's id. They are
    keyed as `<load case id>/<load id>`.
- **Classification.**
  - Same id in both, same family: `unchanged` or `changed`.
  - Only left: `removed`. Only right: `added`.
  - Same id, different family: `unresolved`, with a diagnostic.
- **Field diff.** `changed` rows list their field paths from a canonical-JSON
  field diff.
- **Units.** A unit-bearing quantity (`{value, unit}`) whose unit differs
  between sides reports `unit_changed` with both values as authored, never a
  bare numeric delta (REQ-14-03-007).
- **Output.** Summary counts and rows sorted by family, then id.
- **Parity with the reference engine.** A committed golden of the TypeScript
  output for invented state pairs is checked by a pytest. The pytest runs
  `compare_model_states` on the same flattened entity lists and requires equal
  summaries and row classifications.

### 4.2 Run comparison

- **Row identity.** A result row's identity is `(kind, entity_ref, basis_ref,
  dimension, metadata.component, metadata.location, metadata.basis)`.
- **Automatic mapping.** Rows map automatically when their `id`s are equal and
  their identities are equal, which is the reference
  `derive_exact_result_id_mappings` rule. Stable entity ids carry across states
  (FR-CMP-006).
- **Delta.**
  - A mapped pair gets `right − left` only when the units are equal and both
    rows' cases are published `NUMERICAL_INTEGRITY_CHECKS_PASSED` with
    certified rows.
  - Otherwise the pair is `not_compared`, with its reason: a unit or standing
    difference (and the standing), or a differing result semantic identity
    (PF against EX, or a different producer contract).
  - No unit conversion is applied in this unit.
- **Unmatched rows** are listed per side.
- **Context deltas.** Solver version, settings and the load-case and
  combination sets are compared like the reference `_settings_deltas`.
- **Which runs.** Two runs of the same project, on the same state or on
  different states, can be compared.
- **Parity:** a pytest compares `compare_analysis_runs` and the TypeScript
  output on a committed golden. The desktop rows are adapted to the
  reference's `quantity_results` shape by a documented, tested adapter.

### 4.3 UI

- **History panel.** A "Compare…" action takes two states or two runs.
- **The Comparison panel** shows the summary, a filterable table of rows and,
  for a changed entity, its field paths with left and right values.
- **Hard-coded comparison removed.** The existing single-run load-case versus
  combination comparison stays as "Within this run". Its hard-coded ids are
  replaced by the user's selection of a case and a combination.
- No export, tolerance or overlay in this unit (§0).

## 5. Report and handoff consumers

- `stateComparisonHandoffSections.ts` stops synthesizing state refs. It emits
  the real state and run records when the report package includes them.
- W-6b uses the state store for its basis state.

## 6. Verification

- **Rust store tests** (persistence; targeted regression):
  - migration v12 on a v11 store with existing rows;
  - state create, reuse and duplicate-hash behaviour;
  - immutability, where an update raises at the SQL level as well as in the
    API;
  - label edits that leave the record hash unchanged;
  - deletion refused while runs reference a state;
  - the run and its new state written in one transaction, rolled back on
    failure;
  - an envelope round trip that preserves the model hash byte-for-byte;
  - reopening a current-run slot with the retired synthetic ref.
- **vitest:**
  - the session flows (save state, open state, record run, compare);
  - the guard on unsaved changes;
  - retirement of the synthetic ref in `analysisRunCompatibility.ts` and
    `HistoricalRunContext`.
- **pytest:**
  - the parity goldens (§4.1, §4.2);
  - state and run records validating against `model_state.schema.json` and
    `analysis_run.v0.3`.
- **Smoke.** The native smoke gains Save Model State, Record Run and Compare
  only if HELP_HUMAN wants it in the G10 procedure. Any `expected.json` change
  goes to HELP_HUMAN with its cause.
- **Review:** an independent Claude reviewer and Codex. Persistence checks are
  run on the implementation head.

## 7. Reconciliation

- **DEL-14-01:** delivery commitment :1 is demonstrated in the product store. The
  "Phase G … gate v0.2 R3" text in :3 records OD-1 and goes to HELP_HUMAN as
  exact old → new.
- **DEL-14-02:** R002 is now a real state reference; delivery commitment :1.
- **DEL-14-03, DEL-14-04:** delivery commitment :1 "in product integration" and
  "through the runtime path".
- **DEL-14-05:** unchanged; its Should items stay open.
- **DEL-07-08:** the panel and the run audit.
- The `deliverable.yaml` code and check pointers follow the code.
- The completeness-map rows FR-CMP-001..006 are for the reconciliation owner.
