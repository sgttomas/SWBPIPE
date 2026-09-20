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
      - button "Select (⎋)" [ref=e28] [cursor=pointer]:
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
      - button "Inspector" [expanded] [ref=e60] [cursor=pointer]:
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
          - button "Issues, 5" [ref=e125] [cursor=pointer]:
            - img [ref=e126]
            - generic [ref=e128]: Issues
            - generic [ref=e129]: "5"
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
              - searchbox "Filter model tree" [ref=e159]: pipe:P-130
            - generic [ref=e160]: 3 of 27 model entities visible
            - button "Clear model tree filter" [ref=e161] [cursor=pointer]:
              - img [ref=e162]
          - tree "Model" [ref=e165]:
            - generic [ref=e166]:
              - treeitem "Pipes" [expanded] [level=1] [ref=e168] [cursor=pointer]:
                - generic [ref=e169]: ▾
                - strong [ref=e170]: Pipes
              - treeitem "Tie-in rise pipe:P-130" [level=2] [selected] [ref=e172] [cursor=pointer]:
                - img [ref=e173]
                - generic [ref=e177]:
                  - strong [ref=e178]: Tie-in rise
                  - generic [ref=e179]: pipe:P-130
              - treeitem "Components" [expanded] [level=1] [ref=e181] [cursor=pointer]:
                - generic [ref=e182]: ▾
                - strong [ref=e183]: Components
              - treeitem "Invented semi-rigid valve marker component:C-130" [level=2] [ref=e185] [cursor=pointer]:
                - img [ref=e186]
                - generic [ref=e193]:
                  - strong [ref=e194]: Invented semi-rigid valve marker
                  - generic [ref=e195]: component:C-130
              - treeitem "Invented expansion joint marker component:C-150" [level=2] [ref=e197] [cursor=pointer]:
                - img [ref=e198]
                - generic [ref=e205]:
                  - strong [ref=e206]: Invented expansion joint marker
                  - generic [ref=e207]: component:C-150
      - separator "Resize table and canvas" [ref=e208]
      - generic [ref=e210]:
        - generic [ref=e211]:
          - group "Viewport controls" [ref=e212]:
            - generic [ref=e213]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e214]:
              - generic "Deformation · unavailable" [ref=e215] [cursor=pointer]
            - group "Viewport display toggles" [ref=e216]:
              - button "Labels" [pressed] [ref=e217]
              - button "Loads" [pressed] [ref=e218]
              - button "Grid" [pressed] [ref=e219]
            - group "Viewport selection tools" [ref=e220]:
              - button "Box Select" [ref=e221]
              - generic [ref=e222]:
                - generic [ref=e223]: Selection filter
                - combobox "Selection filter" [ref=e224]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [ref=e225]
              - button "Isolate" [ref=e226]
              - button "Show All" [disabled] [ref=e227]
              - button "Fit Model" [ref=e228]
              - button "Fit Visible" [ref=e229]
              - button "Fit Selection" [ref=e230]
            - group "Viewport geometry" [ref=e231]:
              - button "Schematic" [pressed] [ref=e232]
              - button "Actual OD" [ref=e233]
              - button "Measure" [ref=e234]
          - generic "Viewport status" [ref=e235]:
            - 'generic "Selected pipe: pipe:P-130" [ref=e236]': "Selected: pipe:P-130"
            - status "Schematic centerline geometry" [ref=e237]
            - status "View command status" [ref=e238]: No view command dispatched.
        - generic [ref=e239]:
          - generic "Three.js pipe centerline viewport" [ref=e240]
          - generic "Viewport entity selection":
            - button "Select Tie-in rise in viewport" [pressed] [ref=e242] [cursor=pointer]:
              - img [ref=e243]
              - generic [ref=e247]: P-130
            - button "Select Vertical riser in viewport" [ref=e248] [cursor=pointer]:
              - img [ref=e249]
              - generic [ref=e253]: P-110
            - button "Select Invented branch connection marker in viewport" [ref=e254] [cursor=pointer]:
              - img [ref=e255]
              - generic [ref=e258]: C-120
            - button "Select Low point elbow in viewport" [ref=e259] [cursor=pointer]:
              - img [ref=e260]
              - generic [ref=e263]: N-110
            - button "Select Riser elbow in viewport" [ref=e264] [cursor=pointer]:
              - img [ref=e265]
              - generic [ref=e268]: N-120
            - button "Select Rack span in viewport" [ref=e269] [cursor=pointer]:
              - img [ref=e270]
              - generic [ref=e274]: P-120
            - button "Select Invented semi-rigid valve marker in viewport" [ref=e275] [cursor=pointer]:
              - img [ref=e276]
              - generic [ref=e279]: C-130
            - button "Select Pump nozzle in viewport" [ref=e280] [cursor=pointer]:
              - img [ref=e281]
              - generic [ref=e284]: N-100
            - button "Select Rack turn in viewport" [ref=e285] [cursor=pointer]:
              - img [ref=e286]
              - generic [ref=e289]: N-130
            - button "Select Terminal tie-in in viewport" [ref=e290] [cursor=pointer]:
              - img [ref=e291]
              - generic [ref=e294]: N-140
            - button "Select Preview one-way terminal stop in viewport" [ref=e295] [cursor=pointer]:
              - img [ref=e296]
              - generic [ref=e299]: NL-140
            - button "Select Anchor at pump nozzle in viewport" [ref=e300] [cursor=pointer]:
              - img [ref=e301]
              - generic [ref=e304]: S-100
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e305]:
            - button "Front" [ref=e306] [cursor=pointer]
            - button "Top" [ref=e307] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e308] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e309]:
          - generic "Object creation tools" [ref=e310]:
            - button "Node" [ref=e311] [cursor=pointer]:
              - img [ref=e312]
              - text: Node
            - button "Pipe" [active] [pressed] [ref=e314] [cursor=pointer]:
              - img [ref=e315]
              - text: Pipe
            - button "Support" [ref=e319] [cursor=pointer]:
              - img [ref=e320]
              - text: Support
            - button "Component" [ref=e323] [cursor=pointer]:
              - img [ref=e324]
              - text: Component
            - button "Load" [ref=e327] [cursor=pointer]:
              - img [ref=e328]
              - text: Load
          - 'generic "Pipe tool armed: pick from/to nodes or complete the pipe form." [ref=e330]': Pipe tool armed
          - group [ref=e331]:
            - generic "Selection & navigation" [ref=e332] [cursor=pointer]
      - generic [ref=e333]:
        - button "Close inspector" [ref=e334]:
          - img [ref=e335]
        - region "Viewport editor intents" [ref=e339]:
          - heading "Create pipe" [level=3] [ref=e340]
          - group [ref=e341]:
            - text: ▾
            - generic "Explicit straight pipe connectivity" [ref=e342]:
              - generic [ref=e343]:
                - generic [ref=e344]: Pipe ID
                - textbox "New pipe ID" [ref=e345]:
                  - /placeholder: pipe:P-2
              - generic [ref=e346]:
                - generic [ref=e347]: Label
                - textbox "New pipe label" [ref=e348]:
                  - /placeholder: Pipe label
              - generic [ref=e349]:
                - generic [ref=e350]:
                  - generic [ref=e351]: From
                  - button "Pick" [pressed] [ref=e352] [cursor=pointer]:
                    - img [ref=e353]
                    - text: Pick
                - generic [ref=e356]:
                  - generic [ref=e357]:
                    - generic [ref=e358]: New pipe from node
                    - combobox "New pipe from node" [expanded] [ref=e359]
                  - generic "New pipe from node current value" [ref=e360]: No target selected
                  - generic [ref=e361]: 5 of 5 targets
                  - listbox "New pipe from node options" [ref=e362]:
                    - generic [ref=e363]:
                      - option "Pump nozzle node:N-100" [ref=e365] [cursor=pointer]:
                        - generic [ref=e366]: Pump nozzle
                        - generic [ref=e367]: node:N-100
                      - option "Low point elbow node:N-110" [ref=e369] [cursor=pointer]:
                        - generic [ref=e370]: Low point elbow
                        - generic [ref=e371]: node:N-110
                      - option "Riser elbow node:N-120" [ref=e373] [cursor=pointer]:
                        - generic [ref=e374]: Riser elbow
                        - generic [ref=e375]: node:N-120
                      - option "Rack turn node:N-130" [ref=e377] [cursor=pointer]:
                        - generic [ref=e378]: Rack turn
                        - generic [ref=e379]: node:N-130
                      - option "Terminal tie-in node:N-140" [ref=e381] [cursor=pointer]:
                        - generic [ref=e382]: Terminal tie-in
                        - generic [ref=e383]: node:N-140
              - group "End mode" [ref=e384]:
                - generic [ref=e385]: End mode
                - generic [ref=e386]:
                  - radio "Existing node" [checked] [ref=e387]
                  - text: Existing node
                - generic [ref=e388]:
                  - radio "New node" [ref=e389]
                  - text: New node
              - generic [ref=e390]:
                - generic [ref=e391]:
                  - generic [ref=e392]: To
                  - button "Pick" [ref=e393] [cursor=pointer]:
                    - img [ref=e394]
                    - text: Pick
                - generic [ref=e397]:
                  - generic [ref=e398]:
                    - generic [ref=e399]: New pipe to node
                    - combobox "New pipe to node" [expanded] [ref=e400]
                  - generic "New pipe to node current value" [ref=e401]: No target selected
                  - generic [ref=e402]: 5 of 5 targets
                  - listbox "New pipe to node options" [ref=e403]:
                    - generic [ref=e404]:
                      - option "Pump nozzle node:N-100" [ref=e406] [cursor=pointer]:
                        - generic [ref=e407]: Pump nozzle
                        - generic [ref=e408]: node:N-100
                      - option "Low point elbow node:N-110" [ref=e410] [cursor=pointer]:
                        - generic [ref=e411]: Low point elbow
                        - generic [ref=e412]: node:N-110
                      - option "Riser elbow node:N-120" [ref=e414] [cursor=pointer]:
                        - generic [ref=e415]: Riser elbow
                        - generic [ref=e416]: node:N-120
                      - option "Rack turn node:N-130" [ref=e418] [cursor=pointer]:
                        - generic [ref=e419]: Rack turn
                        - generic [ref=e420]: node:N-130
                      - option "Terminal tie-in node:N-140" [ref=e422] [cursor=pointer]:
                        - generic [ref=e423]: Terminal tie-in
                        - generic [ref=e424]: node:N-140
              - text: ▾ ▾
              - generic [ref=e425]:
                - generic [ref=e426]:
                  - generic [ref=e427]: New pipe material
                  - combobox "New pipe material" [expanded] [ref=e428]
                - generic "New pipe material current value" [ref=e429]: No target selected
                - generic [ref=e430]: 1 of 1 targets
                - listbox "New pipe material options" [ref=e431]:
                  - option "Invented carbon-steel-like material material:invented-carbon-steel" [ref=e434] [cursor=pointer]:
                    - generic [ref=e435]: Invented carbon-steel-like material
                    - generic [ref=e436]: material:invented-carbon-steel
              - generic [ref=e437]:
                - generic [ref=e438]: OD
                - textbox "New pipe outside diameter" [ref=e439]:
                  - /placeholder: "0.114"
              - generic [ref=e440]:
                - generic [ref=e441]: Wall
                - textbox "New pipe wall thickness" [ref=e442]:
                  - /placeholder: "0.006"
              - generic [ref=e443]:
                - generic [ref=e444]: Length unit
                - combobox "New pipe length unit" [ref=e446] [cursor=pointer]:
                  - generic [ref=e447]: m
                  - text: ▾
              - generic [ref=e448]: "Pipe geometry: m, model metadata"
              - generic [ref=e449]: "Construction plane inactive: existing endpoint uses exact node IDs."
              - generic [ref=e450]: No route ghost is visible.
              - generic [ref=e451]:
                - generic [ref=e452]: Yref X
                - textbox "New pipe y-reference X" [ref=e453]:
                  - /placeholder: "0"
              - generic [ref=e454]:
                - generic [ref=e455]: Yref Y
                - textbox "New pipe y-reference Y" [ref=e456]:
                  - /placeholder: "0"
              - generic [ref=e457]:
                - generic [ref=e458]: Yref Z
                - textbox "New pipe y-reference Z" [ref=e459]:
                  - /placeholder: "1"
              - generic [ref=e460]:
                - generic [ref=e461]: Provenance
                - textbox "New pipe provenance" [ref=e462]
              - generic [ref=e463]:
                - checkbox "Continue from end after Apply; keep the entered material, dimensions, orientation and provenance" [ref=e464]
                - text: Continue from end after Apply; keep the entered material, dimensions, orientation and provenance
              - button "Cancel pipe draft" [ref=e465] [cursor=pointer]
              - button "Add route" [disabled] [ref=e466]:
                - img [ref=e467]
                - text: Add route
              - generic [ref=e471]: Choose an existing start node.; Choose an existing end node.; Enter a pipe ID.; Enter a pipe label.; Choose an existing material ID.; Enter positive outside diameter and wall thickness values.; Enter a finite, nonzero y-reference vector.; Enter pipe provenance.
            - text: ▾ ▾ ▾
            - region "Route review" [ref=e472]:
              - heading "Review and Apply" [level=4] [ref=e473]
              - status [ref=e474]: The affected selection changed. Add again to review the current draft.
              - paragraph [ref=e475]: Add a complete node or route to generate the service validation and exact diff.
              - button "Apply" [disabled] [ref=e476]
            - group [ref=e477]:
              - generic "Unit source" [ref=e478] [cursor=pointer]
              - text: browser preview uses model metadata for viewport length units
          - group [ref=e479]:
            - generic "Pending changes (0)" [ref=e480] [cursor=pointer]
        - region "Property inspector" [ref=e481]:
          - 'heading "Tie-in rise — pipe: pipe:P-130" [level=2] [ref=e482]':
            - text: Tie-in rise
            - generic [ref=e483]: "— pipe: pipe:P-130"
          - tablist "Inspector views" [ref=e484]:
            - tab "Properties" [ref=e485]
            - tab "Task" [selected] [ref=e486]
          - tabpanel "Editor operation intent" [ref=e487]:
            - generic [ref=e489]: "Draft target: pipe: pipe:P-130"
            - heading "Edit name" [level=3] [ref=e490]:
              - img [ref=e491]
              - text: Edit name
            - generic [ref=e494]:
              - generic [ref=e495]:
                - generic [ref=e496]: Property
                - combobox "Property to edit" [ref=e498] [cursor=pointer]:
                  - generic [ref=e499]: Name
                  - text: ▾
              - region "Current value" [ref=e500]:
                - generic [ref=e501]: Current name
                - strong [ref=e502]: Tie-in rise
              - generic [ref=e503]:
                - generic [ref=e504]: New name
                - textbox "New name" [ref=e505]: Tie-in rise
              - generic "Task actions" [ref=e506]:
                - button "Add" [disabled] [ref=e507]:
                  - img [ref=e508]
                  - text: Add
                - button "Cancel" [ref=e509] [cursor=pointer]
                - button "Review" [disabled] [ref=e510]: Review
                - button "Apply" [disabled] [ref=e514]:
                  - img [ref=e515]
                  - text: Apply
            - group [ref=e518]:
              - generic "Operation details" [ref=e519] [cursor=pointer]
            - paragraph [ref=e520]: Validate to check this change before applying it.
          - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e521]:
      - button "Agent" [disabled] [ref=e523]:
        - img [ref=e524]
        - generic [ref=e527]: Agent
  - generic "Workspace status" [ref=e528]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e529] [cursor=pointer]:
      - img [ref=e530]
      - text: 5 Issues
    - generic "Selection" [ref=e532]: "pipe: pipe:P-130"
    - generic "Display units" [ref=e533]: Entered
    - button "About SWBPIPE…" [ref=e534] [cursor=pointer]:
      - img [ref=e535]
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