---
run-id: WORKING_ITEMS_RUN_2026-06-12_TP-C2-EDITOR-001
timestamp: 2026-06-12T20:10:00-0600
completed: 2026-06-12T20:35:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop/src
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C2-EDITOR-001 — rule-pack manager GUI (Phase C2 slice 1)

## Tranche and authority basis

- Tranche: completion-plan Phase C item C2 — "Rule-pack editor GUI (PRD
  §14.5): authoring, validation, checksum/lifecycle surfacing from the
  existing lifecycle/checksum module" (`plans/PLAN_2026-06-10_prd_completion.md`
  §3 C2). This is the GUI slice that consumes the TP-C2-RPLIFE-001 backend
  seam.
- Authority: DEL-07-03 R-007 ("The rule-pack reference editor shall expose
  rule-pack identity, version, checksum, source notice, redistribution
  status, required inputs, and missing required-input findings"), R-008
  (mutations as application-service interactions, not domain bypasses),
  R-010 (surface missing data early; no silent defaults), R-011 (private
  data local/user-controlled, not committed/transmitted), R-012 (distinguish
  editable data / solve readiness / rule-check readiness / human review
  without implying compliance). PRD §14.5, §12.4 (private by default), §14.1
  (rule-pack manager surface). DEC-022 (AST-only authoring), D-02b
  AWAITING_RULING (no writable expression text syntax). OPS-K-PRIV-1,
  OPS-K-AUTH-1, OPS-K-RULE-3.

## Changes (app-owned slice; all under apps/desktop/src and e2e)

1. **`services/rulePackService.ts`** — typed frontend route over the six
   TP-MAC-147 Tauri commands (validate / compute checksum / save / open /
   list / delete). Browser preview short-circuits to an explicit
   `RULE-PACK-BACKEND-DESKTOP-ONLY` unavailable route (the unit-catalog /
   report-renderer precedent), never a synthesized fallback engine. Also
   provides `buildDraftRulePackDocument()` (a private-by-default,
   placeholder-only schema 0.2.0 / grammar 1.0.0 draft) and
   `stampChecksumIntoDocument()`.
2. **`features/rule-packs/RulePackManagerPanel.tsx`** — the "Rule Packs"
   panel: scope banner (project-scoped, local-only), new draft / discard,
   refresh / open / delete against the local list, raw document JSON editor
   (native canonical form; expressions are declarative-AST JSON per
   DEC-022), validate, compute-and-stamp checksum, validation-findings
   render, computed-checksum render, and a permanent boundary note (D-02b
   status, private-only handling, no-claims language).
3. **`App.tsx`** — new `rule-packs` `WorkspaceSectionId` + WORKSPACE_SECTIONS
   entry (placed between Load Cases and Solve, the journey position where
   user rule checks consume loads and feed the solve/check flow), section
   render, and import.
4. **Tests** — `RulePackManagerPanel.test.tsx` (8 Vitest cases: scope
   honesty, private-default draft, discard, the desktop-only diagnostic on
   validate/refresh, compute-checksum/save seam transitions, no-project save
   block, invalid-JSON honesty, boundary visibility, and the two service
   helpers). Playwright `r2-smoke.spec.ts` third test drives the section
   through visible controls at both viewports.

## Validation

- Vitest **251/251** (11 files; 8 new; the permanent dead-control audit
  `App.deadControls.test.tsx` passes — disabled draft buttons carry
  accessible `title` reasons, enabled buttons produce observable change).
- Playwright dev **6/6** (chromium-desktop 1440×920 + chromium-compact
  1280×800; new rule-pack journey test).
- Playwright dist **1/1**; `tsc -b` + `vite build` clean.

## Pre-commit adversarial review and dispositions

A three-lens review (react-correctness, governance-boundary,
evidence-honesty) ran before commit. The governance/evidence lenses aborted
on an infra model-access error, so the surfaced findings were judged
manually against the code rather than trusted from the partial run. Three
were legitimate and fixed in this tranche:

- **Unhandled desktop backend rejections** → every `invoke`-backed handler
  now wraps the await in try/catch and surfaces `RULE-PACK-BACKEND-ERROR`
  (honest, not silent). Browser mode never reaches the throw (services
  short-circuit), so this is a desktop-runtime safety net not exercised by
  the browser test lanes — recorded here rather than asserted.
- **Stale stored-pack list on project switch** → a `useEffect` keyed on
  `projectId` clears the list so it cannot contradict the scope banner.
- **Two vacuous e2e assertions** (compute-checksum and save write the same
  `action-status` text already present from validate) → removed from e2e;
  per-button seam coverage moved to Vitest where each transition is asserted
  from a distinct prior state.

Residual (not fixed; routed to the composer slice): a mid-request textarea
edit can be clobbered when an async response calls `setDraft`; a busy guard
belongs with the structured composer rework of this surface.

## Residuals and hand-offs (next C2 slices)

- **Structured AST expression composer** — replace raw-JSON expression
  editing with a tree/form composer (PRD §14.5 "Expression editor"); still
  no text syntax until D-02b is ruled.
- **Required-input / value-slot / load-combination form builders** — the
  remaining PRD §14.5 sub-surfaces (variable browser, allowable definition,
  load-combination mapping) as structured forms instead of raw JSON.
- **In-request busy guard** + per-finding inline surfacing on the document.
- **Engine-side rule evaluation on solved user models** (USER_RULE_CHECKED /
  USER_RULE_FAILED end-to-end) is C4, not this slice.

## Boundary review

- Private rule packs persist only in the local SQLite store (TP-MAC-147 v10
  table); the GUI never transmits or commits them; the draft template
  carries only placeholders and invented wording (no protected code
  equations, no standards values — PRD §14.5).
- Validation/checksum statuses are software findings; the panel's permanent
  notice states no professional, certification, sealing, authentication,
  approval, or code-compliance claim. No lifecycle state changed; DEL-07-03
  remains CHECKING.
