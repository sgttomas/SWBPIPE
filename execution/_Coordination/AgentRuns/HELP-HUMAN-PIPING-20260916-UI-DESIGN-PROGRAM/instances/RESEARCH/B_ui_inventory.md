# B — Quantitative UI Inventory: OpenPipeStress desktop app

**Scope.** Numbers only, no design judgments.
**Repository root:** `{REPO_ROOT}`
**App source:** `projects/chirality-piping/apps/desktop/src`
**Git revision at measurement:** branch `claude/chirality-piping-ui-design-a31fd2`, HEAD `28ad73cc3`, working tree clean.
**Measurement date:** 2026-09-16. **Mode:** read-only; no repository file was modified.

Unless a command says otherwise, every command below was run from
`{REPO_ROOT}/projects/chirality-piping/apps/desktop`
and "non-test" means the file-name filter `-not -name '*.test.ts' -not -name '*.test.tsx'`.

---

## 1. Structure

### 1.1 Headline counts

| Quantity | Value | Command |
|---|---|---|
| Directories under `src/features/` | **57** | `find src/features -mindepth 1 -maxdepth 1 -type d \| wc -l` |
| Loose files under `src/features/` | 1 (`exportUnitDisclosure.ts`) | `find src/features -mindepth 1 -maxdepth 1 -type f` |
| `*Panel.tsx` components (non-test) | **50** | `find src -name '*Panel.tsx' -not -name '*.test.tsx' \| wc -l` |
| `*Panel.tsx` test files | 0 (no `*Panel.test.tsx` exists) | `find src -name '*Panel.tsx' \| wc -l` → 50 |
| Distinct `…Panel` identifiers on `import` lines in `App.tsx` | **51** | `grep -E '^import ' src/App.tsx \| grep -oE '\b[A-Z][A-Za-z0-9]*Panel\b' \| sort -u \| wc -l` |
| `import …Panel…` statements in `App.tsx` | 51 | `grep -cE '^import .*Panel' src/App.tsx` |
| Non-test `.ts`/`.tsx` files in `src` | **117** | `find src \( -name '*.ts' -o -name '*.tsx' \) <non-test> -not -path '*/test/*' \| wc -l` |
| Test files in `src` | **57** | `find src \( -name '*.test.ts' -o -name '*.test.tsx' \) \| wc -l` |

The 51 imported `…Panel` identifiers exceed the 50 `*Panel.tsx` files because `HistoricalRunPanel` lives in
`src/features/results/HistoricalRunContext.tsx`.
Conversely 5 feature dirs contain no `*Panel.tsx` at all (`component-creation`, `model-tree`, `model-workspace`,
`rich-authoring`, `support-configuration`, `wind-exposure`, `display-units`, `material-temperature`, `workspace`).

### 1.2 Total LOC

| Bucket | LOC | Files |
|---|---|---|
| Non-test `.ts`/`.tsx` under `src` | **52,346** | 117 |
| Test `.ts`/`.tsx` under `src` | **28,687** | 57 |
| `src/styles.css` | 3,438 | 1 |
| Test : non-test ratio | **0.55 : 1** | — |

```bash
find src \( -name '*.ts' -o -name '*.tsx' \) -not -name '*.test.ts' -not -name '*.test.tsx' \
  -not -path '*/test/*' -not -path '*/__tests__/*' -exec cat {} + | wc -l     # 52346
find src \( -name '*.test.ts' -o -name '*.test.tsx' \) -exec cat {} + | wc -l # 28687
```

Largest single files:

| File | Lines | Bytes | avg line len | max line len | lines > 120 ch |
|---|---:|---:|---:|---:|---:|
| `src/App.test.tsx` | 17,084 | 740,499 | 42.3 | 768 | 181 |
| `src/App.tsx` | 3,709 | 152,676 | 40.1 | 305 | 76 |
| `src/features/viewport/PipeViewport.tsx` | 3,097 | 131,119 | 41.3 | 379 | 83 |
| `src/styles.css` | 3,438 | 75,080 | 20.8 | 348 | 38 |
| `src/services` (non-test, 21 files) | 4,304 | — | — | — | — |

```bash
wc -l src/App.tsx src/types.ts src/main.tsx src/styles.css src/App.test.tsx
node -e 'const fs=require("fs");for(const p of ["src/App.tsx",...]){/* avg/max/over120 */}'
```

### 1.3 LOC per feature directory (non-test `.ts`/`.tsx`), descending

```bash
for d in $(find src/features -mindepth 1 -maxdepth 1 -type d | sort); do
  loc=$(find "$d" \( -name '*.ts' -o -name '*.tsx' \) -not -name '*.test.ts' -not -name '*.test.tsx' \
        -not -path '*/__tests__/*' -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
  n=$(find "$d" \( -name '*.ts' -o -name '*.tsx' \) -not -name '*.test.ts' -not -name '*.test.tsx' \
        -not -path '*/__tests__/*' | wc -l | tr -d ' ')
  echo "$(basename $d)|$loc|$n"
done | sort -t'|' -k2 -rn
```

| # | Feature dir | LOC | files | | # | Feature dir | LOC | files |
|---:|---|---:|---:|---|---:|---|---:|---:|
| 1 | model-tree | 4,408 | 2 | | 30 | design-workspace | 367 | 1 |
| 2 | viewport | 3,888 | 3 | | 31 | model-workspace | 362 | 1 |
| 3 | rule-packs | 3,367 | 4 | | 32 | adapter-framework | 353 | 1 |
| 4 | load-cases | 2,716 | 1 | | 33 | security-threat-model | 349 | 1 |
| 5 | report | 2,462 | 6 | | 34 | project-storage | 347 | 1 |
| 6 | redaction-controls | 1,654 | 3 | | 35 | support-configuration | 314 | 1 |
| 7 | library | 1,431 | 1 | | 36 | telemetry | 303 | 1 |
| 8 | rule-check | 1,413 | 2 | | 37 | build-readiness | 270 | 1 |
| 9 | export-review | 1,283 | 1 | | 38 | geometry-tools | 247 | 3 |
| 10 | results | 1,240 | 4 | | 39 | accessibility-baseline | 233 | 1 |
| 11 | native-package | 990 | 1 | | 40 | diagnostics | 231 | 1 |
| 12 | report-lint | 983 | 1 | | 41 | rich-authoring | 200 | 1 |
| 13 | project-validation | 958 | 1 | | 42 | hanger-selection | 199 | 3 |
| 14 | toolkit | 926 | 6 | | 43 | comparison | 158 | 1 |
| 15 | stress-neutral | 822 | 1 | | 44 | agent-proposals | 156 | 1 |
| 16 | pcf-export | 793 | 1 | | 45 | run-audit | 142 | 1 |
| 17 | review-geometry | 776 | 1 | | 46 | knowledge | 140 | 1 |
| 18 | caepipe-mbf | 765 | 1 | | 47 | wind-exposure | 137 | 1 |
| 19 | editor-contract | 677 | 1 | | 48 | boundary-authoring | 132 | 2 |
| 20 | operations | 650 | 2 | | 49 | display-units | 116 | 2 |
| 21 | local-fea-handoff | 589 | 1 | | 50 | offline-proposal-intake | 112 | 3 |
| 22 | missing-data | 557 | 1 | | 51 | self-weight-authoring | 99 | 2 |
| 23 | caepipe-external | 544 | 1 | | 52 | material-temperature | 89 | 1 |
| 24 | export-adapter-sdk | 533 | 1 | | 53 | workspace | 41 | 1 |
| 25 | component-creation | 453 | 1 | | | | | |
| 26 | external-prover | 452 | 1 | | | | | |
| 27 | secret-private-library | 438 | 1 | | | | | |
| 28 | result-export | 421 | 3 | | | | | |
| 29 | solve / headless-runner / handoff / diff-preview / validation-evidence | 409 / 395 / 384 / 384 / 374 | 1 each | | | | | |

Sum of feature-dir LOC = 43,764 (83.6% of the 52,346 non-test total).
Top 5 dirs = 16,841 LOC = 38.5% of all feature LOC.

---

## 2. Navigation surfaces

### 2.1 Counts

| Surface | Count | Where | Command / basis |
|---|---:|---|---|
| `WORKSPACE_SECTIONS` entries | **10** | `src/App.tsx:373-421` | `sed -n '373,421p' src/App.tsx` (literal array) |
| `MenuCommandId` union arms as written | **22** | `src/App.tsx:345-365` | 21 string literals + 1 template `` `view.section.${WorkspaceSectionId}` `` |
| `MenuCommandId` members after expansion | **31** | — | 21 + 10 sections |
| `NATIVE_MENU_COMMAND_IDS` set size | **31** | `src/App.tsx:423-449` | `node` parse: 21 explicit literals + `WORKSPACE_SECTIONS.map(...)` |
| Top-level menus rendered by `MenuBar` | **5** (File, Edit, View, Insert, Analyze) | `src/App.tsx:2691-2756` | manual read of `const menus` |
| Menu **command** items rendered | **31** | same | File 6 + Edit 2 + View 15 + Insert 5 + Analyze 3 |
| Menu **separators** rendered | **7** | same | File 2, Edit 0, View 3, Insert 1, Analyze 1 |
| Total menu DOM items | **38** | same | 31 + 7 |
| Menu bar is present only when | `!isTauriRuntime()` | `src/App.tsx:2038,2110` | `grep -n 'showInAppMenuBar' src/App.tsx` |
| "Ribbon" | **0** | — | `grep -n 'ribbon\|Ribbon' src/App.tsx` → no matches. The nearest equivalent is `WorkspaceToolbar`. |
| `workspace-task-nav` stops (toolbar row 1) | **6** (Model, Loads, Analyze, Results, Rules, Report) | `src/features/workspace/WorkspaceToolbar.tsx:22-31` | literal array |
| `workspace-edit-actions` buttons (toolbar row 2) | **5** (Select, Undo, Redo, Review, + Toolkit entry passed as children) | `WorkspaceToolbar.tsx:32-38` + `App.tsx:2139-2142` | `grep -c '<button' src/features/workspace/WorkspaceToolbar.tsx` → 5 |
| **Command-bar** buttons (`section.command-bar`) | **6** (Node, Pipe, Support, Component, Load + `Queue preview`) | `src/features/viewport/PipeViewport.tsx:1220-1304` | manual read |
| Command-bar disclosures | 1 `<details class="command-context">` | `PipeViewport.tsx:1287` | — |
| Viewport-toolbar buttons | **3** (Labels, Loads, Grid) | `PipeViewport.tsx:1122-1152` | — |
| Viewport-toolbar disclosures | 1 `<details class="viewport-deformation-status">` | `PipeViewport.tsx:1112` | — |
| **Status-bar** openers | **2 buttons** (`audit-drawer-toggle`, `issues-drawer-toggle`) + **3–4 `<details>` pills** (Mechanics, Rule check, Professional, and Solve proof when present) | `src/App.tsx:2944-2981`, `StatusPill` at `App.tsx:3099` | manual read |
| Drawers (`aside.workspace-drawer`, `position: fixed`) | **2** (`AuditBoundaryDrawer`, `IssuesHome`) | `App.tsx:3000`, `App.tsx:3066`; CSS `styles.css:836-852` | `grep -n 'workspace-drawer' src/*.tsx src/styles.css` |
| `role="dialog"` elements | **1** (`ToolkitPalette` command palette, `aria-modal="true"`) | `src/features/toolkit/ToolkitPalette.tsx:69` | `grep -rn 'role="dialog"' src \| grep -v '\.test\.'` |
| `<dialog>` elements | **0** | — | `grep -ohF '<dialog' …` → 0 |
| Popover-style `<details>` (absolutely positioned when `[open]`) | **4** classes: `.display-preference-control`, `.command-context`, `.viewport-deformation-status`, `.status-pill` | `styles.css:3274, 3309, 3376, 3437` | `grep -nE '\[open\]' src/styles.css` |
| Inline drawer inside the dock | 1 (`.review-apply-drawer`) | `App.tsx:2347`, `styles.css:3054` | — |
| **Dock sections mounted simultaneously** | **10** (inactive ones are `display:none`, not unmounted — `styles.css:3387`, comment at `App.tsx:2651-2653`) | — | — |

Composite: **navigation entry points to the same 10 dock sections exist in 3 parallel places** — the View menu (10 items),
the `workspace-task-nav` (6 of the 10 + "Model" = viewport-only), and the `view.section.*` native-menu commands (10).

### 2.2 Components rendered per dock section

```bash
node -e '/* scan App.tsx between data-testid="workspace-section-<id>" markers */'
```

| Dock section | Components mounted |
|---|---:|
| exports | **15** |
| operations | **12** |
| evidence | 7 |
| solve | 4 |
| results | 4 |
| report | 3 |
| project | 2 |
| loads | 1 |
| libraries | 1 |
| rule-packs | 1 |
| **Total** | **50** |

### 2.3 `data-testid` and ARIA

| Quantity | Count | Command |
|---|---:|---|
| `data-testid=` occurrences, non-test `.ts`/`.tsx` | **829** | `find src … -exec grep -ohE 'data-testid=' {} + \| wc -l` |
| …distinct **static** string values | **672** | `… grep -ohE 'data-testid="[^"{]*"' {} + \| sed 's/data-testid="//;s/"$//' \| sort -u \| wc -l` |
| …distinct template-literal forms (dynamic, expand at runtime) | **84** | `… grep -ohE 'data-testid=\{`[^`]*`\}' {} + \| sort -u \| wc -l` |
| …other dynamic expressions | 3 | `… grep -ohE 'data-testid=\{[^`][^}]*\}' {} +` |
| Non-test files carrying at least one `data-testid` | **57 of 117** | `… -exec grep -l 'data-testid=' {} + \| wc -l` |
| `aria-label=` occurrences | **307** | `… grep -ohE 'aria-label=' {} + \| wc -l` |
| …distinct static values | **271** | `… grep -ohE 'aria-label="[^"{]*"' {} + \| sort -u \| wc -l` |
| `aria-pressed=` | 27 | `… grep -ohE 'aria-(labelledby\|describedby\|expanded\|pressed\|haspopup\|modal\|live)=' {} + \| sort \| uniq -c` |
| `aria-describedby=` | 14 | same |
| `aria-expanded=` | 7 | same |
| `aria-labelledby=` | 2 | same |
| `aria-haspopup=` / `aria-modal=` | 1 / 1 | same |

Top files by `data-testid`: `PipeViewport.tsx` 92, `LoadCaseManagerPanel.tsx` 88, `PropertyInspector.tsx` 75,
`App.tsx` 48, `LibraryManagerPanel.tsx` 45, `RuleCheckRunPanel.tsx` 39, `ExpressionComposer.tsx` 29,
`ResultsPanel.tsx` 24, `RulePackManagerPanel.tsx` 23, `RenderedReportPanel.tsx` 23, `ModelTree.tsx` 23,
`DeclarationsEditor.tsx` 20.

Top files by `aria-label`: `PropertyInspector.tsx` 53, `LoadCaseManagerPanel.tsx` 53, `PipeViewport.tsx` 52,
`App.tsx` 30, `ResultsPanel.tsx` 15, `ModelTree.tsx` 11.

---

## 3. Interactive controls (static JSX occurrences, non-test)

```bash
for tag in '<button' '<input' '<select' '<textarea' '<details' '<summary' '<table' '<form' '<label' '<a ' '<canvas' '<dialog' '<nav' '<section' '<pre'; do
  find src \( -name '*.ts' -o -name '*.tsx' \) -not -name '*.test.ts' -not -name '*.test.tsx' \
    -print0 | xargs -0 grep -ohF "$tag" | wc -l
done
# and the same restricted to src/features
```

| Tag | `src` total | inside `src/features` |
|---|---:|---:|
| `<button` | **175** | 153 |
| `<input` | **117** | 117 |
| `<select` | **79** | 79 |
| `<label` | 189 | 189 |
| `<textarea` | **4** | 4 |
| `<details` | **26** | 22 |
| `<summary` | **26** | 22 |
| `<table` | **3** | 3 |
| `<section` | 123 | 104 |
| `<pre` | 12 | 11 |
| `<nav` | 3 | 1 |
| `<a ` | 2 | 2 |
| `<form` | **0** | 0 |
| `<canvas` (literal tag) | **0** | 0 |
| `<dialog` | **0** | 0 |
| `onClick=` | 178 | — |
| `onChange=` | 374 | — |
| `className="panel` | 100 | — |

These are *static source occurrences*; several sit inside `.map()` loops, so the rendered DOM count is higher
(e.g. the 31 menu items come from 1 `<button` in the source).

Buttons per file (top): `App.tsx` 22, `PipeViewport.tsx` 20, `LoadCaseManagerPanel.tsx` 16,
`PropertyInspector.tsx` 10, `LibraryManagerPanel.tsx` 10, `RulePackManagerPanel.tsx` 8, `ModelTree.tsx` 8.

Inputs per file (top): `PipeViewport.tsx` 29, `LoadCaseManagerPanel.tsx` 25, `PropertyInspector.tsx` 24.
Selects per file (top): `LoadCaseManagerPanel.tsx` 19, `PropertyInspector.tsx` 13, `PipeViewport.tsx` 12.

`<details>` disclosures are concentrated: `PropertyInspector.tsx` 9, `PipeViewport.tsx` 4, `App.tsx` 4,
`ToolkitPalette.tsx` 2, `BatchReviewPanel.tsx` 2, `HangerSelectionPanel.tsx` 2, 1 each in 3 more files.

`<table>` appears only at `src/features/model-tree/ModelTree.tsx:606`,
`src/features/results/ResultsPanel.tsx:125` and `:428`.

### 3.1 Panels with no `<button>` at all

```bash
for f in $(find src -name '*Panel.tsx' -not -name '*.test.tsx' | sort); do
  echo "$(grep -c '<button' "$f") $f"; done | awk '$1==0' | wc -l
```

**31 of the 50 `*Panel.tsx` files (62%) contain zero `<button` occurrences** — they render read-only text.
They are: AccessibilityBaseline, AdapterFramework, BuildReadiness, CaepipeExternalHarness, CaepipeMbfExport,
DesignWorkspace, DiffPreview, EditorContract, ExportAdapterSdk, ExportReview, ExternalProverBoundary, Handoff,
HeadlessRunner, Knowledge, LocalFeaHandoff, MissingDataBlocking, NativePackage, PcfExport, ProjectStorageAudit,
ProjectValidation, RedactionExportControls, ReportLint, Report, ResultExport, ReviewGeometry, RuleCheck,
RunAudit, SecretPrivateLibrary, SecurityThreatModel, StressNeutralExport, ValidationEvidence.

---

## 4. Machine-string exposure in rendered text

### 4.1 Method

Exact counting of "what is rendered" is impractical without executing the app, so this is a **stated proxy**.
A Node script (written to the scratchpad, not the repo) extracts from every non-test `.tsx` file:

1. every template literal, with `${…}` expressions blanked;
2. every JSX text node matched by `/>([^<>{}]*[A-Za-z][^<>{}]*)</g`;
3. every double-quoted string literal **not** immediately preceded by a code-only attribute
   (`className`, `data-testid`, `key`, `htmlFor`, `id`, `href`, `type`, `role`, `ref`, `name`,
   `aria-controls`, `aria-describedby`, `aria-labelledby`, `surface`, `focusTestId`, `elementId`).

It then counts each term case-insensitively in that extract. 71 `.tsx` files were scanned; 64 had ≥ 1 hit.
The script is at
`…/scratchpad/work/strings.cjs` and was run as `node …/strings.cjs "$(pwd)/src"`.
This over-counts (some quoted literals are never rendered) and under-counts (strings assembled across variables),
so treat it as order-of-magnitude.

### 4.2 Totals

| Term | Renderable-proxy count | Raw `grep -o` count over all non-test `.tsx` |
|---|---:|---:|
| `TBD` | **629** | 620 |
| `not_` | **350** | 365 |
| `boundary` | **258** | 605 |
| `evidence` | **214** | 431 |
| `hash` | **190** | 420 |
| `professional` | **146** | 500 |
| `invented` | **117** | 106 |
| `_id` | 91 | 1,329 |
| `manifest` | 77 | 219 |
| `telemetry` | 72 | 136 |
| `UNKNOWN` | 29 | 2 (case-sensitive) |
| `sha256` | 26 | 42 |
| `generation` | 13 | 60 |
| `seam` | 4 | 23 |
| `testid` | 4 | 831 |
| **Sum (proxy)** | **2,220** | — |

Raw counts:
`for t in sha256 hash seam generation boundary evidence telemetry professional invented testid TBD UNKNOWN not_ _id manifest; do find src -name '*.tsx' -not -name '*.test.tsx' -exec grep -oh "$t" {} + | wc -l; done`

### 4.3 Per-file, top 12 by total hits (proxy)

| File | total | sha256 | hash | seam | generation | boundary | evidence | telemetry | professional | invented | testid | TBD | UNKNOWN | not_ | _id | manifest |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `features/export-review/ExportReviewPanel.tsx` | **170** | 3 | 2 | 0 | 0 | 30 | 36 | 17 | 3 | 1 | 0 | 45 | 0 | 22 | 2 | 9 |
| `features/report-lint/ReportLintPanel.tsx` | **129** | 0 | 3 | 0 | 0 | 19 | 47 | 7 | 29 | 12 | 0 | 10 | 0 | 2 | 0 | 0 |
| `features/model-tree/PropertyInspector.tsx` | **126** | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 2 | 0 | 0 | 78 | 1 | 41 | 0 | 0 |
| `features/load-cases/LoadCaseManagerPanel.tsx` | **110** | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 13 | 0 | 0 | 39 | 1 | 57 | 0 | 0 |
| `App.tsx` | **104** | 3 | 13 | 2 | 7 | 15 | 11 | 5 | 6 | 1 | 1 | 1 | 1 | 29 | 1 | 8 |
| `features/project-validation/ProjectValidationPanel.tsx` | 95 | 0 | 44 | 0 | 0 | 2 | 14 | 1 | 4 | 1 | 0 | 1 | 0 | 27 | 0 | 1 |
| `features/native-package/NativePackagePanel.tsx` | 93 | 3 | 18 | 0 | 0 | 5 | 3 | 1 | 3 | 5 | 0 | 24 | 1 | 21 | 3 | 6 |
| `features/stress-neutral/StressNeutralExportPanel.tsx` | 82 | 7 | 12 | 0 | 0 | 6 | 8 | 0 | 5 | 3 | 0 | 12 | 4 | 3 | 11 | 11 |
| `features/caepipe-mbf/CaepipeMbfExportPanel.tsx` | 73 | 0 | 6 | 0 | 0 | 9 | 2 | 0 | 3 | 9 | 0 | 26 | 0 | 7 | 6 | 5 |
| `features/results/HistoricalRunContext.tsx` | 64 | 5 | 22 | 0 | 0 | 4 | 3 | 0 | 2 | 1 | 0 | 5 | 3 | 2 | 6 | 11 |
| `features/editor-contract/EditorContractPanel.tsx` | 63 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 3 | 5 | 0 | 42 | 0 | 8 | 1 | 0 |
| `features/pcf-export/PcfExportPanel.tsx` | 62 | 0 | 6 | 0 | 0 | 7 | 4 | 0 | 3 | 5 | 0 | 22 | 0 | 5 | 5 | 5 |

Top 12 files hold 1,171 of the 2,220 proxy hits (52.7%).

---

## 5. Styles (`src/styles.css`)

```bash
wc -lc src/styles.css                                   # 3438 lines, 75080 bytes
grep -o '{' src/styles.css | wc -l                      # 588
grep -o '}' src/styles.css | wc -l                      # 588
grep -c '@media' src/styles.css                         # 4
grep -o '!important' src/styles.css | wc -l             # 0
grep -ohE '#[0-9a-fA-F]{3,8}\b' src/styles.css | tr 'A-F' 'a-f' | sort -u | wc -l   # 135
grep -ohE 'rgba?\([^)]*\)' src/styles.css | sort -u | wc -l                          # 16
grep -ohE 'font-size:\s*[^;]+' src/styles.css | sed 's/font-size:[ ]*//' | tr -d ' ' | sort -u | wc -l   # 12
grep -ohE 'border-radius:\s*[^;]+' … | sort -u                                       # 7
grep -ohE 'box-shadow:\s*[^;]+' … | sort -u | wc -l                                  # 12
grep -ohE 'font-family:\s*[^;]+' … | sort | uniq -c                                  # 3 distinct
grep -cE 'min-(width|height):' src/styles.css                                        # 174
grep -n 'clamp(' src/styles.css                                                      # 2
```

| Metric | Value |
|---|---:|
| Lines / bytes | 3,438 / 75,080 |
| `{` (≈ rule blocks, incl. 4 `@media` wrappers) | **588** (`}` also 588 — balanced) |
| Rule blocks excluding the 4 at-rule wrappers | **584** |
| Selector lines ending in `{` | 447 |
| `@media` queries | **4** — `(max-width: 1040px)` L1823, `(max-height: 840px)` L1831, `(max-width: 1320px)` L2951, `(max-width: 1100px)` L3412 |
| Other at-rules (`@supports`, `@keyframes`, `@layer`, `@import`, custom props at-rules) | **0** |
| `!important` | **0** |
| Distinct hex colours (case-folded) | **135** |
| Distinct `rgb()`/`rgba()` literals | **16** |
| Distinct `hsl()` / `color-mix()` | 0 / 0 |
| **Distinct colour literals total** | **151** |
| `font-size:` declarations | 152 |
| Distinct `font-size` values | **12** — `12px`×63, `11px`×62, `13px`×9, `10px`×8, `15px`×2, `14px`×2, `9px`, `20px`, `18px`, `16px`, `0.75em`, `0.72rem` |
| `border-radius:` declarations | 79 |
| Distinct `border-radius` values | **7** — `4px`×51, `6px`×11, `5px`×11, `0`×3, `9px`, `10px`, `0.4rem` |
| Distinct `box-shadow` values | **12** (all single-layer; 5 hex-alpha, 5 `rgb()/rgba()`, 2 `inset` solid) |
| `font-family` declarations | **3 distinct** — the Inter stack (1), `inherit` (2), `monospace` (1) |
| `min-width:` / `min-height:` declarations | **174** total |
| CSS custom properties defined | 4 grid column tokens (`--pane-tree-col`, `--pane-viewport-col`, `--pane-agent-col`, `--pane-inspector-col`) + 2 fallback refs (`--border-color`) |

### 5.1 Shell pixel floors

| Declaration | Line |
|---|---|
| `body { min-width: 1120px; min-height: 760px; }` | `styles.css:38-42` |
| `.app-shell { grid-template-rows: auto auto auto 1fr auto auto; height: 100vh; min-height: 760px; }` | `styles.css:63-67` |
| `.app-shell.native-menu { grid-template-rows: auto auto 1fr auto auto; }` | `styles.css:71-73` |
| **later override**: `body { min-width: 760px; min-height: 540px; }` | `styles.css:3265` |
| **later override**: `.app-shell, .app-shell.native-menu { display: flex; flex-direction: column; height: 100dvh; min-height: 540px; overflow: hidden; }` | `styles.css:3266` |

The effective shell is therefore **flex column, 100dvh, floor 760 × 540 px** — the grid-rows declarations at L63/L71
are dead (overridden by `display:flex` at L3266).

### 5.2 `.modeling-workspace` grid-template-columns

| Context | `grid-template-columns` | Line |
|---|---|---|
| Base (dead) | `300px minmax(560px, 1.6fr) minmax(300px,340px) 390px` | `styles.css:496-509` |
| Base, neither rail collapsed (dead) | `300px minmax(560px,1.6fr) 390px` | `styles.css:524-529` |
| `@media (max-width: 1320px)` (dead) | `250px minmax(400px,1fr) minmax(280px,320px) 340px` | `styles.css:2955-2961` |
| **Effective (last wins)** | `var(--pane-tree-col) var(--pane-viewport-col) var(--pane-inspector-col)` = **`215px minmax(0,1fr) 280px`**, `gap: 6px` | `styles.css:3289` |
| Effective, `@media (max-width: 1100px)` | **`180px minmax(0,1fr) 260px`** | `styles.css:3415` |
| Rail collapsed | `--pane-tree-col: 30px` / `--pane-inspector-col: 30px` | `styles.css:3290-3291`, `3416-3417` |

The `--pane-agent-col` track is never used in the effective rule (the agent pane is `display:none` at `styles.css:531`).

### 5.3 `clamp()` on viewport height

Only **two** `clamp()` uses in the whole stylesheet:

| Line | Declaration | Effective? |
|---|---|---|
| `styles.css:509` | `.modeling-workspace { min-height: clamp(230px, 40vh, 540px); }` | **No** — overridden by `min-height: 0` at `styles.css:3289` |
| `styles.css:3380` | `.workspace-dock { flex: 0 0 clamp(180px, 30vh, 280px); max-height: 40%; min-height: 0; }` | **Yes** |

So the single live viewport-height clamp is the **dock height: `clamp(180px, 30vh, 280px)`, capped at 40% of the
workspace box**. `.workspace-dock.collapsed { display: none; }` (`styles.css:386-388`).

---

## 6. Canvas budget

### 6.1 Effective chrome rows (all `box-sizing: border-box`, so `min-height` is the full outer height)

| Row | Effective `min-height` | Line |
|---|---:|---|
| native macOS title bar (outside the WebView) | ~24–32 px | — |
| `.titlebar` | 48 px | `styles.css:3267` |
| `.project-strip` | 22 px | `styles.css:3275` |
| `.app-menu-bar` (**browser only**; `.app-menu-trigger` 26 px + 1 px border) | 27 px | `styles.css:3279-3280` |
| `.workspace-toolbar` | 44 px | `styles.css:3281` |
| `.workspace` | `flex: 1 1 0`, `padding: 6px`, `gap: 6px` | `styles.css:3288` |
| `.status-bar` | 30 px | `styles.css:3395` |
| `.app-footer` | 21 px | `styles.css:3400` |

Fixed chrome, **Tauri/native-menu mode** = 48 + 22 + 44 + 30 + 21 = **165 px**.
Fixed chrome, **browser mode** (in-DOM menu bar) = **192 px**.

Inside `.workspace-pane-viewport`: `.workspace-pane` has a 1 px border each side (`styles.css:3292`), and
`.viewport-shell` is `grid-template-rows: auto auto minmax(0,1fr)` (`styles.css:3298`) with
`.command-bar` min-height **38 px** (`styles.css:3303`) and `.viewport-toolbar` min-height **30 px** (`styles.css:3299`).
Fixed viewport chrome = 2 + 38 + 30 = **70 px**.
(`.viewport-intents` is `display:none` unless a creation tool is armed — `styles.css:1563`.)

### 6.2 Arithmetic — native-menu mode

```
canvas_h = H − 165 (shell chrome) − 12 (.workspace padding) − [dock ? (6 gap + dock_h) : 0] − 70 (pane+viewport chrome)
dock_h   = min( clamp(180, 0.30·H, 280) , 0.40 · (H − 177) )
```

| Window (CSS px) | Dock | dock_h | `.modeling-workspace` h | **canvas h** | % of window h |
|---|---|---:|---:|---:|---:|
| 1024 × 768 | **open** | 30vh = 230.4 (< 40% × 591 = 236.4) | 354.6 | **284.6 px** | 37.1 % |
| 1024 × 768 | **closed** | 0 | 591 | **521 px** | 67.8 % |
| 1280 × 800 | **open** | 30vh = 240 (< 40% × 623 = 249.2) | 377 | **307 px** | 38.4 % |
| 1280 × 800 | **closed** | 0 | 623 | **553 px** | 69.1 % |

Widths (`.workspace` inner width = W − 12; `gap: 6px` × 2):

| Window | Effective columns | viewport column | canvas content w | **canvas area (open)** | **canvas area (closed)** |
|---|---|---:|---:|---:|---:|
| 1024 × 768 (`@media max-width:1100px`, `styles.css:3415`) | 180 / 1fr / 260 | 560 | **558 px** | 558 × 284.6 = **158,800 px²** | 558 × 521 = **290,718 px²** |
| 1280 × 800 (`styles.css:3289`) | 215 / 1fr / 280 | 761 | **759 px** | 759 × 307 = **233,013 px²** | 759 × 553 = **419,727 px²** |

Share of the whole window area: 1024×768 = 786,432 px² → canvas is **20.2 %** (dock open) / **37.0 %** (dock closed).
1280×800 = 1,024,000 px² → **22.8 %** / **41.0 %**.
In browser mode subtract a further 27 px of canvas height (menu bar row).

### 6.3 Cross-check against the captured screenshot

File: `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/NATIVE/FINAL_NATIVE_1024x768.png`
(1024 × 768 px, confirmed with `sips -g pixelWidth -g pixelHeight`; committed in `831bd21d4`, 2026-09-08).

Measured with a Pillow scan script (scratchpad only) that walks a column at *x* = 780 and *x* = 620 and a row at
*y* = 430, flagging any pixel whose channel deltas exceed 8–12:

| Band | y-range (inclusive) | height (image px) |
|---|---|---:|
| native macOS title bar | 0–23 | 24 |
| `.titlebar` | 24–59 | 36 |
| border | 60 | 1 |
| `.project-strip` | 61–76 | 16 |
| `.workspace-toolbar` (+ border) | 77–109 | 33 |
| `.workspace` padding + pane border | 110–115 | 6 |
| `.command-bar` | 116–144 | 29 |
| `.viewport-toolbar` | 145–169 | 25 |
| **canvas (`.viewport-frame`)** | **170–467** | **298** |
| pane border + workspace gap | 468–474 | 7 |
| `.workspace-dock` (section "Results" open) | 475–688 | 214 |
| `.status-bar` (content wrapped onto 3 lines of hash text) | 689–752 | 64 |
| `.app-footer` | 753–767 | 15 |
| | | **768** |

Horizontal: viewport pane outer border at *x* = 172 and *x* = 803 → content **x = 173…802 = 630 px**.

**Measured canvas rectangle ≈ 630 × 298 = 187,740 image px² = 23.9 % of the 1024 × 768 image, 38.8 % of its height.**

**Caveat — the screenshot is not a 1024 × 768 *CSS-pixel* render.** Every fixed band is exactly 0.75 × its CSS
`min-height` (`.titlebar` 48 → 36, `.workspace-toolbar` 44 → 33, `.project-strip` 22 → 16, `.app-footer` 21 → 15).
The same density block was already present at the screenshot's commit
(`git show 831bd21d4:…/styles.css | grep -n 'Modeling workspace, September 2026'` → line 3261, same values), so the
0.75 factor is a capture downscale, not a CSS change. Dividing out: the window was ≈ **1365 × 1024 CSS px**
(1024/1365 = 768/1024 = 0.75), and the canvas was ≈ **840 × 397 CSS px**. That reconciles with §6.2:
992 CSS px of WebView height − 165 chrome − 12 padding − 6 gap − 280 dock (`clamp` ceiling) − 70 viewport chrome
− 62 wrapped status-bar overflow ≈ 397 px. Agreement with the model is within ~1 %.

---

## 7. Icons

```bash
node -e '/* parse every `import { … } from "lucide-react"` in non-test .ts/.tsx */'
```

| Metric | Value |
|---|---:|
| `lucide-react` import statements (non-test) | **50** |
| Files importing `lucide-react` | **50** of 117 (42.7 %) |
| **Distinct icon components imported** | **73** |

Accessibility, AlertTriangle, Anchor, Ban, BookOpen, Bot, Box, Boxes, ChartNoAxesCombined, CheckCheck, CheckSquare,
ChevronLeft, ChevronRight, Circle, CircleDot, CirclePlus, ClipboardCheck, ClipboardList, Crosshair, Database,
Download, EyeOff, FileCheck, FileJson, FileOutput, FilePlus, FileSearch, FileText, FileWarning, Fingerprint,
FolderOpen, GitBranch, GitCompare, HardDrive, KeyRound, Layers, Library, List, ListChecks, ListPlus, ListTree,
LockKeyhole, MousePointer2, MoveDown, PanelLeft, PanelRight, Pencil, Play, PlayCircle, PlusCircle, Printer, Puzzle,
Redo2, Save, Scale, Search, SearchCheck, Share2, ShieldAlert, ShieldCheck, SlidersHorizontal, Sparkles, Square,
SquareStack, Table2, Terminal, Trash2, Undo2, Wand2, Waypoints, Wrench, X, Zap.

---

## 8. Tests

```bash
find src \( -name '*.test.ts' -o -name '*.test.tsx' \) | wc -l                                  # 57
find src \( -name '*.test.ts' -o -name '*.test.tsx' \) -exec grep -ohE '\b(it|test)\(' {} + | wc -l   # 550
find src \( -name '*.test.ts' -o -name '*.test.tsx' \) -exec grep -ohE '\bdescribe\(' {} + | wc -l    # 102
find e2e -type f -name '*.spec.ts' | wc -l                                                     # 9
grep -ohE '\btest\(|\bit\(' e2e/*.spec.ts | wc -l                                              # 20
```

| Metric | Value |
|---|---:|
| Unit/component test files under `src` | **57** |
| `it(` cases | 543 |
| `test(` cases | 7 |
| **Total unit cases (approx., static `grep`)** | **550** |
| `describe(` blocks | 102 |
| Test LOC | 28,687 (55 % of non-test LOC) |
| `src/App.test.tsx` alone | 17,084 lines (59.6 % of all test LOC), 740 KB |
| **E2E spec files under `apps/desktop/e2e`** | **9** |
| E2E cases | **20** |

| E2E spec | cases | lines |
|---|---:|---:|
| `e2e/r2-smoke.spec.ts` | 10 | 1,870 |
| `e2e/gui-workflow-validation.spec.ts` | 3 | 418 |
| `e2e/linear-authoring.spec.ts` | 1 | 249 |
| `e2e/result-compatibility.spec.ts` | 1 | 73 |
| `e2e/result-compatibility-dist.spec.ts` | 1 | 60 |
| `e2e/workspace-layout.spec.ts` | 1 | 59 |
| `e2e/toolkit-batch-dist.spec.ts` | 1 | 53 |
| `e2e/report-package-dist.spec.ts` | 1 | 30 |
| `e2e/wasm-engine-dist.spec.ts` | 1 | 26 |

---

## 9. Toolkit catalogue (`src/features/toolkit/capabilityCatalog.ts`, 386 lines)

The source contains **29 object literals**, one of which is a `.map()` over `["copy","rotate","mirror"]`
(`capabilityCatalog.ts:291-300`), so the **runtime array holds 31 entries**. Counted by bundling and evaluating:

```bash
npx --no-install esbuild src/features/toolkit/capabilityCatalog.ts --bundle --format=cjs \
  --outfile=$SCRATCH/cat.cjs --external:../../types
node -e 'const m=require("$SCRATCH/cat.cjs"); /* tally group/status */'
```

| Group | entries | supported | partial | unavailable | gated |
|---|---:|---:|---:|---:|---:|
| Loads | **7** | 7 | 0 | 0 | 0 |
| Properties | **6** | 5 | 1 | 0 | 0 |
| Edit | **6** | 5 | 1 | 0 | 0 |
| Build | **4** | 3 | 1 | 0 | 0 |
| Supports | **4** | 4 | 0 | 0 | 0 |
| Select and View | **2** | 2 | 0 | 0 | 0 |
| Review | **2** | 1 | 1 | 0 | 0 |
| **Total** | **31** | **27** | **4** | **0** | **0** |

| Metric | Value |
|---|---:|
| `toolkitGroups` declared | **7** |
| Entries with a `route` | 29 |
| Entries with `history` (undo/redo) instead of a route | 2 |
| Entries with a `requires` gate (may render disabled) | **9** — `support-context`×3, `load`×2, `material`, `pipe`, `removable`, `wind` |
| Entries with `status: "unavailable"` or `"gated"` | **0** (both are declared in the union at `capabilityCatalog.ts:14` but never used) |
| `toolkitRoadmap` entries (rendered in a "Deferred roadmap" `<details>`, `ToolkitPalette.tsx:129`) | **3** — "Node renumbering", "Snubbers", "Cold spring" |

So "roadmap/unavailable" = **3 roadmap labels + 0 unavailable statuses + 4 `partial` statuses**; 9 of 31 can be
rendered disabled depending on the current selection.

---

## 10. Runtime dependencies (`apps/desktop/package.json`)

```bash
cat package.json
```

| Package | Version range |
|---|---|
| `@tauri-apps/api` | `^2.0.0` |
| `@vitejs/plugin-react` | `^5.0.0` |
| `three` | `^0.181.0` |
| `vite` | `^7.2.0` |
| `react` | `^19.0.0` |
| `react-dom` | `^19.0.0` |
| `lucide-react` | `^0.468.0` |

**7 runtime dependencies.** (`vite` and `@vitejs/plugin-react` are declared under `dependencies`, not `devDependencies`.)
No UI-component library, no CSS framework, no state-management library, no router, no charting library, no form library.

12 `devDependencies`: `@playwright/test ^1.60.0`, `@tauri-apps/cli ^2.0.0`, `@testing-library/jest-dom ^6.6.0`,
`@testing-library/react ^16.1.0`, `@types/node ^22.10.0`, `@types/react ^19.0.0`, `@types/react-dom ^19.0.0`,
`@types/three ^0.181.0`, `@vitest/coverage-v8 ^4.1.7`, `jsdom ^25.0.0`, `typescript ^5.7.0`, `vitest ^4.1.7`.

---

## Methods summary

* Host: macOS (Darwin 25.6.0), zsh, Node v24.18.0, Python 3.13 with Pillow 12.3.0, esbuild 0.27.7 (`npx --no-install`).
* All counts were produced with `find` / `grep` / `wc` / `sort` / `uniq` / `awk`, three throwaway Node scripts, and one
  Pillow script. Every helper script was written to the session scratchpad
  (`…/32ce21ab-1369-4dbb-b335-c9e6ea910a67/scratchpad/work/`), never to the repository.
* No repository file was created, modified, or deleted.
* Known imprecision, stated where it applies:
  * §3 counts **static source occurrences** of JSX tags, not rendered DOM nodes; elements inside `.map()` render more.
  * §4 is an explicitly-described textual **proxy** for rendered strings (see §4.1); it both over- and under-counts.
  * §5 "rules" = count of `{`, which includes the 4 `@media` wrapper braces (584 after subtracting them).
  * §6.2 assumes each chrome row renders at its CSS `min-height` (i.e. its content does not wrap); the screenshot in
    §6.3 shows the status bar wrapping to roughly double its floor, which removes a further ~30–60 px from the canvas.
  * §6.3 screenshot band edges are ±1 px, read from single-column scans.
  * §9 required evaluating the module because one catalogue entry is generated by a spread `.map()`.

---

## Notable numbers

57 feature directories and 50 `*Panel.tsx` components feed **51 panel imports into a single 3,709-line `App.tsx`**;
**31 of those 50 panels (62 %) contain no `<button>` at all**. Ten dock sections are **all mounted at once** and hidden
with CSS, with 15 components in `exports` and 12 in `operations` alone. The app exposes **31 menu commands across 5
menus**, 6 toolbar task stops, 6 command-bar buttons and 2 status-bar drawer openers, against only **175 `<button>`,
117 `<input>`, 79 `<select>`, 4 `<textarea>`, 26 `<details>` and 3 `<table>` static occurrences** — and **0 `<form>`
elements**. Instrumentation runs to **829 `data-testid` attributes (672 distinct static values) and 307 `aria-label`s
across 117 non-test files**, i.e. roughly one testid per 63 lines of non-test code. Machine vocabulary in renderable
text totals ~**2,220 hits** for 15 terms, led by **TBD 629, not_ 350, boundary 258, evidence 214, hash 190,
professional 146, invented 117**; `ExportReviewPanel.tsx` alone carries 170. `styles.css` is 3,438 lines / 588 rule
blocks with **151 distinct colour literals, 135 of them hex**, 12 font sizes, 7 radii, 12 shadows, **4 media queries,
0 `!important`**, and 174 `min-width`/`min-height` floors — of which the live shell floor is **760 × 540 px**. Only
**2 `clamp()`** declarations exist and only one is live: the dock's `clamp(180px, 30vh, 280px)`. That dock takes
230–240 px, leaving the 3D canvas **284.6 px tall at 1024 × 768 and 307 px at 1280 × 800 with a section open — 37–38 %
of window height and ~20–23 % of window area** — versus 521 px and 553 px with the dock closed; the measured screenshot
rectangle is 630 × 298 = 187,740 px², 23.9 % of the image. **73 distinct lucide icons** are imported across 50 files.
Tests are **57 unit files / ~550 cases / 28,687 lines (55 % of product LOC, 17,084 of them in one file)** plus **9 e2e
specs holding 20 cases**. All of it rests on **7 runtime dependencies** with no UI, CSS, state, router or form library.
