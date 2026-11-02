# Joint analysis-run publication for hanger sizing (1d-β) and modal (O-8): design proposal

**Status: a proposal, not adopted.** Agent 0 approved the joint analysis-run design for
design only (P2, 2026-11-02). This document adopts no format, store migration or shared
source. It asks for the coordinated decision listed in §11. On adoption, the governing
text moves to the record contract's owner: DEL-14-02, which has no Work directory today,
with DEL-00-06 for the contract statement (§12). Until then, a hanger-sizing result is
not a recordable run (Design `hanger-sizing.md` §8.3).

**Bases read:**
- **main at 723e4058f:**
  - the hanger-sizing result and readers (`hanger-sizing.md` §8.3,
    `core/analysis/hanger_sizing`, `schemas/hanger_sizing_result.schema.json`);
  - analysis record 0.3.0 (`schemas/analysis_run.v0.3.schema.json`);
  - the store (`apps/desktop/src-tauri/src/analysis_runs.rs`, store v13; `model_states.rs`);
  - the TypeScript builder (`apps/desktop/src/services/analysisRunCompatibility.ts`,
    `buildAnalysisRunV03`);
  - the input manifest (`inputManifestService.ts`, `openpipestress.current_session_input_manifest` 1.0.0);
  - the modal design (DEL-04-09 `modal-analysis.md` §1.4, §1.5, §5.2, O-8).
- **C1 native custody at ee7a7f13b** (`claude/rt-perf-c1-custody`, 76 commits ahead of
  main, not merged): `native_persistence.rs`, `analysis_run_v03_build.rs`,
  `native_verdict.rs` (`route_header`). It is cited at that revision, which may change.
- **C1's seven consumer constraints** (Agent 0, 2026-11-02), cited below as C1-1 to
  C1-7.

**Modal parts are marked [modal: open].** Questions to the dynamics lane went through
Agent 0 on 2026-11-02. Where this proposal constrains modal, it states the constraint;
it does not choose modal's document.

## 1. Why the current record cannot carry these runs

Analysis record 0.3.0 is a mechanics-solve record. Each of the following is fixed in its
schema or its producers:
- **The run kind and binding are constants.** `run_kind` is const `mechanics_solve`;
  `run_contract_status.result_binding` is const `received_mechanics_result`; and every
  object is closed (`additionalProperties: false`).
- **Results are tabled rows.** `result_refs` are `ResultRef` rows of one of six
  semantic tables (`SemanticContract.id` enum). The TypeScript and C1 builders resolve each row
  against that six-table registry and refuse any other (`SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED`;
  C1-4).
- **Bases are load cases.** `load_basis_refs` admit `LoadCase`, `Combination` and
  `ResultBasis` only (`ANALYSIS_LOAD_BASIS_INVALID`). A modal mode is not a load case,
  and `ResultBasis` is a representation, not a validated family (C1-5).
- **Inputs are a mechanics manifest.** The input manifest carries
  `model_basis.model_payload`, `solver_basis`, `load_basis`, `active_rule_packs` and
  `external_assets`. It has no slot for a sizing request, a hanger library or a modal
  request. `settings_ref` is only a name over the manifest hash.
- **The store's binding is the bare model.** Record Run requires the manifest's
  `model_basis.model_payload` to hash to the state's `model_hash`. Under C1, the
  admitted intake bytes must also parse to that payload with JavaScript-number
  equality (`bound_intake`, `same_js_data`). A wrapped request such as
  `{request: {model, materials}, sizing, library}` is not covered (C1-3). This proposal
  does not claim the current gate supports it.

So no hanger or modal run fits 0.3.0 without changing it.

## 2. Alternatives

| | Alternative | For | Against |
|---|---|---|---|
| A | **analysis_run 0.4.0 superseding 0.3.0**, with a `run_kind` discriminator | One record format | Under the owner's one-version rule (DEL-14-02 CLM-004, owner 2026-10-10: "each format keeps only its current version"), 0.3.0 would then be retired and refused by name. That contradicts "legacy 0.3 unchanged read". It would also repin every 0.3 producer, reader and fixture: TypeScript, C1's native builder, Python compatibility, the store's checks and the 24 committed `*.analysis_run.json` fixtures (20 under `fixtures/`, 4 desktop test-support). Avoiding that needs an owner exception: two current versions of one format |
| B | **Additions within 0.3.0** (modal O-8's first option) | No version number | Not possible. The constants and closed objects mean any addition changes the accepted set under an unchanged version, so a 0.3.0 record would no longer mean one thing (C1-4: "0.3 version does not admit new task/table/run_kind") |
| C | **A sibling record format for task runs**, with a common envelope and family documents by reference (§3); 0.3.0 unchanged and current for `mechanics_solve` | The one-version rule holds per format. No 0.3 migration or repinning. Families keep their own closed documents: no scalar hanger rows, and no mode-to-LoadCase coercion | A second record format in the store and in the desktop's run list (§7). A new store version, so older builds refuse a newer store (`refuse_newer_store`) |
| D | **One record format per family** | Each family is independent | Divergent formats for the same envelope duties. Contrary to "coordinate shared extensions rather than divergent formats" (C1) |

**Recommendation: C.** It is the only alternative that satisfies "legacy 0.3 unchanged
read, no migration" without an owner exception to the one-version rule. It also keeps
the common record envelope separate from the family-specific result documents, as
Agent 0's P1 requires. "0.4.0" then names nothing; the new format starts at 1.0.0
(§3). If the owner prefers A with an exception, §3–§9 still apply, with
`schema_version: "0.4.0"` and `run_kind` in place of `task_kind`. A's repinning cost is
then the owner's decision (§11, D-1).

## 3. The common record envelope (alternative C)

`document_kind: "openpipestress.analysis_task_record"`, `schema_version: "1.0.0"`,
JSON Schema 2020-12, every object closed. All hashes use `openpipestress_jcs_ijson_v1`
as bare 64-hex (the 0.3.0 hash boundary, DEL-14-02 CLM-004). The proposed name is open
(§11, D-2).

| Member | Content |
|---|---|
| `task_kind` | **Discriminator**, a closed enum. 1.0.0 admits `hanger_sizing` only. `modal` is added when its family section (§5) is adopted. Each kind fixes its input-bundle kind, its result document kind and version, its semantic contract id and its reader. A record whose members belong to another kind is refused (`ANALYSIS_TASK_RECORD_KIND_MISMATCH`) |
| `record_contract` | Constants: `strict_analysis_task_record_v1`; `result_binding: "received_family_document"`; `external_validation_boundary: "reference_only_not_determined_by_software"` (0.3.0's wording) |
| `run_id`, `created_at` | As 0.3.0: `created_at` is `null` in the transient record and allocated inside the Record transaction. The name, tags, notes and assumptions are labels outside the hash, as for 0.3.0 runs |
| `model_state_ref` | `{object_type: "ModelState", ref}`, transient `state:<model ref>:preview` as 0.3.0 |
| `engine` | `{engine_name, engine_version, build_ref}` from the producing build. This is the build that reproduction is claimed against (§6) |
| `input_bundle` | `{document_kind, canonical_profile, sha256, members: [{role, sha256}]}`. The bundle's full canonical text is stored beside the record (§7), not inside it. `members` lists each authoritative input the family document binds, by role, with its hash (§4.1) |
| `result` | `{document_kind, schema_version, semantic_contract_id, status, sha256}`: the family document's identity members copied, plus the hash of its checked canonical text. `status` is the family's own (for hanger: `sized`) |
| `standings` | The family reader's standings on the stored bytes at record time, by name, each `passed`. Recording requires every standing the kind requires (§6). Not a claim of correctness |
| `diagnostics` | The family document's refusals and disclosures, carried unchanged as source annotations (REQ-06-02); no class enum of its own, as 0.3.0 |
| `analysis_status` | Must contain `HUMAN_REVIEW_REQUIRED`; plus `TASK_COMPLETED` (§11, D-4) |
| `immutability_policy`, `professional_boundary`, `provenance` | 0.3.0's members and constants, unchanged in meaning |
| `hashes` | Exactly three checksums, each `{canonicalization, payload_ref, scope, value}` as 0.3.0's: the record's own (its projection without this member, as 0.3.0), the result document's, and the input bundle's |

No `result_refs` rows, `load_basis_refs` or semantic-table row checksums. Rows belong to
the family document and its tables (§5). Rule packs and library references stay out of
the envelope unless a family binds them; hanger binds its library through the bundle.

## 4. Hanger sizing (the implemented PF capability only)

### 4.1 Input bundle and binding

- **Bundle.** Kind `openpipestress.hanger_sizing.input`. Its content is exactly the
  headless input of Design §8.3.2, `{request: {model, materials}, sizing, library}`, as
  admitted (§8.3.1), stored as its checked canonical text. Members and roles:
  - `solve_request`: the hash of `request`;
  - `sizing_request`: the hash of `sizing`;
  - `library`: the hash of `library`.
- **Agreement with the result.** The result document's `inputs` must agree with the
  bundle, or recording is refused (`ANALYSIS_TASK_RECORD_INPUT_MISMATCH`):
  - `solve_request.sha256`, `sizing_request_sha256` and `library.sha256` equal the
    member hashes;
  - `inputs.sizing_request` equals `sizing` as data;
  - `library.library_id` equals `library.hanger_library.library_id`.
- **Route.** Only `route: "pressure_free"` is recordable. The exact route stays refused
  (`HANGER_SIZING_EXACT_ROUTE_PENDING`), and variable-spring preload stays unapplied
  (`SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`). Neither is an envelope capability.
- **Status.** Only a `sized` document is recordable; a `refused` one stays session-only.
  This is the recommendation in §11, D-5.

**Binding the wrapped request to the model state.** This is new. The current gate binds
a bare model (C1-3) and is not reused as if it covered this. In order, each step refuses
before anything is written:
1. The bundle text is its own checked canonical text, and it hashes to
   `input_bundle.sha256`.
2. The model the session sized is `bundle.request.model`. It must be the state's model:
   - for a named state, its canonical text hashes to the state's `model_hash`, and it
     equals the state's stored `model_canonical` as data under §8.3.6's numeric
     equality;
   - for a new `post_solve_snapshot`, the state is created from exactly this model.
3. **Materials.**
   - **Desktop.** The desktop's solve sends `materials: []` (`previewService.ts:164`).
     A desktop-originated bundle therefore has `materials: []`. Any other value is
     refused until a desktop material source exists to bind it
     (`ANALYSIS_TASK_RECORD_MATERIALS_UNBOUND`).
   - **Headless.** A headless bundle may carry materials, and it is not importable
     (§7).
4. The sizing request and the library are authoritative in full in the bundle, not
   only by hash. The library must pass the import boundary again on Record
   (`validate_library_import`, kind `hanger`, visibility private).
5. **Native custody** (C1 owner, increment 3). The sizing job is a custody job whose
   admitted bundle digest the registry records at admission. Record checks the held
   result bytes against the publication digest and the bundle against the admitted
   digest, as C1's `bound_custody` and `bound_intake` do for a solve. A restored
   result is never recordable (C1-6).

### 4.2 Tables, rows and identities

The family document already has the identities. The record adds none, and nothing is
flattened into scalar rows:

| Identity | Form |
|---|---|
| Candidate (table row) | `candidate:<support>`, unique per document |
| Operating entry | `candidate:<support>/case:<case>`, one per operating case, in operating order |
| Selection | `candidate:<support>/record:<hanger_id>` with the record's `record_sha256` |
| Derived solve | `solve:<label>` (`S_W`, `S_C`, `S_O<i>`, `V_C`, `V_O<i>`) with `derived_document_sha256` |
| Case | `case:<id>` with its role (`weight`/`operating`). It is a request reference, not a load-basis row |
| Per-record evaluation (detail view) | `candidate:<support>/record:<hanger_id>`, recomputed from the bundle, never stored (Design §8.1) |

Display tables, the detail view, report sections and CSV (DEL-08-04, after #59) are
separate surfaces built on these identities. None is a 0.3 semantic table.

### 4.3 Readers

The Rust reader and the independent Python verifier of §8.3.7 are the family reader
(the `hanger_sizing` crate's `reader`, and `tests/verifiers/hanger_sizing_evidence.py`).
The record adds an envelope reader for both:
- closure of the envelope;
- the three checksums;
- agreement between the envelope and the family document's identity;
- `task_kind` consistency;
- bundle-to-result agreement (§4.1).

The family readers then run on the stored bytes with the bundle supplied.

## 5. Modal [modal: open]

Constraints from the envelope, for whichever document modal chooses:
- **Result document.** Its document must be addressable by `result.document_kind`,
  `schema_version` and `semantic_contract_id`. Modal's current design (§5.2) publishes a
  `results.v0.3` envelope with profile `product_preview_modal_v1` and table
  `…/modal-1`. The envelope admits that document as the family document; it does not
  require modal to adopt a separate closed document.
- **Bases.** Mode identities stay `basis_ref {ref_type: "mode", ref_id: "mode:<i>"}`
  and cluster refs, inside the family document. The envelope has no
  `load_basis_refs`, so there is no mode-to-LoadCase coercion. A `state_case` is a
  request input (a `LoadCase` reference in the bundle), not a basis.
- **Bundle.** Modal's request members (§1.4) are authoritative in full: the extraction
  bound, the modulus basis, `weight_to_mass_acceleration`, `state_case`,
  `include_dynamic_only_restraints` and `solver_mode`, with the model. A wrapper needs
  the same explicit model extraction as §4.1, steps 1–2.
- **Standings.** Modal's readers do not re-run the eigen-solve (modal §5.2). Its
  restored standings are therefore closure, binding and the re-derived checks, never
  reproduction.

The dynamics lane's six questions (identity, bound inputs, row and table identities,
restored standings, limits, and one joint version versus family additions) decide the
`modal` row of `task_kind`.

## 6. Fresh and restored standings

| | Fresh (held in session) | Restored (reopened from the store) |
|---|---|---|
| Source | The producing job's held bytes, under native custody | Stored `result_json`, `input_bundle_json` and `record_json` bytes, by their hashes |
| Hanger standings available | `closure`, `binding`, `arithmetic_consistent_with_published_positions`, `derivation`, and `reproduction` (this build) | `closure`, `binding` and `arithmetic` on the stored bytes; `derivation` (deterministic from the stored bundle and published roles). `reproduction` is **`not_checked`** on restore: it holds only for the same build and platform (§8.3.7), and a re-run is a new run, not a restoration |
| Recordable | Yes, if every required standing passed | Never. Restored is not Current and cannot be recorded or saved as fresh (C1-6) |
| Mint Current, feed a solve, report or a new record | Only a fresh result | No |

- **Record requires** `closure`, `binding`, `arithmetic` and `derivation` passed.
  `reproduction` is shown, not required.
- **Re-checks on reopen.** A record whose stored bytes no longer hash to its checksums
  is listed as unreadable, as v13 runs are (`unreadable_run_entry`). It is never
  repaired.

## 7. Record, store, read, reopen and re-save

- **Store.** A new table in a new store version (v14 on current main and C1). Columns:
  `project_id`, `recording_id`, `state_id`, `task_kind`, `created_at`, `record_json`,
  `record_hash`, `input_bundle_json`, `input_bundle_hash`, `result_json` and
  `result_hash`.
  - It has the v13 run table's immutability and no-replace triggers, its state foreign
    key (`NO ACTION`) and its cascade from the project row.
  - It has its own label table, or v13's generalized; that is open (§11, D-6).
  - `analysis_runs` and its v13 readers are unchanged.
  - A v14 store is refused by an older build (`refuse_newer_store`). That is the
    forward-compatibility cost of any store change, and owner-visible.
- **Record.** One `BEGIN IMMEDIATE` transaction:
  1. allocate `created_at` as v13 does;
  2. run the binding steps of §4.1;
  3. recompute the three checksums;
  4. require the family readers' standings on the stored bytes to equal the transient
     record's;
  5. write.

  Any refusal writes nothing. Stored bytes are written as submitted, never rewritten
  (C1-7).
- **List and read.**
  - Listing reads the envelope and labels only. It never parses `result_json` or
    `input_bundle_json`, so a summary has no row-scale fields; C1's 4 MiB header rule
    applies.
  - Reading a record prices `result_json` and `input_bundle_json` before parsing.
    An oversized record is refused, typed, as `refuse_undisplayable_run` does.
  - The hanger document scales with the candidates and 2n + 3 solves, not with the
    catalogue (§10). The bundle scales with the model and library; the 64 MiB headless
    cap is a ceiling, not a display budget.
  - A budget-withheld record leaves the model editable (C1-7).
- **Reopen.** This gives a restored result with §6's standings, and never a solve
  intake, publication, capture or Current (C1-6).
- **Re-save.**
  - Save Project does not carry task runs in the project row's current-run slot, so
    the project envelope (`computeProjectEnvelopeHash` over `{model, …, mechanics_result,
    analysis_run, model_hash}`) and `project_persistence` are unchanged. A fresh sizing
    result lives in the session until Record.
  - Copying a project copies the stored record, bundle and result bytes, and their
    checksums, unchanged.
- **Headless.** `size-hangers` stays a document producer. A headless document is not
  importable as a record: it has no session custody, and its hashes identify inputs but
  authenticate no origin (P4).

**Refusal codes** (proposed names, typed per DEL-00-06):
- `ANALYSIS_TASK_RECORD_KIND_MISMATCH`
- `ANALYSIS_TASK_RECORD_INPUT_MISMATCH`
- `ANALYSIS_TASK_RECORD_MODEL_MISMATCH`
- `ANALYSIS_TASK_RECORD_MATERIALS_UNBOUND`
- `ANALYSIS_TASK_RECORD_LIBRARY_NOT_ADMITTED`
- `ANALYSIS_TASK_RECORD_STANDING_FAILED`
- `ANALYSIS_TASK_RECORD_RESTORED_NOT_RECORDABLE`
- `ANALYSIS_TASK_RECORD_TOO_LARGE`
- the v13 custody, staleness and oversize codes, reused where the meaning is the same.

## 8. Legacy 0.3.0

Nothing changes for 0.3.0 or its consumers:
- `schemas/analysis_run.schema.json` and `analysis_run.v0.3.schema.json`;
- `buildAnalysisRunV03` and C1's native builder;
- the six-table registry, `core/analysis_runs`, the store's v13 tables and checks;
- every committed 0.3 fixture.

There is no migration and no repinning. A 0.3 record is read exactly as today. Under
alternative C, 0.3.0 stays the one current version of the analysis record format. A task
record is a different format, refused by every 0.3 reader by name, and that refusal is
tested (as §8.3.8 tests for the hanger document today).

## 9. Shared files and commands (for the implementation, not released here)

| Surface | Files and commands | Owner to coordinate |
|---|---|---|
| Record schema | `schemas/analysis_task_record.v1.schema.json` (new); `tools/release/schema_manifest.json` (regenerated) | DEL-14-02, DEL-00-06 |
| Envelope reader | A Rust crate under `core/` (placement open), plus an independent Python verifier under `tests/verifiers/` | DEL-14-02 |
| Store | `apps/desktop/src-tauri/src/lib.rs` (migration list, v14), a new `analysis_task_runs.rs`, `model_states.rs` (state lookup only) | DEL-14-01, C1 |
| Native custody | `solve_jobs` (a sizing job kind, its bundle digest at admission), `native_persistence.rs` (task-record binding), `native_verdict.rs` (a hanger route-header entry, metadata only, C1-1) | C1 (performance lane) |
| Commands | Record, list, read, delete and label for task runs (new Tauri commands, each with ingress and argument limits as v13's) | DEL-14-01, C1 |
| TypeScript | A task-run service beside `analysisRunService.ts`; the run list shows both formats, each read by its own reader | DEL-14-01 |
| Desktop display, CSV | The hanger table (Design §8.1), CSV after #59 (DEL-08-04) | this deliverable, increment 3 |
| Boundaries | DEL-08-01 reach-scan pins if the runner or shell gains a dependency; `tools/hosted-ci-routing.json` re-derived | DEL-08-01; routing |

## 10. Evidence the adoption needs

- **Independent references.**
  - The family arithmetic keeps R-1 to R-5 (`validation/references/hanger_sizing/`).
  - The envelope needs a hand-built record corpus: one record per kind, built by a
    generator that shares no code with the producer.
- **Parity.** The Rust envelope reader and the Python verifier agree on every committed
  record and mutation (closure, checksums, kind and identity agreement, bundle–result
  agreement).
- **Mutations**, each refused by the named code:
  - bundle swapped;
  - model swapped under the same project id;
  - `materials` non-empty;
  - library swapped, or a record edited;
  - result swapped, or its status changed;
  - `task_kind` against the family document;
  - record checksum, or a stale transient;
  - restored-as-fresh;
  - oversized record;
  - a 0.3 record presented as a task record, and the reverse.
- **Legacy.** Every committed 0.3 fixture and test is unchanged byte for byte. A test
  shows each 0.3 reader refusing a task record.
- **Store.**
  - Migration v13 to v14, idempotent, with v13 rows untouched.
  - A newer store is refused.
  - Immutability and no-replace triggers.
  - Atomicity on every refusal.
  - Size limits before allocation.
- **Fresh and restored.** Fresh and restored standings as §6, including reproduction
  `not_checked` on restore.
- **Scrutiny.** An independent design and persistence review (gpt-6-astra, medium) of
  the adopted text before any schema or store write. Implementation reviews per slice.

## 11. Decisions requested (Agent 0, the owner where reserved)

| Id | Decision | Recommendation |
|---|---|---|
| D-1 | Alternative C (a sibling task-record format, 0.3.0 unchanged) or A (0.4.0 with an owner exception to the one-version rule) | C |
| D-2 | Format name and version: `openpipestress.analysis_task_record` 1.0.0, or another name | As proposed, or Agent 0's choice |
| D-3 | Ownership of the task-record contract: DEL-14-02 (record) with DEL-00-06 (contract statement), the governing text moving to a DEL-14-02 Work design | Yes |
| D-4 | `analysis_status` for task runs: `TASK_COMPLETED` + `HUMAN_REVIEW_REQUIRED`, or REQ-06-04's existing vocabulary only | `TASK_COMPLETED`, so that a completed sizing is never read as `MECHANICS_SOLVED` |
| D-5 | Whether a `refused` hanger document is recordable | No: session-only, as a refused solve |
| D-6 | Separate label table for task runs, or v13's generalized | Separate, leaving v13 unchanged |
| D-7 | Desktop materials: refuse non-empty until a desktop material source exists | Yes |
| D-8 | Modal's row of `task_kind`, and whether modal keeps its `results.v0.3` envelope as its family document | The dynamics lane, through Agent 0 |
| D-9 | Store v14's forward-compatibility cost (older builds refuse the store) | Owner-visible; accept with the first adopted slice |

## 12. Commitment text proposed (old and new; not applied)

**DEL-00-06 `ArchitectureBasis.md`, "Sanctioned additive classes".**

Old:
> Sanctioned additive classes (extensions, not contradictions of REQ-06-02): the analysis-run record keeps one version, 0.3.0 (`schemas/analysis_run.v0.3.schema.json`), which carries each received diagnostic unchanged as a source annotation and declares no warning-class enum of its own; …

New (under D-1 = C):
> Sanctioned additive classes (extensions, not contradictions of REQ-06-02): the analysis-run record keeps one version, 0.3.0 (`schemas/analysis_run.v0.3.schema.json`), for mechanics-solve runs, which carries each received diagnostic unchanged as a source annotation and declares no warning-class enum of its own. Task runs that are not mechanics solves (hanger sizing; modal when adopted) use the analysis task record, one version, 1.0.0 (`schemas/analysis_task_record.v1.schema.json`). It binds a family result document and its full input bundle by hash, carries the family's diagnostics unchanged as source annotations, and declares no warning-class enum of its own. …

The `code_paths` of DEL-00-06 gain `schemas/analysis_task_record.v1.schema.json`.

**DEL-14-02 `ScopeOfWork.md`, CLM-004, "Architecture basis" row.**

Old:
> … DEL-14-02 keeps one version, analysis record 0.3.0, on the checked JCS/I-JSON profile: the owner decided on 2026-10-10 that each format keeps only its current version …

New:
> … DEL-14-02 keeps one version of each record format on the checked JCS/I-JSON profile: analysis record 0.3.0 for mechanics-solve runs, and analysis task record 1.0.0 for task runs whose result is a family document (hanger sizing; modal when adopted). The owner decided on 2026-10-10 that each format keeps only its current version …

**DEL-04-11 `deliverable.yaml`, the DEL-00-06 need.** In "…enters only through the
joint analysis-run 0.4.0 under DEL-00-06's contract, which is approved for design and
not adopted…", replace "the joint analysis-run 0.4.0" with "the joint task-run record
(`Work/joint-analysis-run-proposal.md`)". The rest is unchanged. The same wording
change applies at `ScopeOfWork.md` "Analysis run" and Design §8.1/§8.3.

These replacements are made only after D-1 to D-3 are decided. The proposal itself
changes none of them.
