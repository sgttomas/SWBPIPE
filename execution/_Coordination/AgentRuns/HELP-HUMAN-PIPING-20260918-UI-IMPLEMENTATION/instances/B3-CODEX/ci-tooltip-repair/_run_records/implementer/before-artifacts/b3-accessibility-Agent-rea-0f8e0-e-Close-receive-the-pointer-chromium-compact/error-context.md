# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b3-accessibility.spec.ts >> Agent reason preserves hover and lets focused page Close receive the pointer
- Location: e2e/b3-accessibility.spec.ts:308:1

# Error details

```
Test timeout of 120000ms exceeded.
```

```
Error: locator.click: Test timeout of 120000ms exceeded.
Call log:
  - waiting for getByTestId('workspace-dock-close')
    - locator resolved to <button type="button" title="Close (⎋)" data-testid="workspace-dock-close">…</button>
  - attempting click action
    2 × waiting for element to be visible, enabled and stable
      - element is visible, enabled and stable
      - scrolling into view if needed
      - done scrolling
      - <span role="tooltip" class="shell-reason" id="agent-strip-reason">Agent: not available yet</span> from <aside aria-label="Agent" class="shell-agent-strip" data-testid="agent-strip">…</aside> subtree intercepts pointer events
    - retrying click action
    - waiting 20ms
    2 × waiting for element to be visible, enabled and stable
      - element is visible, enabled and stable
      - scrolling into view if needed
      - done scrolling
      - <span role="tooltip" class="shell-reason" id="agent-strip-reason">Agent: not available yet</span> from <aside aria-label="Agent" class="shell-agent-strip" data-testid="agent-strip">…</aside> subtree intercepts pointer events
    - retrying click action
      - waiting 100ms
    229 × waiting for element to be visible, enabled and stable
        - element is visible, enabled and stable
        - scrolling into view if needed
        - done scrolling
        - <span role="tooltip" class="shell-reason" id="agent-strip-reason">Agent: not available yet</span> from <aside aria-label="Agent" class="shell-agent-strip" data-testid="agent-strip">…</aside> subtree intercepts pointer events
      - retrying click action
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
      - paragraph [ref=e18]: Invented Utility Loop Preview
    - group "Editing tools" [ref=e19]:
      - button "Undo model edit" [disabled] [ref=e20]:
        - img [ref=e21]
      - button "Redo model edit" [disabled] [ref=e24]:
        - img [ref=e25]
      - button "Select (⎋)" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
    - group "View" [ref=e31]:
      - button "Table" [ref=e33] [cursor=pointer]:
        - img [ref=e34]
        - generic [ref=e36]: Table
      - button "Model" [ref=e38] [cursor=pointer]:
        - img [ref=e39]
        - generic [ref=e42]: Model
      - button "Both" [pressed] [ref=e44] [cursor=pointer]:
        - img [ref=e45]
        - generic [ref=e47]: Both
    - generic [ref=e48]:
      - button "Run" [ref=e49] [cursor=pointer]:
        - img [ref=e50]
        - generic [ref=e52]: Run
      - button "Issues, 5" [ref=e53] [cursor=pointer]:
        - img [ref=e54]
        - generic [ref=e56]: Issues
        - generic [ref=e57]: "5"
    - group "Panels" [ref=e58]:
      - button "Inspector" [ref=e60] [cursor=pointer]:
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
          - button "Model" [ref=e85] [cursor=pointer]:
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
          - button "Libraries" [expanded] [ref=e113] [cursor=pointer]:
            - img [ref=e114]
            - generic [ref=e116]: Libraries
        - listitem [ref=e117]:
          - button "Rules" [ref=e118] [cursor=pointer]:
            - img [ref=e119]
            - generic [ref=e123]: Rules
        - listitem [ref=e124]:
          - button "Issues, 5" [ref=e125] [cursor=pointer]:
            - img [ref=e126]
            - generic [ref=e128]: Issues
            - generic [ref=e129]: "5"
    - generic [ref=e130]:
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
              - button "Tree" [pressed] [ref=e146]:
                - img [ref=e147]
                - text: Tree
              - button "Grid" [ref=e150]:
                - img [ref=e151]
                - text: Grid
            - region "Model tree filtering" [ref=e153]:
              - generic [ref=e154]:
                - img [ref=e155]
                - generic [ref=e158]: Filter model
                - searchbox "Filter model tree" [ref=e159]
              - generic [ref=e160]: 27 of 27 model entities visible
              - button "Clear model tree filter" [disabled] [ref=e161]:
                - img [ref=e162]
            - tree "Model" [ref=e165]:
              - generic [ref=e166]:
                - treeitem "Invented Utility Loop Preview project:invented-loop-01" [level=1] [selected] [ref=e168] [cursor=pointer]:
                  - img [ref=e169]
                  - generic [ref=e173]:
                    - strong [ref=e174]: Invented Utility Loop Preview
                    - generic [ref=e175]: project:invented-loop-01
                - treeitem "Materials" [expanded] [level=1] [ref=e177] [cursor=pointer]:
                  - generic [ref=e178]: ▾
                  - strong [ref=e179]: Materials
                - treeitem "Invented carbon-steel-like material material:invented-carbon-steel" [level=2] [ref=e181] [cursor=pointer]:
                  - img [ref=e182]
                  - generic [ref=e185]:
                    - strong [ref=e186]: Invented carbon-steel-like material
                    - generic [ref=e187]: material:invented-carbon-steel
                - treeitem "Nodes" [expanded] [level=1] [ref=e189] [cursor=pointer]:
                  - generic [ref=e190]: ▾
                  - strong [ref=e191]: Nodes
                - treeitem "Pump nozzle node:N-100" [level=2] [ref=e193] [cursor=pointer]:
                  - img [ref=e194]
                  - generic [ref=e197]:
                    - strong [ref=e198]: Pump nozzle
                    - generic [ref=e199]: node:N-100
                - treeitem "Low point elbow node:N-110" [level=2] [ref=e201] [cursor=pointer]:
                  - img [ref=e202]
                  - generic [ref=e205]:
                    - strong [ref=e206]: Low point elbow
                    - generic [ref=e207]: node:N-110
                - treeitem "Riser elbow node:N-120" [level=2] [ref=e209] [cursor=pointer]:
                  - img [ref=e210]
                  - generic [ref=e213]:
                    - strong [ref=e214]: Riser elbow
                    - generic [ref=e215]: node:N-120
                - treeitem "Rack turn node:N-130" [level=2] [ref=e217] [cursor=pointer]:
                  - img [ref=e218]
                  - generic [ref=e221]:
                    - strong [ref=e222]: Rack turn
                    - generic [ref=e223]: node:N-130
                - treeitem "Terminal tie-in node:N-140" [level=2] [ref=e225] [cursor=pointer]:
                  - img [ref=e226]
                  - generic [ref=e229]:
                    - strong [ref=e230]: Terminal tie-in
                    - generic [ref=e231]: node:N-140
                - treeitem "Pipes" [expanded] [level=1] [ref=e233] [cursor=pointer]:
                  - generic [ref=e234]: ▾
                  - strong [ref=e235]: Pipes
                - treeitem "Pump discharge run pipe:P-100" [level=2] [ref=e237] [cursor=pointer]:
                  - img [ref=e238]
                  - generic [ref=e242]:
                    - strong [ref=e243]: Pump discharge run
                    - generic [ref=e244]: pipe:P-100
                - treeitem "Vertical riser pipe:P-110" [level=2] [ref=e246] [cursor=pointer]:
                  - img [ref=e247]
                  - generic [ref=e251]:
                    - strong [ref=e252]: Vertical riser
                    - generic [ref=e253]: pipe:P-110
                - treeitem "Rack span pipe:P-120" [level=2] [ref=e255] [cursor=pointer]:
                  - img [ref=e256]
                  - generic [ref=e260]:
                    - strong [ref=e261]: Rack span
                    - generic [ref=e262]: pipe:P-120
                - treeitem "Tie-in rise pipe:P-130" [level=2] [ref=e264] [cursor=pointer]:
                  - img [ref=e265]
                  - generic [ref=e269]:
                    - strong [ref=e270]: Tie-in rise
                    - generic [ref=e271]: pipe:P-130
                - treeitem "Supports" [expanded] [level=1] [ref=e273] [cursor=pointer]:
                  - generic [ref=e274]: ▾
                  - strong [ref=e275]: Supports
                - treeitem "Anchor at pump nozzle support:S-100" [level=2] [ref=e277] [cursor=pointer]:
                  - img [ref=e278]
                  - generic [ref=e281]:
                    - strong [ref=e282]: Anchor at pump nozzle
                    - generic [ref=e283]: support:S-100
                - treeitem "Guide on riser support:S-120" [level=2] [ref=e285] [cursor=pointer]:
                  - img [ref=e286]
                  - generic [ref=e289]:
                    - strong [ref=e290]: Guide on riser
                    - generic [ref=e291]: support:S-120
                - treeitem "Rack shoe support:S-130" [level=2] [ref=e293] [cursor=pointer]:
                  - img [ref=e294]
                  - generic [ref=e297]:
                    - strong [ref=e298]: Rack shoe
                    - generic [ref=e299]: support:S-130
                - treeitem "Preview one-way terminal stop support:NL-140" [level=2] [ref=e301] [cursor=pointer]:
                  - img [ref=e302]
                  - generic [ref=e305]:
                    - strong [ref=e306]: Preview one-way terminal stop
                    - generic [ref=e307]: support:NL-140
                - treeitem "Preview sliding-friction rack shoe support:NL-130-FRIC" [level=2] [ref=e309] [cursor=pointer]:
                  - img [ref=e310]
                  - generic [ref=e313]:
                    - strong [ref=e314]: Preview sliding-friction rack shoe
                    - generic [ref=e315]: support:NL-130-FRIC
                - treeitem "Invented variable spring hanger support:SH-140" [level=2] [ref=e317] [cursor=pointer]:
                  - img [ref=e318]
                  - generic [ref=e321]:
                    - strong [ref=e322]: Invented variable spring hanger
                    - generic [ref=e323]: support:SH-140
                - treeitem "Invented constant-effort support support:CE-120" [level=2] [ref=e325] [cursor=pointer]:
                  - img [ref=e326]
                  - generic [ref=e329]:
                    - strong [ref=e330]: Invented constant-effort support
                    - generic [ref=e331]: support:CE-120
                - treeitem "Components" [expanded] [level=1] [ref=e333] [cursor=pointer]:
                  - generic [ref=e334]: ▾
                  - strong [ref=e335]: Components
                - treeitem "Invented elbow marker component:C-110" [level=2] [ref=e337] [cursor=pointer]:
                  - img [ref=e338]
                  - generic [ref=e345]:
                    - strong [ref=e346]: Invented elbow marker
                    - generic [ref=e347]: component:C-110
                - treeitem "Invented branch connection marker component:C-120" [level=2] [ref=e349] [cursor=pointer]:
                  - img [ref=e350]
                  - generic [ref=e357]:
                    - strong [ref=e358]: Invented branch connection marker
                    - generic [ref=e359]: component:C-120
                - treeitem "Invented semi-rigid valve marker component:C-130" [level=2] [ref=e361] [cursor=pointer]:
                  - img [ref=e362]
                  - generic [ref=e369]:
                    - strong [ref=e370]: Invented semi-rigid valve marker
                    - generic [ref=e371]: component:C-130
                - treeitem "Invented tie-in marker component:C-140" [level=2] [ref=e373] [cursor=pointer]:
                  - img [ref=e374]
                  - generic [ref=e381]:
                    - strong [ref=e382]: Invented tie-in marker
                    - generic [ref=e383]: component:C-140
                - treeitem "Invented expansion joint marker component:C-150" [level=2] [ref=e385] [cursor=pointer]:
                  - img [ref=e386]
                  - generic [ref=e393]:
                    - strong [ref=e394]: Invented expansion joint marker
                    - generic [ref=e395]: component:C-150
                - treeitem "Load Cases" [expanded] [level=1] [ref=e397] [cursor=pointer]:
                  - generic [ref=e398]: ▾
                  - strong [ref=e399]: Load Cases
                - treeitem "Invented operating gravity and pressure preview load:L-100" [level=2] [ref=e401] [cursor=pointer]:
                  - img [ref=e402]
                  - generic [ref=e404]:
                    - strong [ref=e405]: Invented operating gravity and pressure preview
                    - generic [ref=e406]: load:L-100
                - treeitem "Invented alternate gravity and pressure preview load:L-200" [level=2] [ref=e408] [cursor=pointer]:
                  - img [ref=e409]
                  - generic [ref=e411]:
                    - strong [ref=e412]: Invented alternate gravity and pressure preview
                    - generic [ref=e413]: load:L-200
                - treeitem "Combinations" [expanded] [level=1] [ref=e415] [cursor=pointer]:
                  - generic [ref=e416]: ▾
                  - strong [ref=e417]: Combinations
                - treeitem "Invented explicit operating plus alternate preview combination:C-OPER-ALT" [level=2] [ref=e419] [cursor=pointer]:
                  - img [ref=e420]
                  - generic [ref=e424]:
                    - strong [ref=e425]: Invented explicit operating plus alternate preview
                    - generic [ref=e426]: combination:C-OPER-ALT
                - treeitem "Diagnostics" [expanded] [level=1] [ref=e428] [cursor=pointer]:
                  - generic [ref=e429]: ▾
                  - strong [ref=e430]: Diagnostics
                - treeitem "RULE_INPUTS_MISSING RULE_INPUTS_MISSING:0" [level=2] [ref=e432] [cursor=pointer]:
                  - img [ref=e433]
                  - generic [ref=e436]:
                    - strong [ref=e437]: RULE_INPUTS_MISSING
                    - generic [ref=e438]: RULE_INPUTS_MISSING:0
        - separator "Resize table and canvas" [ref=e439]
        - generic [ref=e441]:
          - generic [ref=e442]:
            - group "Viewport controls" [ref=e443]:
              - generic [ref=e444]: 3D Centerline
              - group "Viewport deformation overlay status" [ref=e445]:
                - generic "Deformation · unavailable" [ref=e446] [cursor=pointer]
              - group "Viewport display toggles" [ref=e447]:
                - button "Labels" [pressed] [ref=e448]
                - button "Loads" [pressed] [ref=e449]
                - button "Grid" [pressed] [ref=e450]
              - group "Viewport selection tools" [ref=e451]:
                - button "Box Select" [ref=e452]
                - generic [ref=e453]:
                  - generic [ref=e454]: Selection filter
                  - combobox "Selection filter" [ref=e455]:
                    - option "All" [selected]
                    - option "Pipes"
                    - option "Nodes"
                    - option "Supports"
                    - option "Components"
                - button "Hide" [disabled] [ref=e456]
                - button "Isolate" [disabled] [ref=e457]
                - button "Show All" [disabled] [ref=e458]
                - button "Fit Model" [ref=e459]
                - button "Fit Visible" [ref=e460]
                - button "Fit Selection" [disabled] [ref=e461]
              - group "Viewport geometry" [ref=e462]:
                - button "Schematic" [pressed] [ref=e463]
                - button "Actual OD" [ref=e464]
                - button "Measure" [ref=e465]
            - generic "Viewport status" [ref=e466]:
              - 'generic "Selected project: project:invented-loop-01" [ref=e467]': "Selected: project:invented-loop-01"
              - status "Schematic centerline geometry" [ref=e468]
              - status "View command status" [ref=e469]: No view command dispatched.
          - generic [ref=e470]:
            - generic "Three.js pipe centerline viewport" [ref=e471]
            - generic "Viewport entity selection":
              - button "Select Vertical riser in viewport" [ref=e473] [cursor=pointer]:
                - img [ref=e474]
                - generic [ref=e478]: P-110
              - button "Select Invented branch connection marker in viewport" [ref=e479] [cursor=pointer]:
                - img [ref=e480]
                - generic [ref=e483]: C-120
              - button "Select Low point elbow in viewport" [ref=e484] [cursor=pointer]:
                - img [ref=e485]
                - generic [ref=e488]: N-110
              - button "Select Riser elbow in viewport" [ref=e489] [cursor=pointer]:
                - img [ref=e490]
                - generic [ref=e493]: N-120
              - button "Select Rack span in viewport" [ref=e494] [cursor=pointer]:
                - img [ref=e495]
                - generic [ref=e499]: P-120
              - button "Select Tie-in rise in viewport" [ref=e500] [cursor=pointer]:
                - img [ref=e501]
                - generic [ref=e505]: P-130
              - button "Select Invented semi-rigid valve marker in viewport" [ref=e506] [cursor=pointer]:
                - img [ref=e507]
                - generic [ref=e510]: C-130
              - button "Select Pump nozzle in viewport" [ref=e511] [cursor=pointer]:
                - img [ref=e512]
                - generic [ref=e515]: N-100
              - button "Select Rack turn in viewport" [ref=e516] [cursor=pointer]:
                - img [ref=e517]
                - generic [ref=e520]: N-130
              - button "Select Terminal tie-in in viewport" [ref=e521] [cursor=pointer]:
                - img [ref=e522]
                - generic [ref=e525]: N-140
              - button "Select Preview one-way terminal stop in viewport" [ref=e526] [cursor=pointer]:
                - img [ref=e527]
                - generic [ref=e530]: NL-140
              - button "Select Anchor at pump nozzle in viewport" [ref=e531] [cursor=pointer]:
                - img [ref=e532]
                - generic [ref=e535]: S-100
            - img "Orientation gizmo showing X, Y, Z axes"
            - generic "View controls" [ref=e536]:
              - button "Front" [ref=e537] [cursor=pointer]
              - button "Top" [ref=e538] [cursor=pointer]
              - button "Isometric" [pressed] [ref=e539] [cursor=pointer]
            - generic:
              - generic: 1 m
          - region "Command and selection bar" [ref=e540]:
            - generic "Object creation tools" [ref=e541]:
              - button "Node" [ref=e542] [cursor=pointer]:
                - img [ref=e543]
                - text: Node
              - button "Pipe" [ref=e545] [cursor=pointer]:
                - img [ref=e546]
                - text: Pipe
              - button "Support" [ref=e550] [cursor=pointer]:
                - img [ref=e551]
                - text: Support
              - button "Component" [ref=e554] [cursor=pointer]:
                - img [ref=e555]
                - text: Component
              - button "Load" [ref=e558] [cursor=pointer]:
                - img [ref=e559]
                - text: Load
            - generic "Model focus" [ref=e561]: Select
            - group [ref=e562]:
              - generic "Selection & navigation" [ref=e563] [cursor=pointer]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
      - region "Workspace sections" [ref=e564]:
        - generic [ref=e565]:
          - heading "Libraries" [level=2] [ref=e566]
          - button "Close" [ref=e567] [cursor=pointer]:
            - img [ref=e568]
            - text: Close
        - region "Libraries section" [ref=e572]:
          - region "Library manager" [ref=e573]:
            - generic [ref=e574]:
              - img [ref=e575]
              - text: Private Library Manager (local-only)
            - generic [ref=e578]: "Project scope: project:invented-loop-01; storage: local SQLite only."
            - generic [ref=e579]:
              - generic [ref=e580]: Library kind
              - combobox "Library kind" [ref=e581]:
                - option "Material library" [selected]
                - option "Section library"
                - option "Component library"
                - option "Hanger library"
              - generic [ref=e582]: Intended visibility (validation preview)
              - combobox "Intended visibility (validation preview)" [ref=e583]:
                - option "Private (local-only)" [selected]
                - option "Public (validation preview)"
              - generic [ref=e584]: Visibility shapes the validation preview only. Save always persists to the private local store and stores only an accepted private import (DEC-036).
            - generic [ref=e585]:
              - button "Load invented sample" [ref=e586]
              - button "Refresh local list" [ref=e587]
            - generic [ref=e589]: Local library list not refreshed for this project yet.
            - generic [ref=e590]:
              - generic [ref=e591]: Library import document JSON (an already-parsed material/section/component library document with provenance — this manager validates and stores it; it does not parse external file formats)
              - textbox "Library import document JSON (an already-parsed material/section/component library document with provenance — this manager validates and stores it; it does not parse external file formats)" [disabled] [ref=e592]:
                - /placeholder: No import document. Load an invented sample or paste a library document.
              - generic [ref=e593]:
                - button "Validate import" [disabled] [ref=e594]
                - button "Save to local store" [disabled] [ref=e595]
                - button "Discard import document" [disabled] [ref=e596]
              - generic [ref=e597]: No library-import action has run in this session.
            - generic [ref=e598]:
              - img [ref=e599]
              - text: Imported private libraries stay in local project storage only — never committed to the repository, transmitted, or bundled into public artifacts. Only an accepted private import is stored; a suspected-protected or otherwise-blocked import is refused, not stored (DEC-036). Import validation reports software findings over an already-parsed payload; no protected standards content — code-specific data is user-supplied.
    - complementary "Agent" [ref=e601]:
      - generic [ref=e602]:
        - button "Agent" [disabled] [active] [ref=e603]:
          - img [ref=e604]
          - generic [ref=e607]: Agent
        - 'tooltip "Agent: not available yet" [ref=e608]'
  - generic "Workspace status" [ref=e609]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e610] [cursor=pointer]:
      - img [ref=e611]
      - text: 5 Issues
    - generic "Selection" [ref=e613]: "project: project:invented-loop-01"
    - generic "Display units" [ref=e614]: Entered
    - button "About SWBPIPE…" [ref=e615] [cursor=pointer]:
      - img [ref=e616]
```

# Test source

```ts
  223 |     expect(placement.topmost).toBe(true);
  224 |     expect(placement.rect.left).toBeGreaterThanOrEqual(0);
  225 |     expect(placement.rect.right).toBeLessThanOrEqual(placement.width);
  226 |     expect(placement.rect.top).toBeGreaterThanOrEqual(0);
  227 |     expect(placement.rect.bottom).toBeLessThanOrEqual(placement.height);
  228 |     await page.keyboard.press("ArrowDown");
  229 |     await expect(unit).toHaveAttribute("data-value", "m");
  230 |     await page.keyboard.press("Escape");
  231 |     await expect(list).toHaveCount(0);
  232 |     await expect(unit).toBeFocused();
  233 |     await expect(unit).toHaveAttribute("data-value", "m");
  234 |     await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
  235 |     await page.keyboard.press("Escape");
  236 |     await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  237 |     await expect(page.getByTestId("toggle-inspector")).toBeFocused();
  238 |     await page.getByTestId("toggle-inspector").click();
  239 |     await expect(draft).toHaveValue("Retained popup draft");
  240 |     expect(await draft.evaluate((el, old) => el === old, retained)).toBe(true);
  241 | 
  242 |     // Silent same-value commits use the same explicit lifecycle as changed commits.
  243 |     await unit.click();
  244 |     await list.locator('[role="option"][data-value="m"]').click();
  245 |     await expect(unit).toHaveAttribute("aria-expanded", "false");
  246 |     await expect(unit).toBeFocused();
  247 |     await page.keyboard.press("Escape");
  248 |     await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  249 |     await page.getByTestId("toggle-inspector").click();
  250 |     await unit.focus();
  251 |     await page.keyboard.press("Space");
  252 |     await page.keyboard.press("Enter");
  253 |     await expect(unit).toHaveAttribute("aria-expanded", "false");
  254 |     await expect(unit).toHaveAttribute("data-value", "m");
  255 |     // Browser preview has entered-unit metadata only; alternate unit catalog
  256 |     // choices are an actual Tauri check, not injected into this browser scenario.
  257 |     await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  258 |     await expect(page.getByTestId("workspace-redo")).toBeDisabled();
  259 |     await witness(info, `compact-${layout}-ax`, await nativeAXNames(page));
  260 | 
  261 |     // A retained control's open popup is dismissed on stage change, without remounting its draft.
  262 |     await unit.click();
  263 |     await page.getByTestId("rail-stage-loads").click();
  264 |     await expect(list).toHaveCount(0);
  265 |     await expect(page.getByTestId("rail-stage-loads")).toBeFocused();
  266 |     await page.getByTestId("rail-stage-model").click();
  267 |     await expect(draft).toHaveValue("Retained popup draft");
  268 |     expect(await draft.evaluate((el, old) => el === old, retained)).toBe(true);
  269 | 
  270 |     // Exercise a changed value using options actually available in browser mode.
  271 |     await page.getByTestId("workspace-select").click();
  272 |     await selectTreeEntity(page, "node", "node:N-100");
  273 |     await startPropertyTaskFromCurrentSelection(page, "node", "node:N-100");
  274 |     const field = page.getByTestId("editor-intent-field");
  275 |     await selectCompactOption(field, "position.x");
  276 |     await expect(field).toHaveAttribute("data-value", "position.x");
  277 |     await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  278 |     await expect(page.getByTestId("workspace-redo")).toBeDisabled();
  279 |   });
  280 | }
  281 | 
  282 | test("support family popup dismissal does not invent a missing engineering choice", async ({ page }) => {
  283 |   await page.goto("/");
  284 |   await selectTreeEntity(page, "node", "node:N-100");
  285 |   await page.getByTestId("toolkit-entry").click();
  286 |   await page.getByTestId("toolkit-supports.restraint").click();
  287 |   const form = page.getByRole("region", { name: "Support configuration", exact: true });
  288 |   const family = form.getByRole("combobox", { name: "Support family", exact: true });
  289 |   await expect(family).toHaveAttribute("data-value", "");
  290 |   await expect(family).toContainText("Not provided (preserved)");
  291 |   await family.click();
  292 |   await expect(family).toHaveAttribute("aria-expanded", "true");
  293 |   await page.keyboard.press("Tab");
  294 |   await expect(family).toHaveAttribute("aria-expanded", "false");
  295 |   await expect(family).toHaveAttribute("data-value", "");
  296 |   await form.getByRole("textbox", { name: "Support ID", exact: true }).fill("support:compact-preserved");
  297 |   await form.getByRole("textbox", { name: "Support label", exact: true }).fill("Explicit source preservation test");
  298 |   await form.getByRole("textbox", { name: "Support provenance", exact: true }).fill("Explicit browser test draft");
  299 |   await family.click();
  300 |   await form.getByRole("button", { name: "Queue support creation", exact: true }).click();
  301 |   await expect(form).toContainText("Support creation queued for validation and review.");
  302 |   await expect(family).toHaveAttribute("data-value", "");
  303 |   await expect(family).toContainText("Not provided (preserved)");
  304 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  305 | });
  306 | 
  307 | 
  308 | test("Agent reason preserves hover and lets focused page Close receive the pointer", async ({ page }, info) => {
  309 |   await page.goto("/");
  310 |   await openWorkspaceSection(page, "libraries");
  311 |   const agent = page.getByTestId("agent-strip-open");
  312 |   const reason = page.locator("#agent-strip-reason");
  313 |   const close = page.getByTestId("workspace-dock-close");
  314 |   await agent.focus();
  315 |   await expect(reason).toBeVisible();
  316 |   await reason.hover();
  317 |   await expect(reason).toBeVisible();
  318 |   await witness(info, "agent-reason-close-hit", await close.evaluate(el => {
  319 |     const r = el.getBoundingClientRect();
  320 |     const hit = document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2);
  321 |     return { close: r.toJSON(), hit: hit?.outerHTML, ownsPointer: el.contains(hit), focused: document.activeElement?.outerHTML };
  322 |   }));
> 323 |   await close.click();
      |               ^ Error: locator.click: Test timeout of 120000ms exceeded.
  324 |   await expect(close).toHaveCount(0);
  325 | });
  326 | 
```