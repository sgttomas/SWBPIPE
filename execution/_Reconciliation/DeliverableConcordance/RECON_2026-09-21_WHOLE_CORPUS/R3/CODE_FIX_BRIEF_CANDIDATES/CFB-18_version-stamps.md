# CFB-18 — Software/application and solver version stamps in manifests and rendered reports

**Candidate brief (H2). Not executed.** Area: Reporting (DEL-08-01, DEL-08-02, DEL-08-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add the software/application version alongside the solver stamp in the manifest and audit-manifest records, and carry software and solver versions into the rendered report content (or mark them TBD where not supplied).

## Affected claims

7 claim rows on 3 deliverable(s): DEL-08-01, DEL-08-02, DEL-08-03.

Classes (portion in this brief / class total): T6-C02 7/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-18"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-08-01:SOW#CLM-011/R-08-01-002` | T6-C02 | NONE | — | — | Add software/application version and solver version to the rendered report content; bind rule-pack name/version/checksum in the product path. |
| `DEL-08-01:SOW#CLM-028.r06` | T6-C02 | NONE | — | — | Bind software/solver versions into the report and rule-pack checksums in the product path. |
| `DEL-08-02:SOW#CLM-006.r03` | T6-C02 | NONE | — | — | Add an application/software version to the manifest record. |
| `DEL-08-02:SOW#CLM-011.r04` | T6-C02 | NONE | — | — | Add a software/application version to the audit manifest. |
| `DEL-08-02:SOW#CLM-013/V-7` | T6-C02 | NONE | — | — | Carry software and solver versions into the rendered report. |
| `DEL-08-02:SOW#CLM-025.r04` | T6-C02 | NONE | — | — | Add an application/software version alongside the solver stamp. |
| `DEL-08-03:SOW#CLM-012/DEL-08-03-REQ-009` | T6-C02 | NONE | — | — | Reference software and solver versions (or mark them TBD) in the report sections or rendered report. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-08-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-01/DEL-08-01_forward.csv`; ImplementationEvidence cited: `core/reporting/report_renderer/src/lib.rs`, `core/reporting/report_package/src/lib.rs`, `core/reporting/report_package/src/wire.rs`, `apps/desktop/src/features/report/renderableReportInput.ts`, `apps/desktop/src/features/report/reportPackageRequest.ts`.
- DEL-08-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-02/DEL-08-02_forward.csv`; ImplementationEvidence cited: `core/reporting/audit_manifest/src/lib.rs`, `apps/desktop/src/services/inputManifestService.ts`, `core/reporting/report_package/src/wire.rs`, `apps/desktop/src-tauri/src/report_package_bridge.rs`, `core/reporting/report_package/src/lib.rs` ….
- DEL-08-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-03/DEL-08-03_forward.csv`; ImplementationEvidence cited: `core/reporting/report_sections/src/lib.rs`, `core/reporting/report_renderer/src/lib.rs`, `apps/desktop/src/features/report/renderableReportInput.ts`, `core/reporting/audit_manifest/src/lib.rs`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A rendered-report test asserts software and solver versions appear.
- Manifest and audit-manifest schema tests assert the application version field.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Dependencies

CFB-17.

