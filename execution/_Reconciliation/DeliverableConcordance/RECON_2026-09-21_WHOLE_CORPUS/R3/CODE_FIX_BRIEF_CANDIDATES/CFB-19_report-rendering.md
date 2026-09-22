# CFB-19 — Report rendering: diagnostic class, provenance notes, assumption source and warning fixtures

**Candidate brief (H2). Not executed.** Area: Report renderer (DEL-08-01, DEL-08-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Render the diagnostic class, source and affected object (carry upstream class rather than deriving it from severity) and provenance references; render redistribution and review status in the provenance-notes table; supply an upstream assumption source to product reports or record why none is carried; add fixtures for each supported warning class, user-rule-checked and protected-content-risk reports, and a reproducibility-reference check including versions.

## Affected claims

9 claim rows on 2 deliverable(s): DEL-08-01, DEL-08-03.

Classes (portion in this brief / class total): T6-C01 5/180 (Authority NONE); T6-C02 4/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-19"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-08-01:SOW#CLM-011/R-08-01-007` | T6-C02 | NONE | — | — | Render the diagnostic class (and source/affected object) in the diagnostics table; carry upstream class when supplied instead of deriving it from severity. |
| `DEL-08-01:SOW#CLM-013.s01` | T6-C01 | NONE | — | — | Add user-rule-checked and protected-content-risk report fixtures to the renderer tests; record the human review of non-assertion for the current renderer. |
| `DEL-08-03:CONTEXT#description` | T6-C01 | NONE | — | — | Supply an assumption source to product reports. |
| `DEL-08-03:SOW#CLM-006.r03` | T6-C01 | NONE | — | — | Supply an upstream assumption source to the product report input, or record why product reports carry none. |
| `DEL-08-03:SOW#CLM-006.r05` | T6-C02 | NONE | — | — | Render redistribution status and review status in the provenance-notes table. |
| `DEL-08-03:SOW#CLM-012/DEL-08-03-REQ-003` | T6-C02 | NONE | — | — | Render class, source, affected object and provenance references for diagnostics; keep unsupplied remediation as TBD. |
| `DEL-08-03:SOW#CLM-014` | T6-C01 | NONE | — | — | Add a fixture covering each supported warning class and a reproducibility-reference check that includes versions. |
| `DEL-08-03:SOW#CLM-029.r03` | T6-C01 | NONE | — | — | Supply an assumption source to product reports. |
| `DEL-08-03:SOW#CLM-029.r05` | T6-C02 | NONE | — | — | Render class, source and affected object; carry NONLINEAR_WARNING when the producer supplies it. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-08-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-01/DEL-08-01_forward.csv`; ImplementationEvidence cited: `core/reporting/report_sections/src/lib.rs`, `core/reporting/report_renderer/src/lib.rs`, `apps/desktop/src/features/report/renderableReportInput.ts`, `apps/desktop/src/features/report/reportPackageRequest.ts`, `core/reporting/report_renderer/tests/render.rs` ….
- DEL-08-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-03/DEL-08-03_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-03_Warnings, assumptions, and provenance report section/_CONTEXT.md`, `core/reporting/report_sections/src/lib.rs`, `core/reporting/report_renderer/src/lib.rs`, `apps/desktop/src/features/report/renderableReportInput.ts`, `schemas/report_sections.schema.yaml` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Renderer tests assert class, source, affected object and provenance columns; NONLINEAR_WARNING is carried when supplied.
- A fixture per warning class renders without inventing values; unsupplied remediation stays TBD.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Notes and open views

- DEL-08-01 CLM-013.s01 also asks for a human review record of non-assertion for the current renderer; that is a review act (context for H3), not code.

## Dependencies

CFB-13, CFB-17, CFB-18.

