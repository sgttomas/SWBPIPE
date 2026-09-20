# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: c3-viewport-visibility.spec.ts >> C3 deletion retains active empty snapshot; new geometry dims; project replacement clears it
- Location: e2e/c3-viewport-visibility.spec.ts:244:1

# Error details

```
Test timeout of 120000ms exceeded.
```

```
Error: locator.click: Test timeout of 120000ms exceeded.
Call log:
  - waiting for getByTestId('queue-delete-node-intent')
    - locator resolved to <button type="button" title="Queue node delete intent" data-testid="queue-delete-node-intent">…</button>
  - attempting click action
    2 × waiting for element to be visible, enabled and stable
      - element is not visible
    - retrying click action
    - waiting 20ms
    2 × waiting for element to be visible, enabled and stable
      - element is not visible
    - retrying click action
      - waiting 100ms
    231 × waiting for element to be visible, enabled and stable
        - element is not visible
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
              - searchbox "Filter model tree" [ref=e162]: node:C3-loose
            - generic [ref=e163]: 1 of 13 model entities visible
            - button "Clear model tree filter" [ref=e164] [cursor=pointer]:
              - img [ref=e165]
          - tree "Model" [ref=e168]:
            - generic [ref=e169]:
              - treeitem "Nodes" [expanded] [level=1] [ref=e171] [cursor=pointer]:
                - generic [ref=e172]: ▾
                - strong [ref=e173]: Nodes
              - treeitem "node:C3-loose node:C3-loose" [level=2] [selected] [ref=e175] [cursor=pointer]:
                - img [ref=e176]
                - generic [ref=e179]:
                  - strong [ref=e180]: node:C3-loose
                  - generic [ref=e181]: node:C3-loose
      - separator "Resize table and canvas" [ref=e182]
      - generic [ref=e184]:
        - generic [ref=e185]:
          - group "Viewport controls" [ref=e186]:
            - generic [ref=e187]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e188]:
              - generic "Deformation · unavailable" [ref=e189] [cursor=pointer]
            - group "Viewport display toggles" [ref=e190]:
              - button "Labels" [pressed] [ref=e191]
              - button "Loads" [pressed] [ref=e192]
              - button "Grid" [pressed] [ref=e193]
            - group "Viewport selection tools" [ref=e194]:
              - button "Box Select" [ref=e195]
              - generic [ref=e196]:
                - generic [ref=e197]: Selection filter
                - combobox "Selection filter" [ref=e198]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [ref=e199]
              - button "Isolate" [ref=e200]
              - button "Show All" [ref=e201]
              - generic [ref=e202]: Isolate captures the selected geometry. Other shown geometry is dimmed to 20% and remains available to click, hover and box selection; nearer dimmed geometry can be picked before farther undimmed geometry. Selection does not change the snapshot. Hide takes precedence. Show All clears both. I and H apply while focus is in the viewport.
              - button "Fit Model" [ref=e203]
              - button "Fit Visible" [ref=e204]
              - button "Fit Selection" [ref=e205]
            - group "Viewport geometry" [ref=e206]:
              - button "Schematic" [pressed] [ref=e207]
              - button "Actual OD" [ref=e208]
              - button "Measure" [ref=e209]
          - generic "Viewport status" [ref=e210]:
            - 'generic "Selected node: node:C3-loose" [ref=e211]': "Selected: node:C3-loose"
            - status "Schematic centerline geometry" [ref=e212]
            - status "View command status" [ref=e213]: 1 selected item isolated; other shown geometry is dimmed to 20%.
        - generic [ref=e214]:
          - generic "Three.js pipe centerline viewport" [ref=e215]
          - generic "Viewport entity selection":
            - button "Select node:C3-loose in viewport" [pressed] [ref=e217] [cursor=pointer]:
              - img [ref=e218]
              - generic [ref=e221]: C3-loose
            - button "Select C3 A in viewport" [ref=e222] [cursor=pointer]:
              - img [ref=e223]
              - generic [ref=e227]: C3-A
            - button "Select C3 B in viewport" [ref=e228] [cursor=pointer]:
              - img [ref=e229]
              - generic [ref=e233]: C3-B
            - button "Select node:C3-A0 in viewport" [ref=e234] [cursor=pointer]:
              - img [ref=e235]
              - generic [ref=e238]: C3-A0
            - button "Select node:C3-A1 in viewport" [ref=e239] [cursor=pointer]:
              - img [ref=e240]
              - generic [ref=e243]: C3-A1
            - button "Select node:C3-B0 in viewport" [ref=e244] [cursor=pointer]:
              - img [ref=e245]
              - generic [ref=e248]: C3-B0
            - button "Select node:C3-B1 in viewport" [ref=e249] [cursor=pointer]:
              - img [ref=e250]
              - generic [ref=e253]: C3-B1
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e254]:
            - button "Front" [ref=e255] [cursor=pointer]
            - button "Top" [ref=e256] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e257] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e258]:
          - generic "Object creation tools" [ref=e259]:
            - button "Node" [ref=e260] [cursor=pointer]:
              - img [ref=e261]
              - text: Node
            - button "Pipe" [ref=e263] [cursor=pointer]:
              - img [ref=e264]
              - text: Pipe
            - button "Support" [ref=e268] [cursor=pointer]:
              - img [ref=e269]
              - text: Support
            - button "Component" [ref=e272] [cursor=pointer]:
              - img [ref=e273]
              - text: Component
            - button "Load" [ref=e276] [cursor=pointer]:
              - img [ref=e277]
              - text: Load
          - generic "Model focus" [ref=e279]: Select
          - group [ref=e280]:
            - generic "Selection & navigation" [ref=e281] [cursor=pointer]
      - generic [ref=e282]:
        - button "Close inspector" [ref=e283]:
          - img [ref=e284]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
        - region "Property inspector" [ref=e287]:
          - 'heading "node:C3-loose — node: node:C3-loose" [level=2] [ref=e288]':
            - text: node:C3-loose
            - generic [ref=e289]: "— node: node:C3-loose"
          - tablist "Inspector views" [ref=e290]:
            - tab "Properties" [ref=e291]
            - tab "Task" [selected] [ref=e292]
          - tabpanel "Editor operation intent" [ref=e293]:
            - generic [ref=e295]: "Draft target: node: node:C3-loose"
            - heading "Edit name" [level=3] [ref=e296]:
              - img [ref=e297]
              - text: Edit name
            - generic [ref=e300]:
              - generic [ref=e301]:
                - generic [ref=e302]: Property
                - combobox "Property to edit" [ref=e304] [cursor=pointer]:
                  - generic [ref=e305]: Name
                  - text: ▾
              - region "Current value" [ref=e306]:
                - generic [ref=e307]: Current name
                - strong [ref=e308]: node:C3-loose
              - generic [ref=e309]:
                - generic [ref=e310]: New name
                - textbox "New name" [ref=e311]: node:C3-loose
              - generic "Task actions" [ref=e312]:
                - button "Add" [disabled] [ref=e313]:
                  - img [ref=e314]
                  - text: Add
                - button "Cancel" [ref=e315] [cursor=pointer]
                - button "Review" [disabled] [ref=e316]: Review
                - button "Apply" [disabled] [ref=e320]:
                  - img [ref=e321]
                  - text: Apply
            - group [ref=e324]:
              - generic "Operation details" [ref=e325] [cursor=pointer]
            - paragraph [ref=e326]: Validate to check this change before applying it.
          - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e327]:
      - button "Agent" [disabled] [ref=e329]:
        - img [ref=e330]
        - generic [ref=e333]: Agent
  - generic "Workspace status" [ref=e334]:
    - generic "Analysis statuses" [ref=e335]:
      - button "Solver · Not solved" [ref=e337] [cursor=pointer]
    - button "2 Issues" [ref=e338] [cursor=pointer]:
      - img [ref=e339]
      - text: 2 Issues
    - generic "Selection" [ref=e341]: "node: node:C3-loose"
    - generic "Display units" [ref=e342]: Entered
    - button "About SWBPIPE…" [ref=e343] [cursor=pointer]:
      - img [ref=e344]
```

# Test source

```ts
  149 |   expect(await history(page)).toBe(beforeHistory);
  150 |   await info.attach("shown-after-clear", { body: await page.getByTestId("viewport-canvas").screenshot(), contentType: "image/png" });
  151 | });
  152 | 
  153 | test("C3 dimmed geometry remains available to hover, click and box selection", async ({ page }, info) => {
  154 |   await gotoModel(page, await fixture());
  155 |   await control(page, "Front").click();
  156 |   await page.getByTestId("viewport-fit-model").click();
  157 |   await selectTreeRow(page, "pipe", "pipe:C3-A");
  158 |   await control(page, "Isolate").click();
  159 |   await page.getByTestId("toggle-viewport-labels").click();
  160 |   const p = await point(page, { x: 0, y: 3, z: 0 });
  161 |   await expectMainCanvasPoint(page, p);
  162 |   await page.mouse.move(0, 0); await ready(page);
  163 |   const unhovered = await page.getByTestId("viewport-canvas").screenshot();
  164 |   await page.mouse.move(p.x, p.y); await ready(page);
  165 |   const hovered = await page.getByTestId("viewport-canvas").screenshot();
  166 |   expect(hovered.equals(unhovered), "dimmed body gains a hover halo").toBe(false);
  167 |   await info.attach("dimmed-hover", { body: hovered, contentType: "image/png" });
  168 |   await page.mouse.click(p.x, p.y);
  169 |   await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  170 |   await page.getByTestId("toggle-viewport-labels").click();
  171 |   await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  172 |   await page.getByTestId("toggle-viewport-labels").click();
  173 |   await page.getByTestId("viewport-box-select").click();
  174 |   await page.getByTestId("viewport-selection-filter").selectOption("pipes");
  175 |   const left = await point(page, { x: -4, y: 3, z: 0 });
  176 |   const right = await point(page, { x: 4, y: 3, z: 0 });
  177 |   const start = { x: Math.min(left.x, right.x) - 12, y: Math.min(left.y, right.y) - 12 };
  178 |   const end = { x: Math.max(left.x, right.x) + 12, y: Math.max(left.y, right.y) + 12 };
  179 |   await expectMainCanvasPoint(page, start); await expectMainCanvasPoint(page, end);
  180 |   await page.mouse.move(start.x, start.y); await page.mouse.down();
  181 |   await page.mouse.move(end.x, end.y, { steps: 5 }); await page.mouse.up();
  182 |   await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  183 |   await page.getByTestId("viewport-box-select").click();
  184 |   await page.getByTestId("toggle-viewport-labels").click();
  185 |   await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  186 | });
  187 | 
  188 | test("C3 nearer dimmed pipe wins unchanged point ordering and viewport keyboard focus is visible", async ({ page }, info) => {
  189 |   await gotoModel(page, await fixture(true));
  190 |   await control(page, "Front").click();
  191 |   await page.getByTestId("viewport-fit-model").click();
  192 |   const canvas = page.getByTestId("viewport-canvas");
  193 |   await narrowBothPane(page);
  194 |   await page.getByTestId("viewport-fit-model").click();
  195 |   await page.getByTestId("toggle-viewport-labels").click();
  196 |   const far = await point(page, { x: 0, y: 0, z: 0 });
  197 |   const near = await point(page, { x: 0, y: 0, z: 2 });
  198 |   expect(far.x).toBeCloseTo(near.x, 5); expect(far.y).toBeCloseTo(near.y, 5);
  199 |   await expectMainCanvasPoint(page, near); await page.mouse.click(near.x, near.y);
  200 |   await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  201 |   await selectTreeRow(page, "pipe", "pipe:C3-A");
  202 |   await canvas.focus(); await page.keyboard.press("i");
  203 |   await expect(control(page, "Show All")).toBeEnabled();
  204 |   const focus = await canvas.evaluate(el => {
  205 |     const s = getComputedStyle(el), r = el.getBoundingClientRect();
  206 |     return { visible: el.matches(":focus-visible"), width: s.outlineWidth, offset: s.outlineOffset, color: s.outlineColor, rect: r.toJSON() };
  207 |   });
  208 |   expect(focus.visible).toBe(true); expect(parseFloat(focus.width)).toBeGreaterThan(0); expect(parseFloat(focus.offset)).toBeLessThan(0);
  209 |   await info.attach("canvas-keyboard-focus", { body: await canvas.screenshot(), contentType: "image/png" });
  210 |   await info.attach("canvas-focus-style", { body: JSON.stringify(focus), contentType: "application/json" });
  211 |   const p = await point(page, { x: 0, y: 0, z: 2 });
  212 |   expect(p.x).toBeCloseTo(near.x, 5); expect(p.y).toBeCloseTo(near.y, 5);
  213 |   await expectMainCanvasPoint(page, p); await page.mouse.click(p.x, p.y);
  214 |   await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  215 |   await canvas.focus(); await page.keyboard.press("h");
  216 |   await expect(page.getByTestId("viewport-hidden-count")).toHaveText("1 hidden · Show all");
  217 |   // Shortcuts are inert while typing outside the viewport and with modifiers.
  218 |   const filter = page.getByTestId("model-tree-filter-input");
  219 |   await filter.fill("i"); await filter.press("h");
  220 |   await expect(page.getByTestId("viewport-hidden-count")).toHaveText("1 hidden · Show all");
  221 |   await filter.fill("");
  222 |   await control(page, "Show All").click();
  223 |   await canvas.focus(); await page.keyboard.press("Alt+i");
  224 |   await expect(control(page, "Show All")).toBeDisabled();
  225 | });
  226 | 
  227 | test("C3 isolation remains clearable when every drawable entity belongs to the snapshot", async ({ page }) => {
  228 |   const model = await fixture();
  229 |   model.nodes = [model.nodes[4]];
  230 |   model.pipe_segments = [];
  231 |   await gotoModel(page, model);
  232 |   await selectTreeRow(page, "node", "node:C3-loose");
  233 |   await control(page, "Isolate").click();
  234 |   await expect(label(page, "node:C3-loose")).toHaveAttribute("data-dimmed", "false");
  235 |   await expect(page.locator('.viewport-select-target[data-dimmed="true"]')).toHaveCount(0);
  236 |   await expect(page.getByTestId("viewport-hidden-count")).toHaveCount(0);
  237 |   await selectTreeRow(page, "node", "node:C3-loose", { toggle: true });
  238 |   await expect.poll(() => selected(page)).toEqual([]);
  239 |   await expect(control(page, "Show All")).toBeEnabled();
  240 |   await activateWithKeyboard(page, control(page, "Show All"));
  241 |   await expect(control(page, "Show All")).toBeDisabled();
  242 | });
  243 | 
  244 | test("C3 deletion retains active empty snapshot; new geometry dims; project replacement clears it", async ({ page }) => {
  245 |   await gotoModel(page, await fixture());
  246 |   await selectTreeRow(page, "node", "node:C3-loose");
  247 |   await control(page, "Isolate").click();
  248 |   await startPropertyTaskFromTreeEntity(page, "node", "node:C3-loose");
> 249 |   await page.getByTestId("queue-delete-node-intent").click();
      |                                                      ^ Error: locator.click: Test timeout of 120000ms exceeded.
  250 |   await openWorkspaceSection(page, "operations");
  251 |   await page.locator('[data-testid^="operation-apply-row-"]').filter({ hasText: "node:C3-loose" }).getByRole("button", { name: "Apply", exact: true }).click();
  252 |   await showCanvas(page);
  253 |   await clearTreeFilter(page);
  254 |   await expect(typedTreeRow(page, "node", "node:C3-loose")).toHaveCount(0);
  255 |   await expect(control(page, "Show All")).toBeEnabled();
  256 |   await expect(label(page, "pipe:C3-A")).toHaveAttribute("data-dimmed", "true");
  257 |   await page.getByTestId("command-node").click();
  258 |   for (const [field, value] of Object.entries({ id: "node:C3-new", label: "C3 new", x: "1", y: "-2", z: "0", provenance: "invented_c3_visibility_input" })) {
  259 |     await page.getByTestId(`viewport-create-node-${field}`).fill(value);
  260 |   }
  261 |   await page.getByTestId("queue-explicit-node-intent").click();
  262 |   await page.getByTestId("apply-reviewed-draft").click();
  263 |   await page.getByTestId("workspace-select").click();
  264 |   await page.getByTestId("viewport-fit-model").click();
  265 |   await expect(label(page, "node:C3-new")).toHaveAttribute("data-dimmed", "true");
  266 |   await projectCommand(page, "new-blank");
  267 |   await expect(control(page, "Show All")).toBeDisabled();
  268 |   await expect(page.getByTestId("viewport-hidden-count")).toHaveCount(0);
  269 | });
  270 | 
  271 | test("C3 visibility leaves Current results current and never revives Historical overlays", async ({ page }) => {
  272 |   await page.goto("/");
  273 |   await openWorkspaceSection(page, "solve");
  274 |   await page.getByTestId("run-mechanics-preview").click();
  275 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  276 |   await showCanvas(page);
  277 |   await selectTreeRow(page, "pipe", "pipe:P-100");
  278 |   await control(page, "Isolate").click();
  279 |   await control(page, "Hide").click();
  280 |   await control(page, "Show All").click();
  281 |   await openWorkspaceSection(page, "results");
  282 |   await expect(page.getByTestId("historical-run-context")).toHaveCount(0);
  283 |   await projectCommand(page, "save-local", true);
  284 |   await expect(page.getByTestId("local-project-message")).toContainText("Saved");
  285 |   await projectCommand(page, "open-local", true);
  286 |   await openWorkspaceSection(page, "results");
  287 |   await expect(page.getByTestId("historical-run-context")).toBeVisible();
  288 |   await showCanvas(page);
  289 |   await selectTreeRow(page, "pipe", "pipe:P-100");
  290 |   await control(page, "Isolate").click();
  291 |   await control(page, "Show All").click();
  292 |   await expect(page.getByTestId("viewport-deformation-summary")).toContainText("result rows=0");
  293 |   await openWorkspaceSection(page, "results");
  294 |   await expect(page.getByTestId("historical-run-context")).toBeVisible();
  295 |   await openWorkspaceSection(page, "evidence");
  296 |   await expect(page.getByTestId("status-pill-solve-proof")).toHaveCount(0);
  297 | });
  298 | 
```