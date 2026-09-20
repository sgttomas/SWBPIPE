# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: linear-authoring.spec.ts >> compact blank-to-straight authoring keeps the canvas and exact Add/Apply review
- Location: e2e/linear-authoring.spec.ts:58:1

# Error details

```
Error: expect(locator).toBeChecked() failed

Locator: getByRole('radio', { name: 'New node', exact: true })
Expected: checked
Timeout: 10000ms
Error: element(s) not found

Call log:
  - Expect "toBeChecked" with timeout 10000ms
  - waiting for getByRole('radio', { name: 'New node', exact: true })

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
  - group "Editing tools":
    - button "Undo model edit"
    - button "Redo model edit" [disabled]
    - button "Select (⎋)"
  - group "View":
    - button "Table view (⌘1)"
    - button "Model view (⌘2)"
    - button "Both view (⌘3)" [pressed]
  - button "Run"
  - button "Issues, 5": Issues 5
  - group "Panels":
    - button "Inspector"
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
        - button "Issues, 5": Issues
  - region "Modeling workspace":
    - group "Tables":
      - button "Model" [pressed]
      - button "Review changes"
      - button "Collapse table drawer" [expanded]
    - text: Model
    - region "Layout grid mode":
      - button "Tree" [pressed]
      - button "Grid"
    - region "Model tree filtering":
      - text: Filter model
      - searchbox "Filter model tree": pipe:UI-A-100
      - text: 1 of 7 model entities visible
      - button "Clear model tree filter"
    - tree "Model":
      - treeitem "Pipes" [expanded] [level=1]:
        - strong: Pipes
      - treeitem "Straight run pipe:UI-A-100" [level=2]:
        - strong: Straight run
        - text: pipe:UI-A-100
    - group "Viewport controls":
      - text: 3D Centerline
      - group "Viewport deformation overlay status": Deformation · unavailable
      - group "Viewport display toggles":
        - button "Labels" [pressed]
        - button "Loads" [pressed]
        - button "Grid" [pressed]
      - group "Viewport selection tools":
        - button "Box Select"
        - text: Selection filter
        - combobox "Selection filter":
          - option "All" [selected]
          - option "Pipes"
          - option "Nodes"
          - option "Supports"
          - option "Components"
        - button "Hide"
        - button "Isolate"
        - button "Show All" [disabled]
        - button "Fit Model"
        - button "Fit Visible"
        - button "Fit Selection"
      - group "Viewport geometry":
        - button "Schematic" [pressed]
        - button "Actual OD"
        - button "Measure"
    - text: "Selected: node:UI-A-110"
    - status "Schematic centerline geometry"
    - status "View command status": No view command dispatched.
    - button "Select Loaded end in viewport" [pressed]: UI-A-110
    - button "Select Straight run in viewport": UI-A-100
    - button "Select Anchor in viewport": UI-A-100
    - img "Orientation gizmo showing X, Y, Z axes"
    - button "Front"
    - button "Top"
    - button "Isometric" [pressed]
    - text: 1 m
    - region "Command and selection bar":
      - button "Node"
      - button "Pipe" [pressed]
      - button "Support"
      - button "Component"
      - button "Load"
      - text: Pipe tool armed
      - group: Selection & navigation
    - separator "Resize table drawer"
  - complementary "Agent":
    - button "Agent" [disabled]
  - button "Solver · Model incomplete"
  - button "5 Issues"
  - text: "node: node:UI-A-110 Entered"
  - button "About SWBPIPE…"
```

# Test source

```ts
  66  |   await expectStatusChip(page, "status-pill-mechanics", "MODEL_INCOMPLETE", "Solver · Model incomplete");
  67  | 
  68  |   await page.getByTestId("command-node").click();
  69  |   await page.getByTestId("viewport-create-node-id").fill("node:UI-A-100");
  70  |   await page.getByTestId("viewport-create-node-label").fill("Anchor");
  71  |   await page.getByTestId("viewport-create-node-x").fill("0");
  72  |   await page.getByTestId("viewport-create-node-y").fill("2.4");
  73  |   await page.getByTestId("viewport-create-node-z").fill("0");
  74  |   await page.getByTestId("viewport-create-node-provenance").fill("invented_synthetic_ui_acceptance_input");
  75  |   await assertPersistentCompactCanvas(page);
  76  |   await page.getByTestId("queue-explicit-node-intent").click();
  77  |   await expect(page.getByTestId("viewport-draft-review-preview")).toContainText("Single operation");
  78  |   await page.getByTestId("apply-reviewed-draft").click();
  79  |   await expectTreeEntity(page, "node", "node:UI-A-100");
  80  |   await projectCommand(page, "save-local");
  81  |   await projectCommand(page, "open-local");
  82  |   await expectTreeEntity(page, "node", "node:UI-A-100");
  83  |   await expectStatusChip(page, "status-pill-mechanics", "MODEL_INCOMPLETE", "Solver · Model incomplete");
  84  | 
  85  |   if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  86  |   await page.getByTestId("toolkit-entry").click();
  87  |   await page.getByTestId("toolkit-properties.material").click();
  88  |   await page.getByTestId("create-material-id").fill("material:ui-phase-a-invented");
  89  |   await page.getByTestId("create-material-label").fill("Invented carbon steel");
  90  |   await page.getByTestId("create-material-elastic").fill("200000000000");
  91  |   await page.getByTestId("create-material-shear").fill("77000000000");
  92  |   await page.getByTestId("create-material-provenance").fill("invented_synthetic_ui_acceptance_input");
  93  |   await page.getByTestId("queue-create-material-intent").click();
  94  |   await applyQueued(page);
  95  | 
  96  |   await page.getByTestId("toolkit-entry").click();
  97  |   await page.getByTestId("toolkit-properties.section").click();
  98  |   await page.getByTestId("create-section-id").fill("section:ui-phase-a-straight");
  99  |   await page.getByTestId("create-section-name").fill("Invented straight pipe");
  100 |   await page.getByTestId("create-section-od").fill("0.168");
  101 |   await page.getByTestId("create-section-wall").fill("0.007");
  102 |   await page.getByTestId("create-section-provenance").fill("invented_synthetic_ui_acceptance_input");
  103 |   await page.getByTestId("queue-create-section-intent").click();
  104 |   await applyQueued(page);
  105 | 
  106 |   await page.getByTestId("command-pipe").click();
  107 |   await page.getByTestId("viewport-create-pipe-id").fill("pipe:UI-A-100");
  108 |   await page.getByTestId("viewport-create-pipe-label").fill("Straight run");
  109 |   await chooseVirtualTarget(page, "viewport-create-pipe-from", "node:UI-A-100");
  110 |   await page.getByRole("radio", { name: "New node", exact: true }).check();
  111 |   await expect(page.getByTestId("viewport-construction-plane")).toContainText("XZ · Y=2.4 m · through node:UI-A-100");
  112 |   await page.getByRole("radio", { name: "X", exact: true }).check();
  113 |   await expect(page.getByRole("radio", { name: "Y", exact: true })).toBeDisabled();
  114 | 
  115 |   const canvas = page.getByTestId("viewport-canvas").locator("canvas");
  116 |   const bounds = await canvas.boundingBox();
  117 |   expect(bounds).not.toBeNull();
  118 |   const start = { x: bounds!.x + bounds!.width * 0.58, y: bounds!.y + bounds!.height * 0.48 };
  119 |   await page.mouse.move(start.x, start.y);
  120 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("hover route ghost");
  121 | 
  122 |   await page.mouse.move(start.x, start.y);
  123 |   await page.mouse.down();
  124 |   await page.mouse.move(start.x + 45, start.y + 32, { steps: 5 });
  125 |   await page.mouse.up();
  126 |   await expect(page.getByTestId("viewport-route-end-x")).toHaveValue("");
  127 |   await expect(page.getByTestId("viewport-route-end-y")).toHaveValue("");
  128 |   await expect(page.getByTestId("viewport-route-end-z")).toHaveValue("");
  129 |   await expect(page.getByTestId("viewport-pointer-placement-status")).toContainText("exceeded 4 CSS pixels");
  130 | 
  131 |   await canvas.click({ position: { x: bounds!.width * 0.58, y: bounds!.height * 0.48 } });
  132 |   await expect(page.getByTestId("viewport-route-end-id")).toHaveValue("node:V-001");
  133 |   await expect(page.getByTestId("viewport-route-end-label")).toHaveValue("Viewport node V-001");
  134 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("captured route ghost");
  135 |   await expect(page.getByTestId("viewport-route-end-y")).toHaveValue("2.4");
  136 | 
  137 |   await page.getByTestId("viewport-route-end-id").fill("node:UI-A-110");
  138 |   await page.getByTestId("viewport-route-end-label").fill("Loaded end");
  139 |   await page.getByTestId("viewport-route-end-x").fill("3.2");
  140 |   await page.getByTestId("viewport-route-end-y").fill("2.4");
  141 |   await page.getByTestId("viewport-route-end-z").fill("0");
  142 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("No route ghost is visible");
  143 |   await page.getByTestId("viewport-route-end-provenance").fill("invented_synthetic_ui_acceptance_input");
  144 |   await chooseVirtualTarget(page, "viewport-create-pipe-material", "material:ui-phase-a-invented");
  145 |   await page.getByTestId("viewport-create-pipe-od").fill("0.168");
  146 |   await page.getByTestId("viewport-create-pipe-wall").fill("0.007");
  147 |   await page.getByTestId("viewport-create-pipe-yref-x").fill("0");
  148 |   await page.getByTestId("viewport-create-pipe-yref-y").fill("0");
  149 |   await page.getByTestId("viewport-create-pipe-yref-z").fill("1");
  150 |   await page.getByTestId("viewport-create-pipe-provenance").fill("invented_synthetic_ui_acceptance_input");
  151 |   await page.getByTestId("continue-pipe-after-queue").check();
  152 |   await page.getByTestId("queue-explicit-pipe-intent").click();
  153 |   const review = page.getByTestId("viewport-draft-review-preview");
  154 |   await expect(review).toContainText("Atomic batch");
  155 |   await expect(review).toContainText("op:viewport-create-node-node:UI-A-110");
  156 |   await expect(review).toContainText("op:viewport-connect-pipe-pipe:UI-A-100");
  157 |   await assertPersistentCompactCanvas(page);
  158 |   await page.getByTestId("apply-reviewed-draft").click();
  159 |   await expectTreeEntity(page, "node", "node:UI-A-110");
  160 |   await expectTreeEntity(page, "pipe", "pipe:UI-A-100");
  161 |   await expect(page.getByTestId("session-history-chip")).toContainText("3 undo / 0 redo");
  162 |   await openWorkspaceSection(page, "operations");
  163 |   await expect(page.getByTestId("operation-applied-ledger")).toContainText("Applied through local_wasm_engine");
  164 |   await closeWorkspacePanels(page);
  165 |   await expectVirtualTarget(page, "viewport-create-pipe-from", "node:UI-A-110");
> 166 |   await expect(page.getByRole("radio", { name: "New node", exact: true })).toBeChecked();
      |                                                                            ^ Error: expect(locator).toBeChecked() failed
  167 |   await expect(page.getByTestId("viewport-routing-plane")).toHaveValue("XZ");
  168 |   await expect(page.getByRole("radio", { name: "X", exact: true })).toBeChecked();
  169 |   await expect(page.getByTestId("viewport-route-end-unit")).toHaveValue("m");
  170 |   await expect(page.getByTestId("viewport-construction-plane")).toContainText("XZ · Y=2.4 m · through node:UI-A-110");
  171 | 
  172 |   await page.getByRole("radio", { name: "Existing node", exact: true }).check();
  173 |   await chooseVirtualTarget(page, "viewport-create-pipe-to", "node:UI-A-100");
  174 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("existing route ghost");
  175 |   await expect(page.getByTestId("viewport-routing-aids")).toHaveAttribute("disabled", "");
  176 |   await page.getByRole("radio", { name: "New node", exact: true }).check();
  177 |   await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("No route ghost is visible");
  178 |   await assertPersistentCompactCanvas(page);
  179 | 
  180 |   await openWorkspaceSection(page, "operations");
  181 |   await page.getByTestId("undo-session-model-edit").click();
  182 |   await expectTreeEntityMissing(page, "node", "node:UI-A-110");
  183 |   await expectTreeEntityMissing(page, "pipe", "pipe:UI-A-100");
  184 |   await openWorkspaceSection(page, "operations");
  185 |   await page.getByTestId("redo-session-model-edit").click();
  186 |   await expectTreeEntity(page, "node", "node:UI-A-110");
  187 | 
  188 |   await selectTreeEntity(page, "pipe", "pipe:UI-A-100");
  189 |   await page.getByTestId("toolkit-entry").click();
  190 |   await page.getByTestId("toolkit-properties.assign-section").click();
  191 |   await page.getByLabel("Shared section").selectOption("section:ui-phase-a-straight");
  192 |   await page.getByRole("button", { name: "Queue section assignment" }).click();
  193 |   await applyQueued(page);
  194 | 
  195 |   await page.getByTestId("command-support").click();
  196 |   await page.getByTestId("create-support-id").fill("support:UI-A-100");
  197 |   await page.getByTestId("create-support-label").fill("Anchor support");
  198 |   await chooseVirtualTarget(page, "create-support-node", "node:UI-A-100");
  199 |   for (const restraint of ["RX", "RY", "RZ"]) await page.getByTestId(`create-support-restraint-${restraint}`).check();
  200 |   await page.getByTestId("create-support-provenance").fill("invented_synthetic_ui_acceptance_input");
  201 |   await page.getByTestId("queue-create-support-intent").click();
  202 |   await applyQueued(page);
  203 | 
  204 |   await openWorkspaceSection(page, "loads");
  205 |   await page.getByTestId("load-manager-create-load-id").fill("load:UI-A");
  206 |   await page.getByTestId("load-manager-create-load-label").fill("Invented Phase A force");
  207 |   await page.getByTestId("load-manager-create-load-kind").fill("primitive_user_load");
  208 |   await page.getByTestId("load-manager-create-load-status").fill("preview_only");
  209 |   await page.getByTestId("load-manager-create-load-provenance").fill("invented_synthetic_ui_acceptance_input");
  210 |   await page.getByTestId("queue-create-load-case-intent").click();
  211 |   await applyQueued(page);
  212 |   await openWorkspaceSection(page, "loads");
  213 |   await chooseVirtualTarget(page, "load-manager-create-primitive-load-case", "load:UI-A");
  214 |   await page.getByTestId("load-manager-create-primitive-category").selectOption("concentrated_force");
  215 |   await page.getByTestId("load-manager-create-primitive-id").fill("load:UI-A-FY");
  216 |   await chooseVirtualTarget(page, "load-manager-create-primitive-node", "node:UI-A-110");
  217 |   await page.getByTestId("load-manager-create-primitive-direction").selectOption("global_y");
  218 |   await page.getByTestId("load-manager-create-primitive-magnitude").fill("350");
  219 |   await page.getByTestId("load-manager-create-primitive-provenance").fill("invented_synthetic_ui_acceptance_input");
  220 |   await page.getByTestId("queue-create-primitive-intent").click();
  221 |   await applyQueued(page);
  222 | 
  223 |   const baseline350Hash = await currentModelHash(page);
  224 |   await openWorkspaceSection(page, "solve");
  225 |   await page.getByTestId("run-mechanics-preview").click();
  226 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  227 |   await page.getByTestId("issues-drawer-toggle").click();
  228 |   await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toContainText("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL");
  229 |   await page.getByTestId("issues-home").getByRole("button", { name: /Close/i }).click();
  230 |   await expectStatusChip(page, "status-pill-mechanics", "MODEL_INCOMPLETE", "Solver · Model incomplete");
  231 |   // Slice B3: the Analyze page lies over the stage's surfaces; close it to reach the canvas.
  232 |   await showCanvas(page);
  233 |   await page.getByTestId("viewport-deformation-status").locator(":scope > summary").click();
  234 |   await expect(page.getByTestId("viewport-deformation-summary")).toBeVisible();
  235 |   await expect(page.getByTestId("viewport-deformation-summary")).toHaveText("blocked; mechanics=Solver · Model incomplete (MODEL_INCOMPLETE); rows=0");
  236 |   await expect(page.getByTestId("viewport-deformation-boundary")).toHaveText("scale=not_generated; professional_claim=false");
  237 |   await page.getByTestId("viewport-deformation-status").locator(":scope > summary").click();
  238 |   await expect(page.getByTestId("viewport-deformation-summary")).toBeHidden();
  239 |   await openWorkspaceSection(page, "solve");
  240 |   await expect(page.getByTestId("rule-check-run")).toBeDisabled();
  241 |   await projectCommand(page, "save-local");
  242 |   await projectCommand(page, "open-local");
  243 |   await openWorkspaceSection(page, "results");
  244 |   await expect(page.getByTestId("historical-run-context")).toBeVisible();
  245 |   await expect(page.getByTestId("historical-run-context")).toContainText("HISTORICAL_INPUT_MANIFEST_MISSING");
  246 |   await expect(page.getByTestId("historical-run-context")).toContainText("MODEL_INCOMPLETE");
  247 |   expect(await currentModelHash(page)).toBe(baseline350Hash);
  248 |   await page.getByTestId("viewport-deformation-status").locator(":scope > summary").click();
  249 |   await expect(page.getByTestId("viewport-deformation-summary")).toBeVisible();
  250 |   await expect(page.getByTestId("viewport-deformation-summary")).toHaveText("not started; result rows=0");
  251 |   await expect(page.getByTestId("viewport-deformation-boundary")).toHaveText("scale=not_generated; professional_claim=false");
  252 | 
  253 |   await page.getByTestId("viewport-deformation-status").locator(":scope > summary").click();
  254 |   await expect(page.getByTestId("viewport-deformation-summary")).toBeHidden();
  255 | 
  256 |   // Fresh-task helpers require the separately sealed workspace-driver patch.
  257 |   const inspector = await startPropertyTaskFromTreeEntity(page, "load", "load:UI-A");
  258 |   await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  259 |   await inspector.getByTestId("editor-intent-value").fill("500");
  260 |   await inspector.getByTestId("apply-editor-intent-inline").click();
  261 |   await openWorkspaceSection(page, "operations");
  262 |   await expect(page.getByTestId("operation-apply-message")).toContainText("Applied op:editor-intent-load-load:UI-A-primitive_loads.0.magnitude.value");
  263 |   await openWorkspaceSection(page, "results");
  264 |   await expect(page.getByTestId("historical-run-context")).toHaveCount(0);
  265 |   await expect.poll(() => currentModelHash(page)).not.toBe(baseline350Hash);
  266 |   const edited500Hash = await currentModelHash(page);
```