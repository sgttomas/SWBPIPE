---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C3-LIBGUI-001
timestamp: 2026-06-13T21:35:00-0600
completed: 2026-06-13T22:45:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C3-LIBGUI-001 — private library manager GUI (Phase C3 wizard/panel slice)

## Tranche and authority basis

- Tranche: completion-plan Phase C item **C3** (private library management GUI),
  **GUI slice** — the import-wizard panel + workspace section + list panel that
  the three prior C3 slices explicitly handed off. Selected per the
  `_COORDINATION.md` Application Integration And Issuance Loop step 3.1 (earliest
  unblocked R3/Phase C dependency-spine item: C2 done → C3 → C4), continuing the
  C3 sequence after the foundation crate (`TP-C3-IMPORTVALIDATE-001`), the
  validation seam (`TP-C3-IMPORTCMD-001`), and the local store
  (`TP-C3-LIBSTORE-001`). Pre-approved unblocked completion-plan item; not
  blocked by any human decision (`D-02b` gates only C2 expression text syntax).
  Replicates the C2 rule-pack **manager GUI** pattern
  (`RulePackManagerPanel`, `TP-C2-EDITOR-001`).
- Authority: **DEL-03-07** (Public/private library import provenance checker;
  CHECKING) for the import-boundary verdict the GUI surfaces; PRD §13
  (material/section/component libraries), §13.5 (import warnings —
  blocking-vs-advisory display), §14.6 / §14.1 (library manager workspace
  surface), §12.4 (private-by-default), FR-022 (private libraries). IP boundary
  (DEL-01-02 / IP_AND_DATA_BOUNDARY.md: protected content quarantined and never
  used; private data never redistributed), OPS-K-PRIV-1 (local-only),
  OPS-K-AUTH-1 (software findings only), `DEC-036` (refuse-to-store policy the
  GUI surfaces honestly).
- Regression gate (loop step 3.2): full desktop Vitest green at the start HEAD
  (`979ab9ece`) with the wasm engine prebuilt; 317/317 before this slice. No
  regression; this is new in-stage C3 scope.

## Scope boundary (what this slice is / is not)

In scope: the **GUI** — a `LibraryManagerPanel` React component and a new
"Libraries" workspace section, replicating the C2 `RulePackManagerPanel`:
library-kind + intended-visibility selectors, an import-document JSON textarea,
a built-in invented private starting template, validate (`validateLibraryImport`)
with the **PRD §13.5 blocking-vs-advisory findings display**
(`partitionLibraryImportFindings`), save (`saveLocalLibrary`) with the **DEC-036
refuse-to-store surfacing** (`stored:false` + findings), and a project-scoped
list panel (`listLocalLibraries` / `openLocalLibrary` / `deleteLocalLibrary`),
plus the private-data + professional boundary notice. **Not** in scope: external
file-format parsing (the manager consumes an already-parsed document); the
**rule-pack ↔ library reference wiring** (a separate C3 slice — it couples the
C2 and C3 subsystems); the public-import maintainer-review workflow beyond the
validation preview. No backend change — all four Tauri commands and the typed
service routes already landed in the prior C3 slices.

## Changes

1. **`apps/desktop/src/services/libraryImportService.ts` — starting template
   helper.** Added `buildInventedLibraryImportTemplate(libraryKind)` returning a
   minimal **private-classified, provenance-complete** library document per kind
   (`material`/`section`/`component`), mirroring the desktop backend's own
   canonical accepted payload (`storable_material_payload` in src-tauri tests):
   `privacy_class: "private_user_data"`, all seven required provenance fields
   present, `redistribution_status: "private_only"`, an empty record array. Under
   a private import it validates to `PRIVATE_LOCAL_ONLY` with no findings and is
   storable — the GUI's happy-path starting point the user edits. Invented,
   non-engineering, carries no real values. Mirrors the C2
   `buildDraftRulePackDocument` convenience.
2. **`apps/desktop/src/features/library/LibraryManagerPanel.tsx` (new).** The
   manager component: project-scope banner; kind/visibility selectors with a
   note that visibility shapes the *validation preview* only and that **save
   always persists to the private local store and stores only an accepted
   private import (DEC-036)**; load-invented-sample / refresh-list actions; a
   project-scoped list with per-entry open/delete; the import-document textarea
   with validate / save / discard; an `Import validation` block that renders the
   §13.5 split into a **Blocking / quarantine** group and an **Advisory** group
   (each finding showing code / severity / path / message / remediation) plus the
   backend `professional_boundary_notice`; and a private-data + professional
   boundary note. An `inFlight` busy guard disables the textarea, selectors, and
   every action while a backend request is awaiting (mirrors the C2 slice-1
   clobber fix). Backend errors surface as an explicit `LIBRARY-IMPORT-BACKEND-
   ERROR (<action>)` status rather than a silent unhandled rejection.
3. **`apps/desktop/src/App.tsx` — workspace integration.** New `"libraries"`
   `WorkspaceSectionId`, `WORKSPACE_SECTIONS` entry, and section render
   (`LibraryManagerPanel`), placed **immediately before** the rule-pack manager
   (journey order: operations → loads → libraries → rule-packs → solve) because
   both are private local-only asset managers and rule packs reference imported
   library allowables. Stale comment about "material/component library editors
   are intentionally absent" updated — that surface is now implemented.
4. **Tests.**
   - `apps/desktop/src/features/library/LibraryManagerPanel.test.tsx` (new, 9):
     scope banner with/without a project; invented-sample load per kind +
     discard; the browser-preview desktop-only diagnostic for validate/list
     (no `invoke` called); save blocked with an honest reason when no project;
     invalid-JSON honesty; boundary note (`DEC-036`, never-committed,
     never-a-legal-claim). Two **desktop-mode** tests set `__TAURI_INTERNALS__`
     and mock `invoke`: the §13.5 partition rendering (2 blocking codes in the
     blocking group, 1 advisory code in the advisory group, not cross-listed)
     and the **DEC-036 refuse-to-store** surfacing (`stored=false` + refuse note
     + blocked validation riding through to the display). Plus a template-shape
     unit test (private-by-default, all 7 provenance fields, per kind).
   - `apps/desktop/e2e/r2-smoke.spec.ts` (extended, 1 test, both viewport
     projects): navigate to the Libraries section, assert the boundary note
     (`DEC-036`, never-committed) and `local SQLite only` scope, load the
     invented material sample through visible controls and assert the private
     classification, switch the kind selector to `component` and reload, then
     assert validate/refresh route to the honest `LIBRARY-IMPORT-BACKEND-
     DESKTOP-ONLY` seam in browser preview, and discard. Mirrors the existing
     rule-pack manager e2e test exactly.
5. **`apps/desktop/SMOKE.md`** — TP-MAC-155 GUI entry (the rendered surface; UI
   evidence is the Playwright spec extension + the Vitest panel suite per the H4
   posture, plus a live-browser confirmation).

## Validation

- `npm test` (desktop Vitest, wasm prebuilt) — **326/326** across 16 files
  (9 new in `LibraryManagerPanel.test.tsx`). No regression (317 → 326).
- `npm run build` (desktop, `tsc -b && vite build`) — clean.
- `npm run test:e2e -- --grep "library manager"` — **2 passed** (chromium-desktop
  1440×920 and chromium-compact lanes).
- Live-browser confirmation (dev server, real Chrome via the preview tools):
  the Libraries nav activates the section; the panel renders project-scoped
  (`project:invented-loop-01`); load-sample populates the textarea with a
  private-classified material library and the private-by-default status message.
- No Rust/Python change in this slice, so no `cargo`/`pytest` surface is owed;
  the backend commands and the crate↔Python parity guard from the prior C3
  slices are unchanged.
- DEC-025 five-surface evidence sweep at the committed HEAD; summary committed
  alongside this tranche.
- Evidence posture (H4): a user-visible desktop surface, so the default UI
  evidence is the Playwright e2e spec extension (real browser, both viewports)
  plus the new Vitest component suite at/above the slice's coverage pattern
  (the §13.5 display and refuse-to-store paths are exercised via mocked `invoke`
  because that backend is desktop-only — the documented reason browser e2e does
  not cover accept/store outcomes; those are covered by the src-tauri Rust tests).

## Pre-commit adversarial review and dispositions

Self-review across four lenses (UI correctness; IP-boundary/policy honesty;
test non-vacuousness; scope):

- **UI correctness.** The `inFlight` guard disables the textarea, selectors, and
  all actions during a backend request so an async `open`/`save` response cannot
  clobber a mid-request edit. The list is project-scoped and cleared on project
  change (the scope banner cannot contradict a stale list). The validation block
  reads from whichever action ran last (validate or save), so the refuse-to-store
  save surfaces the blocking findings, not a stale accepted state.
- **IP-boundary / policy honesty.** Save never claims to store a blocked import:
  the status shows `stored=<bool>` and, when false, the explicit `DEC-036
  refuse-to-store` note; the §13.5 split labels blocking/quarantine as blocking
  acceptance and review_required as advisory. The boundary note states
  local-only, never-committed/transmitted/bundled, refuse-to-store, and
  software-findings-only (no legal/redistribution/certification/approval claim).
  The visibility selector is labeled "validation preview" with an explicit note
  that save is always private — no silent visibility coupling.
- **Test non-vacuousness.** The desktop-mode tests assert the specific finding
  codes land in the correct §13.5 group and are *not* cross-listed, and assert
  the exact `stored=false` + refuse-note text; the browser-preview tests assert
  `invoke` is never called. A rename, a dropped field, or a mis-partition reddens
  a test. The e2e drives only visible controls (selectOption / click / fill),
  exactly as a human would.
- **Scope.** No backend change; no rule-pack↔library wiring (hand-off below); no
  file parsing. The template helper mirrors the backend's own accepted payload
  rather than inventing a new contract, so it cannot drift into a shape the
  validator rejects on the happy path.

## Residuals and hand-offs (next Phase C work)

- **C3 wiring:** rule-pack ↔ library reference wiring (let an authored rule pack
  reference an imported library's allowables/records). Couples C2 and C3; its own
  slice.
- **C3 GUI follow-ons (optional):** richer import affordances (file picker /
  paste-assist) beyond the JSON textarea; surfacing the stored library's
  documents into the model-authoring inspectors (so an imported material/section
  is selectable when authoring entities).
- **C4:** end-to-end rule checks on authored models that consume imported
  libraries (`USER_RULE_CHECKED` / `USER_RULE_FAILED` / `RULE_INPUTS_INCOMPLETE`
  driven from solves of user-authored models).
- Parity discipline (carried from `TP-C3-IMPORTVALIDATE-001`): this slice adds
  no validation semantics — it surfaces the crate's verdict verbatim through the
  existing seam; the crate ↔ Python parity guard remains the cross-language
  anchor.
