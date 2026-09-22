# CFB-08 — Project package: populated rule-pack reference round trip, explicit migrate, compatibility window

**Candidate brief (H2). Not executed.** Area: Project persistence (DEL-02-05, DEL-08-01 R01). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add missing/private rule-pack reference diagnostics to envelope validation and a populated-reference round-trip fixture; implement the explicit migrate-project operation (DEC-019); implement the H2 unification (move DEC-019 migration evaluation into the wasm-compilable crate and replace `evaluateModelDocumentLocal`) or record its deferral; package compatibility-window reader behaviour once DEC-028 is ruled.

## Affected claims

8 claim rows on 2 deliverable(s): DEL-02-05, DEL-08-01.

Classes (portion in this brief / class total): T6-C01 6/180 (Authority NONE); T6-C02 2/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-08"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-02-05:SOW#CLM-014/REQ-02-05-024` | T6-C01 | NONE | — | — | Add missing/private rule-pack reference diagnostics to envelope validation and a populated-reference round-trip fixture. |
| `DEL-02-05:SOW#CLM-019.r04` | T6-C01 | NONE | — | — | (none recorded; see Notes in ledger) |
| `DEL-02-05:SOW#CLM-019.r06` | T6-C02 | NONE | — | — | Implement the explicit migrate-project operation (DEC-019 explicit-operation evidence) and restate the framework note. |
| `DEL-02-05:SOW#CLM-022.r05` | T6-C01 | NONE | — | — | Add a populated rule-pack reference round-trip fixture and assertions. |
| `DEL-02-05:SOW#CLM-030.r03` | T6-C01 | NONE | — | — | (none recorded; see Notes in ledger) |
| `DEL-02-05:SOW#CLM-030.r08` | T6-C01 | NONE | — | — | As DEL-02-05:SOW#CLM-022.r05. |
| `DEL-02-05:STATUS#remaining/R03` | T6-C01 | NONE | — | — | Implement the H2 unification or record its deferral. |
| `DEL-08-01:STATUS#remaining/R01` | T6-C02 | NONE | B12 | — | Define and implement the package compatibility-window/versioning policy (reader behaviour for other manifest schema versions). |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-02-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-02/DEL-02-05/DEL-02-05_forward.csv`; ImplementationEvidence cited: `schemas/project_persistence.schema.yaml`, `core/project_persistence/service.py`, `apps/desktop/src-tauri/src/model_document_migration.rs`, `apps/desktop/src-tauri/src/lib.rs`, `tests/test_persistence_schema.py` ….
- DEL-08-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-01/DEL-08-01_forward.csv`; ImplementationEvidence cited: `core/reporting/report_package/src/lib.rs`, `apps/desktop/src-tauri/src/atomic_report_package_save.rs`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A round-trip test with populated rule-pack references shows they survive save/open intact.
- A negative test shows a missing or private rule-pack reference yields a diagnostic.
- Cross-engine parity tests cover migrated-bytes hash integrity if H2 lands.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

1 of 8 claim rows carry a block: B12 (1). `H3[<class>]` names the H3 register item for that class (review before repair). Unblocked rows may proceed separately once selected.

## Notes and open views

- DEL-08-01:STATUS#remaining/R01 waits on the DEC-028 compatibility window (B12, T6-C04 D10).

## Dependencies

B12 (D10), CFB-17.

