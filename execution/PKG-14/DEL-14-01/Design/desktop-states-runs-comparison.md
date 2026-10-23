# Model states, run history and comparison in the desktop (W-12)

**Basis:** `origin/main` at `9d76659e02` (2026-10-23), after model document 0.7.0
(DEL-02-01 Design `model-document-0.7.0.md`). This is revision 2, after two
review rounds (an independent Claude reviewer and Codex). Design before
implementation. Persistence changes get targeted regression tests and dual
review. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings and scope

- **OD-1 (owner, 2026-10-23, "I approve OD-1 through OD-5 as you recommended"):**
  model states, run history, and state and run comparison are in v0.1. This
  settles D-11/DEC-062's "W7 — Forward horizon v0.2" placement for these items.
- **Scope (PRD §23 MVP):** "named immutable model states; analysis runs tied to
  states; basic model-state comparison; basic analysis-run comparison". That
  means FR-CMP-001, -002, -003, -004 and -006 (Must).
- **Out of this unit.**
  - The Should items FR-CMP-005/007 (mapping editor, manual mappings), -008
    (tolerance profiles), -009 (graphical diffs) and -010 (comparison export).
    PRD §24 places them in R3. They stay open, and no Must is narrowed: FR-CMP-006
    is met by automatic stable-id mapping, and FR-CMP-004 by stable ids across
    states.
  - DEL-14-05's output-contract acceptance (SCA-011) stays held. Comparison
    output is not tolerance-checked, and nothing is classified as within or
    outside a tolerance.
- **Reference behaviour (PKG-14 headless):**
  - `core/comparison/model_state/engine.py`;
  - `core/comparison/analysis_run/engine.py`;
  - `core/analysis_runs/compatibility.py`;
  - `schemas/model_state.schema.json`, `analysis_run.v0.3.schema.json`;
  - `core/project_persistence/service.py`.

## 1. Where things stand

- **Store.** The desktop store (`openpipestress-projects.sqlite3`, migrations to
  v11, `apps/desktop/src-tauri/src/lib.rs:743,896`; the migrations are not
  transactional, `:942-950`) keeps one row per project, with one model, one
  result and one analysis-run column. The upsert
  (`ON CONFLICT DO UPDATE`, `:1112-1129`) and the replacement guard (`:1376-1408`)
  run inside `BEGIN IMMEDIATE` (`:1065`).
- **Run identity.** Every solve carries the producer's fixed `run_id`,
  `run:preview-linear-static-001` (`core/product_physics/src/lib.rs:3151`).
- **Run record.** The analysis-run record is built at solve time
  (`workspaceSession.ts:947`). Its `model_state_ref` is the synthetic
  `state:<model_ref>:preview` (`analysisRunCompatibility.ts:164`). The synthetic
  ref is validated in TypeScript (`:237`) and Python (`compatibility.py:155,524`),
  used by `resultExportAdapter.ts:253`, and pinned in fixtures
  (`fixtures/results/*.analysis_run.json`, the persistence fixture).
- **Model hash.** `MechanicsResult` has no model hash. The project row stores the
  session's model hash (`model_hash_json`).
- **Comparison.** It exists only within one run, between hard-coded ids
  (`previewService.ts:654`).
- **Standing.** Numerical standing is Rust's verdict
  (`features/results/resultStanding.ts:1-12`). Reopened runs that depend on a
  registration read `needs_recompute` (`HistoricalRunContext.tsx:102`).

## 2. Model states (FR-CMP-001)

**A state** is a named, immutable snapshot: the canonical JCS text of the model
document (the W-6a bytes) plus its sha256, which is the session's model hash.

**Store (migration v12).** Every statement uses `IF NOT EXISTS`, including the
triggers, because the migrations are not transactional.

```sql
CREATE TABLE IF NOT EXISTS model_states(
  project_id TEXT NOT NULL REFERENCES local_projects(project_id) ON DELETE CASCADE,
  state_id TEXT NOT NULL, state_kind TEXT NOT NULL, created_at TEXT NOT NULL,
  parent_state_id TEXT, model_canonical TEXT NOT NULL, model_hash TEXT NOT NULL,
  record_json TEXT NOT NULL, record_hash TEXT NOT NULL,
  PRIMARY KEY(project_id, state_id), UNIQUE(project_id, model_hash),
  FOREIGN KEY(project_id, parent_state_id) REFERENCES model_states(project_id, state_id));
CREATE TABLE IF NOT EXISTS model_state_labels(
  project_id TEXT NOT NULL, state_id TEXT NOT NULL, state_name TEXT NOT NULL,
  tags_json TEXT NOT NULL, notes TEXT, updated_at TEXT NOT NULL,
  PRIMARY KEY(project_id, state_id),
  FOREIGN KEY(project_id, state_id) REFERENCES model_states(project_id, state_id) ON DELETE CASCADE);
CREATE TRIGGER IF NOT EXISTS model_states_immutable BEFORE UPDATE ON model_states
  BEGIN SELECT RAISE(ABORT, 'MODEL-STATE-IMMUTABLE'); END;
```

**Writes.**
- **Hash check.** Rust re-canonicalizes the submitted snapshot with the same
  unchecked `canonical_json` and recomputes its sha256. It refuses
  (`MODEL-STATE-HASH-MISMATCH`) unless both equal what the webview claims.
- **Duplicates.** `UNIQUE(project_id, model_hash)` enforces no-duplicate
  across windows. Saving an already-saved model names the existing state and
  offers to rename it.
- **Admission.** Every new command passes `command_ingress::admit` with its own
  byte budget.
- **Transactions.** Writes run in `BEGIN IMMEDIATE`, with the project row's
  existing guard. A state requires the project row: if there is none, Save Local
  Project's flow (with its confirmation rules) runs first.

**Record.** `record_json` is a `model_state.schema.json` `ModelStateRecord`:
- `state_kind` is one of `design_snapshot`, `pre_solve_snapshot`,
  `post_solve_snapshot`, `comparison_basis` or `handoff_basis`.
- `model_ref` is a structured `Reference` to the project.
- `parent_state_refs` holds the state the working model was opened from, if any.
- `external_references` is `[]`.
- `unresolved_assumptions` holds the document's unresolved records, mapped:
  `affected_refs` id strings become `Reference` objects, and a product
  provenance record is added.
- `warnings` holds the document validator's warnings.
- `analysis_status` comes from the document.
- `hashes`:
  - `model_payload`, `canonicalization: JCS`;
  - `model_state_record`, computed as the schema's contract states
    (`state_contract_status.canonicalization: sorted_compact_json_payload_hashes`)
    over the record with `hashes` and the label fields removed.
- `immutability_policy`, `professional_boundary` and `provenance`.

**Labels.**
- `state_name`, `tags` and `notes` are required record fields. They live in
  `model_state_labels` and are joined into the emitted record when it is read.
- The record hash excludes them. FR-MOD-004 says users label states with
  flexible naming, and FR-CMP-001 says "tag … without mutation" of the snapshot.
- Label edits are the only update. They are guarded and keep `record_hash`
  unchanged.

**Commands.**
- **Save Model State…** takes a name (required), tags, notes and a kind
  (default `design_snapshot`).
- **Open State** loads the snapshot as the working model, behind the existing
  unsaved-changes guard. Editing it produces a new working model, and the state
  is unchanged.
- **Delete State** is allowed only when no recorded run references it. The FK
  uses `NO ACTION` from runs, so a reference blocks the delete. Project-level
  cascade deletes runs first.
- **There is no project-delete command today.** The cascade only keeps the
  schema consistent.
- **Replacing the project row** with a different model (an existing confirmed
  replacement, `:1376-1408`) keeps the project's states and runs. They are the
  history of that project id, and the History panel shows them.

## 3. Runs tied to states (FR-CMP-002)

**Two kinds of run record:**
1. **Transient.** Solve-time records are unchanged: the current-run slot, the
   synthetic `model_state_ref`, the existing validators and the fixtures. The
   synthetic ref stays as the ref of a record that is not yet bound to a state.
2. **Recorded.** **Record Run** creates a new immutable record revision of the
   transient record, as `analysis_run.v0.3` permits
   (`mutation_policy: changes_create_new_immutable_record_revision`,
   `new_mechanics_run_required_for_record_revision: false`).
   - The revision carries the real `model_state_ref`.
   - It carries the document's `rule_pack_refs` and the libraries resolved from
     `provenance_record.library_ref`, as bare v0.3 `{object_type, ref}`
     references.
   - Its hashes are recomputed per v0.3: `openpipestress_jcs_ijson_v1`, bare
     64-hex values.
   - The received-result bytes are never rewritten.

**Binding check.**
- The state for a recorded run is the project state whose `model_hash` equals
  the model hash the session held when it solved. The session records that hash
  with the transient run, as the project row already does.
- If no such state exists, the user names a new `post_solve_snapshot`, and the
  state and the run are written in one transaction.
- Rust verifies that the state's stored canonical snapshot hashes to that
  model hash. The binding is checked, not asserted.

**TypeScript and Python validators** accept either form:
- a transient record with the synthetic ref, as today;
- a recorded revision whose `model_state_ref` names a stored state, checked
  against that state's model hash, which is passed in.

`analysisRunCompatibility.ts` and `core/analysis_runs/compatibility.py` change
together, with mirrored tests.

**Store (v12 continued):**

```sql
CREATE TABLE IF NOT EXISTS analysis_runs(
  project_id TEXT NOT NULL, recording_id TEXT NOT NULL, state_id TEXT NOT NULL,
  producer_run_id TEXT NOT NULL, created_at TEXT NOT NULL,
  run_record_json TEXT NOT NULL, run_record_hash TEXT NOT NULL,
  result_json TEXT NOT NULL, result_hash TEXT NOT NULL,
  PRIMARY KEY(project_id, recording_id), UNIQUE(project_id, run_record_hash),
  FOREIGN KEY(project_id, state_id) REFERENCES model_states(project_id, state_id) ON DELETE NO ACTION,
  FOREIGN KEY(project_id) REFERENCES local_projects(project_id) ON DELETE CASCADE);
CREATE TRIGGER IF NOT EXISTS analysis_runs_immutable BEFORE UPDATE ON analysis_runs
  BEGIN SELECT RAISE(ABORT, 'ANALYSIS-RUN-IMMUTABLE'); END;
```

- **Identity.** `recording_id` is `runrec:` plus the first 32 hex characters of
  `run_record_hash`. The producer's `run_id` is kept as data. Repeated solves,
  rule-check revisions and other projects never collide.
- **Current-run slot.** Record Run does not touch it, so the project envelope
  hash is unchanged. The slot keeps the transient record.
- **Deletion.** A recorded run can be deleted (Delete Recorded Run). Delete
  operations are the only mutation.
- **Size.**
  - Recording uses the persistence memory guard Save Local Project uses (W-5).
  - The History panel lists runs from columns, without loading `result_json`.

## 4. Comparison (FR-CMP-003, -004, -006)

Both comparisons run in the desktop's TypeScript session. They are
deterministic and use stable ids automatically, with no manual mapping.

### 4.1 State comparison

**Flattening** is complete and fails closed:
- Every top-level member of the document is assigned to an entity family or to
  the single `model` entity.
- **Families:** nodes, pipe segments, sections, materials, supports,
  components, load cases, combinations, reference configurations, lines,
  unresolved assumptions, constraints, equipment interfaces and rule-pack
  references.
- **The `model` entity** holds `project`, `pressure_contract`, `data_boundary`,
  `analysis_status`, `diagnostics`, `schema_version` and `document_kind`.
- **Primitive loads** are entities keyed `<load case id>/<load id>`.
- An unassigned member refuses the comparison with
  `COMPARISON-MEMBER-UNASSIGNED`, so a future member cannot be silently skipped.
- **Invariant:** equal model hashes ⇔ zero non-`unchanged` rows. This is tested.

**Classification follows the reference engine:**
- same stable id in both: `unchanged` or `changed`;
- left only: `removed`; right only: `added`;
- a duplicate id within one side refuses the comparison, as the reference
  raises (`engine.py:226-227`).

**Field paths.**
- `changed` rows list their field paths. Each top-level key of the entity is
  diffed as the reference does (`engine.py:400-416`).
- **Desktop extension: nested paths.** The desktop adds nested paths for
  display.
- **Desktop extension: units.** A `{value, unit}` quantity whose unit differs
  is reported as `unit_changed`, never as a numeric delta. In parity it maps to
  the reference's `unresolved` with `INCOMPATIBLE_UNIT`.

**Parity.** A committed golden of invented state pairs is checked by a pytest.
It runs `compare_model_states` on the same flattened lists, through an adapter
(`family → category`, the entity's `id → stable_id`, unit-bearing fields
declared in `settings`). It requires equal summaries and per-row
classifications on that shared projection. The desktop extensions are tested
separately.

### 4.2 Run comparison

**Row identity** is the reference's
`(result_id, family, object_ref, basis_ref, dimension)`
(`analysis_run/engine.py:512-519`), plus the desktop's `metadata.component`,
`location`, `basis`, `coordinate_system` and `sign_convention`. The desktop
fields keep complete result semantics. Rows map automatically when all of these
are equal.

**Standing gate.** A mapped pair gets a `right − left` delta only when:
- the units are equal;
- both sides' cases stand ordinary by **Rust's verdict** (`numericalResultStanding`,
  the same verdict the Results panel shows). For a combination, every
  constituent case must stand ordinary.
- both runs carry the same result semantic identity (producer contract, route).

Otherwise the pair is `not_compared`, with the reason and the standing.

**Stored runs.**
- A stored run's standing is what `HistoricalRunContext` re-derives from the
  stored record. It never reconstructs live invocation custody from saved
  hashes.
- Registration-dependent runs read `needs_recompute` and compare as
  `not_compared`, and the UI says so plainly.
- The UI uses no wording that implies certification (F-PIP-2).

**Context deltas.**
- Solver version, settings and the load-case and combination sets are compared
  like the reference `_settings_deltas`.
- Solve diagnostics are compared by `(code, affected ref)` as added or removed
  (PRD §15.3).
- Unmatched rows are listed per side.

**Parity.** A pytest runs `compare_analysis_runs` on a committed golden, through
an adapter from desktop rows to `quantity_results`. It covers
`SUPPORTED_RESULT_FAMILIES` and ungated pairs only. The gate, the extra identity
fields and the diagnostics comparison are tested separately.

**Memory.** Comparing two stored runs loads both results. The combined size is
checked against the ingress budget and refused with a named diagnostic if it is
over.

### 4.3 UI

- **History panel:** states (name, tags, kind, created, short hash, run count)
  and the recorded runs under each. It provides Save Model State…, Open State,
  Record Run, Compare… (two states or two runs), rename and tag, and delete.
- **Comparison panel:** summary counts, a filterable table, field paths with
  left and right values, deltas with units, `not_compared` reasons and unmatched
  lists.
- **"Within this run"** is today's comparison, kept. Its hard-coded ids are
  replaced by the user's selection of a case and a combination.

## 5. Consumers

- `stateComparisonHandoffSections.ts` emits real state and recorded-run records
  when the report package includes them. Transient runs keep their present
  form.
- W-6b uses the state store for its basis state.

## 6. Verification

**Rust store tests (persistence; targeted regression):**
- v12 on a v11 store with existing rows, run twice (idempotent);
- state create, the unique-hash refusal and the hash-mismatch refusal;
- `UPDATE` raising at the SQL level on both tables;
- a label edit leaving `record_hash` unchanged;
- delete blocked by a run;
- run and new state written in one transaction, with rollback on injected
  failure;
- two recordings of the same producer `run_id` stay distinct;
- cross-project FK refusal;
- project-row replacement keeping history;
- a stale-window write refused by the existing guard;
- the canonical snapshot round trip giving an equal hash;
- the ingress budgets.

**vitest:**
- the session flows and the guards;
- the validator accepting transient and recorded forms;
- the flattening fail-closed check;
- the standing gate;
- the hash ⇔ zero-change invariant.

**pytest:**
- the parity goldens (§4.1, §4.2);
- the mirrored `compatibility.py` change;
- emitted state and recorded-run records validating against
  `model_state.schema.json` and `analysis_run.v0.3`.

**Native smoke:** gains Save Model State, Record Run and Compare only if
HELP_HUMAN wants them in G10. Any `expected.json` change goes to HELP_HUMAN with
its cause.

**Review:** an independent Claude reviewer and Codex, on the implementation
head.

## 7. Reconciliation and owner-reserved text (to HELP_HUMAN as exact old → new)

1. **DEL-14-01 :3** "Phase G … gate v0.2 R3". Only states, runs, comparison and
   the workspace GUI move to v0.1, which records OD-1. Mapping, tolerance and
   export stay v0.2.
2. **DEL-14-01 :1** asks to "preserve DEC-017/028 strategy", under which SQLite is
   a rebuildable projection of canonical members. In v0.1, states and recorded
   runs live only in the SQLite store. A canonical package export of history
   needs the DEC-028 container (OI-015-OPEN).
   - This is a tension with the commitment text. It goes to HELP_HUMAN as a
     reserved choice:
     - (a) accept SQLite-held history in v0.1 and record the export as open;
     - or (b) add a history export, which needs the container ruling.
   - Recommendation: (a).
3. **DEL-14-03 :1 and DEL-14-04 :1.** Record them as partial: "mapping" and
   "unit-aware delta behavior" are only partly discharged without manual
   mapping and unit conversion. This is reconciliation text, not a change to
   the commitment.
4. **DEL-14-02:** R002's state reference and delivery commitment :1 pointers.
5. **DEL-14-05:** unchanged.
6. **DEL-07-08:** panel pointers.
7. **`deliverable.yaml`:** pointers follow the code.
8. **Completeness map:** the FR-CMP rows are for the reconciliation owner.
