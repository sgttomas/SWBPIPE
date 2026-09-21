# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 short Grid keeps vertical chrome fixed and retains alternate review drafts
- Location: e2e/b4-table-editing.spec.ts:175:1

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 1
+ Received  + 1

@@ -24,11 +24,11 @@
      "y": 656.203125,
    },
    "header": Object {
      "height": 33,
      "width": 700,
-     "x": 23,
+     "x": 68,
      "y": 369.1875,
    },
    "host": Object {
      "height": 667,
      "width": 348,

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
            - button "Grid" [pressed] [ref=e150]:
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
                    - 'row "node:N-100 node:N-100 Label: Pump nozzle invalid retained node:N-100 Y: 0 m node:N-100 Z: 0 m node:N-100 Provenance: invented_example" [selected] [ref=e195]':
                      - rowheader "node:N-100" [ref=e196]:
                        - button "node:N-100" [ref=e197]
                      - 'gridcell "node:N-100 Label: Pump nozzle" [ref=e198]':
                        - 'button "node:N-100 Label: Pump nozzle" [ref=e199]': Pump nozzle
                      - gridcell "invalid retained" [selected] [ref=e200]:
                        - textbox "node:N-100 X [m]" [active] [ref=e201]: invalid retained
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
                - generic [ref=e265]: Editing X · node:N-100
                - button "Apply" [ref=e266]
                - button "Cancel" [ref=e267]
              - alert [ref=e268]: Enter a finite number in the entered unit.
            - button "Review multiple changes" [ref=e269] [cursor=pointer]
      - separator "Resize table and canvas" [ref=e270]
      - generic [ref=e272]:
        - generic [ref=e273]:
          - group "Viewport controls" [ref=e274]:
            - generic [ref=e275]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e276]:
              - generic "Deformation · unavailable" [ref=e277] [cursor=pointer]
            - group "Viewport display toggles" [ref=e278]:
              - button "Labels" [pressed] [ref=e279]
              - button "Loads" [pressed] [ref=e280]
              - button "Grid" [pressed] [ref=e281]
            - group "Viewport selection tools" [ref=e282]:
              - button "Box Select" [ref=e283]
              - generic [ref=e284]:
                - generic [ref=e285]: Selection filter
                - combobox "Selection filter" [ref=e286]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [ref=e287]
              - button "Isolate" [ref=e288]
              - button "Show All" [disabled] [ref=e289]
              - generic [ref=e290]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e291]
              - button "Fit Visible" [ref=e292]
              - button "Fit Selection" [ref=e293]
            - group "Viewport geometry" [ref=e294]:
              - button "Schematic" [pressed] [ref=e295]
              - button "Actual OD" [ref=e296]
              - button "Measure" [ref=e297]
          - generic "Viewport status" [ref=e298]:
            - 'generic "Selected node: node:N-100" [ref=e299]': "Selected: node:N-100"
            - status "Schematic centerline geometry" [ref=e300]
            - status "View command status" [ref=e301]: No view command dispatched.
        - generic [ref=e302]:
          - generic "Three.js pipe centerline viewport" [ref=e303]
          - generic "Viewport entity selection":
            - button "Select Pump nozzle in viewport" [pressed] [ref=e305] [cursor=pointer]:
              - img [ref=e306]
              - generic [ref=e309]: N-100
            - button "Select Vertical riser in viewport" [ref=e310] [cursor=pointer]:
              - img [ref=e311]
              - generic [ref=e315]: P-110
            - button "Select Invented branch connection marker in viewport" [ref=e316] [cursor=pointer]:
              - img [ref=e317]
              - generic [ref=e320]: C-120
            - button "Select Low point elbow in viewport" [ref=e321] [cursor=pointer]:
              - img [ref=e322]
              - generic [ref=e325]: N-110
            - button "Select Riser elbow in viewport" [ref=e326] [cursor=pointer]:
              - img [ref=e327]
              - generic [ref=e330]: N-120
            - button "Select Rack span in viewport" [ref=e331] [cursor=pointer]:
              - img [ref=e332]
              - generic [ref=e336]: P-120
            - button "Select Tie-in rise in viewport" [ref=e337] [cursor=pointer]:
              - img [ref=e338]
              - generic [ref=e342]: P-130
            - button "Select Invented semi-rigid valve marker in viewport" [ref=e343] [cursor=pointer]:
              - img [ref=e344]
              - generic [ref=e347]: C-130
            - button "Select Rack turn in viewport" [ref=e348] [cursor=pointer]:
              - img [ref=e349]
              - generic [ref=e352]: N-130
            - button "Select Terminal tie-in in viewport" [ref=e353] [cursor=pointer]:
              - img [ref=e354]
              - generic [ref=e357]: N-140
            - button "Select Preview one-way terminal stop in viewport" [ref=e358] [cursor=pointer]:
              - img [ref=e359]
              - generic [ref=e362]: NL-140
            - button "Select Anchor at pump nozzle in viewport" [ref=e363] [cursor=pointer]:
              - img [ref=e364]
              - generic [ref=e367]: S-100
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e368]:
            - button "Front" [ref=e369] [cursor=pointer]
            - button "Top" [ref=e370] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e371] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e372]:
          - generic "Object creation tools" [ref=e373]:
            - button "Node" [ref=e374] [cursor=pointer]:
              - img [ref=e375]
              - text: Node
            - button "Pipe" [ref=e377] [cursor=pointer]:
              - img [ref=e378]
              - text: Pipe
            - button "Support" [ref=e382] [cursor=pointer]:
              - img [ref=e383]
              - text: Support
            - button "Component" [ref=e386] [cursor=pointer]:
              - img [ref=e387]
              - text: Component
            - button "Load" [ref=e390] [cursor=pointer]:
              - img [ref=e391]
              - text: Load
          - generic "Model focus" [ref=e393]: Select
          - group [ref=e394]:
            - generic "Selection & navigation" [ref=e395] [cursor=pointer]
      - generic [ref=e396]:
        - button "Close inspector" [ref=e397]:
          - img [ref=e398]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e401]:
          - 'heading "Pump nozzle — node: node:N-100" [level=2] [ref=e402]':
            - text: Pump nozzle
            - generic [ref=e403]: "— node: node:N-100"
          - tablist "Inspector views" [ref=e404]:
            - tab "Properties" [selected] [ref=e405]
            - tab "Task" [ref=e406]
          - tabpanel [ref=e407]:
            - group [ref=e408]:
              - generic "All properties" [ref=e409] [cursor=pointer]
          - generic [ref=e410]:
            - group [ref=e411]:
              - generic "Sources and units" [ref=e412] [cursor=pointer]
            - group [ref=e413]:
              - generic "New support configuration" [ref=e414] [cursor=pointer]
              - text: ▾
            - group [ref=e415]:
              - generic "New section" [ref=e416] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e417]:
              - generic "New material" [ref=e418] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e419]:
              - generic "New support" [ref=e420] [cursor=pointer]
              - text: ▾
            - group [ref=e421]:
              - generic "New component" [ref=e422] [cursor=pointer]
              - text: ▾ ▾ ▾
            - region "Delete node intent" [ref=e423]:
              - heading "Delete node" [level=3] [ref=e424]
              - button "Queue delete node" [ref=e426] [cursor=pointer]:
                - img [ref=e427]
                - text: Queue delete node
              - article [ref=e430]:
                - generic [ref=e431]:
                  - generic [ref=e432]:
                    - generic [ref=e433]: Operation
                    - strong [ref=e434]: op:delete-node-node:N-100; delete; proposed
                  - generic [ref=e435]:
                    - generic [ref=e436]: Target
                    - strong [ref=e437]: Node; node:N-100
                  - generic [ref=e438]:
                    - generic [ref=e439]: Change
                    - strong [ref=e440]: delete_node; Explicit node deletion; nodes; before=Pump nozzle; x=0; y=0; z=0; after=not_present
                  - generic [ref=e441]:
                    - generic [ref=e442]: Unit basis
                    - strong [ref=e443]: dimensionless; unit=none; explicit user-entered node deletion; endpoint and load reference integrity required
                  - generic [ref=e444]:
                    - generic [ref=e445]: Validation
                    - strong [ref=e446]: not_run; not_run; not_required_dimensionless; not_generated; not_applied
                  - generic [ref=e447]:
                    - generic [ref=e448]: Audit boundary
                    - strong [ref=e449]: Routed through the structured operations only; no direct model mutation; requires your acceptance; does not change the accepted model until applied
                  - generic [ref=e450]:
                    - generic [ref=e451]: Professional boundary
                    - strong [ref=e452]: Requires human review; no compliance claim; no approval claim
                  - generic [ref=e453]:
                    - generic [ref=e454]: Rationale
                    - strong [ref=e455]: explicit user-entered node deletion for project:invented-loop-01; requires reference validation before durable model change.
    - complementary "Agent" [ref=e456]:
      - button "Agent" [disabled] [ref=e458]:
        - img [ref=e459]
        - generic [ref=e462]: Agent
  - generic "Workspace status" [ref=e463]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e464] [cursor=pointer]:
      - img [ref=e465]
      - text: 5 Issues
    - generic "Selection" [ref=e467]: "node: node:N-100"
    - generic "Display units" [ref=e468]: Entered
    - button "About SWBPIPE…" [ref=e469] [cursor=pointer]:
      - img [ref=e470]
```

# Test source

```ts
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
  113 |   const headerZ = await grid.getByRole("button", { name: "Sort Z", exact: true }).locator("..").boundingBox(); const bodyZ = await z.locator("..").boundingBox();
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
  180 |   await rows.hover({ position: { x: 20, y: 20 } }); wheelEvidence.push(await tableWheel(page, 600, info));
  181 |   await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  182 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info));
  183 |   await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  184 |   expect(await rows.evaluate((node) => ({ client: node.clientHeight, scroll: node.scrollHeight, top: node.scrollTop }))).toEqual({ client: 180, scroll: 180, top: 0 });
  185 |   const cell = page.getByTestId("table-cell-node:N-100-x"); await cell.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 X [m]" }); await editor.fill("invalid retained"); await editor.press("Enter");
  186 |   await expect(editor).toHaveAttribute("aria-invalid", "true");
  187 |   expect(await page.getByTestId("engineering-table").locator(".engineering-table-body-slot").evaluate((node) => node.clientHeight)).toBeGreaterThan(0);
> 188 |   const errorState = await gridChromeBounds(page); await rows.hover({ position: { x: 20, y: 20 } }); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(errorState);
      |                                                                                                                                                                                                             ^ Error: expect(received).toEqual(expected) // deep equality
  189 |   const toggle = page.getByTestId("node-grid-review-disclosure"); await toggle.click(); await expect(table).toBeHidden(); await expect(page.getByTestId("retained-direct-draft")).toBeVisible();
  190 |   await expect(toggle).toContainText("Return to node fields"); const bulk = page.getByTestId("review-cell-node:N-100-y"); await bulk.dblclick(); await page.getByTestId("engineering-table-review").getByRole("textbox").fill("0.5"); await page.getByRole("button", { name: "Keep draft", exact: true }).click();
  191 |   const reviewState = await gridChromeBounds(page, true); await page.getByTestId("engineering-table-review-rows").hover({ position: { x: 20, y: 20 } }); wheelEvidence.push(await tableWheel(page, 600, info, false, "engineering-table-review-rows")); await expect.poll(() => gridChromeBounds(page, true)).toEqual(reviewState);
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
  214 |   await rows.hover({ position: { x: 20, y: 20 } }); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  215 |   wheelEvidence.push(await tableWheel(page, 1000000, info, true)); await expect.poll(async () => { const state = await scrollState(); return state.maximum - state.top; }).toBe(0);
  216 |   wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  217 |   wheelEvidence.push(await tableWheel(page, -1000000, info, true));
  218 |   // CDP's large reversal can end a few pixels above the boundary. One further
  219 |   // real wheel input establishes top; the following separate input tests chaining.
  220 |   if ((await scrollState()).top > 0) wheelEvidence.push(await tableWheel(page, -600, info, true));
  221 |   await expect.poll(async () => (await scrollState()).top).toBe(0);
  222 |   wheelEvidence.push(await tableWheel(page, -600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  223 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  224 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(model.nodes.at(-1).id);
  225 |   await expect(rows.locator('[role="row"]')).toHaveCount(1); const filtered = await gridChromeBounds(page); await rows.hover({ position: { x: 20, y: 20 } }); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(filtered);
  226 |   await filter.fill(""); const restored = await gridChromeBounds(page); await rows.hover({ position: { x: 20, y: 20 } }); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(restored);
  227 |   // Swap the mounted virtual body through a small family, changing available
  228 |   // width while it is absent, then verify the new element owns its observation.
  229 |   const review = page.getByTestId("node-grid-review-disclosure"); await review.click();
  230 |   const bulkRows = page.getByTestId("engineering-table-review-rows"); await expect(bulkRows).toBeVisible();
  231 |   await page.getByTestId("entity-grid-type-sections").click(); await expect(bulkRows).toBeHidden();
  232 |   await page.getByTestId("toggle-inspector").click(); await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulkRows).toBeVisible();
  233 |   await expect.poll(() => bulkRows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  234 |   const bulkChrome = await gridChromeBounds(page, true); await bulkRows.hover({ position: { x: 20, y: 20 } }); wheelEvidence.push(await tableWheel(page, 700, info, true, "engineering-table-review-rows"));
  235 |   await expect.poll(() => bulkRows.evaluate((node) => node.scrollTop)).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page, true)).toEqual(bulkChrome);
  236 |   await review.click(); await expect(rows).toBeVisible(); await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  237 |   // A page makes the still-laid-out stage inert. Positive resize observations
  238 |   // must remain current even before interaction is restored.
  239 |   const retainedCell = page.getByTestId(`table-cell-${model.nodes[0].id}-x`); await retainedCell.dblclick();
  240 |   const retainedEditor = page.getByTestId("engineering-table").getByRole("textbox", { name: `${model.nodes[0].id} X [${model.project.units.length}]` });
  241 |   await retainedEditor.fill("retained page draft"); await retainedEditor.press("Enter"); await expect(retainedEditor).toHaveAttribute("aria-invalid", "true");
  242 |   const originalViewport = page.viewportSize()!;
  243 |   const selectionBeforePage = await page.getByTestId("command-selection-readout").textContent();
  244 |   await openWorkspaceSection(page, "libraries");
  245 |   await page.setViewportSize({ width: originalViewport.width, height: originalViewport.height + 120 });
  246 |   await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  247 |   await page.getByTestId("workspace-dock-close").click(); await expect(rows).toBeVisible();
  248 |   await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  249 |   expect(await page.evaluate(() => document.activeElement !== document.body)).toBe(true);
  250 |   expect(await page.getByTestId("command-selection-readout").textContent()).toBe(selectionBeforePage);
  251 |   await expect(retainedEditor).toHaveValue("retained page draft"); await expect(retainedEditor).toHaveAttribute("aria-invalid", "true");
  252 |   await page.setViewportSize(originalViewport); await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  253 |   await expect(retainedEditor).toHaveValue("retained page draft");
  254 |   await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click(); await expect(retainedCell).toHaveText(String(model.nodes[0].position.x));
  255 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  256 |   await info.attach("vertical-wheel-steps", { body: JSON.stringify({ before, filtered, restored, bulkChrome, wheelEvidence }, null, 2), contentType: "application/json" });
  257 |   await page.screenshot({ path: info.outputPath("b4-virtual-fixed-chrome.png") });
  258 | });
  259 | 
  260 | test("B4 text fields apply, recover filtered focus, preserve text Undo and retain review drafts through pointer Queue and Clear", async ({ page, browser }, info) => {
  261 |   await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  262 |   await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  263 |   const table = page.getByTestId("engineering-table"); const label = table.getByTestId("table-cell-node:N-100-label");
  264 |   const original = await label.textContent(); expect(original).toBeTruthy();
  265 |   await label.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 Label", exact: true });
  266 |   await editor.fill(` ${original} `); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  267 |   await label.dblclick(); await editor.fill("   "); await table.getByRole("button", { name: "Apply", exact: true }).click();
  268 |   await expect(editor).toHaveAttribute("aria-invalid", "true"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  269 |   await editor.fill("Cancel me"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(label).toHaveText(original!);
  270 |   await label.dblclick(); await editor.fill("Unique filter label"); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(label).toHaveText("Unique filter label");
  271 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("Unique filter label");
  272 |   const selection = await page.getByTestId("command-selection-readout").textContent();
  273 |   await label.dblclick(); await editor.fill("Renamed outside filter"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  274 |   await expect(label).toHaveCount(0); await expect(table.getByRole("group", { name: "Node fields footer" })).toBeFocused();
  275 |   expect(await page.getByTestId("command-selection-readout").textContent()).toBe(selection);
  276 |   await filter.fill(""); await expect(label).toHaveText("Renamed outside filter");
  277 |   const provenance = table.getByTestId("table-cell-node:N-100-provenance"); const beforeProvenance = await provenance.textContent();
  278 |   await provenance.dblclick(); const text = table.getByRole("textbox", { name: "node:N-100 Provenance", exact: true });
  279 |   await text.fill("TBD"); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(provenance).toHaveText("TBD");
  280 |   await provenance.dblclick(); await text.press("End"); await text.pressSequentially(" transient"); await text.press("Meta+z");
  281 |   await expect(provenance).toHaveCount(0); await expect(text).toHaveValue("TBD");
  282 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(provenance).toHaveText("TBD");
  283 |   await page.getByTestId("workspace-undo").click(); await expect(provenance).toHaveText(beforeProvenance!); await page.getByTestId("workspace-redo").click(); await expect(provenance).toHaveText("TBD");
  284 |   await page.getByTestId("node-grid-review-disclosure").click(); const review = page.getByTestId("engineering-table-review");
  285 |   await review.getByTestId("review-cell-node:N-100-label").focus(); await page.keyboard.press("Q");
  286 |   await review.getByRole("button", { name: "Keep draft", exact: true }).click(); await expect(review.getByRole("status")).toHaveText("Draft retained; model unchanged.");
  287 |   await review.getByTestId("review-cell-node:N-110-label").dblclick(); await review.getByRole("textbox").fill("hidden retained");
  288 |   await page.getByTestId("node-grid-review-disclosure").click(); await expect(label).toHaveText("Renamed outside filter");
```