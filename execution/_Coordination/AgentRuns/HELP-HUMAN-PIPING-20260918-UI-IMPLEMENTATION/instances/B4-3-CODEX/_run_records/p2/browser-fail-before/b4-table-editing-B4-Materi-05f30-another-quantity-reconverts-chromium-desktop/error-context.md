# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 Materials Apply Tab keeps the first Shear character while another quantity reconverts
- Location: e2e/b4-table-editing.spec.ts:506:1

# Error details

```
Error: expect(locator).toContainText(expected) failed

Locator: getByTestId('material-engineering-table').getByRole('status')
Expected substring: "sort unavailable"
Error: strict mode violation: getByTestId('material-engineering-table').getByRole('status') resolved to 2 elements:
    1) <span role="status">Quantity sort unavailable; showing input order wh…</span> aka getByText('Quantity sort unavailable;')
    2) <p role="status" class="engineering-table-message">Cell applied.</p> aka getByText('Cell applied.')

Call log:
  - Expect "toContainText" with timeout 10000ms
  - waiting for getByTestId('material-engineering-table').getByRole('status')

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
      - paragraph [ref=e18]: Generated UI local-render-origin precision probe · Edited
    - group "Editing tools" [ref=e19]:
      - button "Undo model edit" [ref=e20] [cursor=pointer]:
        - img [ref=e21]
      - button "Redo model edit" [disabled] [ref=e24]:
        - img [ref=e25]
      - button "Select" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
        - generic [ref=e31]: Select
    - group "View" [ref=e32]:
      - button "Table" [pressed] [ref=e34] [cursor=pointer]:
        - img [ref=e35]
        - generic [ref=e37]: Table
      - button "Model" [ref=e39] [cursor=pointer]:
        - img [ref=e40]
        - generic [ref=e43]: Model
      - button "Both" [ref=e45] [cursor=pointer]:
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
      - button "Inspector" [disabled] [ref=e61]:
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
            - button "Tree" [ref=e149]:
              - img [ref=e150]
              - text: Tree
            - button "Grid" [pressed] [ref=e153]:
              - img [ref=e154]
              - text: Grid
          - region "Model tree filtering" [ref=e156]:
            - generic [ref=e157]:
              - img [ref=e158]
              - generic [ref=e161]: Filter model
              - searchbox "Filter model tree" [ref=e162]
            - generic [ref=e163]: 49 of 49 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e164]:
              - img [ref=e165]
          - region "Bulk entity grid" [ref=e169]:
            - generic "Grid entity type" [ref=e170]:
              - button "Nodes" [ref=e171]
              - button "Pipes" [ref=e172]
              - button "Supports" [ref=e173]
              - button "Materials" [pressed] [ref=e174]
              - button "Sections" [ref=e175]
              - button "Components" [ref=e176]
              - button "Load Cases" [ref=e177]
              - button "Combinations" [ref=e178]
            - generic [ref=e180]:
              - grid "Material fields" [ref=e181]:
                - row "Material Sort Label Sort Elastic Sort Shear Sort Thermal Sort Provenance" [ref=e182]:
                  - columnheader "Material" [ref=e183]
                  - columnheader "Sort Label" [ref=e184]:
                    - button "Sort Label" [ref=e185]: Label ↕
                  - columnheader "Sort Elastic" [ref=e186]:
                    - button "Sort Elastic" [ref=e187]: Elastic [per-row entered unit] ↑
                  - columnheader "Sort Shear" [ref=e188]:
                    - button "Sort Shear" [ref=e189]: Shear [per-row entered unit] ↕
                  - columnheader "Sort Thermal" [ref=e190]:
                    - button "Sort Thermal" [ref=e191]: Thermal [per-row entered unit] ↕
                  - columnheader "Sort Provenance" [ref=e192]:
                    - button "Sort Provenance" [ref=e193]: Provenance ↕
                - rowgroup [ref=e195]:
                  - generic [ref=e196]:
                    - 'row "material:UIF-INVENTED-01 material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material material:UIF-INVENTED-01 Elastic: 210000 MPa MPa Quantity readout material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [selected] [ref=e198]':
                      - rowheader "material:UIF-INVENTED-01" [ref=e199]:
                        - button "material:UIF-INVENTED-01" [ref=e200]
                      - 'gridcell "material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material" [ref=e201]':
                        - 'button "material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material" [ref=e202]': Invented UI benchmark elastic material
                      - 'gridcell "material:UIF-INVENTED-01 Elastic: 210000 MPa MPa Quantity readout" [ref=e203]':
                        - 'button "material:UIF-INVENTED-01 Elastic: 210000 MPa" [ref=e204]': "210000"
                        - generic "MPa" [ref=e205]
                        - generic "Quantity readout" [ref=e206]:
                          - generic [ref=e207]: 210000 MPa
                      - 'gridcell "material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout" [selected] [ref=e208]':
                        - 'button "material:UIF-INVENTED-01 Shear: 77000000000 Pa" [active] [ref=e209]': "77000000000"
                        - generic "Pa" [ref=e210]
                        - generic "Quantity readout" [ref=e211]:
                          - generic [ref=e212]: 77000000000 Pa
                      - 'gridcell "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e213]':
                        - 'button "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC" [ref=e214]': "0.000012"
                        - generic "1/degC" [ref=e215]
                        - generic "Quantity readout" [ref=e216]:
                          - generic [ref=e217]: 0.000012 1/degC
                      - 'gridcell "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e218]':
                        - 'button "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e219]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                    - 'row "material:B4-P2-second material:B4-P2-second Label: Invented UI benchmark elastic material material:B4-P2-second Elastic: 100000000000 Pa Pa Quantity readout material:B4-P2-second Shear: 77000000000 Pa Pa Quantity readout material:B4-P2-second Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-P2-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e221]':
                      - rowheader "material:B4-P2-second" [ref=e222]:
                        - button "material:B4-P2-second" [ref=e223]
                      - 'gridcell "material:B4-P2-second Label: Invented UI benchmark elastic material" [ref=e224]':
                        - 'button "material:B4-P2-second Label: Invented UI benchmark elastic material" [ref=e225]': Invented UI benchmark elastic material
                      - 'gridcell "material:B4-P2-second Elastic: 100000000000 Pa Pa Quantity readout" [ref=e226]':
                        - 'button "material:B4-P2-second Elastic: 100000000000 Pa" [ref=e227]': "100000000000"
                        - generic "Pa" [ref=e228]
                        - generic "Quantity readout" [ref=e229]:
                          - generic [ref=e230]: 100000000000 Pa
                      - 'gridcell "material:B4-P2-second Shear: 77000000000 Pa Pa Quantity readout" [ref=e231]':
                        - 'button "material:B4-P2-second Shear: 77000000000 Pa" [ref=e232]': "77000000000"
                        - generic "Pa" [ref=e233]
                        - generic "Quantity readout" [ref=e234]:
                          - generic [ref=e235]: 77000000000 Pa
                      - 'gridcell "material:B4-P2-second Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e236]':
                        - 'button "material:B4-P2-second Thermal: 0.000012 1/degC" [ref=e237]': "0.000012"
                        - generic "1/degC" [ref=e238]
                        - generic "Quantity readout" [ref=e239]:
                          - generic [ref=e240]: 0.000012 1/degC
                      - 'gridcell "material:B4-P2-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e241]':
                        - 'button "material:B4-P2-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e242]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
              - group "Material fields footer" [ref=e243]:
                - generic [ref=e244]: 2 of 2 rows
                - status [ref=e245]: Quantity sort unavailable; showing input order while values or units cannot be converted.
                - button "Requested sort by Elastic · Clear" [ref=e246]
              - status [ref=e247]: Cell applied.
            - button "Review multiple changes" [ref=e248] [cursor=pointer]
      - option "All" [selected]
      - option "Pipes"
      - option "Nodes"
      - option "Supports"
      - option "Components"
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e249]:
      - button "Agent" [disabled] [ref=e251]:
        - img [ref=e252]
        - generic [ref=e255]: Agent
  - generic "Workspace status" [ref=e256]:
    - generic "Analysis statuses" [ref=e257]:
      - button "Solver · Not solved" [ref=e259] [cursor=pointer]
    - button "2 Issues" [ref=e260] [cursor=pointer]:
      - img [ref=e261]
      - text: 2 Issues
    - generic "Selection" [ref=e263]: "material: material:UIF-INVENTED-01"
    - generic "Display units" [ref=e264]: Entered
    - button "About SWBPIPE…" [ref=e265] [cursor=pointer]:
      - img [ref=e266]
```

# Test source

```ts
  431 |   for (const unit of geometry.units) { expect(unit.width).toBeGreaterThan(0); expect(unit.scroll).toBeLessThanOrEqual(unit.client); }
  432 |   await info.attach("materials-contained-units", { body: JSON.stringify(geometry, null, 2), contentType: "application/json" });
  433 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  434 |   await showModelTree(page); await page.getByTestId("material-grid-review-disclosure").click();
  435 |   await page.evaluate(() => { (window as any).__b4MaterialConversionGate.hold = true; });
  436 |   await reviewCell.dblclick(); await review.getByRole("textbox").fill("70");
  437 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  438 |   await page.getByTestId("entity-grid-type-nodes").click();
  439 |   const nodeLabel = page.getByTestId(`table-cell-${model.nodes[0].id}-label`); await nodeLabel.dblclick(); const nodeEditor = page.getByTestId("engineering-table").getByRole("textbox"); await nodeEditor.fill(" ");
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
> 531 |   await expect(table.getByRole("status")).toContainText("sort unavailable"); await expect(table.getByRole("rowheader").first()).toHaveText(firstId);
      |                                           ^ Error: expect(locator).toContainText(expected) failed
  532 |   await expect(page.getByTestId("workspace-undo")).toBeEnabled();
  533 |   // No locator focus/fill repair: this is the first actual key after Apply+Tab.
  534 |   await page.keyboard.press("8");
  535 |   const firstKey = await table.evaluate((root) => ({ editorValue: root.querySelector<HTMLInputElement>("input")?.value ?? null,
  536 |     activeTag: document.activeElement?.tagName, activeColumn: (document.activeElement as HTMLElement)?.dataset.columnKey,
  537 |     shearReadonly: root.querySelector('[data-column-key="shear"]')?.parentElement?.getAttribute("aria-readonly") }));
  538 |   await info.attach("material-p2-first-key", { body: JSON.stringify(firstKey), contentType: "application/json" });
  539 |   const input = table.getByRole("textbox", { name: `${firstId} Shear [${material.shear_modulus.unit}]` });
  540 |   await expect(input).toHaveValue("8"); await expect(input).toBeFocused();
  541 |   await page.keyboard.press("7"); await expect(input).toHaveValue("87");
  542 |   expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([2, 2]);
  543 |   await expect(table.getByRole("status")).toContainText("sort unavailable");
  544 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(shear).toHaveText(String(material.shear_modulus.value));
  545 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialP2Gate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  546 |   await expect(table.getByRole("rowheader").first()).toHaveText(secondId);
  547 |   // Exactly one accepted model operation; cancelled Shear entry adds no history.
  548 |   await page.getByTestId("workspace-undo").click(); await expect(elastic).toHaveText("200000"); await expect(shear).toHaveText(String(material.shear_modulus.value)); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  549 | });
  550 | 
```