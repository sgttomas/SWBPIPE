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
  `MODEL-STATE-INTEGRITY`, naming the state.
- **Unreadable rows in the list (W-12, Agent 0's decision; replaces S1 and
  S2's whole-list refusal).** One unreadable row no longer hides a project's
  history. The list returns each readable state as its summary.
  - **Unreadable entries.** A row that cannot be read appears only as
    `{"unreadable": {"id", "refusal", "handle", "references"}}`. A row is
    unreadable when a column is oversized, a stored value has the wrong type
    or is not UTF-8, or its record or labels row does not verify. Each row is
    decoded on its own. The `id` is display text, cut as messages quote it.
    The `refusal` is the named, capped integrity refusal. `references` counts
    the other stored rows that name the state, from their columns. None of
    the row's unverified fields is returned. Its oversized columns are never
    loaded. Every separate column fetch (the record, the labels, the
    snapshot) classifies a value of the wrong type or not UTF-8 as the row's
    own failure: in a list the row is unreadable, and a single read is
    refused as `MODEL-STATE-INTEGRITY`. Database-level failures (corruption,
    I/O, the statement itself) and memory refusals still refuse the list.
  - **Admission order.** Nothing per row is allocated before it is admitted.
    The rows with an oversized column are counted first, and all their
    entries, with their list slots, are reserved together at their bound
    (`UNREADABLE_ROW_BYTES` each) before any is collected; too many are
    refused by name (`STORE-READ-MEMORY-UNAVAILABLE`). Every other
    unreadable entry is reserved at the same bound before anything of its
    row is read for it.
  - **Delete handle.** The `handle` is `state-row:<rowid>:<id byte
    length>:<digest>`. The digest is the first 16 hex of the SHA-256 of the
    whole stored id (its bytes). The id is read in the same transaction in
    64 KiB `substr` chunks, each hashed where SQLite holds it, within one
    reservation of the chunk buffer; it is never copied whole into Rust.
    SQLite still materializes the whole value for each chunk, within the
    command's reserved SQLite working set (§3, Memory). The handle is distinct from the display text, so two rows whose
    cut ids are equal keep different handles, and a row that reuses a rowid
    with a different id never matches an old handle.
    `delete_model_state {projectId, unreadableHandle}` deletes exactly the
    project's row with that rowid whose stored id still has the handle's
    length and digest. A stale or wrong handle is not found. Inside its write
    transaction the delete re-checks the row by the list's own bounded check;
    a row that reads now is refused (`MODEL-STATE-ROW-CHANGED`: "this row
    changed since it was listed; refresh") and nothing is deleted. A
    replacement with an identical id that is also unreadable may be deleted,
    which is consistent with the user's intent: it is the row they asked to
    delete, as listed. No store migration is needed (the store stays v13).
    The delete is refused (`MODEL-STATE-DELETE-BLOCKED`, with the counts)
    while any stored row names the state. A readable state is still deleted
    by its verified `stateId`; an unreadable one is not deleted by any id.
  - **Relationships.** A summary presents only verified rows:
    - `child_state_ids` are the child states that name the state and are
      themselves readable, each verified as the list verifies it, one at a
      time within its own reservation.
    - `unreadable_references` counts the child-state rows that name it but
      cannot be read.
    - `recorded_run_count` counts the run rows that name it, from their
      columns: an integer, with nothing of the runs loaded or priced. The
      runs themselves come, each verified, from the run list, which the
      History panel groups by state. A state list therefore never loads a
      run.
    - Candidate children are loaded only within their bounds, and a related
      row's oversized column is never measured into another row's summary.
    - A blocked delete names verified children only and counts the runs and
      the unreadable rows.
  - Opening, reading, comparing or recording against an unreadable state
    stays refused.
  - **History panel.** Unreadable rows are listed apart from the readable
    ones, with the refusal and the guidance, and only Delete is offered.
    Delete is disabled while other rows name the state.
- **Memory (S2 repairs r1 to r6).** All five state commands are metered as
  §3's As built (S2) describes (`stored_read.rs`): `save_model_state`,
  `list_model_states`, `read_model_state`, the label edit and the delete.
  - **Before loading.** Each takes memory-pool leases sized from the stored
    lengths before loading anything: records, snapshots, label columns, the
    child-id lists, and every identity and time column. It measures and
    loads in one transaction.
  - **Identity columns (r5, r6).** A row whose identity or time column exceeds
    a bound the app holds at every write is refused before it is loaded. A
    project id is measured and priced, never refused by size.
  - **Hash texts.** It reserves the record's sorted-compact hash text at its
    exact escaped length before writing it. The writer that counts the length
    is the one that writes the text.
  - **Holding.** It holds its leases until its response is serialized.
  - **Commit last (r4).** Save, the label edit and the delete serialize their
    response before they commit. So a refusal of the response writes nothing.
  - **Project row (r4).** Save checks the project row and its stored base
    with a targeted, priced query. The stored project name is quoted only when
    the save is refused.
  - **Names in messages (r4, r5).** Messages and refusals quote at most 120
    characters of any interpolated value, then `…`. They describe a serde or
    SQLite error by its category or code, never by its text. A blocked delete
    names at most ten blockers in all, then `and N more`.
  - **Refusal.** A command whose lease the pool refuses is refused with
    `STORE-READ-MEMORY-UNAVAILABLE`, and nothing large is loaded.
- **Label bounds (r2, r3).** Every label write (Save Model State, Record Run,
  both label edits) keeps at most 256 tags, notes and assumptions together.
  - **JSON size.** It also keeps at most 196,608 bytes (192 KiB) of the whole
    labels object as the label edit carries it, serialized with its escapes:
    `{name, tags, notes, assumptions}`, name included. A name of 196,602 NULs
    serializes to 1,179,667 bytes and is refused.
  - **Editability.** A compile-time assertion keeps these 192 KiB, plus a
    project id of up to 63 KiB and 1 KiB of fixed members, within the label
    edit's 256 KiB argument budget (r6: the app does not bound project ids, so
    the length is stated). So any kept labels can be edited in a project with
    an id up to 63 KiB, which a test checks for both edits. Smaller labels
    leave room for a longer id.
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
  small commands take 256 KiB of arguments each. Before S2 r7 the list, read
  and delete took 4 KiB, which left a project with a longer id (the app keeps
  any) without a reachable history. A request over the budget is refused by
  name at ingress (`INGRESS-ARGS-TOO-LARGE`), and the History panel shows that
  refusal. A label edit names the labels' `updated_at` it read
  (`MODEL-STATE-LABELS-MOVED` otherwise).
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
   C1 (RT-U4; DEL-04-05 `Work/c1-native-custody-plan.md` slice 5) adds a native
   builder of the same transient record from a retained solve result
   (`analysis_run_v03_build.rs`): the record's bytes in the stored form and its
   checksums equal the TypeScript builder's. The webview receives only its
   header. Record Run and Save/Create Project of such a result take the job
   handle (`native_persistence.rs`, as built below): the record is rebuilt from
   the held bytes and stored in today's forms; a taken or restored result is
   recorded and saved as stated below, unchanged.
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
   - takes `rule_pack_refs` from the input manifest's `active_rule_packs` and
     `library_refs` from the model's `library_ref`s, as the model document's
     one walk finds them. At solve time the session resolves the model's
     `rule_pack_refs` against the local rule-pack store, for the model's
     project (`modelRulePackResolution.ts`). A reference enters
     `active_rule_packs`, in the model's order, as `{rule_pack_id, version,
     checksum_sha256, source_notice}` from the stored pack, when the store holds
     the pack, the stored document's stamped checksum equals the reference's,
     and the document recomputes to that checksum. Any other reference is left
     out of the manifest and becomes a diagnostic of the run's analysis record,
     not a manifest member (the manifest stays 1.0.0 and is stored with
     recorded runs). The diagnostics follow the producer's in the record's
     `diagnostics`, in the model's order, in the producer's diagnostic shape:
     code `MODEL-RULE-PACK-REF-UNRESOLVED` (the pack is absent, or the store is
     unavailable or fails) or `MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH` (the
     checksums differ), severity `warning`, `affected_refs` `[<identity>]`
     with the pack's identity as `rule_pack_refs` name it (below), the `id`
     `diagnostic:rule-pack-ref:<identity>`, and a fixed message, with the
     identity shown to at most 160 code points, that says the pack is not
     among the run's rule-pack references. The recorded revision keeps them
     unchanged under its record hash (Rust, TypeScript and Python). The
     validators (TypeScript and Python) and the native Record Run and read
     accept, after the producer's diagnostics, only diagnostics exactly as
     their code and identity determine them, with no identity repeated. A
     recorded revision's, and natively a submitted transient record's, must
     name exactly the model references
     (`model_basis.model_payload.rule_pack_refs`) its stored manifest leaves
     out of `active_rule_packs`, compared as (id, version) pairs, in order
     (`ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH` in the validators;
     natively `ANALYSIS-RUN-MANIFEST-MISMATCH` at Record Run, or
     `ANALYSIS-RUN-RESULT-MISMATCH` when the result's diagnostics are not
     first, and `ANALYSIS-RUN-INTEGRITY` at read). The reason
     (which code) is not derivable from the manifest and is protected by the
     record hash. A rule-check revision of the transient record carries them
     over. They are shown with the solve, in the Record Run form and in a
     recorded run's check, read from the record. A model without references
     makes no store call, and its manifest (`active_rule_packs: []`) and
     transient record keep their earlier bytes. The resolution only records
     identity. The solve does not read, apply or check a pack, nothing in it
     reaches the native solve, and a reference never makes a check pass. The
     browser preview refuses the solve before building a manifest;
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
  `read_analysis_run`, `delete_analysis_run` and `edit_analysis_run_labels`
  take 256 KiB of arguments each (r7; 4 KiB for the first three before). A
  request over the budget is refused by name at ingress
  (`INGRESS-ARGS-TOO-LARGE`). The browser preview refuses them all
  (`MODEL-STATE-STORE-UNAVAILABLE`). `read_analysis_run` and the comparison
  admission refuse a run whose stored result or run record exceeds the
  desktop's displayable-result limit (64 MiB, RT-U4) with
  `STORED-RESULT-TOO-LARGE`, judged from both stored lengths before either is
  loaded; the run is unchanged (DEL-04-07 `Work/memory-budget-decision.md`).
  These whole-value reads remain for the comparison sides and the browser
  preview; the History panel's run check and the project open read natively
  (below).
- **Native reopen (C1 slice 6; DEL-04-05 `Work/c1-native-custody-plan.md`).**
  On the desktop, a recorded run's check reads
  `read_analysis_run_in_native_custody {projectId, recordingId}`: the same
  verified store read (every integrity refusal above, in one read transaction),
  without the 64 MiB webview limit, with the stored result's and record's texts
  admitted as pool leases (capped at the per-invocation budget and 12 GiB) from
  their measured lengths before either is loaded, held natively as a restored
  handle of stored origin (`recorded_run`, the SHA-256 of the texts held). The
  reply (a JSON body) carries the verified summaries, the stored manifest's
  canonical text, the record's header without its row-scale members, and
  `buildRecordedRunContext`'s findings computed natively on the stored texts
  (`analysis_run_v03_validate.rs`, against the TypeScript oracle
  `historical_reopen_cases.json`); never the result or a `ResultRef`. The check
  holds no rows, so the webview releases the handle at once. Opening a project
  reads `open_local_project_in_native_custody`: `open_local_project`'s members
  (the same model-document refusals and serde messages) and the saved run held
  natively (`saved_project_run`) with `buildHistoricalRunContext`'s findings
  and the natively recomputed envelope payload hash, or, when the pool or
  retention does not admit it, `STORED-RESULT-RESTORE-REFUSED` with the model
  open for editing (a save in place is then refused until a new solve). Nothing
  is written or migrated by either; a restored handle is refused by Record Run,
  Save Project and the record preview from a job handle
  (`NATIVE-PERSISTENCE-CUSTODY`, `ANALYSIS-RUN-PREVIEW-READ-FAILED`) and has no
  exact-route custody.
- **Saving a reopened project (C1 slice 6's save continuity).** Save or Create
  of a project showing its reopened saved run sends
  `save_restored_result_local_project` the restored handle, its stored source,
  the envelope basis the session holds and the project's small fields; no result,
  record, model hash or envelope hash. The stored run is written as the legacy
  session wrote it back (serde_json's reading of `JSON.stringify` of its stored
  values), the stored project's model hash evidence (held with the handle), and
  the envelope hash over what is stored, or the basis's stored evidence when that
  payload is unchanged; the record is the stored record, never rebuilt. The
  replacement guard, SQLite's exact lengths, the model-only save, the
  withheld-run guard and the no-save rollback are the native save's. Only a
  restored saved project run binds (`NATIVE-PERSISTENCE-CUSTODY` otherwise,
  nothing written); the handle stays held and historical, and the reply carries
  the findings of the project as stored.
- **Request.** It carries the transient record, the result's JSON text (stored
  as given), the manifest's canonical text, the session's base, and optionally
  the state the session expects (`state_id`, checked:
  `ANALYSIS-RUN-STATE-MISMATCH`). When no state holds the solved model, it names a
  `post_solve_snapshot` (`new_state`); without one the request is refused with
  `ANALYSIS-RUN-STATE-REQUIRED`. The snapshot is the manifest's model payload. Its
  parent is the session's basis state.
- **Native Record Run (C1 slice 5; DEL-04-05 `Work/c1-native-custody-plan.md`).**
  For a native-custody solve the webview sends
  `record_preview_mechanics_job_analysis_run` the job id, the record's inputs as
  the native preview read them (the manifest evidence without its parsed copy,
  the rule-check aggregate, the rule-pack exclusions), the previewed record's run
  id and both checksums, the model text the job was sent (`intakeText`,
  `deliveredIntakeText`) and `request: {expected_base, state_id, new_state,
  run_labels}`; no record, result text or manifest object. The job must be held
  natively, its bytes its publication's and its solver mode the manifest's, and
  the model text must hash to the intake digest recorded when the job was
  admitted and parse to the manifest's model (JSON data with JavaScript's
  numbers), so a manifest model swapped under the same project id is refused
  (`NATIVE-PERSISTENCE-CUSTODY`); the record rebuilt from the held bytes must be
  the previewed one (`NATIVE-PERSISTENCE-RECORD-STALE`). The checks below then
  apply to the rebuilt record's members that are not row-scale (the manifest's
  canonical text, hash, ref and project; the transient shape and status; the
  members the manifest fixes; the received result's checksum, ref, run and
  project; the exact checksum inventory). The row checksums and diagnostics are
  the builder's own, computed from the held bytes, so nothing of them is
  asserted by the webview. The transaction, the state, the recording time and
  the revision are this section's; `result_json` is
  `jsonTextKeepingNegativeZero` of the held bytes' parse (written natively in
  JavaScript's member order) and `run_record_json` the revision's stored form,
  byte for byte the submitted path's (`native_persistence_tests.rs`). Before
  anything is allocated or written, the writing connection's
  `SQLITE_LIMIT_LENGTH` bounds every bound value and the run row's record; above
  it the run is refused `RUN-RECORD-RESULT-TOO-LARGE (result_bytes, record_bytes,
  row_bytes, limit_bytes)` with nothing written, and a row SQLite still refuses
  rolls the transaction back, a new snapshot included. The outcome is
  `record_analysis_run`'s.
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
  - `rule_pack_refs` are `{RulePack, <identity>}`, in the manifest's order,
    which is the model's reference order (the manifest no longer sorts them).
    A pack's identity is `<rule_pack_id>@<version>` with `%` (as `%25`) and
    then `@` (as `%40`) percent-encoded in each component, so the one `@`
    left separates them and distinct (id, version) pairs never share a string
    (`invented@branch` 1.0.0 is `invented%40branch@1.0.0`; `invented`
    `branch@1.0.0` is `invented@branch%401.0.0`). An id and version without
    `%` or `@` are unchanged. Rust (`rule_pack_identity`), TypeScript
    (`rulePackIdentity`) and Python (`rule_pack_identity`) encode the same.
    Comparisons use the (id, version) pairs.
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
    manifest. A row that does not verify, or has a value of the wrong type
    or one that is not UTF-8, is listed by its id, refusal and handle only,
    as for states (§2), with the same admission order and the same
    classification of every column fetch (the record, the result, the
    manifest; a single read is refused as `ANALYSIS-RUN-INTEGRITY`). Its
    handle is `run-row:<rowid>:<length>:<digest>`, the digest of the whole
    stored id read in chunks as for states, and
    `delete_analysis_run {projectId, unreadableHandle}` deletes exactly
    that row and its labels, after re-checking it inside its write
    transaction by the list's own check (`ANALYSIS-RUN-ROW-CHANGED` when it
    reads now; a replacement with an identical id that is also unreadable
    may be deleted). The other runs are listed as usual. A readable
    run is deleted by its verified `recordingId`, whatever its state's
    readability. A readable run whose state cannot be read is shown in the
    History panel under "Runs whose state cannot be read", with Check record
    and Delete. Once nothing names the state, the state can be deleted too.
    The list checks the record hash and the identity columns against the
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
  - **Runs recorded before rule-pack resolution.** S2-era runs (#1596) of a
    model with `rule_pack_refs` have `active_rule_packs: []` in their
    manifest and only the producer's diagnostics. Read and Record Run refuse
    them by name with `ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED`: "recorded before
    rule-pack resolution; re-record this run". The code applies when the
    manifest's `active_rule_packs` is `[]`, the model has references and no
    exclusion diagnostic is present. Rust, TypeScript and Python refuse such
    a run alike (Agent 0's decision: one current version, no lenient reading).
    Any other mismatch keeps its code. **Precedence:** all three give
    `ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED` only after every other applicable
    check passes: the record hash and checksums, the identity columns, the
    manifest binding, the linked state and its model hash, and the
    diagnostics prefix. Corruption always gets its integrity code first.
    Natively, the read refuses by name after the linked state's checks, and
    Record Run after the state resolution. The TypeScript and Python
    validators refuse by name after the record checksum. The list reads no manifest, so such a
    run lists as its verified summary. Every read of it (Check record,
    compare, export) is refused by name, and the History panel shows the
    refusal with "Re-record this run". The fixture is
    `fixtures/persistence/s2_unresolved_rule_pack_run.json`, written by
    `recordedRunParity.test.ts` along S2's path.
- **Delete.**
  - **State.** A state with recorded runs is not deleted
    (`MODEL-STATE-DELETE-BLOCKED`, counting the runs; W-12). The `NO ACTION` foreign key
    refuses the deletion in SQL as well.
  - **Run.** A run is deleted only as a whole row with its labels; no other row
    depends on a run. An unreadable run may be deleted (W-12, Agent 0's
    decision), by its list handle only, never by a shown id. Nothing of it
    is trusted, and it is loaded only as the list loads it.
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
- **Memory (repairs r1 to r6, `stored_read.rs`).** Ten commands are metered:
  the five state commands, the five run commands (`record_analysis_run`,
  `list_analysis_runs`, `read_analysis_run`, the run label edit and the run
  delete).
  - **Leases.** Each reserves its work before it makes it, through interactive
    memory-pool leases: Save Local Project's ingress admission (the pool, fresh
    headroom, and the per-invocation budget over the command's live leases
    together). The accounting starts from the command's argument lease, so
    every admission counts it with the command's reservations (r4).
  - **Before loading.** A command takes, with `octet_length` and `COUNT`, the
    byte lengths of every column it will load and of the state's child and run
    lists, without loading them. It then reserves:
    - loading each column: records, results, manifests, snapshots, and every
      label column (name, tags, notes, assumptions), as SQLite's value plus the
      copied `String`;
    - the relationship lists (child and blocker ids) it materializes;
    - every identity and time column it loads, as measured (r5);
    - a fixed 512 KiB overhead.
  - **Every column measured (r5).** No loaded column has a fixed allowance.
    - **Bounds (r6).** Only columns the app bounds at every write have a
      fixed bound:
      - ids the app generates, and the references to them: 4 KiB (state ids
        and recording ids are 38 and 39 bytes: `state:` or `runrec:` and 32
        hex);
      - kinds: 64 bytes (a state kind is one of the five);
      - times: 32 bytes (`utc_iso_millis` writes 24);
      - hashes: 80 bytes (64 hex, or `sha256:` and 64 hex).

      Ids the app does not bound are measured and priced, never refused by
      size: a project id (Save Local Project keeps any) and a producer's run
      id. Data that S1 or project save wrote with a 4,097-byte or 64 KiB
      project id still lists, reads, edits its labels and saves its duplicate
      in the store.
      - **Through the commands (r7).** The six history commands (both lists,
        both reads, both deletes) are admitted with the label edits' 256 KiB
        argument budget, so they reach a project id of up to about 256 KiB.
      - **Longer ids.** A longer id is refused by name at ingress
        (`INGRESS-ARGS-TOO-LARGE`), and the History panel shows that refusal.
    - **State rows:** state, kind, created_at, parent, model and record
      hashes, and the labels' `updated_at` are bounded; the project id is
      measured only.
    - **Run rows:** recording, state, created_at, record and result hashes,
      and the labels' `updated_at` are bounded; the project id and producer
      run id are measured only.
    - **Refusal.** Before anything of a row loads, a row whose column exceeds
      its bound is an integrity refusal, naming the row by the head of its id.
      The list shows that row as unreadable and lists the others.
    - **The auxiliary loads are bounded too:**
      - the latest `created_at` Record Run reads, after every run's
        `created_at` is checked against its bound;
      - the existing state found by its model hash, whose id loads only within
        its bound;
      - the blockers' heads;
      - the project row's measured envelope hash, and its name's head.
    - **Status lists (r5, r6).** A verified record's `analysis_status` is
      copied into a summary only when it serializes to at most 4 KiB. The copy
      outlives the record's reservation, so it is reserved as it is retained:
      from its element count and their bytes (each `String`'s header
      included), with its transient `Value` copy. Record Run accepts only
      v0.3's six status words (the schema's enum), each at most once, as the
      app produces them. This is stricter than the schema, which leaves the
      list's length open.
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
  - **Messages and refusals (r4, r5).** Every message, receipt, refusal and
    reservation description quotes at most 120 characters of each
    interpolated value (`stored_read::shown`), then `…`: names, ids, times,
    kinds, hashes, project ids and the document's version. The structured
    fields keep the full, metered values.
    - **Serde errors.** A serde error is described by its category and
      position (`json_error`), never by its text, which can quote the value.
    - **SQLite errors.** A SQLite error is described by its code and extended
      code (`sql_error`), with its message only when that is at most 120
      printable characters (SQLite's own texts and the store's trigger codes).
      rusqlite copies the message into the error before the store sees it;
      nothing longer is copied again.
    - **Foreign messages.** The core canonicalizer's refusals are cut to 120
      characters. The shared store opener's refusals are capped. The
      model-version and malformed-slot refusals are rebuilt with cut values,
      at most three findings, then `and N more`.
    - **Last net.** Every error a command returns is cut to 1024 characters.
    - A blocked delete names at most ten blockers across its child states and
      its runs together (r5), then `and N more`. It reads only the head of
      each blocker's id and name from SQLite (`shown_column`: the first 484
      bytes and the byte length).
    - A state summary loads its runs' ids, not their names.
    - The project-row guard of Save and Record Run quotes the stored project
      name only when it refuses, and so does `refuse_unconfirmed_replacement`'s
      message (its decisions are unchanged).

    So every message built from stored data has a small constant bound (at
    most six serialized bytes a character) and is not reserved.
  - **Project row (r4).** Save and Record Run check the project row with
    targeted queries, never the project index:
    - the row's existence;
    - its envelope hash, reserved before it is loaded and parsed only when the
      prescan accepts it.

    The session's base must equal the stored hash value, as
    `refuse_unconfirmed_replacement` decides with no confirmed revision.
    Project save and open are unchanged.

    One deliberate difference from the former guard (r5) concerns a stored
    envelope hash the ingress prescan refuses, such as one holding serde_json's
    private `$serde_json::private::RawValue` key. The guard reads it as no
    hash, so the save or recording is refused as `PROJECT-BASE-MOVED`. The
    former guard parsed it and admitted a matching base. Ordinary hash rows
    are read as before.
  - **The response.** The response is serialized at its exact length, counted
    first without allocating, into a buffer reserved at that length.
  - **Holding.** A value read or built from the store is a `Metered` value
    that cannot be taken apart. It is borrowed, mapped or joined with its
    reservations kept, and it leaves them only when it is serialized. An
    unmetered caller can take its value only when it holds no lease. So:
    - a list's labels, a read's state summary and the current summary of an
      edit, while its record is loaded, stay reserved while they are live;
    - a delete receipt, a save outcome and a Record Run outcome are held
      through serialization;
    - a serialized response keeps its buffer's reservation across the commit
      until it is handed to Tauri (r5);
    - the project row's parsed envelope hash keeps its parse's reservation
      while it lives (r5). Since r6 it is one `Metered` value, as decoded
      labels are, so the value is dropped before its reservation. No binding
      pairs a value with its reservation in a tuple.

    `map`, `join` and the commit-last `finish` steps take closures. The rule
    is that a closure moves the value into its result and does not capture or
    keep it elsewhere. The type checks a closure's result, not its captures,
    so this is a reviewed rule, not an enforced one.
  - **Commit last (r4).** Each of the six mutating commands passes its
    outcome, still holding its reservations, to a `finish` step before it
    commits: Save, Record Run, both label edits and both deletes. In the
    command, `finish` serializes the response (`serialize_metered`). So a
    refusal of the response, or of anything before it, rolls the transaction
    back: nothing is written and the failure is reported. After the commit
    the command only hands the already serialized text to Tauri. A failed
    commit writes nothing.
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
    - **Commit last (r4).** For each mutating command, a refused response is
      reported by name and writes nothing: no state, run or snapshot is saved,
      no labels change, the deleted rows stay. Admitted, the delete commits
      after its response exists.
    - **Bounded refusals (r4).** These refusals serialize to under 2 KiB, with
      the allocator peak within their reservations:
      - a blocked delete naming a 3 MiB run name;
      - stale-label refusals of a state and of a run quoting a 3 MiB NUL name
        (Codex r4 measured 18,874,519 bytes serialized).

      Twelve blocking runs are named as ten and `and 2 more`.
    - **Project name (r4).** With a 3 MiB stored project name, Save and Record
      Run succeed under the 2 MiB pool within their reservations. A stale save
      is refused quoting the name's head.
    - **Invocation total (r4).** A Record Run whose argument lease plus its
      reservations exceed the per-invocation budget is refused before it
      records. With a small argument lease, the same run is admitted.
    - **Mutations.** Each r3 and r4 test fails when its defect is put back:
      - r3: the snapshot broken, the hash priced from a sixth of its length,
        the current summary dropped early, the receipt's message unreserved;
      - r4: a commit before `finish`, uncut names, the old project guard, an
        unseeded invocation total.
    - **Errors (r5).** Codex r5's cases each serialize to under 2 KiB:
      - a state's and a run's `tags_json` holding a 3 MiB NUL string;
      - a 4,000-NUL recording id with no labels row;
      - a 3 MiB recording id;
      - a stored trigger whose message is 3 MiB.
    - **Identity columns (r5).** A 3 MiB value is put in each identity and
      time column of a state row (except its project id, by which rows are
      found) and of a run row. Each is refused before loading under the 2 MiB
      pool, through the list and the read, with a small message and a peak
      within its reservations or below 64 KiB. So are:
      - the latest `created_at` that Record Run reads;
      - the existing state's id that Save finds.
    - **Held across the commit (r5).** Recording against a state with a 3 MiB
      name keeps its 3 MiB response reserved when the commit begins and after
      it. The reservation is released when the response is handed on.
    - **Minor (r5).** A stored hash with the RawValue key is refused as
      `PROJECT-BASE-MOVED` (the former reading admitted it). Six child states
      and six runs are named as ten blockers in all, then `and 2 more`.
    - **Mutations (r5).** Each r5 test fails when its defect is put back:
      - serde's or SQLite's text quoted;
      - the list's bound check removed;
      - the response's reservation released before the commit;
      - separate blocker caps.
    - **Drop order (r6).** At every release of a reservation, what is live is
      within what is still held, with 1 MiB for small unpriced allocations.
      This is checked over:
      - Save with Codex's envelope hash of 100,000 empty strings, which left
        4,202,224 bytes live with 1,581,776 held before the fix;
      - the lists, reads, label edits and deletes of states and runs with
        large labels.
    - **Status (r6).** Record Run refuses 1,000 empty status strings, a
      repeated word and an unknown word. Codex's case is 128 runs, forged past
      Record Run, each with 1,000 empty status strings. Their list peaks within
      its reservations; before the fix, 5,083,456 bytes were priced against
      3,988,608 reserved.
    - **Long project ids (r6, r7).** For 4,097-byte and 64 KiB project ids, the
      following work:
      - in the store: list, read, both label edits (through their commands)
        and the duplicate save;
      - Record Run's state-required refusal and a blocked delete, each under
        2 KiB;
      - since r7, through the real writers (project save, Save Model State,
        Record Run) and then the six history commands' argument decoders.

      A project id over 256 KiB is refused by name at ingress.
    - **Times (r6).** A 24-byte stored time whose last character is two bytes
      wide no longer panics:
      - **Record Run** refuses a run's malformed `created_at` as
        `ANALYSIS-RUN-INTEGRITY` ("is not a recording time").
      - **A label edit** does not refuse a malformed previous `updated_at`. It
        reads it as no earlier time and writes the current time in its place.
    - **Mutations (r6).** Each r6 test fails when its defect is put back:
      - the tuple drop order;
      - the status reservation removed;
      - the project id bounded again;
      - the project id uncapped in the state-required refusal;
      - the byte slice.
    - **Large pool.** Each tested read, list, label edit and delete
      serializes its response within its held reservation, with its peak
      within its reservations' peak.
  - **S3.** §4.2's comparison of two stored results reuses this pricing.
  - **What is bounded.** Every read and mutation of stored data:
    - every column it loads is measured and reserved before it loads;
    - the identity and time columns the app bounds at every write are
      bounded;
    - every parse, canonical text and response is reserved before it is made;
    - every message is capped.
  - **What is not bounded.** The owner accepted the six items below as
    documented v0.1 residuals (owner ruling, 2026-10-24, relayed by HELP_HUMAN:
    "for this W-12 issue I approve Option A as recommended"). They are
    instances of `memory-budget-decision.md`'s existing residual "command
    working memory and responses are not separately bounded" (DEL-04-07 Design).
    Sizes are Codex review r7's conditional planning estimates, not measured
    whole-command bounds:
    - **Tauri and IPC.** The `Response` handed to Tauri, and the IPC copies
      made after the command returns.
    - **SQLite.** SQLite's own allocations are priced by estimates, not
      bounded by measurement in each command:
      - **Loaded columns.** A column a command loads (a record, result,
        manifest, snapshot or label text) is priced by the load estimate,
        three envelopes of its length.
      - **Reserved working set (W-12, repair r7).** SQLite materializes
        the whole stored value whenever a statement compares, joins, sorts,
        seeks an index by, takes a `substr` of or deletes it. An index seek
        can also materialize another row's key on its search path. Every
        state and run command (list, read, label edit, delete by id or by
        handle, Save Model State, Record Run) therefore reserves SQLite's
        working set first in its transaction, before any other statement
        touches a row. The working set is 8 times the bytes of the largest
        stored keyed row, measured over every row of the history tables.
        Its columns are the key, identity and time columns of
        `analysis_runs`, `model_states` and both label tables, and the
        project id in `local_projects`. The project name is read only for
        the `PROJECT-BASE-MOVED` refusal's head; that read first reserves 8
        times the stored name's length.
      - **How it is measured.** The measurement uses only `octet_length`
        under `NOT INDEXED`, which reads record headers and loads no value
        (SQLite 3.43+; the bundled SQLite is 3.46.0).
      - **Refusal.** A store whose working set does not fit is refused by
        name (`STORE-READ-MEMORY-UNAVAILABLE`) before SQLite loads the
        value.
      - **Basis for the factor.** The factor comes from
        `sqlite3_status64(SQLITE_STATUS_MEMORY_USED)` peaks, measured for a
        row whose 8 MiB id is also its labels row's key:
        - a list peaks at 2.04 times the id;
        - a run's delete by handle at 5.33 times;
        - a state's delete by handle at 6.34 times. That covers its shown
          head, the chunked digest, the re-check, the index and page work,
          and the labels cascade.

        8 covers the largest of these with about 26% margin.
      - **Regression test.** The test reruns itself in its own process
        (SQLite's status is process-wide) and checks the following:
        - under a 2 MiB pool, the list and the delete of such a row are
          refused, with SQLite's peak below a quarter of the id;
        - under a large pool, both succeed with SQLite's peak within the
          working set;
        - normal-size rows list and delete under the 2 MiB pool.
      - **Page cache (accepted residual estimate).** SQLite's page cache is
        not reserved per command. It is the accepted residual estimate, not
        a measured bound and not within the fixed 512 KiB read overhead:
        - a header scan of 100,000 rows peaked at 2.56 MB;
        - a state delete with many large keys peaked at 9.05 MB against
          8.52 MB reserved, with cache allocation separately peaking at
          2.48 MB (Codex review r8).

        The default cache limit (about 2 MB per connection) bounds it. The
        regression tests allow 3 MiB of SQLite allocation above what a
        command holds.
      - **Other columns (repair r8).** A column a refusal reads from a
        non-history table is loaded only when it can be priced. The stale
        project guard reads the project's `updated_at_unix` only when
        `typeof` (header-only) says it is an integer; SQLite keeps any text
        in that column. Otherwise the guard refuses by name
        (`PROJECT-BASE-MOVED`, "its last save time cannot be read") without
        loading the value.
      - **Blocker names.** A blocked delete names each verified child from
        the bounded display text kept at its verification (at most ten,
        priced with the summary). It never reloads a name after the child's
        reservation is released: S1-era labels have no size bound.
      - **Not covered.** The project commands (Save Local Project and the
        project row's cascade to states and runs) are outside this
        reservation. No command deletes a project.
    - **Unpriced working memory.** Two items are not separately priced:
      - Record Run's transient record copies (the hash projection and the
        recorded revision);
      - the `Value` of a new state's record, built from its snapshot (mapped
        assumptions, validation warnings).

      They are covered by the request's argument lease only by assumption.
      This remains explicitly unmeasured. Review r2 measured about 26.0 MiB
      against a 10.1 MiB lease for Record Run's large case before r2; this has
      not been re-measured.
    - **Transient copies before a cap.**
      - The core canonicalizer, the shared store opener and rusqlite make
        their own messages before they are cut. These are copies of text the
        command has already priced or the argument carries. They remain
        explicitly unmeasured.
      - The length counters allocate each object's sorted key list (space
        proportional to the object's width, one pointer a key) and each
        number's short text.
  - **No whole-command claim.** S2 bounds each priced stage as above. It makes
    no claim of a complete memory bound for a whole command.
  - **Native measurement (development testing; D-72 item 6).** The IPC copies of
    a run read are to be measured natively during development testing:
    - the run-read response is estimated at about 200–250 MiB for a
      100,000-row run, and about 0.6–1.2 GiB in IPC and webview copies;
    - the measurement takes the native process's footprint while a large
      recorded run is read through the History panel.

    If the copies matter in practice, paged or streamed run reads become a
    scheduled unit (owner ruling, 2026-10-24, Option A).
  - **Stores modified outside the app (class B).** A row changed outside the
    app (tampered or corrupted) is detected by the integrity checks above: the
    record hash, identity columns, exact descriptors and the manifest binding.
    It is refused by name. Since W-12 the lists show such a row individually
    (id, refusal and handle only) and list the project's other rows. Every
    read of it stays refused. It can be deleted by its handle (a state only
    when nothing names it). Readable rows never present its id as a
    relationship.
    S2 claims no memory bound for the work done before such a refusal beyond
    the stage pricing above.

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
  `analysis_status`, `diagnostics`, `schema_version`, `document_kind` and `model_settings`. Model document 0.9.0's `condition_sets` and `spectra` are retained verbatim as fields of this same entity: nested values, units, record identities and array order participate in the existing field comparison. No values are synthesized or dropped, and no new comparison family is introduced (S1, #50). It
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

**As built (S3a, state comparison).** `apps/desktop/src/features/comparison/stateComparison.ts`
(flattening and comparison), `StateComparisonView.tsx` beside it (the view), the
History panel and `useModelStateHistory.ts` (Compare…), and the store's
`admit_model_state_comparison` and `read_model_state_for_comparison`
(`model_states.rs`).
- **Member assignment (0.7.0, all 22 members).** The `model` entity holds
  `schema_version`, `document_kind`, `project`, `pressure_contract`,
  `data_boundary`, `analysis_status`, `diagnostics` and `model_settings` as
  they are. The fourteen collections are families: `nodes`, `pipe_segments`,
  `sections`, `materials`, `supports`, `components`, `load_cases`,
  `combinations`, `reference_configurations`, `lines` and
  `equipment_interfaces` keyed by `id`; `unresolved_assumptions` by
  `assumption_id`; `constraints` by `constraint_id`; `rule_pack_refs` by
  `rule_pack_id`, as the operation applier keys them (a version change is a
  `changed` reference, not a removal and an addition). `tags`, `notes` and
  `provenance_record` are not top-level members in 0.7.0: they are fields of
  the records that carry them and are compared with them. A test reads
  `schemas/model.schema.yaml` and fails when the schema gains a member that is
  not assigned (the pytest checks the golden's assignment the same way).
- **Primitive loads** are the `primitive_load` family, keyed by the pair
  (load case id, load id) and shown `<load case id>/<load id>`; a load id may
  repeat across load cases. The load case entity excludes `primitive_loads`.
- **Order.** The `model` entity also holds `collection_order` (each present
  collection's ids in document order) and `primitive_load_order` (each load
  case that has `primitive_loads`, with its load ids in order). So a reorder,
  a collection left out and an empty collection added are each a `changed`
  model entity.
- **Refusals, by name.** `COMPARISON-MEMBER-UNASSIGNED` (a member no family
  holds), `COMPARISON-MEMBER-MALFORMED` (a collection that is not an array of
  objects), `COMPARISON-ENTITY-ID-MISSING` (an entity without a stable id) and
  `COMPARISON-DUPLICATE-ID` (a stable id repeated within one family on one
  side; equal ids in different families are distinct).
- **Field diff.** Each entity's fields (the record without its own id field,
  or both key fields for a primitive load) are read as leaves: a scalar, a
  quantity (`{value, unit}` or `{x, y, z, unit}`: a text unit and only numbers
  besides), an empty object, or an array of scalars. Objects recurse by key,
  and arrays holding objects or arrays by index. Leaves compare by their
  key-sorted JSON, so equality is JCS equality. A change is `added`,
  `removed`, `changed` or `unit_changed` (two quantities whose units differ),
  with its path and both values. Nothing is converted or subtracted. A present
  `null` and an absent member differ.
- **Invariant.** Equal model hashes hold exactly when every row is
  `unchanged`; vitest checks it with the session's model hash over identical,
  member-order-only and reorder-only pairs, an empty collection added and
  sixteen single mutations, and the golden records it per pair.
- **Parity.** `fixtures/comparison/model_state_comparison_golden.json`, eight
  invented pairs from `invented_every_slot_model.json`, is written by
  `stateComparisonParity.test.ts` (`OPENPIPESTRESS_WRITE_STATE_COMPARISON_GOLDEN`)
  and checked by `tests/test_model_state_comparison_parity.py`. The adapter
  projects each entity's leaves to top-level `field:<path>` keys, so none meets
  the reference's skipped ids. A path that holds a quantity on both sides of a
  mapped pair, and on one side only nowhere, is declared in
  `unit_bearing_fields`, each quantity given one declared dimension. Beyond
  summaries and classifications, the pytest requires a `changed` row's field
  paths to equal the desktop's, and an `unresolved` row to carry
  `INCOMPATIBLE_UNIT`. The duplicate pair is refused by both. Outside the
  shared projection: a present `null` against an absent member (the
  reference's `.get` reads both as `None`) and a quantity with an empty unit.
- **Reading two states (ordinary decisions).**
  - **Same project only.** The History panel lists one project's states, and
    the admission takes one project id, so a state of another project is
    `MODEL-STATE-NOT-FOUND`. The reference engine does not expect
    cross-project comparison.
  - **Size admission: a small Rust command.** The list carries no snapshot
    lengths, so `admit_model_state_comparison {projectId, leftStateId,
    rightStateId}` (256 KiB of arguments, as the history commands) reserves
    SQLite's working set, reads both snapshots' `octet_length` in one read
    transaction, and refuses by name (`COMPARISON-STATES-TOO-LARGE`) when their
    sum exceeds the per-invocation budget, before either is read. It returns
    four numbers and echoes no id. It is a 1× cap on the stored input size, not
    a bound on webview memory: it is a check, not a lease, and has no
    multiplier. The webview's parse, flatten and render memory stays unpriced
    under the documented residual ("command working memory and responses are
    not separately bounded", DEL-04-07).
  - **The comparison read (r1).** `read_model_state_for_comparison
    {projectId, stateId}` (256 KiB of arguments) is `read_model_state` for a
    comparison: the same metering and leases, SQLite working-set reservation,
    hash and integrity checks, version refusal and refusals of unreadable or
    tampered rows. It alone admits blocking document findings, so a
    `MODEL_INCOMPLETE` state, kept as a comparison basis, is compared;
    `read_model_state` (Open State) still refuses it, so it is never opened.
  - **Exactness at the webview boundary (r1).** The webview decodes JSON
    numbers as binary64, so a stored integer beyond 2^53 (9007199254740993)
    arrives as another number, and two states the store holds apart would read
    unchanged. Before comparing, each received snapshot's model hash is
    recomputed in the webview (the session's wasm canonical hash) and must
    equal the state's stored model hash; otherwise the comparison is refused
    by name (`COMPARISON-STATE-NOT-EXACT`: the saved state holds values the
    desktop cannot represent exactly). There is no exact-number comparison.
    Open State already makes the same recomputation and refuses a mismatch as
    `MODEL-STATE-INTEGRITY`.
  - **Sides.** The earlier-saved state is the left side.
  - **Unreadable rows** have no selection; their compare checkbox is
    disabled with the reason. Compare… needs two selected readable states, goes
    through the one-owner check and the busy refusal, and is disabled with the
    stated reason when busy or when two are not selected.
- **View.** Counts per classification, entities and unit-changed fields; a
  table filterable by classification (differences only by default), family and
  id, paged at 200 rows with Previous and Next and "Rows a–b of n", back to the
  first page on any filter change (r1), so every row is reachable; each changed
  entity's field paths with both
  values as stored (a quantity with its own unit). View-only, with no export
  (FR-CMP-010 is v0.2), and no wording of standing. The within-run
  `ComparisonPanel.tsx` is unchanged (its case and combination selection is S3c).

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

**As built (S3b, run comparison).** `apps/desktop/src/features/comparison/runComparison.ts`
(identity, mapping, gate, context and diagnostics), `runComparisonSides.ts` (the
two sides and their standing) and `RunComparisonView.tsx` beside them (the view),
the History panel and `useModelStateHistory.ts` (Compare… of runs), and the
store's `admit_analysis_run_comparison` (`analysis_runs.rs`).
- **Sides.** Two recorded runs of the session's project, or one and the
  session's current run. The current run is offered only while the session
  holds its solve with the input manifest the record names (`recordableRun`); a
  run restored from the saved project has none and is not offered, with the
  reason. The earlier-recorded run is the left side; the current run is always
  the right. A comparison with the current run holds only for that slot (its
  result, record and manifest objects): if an edit, another solve, a cleared
  result or a project replacement changes the slot while the comparison is
  prepared, it is refused (`COMPARISON-CURRENT-RUN-CHANGED`), and a shown one is
  closed with the reason in the status (Codex r1 on #1617); a comparison of two
  recorded runs is unaffected.
- **Row identity.** `flattenRunResult` reads each row's `family` (a physical
  quantity's family, else null) and `dimension` (the signature's
  `source_physical_semantic_dimension`, the record's `source_dimension`) through
  the Rust reader's row semantics of that result; `object_ref` is the row's
  `entity_ref`, `basis_ref` its `{ref_type, ref_id}`, and the five metadata
  members are read as text or null. Rows map when all ten are equal. A result id
  repeated within either run, or a row without one, maps to nothing (as the
  reference's `_unique_quantity_records`). An unmapped row is listed on its side
  with its reason: no row with this id in the other run, `identity_differs`
  (naming the members), `duplicate_result_id` or `result_id_missing`.
  - The reader's signature is per kind and unit, so a row whose unit changed
    reads under another signature (or none): it is unmapped as
    `identity_differs` (family, dimension), not compared as `units_differ`. The
    unit condition stays as a check.
  - A row whose metadata `basis` text names the solve (the solver-mode row, the
    nonlinear support rows) differs between a sparse and a dense solve, so such
    rows do not map across solver modes.
- **Gate.** The whole-run conditions are each run's
  `numericalResultStanding(...).eligible` and an equal semantic identity: the
  reader's route with its table binding (contract id and sha256), else the
  producer's claimed contract id. A mapped pair is then compared only when the
  units are equal and non-empty, both values are finite numbers, and its family
  is one of the reference's `SUPPORTED_RESULT_FAMILIES` (an ordinary decision: as
  the reference refuses other families, no delta of a solver-mode code or a
  residual count is given). Otherwise it is `not_compared` with every reason that
  applies, in a fixed order. The delta is `right − left` in binary64, in the rows'
  own unit; nothing is converted, and no tolerance is applied.
- **Standing.** A recorded run is evaluated by `numericalResultStanding` over its
  stored result and the solved model its stored manifest holds (the snapshot,
  bound by the store to the state's model hash), after the checks below. Nothing
  registers a recorded result, so a run on the pressure-source or retained route
  reads `needs_recompute`, and the view says why. The current run uses the call
  the session's own verdict makes (`resultsSessionState.ts`), on the session's
  result object, so a pressure-source solve this session registered keeps its
  live registration.
- **The comparison read.** Compare… reads each recorded run through
  `read_analysis_run`, Check record's read, unchanged: its metering, working-set
  reservation, integrity checks and refusals all apply. That read refuses nothing
  for opening or display only, so no separate comparison read is needed. A run
  recorded before rule-pack resolution is refused by name
  (`ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED`). The list reads no manifest and cannot
  tell such a run, so once Check record or Compare… has been refused by that name,
  the session keeps the refusal (runs are immutable) and the History panel shows
  that run's compare selection disabled with it. An unreadable run row has a
  disabled selection with the reason.
- **Exactness at the webview boundary.** Each received recorded run is checked
  again in the webview (`buildRecordedRunContext`): its result is re-hashed in the
  checked profile and compared with the record's received-result hash, its
  snapshot with the state's model hash, its manifest with the record's manifest
  hash, and the record with its own hash, each in the store's canonical form. A
  hash that differs is refused as `COMPARISON-RUN-NOT-EXACT` ("the recorded run …
  holds values the desktop cannot represent exactly (its … as received does not
  hash to the stored hash)", as `notExactReason` words it for states). Any other
  disagreement with the stored state and record is refused as
  `COMPARISON-RUN-RECORD-MISMATCH`, naming the findings. The result's checked
  profile admits only binary64-exact numbers, so in practice the gap is in the
  snapshot and manifest (JCS); the regression holds 9007199254740993 in the solved
  model.
- **Context deltas.** `runContext` makes one flat map from each run's manifest:
  `solver_name`, `solver_version`, `solver_build_ref`, `solver_mode`,
  `settings.<key>` for each solver setting, and `load_case:<id>` and
  `combination:<id>` for each member of the load basis. `contextDeltas` is the
  reference's `_settings_deltas` over two such maps (keys in code-point order;
  added, removed or changed). Equal JSON data are equal whatever their member
  order.
- **Diagnostics.** A run's solve diagnostics are its record's (the producer's,
  then the rule-pack reference diagnostics). They are compared as multisets keyed
  by code and the affected-ref list as given: removed (left only), added (right
  only), and a count in both.
- **Size admission (Memory).** `admit_analysis_run_comparison {projectId,
  recordingIds, currentRunBytes?}` takes 256 KiB of arguments, as the history
  commands do. It names two different recorded runs, or one and
  `currentRunBytes`; anything else is `COMPARISON-RUNS-ARGS`. In one read
  transaction it first reserves SQLite's working set (#1602's rule). It then
  finds and verifies each run as the run list does (`find_run_with`: identity
  columns bounded before they load, the record and labels metered and checked),
  so a run not of the project is `ANALYSIS-RUN-NOT-FOUND` and an unreadable row
  is refused with its integrity refusal. It reads the stored lengths of each run's
  result and input manifest (`octet_length`, header-only); the manifest is
  counted because the comparison reads it for the snapshot. When their sum,
  plus the current run's result and manifest text as the webview counts them
  (UTF-8 bytes), exceeds the per-invocation budget, it refuses by name
  (`COMPARISON-RUNS-TOO-LARGE`) before either run is read. The response is the
  lengths only, with no id echoed. Every reservation is released when it
  returns.
  - This is a 1× cap on the stored input size, not a bound on webview memory. It
    is a check, not a lease, and has no multiplier. The webview's parse,
    flatten, compare and render memory is unpriced under the documented residual
    ("command working memory and responses are not separately bounded",
    DEL-04-07), as are the read responses' IPC copies (§3, Memory).
- **Parity.** `fixtures/comparison/analysis_run_comparison_golden.json`
  (about 263 KiB) holds three runs as the desktop flattened them: the connected
  preview pair's sparse and dense producer results, and invented edits of the
  sparse one (changed values, a unit change, rows removed, added, repeated and
  moved to another entity, and setting and load-case changes). It also holds
  three pairs with the desktop's comparison under an open gate (both eligible,
  one identity). It is written by `runComparisonParity.test.ts`
  (`OPENPIPESTRESS_WRITE_RUN_COMPARISON_GOLDEN`) and checked by
  `tests/test_analysis_run_comparison_parity.py`. The adapter maps a row to a
  `quantity_result` (`object_ref` as an `Entity` reference, `magnitude` its value)
  and the contexts to `left_settings`/`right_settings`. The pytest requires:
  - the reference's mappings (`derive_exact_result_id_mappings`) to be the
    desktop's, apart from rows the desktop leaves unmapped because only its
    metadata members differ;
  - equal raw magnitudes and raw deltas for every compared pair;
  - the reference's family or unit refusal and no delta for every pair that is
    not compared;
  - equal settings deltas.

  The gate, the extra identity fields, the diagnostics and the current-run side
  are tested in vitest (`runComparison.test.ts`, `compareRuns.test.tsx`).
- **View.** The view shows:
  - each side's description, recording time, state, and its standing as the
    reader gives it (status, route and findings);
  - the basis of the mapping and the gate, and a note when no pair is compared;
  - counts: mapped, compared, non-zero, not compared, left only, right only,
    context differences, and diagnostics added and removed;
  - the context deltas and the diagnostic changes;
  - a table of mapped pairs and unmatched rows, filterable by status
    (compared, non-zero, not compared, either side only), family and result id,
    paged at 200 rows as §4.1's. A compared row shows both values and the delta
    with the unit; any other row shows its reasons.

  It is view-only, with no export (FR-CMP-010 is v0.2), and has no wording of
  standing beyond the reader's own status.
- **Compare….** The one Compare… now takes two saved states or two runs.
  Selecting a run clears the selected states, and selecting a state clears the
  runs. It goes through the one-owner check and the busy refusal, and is
  disabled with the stated reason when busy, when fewer than two are selected, or
  when a selected run was refused. A third selection is disabled with its reason.
- **Residuals.**
  - FR-CMP-005/007/008/009/010 stay v0.2, and no unit is converted.
  - No per-case gate: standing is per envelope (above).
  - The diagnostics comparison is shown, not part of the reference parity (the
    reference lists run diagnostics but does not diff them).
  - The comparison is not persisted and is not a DEL-14-05 comparison-output
    document (SCA-011 holds).

### 4.3 UI

- **History panel:** states and recorded runs, with labels. It provides Save
  Model State…, Open State, Record Run, Compare… (two states or two runs),
  edit labels and assumptions, and delete.
- **Comparison panel:** summary, a filterable table, field paths, deltas with
  units, `not_compared` reasons and unmatched lists.
- **"Within this run"** keeps today's comparison. The user selects the case and
  the combination instead of the hard-coded ids.

**As built (S3c, within this run).** `apps/desktop/src/features/comparison/withinRunComparison.ts`
(options, default, revalidation, packaged comparison and unavailable record),
the Comparison panel's "Within this run" selects (`ComparisonPanel.tsx`), the
package notice (`ComparisonPackageNotice.tsx`), the session's cells
(`resultsSessionState.ts`) and handler (`workspaceSession.ts`).
- **Options and default.** The selects offer each load case and combination that
  has result rows in the Current run (`basis_ref` of type `load_case` or
  `combination`, with its row count), in the solved model's order and then
  code-point (Unicode scalar) order. The default is `load:L-100` →
  `combination:C-OPER-ALT` when both are offered, else the first case and
  combination; with no case or no combination the panel says so instead of
  showing an empty comparison (the demo's combination is withheld, so it says
  there is no combination with rows).
- **The comparison.** `buildPreviewComparison`, unchanged except that a right
  row pairs only with a source row of the left basis (Codex r1 on #1619; before,
  the first source row found anywhere, so a chosen L-200 was compared against
  L-100 rows). The fixed pair's output is byte-identical on every committed
  result fixture; a C-OPER-ALT without L-100 rows now gives 0 pairs instead of
  pairing other cases under an L-100 label.
- **Stale choice: revert with a notice.** The choice stays with the session
  across solves, edits and replacements and applies to every Current run that
  offers both of its bases. When a Current run does not offer one, the session
  drops the choice in the render that first shows that run, reverts to the
  default and names the dropped bases until the next choice ("The choice was
  dropped because a Current run had no result rows for …"), in the Comparison
  panel and beside the package actions in Report and Exports; a dropped choice is
  not restored when a later run offers its bases again. An id the run does not
  offer is never compared, and nothing is carried or reset without the notice.
  (Reverting keeps the panel usable; refusing by name would leave the panel and
  the packages without a comparison until the user acts. The producer's `run_id`
  repeats across solves, so it does not identify a run.)
- **Busy.** The selects are disabled with the reason, and a choice is refused,
  while a project operation or prompt owns the session, a solve runs or a report
  package is prepared; the handler reads these at the call, so a handler from an
  earlier render cannot choose.
- **Packages follow the choice (Agent 0, option A).** The report panel line and
  export JSON, the report package's comparison records, the preview handoff
  summary (`HandoffPanel.tsx`), the export review and the design-workspace
  downloads carry the chosen pair's comparison; without a choice, the fixed pair's
  comparison exactly as before S3c. Each records both bases: the comparison id is
  `comparison-pair:<run>:<left>:<right>` with the run and each basis escaped
  injectively and without `:` (`previewComparisonId`, read back by
  `previewComparisonBases`), and the report summary also holds `left_basis_ref`
  and `right_basis_ref`. The fixed pair alone keeps its pre-S3c sanitized id
  (`comparison:<run>:load-L-100-to-combination-C-OPER-ALT`), so a document built
  without a choice is byte-identical; that form is not injective and is used for
  no other pair. The two forms are disjoint (`comparison:` against
  `comparison-pair:`), and the pair form is read first (Codex r2 on #1619: an
  earlier `comparison:<run>:<left>:to:<right>` form could equal a fixed-pair id). When the builder refuses the chosen pair
  (`ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED`: the committed producer outputs'
  combination rows include `support_reaction_*_v2` and
  `combination_modulus_basis_record`, the known gap routed to T6), each of these
  carries a `comparison_unavailable` record (`openpipestress.technical_preview.comparison_unavailable`:
  both bases, the run, the reason code and the reason) and `comparison_ref`
  names its id; the member is absent otherwise. The Comparison panel, the Report
  section and the Exports section state which comparison the packages include,
  or that they omit it and why. The saved `.swbhandoff` package
  (`handoffPackageSave.ts`) carries no within-run comparison and is unaffected;
  only the preview handoff summary carries the choice.

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
