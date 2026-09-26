# TASK S5 — TypeScript readers and the minimum text-only UI

Read `_COMMON.md` first. Desktop app: `P/apps/desktop` (`npm` scripts in its `package.json`; `export PATH=/opt/node24/bin:$PATH`).

## Assignment (DESIGN §6 TS rows and §7; S1_INTERFACE §9–§10)
1. `src/features/results/numericalResultQuality.ts` and `resultSemantics.ts`: register preview-physics-1 (`sourceContract` value e.g. `'preview_physics'`, the table import, sha pin `ae55503d…`), first-match `source_basis` lookup as in S1 §1.
2. New `src/features/results/previewPhysicsEvidence.ts`: every S1 §9 check; wire it wherever `physicsResultEvidence` is used for admission (`services/analysisRunCompatibility.ts` preview-evidence check, `services/previewService.ts`).
3. New `src/features/results/knownSemanticLimitations.ts`: the frozen notices N-P1, N-SB, N-SB-MIXED, N-HEADLINE*, N-REPORT, N-RULE-RETIRED, N-INTENSIFIED and the gate reasons, verbatim from S1 §10, plus the static fresh-identity set and `standingReason`.
4. `src/features/results/sourceBlockRecovery.ts`: ordinary-case standing (non-composite only). `src/features/workspace/resultsSessionState.ts`: Current only for fresh identities with Current standing.
5. `services/ruleCheckService.ts`: pre-check mirroring `rule_binding_refusal`, showing N-SB before invoking; retired bindings show N-RULE-RETIRED.
6. `features/report/{reportPackageRequest.ts, ReportPanel.tsx}`: report unavailable for every fresh identity with N-REPORT; its SIF section no longer shows SIF×k rows as stress.
7. Text-only UI (no authoring, model or layout change): `ResultsPanel.tsx`, `ComparisonPanel.tsx`, stress-neutral and result-export panels, Handoff, LocalFeaHandoff and NativePackage: headline label and withheld reason; precision-1 notice; N-SB on all-selected source-blocks-1 including in the export UI for Current exports (N-A: UI only, never a document or manifest field); gate reasons where combination rows would appear; mixed source-blocks standing reason; intensified label; new kinds displayed with their units.
8. `src/test/nativeMechanicsReplay.ts`: add a `preview` profile over `P/fixtures/model_operations/precision_connected_ui_model.json` and `P/fixtures/results/preview_physics_connected_{sparse,dense}.json` (the manager generates these; until then write the code and leave the fixture tests pending). Keep the precision profile as historical replay.
9. Tests: new unit tests for all of the above; update the fixture-using tests named in DESIGN §6 (`previewService.test.ts`, `rendererIntegration.test.tsx`, `currentResultUnitPolicy.test.tsx`, `sourceBlockRecovery.test.ts`, `physicsResultExport.test.ts`) and the e2e specs `r2-smoke.spec.ts`, `gui-workflow-validation.spec.ts` where they assume precision-1 is Current or read retired kinds. TS tamper tests as DESIGN §9.2 (same list as S3).

## Write boundary
`P/apps/desktop/src/**` and `P/apps/desktop/e2e/**`. `src/types.ts` only if a type is missing. Not `src-tauri`, not `dist`, not fixtures.

## Checks
In `P/apps/desktop`: typecheck (`npx tsc --noEmit` or the package script), `npm test` (vitest) — full run once at the end — and the e2e specs you touched if the headless browser works here (say if not). Report counts and failures verbatim.
