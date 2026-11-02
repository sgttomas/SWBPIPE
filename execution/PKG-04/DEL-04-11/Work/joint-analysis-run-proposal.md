# Joint analysis-run publication for hanger sizing (1d-β) and modal (O-8): design proposal

**Status: a proposal, not adopted.**
- Agent 0 approved the joint analysis-run design for design only (P2, 2026-11-02).
- This document adopts no format, name, status vocabulary, store migration or shared
  source.
- Its recommendations go to the coordinated decision in §11.
- Agent 0 has assigned the record contract to DEL-14-02, with DEL-00-06 for the contract
  statement (D-3). The governing design is prepared there when adopted; this file is its
  draft.
- Until adoption, a hanger-sizing result is not a recordable run (Design
  `hanger-sizing.md` §8.3).

**Bases read:**
- **main at 723e4058f:**
  - the hanger-sizing result and readers (`hanger-sizing.md` §8.3,
    `core/analysis/hanger_sizing`, `schemas/hanger_sizing_result.schema.json`);
  - analysis record 0.3.0 (`schemas/analysis_run.v0.3.schema.json`);
  - the store (`apps/desktop/src-tauri/src/analysis_runs.rs`, store v13; `model_states.rs`;
    `lib.rs` `refuse_newer_store`);
  - the TypeScript builder (`analysisRunCompatibility.ts` `buildAnalysisRunV03`);
  - the input manifest (`inputManifestService.ts`; `workspaceSession.ts:1144-1158`);
  - report-package save limits (`report_package_save.rs:204`).
- **C1 native custody at ee7a7f13b** (`claude/rt-perf-c1-custody`, not merged):
  - `native_persistence.rs` `bound_custody` and `bound_intake`;
  - `analysis_run_v03_build.rs`;
  - `native_verdict.rs` `route_header`.

  C1's seven consumer constraints (Agent 0, 2026-11-02) are cited as C1-1 to C1-7.
- **Modal:** DEL-04-09 `modal-analysis.md` §1.4, §1.5, §3.4, §5.2 and O-8, and the
  dynamics lane's answers to questions 1–6. Those answers were relayed by Agent 0 on
  2026-11-02, at modal 4cf32bfd4, S2 API 064dc8bb9 and C1 ee7a7f13b, and are cited as
  M-1 to M-6.
- **Dynamics consumer check of r2** (4426a59fc, relayed by Agent 0): no architecture
  objection, subject to five bounded corrections, which r3 applies. This was a
  consumer check, not an independent review or an adoption.
- **Independent design and persistence review of r3** (gpt-6-astra, medium): "revise
  before adoption". C is preferred to A, and the typed manifest payload is kept as the
  input home, both subject to the findings, all of which r4 applies (§10).

## 1. What the current record and manifest can and cannot carry

**The input manifest has a full-payload extension point.** That point is
`solver_basis.settings` (`Record<string, unknown>`):
- `buildCurrentSessionInputManifest` clones it into the manifest
  (`structuredClone(args.solver.settings)`), and the manifest's identity covers it
  (`rfc8785_jcs` over the entire manifest);
- Record Run (v13) and C1 store the manifest bytes (`input_manifest_json`);
- today it carries two keys, `nonlinear_iteration_policy` and `sparse_evidence_lane`;
- it is not a general JSON payload slot:
  - its validator (`isCompleteJson`, `inputManifestService.ts:249-259`) rejects `null`
    at any depth, and non-finite numbers;
  - the manifest's arguments require at least one load case.

  A task payload with legitimate nulls, or a load-free modal model, needs task-specific
  validation (§2.1).

So a task's payload has a home. What is missing is three things:
- a typed, validated task payload;
- validators that read it;
- a binding from the wrapped task to the model state.

The record's `settings_ref` is only a name over the manifest hash. It carries nothing.

**Analysis record 0.3.0 cannot carry a task result:**
- `run_kind` is const `mechanics_solve` and `result_binding` is const
  `received_mechanics_result`, and every object is closed;
- results are `ResultRef` rows of six semantic tables, resolved by the TypeScript and C1
  builders, which refuse any other table (C1-4);
- `load_basis_refs` admit `LoadCase`, `Combination` and `ResultBasis` only, and
  `ResultBasis` is a representation, not a validated family (C1-5).

**The store's binding is the bare model.**
- Record Run requires `model_basis.model_payload` to hash to the state's `model_hash`.
- Under C1, the admitted intake bytes must also parse to that payload with
  JavaScript-number equality (`bound_intake`).
- A wrapped task is not covered (C1-3, M-4). Model extraction and equality of every
  option must be designed and implemented. This proposal claims nothing for the current
  gate.

## 2. Alternatives

There are two separate questions: where the authoritative inputs live (§2.1), and which
record carries the result (§2.2).

### 2.1 Where the authoritative inputs live

| | Typed task payload in `solver_basis.settings` (existing manifest) | A new input bundle beside the record |
|---|---|---|
| Storage | Existing `input_manifest_json`, already stored by v13 and C1 | A new stored column or blob |
| Identity | The manifest hash (`rfc8785_jcs`), already in the record's `reproducibility` | A new bundle hash |
| Model | `model_basis.model_payload`, the model part of the task request. The bare-model binding applies to it unchanged | Inside the wrapper; needs extraction |
| Typing | Needs a closed per-kind schema under a new `settings.task` member, and task-specific validators. Today's validator rejects nulls and requires a load case (§1) | Closed by its own schema |
| Size | The manifest grows by the library (hanger) or the options (modal). Consumers with manifest caps (`MAX_INPUT_MANIFEST_TEXT_BYTES` = 2 × the model-document maximum, report-package save) refuse larger manifests; a large private library could reach that cap | The bundle has its own cap; the manifest is unchanged |
| Privacy | **Existing reports emit settings content.** `renderableReportInput.ts:657-678` (TypeScript) and `report_package/src/user_release.rs:640-667` (Rust) copy every settings key and its canonical value into `report_context.solver_settings`, and `report_renderer/src/lib.rs:703-709` renders each as a "Solver setting" row. A private payload under `settings.task` would reach report content wherever those consumers receive the manifest. Hash-only package binding and `included_as_package_member: false` do not prevent this. Today's mechanics and user-release eligibility checks mean no unimplemented task route exports a library now, but the condition "no export emits settings content" is false (D-10) | The private payloads stay out of the manifest and its report projections |
| Lanes | Modal names the existing settings as its proposed payload home (M-4, M-5) | One new surface for both |

**Recommendation: the typed task payload in the existing manifest's settings.** It
reuses the stored manifest, its identity and the bare-model binding for the model part.
It is what modal proposes, and it adds no stored column.

**Conditions:**
1. **A closed `settings.task` schema per kind,** validated on build and on Record by
   task-specific validators:
   - legitimate nulls, and load-free models where a family needs them, are admitted
     inside `settings.task` only;
   - the mechanics manifest's validation and output stay unchanged.
2. **Checked admission of the original task input** under `openpipestress_jcs_ijson_v1`
   (§8.3.1) before any lossy parse. Two separate identities are then computed: the
   manifest's `rfc8785_jcs` identity, and the checked family hashes. Neither substitutes
   for the other.
3. **Explicit refusal at the manifest caps,** with no truncation.
4. **D-10, a consumer change in both languages,** before adoption:
   - the report context's settings projection becomes a reviewed allow-list that
     excludes `settings.task`'s private payloads and keeps their identity references
     (kind and hashes);
   - implementation carries sentinel-based export regression checks: a marker string in
     a private payload must appear in no report context, rendered report or package.

**Request hash and request payload stay distinct.**
- The manifest carries the payloads.
- On Record, each family hash is recomputed from the payloads by **that family's own
  binding**; a hash never stands in for a payload.
- **For hanger** (§4.1), the family document binds `solve_request.sha256`, the hash of
  the checked canonical `{model, materials}`, and binds the sizing request and library
  separately.
- **For modal** (§5), the per-kind request and its hash projection are open. They must
  bind every option, the state, the extraction bound, the basis, `a`, the materials
  and the model. Modal's task hash is not hanger's solve-request hash.
- **Every kind.** The only unhashed members are **display annotations that nothing
  consumes**: the run's name, tags, notes, and free-text assumption remarks shown with
  it. No computation, validation or reader reads them.
  - Every engineering, model, state, extraction or constitutive assumption that a
    computation or validation uses is hashed, authoritative payload, never a label.
  - A remark that would change a result belongs in the task payload instead, or it is
    not offered.

### 2.2 Which record carries the result

**The rule in force:**
- DEL-14-02 CLM-004: "DEL-14-02 keeps one version, analysis record 0.3.0 … the owner
  decided on 2026-10-10 that each format keeps only its current version ("as you
  recommended, no-compatibility principle carries"), so the 0.1 sorted-compact and 0.2
  records are refused by name."
- DEL-00-06: "the analysis-run record keeps one version, 0.3.0."
- The dispatcher: "Records of 0.1.0 and 0.2.0 are retired and refused by name
  (ANALYSIS_RUN_SCHEMA_VERSION_RETIRED); no migration exists. Re-run the analysis to
  record a current run."

**What today's compatibility actually requires:**
- 0.3.0 records are read, verified and re-saved exactly as stored, under their recorded
  hash, table and version;
- JavaScript-binary64 historical forms stay distinct from checked hashes (C1-7);
- there is no migration and no repinning.

| | Alternative | Consequence under the rule |
|---|---|---|
| A | **analysis_run 0.4.0 superseding 0.3.0**, with a `run_kind` discriminator | 0.4.0 becomes the one current version, so 0.3.0 is retired and refused by name, as 0.1 and 0.2 were. Stored 0.3.0 records keep their bytes, but they are no longer admitted by current-version validation, so supported reopening and re-saving of them cannot be promised ("re-run the analysis"). Every 0.3 producer, reader and fixture is retired or repinned: TypeScript, C1's native builder, `core/analysis_runs`, the store's checks and 24 committed `*.analysis_run.json` fixtures. Avoiding that needs an **owner policy decision**: two current versions of one format, or a retained archival 0.3.0 reader, each an exception |
| B | **Additions within 0.3.0** | Not available. The constants and closed objects mean an addition changes the accepted set under an unchanged version (C1-4) |
| C | **A sibling task-record format** with a common envelope and typed family results by reference; 0.3.0 unchanged | The rule holds per format. 0.3.0 keeps its current status and behaviour |
| D | **One record format per family** | Divergent formats for the same envelope duties, contrary to C1's "coordinate shared extensions" |

**C is the preferred design alternative** (Agent 0, D-1/D-2), and it is not adopted.
It is a distinct record, not 0.3.0 renamed. It does not hold or succeed any mechanics
record:

| | Analysis record 0.3.0 | Task record (C) |
|---|---|---|
| What it records | One mechanics solve | One typed task whose product is a family document derived from solves (hanger) or an eigen-analysis (modal) |
| Result binding | `received_mechanics_result`: rows of the six mechanics tables | A family document with its own kind, version and semantic contract, bound whole by hash |
| Bases | Load cases and combinations | Family identities (candidates and solves; modes); cases appear only as request references |
| Readers | The six-table row readers | The family's own readers (§4.3, §5) |
| `mechanics_solve` | The only kind | **Not admitted.** A mechanics solve can never be recorded as a task record, so 0.3.0's role is not split or taken over |

**C's own evolution.** The rule that rejects B applies to C too. If the envelope fixes
`task_kind` as a closed enum, adding `modal` later changes its accepted set. That needs
a successor envelope version, which under the one-version rule would retire stored
hanger task records. Two forms of C differ here:

| Form | Description | Adding modal |
|---|---|---|
| C-closed | One envelope version whose closed `task_kind` enum lists the admitted families | A new envelope version, retiring the earlier one, as for A |
| **C-layered** (recommended) | A stable common envelope plus a separately versioned **family binding** per kind. A record names its family binding (`family_binding: {id, version}`), and the envelope dispatches validation to it. The envelope's own accepted set is structural and does not list families | A new family binding (modal 1.0.0); the envelope and the hanger binding are unchanged, so stored hanger records stay current |

C-layered is the reviewer's "minimal separately versioned family-document binding": C
with family admission moved out of the envelope's version. It is an eligible, preferred
**design**, and it adopts no exception to the one-version rule. Stated plainly: under
C-layered, the set of records the format accepts **widens** when a family is added.
A C-closed enum would widen the same way. C-layered does not avoid that; it makes the
widening an explicit, versioned registry step. Whether that is permitted is the owner's
policy decision (D-11).

**The model D-11 would permit:**
- **Two acceptance layers.**
  - *Syntax acceptance* is the envelope schema at its one version: structure, closure,
    checksums, and the presence and form of `family_binding: {id, version}`.
  - *Semantic admission* is the named family binding at its named version: the
    `settings.task` schema, the family document's kind, version and contract, and its
    reader.
- **Unknown family.** A syntactically valid record naming a binding this build does not
  recognize gets *recognition refusal*. It is not admitted, read for standings,
  recorded or re-saved as a recognized run. It may be listed for inspection by its
  envelope only, and is shown as "family not supported by this build".
- **Allowed and forbidden registry changes.**
  - *Allowed:* registering a new binding id, or a new version of an existing binding
    alongside the earlier one.
  - *Forbidden:* changing an existing `{id, version}`'s meaning. Its schema, document
    kind, contract and reader are fixed once registered.
  - A breaking change to a family is a new binding version. Whether an earlier binding
    version stays admitted after a successor is itself the one-version question, at
    family level, and needs the owner's answer.
- **Immutability.** A record's `family_binding` is part of its hashed envelope and
  never changes. A re-save keeps it byte for byte.

**Compatibility consequences, concretely:**

| Reader and store | Hanger task record | Modal task record, added later |
|---|---|---|
| A build from before any task record (0.3.0 only) | Not seen: the store refuses newer versions (D-9) | Not seen, as for hanger |
| A build with the envelope and the hanger binding | Admitted | Recognition refusal; envelope inspection only |
| A build with both bindings | Admitted | Admitted |
| Any build | 0.3.0 records unchanged, read as today | Unchanged |

- **Store.** The task table arrives with the first task-record build's store migration.
  A later family needs no further store migration if it fits the same columns.
- **What the policy is not.** It does not claim the accepted set is unchanged. The
  owner's decision packet asks whether to permit this explicitly versioned family
  extension, against the concrete alternatives: C-closed with a successor envelope
  version that retires the earlier one, or A.
- **What it does not avoid.** A change to the envelope itself is still a version event
  under the one-version rule, as it would be for A.

If the owner instead decides A with an exception, §3–§10 still apply with
`schema_version: "0.4.0"` and `run_kind` in place of `task_kind`. The repinning list and
the read consequence above are then the owner's decision (§11, D-1).

## 3. The common envelope (alternative C)

The proposed name is `openpipestress.analysis_task_record` 1.0.0, which is not
decided. It is JSON Schema 2020-12, every object closed, and its hashes are
`openpipestress_jcs_ijson_v1` bare 64-hex (the 0.3.0 record's hash boundary).

| Member | Content |
|---|---|
| `task_kind`, `family_binding` | `task_kind` is the discriminator and is never `mechanics_solve`. `family_binding: {id, version}` names the separately versioned binding (C-layered, §2.2), which fixes the kind's `settings.task` schema, family document kind and version, semantic contract id and reader. The first binding is `hanger_sizing` 1.0.0; `modal` gets its own when adopted. A record naming an unregistered binding, or members of another kind, is refused (`ANALYSIS_TASK_RECORD_KIND_MISMATCH`) |
| `record_contract` | Constants: `strict_analysis_task_record_v1`; `result_binding: "received_family_document"`; `external_validation_boundary` with 0.3.0's wording |
| `run_id`, `created_at`, `model_state_ref` | As 0.3.0, including the transient `null` `created_at` and `state:<model ref>:preview`. Outside the hash are only non-consumed display annotations: name, tags, notes and free-text assumption remarks that no computation, validation or reader reads. Any assumption used in computation or validation is in the hashed task payload (§2.1) |
| `engine` | `{engine_name, engine_version, build_ref}`: the build that reproduction is claimed against (§6) |
| `reproducibility` | 0.3.0's form: `input_manifest_refs` and `input_manifest_hashes`, one entry each. The manifest carries the typed task (§2.1). Also `task_input_hashes: [{role, sha256}]`: each family-bound input's checked hash, recomputed on Record from the manifest's payloads |
| `result` | `{document_kind, schema_version, semantic_contract_id, status, sha256}`: the family document's identity members and the hash of its checked canonical text |
| `standings` | The family reader's required validation checks on the stored bytes at record time, each `passed`. These are the family's validation checks, not every health or census indicator being good: a validated modal result may truthfully carry Sensitive standings where the modal contract permits them, and completed clusters while the whole finite census stays unresolved (§5). Published clusters are always complete: "A partial cluster is never published" (modal §1.4). Not a claim of correctness |
| `diagnostics` | The family's refusals and disclosures, carried unchanged as source annotations (REQ-06-02) |
| `analysis_status` | Not settled; options in §3.1 |
| `immutability_policy`, `professional_boundary`, `provenance` | 0.3.0's members and constants, unchanged in meaning |
| `hashes` | Exactly two checksums in 0.3.0's form: the record's own (over its projection without this member) and the result document's. The manifest's hash is in `reproducibility` |

There are no `result_refs` rows and no `load_basis_refs`. Rows belong to the family
document.

### 3.1 Status vocabulary (for review; nothing minted)

REQ-06-04 requires distinguishing:
- invalid input;
- an incomplete model;
- a mechanics result;
- a rule-check result;
- states needing human review.

0.3.0's status enum is `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`,
`USER_RULE_CHECKED`, `USER_RULE_FAILED` and `HUMAN_REVIEW_REQUIRED`, with
`HUMAN_REVIEW_REQUIRED` always present. It has no separate invalid-input status:
invalid input is carried by typed refusals, and a task record keeps that distinction
the same way.

| Option | For | Against |
|---|---|---|
| (a) 0.3.0's vocabulary, with `MECHANICS_SOLVED` for a completed task | No new term. A sizing is derived from mechanics solves, and an eigen-analysis is a mechanics result in the broad engineering sense | `MECHANICS_SOLVED` is tied to the specific `mechanics_solve` record contract. Reusing it for a task record invites confusion between the two record formats |
| (b) A family-neutral completion status beside `HUMAN_REVIEW_REQUIRED`, for example `TASK_COMPLETED` | Never read as a mechanics solve | A new class under REQ-06-04 needs DEL-00-06's review, and it is not settled |
| (c) Family-qualified statuses | Most specific | Grows the vocabulary per family |

**Leaning:** (b), subject to DEL-00-06. The independent review found (b) reasonable.
`TASK_COMPLETED` would mean that the algorithm completed. It would not mean suitability,
validation success or acceptance. Under every option,
`HUMAN_REVIEW_REQUIRED` stays mandatory, and the family's own standing (hanger's
`verification_standing`) stays in the family document.

## 4. Hanger sizing (the implemented PF capability only)

### 4.1 Task payload and binding

**`settings.task`** for `hanger_sizing`, closed:
`{kind: "hanger_sizing", materials, sizing_request, library}`.
- `sizing_request` is Design §8.3.4's `SizingRequest`.
- `library` is the whole hanger library document.
- `materials` is the solve request's material array.
- The model is `model_basis.model_payload`.
- The task request the engine sizes is
  `{request: {model: model_payload, materials}, sizing: sizing_request, library}`,
  which is exactly the headless input of §8.3.2.

**Agreement with the result,** recomputed on Record. A failure gives
`ANALYSIS_TASK_RECORD_INPUT_MISMATCH`:
- `solve_request.sha256` = H(`{model, materials}`);
- `sizing_request_sha256` = H(`sizing_request`), with `inputs.sizing_request` equal to it
  as data;
- `library.sha256` = H(`library`), with `library_id` equal;

where H is the checked hash.

**Route and status:**
- Only `route: "pressure_free"` is recordable. The exact route stays refused
  (`HANGER_SIZING_EXACT_ROUTE_PENDING`), and preload stays unapplied.
- **Refused documents** (D-5). In this first slice, a `refused` hanger document is not
  recordable. It is displayed truthfully in the session, with every refusal. This says
  nothing about refused mechanics solves.

**Binding steps on Record,** each refusing before any write:
1. The manifest validates, `settings.task` included, against its closed per-kind
   schema.
2. The bare-model binding applies unchanged to `model_payload`: its hash is the state's
   `model_hash`, and under C1 the intake equals it. This binds the model only.
3. The task's other operands are bound explicitly:
   - **Materials.** The desktop's product solve sends `materials: []`
     (`previewService.ts:164`), so the desktop route admits only `materials: []`
     (`ANALYSIS_TASK_RECORD_MATERIALS_UNBOUND`). This is a restriction of the desktop
     recording route to the inputs that route actually has. It is not a rule on model
     validity, and a headless document with materials stays a valid document (D-7).
   - **Library.** It passes the import boundary again (`validate_library_import`, kind
     `hanger`, visibility private).
   - **Sizing request.** It matches `$defs.SizingRequest`.
4. The result agrees with the payloads, as above.
5. **Native custody** (C1 owner, increment 3). The sizing job records at admission the
   digest of the typed task it was given: manifest, model and task payload. Record
   checks the held result against its publication digest and the task against that
   digest, as `bound_custody` and `bound_intake` do for a solve. A restored result is
   never recordable (C1-6).

### 4.2 Identities

The family document already carries these; the record adds none and flattens nothing:
- candidate `candidate:<support>`;
- operating entry `candidate:<support>/case:<case>`;
- selection `candidate:<support>/record:<hanger_id>`, with `record_sha256`;
- derived solve `solve:<label>`, with `derived_document_sha256`;
- case `case:<id>` with its role, as a request reference;
- the per-record evaluations of the detail view, recomputed and never stored.

Display tables, the detail view, report sections and CSV (DEL-08-04, after #59) are
separate surfaces.

### 4.3 Readers

The family readers are §8.3.7's Rust reader and independent Python verifier. An envelope
reader is added in both languages. It checks:
- closure;
- the checksums;
- identity agreement with the family document;
- `task_kind` against `settings.task.kind`;
- the payload-to-result agreement of §4.1.

## 5. Modal (from M-1 to M-6; open items stay open)

- **State.** Modal's publication is governed future intent and is not an implemented
  route (M-1). The design names:
  - profile `product_preview_modal_v1`;
  - table `openpipestress.result_semantics/0.3.0/modal-1`;
  - two fixed limitations;
  - a planned run kind;
  - independent Rust and Python readers.

  The document kind, result schema version, producer, table bytes and hash, and O-8
  compatibility are unknown. A typed modal result with a family reader is a proposal;
  there is no forced mechanics envelope and no final 0.4 id.
- **Task payload** (M-2, M-4). In the existing manifest settings, authoritative in full:
  - the extraction bound: positive `mode_count` and/or `frequency_cutoff`, with no
    default, cluster completion governing;
  - `solver_mode`;
  - `state_case`, required when the document is a load or reference-state profile or
    document, or when the model has nonlinear supports (modal §1.4). A plain model with
    load cases does not need one;
  - the modulus basis, only without a state case, and otherwise a conflict;
  - `weight_to_mass_acceleration`, positive, for component mass;
  - the later dynamic-restraint choice.

  The model, the outer materials actually used, every option's presence and units, and
  the effective case, material, section, support, library and rule operands are
  payloads, not digests.

  Two things are not settings:
  - the mass formulation is decided, not a user choice;
  - Sturm shifts are computed evidence.
- **Rows and identities** (M-3):
  - Per mode: f, ω and T; six components per node with the normalization case and
    sign; the generalized mass; Γ and the effective mass in X, Y and Z, with fractions
    and cumulative fractions. Then the category, free and total masses, and cluster
    completion and counts.
  - Units follow the normalization case, as modal §1.5 gives them.
  - Undefined fractions stay undefined, with their reason.
  - **Below-normal-range values.** A nonzero published value below the binary64 normal
    range carries a per-value below-normal-range flag (modal `BelowNormalRange`, §3.2).
    It stays distinct from an unflagged 0. NaN and infinity are never substituted.
  - `mode:<i>` is run-local only, not a persistent physical identity. The concrete index
    and row spellings, and the signatures, are unknown.
  - The global mass summary needs an explicit model or group basis, not mode 0 and not
    a blanket all-rows mode (M-3). **This conflicts with modal §5.2 as written** ("Each
    row carries `basis_ref {ref_type: "mode", …}`"). It is a governing-text change to
    coordinate with the modal lane on adoption, not a statement of the current
    contract. Modal §5.2's planned `modal` run kind in `analysis_run.v0.3` is likewise a
    text to reconcile; O-8 leaves its compatibility open.
  - Rows are `ResultRef`-like: id, ordinal and canonical checksum. There is no LoadCase
    coercion.
  - RT-R2 views need their own family definition; storage alone gives no UI support.
- **Readers** (M-4). The governed full reader re-derives the model-based free and total
  masses and checks, without an eigen-solve:
  - normalization and sign;
  - Γ²m = m_eff, the fractions and cumulative fractions;
  - ordering, clusters, completeness and the numerical evidence (modal §5.2, §3.4).
- **Fresh standing** (proposed by modal, M-4). Fresh closure binds:
  - the captured typed task with all its options;
  - the model and material sources, and the actual state basis;
  - the family core and TypeScript reader with its full verdict, S-health and current
    standing;
  - the publication digest, job mode and Solve origin.

  The fresh evidence carries η, the conditioning, e_λ, orthogonality, formation, the
  count, Sensitive and refused states. It is not the static M03 standing. The strict
  seven-member route header grants no standing, attestation or Current (C1-1).
- **Restored** (M-4).
  - It inspects the preserved result, record and manifest under the recorded hash,
    table and version, with no fresh intake and no Current.
  - A pure evidence recheck uses the complete authoritative payload only, with no new
    state, no eigen-solve and no reconstruction from digests.
  - A missing payload gives scoped findings, a no-model summary or "unsupported".
  - Re-save keeps the original record, references, checksums and envelope basis.
  - JavaScript-binary64 historical and checked hash profiles stay distinct.
- **Limits** (M-5).
  - Source resource basis: the dense kernel is 32n²B, and the combined product with
    Sturm is about 64n², an estimate and not a final count. Work is 9n³ + s·n³/3, plus
    band 4pn² in exact product insertions and 2n² visits (not binary64 flops).
  - No modal **product** time, output-byte budget or worker qualification has been
    measured. Kernel-buffer and static-stack evidence exists. The ~64n² estimate stays
    separate from the exact product cost, and paging, depth and worker qualification
    remain unknown.
  - Outputs are O(k·n), plus mass, cluster and Sturm evidence.
  - The family must keep row-scale arrays out of the 4 MiB header, and must define
    bounded pages, detail, summary and CSV.
  - The 127-container and deep-header risks need a family assessment.
  - A withheld run leaves the model editable.
  - No modal native worker, attestation or token exists, and the I1 retained-stack
    witnesses are not modal desktop qualification.
- **Open** (M-6): the final kind, version and run kind; DEL-00-06 and the owner policy
  decision; the task extraction and the manifest, result and store closure; the table,
  signatures and row basis, including the global mass; the route, readers, summary,
  CSV, headless and historical parity; the angular-frequency-squared catalog
  extension; and the S2 single-resolution and original-effective-case observation.
- **Production pins and format conditions** before engineering publication:
  - production pins for O-6, identity, residual, orthogonality, cluster, QL and Sturm,
    and the O-4 work guard. The test τ_e of 1e-6 is not a production value.
- **Standing rules** (unchanged):
  - **Unresolved census.** Where the census is general or not a coordinate null space,
    and no independent census is established, it stays unresolved. There is no rank
    truncation or rescue.
  - **Whole-spectrum endpoint.** Refused (`MODAL_FINITE_COUNT_UNRESOLVED`), but the
    valid lower-cluster or gap path remains.
  - **Above-band modes.** Not judged by τ_e, and the full per-mode standing is
    mandatory.
- **Release and completeness obligations** (separate, and not a gate on every modal
  preview):
  - **Benchmarks.** The O-3 benchmark rights and comparison. Modal §4.4 and D-q permit
    the M1–M3 reference classes a and b, with FR-SOL-016 PARTIAL until a published
    benchmark is compared after M1b. The rights-uncertain source hold is unchanged,
    with no transcription.
  - **Documentation.** The guide and theory.

  This neither weakens nor adds a gate. No dates or pins are promised.

## 6. Fresh and restored standings (hanger)

| | Fresh (held in session) | Restored (reopened from the store) |
|---|---|---|
| Source | The sizing job's held bytes under native custody, and the session's manifest | Stored record, result and manifest bytes, under their recorded hashes and versions |
| Standings | `closure`, `binding`, `arithmetic_consistent_with_published_positions`, `derivation`, and `reproduction` (this build) | `closure`, `binding`, `arithmetic` and `derivation` from the complete stored payloads. `reproduction` is `not_checked`: §8.3.7 claims it for the same build and platform only, and a re-run is a new run. A missing payload gives scoped findings, never a reconstruction from digests |
| Recordable | Yes, if `closure`, `binding`, `arithmetic` and `derivation` passed (`reproduction` shown, not required) | Never. Restored is not Current (C1-6) |

**Integrity checks, by step:**
- **Listing** checks the envelope and the labels, and a record failing them is listed
  as unreadable, as v13 does (`unreadable_run_entry`).
- **A full read** checks the result and the manifest against their hashes. A corrupted
  payload may therefore appear only when the record is opened, not in the list.

Nothing is repaired.

## 7. Record, store, read, reopen and re-save

- **Store.** A new run table and a new label table, in **the next store migration
  allocated at implementation** (no number reserved; D-9).
  - The run table's columns are `project_id`, `recording_id`, `state_id`, `task_kind`,
    `created_at`, `record_json`, `record_hash`, `input_manifest_json`, `result_json` and
    `result_hash`.
  - It has the v13 run table's immutability and no-replace triggers, its state foreign
    key and the project cascade.
  - `analysis_runs` and its v13 readers are unchanged.
- **Separate labels (D-6).** v13's `analysis_run_labels` has a foreign key to
  `analysis_runs`. Generalizing it would mean rebuilding an existing, data-bearing
  table, because SQLite cannot alter a foreign key.
  - Store migrations are routine, and the format rule's "no migration" is about retired
    record formats, not store tables. A separate table is recommended because it leaves
    existing rows and their referential integrity untouched, with less migration risk.
  - Its cost is a second set of label statements. Shared SQL helpers select only fixed
    internal table names, never a caller-supplied identifier.
- **State relationships.** `model_states.rs` counts a state's references from
  `analysis_runs` only (around lines 1285-1289 and 1688-1694), and its deletion path
  uses those counts to explain what blocks a deletion.
  - Task runs must be added to the state reference counts, the handling of unreadable
    references, and the deletion explanations.
  - Otherwise a state referenced only by task runs would show no recorded runs, and its
    deletion would fail on the foreign key instead of on the named blocker.
  - The state's `NO ACTION` relationship and the project cascade are kept. This is a
    consumer update. It does not allow task evidence to be deleted implicitly.
- **Record.** One `BEGIN IMMEDIATE` transaction:
  1. allocate `created_at`;
  2. run the selected family's binding steps (hanger §4.1; modal §5 when adopted);
  3. recompute the checksums;
  4. require the family readers' standings on the stored bytes to equal the transient
     record's;
  5. write.

  Any refusal writes nothing. The first recording constructs the timestamped,
  state-bound revision from the transient record, as v13 does. From then on, that
  stored revision's bytes are preserved exactly (C1-7).
- **List and read.**
  - Listing reads the envelope and labels only, never `result_json` or the manifest, so
    there are no row-scale fields in the 4 MiB header (C1-7, M-5). The envelope copies
    the family's diagnostics, so the envelope's serialized size is bounded explicitly,
    and listing admits the aggregate of all listed envelopes against a budget. Leaving
    result rows out alone bounds nothing.
  - Reading prices the result and manifest before parsing, and an oversized record is
    refused, typed, as `refuse_undisplayable_run` does.
  - A budget-withheld record leaves the model editable.
- **Reopen.** A restored result with §6's standings, and never a solve intake,
  publication, capture or Current.
- **Re-save.**
  - Save Project does not put task runs in the project row's current-run slot, so the
    project envelope (`computeProjectEnvelopeHash`) and `project_persistence` are
    unchanged.
  - A fresh sizing result lives in the session until Record.
  - Re-save copies the stored revision's record, manifest and result bytes, and its
    checksums, unchanged.
- **Headless.** `size-hangers` stays a document producer and is not importable as a
  record: it has no session custody, and its hashes identify inputs without
  authenticating an origin (P4).
- **Owner-visible store cost (D-9).**
  - **Data access.** Once a build with the new migration opens a store, an older build
    refuses the whole store: "STORE-SCHEMA-NEWER … Nothing was read or written; open it
    with that build" (`refuse_newer_store`). This covers every project in that store,
    not only those with task runs.
  - **When it happens.** The migration runs when the store is opened, not at the first
    task recording.
  - **Rollback.** A copy of the store taken before the migration restores that earlier
    snapshot only. Writes made after the migration cannot be recovered into the older
    build.
  - **Implementation requirements.** The backup is consistent with SQLite and WAL, and
    an interrupted migration can be recovered.

  These must be decided before adoption.

**Refusal codes** (proposed names, typed per DEL-00-06):
- `ANALYSIS_TASK_RECORD_KIND_MISMATCH`
- `ANALYSIS_TASK_RECORD_TASK_INVALID`
- `ANALYSIS_TASK_RECORD_INPUT_MISMATCH`
- `ANALYSIS_TASK_RECORD_MODEL_MISMATCH`
- `ANALYSIS_TASK_RECORD_MATERIALS_UNBOUND`
- `ANALYSIS_TASK_RECORD_LIBRARY_NOT_ADMITTED`
- `ANALYSIS_TASK_RECORD_STANDING_FAILED`
- `ANALYSIS_TASK_RECORD_RESTORED_NOT_RECORDABLE`
- `ANALYSIS_TASK_RECORD_TOO_LARGE`
- the manifest's size refusals;
- v13's custody, staleness and oversize codes, where the meaning is the same.

## 8. Legacy 0.3.0 under C

Nothing changes:
- `analysis_run.schema.json` and `analysis_run.v0.3.schema.json`;
- `buildAnalysisRunV03` and C1's native builder;
- the six-table registry, `core/analysis_runs`, the v13 tables and checks;
- the input manifest's existing members and meaning;
- the 24 committed `*.analysis_run.json` fixtures.

A 0.3.0 record is read, verified and re-saved as today. A task record is refused by
every 0.3 reader by name, and that refusal is tested. The manifest's new
`settings.task` is absent from mechanics-solve manifests, so their bytes and hashes are
unchanged.

## 9. Shared files and commands (for implementation; not released)

| Surface | Files and commands | Owner to coordinate |
|---|---|---|
| Record schema | A new task-record schema; `tools/release/schema_manifest.json` (regenerated) | DEL-14-02, DEL-00-06 |
| Task payload | A per-kind `settings.task` schema; `inputManifestService.ts` validation; the Rust manifest checks (`analysis_runs.rs` `manifest_members`, and C1's builder's manifest checks) | DEL-14-01, C1 |
| Envelope reader | A Rust reader (placement open) and an independent Python verifier | DEL-14-02 |
| Store | `lib.rs` migration list (next number), a new task-run module, `model_states.rs` (reference counts, unreadable-reference handling and deletion explanations, §7) | DEL-14-01, C1 |
| Native custody | `solve_jobs` (a task job kind, its task digest at admission), `native_persistence.rs` (task binding), `native_verdict.rs` (a family route-header entry, metadata only, C1-1) | C1 |
| Commands | Record, list, read, delete and label for task runs, each with v13's ingress and argument limits | DEL-14-01, C1 |
| TypeScript | A task-run service; the run list shows both formats, each through its own reader | DEL-14-01 |
| Display, CSV | Hanger table (Design §8.1); CSV after #59 (DEL-08-04); modal views (RT-R2, its own definition) | each family |
| Boundaries | DEL-08-01 reach-scan pins on any runner or shell dependency; routing re-derived | DEL-08-01 |

## 10. Evidence the adoption needs

- **Independent references:**
  - R-1 to R-5 for the hanger arithmetic;
  - a hand-built task-record corpus, with a generator sharing no producer code;
  - the modal references of modal §4.
- **Parity.** The Rust envelope reader and the Python verifier agree on every committed
  record and mutation.
- **Mutations**, each refused by its named code:
  - the task payload swapped or retyped;
  - the model swapped under the same project id;
  - hanger only: `materials` non-empty on the desktop route (D-7);
  - the library swapped, or a record edited;
  - the result swapped, or its status changed;
  - `task_kind` against `settings.task.kind`;
  - a stale transient;
  - restored-as-fresh;
  - oversized;
  - a manifest past its cap;
  - a 0.3 record as a task record, and the reverse;
  - a private payload's sentinel string found in any report context, rendered report or
    package (D-10);
  - a state referenced only by task runs, whose deletion must name its blocker.
- **Legacy.** Every 0.3 fixture and test unchanged byte for byte. Mechanics-solve
  manifests unchanged.
- **Store.**
  - The migration is idempotent, with v13 rows untouched.
  - A newer store is refused.
  - The triggers hold.
  - Every refusal is atomic.
  - Size limits are applied before allocation.
- **Fresh and restored.** As §6, including `reproduction` `not_checked` on restore.
- **Scrutiny.** Independent design and persistence review of r3 (gpt-6-astra, medium,
  2026-11-02): "revise before adoption". Its two blocking findings (D-10, and the
  partial-cluster wording) and six should-fix findings are applied in r4. The adopted
  governing text gets its own review; implementation is reviewed per slice.

## 11. Decisions (Agent 0's 2026-11-02 rulings recorded; the rest open)

| Id | Decision | State |
|---|---|---|
| D-1 | C or A | **C preferred** for distinct typed task results (Agent 0). Not adopted. A needs an owner policy decision |
| D-2 | Name and version | Open. `openpipestress.analysis_task_record` 1.0.0 is a placeholder |
| D-3 | Contract owner | **DEL-14-02 with DEL-00-06** (Agent 0) |
| D-4 | Status vocabulary | Open (§3.1); (b) leaning, for review |
| D-5 | Recording a refused hanger document | **Not in the first slice**; displayed truthfully (Agent 0's preference) |
| D-6 | Separate label table | Recommended (§7); the independent review agreed, on the basis of risk rather than the format rule |
| D-7 | Desktop materials | `[]` only, as a restriction of the desktop route (§4.1), for review |
| D-8 | Modal's `task_kind` row and document | Open (§5, M-6) |
| D-9 | Store migration number, older-build refusal, rollback and data access | Number allocated at implementation. The cost is owner-visible and decided before adoption |
| D-10 | Private payloads in the manifest's settings (hanger's library; modal's private materials, library and constitutive payload) | **Checked: failed as stated.** The existing report consumers emit every settings key (§2.1). An allow-listed report projection is **mandatory before any typed private payload is deployed**. The allow-list is bounded and exact, with parity across the TypeScript report input, the Rust user-release context and the renderer, and with sentinel private fields in the checks. This is not permission to edit C1 or the report consumers now |
| D-11 | **Owner policy.** Whether to permit an explicitly versioned family-extension model under a stable envelope (§2.2). It is defined by four rules: syntax acceptance separate from semantic admission; recognition refusal of unknown families; a registry that only adds, with existing bindings immutable; and record binding versions that never change. Under it, the accepted set widens when a family is added. The alternatives are C-closed with a successor envelope version, and A. The owner's rule in force is quoted in §2.2 | Open. Not adopted; C-layered is the preferred design only |

## 12. Commitment text proposed (old and new; not applied)

**DEL-00-06 `ArchitectureBasis.md`, "Sanctioned additive classes".**

Old:
> Sanctioned additive classes (extensions, not contradictions of REQ-06-02): the analysis-run record keeps one version, 0.3.0 (`schemas/analysis_run.v0.3.schema.json`), which carries each received diagnostic unchanged as a source annotation and declares no warning-class enum of its own; …

New (if D-1 = C is adopted):
> Sanctioned additive classes (extensions, not contradictions of REQ-06-02): the analysis-run record keeps one version, 0.3.0 (`schemas/analysis_run.v0.3.schema.json`), for mechanics-solve runs, which carries each received diagnostic unchanged as a source annotation and declares no warning-class enum of its own. Typed task runs whose result is a family document (hanger sizing; modal when adopted) use the analysis task record, one envelope version (`<adopted schema path>`), with each family admitted through its own versioned family binding. It never records a mechanics solve, binds its family document and input manifest by hash, carries the family's diagnostics unchanged as source annotations, and declares no warning-class enum of its own. …

DEL-00-06 `code_paths` gain the adopted schema path.

**DEL-14-02 `ScopeOfWork.md`, CLM-004, "Architecture basis".**

Old:
> … DEL-14-02 keeps one version, analysis record 0.3.0, on the checked JCS/I-JSON profile: the owner decided on 2026-10-10 that each format keeps only its current version ("as you recommended, no-compatibility principle carries"), so the 0.1 sorted-compact and 0.2 records are refused by name.

New:
> … DEL-14-02 keeps one version of each record format on the checked JCS/I-JSON profile: analysis record 0.3.0 for mechanics-solve runs, and the analysis task record, one version, for typed task runs whose result is a family document; a task record never records a mechanics solve. The owner decided on 2026-10-10 that each format keeps only its current version ("as you recommended, no-compatibility principle carries"), so the 0.1 sorted-compact and 0.2 records are refused by name.

**DEL-04-11.**
- In the DEL-00-06 need in `deliverable.yaml`, "the joint analysis-run 0.4.0" becomes
  "the joint task-run record (`Work/joint-analysis-run-proposal.md`; DEL-14-02 on
  adoption)".
- The same wording change applies at `ScopeOfWork.md` "Analysis run" and Design §8.1
  and §8.3. The full old and new passages will be written out when D-1, D-2 and D-11
  fix the names.

The proposal applies none of these. They wait for D-1, D-2, D-9, D-10 and D-11, and for the review of the adopted text.
