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
      - button "Select (⎋)" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
    - group "View" [ref=e31]:
      - button "Table" [ref=e33] [cursor=pointer]:
        - img [ref=e34]
        - generic [ref=e36]: Table
      - button "Model" [ref=e38] [cursor=pointer]:
        - img [ref=e39]
        - generic [ref=e42]: Model
      - button "Both" [pressed] [ref=e44] [cursor=pointer]:
        - img [ref=e45]
        - generic [ref=e47]: Both
    - generic [ref=e48]:
      - button "Run" [ref=e49] [cursor=pointer]:
        - img [ref=e50]
        - generic [ref=e52]: Run
      - button "Issues, 5" [ref=e53] [cursor=pointer]:
        - img [ref=e54]
        - generic [ref=e56]: Issues
        - generic [ref=e57]: "5"
    - group "Panels" [ref=e58]:
      - button "Inspector" [expanded] [ref=e60] [cursor=pointer]:
        - img [ref=e61]
      - button "Agent" [disabled] [ref=e64]:
        - img [ref=e65]
    - generic "Display units" [ref=e68]:
      - combobox "Display units" [ref=e69]:
        - option "Entered" [selected]
        - option "SI"
        - option "US"
    - group [ref=e70]:
      - generic "Appearance" [ref=e71] [cursor=pointer]:
        - img [ref=e72]
      - option "System" [selected]
      - option "Light"
      - option "Dark"
      - option "Comfortable" [selected]
      - option "Compact"
    - region "Human toolkit" [ref=e74]:
      - button "Find modeling commands" [ref=e75] [cursor=pointer]:
        - img [ref=e76]
        - generic [ref=e79]: Search or command…
        - generic "Command K" [ref=e80]: ⌘K
  - generic [ref=e81]:
    - navigation "Stages" [ref=e82]:
      - list [ref=e83]:
        - listitem [ref=e84]:
          - button "Model" [pressed] [ref=e85] [cursor=pointer]:
            - img [ref=e86]
            - generic [ref=e89]: Model
        - listitem [ref=e90]:
          - button "Loads" [ref=e91] [cursor=pointer]:
            - img [ref=e92]
            - generic [ref=e96]: Loads
        - listitem [ref=e97]:
          - button "Results" [disabled] [ref=e98]:
            - img [ref=e99]
            - generic [ref=e102]: Results
        - listitem [ref=e103]:
          - button "Review" [disabled] [ref=e104]:
            - img [ref=e105]
            - generic [ref=e109]: Review
      - separator [ref=e110]
      - list [ref=e111]:
        - listitem [ref=e112]:
          - button "Libraries" [ref=e113] [cursor=pointer]:
            - img [ref=e114]
            - generic [ref=e116]: Libraries
        - listitem [ref=e117]:
          - button "Rules" [ref=e118] [cursor=pointer]:
            - img [ref=e119]
            - generic [ref=e123]: Rules
        - listitem [ref=e124]:
          - button "Issues, 5" [ref=e125] [cursor=pointer]:
            - img [ref=e126]
            - generic [ref=e128]: Issues
            - generic [ref=e129]: "5"
    - region "Modeling workspace" [ref=e131]:
      - generic [ref=e132]:
        - group "Tables" [ref=e133]:
          - button "Model" [pressed] [ref=e134] [cursor=pointer]
          - button "Review changes" [ref=e135] [cursor=pointer]
          - button "Collapse table drawer" [disabled] [expanded] [ref=e138]:
            - img [ref=e139]
        - generic "Model tree" [ref=e143]:
          - generic [ref=e144]: Model
          - region "Layout grid mode" [ref=e145]:
            - button "Tree" [ref=e146]:
              - img [ref=e147]
              - text: Tree
            - button "Grid" [active] [pressed] [ref=e150]:
              - img [ref=e151]
              - text: Grid
          - region "Model tree filtering" [ref=e153]:
            - generic [ref=e154]:
              - img [ref=e155]
              - generic [ref=e158]: Filter model
              - searchbox "Filter model tree" [ref=e159]
            - generic [ref=e160]: 27 of 27 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e161]:
              - img [ref=e162]
          - region "Bulk entity grid" [ref=e166]:
            - generic "Grid entity type" [ref=e167]:
              - button "Nodes" [pressed] [ref=e168]
              - button "Pipes" [ref=e169]
              - button "Supports" [ref=e170]
              - button "Materials" [ref=e171]
              - button "Sections" [ref=e172]
              - button "Components" [ref=e173]
              - button "Load Cases" [ref=e174]
              - button "Combinations" [ref=e175]
            - generic [ref=e177]:
              - grid "Node coordinates" [ref=e178]:
                - row "Node Sort X Sort Y Sort Z" [ref=e179]:
                  - columnheader "Node" [ref=e180]
                  - columnheader "Sort X" [ref=e181]:
                    - button "Sort X" [ref=e182]: X [m] ↕
                  - columnheader "Sort Y" [ref=e183]:
                    - button "Sort Y" [ref=e184]: Y [m] ↕
                  - columnheader "Sort Z" [ref=e185]:
                    - button "Sort Z" [ref=e186]: Z [m] ↕
                - rowgroup [ref=e187]:
                  - generic [ref=e188]:
                    - 'row "node:N-100 node:N-100 X: 0 m node:N-100 Y: 0 m node:N-100 Z: 0 m" [ref=e190]':
                      - rowheader "node:N-100" [ref=e191]:
                        - button "node:N-100" [ref=e192]
                      - 'gridcell "node:N-100 X: 0 m" [ref=e193]':
                        - 'button "node:N-100 X: 0 m" [ref=e194]': "0"
                      - 'gridcell "node:N-100 Y: 0 m" [ref=e195]':
                        - 'button "node:N-100 Y: 0 m" [ref=e196]': "0"
                      - 'gridcell "node:N-100 Z: 0 m" [ref=e197]':
                        - 'button "node:N-100 Z: 0 m" [ref=e198]': "0"
                    - 'row "node:N-110 node:N-110 X: 3.2 m node:N-110 Y: 0 m node:N-110 Z: 0 m" [ref=e200]':
                      - rowheader "node:N-110" [ref=e201]:
                        - button "node:N-110" [ref=e202]
                      - 'gridcell "node:N-110 X: 3.2 m" [ref=e203]':
                        - 'button "node:N-110 X: 3.2 m" [ref=e204]': "3.2"
                      - 'gridcell "node:N-110 Y: 0 m" [ref=e205]':
                        - 'button "node:N-110 Y: 0 m" [ref=e206]': "0"
                      - 'gridcell "node:N-110 Z: 0 m" [ref=e207]':
                        - 'button "node:N-110 Z: 0 m" [ref=e208]': "0"
                    - 'row "node:N-120 node:N-120 X: 3.2 m node:N-120 Y: 2.4 m node:N-120 Z: 0 m" [ref=e210]':
                      - rowheader "node:N-120" [ref=e211]:
                        - button "node:N-120" [ref=e212]
                      - 'gridcell "node:N-120 X: 3.2 m" [ref=e213]':
                        - 'button "node:N-120 X: 3.2 m" [ref=e214]': "3.2"
                      - 'gridcell "node:N-120 Y: 2.4 m" [ref=e215]':
                        - 'button "node:N-120 Y: 2.4 m" [ref=e216]': "2.4"
                      - 'gridcell "node:N-120 Z: 0 m" [ref=e217]':
                        - 'button "node:N-120 Z: 0 m" [ref=e218]': "0"
                    - 'row "node:N-130 node:N-130 X: 7.6 m node:N-130 Y: 2.4 m node:N-130 Z: 0 m" [ref=e220]':
                      - rowheader "node:N-130" [ref=e221]:
                        - button "node:N-130" [ref=e222]
                      - 'gridcell "node:N-130 X: 7.6 m" [ref=e223]':
                        - 'button "node:N-130 X: 7.6 m" [ref=e224]': "7.6"
                      - 'gridcell "node:N-130 Y: 2.4 m" [ref=e225]':
                        - 'button "node:N-130 Y: 2.4 m" [ref=e226]': "2.4"
                      - 'gridcell "node:N-130 Z: 0 m" [ref=e227]':
                        - 'button "node:N-130 Z: 0 m" [ref=e228]': "0"
                    - 'row "node:N-140 node:N-140 X: 7.6 m node:N-140 Y: 2.4 m node:N-140 Z: 2.2 m" [ref=e230]':
                      - rowheader "node:N-140" [ref=e231]:
                        - button "node:N-140" [ref=e232]
                      - 'gridcell "node:N-140 X: 7.6 m" [ref=e233]':
                        - 'button "node:N-140 X: 7.6 m" [ref=e234]': "7.6"
                      - 'gridcell "node:N-140 Y: 2.4 m" [ref=e235]':
                        - 'button "node:N-140 Y: 2.4 m" [ref=e236]': "2.4"
                      - 'gridcell "node:N-140 Z: 2.2 m" [ref=e237]':
                        - 'button "node:N-140 Z: 2.2 m" [ref=e238]': "2.2"
              - group "Node coordinates footer" [ref=e239]:
                - generic [ref=e240]: 5 of 5 rows
            - group [ref=e241]:
              - generic "Review multiple changes" [ref=e242] [cursor=pointer]
      - separator "Resize table and canvas" [ref=e243]
      - generic [ref=e245]:
        - generic [ref=e246]:
          - group "Viewport controls" [ref=e247]:
            - generic [ref=e248]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e249]:
              - generic "Deformation · unavailable" [ref=e250] [cursor=pointer]
            - group "Viewport display toggles" [ref=e251]:
              - button "Labels" [pressed] [ref=e252]
              - button "Loads" [pressed] [ref=e253]
              - button "Grid" [pressed] [ref=e254]
            - group "Viewport selection tools" [ref=e255]:
              - button "Box Select" [ref=e256]
              - generic [ref=e257]:
                - generic [ref=e258]: Selection filter
                - combobox "Selection filter" [ref=e259]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e260]
              - button "Isolate" [disabled] [ref=e261]
              - button "Show All" [disabled] [ref=e262]
              - generic [ref=e263]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e264]
              - button "Fit Visible" [ref=e265]
              - button "Fit Selection" [disabled] [ref=e266]
            - group "Viewport geometry" [ref=e267]:
              - button "Schematic" [pressed] [ref=e268]
              - button "Actual OD" [ref=e269]
              - button "Measure" [ref=e270]
          - generic "Viewport status" [ref=e271]:
            - 'generic "Selected project: project:invented-loop-01" [ref=e272]': "Selected: project:invented-loop-01"
            - status "Schematic centerline geometry" [ref=e273]
            - status "View command status" [ref=e274]: No view command dispatched.
        - generic [ref=e275]:
          - generic "Three.js pipe centerline viewport" [ref=e276]
          - generic "Viewport entity selection":
            - button "Select Vertical riser in viewport" [ref=e278] [cursor=pointer]:
              - img [ref=e279]
              - generic [ref=e283]: P-110
            - button "Select Invented branch connection marker in viewport" [ref=e284] [cursor=pointer]:
              - img [ref=e285]
              - generic [ref=e288]: C-120
            - button "Select Low point elbow in viewport" [ref=e289] [cursor=pointer]:
              - img [ref=e290]
              - generic [ref=e293]: N-110
            - button "Select Riser elbow in viewport" [ref=e294] [cursor=pointer]:
              - img [ref=e295]
              - generic [ref=e298]: N-120
            - button "Select Rack span in viewport" [ref=e299] [cursor=pointer]:
              - img [ref=e300]
              - generic [ref=e304]: P-120
            - button "Select Tie-in rise in viewport" [ref=e305] [cursor=pointer]:
              - img [ref=e306]
              - generic [ref=e310]: P-130
            - button "Select Invented semi-rigid valve marker in viewport" [ref=e311] [cursor=pointer]:
              - img [ref=e312]
              - generic [ref=e315]: C-130
            - button "Select Pump nozzle in viewport" [ref=e316] [cursor=pointer]:
              - img [ref=e317]
              - generic [ref=e320]: N-100
            - button "Select Rack turn in viewport" [ref=e321] [cursor=pointer]:
              - img [ref=e322]
              - generic [ref=e325]: N-130
            - button "Select Terminal tie-in in viewport" [ref=e326] [cursor=pointer]:
              - img [ref=e327]
              - generic [ref=e330]: N-140
            - button "Select Preview one-way terminal stop in viewport" [ref=e331] [cursor=pointer]:
              - img [ref=e332]
              - generic [ref=e335]: NL-140
            - button "Select Anchor at pump nozzle in viewport" [ref=e336] [cursor=pointer]:
              - img [ref=e337]
              - generic [ref=e340]: S-100
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e341]:
            - button "Front" [ref=e342] [cursor=pointer]
            - button "Top" [ref=e343] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e344] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e345]:
          - generic "Object creation tools" [ref=e346]:
            - button "Node" [ref=e347] [cursor=pointer]:
              - img [ref=e348]
              - text: Node
            - button "Pipe" [ref=e350] [cursor=pointer]:
              - img [ref=e351]
              - text: Pipe
            - button "Support" [ref=e355] [cursor=pointer]:
              - img [ref=e356]
              - text: Support
            - button "Component" [ref=e359] [cursor=pointer]:
              - img [ref=e360]
              - text: Component
            - button "Load" [ref=e363] [cursor=pointer]:
              - img [ref=e364]
              - text: Load
          - generic "Model focus" [ref=e366]: Select
          - group [ref=e367]:
            - generic "Selection & navigation" [ref=e368] [cursor=pointer]
      - generic [ref=e369]:
        - button "Close inspector" [ref=e370]:
          - img [ref=e371]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e374]:
          - 'heading "Invented Utility Loop Preview — project: project:invented-loop-01" [level=2] [ref=e375]':
            - text: Invented Utility Loop Preview
            - generic [ref=e376]: "— project: project:invented-loop-01"
          - tablist "Inspector views" [ref=e377]:
            - tab "Properties" [selected] [ref=e378]
            - tab "Task" [ref=e379]
          - tabpanel [ref=e380]:
            - group [ref=e381]:
              - generic "All properties" [ref=e382] [cursor=pointer]
          - generic [ref=e383]:
            - group [ref=e384]:
              - generic "Sources and units" [ref=e385] [cursor=pointer]
            - group [ref=e386]:
              - generic "New support configuration" [ref=e387] [cursor=pointer]
              - text: ▾
            - group [ref=e388]:
              - generic "New section" [ref=e389] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e390]:
              - generic "New material" [ref=e391] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e392]:
              - generic "New support" [ref=e393] [cursor=pointer]
              - text: ▾
            - group [ref=e394]:
              - generic "New component" [ref=e395] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e396]:
      - button "Agent" [disabled] [ref=e398]:
        - img [ref=e399]
        - generic [ref=e402]: Agent
  - generic "Workspace status" [ref=e403]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e404] [cursor=pointer]:
      - img [ref=e405]
      - text: 5 Issues
    - generic "Selection" [ref=e407]: "project: project:invented-loop-01"
    - generic "Display units" [ref=e408]: Entered
    - button "About SWBPIPE…" [ref=e409] [cursor=pointer]:
      - img [ref=e410]
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