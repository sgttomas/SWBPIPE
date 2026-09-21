# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 Materials preserve mixed-unit editing, delayed review sort, history and saved ownership
- Location: e2e/b4-table-editing.spec.ts:362:1

# Error details

```
Error: expect(locator).toHaveValue(expected) failed

Locator:  getByTestId('material-engineering-table-review').getByRole('textbox')
Expected: "50"
Received: "530"
Timeout:  10000ms

Call log:
  - Expect "toHaveValue" with timeout 10000ms
  - waiting for getByTestId('material-engineering-table-review').getByRole('textbox')
    24 × locator resolved to <input value="530" aria-invalid="false" aria-label="material:UIF-INVENTED-01 Elastic [MPa]"/>
       - unexpected value "530"

```

```yaml
- textbox "material:UIF-INVENTED-01 Elastic [MPa]": "530"
```

# Test source

```ts
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
  333 |       await expect(editor).toHaveValue(first + second);
  334 |       expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([2, 2]);
  335 |       await editor.press("ArrowLeft");
  336 |       await table.getByRole("button", { name: policy === "review" ? "Keep draft" : "Apply", exact: true }).focus();
  337 |       await editor.focus();
  338 |       expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([1, 1]);
  339 |       await editor.press(field === "label" ? "s" : "3");
  340 |       await expect(editor).toHaveValue(first + (field === "label" ? "s" : "3") + second);
  341 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(original!); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  342 |       await cell.focus(); await page.keyboard.press(first); await page.keyboard.press(second); await expect(editor).toHaveValue(first + second);
  343 |       await table.getByRole("button", { name: policy === "review" ? "Keep draft" : "Apply", exact: true }).click(); await expect(cell).toHaveText(first + second);
  344 |       if (policy === "review") {
  345 |         await expect(table.getByRole("status")).toHaveText("Draft retained; model unchanged."); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  346 |       } else {
  347 |         await expect(page.getByTestId("workspace-undo")).toBeEnabled(); await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText(original!);
  348 |         await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await page.getByTestId("workspace-redo").click(); await expect(cell).toHaveText(first + second);
  349 |       }
  350 |       await cell.focus(); await page.keyboard.press("Enter");
  351 |       expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, 2]);
  352 |       await editor.press(field === "label" ? "t" : "4"); await expect(editor).toHaveValue(field === "label" ? "t" : "4");
  353 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(first + second);
  354 |       await cell.dblclick(); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, 2]);
  355 |       await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(first + second);
  356 |     });
  357 |   }
  358 | }
  359 | 
  360 | // Real operation/conversion services; only the public synthetic starting model
  361 | // and delivery timing of conversion results are controlled by this scenario.
  362 | test("B4 Materials preserve mixed-unit editing, delayed review sort, history and saved ownership", async ({ page, browser }, info) => {
  363 |   await attachBrowserIdentity(browser, info);
  364 |   const { model } = await readFixture("precision-origin-base.model.json");
  365 |   const material = model.materials[0]; const firstId = material.id; const secondId = "material:B4-second";
  366 |   model.materials = [{ ...material, elastic_modulus: { value: 200000, unit: "MPa" } },
  367 |     { ...material, id: secondId, label: "Synthetic second material", elastic_modulus: { value: 100000000000, unit: "Pa" } }];
  368 |   // Delay delivery after the existing Rust conversion has finished. No factors,
  369 |   // replacement conversion answers, or model mutation are injected.
  370 |   await page.route("**/src/services/displayQuantityService.ts", async (route) => {
  371 |     const response = await route.fetch(); const source = await response.text();
  372 |     expect(source).toContain("export async function convertDisplayQuantities(");
  373 |     await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
  374 | export async function convertDisplayQuantities(items) {
  375 |   const result = await originalConvertDisplayQuantities(items);
  376 |   const gate = window.__b4MaterialConversionGate;
  377 |   if (gate && gate.hold && items.some(item => item.id.includes('material') && item.id.includes('elastic'))) {
  378 |     await new Promise(resolve => gate.pending.push(resolve));
  379 |   }
  380 |   return result;
  381 | }
  382 | ` });
  383 |   });
  384 |   await page.addInitScript(() => { (window as any).__b4MaterialConversionGate = { hold: false, pending: [] }; });
  385 |   await gotoModel(page, model);
  386 |   await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  387 |   const direct = page.getByTestId("material-engineering-table"); const elastic = direct.getByTestId(`table-cell-${firstId}-elastic`);
  388 |   await expect(elastic.locator("..")).toHaveAttribute("aria-readonly", "false");
  389 |   await expect(elastic.locator("..").locator(".engineering-table-unit")).toHaveText("MPa");
  390 |   await direct.getByRole("button", { name: "Sort Elastic", exact: true }).click();
  391 |   await expect(direct.getByRole("rowheader").first()).toHaveText(secondId);
  392 |   await elastic.dblclick(); const edit = direct.getByRole("textbox"); await edit.fill("0"); await direct.getByRole("button", { name: "Apply", exact: true }).click();
  393 |   await expect(edit).toHaveAttribute("aria-invalid", "true"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  394 |   await edit.fill("210000"); await direct.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(elastic).toHaveText("200000");
  395 |   await elastic.dblclick(); await edit.fill("210000"); await direct.getByRole("button", { name: "Apply", exact: true }).click(); await expect(elastic).toHaveText("210000");
  396 |   await expect(page.getByTestId("project-edited")).toBeVisible(); await page.getByTestId("workspace-undo").click(); await expect(elastic).toHaveText("200000");
  397 |   await page.getByTestId("workspace-redo").click(); await expect(elastic).toHaveText("210000");
  398 |   await page.getByTestId("material-grid-review-disclosure").click(); const review = page.getByTestId("material-engineering-table-review");
  399 |   await review.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await review.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await expect(review.getByRole("rowheader").first()).toHaveText(firstId);
  400 |   const reviewCell = review.getByTestId(`review-cell-${firstId}-elastic`); await reviewCell.dblclick(); const input = review.getByRole("textbox");
  401 |   const retainedInput = await input.elementHandle();
  402 |   await page.evaluate(() => { (window as any).__b4MaterialConversionGate.hold = true; });
  403 |   await input.press("ControlOrMeta+a"); await input.press("5"); await input.press("0"); await input.press("ArrowLeft");
  404 |   await expect(input).toHaveValue("50"); await expect(input).toBeFocused();
  405 |   await expect(review.getByRole("status")).toContainText("sort unavailable"); await expect(review.getByRole("rowheader").first()).toHaveText(firstId);
  406 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  407 |   expect(await input.evaluate((node, original) => node === original, retainedInput)).toBe(true);
  408 |   expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([1, 1]);
  409 |   // Descending completion physically moves the active row while its native
  410 |   // editor, selection and text-Undo stack must remain owned and intact.
  411 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  412 |   await expect(review.getByRole("rowheader").first()).toHaveText(secondId); await expect(input).toBeFocused();
  413 |   expect(await input.evaluate((node, original) => node === original, retainedInput)).toBe(true);
  414 |   expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([1, 1]);
> 415 |   await input.press("3"); await expect(input).toHaveValue("530"); await input.press("ControlOrMeta+z"); await expect(input).toHaveValue("50");
      |                                                                                                                             ^ Error: expect(locator).toHaveValue(expected) failed
  416 |   await expect(elastic).toHaveText("210000");
  417 |   await review.getByRole("button", { name: "Keep draft", exact: true }).click(); await expect(review.getByText("Draft retained; model unchanged.")).toBeVisible();
  418 |   await page.getByTestId("entity-grid-type-nodes").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(reviewCell).toHaveText("50");
  419 |   await review.getByTestId(`review-cell-${secondId}-label`).dblclick(); await review.getByRole("textbox").fill("retained second"); await review.getByRole("button", { name: "Keep draft", exact: true }).click();
  420 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(firstId);
  421 |   await page.getByTestId("queue-entity-grid-intents").click(); await expect(page.getByTestId("operation-apply-row-editor-intent-1")).toContainText('"unit":"MPa"');
  422 |   await page.getByTestId("apply-intent-editor-intent-1").click(); await expect(page.getByTestId("operation-apply-summary")).toContainText("2 applied");
  423 |   await showModelTree(page); await filter.fill(""); await expect(review.getByTestId(`review-cell-${secondId}-label`)).toHaveText("retained second");
  424 |   await page.getByTestId("clear-entity-grid-drafts").click(); await expect(review.getByTestId(`review-cell-${secondId}-label`)).toHaveText("Synthetic second material");
  425 |   await page.getByTestId("material-grid-review-disclosure").click(); await expect(elastic).toHaveText("50");
  426 |   const geometry = await direct.evaluate((root) => {
  427 |     const box = (element: Element) => { const r = element.getBoundingClientRect(); return { x: r.x, y: r.y, width: r.width, height: r.height, bottom: r.bottom }; };
  428 |     return { pane: box(root.closest(".shell-table-pane")!), header: box(root.querySelector(".engineering-table-header")!), body: box(root.querySelector(".engineering-table-body-slot")!), footer: box(root.querySelector(".engineering-table-footer")!), units: [...root.querySelectorAll(".engineering-table-unit")].map((unit) => ({ text: unit.textContent, ...box(unit), client: unit.clientWidth, scroll: unit.scrollWidth })) };
  429 |   });
  430 |   expect(geometry.body.height).toBeGreaterThan(0); expect(geometry.footer.bottom).toBeLessThanOrEqual(geometry.pane.bottom); expect(geometry.header.bottom).toBeLessThanOrEqual(geometry.body.y);
  431 |   for (const unit of geometry.units) { expect(unit.width).toBeGreaterThan(0); expect(unit.scroll).toBeLessThanOrEqual(unit.client); }
  432 |   await info.attach("materials-contained-units", { body: JSON.stringify(geometry, null, 2), contentType: "application/json" });
  433 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  434 |   await showModelTree(page); await page.getByTestId("material-grid-review-disclosure").click();
  435 |   await page.evaluate(() => { (window as any).__b4MaterialConversionGate.hold = true; });
  436 |   await reviewCell.dblclick(); await review.getByRole("textbox").fill("70");
  437 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  438 |   await page.getByTestId("entity-grid-type-nodes").click();
  439 |   const nodeLabel = page.getByTestId("table-cell-node:N-100-label"); await nodeLabel.dblclick(); const nodeEditor = page.getByTestId("engineering-table").getByRole("textbox"); await nodeEditor.fill(" ");
  440 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  441 |   await expect(nodeEditor).toBeFocused(); await expect(nodeEditor).toHaveValue(" "); await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click();
  442 |   await page.getByTestId("entity-grid-type-materials").click(); await reviewCell.dblclick(); await review.getByRole("textbox").fill("80");
  443 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  444 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  445 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(elastic).toHaveText("50"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  446 |   await direct.getByTestId(`table-cell-${firstId}-label`).dblclick(); const newEditor = direct.getByRole("textbox"); await newEditor.press("R"); await newEditor.press("S");
  447 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  448 |   await expect(newEditor).toHaveValue("RS"); await expect(newEditor).toBeFocused(); await direct.getByRole("button", { name: "Cancel", exact: true }).click();
  449 |   await page.screenshot({ path: info.outputPath("b4-materials-reopened.png") });
  450 | });
  451 | 
```