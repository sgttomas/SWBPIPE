# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b3-accessibility.spec.ts >> compact routing selector owns popup Escape before configured shell consumers
- Location: e2e/b3-accessibility.spec.ts:202:3

# Error details

```
Test timeout of 120000ms exceeded.
```

```
Error: locator.click: Test timeout of 120000ms exceeded.
Call log:
  - waiting for getByRole('listbox', { name: 'New node coordinate unit' }).locator('[role="option"][data-value="mm"]')

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
      - generic [ref=e16]:
        - heading "SWBPIPE" [level=1] [ref=e17]
        - paragraph [ref=e18]: Invented Utility Loop Preview
      - group "Editing tools" [ref=e19]:
        - button "Undo model edit" [disabled] [ref=e20]:
          - img [ref=e21]
        - button "Redo model edit" [disabled] [ref=e24]:
          - img [ref=e25]
        - button "Select" [ref=e28] [cursor=pointer]:
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
        - button "Inspector" [expanded] [ref=e61] [cursor=pointer]:
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
            - button "Issues, 5" [ref=e128] [cursor=pointer]:
              - img [ref=e129]
              - generic [ref=e131]: Issues
              - generic [ref=e132]: "5"
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
              - button "Node" [pressed] [ref=e556] [cursor=pointer]:
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
            - 'generic "Node tool armed: click empty canvas to fill coordinates, then queue node." [ref=e575]': Node tool armed
            - group [ref=e576]:
              - generic "Selection & navigation" [ref=e577] [cursor=pointer]
        - generic [ref=e578]:
          - button "Close inspector" [ref=e579]:
            - img [ref=e580]
          - region "Viewport editor intents" [ref=e584]:
            - heading "Create node" [level=3] [ref=e585]
            - group [ref=e586]:
              - generic "Explicit node geometry" [ref=e587]:
                - generic [ref=e588]:
                  - generic [ref=e589]: Node ID
                  - textbox "New node ID" [ref=e590]:
                    - /placeholder: node:N-3
                - generic [ref=e591]:
                  - generic [ref=e592]: Label
                  - textbox "New node label" [ref=e593]:
                    - /placeholder: Node label
                    - text: Retained popup draft
                - generic [ref=e594]:
                  - generic [ref=e595]: X
                  - textbox "New node X coordinate" [ref=e596]:
                    - /placeholder: "0"
                - generic [ref=e597]:
                  - generic [ref=e598]: "Y"
                  - textbox "New node Y coordinate" [ref=e599]:
                    - /placeholder: "0"
                - generic [ref=e600]:
                  - generic [ref=e601]: Z
                  - textbox "New node Z coordinate" [ref=e602]:
                    - /placeholder: "0"
                - generic [ref=e603]:
                  - generic [ref=e604]: Coordinate unit
                  - combobox "New node coordinate unit" [expanded] [active] [ref=e606] [cursor=pointer]:
                    - generic [ref=e607]: m
                    - text: ▾
                - generic [ref=e608]: "Coordinates: m, model metadata"
                - generic [ref=e609]: "Pointer plane: global XZ · Y=0 m"
                - generic "Click within 4 CSS pixels on the visible 3D canvas to capture on global XZ at Y=0." [ref=e610]
                - generic [ref=e611]:
                  - generic [ref=e612]: Provenance
                  - textbox "New node provenance" [ref=e613]
                - button "Add node" [disabled] [ref=e614]:
                  - img [ref=e615]
                  - text: Add node
                - generic [ref=e617]: Enter a node ID.; Enter finite X, Y, and Z coordinates.; Enter node provenance.
              - text: ▾ ▾ ▾ ▾ ▾ ▾
              - region "Route review" [ref=e618]:
                - heading "Review and Apply" [level=4] [ref=e619]
                - status [ref=e620]: The affected selection changed. Add again to review the current draft.
                - paragraph [ref=e621]: Add a complete node or route to generate the service validation and exact diff.
                - button "Apply" [disabled] [ref=e622]
              - group [ref=e623]:
                - generic "Unit source" [ref=e624] [cursor=pointer]
            - group [ref=e625]:
              - generic "Pending changes (0)" [ref=e626] [cursor=pointer]
          - region "Property inspector" [ref=e627]:
            - 'heading "Invented Utility Loop Preview — project: project:invented-loop-01" [level=2] [ref=e628]':
              - text: Invented Utility Loop Preview
              - generic [ref=e629]: "— project: project:invented-loop-01"
            - tablist "Inspector views" [ref=e630]:
              - tab "Properties" [selected] [ref=e631]
              - tab "Task" [ref=e632]
            - tabpanel [ref=e633]:
              - group [ref=e634]:
                - generic "All properties" [ref=e635] [cursor=pointer]
            - generic [ref=e636]:
              - group [ref=e637]:
                - generic "Sources and units" [ref=e638] [cursor=pointer]
              - group [ref=e639]:
                - generic "New support configuration" [ref=e640] [cursor=pointer]
                - text: ▾
              - group [ref=e641]:
                - generic "New section" [ref=e642] [cursor=pointer]
                - text: ▾ ▾
              - group [ref=e643]:
                - generic "New material" [ref=e644] [cursor=pointer]
                - text: ▾ ▾
              - group [ref=e645]:
                - generic "New support" [ref=e646] [cursor=pointer]
                - text: ▾
              - group [ref=e647]:
                - generic "New component" [ref=e648] [cursor=pointer]
                - text: ▾ ▾ ▾
      - complementary "Agent" [ref=e649]:
        - button "Agent" [disabled] [ref=e651]:
          - img [ref=e652]
          - generic [ref=e655]: Agent
    - generic "Workspace status" [ref=e656]:
      - generic "Analysis statuses"
      - button "5 Issues" [ref=e657] [cursor=pointer]:
        - img [ref=e658]
        - text: 5 Issues
      - generic "Selection" [ref=e660]: "project: project:invented-loop-01"
      - generic "Display units" [ref=e661]: Entered
      - button "About SWBPIPE…" [ref=e662] [cursor=pointer]:
        - img [ref=e663]
  - listbox "New node coordinate unit" [ref=e665]:
    - option "m" [selected] [ref=e666]
```

# Test source

```ts
  156 |       await page.keyboard.press("Enter");
  157 |     } else {
  158 |       // The menu command is removed when activated, so Close needs a safe fallback.
  159 |       await page.getByTestId("menu-view").click();
  160 |       await page.getByTestId(`menu-item-view.section.${section}`).focus();
  161 |       await page.keyboard.press("Enter");
  162 |     }
  163 |     await expect(page.getByTestId(`workspace-section-${section}`)).toBeVisible();
  164 |     await tabTo(page, "workspace-dock-close");
  165 |     await page.keyboard.press("Enter");
  166 |     await expect(page.getByTestId("workspace-dock-close")).toHaveCount(0);
  167 |     // No focus call after Close: the application must establish this destination.
  168 |     const focus = await page.evaluate(() => {
  169 |       const el = document.activeElement as HTMLElement | null;
  170 |       if (!el) return null;
  171 |       const r = el.getBoundingClientRect(), style = getComputedStyle(el);
  172 |       return {
  173 |         tag: el.tagName, testId: el.dataset.testid, text: el.textContent?.trim().slice(0, 200),
  174 |         inert: Boolean(el.closest("[inert]")), rect: r.toJSON(),
  175 |         visible: style.visibility === "visible" && style.display !== "none" && r.width > 0 && r.height > 0,
  176 |         unobscured: el.contains(document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2))
  177 |       };
  178 |     });
  179 |     const session = await page.context().newCDPSession(page);
  180 |     let focusedAX;
  181 |     try {
  182 |       const { result } = await session.send("Runtime.evaluate", { expression: "document.activeElement" });
  183 |       const { nodes } = await session.send("Accessibility.getPartialAXTree", { objectId: result.objectId!, fetchRelatives: false });
  184 |       focusedAX = nodes.map(node => ({ ignored: node.ignored, role: node.role?.value, name: node.name?.value }));
  185 |     } finally { await session.detach(); }
  186 |     await witness(info, `${section}-keyboard-close-focus`, { focus, focusedAX });
  187 |     expect.soft(focus?.tag).not.toBe("BODY");
  188 |     expect.soft(focus?.visible).toBe(true);
  189 |     expect.soft(focus?.inert).toBe(false);
  190 |     expect.soft(focus?.unobscured).toBe(true);
  191 |     expect.soft(focusedAX.some(node => !node.ignored && node.role === "button")).toBe(true);
  192 |     if (section === "libraries") await expect.soft(page.getByTestId("rail-page-libraries")).toBeFocused();
  193 |     expect(await canvas!.evaluate(el => el === document.querySelector("canvas"))).toBe(true);
  194 |     expect(await inspector!.evaluate(el => el === document.querySelector('[data-testid="property-inspector"]'))).toBe(true);
  195 |     expect(await draft!.evaluate(el => el === document.querySelector('[data-testid="editor-intent-value"]'))).toBe(true);
  196 |     await expect(page.getByTestId("editor-intent-value")).toHaveValue("Retained keyboard Close draft");
  197 |     expect(await nativeAXNames(page)).toContain("region: Modeling workspace");
  198 |   });
  199 | }
  200 | 
  201 | for (const layout of ["configured", "narrow"] as const) {
  202 |   test(`compact routing selector owns popup Escape before ${layout} shell consumers`, async ({ page }, info) => {
  203 |     if (layout === "narrow") await page.setViewportSize({ width: 1200, height: 800 });
  204 |     await page.goto("/");
  205 |     await page.getByTestId("command-node").click();
  206 |     const unit = page.getByTestId("viewport-create-node-unit");
  207 |     const draft = page.getByTestId("viewport-create-node-label");
  208 |     await draft.fill("Retained popup draft");
  209 |     const retained = await draft.elementHandle();
  210 |     await unit.focus();
  211 |     await page.keyboard.press("Space");
  212 |     await expect(unit).toHaveAttribute("aria-expanded", "true");
  213 |     const list = page.getByRole("listbox", { name: "New node coordinate unit" });
  214 |     await expect(list).toBeVisible();
  215 |     const placement = await list.evaluate(el => {
  216 |       const rect = el.getBoundingClientRect();
  217 |       const center = document.elementFromPoint((rect.left + rect.right) / 2, (rect.top + rect.bottom) / 2);
  218 |       return { rect: rect.toJSON(), width: innerWidth, height: innerHeight, portalled: el.parentElement === document.body,
  219 |         topmost: el.contains(center) };
  220 |     });
  221 |     await witness(info, `compact-${layout}-popup-placement`, placement);
  222 |     expect(placement.portalled).toBe(true);
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
  255 |     await unit.click();
> 256 |     await list.locator('[role="option"][data-value="mm"]').click();
      |                                                            ^ Error: locator.click: Test timeout of 120000ms exceeded.
  257 |     await expect(unit).toHaveAttribute("data-value", "mm");
  258 |     await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  259 |     await expect(page.getByTestId("workspace-redo")).toBeDisabled();
  260 |     await witness(info, `compact-${layout}-ax`, await nativeAXNames(page));
  261 | 
  262 |     // A retained control's open popup is dismissed on stage change, without remounting its draft.
  263 |     await unit.click();
  264 |     await page.getByTestId("rail-stage-loads").click();
  265 |     await expect(list).toHaveCount(0);
  266 |     await expect(page.getByTestId("rail-stage-loads")).toBeFocused();
  267 |     await page.getByTestId("rail-stage-model").click();
  268 |     await expect(draft).toHaveValue("Retained popup draft");
  269 |     expect(await draft.evaluate((el, old) => el === old, retained)).toBe(true);
  270 |   });
  271 | }
  272 | 
```