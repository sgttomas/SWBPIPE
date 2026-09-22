# CFB-09 — Material and library persistence round trip; component fixture coverage

**Candidate brief (H2). Not executed.** Area: Material, library and component records (DEL-03-01, DEL-03-02, DEL-03-05). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add or locate a material serialization round-trip check and evidence deterministic persistence compatibility for library documents; add schema-shape or invented fixture records for reducer, flange and a component-library valve, or restate the artifact as one shared rigid-family fixture.

## Affected claims

4 claim rows on 3 deliverable(s): DEL-03-01, DEL-03-02, DEL-03-05.

Classes (portion in this brief / class total): T6-C01 1/180 (Authority NONE); T6-C02 3/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-09"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-03-01:SOW#CLM-011/REQ-03-01-007` | T6-C02 | NONE | C6 | — | Add or locate a material serialization round-trip check, or record a ruling that defers it. |
| `DEL-03-01:SOW#production-and-verification-method-praxeology/VER-001` | T6-C02 | NONE | C6 | — | Evidence deterministic persistence compatibility for material data (round-trip), or record its deferral. |
| `DEL-03-02:SOW#production-and-verification-method-praxeology/VER-001` | T6-C02 | NONE | — | — | Evidence deterministic persistence compatibility for library documents, or record where it is governed. |
| `DEL-03-05:CONTEXT#anticipated-artifacts` | T6-C01 | NONE | — | — | Add schema-shape or invented fixture records for reducer and flange (and a component-library valve record), or restate the artifact as one shared rigid-family fixture. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-03-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-03/DEL-03-01/DEL-03-01_forward.csv`; ImplementationEvidence cited: `schemas/material.schema.yaml`, `fixtures/material/invented_material_library_valid.json`, `projects/chirality-piping/apps/desktop/src-tauri/src/model_document_migration.rs`.
- DEL-03-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-03/DEL-03-02/DEL-03-02_forward.csv`; ImplementationEvidence cited: `schemas/section.schema.yaml`, `schemas/component.schema.yaml`, `fixtures/component/invented_section_library_valid.json`, `fixtures/component/invented_component_library_valid.json`.
- DEL-03-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-03/DEL-03-05/DEL-03-05_forward.csv`; ImplementationEvidence cited: `schemas/component.schema.yaml`, `fixtures/component/invented_component_library_valid.json`, `fixtures/product_preview/invented_preview_model.json`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A deterministic round-trip test exists for material and library documents.
- Reducer, flange and valve invented fixtures validate against their schemas, or the SOW is restated.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

2 of 4 claim rows carry a block: C6 (2). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- CONTESTED (T6-C02): the two DEL-03-01 rows; the Notes may overstate the absence of round-trip evidence, and DEL-03-05 CLM-010.r08 judges the same evidence the other way. First step is to re-check whether existing evidence already meets the claim.
- Both DEL-03-01 rows offer "or record a ruling that defers it" with no packet among A2, B3, B7 or B8. This is U7, placed with C6 (M2); both rows carry C6.

## Dependencies

C6 (U7).

