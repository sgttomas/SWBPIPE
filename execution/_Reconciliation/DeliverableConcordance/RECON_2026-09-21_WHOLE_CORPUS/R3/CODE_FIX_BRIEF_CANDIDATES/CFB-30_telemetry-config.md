# CFB-30 — Telemetry: product config default-false opt-in and field-class rejection test

**Candidate brief (H2). Not executed.** Area: Telemetry (DEL-12-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Choose the product config surface and add the default-false telemetry opt-in value with a fixture and test (TEL-REQ-005, TEL-TEST-002, anticipated artifact); add rule-pack, material and component field classes to the rejection test (TEL-TEST-004).

## Affected claims

4 claim rows on 1 deliverable(s): DEL-12-03.

Classes (portion in this brief / class total): T6-C01 3/180 (Authority NONE); T6-C02 1/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-30"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-12-03:CONTEXT#anticipated-artifacts` | T6-C02 | NONE | — | — | Product config default for telemetry once the config surface is chosen. |
| `DEL-12-03:SOW#CLM-011/TEL-REQ-005` | T6-C01 | NONE | — | — | Choose the product config surface and add the default-false opt-in value with a fixture. |
| `DEL-12-03:SOW#CLM-013/TEL-TEST-002` | T6-C01 | NONE | — | — | Add once the product config surface exists. |
| `DEL-12-03:SOW#CLM-013/TEL-TEST-004` | T6-C01 | NONE | — | — | Add rule-pack, material and component field classes to the rejection test. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-12-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-12/DEL-12-03/DEL-12-03_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/docs/security/telemetry_policy.md`, `core/security/telemetry_policy/controls.py`, `apps/desktop/src/services/telemetryPolicyService.ts`, `NONE_FOUND`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A fresh config reads telemetry disabled; a fixture test asserts it.
- The rejection test covers rule-pack, material and component field classes.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 3 of 4 rows are at INVARIANT tier or carry protected layers (SECURITY). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Notes and open views

- Consumer telemetry routes are a held owner selection (B12, T6-C04 D5) and are not in this brief. TEL-REQ-008 (diagnostic alignment) is in CFB-13.

## Dependencies

B12 (D5, context), CFB-13.

