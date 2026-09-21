# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 Both with Inspector keeps pointer horizontal scrolling inside the coordinate table
- Location: e2e/b4-table-editing.spec.ts:80:1

# Error details

```
Error: expect(received).toBeGreaterThan(expected)

Expected: > 0
Received:   0

Call Log:
- Timeout 10000ms exceeded while waiting on the predicate
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
    - generic [ref=e16]:
      - heading "SWBPIPE" [level=1] [ref=e17]
      - paragraph [ref=e18]: Invented Utility Loop Preview
    - group "Editing tools" [ref=e19]:
      - button "Undo model edit" [disabled] [ref=e20]:
        - img [ref=e21]
      - button "Redo model edit" [disabled] [ref=e24]:
        - img [ref=e25]
      - button "Select" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
        - generic [ref=e31]: Select
    - group "View" [ref=e32]:
      - button "Table" [ref=e34] [cursor=pointer]:
        - img [ref=e35]
        - generic [ref=e37]: Table
      - button "Model" [ref=e39] [cursor=pointer]:
        - img [ref=e40]
        - generic [ref=e43]: Model
      - button "Both" [pressed] [ref=e45] [cursor=pointer]:
        - img [ref=e46]
        - generic [ref=e48]: Both
    - generic [ref=e49]:
      - button "Run" [ref=e50] [cursor=pointer]:
        - img [ref=e51]
        - generic [ref=e53]: Run
      - button "Issues, 5" [ref=e54] [cursor=pointer]:
        - img [ref=e55]
        - generic [ref=e57]: Issues
        - generic [ref=e58]: "5"
    - group "Panels" [ref=e59]:
      - button "Inspector" [expanded] [ref=e61] [cursor=pointer]:
        - img [ref=e62]
        - generic [ref=e64]: Inspector
      - button "Agent" [disabled] [ref=e66]:
        - img [ref=e67]
        - generic [ref=e70]: Agent
    - generic "Display units" [ref=e71]:
      - combobox "Display units" [ref=e72]:
        - option "Entered" [selected]
        - option "SI"
        - option "US"
    - group [ref=e73]:
      - generic "Appearance" [ref=e74] [cursor=pointer]:
        - img [ref=e75]
      - option "System" [selected]
      - option "Light"
      - option "Dark"
      - option "Comfortable" [selected]
      - option "Compact"
    - region "Human toolkit" [ref=e77]:
      - button "Find modeling commands" [ref=e78] [cursor=pointer]:
        - img [ref=e79]
        - generic [ref=e82]: Search or command…
        - generic "Command K" [ref=e83]: ⌘K
  - generic [ref=e84]:
    - navigation "Stages" [ref=e85]:
      - list [ref=e86]:
        - listitem [ref=e87]:
          - button "Model" [pressed] [ref=e88] [cursor=pointer]:
            - img [ref=e89]
            - generic [ref=e92]: Model
        - listitem [ref=e93]:
          - button "Loads" [ref=e94] [cursor=pointer]:
            - img [ref=e95]
            - generic [ref=e99]: Loads
        - listitem [ref=e100]:
          - button "Results" [disabled] [ref=e101]:
            - img [ref=e102]
            - generic [ref=e105]: Results
        - listitem [ref=e106]:
          - button "Review" [disabled] [ref=e107]:
            - img [ref=e108]
            - generic [ref=e112]: Review
      - separator [ref=e113]
      - list [ref=e114]:
        - listitem [ref=e115]:
          - button "Libraries" [ref=e116] [cursor=pointer]:
            - img [ref=e117]
            - generic [ref=e119]: Libraries
        - listitem [ref=e120]:
          - button "Rules" [ref=e121] [cursor=pointer]:
            - img [ref=e122]
            - generic [ref=e126]: Rules
        - listitem [ref=e127]:
          - button "Issues, 5" [ref=e128] [cursor=pointer]:
            - img [ref=e129]
            - generic [ref=e131]: Issues
            - generic [ref=e132]: "5"
    - region "Modeling workspace" [ref=e134]:
      - generic [ref=e135]:
        - group "Tables" [ref=e136]:
          - button "Model" [pressed] [ref=e137] [cursor=pointer]
          - button "Review changes" [ref=e138] [cursor=pointer]
          - button "Collapse table drawer" [disabled] [expanded] [ref=e141]:
            - img [ref=e142]
        - generic "Model tree" [ref=e146]:
          - generic [ref=e147]: Model
          - region "Layout grid mode" [ref=e148]:
            - button "Tree" [ref=e149]:
              - img [ref=e150]
              - text: Tree
            - button "Grid" [active] [pressed] [ref=e153]:
              - img [ref=e154]
              - text: Grid
          - region "Model tree filtering" [ref=e156]:
            - generic [ref=e157]:
              - img [ref=e158]
              - generic [ref=e161]: Filter model
              - searchbox "Filter model tree" [ref=e162]
            - generic [ref=e163]: 27 of 27 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e164]:
              - img [ref=e165]
          - region "Bulk entity grid" [ref=e169]:
            - generic "Grid entity type" [ref=e170]:
              - button "Nodes" [pressed] [ref=e171]
              - button "Pipes" [ref=e172]
              - button "Supports" [ref=e173]
              - button "Materials" [ref=e174]
              - button "Sections" [ref=e175]
              - button "Components" [ref=e176]
              - button "Load Cases" [ref=e177]
              - button "Combinations" [ref=e178]
            - generic [ref=e180]:
              - grid "Node coordinates" [ref=e181]:
                - row "Node Sort X Sort Y Sort Z" [ref=e182]:
                  - columnheader "Node" [ref=e183]
                  - columnheader "Sort X" [ref=e184]:
                    - button "Sort X" [ref=e185]: X [m] ↕
                  - columnheader "Sort Y" [ref=e186]:
                    - button "Sort Y" [ref=e187]: Y [m] ↕
                  - columnheader "Sort Z" [ref=e188]:
                    - button "Sort Z" [ref=e189]: Z [m] ↕
                - rowgroup [ref=e190]:
                  - generic [ref=e191]:
                    - 'row "node:N-100 node:N-100 X: 0 m node:N-100 Y: 0 m node:N-100 Z: 0 m" [ref=e193]':
                      - rowheader "node:N-100" [ref=e194]:
                        - button "node:N-100" [ref=e195]
                      - 'gridcell "node:N-100 X: 0 m" [ref=e196]':
                        - 'button "node:N-100 X: 0 m" [ref=e197]': "0"
                      - 'gridcell "node:N-100 Y: 0 m" [ref=e198]':
                        - 'button "node:N-100 Y: 0 m" [ref=e199]': "0"
                      - 'gridcell "node:N-100 Z: 0 m" [ref=e200]':
                        - 'button "node:N-100 Z: 0 m" [ref=e201]': "0"
                    - 'row "node:N-110 node:N-110 X: 3.2 m node:N-110 Y: 0 m node:N-110 Z: 0 m" [ref=e203]':
                      - rowheader "node:N-110" [ref=e204]:
                        - button "node:N-110" [ref=e205]
                      - 'gridcell "node:N-110 X: 3.2 m" [ref=e206]':
                        - 'button "node:N-110 X: 3.2 m" [ref=e207]': "3.2"
                      - 'gridcell "node:N-110 Y: 0 m" [ref=e208]':
                        - 'button "node:N-110 Y: 0 m" [ref=e209]': "0"
                      - 'gridcell "node:N-110 Z: 0 m" [ref=e210]':
                        - 'button "node:N-110 Z: 0 m" [ref=e211]': "0"
                    - 'row "node:N-120 node:N-120 X: 3.2 m node:N-120 Y: 2.4 m node:N-120 Z: 0 m" [ref=e213]':
                      - rowheader "node:N-120" [ref=e214]:
                        - button "node:N-120" [ref=e215]
                      - 'gridcell "node:N-120 X: 3.2 m" [ref=e216]':
                        - 'button "node:N-120 X: 3.2 m" [ref=e217]': "3.2"
                      - 'gridcell "node:N-120 Y: 2.4 m" [ref=e218]':
                        - 'button "node:N-120 Y: 2.4 m" [ref=e219]': "2.4"
                      - 'gridcell "node:N-120 Z: 0 m" [ref=e220]':
                        - 'button "node:N-120 Z: 0 m" [ref=e221]': "0"
                    - 'row "node:N-130 node:N-130 X: 7.6 m node:N-130 Y: 2.4 m node:N-130 Z: 0 m" [ref=e223]':
                      - rowheader "node:N-130" [ref=e224]:
                        - button "node:N-130" [ref=e225]
                      - 'gridcell "node:N-130 X: 7.6 m" [ref=e226]':
                        - 'button "node:N-130 X: 7.6 m" [ref=e227]': "7.6"
                      - 'gridcell "node:N-130 Y: 2.4 m" [ref=e228]':
                        - 'button "node:N-130 Y: 2.4 m" [ref=e229]': "2.4"
                      - 'gridcell "node:N-130 Z: 0 m" [ref=e230]':
                        - 'button "node:N-130 Z: 0 m" [ref=e231]': "0"
                    - 'row "node:N-140 node:N-140 X: 7.6 m node:N-140 Y: 2.4 m node:N-140 Z: 2.2 m" [ref=e233]':
                      - rowheader "node:N-140" [ref=e234]:
                        - button "node:N-140" [ref=e235]
                      - 'gridcell "node:N-140 X: 7.6 m" [ref=e236]':
                        - 'button "node:N-140 X: 7.6 m" [ref=e237]': "7.6"
                      - 'gridcell "node:N-140 Y: 2.4 m" [ref=e238]':
                        - 'button "node:N-140 Y: 2.4 m" [ref=e239]': "2.4"
                      - 'gridcell "node:N-140 Z: 2.2 m" [ref=e240]':
                        - 'button "node:N-140 Z: 2.2 m" [ref=e241]': "2.2"
              - group "Node coordinates footer" [ref=e242]:
                - generic [ref=e243]: 5 of 5 rows
            - group [ref=e244]:
              - generic "Review multiple changes" [ref=e245] [cursor=pointer]
      - separator "Resize table and canvas" [ref=e246]
      - generic [ref=e248]:
        - generic [ref=e249]:
          - group "Viewport controls" [ref=e250]:
            - generic [ref=e251]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e252]:
              - generic "Deformation · unavailable" [ref=e253] [cursor=pointer]
            - group "Viewport display toggles" [ref=e254]:
              - button "Labels" [pressed] [ref=e255]
              - button "Loads" [pressed] [ref=e256]
              - button "Grid" [pressed] [ref=e257]
            - group "Viewport selection tools" [ref=e258]:
              - button "Box Select" [ref=e259]
              - generic [ref=e260]:
                - generic [ref=e261]: Selection filter
                - combobox "Selection filter" [ref=e262]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e263]
              - button "Isolate" [disabled] [ref=e264]
              - button "Show All" [disabled] [ref=e265]
              - generic [ref=e266]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e267]
              - button "Fit Visible" [ref=e268]
              - button "Fit Selection" [disabled] [ref=e269]
            - group "Viewport geometry" [ref=e270]:
              - button "Schematic" [pressed] [ref=e271]
              - button "Actual OD" [ref=e272]
              - button "Measure" [ref=e273]
          - generic "Viewport status" [ref=e274]:
            - 'generic "Selected project: project:invented-loop-01" [ref=e275]': "Selected: project:invented-loop-01"
            - status "Schematic centerline geometry" [ref=e276]
            - status "View command status" [ref=e277]: No view command dispatched.
        - generic [ref=e278]:
          - generic "Three.js pipe centerline viewport" [ref=e279]
          - generic "Viewport entity selection":
            - button "Select Vertical riser in viewport" [ref=e281] [cursor=pointer]:
              - img [ref=e282]
              - generic [ref=e286]: P-110
            - button "Select Invented branch connection marker in viewport" [ref=e287] [cursor=pointer]:
              - img [ref=e288]
              - generic [ref=e291]: C-120
            - button "Select Low point elbow in viewport" [ref=e292] [cursor=pointer]:
              - img [ref=e293]
              - generic [ref=e296]: N-110
            - button "Select Riser elbow in viewport" [ref=e297] [cursor=pointer]:
              - img [ref=e298]
              - generic [ref=e301]: N-120
            - button "Select Rack span in viewport" [ref=e302] [cursor=pointer]:
              - img [ref=e303]
              - generic [ref=e307]: P-120
            - button "Select Pump discharge run in viewport" [ref=e308] [cursor=pointer]:
              - img [ref=e309]
              - generic [ref=e313]: P-100
            - button "Select Tie-in rise in viewport" [ref=e314] [cursor=pointer]:
              - img [ref=e315]
              - generic [ref=e319]: P-130
            - button "Select Invented semi-rigid valve marker in viewport" [ref=e320] [cursor=pointer]:
              - img [ref=e321]
              - generic [ref=e324]: C-130
            - button "Select Pump nozzle in viewport" [ref=e325] [cursor=pointer]:
              - img [ref=e326]
              - generic [ref=e329]: N-100
            - button "Select Rack turn in viewport" [ref=e330] [cursor=pointer]:
              - img [ref=e331]
              - generic [ref=e334]: N-130
            - button "Select Terminal tie-in in viewport" [ref=e335] [cursor=pointer]:
              - img [ref=e336]
              - generic [ref=e339]: N-140
            - button "Select Preview sliding-friction rack shoe in viewport" [ref=e340] [cursor=pointer]:
              - img [ref=e341]
              - generic [ref=e344]: NL-130-FRIC
            - button "Select Preview one-way terminal stop in viewport" [ref=e345] [cursor=pointer]:
              - img [ref=e346]
              - generic [ref=e349]: NL-140
            - button "Select Anchor at pump nozzle in viewport" [ref=e350] [cursor=pointer]:
              - img [ref=e351]
              - generic [ref=e354]: S-100
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e355]:
            - button "Front" [ref=e356] [cursor=pointer]
            - button "Top" [ref=e357] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e358] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e359]:
          - generic "Object creation tools" [ref=e360]:
            - button "Node" [ref=e361] [cursor=pointer]:
              - img [ref=e362]
              - text: Node
            - button "Pipe" [ref=e364] [cursor=pointer]:
              - img [ref=e365]
              - text: Pipe
            - button "Support" [ref=e369] [cursor=pointer]:
              - img [ref=e370]
              - text: Support
            - button "Component" [ref=e373] [cursor=pointer]:
              - img [ref=e374]
              - text: Component
            - button "Load" [ref=e377] [cursor=pointer]:
              - img [ref=e378]
              - text: Load
          - generic "Model focus" [ref=e380]: Select
          - group [ref=e381]:
            - generic "Selection & navigation" [ref=e382] [cursor=pointer]
      - generic [ref=e383]:
        - button "Close inspector" [ref=e384]:
          - img [ref=e385]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e388]:
          - 'heading "Invented Utility Loop Preview — project: project:invented-loop-01" [level=2] [ref=e389]':
            - text: Invented Utility Loop Preview
            - generic [ref=e390]: "— project: project:invented-loop-01"
          - tablist "Inspector views" [ref=e391]:
            - tab "Properties" [selected] [ref=e392]
            - tab "Task" [ref=e393]
          - tabpanel [ref=e394]:
            - group [ref=e395]:
              - generic "All properties" [ref=e396] [cursor=pointer]
          - generic [ref=e397]:
            - group [ref=e398]:
              - generic "Sources and units" [ref=e399] [cursor=pointer]
            - group [ref=e400]:
              - generic "New support configuration" [ref=e401] [cursor=pointer]
              - text: ▾
            - group [ref=e402]:
              - generic "New section" [ref=e403] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e404]:
              - generic "New material" [ref=e405] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e406]:
              - generic "New support" [ref=e407] [cursor=pointer]
              - text: ▾
            - group [ref=e408]:
              - generic "New component" [ref=e409] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e410]:
      - button "Agent" [disabled] [ref=e412]:
        - img [ref=e413]
        - generic [ref=e416]: Agent
  - generic "Workspace status" [ref=e417]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e418] [cursor=pointer]:
      - img [ref=e419]
      - text: 5 Issues
    - generic "Selection" [ref=e421]: "project: project:invented-loop-01"
    - generic "Display units" [ref=e422]: Entered
    - button "About SWBPIPE…" [ref=e423] [cursor=pointer]:
      - img [ref=e424]
```

# Test source

```ts
  1   | import { expect, test } from "@playwright/test";
  2   | import { attachBrowserIdentity, currentModelHashThroughVisibleExport, gotoRoutedFixture } from "./ui-foundation-workflows";
  3   | import { ensureTreeExpanded, openWorkspaceSection, showModelTree } from "./workspace-driver";
  4   | 
  5   | // One connected journey per configured source viewport; no explicit-size repetition.
  6   | test("B4 node coordinates apply through one operation with keyboard, history and saved-state ownership", async ({ page, browser }, info) => {
  7   |   await attachBrowserIdentity(browser, info);
  8   |   await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  9   |   await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  10  |   const table = page.getByTestId("engineering-table");
  11  |   const cell = page.getByTestId("table-cell-node:N-100-y");
  12  |   await cell.click(); await expect(table.getByRole("button", { name: "Apply", exact: true })).toHaveCount(0);
  13  |   await cell.click(); const editor = table.getByRole("textbox", { name: "node:N-100 Y [m]" });
  14  |   await editor.fill("invalid"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  15  |   await expect(editor).toHaveValue("invalid"); await expect(editor).toHaveAttribute("aria-invalid", "true");
  16  |   await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  17  |   await expect(editor).toHaveValue("invalid"); await table.getByRole("button", { name: "Cancel", exact: true }).click();
  18  |   await expect(cell).toHaveText("0");
  19  |   // A valid draft must also cancel without blur submitting an operation first.
  20  |   await cell.dblclick(); await editor.fill("4.6"); await table.getByRole("button", { name: "Cancel", exact: true }).click();
  21  |   await expect(cell).toHaveText("0"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  22  |   await expect(page.getByTestId("project-edited")).toHaveCount(0);
  23  |   await cell.dblclick(); await editor.fill("0.5"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  24  |   await expect(cell).toHaveText("0.5"); await expect(page.getByTestId("project-edited")).toBeVisible();
  25  |   await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText("0"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  26  |   await page.getByTestId("workspace-redo").click(); await expect(cell).toHaveText("0.5");
  27  |   await table.getByRole("button", { name: "Sort X", exact: true }).click(); await table.getByRole("button", { name: "Sort X", exact: true }).click();
  28  |   await cell.focus(); await page.keyboard.press("Enter"); await editor.fill("0.75"); await page.keyboard.press("Enter");
  29  |   await expect(cell).toHaveText("0.75");
  30  |   await expect(page.getByTestId("table-cell-node:N-110-y")).toBeFocused();
  31  |   await page.keyboard.press("Tab"); await expect(page.getByTestId("table-cell-node:N-110-z")).toBeFocused();
  32  |   await page.keyboard.press("2"); await page.keyboard.press("Escape"); await expect(page.getByTestId("table-cell-node:N-110-z")).toHaveText("0");
  33  |   // The boundaries must leave the coordinate control, including blur-driven no-op Apply.
  34  |   await table.getByRole("button", { name: /Sorted by/ }).click();
  35  |   const first = table.locator("[data-table-cell]").first(); await first.focus(); await page.keyboard.press("Shift+Tab"); await expect(first).not.toBeFocused();
  36  |   const last = table.locator("[data-table-cell]").last(); const lastLabel = await last.getAttribute("aria-label");
  37  |   await last.focus(); await page.keyboard.press("Tab"); await expect(last).not.toBeFocused();
  38  |   await last.dblclick(); const lastEditor = table.getByRole("textbox"); await page.keyboard.press("Tab");
  39  |   await expect(lastEditor).toHaveCount(0); await expect(last).not.toBeFocused();
  40  |   expect(await page.evaluate(() => document.activeElement !== document.body)).toBe(true);
  41  |   expect(lastLabel).toBeTruthy();
  42  |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click();
  43  |   await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project"); await expect(page.getByTestId("project-edited")).toHaveCount(0);
  44  |   await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  45  |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await expect(cell).toHaveText("0.75"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  46  |   await page.screenshot({ path: info.outputPath("b4-reopened-coordinate.png") });
  47  | });
  48  | 
  49  | 
  50  | test("B4 virtualized invalid editor survives scrolling and a filter threshold without changing the model", async ({ page, browser }, info) => {
  51  |   await attachBrowserIdentity(browser, info);
  52  |   const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  53  |   const hashBefore = await currentModelHashThroughVisibleExport(page);
  54  |   await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  55  |   const table = page.getByTestId("engineering-table");
  56  |   const first = model.nodes[0]; const last = model.nodes.at(-1);
  57  |   const cell = page.getByTestId(`table-cell-${first.id}-x`);
  58  |   await cell.dblclick(); const editor = table.getByRole("textbox", { name: `${first.id} X [${model.project.units.length}]` });
  59  |   await editor.fill("retained invalid coordinate"); await editor.press("Enter");
  60  |   await expect(editor).toHaveAttribute("aria-invalid", "true"); await expect(editor).toBeFocused();
  61  |   const rows = page.getByTestId("engineering-table-rows"); await rows.hover(); await page.mouse.wheel(0, 2400);
  62  |   await expect.poll(() => rows.evaluate((element) => element.scrollTop)).toBeGreaterThan(1000);
  63  |   await expect(editor).toHaveValue("retained invalid coordinate"); await expect(editor).toBeFocused();
  64  |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(last.id);
  65  |   await expect(table.getByText("Editing row retained outside the filter.")).toBeVisible();
  66  |   await expect(editor).toHaveValue("retained invalid coordinate"); await expect(filter).toBeFocused();
  67  |   await expect(rows.locator('[role="row"]')).toHaveCount(2);
  68  |   await filter.fill(""); await expect(editor).toHaveValue("retained invalid coordinate");
  69  |   await table.getByRole("button", { name: "Cancel", exact: true }).click();
  70  |   await expect(cell).toBeFocused(); await expect(cell).toHaveText(String(first.position.x));
  71  |   // A filtered-out remembered cell must leave a real keyboard entry in the view.
  72  |   await filter.fill(last.id);
  73  |   await table.getByRole("rowheader").getByRole("button", { name: last.id, exact: true }).click();
  74  |   await page.keyboard.press("Tab");
  75  |   await expect(page.getByTestId(`table-cell-${last.id}-x`)).toBeFocused();
  76  |   expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
  77  |   await page.screenshot({ path: info.outputPath("b4-virtual-editor-cancelled.png") });
  78  | });
  79  | 
  80  | test("B4 Both with Inspector keeps pointer horizontal scrolling inside the coordinate table", async ({ page, browser }, info) => {
  81  |   await attachBrowserIdentity(browser, info);
  82  |   await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  83  |   await page.getByTestId("view-switch-both").click();
  84  |   if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  85  |   await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  86  |   const table = page.getByTestId("engineering-table"); const grid = table.getByRole("grid", { name: "Node coordinates" });
  87  |   const measure = () => page.evaluate(() => {
  88  |     const state: Record<string, { x: number; y: number; width: number; right: number; clientWidth: number; scrollWidth: number; scrollLeft: number }> = {};
  89  |     for (const [name, selector] of Object.entries({ grid: '.engineering-table [role="grid"]', model: '.model-tree', pane: '.shell-table-pane', canvas: '[data-testid="viewport-canvas"]', filter: '[data-testid="model-tree-filter-input"]', tabs: '.entity-grid-tabs', footer: '.engineering-table-footer', header: '.engineering-table-header', body: '[data-testid="engineering-table-rows"]' })) {
  90  |       const element = document.querySelector(selector)!; const rect = element.getBoundingClientRect();
  91  |       state[name] = { x: rect.x, y: rect.y, width: rect.width, right: rect.right, clientWidth: element.clientWidth, scrollWidth: element.scrollWidth, scrollLeft: element.scrollLeft };
  92  |     }
  93  |     return state;
  94  |   });
  95  |   const before = await measure();
  96  |   await grid.hover({ position: { x: 150, y: 100 } }); await page.mouse.wheel(700, 0);
> 97  |   await expect.poll(async () => (await measure()).grid.scrollLeft).toBeGreaterThan(0);
      |                                                                    ^ Error: expect(received).toBeGreaterThan(expected)
  98  |   const after = await measure();
  99  |   await info.attach("coordinate-scroll-widths", { body: JSON.stringify({ viewport: page.viewportSize(), before, after }, null, 2), contentType: "application/json" });
  100 |   expect(after.grid.scrollWidth).toBeGreaterThan(after.grid.clientWidth);
  101 |   expect(after.model.scrollLeft).toBe(0);
  102 |   for (const name of ["filter", "tabs", "footer", "pane", "canvas"]) {
  103 |     expect(after[name].x).toBe(before[name].x); expect(after[name].width).toBe(before[name].width);
  104 |   }
  105 |   for (const name of ["filter", "tabs", "footer"]) {
  106 |     expect(after[name].x).toBeGreaterThanOrEqual(after.pane.x); expect(after[name].right).toBeLessThanOrEqual(after.pane.right);
  107 |   }
  108 |   expect(after.header.width).toBe(after.body.width);
  109 |   const z = page.getByTestId("table-cell-node:N-100-z");
  110 |   const headerZ = await grid.getByRole("columnheader").last().boundingBox(); const bodyZ = await z.locator("..").boundingBox();
  111 |   expect(bodyZ!.x).toBeCloseTo(headerZ!.x, 1); expect(bodyZ!.width).toBeCloseTo(headerZ!.width, 1);
  112 |   await z.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 Z [m]" }); await editor.fill("4.6");
  113 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(z).toHaveText("0");
  114 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  115 |   await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  116 |   await expect(grid).toBeVisible();
  117 |   await page.screenshot({ path: info.outputPath("b4-inspector-pointer-fit.png") });
  118 | });
  119 | 
```