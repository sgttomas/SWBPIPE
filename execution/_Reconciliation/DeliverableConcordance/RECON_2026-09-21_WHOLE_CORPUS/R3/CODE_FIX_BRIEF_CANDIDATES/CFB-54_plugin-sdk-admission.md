# CFB-54 — Plugin SDK admission contract: external-execution policy, registry states, checklist categories

**Candidate brief (H2). Not executed.** Area: Plugin SDK contract (DEL-17-09). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add an external-execution policy field and a non-authoritative run-evidence record to the contract and registry records, or record that external execution is excluded; add contract-ready and implementation-gated states or amend the guidance to the implemented state set; align checklist categories with the eleven Step 5 topics or record why some are carried elsewhere; restate the description to the admission contract or implement the SDK surface under its owner.

## Affected claims

8 claim rows on 1 deliverable(s): DEL-17-09.

Classes (portion in this brief / class total): T6-C01 8/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-54"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-09:CONTEXT#description` | T6-C01 | NONE | B10 | — | Implement the runtime SDK surface under its owning deliverable, or restate the description to the admission contract. |
| `DEL-17-09:SOW#CLM-013/DEL-17-09-REQ-010` | T6-C01 | NONE | C7 | T8 TIER_IN_SCOPE_REQ: NO_ACTION; DISAGREES (T8 NO_ACTION / class CODE_FIX_CANDIDATE) | Add an external-execution policy and non-authoritative run-evidence record to the contract, or record that external execution is excluded. |
| `DEL-17-09:SOW#CLM-027` | T6-C01 | NONE | — | — | Add an external execution policy field to registry records. |
| `DEL-17-09:SOW#CLM-029` | T6-C01 | NONE | — | — | Align checklist categories with the eleven Step 5 topics or record why some are carried elsewhere. |
| `DEL-17-09:SOW#CLM-040` | T6-C01 | NONE | — | — | Add contract-ready and implementation-gated states or amend the guidance to the implemented state set. |
| `DEL-17-09:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C01 | NONE | — | — | Represent external-execution limits in the contract and reconcile registry states with CLM-040. |
| `DEL-17-09:SOW#production-and-verification-method-praxeology/VER-001` | T6-C01 | NONE | — | — | Refresh parity against the frozen SOW and represent external-run limits. |
| `DEL-17-09:STATUS#remaining/R01` | T6-C01 | NONE | B10 | — | Assign the runtime loader binding (candidate owners DEL-10-01/DEL-10-02 per CLM-030) and record provenance-detail grains. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-09: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-09/DEL-17-09_forward.csv`; ImplementationEvidence cited: `core/handoff/export_adapter_sdk/package.py`, `schemas/export_adapter_sdk.schema.json`, `apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx`, `tests/test_export_adapter_sdk.py`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Registry schema test asserts the external-execution policy field and the state set.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 8 claim rows carry a block: B10 (2); C7 (1). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- STATUS R01 (runtime loader binding, candidate owners DEL-10-01/DEL-10-02) is an assignment, not code; it waits on B10.
- REQ-010 is CP-11: T8 reads NO_ACTION (C7).
- CONTEXT#description ("implement the runtime SDK surface") waits on the plugin runtime, B10, as the sibling STATUS R01 row does.
- The quarantine/checklist taxonomy (PDU-034) is B10/B12 context.

## Dependencies

B10, C7.

