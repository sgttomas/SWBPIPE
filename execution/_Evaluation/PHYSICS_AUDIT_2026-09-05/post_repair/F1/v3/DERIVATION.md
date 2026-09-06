# Fixture derivation before V3 edits

Read previewService.ts:425–494, PipeViewport.tsx:2265–2361, SolvePanel.tsx:26/320, DiagnosticsPanel.tsx:22–25 and ReportPanel.tsx:324. Independently enumerated JSON rows with the same explicit selection/matching rules, without importing production calculations.

Comparison: left load L-100 rows unmatched by first existing equal-unit source edge of combination C-OPER-ALT grow 18 to 23. Five displacement magnitude source rows (nodes N-100 through N-140) are newly unmatched because repaired combination magnitudes reference their combination UX/UY/UZ components, rather than the primitive magnitude rows. All right rows remain matched; no new rows were introduced.

Deformation: the maximum displacement_magnitude across model node rows is combination C-OPER-ALT / node N-140, 4.927109 mm (old 4.567557), with complete signed components; model nodes stay five.

New fixture adds one warning SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED for support CE-120 with no declared translational acting DOF; no diagnostic removed. Raw fixture is 31 diagnostics (11 warning,20 info), formerly30 (10 warning,20 info). Model contributes1 warning; knowledge contributes1 warning and1 info. Solve readiness therefore32 (12 warnings), diagnostics panel34 (13 warnings,21 info). Filter result:stress:pipe-P-130 still matches one existing combination diagnostic; denominator34. No new friction law is implied.

Other result-export diagnostic literals are not blindly changed: their expected source is raw result31, rather than readiness32. Any separately failing assertion is investigated in its consumer context.

Follow-on derivation: SolvePanel diagnosticsFor aggregates model+result (32); ResultExportPanel:143, HeadlessRunnerPanel:116, AdapterFrameworkPanel:101 and NativePackagePanel:202 explicitly aggregate model+result, so their diagnostic count/length assertions become32. ReportPanel:324 aggregates model+knowledge+result, so report/handoff totals34 and warnings13; info21 unchanged. Within the same authorized case dormant redaction-protected expectation blocks are synchronized too. Unrelated result-filter count33 remains untouched.

The same case also displays the already-derived comparison unmatched count23 in its summary/export. Its support S-120 comparison quantity is raw_delta (ComparisonPanel.tsx:113), so generated right957.220159 minus left765.776127 =191.444032 N, displayed191.444; prior literal192.514 replaced while retaining existing 3-digit tolerance. No tolerance change.
