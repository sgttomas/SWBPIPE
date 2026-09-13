# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: gui-workflow-validation.spec.ts >> DEL-09-04 invented fixture exposes warnings, boundaries, and honest solve/result transitions
- Location: e2e/gui-workflow-validation.spec.ts:100:1

# Error details

```
Error: expect(locator).toContainText(expected) failed

Locator: getByTestId('viewport-deformation-status')
Expected substring: "blocked; mechanics=model incomplete; rows=0"
Received string:    "Deformation · unavailablenot started; result rows=0scale=not_generated; professional_claim=false"
Timeout: 10000ms

Call log:
  - Expect "toContainText" with timeout 10000ms
  - waiting for getByTestId('viewport-deformation-status')
    24 × locator resolved to <details open="" data-testid="viewport-deformation-status" class="viewport-deformation-status not_started" aria-label="Viewport deformation overlay status">…</details>
       - unexpected value "Deformation · unavailablenot started; result rows=0scale=not_generated; professional_claim=false"

```

```yaml
- group "Viewport deformation overlay status":
  - text: Deformation · unavailable
  - strong: not started; result rows=0
  - text: scale=not_generated; professional_claim=false
```

# Test source

```ts
  185 |   await initialAudit.getByRole("button", { name: /Close/i }).click();
  186 | 
  187 |   // Return to the full-height authoring layout, then check actual control
  188 |   // actionability and containment instead of an obsolete fixed rail width.
  189 |   await page.getByTestId("workspace-dock-close").click();
  190 |   await expect(page.getByTestId("workspace-dock")).toHaveClass(/collapsed/);
  191 |   await expect(page.getByTestId("workspace-section-solve")).toBeHidden();
  192 | 
  193 |   // Edit explicit invented load data through the visible inspector and apply it
  194 |   // through the product's local WASM operation engine.
  195 |   await page.getByTestId(`tree-row-${editedLoadCase.id}`).click();
  196 |   const editor = page.getByTestId("editor-intent-panel");
  197 |   await editor.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  198 |   await expect(editor.getByTestId("editor-intent-value")).toHaveValue(
  199 |     String(editedLoadCase.primitive_loads[0].magnitude.value)
  200 |   );
  201 |   await expect(editor.getByTestId("editor-intent-unit")).toHaveValue(editedLoadCase.primitive_loads[0].magnitude.unit);
  202 |   await editor.getByLabel("New first primitive magnitude", { exact: true }).fill("-225");
  203 |   await expect(page.getByTestId("viewport-canvas")).toBeVisible();
  204 |   for (const controlId of ["editor-intent-field", "editor-intent-value", "editor-intent-unit", "queue-editor-intent"]) {
  205 |     const control = editor.getByTestId(controlId);
  206 |     await control.scrollIntoViewIfNeeded();
  207 |     await expect(control).toBeVisible();
  208 |     await expect(control).toBeEnabled();
  209 |     await control.click({ trial: true });
  210 |     const [bounds, rail, canvas] = await Promise.all([
  211 |       control.boundingBox(), page.locator(".workspace-pane-inspector").boundingBox(),
  212 |       page.getByTestId("viewport-canvas").boundingBox(),
  213 |     ]);
  214 |     expect(bounds).not.toBeNull();
  215 |     expect(rail).not.toBeNull();
  216 |     expect(bounds!.width).toBeGreaterThan(0);
  217 |     expect(bounds!.height).toBeGreaterThan(0);
  218 |     expect(bounds!.x).toBeGreaterThanOrEqual(rail!.x);
  219 |     expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(rail!.x + rail!.width);
  220 |     expect(bounds!.y).toBeGreaterThanOrEqual(rail!.y);
  221 |     expect(bounds!.y + bounds!.height).toBeLessThanOrEqual(rail!.y + rail!.height);
  222 |     expect(canvas).not.toBeNull();
  223 |     expect(canvas!.width).toBeGreaterThan(64);
  224 |     expect(canvas!.height).toBeGreaterThan(64);
  225 |   }
  226 |   await openNamedDisclosure(editor, "Operation details");
  227 |   await expect(editor.getByTestId("editor-operation-preview")).toContainText(
  228 |     'after={"value":-225,"unit":"N/m"}'
  229 |   );
  230 |   await editor.getByTestId("queue-editor-intent").click();
  231 |   await openWorkspaceSection(page, "operations");
  232 |   await page.getByTestId("apply-intent-editor-intent-1").click();
  233 |   const receipt = page.getByTestId("applied-operation-route-applied-1-editor-intent-1");
  234 |   await expect(receipt).toContainText("Applied through local_wasm_engine");
  235 |   await expect(receipt).toContainText("professional approval not recorded");
  236 |   await expect(page.getByTestId("workspace-section-operations")).toBeVisible();
  237 |   await page.getByTestId("menu-view").click();
  238 |   await page.getByTestId("menu-item-view.section.solve").click();
  239 |   await expect(page.getByTestId("workspace-section-solve")).toBeVisible();
  240 |   await expect(page.getByTestId("readiness-mechanics")).toBeVisible();
  241 |   await expect(page.getByTestId("readiness-mechanics")).toContainText("preview run not started");
  242 |   const resetViewportStatus = page.getByTestId("viewport-deformation-status");
  243 |   await expect(resetViewportStatus).toBeVisible();
  244 |   await setDisclosure(resetViewportStatus);
  245 |   await expect(resetViewportStatus).toContainText("not started; result rows=0");
  246 |   await setDisclosure(resetViewportStatus, false);
  247 |   await openWorkspaceSection(page, "results");
  248 |   await expect(page.getByTestId("results-panel")).toContainText(
  249 |     "Run the bounded preview mechanics path to populate result summaries."
  250 |   );
  251 | 
  252 |   // Save, list, and reopen by stable project id through normal visible controls.
  253 |   await page.getByRole("button", { name: "Save local" }).click();
  254 |   await expect(page.getByTestId("local-project-message")).toContainText(
  255 |     "Saved local browser-preview project snapshot without external file copies."
  256 |   );
  257 |   await page.getByRole("button", { name: "List local" }).click();
  258 |   await expect(page.getByTestId("local-project-message")).toContainText(
  259 |     "Listed 1 local project snapshot from the local store index."
  260 |   );
  261 |   await setDisclosure(page.getByLabel("Project summary"));
  262 |   await expect(page.getByTestId("project-index-picker")).toBeVisible();
  263 |   await page.getByTestId("project-index-open-project:invented-loop-01").click();
  264 |   await expect(page.getByTestId("local-project-message")).toContainText(
  265 |     "Opened local browser-preview project snapshot by id project:invented-loop-01."
  266 |   );
  267 | 
  268 |   await setDisclosure(page.getByLabel("Project summary"), false);
  269 | 
  270 |   // The edited unit-bearing value is still the current value after reopen.
  271 |   await page.getByTestId(`tree-row-${editedLoadCase.id}`).click();
  272 |   await editor.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  273 |   await expect(editor.getByTestId("editor-intent-value")).toHaveValue("-225");
  274 |   await expect(editor.getByTestId("editor-intent-unit")).toHaveValue("N/m");
  275 | 
  276 |   // Browser Playwright intentionally has no native solver fallback for an
  277 |   // edited model. Validate the user-visible blocking state instead of allowing
  278 |   // stale fixture results to masquerade as a successful re-solve.
  279 |   await openWorkspaceSection(page, "solve");
  280 |   await page.getByTestId("run-mechanics-preview").click();
  281 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  282 |   await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=0");
  283 |   await expectRecordedStatus(page, "status-pill-mechanics", "MODEL_INCOMPLETE");
  284 |   await setDisclosure(page.getByTestId("viewport-deformation-status"));
> 285 |   await expect(page.getByTestId("viewport-deformation-status")).toContainText(
      |                                                                 ^ Error: expect(locator).toContainText(expected) failed
  286 |     "blocked; mechanics=model incomplete; rows=0"
  287 |   );
  288 |   await setDisclosure(page.getByTestId("viewport-deformation-status"), false);
  289 |   await page.getByTestId("issues-drawer-toggle").click();
  290 |   await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toBeVisible();
  291 |   const blockedIssues = page.getByTestId("issues-home");
  292 |   await expect(blockedIssues.getByTestId("missing-data-summary")).toContainText("solve_blocked=true");
  293 |   await expect(blockedIssues.getByTestId("missing-data-summary")).toContainText("rule_blocked=true");
  294 |   await expect(blockedIssues.getByTestId("missing-data-warning-solve-required-physical-inputs")).toContainText(
  295 |     "SOLVE_BLOCKING"
  296 |   );
  297 |   await expect(blockedIssues.getByTestId("missing-data-warning-solve-required-physical-inputs")).toContainText(
  298 |     "Mechanics solve-required data is incomplete."
  299 |   );
  300 |   await blockedIssues.getByRole("button", { name: /Close/i }).click();
  301 |   await expect(blockedIssues).toHaveCount(0);
  302 | 
  303 |   await openWorkspaceSection(page, "results");
  304 |   await expect(page.getByTestId("result-filter-summary")).toContainText("0 of 0 results match filter");
  305 |   await expect(page.getByTestId("result-filter-empty")).toContainText("No computed preview result rows");
  306 | 
  307 |   await page.getByTestId("audit-drawer-toggle").click();
  308 |   const finalAudit = page.getByTestId("audit-boundary-drawer");
  309 |   await expect(finalAudit).toBeVisible();
  310 |   for (const boundaryValue of Object.values(modelFixture.data_boundary)) {
  311 |     await expect(finalAudit.getByTitle(boundaryValue)).toBeVisible();
  312 |   }
  313 |   await expect(finalAudit.getByTestId("local-project-status")).toContainText("network=false");
  314 |   await expect(finalAudit.getByTestId("local-project-status")).toContainText("telemetry=false");
  315 |   expect(externalRequests).toEqual([]);
  316 | });
  317 | 
  318 | for (const { drawerId, toggleId, contentId, contentText } of [
  319 |   { drawerId: "issues-home", toggleId: "issues-drawer-toggle", contentId: "missing-data-summary", contentText: "solve_blocked=false" },
  320 |   { drawerId: "audit-boundary-drawer", toggleId: "audit-drawer-toggle", contentId: "local-project-status", contentText: "network=false" }
  321 | ]) {
  322 |   test(`shared drawer menu overlap preserves ordinary Close and active menu priority: ${drawerId}`, async ({ page }, testInfo) => {
  323 |     await page.goto("/");
  324 |     await ensureEngineReady(page);
  325 |     await openWorkspaceSection(page, "solve");
  326 |     await page.getByTestId("run-mechanics-preview").click();
  327 |     await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  328 |     await expect(page.getByTestId("solve-job-summary")).toContainText(`result_rows=${resultFixture.results.length}`);
  329 | 
  330 |     const drawer = page.getByTestId(drawerId);
  331 |     const close = drawer.getByRole("button", { name: /Close/i });
  332 |     const menu = page.getByTestId("app-menu-bar");
  333 |     const measure = async (phase: string) => {
  334 |       const geometry = await close.evaluate((button) => {
  335 |         const nav = document.querySelector<HTMLElement>('[data-testid="app-menu-bar"]')!;
  336 |         const aside = button.closest<HTMLElement>(".workspace-drawer")!;
  337 |         const rect = button.getBoundingClientRect();
  338 |         const center = { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 };
  339 |         const hit = document.elementFromPoint(center.x, center.y);
  340 |         return {
  341 |           center, close: rect.toJSON(), menu: nav.getBoundingClientRect().toJSON(),
  342 |           drawer: aside.getBoundingClientRect().toJSON(),
  343 |           lastTriggerRight: Math.max(...Array.from(nav.querySelectorAll("button")).map((item) => item.getBoundingClientRect().right)),
  344 |           viewport: { width: innerWidth, height: innerHeight },
  345 |           hit: hit ? `${hit.tagName.toLowerCase()}.${hit.className}` : null,
  346 |           closeReceivesHit: hit === button || button.contains(hit),
  347 |           menuReceivesHit: hit === nav || nav.contains(hit),
  348 |           menuExpanded: nav.querySelector('[aria-expanded="true"]') !== null
  349 |         };
  350 |       });
  351 |       await testInfo.attach(`${drawerId}-${phase}`, { body: JSON.stringify(geometry, null, 2), contentType: "application/json" });
  352 |       return geometry;
  353 |     };
  354 | 
  355 |     // Exercise the ordinary fixture layout before imposing the controlled overlap.
  356 |     await page.getByTestId(toggleId).click();
  357 |     await expect(drawer.getByTestId(contentId)).toContainText(contentText);
  358 |     const ordinary = await measure("ordinary-layout");
  359 |     expect(ordinary.closeReceivesHit, `ordinary Close intercepted by ${ordinary.hit}`).toBe(true);
  360 |     await close.click();
  361 |     await expect(drawer).toHaveCount(0);
  362 | 
  363 |     await page.getByTestId(toggleId).click();
  364 |     await expect(drawer).toBeVisible();
  365 |     // Position only this existing drawer: no stacking, pointer-event, visibility,
  366 |     // or handler changes. This geometry is deterministic, not a hosted-pixel claim.
  367 |     await drawer.evaluate((aside) => {
  368 |       const nav = document.querySelector('[data-testid="app-menu-bar"]')!.getBoundingClientRect();
  369 |       const button = Array.from(aside.querySelectorAll("button")).find((item) => /Close/i.test(item.textContent ?? ""))!;
  370 |       const rect = button.getBoundingClientRect();
  371 |       const offset = rect.y + rect.height / 2 - aside.getBoundingClientRect().top;
  372 |       aside.style.top = `${nav.y + nav.height / 2 - offset}px`;
  373 |       aside.style.bottom = "auto";
  374 |     });
  375 |     const overlap = await measure("controlled-closed-menu");
  376 |     expect(overlap.menuExpanded).toBe(false);
  377 |     expect(overlap.center.x).toBeGreaterThan(overlap.lastTriggerRight);
  378 |     expect(overlap.center.x).toBeGreaterThan(overlap.menu.left);
  379 |     expect(overlap.center.x).toBeLessThan(overlap.menu.right);
  380 |     expect(overlap.center.y).toBeGreaterThan(overlap.menu.top);
  381 |     expect(overlap.center.y).toBeLessThan(overlap.menu.bottom);
  382 |     expect(overlap.close.left).toBeGreaterThanOrEqual(0);
  383 |     expect(overlap.close.top).toBeGreaterThanOrEqual(0);
  384 |     expect(overlap.close.right).toBeLessThanOrEqual(overlap.viewport.width);
  385 |     expect(overlap.close.bottom).toBeLessThanOrEqual(overlap.viewport.height);
```