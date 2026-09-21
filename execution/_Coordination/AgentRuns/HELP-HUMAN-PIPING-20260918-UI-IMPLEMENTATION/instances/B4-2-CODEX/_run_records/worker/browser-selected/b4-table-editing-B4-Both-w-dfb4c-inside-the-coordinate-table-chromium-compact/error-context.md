# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 Both with Inspector keeps pointer horizontal scrolling inside the coordinate table
- Location: e2e/b4-table-editing.spec.ts:80:1

# Error details

```
Test timeout of 120000ms exceeded.
```

```
Error: locator.boundingBox: Test timeout of 120000ms exceeded.
Call log:
  - waiting for getByTestId('engineering-table').getByRole('grid', { name: 'Node fields' }).getByRole('columnheader').filter({ has: getByTestId('engineering-table').getByRole('grid', { name: 'Node fields' }).getByRole('button', { name: 'Sort Z', exact: true }) })

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
              - grid "Node fields" [ref=e178]:
                - row "Node Sort Label Sort X Sort Y Sort Z Sort Provenance" [ref=e179]:
                  - columnheader "Node" [ref=e180]
                  - columnheader "Sort Label" [ref=e181]:
                    - button "Sort Label" [ref=e182]: Label ↕
                  - columnheader "Sort X" [ref=e183]:
                    - button "Sort X" [ref=e184]: X [m] ↕
                  - columnheader "Sort Y" [ref=e185]:
                    - button "Sort Y" [ref=e186]: Y [m] ↕
                  - columnheader "Sort Z" [ref=e187]:
                    - button "Sort Z" [ref=e188]: Z [m] ↕
                  - columnheader "Sort Provenance" [ref=e189]:
                    - button "Sort Provenance" [ref=e190]: Provenance ↕
                - rowgroup [ref=e192]:
                  - generic [ref=e193]:
                    - 'row "node:N-100 node:N-100 Label: Pump nozzle node:N-100 X: 0 m node:N-100 Y: 0 m node:N-100 Z: 0 m node:N-100 Provenance: invented_example" [ref=e195]':
                      - rowheader "node:N-100" [ref=e196]:
                        - button "node:N-100" [ref=e197]
                      - 'gridcell "node:N-100 Label: Pump nozzle" [ref=e198]':
                        - 'button "node:N-100 Label: Pump nozzle" [ref=e199]': Pump nozzle
                      - 'gridcell "node:N-100 X: 0 m" [ref=e200]':
                        - 'button "node:N-100 X: 0 m" [ref=e201]': "0"
                      - 'gridcell "node:N-100 Y: 0 m" [ref=e202]':
                        - 'button "node:N-100 Y: 0 m" [ref=e203]': "0"
                      - 'gridcell "node:N-100 Z: 0 m" [ref=e204]':
                        - 'button "node:N-100 Z: 0 m" [ref=e205]': "0"
                      - 'gridcell "node:N-100 Provenance: invented_example" [ref=e206]':
                        - 'button "node:N-100 Provenance: invented_example" [ref=e207]': invented_example
                    - 'row "node:N-110 node:N-110 Label: Low point elbow node:N-110 X: 3.2 m node:N-110 Y: 0 m node:N-110 Z: 0 m node:N-110 Provenance: invented_example" [ref=e209]':
                      - rowheader "node:N-110" [ref=e210]:
                        - button "node:N-110" [ref=e211]
                      - 'gridcell "node:N-110 Label: Low point elbow" [ref=e212]':
                        - 'button "node:N-110 Label: Low point elbow" [ref=e213]': Low point elbow
                      - 'gridcell "node:N-110 X: 3.2 m" [ref=e214]':
                        - 'button "node:N-110 X: 3.2 m" [ref=e215]': "3.2"
                      - 'gridcell "node:N-110 Y: 0 m" [ref=e216]':
                        - 'button "node:N-110 Y: 0 m" [ref=e217]': "0"
                      - 'gridcell "node:N-110 Z: 0 m" [ref=e218]':
                        - 'button "node:N-110 Z: 0 m" [ref=e219]': "0"
                      - 'gridcell "node:N-110 Provenance: invented_example" [ref=e220]':
                        - 'button "node:N-110 Provenance: invented_example" [ref=e221]': invented_example
                    - 'row "node:N-120 node:N-120 Label: Riser elbow node:N-120 X: 3.2 m node:N-120 Y: 2.4 m node:N-120 Z: 0 m node:N-120 Provenance: invented_example" [ref=e223]':
                      - rowheader "node:N-120" [ref=e224]:
                        - button "node:N-120" [ref=e225]
                      - 'gridcell "node:N-120 Label: Riser elbow" [ref=e226]':
                        - 'button "node:N-120 Label: Riser elbow" [ref=e227]': Riser elbow
                      - 'gridcell "node:N-120 X: 3.2 m" [ref=e228]':
                        - 'button "node:N-120 X: 3.2 m" [ref=e229]': "3.2"
                      - 'gridcell "node:N-120 Y: 2.4 m" [ref=e230]':
                        - 'button "node:N-120 Y: 2.4 m" [ref=e231]': "2.4"
                      - 'gridcell "node:N-120 Z: 0 m" [ref=e232]':
                        - 'button "node:N-120 Z: 0 m" [ref=e233]': "0"
                      - 'gridcell "node:N-120 Provenance: invented_example" [ref=e234]':
                        - 'button "node:N-120 Provenance: invented_example" [ref=e235]': invented_example
                    - 'row "node:N-130 node:N-130 Label: Rack turn node:N-130 X: 7.6 m node:N-130 Y: 2.4 m node:N-130 Z: 0 m node:N-130 Provenance: invented_example" [ref=e237]':
                      - rowheader "node:N-130" [ref=e238]:
                        - button "node:N-130" [ref=e239]
                      - 'gridcell "node:N-130 Label: Rack turn" [ref=e240]':
                        - 'button "node:N-130 Label: Rack turn" [ref=e241]': Rack turn
                      - 'gridcell "node:N-130 X: 7.6 m" [ref=e242]':
                        - 'button "node:N-130 X: 7.6 m" [ref=e243]': "7.6"
                      - 'gridcell "node:N-130 Y: 2.4 m" [ref=e244]':
                        - 'button "node:N-130 Y: 2.4 m" [ref=e245]': "2.4"
                      - 'gridcell "node:N-130 Z: 0 m" [ref=e246]':
                        - 'button "node:N-130 Z: 0 m" [ref=e247]': "0"
                      - 'gridcell "node:N-130 Provenance: invented_example" [ref=e248]':
                        - 'button "node:N-130 Provenance: invented_example" [ref=e249]': invented_example
                    - 'row "node:N-140 node:N-140 Label: Terminal tie-in node:N-140 X: 7.6 m node:N-140 Y: 2.4 m node:N-140 Z: 2.2 m node:N-140 Provenance: invented_example" [ref=e251]':
                      - rowheader "node:N-140" [ref=e252]:
                        - button "node:N-140" [ref=e253]
                      - 'gridcell "node:N-140 Label: Terminal tie-in" [ref=e254]':
                        - 'button "node:N-140 Label: Terminal tie-in" [ref=e255]': Terminal tie-in
                      - 'gridcell "node:N-140 X: 7.6 m" [ref=e256]':
                        - 'button "node:N-140 X: 7.6 m" [ref=e257]': "7.6"
                      - 'gridcell "node:N-140 Y: 2.4 m" [ref=e258]':
                        - 'button "node:N-140 Y: 2.4 m" [ref=e259]': "2.4"
                      - 'gridcell "node:N-140 Z: 2.2 m" [ref=e260]':
                        - 'button "node:N-140 Z: 2.2 m" [ref=e261]': "2.2"
                      - 'gridcell "node:N-140 Provenance: invented_example" [ref=e262]':
                        - 'button "node:N-140 Provenance: invented_example" [ref=e263]': invented_example
              - group "Node fields footer" [ref=e264]:
                - generic [ref=e265]: 5 of 5 rows
            - button "Review multiple changes" [ref=e266] [cursor=pointer]
      - separator "Resize table and canvas" [ref=e267]
      - generic [ref=e269]:
        - generic [ref=e270]:
          - group "Viewport controls" [ref=e271]:
            - generic [ref=e272]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e273]:
              - generic "Deformation · unavailable" [ref=e274] [cursor=pointer]
            - group "Viewport display toggles" [ref=e275]:
              - button "Labels" [pressed] [ref=e276]
              - button "Loads" [pressed] [ref=e277]
              - button "Grid" [pressed] [ref=e278]
            - group "Viewport selection tools" [ref=e279]:
              - button "Box Select" [ref=e280]
              - generic [ref=e281]:
                - generic [ref=e282]: Selection filter
                - combobox "Selection filter" [ref=e283]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e284]
              - button "Isolate" [disabled] [ref=e285]
              - button "Show All" [disabled] [ref=e286]
              - generic [ref=e287]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e288]
              - button "Fit Visible" [ref=e289]
              - button "Fit Selection" [disabled] [ref=e290]
            - group "Viewport geometry" [ref=e291]:
              - button "Schematic" [pressed] [ref=e292]
              - button "Actual OD" [ref=e293]
              - button "Measure" [ref=e294]
          - generic "Viewport status" [ref=e295]:
            - 'generic "Selected project: project:invented-loop-01" [ref=e296]': "Selected: project:invented-loop-01"
            - status "Schematic centerline geometry" [ref=e297]
            - status "View command status" [ref=e298]: No view command dispatched.
        - generic [ref=e299]:
          - generic "Three.js pipe centerline viewport" [ref=e300]
          - generic "Viewport entity selection":
            - button "Select Vertical riser in viewport" [ref=e302] [cursor=pointer]:
              - img [ref=e303]
              - generic [ref=e307]: P-110
            - button "Select Invented branch connection marker in viewport" [ref=e308] [cursor=pointer]:
              - img [ref=e309]
              - generic [ref=e312]: C-120
            - button "Select Low point elbow in viewport" [ref=e313] [cursor=pointer]:
              - img [ref=e314]
              - generic [ref=e317]: N-110
            - button "Select Riser elbow in viewport" [ref=e318] [cursor=pointer]:
              - img [ref=e319]
              - generic [ref=e322]: N-120
            - button "Select Rack span in viewport" [ref=e323] [cursor=pointer]:
              - img [ref=e324]
              - generic [ref=e328]: P-120
            - button "Select Tie-in rise in viewport" [ref=e329] [cursor=pointer]:
              - img [ref=e330]
              - generic [ref=e334]: P-130
            - button "Select Invented semi-rigid valve marker in viewport" [ref=e335] [cursor=pointer]:
              - img [ref=e336]
              - generic [ref=e339]: C-130
            - button "Select Pump nozzle in viewport" [ref=e340] [cursor=pointer]:
              - img [ref=e341]
              - generic [ref=e344]: N-100
            - button "Select Rack turn in viewport" [ref=e345] [cursor=pointer]:
              - img [ref=e346]
              - generic [ref=e349]: N-130
            - button "Select Terminal tie-in in viewport" [ref=e350] [cursor=pointer]:
              - img [ref=e351]
              - generic [ref=e354]: N-140
            - button "Select Preview one-way terminal stop in viewport" [ref=e355] [cursor=pointer]:
              - img [ref=e356]
              - generic [ref=e359]: NL-140
            - button "Select Anchor at pump nozzle in viewport" [ref=e360] [cursor=pointer]:
              - img [ref=e361]
              - generic [ref=e364]: S-100
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e365]:
            - button "Front" [ref=e366] [cursor=pointer]
            - button "Top" [ref=e367] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e368] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e369]:
          - generic "Object creation tools" [ref=e370]:
            - button "Node" [ref=e371] [cursor=pointer]:
              - img [ref=e372]
              - text: Node
            - button "Pipe" [ref=e374] [cursor=pointer]:
              - img [ref=e375]
              - text: Pipe
            - button "Support" [ref=e379] [cursor=pointer]:
              - img [ref=e380]
              - text: Support
            - button "Component" [ref=e383] [cursor=pointer]:
              - img [ref=e384]
              - text: Component
            - button "Load" [ref=e387] [cursor=pointer]:
              - img [ref=e388]
              - text: Load
          - generic "Model focus" [ref=e390]: Select
          - group [ref=e391]:
            - generic "Selection & navigation" [ref=e392] [cursor=pointer]
      - generic [ref=e393]:
        - button "Close inspector" [ref=e394]:
          - img [ref=e395]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e398]:
          - 'heading "Invented Utility Loop Preview — project: project:invented-loop-01" [level=2] [ref=e399]':
            - text: Invented Utility Loop Preview
            - generic [ref=e400]: "— project: project:invented-loop-01"
          - tablist "Inspector views" [ref=e401]:
            - tab "Properties" [selected] [ref=e402]
            - tab "Task" [ref=e403]
          - tabpanel [ref=e404]:
            - group [ref=e405]:
              - generic "All properties" [ref=e406] [cursor=pointer]
          - generic [ref=e407]:
            - group [ref=e408]:
              - generic "Sources and units" [ref=e409] [cursor=pointer]
            - group [ref=e410]:
              - generic "New support configuration" [ref=e411] [cursor=pointer]
              - text: ▾
            - group [ref=e412]:
              - generic "New section" [ref=e413] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e414]:
              - generic "New material" [ref=e415] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e416]:
              - generic "New support" [ref=e417] [cursor=pointer]
              - text: ▾
            - group [ref=e418]:
              - generic "New component" [ref=e419] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e420]:
      - button "Agent" [disabled] [ref=e422]:
        - img [ref=e423]
        - generic [ref=e426]: Agent
  - generic "Workspace status" [ref=e427]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e428] [cursor=pointer]:
      - img [ref=e429]
      - text: 5 Issues
    - generic "Selection" [ref=e431]: "project: project:invented-loop-01"
    - generic "Display units" [ref=e432]: Entered
    - button "About SWBPIPE…" [ref=e433] [cursor=pointer]:
      - img [ref=e434]
```

# Test source

```ts
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
  75  |   await expect(page.getByTestId(`table-cell-${last.id}-label`)).toBeFocused();
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
  86  |   const table = page.getByTestId("engineering-table"); const grid = table.getByRole("grid", { name: "Node fields" });
  87  |   const measure = () => page.evaluate(() => {
  88  |     const state: Record<string, { x: number; y: number; width: number; height: number; right: number; clientWidth: number; scrollWidth: number; scrollLeft: number }> = {};
  89  |     for (const [name, selector] of Object.entries({ grid: '.engineering-table [role="grid"]', model: '.model-tree', pane: '.shell-table-pane', canvas: '[data-testid="viewport-canvas"]', filter: '[data-testid="model-tree-filter-input"]', tabs: '.entity-grid-tabs', footer: '.engineering-table-footer', header: '.engineering-table-header', body: '[data-testid="engineering-table-rows"]' })) {
  90  |       const element = document.querySelector(selector)!; const rect = element.getBoundingClientRect();
  91  |       state[name] = { x: rect.x, y: rect.y, width: rect.width, height: rect.height, right: rect.right, clientWidth: element.clientWidth, scrollWidth: element.scrollWidth, scrollLeft: element.scrollLeft };
  92  |     }
  93  |     return state;
  94  |   });
  95  |   const before = await measure();
  96  |   await grid.hover({ position: { x: 150, y: 100 } }); await page.mouse.wheel(700, 0);
  97  |   await expect.poll(async () => (await measure()).grid.scrollLeft).toBeGreaterThan(0);
  98  |   const after = await measure();
  99  |   await info.attach("coordinate-scroll-widths", { body: JSON.stringify({ viewport: page.viewportSize(), before, after }, null, 2), contentType: "application/json" });
  100 |   expect(after.grid.scrollWidth).toBeGreaterThan(after.grid.clientWidth);
  101 |   expect(after.model.scrollLeft).toBe(0);
  102 |   for (const name of ["filter", "tabs", "footer", "pane", "canvas"]) {
  103 |     expect(after[name].x).toBe(before[name].x); expect(after[name].width).toBe(before[name].width);
  104 |   }
  105 |   for (const name of ["pane", "canvas"]) {
  106 |     expect(after[name].y).toBe(before[name].y); expect(after[name].height).toBe(before[name].height);
  107 |   }
  108 |   for (const name of ["filter", "tabs", "footer"]) {
  109 |     expect(after[name].x).toBeGreaterThanOrEqual(after.pane.x); expect(after[name].right).toBeLessThanOrEqual(after.pane.right);
  110 |   }
  111 |   expect(after.header.width).toBe(after.body.width);
  112 |   const z = page.getByTestId("table-cell-node:N-100-z");
> 113 |   const headerZ = await grid.getByRole("columnheader").filter({ has: grid.getByRole("button", { name: "Sort Z", exact: true }) }).boundingBox(); const bodyZ = await z.locator("..").boundingBox();
      |                                                                                                                                   ^ Error: locator.boundingBox: Test timeout of 120000ms exceeded.
  114 |   expect(bodyZ!.x).toBeCloseTo(headerZ!.x, 1); expect(bodyZ!.width).toBeCloseTo(headerZ!.width, 1);
  115 |   await z.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 Z [m]" }); await editor.fill("4.6");
  116 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(z).toHaveText("0");
  117 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  118 |   await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  119 |   await expect(grid).toBeVisible();
  120 |   await page.screenshot({ path: info.outputPath("b4-inspector-pointer-fit.png") });
  121 | });
  122 | 
  123 | async function gridChromeBounds(page: import("@playwright/test").Page, review = false) {
  124 |   return page.evaluate((review) => {
  125 |     const bounds: Record<string, { x: number; y: number; width: number; height: number }> = {};
  126 |     const selectors = { pane: ".shell-table-pane", host: ".shell-tree-host", title: ".model-tree > .panel-title", mode: ".layout-mode-toggle", filter: ".model-tree-controls", families: ".entity-grid-tabs", header: review ? ".entity-grid-summary" : ".engineering-table-header", footer: review ? ".entity-grid-actions" : ".engineering-table-footer", viewportHost: '[data-testid="viewport-canvas"]', drawnCanvas: '[data-testid="viewport-canvas"] canvas' };
  127 |     for (const [key, selector] of Object.entries(selectors)) {
  128 |       const rect = document.querySelector(selector)!.getBoundingClientRect(); bounds[key] = { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
  129 |     }
  130 |     return bounds;
  131 |   }, review);
  132 | }
  133 | 
  134 | async function openBoundedGrid(page: import("@playwright/test").Page) {
  135 |   await page.getByTestId("view-switch-both").click();
  136 |   if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  137 |   await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  138 | }
  139 | 
  140 | // Wheel input returns before browser delivery. Observe that local event and two
  141 | // paints so an unchanged-boundary assertion cannot pass before the input occurs.
  142 | async function tableWheel(page: import("@playwright/test").Page, deltaY: number, info: import("@playwright/test").TestInfo, moves = false, scroller = "engineering-table-rows") {
  143 |   const observed = await page.locator(".model-tree").evaluateHandle((root, { moves, scroller }) => {
  144 |     const rows = root.querySelector<HTMLElement>(`[data-testid="${scroller}"]`)!;
  145 |     const state: { complete: boolean; ended: boolean; supported: boolean; result?: unknown; cleanup?: () => void } = { complete: false, ended: false, supported: "onscrollend" in rows };
  146 |     const sample = () => {
  147 |       const slot = rows.parentElement!;
  148 |       return { top: rows.scrollTop, clientHeight: rows.clientHeight, scrollHeight: rows.scrollHeight, slotHeight: slot.clientHeight,
  149 |         filterY: root.querySelector(".model-tree-controls")!.getBoundingClientRect().y,
  150 |         familyY: root.querySelector(".entity-grid-tabs")!.getBoundingClientRect().y,
  151 |         footerY: root.querySelector(scroller === "engineering-table-rows" ? ".direct-coordinate-workarea .engineering-table-footer" : ".entity-grid-actions")!.getBoundingClientRect().y };
  152 |     };
  153 |     const ended = () => { state.ended = true; };
  154 |     if (moves) rows.addEventListener("scrollend", ended, { once: true });
  155 |     state.cleanup = () => rows.removeEventListener("scrollend", ended);
  156 |     root.addEventListener("wheel", (event) => {
  157 |       const target = (event.target as Element).outerHTML.slice(0, 400);
  158 |       requestAnimationFrame(() => { const first = sample(); requestAnimationFrame(() => {
  159 |         state.result = { deltaX: (event as WheelEvent).deltaX, deltaY: (event as WheelEvent).deltaY, target, frames: [first, sample()] }; state.complete = true;
  160 |       }); });
  161 |     }, { once: true, passive: true, capture: true });
  162 |     return state;
  163 |   }, { moves, scroller });
  164 |   try {
  165 |     if (moves) expect(await observed.evaluate((state) => state.supported)).toBe(true);
  166 |     await page.mouse.wheel(0, deltaY);
  167 |     await expect.poll(() => observed.evaluate((state) => state.complete)).toBe(true);
  168 |     if (moves) await expect.poll(() => observed.evaluate((state) => state.ended), { message: "scrolling gesture completed" }).toBe(true);
  169 |     const result = await observed.evaluate((state) => ({ receipt: state.result, scrollEnded: state.ended }));
  170 |     await info.attach(`wheel-${deltaY}-${Date.now()}`, { body: JSON.stringify(result), contentType: "application/json" });
  171 |     return result;
  172 |   } finally { await observed.evaluate((state) => state.cleanup?.()); await observed.dispose(); }
  173 | }
  174 | 
  175 | test("B4 short Grid keeps vertical chrome fixed and retains alternate review drafts", async ({ page, browser }, info) => {
  176 |   await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible(); await openBoundedGrid(page);
  177 |   const table = page.getByTestId("engineering-table"); const rows = page.getByTestId("engineering-table-rows");
  178 |   const wheelEvidence: unknown[] = [];
  179 |   const before = await gridChromeBounds(page);
  180 |   await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info));
  181 |   await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  182 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info));
  183 |   await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  184 |   expect(await rows.evaluate((node) => ({ client: node.clientHeight, scroll: node.scrollHeight, top: node.scrollTop }))).toEqual({ client: 180, scroll: 180, top: 0 });
  185 |   const cell = page.getByTestId("table-cell-node:N-100-x"); await cell.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 X [m]" }); await editor.fill("invalid retained"); await editor.press("Enter");
  186 |   await expect(editor).toHaveAttribute("aria-invalid", "true");
  187 |   expect(await page.getByTestId("engineering-table").locator(".engineering-table-body-slot").evaluate((node) => node.clientHeight)).toBeGreaterThan(0);
  188 |   const errorState = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(errorState);
  189 |   const toggle = page.getByTestId("node-grid-review-disclosure"); await toggle.click(); await expect(table).toBeHidden(); await expect(page.getByTestId("retained-direct-draft")).toBeVisible();
  190 |   await expect(toggle).toContainText("Return to node fields"); const bulk = page.getByTestId("review-cell-node:N-100-y"); await bulk.dblclick(); await page.getByTestId("engineering-table-review").getByRole("textbox").fill("0.5"); await page.getByRole("button", { name: "Keep draft", exact: true }).click();
  191 |   const reviewState = await gridChromeBounds(page, true); await page.getByTestId("engineering-table-review-rows").hover(); wheelEvidence.push(await tableWheel(page, 600, info, false, "engineering-table-review-rows")); await expect.poll(() => gridChromeBounds(page, true)).toEqual(reviewState);
  192 |   await page.getByTestId("entity-grid-type-pipes").click(); await expect(page.getByTestId("entity-grid-table-pipes")).toBeVisible();
  193 |   await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulk).toHaveText("0.5"); await toggle.click(); await expect(table).toBeVisible(); await expect(editor).toHaveValue("invalid retained");
  194 |   await expect(toggle).toContainText("1 retained draft"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText("0");
  195 |   await toggle.click(); await page.getByTestId("clear-entity-grid-drafts").click(); await toggle.click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  196 |   const controlledId = await toggle.getAttribute("aria-controls"); expect(controlledId).toBeTruthy();
  197 |   const controlled = page.locator(`[id="${controlledId}"]`); await expect(controlled).toBeHidden();
  198 |   await page.getByTestId("table-cell-node:N-140-provenance").dblclick(); await page.keyboard.press("Tab");
  199 |   await expect(table.getByRole("group", { name: "Node fields footer" })).toBeFocused();
  200 |   await page.keyboard.press("Tab"); await expect(toggle).toBeFocused();
  201 |   await page.keyboard.press("Enter"); await expect(toggle).toHaveAttribute("aria-expanded", "true"); await expect(controlled).toBeVisible(); await expect(table).toBeHidden();
  202 |   await page.keyboard.press("Space"); await expect(toggle).toHaveAttribute("aria-expanded", "false"); await expect(controlled).toBeHidden(); await expect(table).toBeVisible();
  203 |   await expect(toggle).toHaveAttribute("aria-controls", controlledId!); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  204 |   await info.attach("vertical-fixed-rectangles", { body: JSON.stringify({ before, errorState, reviewState, wheelEvidence }, null, 2), contentType: "application/json" });
  205 |   await page.screenshot({ path: info.outputPath("b4-short-fixed-chrome.png") });
  206 | });
  207 | 
  208 | test("B4 virtual Grid confines body scrolling and boundary wheel without moving chrome", async ({ page, browser }, info) => {
  209 |   await attachBrowserIdentity(browser, info); const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json"); await openBoundedGrid(page);
  210 |   const wheelEvidence: unknown[] = [];
  211 |   const before = await gridChromeBounds(page); const rows = page.getByTestId("engineering-table-rows");
  212 |   const scrollState = () => rows.evaluate((node) => ({ top: node.scrollTop, maximum: node.scrollHeight - node.clientHeight, height: node.clientHeight }));
  213 |   expect((await scrollState()).height).toBeGreaterThan(0);
```