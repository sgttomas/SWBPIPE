# Model states, run history and comparison in the desktop (W-12)

**Basis:** `origin/main` at `9d76659e02` (2026-10-23), after model document 0.7.0
(DEL-02-01 Design `model-document-0.7.0.md`). This is revision 3, after two
review rounds, each by an independent Claude reviewer and Codex. Design before
implementation. Persistence changes get targeted regression tests and dual
review. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings and scope

- **OD-1 (owner, 2026-10-23, "I approve OD-1 through OD-5 as you recommended"):**
  model states, run history, and state and run comparison are in v0.1. This
  settles D-11/DEC-062's "W7 — Forward horizon v0.2" placement for these items.
- **Scope (PRD §23 MVP):** "named immutable model states; analysis runs tied to
  states; basic model-state comparison; basic analysis-run comparison". That
  means FR-CMP-001, -002, -003, -004 and -006 (Must). FR-MOD-006's state and
  run levels are added (§2, §3).
- **Out of this unit.**
  - The Should items FR-CMP-005/007 (mapping editor, manual mappings), -008
    (tolerance profiles), -009 (graphical diffs) and -010 (comparison export).
    PRD §24 places them in R3. They stay open, and no Must is narrowed:
    FR-CMP-006 is met by automatic stable-id mapping, and FR-CMP-004 by stable
    ids across states.
  - DEL-14-05's output-contract acceptance (SCA-011) stays held. Comparison
    output is not tolerance-checked.
- **Reference behaviour (PKG-14 headless):**
  - `core/comparison/model_state/engine.py`;
  - `core/comparison/analysis_run/engine.py`;
  - `core/analysis_runs/compatibility.py`;
  - `schemas/model_state.schema.json`, `analysis_run.v0.3.schema.json`;
  - `core/project_persistence/service.py`.

## 1. Where things stand

- **Store.** The desktop store (`openpipestress-projects.sqlite3`, migrations to
  v11, `apps/desktop/src-tauri/src/lib.rs:743,896`) keeps one row per project,
  with one model, one result and one analysis-run column.
  - The migrations are not transactional (`:942-950`).
  - `PRAGMA foreign_keys = ON` is set at open (`:937`).
  - The upsert (`:1112-1129`) and the replacement guard (`:1376-1408`) run
    inside `BEGIN IMMEDIATE` (`:1065`).
- **Run identity.** Every solve carries the producer's fixed `run_id`,
  `run:preview-linear-static-001` (`core/product_physics/src/lib.rs:3151`).
- **Run record.** It is built at solve time (`workspaceSession.ts:947`), with
  `created_at: null` and the synthetic `model_state_ref`
  `state:<model_ref>:preview` (`analysisRunCompatibility.ts:164`). That ref is
  validated in TypeScript (`:237`) and Python (`compatibility.py:155,524`), used
  by `resultExportAdapter.ts:253`, and pinned in fixtures.
- **Model binding.** The solved model is bound only through the input manifest,
  which embeds `model_payload` (`inputManifestService.ts:79-83`) and is not
  stored. `MechanicsResult` has no model hash.
- **Reopened runs.** `HistoricalRunContext` establishes no standing. It always
  adds `HISTORICAL_INPUT_MANIFEST_MISSING` (`:95`) and flags any non-synthetic
  state ref (`:108`).
- **Comparison** exists only within one run (`previewService.ts:654`).

## 2. Model states (FR-CMP-001; FR-MOD-006 state level)

**A state** is a named, immutable snapshot: the canonical JCS text of the model
document (the W-6a bytes) plus its sha256, which is the session's model hash.

**Store (migration v12).** Every statement uses `IF NOT EXISTS`, including the
triggers and the index. v12 holds the state tables only; the run tables of §3
are v13, added with recorded runs (W-12 S2).

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
  tags_json TEXT NOT NULL, notes_json TEXT NOT NULL, assumptions_json TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  PRIMARY KEY(project_id, state_id),
  FOREIGN KEY(project_id, state_id) REFERENCES model_states(project_id, state_id) ON DELETE CASCADE);
CREATE TRIGGER IF NOT EXISTS model_states_immutable BEFORE UPDATE ON model_states
  BEGIN SELECT RAISE(ABORT, 'MODEL-STATE-IMMUTABLE'); END;
```

**Writes.**
- **Hash check.** Rust re-canonicalizes the submitted snapshot with the unchecked
  `canonical_json` and recomputes its sha256. It refuses
  (`MODEL-STATE-HASH-MISMATCH`) unless both equal the claim.
- **One transaction.** The state row and its labels row are written in one
  `BEGIN IMMEDIATE` transaction, with the project row's existing guard.
- **Duplicates.** `UNIQUE(project_id, model_hash)` enforces no-duplicate across
  windows. Saving an already-saved model names the existing state.
- **Admission.** Every new command passes `command_ingress::admit` with its own
  byte budget.
- **Project row required.** A state requires the project row. If there is none,
  Save Local Project's flow runs first.

**Record.** `record_json` is a `model_state.schema.json` `ModelStateRecord`.
It is built by these mappings and validated by a populated witness:

| Field | Mapping |
|---|---|
| `state_id` | `state:<random token>` |
| `state_kind` | `design_snapshot`, `pre_solve_snapshot`, `post_solve_snapshot`, `comparison_basis` or `handoff_basis` |
| `created_at` | UTC ISO time, millisecond precision, as recorded runs use |
| `model_ref` | `{Project, <project id>}` |
| `parent_state_refs` | `[{ModelState, <parent>}]` or `[]` |
| `external_references` | `[]` |
| `unresolved_assumptions` | the model's `unresolved` records plus the state-level ones (labels row). Each becomes `AssumptionRecord` with `assumption_id`, `statement` (an open issue is prefixed "Open issue: "), `status` (`unresolved`/`resolved`/`rejected` as is), `affected_refs` and a product `provenance`. Entity ids become `{Model, "<project id>#<entity id>"}`, because `Reference.object_type` has no entity types. `tags`/`notes` are dropped |
| `warnings` | the document validator's warnings, as `Diagnostic` records with model-qualified refs |
| `analysis_status` | an array: always `HUMAN_REVIEW_REQUIRED`, plus `MODEL_INCOMPLETE` when the validator has a blocking finding. The document's free-text `analysis_status` object is not copied |
| `tags` | `StateTag {tag, tag_kind: user_label, provenance}` per label tag |
| `notes` | `StateNote {note_id, note_type: design_note, statement, visibility, provenance}` per label note |
| `hashes` | `model_payload` (`canonicalization: JCS`), and `model_state_record` computed as the contract states (`sorted_compact_json_payload_hashes`) over the record with `hashes` and the label-derived fields removed (`state_name`, `tags`, `notes`, state-level assumptions) |
| `immutability_policy`, `professional_boundary`, `provenance` | the schema's values and a product provenance |

**Serializer pin.** The sorted-compact serializer is pinned to the Python
persistence service's exact behaviour (`core/project_persistence/service.py`,
including its `ensure_ascii` setting). A non-ASCII test vector checks Rust and
Python agree.

**Labels.**
- `state_name`, tags, notes and state-level assumptions live in the labels row.
  They are joined into the emitted record when it is read, and they are the
  only editable part.
- The record hash excludes them, so a label edit leaves `record_hash`
  unchanged.
- **HELP_HUMAN, 2026-10-23: labels outside the record hash "is right": a label
  edit isn't a new state.** The emitted record's
  `changes_create_new_model_state` therefore applies to the snapshot and the
  immutable fields, not to the labels. The History panel says so where labels are edited:
  "Renaming, tagging or adding notes does not change the saved state."

**Commands.**
- **Save Model State…** takes a name (required), tags, notes, state-level
  assumptions and a kind (default `design_snapshot`).
- **Open State** loads the snapshot as the working model, behind the
  unsaved-changes guard. Edits produce a new working model.
- **Delete State** is blocked while any recorded run or any child state
  references it (both FKs are `NO ACTION`). The UI names the blockers.
  Project-level cascade deletes runs, then child states, in one statement.
  There is no project-delete command today.
- **Replacing the project row** with a different model (the existing confirmed
  replacement, including `PROJECT-ID-EXISTS`, `:1376-1408`) keeps the
  project's states and runs. The confirmation prompt states how many saved
  states and recorded runs will remain attached to that project id.

**As built (S1, states).** `apps/desktop/src-tauri/src/model_states.rs`,
`apps/desktop/src/services/modelStateService.ts` and `apps/desktop/src/features/history/`.
- **Versions.** S1 made `STORE_SCHEMA_TARGET_VERSION` 12 (S2 made it 13). A store whose
  `user_version` is above the target was written by a newer build. It is
  refused (`STORE-SCHEMA-NEWER`) before any pragma or migration writes;
  before S1 it was opened as current.
- **Admission.** A state save applies the one-current-version check alone
  (`check_model_document_version`, the version part of
  `check_model_document_for_persist`). A state may therefore hold a model with
  blocking slot findings: its record says `MODEL_INCOMPLETE`, and it is kept
  as a comparison basis. The record mapping still refuses, by name, any slot
  it cannot map. Project create and save keep #1565's blocking-slot refusal,
  and opening a state keeps it too, so an incomplete state is never opened.
- **Hash forms.** The `model_hash` column holds the session's form
  (`sha256:<hex>`). The record's `hashes` values are bare hex, as the Python
  service writes them. `record_json` stores the record without the label
  fields; `record_hash` is over that record without `hashes`.
- **Integrity.** Every list, summary and read verifies the record against its
  hash and its `hashes`. It then checks each identity field the record
  duplicates against the row's columns: project, state id, kind,
  `created_at`, parent ref and model hash. The record digest leaves `hashes`
  out, so the whole `hashes` array must equal the two expected descriptors
  (algorithm, canonicalization, scope, payload ref and value). A duplicate,
  extra or missing entry is refused. A read also checks the snapshot:
  its JCS hash and its project id. Any difference is refused as
  `MODEL-STATE-INTEGRITY`, naming the state; a list with such a row is
  refused.
- **Memory (S2 repairs r1 to r3).** All five state commands are metered as
  §3's As built (S2) describes (`stored_read.rs`): `save_model_state`,
  `list_model_states`, `read_model_state`, the label edit and the delete.
  - **Before loading.** Each takes memory-pool leases sized from the stored
    lengths before loading anything: records, snapshots, label columns and
    the child and run lists. It measures and loads in one transaction.
  - **Hash texts.** It reserves the record's sorted-compact hash text at its
    exact escaped length before writing it. The writer that counts the length
    is the one that writes the text.
  - **Holding.** It holds its leases until its response is serialized.
  - **Refusal.** A command whose lease the pool refuses is refused with
    `STORE-READ-MEMORY-UNAVAILABLE`, and nothing large is loaded.
- **Label bounds (r2, r3).** Every label write (Save Model State, Record Run,
  both label edits) keeps at most 256 tags, notes and assumptions together.
  - **JSON size.** It also keeps at most 196,608 bytes (192 KiB) of the whole
    labels object as the label edit carries it, serialized with its escapes:
    `{name, tags, notes, assumptions}`, name included. A name of 196,602 NULs
    serializes to 1,179,667 bytes and is refused.
  - **Editability.** A compile-time assertion keeps these 192 KiB, plus a
    project id at the list's whole 4 KiB budget and 1 KiB of fixed members,
    within the label edit's 256 KiB argument budget. So any kept labels can be
    edited, which a test checks for both edits.
  - **Reads.** Reads still price whatever is stored.
- **Serializer.** Keys sort by code point, and i64/u64 integers are written
  in decimal. No field of `model_state.schema.json` has a number type, so the
  record hash fails closed on any other number (a float, or an integer outside
  i64/u64), with `MODEL-STATE-RECORD-NUMBER-UNSUPPORTED`. It does not
  approximate Python's `repr(float)`, whose tie-breaking Rust's shortest-digit
  formatter does not share (Codex r1).
- **Parity checks.** `tools/fixtures/sorted_compact_json_vectors.py` writes
  `fixtures/persistence/sorted_compact_json_vectors.json` from `service.py`:
  the vectors and the refused numbers. A witness of a populated record and
  of an empty-label record is written by the save and read path
  (`model_state_record_witness.json`). The Rust tests and
  `tests/test_model_state_record_parity.py` check both files.
- **Immutability in SQL.**
  - `model_states_immutable` refuses every `UPDATE`.
  - `model_states_no_replace` (v12, `BEFORE INSERT`) refuses an insert that
    meets an existing row of the project on `state_id` or `model_hash`. It
    fires before conflict resolution, so `INSERT OR REPLACE` and `REPLACE`
    can neither overwrite a snapshot nor cascade-delete its labels.
  - **Deleted identity.** A state id is a 128-bit random token and is never
    reused. Re-saving a deleted state's model creates a new state, which
    UNIQUE allows after the delete.
  - **Concurrent saves.** Two windows saving one model at once give one
    `saved` and one `already_saved`.
- **Labels.** The state-level assumptions use the model's
  `unresolved_assumptions` vocabulary. An empty `affected_refs` names the state
  (`{ModelState, <state id>}`). An id may not repeat a model assumption id.
  Notes become `StateNote`s `<state id>:note:<n>`, `visibility: private`.
  Every provenance in the record is `review_status: pending`.
- **Commands.** `save_model_state`, `list_model_states`, `read_model_state`,
  `edit_model_state_labels` and `delete_model_state`. The save carries the
  canonical text, the claim and the session's base (`expected_base`). The four
  small commands take 4 KiB of arguments, 256 KiB for a label edit
  (`INGRESS-ARGS-TOO-LARGE`). A label edit names the labels' `updated_at` it
  read (`MODEL-STATE-LABELS-MOVED` otherwise).
- **Prerequisite project save.** When Save Model State… first runs Save
  Local Project's flow, the state follows the save that landed. After "Save
  as a new project" that is the renamed working model, its new project id
  and base, with no parent state from the old project. Cancel saves neither.
- **Delete.** A state's deletion is blocked by its child states and, since S2, by
  its recorded runs.
- **Open State.** The desktop had no unsaved-changes guard. S1 adds one, shared
  by every command that replaces the working model (DEL-07-11 Design
  `model-document-open-and-native-smoke.md`): Cancel (the default), Save Local
  Project first, or Discard. A `MODEL_INCOMPLETE` state is kept as a comparison
  basis but is not opened; the History panel says why. Opening starts a new project session
  with empty undo and redo stacks, so undo never reaches the previous model.
  The state's hash becomes the saved basis, and the state is the parent of the
  next state saved from that session.
  - **Binding.** The session becomes the state's project. Its stored base is
    one the session itself saw for that project, never one read afresh from
    the store, so another window's save still meets the stale-window check:
    - the base it holds now, when that is the project's (no save, or the
      guard's Save first saved in place);
    - otherwise the base captured before the guard, when that was the
      project's (Save first went to "Save as a new project");
    - otherwise none, so a later save asks.
    The project summary and envelope hash follow the base.
  - **Library references.** The adopted snapshot's library references are
    checked as a project open checks them (#1566). The warnings join the
    message only while the session is still the one that opened the state.
- **Browser preview.** It keeps no states (SQLite-only history) and refuses
  every state command (`MODEL-STATE-STORE-UNAVAILABLE`).

## 3. Runs tied to states (FR-CMP-002; FR-MOD-006 run level)

**Two kinds of run record:**
1. **Transient.** Solve-time records are unchanged: the current-run slot, the
   synthetic `model_state_ref`, the existing validators and the fixtures.
2. **Recorded.** **Record Run** creates a new immutable record revision of the
   transient record, as `analysis_run.v0.3` permits
   (`mutation_policy: changes_create_new_immutable_record_revision`,
   `record_revision_identity: analysis_run_record_sha256`). The revision:
   - sets `created_at` to the recording time. Rust allocates it inside the
     `BEGIN IMMEDIATE` transaction as the later of the current UTC time
     (millisecond ISO) and one millisecond after the project's latest recorded
     `created_at`. Every recording event therefore has its own revision hash,
     even when clock readings are equal. Rust then computes the v0.3 hashes. A
     parity test checks them against the TypeScript builder;
   - sets the real `model_state_ref` `{ModelState, <state id>}`;
   - takes `rule_pack_refs` from the input manifest's `active_rule_packs` (what
     the solve used, not the document's identity-only references) and
     `library_refs` from the model's `library_ref`s, as the model document's
     one walk finds them;
   - drops the `created_at_unavailable` determinism note, in both TypeScript
     (`analysisRunCompatibility.ts:169` hard-codes it today) and Python
     (`compatibility.py:147` already drops it when `created_at` is set);
   - is hashed per v0.3 (`openpipestress_jcs_ijson_v1`, bare 64-hex);
   - never rewrites the received-result bytes.

**Binding (checked, not asserted).**
- At solve, the session already builds the input manifest, which embeds the
  model payload. The transient run keeps the manifest's canonical bytes in
  memory.
- Record Run submits them. Rust then:
  - re-canonicalizes the submitted manifest bytes and requires them to be
    unchanged;
  - checks that their sha256 equals the record's
    `reproducibility.input_manifest_hashes[0].value`
    (`analysisRunCompatibility.ts:169`);
  - computes the JCS sha256 of the manifest's `model_basis.model_payload` (the
    manifest stores no model hash, `inputManifestService.ts:78-112`) and
    requires it to equal the state's `model_hash`.
- **Restored slot.** A current-run slot restored from the project row has no
  manifest. For it, Record Run is unavailable with the reason
  `RUN-RECORD-MANIFEST-UNAVAILABLE`: "re-solve to record this run".
- The recorded run stores the manifest bytes, so the binding stays checkable.
- The state is the project state with that model hash. If there is none, the
  user names a new `post_solve_snapshot`, and the state, its labels, the run
  and its labels are written in one transaction.

**Validators (TypeScript and Python, changed together with mirrored tests)**
accept either form:
- a transient record with the synthetic ref, as today;
- a recorded revision whose ref names a stored state, checked against that
  state's model hash, which is passed in.

`HistoricalRunContext` accepts real state refs from recorded runs, verified
against the stored state, and keeps flagging any other mismatch.

**Store (v13, with recorded runs):**

```sql
CREATE TABLE IF NOT EXISTS analysis_runs(
  project_id TEXT NOT NULL, recording_id TEXT NOT NULL, state_id TEXT NOT NULL,
  producer_run_id TEXT NOT NULL, created_at TEXT NOT NULL,
  run_record_json TEXT NOT NULL, run_record_hash TEXT NOT NULL,
  input_manifest_json TEXT NOT NULL, result_json TEXT NOT NULL, result_hash TEXT NOT NULL,
  PRIMARY KEY(project_id, recording_id), UNIQUE(project_id, run_record_hash),
  FOREIGN KEY(project_id, state_id) REFERENCES model_states(project_id, state_id),
  FOREIGN KEY(project_id) REFERENCES local_projects(project_id) ON DELETE CASCADE);
CREATE INDEX IF NOT EXISTS analysis_runs_by_state ON analysis_runs(project_id, state_id);
CREATE TABLE IF NOT EXISTS analysis_run_labels(
  project_id TEXT NOT NULL, recording_id TEXT NOT NULL, run_name TEXT NOT NULL,
  tags_json TEXT NOT NULL, notes_json TEXT NOT NULL, assumptions_json TEXT NOT NULL,
  updated_at TEXT NOT NULL, PRIMARY KEY(project_id, recording_id),
  FOREIGN KEY(project_id, recording_id) REFERENCES analysis_runs(project_id, recording_id) ON DELETE CASCADE);
CREATE TRIGGER IF NOT EXISTS analysis_runs_immutable BEFORE UPDATE ON analysis_runs
  BEGIN SELECT RAISE(ABORT, 'ANALYSIS-RUN-IMMUTABLE'); END;
```

- **Identity.** `recording_id` is `runrec:` plus the first 32 hex characters of
  `run_record_hash`. The producer's `run_id` is kept as data.
  - Two recordings of identical solves differ in their transaction-allocated
    `created_at`, which is unique per project at millisecond resolution. Should
    two hashes still coincide, the UNIQUE constraint refuses rather than
    overwriting.
  - A test records identical solves from two connections with the clock frozen.
- **Labels and FR-MOD-006.** Run-level assumptions, name, tags and notes live
  in `analysis_run_labels`, which holds editable metadata outside the record
  hash. `analysis_run.v0.3` has no assumption slot, so they are shown with the
  run and in its report sections but are not in the v0.3 record.
- **Current-run slot.** Record Run does not touch it, so the project envelope
  hash is unchanged.
- **Size.** Recording uses Save Local Project's memory guard (W-5). The History
  panel lists runs from columns, without loading `result_json`.

**As built (S2, recorded runs).** `apps/desktop/src-tauri/src/analysis_runs.rs`
(tests in `analysis_runs_tests.rs`), `apps/desktop/src/services/analysisRunService.ts`,
the History panel and `useModelStateHistory.ts`.
- **Versions.** `STORE_SCHEMA_TARGET_VERSION` is 13 (`store-v13-analysis-runs-tables`).
  The tables, index and trigger are the SQL above, plus `analysis_runs_no_replace`
  (`BEFORE INSERT`, on `recording_id` or `run_record_hash`, as S1's trigger). A v14
  store is refused (`STORE-SCHEMA-NEWER`).
- **Commands.** `record_analysis_run` is admitted as `save_local_project` is (the
  per-invocation budget through its lease). `list_analysis_runs`,
  `read_analysis_run` and `delete_analysis_run` take 4 KiB of arguments, and
  `edit_analysis_run_labels` 256 KiB. The browser preview refuses them all
  (`MODEL-STATE-STORE-UNAVAILABLE`).
- **Request.** It carries the transient record, the result's JSON text (stored
  as given), the manifest's canonical text, the session's base, and optionally
  the state the session expects (`state_id`, checked:
  `ANALYSIS-RUN-STATE-MISMATCH`). When no state holds the solved model, it names a
  `post_solve_snapshot` (`new_state`); without one the request is refused with
  `ANALYSIS-RUN-STATE-REQUIRED`. The snapshot is the manifest's model payload. Its
  parent is the session's basis state.
- **Checks before the store.**
  - `ANALYSIS-RUN-MANIFEST-MISMATCH`: the manifest is not its own canonical text;
    its hash is not `input_manifest_hashes[0]`; that entry's ref is not
    `input_manifest_refs[0]`; or the manifest is not the project's.
  - `ANALYSIS-RUN-NOT-TRANSIENT`: the record is not a v0.3 record with
    `created_at: null` and the synthetic ref, or its own checksum does not verify.
    A recorded revision is never recorded again.
  - `ANALYSIS-RUN-RESULT-MISMATCH`: the result's checked hash or ref is not the
    record's `received_result`, or the result is not the record's run of the
    project.
  - `ANALYSIS-RUN-CHECKSUM-MISMATCH`: the record's whole `hashes` inventory is
    not exactly the v0.3 descriptors. That means the record checksum, then the
    received-result checksum, each with exactly its algorithm, profile, scope,
    payload ref and value, and nothing else. Each result ref must also carry
    exactly its row's checksum descriptor. The record digest leaves its own
    checksum out, so only this check sees a mislabelled algorithm or profile
    (Codex r1).
  - `ANALYSIS-RUN-MANIFEST-MISMATCH` also covers every record member the
    manifest fixes, derived from the manifest and never from the record:
    - its ref and checksum descriptor;
    - the solver name, version and build ref;
    - the settings ref and the unit-system ref;
    - the load basis (the manifest's load cases, then its combinations);
    - the rule-pack and library references, which are empty in a transient
      record.
  - Rust does not re-derive the rest of the record's semantics; the validators
    do.
- **The revision.**
  - `rule_pack_refs` are `{RulePack, "<rule_pack_id>@<version>"}`, in the
    manifest's order (the manifest sorts them when it is built).
  - `library_refs` are `{Library, "library:<kind>/<id>[@<version>][#<record id>]"}`,
    deduplicated and sorted by code point.
    - **Sites.** The sites are every `library_ref` object at any depth and
      under any owner, found by the model document's one walk. Rust reuses
      `model_document_save::library_ref_sites`, the native save's walk. The
      TypeScript builder uses `libraryRefSites`, and Python mirrors it as
      `library_ref_sites`.
    - **Normalization.** Kind, id and record id are normalized as that walk
      does (`library_ref_identity`): Rust's White_Space trim (`trimLikeRust`
      in TypeScript, the same 25 characters in Python), with nothing when a
      value is blank or not text.
    - **Version.** `library_version` is kept verbatim when it is text, as the
      store check compares it.
    - **Null identities.** A site whose kind or id is blank or not text (a
      null `identity` in the table) names no library and is omitted. The
      state's hashed model payload still holds it, and the open-time store
      check reports it as `MODEL-LIBRARY-REF-UNRESOLVED`.
    - **Unknown kinds** are kept, as the native walk keeps them.
    - **Pinning.** Rust, TypeScript and Python each pin this to
      `fixtures/model_document/library_ref_sites.json`. Each test also pins
      a shared White_Space vector: U+FEFF and U+001F stay, and U+3000,
      U+0085 and U+2003 are trimmed.
    - **Before the alignment with #1595.** Only a `library_ref` directly under a
      `provenance_record` was collected, untrimmed. The committed witness's
      model has no `library_ref`, so it is unchanged.
  - Rust hashes with `open_pipe_stress_canonical_json::canonical_json_checked_v1_text`,
    now a direct dependency of the desktop crate.
  - Parity: the TypeScript builder `recordedAnalysisRunRevision` and Python's
    `recorded_revision` rebuild the store's witness
    (`fixtures/persistence/recorded_run_witness.json`) exactly. The store writes
    the witness from a transient record that the TypeScript session builder made
    for a real producer result (`transient_run_input.json`).
- **Integrity.**
  - **List.** The list reads each run's record, but never its result or
    manifest. It checks the record hash and the identity columns against the
    record:
    - the recording id against the hash;
    - the state id against `model_state_ref`;
    - the producer run id, `created_at` and the result hash against
      `received_result`;
    - the project against the record's `unit_system_ref`, which names the
      project and the manifest hash;
    - the whole checksum inventory, the manifest's ref and checksum, and the
      settings and unit-system refs against the descriptors that the verified
      fields, the project and the manifest hash give.
  - **Read.** A read also checks:
    - the stored result's checked hash and each row's checksum descriptor;
    - the manifest's canonical form, its hash against the record, and its
      project;
    - every record member the manifest fixes (as at Record Run, with the
      rule-pack and library references);
    - the manifest's model payload hash against the state's model hash.
  - A coherently resealed forgery, one that names another state or contradicts
    its manifest, is refused there.
  - Any difference is refused as `ANALYSIS-RUN-INTEGRITY`, naming the run.
- **Delete.**
  - **State.** A state with recorded runs is not deleted
    (`MODEL-STATE-DELETE-BLOCKED`, naming the runs). The `NO ACTION` foreign key
    refuses the deletion in SQL as well.
  - **Run.** A run is deleted only as a whole row with its labels; no other row
    depends on a run.
  - **Project.** Deleting the project row removes its runs, child states,
    states and all their labels in one statement. A Replace keeps all of them.
  - **Replace prompt.** It says how many saved states and recorded runs remain
    attached.
- **Labels.** The run's name, tags, notes and run-level assumptions follow
  S1's label rules (`ANALYSIS-RUN-LABELS-INVALID`), with `run_name` required. A
  label edit names the `updated_at` it read (`ANALYSIS-RUN-LABELS-MOVED`).
  "Renaming, tagging or adding notes does not change the recorded run."
- **Validators.**
  - TypeScript `validateAnalysisRunV03(…, recordedState)` and Python
    `validate_analysis_run_v0_3(…, recorded_state=…)` read the recorded form. They
    check the state ref, `created_at`, the stored manifest's project, and its
    model payload against the state's model hash
    (`ANALYSIS_RECORDED_MODEL_STATE_HASH_MISMATCH`).
  - **Derived from the manifest (repair r1).** These checks run first, and every
    expected value comes from the stored manifest, never from the record:
    - the manifest's ref and checksum (`…_INPUT_MANIFEST_MISMATCH`);
    - the solver identity and build ref (`…_SOLVER_MISMATCH`);
    - the settings ref (`…_SETTINGS_MISMATCH`);
    - the unit-system ref (`…_UNITS_MISMATCH`);
    - the load basis (`…_LOAD_BASIS_MISMATCH`);
    - the rule-pack and library references (`…_REFERENCES_MISMATCH`).
  - **The load basis.** The recorded form's load basis is the manifest's. A
    caller's basis that differs from it is refused.
  - **Equality.** References compare structurally in all three languages:
    member order is not significant, while array order and extra members are.
  - Each form is refused in the other's place. Transient records and their
    fixtures validate as before.
- **`HistoricalRunContext`.**
  - `buildRecordedRunContext` (`designation: recorded_run`) accepts the real
    state ref of the stored state it was read with, and adds no
    `HISTORICAL_INPUT_MANIFEST_MISSING`. It still flags every other mismatch.
  - A project row's saved run keeps today's checks.
  - The History panel's "Check record" shows its findings. It establishes no
    standing and does not replace the session's result.
- **UI.**
  - **Placement.** Record Run… sits in the History panel. It goes through the
    one-owner check and the busy refusal, and is disabled with a stated reason.
  - **Unavailable.** Without a solve it shows `RUN-RECORD-NO-RUN`. A slot
    restored from the project row (no manifest) shows
    `RUN-RECORD-MANIFEST-UNAVAILABLE`: "re-solve to record this run".
  - **Prerequisite project save.** When Record Run first runs Save Local
    Project's flow, it follows the save that landed. After "Save as a new
    project" the solve is of the old project id, so nothing is recorded and the
    message says why.
  - **Current-run slot.** Record Run never changes it, so the project envelope
    hash is unchanged.
  - **Results view.** The producer's `run_id` is unchanged and recorded runs are
    not shown in the results view, so its per-run state (page, filters, selected
    row) keys on nothing new. A re-solve and a recording keep the page
    (`recordRun.session.test.tsx`).
- **Memory (repairs r1 to r3, `stored_read.rs`).** Ten commands are metered:
  the five state commands, the five run commands (`record_analysis_run`,
  `list_analysis_runs`, `read_analysis_run`, the run label edit and the run
  delete).
  - **Leases.** Each reserves its work before it makes it, through interactive
    memory-pool leases: Save Local Project's ingress admission (the pool, fresh
    headroom, and the per-invocation budget over the command's live leases
    together). The leases add to the command's argument lease.
  - **Before loading.** A command takes, with `octet_length` and `COUNT`, the
    byte lengths of every column it will load and of the state's child and run
    lists, without loading them. It then reserves:
    - loading each column: records, results, manifests, snapshots, and every
      label column (name, tags, notes, assumptions), as SQLite's value plus the
      copied `String`;
    - the relationship lists (child and blocker ids) it materializes;
    - a fixed 512 KiB overhead.
  - **One snapshot.** The measurement and every load it prices run in one
    transaction. The reads use `BEGIN DEFERRED`, whose first statement fixes
    the WAL snapshot. The edits, deletes, Save and Record Run use their
    `BEGIN IMMEDIATE`. The metered readers take a `Transaction`, so a load
    outside the measured snapshot does not type-check. A row another connection
    commits after the measurement is not loaded.
  - **Before parsing.** Each JSON text is prescanned after it loads and before
    it is deserialized. Text the prescan finds invalid is refused before
    deserialization as `MODEL-STATE-INTEGRITY` or `ANALYSIS-RUN-INTEGRITY`
    (for Record Run, as `ANALYSIS-RUN-RESULT-MISMATCH` or
    `ANALYSIS-RUN-MANIFEST-MISMATCH`), naming the row. Valid text reserves its
    parse at the prescan's bound.
  - **Before building a hash or canonical text.** Every text built to hash or
    compare is counted first, without building it:
    - the record hash's sorted-compact text (`sorted_compact_length`);
    - the JCS texts of a snapshot, a manifest and a model payload
      (`jcs_length`);
    - the checked texts of a run record, a result and each result row (their
      serde text, its checked parse and the JCS text).

    The text is then reserved at that exact length and built:
    - the sorted-compact text, and serde texts, into a buffer of exactly that
      capacity;
    - the JCS text through the canonicalizer's growing buffer, reserved at six
      times the length for its growth and per-string escapes.

    Escapes make a character up to six bytes, and a number's canonical form up
    to about five times its stored form. So no text is priced from a stored
    length. The record hash writes the record without `hashes`, without
    copying it. A text whose counted length differs from the stored text's is
    not canonical and is refused without building it.
  - **Messages.** A response message that quotes stored names (a delete's
    receipt, a save's or Record Run's message) is reserved at its exact length
    and built once.
  - **The response.** The response is serialized at its exact length, counted
    first without allocating, into a buffer reserved at that length.
  - **Holding.** A value read or built from the store is a `Metered` value
    that cannot be taken apart. It is borrowed, mapped or joined with its
    reservations kept, and it leaves them only when it is serialized. An
    unmetered caller can take its value only when it holds no lease. So:
    - a list's labels, a read's state summary and the current summary of an
      edit, while its record is loaded, stay reserved while they are live;
    - a delete receipt, a save outcome and a Record Run outcome are held
      through serialization.
  - **Refusal.** A refusal is `STORE-READ-MEMORY-UNAVAILABLE`, naming what was
    to be read or built, and nothing more is loaded. All leases are released
    when the command returns.
  - **Lists.** A list reserves its largest record and the sum of the labels and
    relationship lists it returns. It then verifies records one at a time:
    - each record's parse and hash texts are reserved after it loads;
    - they are released, with the record, once it is verified.

    It never loads a result or a manifest.
  - **Record Run and Save.**
    - The submitted manifest's and result's parses, and every canonical text,
      are reserved before they are made.
    - Every summary they read is metered: the named or found state, the
      parent, the new state, the run.
    - The new record's hash text and serialized text are reserved at their
      exact lengths.
  - **Tests.** Counting-allocator tests with pool accounting
    (`analysis_runs_tests.rs`).
    - **Large rows** (3 MiB snapshots, results, records and run names; 1024
      state notes), read under a 2 MiB pool:
      - the same read of small rows is admitted;
      - the large one is refused with its column unloaded (allocator peak
        below 768 KiB) and every lease released.
    - **Invalid JSON.** 1 MiB of prescan-invalid JSON stored in the run
      record, result or labels, or in the state record or snapshot, is refused
      before it is parsed. The peak stays below 16 times its length.
    - **Escaped record (r3).** A normally saved state's 3 MiB U+007F
      assumption escapes to an 18 MiB record-hash text:
      - its list, read and label edit, and the read of a run recorded against
        it, reserve the escaped text and peak within their reservations;
      - Record Run against it and both saves of it (existing and new) reserve
        the escaped text (their peak is not asserted).

      Codex r3's probe peaked at 62,948,320 bytes against 34,134,656
      reserved.
    - **Snapshot (r3).** Another connection commits a 3 MiB run, or an
      18 MiB-escaped state, between a list's measurement and its loads. The
      list returns the rows it measured, within its reservation under the
      2 MiB pool.
    - **Holding (r3).**
      - An edit of a state with a stored 3 MiB name still holds the name's
        reservation when it reserves the record.
      - State and run delete receipts quoting a 3 MiB name are held through
        serialization.
    - **Labels (r3).** The 196,602-NUL name is refused.
    - **Exact lengths (r3).** Over 2,000 adversarial values (controls, DEL,
      escapes, separators, non-BMP characters, extreme integers and floats),
      the length counters equal the texts' lengths. So does the labels
      object's.
    - **Mutations.** Each r3 test fails when its defect is put back: the
      snapshot broken, the hash priced from a sixth of its length, the current
      summary dropped early, the receipt's message unreserved.
    - **Large pool.** Each tested read, list, label edit and delete
      serializes its response within its held reservation, with its peak
      within its reservations' peak.
  - **S3.** §4.2's comparison of two stored results reuses this pricing.
  - **Residual (not bounded by this pricing):**
    - **After the command.** The `Response` handed to Tauri, and the IPC
      copies made after the command returns.
    - **Argument lease.** These are working memory within the request's
      argument lease, not reserved separately:
      - Record Run's transient record clones (the hash projection and the
        recorded revision);
      - the `Value` of a new state's record, built from its snapshot (mapped
        assumptions, validation warnings).

      Review r2 measured about 26.0 MiB against a 10.1 MiB lease for Record
      Run's large case before r2; this has not been re-measured since r2's and
      r3's reservations.
    - **Refusals.** A refusal message that quotes stored names, such as a
      blocked delete naming its blockers, outlives the command's leases. It is
      built once, at its exact length.
    - **SQLite.** SQLite's own allocations are priced by the load estimate
      (three envelopes of each loaded column). The tests' counting allocator
      counts Rust allocations, not SQLite's.
    - **Small allocations.** The length counters allocate only each object's
      sorted key list and each number's short text: small, and not
      proportional to escapes.

## 4. Comparison (FR-CMP-003, -004, -006)

Both comparisons run in the desktop's TypeScript session. They are
deterministic and use stable ids automatically, with no manual mapping.

### 4.1 State comparison

**Flattening** is complete and fails closed:
- Every top-level member of the document is assigned to an entity family or to
  the single `model` entity.
- **Families:** nodes, pipe segments, sections, materials, supports,
  components, load cases (excluding `primitive_loads`), combinations,
  reference configurations, lines, unresolved assumptions, constraints,
  equipment interfaces and rule-pack references.
- **Primitive loads** are entities keyed `<load case id>/<load id>`.
- **The `model` entity** holds `project`, `pressure_contract`, `data_boundary`,
  `analysis_status`, `diagnostics`, `schema_version` and `document_kind`. It
  also holds **the ordered id list of every collection**, so a reordering is a
  `changed` model entity.
- **Fail closed.** An unassigned member refuses the comparison with
  `COMPARISON-MEMBER-UNASSIGNED`.
- **Invariant:** equal model hashes ⇔ zero non-`unchanged` rows. This is tested,
  including a reorder-only pair.

**Classification follows the reference engine:**
- same stable id in both: `unchanged` or `changed`;
- left only: `removed`; right only: `added`;
- a duplicate id within one side refuses the comparison
  (`engine.py:226-227`).

**Field diff.**
- The desktop diff skips only the entity's own id field. The reference skips
  `ref`, `reference`, `entity_id` and `stable_id` (`engine.py:401`), so the
  parity adapter projects the same way.
- **Desktop extensions** (nested field paths, and `unit_changed` for a quantity
  whose unit differs) are tested separately. In parity, `unit_changed` maps to
  the reference's `unresolved` with `INCOMPATIBLE_UNIT`.

**Parity.** A committed golden of invented state pairs is checked by a pytest.
It runs `compare_model_states` on the same flattened lists, through an adapter:
- `family` becomes `category`;
- the entity id becomes `stable_id`;
- unit-bearing fields are declared in `settings`.

It requires equal summaries and per-row classifications on that shared
projection.

### 4.2 Run comparison

**Row identity** is the reference's
`(result_id, family, object_ref, basis_ref, dimension)`
(`analysis_run/engine.py:512-519`), plus the desktop's `metadata.component`,
`location`, `basis`, `coordinate_system` and `sign_convention`. Rows map
automatically when all of these are equal.

**Standing gate (conservative, whole run).** A mapped pair gets a `right − left`
delta only when all three hold:
- **Units:** they are equal.
- **Envelope verdict:** each run's `numericalResultStanding` result has
  `eligible === true` (status `integrity_checked`).
  - It is computed by `numericalResultStanding(result, snapshot)`
    (`resultStanding.ts:109-119`) for that run's result and its state's
    snapshot.
  - A current run uses its live verdict. A recorded run is evaluated from its
    stored result and stored state snapshot, plus the stored-hash checks.
  - That function needs no registration outside the pressure-source and
    retained routes. Runs on those routes read `needs_recompute`, compare as
    `not_compared`, and the UI says so. Live invocation custody is never
    reconstructed from saved hashes.
  - The verdict is per envelope, not per case: `numericalResultStanding`
    aggregates cases (`reader_verdict.rs:391-401,504-515`). A per-case gate
    would need a new Rust interface and is not in this unit.
- **Semantic identity:** both runs carry the same result semantic identity
  (producer contract, route).

Otherwise the pair is `not_compared`, with the reason. The UI uses no wording
that implies certification (F-PIP-2).

**Context deltas.**
- Solver version, settings and the load-case and combination sets are compared
  like the reference `_settings_deltas`.
- Solve diagnostics are compared as added or removed by `(code, affected ref)`
  (PRD §15.3).
- Unmatched rows are listed per side.

**Parity.** A pytest runs `compare_analysis_runs` on a committed golden, through
an adapter from desktop rows to `quantity_results`. It covers
`SUPPORTED_RESULT_FAMILIES` and ungated pairs only. The gate and the extra
identity fields are tested separately.

**Memory.** The combined size of two stored results is checked against the
ingress budget. If it is over, the comparison is refused with a named
diagnostic.

### 4.3 UI

- **History panel:** states and recorded runs, with labels. It provides Save
  Model State…, Open State, Record Run, Compare… (two states or two runs),
  edit labels and assumptions, and delete.
- **Comparison panel:** summary, a filterable table, field paths, deltas with
  units, `not_compared` reasons and unmatched lists.
- **"Within this run"** keeps today's comparison. The user selects the case and
  the combination instead of the hard-coded ids.

## 5. Consumers

- `stateComparisonHandoffSections.ts` emits real state and recorded-run records
  when the report package includes them. Transient runs keep their present
  form.
- W-6b uses the state store for its basis state.

## 6. Verification

**Rust store tests (persistence; targeted regression):**
- v12 on a v11 store with rows, run twice (idempotent);
- state create, unique-hash refusal and hash-mismatch refusal;
- `UPDATE` raising on both immutable tables;
- a label edit leaving `record_hash` unchanged;
- delete blocked by a run and by a child state;
- state, labels, run and run labels written in one transaction, with rollback on
  injected failure;
- two recordings of the same producer `run_id` (and of identical solves)
  staying distinct;
- the manifest binding check (good, tampered manifest, wrong state);
- cross-project FK refusal;
- replacement keeping history, with the prompt counts;
- a stale-window write refused;
- the canonical snapshot round trip giving an equal hash;
- the ingress budgets;
- the sorted-compact non-ASCII vector against Python.

**vitest:**
- the session flows and the guards;
- the validators accepting transient and recorded forms;
- `HistoricalRunContext` accepting recorded runs;
- flattening fail-closed;
- the reorder invariant;
- the standing gate.

**pytest:**
- the parity goldens;
- the mirrored `compatibility.py` change;
- emitted state and recorded-run records validating against
  `model_state.schema.json` and `analysis_run.v0.3`, with a populated witness.

**Native smoke:** Save Model State, Record Run and Compare join it only if
HELP_HUMAN wants them in G10. Any `expected.json` change goes to HELP_HUMAN with
its cause.

**Review:** an independent Claude reviewer and Codex, on the implementation
head.

## 7. Reconciliation and owner-reserved text (through HELP_HUMAN; exact old → new with the slice)

1. **DEL-14-01 :3** "Phase G … gate v0.2 R3". Only states, runs, comparison and
   the workspace GUI move to v0.1, which records OD-1. Mapping, tolerance and
   export stay v0.2.
2. **Decided (HELP_HUMAN, 2026-10-23).**
   - **SQLite-only history:** option (a). In v0.1, states and recorded runs live
     only in the SQLite store. A history export is recorded as open under
     OI-015, the DEC-028 container. The reconciliation of DEL-14-01 :1
     ("preserve DEC-017/028 strategy") states this as an open item, not as
     discharged.
   - **Labels outside the record hash:** confirmed (§2).
3. **DEL-14-03 :1, DEL-14-04 :1.** Record them as partial: "mapping" and
   "unit-aware delta behavior" are only partly discharged without manual
   mapping and unit conversion. This is reconciliation text.
4. **Reconciliation:**
   - DEL-14-02 R002 and delivery commitment :1 pointers;
   - DEL-14-05 unchanged;
   - DEL-07-08 panel pointers;
   - `deliverable.yaml` pointers follow the code;
   - the completeness-map FR-CMP rows are for the reconciliation owner.
