# CFB-48 — MBF profile pass-through options and open questions

**Candidate brief (H2). Not executed.** Area: CAEPIPE MBF export (DEL-17-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Implement pass-through options as profile metadata with a test that they reach no checking logic, or narrow REQ-006; carry line-ending and encoding as open questions in CLM-022 or record them closed.

## Affected claims

4 claim rows on 1 deliverable(s): DEL-17-04.

Classes (portion in this brief / class total): T6-C01 4/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-48"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-04:CONTEXT#context-envelope` | T6-C01 | NONE | — | — | As REQ-006. |
| `DEL-17-04:SOW#CLM-005.r10` | T6-C01 | NONE | — | — | If pass-through options are wanted, add a target-configuration field to the profile schema and builder; otherwise record them as out of the foundation. |
| `DEL-17-04:SOW#CLM-009/DEL-17-04-REQ-006` | T6-C01 | NONE | — | — | Implement pass-through options as profile metadata with a test that they reach no checking logic, or narrow the requirement. |
| `DEL-17-04:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C01 | NONE | — | — | Carry line-ending and encoding as open questions in CLM-022 (or record them closed) and give pass-through options a contract surface. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-04/DEL-17-04_forward.csv`; ImplementationEvidence cited: `core/handoff/caepipe_mbf/package.py`, `schemas/caepipe_mbf_export.schema.json`, `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/ScopeOfWork.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A test shows pass-through options appear in profile metadata and affect no check.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Notes and open views

- CP-11 NOT_STARTED rows (T6-C01 notes).

## Dependencies

None.

