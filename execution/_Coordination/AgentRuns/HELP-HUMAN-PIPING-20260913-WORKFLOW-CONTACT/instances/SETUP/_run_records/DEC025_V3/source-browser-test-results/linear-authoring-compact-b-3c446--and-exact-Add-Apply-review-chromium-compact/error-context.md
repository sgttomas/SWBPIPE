# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: linear-authoring.spec.ts >> compact blank-to-straight authoring keeps the canvas and exact Add/Apply review
- Location: e2e/linear-authoring.spec.ts:43:1

# Error details

```
Error: expect(locator).toContainText(expected) failed

Locator: getByTestId('viewport-deformation-status')
Expected substring: "result rows=0"
Received string:    "Deformation · blockedblocked; mechanics=model incomplete; rows=0scale=not_generated; professional_claim=false"
Timeout: 10000ms

Call log:
  - Expect "toContainText" with timeout 10000ms
  - waiting for getByTestId('viewport-deformation-status')
    24 × locator resolved to <details data-testid="viewport-deformation-status" class="viewport-deformation-status blocked" aria-label="Viewport deformation overlay status">…</details>
       - unexpected value "Deformation · blockedblocked; mechanics=model incomplete; rows=0scale=not_generated; professional_claim=false"

```

```yaml
- group "Viewport deformation overlay status": Deformation · blocked
```

# Test source

```ts
  112 |   await expect(page.getByTestId("viewport-pointer-placement-status")).toContainText("exceeded 4 CSS pixels");
  113 | 
  114 |   await canvas.click({ position: { x: bounds!.width * 0.58, y: bounds!.height * 0.48 } });
  115 |   await expect(page.getByTestId("viewport-route-end-id")).toHaveValue("node:V-001");
  116 |   await expect(page.getByTestId("viewport-route-end-label")).toHaveValue("Viewport node V-001");
  117 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("captured route ghost");
  118 |   await expect(page.getByTestId("viewport-route-end-y")).toHaveValue("2.4");
  119 | 
  120 |   await page.getByTestId("viewport-route-end-id").fill("node:UI-A-110");
  121 |   await page.getByTestId("viewport-route-end-label").fill("Loaded end");
  122 |   await page.getByTestId("viewport-route-end-x").fill("3.2");
  123 |   await page.getByTestId("viewport-route-end-y").fill("2.4");
  124 |   await page.getByTestId("viewport-route-end-z").fill("0");
  125 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("No route ghost is visible");
  126 |   await page.getByTestId("viewport-route-end-provenance").fill("invented_synthetic_ui_acceptance_input");
  127 |   await page.getByTestId("viewport-create-pipe-material").selectOption("material:ui-phase-a-invented");
  128 |   await page.getByTestId("viewport-create-pipe-od").fill("0.168");
  129 |   await page.getByTestId("viewport-create-pipe-wall").fill("0.007");
  130 |   await page.getByTestId("viewport-create-pipe-yref-x").fill("0");
  131 |   await page.getByTestId("viewport-create-pipe-yref-y").fill("0");
  132 |   await page.getByTestId("viewport-create-pipe-yref-z").fill("1");
  133 |   await page.getByTestId("viewport-create-pipe-provenance").fill("invented_synthetic_ui_acceptance_input");
  134 |   await page.getByTestId("continue-pipe-after-queue").check();
  135 |   await page.getByTestId("queue-explicit-pipe-intent").click();
  136 |   const review = page.getByTestId("viewport-draft-review-preview");
  137 |   await expect(review).toContainText("Atomic batch");
  138 |   await expect(review).toContainText("op:viewport-create-node-node:UI-A-110");
  139 |   await expect(review).toContainText("op:viewport-connect-pipe-pipe:UI-A-100");
  140 |   await assertPersistentCompactCanvas(page);
  141 |   await page.getByTestId("apply-reviewed-draft").click();
  142 |   await expect(page.getByTestId("tree-row-node:UI-A-110")).toBeAttached();
  143 |   await expect(page.getByTestId("tree-row-pipe:UI-A-100")).toBeAttached();
  144 |   await expect(page.getByTestId("session-history-chip")).toContainText("3 undo / 0 redo");
  145 |   await expect(page.getByTestId("operation-applied-ledger")).toContainText("Applied through local_wasm_engine");
  146 |   await expect(page.getByTestId("viewport-create-pipe-from")).toHaveValue("node:UI-A-110");
  147 |   await expect(page.getByRole("radio", { name: "New node", exact: true })).toBeChecked();
  148 |   await expect(page.getByTestId("viewport-routing-plane")).toHaveValue("XZ");
  149 |   await expect(page.getByRole("radio", { name: "X", exact: true })).toBeChecked();
  150 |   await expect(page.getByTestId("viewport-route-end-unit")).toHaveValue("m");
  151 |   await expect(page.getByTestId("viewport-construction-plane")).toContainText("XZ · Y=2.4 m · through node:UI-A-110");
  152 | 
  153 |   await page.getByRole("radio", { name: "Existing node", exact: true }).check();
  154 |   await page.getByTestId("viewport-create-pipe-to").selectOption("node:UI-A-100");
  155 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("existing route ghost");
  156 |   await expect(page.getByTestId("viewport-routing-aids")).toHaveAttribute("disabled", "");
  157 |   await page.getByRole("radio", { name: "New node", exact: true }).check();
  158 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("No route ghost is visible");
  159 |   await assertPersistentCompactCanvas(page);
  160 | 
  161 |   await page.getByTestId("workspace-review").click();
  162 |   await page.getByTestId("undo-session-model-edit").click();
  163 |   await expect(page.getByTestId("tree-row-node:UI-A-110")).toHaveCount(0);
  164 |   await expect(page.getByTestId("tree-row-pipe:UI-A-100")).toHaveCount(0);
  165 |   await page.getByTestId("redo-session-model-edit").click();
  166 |   await expect(page.getByTestId("tree-row-node:UI-A-110")).toBeAttached();
  167 | 
  168 |   if (await page.getByTestId("toggle-tree").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-tree").click();
  169 |   await page.getByTestId("tree-row-pipe:UI-A-100").click();
  170 |   await page.getByTestId("toolkit-entry").click();
  171 |   await page.getByTestId("toolkit-properties.assign-section").click();
  172 |   await page.getByLabel("Shared section").selectOption("section:ui-phase-a-straight");
  173 |   await page.getByRole("button", { name: "Queue section assignment" }).click();
  174 |   await applyQueued(page);
  175 | 
  176 |   await page.getByTestId("command-support").click();
  177 |   await page.getByTestId("create-support-id").fill("support:UI-A-100");
  178 |   await page.getByTestId("create-support-label").fill("Anchor support");
  179 |   await page.getByTestId("create-support-node").selectOption("node:UI-A-100");
  180 |   for (const restraint of ["RX", "RY", "RZ"]) await page.getByTestId(`create-support-restraint-${restraint}`).check();
  181 |   await page.getByTestId("create-support-provenance").fill("invented_synthetic_ui_acceptance_input");
  182 |   await page.getByTestId("queue-create-support-intent").click();
  183 |   await applyQueued(page);
  184 | 
  185 |   await page.getByTestId("workspace-task-loads").click();
  186 |   await page.getByTestId("load-manager-create-load-id").fill("load:UI-A");
  187 |   await page.getByTestId("load-manager-create-load-label").fill("Invented Phase A force");
  188 |   await page.getByTestId("load-manager-create-load-kind").fill("primitive_user_load");
  189 |   await page.getByTestId("load-manager-create-load-status").fill("preview_only");
  190 |   await page.getByTestId("load-manager-create-load-provenance").fill("invented_synthetic_ui_acceptance_input");
  191 |   await page.getByTestId("queue-create-load-case-intent").click();
  192 |   await applyQueued(page);
  193 |   await page.getByTestId("workspace-task-loads").click();
  194 |   await page.getByTestId("load-manager-create-primitive-load-case").selectOption("load:UI-A");
  195 |   await page.getByTestId("load-manager-create-primitive-category").selectOption("concentrated_force");
  196 |   await page.getByTestId("load-manager-create-primitive-id").fill("load:UI-A-FY");
  197 |   await page.getByTestId("load-manager-create-primitive-node").selectOption("node:UI-A-110");
  198 |   await page.getByTestId("load-manager-create-primitive-direction").selectOption("global_y");
  199 |   await page.getByTestId("load-manager-create-primitive-magnitude").fill("350");
  200 |   await page.getByTestId("load-manager-create-primitive-provenance").fill("invented_synthetic_ui_acceptance_input");
  201 |   await page.getByTestId("queue-create-primitive-intent").click();
  202 |   await applyQueued(page);
  203 | 
  204 |   const baseline350Hash = await currentModelHash(page);
  205 |   await page.getByTestId("workspace-task-solve").click();
  206 |   await page.getByTestId("run-mechanics-preview").click();
  207 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  208 |   await page.getByTestId("issues-drawer-toggle").click();
  209 |   await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toContainText("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL");
  210 |   await page.getByTestId("issues-home").getByRole("button", { name: /Close/i }).click();
  211 |   await expect(page.getByTestId("status-pill-mechanics")).toContainText("MODEL_INCOMPLETE");
> 212 |   await expect(page.getByTestId("viewport-deformation-status")).toContainText("result rows=0");
      |                                                                 ^ Error: expect(locator).toContainText(expected) failed
  213 |   await expect(page.getByTestId("rule-check-run")).toBeDisabled();
  214 |   await page.getByRole("button", { name: "Save local" }).click();
  215 |   await page.getByRole("button", { name: "Open local" }).click();
  216 |   await expect(page.getByTestId("historical-run-context")).toContainText("HISTORICAL_INPUT_MANIFEST_MISSING");
  217 |   await expect(page.getByTestId("historical-run-context")).toContainText("MODEL_INCOMPLETE");
  218 |   expect(await currentModelHash(page)).toBe(baseline350Hash);
  219 |   await expect(page.getByTestId("viewport-deformation-status")).toContainText("result rows=0");
  220 | 
  221 |   await page.getByTestId("tree-row-load:UI-A").click();
  222 |   const inspector = page.getByLabel("Property inspector");
  223 |   await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  224 |   await inspector.getByTestId("editor-intent-value").fill("500");
  225 |   await inspector.getByTestId("apply-editor-intent-inline").click();
  226 |   await expect(page.getByTestId("operation-apply-message")).toContainText("Applied op:editor-intent-load:UI-A-primitive_loads.0.magnitude.value");
  227 |   await expect(page.getByTestId("historical-run-context")).toHaveCount(0);
  228 |   await expect.poll(() => currentModelHash(page)).not.toBe(baseline350Hash);
  229 |   const edited500Hash = await currentModelHash(page);
  230 |   await page.getByTestId("workspace-review").click();
  231 |   await page.getByTestId("undo-session-model-edit").click();
  232 |   await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  233 |   await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("350");
  234 |   await expect.poll(() => currentModelHash(page)).toBe(baseline350Hash);
  235 |   await page.getByTestId("redo-session-model-edit").click();
  236 |   await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  237 |   await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("500");
  238 |   await expect.poll(() => currentModelHash(page)).toBe(edited500Hash);
  239 |   await page.getByRole("button", { name: "Save local" }).click();
  240 |   await page.getByRole("button", { name: "Open local" }).click();
  241 |   await expect(page.getByTestId("tree-row-pipe:UI-A-100")).toBeAttached();
  242 |   await page.getByTestId("tree-row-load:UI-A").click();
  243 |   await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  244 |   await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("500");
  245 |   expect(await currentModelHash(page)).toBe(edited500Hash);
  246 |   await expect(page.getByTestId("historical-run-context")).toHaveCount(0);
  247 | });
  248 | 
```