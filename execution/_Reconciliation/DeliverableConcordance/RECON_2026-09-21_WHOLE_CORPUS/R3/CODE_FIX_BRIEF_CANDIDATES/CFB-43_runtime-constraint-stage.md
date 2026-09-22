# CFB-43 — Constraint-validation stage on the runtime operation route

**Candidate brief (H2). Not executed.** Area: Runtime model operations (DEL-16-02, DEL-16-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Integrate a constraint-validation stage (DEL-13-03) into the runtime operation route before controlled application, and gate runtime acceptance receipts on it; at the freeze intents carry `"constraint_validation": "not_run"` (`projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:4213`, per the T6-C02 representative key).

## Affected claims

3 claim rows on 2 deliverable(s): DEL-16-02, DEL-16-03.

Classes (portion in this brief / class total): T6-C02 3/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-43"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-16-02:CONTEXT#description` | T6-C02 | NONE | B3 | — | Same as REQ-16-02-003. |
| `DEL-16-02:SOW#CLM-010/REQ-16-02-003` | T6-C02 | NONE | B3 | — | Integrate a constraint-validation stage (DEL-13-03) into the runtime operation route or record a ruling deferring it. |
| `DEL-16-03:SOW#CLM-010/DEL-16-03-REQ-002` | T6-C02 | NONE | B3 | — | Gate runtime acceptance receipts on a constraint-validation stage or record a ruling. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-16-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-16/DEL-16-02/DEL-16-02_forward.csv`; ImplementationEvidence cited: `core/model_operations/validation_preview/engine.py`, `core/model_operations/operation_applier/src/lib.rs`, `apps/desktop/src-tauri/src/lib.rs`.
- DEL-16-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-16/DEL-16-03/DEL-16-03_forward.csv`; ImplementationEvidence cited: `core/model_operations/audit_trail/engine.py`, `core/model_operations/operation_applier/src/lib.rs`, `apps/desktop/src-tauri/src/lib.rs`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A runtime edit that violates a constraint is rejected before application, with a diagnostic.
- Acceptance receipts record the constraint-validation result.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 3 claim rows carry a block: B3 (3). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- The applier sits outside the DEL-16-02/16-03 scopes (B3); the stage lands wherever B3 places the applier. The "record a ruling deferring it" branch is an owner alternative.

## Dependencies

B3, CFB-33.

