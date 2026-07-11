# MEMORY - DEL-08-05 Report Protected-Content Linter

## 2026-06-18 - TP-UNITS-BTAIL-EXPORTREVREPORTLINTUNITS-001 supporting report-lint evidence

- Supporting role for Phase B-tail Export Safety Review matrix cleanup:
  Export Review now references the existing
  `unit-policy-evidence:report-lint-public-surfaces` packet evidence when it
  classifies `report_protected_content_lint` as unit-evidence-required.
- Report Content Lint remains an inventory source with 44 unit-policy targets,
  two conversion-witness targets, `lint_performs_conversion=false`,
  `lint_asserts_target_format_compatibility=false`, and
  `clean_scan_is_clearance=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVREPORTLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-270; completion log entry; primary DEL-12-02
  run record and supporting DEL-02-02 run record.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction certification, target-format compatibility assertion,
  target writer, manifest-level unit conversion, protected standards content,
  private payload, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Report Content Lint inventory: the report
  linter now includes `PropertyInspector.tsx` as an explicit public target
  and records `property-inspector-unit-validation-surface` in
  `unit_policy_evidence.target_refs`.
- `data-testid="report-lint-unit-policy"` now reports 40 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
- Static report-lint target count is now 44; solved report packets include 45
  targets after the generated preview-report JSON target is appended.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-257; completion log entry; supporting
  DEL-07-02, DEL-16-02, and DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render; focused Property
  Inspector create-intent App tests 4/4; focused R2 Playwright smoke 2/2;
  full desktop Vitest 399/399; single-worker R2/R3 Playwright smoke 18/18;
  and desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, Property
  Inspector behavior, operation validation/application, unit-conversion API,
  DEC-018 catalog constant, schema dimension enum, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Report Content Lint inventory: the report
  linter now includes `LoadCaseManagerPanel.tsx` as an explicit public target
  and records `load-manager-unit-validation-surface` in
  `unit_policy_evidence.target_refs`.
- `data-testid="report-lint-unit-policy"` now reports 39 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
- Static report-lint target count is now 43; solved report packets include 44
  targets after the generated preview-report JSON target is appended.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-256; completion log entry; supporting
  DEL-07-02, DEL-05-01, DEL-05-02, DEL-16-02, and DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render; focused load/unit
  App tests 26/26; focused R2 Playwright smoke 2/2; full desktop Vitest
  399/399; single-worker R2/R3 Playwright smoke 18/18; `git diff --check`;
  and desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, load-case
  schema, primitive-load or combination behavior, operation validation,
  operation application, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-LIBRARYLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Report Content Lint inventory: the report
  linter now includes `LibraryManagerPanel.tsx` as an explicit public target
  and records `library-unit-helper-surfaces` in
  `unit_policy_evidence.target_refs`.
- `data-testid="report-lint-unit-policy"` now reports 38 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
- Static report-lint target count is now 42; solved report packets include 43
  targets after the generated preview-report JSON target is appended.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LIBRARYLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-255; completion log entry; supporting
  DEL-07-03, DEL-03-01, DEL-03-02, and DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render; focused
  LibraryManagerPanel Vitest 9/9; focused R2/library-manager Playwright smoke
  4/4; full desktop Vitest 399/399; single-worker R2/R3 Playwright smoke
  18/18; `git diff --check`; and desktop production build with the existing
  Vite large-chunk warning.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, library schema,
  library import storage, private-library payload handling, unit-conversion
  API, DEC-018 catalog constant, schema dimension enum, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-RULEPACKEXPRLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Report Content Lint inventory: the report
  linter now includes `ExpressionComposer.tsx` as an explicit public target
  and records `rule-pack-expression-unit-policy` in
  `unit_policy_evidence.target_refs`.
- `data-testid="report-lint-unit-policy"` now reports 37 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULEPACKEXPRLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-254; completion log entry; supporting
  DEL-07-03, DEL-06-02, and DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render and local
  create/save/open round-trip tests; focused ExpressionComposer/
  RulePackManagerPanel Vitest 6/6; focused R2/rule-pack Playwright smoke 4/4;
  full desktop Vitest 399/399; single-worker R2/R3 Playwright smoke 18/18;
  and desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, rule-pack
  schema, expression grammar, writable text parser/syntax, evaluator
  behavior, backend validation, persistence, unit-conversion API, DEC-018
  catalog constant, schema dimension enum, protected standards content,
  private data, lifecycle transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## 2026-06-18 - TP-UNITS-BTAIL-RULEPACKDECLLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Report Content Lint inventory: the report
  linter now includes `DeclarationsEditor.tsx` as an explicit public target
  and records `rule-pack-declarations-unit-policy` in
  `unit_policy_evidence.target_refs`.
- `data-testid="report-lint-unit-policy"` now reports 36 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULEPACKDECLLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-253; completion log entry; supporting
  DEL-07-03, DEL-06-01, and DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render and local
  create/save/open round-trip tests; focused RulePackManagerPanel/
  DeclarationsEditor Vitest 5/5; focused R2/rule-pack Playwright smoke 4/4;
  full desktop Vitest 399/399; single-worker R2/R3 Playwright smoke 18/18;
  `git diff --check`; and desktop production build with the existing Vite
  large-chunk warning.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, rule-pack
  schema, expression grammar, parser/text syntax, evaluator behavior, backend
  validation, persistence, unit-conversion API, DEC-018 catalog constant,
  schema dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-DIAGNOSTICLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Report Content Lint inventory: the report
  linter now includes `DiagnosticsPanel.tsx` as an explicit public target and
  records `diagnostic-unit-context` in `unit_policy_evidence.target_refs`.
- `data-testid="report-lint-unit-policy"` now reports 35 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-DIAGNOSTICLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-252; completion log entry; supporting
  DEL-07-07 and DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render, diagnostic detail,
  and local create/save/open round-trip tests; focused Playwright R2 smoke
  2/2; focused diagnostic Playwright 2/2; full desktop Vitest 399/399;
  single-worker R2/R3 Playwright smoke 18/18; `git diff --check`; and
  desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, diagnostic
  schema, diagnostic interpretation behavior, solver behavior, result values,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-RULECOMPLETENESSLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Report Content Lint inventory: the Rule-Check
  Completeness panel now carries explicit rule-input unit-policy evidence, and
  the report linter records `RuleCheckPanel.tsx` as
  `rule-completeness-unit-policy`.
- `data-testid="report-lint-unit-policy"` now reports 41 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  Static report-lint target count is 45; solved report packets include 46
  targets after the generated preview-report JSON target is appended.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULECOMPLETENESSLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-258; active plan entry; supporting
  DEL-06-03, DEL-07-04, DEL-05-04, DEL-08-03, and DEL-02-02 run records.
- Validation passed: focused App render test, focused solved-results App test,
  full desktop Vitest 399/399, desktop production build with the existing Vite
  large-chunk warning, focused R2 Playwright 2/2, and single-worker Playwright
  18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, rule evaluator
  semantics, required-input checker behavior, analysis status semantics,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-RULECHECKLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Run Rule Checks report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 34 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Run Rule Checks target ref with
  `unit_policy_surface_id=rule-check-unit-binding-policy`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused RuleCheckRunPanel Vitest 18/18, focused
  R2/run-check Playwright smoke 4/4 configured project tests, full desktop
  Vitest 18/18 files and 399/399 tests, desktop production build with the
  existing Vite large-chunk warning, and single-worker R2/R3 Playwright smoke
  18/18.
- Boundary preserved: no rule-pack schema, expression grammar,
  parser/text-syntax, backend completeness/evaluator behavior, rule-pack
  persistence, solver behavior, report-linter protected-content semantics,
  legal clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-COMPARISONLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Comparison workspace report-lint inventory:
  the Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/comparison/ComparisonPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 33 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Comparison workspace target ref with
  `unit_policy_surface_id=comparison-unit-policy`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no comparison delta math, result mapping, tolerance
  profile, default tolerance, solver behavior, report-linter
  protected-content semantics, legal clearance, redaction controls, target
  writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-RENDEREDREPORTLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Rendered Report report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/report/RenderedReportPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 32 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Rendered Report target ref with
  `unit_policy_surface_id=rendered-report-unit-basis`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no report renderer route, canonical hash policy,
  export-gate policy, report content, print/PDF behavior, report-linter
  protected-content semantics, legal clearance, redaction controls, target
  writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-RESULTVIEWLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Results viewer report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/results/ResultsPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 31 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Results viewer target ref with
  `unit_policy_surface_id=result-unit-policy`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no result math, result filtering, pagination semantics,
  selected-result interpretation, solver behavior, comparison delta math,
  tolerance profile, default tolerance, report-linter protected-content
  semantics, legal clearance, redaction controls, target writer
  compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-OPAPPLYLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Apply Operations report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/operations/OperationApplyPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 30 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Apply Operations target ref with
  `unit_policy_surface_id=operation-apply-unit-policy`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no operation application, acceptance semantics, durable
  audit persistence, receipt schema, operation validation, report-linter
  protected-content semantics, legal clearance, redaction controls, target
  writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-KNOWLEDGELINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Design Knowledge report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/knowledge/KnowledgePanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 29 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Design Knowledge target ref with
  `unit_policy_surface_id=knowledge-unit-context`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no design-knowledge schema, provenance model, result
  values, solver behavior, report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-RUNAUDITLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Run Audit report-lint inventory: the Report
  Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/run-audit/RunAuditPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 28 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Run Audit target ref with
  `unit_policy_surface_id=run-audit-units`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no analysis-run schema, hash policy, solver behavior,
  result values, report-linter protected-content semantics, legal clearance,
  redaction controls, target writer compatibility, unit-conversion API,
  DEC-018 catalog constant, schema dimension enum, protected standards
  content, private data, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-SOLVEJOBLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Solve execution report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/solve/SolvePanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 27 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Solve execution target ref with
  `unit_policy_surface_id=solve-job-unit-policy`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no solve execution behavior, analysis-run hash
  generation, cancellation/progress semantics, backend job behavior,
  report-linter protected-content semantics, legal clearance, redaction
  controls, target writer compatibility, unit-conversion API, DEC-018 catalog
  constant, schema dimension enum, protected standards content, private data,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-OPLEDGERLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Operation Review Ledger report-lint inventory:
  the Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/operations/OperationLedgerPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 26 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the Operation Review Ledger target ref
  with `unit_policy_surface_id=operation-ledger-unit-policy`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest rerun 399/399 after an order-sensitive
  Operation Ledger status mismatch was confirmed passing in isolation,
  desktop production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-ADAPTERFRAMEWORKLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Adapter Framework report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 25 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the adapter framework target ref with
  `unit_policy_surface_id=adapter-framework-units`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest rerun 399/399 after an isolated
  transient DeclarationsEditor timing failure was confirmed passing, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-HEADLESSRUNNERLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Headless Runner report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 24 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the headless runner target ref with
  `unit_policy_surface_id=headless-runner-unit-witnesses`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 399/399, desktop production build with
  the existing Vite large-chunk warning, and single-worker R2/R3 Playwright
  smoke 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-HANDOFFLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Handoff Package report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/handoff/HandoffPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 23 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the handoff target ref with
  `unit_policy_surface_id=handoff-unit-witnesses`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 399/399, desktop production build with
  the existing Vite large-chunk warning, and single-worker R2/R3 Playwright
  smoke 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-RESULTEXPORTLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Result Export report-lint inventory: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/result-export/ResultExportPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 22 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the result-export target ref with
  `unit_policy_surface_id=result-export-unit-witnesses`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 399/399, desktop production build with
  the existing Vite large-chunk warning, and single-worker R2/R3 Playwright
  smoke 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-DIFFPREVIEWLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Operation Diff Preview report-lint inventory:
  the Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 21 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the diff-preview target ref with
  `unit_policy_surface_id=operation-diff-unit-witnesses`.
- Validation passed: focused App Vitest workspace-render and queued-intent
  selected tests, focused Chromium desktop R2 smoke, full desktop Vitest
  399/399, desktop production build with the existing Vite large-chunk
  warning, and single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-NATIVEPACKAGEUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-17-03 native package unit-inventory tranche: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/native-package/NativePackagePanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 19 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the native package target ref with
  `unit_policy_surface_id=native-package-unit-witnesses`.
- Validation passed after updating stale target-count assertions and
  restoring one scenario-specific export-review lint count assertion:
  focused App Vitest selected tests, full desktop Vitest 399/399, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## Current Implementation

2026-05-02 implementation from sealed dispatch brief
`execution/_Coordination/DEV-001_DISPATCH_DEL-08-05.md`:

- Added `schemas/report_protected_content_linter.schema.yaml` as a strict-JSON
  JSON Schema 2020-12 contract for deterministic report protected-content
  linter configuration and findings.
- Added `core/reporting/protected_content_linter/` as a dependency-free Rust
  support crate for caller-supplied public-surface text scanning and
  deterministic finding generation.
- Added invented/synthetic linter fixtures under `fixtures/report_lint/invented/`.
- Added `tests/test_report_protected_content_linter.py` for schema and fixture
  contract checks.
- Updated focused `docs/SPEC.md` and `docs/TYPES.md` sections for the linter
  boundary.

## Reconciliation Basis

`execution/_Reconciliation/Reconciliation_Run_Summary_2026-05-02_DEL0805_CANDIDATE_E0621.md`
keeps `DAG-001-E0621` as `CANDIDATE` and non-gating. The implementation uses
invented/synthetic linter fixtures and does not depend on actual `DEL-11-04`
educational example models.

## Guardrails

- No protected standards text, protected tables, protected examples,
  proprietary formulas, private project data, private rule-pack payloads,
  private library content, or real secrets are used.
- The linter output is heuristic review evidence only; it is not legal
  clearance, security sufficiency, professional approval, certification,
  sealing, endorsement, authentication, or code-compliance proof.
- Private user surfaces are skipped by default unless a caller explicitly
  opts into scanning.
- CI provider, release policy, redaction/export controls, quarantine file
  movement, GUI/CLI/API/adapter integration, and final legal review workflow
  remain `TBD`.

## Verification

Implementation verification should include:

- `python3 tests/test_report_protected_content_linter.py`
- `cargo fmt --manifest-path core/reporting/protected_content_linter/Cargo.toml -- --check`
- `cargo test --manifest-path core/reporting/protected_content_linter/Cargo.toml`
- adjacent report contract tests
- `git diff --check`

## Open Items

- Lifecycle transition, local dependency mirror annotation, implementation
  evidence registration, blocker queue refresh, staging, and commit require
  separate human authorization.

2026-05-02 closeout authorization:

- Set lifecycle display state to `CHECKING`.
- Annotated active non-architecture local dependency rows `DAG-001-E0529`
  through `DAG-001-E0531` as `SATISFIED`.
- Registered working-tree implementation evidence for `DEL-08-05`.
- Rebuilt the blocker queue at 67 unblocked / 6 blocked before commit-backed
  evidence promotion.
- Implementation and closeout alignment committed as
  `69adffa schema: add report protected-content linter`.
- Promoted implementation evidence to `COMMITTED` for commit `69adffa` and
  rebuilt the blocker queue at 68 unblocked / 5 blocked.
- `DEL-11-04` is newly implementation-unblocked; `DEL-09-05` still waits on
  `DEL-09-03`, and `DEL-10-04` still waits on `DEL-09-05`.

## 2026-05-11 TP-RECON-01 Reconciliation

Reconciled deliverable-local history for `DEL-08-05` from the TP-RECON-01
dispatch row and archived DEV/SCA evidence bundle. The committed evidence rows
record `DEL-08-05` as `COMMITTED` on 2026-05-02 at commit `69adffa`
(`schema: add report protected-content linter`), and `git show --name-status
69adffa` corroborates the schema, bounded Rust linter support crate, invented
fixtures, deterministic linter tests, focused docs updates, deliverable memory,
status, and dispatch closeout changes.

Archived lifecycle evidence carries `DEL-08-05` forward in `CHECKING`; this
reconciliation preserves that state and records no release gate outcome. The
archived dispatch records the `DAG-001-E0621` candidate edge as retained
non-gating because the implementation used invented/synthetic linter fixtures
rather than depending on `DEL-11-04` educational examples. Tranche B/C and
DAG-002 handoff evidence carry the same committed evidence hash forward.

Deferred scope remains the downstream CI/release workflow, redaction/export
controls, quarantine file movement, final legal-review workflow, GUI/CLI/API/
adapter integration, educational example publication, and actual private or
protected-content scanning. TP-RECON-01 made no code, schema, fixture,
procedure, dependency, or coordination-state changes for this deliverable.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/_REVIEW.md` and `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/Review_Findings.csv`.
- Package audit summary is `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_run_records/TASK_RUN_2026-05-16_pkg08_pkg02_downstream_audit.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-19 - TP-VERIFY-014C protected-content linter evidence check

TP-VERIFY-014C verified current protected-content linter evidence for parent
fan-in into `DEL-09-05`.

Validation:
- `python3 tests/test_report_protected_content_linter.py` passed.
- `cargo test --manifest-path core/reporting/protected_content_linter/Cargo.toml`
  passed with 4 tests and 0 failures.

Disposition:
- The linter evidence is citeable as deterministic heuristic public-surface
  protected-content screening evidence for report-template gate routing.
- It is not legal clearance, security sufficiency, professional approval,
  certification, sealing, endorsement, authentication, or code-compliance proof.
- CI/release policy, redaction/export controls, quarantine movement, and final
  human/legal review workflow remain governed downstream decisions.

Local run record:
- `_run_records/TASK_RUN_2026-05-19_TP-VERIFY-014C.md`

Boundary: this audit changed only this `MEMORY.md` and the local run record. It
did not change lifecycle state, CI workflows, release records, candidate rows,
blocker queues, implementation evidence, professional-boundary decisions, or
code-compliance decisions.

## 2026-06-03 - TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001 CHECKING transition
- Human approval accepted non-resolving DEV-001 evidence commits as migration-caused aberrations and approved lifecycle advancement to `CHECKING` for formal review.
- Evidence basis: `TP-CODE-EVIDENCE-AUDIT-001_2026-06-03` found current source/schema/fixture/test evidence and passing targeted/full-gate checks; `TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001_2026-06-03` reconciled the migration-era commit-pointer gap.
- Local `_STATUS.md`, DEV-001 blocker queue lifecycle displays, and DAG-005 deliverable display surfaces were aligned to `CHECKING` where applicable.
- Boundary preserved: this is review-readiness only; no `ISSUED`, release-readiness, external compatibility, code-compliance, protected-IP/private-data, or professional-engineering authentication claim is made.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-08-05`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-REPORTLINTUNITS-001 report-lint unit policy inventory

- Primary role for Phase B-tail Report Content Lint unit-policy inventory:
  the desktop report linter now exposes visible and exported inventory
  evidence for public report/export lint targets that carry unit-policy or
  target-format conversion-witness surfaces.
- `data-testid="report-lint-unit-policy"` reports 17 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The lint JSON exports `unit_policy_evidence` with
  `evidence_kind=public_surface_unit_policy_inventory`,
  `lint_performs_conversion=false`, and
  `lint_asserts_target_format_compatibility=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-REPORTLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-231; completion log entry; supporting
  DEL-02-02 run record.
- Validation passed: focused App Vitest 1/1 selected test, focused
  Playwright 1/1 Chromium desktop test, full desktop Vitest 399/399,
  single-worker R2/R3 Playwright smoke 18/18, `git diff --check`, and
  desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-STORAGEUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-02-05 project-storage unit-policy tranche: the
  Report Content Lint public unit-policy inventory now includes
  `apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx`.
- `data-testid="report-lint-unit-policy"` now reports 18 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
  The exported lint packet includes the storage-audit target ref with
  `unit_policy_surface_id=project-storage-unit-round-trip`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-STORAGEUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-234; completion log entry; primary
  DEL-02-05 run record and supporting DEL-02-02 run record.
- Validation passed: focused App Vitest initial storage, local
  create/save/open round-trip, and solved report-lint tests; focused
  Playwright 2/2 Chromium desktop tests; full desktop Vitest 399/399;
  single-worker R2/R3 Playwright smoke 18/18; `git diff --check`; and
  desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001 primary report-lint inventory evidence

- Primary role for Phase B-tail Report Content Lint inventory: the report
  linter now includes `ProjectValidationPanel.tsx` as an explicit public
  target and records `project-validation-unit-policy` in
  `unit_policy_evidence.target_refs`.
- `data-testid="report-lint-unit-policy"` now reports 20 unit-policy targets,
  two target-format conversion-witness targets, and `lint_conversion=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-237; completion log entry; supporting
  DEL-02-05 and DEL-02-02 run records.
- Validation passed: focused App Vitest workspace-render test after catching
  the missing explicit target entry, focused Playwright 1/1 Chromium desktop
  test, focused queued-intent App Vitest after restoring an over-broad
  export-review metadata assertion, full desktop Vitest 399/399,
  single-worker R2/R3 Playwright smoke 18/18, and desktop production build
  with the existing Vite large-chunk warning.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  validation-preflight semantics, project persistence semantics,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private data, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-AGENTPROPOSALUNITS-001 supporting report-lint evidence

- Supporting role for DEL-08-05: Report Content Lint now classifies the Agent
  Proposal panel as a public unit-policy surface.
- The lint packet now includes
  `apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx` and
  `agent-proposal-unit-policy`; visible unit-policy targets increase to 42
  while conversion-witness targets remain two and `lint_conversion=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-AGENTPROPOSALUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-260; completion log entry; primary
  DEL-16-04 run record and supporting DEL-16-02/DEL-16-03/DEL-12-02/DEL-08-04/DEL-02-02
  records.
- Validation passed: focused App proposal and queued-intent tests, full App
  test file 56/56, full desktop Vitest 399/399, desktop build with existing
  Vite large-chunk warning, focused R2 Playwright 2/2, and full single-worker
  Playwright 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, unit conversion,
  private payload, protected content, lifecycle transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001 supporting report-lint evidence

- Supporting role for DEL-08-05: Report Content Lint now inventories the
  Security Threat Model panel's unit-check no-bypass evidence.
- The lint packet now includes
  `apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx`
  as `security-threat-model-unit-policy`; visible unit-policy targets increase
  to 43 while conversion-witness targets remain two and
  `lint_conversion=false`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-261; completion log entry; primary
  DEL-12-05 run record and supporting DEL-12-02/DEL-02-02 records.
- Validation passed: full App test file 56/56, full desktop Vitest 399/399,
  desktop build with the existing Vite large-chunk warning, focused R2
  Playwright 2/2 after updating stale report-lint target-count assertions,
  and full single-worker Playwright 18/18.
- Boundary preserved: no report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility, unit conversion,
  private payload, protected content, lifecycle transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## 2026-06-18 - TP-UNITS-BTAIL-FIXTURELINTUNITS-001 primary fixture unit inventory evidence

- Primary role for DEL-08-05: Report Content Lint now inventories the
  invented `fixtures/product_preview` bundle as a public unit-policy target.
- The lint packet now includes `fixtures/product_preview` as
  `product-preview-fixture-unit-policy`; visible unit-policy targets increase
  to 44 while conversion-witness targets remain two and
  `lint_conversion=false`.
- The fixture target text records that the bundle carries explicit model
  quantities, rule-pack unit refs, mechanics-result units, and the active
  model unit system without modifying fixture payloads.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-FIXTURELINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-262; completion log entry; supporting
  DEL-02-02 run record.
- Validation passed: full App test file 56/56, full desktop Vitest 399/399,
  desktop build with the existing Vite large-chunk warning, focused R2
  Playwright 2/2, and full single-worker Playwright 18/18.
- Boundary preserved: no fixture data, report-linter protected-content
  semantics, legal clearance, redaction controls, target writer compatibility,
  unit conversion, private payload, protected content, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-07-10 - TP-E7-SCANEXT-001 DEC-058 scanner extension and scan-record tooling

- Under the D-20 ruling (`DEC-058`), the engine gained standards-table
  signature detection: designator-name/clause-label tokens within eight
  lines of a dense numeric grid fail toward
  `UnknownProvenanceReviewRequired` (`OPS-K-IP-3`, `HumanIpReview`),
  never silent pass, never a protected-content determination. Tokens are
  names/labels only; no standards content embedded.
- New engine CLI `protected_content_lint_cli` (JSON findings, zero new
  dependencies) and stdlib-only runner
  `tools/release/run_release_candidate_scan.py`: AC-1..AC-6 inventory walk
  with `not_applicable` recording, engine lint, IP-boundary §4 provenance
  check, security profile, quarantine hygiene, and an UNSIGNED
  `SCAN_<candidate>_<utc>_<commit12>.json` emitter with the owner
  sign-off block pending. The scan act, dispositions, and signature remain
  owner-only (`DEC-027`); `validation/evidence/releases/` stays absent
  until the owner scans.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E7-SCANEXT-001.md`;
  crate tests 15/15; `tests/test_release_candidate_scan.py` 14/14.
- Boundary preserved: no legal clearance, release-readiness claim,
  certification, sealing, authentication, code-compliance claim, protected
  standards content, private data, lifecycle transition, or new external
  dependency.
