# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 Materials Apply Tab keeps the first Shear character while another quantity reconverts
- Location: e2e/b4-table-editing.spec.ts:506:1

# Error details

```
Error: expect(locator).toHaveValue(expected) failed

Locator: getByTestId('material-engineering-table').getByRole('textbox', { name: 'material:UIF-INVENTED-01 Shear [Pa]' })
Expected: "8"
Timeout: 10000ms
Error: element(s) not found

Call log:
  - Expect "toHaveValue" with timeout 10000ms
  - waiting for getByTestId('material-engineering-table').getByRole('textbox', { name: 'material:UIF-INVENTED-01 Shear [Pa]' })

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
  - paragraph: Generated UI local-render-origin precision probe · Edited
  - group "Editing tools":
    - button "Undo model edit"
    - button "Redo model edit" [disabled]
    - button "Select" [pressed]
  - group "View":
    - button "Table" [pressed]
    - button "Model"
    - button "Both"
  - button "Run"
  - button "Issues, 2": Issues 2
  - group "Panels":
    - button "Inspector" [disabled]
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
        - button "Issues, 2": Issues
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
      - text: 49 of 49 model entities visible
      - button "Clear model tree filter" [disabled]
    - region "Bulk entity grid":
      - button "Nodes"
      - button "Pipes"
      - button "Supports"
      - button "Materials" [pressed]
      - button "Sections"
      - button "Components"
      - button "Load Cases"
      - button "Combinations"
      - grid "Material fields":
        - row "Material Sort Label Sort Elastic Sort Shear Sort Thermal Sort Provenance":
          - columnheader "Material"
          - columnheader "Sort Label":
            - button "Sort Label": Label
          - columnheader "Sort Elastic":
            - button "Sort Elastic": Elastic [per-row entered unit]
          - columnheader "Sort Shear":
            - button "Sort Shear": Shear [per-row entered unit]
          - columnheader "Sort Thermal":
            - button "Sort Thermal": Thermal [per-row entered unit]
          - columnheader "Sort Provenance":
            - button "Sort Provenance": Provenance
        - rowgroup:
          - 'row "material:UIF-INVENTED-01 material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material material:UIF-INVENTED-01 Elastic: 210000 MPa MPa Quantity readout material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [selected]':
            - rowheader "material:UIF-INVENTED-01":
              - button "material:UIF-INVENTED-01"
            - 'gridcell "material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material"':
              - 'button "material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material"': Invented UI benchmark elastic material
            - 'gridcell "material:UIF-INVENTED-01 Elastic: 210000 MPa MPa Quantity readout"':
              - 'button "material:UIF-INVENTED-01 Elastic: 210000 MPa"': "210000"
              - text: MPa 210000 MPa
            - 'gridcell "material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout" [selected]':
              - 'button "material:UIF-INVENTED-01 Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
          - 'row "material:B4-P2-second material:B4-P2-second Label: Invented UI benchmark elastic material material:B4-P2-second Elastic: 100000000000 Pa Pa Quantity readout material:B4-P2-second Shear: 77000000000 Pa Pa Quantity readout material:B4-P2-second Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-P2-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
            - rowheader "material:B4-P2-second":
              - button "material:B4-P2-second"
            - 'gridcell "material:B4-P2-second Label: Invented UI benchmark elastic material"':
              - 'button "material:B4-P2-second Label: Invented UI benchmark elastic material"': Invented UI benchmark elastic material
            - 'gridcell "material:B4-P2-second Elastic: 100000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-P2-second Elastic: 100000000000 Pa"': "100000000000"
              - text: Pa 100000000000 Pa
            - 'gridcell "material:B4-P2-second Shear: 77000000000 Pa Pa Quantity readout"':
              - 'button "material:B4-P2-second Shear: 77000000000 Pa"': "77000000000"
              - text: Pa 77000000000 Pa
            - 'gridcell "material:B4-P2-second Thermal: 0.000012 1/degC 1/degC Quantity readout"':
              - 'button "material:B4-P2-second Thermal: 0.000012 1/degC"': "0.000012"
              - text: 1/degC 0.000012 1/degC
            - 'gridcell "material:B4-P2-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"':
              - 'button "material:B4-P2-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data"': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
      - group "Material fields footer":
        - text: 2 of 2 rows
        - status: Quantity sort unavailable; showing input order while values or units cannot be converted.
        - button "Requested sort by Elastic · Clear"
      - status: Cell applied.
      - button "Review multiple changes"
  - complementary "Agent":
    - button "Agent" [disabled]
  - button "Solver · Not solved"
  - button "2 Issues"
  - text: "material: material:UIF-INVENTED-01 Entered"
  - button "About SWBPIPE…"
```

# Test source

```ts
  440 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  441 |   await expect(nodeEditor).toBeFocused(); await expect(nodeEditor).toHaveValue(" "); await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click();
  442 |   await page.getByTestId("entity-grid-type-materials").click(); await reviewCell.dblclick(); await review.getByRole("textbox").fill("80");
  443 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  444 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  445 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(elastic).toHaveText("50"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  446 |   if (await page.getByTestId("material-grid-review-disclosure").getAttribute("aria-expanded") === "true") await page.getByTestId("material-grid-review-disclosure").click();
  447 |   await direct.getByTestId(`table-cell-${firstId}-label`).dblclick(); const newEditor = direct.getByRole("textbox"); await newEditor.press("R"); await newEditor.press("S");
  448 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  449 |   await expect(newEditor).toHaveValue("RS"); await expect(newEditor).toBeFocused(); await direct.getByRole("button", { name: "Cancel", exact: true }).click();
  450 |   await page.screenshot({ path: info.outputPath("b4-materials-reopened.png") });
  451 | });
  452 | 
  453 | test("B4 Materials stable editor owns character starts, virtual scrolling, filtering and resize", async ({ page, browser }, info) => {
  454 |   await attachBrowserIdentity(browser, info);
  455 |   const { model } = await readFixture("precision-origin-base.model.json"); const material = model.materials[0];
  456 |   model.materials = Array.from({ length: 140 }, (_, index) => ({ ...material, id: index === 0 ? material.id : `material:B4-${index}`, label: `Synthetic material ${index}` }));
  457 |   await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  458 |   const table = page.getByTestId("material-engineering-table"); const cell = table.getByTestId(`table-cell-${material.id}-elastic`);
  459 |   await expect(cell.locator("..")).toHaveAttribute("aria-readonly", "false");
  460 |   await cell.focus(); await page.keyboard.press("x"); const editor = table.getByRole("textbox"); await expect(editor).toBeFocused();
  461 |   await page.keyboard.press("y"); await expect(editor).toHaveValue("xy"); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([2, 2]);
  462 |   await page.keyboard.press("ArrowLeft"); const retained = await editor.elementHandle();
  463 |   const originalViewport = page.viewportSize()!; await page.setViewportSize({ ...originalViewport, height: originalViewport.height + 80 });
  464 |   await expect(editor).toBeFocused(); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([1, 1]);
  465 |   const aligned = async () => editor.evaluate((input) => {
  466 |     const anchor = input.closest(".engineering-table")!.querySelector("[data-editor-anchor]")!; const a = anchor.getBoundingClientRect(), e = input.getBoundingClientRect();
  467 |     return { dx: e.x - a.x, dy: e.y - a.y, dw: e.width - a.width, dh: e.height - a.height };
  468 |   });
  469 |   await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  470 |   await openWorkspaceSection(page, "libraries");
  471 |   await page.setViewportSize({ ...originalViewport, height: originalViewport.height + 120 });
  472 |   await page.getByTestId("workspace-dock-close").click();
  473 |   const pageReturn = await table.evaluate((root) => {
  474 |     const input = root.querySelector<HTMLInputElement>("input")!; const anchor = root.querySelector("[data-editor-anchor]")!;
  475 |     return { inputVisibility: getComputedStyle(input).visibility, value: input.value, owner: document.activeElement?.outerHTML.slice(0, 300),
  476 |       input: input.getBoundingClientRect().toJSON(), anchor: anchor.getBoundingClientRect().toJSON(), ancestorInert: Boolean(root.closest("[inert]")) };
  477 |   });
  478 |   await info.attach("material-page-inert-return", { body: JSON.stringify(pageReturn, null, 2), contentType: "application/json" });
  479 |   await expect(editor).toBeVisible(); await expect(editor).toHaveValue("xy");
  480 |   await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  481 |   expect(await editor.evaluate((node, original) => node === original, retained)).toBe(true);
  482 |   await expect(editor).not.toBeFocused(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  483 |   const rows = page.getByTestId("material-engineering-table-rows"); await hoverTableBody(page, rows); await page.mouse.wheel(0, 2200);
  484 |   await expect.poll(() => rows.evaluate((element) => element.scrollTop)).toBeGreaterThan(1000);
  485 |   expect(await retained!.evaluate((input) => input.isConnected)).toBe(true);
  486 |   await expect(editor).toHaveValue("xy");
  487 |   // The clipped editor cannot intercept header/footer pointer controls.
  488 |   const hits = await table.evaluate((root) => [".engineering-table-header", ".engineering-table-footer"].map((selector) => {
  489 |     const element = root.querySelector(selector)!; const r = element.getBoundingClientRect(); const hit = document.elementFromPoint(r.left + 12, r.top + r.height / 2);
  490 |     return Boolean(hit && element.contains(hit));
  491 |   })); expect(hits).toEqual([true, true]);
  492 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("material:B4-139"); await expect(rows.locator('[role="row"]')).toHaveCount(2);
  493 |   expect(await editor.evaluate((node, original) => node === original, retained)).toBe(true); await expect(editor).toHaveValue("xy");
  494 |   await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  495 |   await page.getByTestId("entity-grid-type-nodes").click(); await expect(editor).toBeHidden(); await page.getByTestId("entity-grid-type-materials").click(); await expect(editor).toHaveValue("xy");
  496 |   await expect(page.getByTestId("entity-grid-type-materials")).toBeFocused();
  497 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(editor).toHaveCount(0); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  498 |   await filter.fill(""); await cell.focus(); await page.keyboard.press("Enter"); await expect(editor).toBeFocused();
  499 |   expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, String(material.elastic_modulus.value).length]);
  500 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await cell.dblclick(); await expect(editor).toBeFocused();
  501 |   expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, String(material.elastic_modulus.value).length]);
  502 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await page.setViewportSize(originalViewport);
  503 |   await info.attach("material-editor-clipping", { body: JSON.stringify({ headerFooterHitOwnership: hits }), contentType: "application/json" });
  504 | });
  505 | 
  506 | test("B4 Materials Apply Tab keeps the first Shear character while another quantity reconverts", async ({ page, browser }, info) => {
  507 |   await attachBrowserIdentity(browser, info);
  508 |   const { model } = await readFixture("precision-origin-base.model.json"); const material = model.materials[0]; const firstId = material.id; const secondId = "material:B4-P2-second";
  509 |   model.materials = [{ ...material, elastic_modulus: { value: 200000, unit: "MPa" } },
  510 |     { ...material, id: secondId, elastic_modulus: { value: 100000000000, unit: "Pa" } }];
  511 |   await page.route("**/src/services/displayQuantityService.ts", async (route) => {
  512 |     const response = await route.fetch(); const source = await response.text(); expect(source).toContain("export async function convertDisplayQuantities(");
  513 |     await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
  514 | export async function convertDisplayQuantities(items) {
  515 |   const result = await originalConvertDisplayQuantities(items);
  516 |   const gate = window.__b4MaterialP2Gate;
  517 |   if (gate.hold && items.some(item => item.id.includes('material') && item.id.includes('elastic'))) await new Promise(resolve => gate.pending.push(resolve));
  518 |   return result;
  519 | }
  520 | ` });
  521 |   });
  522 |   await page.addInitScript(() => { (window as any).__b4MaterialP2Gate = { hold: false, pending: [] }; });
  523 |   await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  524 |   const table = page.getByTestId("material-engineering-table"); const elastic = table.getByTestId(`table-cell-${firstId}-elastic`); const shear = table.getByTestId(`table-cell-${firstId}-shear`);
  525 |   await expect(elastic.locator("..")).toHaveAttribute("aria-readonly", "false"); await expect(shear.locator("..")).toHaveAttribute("aria-readonly", "false");
  526 |   await table.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await expect(table.getByRole("rowheader").first()).toHaveText(secondId);
  527 |   await elastic.dblclick(); await table.getByRole("textbox").fill("210000");
  528 |   await page.evaluate(() => { (window as any).__b4MaterialP2Gate.hold = true; });
  529 |   await page.keyboard.press("Tab"); await expect(elastic).toHaveText("210000"); await expect(shear).toBeFocused();
  530 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialP2Gate.pending.length)).toBeGreaterThan(0);
  531 |   await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toContainText("sort unavailable"); await expect(table.getByRole("rowheader").first()).toHaveText(firstId);
  532 |   await expect(page.getByTestId("workspace-undo")).toBeEnabled();
  533 |   // No locator focus/fill repair: this is the first actual key after Apply+Tab.
  534 |   await page.keyboard.press("8");
  535 |   const firstKey = await table.evaluate((root) => ({ editorValue: root.querySelector<HTMLInputElement>("input")?.value ?? null,
  536 |     activeTag: document.activeElement?.tagName, activeColumn: (document.activeElement as HTMLElement)?.dataset.columnKey,
  537 |     shearReadonly: root.querySelector('[data-column-key="shear"]')?.parentElement?.getAttribute("aria-readonly") }));
  538 |   await info.attach("material-p2-first-key", { body: JSON.stringify(firstKey), contentType: "application/json" });
  539 |   const input = table.getByRole("textbox", { name: `${firstId} Shear [${material.shear_modulus.unit}]` });
> 540 |   await expect(input).toHaveValue("8"); await expect(input).toBeFocused();
      |                       ^ Error: expect(locator).toHaveValue(expected) failed
  541 |   await page.keyboard.press("7"); await expect(input).toHaveValue("87");
  542 |   expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([2, 2]);
  543 |   await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toContainText("sort unavailable");
  544 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(shear).toHaveText(String(material.shear_modulus.value));
  545 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialP2Gate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  546 |   await expect(table.getByRole("rowheader").first()).toHaveText(secondId);
  547 |   // Exactly one accepted model operation; cancelled Shear entry adds no history.
  548 |   await page.getByTestId("workspace-undo").click(); await expect(elastic).toHaveText("200000"); await expect(shear).toHaveText(String(material.shear_modulus.value)); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  549 | });
  550 | 
```