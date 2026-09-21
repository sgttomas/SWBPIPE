# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 text fields apply, recover filtered focus, preserve text Undo and retain review drafts through pointer Queue and Clear
- Location: e2e/b4-table-editing.spec.ts:260:1

# Error details

```
Error: expect(locator).toContainText(expected) failed

Locator: getByTestId('operation-apply-summary')
Expected substring: "1 applied"
Received string:    "0 queued; 4 applied this session. Applying validates through the structured-operation seam and replaces the session model document; saving the project stores it locally."
Timeout: 10000ms

Call log:
  - Expect "toContainText" with timeout 10000ms
  - waiting for getByTestId('operation-apply-summary')
    24 × locator resolved to <p class="muted" data-testid="operation-apply-summary">0 queued; 4 applied this session. Applying valida…</p>
       - unexpected value "0 queued; 4 applied this session. Applying validates through the structured-operation seam and replaces the session model document; saving the project stores it locally."

```

```yaml
- paragraph: 0 queued; 4 applied this session. Applying validates through the structured-operation seam and replaces the session model document; saving the project stores it locally.
```

# Test source

```ts
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
  214 |   await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
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
  225 |   await expect(rows.locator('[role="row"]')).toHaveCount(1); const filtered = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(filtered);
  226 |   await filter.fill(""); const restored = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(restored);
  227 |   // Swap the mounted virtual body through a small family, changing available
  228 |   // width while it is absent, then verify the new element owns its observation.
  229 |   const review = page.getByTestId("node-grid-review-disclosure"); await review.click();
  230 |   const bulkRows = page.getByTestId("engineering-table-review-rows"); await expect(bulkRows).toBeVisible();
  231 |   await page.getByTestId("entity-grid-type-sections").click(); await expect(bulkRows).toBeHidden();
  232 |   await page.getByTestId("toggle-inspector").click(); await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulkRows).toBeVisible();
  233 |   await expect.poll(() => bulkRows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  234 |   const bulkChrome = await gridChromeBounds(page, true); await bulkRows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true, "engineering-table-review-rows"));
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
  289 |   await page.getByTestId("layout-mode-tree").click(); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("node-grid-review-disclosure").click();
  290 |   await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  291 |   await expect(review.getByTestId("review-cell-node:N-100-label")).toHaveText("Q"); await expect(review.getByTestId("review-cell-node:N-110-label")).toHaveText("hidden retained");
  292 |   await filter.fill("node:N-100"); await review.getByTestId("review-cell-node:N-100-label").dblclick(); await review.getByRole("textbox").fill("   ");
  293 |   // Pointer Queue from the active editor must consume the current raw blank once.
  294 |   await page.getByTestId("queue-entity-grid-intents").click(); await expect(page.getByTestId("operation-apply-row-editor-intent-1")).toContainText("TBD");
> 295 |   await page.getByTestId("apply-intent-editor-intent-1").click(); await expect(page.getByTestId("operation-apply-summary")).toContainText("1 applied");
      |                                                                                                                             ^ Error: expect(locator).toContainText(expected) failed
  296 |   await showModelTree(page); await filter.fill(""); await expect(review.getByTestId("review-cell-node:N-110-label")).toHaveText("hidden retained");
  297 |   await review.getByTestId("review-cell-node:N-100-provenance").dblclick(); await review.getByRole("textbox").fill("clear active");
  298 |   await page.getByTestId("clear-entity-grid-drafts").click(); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("0 changed cells");
  299 |   await expect(review.getByRole("textbox")).toHaveCount(0); await page.getByTestId("node-grid-review-disclosure").click(); await expect(label).toHaveText("TBD");
  300 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  301 |   await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  302 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await expect(label).toHaveText("TBD"); await expect(provenance).toHaveText("TBD"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  303 | });
  304 | 
```