# CFB-50 — Stress-neutral CSV: contract location, traceability table, owning workflows

**Candidate brief (H2). Not executed.** Area: Stress-neutral export (DEL-17-06); record items. Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Record the CSV contract location and whether comparison fixtures are still wanted; write the requirement-to-evidence traceability table; run the owning semantic and dependency workflows. Guard (chirality-piping execution profile §8): dependency-graph changes are not made under this brief; they go to the separate owner-directed DAG rebuild (see U2 and C6).

## Affected claims

3 claim rows on 1 deliverable(s): DEL-17-06.

Classes (portion in this brief / class total): T6-C01 3/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-50"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-06:CONTEXT#anticipated-artifacts` | T6-C01 | NONE | — | — | Record the CSV contract location and whether comparison fixtures are still wanted. |
| `DEL-17-06:SOW#CLM-015/DEL-17-06-VER-006` | T6-C01 | NONE | — | — | Write the requirement-to-evidence traceability table. |
| `DEL-17-06:STATUS#remaining/R02` | T6-C01 | NONE | — | — | Run the owning semantic and dependency workflows. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-06: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-06/DEL-17-06_forward.csv`; ImplementationEvidence cited: `schemas/stress_neutral_export.schema.json`, `schemas/stress_neutral_export.v0.1.schema.json`, `schemas/stress_neutral_export.v0.2.schema.json`, `core/handoff/stress_neutral/package.py`, `fixtures/stress_neutral/invented/stress_neutral_export_package.json` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- The traceability table covers every DEL-17-06 VER item.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Notes and open views

- Record or document work, not code (T6-C01 notes); carried as record items. The DEL-17-06 table inventory contradiction is C1.

## Dependencies

C1 (context).

