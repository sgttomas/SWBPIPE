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
              - button "Materials" [active] [pressed] [ref=e174]
              - button "Sections" [ref=e175]
              - button "Components" [ref=e176]
              - button "Load Cases" [ref=e177]
              - button "Combinations" [ref=e178]
            - generic [ref=e180]:
              - generic [ref=e181]:
                - generic [ref=e182]: 2 of 2 Materials
                - generic [ref=e183]: 0 changed cells
              - generic [ref=e185]:
                - grid "Material review drafts" [ref=e186]:
                  - row "Material Sort Label Sort Elastic Sort Shear Sort Thermal Sort Provenance" [ref=e187]:
                    - columnheader "Material" [ref=e188]
                    - columnheader "Sort Label" [ref=e189]:
                      - button "Sort Label" [ref=e190]: Label ↕
                    - columnheader "Sort Elastic" [ref=e191]:
                      - button "Sort Elastic" [ref=e192]: Elastic [per-row entered unit] ↓
                    - columnheader "Sort Shear" [ref=e193]:
                      - button "Sort Shear" [ref=e194]: Shear [per-row entered unit] ↕
                    - columnheader "Sort Thermal" [ref=e195]:
                      - button "Sort Thermal" [ref=e196]: Thermal [per-row entered unit] ↕
                    - columnheader "Sort Provenance" [ref=e197]:
                      - button "Sort Provenance" [ref=e198]: Provenance ↕
                  - rowgroup [ref=e200]:
                    - generic [ref=e201]:
                      - 'row "material:UIF-INVENTED-01 material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material material:UIF-INVENTED-01 Elastic: 50 MPa MPa Quantity readout material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e203]':
                        - rowheader "material:UIF-INVENTED-01" [ref=e204]:
                          - button "material:UIF-INVENTED-01" [ref=e205]
                        - 'gridcell "material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material" [ref=e206]':
                          - 'button "material:UIF-INVENTED-01 Label: Invented UI benchmark elastic material" [ref=e207]': Invented UI benchmark elastic material
                        - 'gridcell "material:UIF-INVENTED-01 Elastic: 50 MPa MPa Quantity readout" [ref=e208]':
                          - 'button "material:UIF-INVENTED-01 Elastic: 50 MPa" [ref=e209]': "50"
                          - generic "MPa" [ref=e210]
                          - generic "Quantity readout" [ref=e211]:
                            - generic [ref=e212]: 50 MPa
                        - 'gridcell "material:UIF-INVENTED-01 Shear: 77000000000 Pa Pa Quantity readout" [ref=e213]':
                          - 'button "material:UIF-INVENTED-01 Shear: 77000000000 Pa" [ref=e214]': "77000000000"
                          - generic "Pa" [ref=e215]
                          - generic "Quantity readout" [ref=e216]:
                            - generic [ref=e217]: 77000000000 Pa
                        - 'gridcell "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e218]':
                          - 'button "material:UIF-INVENTED-01 Thermal: 0.000012 1/degC" [ref=e219]': "0.000012"
                          - generic "1/degC" [ref=e220]
                          - generic "Quantity readout" [ref=e221]:
                            - generic [ref=e222]: 0.000012 1/degC
                        - 'gridcell "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e223]':
                          - 'button "material:UIF-INVENTED-01 Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e224]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                      - 'row "material:B4-second material:B4-second Label: Synthetic second material material:B4-second Elastic: 100000000000 Pa Pa Quantity readout material:B4-second Shear: 77000000000 Pa Pa Quantity readout material:B4-second Thermal: 0.000012 1/degC 1/degC Quantity readout material:B4-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e226]':
                        - rowheader "material:B4-second" [ref=e227]:
                          - button "material:B4-second" [ref=e228]
                        - 'gridcell "material:B4-second Label: Synthetic second material" [ref=e229]':
                          - 'button "material:B4-second Label: Synthetic second material" [ref=e230]': Synthetic second material
                        - 'gridcell "material:B4-second Elastic: 100000000000 Pa Pa Quantity readout" [ref=e231]':
                          - 'button "material:B4-second Elastic: 100000000000 Pa" [ref=e232]': "100000000000"
                          - generic "Pa" [ref=e233]
                          - generic "Quantity readout" [ref=e234]:
                            - generic [ref=e235]: 100000000000 Pa
                        - 'gridcell "material:B4-second Shear: 77000000000 Pa Pa Quantity readout" [ref=e236]':
                          - 'button "material:B4-second Shear: 77000000000 Pa" [ref=e237]': "77000000000"
                          - generic "Pa" [ref=e238]
                          - generic "Quantity readout" [ref=e239]:
                            - generic [ref=e240]: 77000000000 Pa
                        - 'gridcell "material:B4-second Thermal: 0.000012 1/degC 1/degC Quantity readout" [ref=e241]':
                          - 'button "material:B4-second Thermal: 0.000012 1/degC" [ref=e242]': "0.000012"
                          - generic "1/degC" [ref=e243]
                          - generic "Quantity readout" [ref=e244]:
                            - generic [ref=e245]: 0.000012 1/degC
                        - 'gridcell "material:B4-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e246]':
                          - 'button "material:B4-second Provenance: invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data" [ref=e247]': invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data
                - group "Material review drafts footer" [ref=e248]:
                  - generic [ref=e249]: 2 of 2 rows
                  - status [ref=e250]: Quantity sort unavailable; showing input order while values or units cannot be converted.
                  - button "Requested sort by Elastic · Clear" [ref=e251]
              - generic [ref=e252]:
                - button "Queue changed cells" [disabled] [ref=e253]:
                  - img [ref=e254]
                  - text: Queue changed cells
                - button "Clear grid edits" [disabled] [ref=e256]:
                  - img [ref=e257]
                  - text: Clear grid edits
              - paragraph [ref=e260]: Grid mode fans each changed cell into a structured review intent; storage remains local. Blank or whitespace text becomes TBD when queued; keeping a draft does not change the model.
            - button "Return to material fields" [expanded] [ref=e261] [cursor=pointer]
      - option "All" [selected]
      - option "Pipes"
      - option "Nodes"
      - option "Supports"
      - option "Components"
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e262]:
      - button "Agent" [disabled] [ref=e264]:
        - img [ref=e265]
        - generic [ref=e268]: Agent
  - generic "Workspace status" [ref=e269]:
    - generic "Analysis statuses" [ref=e270]:
      - button "Solver · Not solved" [ref=e272] [cursor=pointer]
    - button "2 Issues" [ref=e273] [cursor=pointer]:
      - img [ref=e274]
      - text: 2 Issues
    - generic "Selection" [ref=e276]: "project: project:UIF-PRECISION-ORIGIN"
    - generic "Display units" [ref=e277]: Entered
    - button "About SWBPIPE…" [ref=e278] [cursor=pointer]:
      - img [ref=e279]
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