# CFB-34 — Model-state product save/list/open and comparison through the application-service boundary

**Candidate brief (H2). Not executed.** Area: Model-state records and comparison (DEL-14-01, DEL-14-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

If B8 brings model-state persistence and comparison into the product: wire a product save/list/open path for named model-state records that emits schema-valid records; expose model-state comparison through the application-service boundary and add a product caller; validate the comparison test fixtures against `schemas/model_state.schema.json`.

## Affected claims

6 claim rows on 2 deliverable(s): DEL-14-01, DEL-14-03.

Classes (portion in this brief / class total): NOT_DIVERGENT 1/— (Authority NONE); T6-C01 1/180 (Authority NONE); T6-C02 4/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-34"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-14-01:SOW#CLM-011.r01` | T6-C02 | NONE | B8 | T12 T12-C02: OWNER_DECISION | Wire a product save/list/open path for named model-state records (desktop command or application service) that emits schema-valid records into the project store |
| `DEL-14-01:SOW#CLM-024` | NOT_DIVERGENT | NONE | B8;C7 | T8 F1_ON_CONTEXT: CODE_FIX_CANDIDATE; T12 T12-C02: OWNER_DECISION; DISAGREES (T8 CODE_FIX_CANDIDATE / class NOT_DIVERGENT) | (none recorded; see Notes in ledger) |
| `DEL-14-03:CONTEXT#architecture-basis-injection.s03` | T6-C02 | NONE | B8;A1 | T8 DEC-009: OWNER_DECISION; DISAGREES (T8 OWNER_DECISION / class CODE_FIX_CANDIDATE) | Expose model-state comparison through the application-service boundary named by the architecture basis and add a product caller, or record an owner decision that the Pyt… |
| `DEL-14-03:SOW#CLM-004` | T6-C02 | NONE | B8;A1 | T8 DEC-009: OWNER_DECISION; DISAGREES (T8 OWNER_DECISION / class CODE_FIX_CANDIDATE) | Expose model-state comparison through the application-service boundary named by the architecture basis and add a product caller, or record an owner decision that the Pyt… |
| `DEL-14-03:SOW#CLM-006` | T6-C02 | NONE | B8;A1 | T8 DEC-009: OWNER_DECISION; DISAGREES (T8 OWNER_DECISION / class CODE_FIX_CANDIDATE) | Expose model-state comparison through the application-service boundary named by the architecture basis and add a product caller, or record an owner decision that the Pyt… |
| `DEL-14-03:SOW#CLM-012/REQ-14-03-001` | T6-C01 | NONE | B8 | T12 T12-C02: CODE_FIX_CANDIDATE | Validate the comparison test state fixtures against schemas/model_state.schema.json (the requirement's verification method) |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-14-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-01/DEL-14-01_forward.csv`; ImplementationEvidence cited: `schemas/model_state.schema.json`, `core/project_persistence/service.py#L98`, `apps/desktop/src-tauri/src/lib.rs#L2256`, `core/comparison/model_state/engine.py`, `core/reporting/state_comparison_handoff_sections/engine.py`.
- DEL-14-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-03/DEL-14-03_forward.csv`; ImplementationEvidence cited: `core/comparison/model_state/engine.py`, `core/comparison/model_state/engine.py#L64`, `schemas/model_state.schema.json`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A product-path test saves, lists and reopens a named model-state record that validates against the schema.
- Comparison fixtures validate against the schema (the REQ-14-03-001 verification method).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

6 of 6 claim rows carry a block: A1 (3); B8 (6); C7 (1). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- All rows wait on B8. Three DEL-14-03 rows also carry the T8 DEC-009 reading (T8 OWNER_DECISION vs class CODE_FIX_CANDIDATE; A1).
- DEL-14-01 CLM-024 is not divergent in the effective values; T8 K4 (F1 on CONTEXT) reads it as a PARTIAL_SLICE joining FG-DEL-14-01-02 (C7 confirmation).
- T12-C02 names DEL-14-03 CLM-012/REQ-14-03-001 as unreached.

## Dependencies

B8, A1, C7.

