# D-09 — Native Package Physical Container Format (Single-File Project Container, Public Transport Form)

**Status:** RULED — 2026-06-11 the human project authority selected **Option C** (multi-member archive package per the PKG-17 export-package manifest contracts; canonical-JSON members carry the truth; evidence binds to members and manifest per-member JCS hashes, not container bytes; archive mechanics are implementation work; naming lands with D-06). Recorded as `DEC-028` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row updated.
**Prepared:** 2026-06-11 by TASK (Type 2), requested by WORKING_ITEMS (Type 1), tranche TP-DECIDE-PREP-002 decision-preparation subscope.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-09.
**Plan basis:** `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-09.
**Epistemic posture:** evidence below is `FACT` with citations pinned at repo HEAD `4b2c0619e`; inferences and general container knowledge are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** what "a project as a file" physically is for OpenPipeStress — the single-file project container a user can copy, archive, and hand to another machine, and the public transport form named in the register row — including canonical-truth location, hash binding across save/open round-trips, versioning semantics inside the container, atomic-save/corruption posture, large-file reference handling, and compatibility-on-open behavior.

**In scope:** the per-project physical container and its relationship to the existing app-wide local SQLite store; the public transport form; invented file extension naming; how DEC-017/DEC-019 versioning and hash discipline bind across the container boundary.

**Out of scope:** the PKG-17 target-format exporters themselves (CAEPIPE MBF, PCF, glTF — contracted in `execution/PKG-17_Export Format Interoperability/`), report rendering (ruled, `DEC-021`), installers/signing/file-type registration mechanics (D-06), redaction/export-review tooling (Phase E item E3), and any cloud/sync/network path (prohibited by `DEC-017`).

D-09 blocks the FR-001 residual "file-container semantics" (`plans/PLAN_2026-06-10_prd_completion.md:144`) and Phase E distribution of projects (`_REGISTER.md:27`; plan §2 row D-09, `plans/PLAN_2026-06-10_prd_completion.md:48`).

---

## 2. Current state evidence

Citations pinned at HEAD `4b2c0619e`. Symbol names are the durable anchors if line numbers drift.

### 2.1 Ruled foundations this decision builds on

- **FACT — `DEC-017`** (`execution/_Decomposition/SOFTWARE_DECOMP.md:587`): MVP storage profile — SQLite is the *local project store/index substrate*; canonical JSON/JCS remains *domain truth*; FTS5/BM25 sidecars are rebuildable; large files are *referenced in place by path/URI plus hash/metadata* by default; no hosted DB, daemon, required network, cloud sync, or direct plugin/adapter SQL access.
- **FACT — `DEC-019`** (`SOFTWARE_DECOMP.md:589`): two-track versioning — store schema via the SQLite `user_version` ledger; model documents via per-document semver with an explicit transform chain; migrate-in-memory-on-open with persist-on-save; no silent destructive rewrites.
- **FACT — `DEC-010`** (`SOFTWARE_DECOMP.md:580`): JSON Schema 2020-12 + canonical JSON/JCS hashing as the schema/persistence baseline; its notes state "public transport protocol, import/export formats … remain TBD" — D-09 is part of closing that TBD.
- **FACT — `DEC-012`** (`SOFTWARE_DECOMP.md:582`): kept "physical project package/container" an implementation-level TBD; its notes say that portion was "superseded by DEC-017" — but DEC-017 ruled only the *local store substrate*, not the portable single-file container or transport form, which the register still tracks as open (`_REGISTER.md:27`).
- **FACT — `DEC-021`** (`SOFTWARE_DECOMP.md:591`): the calculation report is a deterministic single-file HTML artifact; the D-10 packet's impact map recorded that a single-file report "is trivially storable in whatever container D-09 selects" (`D-10_report_rendering_target.md`, §6 row "D-09 container").

### 2.2 What project persistence physically is today

- **FACT:** the local store is **one app-wide SQLite database file holding all projects as rows** — `PROJECT_STORE_FILE = "openpipestress-projects.sqlite3"` (`apps/desktop/src-tauri/src/lib.rs:21`) located in the OS app-local-data dir (`app_store_path`, `lib.rs:144–150`). There is **no per-project file on disk today**; projects are not user-addressable as files.
- **FACT:** schema is `local_projects` (project_id PK; `model_json`, `editor_intents_json`, `proposal_json`, `mechanics_result_json`, `analysis_run_json`, `model_hash_json`, `project_envelope_hash_json`, `model_migration_ledger_json` TEXT columns) plus an FTS5 sidecar table (`migrate_v1_base_tables`, `lib.rs:277–289`; `upsert_project` column list, `lib.rs:458–471`). JSON is stored as serialized text inside SQLite — the DEC-017 "canonical JSON inside a store" posture, implemented.
- **FACT:** store schema is at `user_version` ledger target **v9** (`STORE_SCHEMA_TARGET_VERSION`, `lib.rs:153`), migrate-on-open (`apply_store_migrations`, `lib.rs:308+`), WAL journal mode (`lib.rs:311–316`). Per-document semver evaluation lands separately (`model_document_migration.rs:5`; browser mirror with refuse-newer posture in `apps/desktop/src/services/projectService.ts:42–76`, `SUPPORTED_MODEL_SCHEMA_VERSION = "0.1.0"` at `:33`).
- **FACT:** the app exposes Tauri commands `create_local_project` (`lib.rs:1355`), `open_local_project` (`lib.rs:1428`), `list_local_projects` (`lib.rs:1481`), `save_local_project` (`lib.rs:1488`), consumed by `apps/desktop/src/services/projectService.ts`. Open/save operate on store rows by `project_id`; there is no file dialog, no Save As, no import/export of a project file.
- **FACT:** plan §3 row A2 is **LANDED 2026-06-10** ("store v9 ledger + in-document semver", `plans/PLAN_2026-06-10_prd_completion.md:66`); the FR-001 status row reads "Largely met … residuals: compatibility-window ruling, explicit migrate operation, **file-container semantics**" pointing at D-09 (`:144`).
- **FACT:** a Python reference implementation of the persistence envelope + SQLite profile exists at `core/project_persistence/service.py:1–7` ("SQLite is used as a local payload container and projection substrate; canonical JSON/JCS-compatible payload bytes remain the domain truth").

### 2.3 PRD requirements

- **FACT — FR-001 (Must):** "Create, open, save, and version projects — **Project file** round-trips without loss of model, units, loads, rule-pack references, or provenance metadata" (`docs/PRD.md:334`). The acceptance wording presumes a project *file*.
- **FACT:** MVP scope lists "**YAML/JSON project files**" (`docs/PRD.md:305`). FR-023 (Could): import/export "using open formats" (`:356`). Reproducibility is stated against "a given project file and solver version" (`:557`) and reports must carry metadata to reproduce "from the same project file…" (`:882`, §15.3).
- **FACT — R2 exit criterion** (§22.3, `docs/PRD.md:1191+`): "create, solve, and report a small piping model **without editing raw files**." It demands no file container; the store-backed flow already satisfies the persistence leg. ASSUMPTION: file-level open/save is a usability expectation for a GUI MVP, but nothing in §22.3 forces it — R2 need from this ruling is timing clarity only (§3 Q8).

### 2.4 SPEC and schema state

- **FACT — SPEC §4.4** (`docs/SPEC.md:321–368`): the persistence envelope is `schemas/project_persistence.schema.yaml`; SQLite tables/sidecars are "storage/projection details, not public contracts or domain truth" (`:333–335`); JCS hashes must identify payload scope (project envelope, model payload, external artifact…) (`:345–349`); and "portable export/copy workflows … remain separate `TBD` decisions" (`:363–368`; reiterated `:405`). D-09 is exactly that named TBD.
- **FACT:** `schemas/project_persistence.schema.yaml` already defines a `PhysicalContainer` block — `profile` enum `sqlite_local_project_store`, `storage_role: local_store_index_projection` (const), `canonical_truth: canonical_json_jcs_payload` (const), `sql_public_contract: false`, `decision_ref: SCA-003` (`schemas/project_persistence.schema.yaml:341–402`) — plus `ExternalArtifactReference` with required `uri_or_path`, `hash`, `classification`, `verification_status`, `copy_policy` (`:403+`). A ruled container extends this enum; the canonical-truth const is already pinned to JSON/JCS.
- **FACT:** round-trip acceptance is semantic equality with documented volatile-field exclusions and no silent engineering defaults (`docs/SPEC.md:351–356`).

### 2.5 PKG-17 export design authority (CHECKING deliverables — mature design context)

- **FACT:** DEL-17-02 (state CHECKING, `_STATUS.md:3`) contracts the *logical* package shape: every export package lists all members (REQ-011, `Specification.md:32`); manifests record source model ref, package/profile IDs, member inventory, source basis (REQ-040–043, `:66–69`); member hashing uses the project JCS basis (REQ-007, `:24`); sidecar ID maps pair with targets that cannot carry canonical IDs (REQ-033, `:56`). **The physical archive form (zip vs directory vs single file) is not pinned anywhere in the contract** — grep over DEL-17-02/03 Specification/Guidance finds no container/zip/archive requirement.
- **FACT:** DEL-17-03 (state CHECKING) delivered the native open JSON export package: schema `schemas/native_json_export.schema.json` (single strict JSON object carrying model payload, stable ID map, loss report, manifest, provenance, privacy, professional boundary — `:1–10`), writer `core/handoff/native_json/package.py`. Its TBD register closes the schema/hash/fixture questions but records: "**API/CLI/GUI and project-store binding remain future scoped work**" (TBD-17-03-004, `Guidance.md:26`).
- **FACT — IP boundary:** user-created models/private libraries live in user-controlled local paths, never the public repository; exports default to excluding private data unless the user intentionally exports it (`docs/IP_AND_DATA_BOUNDARY.md:85–87, 91`).

---

## 3. Open questions awaiting ruling

1. **Container identity:** is the project file the *primary working artifact* (app opens/saves files directly) or an *import/export form* over the app-wide SQLite store (store stays primary, files travel)? Today only the store exists (§2.2).
2. **Canonical-truth carrier:** DEC-017 fixes canonical JSON/JCS as domain truth — does the container carry the truth bytes themselves, or a projection that the store re-derives? (A SQLite container carries truth *inside* a binary projection; a JSON container *is* the truth bytes.)
3. **Hash binding across the boundary:** which hash travels and is re-verified on open — the existing `project_envelope_hash` / `model_hash` records (§2.2), a whole-file hash, or both? ASSUMPTION: SQLite file bytes are not canonically reproducible for identical logical content (page layout, freelist, WAL state vary), so a whole-file hash over a `.db` cannot serve as a JCS-style evidence hash; a canonical-JSON file can be hashed byte-for-byte.
4. **Versioning inside the container:** DEC-019's two tracks — per-document semver + transform-chain ledger clearly travel with the project; does the store `user_version` travel too? ASSUMPTION: it is a property of the local store, not the project, and embedding it in a transport form entangles store DDL with interchange compatibility (this is the strongest structural argument against shipping the store file itself).
5. **Atomic save and corruption posture:** SQLite provides transactional atomicity natively (§2.2 WAL); a JSON/zip file needs an explicit write-temp → flush → atomic-rename discipline (ASSUMPTION: standard technique, not yet specified anywhere in-repo — `TBD`).
6. **Large-file references:** DEC-017's reference-in-place rule keeps containers small but breaks portability when the referenced path doesn't exist on the receiving machine. The schema's `copy_policy` + `verification_status` fields (§2.4) anticipate this; the ruling should fix the default (reference-only with surfaced missing-artifact diagnostics vs materialize-on-export option). `TBD`.
7. **Compatibility window on open:** refuse-newer is implemented for model documents (§2.2); the breadth of backward window remains the A2 residual human ruling (`plans/PLAN_2026-06-10_prd_completion.md:144`) — this packet does not resolve it, but the container must record enough (semver + ledger) for whatever window is later ruled.
8. **R2 timing:** does the human want file open/save in the R2 GUI MVP, or is the store-backed flow sufficient until Phase E? §2.3 shows R2's exit criterion does not require a file. `TBD` — human preference.
9. **One form or two:** is the single-file container *also* the public transport form, or are they distinct (working container + PKG-17 multi-member export package)? The register row names both in one breath (`_REGISTER.md:27`).
10. **Naming:** invented extension (e.g. `.opsproj`) and document kind string — feeds D-06 file-type registration. `TBD` until ruled.

---

## 4. Options

All options preserve DEC-017 prohibitions: local-only, no hosted DB, no network, no cloud sync. Extensions shown are invented examples.

### Option A — The SQLite database file IS the single-file project container

Move from the app-wide store to one SQLite file per project (e.g. `bridge-loop.opsproj`, a SQLite db with the existing schema scoped to one project). Transport = copy the file. The app-wide index either goes away or becomes a recent-files list.

- For: single file by construction; atomic saves and crash recovery come free from SQLite's transaction machinery (§2.2 WAL); FTS sidecar travels with the project; minimal new serialization code — the row schema already exists.
- Against: contradicts the ruled posture that SQLite is a *projection, not domain truth or public contract* (DEC-017; SPEC `:333–335`; schema consts `storage_role: local_store_index_projection`, `sql_public_contract: false`, §2.4) — making the `.db` the public transport form elevates table layout into an interchange contract and invites the prohibited direct-SQL consumption path; the store `user_version` ledger would travel with every file, entangling store DDL migrations with interchange compatibility (§3 Q4); file bytes are not canonically hashable (§3 Q3 ASSUMPTION), so round-trip evidence still has to be JSON-level anyway; significant rework of the landed A2 store (one db → many) for no evidence gain.

### Option B — Single-file canonical-JSON project document as both container and public transport form

The project file is **one UTF-8 canonical-JSON document** conforming to (an extended) `schemas/project_persistence.schema.yaml` envelope (§2.4): model payload + per-document semver + transform-chain ledger + hash records + external-artifact references + provenance. The app-wide SQLite store remains exactly what DEC-017 says it is — a local index/working store; import = validate, hash-verify, upsert a row; export = emit the envelope from the row. Large files stay referenced by path/URI + hash with `copy_policy`/`verification_status` surfaced on open.

- For: the container *is* the canonical truth bytes — JCS hash binds the whole file byte-for-byte across save/open round-trips with the discipline already in place (`project_envelope_hash`, §2.2); zero change to the landed A2 store; the persistence-envelope schema already exists and already pins `canonical_truth: canonical_json_jcs_payload`; satisfies PRD "YAML/JSON project files" (`docs/PRD.md:305`) and FR-001's "project file round-trips" literally; diffable, schema-validatable, text — the protected-content/private-data review tooling of Phase E operates on it directly; refuse-newer/semver logic already implemented applies unchanged.
- Against: needs an explicit atomic-write discipline (§3 Q5, `TBD`); cannot *embed* large binary artifacts (reference-only — a project depending on local artifacts is not fully self-contained on copy, §3 Q6); results/run envelopes inside one document can make files large (ASSUMPTION: acceptable at MVP model scale; no in-repo measurement — `TBD`).

### Option C — Multi-member package (zip or directory) per the PKG-17 contracts as the container

The project file is an archive whose members are canonical-JSON documents plus optional materialized artifacts and the DEC-021 HTML report, with a DEL-17-02-style manifest listing every member with JCS hashes (REQ-011/040–043, §2.5). Single-file transport = the zip; the store remains the working index as in Option B.

- For: handles everything Option B cannot embed — materialized large artifacts, attached reports, sidecar ID maps — inside one portable file; reuses the contracted PKG-17 manifest/loss-report shape, so the project container and export packages share one logical grammar.
- Against: ASSUMPTION: zip is not byte-canonical without extra discipline (member order, timestamps, compression settings), so whole-file hashing needs a normalization rule the repo does not yet have (`TBD`); heavier than the MVP needs — today's project state is a handful of JSON values (§2.2); two layers of hashing (member + archive) to specify; PKG-17's physical archive form is itself unpinned (§2.5), so this option must rule that too.

### Option D — Defer the portable container to Phase E; rule only the local store form now

Affirm the status quo (app-wide SQLite store, no project file) as the ruled local form; postpone the single-file container and transport form to a D-09b at the Phase E lead-up.

- For: zero Phase A/B cost; Phase E (E3 export controls, D-06 packaging) is when distribution genuinely activates.
- Against: leaves FR-001's "project file round-trips" acceptance and the §2.3 PRD wording unsatisfiable until Phase E; the FR-001 residual row stays open against a Must requirement; users have no way to back up, share, or archive a single project (only the whole opaque store file); defers exactly the question the register said was mid-plan (`plans/PLAN_2026-06-10_prd_completion.md:48`).

---

## 5. Recommendation — `PROPOSAL`

Adopt **Option B now, with Option C's archive form named as the Phase E follow-up decision point (suggested D-09b) for materialized-artifact transport**:

1. **Container = canonical-JSON project document.** One UTF-8 file (invented extension example `.opsproj`; final name `TBD` at ruling) conforming to the extended `project_persistence` envelope; it carries the domain truth bytes per DEC-017, the per-document semver + transform-chain ledger per DEC-019, and the existing `model_hash`/`project_envelope_hash` records. The same file is the **public transport form** — what FR-023's "open formats" and the register row's transport language receive.
2. **Store stays a projection.** The app-wide SQLite store (§2.2) is unchanged: import validates against schema, verifies JCS hashes, evaluates semver (refuse-newer posture already implemented), and upserts a row; export re-emits the envelope. The store `user_version` never travels (§3 Q4). FTS sidecars are rebuilt on import per DEC-017.
3. **Hash binding:** the file's bytes are JCS-canonical, so the recorded envelope hash *is* the file evidence hash; open re-verifies and surfaces mismatch as a blocking diagnostic, never a silent repair (SPEC §4.4 round-trip rules, §2.4).
4. **Atomic save:** write-temp → flush → atomic-rename, with the previous file retained until rename completes (ASSUMPTION: standard technique; exact discipline is implementation detail below ruling level, recorded in the implementing tranche).
5. **Large files:** reference-in-place by path/URI + hash with `copy_policy` and open-time `verification_status` diagnostics (the schema fields already exist, §2.4). Materializing referenced artifacts into a portable archive is exactly the D-09b/Option C follow-up, scheduled with Phase E E3.
6. **R2 timing:** R2 requires nothing from this ruling (§2.3); implementing file open/save can be scheduled as a small Phase A/B tranche purely to close the FR-001 residual, or deferred — human's pacing call (§3 Q8).
7. **D-06 handoff:** the ruled extension and document-kind string feed D-06 file-type registration; nothing else couples.

Rationale: Option B is the only option where the container, the canonical truth, and the hash evidence are the *same bytes* — the entire DEC-010/DEC-017 discipline transfers to the file boundary with no new theory. Option A inverts the ruled store-is-projection posture and makes interchange depend on table layout; Option C buys generality the MVP does not need at the cost of an unpinned archive-canonicalization rule; Option D leaves a Must-level acceptance criterion unsatisfiable. The choice also closes DEL-17-03's recorded "project-store binding" future work (§2.5) along the path that deliverable already built: a single strict JSON package.

This recommendation is a `PROPOSAL` only. It confers no authority and changes no state.

---

## 6. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **FR-001 residual** | "File-container semantics" residual closes; FR-001 acceptance ("project file round-trips without loss", `docs/PRD.md:334`) becomes testable against the envelope file (`plans/PLAN_2026-06-10_prd_completion.md:144`). |
| **A2 residuals** | The compatibility-window ruling and explicit "Migrate project" operation (plan `:66`, `:144`) now have a defined artifact to apply to; this packet does not resolve them. |
| **`schemas/project_persistence.schema.yaml`** | `PhysicalContainer.profile` enum gains the ruled container profile alongside `sqlite_local_project_store`; `decision_ref` gains the D-09 ruling ID (`:341–402`). |
| **PKG-17 / DEL-17-03** | TBD-17-03-004's "project-store binding remain future scoped work" (`Guidance.md:26`) gets its binding direction; the native JSON export package and the project container stay distinct documents with shared JCS hash discipline (REQ-007, §2.5). |
| **Phase E (E3/E4) and D-06** | E3 export-control/redaction tooling targets a text JSON surface; D-06 installers register the ruled extension; D-09b (archive form for materialized artifacts) schedules with the Phase E lead-up (`_REGISTER.md` row D-06 NOT_PREPARED). |
| **DEC-021 report artifact** | The single-file HTML report remains a referenced/exported artifact, not embedded in the MVP container; D-09b decides packaged-together transport (`SOFTWARE_DECOMP.md:591`). |
| **IP/data boundary** | Project files are user-local artifacts under `docs/IP_AND_DATA_BOUNDARY.md:85–87`; export paths inherit the default-exclude-private-data rule (`:87`) — container work must not add a repo-committed project-file path. |
| **SPEC §4.4 TBD list** | "Portable export/copy workflows" leaves the TBD list (`docs/SPEC.md:363–368, 405`); storage roots, encryption, and key management remain TBD. |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-09. Agents prepared this packet and may not certify, approve, or adopt it.

Per existing decision practice, the accepted ruling is recorded as a `DEC` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (D-08/D-10 were recorded as `DEC-019`/`DEC-021` this way, `_REGISTER.md` rows D-08/D-10), after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-09 from `AWAITING_RULING` to `RULED` with a pointer. This packet does not edit the register and does not resolve the decision.
