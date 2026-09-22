# CFB-07 — Route Python persistence and model-state hashes through the project JCS profile

**Candidate brief (H2). Not executed.** Area: Canonical JSON and hashing (DEL-02-05, DEL-14-01). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Route the Python project-persistence service, schema checksum labels and model-state record hashes through the project canonical-JSON profile (`core/serialization/canonical_json`) instead of the sorted-compact serializer (freeze `projects/chirality-piping/core/project_persistence/service.py:28` labels `SORTED_COMPACT_JSON`). The alternative every row names, a ruling that narrows the DEC-010 JCS-compatible basis, is an owner choice held in A2.

## Affected claims

12 claim rows on 2 deliverable(s): DEL-02-05, DEL-14-01.

Classes (portion in this brief / class total): T6-C02 12/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-07"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-02-05:CONTEXT#context-envelope` | T6-C02 | NONE | A2 | — | Route the Python persistence service and schema checksum labels through the project JCS profile (core/serialization/canonical_json), or record a ruling that permits sort… |
| `DEL-02-05:CONTEXT#sca-003-storage-profile-injection` | T6-C02 | NONE | A2 | — | Route the Python persistence service and schema checksum labels through the project JCS profile (core/serialization/canonical_json), or record a ruling that permits sort… |
| `DEL-02-05:SOW#CLM-005.r05` | T6-C02 | NONE | A2 | — | Route the Python persistence service and schema checksum labels through the project JCS profile (core/serialization/canonical_json), or record a ruling that permits sort… |
| `DEL-02-05:SOW#CLM-014/REQ-02-05-005` | T6-C02 | NONE | A2 | — | Route the Python persistence service and schema checksum labels through the project JCS profile (core/serialization/canonical_json), or record a ruling that permits sort… |
| `DEL-02-05:SOW#CLM-021.r02` | T6-C02 | NONE | A2 | — | Route the Python persistence service and schema checksum labels through the project JCS profile (core/serialization/canonical_json), or record a ruling that permits sort… |
| `DEL-02-05:SOW#CLM-030.r05` | T6-C02 | NONE | A2 | — | Route the Python persistence service and schema checksum labels through the project JCS profile (core/serialization/canonical_json), or record a ruling that permits sort… |
| `DEL-14-01:CONTEXT#architecture-basis-injection.s03` | T6-C02 | NONE | A2 | — | Implement RFC 8785/JCS canonicalization for model-state record hashes, or obtain a ruling that narrows the DEC-010 JCS-compatible basis to the sorted-compact label, then… |
| `DEL-14-01:SOW#CLM-004` | T6-C02 | NONE | A2;B8 | T12 T12-C02: OWNER_DECISION | Implement RFC 8785/JCS canonicalization for model-state record hashes, or obtain a ruling that narrows the DEC-010 JCS-compatible basis to the sorted-compact label, then… |
| `DEL-14-01:SOW#CLM-005` | T6-C02 | NONE | A2;A1;B8 | T8 DEC-009: OWNER_DECISION; T12 T12-C02: OWNER_DECISION; DISAGREES (T8 OWNER_DECISION / class CODE_FIX_CANDIDATE) | Implement RFC 8785/JCS canonicalization for model-state record hashes, or obtain a ruling that narrows the DEC-010 JCS-compatible basis to the sorted-compact label, then… |
| `DEL-14-01:SOW#CLM-011.r04` | T6-C02 | NONE | A2;B8 | T12 T12-C02: OWNER_DECISION | Implement RFC 8785/JCS canonicalization for model-state record hashes, or obtain a ruling that narrows the DEC-010 JCS-compatible basis to the sorted-compact label, then… |
| `DEL-14-01:SOW#CLM-012` | T6-C02 | NONE | A2 | — | Implement RFC 8785/JCS canonicalization for model-state record hashes, or obtain a ruling that narrows the DEC-010 JCS-compatible basis to the sorted-compact label, then… |
| `DEL-14-01:SOW#CLM-019` | T6-C02 | NONE | A2 | — | Implement RFC 8785/JCS canonicalization for model-state record hashes, or obtain a ruling that narrows the DEC-010 JCS-compatible basis to the sorted-compact label, then… |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-02-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-02/DEL-02-05/DEL-02-05_forward.csv`; ImplementationEvidence cited: `core/project_persistence/service.py`, `apps/desktop/src-tauri/src/lib.rs`, `apps/desktop/src/services/hashService.ts`, `schemas/project_persistence.schema.yaml`, `core/serialization/canonical_json/src/lib.rs`.
- DEL-14-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-01/DEL-14-01_forward.csv`; ImplementationEvidence cited: `schemas/model_state.schema.json`, `core/project_persistence/service.py#L28`, `core/project_persistence/service.py#L240`, `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-01_Immutable model state records/Dependencies.csv`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Hash outputs for a shared invented fixture match between the Python services and the canonical_json profile.
- No persistence or model-state label claims JCS compatibility unless the JCS profile produced the bytes.
- Existing stored hashes are migrated or versioned, per the A2 ruling.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

12 of 12 claim rows carry a block: A1 (1); A2 (12); B8 (3). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). All rows are blocked.

## Notes and open views

- Every row waits on A2. DEL-14-01:SOW#CLM-005 also carries the T8 DEC-009 reading (OWNER_DECISION; A1).
- DEL-14-01 CLM-004, CLM-005 and CLM-011.r04 also carry B8: their T12-C02 reading is OWNER_DECISION, and B8 §5 asks H2 to mark them.
- CONTESTED: the four DEL-02-05 rows split FROZEN_CONTRACT vs NONE BaselineClass with DEL-02-02 FG-DEL-02-02-01 (T6-C02 notes).

## Dependencies

A2, A1, B8.

