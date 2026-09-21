# DEL-02-05 notes — Project persistence and round-trip serialization

Worker G2, gate wave W2, PKG-02. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger sealed at SHA-256 `28b9f6f6d12f32ca0927786f1aa186b16c674822e73395657190662eb9029eb1`
(183 rows, 98 required keys, 7 canonical; validator with `--notes-gap`: PASS, 0 findings).

## Path aliases

- The deliverable folder path contains spaces and commas, so deliverable-local records are cited only
  in `ContextRefs` (full path) and named in Notes (same convention as DEL-02-04).
- `SVC` = `core/project_persistence/service.py` (Python contract service); `PSCH` =
  `schemas/project_persistence.schema.yaml`; `LIB` = `apps/desktop/src-tauri/src/lib.rs` (desktop
  SQLite project store: `create_local_project`, `open_local_project`, `save_local_project`,
  `open_project_store`); `MIGR` = `apps/desktop/src-tauri/src/model_document_migration.rs`; `PSVC` =
  `apps/desktop/src/services/projectService.ts`; `HASH` = `apps/desktop/src/services/hashService.ts`.
- Two persistence paths exist. The **product path** is the desktop Rust/SQLite store with DEC-019
  migration and wasm JCS hashing. The **Python contract path** is `SVC` plus `PSCH`, which has no
  product caller (confirmed: no non-test import; the desktop does not validate against `PSCH`).

## Judgment calls

1. **JCS divergence (FG-DEL-02-05-03, PROJECT_BASELINE).** AB-00-04 requires canonical JSON with
   JCS-compatible canonicalization for JSON payload hashes. The desktop hashes through the Rust
   `canonical_json` engine labelled `rfc8785_jcs`. The Python service uses sorted compact JSON and says
   it is "not RFC 8785/JCS". The schema's canonicalization enum is `SORTED_COMPACT_JSON`, the result of
   the DEC-074 E1 label-fidelity repair, which relabelled rather than changed the serializer.
   `persistence_contract.md` line 72 still says the schema records `hash.canonicalization = "JCS"`,
   which contradicts the schema. Disposed PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE ·
   BASELINE;RECORD. No ruling permitting the divergence was found.
2. **Rulings overtaking setup TBDs (FG-DEL-02-05-02).** DEC-017 (SQLite store profile), DEC-028
   (multi-member archive package; `.opsproj` per DEC-057) and DEC-019/DEC-033 (two-track versioning,
   migrate-in-memory-on-open, status set) decided what the SOW still calls TBD "under SCA-001" (container,
   migration framework, newer/current labels). Disposed STALE_SETUP_SPECIFICATION ·
   SCOPE_REDIRECTED_BY_RULING. AC-001 is STALE_REVIEW_OR_EVIDENCE because its text first appears at the
   SOW migration `bc3b65aa9` (F3).
3. **Landed schema vs layout/field/harness TBDs (FG-DEL-02-05-01).** DOC_BEHIND_CODE, the same treatment
   as DEL-02-04 FG-01. CLM-020's "(PROPOSAL)" class labels are kept ALIGNED: the draft schema implements
   the proposed names, but no ruling approved them, so "PROPOSAL" is still accurate. This differs from
   "TBD", which the landed fields overtook.
4. **Explicit migrate operation (FG-DEL-02-05-05).** REQ-02-05-022 is about the contract defining the
   operation, which the schema's `PersistenceOperation` does, so it is ALIGNED. CLM-019.r06 (migrate
   returns a migrated project) is PARTIALLY_IMPLEMENTED: the product has migrate-on-open only, and the
   Python `migrate_project_store` only classifies. STATUS R01 and R02 are PARTIALLY_IMPLEMENTED
   themselves (F2 second branch), because their container and compatibility-window work has no
   non-aligned governing SOW row: the SOW predates DEC-028. A `.opsproj` multi-member writer landed for
   report packages on 2026-07-23 (`atomic_report_package_save.rs`), after these items were written, but
   no project-package open or round trip exists.
5. **Rule-pack reference coverage (FG-DEL-02-05-06).** `validate_project_persistence_envelope` never
   inspects `rule_pack_refs`, although `validation_profile.rule_pack_reference_check` is `true`. The
   fixture's `rule_pack_refs` is empty. REQ-02-05-024, CLM-022.r05 and CLM-030.r08 are
   PARTIALLY_IMPLEMENTED.
6. **Fixture data-boundary review record (FG-DEL-02-05-04).** No `public_fixture_data_boundary_review`
   (or equivalent) record was found. `git grep` finds the name only in this deliverable and in an
   excluded prior-run file, whose content was not read. Disposed UNKNOWN · EVIDENCE_NOT_LOCATED ·
   INVARIANT · IP_DATA;RECORD · REVIEW, matching the W1 resolution for DEL-07-02 CLM-026.
7. **GUI exclusion (CLM-006, CLM-030.r01).** Several desktop panels list DEL-02-05 among their
   `deliverable_refs` and display persistence packets, and MEMORY describes DEL-02-05 tranches that edited
   those panels. I read the panels as shared GUI work consuming persistence data, so the
   no-GUI-views condition is ALIGNED at MEDIUM confidence. A verifier may prefer a sub-claim
   (IMPLEMENTED_DIFFERENTLY · SCOPE_GREW_BY_DIRECTION · PROJECT_BASELINE, since the PKG-02 exclusion is a
   decomposition baseline).
8. **STATUS surface.** CP-05 on the SURFACE row (Last Updated 2026-07-12 < history 2026-07-16). The
   `.opsproj` rename residue is on `STATUS.s01`, using CP-04's special fields for `.opsproj`: BaselineClass
   NONE, tier PROJECT_BASELINE, layer RECORD, OWNER, persistence-compatibility obligation noted.
9. **R03 (H2).** DOCUMENTED_UNIMPLEMENTED · NOT_STARTED: `evaluateModelDocumentLocal` remains and
   migration evaluation is not in a wasm crate. No SOW row carries the work (CLM-034 endorses the current
   mirror), so the Remaining row takes the gap (F2).

## Canonical departures

None on CS-keyed rows. CS-04 split into `.s01` (PKG-00 state) and `.s02` (still-TBD list: migration
framework and DB migration details decided by DEC-019; formats via SCA-004).

## Convention friction

- Two implementations of one deliverable (Python contract service vs desktop product store) make F7
  hard to apply row by row. See the reverse-pass section.
- CP-04 and CP-05 both want the STATUS SURFACE row; resolved with a `.s01` sub-claim.
- Code identifiers carrying the former name that the SOW does not name: `openpipestress-projects.sqlite3`
  (store file, CP-04 special case), `document_kind` `openpipestress.project_persistence`, and the schema
  `$id` and title. Listed here for R3. `.opsproj` is keyed via STATUS.s01 because the Remaining items
  name it.

## UNKNOWN rows and smallest checks

| Row | Smallest next check |
|---|---|
| SOW#CLM-029 | `git grep -il "fixture.*review\|data_boundary_review"` over `execution/_Coordination/AgentRuns/` and DEL-02-05 `_run_records/` for a review of `fixtures/persistence/invented_persisted_preview_project.json`; if none, run and record that review |
| SOW#CLM-030.r11 | Same check |
| SOW#CLM-031 | Same check |

## Reverse pass: effect on sealed rows (not edited)

- **F7 marker gap and likely re-dispositions (errors I would correct).** The routing note on
  RC-02-0118 and my own check confirm that the Python service has no product caller. 46 ALIGNED rows whose
  implementation evidence is only `SVC` and/or `PSCH` lack `PRODUCT_CALLER: NONE` (the full list is
  produced by filtering ALIGNED rows whose ImplementationEvidence contains no `apps/desktop` path).
  - Most are schema or contract definitions, which the marker alone would fix.
  - Four make runtime claims that the product path does not meet as stated, so under F7 I would
    re-dispose them to PARTIALLY_IMPLEMENTED:
    - REQ-02-05-011 (open/save failures as structured diagnostics): the desktop store commands return
      plain `String` errors; only migration refusals are structured.
    - REQ-02-05-008 (unsupplied provenance as a finding at persistence).
    - CLM-019.r04 (validate-project diagnostics).
    - Possibly REQ-02-05-018 (the round-trip acceptance comparison is Python-only; the desktop has
      unit-signature and e2e evidence).
- The routing file has **no capability row** for the desktop local project store commands
  (`create_local_project`, `open_local_project`, `save_local_project`, `open_project_store`) or for
  `atomic_report_package_save.rs`. Both are presumably routed elsewhere. R3 should confirm the store
  commands are attributed to DEL-02-05.

## Batch consistency

`--batch` over the DEL-02-04 and DEL-02-05 forward ledgers: FAIL, 1 finding, on DEL-02-05:STATUS.s01
(CP-04 group; tier PROJECT_BASELINE vs majority LOCAL_DESIGN). The difference is justified: CP-04
prescribes PROJECT_BASELINE and BaselineClass NONE for `.opsproj`, while the two SOW SURFACE rows are
ordinary prose residue using CP-04's defaults. It is not an error, but the row lacks the
`CANONICAL_DEPARTURE:` marker the batch check looks for.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work
through owner-steered work graphs, not `## Remaining`.

## Claim fence

These are agent dispositions, not owner rulings, and they state no release, approval, compliance or
certification. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
