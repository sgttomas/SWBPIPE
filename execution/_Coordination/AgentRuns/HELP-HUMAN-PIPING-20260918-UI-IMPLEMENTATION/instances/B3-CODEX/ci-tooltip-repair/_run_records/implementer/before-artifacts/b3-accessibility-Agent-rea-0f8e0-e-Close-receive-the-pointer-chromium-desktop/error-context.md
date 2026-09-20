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
    228 × waiting for element to be visible, enabled and stable
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
      - button "Select" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
        - generic [ref=e31]: Select
    - group "View" [ref=e32]:
      - button "Table" [ref=e34] [cursor=pointer]:
        - img [ref=e35]
        - generic [ref=e37]: Table
      - button "Model" [ref=e39] [cursor=pointer]:
        - img [ref=e40]
        - generic [ref=e43]: Model
      - button "Both" [pressed] [ref=e45] [cursor=pointer]:
        - img [ref=e46]
        - generic [ref=e48]: Both
    - generic [ref=e49]:
      - button "Run" [ref=e50] [cursor=pointer]:
        - img [ref=e51]
        - generic [ref=e53]: Run
      - button "Issues, 5" [ref=e54] [cursor=pointer]:
        - img [ref=e55]
        - generic [ref=e57]: Issues
        - generic [ref=e58]: "5"
    - group "Panels" [ref=e59]:
      - button "Inspector" [ref=e61] [cursor=pointer]:
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
          - button "Model" [ref=e88] [cursor=pointer]:
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
          - button "Libraries" [expanded] [ref=e116] [cursor=pointer]:
            - img [ref=e117]
            - generic [ref=e119]: Libraries
        - listitem [ref=e120]:
          - button "Rules" [ref=e121] [cursor=pointer]:
            - img [ref=e122]
            - generic [ref=e126]: Rules
        - listitem [ref=e127]:
          - button "Issues, 5" [ref=e128] [cursor=pointer]:
            - img [ref=e129]
            - generic [ref=e131]: Issues
            - generic [ref=e132]: "5"
    - generic [ref=e133]:
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
              - button "Tree" [pressed] [ref=e149]:
                - img [ref=e150]
                - text: Tree
              - button "Grid" [ref=e153]:
                - img [ref=e154]
                - text: Grid
            - region "Model tree filtering" [ref=e156]:
              - generic [ref=e157]:
                - img [ref=e158]
                - generic [ref=e161]: Filter model
                - searchbox "Filter model tree" [ref=e162]
              - generic [ref=e163]: 27 of 27 model entities visible
              - button "Clear model tree filter" [disabled] [ref=e164]:
                - img [ref=e165]
            - tree "Model" [ref=e168]:
              - generic [ref=e169]:
                - treeitem "Invented Utility Loop Preview project:invented-loop-01" [level=1] [selected] [ref=e171] [cursor=pointer]:
                  - img [ref=e172]
                  - generic [ref=e176]:
                    - strong [ref=e177]: Invented Utility Loop Preview
                    - generic [ref=e178]: project:invented-loop-01
                - treeitem "Materials" [expanded] [level=1] [ref=e180] [cursor=pointer]:
                  - generic [ref=e181]: ▾
                  - strong [ref=e182]: Materials
                - treeitem "Invented carbon-steel-like material material:invented-carbon-steel" [level=2] [ref=e184] [cursor=pointer]:
                  - img [ref=e185]
                  - generic [ref=e188]:
                    - strong [ref=e189]: Invented carbon-steel-like material
                    - generic [ref=e190]: material:invented-carbon-steel
                - treeitem "Nodes" [expanded] [level=1] [ref=e192] [cursor=pointer]:
                  - generic [ref=e193]: ▾
                  - strong [ref=e194]: Nodes
                - treeitem "Pump nozzle node:N-100" [level=2] [ref=e196] [cursor=pointer]:
                  - img [ref=e197]
                  - generic [ref=e200]:
                    - strong [ref=e201]: Pump nozzle
                    - generic [ref=e202]: node:N-100
                - treeitem "Low point elbow node:N-110" [level=2] [ref=e204] [cursor=pointer]:
                  - img [ref=e205]
                  - generic [ref=e208]:
                    - strong [ref=e209]: Low point elbow
                    - generic [ref=e210]: node:N-110
                - treeitem "Riser elbow node:N-120" [level=2] [ref=e212] [cursor=pointer]:
                  - img [ref=e213]
                  - generic [ref=e216]:
                    - strong [ref=e217]: Riser elbow
                    - generic [ref=e218]: node:N-120
                - treeitem "Rack turn node:N-130" [level=2] [ref=e220] [cursor=pointer]:
                  - img [ref=e221]
                  - generic [ref=e224]:
                    - strong [ref=e225]: Rack turn
                    - generic [ref=e226]: node:N-130
                - treeitem "Terminal tie-in node:N-140" [level=2] [ref=e228] [cursor=pointer]:
                  - img [ref=e229]
                  - generic [ref=e232]:
                    - strong [ref=e233]: Terminal tie-in
                    - generic [ref=e234]: node:N-140
                - treeitem "Pipes" [expanded] [level=1] [ref=e236] [cursor=pointer]:
                  - generic [ref=e237]: ▾
                  - strong [ref=e238]: Pipes
                - treeitem "Pump discharge run pipe:P-100" [level=2] [ref=e240] [cursor=pointer]:
                  - img [ref=e241]
                  - generic [ref=e245]:
                    - strong [ref=e246]: Pump discharge run
                    - generic [ref=e247]: pipe:P-100
                - treeitem "Vertical riser pipe:P-110" [level=2] [ref=e249] [cursor=pointer]:
                  - img [ref=e250]
                  - generic [ref=e254]:
                    - strong [ref=e255]: Vertical riser
                    - generic [ref=e256]: pipe:P-110
                - treeitem "Rack span pipe:P-120" [level=2] [ref=e258] [cursor=pointer]:
                  - img [ref=e259]
                  - generic [ref=e263]:
                    - strong [ref=e264]: Rack span
                    - generic [ref=e265]: pipe:P-120
                - treeitem "Tie-in rise pipe:P-130" [level=2] [ref=e267] [cursor=pointer]:
                  - img [ref=e268]
                  - generic [ref=e272]:
                    - strong [ref=e273]: Tie-in rise
                    - generic [ref=e274]: pipe:P-130
                - treeitem "Supports" [expanded] [level=1] [ref=e276] [cursor=pointer]:
                  - generic [ref=e277]: ▾
                  - strong [ref=e278]: Supports
                - treeitem "Anchor at pump nozzle support:S-100" [level=2] [ref=e280] [cursor=pointer]:
                  - img [ref=e281]
                  - generic [ref=e284]:
                    - strong [ref=e285]: Anchor at pump nozzle
                    - generic [ref=e286]: support:S-100
                - treeitem "Guide on riser support:S-120" [level=2] [ref=e288] [cursor=pointer]:
                  - img [ref=e289]
                  - generic [ref=e292]:
                    - strong [ref=e293]: Guide on riser
                    - generic [ref=e294]: support:S-120
                - treeitem "Rack shoe support:S-130" [level=2] [ref=e296] [cursor=pointer]:
                  - img [ref=e297]
                  - generic [ref=e300]:
                    - strong [ref=e301]: Rack shoe
                    - generic [ref=e302]: support:S-130
                - treeitem "Preview one-way terminal stop support:NL-140" [level=2] [ref=e304] [cursor=pointer]:
                  - img [ref=e305]
                  - generic [ref=e308]:
                    - strong [ref=e309]: Preview one-way terminal stop
                    - generic [ref=e310]: support:NL-140
                - treeitem "Preview sliding-friction rack shoe support:NL-130-FRIC" [level=2] [ref=e312] [cursor=pointer]:
                  - img [ref=e313]
                  - generic [ref=e316]:
                    - strong [ref=e317]: Preview sliding-friction rack shoe
                    - generic [ref=e318]: support:NL-130-FRIC
                - treeitem "Invented variable spring hanger support:SH-140" [level=2] [ref=e320] [cursor=pointer]:
                  - img [ref=e321]
                  - generic [ref=e324]:
                    - strong [ref=e325]: Invented variable spring hanger
                    - generic [ref=e326]: support:SH-140
                - treeitem "Invented constant-effort support support:CE-120" [level=2] [ref=e328] [cursor=pointer]:
                  - img [ref=e329]
                  - generic [ref=e332]:
                    - strong [ref=e333]: Invented constant-effort support
                    - generic [ref=e334]: support:CE-120
                - treeitem "Components" [expanded] [level=1] [ref=e336] [cursor=pointer]:
                  - generic [ref=e337]: ▾
                  - strong [ref=e338]: Components
                - treeitem "Invented elbow marker component:C-110" [level=2] [ref=e340] [cursor=pointer]:
                  - img [ref=e341]
                  - generic [ref=e348]:
                    - strong [ref=e349]: Invented elbow marker
                    - generic [ref=e350]: component:C-110
                - treeitem "Invented branch connection marker component:C-120" [level=2] [ref=e352] [cursor=pointer]:
                  - img [ref=e353]
                  - generic [ref=e360]:
                    - strong [ref=e361]: Invented branch connection marker
                    - generic [ref=e362]: component:C-120
                - treeitem "Invented semi-rigid valve marker component:C-130" [level=2] [ref=e364] [cursor=pointer]:
                  - img [ref=e365]
                  - generic [ref=e372]:
                    - strong [ref=e373]: Invented semi-rigid valve marker
                    - generic [ref=e374]: component:C-130
                - treeitem "Invented tie-in marker component:C-140" [level=2] [ref=e376] [cursor=pointer]:
                  - img [ref=e377]
                  - generic [ref=e384]:
                    - strong [ref=e385]: Invented tie-in marker
                    - generic [ref=e386]: component:C-140
                - treeitem "Invented expansion joint marker component:C-150" [level=2] [ref=e388] [cursor=pointer]:
                  - img [ref=e389]
                  - generic [ref=e396]:
                    - strong [ref=e397]: Invented expansion joint marker
                    - generic [ref=e398]: component:C-150
                - treeitem "Load Cases" [expanded] [level=1] [ref=e400] [cursor=pointer]:
                  - generic [ref=e401]: ▾
                  - strong [ref=e402]: Load Cases
                - treeitem "Invented operating gravity and pressure preview load:L-100" [level=2] [ref=e404] [cursor=pointer]:
                  - img [ref=e405]
                  - generic [ref=e407]:
                    - strong [ref=e408]: Invented operating gravity and pressure preview
                    - generic [ref=e409]: load:L-100
                - treeitem "Invented alternate gravity and pressure preview load:L-200" [level=2] [ref=e411] [cursor=pointer]:
                  - img [ref=e412]
                  - generic [ref=e414]:
                    - strong [ref=e415]: Invented alternate gravity and pressure preview
                    - generic [ref=e416]: load:L-200
                - treeitem "Combinations" [expanded] [level=1] [ref=e418] [cursor=pointer]:
                  - generic [ref=e419]: ▾
                  - strong [ref=e420]: Combinations
                - treeitem "Invented explicit operating plus alternate preview combination:C-OPER-ALT" [level=2] [ref=e422] [cursor=pointer]:
                  - img [ref=e423]
                  - generic [ref=e427]:
                    - strong [ref=e428]: Invented explicit operating plus alternate preview
                    - generic [ref=e429]: combination:C-OPER-ALT
                - treeitem "Diagnostics" [expanded] [level=1] [ref=e431] [cursor=pointer]:
                  - generic [ref=e432]: ▾
                  - strong [ref=e433]: Diagnostics
                - treeitem "RULE_INPUTS_MISSING RULE_INPUTS_MISSING:0" [level=2] [ref=e435] [cursor=pointer]:
                  - img [ref=e436]
                  - generic [ref=e439]:
                    - strong [ref=e440]: RULE_INPUTS_MISSING
                    - generic [ref=e441]: RULE_INPUTS_MISSING:0
        - separator "Resize table and canvas" [ref=e442]
        - generic [ref=e444]:
          - generic [ref=e445]:
            - group "Viewport controls" [ref=e446]:
              - generic [ref=e447]: 3D Centerline
              - group "Viewport deformation overlay status" [ref=e448]:
                - generic "Deformation · unavailable" [ref=e449] [cursor=pointer]
              - group "Viewport display toggles" [ref=e450]:
                - button "Labels" [pressed] [ref=e451]
                - button "Loads" [pressed] [ref=e452]
                - button "Grid" [pressed] [ref=e453]
              - group "Viewport selection tools" [ref=e454]:
                - button "Box Select" [ref=e455]
                - generic [ref=e456]:
                  - generic [ref=e457]: Selection filter
                  - combobox "Selection filter" [ref=e458]:
                    - option "All" [selected]
                    - option "Pipes"
                    - option "Nodes"
                    - option "Supports"
                    - option "Components"
                - button "Hide" [disabled] [ref=e459]
                - button "Isolate" [disabled] [ref=e460]
                - button "Show All" [disabled] [ref=e461]
                - button "Fit Model" [ref=e462]
                - button "Fit Visible" [ref=e463]
                - button "Fit Selection" [disabled] [ref=e464]
              - group "Viewport geometry" [ref=e465]:
                - button "Schematic" [pressed] [ref=e466]
                - button "Actual OD" [ref=e467]
                - button "Measure" [ref=e468]
            - generic "Viewport status" [ref=e469]:
              - 'generic "Selected project: project:invented-loop-01" [ref=e470]': "Selected: project:invented-loop-01"
              - status "Schematic centerline geometry" [ref=e471]
              - status "View command status" [ref=e472]: No view command dispatched.
          - generic [ref=e473]:
            - generic "Three.js pipe centerline viewport" [ref=e474]
            - generic "Viewport entity selection":
              - button "Select Vertical riser in viewport" [ref=e476] [cursor=pointer]:
                - img [ref=e477]
                - generic [ref=e481]: P-110
              - button "Select Invented branch connection marker in viewport" [ref=e482] [cursor=pointer]:
                - img [ref=e483]
                - generic [ref=e486]: C-120
              - button "Select Low point elbow in viewport" [ref=e487] [cursor=pointer]:
                - img [ref=e488]
                - generic [ref=e491]: N-110
              - button "Select Riser elbow in viewport" [ref=e492] [cursor=pointer]:
                - img [ref=e493]
                - generic [ref=e496]: N-120
              - button "Select Rack span in viewport" [ref=e497] [cursor=pointer]:
                - img [ref=e498]
                - generic [ref=e502]: P-120
              - button "Select Pump discharge run in viewport" [ref=e503] [cursor=pointer]:
                - img [ref=e504]
                - generic [ref=e508]: P-100
              - button "Select Tie-in rise in viewport" [ref=e509] [cursor=pointer]:
                - img [ref=e510]
                - generic [ref=e514]: P-130
              - button "Select Invented semi-rigid valve marker in viewport" [ref=e515] [cursor=pointer]:
                - img [ref=e516]
                - generic [ref=e519]: C-130
              - button "Select Pump nozzle in viewport" [ref=e520] [cursor=pointer]:
                - img [ref=e521]
                - generic [ref=e524]: N-100
              - button "Select Rack turn in viewport" [ref=e525] [cursor=pointer]:
                - img [ref=e526]
                - generic [ref=e529]: N-130
              - button "Select Terminal tie-in in viewport" [ref=e530] [cursor=pointer]:
                - img [ref=e531]
                - generic [ref=e534]: N-140
              - button "Select Preview sliding-friction rack shoe in viewport" [ref=e535] [cursor=pointer]:
                - img [ref=e536]
                - generic [ref=e539]: NL-130-FRIC
              - button "Select Preview one-way terminal stop in viewport" [ref=e540] [cursor=pointer]:
                - img [ref=e541]
                - generic [ref=e544]: NL-140
              - button "Select Anchor at pump nozzle in viewport" [ref=e545] [cursor=pointer]:
                - img [ref=e546]
                - generic [ref=e549]: S-100
            - img "Orientation gizmo showing X, Y, Z axes"
            - generic "View controls" [ref=e550]:
              - button "Front" [ref=e551] [cursor=pointer]
              - button "Top" [ref=e552] [cursor=pointer]
              - button "Isometric" [pressed] [ref=e553] [cursor=pointer]
            - generic:
              - generic: 1 m
          - region "Command and selection bar" [ref=e554]:
            - generic "Object creation tools" [ref=e555]:
              - button "Node" [ref=e556] [cursor=pointer]:
                - img [ref=e557]
                - text: Node
              - button "Pipe" [ref=e559] [cursor=pointer]:
                - img [ref=e560]
                - text: Pipe
              - button "Support" [ref=e564] [cursor=pointer]:
                - img [ref=e565]
                - text: Support
              - button "Component" [ref=e568] [cursor=pointer]:
                - img [ref=e569]
                - text: Component
              - button "Load" [ref=e572] [cursor=pointer]:
                - img [ref=e573]
                - text: Load
            - generic "Model focus" [ref=e575]: Select
            - group [ref=e576]:
              - generic "Selection & navigation" [ref=e577] [cursor=pointer]
        - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
      - region "Workspace sections" [ref=e578]:
        - generic [ref=e579]:
          - heading "Libraries" [level=2] [ref=e580]
          - button "Close" [ref=e581] [cursor=pointer]:
            - img [ref=e582]
            - text: Close
        - region "Libraries section" [ref=e586]:
          - region "Library manager" [ref=e587]:
            - generic [ref=e588]:
              - img [ref=e589]
              - text: Private Library Manager (local-only)
            - generic [ref=e592]: "Project scope: project:invented-loop-01; storage: local SQLite only."
            - generic [ref=e593]:
              - generic [ref=e594]: Library kind
              - combobox "Library kind" [ref=e595]:
                - option "Material library" [selected]
                - option "Section library"
                - option "Component library"
                - option "Hanger library"
              - generic [ref=e596]: Intended visibility (validation preview)
              - combobox "Intended visibility (validation preview)" [ref=e597]:
                - option "Private (local-only)" [selected]
                - option "Public (validation preview)"
              - generic [ref=e598]: Visibility shapes the validation preview only. Save always persists to the private local store and stores only an accepted private import (DEC-036).
            - generic [ref=e599]:
              - button "Load invented sample" [ref=e600]
              - button "Refresh local list" [ref=e601]
            - generic [ref=e603]: Local library list not refreshed for this project yet.
            - generic [ref=e604]:
              - generic [ref=e605]: Library import document JSON (an already-parsed material/section/component library document with provenance — this manager validates and stores it; it does not parse external file formats)
              - textbox "Library import document JSON (an already-parsed material/section/component library document with provenance — this manager validates and stores it; it does not parse external file formats)" [disabled] [ref=e606]:
                - /placeholder: No import document. Load an invented sample or paste a library document.
              - generic [ref=e607]:
                - button "Validate import" [disabled] [ref=e608]
                - button "Save to local store" [disabled] [ref=e609]
                - button "Discard import document" [disabled] [ref=e610]
              - generic [ref=e611]: No library-import action has run in this session.
            - generic [ref=e612]:
              - img [ref=e613]
              - text: Imported private libraries stay in local project storage only — never committed to the repository, transmitted, or bundled into public artifacts. Only an accepted private import is stored; a suspected-protected or otherwise-blocked import is refused, not stored (DEC-036). Import validation reports software findings over an already-parsed payload; no protected standards content — code-specific data is user-supplied.
    - complementary "Agent" [ref=e615]:
      - generic [ref=e616]:
        - button "Agent" [disabled] [active] [ref=e617]:
          - img [ref=e618]
          - generic [ref=e621]: Agent
        - 'tooltip "Agent: not available yet" [ref=e622]'
  - generic "Workspace status" [ref=e623]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e624] [cursor=pointer]:
      - img [ref=e625]
      - text: 5 Issues
    - generic "Selection" [ref=e627]: "project: project:invented-loop-01"
    - generic "Display units" [ref=e628]: Entered
    - button "About SWBPIPE…" [ref=e629] [cursor=pointer]:
      - img [ref=e630]
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