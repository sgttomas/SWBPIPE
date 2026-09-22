# CFB-39 — Report-section export references bound through the reporting owner

**Candidate brief (H2). Not executed.** Area: Comparison report sections (DEL-14-05). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Bind report-section export references through the reporting owner, with explicit units, analysis-status and limitation preservation.

## Affected claims

6 claim rows on 1 deliverable(s): DEL-14-05.

Classes (portion in this brief / class total): T6-C01 6/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-39"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-14-05:SOW#CLM-005.r04` | T6-C01 | NONE | — | — | Bind report-section export references through the reporting owner with explicit units, analysis-status and limitation preservation; ReportSectionExportRef currently requ… |
| `DEL-14-05:SOW#CLM-006.r06` | T6-C01 | NONE | — | — | Bind report-section export references through the reporting owner with explicit units, analysis-status and limitation preservation; ReportSectionExportRef currently requ… |
| `DEL-14-05:SOW#CLM-011.r08` | T6-C01 | NONE | — | — | Bind report-section export references through the reporting owner with explicit units, analysis-status and limitation preservation; ReportSectionExportRef currently requ… |
| `DEL-14-05:SOW#CLM-013.r08` | T6-C01 | NONE | — | — | Bind report-section export references through the reporting owner with explicit units, analysis-status and limitation preservation; ReportSectionExportRef currently requ… |
| `DEL-14-05:SOW#CLM-019` | T6-C01 | NONE | — | — | Bind report-section export references through the reporting owner with explicit units, analysis-status and limitation preservation; ReportSectionExportRef currently requ… |
| `DEL-14-05:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C01 | NONE | — | — | Bind report-section export references through the reporting owner with explicit units, analysis-status and limitation preservation; ReportSectionExportRef currently requ… |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-14-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-05/DEL-14-05_forward.csv`; ImplementationEvidence cited: `schemas/comparison_mapping.schema.json`, `schemas/comparison_tolerance.schema.json`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A report-section export test asserts units, analysis status and limitations survive.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Dependencies

CFB-19, CFB-38.

