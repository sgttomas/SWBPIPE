# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: c3-viewport-visibility.spec.ts >> C3 deletion retains active empty snapshot; new geometry dims; project replacement clears it
- Location: e2e/c3-viewport-visibility.spec.ts:244:1

# Error details

```
Error: expect(locator).toBeFocused() failed

Locator:  getByTestId('clear-model-tree-filter')
Expected: focused
Received: inactive
Timeout:  10000ms

Call log:
  - Expect "toBeFocused" with timeout 10000ms
  - waiting for getByTestId('clear-model-tree-filter')
    24 × locator resolved to <button type="button" aria-label="Clear model tree filter" data-testid="clear-model-tree-filter">…</button>
       - unexpected value "inactive"

```

```yaml
- main:
  - navigation "Application menu":
    - button "File"
    - button "Edit"
    - button "View"
    - button "Insert"
    - button "Analyze"
  - heading "SWBPIPE" [level=1]
  - paragraph: Generated UI local-render-origin precision probe · Edited
  - group "Editing tools":
    - button "Undo model edit"
    - button "Redo model edit" [disabled]
    - button "Select" [pressed]
  - group "View":
    - button "Table"
    - button "Model"
    - button "Both" [pressed]
  - button "Run"
  - button "Issues, 2": Issues 2
  - group "Panels":
    - button "Inspector" [expanded]
    - button "Agent" [disabled]
  - combobox "Display units":
    - option "Entered" [selected]
    - option "SI"
    - option "US"
  - group
  - region "Human toolkit":
    - button "Find modeling commands": Search or command… ⌘K
  - navigation "Stages":
    - list:
      - listitem:
        - button "Model" [pressed]
      - listitem:
        - button "Loads"
      - listitem:
        - button "Results" [disabled]
      - listitem:
        - button "Review" [disabled]
    - separator
    - list:
      - listitem:
        - button "Libraries"
      - listitem:
        - button "Rules"
      - listitem:
        - button "Issues, 2": Issues
  - region "Modeling workspace":
    - group "Tables":
      - button "Model"
      - button "Review changes" [pressed]
      - button "Collapse table drawer" [disabled] [expanded]
    - region "Operation Apply section":
      - navigation "Editing and review tools":
        - button "Review changes" [pressed]
        - button "Geometry"
        - button "Supports"
        - button "Self weight"
        - button "Agent"
        - button "Details"
      - region "Batch review":
        - heading "Batch review" [level=3]
        - paragraph: 0 queued batches; 0 batches applied this session.
        - paragraph: Validate to preview all steps on temporary state. Apply commits the entire batch once or leaves the model unchanged. Batch drafts and receipts shown here belong to this session.
        - button "Clear pending changes and batches" [disabled]
      - region "Apply structured operations":
        - text: Apply Operations
        - paragraph: 0 queued; 1 applied this session. Applying validates through the structured-operation seam and replaces the session model document; saving the project stores it locally.
        - text: Engine ready 0 unit-bearing queued, 0 dimensionless queued; passed; 1 applied receipts.
        - button "Undo"
        - button "Redo" [disabled]
        - text: 1 undo / 0 redo
        - paragraph: Applied op:delete-node-node:C3-loose to the session model; previous solve results were cleared. Run a new solve, then save the project to store the edited model locally.
        - paragraph: Queue a review intent from the property inspector or viewport to validate and apply it here.
        - text: Applied this session
        - article:
          - strong: op:delete-node-node:C3-loose
          - text: "Node node:C3-loose; nodes: node:C3-loose; x=0; y=-3; z=0 to not_present Applied through local_wasm_engine. Acceptance basis user_initiated_apply_in_local_session; persistence session_state_only_not_yet_saved; professional approval not recorded. model_hash=sha256:6637777a924da8a6a…"
        - text: Applying a structured operation records a user-initiated local-session acceptance only. Results of earlier solves are cleared because they no longer describe the edited model.
    - separator "Resize table and canvas"
    - group "Viewport controls":
      - text: 3D Centerline
      - group "Viewport deformation overlay status": Deformation · unavailable
      - group "Viewport display toggles":
        - button "Labels" [pressed]
        - button "Loads" [pressed]
        - button "Grid" [pressed]
      - group "Viewport selection tools":
        - button "Box Select"
        - text: Selection filter
        - combobox "Selection filter":
          - option "All" [selected]
          - option "Pipes"
          - option "Nodes"
          - option "Supports"
          - option "Components"
        - button "Hide" [disabled]
        - button "Isolate" [disabled]
        - button "Show All"
        - text: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
        - button "Fit Model"
        - button "Fit Visible"
        - button "Fit Selection" [disabled]
      - group "Viewport geometry":
        - button "Schematic" [pressed]
        - button "Actual OD"
        - button "Measure"
    - text: "Selected: project:UIF-PRECISION-ORIGIN"
    - status "Schematic centerline geometry"
    - status "View command status": 1 selected item isolated; other shown geometry is dimmed to 20%.
    - button "Select C3 A in viewport": C3-A
    - button "Select C3 B in viewport": C3-B
    - button "Select node:C3-A0 in viewport": C3-A0
    - button "Select node:C3-A1 in viewport": C3-A1
    - button "Select node:C3-B0 in viewport": C3-B0
    - button "Select node:C3-B1 in viewport": C3-B1
    - img "Orientation gizmo showing X, Y, Z axes"
    - button "Front"
    - button "Top"
    - button "Isometric" [pressed]
    - text: 1 m
    - region "Command and selection bar":
      - button "Node"
      - button "Pipe"
      - button "Support"
      - button "Component"
      - button "Load"
      - text: Select
      - group: Selection & navigation
    - button "Close inspector"
    - region "Property inspector":
      - 'heading "Generated UI local-render-origin precision probe — project: project:UIF-PRECISION-ORIGIN" [level=2]'
      - tablist "Inspector views":
        - tab "Properties" [selected]
        - tab "Task"
      - tabpanel:
        - group: All properties
      - group: Sources and units
      - group: New support configuration
      - group: New section
      - group: New material
      - group: New support
      - group: New component
  - complementary "Agent":
    - button "Agent" [disabled]
  - button "Solver · Not solved"
  - button "2 Issues"
  - text: "project: project:UIF-PRECISION-ORIGIN Entered"
  - button "About SWBPIPE…"
```

# Test source

```ts
  76  | 
  77  | const fixtureRoot = path.join(import.meta.dirname, "ui-foundation", "fixtures");
  78  | 
  79  | export async function readFixture(name: string): Promise<{ bytes: string; model: any }> {
  80  |   const bytes = await readFile(path.join(fixtureRoot, name), "utf8");
  81  |   return { bytes, model: JSON.parse(bytes) };
  82  | }
  83  | 
  84  | export async function gotoRoutedFixture(
  85  |   page: Page,
  86  |   name = "precision-origin-base.model.json",
  87  |   options: Readonly<{ captureGlobalRaf?: boolean }> = {},
  88  | ): Promise<any> {
  89  |   const fixture = await readFixture(name);
  90  |   await installInstrumentation(page, { captureGlobalRaf: options.captureGlobalRaf === true });
  91  |   await routeModelFixture(page, fixture);
  92  |   await page.goto("/");
  93  |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  94  |   await expect(page.getByTestId("viewport-canvas").locator("canvas")).toHaveCount(1);
  95  |   return fixture.model;
  96  | }
  97  | 
  98  | export async function waitForSettledGlobalRaf(page: Page, timeoutMs = 5_000): Promise<any> {
  99  |   return settledRafState(page, timeoutMs);
  100 | }
  101 | 
  102 | export async function gotoModel(page: Page, model: any): Promise<void> {
  103 |   await installInstrumentation(page, { captureGlobalRaf: false });
  104 |   await routeModelFixture(page, { bytes: JSON.stringify(model) });
  105 |   await page.goto("/");
  106 |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  107 |   await expect(page.getByTestId("viewport-canvas").locator("canvas")).toHaveCount(1);
  108 | }
  109 | 
  110 | export function typedTreeRow(page: Page, type: string, id: string): Locator {
  111 |   return page.getByTestId(treeRowTestId({ type, id }, "candidate"));
  112 | }
  113 | 
  114 | // Slice B3: "tree" is the table pane (its collapse chevron keeps `toggle-tree`; the pane is a
  115 | // drawer in Model view and below 1280 px, and always open elsewhere) and "inspector" is the
  116 | // docked inspector (the toolbar's Inspector toggle keeps `toggle-inspector`; it acts in Both view).
  117 | export async function ensureRail(page: Page, side: "tree" | "inspector", open: boolean): Promise<void> {
  118 |   const toggle = page.getByTestId(side === "tree" ? "toggle-tree" : "toggle-inspector");
  119 |   const expected = String(open);
  120 |   if (await toggle.getAttribute("aria-expanded") !== expected) {
  121 |     await activateWithKeyboard(page, toggle);
  122 |   }
  123 |   await expect(toggle).toHaveAttribute("aria-expanded", expected);
  124 | }
  125 | 
  126 | /** The model tree is the Model stage's first tab. */
  127 | export async function showModelTree(page: Page): Promise<void> {
  128 |   const host = page.getByTestId("shell-tree-host");
  129 |   // An open page lies over the tree without hiding it from the layout: close it first.
  130 |   const close = page.getByTestId("workspace-dock-close");
  131 |   if (await close.isVisible()) await activateWithKeyboard(page, close);
  132 |   if (!await host.isVisible()) {
  133 |     const stage = page.getByTestId("rail-stage-model");
  134 |     if (await stage.getAttribute("aria-current") !== "page") await activateWithKeyboard(page, stage);
  135 |     const tab = page.getByTestId("stage-tab-model-tree");
  136 |     if (await tab.getAttribute("aria-pressed") !== "true") await activateWithKeyboard(page, tab);
  137 |     await ensureRail(page, "tree", true);
  138 |   }
  139 |   await expect(host).toBeVisible();
  140 | }
  141 | 
  142 | export async function revealTreeRow(page: Page, type: string, id: string): Promise<Locator> {
  143 |   await showModelTree(page);
  144 |   await ensureRail(page, "tree", true);
  145 |   const filter = page.getByTestId("model-tree-filter-input");
  146 |   await filter.fill(id);
  147 |   const row = typedTreeRow(page, type, id);
  148 |   await expect(row).toBeVisible();
  149 |   return row;
  150 | }
  151 | 
  152 | export async function selectTreeRow(
  153 |   page: Page,
  154 |   type: string,
  155 |   id: string,
  156 |   modifiers: Readonly<{ shift?: boolean; toggle?: boolean }> = {},
  157 | ): Promise<Locator> {
  158 |   const row = await revealTreeRow(page, type, id);
  159 |   await row.click({
  160 |     modifiers: [
  161 |       ...(modifiers.shift ? ["Shift" as const] : []),
  162 |       ...(modifiers.toggle ? [process.platform === "darwin" ? "Meta" as const : "Control" as const] : []),
  163 |     ],
  164 |   });
  165 |   return row;
  166 | }
  167 | 
  168 | export async function clearTreeFilter(page: Page): Promise<void> {
  169 |   const clear = page.getByTestId("clear-model-tree-filter");
  170 |   if (await clear.isEnabled()) await activateWithKeyboard(page, clear);
  171 |   await expect(page.getByTestId("model-tree-filter-input")).toHaveValue("");
  172 | }
  173 | 
  174 | export async function activateWithKeyboard(page: Page, control: Locator): Promise<void> {
  175 |   await control.focus();
> 176 |   await expect(control).toBeFocused();
      |                         ^ Error: expect(locator).toBeFocused() failed
  177 |   await page.keyboard.press("Enter");
  178 | }
  179 | 
  180 | export async function setAppearance(
  181 |   page: Page,
  182 |   theme: AppearanceTheme,
  183 |   density: AppearanceDensity,
  184 | ): Promise<void> {
  185 |   // Slice B3: the two labelled selects live in the toolbar's Appearance disclosure.
  186 |   const appearance = page.getByTestId("toolbar-appearance");
  187 |   if (await appearance.getAttribute("open") === null) await appearance.locator("summary").click();
  188 |   await page.getByLabel("Appearance theme").selectOption(theme);
  189 |   await page.getByLabel("Workspace density").selectOption(density);
  190 |   await appearance.locator("summary").click();
  191 |   await expect(page.getByLabel("Appearance theme")).toBeHidden();
  192 |   const shell = page.getByTestId("desktop-preview-shell");
  193 |   await expect(shell).toHaveAttribute("data-theme", theme);
  194 |   await expect(shell).toHaveAttribute("data-theme-preference", theme);
  195 |   await expect(shell).toHaveAttribute("data-density", density);
  196 | }
  197 | 
  198 | export async function openWorkspaceSection(page: Page, id: string): Promise<Locator> {
  199 |   const section = page.getByTestId(`workspace-section-${id}`);
  200 |   if (!await section.isVisible()) {
  201 |     // Review changes is a tab of the Model stage's strip; from another stage it is summoned by the
  202 |     // section command, as every other section is.
  203 |     if (id === "operations" && await page.getByTestId("workspace-review").isVisible()) {
  204 |       await activateWithKeyboard(page, page.getByTestId("workspace-review"));
  205 |     } else {
  206 |       await activateWithKeyboard(page, page.getByTestId("menu-view"));
  207 |       await activateWithKeyboard(page, page.getByTestId(`menu-item-view.section.${id}`));
  208 |     }
  209 |   }
  210 |   await expect(section).toBeVisible();
  211 |   return section;
  212 | }
  213 | 
  214 | export async function currentModelHashThroughVisibleExport(page: Page): Promise<string> {
  215 |   await openWorkspaceSection(page, "project");
  216 |   const consent = page.getByTestId("project-validation-export-link-local-private-intent");
  217 |   if (!await consent.isChecked()) await consent.check();
  218 |   const link = page.getByTestId("project-validation-export-link");
  219 |   await expect.poll(async () => decodeModelHash(await link.getAttribute("href"))).toMatch(/^sha256:[0-9a-f]{64}$/);
  220 |   const hash = decodeModelHash(await link.getAttribute("href"));
  221 |   if (!hash) throw new Error("The visible project-validation export did not expose a model hash.");
  222 |   await activateWithKeyboard(page, page.getByTestId("workspace-dock-close"));
  223 |   return hash;
  224 | }
  225 | 
  226 | function decodeModelHash(href: string | null): string | null {
  227 |   if (!href?.includes(",")) return null;
  228 |   try {
  229 |     const payload = JSON.parse(decodeURIComponent(href.split(",").slice(1).join(",")));
  230 |     return typeof payload?.model_hash?.value === "string" ? payload.model_hash.value : null;
  231 |   } catch {
  232 |     return null;
  233 |   }
  234 | }
  235 | 
  236 | export async function expectWorkspaceGeometry(page: Page, viewport: ViewportSize): Promise<void> {
  237 |   const geometry = await page.evaluate(() => {
  238 |     const rect = (selector: string) => document.querySelector<HTMLElement>(selector)?.getBoundingClientRect().toJSON() ?? null;
  239 |     return {
  240 |       bodyOverflowX: document.documentElement.scrollWidth - document.documentElement.clientWidth,
  241 |       bodyOverflowY: document.documentElement.scrollHeight - document.documentElement.clientHeight,
  242 |       canvas: rect('[data-testid="viewport-canvas"]'),
  243 |       workspace: rect('[data-testid="modeling-workspace"]'),
  244 |       tree: rect(".workspace-pane-tree"),
  245 |       inspector: rect(".workspace-pane-inspector"),
  246 |       canvasPane: rect(".workspace-pane-viewport"),
  247 |       view: document.querySelector('[data-testid="modeling-workspace"]')?.getAttribute("data-view") ?? null,
  248 |       shell: rect('[data-testid="desktop-preview-shell"]'),
  249 |       window: { width: innerWidth, height: innerHeight },
  250 |     };
  251 |   });
  252 |   expect(geometry.window).toEqual(viewport);
  253 |   expect(geometry.bodyOverflowX).toBe(0);
  254 |   expect(geometry.bodyOverflowY).toBeLessThanOrEqual(1);
  255 |   // D3 keeps the original drawn-canvas floors and dominance checks. The sole
  256 |   // amendment is wide Both view, where the approved 55/45 allocation makes
  257 |   // the table wider than the canvas; assert that allocation explicitly.
  258 |   expect(["both", "model"]).toContain(geometry.view);
  259 |   const surfaces = geometry.workspace!;
  260 |   const inspectorDocked = (geometry.inspector?.width ?? 0) > 0;
  261 |   const narrow = viewport.width < 1280;
  262 |   if (geometry.view === "both" && !narrow) {
  263 |     expect(geometry.canvasPane!.width).toBeCloseTo(surfaces.width * 0.45, 0);
  264 |     expect(geometry.tree!.width).toBeCloseTo(surfaces.width * 0.55 - (inspectorDocked ? geometry.inspector!.width : 0), 0);
  265 |   } else {
  266 |     expect((geometry.canvas?.width ?? 0) * (geometry.canvas?.height ?? 0)).toBeGreaterThan(
  267 |       (geometry.tree?.width ?? 0) * (geometry.tree?.height ?? 0),
  268 |     );
  269 |   }
  270 |   expect(geometry.canvasPane!.width).toBeGreaterThanOrEqual(220);
  271 |   expect(geometry.canvas?.width).toBeGreaterThan(viewport.width * 0.35);
  272 |   expect(geometry.canvas?.height).toBeGreaterThan(viewport.height * 0.35);
  273 |   expect((geometry.canvas?.width ?? 0) * (geometry.canvas?.height ?? 0)).toBeGreaterThan(
  274 |     (geometry.inspector?.width ?? 0) * (geometry.inspector?.height ?? 0),
  275 |   );
  276 |   expect(geometry.canvas?.bottom).toBeLessThanOrEqual(viewport.height + 1);
```