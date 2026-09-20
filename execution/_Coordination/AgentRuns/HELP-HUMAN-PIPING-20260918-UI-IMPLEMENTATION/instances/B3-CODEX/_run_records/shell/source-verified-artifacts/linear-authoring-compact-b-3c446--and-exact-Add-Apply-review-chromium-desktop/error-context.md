# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: linear-authoring.spec.ts >> compact blank-to-straight authoring keeps the canvas and exact Add/Apply review
- Location: e2e/linear-authoring.spec.ts:58:1

# Error details

```
Test timeout of 120000ms exceeded.
```

```
Error: locator.click: Test timeout of 120000ms exceeded.
Call log:
  - waiting for getByTestId('toggle-tree')
    - locator resolved to <button type="button" aria-expanded="false" data-testid="toggle-tree" title="Expand table drawer" class="shell-drawer-chevron" aria-controls="shell-table-body" aria-label="Expand table drawer">…</button>
  - attempting click action
    2 × waiting for element to be visible, enabled and stable
      - element is visible, enabled and stable
      - scrolling into view if needed
      - done scrolling
      - <button type="button" title="Queue node delete intent" data-testid="queue-delete-node-intent">…</button> from <div id="shell-inspector" class="workspace-pane workspace-pane-inspector">…</div> subtree intercepts pointer events
    - retrying click action
    - waiting 20ms
    2 × waiting for element to be visible, enabled and stable
      - element is visible, enabled and stable
      - scrolling into view if needed
      - done scrolling
      - <button type="button" title="Queue node delete intent" data-testid="queue-delete-node-intent">…</button> from <div id="shell-inspector" class="workspace-pane workspace-pane-inspector">…</div> subtree intercepts pointer events
    - retrying click action
      - waiting 100ms
    228 × waiting for element to be visible, enabled and stable
        - element is visible, enabled and stable
        - scrolling into view if needed
        - done scrolling
        - <button type="button" title="Queue node delete intent" data-testid="queue-delete-node-intent">…</button> from <div id="shell-inspector" class="workspace-pane workspace-pane-inspector">…</div> subtree intercepts pointer events
      - retrying click action
        - waiting 500ms

```

# Page snapshot

```yaml
- main [ref=e3]:
  - navigation "Application menu" [ref=e4]:
    - button "File" [ref=e6] [cursor=pointer]
    - button "Edit" [ref=e8] [cursor=pointer]
    - button "View" [ref=e10] [cursor=pointer]
    - button "Insert" [ref=e12] [cursor=pointer]
    - button "Analyze" [ref=e14] [cursor=pointer]
  - generic "Toolbar" [ref=e15]:
    - heading "SWBPIPE" [level=1] [ref=e17]
    - group "Editing tools" [ref=e18]:
      - button "Undo model edit" [ref=e19] [cursor=pointer]:
        - img [ref=e20]
      - button "Redo model edit" [disabled] [ref=e23]:
        - img [ref=e24]
      - button "Select (⎋)" [ref=e27] [cursor=pointer]:
        - img [ref=e28]
    - group "View" [ref=e30]:
      - button "Table view (⌘1)" [ref=e32] [cursor=pointer]:
        - img [ref=e33]
      - button "Model view (⌘2)" [ref=e36] [cursor=pointer]:
        - img [ref=e37]
      - button "Both view (⌘3)" [pressed] [ref=e41] [cursor=pointer]:
        - img [ref=e42]
    - generic [ref=e44]:
      - button "Run" [ref=e45] [cursor=pointer]:
        - img [ref=e46]
        - generic [ref=e48]: Run
      - button "Issues, 5" [ref=e49] [cursor=pointer]:
        - img [ref=e50]
        - generic [ref=e52]: Issues
        - generic [ref=e53]: "5"
    - group "Panels" [ref=e54]:
      - button "Inspector" [expanded] [ref=e56] [cursor=pointer]:
        - img [ref=e57]
      - button "Agent" [disabled] [ref=e60]:
        - img [ref=e61]
    - generic "Display units" [ref=e64]:
      - combobox "Display units" [ref=e65]:
        - option "Entered" [selected]
        - option "SI"
        - option "US"
    - group [ref=e66]:
      - generic "Appearance" [ref=e67] [cursor=pointer]:
        - img [ref=e68]
      - option "System" [selected]
      - option "Light"
      - option "Dark"
      - option "Comfortable" [selected]
      - option "Compact"
    - region "Human toolkit" [ref=e70]:
      - button "Find modeling commands" [ref=e71] [cursor=pointer]:
        - img [ref=e72]
        - generic [ref=e75]: Search or command…
        - generic "Command K" [ref=e76]: ⌘K
  - generic [ref=e77]:
    - navigation "Stages" [ref=e78]:
      - list [ref=e79]:
        - listitem [ref=e80]:
          - button "Model" [pressed] [ref=e81] [cursor=pointer]:
            - img [ref=e82]
            - generic [ref=e85]: Model
        - listitem [ref=e86]:
          - button "Loads" [ref=e87] [cursor=pointer]:
            - img [ref=e88]
            - generic [ref=e92]: Loads
        - listitem [ref=e93]:
          - button "Results" [disabled] [ref=e94]:
            - img [ref=e95]
            - generic [ref=e98]: Results
        - listitem [ref=e99]:
          - button "Review" [disabled] [ref=e100]:
            - img [ref=e101]
            - generic [ref=e105]: Review
      - separator [ref=e106]
      - list [ref=e107]:
        - listitem [ref=e108]:
          - button "Libraries" [ref=e109] [cursor=pointer]:
            - img [ref=e110]
            - generic [ref=e112]: Libraries
        - listitem [ref=e113]:
          - button "Rules" [ref=e114] [cursor=pointer]:
            - img [ref=e115]
            - generic [ref=e119]: Rules
        - listitem [ref=e120]:
          - button "Issues, 5" [ref=e121] [cursor=pointer]:
            - img [ref=e122]
            - generic [ref=e124]: Issues
            - generic [ref=e125]: "5"
    - region "Modeling workspace" [ref=e127]:
      - group "Tables" [ref=e129]:
        - button "Model" [pressed] [ref=e130] [cursor=pointer]
        - button "Review changes" [ref=e131] [cursor=pointer]
        - button "Expand table drawer" [ref=e134] [cursor=pointer]:
          - img [ref=e135]
      - generic [ref=e138]:
        - generic [ref=e139]:
          - group "Viewport controls" [ref=e140]:
            - generic [ref=e141]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e142]:
              - generic "Deformation · unavailable" [ref=e143] [cursor=pointer]
            - group "Viewport display toggles" [ref=e144]:
              - button "Labels" [pressed] [ref=e145]
              - button "Loads" [pressed] [ref=e146]
              - button "Grid" [pressed] [ref=e147]
            - group "Viewport selection tools" [ref=e148]:
              - button "Box Select" [ref=e149]
              - generic [ref=e150]:
                - generic [ref=e151]: Selection filter
                - combobox "Selection filter" [ref=e152]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [ref=e153]
              - button "Isolate" [ref=e154]
              - button "Show All" [disabled] [ref=e155]
              - button "Fit Model" [ref=e156]
              - button "Fit Visible" [ref=e157]
              - button "Fit Selection" [ref=e158]
            - group "Viewport geometry" [ref=e159]:
              - button "Schematic" [pressed] [ref=e160]
              - button "Actual OD" [ref=e161]
              - button "Measure" [ref=e162]
          - generic "Viewport status" [ref=e163]:
            - 'generic "Selected node: node:UI-A-100" [ref=e164]': "Selected: node:UI-A-100"
            - status "Schematic centerline geometry" [ref=e165]
            - status "View command status" [ref=e166]: No view command dispatched.
        - generic [ref=e167]:
          - generic "Three.js pipe centerline viewport" [ref=e168]
          - generic "Viewport entity selection":
            - button "Select Anchor in viewport" [pressed] [ref=e170] [cursor=pointer]:
              - img [ref=e171]
              - generic [ref=e174]: UI-A-100
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e175]:
            - button "Front" [ref=e176] [cursor=pointer]
            - button "Top" [ref=e177] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e178] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e179]:
          - generic "Object creation tools" [ref=e180]:
            - button "Node" [pressed] [ref=e181] [cursor=pointer]:
              - img [ref=e182]
              - text: Node
            - button "Pipe" [ref=e184] [cursor=pointer]:
              - img [ref=e185]
              - text: Pipe
            - button "Support" [ref=e189] [cursor=pointer]:
              - img [ref=e190]
              - text: Support
            - button "Component" [ref=e193] [cursor=pointer]:
              - img [ref=e194]
              - text: Component
            - button "Load" [ref=e197] [cursor=pointer]:
              - img [ref=e198]
              - text: Load
          - 'generic "Node tool armed: click empty canvas to fill coordinates, then queue node." [ref=e200]': Node tool armed
          - group [ref=e201]:
            - generic "Selection & navigation" [ref=e202] [cursor=pointer]
      - generic [ref=e203]:
        - button "Close inspector" [ref=e204]:
          - img [ref=e205]
        - region "Viewport editor intents" [ref=e209]:
          - heading "Create node" [level=3] [ref=e210]
          - group [ref=e211]:
            - generic "Explicit node geometry" [ref=e212]:
              - generic [ref=e213]:
                - generic [ref=e214]: Node ID
                - textbox "New node ID" [ref=e215]:
                  - /placeholder: node:N-3
                  - text: node:UI-A-100
              - generic [ref=e216]:
                - generic [ref=e217]: Label
                - textbox "New node label" [ref=e218]:
                  - /placeholder: Node label
                  - text: Anchor
              - generic [ref=e219]:
                - generic [ref=e220]: X
                - textbox "New node X coordinate" [ref=e221]:
                  - /placeholder: "0"
                  - text: "0"
              - generic [ref=e222]:
                - generic [ref=e223]: "Y"
                - textbox "New node Y coordinate" [ref=e224]:
                  - /placeholder: "0"
                  - text: "2.4"
              - generic [ref=e225]:
                - generic [ref=e226]: Z
                - textbox "New node Z coordinate" [ref=e227]:
                  - /placeholder: "0"
                  - text: "0"
              - generic [ref=e228]:
                - generic [ref=e229]: Coordinate unit
                - combobox "New node coordinate unit" [ref=e230]:
                  - option "m" [selected]
              - generic [ref=e231]: "Coordinates: m, model metadata"
              - generic [ref=e232]: "Pointer plane: global XZ · Y=0 m"
              - generic "Click within 4 CSS pixels on the visible 3D canvas to capture on global XZ at Y=0." [ref=e233]
              - generic [ref=e234]:
                - generic [ref=e235]: Provenance
                - textbox "New node provenance" [ref=e236]: invented_synthetic_ui_acceptance_input
              - button "Add node" [disabled] [ref=e237]:
                - img [ref=e238]
                - text: Add node
              - generic [ref=e240]: The node ID is already reserved.
            - region "Route review" [ref=e241]:
              - heading "Review and Apply" [level=4] [ref=e242]
              - status [ref=e243]: The affected selection changed. Add again to review the current draft.
              - paragraph [ref=e244]: Add a complete node or route to generate the service validation and exact diff.
              - button "Apply" [disabled] [ref=e245]
            - group [ref=e246]:
              - generic "Unit source" [ref=e247] [cursor=pointer]
          - group [ref=e248]:
            - generic "Pending changes (0)" [ref=e249] [cursor=pointer]
        - region "Property inspector" [ref=e250]:
          - 'heading "Anchor — node: node:UI-A-100" [level=2] [ref=e251]':
            - text: Anchor
            - generic [ref=e252]: "— node: node:UI-A-100"
          - tablist "Inspector views" [ref=e253]:
            - tab "Properties" [selected] [ref=e254]
            - tab "Task" [ref=e255]
          - tabpanel [ref=e256]:
            - group [ref=e257]:
              - generic "All properties" [ref=e258] [cursor=pointer]
          - generic [ref=e259]:
            - group [ref=e260]:
              - generic "Sources and units" [ref=e261] [cursor=pointer]
            - group [ref=e262]:
              - generic "New support configuration" [ref=e263] [cursor=pointer]
              - option "Not provided (preserved)" [disabled] [selected]
              - option "Anchor"
              - option "Guide"
              - option "Line stop"
              - option "Vertical support"
              - option "Spring"
              - option "Variable spring hanger"
              - option "Spring hanger"
              - option "Constant-effort support"
              - option "Nonlinear support"
            - group [ref=e264]:
              - generic "New section" [ref=e265] [cursor=pointer]
              - option "pipe" [selected]
              - option "m, model metadata" [selected]
            - group [ref=e266]:
              - generic "New material" [ref=e267] [cursor=pointer]
              - option "Pa, model metadata" [selected]
              - option "1/C, model metadata" [selected]
            - group [ref=e268]:
              - generic "New support" [ref=e269] [cursor=pointer]
              - option "N/m, model metadata" [selected]
            - group [ref=e270]:
              - generic "New component" [ref=e271] [cursor=pointer]
              - option "bend" [selected]
              - option "tee"
              - option "reducer"
              - option "valve"
              - option "flange"
              - option "expansion_joint"
              - option "m, model metadata" [selected]
              - option "rad, model metadata" [selected]
            - region "Delete node intent" [ref=e272]:
              - heading "Delete node" [level=3] [ref=e273]
              - button "Queue delete node" [ref=e275] [cursor=pointer]:
                - img [ref=e276]
                - text: Queue delete node
              - article [ref=e279]:
                - generic [ref=e280]:
                  - generic [ref=e281]:
                    - generic [ref=e282]: Operation
                    - strong [ref=e283]: op:delete-node-node:UI-A-100; delete; proposed
                  - generic [ref=e284]:
                    - generic [ref=e285]: Target
                    - strong [ref=e286]: Node; node:UI-A-100
                  - generic [ref=e287]:
                    - generic [ref=e288]: Change
                    - strong [ref=e289]: delete_node; Explicit node deletion; nodes; before=Anchor; x=0; y=2.4; z=0; after=not_present
                  - generic [ref=e290]:
                    - generic [ref=e291]: Unit basis
                    - strong [ref=e292]: dimensionless; unit=none; explicit user-entered node deletion; endpoint and load reference integrity required
                  - generic [ref=e293]:
                    - generic [ref=e294]: Validation
                    - strong [ref=e295]: not_run; not_run; not_required_dimensionless; not_generated; not_applied
                  - generic [ref=e296]:
                    - generic [ref=e297]: Audit boundary
                    - strong [ref=e298]: Routed through the structured operations only; no direct model mutation; requires your acceptance; does not change the accepted model until applied
                  - generic [ref=e299]:
                    - generic [ref=e300]: Professional boundary
                    - strong [ref=e301]: Requires human review; no compliance claim; no approval claim
                  - generic [ref=e302]:
                    - generic [ref=e303]: Rationale
                    - strong [ref=e304]: explicit user-entered node deletion for project:blank-local-20260920t021718z; requires reference validation before durable model change.
    - complementary "Agent" [ref=e305]:
      - button "Agent" [disabled] [ref=e307]:
        - img [ref=e308]
        - generic [ref=e311]: Agent
  - generic "Workspace status" [ref=e312]:
    - generic "Analysis statuses" [ref=e313]:
      - button "Solver · Model incomplete" [ref=e315] [cursor=pointer]
    - button "5 Issues" [ref=e316] [cursor=pointer]:
      - img [ref=e317]
      - text: 5 Issues
    - generic "Selection" [ref=e319]: "node: node:UI-A-100"
    - generic "Display units" [ref=e320]: Entered
    - button "About SWBPIPE…" [ref=e321] [cursor=pointer]:
      - img [ref=e322]
```

# Test source

```ts
  1   | import { expect, type Locator, type Page } from "@playwright/test";
  2   | 
  3   | export type TreeEntityType =
  4   |   | "project"
  5   |   | "material"
  6   |   | "section"
  7   |   | "node"
  8   |   | "pipe"
  9   |   | "support"
  10  |   | "component"
  11  |   | "load"
  12  |   | "combination"
  13  |   | "diagnostic";
  14  | 
  15  | type WorkspaceSectionId =
  16  |   | "operations"
  17  |   | "loads"
  18  |   | "libraries"
  19  |   | "rule-packs"
  20  |   | "solve"
  21  |   | "results"
  22  |   | "report"
  23  |   | "project"
  24  |   | "exports"
  25  |   | "evidence";
  26  | 
  27  | export async function openWorkspaceSection(page: Page, sectionId: WorkspaceSectionId): Promise<Locator> {
  28  |   const section = page.getByTestId(`workspace-section-${sectionId}`);
  29  |   if (!await section.isVisible()) {
  30  |     // Review changes is a tab of the Model stage's strip; from another stage it is summoned by the
  31  |     // section command, as every other section is.
  32  |     if (sectionId === "operations" && await page.getByTestId("workspace-review").isVisible()) {
  33  |       await page.getByTestId("workspace-review").click();
  34  |     } else {
  35  |       await page.getByTestId("menu-view").click();
  36  |       await page.getByTestId(`menu-item-view.section.${sectionId}`).click();
  37  |     }
  38  |   }
  39  |   await expect(section).toBeVisible();
  40  |   if (sectionId === "operations") {
  41  |     const review = page.getByTestId("operation-tab-review");
  42  |     if (await review.getAttribute("aria-pressed") !== "true") await review.click();
  43  |     await expect(review).toHaveAttribute("aria-pressed", "true");
  44  |   }
  45  |   return section;
  46  | }
  47  | 
  48  | // Slice B3: the dock is gone. A page closes onto its stage by its close control;
  49  | // "no section on screen" is the Model stage's model tree, reached by the rail and the tab.
  50  | export async function closeWorkspacePanels(page: Page): Promise<void> {
  51  |   const close = page.getByTestId("workspace-dock-close");
  52  |   if (await close.isVisible()) await close.click();
  53  |   await showModelTree(page);
  54  |   await expect(page.locator('[data-testid^="workspace-section-"]:visible')).toHaveCount(0);
  55  | }
  56  | 
  57  | /** The model tree is the Model stage's first tab. */
  58  | export async function showModelTree(page: Page): Promise<void> {
  59  |   const host = page.getByTestId("shell-tree-host");
  60  |   // An open page lies over the tree without hiding it from the layout: close it first.
  61  |   const close = page.getByTestId("workspace-dock-close");
  62  |   if (await close.isVisible()) await close.click();
  63  |   if (!await host.isVisible()) {
  64  |     if (await page.getByTestId("rail-stage-model").getAttribute("aria-current") !== "page") await page.getByTestId("rail-stage-model").click();
  65  |     const tab = page.getByTestId("stage-tab-model-tree");
  66  |     if (await tab.getAttribute("aria-pressed") !== "true") await tab.click();
  67  |     await ensureRailExpanded(page, "toggle-tree");
  68  |   }
  69  |   await expect(host).toBeVisible();
  70  | }
  71  | 
  72  | /** Close the Both view's docked inspector, which takes its 300 px from the canvas. */
  73  | export async function ensureInspectorCollapsed(page: Page): Promise<void> {
  74  |   const toggle = page.getByTestId("toggle-inspector");
  75  |   if (await toggle.getAttribute("aria-disabled") !== "true" && await toggle.getAttribute("aria-expanded") === "true") await toggle.click();
  76  | }
  77  | 
  78  | export async function ensureTreeExpanded(page: Page): Promise<void> {
  79  |   await showModelTree(page);
  80  |   await ensureRailExpanded(page, "toggle-tree");
  81  | }
  82  | 
  83  | // The inspector is docked in Model view, opened by the toolbar's toggle in Both view,
  84  | // and absent in Table view: a test that needs it in Table view asks for Both view first.
  85  | export async function ensureInspectorExpanded(page: Page): Promise<void> {
  86  |   const toggle = page.getByTestId("toggle-inspector");
  87  |   if (await toggle.getAttribute("aria-disabled") === "true" && await toggle.getAttribute("aria-expanded") !== "true") {
  88  |     await page.getByTestId("view-switch-both").click();
  89  |   }
  90  |   await ensureRailExpanded(page, "toggle-inspector");
  91  | }
  92  | 
  93  | async function ensureRailExpanded(page: Page, testId: "toggle-tree" | "toggle-inspector"): Promise<void> {
  94  |   const toggle = page.getByTestId(testId);
> 95  |   if (await toggle.getAttribute("aria-expanded") !== "true") await toggle.click();
      |                                                                           ^ Error: locator.click: Test timeout of 120000ms exceeded.
  96  |   await expect(toggle).toHaveAttribute("aria-expanded", "true");
  97  | }
  98  | 
  99  | export function treeRowTestId(type: TreeEntityType, id: string): string {
  100 |   return `tree-row-${encodeURIComponent(type)}-${encodeURIComponent(id)}`;
  101 | }
  102 | 
  103 | export function treeEntity(page: Page, type: TreeEntityType, id: string): Locator {
  104 |   return page.getByTestId(treeRowTestId(type, id));
  105 | }
  106 | 
  107 | export async function revealTreeEntity(page: Page, type: TreeEntityType, id: string): Promise<Locator> {
  108 |   await ensureTreeExpanded(page);
  109 |   const filter = page.getByTestId("model-tree-filter-input");
  110 |   await filter.fill(id);
  111 |   const row = treeEntity(page, type, id);
  112 |   await expect(row).toBeVisible();
  113 |   return row;
  114 | }
  115 | 
  116 | export async function selectTreeEntity(page: Page, type: TreeEntityType, id: string): Promise<Locator> {
  117 |   const row = await revealTreeEntity(page, type, id);
  118 |   await row.click();
  119 |   await expect(row).toHaveAttribute("aria-selected", "true");
  120 |   return row;
  121 | }
  122 | 
  123 | export async function expectTreeEntity(
  124 |   page: Page,
  125 |   type: TreeEntityType,
  126 |   id: string,
  127 |   expectedText?: string
  128 | ): Promise<Locator> {
  129 |   const row = await revealTreeEntity(page, type, id);
  130 |   if (expectedText) await expect(row).toContainText(expectedText);
  131 |   return row;
  132 | }
  133 | 
  134 | export async function expectTreeEntityMissing(page: Page, type: TreeEntityType, id: string): Promise<void> {
  135 |   await ensureTreeExpanded(page);
  136 |   await page.getByTestId("model-tree-filter-input").fill(id);
  137 |   await expect(treeEntity(page, type, id)).toHaveCount(0);
  138 | }
  139 | 
  140 | export async function startPropertyTaskFromTreeEntity(
  141 |   page: Page,
  142 |   type: TreeEntityType,
  143 |   id: string
  144 | ): Promise<Locator> {
  145 |   await selectTreeEntity(page, type, id);
  146 |   return startPropertyTaskFromCurrentSelection(page, type, id);
  147 | }
  148 | 
  149 | export async function startPropertyTaskFromCurrentSelection(
  150 |   page: Page,
  151 |   type: TreeEntityType,
  152 |   id: string
  153 | ): Promise<Locator> {
  154 |   await ensureInspectorExpanded(page);
  155 |   const inspector = page.getByTestId("property-inspector");
  156 |   const taskTab = inspector.getByRole("tab", { name: "Task", exact: true });
  157 |   if (await taskTab.getAttribute("aria-selected") !== "true") await taskTab.click();
  158 |   await expect(taskTab).toHaveAttribute("aria-selected", "true");
  159 |   // This helper requests a fresh task, never inspection of a retained draft.
  160 |   const frozenTarget = inspector.getByTestId("inspector-frozen-task-target");
  161 |   if (await frozenTarget.isVisible()) {
  162 |     const cancel = inspector.getByTestId("cancel-editor-intent");
  163 |     await expect(cancel).toBeVisible();
  164 |     await expect(cancel).toBeEnabled();
  165 |     await cancel.click();
  166 |     await expect(frozenTarget).toHaveCount(0);
  167 |   }
  168 |   await expect(inspector.getByTestId("inspector-task-empty")).toContainText(`Current selection: ${type}: ${id}`);
  169 |   const start = inspector.getByTestId("inspector-start-task");
  170 |   await expect(start).toBeVisible();
  171 |   await expect(start).toBeEnabled();
  172 |   await start.click();
  173 |   await expect(frozenTarget).toBeVisible();
  174 |   await expect(frozenTarget).toContainText(`Draft target: ${type}: ${id}`);
  175 |   const panel = inspector.getByTestId("editor-intent-panel");
  176 |   await expect(panel).toBeVisible();
  177 |   return panel;
  178 | }
  179 | 
  180 | export async function chooseVirtualTarget(
  181 |   scope: Page | Locator,
  182 |   testId: string,
  183 |   value: string
  184 | ): Promise<Locator> {
  185 |   const picker = scope.getByTestId(testId);
  186 |   const search = picker.getByRole("combobox");
  187 |   await search.fill(value);
  188 |   const option = picker.getByRole("option", { name: new RegExp(`${escapeRegExp(value)}$`) });
  189 |   await expect(option).toBeVisible();
  190 |   await option.click();
  191 |   await expectVirtualTarget(scope, testId, value);
  192 |   return picker;
  193 | }
  194 | 
  195 | export async function chooseVirtualMultiTarget(
```