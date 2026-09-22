# CFB-44 — Durable hash-bound decision/history contract; touching-path check preservation

**Candidate brief (H2). Not executed.** Area: Runtime model operations audit (DEL-16-03, DEL-16-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Define a durable hash-bound decision/history contract with save/load checks for accepted and rejected operations; show each touching path (store, report) preserving schema, provenance, data, diagnostics, hash and boundary checks and restate the stale "until concrete paths are assigned" clause.

## Affected claims

6 claim rows on 2 deliverable(s): DEL-16-03, DEL-16-04.

Classes (portion in this brief / class total): T6-C02 5/116 (Authority NONE); T6-C03 1/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-44"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-16-03:CONTEXT#description` | T6-C02 | NONE | — | — | (none recorded; see Notes in ledger) |
| `DEL-16-03:SOW#CLM-010/DEL-16-03-REQ-005` | T6-C02 | NONE | — | — | Durable hash-bound decision/history contract with save/load checks. |
| `DEL-16-03:SOW#CLM-023` | T6-C02 | NONE | — | — | As REQ-005. |
| `DEL-16-03:STATUS#remaining/R01` | T6-C02 | NONE | — | — | Durable hash-bound decision/history contract with save/load checks. |
| `DEL-16-04:SOW#CLM-009/REQ-16-04-02` | T6-C02 | NONE | — | — | Durable decision/history contract (see DEL-16-03 Remaining). |
| `DEL-16-04:SOW#CLM-009/REQ-16-04-09` | T6-C03 | REVIEW | H3[T6-C03] | — | OC: show each touching path (store, report) preserving the listed checks; restate the stale 'until concrete paths are assigned' clause |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-16-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-16/DEL-16-03/DEL-16-03_forward.csv`; ImplementationEvidence cited: `core/model_operations/audit_trail/engine.py`, `core/model_operations/validation_preview/engine.py`, `schemas/model_operation.schema.json`, `fixtures/model_operations/invented_operation_set_valid.json`, `fixtures/model_operations/invented_accepted_model_state.json` ….
- DEL-16-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-16/DEL-16-04/DEL-16-04_forward.csv`; ImplementationEvidence cited: `core/model_operations/agent_rationale/engine.py`, `apps/desktop/src-tauri/src/lib.rs`, `core/model_operations/operation_applier/src/lib.rs`, `tools/validation/validate_claims_language.py`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Save/load test: history records survive with hashes intact; tampering is detected.
- A per-path test or record for store and report preserving the listed checks.
- Independent review for REQ-16-04-09 (H3 item for T6-C03).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 1 of 6 rows are at INVARIANT tier or carry protected layers (CLAIMS, IP_DATA). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

1 of 6 claim rows carry a block: H3[T6-C03] (1). `H3[<class>]` names the H3 register item for that class (review before repair). Unblocked rows may proceed separately once selected.

## Dependencies

H3[T6-C03], CFB-43.

