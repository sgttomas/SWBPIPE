# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: c3-viewport-visibility.spec.ts >> C3 snapshot, Hide precedence, persistent hidden count, theme and Show All preserve model/history
- Location: e2e/c3-viewport-visibility.spec.ts:78:1

# Error details

```
Error: expect(received).toMatchObject(expected)

- Expected  - 3
+ Received  + 3

  Object {
-   "bottom": true,
+   "bottom": false,
    "focusVisible": true,
    "left": true,
    "right": true,
-   "top": true,
-   "unobscured": true,
+   "top": false,
+   "unobscured": false,
  }
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
      - paragraph [ref=e18]: Generated UI local-render-origin precision probe
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
      - button "Issues, 2" [ref=e54] [cursor=pointer]:
        - img [ref=e55]
        - generic [ref=e57]: Issues
        - generic [ref=e58]: "2"
    - group "Panels" [ref=e59]:
      - button "Inspector" [ref=e61] [cursor=pointer]:
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
      - option "System"
      - option "Light" [selected]
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
          - button "Issues, 2" [ref=e128] [cursor=pointer]:
            - img [ref=e129]
            - generic [ref=e131]: Issues
            - generic [ref=e132]: "2"
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
            - button "Tree" [pressed] [ref=e149]:
              - img [ref=e150]
              - text: Tree
            - button "Grid" [ref=e153]:
              - img [ref=e154]
              - text: Grid
          - region "Model tree filtering" [ref=e156]:
            - generic [ref=e157]:
              - img [ref=e158]
              - generic [ref=e161]: Filter model
              - searchbox "Filter model tree" [ref=e162]: pipe:C3-B
            - generic [ref=e163]: 1 of 13 model entities visible
            - button "Clear model tree filter" [ref=e164] [cursor=pointer]:
              - img [ref=e165]
          - tree "Model" [ref=e168]:
            - generic [ref=e169]:
              - treeitem "Pipes" [expanded] [level=1] [ref=e171] [cursor=pointer]:
                - generic [ref=e172]: ▾
                - strong [ref=e173]: Pipes
              - treeitem "C3 B pipe:C3-B Hidden" [level=2] [selected] [ref=e175] [cursor=pointer]:
                - img [ref=e176]
                - generic [ref=e180]:
                  - strong [ref=e181]: C3 B
                  - generic [ref=e182]: pipe:C3-B
                - generic [ref=e183]: Hidden
      - separator "Resize table and canvas" [ref=e184]
      - generic [ref=e186]:
        - generic [ref=e187]:
          - group "Viewport controls" [ref=e188]:
            - generic [ref=e189]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e190]:
              - generic "Deformation · unavailable" [ref=e191] [cursor=pointer]
            - group "Viewport display toggles" [ref=e192]:
              - button "Labels" [pressed] [ref=e193]
              - button "Loads" [pressed] [ref=e194]
              - button "Grid" [pressed] [ref=e195]
            - group "Viewport selection tools" [ref=e196]:
              - button "Box Select" [ref=e197]
              - generic [ref=e198]:
                - generic [ref=e199]: Selection filter
                - combobox "Selection filter" [ref=e200]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [ref=e201]
              - button "Isolate" [ref=e202]
              - button "Show All" [ref=e203]
              - generic [ref=e204]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e205]
              - button "Fit Visible" [ref=e206]
              - button "Fit Selection" [disabled] [ref=e207]
            - group "Viewport geometry" [ref=e208]:
              - button "Schematic" [pressed] [ref=e209]
              - button "Actual OD" [ref=e210]
              - button "Measure" [ref=e211]
          - generic "Viewport status" [ref=e212]:
            - 'generic "Selected pipe: pipe:C3-B" [ref=e213]': "Selected: pipe:C3-B"
            - button "1 hidden · Show all" [active] [ref=e214]
            - status "1 selected item is hidden" [ref=e215]
            - status "Schematic centerline geometry" [ref=e216]
            - status "View command status" [ref=e217]: 1 selected item hidden.
        - generic [ref=e218]:
          - generic "Three.js pipe centerline viewport" [ref=e219]
          - generic "Viewport entity selection":
            - button "Select C3 A in viewport" [ref=e221] [cursor=pointer]:
              - img [ref=e222]
              - generic [ref=e226]: C3-A
            - button "Select node:C3-loose in viewport" [ref=e227] [cursor=pointer]:
              - img [ref=e228]
              - generic [ref=e231]: C3-loose
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e232]:
            - button "Front" [ref=e233] [cursor=pointer]
            - button "Top" [ref=e234] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e235] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e236]:
          - generic "Object creation tools" [ref=e237]:
            - button "Node" [ref=e238] [cursor=pointer]:
              - img [ref=e239]
              - text: Node
            - button "Pipe" [ref=e241] [cursor=pointer]:
              - img [ref=e242]
              - text: Pipe
            - button "Support" [ref=e246] [cursor=pointer]:
              - img [ref=e247]
              - text: Support
            - button "Component" [ref=e250] [cursor=pointer]:
              - img [ref=e251]
              - text: Component
            - button "Load" [ref=e254] [cursor=pointer]:
              - img [ref=e255]
              - text: Load
          - generic "Model focus" [ref=e257]: Select
          - group [ref=e258]:
            - generic "Selection & navigation" [ref=e259] [cursor=pointer]
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e260]:
      - button "Agent" [disabled] [ref=e262]:
        - img [ref=e263]
        - generic [ref=e266]: Agent
  - generic "Workspace status" [ref=e267]:
    - generic "Analysis statuses" [ref=e268]:
      - button "Solver · Not solved" [ref=e270] [cursor=pointer]
    - button "2 Issues" [ref=e271] [cursor=pointer]:
      - img [ref=e272]
      - text: 2 Issues
    - generic "Selection" [ref=e274]: "pipe: pipe:C3-B"
    - generic "Display units" [ref=e275]: Entered
    - button "About SWBPIPE…" [ref=e276] [cursor=pointer]:
      - img [ref=e277]
```

# Test source

```ts
  25  |   return model;
  26  | }
  27  | const control = (page: Page, name: string) => page.getByRole("button", { name, exact: true });
  28  | const label = (page: Page, id: string) => page.getByTestId(`viewport-select-${id}`);
  29  | async function ready(page: Page) {
  30  |   await expect.poll(async () => page.evaluate(() => {
  31  |     const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
  32  |     return "status" in current.viewport ? null : current.viewport.resources.ownedPendingRafCount;
  33  |   })).toBe(0);
  34  | }
  35  | async function selected(page: Page) {
  36  |   return page.evaluate(() => {
  37  |     const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
  38  |     if ("status" in current.viewport) throw new Error("Viewport unavailable");
  39  |     return current.viewport.selection.orderedRefs;
  40  |   });
  41  | }
  42  | async function point(page: Page, authoredPoint: { x: number; y: number; z: number }) {
  43  |   await ready(page);
  44  |   return page.evaluate((authoredPoint) => {
  45  |     const api = globalThis.__openPipeStressUiDiagnosticsV1;
  46  |     const current = api.readCurrent();
  47  |     if ("status" in current.viewport || current.model.generation === null) throw new Error("Viewport unavailable");
  48  |     const projected = api.projectAuthoredPoint({ modelGeneration: current.model.generation, cameraSequence: current.viewport.camera.sequence, authoredPoint });
  49  |     if (projected.status !== "available" || !projected.insideCanvasCss) throw new Error("Point outside canvas");
  50  |     const bounds = document.querySelector('[data-testid="viewport-canvas"] canvas')!.getBoundingClientRect();
  51  |     return { x: bounds.left + projected.canvasCssPoint.x, y: bounds.top + projected.canvasCssPoint.y };
  52  |   }, authoredPoint);
  53  | }
  54  | async function expectMainCanvasPoint(page: Page, p: { x: number; y: number }) {
  55  |   expect(await page.evaluate(({ x, y }) => document.elementFromPoint(x, y) === document.querySelector('[data-testid="viewport-canvas"] canvas'), p)).toBe(true);
  56  | }
  57  | async function history(page: Page) {
  58  |   await openWorkspaceSection(page, "operations");
  59  |   const value = await page.getByTestId("session-history-chip").innerText();
  60  |   await showCanvas(page);
  61  |   return value;
  62  | }
  63  | 
  64  | async function narrowBothPane(page: Page) {
  65  |   await page.getByTestId("view-switch-both").click();
  66  |   const splitter = page.getByTestId("resize-model-tree");
  67  |   await splitter.focus();
  68  |   for (let step = 0; step < 50; step++) {
  69  |     if (await splitter.getAttribute("aria-valuenow") === await splitter.getAttribute("aria-valuemax")) break;
  70  |     await page.keyboard.press("ArrowRight");
  71  |   }
  72  |   await expect(splitter).toHaveAttribute("aria-valuenow", (await splitter.getAttribute("aria-valuemax"))!);
  73  |   const box = await page.getByTestId("viewport-canvas").boundingBox();
  74  |   expect(box).not.toBeNull();
  75  |   expect(Math.round(box!.width)).toBe(220);
  76  | }
  77  | 
  78  | test("C3 snapshot, Hide precedence, persistent hidden count, theme and Show All preserve model/history", async ({ page }, info) => {
  79  |   await gotoModel(page, await fixture());
  80  |   const beforeHash = await currentModelHashThroughVisibleExport(page);
  81  |   const beforeHistory = await history(page);
  82  |   await selectTreeRow(page, "pipe", "pipe:C3-A");
  83  |   await clearTreeFilter(page);
  84  |   await ready(page);
  85  |   const normalImage = await page.getByTestId("viewport-canvas").screenshot();
  86  |   await control(page, "Isolate").click();
  87  |   await ready(page);
  88  |   const isolatedImage = await page.getByTestId("viewport-canvas").screenshot();
  89  |   expect(isolatedImage.equals(normalImage), "Isolate changes the drawn body presentation").toBe(false);
  90  |   await info.attach("before-isolate", { body: normalImage, contentType: "image/png" });
  91  |   await info.attach("after-isolate", { body: isolatedImage, contentType: "image/png" });
  92  |   await expect(label(page, "pipe:C3-A")).toHaveAttribute("data-dimmed", "false");
  93  |   await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  94  |   await expect(control(page, "Show All")).toBeEnabled();
  95  |   await expect(page.getByTestId("viewport-hidden-count")).toHaveCount(0);
  96  |   for (const theme of ["dark", "light"] as const) {
  97  |     await setAppearance(page, theme, "comfortable");
  98  |     await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  99  |     expect(await label(page, "pipe:C3-B").locator("span").evaluate(el => getComputedStyle(el).opacity)).toBe("0.2");
  100 |   }
  101 |   // Table selection does not rewrite the snapshot, including hidden table selections.
  102 |   await selectTreeRow(page, "pipe", "pipe:C3-B");
  103 |   await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  104 |   await control(page, "Hide").click();
  105 |   await expect(label(page, "pipe:C3-B")).toHaveCount(0);
  106 |   const hidden = page.getByTestId("viewport-hidden-count");
  107 |   await expect(hidden).toHaveText("1 hidden · Show all");
  108 |   await selectTreeRow(page, "pipe", "pipe:C3-B", { toggle: true });
  109 |   await expect.poll(() => selected(page)).toEqual([]);
  110 |   await expect(hidden).toHaveText("1 hidden · Show all");
  111 |   await selectTreeRow(page, "pipe", "pipe:C3-B");
  112 |   await expect(label(page, "pipe:C3-B")).toHaveCount(0);
  113 |   await narrowBothPane(page);
  114 |   await hidden.focus();
  115 |   await page.keyboard.press("Shift+Tab"); await page.keyboard.press("Tab");
  116 |   await expect(hidden).toBeFocused();
  117 |   const containment = await hidden.evaluate(el => {
  118 |     const r = el.getBoundingClientRect(), parent = el.closest(".viewport-toolbar-status-strip")!.getBoundingClientRect();
  119 |     const style = getComputedStyle(el);
  120 |     const points = [[r.left + 2, r.top + 2], [r.right - 2, r.top + 2], [r.left + 2, r.bottom - 2], [r.right - 2, r.bottom - 2], [(r.left + r.right) / 2, (r.top + r.bottom) / 2]];
  121 |     return { top: r.top >= parent.top, bottom: r.bottom <= parent.bottom, left: r.left >= parent.left, right: r.right <= parent.right,
  122 |       width: r.width, height: r.height, focusVisible: el.matches(":focus-visible"), outlineWidth: style.outlineWidth, outlineOffset: style.outlineOffset,
  123 |       unobscured: points.every(([x, y]) => el.contains(document.elementFromPoint(x, y))) };
  124 |   });
> 125 |   expect(containment).toMatchObject({ top: true, bottom: true, left: true, right: true, focusVisible: true, unobscured: true });
      |                       ^ Error: expect(received).toMatchObject(expected)
  126 |   expect(containment.width).toBeGreaterThanOrEqual(24); expect(containment.height).toBeGreaterThanOrEqual(24);
  127 |   expect(parseFloat(containment.outlineWidth)).toBeGreaterThan(0); expect(parseFloat(containment.outlineOffset)).toBeLessThan(0);
  128 |   await info.attach("hidden-count-containment", { body: JSON.stringify(containment), contentType: "application/json" });
  129 |   await info.attach("narrow-hidden-count-focus", { body: await page.locator(".viewport-toolbar-status-strip").screenshot(), contentType: "image/png" });
  130 |   await hidden.click();
  131 |   await expect(hidden).toHaveCount(0);
  132 |   await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "false");
  133 |   await expect(control(page, "Show All")).toBeDisabled();
  134 |   expect(await currentModelHashThroughVisibleExport(page)).toBe(beforeHash);
  135 |   expect(await history(page)).toBe(beforeHistory);
  136 |   await info.attach("shown-after-clear", { body: await page.getByTestId("viewport-canvas").screenshot(), contentType: "image/png" });
  137 | });
  138 | 
  139 | test("C3 dimmed geometry remains available to hover, click and box selection", async ({ page }, info) => {
  140 |   await gotoModel(page, await fixture());
  141 |   await control(page, "Front").click();
  142 |   await page.getByTestId("viewport-fit-model").click();
  143 |   await selectTreeRow(page, "pipe", "pipe:C3-A");
  144 |   await control(page, "Isolate").click();
  145 |   await page.getByTestId("toggle-viewport-labels").click();
  146 |   const p = await point(page, { x: 0, y: 3, z: 0 });
  147 |   await expectMainCanvasPoint(page, p);
  148 |   await page.mouse.move(0, 0); await ready(page);
  149 |   const unhovered = await page.getByTestId("viewport-canvas").screenshot();
  150 |   await page.mouse.move(p.x, p.y); await ready(page);
  151 |   const hovered = await page.getByTestId("viewport-canvas").screenshot();
  152 |   expect(hovered.equals(unhovered), "dimmed body gains a hover halo").toBe(false);
  153 |   await info.attach("dimmed-hover", { body: hovered, contentType: "image/png" });
  154 |   await page.mouse.click(p.x, p.y);
  155 |   await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  156 |   await page.getByTestId("toggle-viewport-labels").click();
  157 |   await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  158 |   await page.getByTestId("toggle-viewport-labels").click();
  159 |   await page.getByTestId("viewport-box-select").click();
  160 |   await page.getByTestId("viewport-selection-filter").selectOption("pipes");
  161 |   const left = await point(page, { x: -4, y: 3, z: 0 });
  162 |   const right = await point(page, { x: 4, y: 3, z: 0 });
  163 |   const start = { x: Math.min(left.x, right.x) - 12, y: Math.min(left.y, right.y) - 12 };
  164 |   const end = { x: Math.max(left.x, right.x) + 12, y: Math.max(left.y, right.y) + 12 };
  165 |   await expectMainCanvasPoint(page, start); await expectMainCanvasPoint(page, end);
  166 |   await page.mouse.move(start.x, start.y); await page.mouse.down();
  167 |   await page.mouse.move(end.x, end.y, { steps: 5 }); await page.mouse.up();
  168 |   await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  169 |   await page.getByTestId("viewport-box-select").click();
  170 |   await page.getByTestId("toggle-viewport-labels").click();
  171 |   await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  172 | });
  173 | 
  174 | test("C3 nearer dimmed pipe wins unchanged point ordering and viewport keyboard focus is visible", async ({ page }, info) => {
  175 |   await gotoModel(page, await fixture(true));
  176 |   await control(page, "Front").click();
  177 |   await page.getByTestId("viewport-fit-model").click();
  178 |   const canvas = page.getByTestId("viewport-canvas");
  179 |   await narrowBothPane(page);
  180 |   await page.getByTestId("viewport-fit-model").click();
  181 |   await page.getByTestId("toggle-viewport-labels").click();
  182 |   const far = await point(page, { x: 0, y: 0, z: 0 });
  183 |   const near = await point(page, { x: 0, y: 0, z: 2 });
  184 |   expect(far.x).toBeCloseTo(near.x, 5); expect(far.y).toBeCloseTo(near.y, 5);
  185 |   await expectMainCanvasPoint(page, near); await page.mouse.click(near.x, near.y);
  186 |   await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  187 |   await selectTreeRow(page, "pipe", "pipe:C3-A");
  188 |   await canvas.focus(); await page.keyboard.press("i");
  189 |   await expect(control(page, "Show All")).toBeEnabled();
  190 |   const focus = await canvas.evaluate(el => {
  191 |     const s = getComputedStyle(el), r = el.getBoundingClientRect();
  192 |     return { visible: el.matches(":focus-visible"), width: s.outlineWidth, offset: s.outlineOffset, color: s.outlineColor, rect: r.toJSON() };
  193 |   });
  194 |   expect(focus.visible).toBe(true); expect(parseFloat(focus.width)).toBeGreaterThan(0); expect(parseFloat(focus.offset)).toBeLessThan(0);
  195 |   await info.attach("canvas-keyboard-focus", { body: await canvas.screenshot(), contentType: "image/png" });
  196 |   await info.attach("canvas-focus-style", { body: JSON.stringify(focus), contentType: "application/json" });
  197 |   const p = await point(page, { x: 0, y: 0, z: 2 });
  198 |   expect(p.x).toBeCloseTo(near.x, 5); expect(p.y).toBeCloseTo(near.y, 5);
  199 |   await expectMainCanvasPoint(page, p); await page.mouse.click(p.x, p.y);
  200 |   await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  201 |   await canvas.focus(); await page.keyboard.press("h");
  202 |   await expect(page.getByTestId("viewport-hidden-count")).toHaveText("1 hidden · Show all");
  203 |   // Shortcuts are inert while typing outside the viewport and with modifiers.
  204 |   const filter = page.getByTestId("model-tree-filter-input");
  205 |   await filter.fill("i"); await filter.press("h");
  206 |   await expect(page.getByTestId("viewport-hidden-count")).toHaveText("1 hidden · Show all");
  207 |   await filter.fill("");
  208 |   await control(page, "Show All").click();
  209 |   await canvas.focus(); await page.keyboard.press("Alt+i");
  210 |   await expect(control(page, "Show All")).toBeDisabled();
  211 | });
  212 | 
  213 | test("C3 isolation remains clearable when every drawable entity belongs to the snapshot", async ({ page }) => {
  214 |   const model = await fixture();
  215 |   model.nodes = [model.nodes[4]];
  216 |   model.pipe_segments = [];
  217 |   await gotoModel(page, model);
  218 |   await selectTreeRow(page, "node", "node:C3-loose");
  219 |   await control(page, "Isolate").click();
  220 |   await expect(label(page, "node:C3-loose")).toHaveAttribute("data-dimmed", "false");
  221 |   await expect(page.locator('.viewport-select-target[data-dimmed="true"]')).toHaveCount(0);
  222 |   await expect(page.getByTestId("viewport-hidden-count")).toHaveCount(0);
  223 |   await selectTreeRow(page, "node", "node:C3-loose", { toggle: true });
  224 |   await expect.poll(() => selected(page)).toEqual([]);
  225 |   await expect(control(page, "Show All")).toBeEnabled();
```