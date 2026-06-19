# SPEC — OpenPipeStress Workspace Redesign (v0.1, harness-independent)

**Status:** Draft for build. R3-exit critical path. Realizes the LOCKED design frame; does not relitigate it.
**Target app:** `chirality-piping/apps/desktop` (Tauri + React + Vite + Three.js).
**Source of truth for current behavior:** files cited inline as `file:line` (verified against the working tree at `chirality-piping/apps/desktop/src/`).
**Governance:** sanctioned as harness-independent v0.1 preparation under `DEC-042` (2026-06-18); on the R3-exit critical path (closes the A3 authoring-usability finding + F-4). The agent-panel seam is gated by `D-21`/`DEC-041` — reserved, not built (§10).
**Anchors verified:** all current-state `file:line` citations were grep-verified against the working tree on 2026-06-18. Component basenames used below resolve per this map:

| Basename (cited below) | Full path |
|---|---|
| `App.tsx`, `App.test.tsx`, `styles.css` | `src/` |
| `PipeViewport.tsx` | `src/features/viewport/` |
| `ModelTree.tsx`, `PropertyInspector.tsx` | `src/features/model-tree/` |
| `OperationApplyPanel.tsx` | `src/features/operations/` |
| `SolvePanel.tsx` | `src/features/solve/` |
| `DiagnosticsPanel.tsx` | `src/features/diagnostics/` |
| `ResultsPanel.tsx` | `src/features/results/` |
| `LoadCaseManagerPanel.tsx` | `src/features/load-cases/` |
| `RulePackManagerPanel.tsx`, `ExpressionComposer.tsx` | `src/features/rule-packs/` |
| `MissingDataBlockingPanel.tsx` | `src/features/missing-data/` |
| `RuleCheckRunPanel.tsx` | `src/features/rule-check/` |
| `LibraryManagerPanel.tsx` | `src/features/library/` |
| `RunAuditPanel.tsx` + evidence panels | `src/features/run-audit/`, `src/features/telemetry/`, … |
| `unitCatalogService.ts` | `src/services/` |

---

## 1. Problem & Target

### 1.1 The C5.7 failure (grounded)

The human packaged-GUI pass (C5.7, intended to close F-4 and A3) was **abandoned on usability grounds**. The verbatim verdict: *"a wall of evidence/telemetry strings, three competing navigation systems, the model a postage stamp."* This keeps **F-4** (completed human packaged-GUI journey) and **A3** (authoring-usability finding) OPEN, so **R3 is not exit-ready** (DEC-035 names F-4 + A3 as the two blocking residuals).

The verdict maps to three concrete, located defects in the current shell:

1. **Three competing navigations.** All three are rendered simultaneously inside `.workspace-dock`:
   - **Journey rail** — 8 steps, `App.tsx:1434-1452` (`<nav className="journey-rail">`), built from `JOURNEY_STEPS`.
   - **Workspace nav tabs** — 10 section buttons, `App.tsx:1057-1076` (`<nav className="workspace-nav">`), built from `WORKSPACE_SECTIONS` (`App.tsx:152-205`).
   - **Guided journey stack** — A12 (12-step) / R3 (6-step) tabbed sub-flows, `App.tsx:1469-1513`, with 6-column step grids (`styles.css:443`). All three drive the same `activeSection` state via different patterns; switching A12↔R3 tabs does not reset `activeA12StepId`/`activeR3StepId`/`r3JourneyState`, so journey state accumulates.

2. **Wall of strings.** Diagnostic intent buried in unformatted machine metadata at five rendering sites (full inventory in §6): storage capability string `App.tsx:931-937`; engine route `OperationApplyPanel.tsx:42-45`; unit-policy summary `OperationApplyPanel.tsx:46-48` + `operationApplyUnitPolicySummary` `OperationApplyPanel.tsx:174-196`; session-history summary `OperationApplyPanel.tsx:70-72`; outcome status string `OperationApplyPanel.tsx:155-159`; boundary strip enums `App.tsx:982-1003`; and the entire 7-panel "evidence" section `App.tsx:1347-1359`.

3. **Model a postage stamp.** The 3D canvas occupies the flexible center column (`grid-template-columns: 280px minmax(420px, 1fr) 380px`, `styles.css:204`) but the viewport **frame** is hard-clamped to `height: clamp(240px, 25vw, 325px)` (`styles.css:1007`). The dock below (`grid-template-columns: repeat(auto-fit, minmax(380px, 1fr))`, `styles.css:659`) plus the three navs out-weigh the canvas. The canvas also does not *read* as 3D: fixed camera with no orbit (`camera.position.set(7.6,7,8)`, `PipeViewport.tsx:126-127`), only a gentle auto-rotation (`PipeViewport.tsx:188`), a bare grid helper (`PipeViewport.tsx:139`), no axis triad, no view cube, no scale/ruler, no view controls.

### 1.2 Acceptance bar (A3 / F-4 / TP-MAC-141 → TP-MAC-189)

The redesign is "done" only when a human completes the **13-step packaged checklist** (TP-MAC-189, successor to TP-MAC-141) on a freshly built bundle and records PASS per step. The 13 steps: (1) launch + boundary status; (2) guided authoring entry visible with next-action; (3) New blank; (4–5) author 2 nodes → material → section → pipe → support → load case → primitive load → combination using invented rehearsal values, applied with `professional_approval=false`; (6) mechanics preview reaches `MECHANICS_SOLVED` with visible result rows; (7) report render with SHA-256; (8) save/reopen/re-solve; (9–10) R3 guide, load private library template, validate, local save; (11) draft private rule-pack via **structured composer only** (no writable text, DEC-037), validate, checksum, local save; (12) run rule-check after solve, **missing-input blockers shown**, no software code-compliance status; (13) quit clean (no network/daemon/telemetry/private-write/compliance claim).

PASS closes F-4 + A3 and unblocks C5.8 (R3 exit evidence package). The redesign must make every one of those 13 steps obvious and reachable through **one** navigation.

---

## 2. Target Information Architecture

The shell becomes a fixed 6-region frame. The **canvas is the hero**; everything compliance/telemetry/evidence is demoted to chrome.

```
┌───────────────────────────────────────────────────────────────────────┐
│ TITLE BAR  OpenPipeStress · project name · [New blank][Open][Save]      │  (slim, was topbar+project-toolbar)
├───────────────────────────────────────────────────────────────────────┤
│ RIBBON   Model ▸ Loads ▸ Analyze ▸ Results ▸ Rules ▸ Report             │  THE ONE NAV = workflow spine
├──────────┬─────────────────────────────────────────────┬──────────────┤
│ LAYOUT   │            MODEL CANVAS  (HERO)              │  INSPECTOR   │
│ GRID     │   axis triad · view cube · ref grid · scale │  deep single │
│ (tree +  │   selection handles · support/load glyphs   │  entity:     │
│  bulk    │                                             │  fields,     │
│  tabular)│                                             │  dual units, │
│          ├─────────────────────────────────────────────┤  provenance, │
│          │ COMMAND/SELECTION BAR (AutoCAD-style)        │  required ⚑  │
├──────────┴─────────────────────────────────────────────┴──────────────┤
│ STATUS BAR   Mechanics · Rule check · Professional   [Audit & boundaries ▸] [⚑ 3 Issues ▸]
└───────────────────────────────────────────────────────────────────────┘
                                            (future docked agent panel SEAM — D-21, not built here)
```

### 2.1 Ribbon (the one navigation)
Horizontal, always visible, six stops in workflow order: **Model → Loads → Analyze → Results → Rules → Report**. Each stop carries a state-derived completeness badge (derived from the same model/result/ruleCheck state the journey-rail currently reads at `App.tsx:1447-1448`). Clicking a stop sets the active workflow context and swaps the Layout grid's content set and the dock content. The ribbon *is* the spine — it replaces journey-rail + workspace-nav + guided-journey-stack. It carries a current-step caption strip beneath it (re-using the "current step" copy from `App.tsx:1453-1467`) so guidance survives without a separate rail.

Ribbon → underlying `WorkspaceSectionId` mapping (preserves existing panels, regroups them):
- **Model** → operations(apply queue) + libraries (materials/sections live in Layout grid + Inspector).
- **Loads** → loads.
- **Analyze** → solve (+ diagnostics, missing-data, rule-check-run feed the Issues home, not inline).
- **Results** → results.
- **Rules** → rule-packs (+ rule-check binding).
- **Report** → report.
- *(project, exports retained but reached from title-bar overflow / Report; evidence demoted to status bar — see §6.)*

### 2.2 Model Canvas (HERO)
Largest region; takes the remaining width AND grows vertically (remove the `clamp(...)` cap at `styles.css:1007`; the canvas fills its grid cell, min-height ~520px). Must *read as real 3D* — additions to `PipeViewport.tsx`:
- **Axis triad** (X/Y/Z gizmo, screen-corner overlay).
- **View cube** (clickable faces → set camera to ortho front/top/iso; replaces the fixed `camera.lookAt` at `PipeViewport.tsx:127`).
- **Orbit/pan/zoom controls** (replace the gentle auto-rotate at `PipeViewport.tsx:188`; auto-rotate removed).
- **Reference grid + scale bar** (upgrade the bare `grid()` helper at `PipeViewport.tsx:139`; show a length scale tied to `model.project.units.length`, `PipeViewport.tsx:73`).
- **Selection handles** on the picked entity (the current orange/teal recolor at `PipeViewport.tsx:162` becomes bounding handles).
- **Distinct support/load glyphs** — distinguish `supportMesh` vs `componentMesh` (`PipeViewport.tsx:167,172`) and add load glyphs (arrows for forces/moments) so loads are visible in 3D, not just in a panel.
- Keep the WebGL fallback (`PipeViewport.tsx:110-121`) and the click-to-pick → queue-intent affordance (`PipeViewport.tsx:202-209`).

The node/pipe **creation forms** currently stacked *below* the canvas (extending the frame; `PipeViewport.tsx:320+`) move OUT of the canvas — creation is driven from the command bar (§2.5) and edited in the Inspector. The canvas renders; it does not host long forms.

### 2.3 Layout Grid (CAEPIPE-style bulk tabular)
Left region. Replaces the single-select hierarchical `ModelTree` (`ModelTree.tsx`) as the *primary* entry surface for bulk work, while retaining its grouping/search. Two modes in one region:
- **Tree mode** (default, navigation): the existing 8 type groups — Materials, Sections, Nodes, Pipes, Supports, Components, LoadCases, Combinations (`ModelTree.tsx:130-237`) with keyword search (`ModelTree.tsx:94-105`).
- **Grid mode** (bulk tabular, NEW): a spreadsheet-style editable table for the active entity type (e.g. all Nodes with X/Y/Z columns, all Supports with restraint columns). Row edits fan out to `EditorOperationIntent`s (one intent per changed cell) through the existing `onQueueIntent` path (`App.tsx:504-513`). This is the CAEPIPE-style affordance the AUTHORING report flags as absent ("No bulk-table editor exists; all editing is single-entity"). **NEW components flagged in §4.**

### 2.4 Inspector (deep single-entity)
Right region. Refactor of `PropertyInspector.tsx`. For the selected entity shows: properties; the single-field **review-only edit intent** builder (`PropertyInspector.tsx:64-100` — field dropdown, proposed value, unit selector, rationale, validate/queue); **dual-unit display** (DEC-018, see §4.4); **provenance** (source_name/license/contributor/redistribution_status/review_status — required on every entity per DATA-MODEL report); and **inline "required" flags** (⚑) for missing required inputs (driven by the completeness/`RequiredInput` model). Create-section / create-material / create-support forms (`PropertyInspector.tsx:80-91`) stay here. Code-neutral done right: show *user value + provenance + a "required" flag* — never raw machine enum strings.

### 2.5 Command / Selection Bar (AutoCAD-style)
NEW thin bar under the canvas. Two halves:
- **Command input** — type or pick a command (`node`, `pipe`, `support`, `load`, `delete`, `material`, `section`). Issuing a command starts a canvas/Inspector capture flow that ends in a queued `EditorOperationIntent`. This replaces the in-canvas creation forms (§2.2) and gives the journey a single muscle-memory entry point.
- **Selection readout** — what's picked (id, type, label) + multi-select count (feeds Grid-mode bulk ops). Mirrors selection state already flowing through `setSelection` (`App.tsx:1008`).

### 2.6 Status Bar ("Audit & boundaries" home for demoted telemetry)
NEW bottom bar. Left: the **three analysis status pills** (Mechanics / Rule check / Professional acceptance), lifted verbatim from `SolvePanel.tsx:27-31`, now globally visible. Right: two controls —
- **"Audit & boundaries"** opener — collapses the entire former boundary strip (`App.tsx:982-1003`) + the 7-panel evidence section (`App.tsx:1347-1359`) + storage capability (`App.tsx:931-937`) into one drawer/dialog. Default state: a single compact summary chip ("Local · no network · no telemetry"); details on demand.
- **"⚑ Issues"** opener — count badge + opens the Issues diagnostics home.

### 2.7 Issues home (diagnostics)
Refactor/relocation of `DiagnosticsPanel.tsx`. One filterable list aggregating the three diagnostic sources already merged at `DiagnosticsPanel.tsx:20-24` (`model.diagnostics`, `knowledge.diagnostics`, `result.diagnostics`) plus operation-apply diagnostics (`OperationApplyPanel.tsx:165-169`) and rule-check missing-input blockers (the `RULE_INPUTS_INCOMPLETE` / `RequiredInput` findings from `MissingDataBlockingPanel.tsx` + `RuleCheckRunPanel.tsx`). Each row: severity icon, human message, affected entity link (jumps the canvas/Inspector selection). This is where the **missing-input blockers** for TP-MAC-189 step 12 surface — visibly, not buried in a string.

### 2.8 Agent panel SEAM (out of scope)
Reserve a right-edge dock slot in the frame grid, collapsed/empty, gated by D-21. **Do not build the panel.** See §10.

---

## 3. Current → Target Component Map

| Current component / file:line | Action | Target |
|---|---|---|
| `topbar` `App.tsx:911-925` + `project-toolbar` `App.tsx:927-980` | **refactor + merge** | Slim **Title bar** (project name + New blank/Open/Save buttons `App.tsx:943-962`). Storage metadata string (`App.tsx:931-937`) **removed** from here → status-bar Audit drawer. |
| `boundary-strip` `App.tsx:982-1003` (4 `BoundaryItem`) | **relocate** | Status-bar **Audit & boundaries** drawer (§2.6). |
| `journey-rail` `App.tsx:1434-1452` (`JOURNEY_STEPS`) | **remove** | Folded into the **Ribbon** (state-derived badges + current-step caption). |
| `workspace-nav` `App.tsx:1057-1076` (`WORKSPACE_SECTIONS` `App.tsx:152-205`) | **refactor** | The **Ribbon** (6 stops). `WORKSPACE_SECTIONS` regrouped per §2.1; `evidence`/`project`/`exports` demoted off the spine. |
| `guided-journey-stack` + A12/R3 tabs `App.tsx:1469-1513` | **remove** | Single ribbon spine + current-step caption. `GuidedWorkbench`/`A12AuthoringJourney`/`R3GuidedJourney` (`App.tsx:1378-1607`) deleted; `buildA12FlowState`/`buildR3FlowState` reduced to ribbon-badge derivation. |
| `modeling-workspace` grid `App.tsx:1006-1031`, `styles.css:202-208` | **refactor** | Frame grid: Layout grid \| Canvas (hero) \| Inspector, canvas un-clamped. |
| `PipeViewport` `PipeViewport.tsx:70-273`; viewport-frame clamp `styles.css:1004-1009` | **refactor (hero)** | Add triad/view-cube/orbit/scale/handles/load-glyphs; remove `clamp` cap (`styles.css:1007`) and auto-rotate (`PipeViewport.tsx:188`); move creation forms out (`PipeViewport.tsx:320+`). |
| `ModelTree` `ModelTree.tsx:12-301` | **refactor + extend** | Layout grid Tree mode (kept) + **NEW Grid mode** (bulk tabular). |
| `PropertyInspector` `PropertyInspector.tsx:14-150+` | **refactor** | Inspector with dual units + provenance + required flags. |
| `OperationApplyPanel` `OperationApplyPanel.tsx:5-150` | **refactor** | Apply queue UI kept; **all string lines removed**: engine status `:42-45`, unit-policy `:46-48`, session-history `:70-72`, outcome string `:155-159` → status bar / Issues / structured chips. |
| `review-apply-drawer` (EditorContract/DiffPreview/OperationLedger/AgentProposal) `App.tsx:1098-1145` | **relocate** | Stays a detail drawer under Model ribbon stop; AgentProposalPanel parked behind D-21 seam (collapsed). |
| `SolvePanel` status grid `SolvePanel.tsx:27-31` | **relocate** | The three pills → global **Status bar**. Rest of SolvePanel stays under **Analyze**. |
| `DiagnosticsPanel` `DiagnosticsPanel.tsx:6-65` + `MissingDataBlockingPanel` + `RuleCheckRunPanel` blockers | **relocate + merge** | **Issues home** (status-bar opener). |
| evidence section: `RunAuditPanel`,`ValidationEvidencePanel`,`BuildReadinessPanel`,`TelemetryBoundaryPanel`,`SecretPrivateLibraryPanel`,`SecurityThreatModelPanel`,`AccessibilityBaselinePanel` `App.tsx:1347-1359` | **relocate** | Status-bar **Audit & boundaries** drawer (collapsed by default). |
| `ResultsPanel` `ResultsPanel.tsx` | **keep + extend** | Under **Results**; add dual-unit columns (DEC-018) — unit policy already declares `unit-system:dec-018-si-dual-display` at `ResultsPanel.tsx:310` but renders single-unit + `conversion_performed:false` (`:316`). |
| `LoadCaseManagerPanel` `LoadCaseManagerPanel.tsx` | **keep + extend** | Under **Loads**; gains bulk-tabular primitive-load editing via Grid mode. |
| `RulePackManagerPanel` + structured composer (`ExpressionComposer`/`CheckDefinitionsEditor`/`DeclarationsEditor`) | **keep** | Under **Rules** (DEC-037: structured only, no writable text). |
| `app-footer` `App.tsx:1364-1366` | **keep** | Unchanged disclaimer. |

---

## 4. Component Inventory (create / modify)

### 4.1 NEW components

- **`AppFrame`** (new shell, replaces the `App.tsx:909-1367` JSX body). Owns the 6-region CSS grid; renders TitleBar, Ribbon, LayoutGrid, ModelCanvas, Inspector, CommandBar, StatusBar, and the collapsed agent seam. Holds no business logic beyond layout; reuses existing handlers/state from `App.tsx`.
- **`Ribbon`** — props: `activeStop`, `stopBadges` (state-derived from model/result/ruleCheckAggregate, same inputs as `App.tsx:1421-1430`), `onSelectStop`. Renders 6 stops + current-step caption. Replaces journey-rail, workspace-nav, guided-journey-stack.
- **`CommandBar`** — props: `selection`, `selectionCount`, `onCommand(commandType)`. Issues commands → `buildIntent`/`handleQueueEditorIntent` (`PipeViewport.tsx:202-209`, `App.tsx:504-513`). NEW.
- **`StatusBar`** — props: `analysisStatus` ({mechanics, rule_check, professional_acceptance} from `result?.status ?? model.analysis_status`, same source as `SolvePanel.tsx:28-30`), `issueCount`, `onOpenAudit`, `onOpenIssues`. NEW.
- **`AuditBoundaryDrawer`** — props: `model`, `storageCapability`, the 7 evidence panels. Collapses everything in §6 list B. Default: one summary chip. NEW container; **reuses** existing evidence panels unchanged inside.
- **`IssuesHome`** — props: aggregated diagnostics + missing-input blockers, `onSelectAffected(entityRef)`. Built on `DiagnosticsPanel`'s aggregation (`DiagnosticsPanel.tsx:20-24`) + `MissingDataBlockingPanel` blockers. NEW container.
- **`EntityGrid`** (Layout grid → Grid mode) — props: `model`, `entityType`, `onQueueIntents(intents[])`. Renders editable rows; each changed cell → one `EditorOperationIntent`. Bulk fan-out builder. **NEW** (the AUTHORING report explicitly confirms no bulk-table editor exists today).
- **`DualUnitValue`** — small presentational component: given a magnitude + entered unit + catalog route, renders `entered (≈ converted)` (DEC-018). Used by Inspector, ResultsPanel, LoadCaseManager. NEW; depends on `describeUnitBasis` (`unitCatalogService.ts:105`) + a conversion helper (NEW, see §4.4).
- **`RequiredFlag`** — presentational ⚑ for a `RequiredInput` that is `NotProvided`/missing provenance (maps to `FindingCode::RuleInputMissing/RuleUnitMissing/RuleProvenanceWarning`). NEW.
- **Canvas helpers** in `PipeViewport.tsx`: `axisTriad()`, `viewCube()`, `referenceGridWithScale()`, `selectionHandles()`, `loadGlyph()`, plus `OrbitControls` wiring. NEW helpers (modifications to existing file).

### 4.2 MODIFY

- `App.tsx` — strip the three navs and the wall-of-strings JSX; keep all state + handlers (`handleQueueEditorIntent` `:504`, `handleValidateIntent` `:522`, `handleApplyIntent` `:536`, undo/redo `:566-577`, run/solve handlers). Render `AppFrame`.
- `PipeViewport.tsx` — hero upgrades (§2.2); remove creation forms (`:320+`) and auto-rotate (`:188`); add view controls.
- `PropertyInspector.tsx` — Inspector role; integrate `DualUnitValue`, `RequiredFlag`, provenance block.
- `ModelTree.tsx` — Tree/Grid mode toggle; host `EntityGrid`.
- `OperationApplyPanel.tsx` — delete the five string lines (`:42-48`, `:70-72`, `:155-159`); keep the queue/validate/apply rows (`:84-122`) and applied ledger (`:123-142`). Diagnostics route to Issues.
- `SolvePanel.tsx` — remove the local status grid (`:27-31`); pills now in StatusBar.
- `ResultsPanel.tsx` — add dual-unit columns.
- `styles.css` — new frame grid; remove `clamp` cap (`:1007`); retire `.journey-rail` (`:247`), `.guided-journey-*` (`:355-383`), 6-col step grids (`:443`).

### 4.3 Data bindings (unchanged contracts to honor)
- **Entities** (7 types: Nodes, Pipes/Elements, Supports, Materials, Sections, Components, Load Cases) — surfaced by Layout grid; selection is `EntityRef {type,id}`.
- **EditorOperationIntent flow** — queue (`App.tsx:504`) → validate (`App.tsx:522`) → apply (`App.tsx:536`). Apply replaces session model (`App.tsx:578`), pushes undo checkpoint (max 25, `App.tsx:566-576`), and **clears prior solve** (`App.tsx:592-596`). The "previous solve results were cleared" message (`App.tsx:598-600`) must surface as a StatusBar pill transition (Mechanics → not-started), not a buried paragraph. Grid mode fans multiple intents through the same path.
- **Results / status** — three-part status vocabulary (mechanics/rule_check/professional_acceptance), each one of six automatic statuses (`MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`); `HUMAN_APPROVED_FOR_PROJECT` is external-only. Rendered as StatusBar pills.
- **Dual units (DEC-018)** — `storage_convention: entered_units_preserved` (`unitCatalogService.ts:32`); display adds a converted companion but does NOT change storage; `conversion_performed` for storage stays false.

### 4.4 Dual-unit display detail
Today the catalog gives `describeUnitBasis` (`unitCatalogService.ts:105`) and dimension equivalence (displacement↔length, stress↔pressure, etc.), but no conversion is rendered (`ResultsPanel.tsx:316` `conversion_performed:false`; no US/SI columns). Add a **display-only** conversion helper keyed off canonical factors in the DEC-018 catalog. `DualUnitValue` renders `<entered value+unit> (≈ <converted value+unit>)`. Storage remains entered-units-preserved; the converted figure is labeled approximate and never written back. This satisfies "dual-unit display (DEC-018)" without violating the storage convention.

---

## 5. Three Navs → One Ribbon

Collapse the following **specific** surfaces into the single Ribbon spine:

1. **`journey-rail`** (`App.tsx:1434-1452`, `styles.css:247`) — its 8 `JOURNEY_STEPS` and per-step status (`journeyStepStatus`, `App.tsx:1447-1448`) become the Ribbon's 6 stop badges + current-step caption (`App.tsx:1453-1467` copy reused).
2. **`workspace-nav`** (`App.tsx:1057-1076`, `styles.css:560`) — its 10 `WORKSPACE_SECTIONS` regroup into the 6 ribbon stops (§2.1). Sections demoted off the spine: `evidence` → status-bar Audit drawer; `project`/`exports` → title-bar overflow / Report stop.
3. **`guided-journey-stack`** + A12/R3 tabs (`App.tsx:1469-1513`, `styles.css:355-383,443`) — deleted. The dual-state-machine accumulation bug (A12/R3 toggling never resets `activeA12StepId`/`activeR3StepId`/`r3JourneyState`) is eliminated because there is one spine and badges are derived purely from model/result/ruleCheck state, holding no separate journey memory. R3 journey-event tracking (`r3JourneyState`, `App.tsx:271-283`) is retained only as evidence for ribbon "Rules" completeness, not as a competing nav.

Result: one mental model. Ribbon = where you are in the workflow = what the workspace shows.

---

## 6. Wall-of-Strings → Demoted Telemetry

**List A — moved to STATUS BAR / Audit & boundaries drawer (removed from inline default surfaces):**
- Storage capability string `engine=…; network=…; daemon=…; telemetry=…; FTS5=…` — `App.tsx:931-937`.
- Boundary strip enums (public/protected/private/professional) — `App.tsx:982-1003`.
- Engine route `engine_route=…; engine_state=…` — `OperationApplyPanel.tsx:42-45`.
- Unit-policy summary `queued_unit_bearing=…; …; conversion=false` — `OperationApplyPanel.tsx:46-48` + `operationApplyUnitPolicySummary` `:174-196`.
- Session-history summary `undo=…; redo=…; local_session_only=true; saved_project_mutated=false` — `OperationApplyPanel.tsx:70-72`.
- The entire **evidence** section (7 panels) `App.tsx:1347-1359` — RunAudit, ValidationEvidence, BuildReadiness, TelemetryBoundary, SecretPrivateLibrary, SecurityThreatModel, AccessibilityBaseline.
- Results unit-policy string `storage=…; conversion=…` — `ResultsPanel.tsx:211`.

**List B — moved to ISSUES home (actionable, human-readable rows):**
- Operation outcome status string `mode=…; application_status=…; schema=…; unit=…; before_state=…; route=…` — `OperationApplyPanel.tsx:155-159` → structured outcome chip + any diagnostics into Issues.
- Operation diff-preview small text — `OperationApplyPanel.tsx:160-163` → kept as structured diff rows in the Model detail drawer; failures → Issues.
- Operation diagnostics `severity: code — message` — `OperationApplyPanel.tsx:165-169` → Issues.
- DiagnosticsPanel list — `DiagnosticsPanel.tsx:44-56` → Issues (primary home).
- Missing-input / `RULE_INPUTS_INCOMPLETE` blockers (MissingDataBlockingPanel, RuleCheckRunPanel) → Issues, surfaced as ⚑ blockers.

After this, the default work surfaces (canvas, grid, inspector, ribbon) carry **zero** raw machine strings. Compliance/telemetry/evidence is one click away, never in the way.

---

## 7. Key Workflow Walkthrough — TP-MAC-141/189 on the new IA

Re-mapping the 13-step packaged checklist proves A3 is fixed (every step has an obvious, single-nav home):

1. **Launch + boundary** → App opens to **Model** ribbon stop, canvas front-and-center. Boundary/telemetry status is the single compact chip in the StatusBar ("Local · no network"), expandable via **Audit & boundaries**. (Replaces `App.tsx:982-1003` strip.)
2. **Authoring entry + next action** → Ribbon shows "Model" active with a current-step caption ("Add nodes to begin"). No three competing rails.
3. **New blank** → Title-bar **New blank** button (`App.tsx:947`). (WASM asset fix, F-4 blocker 1, is orthogonal and already landed.)
4–5. **Author model** → On the canvas/CommandBar: `node` command twice (two nodes); create **material** + **section** in Inspector forms (`PropertyInspector.tsx:80-91`); `pipe` command connects them; `support` command; switch ribbon to **Loads** → create load case + primitive load + combination in `LoadCaseManagerPanel`. Each apply runs through queue→validate→apply (`App.tsx:504/522/536`) with `professional_approval=false`; the StatusBar Mechanics pill flips to not-started after each edit (replaces the buried "results were cleared" paragraph `App.tsx:598-600`). Required-but-missing fields show ⚑ in the Inspector and as blockers in Issues. Bulk-editing 2 nodes is one Grid-mode table, not two single-entity round trips.
6. **Mechanics preview** → Ribbon **Analyze** → Run. StatusBar Mechanics pill → `MECHANICS_SOLVED`; result rows visible under **Results**.
7. **Report + SHA-256** → Ribbon **Report** → render (`RenderedReportPanel`/`ReportPanel`).
8. **Save/reopen/re-solve** → Title-bar Save/Open; re-run from Analyze.
9–10. **R3 library** → Ribbon **Rules** (or Model→Libraries) → `LibraryManagerPanel`: load invented template, validate, local save. R3 completeness reflected in ribbon badge.
11. **Rule-pack draft** → Ribbon **Rules** → `RulePackManagerPanel` structured composer (DEC-037, no writable text) → validate → checksum → local save.
12. **Rule-check after solve** → Analyze/Rules run; **missing-input blockers** appear prominently in the **Issues** home (⚑ rows) and as ribbon "Rules" badge state; no software code-compliance status anywhere (it lives only as analysis status pills, never a compliance claim).
13. **Quit clean** → Audit drawer confirms no network/daemon/telemetry/private-write; footer disclaimer intact (`App.tsx:1364-1366`).

Every step is reachable from the **one ribbon** + title bar; diagnostics/blockers are visible (Issues), not buried; the model is the hero throughout. This is the concrete fix for A3 "no logical layout… things cut off… many things unresponsive."

---

## 8. Acceptance Criteria

A. **Single navigation.** Exactly one navigation control (the Ribbon) exists in the shell; `journey-rail`, `workspace-nav`, and `guided-journey-stack` no longer render. (Verifiable: those `data-testid`s — `guided-journey-tab-a12/-r3`, `journey-step-*` — are gone or repurposed to ribbon stops.)
B. **No inline wall-of-strings.** Default surfaces (canvas, layout grid, inspector, ribbon, command bar) render no raw `key=value;…` machine strings. The seven List-A strings (§6) appear only inside the Audit & boundaries drawer; List-B items appear only in Issues. (Verifiable: the `data-testid`s `operation-engine-status`, `operation-apply-unit-policy`, `session-history-summary`, `local-project-status`, `preview-boundary-strip` are absent from the default DOM.)
C. **Canvas is hero & reads as 3D.** Viewport `clamp` cap (`styles.css:1007`) removed; canvas fills its cell (min-height ≥ ~520px). Axis triad, view cube, orbit controls, reference grid+scale, selection handles, and distinct support/load glyphs are present. Auto-rotate removed.
D. **Dual-unit display.** Inspector and Results render entered + converted (DEC-018) without changing `entered_units_preserved` storage.
E. **Issues home + status bar.** StatusBar shows the three analysis pills globally; the Issues opener shows a live count; clicking a diagnostic selects the affected entity in canvas/inspector. Missing-input blockers (TP-MAC-189 step 12) are visible there.
F. **Behavior preserved.** EditorOperationIntent queue→validate→apply, undo/redo (25-depth), solve-clears-on-edit, save/reopen all still work (existing `App.test.tsx` green; dead-control audit clean).
G. **Packaged human pass (the real bar).** A human completes TP-MAC-189 (13 steps) on a freshly built bundle and records PASS per step (date / performed_by / binary_commit / bundle_path / report_sha256 / notes). **PASS closes A3 and F-4** and unblocks C5.8. Verified at 1280×800 and one larger size with no clipped/unscrollable/unresponsive controls (the A3 repair bar).

---

## 9. Build Sequencing (smallest-shippable-first)

Each increment ships independently, keeps the app runnable, and keeps `App.test.tsx` green.

- **Inc 0 — Frame skeleton.** Add `AppFrame` 6-region grid behind the existing layout; no behavior change yet. Establishes the structural target.
- **Inc 1 — One Ribbon.** Build `Ribbon`; route `setActiveSection` through it; delete `journey-rail` + `guided-journey-stack`; collapse `workspace-nav` into 6 stops. *Closes the "three navs" verdict.* (Highest-leverage; do first after frame.)
- **Inc 2 — Status bar + Audit drawer.** Build `StatusBar` (lift `SolvePanel.tsx:27-31` pills) + `AuditBoundaryDrawer`; move List-A strings + evidence section into it; remove the storage/boundary strings from title bar and boundary strip. *Removes most of the wall.*
- **Inc 3 — Issues home.** Build `IssuesHome` from `DiagnosticsPanel` aggregation + missing-data blockers; strip operation-outcome/diagnostic strings from `OperationApplyPanel`. *Removes the rest of the wall; surfaces TP-MAC-189 step-12 blockers.*
- **Inc 4 — Canvas as hero.** Remove `clamp` cap; add orbit controls, axis triad, view cube, reference grid+scale, selection handles, load glyphs; move creation forms out; wire `CommandBar`. *Closes "postage stamp."*
- **Inc 5 — Inspector + dual units + required flags.** Refactor `PropertyInspector`; add `DualUnitValue`, `RequiredFlag`, provenance block; dual-unit columns in `ResultsPanel`.
- **Inc 6 — Layout grid (bulk tabular). LANDED 2026-06-19** (`TP-R3UX-GRIDMODE-001`, SMOKE TP-MAC-273): added `EntityGrid` + Grid-mode toggle in `ModelTree`; changed cells fan out to existing structured `EditorOperationIntent`s. (Largest new surface; **core to authoring usability**, sequenced last only because single-entity authoring is functional after Inc 5 — so it is non-gating for the *first* TP-MAC-189 re-attempt, but a committed follow-through, not optional.)
- **Inc 7 — Packaged build + human pass prep. LANDED 2026-06-19** (`TP-R3UX-PACKAGEKIT-002`, SMOKE TP-MAC-274): rebuilt bundle, re-ran packaged/dist evidence, boot-checked the `.app`, and handed TP-MAC-189 back to the human for the C5.7 re-attempt.

Minimum to re-attempt the C5.7 human pass: **Inc 0–5** (one nav, demoted strings, hero canvas, dual units, required flags). Inc 6 (bulk-tabular) is **core** to the authoring experience but non-gating for the *first* A3/F-4 re-attempt — a committed follow-through, not optional.

---

## 10. Scope Boundaries

- **Agent panel: SEAM only.** Reserve a collapsed right-edge dock slot in the `AppFrame` grid, gated by **D-21**. Do **not** build the embedded agent UI, message surface, or proposal-driving panel in this work. The existing `AgentProposalPanel` (`App.tsx:1138-1143`) is retained as a parked detail-drawer panel, not promoted into the agent seam.
- **No relitigating the locked frame.** Ribbon order, canvas-as-hero, grid/inspector split, command bar, demoted telemetry, dual units, code-neutral entry are realized as specified, not re-debated.
- **DEC-037 honored.** Rule-pack authoring stays structured-composer-only; no writable expression text.
- **Storage convention unchanged.** Dual units are display-only; `entered_units_preserved` storage is not altered.
- **WASM asset fix (F-4 blocker 1) is out of this spec's scope** — already landed; this redesign addresses the usability half (A3 + the F-4 journey completion).
- **No new backend/engine contracts.** All editing continues through the existing `operationService` intent seam; no new operation kinds are introduced (Grid mode fans existing intents).
- **Bulk-tabular is core, not optional.** The CAEPIPE-style Grid mode (Inc 6) is sequenced after the first human-pass re-attempt for delivery reasons (single-entity authoring is functional sooner), but it is core to the authoring usability the redesign targets — a committed follow-through.
- **No live CAEPIPE to benchmark against.** CAEPIPE is not currently installed, so the bulk-tabular UX cannot be tested side-by-side against it yet; Grid mode is built to the design intent and judged by the project authority (who knows CAEPIPE), not an oracle comparison. Separately, the solver-vs-CAEPIPE analysis-oracle validation (`PLAN_2026-06-12_caepipe_external_oracle_feedback_loop.md`) is likewise blocked until CAEPIPE is available — a Phase-D/R4 concern, out of this UI spec's scope.

---

*v0.1 — buildable, harness-independent. All current-state citations verified against `chirality-piping/apps/desktop/src/` working tree. NEW components (`AppFrame`, `Ribbon`, `CommandBar`, `StatusBar`, `AuditBoundaryDrawer`, `IssuesHome`, `EntityGrid`, `DualUnitValue`, `RequiredFlag`, canvas helpers) are flagged NEW; everything else is a refactor/relocation of an existing, cited component.*
