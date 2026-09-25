# Repair04 — one App case current-contract applicability

Actual predecessor evidence: `../../CONSUMER_CHECKS/REPAIR_03/_run_records/vitest.log` reports 297 passed / 1 failed, App218/219. Failure at prior App10244 expected derivative0.2 but received0.3. Those passes remain predecessor evidence and are not replaced by a new pass claim. No tests/builds/TypeScript/Git/native/dependency operations were performed here.

Only App.test.tsx changed: the named giant case and necessary imports. The entire remainder through its closing brace was inspected. `_run_records/inspected_case.before.tsx` preserves that full basis; the incremental diff identifies every changed assertion. Other thirteen reviewed files are unchanged.

## Changed assertion basis table

All line numbers below refer to the Repair03 preimage; canonical source/schema paths are relative to projects/chirality-piping. Actual hashes and origins are pinned in `_run_records/RUNTIME_ORIGINS.json`.

| Prior assertion location | Correction / additional guard | Selected contract and evidence |
|---|---|---|
| 10005, source preconditions | Assert raw producer wire0.2, precision semantic contract and absent raw dimensions before derivative expectations. | Genuine `fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json`; precision contract fixture; no header/source modification. |
| 10244–10245 | Derivative outer/inner versions0.3; assert copied producer, numerical_quality, formulation_basis and semantic-contract ref. | `schemas/results.v0.3.schema.yaml` properties.schema_version and ResultEnvelope; resultExportAdapter.ts:41–44; maintained resultExportAdapter.test.ts current0.3 cases62–76. |
| 10270 | Received row checksum scope dimension_absent_row; same exact raw-row value digest with checked JCS profile. | Results0.3 Checksum enum; resultExportAdapter.ts:57–58 and103–119. Source has no raw dimensions; no original producer attestation is minted. |
| 10286,10300,10307 | Checked-profile helper for target row, model and derivative document digests. | Existing `openpipestress_jcs_ijson_v1` claims and resultDigest/checked hash helpers; no checksum/value comparison removed. |
| 10304 | Origin class dimension_absent and carrier scope dimension_absent_carrier, retaining false authentic-producer flag/null original checksum, model/run/qualification refs and exact carrier checksum. | Results0.3 origin schema; resultExportAdapter.ts:178–179; maintained dimension-absent origin test67–70. Absence does not prove producer attestation. |
| 10309 comment | Clarify0.3 carries rule completeness; no private rule pack payload guard unchanged. | Current derivative schema plus unchanged analysis_status/no rule_pack_refs assertions. |
| 10318–10321 | Work rows selected by accepted signature category diagnostic_work; per-case count, exact kind/unit and raw dimension absence asserted; target dimension remains null; witness eligibility excludes those rows. | `semantic_contract_v0_3_precision_1.json` free_dof_work_residual signature; analysisRowSemantics; StressNeutralExportPanel.ts:405–417. No N*m-to-moment/energy inference. |
| 10338,10410,10417–10422 | Stress profile/schema0.3 and ops.stress_neutral.v3; bind copied numerical metadata and received-result source checksum to exact source. | `schemas/stress_neutral_export.v0.3.schema.json` version/profile/producer requirements; maintained stress precision test110–129; strict builder source binding440–455. |
| 10432 additions | Explicitly exclude work IDs from witnesses; require matching info withholding diagnostics plus blocking aggregate; all retained row values/units and interpreted dimensions checked (null semantic dimension gives TBD). | StressNeutralExportPanel.ts:405–430,465–490,744–754; maintained legacy work-withholding and current precision source-binding tests remain untouched. No existing privacy/no-conversion guard removed. |
| 10462 addition | Recompute whole package checksum excluding only package_checksum. | Existing complete_package_excluding_self_checksum scope; strict builder547 and validator's same exclusion. |
| 11545 | Full received displacement value/unit, finite value prerequisite. | Current fixture row value3.9773012982613305; ResultsPanel QuantityReadout; same-unit precision contract and displayQuantityPrecision.test.ts. No solver oracle/tolerance changed. |
| 11970 | Full received summary value/unit/ref instead of rounded legacy literal. | Current fixture max_displacement and KnowledgePanel.ts132–133 QuantityReadout. |
| End of case | Repeat exact model/source snapshot equality after all review/queue actions. | Existing source immutability requirement; strengthens final boundary coverage. |

## Remainder inspection and retained controls

Static source/contract observations confirm current row families still52 other/29 reaction/66 displacement/45 rotation/180 force/180 moment/278 stress,830rows; unit set and two diagnostic-work exclusions match the fixture. Those existing assertions, result mapping/filter/page checks, reference selection, proposal/queue clear, model-history/lifecycle, local/native handoff disclosures and all no-conversion/claim/privacy checks remain intact. The headless raw-dimension witness applicability correction from Repair01 remains distinct from semantic stress witnesses. Public-report/native/handoff redaction assertions remain unchanged; legacy-only commented or privacy-unreachable raw-packet blocks are preserved, not re-enabled or used as Current evidence. The numerical comparison tolerance assertion was not changed. No production source, schemas, fixtures or legacy maintained tests changed.

Review artifacts: `_run_records/incremental.diff`, `SOURCE_FREEZE.json`, exact before/after and combined fourteen-file snapshots/diff, `BOUNDARY_CHECK.json`, `STATIC_SOURCE_OBSERVATIONS.json`. Ready for fresh bounded backcheck, then parent-owned single-case Vitest and TypeScript checks. A pass has not been observed for this revision.
