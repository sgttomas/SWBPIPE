# CFB-25 — Adapter framework: envelope rule-pack references, invalid-input encoding, live-path conditions

**Candidate brief (H2). Not executed.** Area: Adapters (DEL-10-01, DEL-10-02, DEL-10-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add a rule-pack reference structure and an explicit invalid-input encoding to the adapter result envelope schema, with assertions (DEL-10-01). The DEL-10-02 rows (conditions, hooks, dimensional checks, loss reporting, round-trip evidence on a live adapter path) and DEL-10-03 (bind the handoff package to the API boundary no-bypass constraints) apply only once an execution model exists.

## Affected claims

12 claim rows on 3 deliverable(s): DEL-10-01, DEL-10-02, DEL-10-03.

Classes (portion in this brief / class total): T6-C01 12/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-25"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-10-01:SOW#CLM-013/DEL-10-01-REQ-09` | T6-C01 | NONE | — | — | Add a rule-pack reference structure (identity, version, checksum, source notice, redistribution, marking) to the envelope schema and assert it in the contract test. |
| `DEL-10-01:SOW#CLM-013/DEL-10-01-REQ-10` | T6-C01 | NONE | — | — | Add an explicit invalid-input encoding to the result envelope. |
| `DEL-10-01:SOW#CLM-015` | T6-C01 | NONE | — | — | Add invalid-input encoding and its assertion; then the status review can pass. |
| `DEL-10-02:CONTEXT#description` | T6-C01 | NONE | B10 | — | As REQ-10-02-02. |
| `DEL-10-02:SOW#CLM-005` | T6-C01 | NONE | B10 | — | Enforce the conditions on a live adapter path once an execution model is selected; re-point the source citations. |
| `DEL-10-02:SOW#CLM-006` | T6-C01 | NONE | B10 | — | Select and implement rule/report hooks under a governed execution model. |
| `DEL-10-02:SOW#CLM-013` | T6-C01 | NONE | B10 | — | Either add the listed tests on a live path or restate the verification list to the selected seam. |
| `DEL-10-02:SOW#CLM-020` | T6-C01 | NONE | B10 | — | Add dimensional checks once a live path exists. |
| `DEL-10-02:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C01 | NONE | B10 | — | Add loss reporting and round-trip evidence when a live adapter path exists. |
| `DEL-10-02:SOW#purpose-and-objective-traceability/OUT-001` | T6-C01 | NONE | B10 | — | Produce round-trip evidence once an adapter execution model is selected (owner-held). |
| `DEL-10-03:SOW#CLM-013/DEL-10-03-REQ-08` | T6-C01 | NONE | B7 | — | Bind the handoff package to the API boundary no-bypass constraints (or a job envelope) when an export adapter is implemented. |
| `DEL-10-03:SOW#CLM-015/REQ-08` | T6-C01 | NONE | B7 | — | As REQ-08. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-10-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-10/DEL-10-01/DEL-10-01_forward.csv`; ImplementationEvidence cited: `api/api_boundary_contract.yaml`, `projects/chirality-piping/docs/architecture/plugin_boundary.md`, `projects/chirality-piping/docs/TYPES.md`.
- DEL-10-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-10/DEL-10-02/DEL-10-02_forward.csv`; ImplementationEvidence cited: `core/adapters/framework/adapter_framework.py`, `schemas/adapter_framework.schema.yaml`, `projects/chirality-piping/docs/_Registers/Deliverables.csv`, `apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx`, `tests/test_adapter_framework_contract.py` ….
- DEL-10-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-10/DEL-10-03/DEL-10-03_forward.csv`; ImplementationEvidence cited: `schemas/local_fea_handoff.schema.yaml`, `api/api_boundary_contract.yaml`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Envelope schema tests assert the rule-pack reference structure and invalid-input encoding (DEL-10-01).
- DEL-10-02 items either run on a live path or the verification list is restated to the selected seam.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

9 of 12 claim rows carry a block: B10 (7); B7 (2). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- DEL-10-02 rows wait on B10 (runtime not selected; `gate_adapter_runtime_dispatch` never dispatches). DEL-10-03 waits on B7 (canonical handoff path).

## Dependencies

B10, B7.

