# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: r2-smoke.spec.ts >> R2 desktop preview smoke covers solve, results, report, and viewport overlay
- Location: e2e/r2-smoke.spec.ts:250:1

# Error details

```
Error: expect(locator).toHaveValue(expected) failed

Locator: getByTestId('viewport-create-pipe-length-unit')
Expected: "m"
Error: Not an input element

Call log:
  - Expect "toHaveValue" with timeout 10000ms
  - waiting for getByTestId('viewport-create-pipe-length-unit')

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
              - searchbox "Filter model tree" [ref=e162]: pipe:P-130
            - generic [ref=e163]: 3 of 27 model entities visible
            - button "Clear model tree filter" [ref=e164] [cursor=pointer]:
              - img [ref=e165]
          - tree "Model" [ref=e168]:
            - generic [ref=e169]:
              - treeitem "Pipes" [expanded] [level=1] [ref=e171] [cursor=pointer]:
                - generic [ref=e172]: ▾
                - strong [ref=e173]: Pipes
              - treeitem "Tie-in rise pipe:P-130" [level=2] [selected] [ref=e175] [cursor=pointer]:
                - img [ref=e176]
                - generic [ref=e180]:
                  - strong [ref=e181]: Tie-in rise
                  - generic [ref=e182]: pipe:P-130
              - treeitem "Components" [expanded] [level=1] [ref=e184] [cursor=pointer]:
                - generic [ref=e185]: ▾
                - strong [ref=e186]: Components
              - treeitem "Invented semi-rigid valve marker component:C-130" [level=2] [ref=e188] [cursor=pointer]:
                - img [ref=e189]
                - generic [ref=e196]:
                  - strong [ref=e197]: Invented semi-rigid valve marker
                  - generic [ref=e198]: component:C-130
              - treeitem "Invented expansion joint marker component:C-150" [level=2] [ref=e200] [cursor=pointer]:
                - img [ref=e201]
                - generic [ref=e208]:
                  - strong [ref=e209]: Invented expansion joint marker
                  - generic [ref=e210]: component:C-150
      - separator "Resize table and canvas" [ref=e211]
      - generic [ref=e213]:
        - generic [ref=e214]:
          - group "Viewport controls" [ref=e215]:
            - generic [ref=e216]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e217]:
              - generic "Deformation · unavailable" [ref=e218] [cursor=pointer]
            - group "Viewport display toggles" [ref=e219]:
              - button "Labels" [pressed] [ref=e220]
              - button "Loads" [pressed] [ref=e221]
              - button "Grid" [pressed] [ref=e222]
            - group "Viewport selection tools" [ref=e223]:
              - button "Box Select" [ref=e224]
              - generic [ref=e225]:
                - generic [ref=e226]: Selection filter
                - combobox "Selection filter" [ref=e227]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [ref=e228]
              - button "Isolate" [ref=e229]
              - button "Show All" [disabled] [ref=e230]
              - button "Fit Model" [ref=e231]
              - button "Fit Visible" [ref=e232]
              - button "Fit Selection" [ref=e233]
            - group "Viewport geometry" [ref=e234]:
              - button "Schematic" [pressed] [ref=e235]
              - button "Actual OD" [ref=e236]
              - button "Measure" [ref=e237]
          - generic "Viewport status" [ref=e238]:
            - 'generic "Selected pipe: pipe:P-130" [ref=e239]': "Selected: pipe:P-130"
            - status "Schematic centerline geometry" [ref=e240]
            - status "View command status" [ref=e241]: No view command dispatched.
        - generic [ref=e242]:
          - generic "Three.js pipe centerline viewport" [ref=e243]
          - generic "Viewport entity selection":
            - button "Select Tie-in rise in viewport" [pressed] [ref=e245] [cursor=pointer]:
              - img [ref=e246]
              - generic [ref=e250]: P-130
            - button "Select Vertical riser in viewport" [ref=e251] [cursor=pointer]:
              - img [ref=e252]
              - generic [ref=e256]: P-110
            - button "Select Invented branch connection marker in viewport" [ref=e257] [cursor=pointer]:
              - img [ref=e258]
              - generic [ref=e261]: C-120
            - button "Select Low point elbow in viewport" [ref=e262] [cursor=pointer]:
              - img [ref=e263]
              - generic [ref=e266]: N-110
            - button "Select Riser elbow in viewport" [ref=e267] [cursor=pointer]:
              - img [ref=e268]
              - generic [ref=e271]: N-120
            - button "Select Rack span in viewport" [ref=e272] [cursor=pointer]:
              - img [ref=e273]
              - generic [ref=e277]: P-120
            - button "Select Pump discharge run in viewport" [ref=e278] [cursor=pointer]:
              - img [ref=e279]
              - generic [ref=e283]: P-100
            - button "Select Invented semi-rigid valve marker in viewport" [ref=e284] [cursor=pointer]:
              - img [ref=e285]
              - generic [ref=e288]: C-130
            - button "Select Pump nozzle in viewport" [ref=e289] [cursor=pointer]:
              - img [ref=e290]
              - generic [ref=e293]: N-100
            - button "Select Rack turn in viewport" [ref=e294] [cursor=pointer]:
              - img [ref=e295]
              - generic [ref=e298]: N-130
            - button "Select Terminal tie-in in viewport" [ref=e299] [cursor=pointer]:
              - img [ref=e300]
              - generic [ref=e303]: N-140
            - button "Select Preview sliding-friction rack shoe in viewport" [ref=e304] [cursor=pointer]:
              - img [ref=e305]
              - generic [ref=e308]: NL-130-FRIC
            - button "Select Preview one-way terminal stop in viewport" [ref=e309] [cursor=pointer]:
              - img [ref=e310]
              - generic [ref=e313]: NL-140
            - button "Select Anchor at pump nozzle in viewport" [ref=e314] [cursor=pointer]:
              - img [ref=e315]
              - generic [ref=e318]: S-100
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e319]:
            - button "Front" [ref=e320] [cursor=pointer]
            - button "Top" [ref=e321] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e322] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e323]:
          - generic "Object creation tools" [ref=e324]:
            - button "Node" [ref=e325] [cursor=pointer]:
              - img [ref=e326]
              - text: Node
            - button "Pipe" [active] [pressed] [ref=e328] [cursor=pointer]:
              - img [ref=e329]
              - text: Pipe
            - button "Support" [ref=e333] [cursor=pointer]:
              - img [ref=e334]
              - text: Support
            - button "Component" [ref=e337] [cursor=pointer]:
              - img [ref=e338]
              - text: Component
            - button "Load" [ref=e341] [cursor=pointer]:
              - img [ref=e342]
              - text: Load
          - 'generic "Pipe tool armed: pick from/to nodes or complete the pipe form." [ref=e344]': Pipe tool armed
          - group [ref=e345]:
            - generic "Selection & navigation" [ref=e346] [cursor=pointer]
      - generic [ref=e347]:
        - button "Close inspector" [ref=e348]:
          - img [ref=e349]
        - region "Viewport editor intents" [ref=e353]:
          - heading "Create pipe" [level=3] [ref=e354]
          - group [ref=e355]:
            - text: ▾
            - generic "Explicit straight pipe connectivity" [ref=e356]:
              - generic [ref=e357]:
                - generic [ref=e358]: Pipe ID
                - textbox "New pipe ID" [ref=e359]:
                  - /placeholder: pipe:P-2
              - generic [ref=e360]:
                - generic [ref=e361]: Label
                - textbox "New pipe label" [ref=e362]:
                  - /placeholder: Pipe label
              - generic [ref=e363]:
                - generic [ref=e364]:
                  - generic [ref=e365]: From
                  - button "Pick" [pressed] [ref=e366] [cursor=pointer]:
                    - img [ref=e367]
                    - text: Pick
                - generic [ref=e370]:
                  - generic [ref=e371]:
                    - generic [ref=e372]: New pipe from node
                    - combobox "New pipe from node" [expanded] [ref=e373]
                  - generic "New pipe from node current value" [ref=e374]: No target selected
                  - generic [ref=e375]: 5 of 5 targets
                  - listbox "New pipe from node options" [ref=e376]:
                    - generic [ref=e377]:
                      - option "Pump nozzle node:N-100" [ref=e379] [cursor=pointer]:
                        - generic [ref=e380]: Pump nozzle
                        - generic [ref=e381]: node:N-100
                      - option "Low point elbow node:N-110" [ref=e383] [cursor=pointer]:
                        - generic [ref=e384]: Low point elbow
                        - generic [ref=e385]: node:N-110
                      - option "Riser elbow node:N-120" [ref=e387] [cursor=pointer]:
                        - generic [ref=e388]: Riser elbow
                        - generic [ref=e389]: node:N-120
                      - option "Rack turn node:N-130" [ref=e391] [cursor=pointer]:
                        - generic [ref=e392]: Rack turn
                        - generic [ref=e393]: node:N-130
                      - option "Terminal tie-in node:N-140" [ref=e395] [cursor=pointer]:
                        - generic [ref=e396]: Terminal tie-in
                        - generic [ref=e397]: node:N-140
              - group "End mode" [ref=e398]:
                - generic [ref=e399]: End mode
                - generic [ref=e400]:
                  - radio "Existing node" [checked] [ref=e401]
                  - text: Existing node
                - generic [ref=e402]:
                  - radio "New node" [ref=e403]
                  - text: New node
              - generic [ref=e404]:
                - generic [ref=e405]:
                  - generic [ref=e406]: To
                  - button "Pick" [ref=e407] [cursor=pointer]:
                    - img [ref=e408]
                    - text: Pick
                - generic [ref=e411]:
                  - generic [ref=e412]:
                    - generic [ref=e413]: New pipe to node
                    - combobox "New pipe to node" [expanded] [ref=e414]
                  - generic "New pipe to node current value" [ref=e415]: No target selected
                  - generic [ref=e416]: 5 of 5 targets
                  - listbox "New pipe to node options" [ref=e417]:
                    - generic [ref=e418]:
                      - option "Pump nozzle node:N-100" [ref=e420] [cursor=pointer]:
                        - generic [ref=e421]: Pump nozzle
                        - generic [ref=e422]: node:N-100
                      - option "Low point elbow node:N-110" [ref=e424] [cursor=pointer]:
                        - generic [ref=e425]: Low point elbow
                        - generic [ref=e426]: node:N-110
                      - option "Riser elbow node:N-120" [ref=e428] [cursor=pointer]:
                        - generic [ref=e429]: Riser elbow
                        - generic [ref=e430]: node:N-120
                      - option "Rack turn node:N-130" [ref=e432] [cursor=pointer]:
                        - generic [ref=e433]: Rack turn
                        - generic [ref=e434]: node:N-130
                      - option "Terminal tie-in node:N-140" [ref=e436] [cursor=pointer]:
                        - generic [ref=e437]: Terminal tie-in
                        - generic [ref=e438]: node:N-140
              - text: ▾ ▾
              - generic [ref=e439]:
                - generic [ref=e440]:
                  - generic [ref=e441]: New pipe material
                  - combobox "New pipe material" [expanded] [ref=e442]
                - generic "New pipe material current value" [ref=e443]: No target selected
                - generic [ref=e444]: 1 of 1 targets
                - listbox "New pipe material options" [ref=e445]:
                  - option "Invented carbon-steel-like material material:invented-carbon-steel" [ref=e448] [cursor=pointer]:
                    - generic [ref=e449]: Invented carbon-steel-like material
                    - generic [ref=e450]: material:invented-carbon-steel
              - generic [ref=e451]:
                - generic [ref=e452]: OD
                - textbox "New pipe outside diameter" [ref=e453]:
                  - /placeholder: "0.114"
              - generic [ref=e454]:
                - generic [ref=e455]: Wall
                - textbox "New pipe wall thickness" [ref=e456]:
                  - /placeholder: "0.006"
              - generic [ref=e457]:
                - generic [ref=e458]: Length unit
                - combobox "New pipe length unit" [ref=e460] [cursor=pointer]:
                  - generic [ref=e461]: m
                  - text: ▾
              - generic [ref=e462]: "Pipe geometry: m, model metadata"
              - generic [ref=e463]: "Construction plane inactive: existing endpoint uses exact node IDs."
              - generic [ref=e464]: No route ghost is visible.
              - generic [ref=e465]:
                - generic [ref=e466]: Yref X
                - textbox "New pipe y-reference X" [ref=e467]:
                  - /placeholder: "0"
              - generic [ref=e468]:
                - generic [ref=e469]: Yref Y
                - textbox "New pipe y-reference Y" [ref=e470]:
                  - /placeholder: "0"
              - generic [ref=e471]:
                - generic [ref=e472]: Yref Z
                - textbox "New pipe y-reference Z" [ref=e473]:
                  - /placeholder: "1"
              - generic [ref=e474]:
                - generic [ref=e475]: Provenance
                - textbox "New pipe provenance" [ref=e476]
              - generic [ref=e477]:
                - checkbox "Continue from end after Apply; keep the entered material, dimensions, orientation and provenance" [ref=e478]
                - text: Continue from end after Apply; keep the entered material, dimensions, orientation and provenance
              - button "Cancel pipe draft" [ref=e479] [cursor=pointer]
              - button "Add route" [disabled] [ref=e480]:
                - img [ref=e481]
                - text: Add route
              - generic [ref=e485]: Choose an existing start node.; Choose an existing end node.; Enter a pipe ID.; Enter a pipe label.; Choose an existing material ID.; Enter positive outside diameter and wall thickness values.; Enter a finite, nonzero y-reference vector.; Enter pipe provenance.
            - text: ▾ ▾ ▾
            - region "Route review" [ref=e486]:
              - heading "Review and Apply" [level=4] [ref=e487]
              - status [ref=e488]: The affected selection changed. Add again to review the current draft.
              - paragraph [ref=e489]: Add a complete node or route to generate the service validation and exact diff.
              - button "Apply" [disabled] [ref=e490]
            - group [ref=e491]:
              - generic "Unit source" [ref=e492] [cursor=pointer]
              - text: browser preview uses model metadata for viewport length units
          - group [ref=e493]:
            - generic "Pending changes (0)" [ref=e494] [cursor=pointer]
        - region "Property inspector" [ref=e495]:
          - 'heading "Tie-in rise — pipe: pipe:P-130" [level=2] [ref=e496]':
            - text: Tie-in rise
            - generic [ref=e497]: "— pipe: pipe:P-130"
          - tablist "Inspector views" [ref=e498]:
            - tab "Properties" [ref=e499]
            - tab "Task" [selected] [ref=e500]
          - tabpanel "Editor operation intent" [ref=e501]:
            - generic [ref=e503]: "Draft target: pipe: pipe:P-130"
            - heading "Edit name" [level=3] [ref=e504]:
              - img [ref=e505]
              - text: Edit name
            - generic [ref=e508]:
              - generic [ref=e509]:
                - generic [ref=e510]: Property
                - combobox "Property to edit" [ref=e512] [cursor=pointer]:
                  - generic [ref=e513]: Name
                  - text: ▾
              - region "Current value" [ref=e514]:
                - generic [ref=e515]: Current name
                - strong [ref=e516]: Tie-in rise
              - generic [ref=e517]:
                - generic [ref=e518]: New name
                - textbox "New name" [ref=e519]: Tie-in rise
              - generic "Task actions" [ref=e520]:
                - button "Add" [disabled] [ref=e521]:
                  - img [ref=e522]
                  - text: Add
                - button "Cancel" [ref=e523] [cursor=pointer]
                - button "Review" [disabled] [ref=e524]: Review
                - button "Apply" [disabled] [ref=e528]:
                  - img [ref=e529]
                  - text: Apply
            - group [ref=e532]:
              - generic "Operation details" [ref=e533] [cursor=pointer]
            - paragraph [ref=e534]: Validate to check this change before applying it.
          - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e535]:
      - button "Agent" [disabled] [ref=e537]:
        - img [ref=e538]
        - generic [ref=e541]: Agent
  - generic "Workspace status" [ref=e542]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e543] [cursor=pointer]:
      - img [ref=e544]
      - text: 5 Issues
    - generic "Selection" [ref=e546]: "pipe: pipe:P-130"
    - generic "Display units" [ref=e547]: Entered
    - button "About SWBPIPE…" [ref=e548] [cursor=pointer]:
      - img [ref=e549]
```

# Test source

```ts
  443 |   await page.getByTestId("load-manager-metadata-field").selectOption("kind");
  444 |   await expect(page.getByTestId("load-manager-selected-case")).toContainText(
  445 |     "field=kind; current=primitive_user_load"
  446 |   );
  447 |   await page.getByTestId("load-manager-metadata-value").selectOption("TBD");
  448 |   await expect(page.getByTestId("load-manager-metadata-preview")).toContainText(
  449 |     "op:load-manager-load:L-100-kind"
  450 |   );
  451 |   await expect(page.getByTestId("load-manager-load-case-delete-preview")).toContainText(
  452 |     "op:load-manager-load:L-100-delete"
  453 |   );
  454 |   await expect(page.getByTestId("load-manager-load-case-delete-preview")).toContainText(
  455 |     "before=load:L-100; Invented operating gravity and pressure preview; primitive_user_load; preview_only; primitives=5; after=not_present; unit=none; dimensionless"
  456 |   );
  457 |   await expect(page.getByTestId("load-manager-selected-combination")).toContainText(
  458 |     "field=basis; current=mechanics"
  459 |   );
  460 |   await expect(page.getByTestId("load-manager-combination-basis-preview")).toContainText("current=mechanics");
  461 |   // TP-APP-R2-COMBEXPR-001 behavior change: the basis editor is a closed-set
  462 |   // selector (mechanics | result_state_subtraction | range_envelope) instead
  463 |   // of free text; cross-shape edits stay honest proposals the engine blocks.
  464 |   await page.getByTestId("load-manager-combination-basis-value").selectOption("result_state_subtraction");
  465 |   await expect(page.getByTestId("load-manager-combination-basis-preview")).toContainText(
  466 |     "op:load-manager-combination:C-OPER-ALT-basis"
  467 |   );
  468 |   await expect(page.getByTestId("load-manager-combination-basis-preview")).toContainText(
  469 |     "before=mechanics; after=result_state_subtraction"
  470 |   );
  471 |   await expect(page.getByTestId("load-manager-combination-entity-delete-preview")).toContainText(
  472 |     "op:load-manager-combination:C-OPER-ALT-delete"
  473 |   );
  474 |   await expect(page.getByTestId("load-manager-combination-entity-delete-preview")).toContainText(
  475 |     "before=combination:C-OPER-ALT; Invented explicit operating plus alternate preview; basis=mechanics; terms=load:L-100 x 1; load:L-200 x 0.5"
  476 |   );
  477 |   await expect(page.getByTestId("load-manager-create-combination-term-heading")).toContainText(
  478 |     "combination:C-OPER-ALT"
  479 |   );
  480 |   await expect(page.getByTestId("load-manager-create-combination-term-preview")).toContainText(
  481 |     "op:load-manager-combination:C-OPER-ALT-term-2-create"
  482 |   );
  483 |   await expect(page.getByTestId("load-manager-create-combination-term-preview")).toContainText(
  484 |     "before=not_present; after=load:L-100 x 1; unit=none; dimensionless"
  485 |   );
  486 |   await page.getByTestId("load-manager-create-combination-term-factor").fill("0.25");
  487 |   await expect(page.getByTestId("load-manager-create-combination-term-preview")).toContainText(
  488 |     "before=not_present; after=load:L-100 x 0.25; unit=none; dimensionless"
  489 |   );
  490 |   await page.getByTestId("load-manager-combination-term-combination:C-OPER-ALT-1").click();
  491 |   await expect(page.getByTestId("load-manager-selected-combination-term")).toContainText("terms.1.factor");
  492 |   await expect(page.getByTestId("load-manager-combination-factor-preview")).toContainText("current=0.5");
  493 |   await page.getByTestId("load-manager-combination-factor-value").fill("0.75");
  494 |   await expect(page.getByTestId("load-manager-combination-factor-preview")).toContainText(
  495 |     "op:load-manager-combination:C-OPER-ALT-term-1-factor"
  496 |   );
  497 |   await expect(page.getByTestId("load-manager-combination-factor-preview")).toContainText(
  498 |     "before=0.5; after=0.75"
  499 |   );
  500 |   await expect(page.getByTestId("load-manager-combination-delete-preview")).toContainText(
  501 |     "op:load-manager-combination:C-OPER-ALT-term-1-delete"
  502 |   );
  503 |   await expect(page.getByTestId("load-manager-combination-delete-preview")).toContainText(
  504 |     "before=load:L-200 x 0.5; after=not_present; unit=none; dimensionless"
  505 |   );
  506 |   // Delete previews belong to the explicitly started task, not a new selection.
  507 |   await startPropertyTaskFromTreeEntity(page, "node", "node:N-120");
  508 |   await expect(page.getByTestId("delete-node-intent-panel")).toContainText("delete_node");
  509 |   await expect(page.getByTestId("delete-node-intent-panel")).toContainText(
  510 |     "before=Riser elbow; x=3.2; y=2.4; z=0"
  511 |   );
  512 |   await expect(page.getByTestId("delete-node-intent-panel")).toContainText("after=not_present");
  513 |   await startPropertyTaskFromTreeEntity(page, "pipe", "pipe:P-130");
  514 |   await expect(page.getByTestId("delete-pipe-intent-panel")).toContainText("delete_pipe_run");
  515 |   await expect(page.getByTestId("delete-pipe-intent-panel")).toContainText(
  516 |     "before=Tie-in rise; node:N-130->node:N-140; material=material:invented-carbon-steel"
  517 |   );
  518 |   await expect(page.getByTestId("delete-pipe-intent-panel")).toContainText("after=not_present");
  519 | 
  520 |   const canvas = page.locator(".viewport-canvas canvas");
  521 |   await expect(canvas).toBeVisible();
  522 |   await expect(page.getByTestId("viewport-editor-intents")).toHaveClass(/collapsed/);
  523 |   await ensureCreationToolArmed(page, "command-node", "Node tool armed");
  524 |   await openNamedDisclosure(page.getByTestId("viewport-editor-intents"), "Unit source");
  525 |   await expect(page.getByTestId("viewport-unit-catalog-status")).toContainText(
  526 |     "browser preview uses model metadata"
  527 |   );
  528 |   await expect(page.getByTestId("viewport-create-node-unit")).toHaveAttribute("data-value", "m");
  529 |   await expect(page.getByTestId("viewport-create-node-unit-basis")).toContainText(
  530 |     "Coordinates: m, model metadata"
  531 |   );
  532 |   await canvas.click({ position: { x: 64, y: 64 } });
  533 |   await expect(page.getByTestId("viewport-create-node-id")).toHaveValue("node:V-001");
  534 |   await expect(page.getByTestId("viewport-create-node-label")).toHaveValue("Viewport node V-001");
  535 |   await expect(page.getByTestId("viewport-create-node-x")).toHaveValue(/^-?\d/);
  536 |   await expect(page.getByTestId("viewport-create-node-y")).toHaveValue("0");
  537 |   await expect(page.getByTestId("viewport-create-node-z")).toHaveValue(/^-?\d/);
  538 |   await expect(page.getByTestId("queue-explicit-node-intent")).toBeDisabled();
  539 |   await page.getByTestId("viewport-create-node-provenance").fill("invented_synthetic_ui_acceptance_input");
  540 |   await expect(page.getByTestId("queue-explicit-node-intent")).toBeEnabled();
  541 | 
  542 |   await ensureCreationToolArmed(page, "command-pipe", "Pipe tool armed");
> 543 |   await expect(page.getByTestId("viewport-create-pipe-length-unit")).toHaveValue("m");
      |                                                                      ^ Error: expect(locator).toHaveValue(expected) failed
  544 |   await expect(page.getByTestId("viewport-create-pipe-unit-basis")).toContainText(
  545 |     "Pipe geometry: m, model metadata"
  546 |   );
  547 | 
  548 |   await ensurePipeEndpointPick(page, "viewport-pick-pipe-from");
  549 |   // Selected entities receive label priority when projected labels overlap.
  550 |   // Tree selection must preserve endpoint capture; the viewport click performs it.
  551 |   await selectTreeEntity(page, "node", "node:N-100");
  552 |   await expect(page.getByTestId("viewport-pick-pipe-from")).toHaveAttribute("aria-pressed", "true");
  553 |   await expect(page.getByTestId("viewport-select-node:N-100")).toBeVisible();
  554 |   await page.getByTestId("viewport-select-node:N-100").click();
  555 |   await expectVirtualTarget(page, "viewport-create-pipe-from", "node:N-100");
  556 |   await expect(page.getByTestId("viewport-pick-pipe-to")).toHaveAttribute("aria-pressed", "true");
  557 |   await selectTreeEntity(page, "node", "node:N-140");
  558 |   await expect(page.getByTestId("viewport-pick-pipe-to")).toHaveAttribute("aria-pressed", "true");
  559 |   await expect(page.getByTestId("viewport-select-node:N-140")).toBeVisible();
  560 |   await page.getByTestId("viewport-select-node:N-140").click();
  561 |   await expectVirtualTarget(page, "viewport-create-pipe-to", "node:N-140");
  562 |   await expect(page.getByTestId("viewport-pick-pipe-to")).toHaveAttribute("aria-pressed", "false");
  563 | 
  564 |   const before = await canvas.screenshot();
  565 |   expect(pngStats(before).uniqueColors).toBeGreaterThan(100);
  566 |   await expect(page.getByTestId("viewport-axis-triad")).toBeVisible();
  567 |   await expect(page.getByTestId("viewport-view-cube")).toBeVisible();
  568 |   await expect(page.getByTestId("viewport-scale-bar")).toContainText("m");
  569 |   await expect(page.getByTestId("command-bar")).toBeVisible();
  570 | 
  571 |   await openWorkspaceSection(page, "solve");
  572 |   await page.getByTestId("issues-drawer-toggle").click();
  573 |   await expect(page.getByTestId("missing-data-unit-policy")).toContainText("required=true");
  574 |   await expect(page.getByTestId("missing-data-unit-policy")).toContainText("default_units=false");
  575 |   await expect(page.getByTestId("missing-data-unit-policy")).toContainText("conversion=false");
  576 |   await page.getByTestId("issues-home").getByRole("button", { name: /Close/i }).click();
  577 |   await page.getByTestId("run-mechanics-preview").click();
  578 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  579 |   await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=830");
  580 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("model=angle=rad,force=N,length=m");
  581 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("N*m/rad,N/m");
  582 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("rows=830");
  583 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("conversion=false");
  584 |   // Slice B3: the Analyze page lies over the stage's surfaces; close it to reach the canvas.
  585 |   await showCanvas(page);
  586 |   await setDisclosure(page.getByTestId("viewport-deformation-status"));
  587 |   await expect(page.getByTestId("viewport-deformation-status")).toContainText("available; nodes=5; max=4.927112 mm");
  588 |   await expect(page.getByTestId("viewport-deformation-boundary")).toContainText(
  589 |     "scale=normalized_display_offset_not_physical_length"
  590 |   );
  591 |   // TP-APP-R2-DEFORMEDDIR-001: the canned preview fixture now carries signed
  592 |   // global ux/uy/uz rows, so the overlay must disclose true directional
  593 |   // rendering instead of vector_direction=TBD.
  594 |   await expect(page.getByTestId("viewport-deformation-boundary")).toContainText(
  595 |     "vector_direction=global_cartesian_displacement_components"
  596 |   );
  597 |   await setDisclosure(page.getByTestId("viewport-deformation-status"), false);
  598 | 
  599 |   await page.getByTestId("audit-drawer-toggle").click();
  600 |   const auditDrawer = page.getByTestId("audit-boundary-drawer");
  601 |   await expect(auditDrawer).toBeVisible();
  602 |   await expect(auditDrawer.getByTestId("secret-private-library-unit-policy")).toContainText("unit_refs=2");
  603 |   await expect(auditDrawer.getByTestId("secret-private-library-unit-policy")).toContainText("required=true");
  604 |   await expect(auditDrawer.getByTestId("secret-private-library-unit-policy")).toContainText("payload=false");
  605 |   await expect(auditDrawer.getByTestId("secret-private-library-unit-policy")).toContainText("conversion=false");
  606 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("model=angle=rad,force=N,length=m");
  607 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("N*m/rad,N/m");
  608 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("rows=830");
  609 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("source=result_envelope");
  610 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("conversion=false");
  611 |   await auditDrawer.getByRole("button", { name: /Close/i }).click();
  612 |   await openWorkspaceSection(page, "solve");
  613 |   await expect(page.getByTestId("knowledge-unit-context")).toContainText("computed_unit_refs=2");
  614 |   await expect(page.getByTestId("knowledge-unit-context")).toContainText("units=N,mm");
  615 |   await expect(page.getByTestId("knowledge-unit-context")).toContainText("source=computed_preview_result");
  616 |   await expect(page.getByTestId("knowledge-unit-context")).toContainText("conversion=false");
  617 | 
  618 |   const solvedCanvasBounds = await canvas.boundingBox();
  619 |   expect(solvedCanvasBounds).not.toBeNull();
  620 |   expect(solvedCanvasBounds!.width).toBeGreaterThanOrEqual(200);
  621 |   expect(solvedCanvasBounds!.height).toBeGreaterThanOrEqual(200);
  622 |   const solvedCanvas = await canvas.screenshot();
  623 |   expect(pngStats(solvedCanvas).uniqueColors).toBeGreaterThan(100);
  624 | 
  625 |   await openWorkspaceSection(page, "results");
  626 |   await expect(page.getByTestId("results-panel")).toBeVisible();
  627 |   await expect(page.getByTestId("result-unit-policy")).toContainText("MPa, N, N*m, mm, rad");
  628 |   await expect(page.getByTestId("result-unit-policy")).toContainText("830 rows");
  629 |   await expect(page.getByTestId("result-unit-policy")).toContainText("entered units preserved");
  630 |   await expect(page.getByTestId("result-filter-summary")).toContainText("830 of 830 results match filter");
  631 |   await expect(page.getByTestId("result-family-count-reaction")).toContainText("29");
  632 |   await page.getByTestId("result-family-reaction").click();
  633 |   await expect(page.getByTestId("result-filter-summary")).toContainText("29 of 830 results match filter");
  634 |   await expect(page.getByTestId("result-page-summary")).toContainText(
  635 |     "Showing 1 to 29 of 29 matching results; page 1 of 1"
  636 |   );
  637 |   await expect(page.getByTestId("result-row-result:reaction:support-S-120")).toBeVisible();
  638 |   await page.getByTestId("result-family-all").click();
  639 |   await expect(page.getByTestId("result-filter-summary")).toContainText("830 of 830 results match filter");
  640 |   await page.getByTestId("result-filter-input").fill("pipe-P-120");
  641 |   await expect(page.getByTestId("result-filter-summary")).toContainText("170 of 830 results match filter");
  642 |   await expect(page.getByTestId("result-page-summary")).toContainText(
  643 |     "Showing 1 to 50 of 170 matching results; page 1 of 4"
```