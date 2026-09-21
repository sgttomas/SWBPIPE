# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 short Grid keeps vertical chrome fixed and retains alternate review drafts
- Location: e2e/b4-table-editing.spec.ts:161:1

# Error details

```
Error: locator.evaluate: Test ended.
```

# Test source

```ts
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
  113 |   const headerZ = await grid.getByRole("columnheader").last().boundingBox(); const bodyZ = await z.locator("..").boundingBox();
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
  142 | async function tableWheel(page: import("@playwright/test").Page, deltaY: number) {
> 143 |   const observed = page.locator(".model-tree").evaluate((root) => new Promise((resolve) => {
      |                                                ^ Error: locator.evaluate: Test ended.
  144 |     root.addEventListener("wheel", (event) => {
  145 |       const sample = () => {
  146 |         const rows = root.querySelector<HTMLElement>('[data-testid="engineering-table-rows"]')!;
  147 |         const slot = root.querySelector<HTMLElement>(".engineering-table-body-slot")!;
  148 |         return { top: rows.scrollTop, clientHeight: rows.clientHeight, scrollHeight: rows.scrollHeight, slotHeight: slot.clientHeight,
  149 |           filterY: root.querySelector(".model-tree-controls")!.getBoundingClientRect().y,
  150 |           familyY: root.querySelector(".entity-grid-tabs")!.getBoundingClientRect().y,
  151 |           footerY: root.querySelector(".engineering-table-footer")!.getBoundingClientRect().y };
  152 |       };
  153 |       const target = (event.target as Element).outerHTML.slice(0, 400);
  154 |       requestAnimationFrame(() => { const first = sample(); requestAnimationFrame(() => resolve({ deltaX: (event as WheelEvent).deltaX, deltaY: (event as WheelEvent).deltaY, target, frames: [first, sample()] })); });
  155 |     }, { once: true, passive: true, capture: true });
  156 |   }));
  157 |   await page.mouse.wheel(0, deltaY);
  158 |   return observed;
  159 | }
  160 | 
  161 | test("B4 short Grid keeps vertical chrome fixed and retains alternate review drafts", async ({ page, browser }, info) => {
  162 |   await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible(); await openBoundedGrid(page);
  163 |   const table = page.getByTestId("engineering-table"); const rows = page.getByTestId("engineering-table-rows");
  164 |   const wheelEvidence: unknown[] = [];
  165 |   const before = await gridChromeBounds(page);
  166 |   await rows.hover(); wheelEvidence.push(await tableWheel(page, 600));
  167 |   await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  168 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600));
  169 |   await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  170 |   expect(await rows.evaluate((node) => ({ client: node.clientHeight, scroll: node.scrollHeight, top: node.scrollTop }))).toEqual({ client: 180, scroll: 180, top: 0 });
  171 |   const cell = page.getByTestId("table-cell-node:N-100-x"); await cell.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 X [m]" }); await editor.fill("invalid retained"); await editor.press("Enter");
  172 |   await expect(editor).toHaveAttribute("aria-invalid", "true");
  173 |   expect(await page.locator(".engineering-table-body-slot").evaluate((node) => node.clientHeight)).toBeGreaterThan(0);
  174 |   const errorState = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600)); await expect.poll(() => gridChromeBounds(page)).toEqual(errorState);
  175 |   const toggle = page.getByTestId("node-grid-review-disclosure"); await toggle.click(); await expect(table).toBeHidden(); await expect(page.getByTestId("retained-direct-draft")).toBeVisible();
  176 |   await expect(toggle).toContainText("Return to node coordinates"); const bulk = page.getByTestId("entity-grid-input-node:N-100-y"); await bulk.fill("0.5");
  177 |   const reviewState = await gridChromeBounds(page, true); await page.locator(".entity-grid-scroll").hover(); wheelEvidence.push(await tableWheel(page, 600)); await expect.poll(() => gridChromeBounds(page, true)).toEqual(reviewState);
  178 |   await page.getByTestId("entity-grid-type-pipes").click(); await expect(page.getByTestId("entity-grid-table-pipes")).toBeVisible();
  179 |   await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulk).toHaveValue("0.5"); await toggle.click(); await expect(table).toBeVisible(); await expect(editor).toHaveValue("invalid retained");
  180 |   await expect(toggle).toContainText("1 retained draft"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText("0");
  181 |   await toggle.click(); await page.getByTestId("clear-entity-grid-drafts").click(); await toggle.click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  182 |   await info.attach("vertical-fixed-rectangles", { body: JSON.stringify({ before, errorState, reviewState, wheelEvidence }, null, 2), contentType: "application/json" });
  183 |   await page.screenshot({ path: info.outputPath("b4-short-fixed-chrome.png") });
  184 | });
  185 | 
  186 | test("B4 virtual Grid confines body scrolling and boundary wheel without moving chrome", async ({ page, browser }, info) => {
  187 |   await attachBrowserIdentity(browser, info); const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json"); await openBoundedGrid(page);
  188 |   const wheelEvidence: unknown[] = [];
  189 |   const before = await gridChromeBounds(page); const rows = page.getByTestId("engineering-table-rows");
  190 |   const scrollState = () => rows.evaluate((node) => ({ top: node.scrollTop, maximum: node.scrollHeight - node.clientHeight, height: node.clientHeight }));
  191 |   expect((await scrollState()).height).toBeGreaterThan(0);
  192 |   await rows.hover(); wheelEvidence.push(await tableWheel(page, 700)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  193 |   wheelEvidence.push(await tableWheel(page, 1000000)); await expect.poll(async () => { const state = await scrollState(); return state.maximum - state.top; }).toBe(0);
  194 |   wheelEvidence.push(await tableWheel(page, 600)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  195 |   wheelEvidence.push(await tableWheel(page, -1000000)); await expect.poll(async () => (await scrollState()).top).toBe(0); wheelEvidence.push(await tableWheel(page, -600)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  196 |   await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  197 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(model.nodes.at(-1).id);
  198 |   await expect(rows.locator('[role="row"]')).toHaveCount(1); const filtered = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600)); await expect.poll(() => gridChromeBounds(page)).toEqual(filtered);
  199 |   await filter.fill(""); const restored = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 700)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(restored);
  200 |   await info.attach("vertical-wheel-steps", { body: JSON.stringify({ before, filtered, restored, wheelEvidence }, null, 2), contentType: "application/json" });
  201 |   await page.screenshot({ path: info.outputPath("b4-virtual-fixed-chrome.png") });
  202 | });
  203 | 
```