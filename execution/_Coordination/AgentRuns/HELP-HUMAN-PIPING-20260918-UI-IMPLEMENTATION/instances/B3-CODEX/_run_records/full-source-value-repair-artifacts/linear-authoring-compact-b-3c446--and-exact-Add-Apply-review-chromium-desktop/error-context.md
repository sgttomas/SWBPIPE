# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: linear-authoring.spec.ts >> compact blank-to-straight authoring keeps the canvas and exact Add/Apply review
- Location: e2e/linear-authoring.spec.ts:60:1

# Error details

```
Error: expect(locator).toHaveAttribute(expected) failed

Locator: getByLabel('Shared section')
Expected: "true"
Error: strict mode violation: getByLabel('Shared section') resolved to 2 elements:
    1) <button id="_r_2f_" type="button" data-value="" role="combobox" aria-expanded="true" aria-haspopup="listbox" aria-label="Shared section" aria-controls="_r_2f_-list" class="compact-select-trigger" aria-activedescendant="_r_2f_-list-0">…</button> aka getByRole('combobox', { name: 'Shared section' })
    2) <div role="listbox" id="_r_2f_-list" aria-label="Shared section" class="compact-select-popup">…</div> aka getByRole('listbox', { name: 'Shared section' })

Call log:
  - Expect "toHaveAttribute" with timeout 10000ms
  - waiting for getByLabel('Shared section')

```

# Page snapshot

```yaml
- generic [ref=e1]:
  - main [ref=e3]:
    - navigation "Application menu" [ref=e4]:
      - button "File" [ref=e6] [cursor=pointer]
      - button "Edit" [ref=e8] [cursor=pointer]
      - button "View" [ref=e10] [cursor=pointer]
      - button "Insert" [ref=e12] [cursor=pointer]
      - button "Analyze" [ref=e14] [cursor=pointer]
    - generic "Toolbar" [ref=e15]:
      - heading "SWBPIPE" [level=1] [ref=e17]
      - group "Editing tools" [ref=e18]:
        - button "Undo model edit" [ref=e19] [cursor=pointer]:
          - img [ref=e20]
        - button "Redo model edit" [disabled] [ref=e23]:
          - img [ref=e24]
        - button "Select (⎋)" [pressed] [ref=e27] [cursor=pointer]:
          - img [ref=e28]
      - group "View" [ref=e30]:
        - button "Table view (⌘1)" [ref=e32] [cursor=pointer]:
          - img [ref=e33]
        - button "Model view (⌘2)" [ref=e36] [cursor=pointer]:
          - img [ref=e37]
        - button "Both view (⌘3)" [pressed] [ref=e41] [cursor=pointer]:
          - img [ref=e42]
      - generic [ref=e44]:
        - button "Run" [ref=e45] [cursor=pointer]:
          - img [ref=e46]
          - generic [ref=e48]: Run
        - button "Issues, 5" [ref=e49] [cursor=pointer]:
          - img [ref=e50]
          - generic [ref=e52]: Issues
          - generic [ref=e53]: "5"
      - group "Panels" [ref=e54]:
        - button "Inspector" [expanded] [ref=e56] [cursor=pointer]:
          - img [ref=e57]
        - button "Agent" [disabled] [ref=e60]:
          - img [ref=e61]
      - generic "Display units" [ref=e64]:
        - combobox "Display units" [ref=e65]:
          - option "Entered" [selected]
          - option "SI"
          - option "US"
      - group [ref=e66]:
        - generic "Appearance" [ref=e67] [cursor=pointer]:
          - img [ref=e68]
        - option "System" [selected]
        - option "Light"
        - option "Dark"
        - option "Comfortable" [selected]
        - option "Compact"
      - region "Human toolkit" [ref=e70]:
        - button "Find modeling commands" [ref=e71] [cursor=pointer]:
          - img [ref=e72]
          - generic [ref=e75]: Search or command…
          - generic "Command K" [ref=e76]: ⌘K
    - generic [ref=e77]:
      - navigation "Stages" [ref=e78]:
        - list [ref=e79]:
          - listitem [ref=e80]:
            - button "Model" [pressed] [ref=e81] [cursor=pointer]:
              - img [ref=e82]
              - generic [ref=e85]: Model
          - listitem [ref=e86]:
            - button "Loads" [ref=e87] [cursor=pointer]:
              - img [ref=e88]
              - generic [ref=e92]: Loads
          - listitem [ref=e93]:
            - button "Results" [disabled] [ref=e94]:
              - img [ref=e95]
              - generic [ref=e98]: Results
          - listitem [ref=e99]:
            - button "Review" [disabled] [ref=e100]:
              - img [ref=e101]
              - generic [ref=e105]: Review
        - separator [ref=e106]
        - list [ref=e107]:
          - listitem [ref=e108]:
            - button "Libraries" [ref=e109] [cursor=pointer]:
              - img [ref=e110]
              - generic [ref=e112]: Libraries
          - listitem [ref=e113]:
            - button "Rules" [ref=e114] [cursor=pointer]:
              - img [ref=e115]
              - generic [ref=e119]: Rules
          - listitem [ref=e120]:
            - button "Issues, 5" [ref=e121] [cursor=pointer]:
              - img [ref=e122]
              - generic [ref=e124]: Issues
              - generic [ref=e125]: "5"
      - region "Modeling workspace" [ref=e127]:
        - group "Tables" [ref=e129]:
          - button "Model" [pressed] [ref=e130] [cursor=pointer]
          - button "Review changes" [ref=e131] [cursor=pointer]
          - button "Expand table drawer" [ref=e134] [cursor=pointer]:
            - img [ref=e135]
        - generic [ref=e138]:
          - generic [ref=e139]:
            - group "Viewport controls" [ref=e140]:
              - generic [ref=e141]: 3D Centerline
              - group "Viewport deformation overlay status" [ref=e142]:
                - generic "Deformation · unavailable" [ref=e143] [cursor=pointer]
              - group "Viewport display toggles" [ref=e144]:
                - button "Labels" [pressed] [ref=e145]
                - button "Loads" [pressed] [ref=e146]
                - button "Grid" [pressed] [ref=e147]
              - group "Viewport selection tools" [ref=e148]:
                - button "Box Select" [ref=e149]
                - generic [ref=e150]:
                  - generic [ref=e151]: Selection filter
                  - combobox "Selection filter" [ref=e152]:
                    - option "All" [selected]
                    - option "Pipes"
                    - option "Nodes"
                    - option "Supports"
                    - option "Components"
                - button "Hide" [ref=e153]
                - button "Isolate" [ref=e154]
                - button "Show All" [disabled] [ref=e155]
                - button "Fit Model" [ref=e156]
                - button "Fit Visible" [ref=e157]
                - button "Fit Selection" [ref=e158]
              - group "Viewport geometry" [ref=e159]:
                - button "Schematic" [pressed] [ref=e160]
                - button "Actual OD" [ref=e161]
                - button "Measure" [ref=e162]
            - generic "Viewport status" [ref=e163]:
              - 'generic "Selected pipe: pipe:UI-A-100" [ref=e164]': "Selected: pipe:UI-A-100"
              - status "Schematic centerline geometry" [ref=e165]
              - status "View command status" [ref=e166]: No view command dispatched.
          - generic [ref=e167]:
            - generic "Three.js pipe centerline viewport" [ref=e168]
            - generic "Viewport entity selection":
              - button "Select Straight run in viewport" [pressed] [ref=e170] [cursor=pointer]:
                - img [ref=e171]
                - generic [ref=e175]: UI-A-100
              - button "Select Anchor in viewport" [ref=e176] [cursor=pointer]:
                - img [ref=e177]
                - generic [ref=e180]: UI-A-100
            - img "Orientation gizmo showing X, Y, Z axes"
            - generic "View controls" [ref=e181]:
              - button "Front" [ref=e182] [cursor=pointer]
              - button "Top" [ref=e183] [cursor=pointer]
              - button "Isometric" [pressed] [ref=e184] [cursor=pointer]
            - generic:
              - generic: 1 m
          - region "Command and selection bar" [ref=e185]:
            - generic "Object creation tools" [ref=e186]:
              - button "Node" [ref=e187] [cursor=pointer]:
                - img [ref=e188]
                - text: Node
              - button "Pipe" [ref=e190] [cursor=pointer]:
                - img [ref=e191]
                - text: Pipe
              - button "Support" [ref=e195] [cursor=pointer]:
                - img [ref=e196]
                - text: Support
              - button "Component" [ref=e199] [cursor=pointer]:
                - img [ref=e200]
                - text: Component
              - button "Load" [ref=e203] [cursor=pointer]:
                - img [ref=e204]
                - text: Load
            - generic "Model focus" [ref=e206]: Select
            - group [ref=e207]:
              - generic "Selection & navigation" [ref=e208] [cursor=pointer]
        - generic [ref=e209]:
          - button "Close inspector" [ref=e210]:
            - img [ref=e211]
          - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾
          - region "Property inspector" [ref=e214]:
            - 'heading "Straight run — pipe: pipe:UI-A-100" [level=2] [ref=e215]':
              - text: Straight run
              - generic [ref=e216]: "— pipe: pipe:UI-A-100"
            - tablist "Inspector views" [ref=e217]:
              - tab "Properties" [selected] [ref=e218]
              - tab "Task" [ref=e219]
            - tabpanel [ref=e220]:
              - group [ref=e221]:
                - generic "All properties" [ref=e222] [cursor=pointer]
              - paragraph [ref=e223]: "Local pipe section: diameter and wall belong to this pipe."
            - generic [ref=e224]:
              - group [ref=e225]:
                - generic "Sources and units" [ref=e226] [cursor=pointer]
              - region "Pipe section assignment" [ref=e227]:
                - heading "Pipe section" [level=3] [ref=e228]
                - paragraph [ref=e229]: Local dimensions. Choose a shared section to use its diameter and wall. Per-pipe supplements remain local.
                - generic [ref=e230]:
                  - text: Shared section
                  - combobox "Shared section" [expanded] [active] [ref=e232]:
                    - generic [ref=e233]: Choose a section
                    - text: ▾
                - button "Queue section assignment" [disabled] [ref=e234]
                - button "Queue detachment to local dimensions" [disabled] [ref=e235]
                - paragraph [ref=e236]: Review the affected pipes and exact dimensions in Pending changes before applying.
              - group [ref=e237]:
                - generic "New section" [ref=e238] [cursor=pointer]
                - region "Create section intent" [ref=e239]:
                  - generic [ref=e240]:
                    - generic [ref=e241]:
                      - generic [ref=e242]: Section ID
                      - textbox "New section ID" [ref=e243]: section:S-1
                    - generic [ref=e244]:
                      - generic [ref=e245]: Name
                      - textbox "New section name" [ref=e246]: Section S-1
                    - generic [ref=e247]:
                      - generic [ref=e248]: Type
                      - combobox "New section type" [ref=e250] [cursor=pointer]:
                        - generic [ref=e251]: pipe
                        - text: ▾
                    - generic [ref=e252]:
                      - generic [ref=e253]: Length unit
                      - combobox "New section length unit" [ref=e255] [cursor=pointer]:
                        - generic [ref=e256]: m, model metadata
                        - text: ▾
                    - generic [ref=e257]:
                      - generic [ref=e258]: Outside diameter (m, model metadata)
                      - textbox "New section outside diameter" [ref=e259]
                    - generic [ref=e260]:
                      - generic [ref=e261]: Wall thickness (m, model metadata)
                      - textbox "New section wall thickness" [ref=e262]
                    - generic [ref=e263]:
                      - generic [ref=e264]: Provenance
                      - textbox "New section provenance" [ref=e265]: user_entered_local_preview
                    - button "Queue section" [disabled] [ref=e266]:
                      - img [ref=e267]
                      - text: Queue section
              - group [ref=e269]:
                - generic "New material" [ref=e270] [cursor=pointer]
                - region "Create material intent" [ref=e271]:
                  - generic [ref=e272]:
                    - generic [ref=e273]:
                      - generic [ref=e274]: Material ID
                      - textbox "New material ID" [ref=e275]: material:M-1
                    - generic [ref=e276]:
                      - generic [ref=e277]: Label
                      - textbox "New material label" [ref=e278]: Material M-1
                    - generic [ref=e279]:
                      - generic [ref=e280]: Modulus unit
                      - combobox "New material modulus unit" [ref=e282] [cursor=pointer]:
                        - generic [ref=e283]: Pa, model metadata
                        - text: ▾
                    - generic [ref=e284]:
                      - generic [ref=e285]: Elastic modulus (Pa, model metadata)
                      - textbox "New material elastic modulus" [ref=e286]
                    - generic [ref=e287]:
                      - generic [ref=e288]: Shear modulus (Pa, model metadata)
                      - textbox "New material shear modulus" [ref=e289]
                    - generic [ref=e290]:
                      - generic [ref=e291]: Thermal expansion unit
                      - combobox "New material thermal expansion unit" [ref=e293] [cursor=pointer]:
                        - generic [ref=e294]: 1/C, model metadata
                        - text: ▾
                    - generic [ref=e295]:
                      - generic [ref=e296]: Thermal expansion (1/C, model metadata)
                      - textbox "New material thermal expansion" [ref=e297]
                    - generic [ref=e298]:
                      - generic [ref=e299]: Provenance
                      - textbox "New material provenance" [ref=e300]: user_entered_local_preview
                    - button "Queue material" [disabled] [ref=e301]:
                      - img [ref=e302]
                      - text: Queue material
              - group [ref=e304]:
                - generic "New support" [ref=e305] [cursor=pointer]
                - text: ▾
              - group [ref=e306]:
                - generic "New component" [ref=e307] [cursor=pointer]
                - text: ▾ ▾ ▾
              - region "Delete pipe intent" [ref=e308]:
                - heading "Delete pipe" [level=3] [ref=e309]
                - button "Queue delete pipe" [ref=e311] [cursor=pointer]:
                  - img [ref=e312]
                  - text: Queue delete pipe
                - article [ref=e315]:
                  - generic [ref=e316]:
                    - generic [ref=e317]:
                      - generic [ref=e318]: Operation
                      - strong [ref=e319]: op:delete-pipe-pipe:UI-A-100; delete; proposed
                    - generic [ref=e320]:
                      - generic [ref=e321]: Target
                      - strong [ref=e322]: Element; pipe:UI-A-100
                    - generic [ref=e323]:
                      - generic [ref=e324]: Change
                      - strong [ref=e325]: delete_pipe_run; Explicit pipe deletion; pipe_segments; before=Straight run; node:UI-A-100->node:UI-A-110; material=material:ui-phase-a-invented; after=not_present
                    - generic [ref=e326]:
                      - generic [ref=e327]: Unit basis
                      - strong [ref=e328]: dimensionless; unit=none; explicit user-entered pipe deletion; primitive-load reference integrity required
                    - generic [ref=e329]:
                      - generic [ref=e330]: Validation
                      - strong [ref=e331]: not_run; not_run; not_required_dimensionless; not_generated; not_applied
                    - generic [ref=e332]:
                      - generic [ref=e333]: Audit boundary
                      - strong [ref=e334]: Routed through the structured operations only; no direct model mutation; requires your acceptance; does not change the accepted model until applied
                    - generic [ref=e335]:
                      - generic [ref=e336]: Professional boundary
                      - strong [ref=e337]: Requires human review; no compliance claim; no approval claim
                    - generic [ref=e338]:
                      - generic [ref=e339]: Rationale
                      - strong [ref=e340]: explicit user-entered pipe deletion for project:blank-local-20260920t055158z; requires reference validation before durable model change.
      - complementary "Agent" [ref=e341]:
        - button "Agent" [disabled] [ref=e343]:
          - img [ref=e344]
          - generic [ref=e347]: Agent
    - generic "Workspace status" [ref=e348]:
      - generic "Analysis statuses" [ref=e349]:
        - button "Solver · Model incomplete" [ref=e351] [cursor=pointer]
      - button "5 Issues" [ref=e352] [cursor=pointer]:
        - img [ref=e353]
        - text: 5 Issues
      - generic "Selection" [ref=e355]: "pipe: pipe:UI-A-100"
      - generic "Display units" [ref=e356]: Entered
      - button "About SWBPIPE…" [ref=e357] [cursor=pointer]:
        - img [ref=e358]
  - listbox "Shared section" [ref=e360]:
    - option "Choose a section" [selected] [ref=e361]
    - option "Invented straight pipe (section:ui-phase-a-straight)" [ref=e362]
```

# Test source

```ts
  217 | export async function expectVirtualTarget(scope: Page | Locator, testId: string, value: string): Promise<void> {
  218 |   await expect(scope.getByTestId(testId).locator(".virtual-target-picker-current")).toContainText(value);
  219 | }
  220 | 
  221 | function escapeRegExp(value: string): string {
  222 |   return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  223 | }
  224 | 
  225 | // --- Slice B3 helpers -------------------------------------------------------
  226 | 
  227 | export type ProjectCommand = "new-local" | "new-blank" | "open-local" | "list-local" | "save-local";
  228 | 
  229 | const PROJECT_BUTTON_NAMES: Record<ProjectCommand, string> = {
  230 |   "new-local": "Create local",
  231 |   "new-blank": "New blank",
  232 |   "open-local": "Open local",
  233 |   "list-local": "List local",
  234 |   "save-local": "Save local"
  235 | };
  236 | 
  237 | /**
  238 |  * The five project buttons live in the Project page's header. This opens that
  239 |  * page when it is not the open page and returns the button, found by its role
  240 |  * and its unchanged accessible name.
  241 |  */
  242 | export async function projectButton(page: Page, command: ProjectCommand): Promise<Locator> {
  243 |   const button = page.getByRole("button", { name: PROJECT_BUTTON_NAMES[command], exact: true });
  244 |   if (!await button.isVisible()) await openWorkspaceSection(page, "project");
  245 |   await expect(button).toBeVisible();
  246 |   return button;
  247 | }
  248 | 
  249 | /**
  250 |  * The same command from the File menu, which runs it through the same command
  251 |  * sink without opening a page over the canvas. `keyboard` drives the menu by
  252 |  * focus and Enter, for tests that hold a pointer button down on the canvas.
  253 |  */
  254 | export async function projectCommand(page: Page, command: ProjectCommand, keyboard = false): Promise<void> {
  255 |   const menu = page.getByTestId("menu-file");
  256 |   const item = page.getByTestId(`menu-item-file.${command}`);
  257 |   if (keyboard) {
  258 |     await menu.focus(); await page.keyboard.press("Enter");
  259 |     await item.focus(); await page.keyboard.press("Enter");
  260 |   } else {
  261 |     await menu.click();
  262 |     await item.click();
  263 |   }
  264 | }
  265 | 
  266 | /**
  267 |  * A status chip of the status bar (specification §5.4): its face is "Domain · Label", the
  268 |  * recorded token is its tooltip and, on a click, the monospace line of its popover.
  269 |  */
  270 | export async function expectStatusChip(page: Page, testId: string, token: string, face: string): Promise<void> {
  271 |   const chip = page.getByTestId(testId);
  272 |   await expect(chip).toHaveText(face);
  273 |   await expect(chip).toHaveAttribute("title", token);
  274 |   await chip.click();
  275 |   const recorded = page.getByTestId(`${testId}-popover`).locator("code");
  276 |   await expect(recorded).toBeVisible();
  277 |   await expect(recorded).toHaveText(token);
  278 |   await chip.click();
  279 |   await expect(page.getByTestId(`${testId}-popover`)).toHaveCount(0);
  280 | }
  281 | 
  282 | export async function expectNoStatusChip(page: Page, testId: string): Promise<void> {
  283 |   await expect(page.getByTestId(testId)).toHaveCount(0);
  284 | }
  285 | 
  286 | /**
  287 |  * A recorded status that the chip policy does not draw stays readable, with its
  288 |  * token, in the Analyze page's readiness summary. The page is closed again.
  289 |  */
  290 | export async function expectRecordedStatusOnAnalyzePage(page: Page, row: "mechanics" | "rule" | "professional", token: string): Promise<void> {
  291 |   const wasOpen = await page.getByTestId("workspace-section-solve").isVisible();
  292 |   const section = await openWorkspaceSection(page, "solve");
  293 |   await expect(section.getByTestId(`readiness-${row}`)).toContainText(`(${token})`);
  294 |   if (!wasOpen) await page.getByTestId("workspace-dock-close").click();
  295 | }
  296 | 
  297 | /**
  298 |  * The canvas is drawn in Model and Both view and is covered by an open page. This closes a
  299 |  * page and, where the current stage is in Table view, returns to the Model stage (Both view
  300 |  * at first open), so that a test can reach the canvas's own controls.
  301 |  */
  302 | export async function showCanvas(page: Page): Promise<void> {
  303 |   const close = page.getByTestId("workspace-dock-close");
  304 |   if (await close.isVisible()) await close.click();
  305 |   const canvas = page.getByTestId("viewport-canvas");
  306 |   if (!await canvas.isVisible()) {
  307 |     await page.getByTestId("rail-stage-model").click();
  308 |     if (!await canvas.isVisible()) await page.getByTestId("view-switch-both").click();
  309 |   }
  310 |   await expect(canvas).toBeVisible();
  311 | }
  312 | 
  313 | /** Choose a canonical value through the compact selector's actual rendered popup. */
  314 | export async function selectCompactOption(control: Locator, value: string): Promise<void> {
  315 |   await expect(control).toHaveAttribute("role", "combobox");
  316 |   await control.click();
> 317 |   await expect(control).toHaveAttribute("aria-expanded", "true");
      |                         ^ Error: expect(locator).toHaveAttribute(expected) failed
  318 |   const listId = await control.getAttribute("aria-controls");
  319 |   if (!listId) throw new Error("Compact selector did not identify its open listbox");
  320 |   const list = control.page().locator(`[id=${JSON.stringify(listId)}]`);
  321 |   const option = list.locator(`[role="option"][data-value=${JSON.stringify(value)}]`);
  322 |   await expect(option).toBeEnabled();
  323 |   await option.click();
  324 |   await expect(control).toHaveAttribute("aria-expanded", "false");
  325 |   await expect(control).toHaveAttribute("data-value", value);
  326 | }
  327 | 
```