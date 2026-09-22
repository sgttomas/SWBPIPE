# CFB-38 — Comparison CSV/JSON exporters for the comparison_review contracts

**Candidate brief (H2). Not executed.** Area: Comparison export (DEL-14-05). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Implement comparison CSV/JSON exporters that emit the `comparison_review_csv_v1`/`json_v1` contracts with instance-level schema validation (only `schemas/comparison_mapping.schema.json` and `schemas/comparison_tolerance.schema.json` exist at the freeze); write contract documentation and a TBD register; repoint the dependency record to DAG-010.

## Affected claims

6 claim rows on 1 deliverable(s): DEL-14-05.

Classes (portion in this brief / class total): T6-C01 6/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-38"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-14-05:CONTEXT#anticipated-artifacts` | T6-C01 | NONE | — | — | Implement comparison CSV/JSON exporters that emit the comparison_review_csv_v1/json_v1 contracts, with instance-level schema validation and CSV parse/round-trip tests. |
| `DEL-14-05:SOW#CLM-010` | T6-C01 | NONE | — | — | Implement comparison CSV/JSON exporters that emit the comparison_review_csv_v1/json_v1 contracts, with instance-level schema validation and CSV parse/round-trip tests. |
| `DEL-14-05:SOW#CLM-013.r05` | T6-C01 | NONE | — | — | Implement comparison CSV/JSON exporters that emit the comparison_review_csv_v1/json_v1 contracts, with instance-level schema validation and CSV parse/round-trip tests. |
| `DEL-14-05:SOW#CLM-013.r06` | T6-C01 | NONE | — | — | Implement comparison CSV/JSON exporters that emit the comparison_review_csv_v1/json_v1 contracts, with instance-level schema validation and CSV parse/round-trip tests. |
| `DEL-14-05:SOW#CLM-013.r07` | T6-C01 | NONE | — | — | Implement comparison CSV/JSON exporters that emit the comparison_review_csv_v1/json_v1 contracts, with instance-level schema validation and CSV parse/round-trip tests. |
| `DEL-14-05:SOW#CLM-014` | T6-C01 | NONE | — | — | Write contract documentation (or cite schema descriptions explicitly) and a TBD register; repoint the dependency record to DAG-010. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-14-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-05/DEL-14-05_forward.csv`; ImplementationEvidence cited: `schemas/comparison_mapping.schema.json`, `schemas/comparison_tolerance.schema.json`, `core/handoff/exporter/workflow.py`, `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/MEMORY.md`, `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/_DEPENDENCIES.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Exporter output validates against the contract schema instance by instance.
- Units, analysis status and limitations are preserved in the export.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Notes and open views

- The comparison-result schema home is a held selection (B12 D7); the rows here carry no block because the class is T6-C01.

## Dependencies

B12 (D7, context), CFB-37.

