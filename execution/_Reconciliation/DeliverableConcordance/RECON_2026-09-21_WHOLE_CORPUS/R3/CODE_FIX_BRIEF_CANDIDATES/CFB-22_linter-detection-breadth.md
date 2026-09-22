# CFB-22 — Protected-content linter: detection beyond synthetic markers; CI and privacy boundary

**Candidate brief (H2). Not executed.** Area: Reporting linter (DEL-08-05); protected subject. Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add heuristics for copied prose, formulas, figures and proprietary text beyond planted synthetic markers (freeze `projects/chirality-piping/core/reporting/protected_content_linter/src/lib.rs:4`, `:294`, `:366`); record the step 5 CI policy and step 6 privacy check within the DEC-059/DEC-058 boundary; bind linter enforcement into the adapter/plugin runtime once one is selected.

## Affected claims

8 claim rows on 1 deliverable(s): DEL-08-05.

Classes (portion in this brief / class total): T6-C01 3/180 (Authority NONE); T6-C02 1/116 (Authority NONE); T6-C03 4/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-22"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-08-05:CONTEXT#description` | T6-C03 | REVIEW | H3[T6-C03] | — | (none recorded; see Notes in ledger) |
| `DEL-08-05:SOW#CLM-006.r01` | T6-C03 | REVIEW | H3[T6-C03] | — | Heuristics for copied prose and formula content beyond synthetic markers |
| `DEL-08-05:SOW#CLM-011/DEL-08-05-REQ-002` | T6-C03 | REVIEW | H3[T6-C03] | — | Heuristics for copied prose, formulas, figures and proprietary text beyond synthetic markers |
| `DEL-08-05:SOW#CLM-011/DEL-08-05-REQ-012` | T6-C02 | NONE | B10 | — | Bind linter enforcement into the adapter/plugin runtime when one is selected |
| `DEL-08-05:SOW#CLM-022` | T6-C01 | NONE | — | — | Step 5 CI policy and step 6 privacy check |
| `DEL-08-05:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C03 | REVIEW | H3[T6-C03] | — | Close FG-01 detection breadth and FG-02 phrase coverage |
| `DEL-08-05:SOW#production-and-verification-method-praxeology/VER-001` | T6-C01 | NONE | — | — | Refresh parity against the frozen SOW; close FG-02 and the privacy check |
| `DEL-08-05:SOW#purpose-and-objective-traceability/OUT-001` | T6-C01 | NONE | — | — | CI integration boundary for the linter (hosted CI deferred by DEC-059; release scan owner-only under DEC-058) |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-08-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-05/DEL-08-05_forward.csv`; ImplementationEvidence cited: `core/reporting/protected_content_linter/src/lib.rs`, `schemas/report_protected_content_linter.schema.yaml`, `core/reporting/report_renderer/src/lib.rs`, `core/reporting/pdf_emitter/src/lib.rs`, `apps/desktop/src/features/report-lint/ReportLintPanel.tsx` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Tests use synthetic or public-domain look-alike inputs only; no protected text is added as test data (DEC-043).
- The CI boundary record cites DEC-059 (hosted CI deferred) and DEC-058 (owner-run release scan).
- Independent review (H3 item for T6-C03).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 4 of 8 rows are at INVARIANT tier or carry an IP_DATA, CLAIMS or SECURITY layer (row layers: CLAIMS, IP_DATA). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

5 of 8 claim rows carry a block: B10 (1); H3[T6-C03] (4). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- CONTESTED: DEL-08-05 AC-001 (whether AC-001 judges contract text or implementation).
- REQ-012 waits on B10.

## Dependencies

H3[T6-C03], B10, C2 (protected-content review context).

