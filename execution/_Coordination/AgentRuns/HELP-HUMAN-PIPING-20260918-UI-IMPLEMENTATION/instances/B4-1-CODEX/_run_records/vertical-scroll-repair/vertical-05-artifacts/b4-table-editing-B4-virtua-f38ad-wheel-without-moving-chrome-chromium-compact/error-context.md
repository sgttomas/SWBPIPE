# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 virtual Grid confines body scrolling and boundary wheel without moving chrome
- Location: e2e/b4-table-editing.spec.ts:200:1

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator:  getByTestId('entity-grid-virtual-rows')
Expected: visible
Received: hidden
Timeout:  10000ms

Call log:
  - Expect "toBeVisible" with timeout 10000ms
  - waiting for getByTestId('entity-grid-virtual-rows')
    23 × locator resolved to <div role="rowgroup" data-testid="entity-grid-virtual-rows">…</div>
       - unexpected value "hidden"

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
  - paragraph: Generated UI foundation 1000-pipe model
  - group "Editing tools":
    - button "Undo model edit" [disabled]
    - button "Redo model edit" [disabled]
    - button "Select (⎋)" [pressed]
  - group "View":
    - button "Table"
    - button "Model"
    - button "Both" [pressed]
  - button "Run"
  - button "Issues, 3": Issues 3
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
        - button "Issues, 3": Issues
  - region "Modeling workspace":
    - group "Tables":
      - button "Model" [pressed]
      - button "Review changes"
      - button "Collapse table drawer" [disabled] [expanded]
    - text: Model
    - region "Layout grid mode":
      - button "Tree"
      - button "Grid" [pressed]
    - region "Model tree filtering":
      - text: Filter model
      - searchbox "Filter model tree"
      - text: 2069 of 2069 model entities visible
      - button "Clear model tree filter" [disabled]
    - region "Bulk entity grid":
      - button "Nodes" [pressed]
      - button "Pipes"
      - button "Supports"
      - button "Materials"
      - button "Sections"
      - button "Components"
      - button "Load Cases"
      - button "Combinations"
      - group:
        - text: Return to node coordinates 1001 of 1001 Nodes 0 changed cells
        - region "Editable model entity table":
          - table "nodes editable grid":
            - row "ID Label X Y Z Provenance":
              - columnheader "ID"
              - columnheader "Label"
              - columnheader "X"
              - columnheader "Y"
              - columnheader "Z"
              - columnheader "Provenance"
            - rowgroup:
              - row "node:UIF-00000 UI benchmark node 00000 0 0 0 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00000":
                  - button "node:UIF-00000"
                - cell "UI benchmark node 00000":
                  - textbox "node:UIF-00000 Label": UI benchmark node 00000
                - cell "0":
                  - textbox "node:UIF-00000 X": "0"
                - cell "0":
                  - textbox "node:UIF-00000 Y": "0"
                - cell "0":
                  - textbox "node:UIF-00000 Z": "0"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00000 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00001 UI benchmark node 00001 0.1 -0.008621 0.003494 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00001":
                  - button "node:UIF-00001"
                - cell "UI benchmark node 00001":
                  - textbox "node:UIF-00001 Label": UI benchmark node 00001
                - cell "0.1":
                  - textbox "node:UIF-00001 X": "0.1"
                - cell "-0.008621":
                  - textbox "node:UIF-00001 Y": "-0.008621"
                - cell "0.003494":
                  - textbox "node:UIF-00001 Z": "0.003494"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00001 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00002 UI benchmark node 00002 0.2 0.007379 0.007128 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00002":
                  - button "node:UIF-00002"
                - cell "UI benchmark node 00002":
                  - textbox "node:UIF-00002 Label": UI benchmark node 00002
                - cell "0.2":
                  - textbox "node:UIF-00002 X": "0.2"
                - cell "0.007379":
                  - textbox "node:UIF-00002 Y": "0.007379"
                - cell "0.007128":
                  - textbox "node:UIF-00002 Z": "0.007128"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00002 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00003 UI benchmark node 00003 0.3 0.007084 0.001271 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00003":
                  - button "node:UIF-00003"
                - cell "UI benchmark node 00003":
                  - textbox "node:UIF-00003 Label": UI benchmark node 00003
                - cell "0.3":
                  - textbox "node:UIF-00003 X": "0.3"
                - cell "0.007084":
                  - textbox "node:UIF-00003 Y": "0.007084"
                - cell "0.001271":
                  - textbox "node:UIF-00003 Z": "0.001271"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00003 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00004 UI benchmark node 00004 0.4 -0.008844 -0.005178 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00004":
                  - button "node:UIF-00004"
                - cell "UI benchmark node 00004":
                  - textbox "node:UIF-00004 Label": UI benchmark node 00004
                - cell "0.4":
                  - textbox "node:UIF-00004 X": "0.4"
                - cell "-0.008844":
                  - textbox "node:UIF-00004 Y": "-0.008844"
                - cell "-0.005178":
                  - textbox "node:UIF-00004 Z": "-0.005178"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00004 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00005 UI benchmark node 00005 0.5 -0.000598 -0.007318 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00005":
                  - button "node:UIF-00005"
                - cell "UI benchmark node 00005":
                  - textbox "node:UIF-00005 Label": UI benchmark node 00005
                - cell "0.5":
                  - textbox "node:UIF-00005 X": "0.5"
                - cell "-0.000598":
                  - textbox "node:UIF-00005 Y": "-0.000598"
                - cell "-0.007318":
                  - textbox "node:UIF-00005 Z": "-0.007318"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00005 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00006 UI benchmark node 00006 0.6 -0.002379 -0.006069 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00006":
                  - button "node:UIF-00006"
                - cell "UI benchmark node 00006":
                  - textbox "node:UIF-00006 Label": UI benchmark node 00006
                - cell "0.6":
                  - textbox "node:UIF-00006 X": "0.6"
                - cell "-0.002379":
                  - textbox "node:UIF-00006 Y": "-0.002379"
                - cell "-0.006069":
                  - textbox "node:UIF-00006 Z": "-0.006069"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00006 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00007 UI benchmark node 00007 0.7 -0.007963 -0.004501 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00007":
                  - button "node:UIF-00007"
                - cell "UI benchmark node 00007":
                  - textbox "node:UIF-00007 Label": UI benchmark node 00007
                - cell "0.7":
                  - textbox "node:UIF-00007 X": "0.7"
                - cell "-0.007963":
                  - textbox "node:UIF-00007 Y": "-0.007963"
                - cell "-0.004501":
                  - textbox "node:UIF-00007 Z": "-0.004501"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00007 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00008 UI benchmark node 00008 0.8 -0.008741 0.008924 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00008":
                  - button "node:UIF-00008"
                - cell "UI benchmark node 00008":
                  - textbox "node:UIF-00008 Label": UI benchmark node 00008
                - cell "0.8":
                  - textbox "node:UIF-00008 X": "0.8"
                - cell "-0.008741":
                  - textbox "node:UIF-00008 Y": "-0.008741"
                - cell "0.008924":
                  - textbox "node:UIF-00008 Z": "0.008924"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00008 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00009 UI benchmark node 00009 0.9 0.005366 -0.000018 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00009":
                  - button "node:UIF-00009"
                - cell "UI benchmark node 00009":
                  - textbox "node:UIF-00009 Label": UI benchmark node 00009
                - cell "0.9":
                  - textbox "node:UIF-00009 X": "0.9"
                - cell "0.005366":
                  - textbox "node:UIF-00009 Y": "0.005366"
                - cell "-0.000018":
                  - textbox "node:UIF-00009 Z": "-0.000018"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00009 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00010 UI benchmark node 00010 1 -0.007599 0.006892 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00010":
                  - button "node:UIF-00010"
                - cell "UI benchmark node 00010":
                  - textbox "node:UIF-00010 Label": UI benchmark node 00010
                - cell "1":
                  - textbox "node:UIF-00010 X": "1"
                - cell "-0.007599":
                  - textbox "node:UIF-00010 Y": "-0.007599"
                - cell "0.006892":
                  - textbox "node:UIF-00010 Z": "0.006892"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00010 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00011 UI benchmark node 00011 1.1 -0.00614 -0.000131 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00011":
                  - button "node:UIF-00011"
                - cell "UI benchmark node 00011":
                  - textbox "node:UIF-00011 Label": UI benchmark node 00011
                - cell "1.1":
                  - textbox "node:UIF-00011 X": "1.1"
                - cell "-0.00614":
                  - textbox "node:UIF-00011 Y": "-0.00614"
                - cell "-0.000131":
                  - textbox "node:UIF-00011 Z": "-0.000131"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00011 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00012 UI benchmark node 00012 1.2 0.006592 -0.003488 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00012":
                  - button "node:UIF-00012"
                - cell "UI benchmark node 00012":
                  - textbox "node:UIF-00012 Label": UI benchmark node 00012
                - cell "1.2":
                  - textbox "node:UIF-00012 X": "1.2"
                - cell "0.006592":
                  - textbox "node:UIF-00012 Y": "0.006592"
                - cell "-0.003488":
                  - textbox "node:UIF-00012 Z": "-0.003488"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00012 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00013 UI benchmark node 00013 1.3 0.000269 -0.005106 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00013":
                  - button "node:UIF-00013"
                - cell "UI benchmark node 00013":
                  - textbox "node:UIF-00013 Label": UI benchmark node 00013
                - cell "1.3":
                  - textbox "node:UIF-00013 X": "1.3"
                - cell "0.000269":
                  - textbox "node:UIF-00013 Y": "0.000269"
                - cell "-0.005106":
                  - textbox "node:UIF-00013 Z": "-0.005106"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00013 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00014 UI benchmark node 00014 1.4 0.003938 -0.008687 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00014":
                  - button "node:UIF-00014"
                - cell "UI benchmark node 00014":
                  - textbox "node:UIF-00014 Label": UI benchmark node 00014
                - cell "1.4":
                  - textbox "node:UIF-00014 X": "1.4"
                - cell "0.003938":
                  - textbox "node:UIF-00014 Y": "0.003938"
                - cell "-0.008687":
                  - textbox "node:UIF-00014 Z": "-0.008687"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00014 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - row "node:UIF-00015 UI benchmark node 00015 1.5 0.007479 -0.003472 invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                - rowheader "node:UIF-00015":
                  - button "node:UIF-00015"
                - cell "UI benchmark node 00015":
                  - textbox "node:UIF-00015 Label": UI benchmark node 00015
                - cell "1.5":
                  - textbox "node:UIF-00015 X": "1.5"
                - cell "0.007479":
                  - textbox "node:UIF-00015 Y": "0.007479"
                - cell "-0.003472":
                  - textbox "node:UIF-00015 Z": "-0.003472"
                - cell "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data":
                  - textbox "node:UIF-00015 Provenance": invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
        - alert: No space is available for review rows. Expand the table view to continue.
        - button "Queue changed cells" [disabled]
        - button "Clear grid edits" [disabled]
        - paragraph: Grid mode fans each changed cell into a structured review intent; storage remains local.
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
        - button "Show All" [disabled]
        - text: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
        - button "Fit Model"
        - button "Fit Visible"
        - button "Fit Selection" [disabled]
      - group "Viewport geometry":
        - button "Schematic" [pressed]
        - button "Actual OD"
        - button "Measure"
    - text: "Selected: project:UIF-1000"
    - status "Schematic centerline geometry"
    - status "View command status": No view command dispatched.
    - button "Select UI benchmark node 00454 in viewport": UIF-00454
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
      - 'heading "Generated UI foundation 1000-pipe model — project: project:UIF-1000" [level=2]'
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
  - button "3 Issues"
  - text: "project: project:UIF-1000 Entered"
  - button "About SWBPIPE…"
```

# Test source

```ts
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
  147 |       const slot = root.querySelector<HTMLElement>(scroller === "engineering-table-rows" ? ".engineering-table-body-slot" : ".bulk-grid-body-slot")!;
  148 |       return { top: rows.scrollTop, clientHeight: rows.clientHeight, scrollHeight: rows.scrollHeight, slotHeight: slot.clientHeight,
  149 |         filterY: root.querySelector(".model-tree-controls")!.getBoundingClientRect().y,
  150 |         familyY: root.querySelector(".entity-grid-tabs")!.getBoundingClientRect().y,
  151 |         footerY: root.querySelector(scroller === "engineering-table-rows" ? ".engineering-table-footer" : ".entity-grid-actions")!.getBoundingClientRect().y };
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
  187 |   expect(await page.locator(".engineering-table-body-slot").evaluate((node) => node.clientHeight)).toBeGreaterThan(0);
  188 |   const errorState = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(errorState);
  189 |   const toggle = page.getByTestId("node-grid-review-disclosure"); await toggle.click(); await expect(table).toBeHidden(); await expect(page.getByTestId("retained-direct-draft")).toBeVisible();
  190 |   await expect(toggle).toContainText("Return to node coordinates"); const bulk = page.getByTestId("entity-grid-input-node:N-100-y"); await bulk.fill("0.5");
  191 |   const reviewState = await gridChromeBounds(page, true); await page.locator(".entity-grid-scroll").hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page, true)).toEqual(reviewState);
  192 |   await page.getByTestId("entity-grid-type-pipes").click(); await expect(page.getByTestId("entity-grid-table-pipes")).toBeVisible();
  193 |   await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulk).toHaveValue("0.5"); await toggle.click(); await expect(table).toBeVisible(); await expect(editor).toHaveValue("invalid retained");
  194 |   await expect(toggle).toContainText("1 retained draft"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText("0");
  195 |   await toggle.click(); await page.getByTestId("clear-entity-grid-drafts").click(); await toggle.click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  196 |   await info.attach("vertical-fixed-rectangles", { body: JSON.stringify({ before, errorState, reviewState, wheelEvidence }, null, 2), contentType: "application/json" });
  197 |   await page.screenshot({ path: info.outputPath("b4-short-fixed-chrome.png") });
  198 | });
  199 | 
  200 | test("B4 virtual Grid confines body scrolling and boundary wheel without moving chrome", async ({ page, browser }, info) => {
  201 |   await attachBrowserIdentity(browser, info); const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json"); await openBoundedGrid(page);
  202 |   const wheelEvidence: unknown[] = [];
  203 |   const before = await gridChromeBounds(page); const rows = page.getByTestId("engineering-table-rows");
  204 |   const scrollState = () => rows.evaluate((node) => ({ top: node.scrollTop, maximum: node.scrollHeight - node.clientHeight, height: node.clientHeight }));
  205 |   expect((await scrollState()).height).toBeGreaterThan(0);
  206 |   await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  207 |   wheelEvidence.push(await tableWheel(page, 1000000, info, true)); await expect.poll(async () => { const state = await scrollState(); return state.maximum - state.top; }).toBe(0);
  208 |   wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  209 |   wheelEvidence.push(await tableWheel(page, -1000000, info, true));
  210 |   // CDP's large reversal can end a few pixels above the boundary. One further
  211 |   // real wheel input establishes top; the following separate input tests chaining.
  212 |   if ((await scrollState()).top > 0) wheelEvidence.push(await tableWheel(page, -600, info, true));
  213 |   await expect.poll(async () => (await scrollState()).top).toBe(0);
  214 |   wheelEvidence.push(await tableWheel(page, -600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  215 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  216 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(model.nodes.at(-1).id);
  217 |   await expect(rows.locator('[role="row"]')).toHaveCount(1); const filtered = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(filtered);
  218 |   await filter.fill(""); const restored = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(restored);
  219 |   // Swap the mounted virtual body through a small family, changing available
  220 |   // width while it is absent, then verify the new element owns its observation.
  221 |   const review = page.getByTestId("node-grid-review-disclosure"); await review.click();
> 222 |   const bulkRows = page.getByTestId("entity-grid-virtual-rows"); await expect(bulkRows).toBeVisible();
      |                                                                                         ^ Error: expect(locator).toBeVisible() failed
  223 |   await page.getByTestId("entity-grid-type-sections").click(); await expect(bulkRows).toHaveCount(0);
  224 |   await page.getByTestId("toggle-inspector").click(); await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulkRows).toBeVisible();
  225 |   await expect.poll(() => bulkRows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  226 |   const bulkChrome = await gridChromeBounds(page, true); await bulkRows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true, "entity-grid-virtual-rows"));
  227 |   await expect.poll(() => bulkRows.evaluate((node) => node.scrollTop)).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page, true)).toEqual(bulkChrome);
  228 |   await review.click(); await expect(rows).toBeVisible(); await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  229 |   // A page makes the still-laid-out stage inert. Positive resize observations
  230 |   // must remain current even before interaction is restored.
  231 |   await openWorkspaceSection(page, "project"); await page.getByTestId("toggle-inspector").click();
  232 |   await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  233 |   await showModelTree(page); await expect(rows).toBeVisible(); await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  234 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  235 |   await info.attach("vertical-wheel-steps", { body: JSON.stringify({ before, filtered, restored, bulkChrome, wheelEvidence }, null, 2), contentType: "application/json" });
  236 |   await page.screenshot({ path: info.outputPath("b4-virtual-fixed-chrome.png") });
  237 | });
  238 | 
```