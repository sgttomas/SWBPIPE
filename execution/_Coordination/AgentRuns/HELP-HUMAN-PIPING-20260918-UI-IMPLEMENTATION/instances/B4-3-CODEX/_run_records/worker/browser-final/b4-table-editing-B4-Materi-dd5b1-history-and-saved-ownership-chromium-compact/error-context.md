# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 Materials preserve mixed-unit editing, delayed review sort, history and saved ownership
- Location: e2e/b4-table-editing.spec.ts:362:1

# Error details

```
Test timeout of 120000ms exceeded.
```

```
Error: locator.dblclick: Test timeout of 120000ms exceeded.
Call log:
  - waiting for getByTestId('material-engineering-table').getByTestId('table-cell-material:UIF-INVENTED-01-label')
    - locator resolved to <button tabindex="0" type="button" data-table-cell="true" data-column-key="label" data-row-key="["material","material:UIF-INVENTED-01"]" data-testid="table-cell-material:UIF-INVENTED-01-label" aria-label="material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material">Invented UI benchmark elastic material</button>
  - attempting dblclick action
    2 × waiting for element to be visible, enabled and stable
      - element is not visible
    - retrying dblclick action
    - waiting 20ms
    2 × waiting for element to be visible, enabled and stable
      - element is not visible
    - retrying dblclick action
      - waiting 100ms
    225 × waiting for element to be visible, enabled and stable
        - element is not visible
      - retrying dblclick action
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
      - button "Select (⎋)" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
    - group "View" [ref=e31]:
      - button "Table" [pressed] [ref=e33] [cursor=pointer]:
        - img [ref=e34]
        - generic [ref=e36]: Table
      - button "Model" [ref=e38] [cursor=pointer]:
        - img [ref=e39]
        - generic [ref=e42]: Model
      - button "Both" [ref=e44] [cursor=pointer]:
        - img [ref=e45]
        - generic [ref=e47]: Both
    - generic [ref=e48]:
      - button "Run" [ref=e49] [cursor=pointer]:
        - img [ref=e50]
        - generic [ref=e52]: Run
      - button "Issues, 2" [ref=e53] [cursor=pointer]:
        - img [ref=e54]
        - generic [ref=e56]: Issues
        - generic [ref=e57]: "2"
    - group "Panels" [ref=e58]:
      - button "Inspector" [disabled] [ref=e60]:
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
          - button "Issues, 2" [ref=e125] [cursor=pointer]:
            - img [ref=e126]
            - generic [ref=e128]: Issues
            - generic [ref=e129]: "2"
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
            - generic [ref=e160]: 49 of 49 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e161]:
              - img [ref=e162]
          - region "Bulk entity grid" [ref=e166]:
            - generic "Grid entity type" [ref=e167]:
              - button "Nodes" [ref=e168]
              - button "Pipes" [ref=e169]
              - button "Supports" [ref=e170]
              - button "Materials" [active] [pressed] [ref=e171]
              - button "Sections" [ref=e172]
              - button "Components" [ref=e173]
              - button "Load Cases" [ref=e174]
              - button "Combinations" [ref=e175]
            - generic [ref=e177]:
              - generic [ref=e178]:
                - generic [ref=e179]: 2 of 2 Materials
                - generic [ref=e180]: 0 changed cells
              - generic [ref=e182]:
                - grid "Material review drafts" [ref=e183]:
                  - row "Material Sort Label Sort Elastic Sort Shear Sort Thermal Sort Provenance" [ref=e184]:
                    - columnheader "Material" [ref=e185]
                    - columnheader "Sort Label" [ref=e186]:
                      - button "Sort Label" [ref=e187]: Label ↕
                    - columnheader "Sort Elastic" [ref=e188]:
                      - button "Sort Elastic" [ref=e189]: Elastic [per-row entered unit] ↓
                    - columnheader "Sort Shear" [ref=e190]:
                      - button "Sort Shear" [ref=e191]: Shear [per-row entered unit] ↕
                    - columnheader "Sort Thermal" [ref=e192]:
                      - button "Sort Thermal" [ref=e193]: Thermal [per-row entered unit] ↕
                    - columnheader "Sort Provenance" [ref=e194]:
                      - button "Sort Provenance" [ref=e195]: Provenance ↕
                  - rowgroup [ref=e197]:
                    - generic [ref=e198]:
                      - 'row "material:UIF-INVENTED-01 material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material material:UIF-INVENTED-01 Elastic: 50 MPa MPa Quantity readout material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e200]':
                        - rowheader "material:UIF-INVENTED-01" [ref=e201]:
                          - button "material:UIF-INVENTED-01" [ref=e202]
                        - 'gridcell "material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material" [ref=e203]':
                          - 'button "material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material" [ref=e204]': Invented UI benchmark elastic material
                        - 'gridcell "material:UIF-INVENTED-01 Elastic: 50 MPa MPa Quantity readout" [ref=e205]':
                          - 'button "material:UIF-INVENTED-01 Elastic: 50 MPa" [ref=e206]': "50"
                          - generic "MPa" [ref=e207]
                          - generic "Quantity readout" [ref=e208]:
                            - generic [ref=e209]: 50 MPa
                        - 'gridcell "material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout" [ref=e210]':
                          - 'button "material:UIF-INVENTED-01 Shear: 77000000000 Pa" [ref=e211]': "77000000000"
                          - generic "Pa" [ref=e212]
                          - generic "Quantity readout" [ref=e213]:
                            - generic [ref=e214]: 77000000000 Pa
                        - 'gridcell "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e215]':
                          - 'button "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC" [ref=e216]': "0.000012"
                          - generic "1/degC" [ref=e217]
                          - generic "Quantity readout" [ref=e218]:
                            - generic [ref=e219]: 0.000012 1/degC
                        - 'gridcell "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e220]':
                          - 'button "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e221]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                      - 'row "material:B4-second material:B4-second Label: Synthetic second material material:B4-second Elastic: 100000000000 Pa Pa Quantity readout material:B4-second Shear: 77000000000 Pa Pa Quantity readout material:B4-second Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e223]':
                        - rowheader "material:B4-second" [ref=e224]:
                          - button "material:B4-second" [ref=e225]
                        - 'gridcell "material:B4-second Label: Synthetic second material" [ref=e226]':
                          - 'button "material:B4-second Label: Synthetic second material" [ref=e227]': Synthetic second material
                        - 'gridcell "material:B4-second Elastic: 100000000000 Pa Pa Quantity readout" [ref=e228]':
                          - 'button "material:B4-second Elastic: 100000000000 Pa" [ref=e229]': "100000000000"
                          - generic "Pa" [ref=e230]
                          - generic "Quantity readout" [ref=e231]:
                            - generic [ref=e232]: 100000000000 Pa
                        - 'gridcell "material:B4-second Shear: 77000000000 Pa Pa Quantity readout" [ref=e233]':
                          - 'button "material:B4-second Shear: 77000000000 Pa" [ref=e234]': "77000000000"
                          - generic "Pa" [ref=e235]
                          - generic "Quantity readout" [ref=e236]:
                            - generic [ref=e237]: 77000000000 Pa
                        - 'gridcell "material:B4-second Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e238]':
                          - 'button "material:B4-second Thermal: 0.000012 1/degC" [ref=e239]': "0.000012"
                          - generic "1/degC" [ref=e240]
                          - generic "Quantity readout" [ref=e241]:
                            - generic [ref=e242]: 0.000012 1/degC
                        - 'gridcell "material:B4-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e243]':
                          - 'button "material:B4-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e244]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                - group "Material review drafts footer" [ref=e245]:
                  - generic [ref=e246]: 2 of 2 rows
                  - status [ref=e247]: Quantity sort unavailable; showing input order while values or units cannot be converted.
                  - button "Requested sort by Elastic · Clear" [ref=e248]
              - generic [ref=e249]:
                - button "Queue changed cells" [disabled] [ref=e250]:
                  - img [ref=e251]
                  - text: Queue changed cells
                - button "Clear grid edits" [disabled] [ref=e253]:
                  - img [ref=e254]
                  - text: Clear grid edits
              - paragraph [ref=e257]: Grid mode fans each changed cell into a structured review intent; storage remains local. Blank or whitespace text becomes TBD when queued; keeping a draft does not change the model.
            - button "Return to material fields" [expanded] [ref=e258] [cursor=pointer]
      - option "All" [selected]
      - option "Pipes"
      - option "Nodes"
      - option "Supports"
      - option "Components"
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e259]:
      - button "Agent" [disabled] [ref=e261]:
        - img [ref=e262]
        - generic [ref=e265]: Agent
  - generic "Workspace status" [ref=e266]:
    - generic "Analysis statuses" [ref=e267]:
      - button "Solver · Not solved" [ref=e269] [cursor=pointer]
    - button "2 Issues" [ref=e270] [cursor=pointer]:
      - img [ref=e271]
      - text: 2 Issues
    - generic "Selection" [ref=e273]: "project: project:UIF-PRECISION-ORIGIN"
    - generic "Display units" [ref=e274]: Entered
    - button "About SWBPIPE…" [ref=e275] [cursor=pointer]:
      - img [ref=e276]
```

# Test source

```ts
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
  415 |   await input.press("3"); await expect(input).toHaveValue("530"); await input.press("ControlOrMeta+z"); await expect(input).toHaveValue("50");
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
  439 |   const nodeLabel = page.getByTestId(`table-cell-${model.nodes[0].id}-label`); await nodeLabel.dblclick(); const nodeEditor = page.getByTestId("engineering-table").getByRole("textbox"); await nodeEditor.fill(" ");
  440 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  441 |   await expect(nodeEditor).toBeFocused(); await expect(nodeEditor).toHaveValue(" "); await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click();
  442 |   await page.getByTestId("entity-grid-type-materials").click(); await reviewCell.dblclick(); await review.getByRole("textbox").fill("80");
  443 |   await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  444 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  445 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(elastic).toHaveText("50"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
> 446 |   await direct.getByTestId(`table-cell-${firstId}-label`).dblclick(); const newEditor = direct.getByRole("textbox"); await newEditor.press("R"); await newEditor.press("S");
      |                                                           ^ Error: locator.dblclick: Test timeout of 120000ms exceeded.
  447 |   await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  448 |   await expect(newEditor).toHaveValue("RS"); await expect(newEditor).toBeFocused(); await direct.getByRole("button", { name: "Cancel", exact: true }).click();
  449 |   await page.screenshot({ path: info.outputPath("b4-materials-reopened.png") });
  450 | });
  451 | 
  452 | test("B4 Materials stable editor owns character starts, virtual scrolling, filtering and resize", async ({ page, browser }, info) => {
  453 |   await attachBrowserIdentity(browser, info);
  454 |   const { model } = await readFixture("precision-origin-base.model.json"); const material = model.materials[0];
  455 |   model.materials = Array.from({ length: 140 }, (_, index) => ({ ...material, id: index === 0 ? material.id : `material:B4-${index}`, label: `Synthetic material ${index}` }));
  456 |   await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  457 |   const table = page.getByTestId("material-engineering-table"); const cell = table.getByTestId(`table-cell-${material.id}-elastic`);
  458 |   await expect(cell.locator("..")).toHaveAttribute("aria-readonly", "false");
  459 |   await cell.focus(); await page.keyboard.press("x"); const editor = table.getByRole("textbox"); await expect(editor).toBeFocused();
  460 |   await page.keyboard.press("y"); await expect(editor).toHaveValue("xy"); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([2, 2]);
  461 |   await page.keyboard.press("ArrowLeft"); const retained = await editor.elementHandle();
  462 |   const originalViewport = page.viewportSize()!; await page.setViewportSize({ ...originalViewport, height: originalViewport.height + 80 });
  463 |   await expect(editor).toBeFocused(); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([1, 1]);
  464 |   const aligned = async () => editor.evaluate((input) => {
  465 |     const anchor = input.closest(".engineering-table")!.querySelector("[data-editor-anchor]")!; const a = anchor.getBoundingClientRect(), e = input.getBoundingClientRect();
  466 |     return { dx: e.x - a.x, dy: e.y - a.y, dw: e.width - a.width, dh: e.height - a.height };
  467 |   });
  468 |   await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  469 |   const rows = page.getByTestId("material-engineering-table-rows"); await hoverTableBody(page, rows); await page.mouse.wheel(0, 2200);
  470 |   await expect.poll(() => rows.evaluate((element) => element.scrollTop)).toBeGreaterThan(1000);
  471 |   expect(await retained!.evaluate((input) => input.isConnected)).toBe(true);
  472 |   await expect(editor).toHaveValue("xy");
  473 |   // The clipped editor cannot intercept header/footer pointer controls.
  474 |   const hits = await table.evaluate((root) => [".engineering-table-header", ".engineering-table-footer"].map((selector) => {
  475 |     const element = root.querySelector(selector)!; const r = element.getBoundingClientRect(); const hit = document.elementFromPoint(r.left + 12, r.top + r.height / 2);
  476 |     return Boolean(hit && element.contains(hit));
  477 |   })); expect(hits).toEqual([true, true]);
  478 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("material:B4-139"); await expect(rows.locator('[role="row"]')).toHaveCount(2);
  479 |   expect(await editor.evaluate((node, original) => node === original, retained)).toBe(true); await expect(editor).toHaveValue("xy");
  480 |   await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  481 |   await page.getByTestId("entity-grid-type-nodes").click(); await expect(editor).toBeHidden(); await page.getByTestId("entity-grid-type-materials").click(); await expect(editor).toHaveValue("xy");
  482 |   await expect(page.getByTestId("entity-grid-type-materials")).toBeFocused();
  483 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(editor).toHaveCount(0); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  484 |   await filter.fill(""); await cell.focus(); await page.keyboard.press("Enter"); await expect(editor).toBeFocused();
  485 |   expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, String(material.elastic_modulus.value).length]);
  486 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await cell.dblclick(); await expect(editor).toBeFocused();
  487 |   expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, String(material.elastic_modulus.value).length]);
  488 |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await page.setViewportSize(originalViewport);
  489 |   await info.attach("material-editor-clipping", { body: JSON.stringify({ headerFooterHitOwnership: hits }), contentType: "application/json" });
  490 | });
  491 | 
```