# CFB-17 — Populate rule-pack and library references through the product report, export and analysis-run path

**Candidate brief (H2). Not executed.** Area: Reporting and analysis records (DEL-06-04, DEL-08-01, DEL-08-02, DEL-08-03, DEL-08-04, DEL-14-02). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Populate rule-pack identity, version and checksum (and library references) from validated local rule packs and the solve/rule-check inputs into the product report input, package request, input and audit manifests, result export writer and analysis-run records. At the freeze these are written empty: `apps/desktop/src/features/report/reportPackageRequest.ts:286` and `:318`, `core/analysis_runs/records.py:132-133`, `apps/desktop/src/services/analysisRunCompatibility.ts:67`. This is the W3 item "analysis-run records always write empty rule-pack and library references".

## Affected claims

29 claim rows and 1 W3 item(s) on 6 deliverable(s): DEL-06-04, DEL-08-01, DEL-08-02, DEL-08-03, DEL-08-04, DEL-14-02.

Classes (portion in this brief / class total): T6-C01 7/180 (Authority NONE); T6-C02 22/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-17"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-06-04:SOW#CLM-003.r07` | T6-C01 | NONE | — | — | Populate rule_pack_refs in product report/export requests from the validated local rule packs. |
| `DEL-06-04:SOW#CLM-010/R-06-04-008` | T6-C01 | NONE | — | — | Populate rule_pack_refs in product report/export requests from validated local rule packs. |
| `DEL-06-04:SOW#CLM-025.r05` | T6-C01 | NONE | — | — | Populate rule_pack_refs in product report/export requests. |
| `DEL-06-04:SOW#purpose-and-objective-traceability/OUT-001` | T6-C01 | NONE | — | — | Wire product report/export requests to carry rule-pack audit refs; define and implement non-JSON/binary manifest-hash partitioning. |
| `DEL-08-01:CONTEXT#description` | T6-C02 | NONE | — | — | Bind rule-pack references in the product report path. |
| `DEL-08-01:SOW#CLM-004.r02` | T6-C02 | NONE | — | — | Bind rule-pack identity/version/checksum into the product report input and package request. |
| `DEL-08-01:SOW#CLM-006.s01` | T6-C02 | NONE | — | — | Supply rule-pack references to the product report input. |
| `DEL-08-01:SOW#CLM-011/R-08-01-001` | T6-C02 | NONE | — | — | Bind rule-pack identity/version/checksum into product report inputs and packages; supply an upstream assumption source if one is scoped. |
| `DEL-08-01:SOW#CLM-011/R-08-01-008` | T6-C02 | NONE | — | — | Bind rule-pack metadata into product reports; add the rule-pack report fixtures named in the verification column. |
| `DEL-08-02:CONTEXT#description` | T6-C02 | NONE | — | — | Bind rule-pack checksum capture in the product path. |
| `DEL-08-02:SOW#CLM-004.r04` | T6-C02 | NONE | — | — | Bind active rule-pack checksums into the product input manifest and audit manifest. |
| `DEL-08-02:SOW#CLM-005.s02` | T6-C02 | NONE | — | — | Bind rule-pack checksums into the product reproducibility context. |
| `DEL-08-02:SOW#CLM-006.r01` | T6-C02 | NONE | — | — | OC: bind active rule-pack references and external-asset references into the product input manifest |
| `DEL-08-02:SOW#CLM-006.r04` | T6-C02 | NONE | — | — | Bind active rule packs into the product manifest path. |
| `DEL-08-02:SOW#CLM-011.r05` | T6-C02 | NONE | — | — | Bind participating rule packs into the product manifest. |
| `DEL-08-03:SOW#CLM-006.r04` | T6-C02 | NONE | — | — | Add rule-pack value rows to the product report sections once rule-pack binding lands. |
| `DEL-08-03:SOW#CLM-012/DEL-08-03-REQ-004` | T6-C02 | NONE | — | — | Bind rule-pack identity/checksum references into product reports. |
| `DEL-08-03:SOW#CLM-029.r04` | T6-C02 | NONE | — | — | Bind rule-pack metadata into product reports. |
| `DEL-08-04:SOW#CLM-011.r04` | T6-C01 | NONE | — | — | Populate rule_pack_refs in the desktop result writer when the analysis run carries a user rule-check status, or record why none applies |
| `DEL-08-04:SOW#CLM-013/V-3` | T6-C01 | NONE | — | — | Add a check that exports a run with missing provenance and missing rule-pack input and asserts the diagnostics |
| `DEL-08-04:SOW#CLM-013/V-4` | T6-C01 | NONE | — | — | Exercise rule-pack refs through the product result writer |
| `DEL-14-02:SOW#CLM-004` | T6-C02 | NONE | — | — | Populate rule_pack_refs and library_refs (identity, version, checksum, provenance) from the solve and rule-check inputs in the product record builder, with tests |
| `DEL-14-02:SOW#CLM-005` | T6-C02 | NONE | — | — | Populate rule_pack_refs and library_refs (identity, version, checksum, provenance) from the solve and rule-check inputs in the product record builder, with tests |
| `DEL-14-02:SOW#CLM-011.r03` | T6-C02 | NONE | — | — | Populate rule_pack_refs and library_refs (identity, version, checksum, provenance) from the solve and rule-check inputs in the product record builder, with tests |
| `DEL-14-02:SOW#CLM-011.r09` | T6-C02 | NONE | — | T8 TIER_IN_SCOPE_REQ: CODE_FIX_CANDIDATE | Populate rule_pack_refs and library_refs (identity, version, checksum, provenance) from the solve and rule-check inputs in the product record builder, with tests |
| `DEL-14-02:SOW#CLM-013` | T6-C02 | NONE | — | — | Populate rule_pack_refs and library_refs (identity, version, checksum, provenance) from the solve and rule-check inputs in the product record builder, with tests |
| `DEL-14-02:SOW#CLM-020` | T6-C02 | NONE | — | — | Populate rule_pack_refs and library_refs (identity, version, checksum, provenance) from the solve and rule-check inputs in the product record builder, with tests |
| `DEL-14-02:SOW#CLM-025` | T6-C02 | NONE | — | — | Populate rule_pack_refs and library_refs (identity, version, checksum, provenance) from the solve and rule-check inputs in the product record builder, with tests |
| `DEL-14-02:SOW#purpose-and-objective-traceability/OUT-001` | T6-C02 | NONE | — | — | Populate rule_pack_refs and library_refs (identity, version, checksum, provenance) from the solve and rule-check inputs in the product record builder, with tests |
| `W3-PC-01` (ITEM) | W3_ASSESSMENT | NONE | — | — | Analysis-run records always write empty rule-pack and library references |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-06-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/DEL-06-04/DEL-06-04_forward.csv`; ImplementationEvidence cited: `core/rules/rule_pack_lifecycle/src/lib.rs`, `core/reporting/audit_manifest/src/lib.rs`, `core/reporting/report_package/src/wire.rs`, `apps/desktop/src/features/report/reportPackageRequest.ts`, `apps/desktop/src/features/report/renderableReportInput.ts` ….
- DEL-08-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-01/DEL-08-01_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_CONTEXT.md`, `core/reporting/report_renderer/src/lib.rs`, `apps/desktop/src/features/report/renderableReportInput.ts`, `apps/desktop/src/features/report/reportPackageRequest.ts`, `core/reporting/report_generator/src/lib.rs`.
- DEL-08-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-02/DEL-08-02_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/_CONTEXT.md`, `core/reporting/audit_manifest/src/lib.rs`, `apps/desktop/src/services/inputManifestService.ts`, `apps/desktop/src/features/report/reportPackageRequest.ts`, `core/reporting/report_package/src/wire.rs` ….
- DEL-08-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-03/DEL-08-03_forward.csv`; ImplementationEvidence cited: `core/reporting/report_sections/src/lib.rs`, `apps/desktop/src/features/report/renderableReportInput.ts`, `core/reporting/report_generator/src/lib.rs`, `core/reporting/report_renderer/src/lib.rs`, `apps/desktop/src/features/report/reportPackageRequest.ts`.
- DEL-08-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-04/DEL-08-04_forward.csv`; ImplementationEvidence cited: `schemas/results.v0.1.schema.yaml`, `core/reporting/result_export/src/lib.rs`, `apps/desktop/src/features/result-export/resultExportAdapter.ts`, `apps/desktop/src/services/previewService.ts`.
- DEL-14-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-02/DEL-14-02_forward.csv`; ImplementationEvidence cited: `schemas/analysis_run.v0.2.schema.json`, `apps/desktop/src/services/analysisRunCompatibility.ts`, `core/analysis_runs/records.py#L132`, `schemas/analysis_run.v0.1.schema.json`, `tests/test_analysis_run_compatibility.py` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A product-path test with an active rule pack shows its identity, version and checksum in the report input, input manifest, audit manifest, result export and analysis-run record.
- A run with no rule-check input records an explicit empty/absent status rather than a silent empty list, and the missing-rule-pack diagnostic fires (DEL-08-04 V-3).
- Rule-pack report fixtures named in the DEL-08-01 verification column exist.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Notes and open views

- The engine side already records rule-pack identity and checksum (T6-C02 representative key DEL-08-02:SOW#CLM-004.r04); the gap is the product request path.
- DEL-14-02:SOW#CLM-011.r09 carries the T8 K6 reading (PROJECT_BASELINE stands; no change).

## Dependencies

CFB-18, CFB-19.

