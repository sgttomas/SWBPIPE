# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 character-start direct label appends ordinary keys and preserves intentional selection
- Location: e2e/b4-table-editing.spec.ts:323:5

# Error details

```
Error: expect(locator).toHaveValue(expected) failed

Locator:  getByTestId('engineering-table').getByRole('textbox')
Expected: "qr"
Received: "r"
Timeout:  10000ms

Call log:
  - Expect "toHaveValue" with timeout 10000ms
  - waiting for getByTestId('engineering-table').getByRole('textbox')
    24 × locator resolved to <input value="r" aria-invalid="false" aria-label="node:N-100 Label"/>
       - unexpected value "r"

```

```yaml
- textbox "node:N-100 Label": r
```

# Test source

```ts
  233 |   await expect.poll(() => bulkRows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  234 |   const bulkChrome = await gridChromeBounds(page, true); await hoverTableBody(page, bulkRows); wheelEvidence.push(await tableWheel(page, 700, info, true, "engineering-table-review-rows"));
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
  295 |   const appliedBefore = Number((await page.getByTestId("operation-apply-summary").textContent())!.match(/(\d+) applied/)![1]);
  296 |   expect(appliedBefore).toBe(3);
  297 |   await page.getByTestId("apply-intent-editor-intent-1").click(); await expect(page.getByTestId("operation-apply-summary")).toContainText(`${appliedBefore + 1} applied`);
  298 |   await expect(page.getByTestId("operation-apply-summary")).toContainText("4 applied");
  299 |   await showModelTree(page); await filter.fill(""); await expect(review.getByTestId("review-cell-node:N-110-label")).toHaveText("hidden retained");
  300 |   await review.getByTestId("review-cell-node:N-100-provenance").dblclick(); await review.getByRole("textbox").fill("clear active");
  301 |   await page.getByTestId("clear-entity-grid-drafts").click(); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("0 changed cells");
  302 |   await expect(review.getByRole("textbox")).toHaveCount(0); await page.getByTestId("node-grid-review-disclosure").click(); await expect(label).toHaveText("TBD");
  303 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  304 |   await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  305 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await expect(label).toHaveText("TBD"); await expect(provenance).toHaveText("TBD"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  306 | });
  307 | 
  308 | // Move the real pointer inside the currently visible body intersection. Locator
  309 | // hover scrolls a wide rowgroup into view and would contaminate wheel-only bounds.
  310 | async function hoverTableBody(page: import("@playwright/test").Page, rows: import("@playwright/test").Locator) {
  311 |   const point = await rows.evaluate((body) => {
  312 |     const row = body.getBoundingClientRect(); const grid = body.closest('[role="grid"]')!.getBoundingClientRect();
  313 |     const left = Math.max(row.left, grid.left); const right = Math.min(row.right, grid.right);
  314 |     const top = Math.max(row.top, grid.top); const bottom = Math.min(row.bottom, grid.bottom);
  315 |     if (right <= left || bottom <= top) throw new Error("No visible table body for wheel input");
  316 |     return { x: (left + right) / 2, y: (top + bottom) / 2 };
  317 |   });
  318 |   await page.mouse.move(point.x, point.y);
  319 | }
  320 | 
  321 | for (const policy of ["direct", "review"] as const) {
  322 |   for (const field of ["label", "x"] as const) {
  323 |     test(`B4 character-start ${policy} ${field} appends ordinary keys and preserves intentional selection`, async ({ page, browser }, info) => {
  324 |       await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  325 |       await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  326 |       if (policy === "review") await page.getByTestId("node-grid-review-disclosure").click();
  327 |       const table = page.getByTestId(policy === "review" ? "engineering-table-review" : "engineering-table");
  328 |       const cell = table.getByTestId(`${policy === "review" ? "review" : "table"}-cell-node:N-100-${field}`);
  329 |       const original = await cell.textContent(); const first = field === "label" ? "q" : "1"; const second = field === "label" ? "r" : "2";
  330 |       const editor = table.getByRole("textbox");
  331 |       await cell.focus(); await page.keyboard.press(first); await page.keyboard.press(second);
  332 |       // Separate physical key events expose the selection defect; fill() would hide it.
> 333 |       await expect(editor).toHaveValue(first + second);
      |                            ^ Error: expect(locator).toHaveValue(expected) failed
  334 |       expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([2, 2]);
  335 |       await editor.press("ArrowLeft"); await editor.press(field === "label" ? "s" : "3");
  336 |       await expect(editor).toHaveValue(first + (field === "label" ? "s" : "3") + second);
  337 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(original!); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  338 |       await cell.focus(); await page.keyboard.press(first); await page.keyboard.press(second); await expect(editor).toHaveValue(first + second);
  339 |       await table.getByRole("button", { name: policy === "review" ? "Keep draft" : "Apply", exact: true }).click(); await expect(cell).toHaveText(first + second);
  340 |       if (policy === "review") {
  341 |         await expect(table.getByRole("status")).toHaveText("Draft retained; model unchanged."); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  342 |       } else {
  343 |         await expect(page.getByTestId("workspace-undo")).toBeEnabled(); await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText(original!);
  344 |         await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await page.getByTestId("workspace-redo").click(); await expect(cell).toHaveText(first + second);
  345 |       }
  346 |       await cell.focus(); await page.keyboard.press("Enter");
  347 |       expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, 2]);
  348 |       await editor.press(field === "label" ? "t" : "4"); await expect(editor).toHaveValue(field === "label" ? "t" : "4");
  349 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(first + second);
  350 |       await cell.dblclick(); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, 2]);
  351 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(first + second);
  352 |     });
  353 |   }
  354 | }
  355 | 
```