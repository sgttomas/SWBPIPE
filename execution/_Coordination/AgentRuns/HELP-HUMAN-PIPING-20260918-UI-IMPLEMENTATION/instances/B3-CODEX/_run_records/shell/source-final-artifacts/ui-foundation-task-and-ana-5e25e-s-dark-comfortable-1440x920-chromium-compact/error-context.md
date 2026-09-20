# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ui-foundation.spec.ts >> task and analysis dock preserve usable canvas dark comfortable 1440x920
- Location: e2e/ui-foundation.spec.ts:671:7

# Error details

```
Error: expect(received).toBeLessThanOrEqual(expected)

Expected: <= 897
Received:    1408.5
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
      - option "System"
      - option "Light"
      - option "Dark" [selected]
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
                - searchbox "Filter model tree" [ref=e162]: node:N-100
              - generic [ref=e163]: 3 of 27 model entities visible
              - button "Clear model tree filter" [ref=e164] [cursor=pointer]:
                - img [ref=e165]
            - tree "Model" [ref=e168]:
              - generic [ref=e169]:
                - treeitem "Nodes" [expanded] [level=1] [ref=e171] [cursor=pointer]:
                  - generic [ref=e172]: ▾
                  - strong [ref=e173]: Nodes
                - treeitem "Pump nozzle node:N-100" [level=2] [selected] [ref=e175] [cursor=pointer]:
                  - img [ref=e176]
                  - generic [ref=e179]:
                    - strong [ref=e180]: Pump nozzle
                    - generic [ref=e181]: node:N-100
                - treeitem "Pipes" [expanded] [level=1] [ref=e183] [cursor=pointer]:
                  - generic [ref=e184]: ▾
                  - strong [ref=e185]: Pipes
                - treeitem "Pump discharge run pipe:P-100" [level=2] [ref=e187] [cursor=pointer]:
                  - img [ref=e188]
                  - generic [ref=e192]:
                    - strong [ref=e193]: Pump discharge run
                    - generic [ref=e194]: pipe:P-100
                - treeitem "Supports" [expanded] [level=1] [ref=e196] [cursor=pointer]:
                  - generic [ref=e197]: ▾
                  - strong [ref=e198]: Supports
                - treeitem "Anchor at pump nozzle support:S-100" [level=2] [ref=e200] [cursor=pointer]:
                  - img [ref=e201]
                  - generic [ref=e204]:
                    - strong [ref=e205]: Anchor at pump nozzle
                    - generic [ref=e206]: support:S-100
        - separator "Resize table and canvas" [ref=e207]
        - generic [ref=e209]:
          - generic [ref=e210]:
            - group "Viewport controls" [ref=e211]:
              - generic [ref=e212]: 3D Centerline
              - group "Viewport deformation overlay status" [ref=e213]:
                - generic "Deformation · unavailable" [ref=e214] [cursor=pointer]
              - group "Viewport display toggles" [ref=e215]:
                - button "Labels" [pressed] [ref=e216]
                - button "Loads" [pressed] [ref=e217]
                - button "Grid" [pressed] [ref=e218]
              - group "Viewport selection tools" [ref=e219]:
                - button "Box Select" [ref=e220]
                - generic [ref=e221]:
                  - generic [ref=e222]: Selection filter
                  - combobox "Selection filter" [ref=e223]:
                    - option "All" [selected]
                    - option "Pipes"
                    - option "Nodes"
                    - option "Supports"
                    - option "Components"
                - button "Hide" [ref=e224]
                - button "Isolate" [ref=e225]
                - button "Show All" [disabled] [ref=e226]
                - button "Fit Model" [ref=e227]
                - button "Fit Visible" [ref=e228]
                - button "Fit Selection" [ref=e229]
              - group "Viewport geometry" [ref=e230]:
                - button "Schematic" [pressed] [ref=e231]
                - button "Actual OD" [ref=e232]
                - button "Measure" [ref=e233]
            - generic "Viewport status" [ref=e234]:
              - 'generic "Selected node: node:N-100" [ref=e235]': "Selected: node:N-100"
              - status "Schematic centerline geometry" [ref=e236]
              - status "View command status" [ref=e237]: No view command dispatched.
          - generic [ref=e238]:
            - generic "Three.js pipe centerline viewport" [ref=e239]
            - generic "Viewport entity selection":
              - button "Select Pump nozzle in viewport" [pressed] [ref=e241] [cursor=pointer]:
                - img [ref=e242]
                - generic [ref=e245]: N-100
              - button "Select Vertical riser in viewport" [ref=e246] [cursor=pointer]:
                - img [ref=e247]
                - generic [ref=e251]: P-110
              - button "Select Invented branch connection marker in viewport" [ref=e252] [cursor=pointer]:
                - img [ref=e253]
                - generic [ref=e256]: C-120
              - button "Select Low point elbow in viewport" [ref=e257] [cursor=pointer]:
                - img [ref=e258]
                - generic [ref=e261]: N-110
              - button "Select Riser elbow in viewport" [ref=e262] [cursor=pointer]:
                - img [ref=e263]
                - generic [ref=e266]: N-120
              - button "Select Rack span in viewport" [ref=e267] [cursor=pointer]:
                - img [ref=e268]
                - generic [ref=e272]: P-120
              - button "Select Pump discharge run in viewport" [ref=e273] [cursor=pointer]:
                - img [ref=e274]
                - generic [ref=e278]: P-100
              - button "Select Tie-in rise in viewport" [ref=e279] [cursor=pointer]:
                - img [ref=e280]
                - generic [ref=e284]: P-130
              - button "Select Invented semi-rigid valve marker in viewport" [ref=e285] [cursor=pointer]:
                - img [ref=e286]
                - generic [ref=e289]: C-130
              - button "Select Rack turn in viewport" [ref=e290] [cursor=pointer]:
                - img [ref=e291]
                - generic [ref=e294]: N-130
              - button "Select Terminal tie-in in viewport" [ref=e295] [cursor=pointer]:
                - img [ref=e296]
                - generic [ref=e299]: N-140
              - button "Select Preview sliding-friction rack shoe in viewport" [ref=e300] [cursor=pointer]:
                - img [ref=e301]
                - generic [ref=e304]: NL-130-FRIC
              - button "Select Preview one-way terminal stop in viewport" [ref=e305] [cursor=pointer]:
                - img [ref=e306]
                - generic [ref=e309]: NL-140
              - button "Select Anchor at pump nozzle in viewport" [ref=e310] [cursor=pointer]:
                - img [ref=e311]
                - generic [ref=e314]: S-100
            - img "Orientation gizmo showing X, Y, Z axes"
            - generic "View controls" [ref=e315]:
              - button "Front" [ref=e316] [cursor=pointer]
              - button "Top" [ref=e317] [cursor=pointer]
              - button "Isometric" [pressed] [ref=e318] [cursor=pointer]
            - generic:
              - generic: 1 m
          - region "Command and selection bar" [ref=e319]:
            - generic "Object creation tools" [ref=e320]:
              - button "Node" [ref=e321] [cursor=pointer]:
                - img [ref=e322]
                - text: Node
              - button "Pipe" [pressed] [ref=e324] [cursor=pointer]:
                - img [ref=e325]
                - text: Pipe
              - button "Support" [ref=e329] [cursor=pointer]:
                - img [ref=e330]
                - text: Support
              - button "Component" [ref=e333] [cursor=pointer]:
                - img [ref=e334]
                - text: Component
              - button "Load" [ref=e337] [cursor=pointer]:
                - img [ref=e338]
                - text: Load
            - 'generic "Pipe tool armed: pick from/to nodes or complete the pipe form." [ref=e340]': Pipe tool armed
            - group [ref=e341]:
              - generic "Selection & navigation" [ref=e342] [cursor=pointer]
        - generic [ref=e343]:
          - button "Close inspector" [ref=e344]:
            - img [ref=e345]
          - region "Viewport editor intents" [ref=e349]:
            - heading "Create pipe" [level=3] [ref=e350]
            - group [ref=e351]:
              - generic "Explicit straight pipe connectivity" [ref=e352]:
                - generic [ref=e353]:
                  - generic [ref=e354]: Pipe ID
                  - textbox "New pipe ID" [ref=e355]:
                    - /placeholder: pipe:P-2
                - generic [ref=e356]:
                  - generic [ref=e357]: Label
                  - textbox "New pipe label" [ref=e358]:
                    - /placeholder: Pipe label
                    - text: Retained pipe draft
                - generic [ref=e359]:
                  - generic [ref=e360]:
                    - generic [ref=e361]: From
                    - button "Pick" [pressed] [ref=e362] [cursor=pointer]:
                      - img [ref=e363]
                      - text: Pick
                  - generic [ref=e366]:
                    - generic [ref=e367]:
                      - generic [ref=e368]: New pipe from node
                      - combobox "New pipe from node" [expanded] [ref=e369]
                    - generic "New pipe from node current value" [ref=e370]: No target selected
                    - generic [ref=e371]: 5 of 5 targets
                    - listbox "New pipe from node options" [ref=e372]:
                      - generic [ref=e373]:
                        - option "Pump nozzle node:N-100" [ref=e375] [cursor=pointer]:
                          - generic [ref=e376]: Pump nozzle
                          - generic [ref=e377]: node:N-100
                        - option "Low point elbow node:N-110" [ref=e379] [cursor=pointer]:
                          - generic [ref=e380]: Low point elbow
                          - generic [ref=e381]: node:N-110
                        - option "Riser elbow node:N-120" [ref=e383] [cursor=pointer]:
                          - generic [ref=e384]: Riser elbow
                          - generic [ref=e385]: node:N-120
                        - option "Rack turn node:N-130" [ref=e387] [cursor=pointer]:
                          - generic [ref=e388]: Rack turn
                          - generic [ref=e389]: node:N-130
                        - option "Terminal tie-in node:N-140" [ref=e391] [cursor=pointer]:
                          - generic [ref=e392]: Terminal tie-in
                          - generic [ref=e393]: node:N-140
                - group "End mode" [ref=e394]:
                  - generic [ref=e395]: End mode
                  - generic [ref=e396]:
                    - radio "Existing node" [checked] [ref=e397]
                    - text: Existing node
                  - generic [ref=e398]:
                    - radio "New node" [ref=e399]
                    - text: New node
                - generic [ref=e400]:
                  - generic [ref=e401]:
                    - generic [ref=e402]: To
                    - button "Pick" [ref=e403] [cursor=pointer]:
                      - img [ref=e404]
                      - text: Pick
                  - generic [ref=e407]:
                    - generic [ref=e408]:
                      - generic [ref=e409]: New pipe to node
                      - combobox "New pipe to node" [expanded] [ref=e410]
                    - generic "New pipe to node current value" [ref=e411]: No target selected
                    - generic [ref=e412]: 5 of 5 targets
                    - listbox "New pipe to node options" [ref=e413]:
                      - generic [ref=e414]:
                        - option "Pump nozzle node:N-100" [ref=e416] [cursor=pointer]:
                          - generic [ref=e417]: Pump nozzle
                          - generic [ref=e418]: node:N-100
                        - option "Low point elbow node:N-110" [ref=e420] [cursor=pointer]:
                          - generic [ref=e421]: Low point elbow
                          - generic [ref=e422]: node:N-110
                        - option "Riser elbow node:N-120" [ref=e424] [cursor=pointer]:
                          - generic [ref=e425]: Riser elbow
                          - generic [ref=e426]: node:N-120
                        - option "Rack turn node:N-130" [ref=e428] [cursor=pointer]:
                          - generic [ref=e429]: Rack turn
                          - generic [ref=e430]: node:N-130
                        - option "Terminal tie-in node:N-140" [ref=e432] [cursor=pointer]:
                          - generic [ref=e433]: Terminal tie-in
                          - generic [ref=e434]: node:N-140
                - generic [ref=e435]:
                  - generic [ref=e436]:
                    - generic [ref=e437]: New pipe material
                    - combobox "New pipe material" [expanded] [ref=e438]
                  - generic "New pipe material current value" [ref=e439]: No target selected
                  - generic [ref=e440]: 1 of 1 targets
                  - listbox "New pipe material options" [ref=e441]:
                    - option "Invented carbon-steel-like material material:invented-carbon-steel" [ref=e444] [cursor=pointer]:
                      - generic [ref=e445]: Invented carbon-steel-like material
                      - generic [ref=e446]: material:invented-carbon-steel
                - generic [ref=e447]:
                  - generic [ref=e448]: OD
                  - textbox "New pipe outside diameter" [ref=e449]:
                    - /placeholder: "0.114"
                - generic [ref=e450]:
                  - generic [ref=e451]: Wall
                  - textbox "New pipe wall thickness" [ref=e452]:
                    - /placeholder: "0.006"
                - generic [ref=e453]:
                  - generic [ref=e454]: Length unit
                  - combobox "New pipe length unit" [ref=e455]:
                    - option "m" [selected]
                - generic [ref=e456]: "Pipe geometry: m, model metadata"
                - generic [ref=e457]: "Construction plane inactive: existing endpoint uses exact node IDs."
                - generic [ref=e458]: No route ghost is visible.
                - generic [ref=e459]:
                  - generic [ref=e460]: Yref X
                  - textbox "New pipe y-reference X" [ref=e461]:
                    - /placeholder: "0"
                - generic [ref=e462]:
                  - generic [ref=e463]: Yref Y
                  - textbox "New pipe y-reference Y" [ref=e464]:
                    - /placeholder: "0"
                - generic [ref=e465]:
                  - generic [ref=e466]: Yref Z
                  - textbox "New pipe y-reference Z" [ref=e467]:
                    - /placeholder: "1"
                - generic [ref=e468]:
                  - generic [ref=e469]: Provenance
                  - textbox "New pipe provenance" [ref=e470]: layout regression draft
                - generic [ref=e471]:
                  - checkbox "Continue from end after Apply; keep the entered material, dimensions, orientation and provenance" [ref=e472]
                  - text: Continue from end after Apply; keep the entered material, dimensions, orientation and provenance
                - button "Cancel pipe draft" [ref=e473] [cursor=pointer]
                - button "Add route" [disabled] [ref=e474]:
                  - img [ref=e475]
                  - text: Add route
                - generic [ref=e479]: Choose an existing start node.; Choose an existing end node.; Enter a pipe ID.; Choose an existing material ID.; Enter positive outside diameter and wall thickness values.; Enter a finite, nonzero y-reference vector.
              - region "Route review" [ref=e480]:
                - heading "Review and Apply" [level=4] [ref=e481]
                - status [ref=e482]: The affected selection changed. Add again to review the current draft.
                - paragraph [ref=e483]: Add a complete node or route to generate the service validation and exact diff.
                - button "Apply" [disabled] [ref=e484]
              - group [ref=e485]:
                - generic "Unit source" [ref=e486] [cursor=pointer]
            - group [ref=e487]:
              - generic "Pending changes (0)" [ref=e488] [cursor=pointer]
          - region "Property inspector" [ref=e489]:
            - 'heading "Pump nozzle — node: node:N-100" [level=2] [ref=e490]':
              - text: Pump nozzle
              - generic [ref=e491]: "— node: node:N-100"
            - tablist "Inspector views" [ref=e492]:
              - tab "Properties" [ref=e493]
              - tab "Task" [selected] [ref=e494]
            - tabpanel "Editor operation intent" [ref=e495]:
              - generic [ref=e497]: "Draft target: node: node:N-100"
              - heading "Edit name" [level=3] [ref=e498]:
                - img [ref=e499]
                - text: Edit name
              - generic [ref=e502]:
                - generic [ref=e503]:
                  - generic [ref=e504]: Property
                  - combobox "Property to edit" [ref=e505]:
                    - option "Name" [selected]
                    - option "X position"
                    - option "Y position"
                    - option "Z position"
                    - option "Provenance"
                - region "Current value" [ref=e506]:
                  - generic [ref=e507]: Current name
                  - strong [ref=e508]: Pump nozzle
                - generic [ref=e509]:
                  - generic [ref=e510]: New name
                  - textbox "New name" [ref=e511]: Retained inspector task
                - generic "Task actions" [ref=e512]:
                  - button "Add" [ref=e513] [cursor=pointer]:
                    - img [ref=e514]
                    - text: Add
                  - button "Cancel" [ref=e515] [cursor=pointer]
                  - button "Review" [ref=e516] [cursor=pointer]: Review
                  - button "Apply" [ref=e520] [cursor=pointer]:
                    - img [ref=e521]
                    - text: Apply
              - group [ref=e524]:
                - generic "Operation details" [ref=e525] [cursor=pointer]
              - paragraph [ref=e526]: Validate to check this change before applying it.
      - region "Workspace sections" [ref=e527]:
        - generic [ref=e528]:
          - heading "Analyze" [level=2] [ref=e529]
          - button "Close" [ref=e530] [cursor=pointer]:
            - img [ref=e531]
            - text: Close
        - region "Solve section" [ref=e535]:
          - region "Solve execution" [ref=e536]:
            - generic [ref=e537]: Execution
            - region "Solve readiness summary" [ref=e538]:
              - generic [ref=e539]:
                - img [ref=e540]
                - generic [ref=e542]: Mechanics readiness
                - strong [ref=e543]: preview run not started; ready for preview diagnostics
              - generic [ref=e544]:
                - img [ref=e545]
                - generic [ref=e547]: Rule-check readiness
                - strong [ref=e548]: Rule pack · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE); mechanics results remain reviewable only
              - generic [ref=e549]:
                - img [ref=e550]
                - generic [ref=e552]: Diagnostics
                - strong [ref=e553]: 1 diagnostic; 1 warning; 0 blocking/error
              - generic [ref=e554]:
                - img [ref=e555]
                - generic [ref=e557]: Professional boundary
                - strong [ref=e558]: Human · Human review required (HUMAN_REVIEW_REQUIRED)
            - generic [ref=e559]:
              - generic [ref=e560]:
                - generic [ref=e561]:
                  - checkbox "Include known private values in this local export" [ref=e562]
                  - text: Include known private values in this local export
                - generic [ref=e563]: decisions=93; findings=93; blocked=true
                - group [ref=e564]:
                  - generic "Details" [ref=e565] [cursor=pointer]
                - generic [ref=e566]:
                  - img [ref=e567]
                  - text: Solve job JSON
              - generic [ref=e570]: state=not_started; events=1; result_rows=0; cancellation_requested=false
            - generic [ref=e571]:
              - generic [ref=e572]:
                - generic [ref=e573]: Solver mode
                - strong [ref=e574]: selected=sparse_interactive; sparse_default=true; dense_scrutiny=false
              - generic [ref=e575]:
                - generic [ref=e576]: Progress
                - strong [ref=e577]: not_started; preview_service_event_state_only_no_percent_stream; percentages_synthesized=false
              - generic [ref=e578]:
                - generic [ref=e579]: Cancellation
                - strong [ref=e580]: control_visible=true; enabled=false; requested=false; seam=no_job_started; token=none_no_active_backend_job; success_claimed=false
              - generic [ref=e581]:
                - generic [ref=e582]: Result binding
                - strong [ref=e583]: not generated; result rows=0; hashes=0
              - generic [ref=e584]:
                - generic [ref=e585]: Unit policy
                - strong [ref=e586]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; results=none; rows=0; conversion=false
              - generic [ref=e587]:
                - generic [ref=e588]: Boundary
                - strong [ref=e589]: private payload=false; protected content=false; release/professional claim=false; human review required
            - group "Solver mode" [ref=e590]:
              - button "Sparse interactive" [pressed] [ref=e591] [cursor=pointer]:
                - img [ref=e592]
                - text: Sparse interactive
              - button "Dense scrutiny" [ref=e594] [cursor=pointer]:
                - img [ref=e595]
                - text: Dense scrutiny
            - button "Run mechanics preview" [ref=e598] [cursor=pointer]:
              - img [ref=e599]
              - text: Run mechanics preview
            - button "Cancel preview" [disabled] [ref=e601] [cursor=pointer]:
              - img [ref=e602]
              - text: Cancel preview
          - region "Rule-check completeness" [ref=e604]:
            - generic [ref=e605]:
              - img [ref=e606]
              - text: Rule-Check Completeness
            - generic [ref=e608]:
              - generic [ref=e609]:
                - generic [ref=e610]:
                  - checkbox "Include known private values in this local export" [ref=e611]
                  - text: Include known private values in this local export
                - generic [ref=e612]: decisions=122; findings=122; blocked=true
                - group [ref=e613]:
                  - generic "Details" [ref=e614] [cursor=pointer]
                - generic [ref=e615]:
                  - img [ref=e616]
                  - text: Local completeness JSON
              - generic [ref=e619]: 3 review findings; rule_check_blocked=true; mechanics_reviewable=false
            - generic [ref=e620]:
              - generic [ref=e621]:
                - generic [ref=e622]: Rule status
                - strong [ref=e623]: not_performed_user_rule_inputs_missing
              - generic [ref=e624]:
                - generic [ref=e625]: Mechanics status
                - strong [ref=e626]: ready_for_preview_diagnostics
              - generic [ref=e627]:
                - generic [ref=e628]: Unit policy
                - strong [ref=e629]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; records=38; rule_input_units=explicit_or_blocking; diagnostic=RULE_UNIT_MISMATCH; conversion=false
              - generic [ref=e630]:
                - generic [ref=e631]: Boundary
                - strong [ref=e632]: private rule data absent; silent defaults used=false; bundled code values=false; no compliance claim
              - generic [ref=e633]:
                - article [ref=e634]:
                  - strong [ref=e635]: RULE_CHECK_BLOCKING
                  - generic [ref=e636]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e637]: User-supplied rule-pack inputs are absent, so rule-check status stays incomplete while mechanics output remains separate. Provide a private rule pack and project-specific criteria with provenance; the public preview does not bundle code defaults.
                - article [ref=e638]:
                  - strong [ref=e639]: RULE_CHECK_BLOCKING
                  - generic [ref=e640]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e641]: Load cases and combinations are mechanics-basis preview inputs, not code-specific rule combinations. Supply private owner/code combination criteria before reporting user-rule checked status.
                - article [ref=e642]:
                  - strong [ref=e643]: PROVENANCE_WARNING
                  - generic [ref=e644]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e645]: Material and pipe-section values are invented public fixture values, not verified project or catalog data. Replace them with user-controlled data and provenance before engineering reliance.
            - generic [ref=e646]: Completeness findings explain missing user/private rule-check data.
          - region "Run rule checks" [ref=e647]:
            - generic [ref=e648]:
              - img [ref=e649]
              - text: Run Rule Checks (user-defined, private)
            - generic [ref=e652]:
              - generic [ref=e653]: "Project scope: project:invented-loop-01."
              - generic [ref=e654]: No solved mechanics result in this session; run a solve first so solver_result inputs can bind to result rows.
            - generic [ref=e655]:
              - button "Load demo rule pack" [ref=e656]
              - button "Refresh saved packs" [ref=e657]
            - generic [ref=e659]: Saved rule-pack list not refreshed yet.
            - generic [ref=e660]:
              - generic [ref=e661]: Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)
              - textbox "Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)" [ref=e662]:
                - /placeholder: No rule-pack document. Load the demo, open a saved pack, or paste one.
            - generic [ref=e663]:
              - button "Run checks" [disabled] [ref=e664]:
                - img [ref=e665]
                - text: Run checks
              - generic [ref=e667]: No rule-check run in this session.
            - generic [ref=e668]:
              - img [ref=e669]
              - text: Rule checks run locally over user-supplied data only. The software emits user-defined check statuses (USER_RULE_CHECKED / USER_RULE_FAILED / RULE_INPUTS_INCOMPLETE).
          - region "Design knowledge" [ref=e671]:
            - generic [ref=e672]:
              - img [ref=e673]
              - text: Design Knowledge
            - generic [ref=e675]:
              - article [ref=e676]:
                - generic [ref=e677]:
                  - strong [ref=e678]: Pump nozzle endpoint
                  - generic [ref=e679]: endpoint · review_required
                - paragraph [ref=e680]: Invented endpoint record used to show how user-supplied design context appears in the preview.
                - generic [ref=e681]: invented_example
              - article [ref=e682]:
                - generic [ref=e683]:
                  - strong [ref=e684]: Rack corridor placeholder
                  - generic [ref=e685]: routing_corridor · review_required
                - paragraph [ref=e686]: Invented corridor note; no owner standard or protected spacing criterion is bundled.
                - generic [ref=e687]: invented_example
              - article [ref=e688]:
                - generic [ref=e689]:
                  - strong [ref=e690]: Support stiffness dimensions unresolved
                  - generic [ref=e691]: assumption · unresolved
                - paragraph [ref=e692]: Linear and rotational support behavior is represented as a preview diagnostic until user-supplied stiffness values and canonical dimensions are provided.
                - generic [ref=e693]: invented_example
    - complementary "Agent" [ref=e694]:
      - button "Agent" [disabled] [ref=e696]:
        - img [ref=e697]
        - generic [ref=e700]: Agent
  - generic "Workspace status" [ref=e701]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e702] [cursor=pointer]:
      - img [ref=e703]
      - text: 5 Issues
    - generic "Selection" [ref=e705]: "node: node:N-100"
    - generic "Display units" [ref=e706]: Entered
    - button "About SWBPIPE…" [ref=e707] [cursor=pointer]:
      - img [ref=e708]
```

# Test source

```ts
  638 |     expect(after.projections, `${modifier ?? "plain"} Box Select authored projection`).toEqual(before.projections);
  639 |     expect(after.canvasRect, `${modifier ?? "plain"} Box Select published canvas rectangle`).toEqual(before.canvasRect);
  640 |     expect(after.canvasDomRect, `${modifier ?? "plain"} Box Select DOM canvas rectangle`).toEqual(before.canvasDomRect);
  641 |     expect(after.ownedPendingRafCount, `${modifier ?? "plain"} Box Select pending product RAF`).toBe(0);
  642 |     boxGestures.push({ modifier: modifier ?? "plain", path, before, after });
  643 |   }
  644 | 
  645 |   await testInfo.attach("box-select-camera-projection-invariance", {
  646 |     body: JSON.stringify({
  647 |       ordinaryOrbit: { path: ordinaryPath, before: ordinaryBefore, after: ordinaryAfter },
  648 |       activeInertiaTakeover: { immediate: activeInertia, before: takeoverBefore, after: takeoverAfter },
  649 |       navigationEndpoints: { firstIsometric, firstFit, orbitMoved, secondFit, rightPanMoved, frontEndpoint, isometricEndpoint },
  650 |       frozenLabelInterception: {
  651 |         sample: labelSample,
  652 |         ordinaryModeHit: ordinaryLabelHit,
  653 |         boxModeHit: boxLabelHit,
  654 |         boxDragPath: labelBoxPath,
  655 |       },
  656 |       boxGestures,
  657 |     }, null, 2),
  658 |     contentType: "application/json",
  659 |   });
  660 | });
  661 | 
  662 | // Frozen regression floor: authoring and analysis together must retain a
  663 | // 200 × 200 CSS-pixel canvas, with controls and drafts still usable. The floor is unchanged.
  664 | // Slice B3 moves the structure it is measured in: Analyze is a page that opens over the stage's
  665 | // surfaces (the dock under the canvas is gone), so the canvas keeps its box under the open page,
  666 | // the page's controls are checked with the page open, and the canvas's own controls and drafts are
  667 | // checked with it closed. The solve proof is read on the Results stage's Evidence tab.
  668 | for (const theme of APPEARANCE_THEMES) {
  669 |   for (const density of APPEARANCE_DENSITIES) {
  670 |     for (const viewport of APPEARANCE_VIEWPORTS) {
  671 |       test(`task and analysis dock preserve usable canvas ${theme} ${density} ${viewport.width}x${viewport.height}`, async ({ page }, testInfo) => {
  672 |         await page.setViewportSize(viewport);
  673 |         await page.goto("/");
  674 |         await setAppearance(page, theme, density);
  675 |         const closePage = async () => {
  676 |           const close = page.getByTestId("workspace-dock-close");
  677 |           if (await close.isVisible()) await activateWithKeyboard(page, close);
  678 |           await expect(page.getByTestId("workspace-dock")).toHaveClass(/collapsed/);
  679 |         };
  680 |         await startPropertyTaskFromTreeEntity(page, "node", "node:N-100");
  681 |         const inspector = page.getByTestId("property-inspector");
  682 |         await inspector.getByTestId("editor-intent-field").selectOption("label");
  683 |         await inspector.getByTestId("editor-intent-value").fill("Retained inspector task");
  684 |         if (viewport.width < 1280) await ensureRail(page, "inspector", false);
  685 |         await activateWithKeyboard(page, page.getByTestId("command-pipe"));
  686 |         const pipeForm = page.getByTestId("viewport-editor-intents");
  687 |         await expect(pipeForm).toHaveClass(/active/);
  688 |         await page.getByTestId("viewport-create-pipe-label").fill("Retained pipe draft");
  689 |         await page.getByTestId("viewport-create-pipe-provenance").fill("layout regression draft");
  690 |         await openWorkspaceSection(page, "solve");
  691 |         const dock = page.getByTestId("workspace-dock");
  692 |         await expect(dock).not.toHaveClass(/collapsed/);
  693 |         await expect(page.getByTestId("workspace-section-solve")).toBeVisible();
  694 | 
  695 |         const evidence: unknown[] = [];
  696 |         const measure = async (phase: string) => {
  697 |           await openWorkspaceSection(page, "solve");
  698 |           await expect(pipeForm).toHaveClass(/active/);
  699 |           await expect(dock).not.toHaveClass(/collapsed/);
  700 |           const geometry = await page.evaluate(() => {
  701 |             const rect = (selector: string) => {
  702 |               const element = document.querySelector<HTMLElement>(selector)!;
  703 |               const box = element.getBoundingClientRect();
  704 |               return { x: box.x, y: box.y, width: box.width, height: box.height, right: box.right, bottom: box.bottom };
  705 |             };
  706 |             const body = document.querySelector<HTMLElement>(".workspace-dock .workspace-dock-body")!;
  707 |             const bodyBox = body.getBoundingClientRect();
  708 |             const bodyClip = { top: Math.max(0, bodyBox.top + body.clientTop),
  709 |               bottom: Math.min(innerHeight, bodyBox.top + body.clientTop + body.clientHeight) };
  710 |             for (let ancestor = body.parentElement; ancestor; ancestor = ancestor.parentElement) {
  711 |               if (!/(auto|scroll|hidden|clip)/.test(getComputedStyle(ancestor).overflowY)) continue;
  712 |               const box = ancestor.getBoundingClientRect();
  713 |               bodyClip.top = Math.max(bodyClip.top, box.top + ancestor.clientTop);
  714 |               bodyClip.bottom = Math.min(bodyClip.bottom, box.top + ancestor.clientTop + ancestor.clientHeight);
  715 |             }
  716 |             return {
  717 |               dockBody: { ...rect(".workspace-dock .workspace-dock-body"), clientHeight: body.clientHeight,
  718 |                 scrollHeight: body.scrollHeight, scrollTop: body.scrollTop, clip: bodyClip,
  719 |                 usableHeight: Math.max(0, bodyClip.bottom - bodyClip.top) },
  720 |               canvas: rect(".viewport-canvas canvas"), toolbar: rect(".viewport-toolbar"),
  721 |               task: rect('[data-testid="viewport-editor-intents"]'), dock: rect(".workspace-dock"),
  722 |               surfaces: rect('[data-testid="modeling-workspace"]'),
  723 |               status: rect('[data-testid="workspace-status-bar"]'),
  724 |               bodyOverflowX: document.documentElement.scrollWidth - innerWidth,
  725 |               bodyOverflowY: document.documentElement.scrollHeight - innerHeight,
  726 |             };
  727 |           });
  728 |           evidence.push({ phase, geometry });
  729 |           await testInfo.attach(`canvas-budget-${phase.replace(/[^a-z0-9]+/gi, "-")}`, { body: JSON.stringify(geometry, null, 2), contentType: "application/json" });
  730 |           expect(geometry.canvas.width, `${phase} canvas width`).toBeGreaterThanOrEqual(200);
  731 |           expect(geometry.canvas.height, `${phase} canvas height`).toBeGreaterThanOrEqual(200);
  732 |           expect(geometry.canvas.x).toBeGreaterThanOrEqual(0);
  733 |           expect(geometry.canvas.y).toBeGreaterThanOrEqual(0);
  734 |           expect(geometry.canvas.right).toBeLessThanOrEqual(viewport.width);
  735 |           // The open page covers the stage's surfaces exactly; the canvas and the task form keep
  736 |           // their boxes inside those surfaces, under the page.
  737 |           for (const edge of ["x", "y", "width", "height"] as const) expect(geometry.dock[edge], `${phase} page ${edge}`).toBeCloseTo(geometry.surfaces[edge], 0);
> 738 |           expect(geometry.canvas.bottom).toBeLessThanOrEqual(geometry.surfaces.bottom + 1);
      |                                        ^ Error: expect(received).toBeLessThanOrEqual(expected)
  739 |           expect(geometry.task.bottom).toBeLessThanOrEqual(geometry.surfaces.bottom + 1);
  740 |           expect(geometry.dock.bottom).toBeLessThanOrEqual(geometry.status.y);
  741 |           expect(geometry.bodyOverflowX).toBeLessThanOrEqual(0);
  742 |           expect(geometry.bodyOverflowY).toBeLessThanOrEqual(0);
  743 |           if (phase.startsWith("solved")) {
  744 |             expect(geometry.dockBody.usableHeight, `${phase} usable dock body height`).toBeGreaterThan(64);
  745 |           }
  746 |           return geometry.canvas;
  747 |         };
  748 |         const beforeSelection = await measure("before solve");
  749 |         const selected = await selectTreeRow(page, "node", "node:N-110");
  750 |         await expect(selected).toHaveAttribute("aria-selected", "true");
  751 |         // Selecting in the tree closes the page onto the Model stage. Restore the same task and
  752 |         // page layout before comparing rectangles (measure reopens the page).
  753 |         if (viewport.width < 1280) await ensureRail(page, "tree", false);
  754 |         await expect(page.locator(".viewport-toolbar-selection-status")).toHaveText("Selected: node:N-110");
  755 |         expect(await measure("changed primary; task/dock restored")).toEqual(beforeSelection);
  756 |         await closePage();
  757 |         await ensureRail(page, "inspector", true);
  758 |         await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText("node: node:N-100");
  759 |         await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained inspector task");
  760 |         const footer = inspector.getByTestId("task-action-footer");
  761 |         for (const control of await footer.getByRole("button").all()) {
  762 |           await control.scrollIntoViewIfNeeded();
  763 |           await expectCenterUnobscured(control);
  764 |           if (await control.isEnabled()) {
  765 |             await control.focus();
  766 |             await expect(control).toBeFocused();
  767 |           }
  768 |         }
  769 |         if (viewport.width < 1280) {
  770 |           await ensureRail(page, "inspector", false);
  771 |         } else {
  772 |           await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "true");
  773 |           await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
  774 |         }
  775 |         expect(await measure("after footer navigation; task/dock restored")).toEqual(beforeSelection);
  776 |         await closePage();
  777 |         const toolbar = page.getByRole("group", { name: "Viewport controls", exact: true });
  778 |         for (const control of await toolbar.locator("button, select, summary").all()) {
  779 |           await control.scrollIntoViewIfNeeded();
  780 |           await expectCenterUnobscured(control);
  781 |           if (await control.isEnabled()) {
  782 |             await control.focus();
  783 |             await expect(control).toBeFocused();
  784 |           }
  785 |         }
  786 |         await page.getByTestId("cancel-pipe-draft").scrollIntoViewIfNeeded();
  787 |         await page.getByTestId("cancel-pipe-draft").focus();
  788 |         await expect(page.getByTestId("cancel-pipe-draft")).toBeFocused();
  789 |         await expectCenterUnobscured(page.getByTestId("cancel-pipe-draft"));
  790 |         await page.getByTestId("queue-explicit-pipe-intent").scrollIntoViewIfNeeded();
  791 |         await expectCenterUnobscured(page.getByTestId("queue-explicit-pipe-intent"));
  792 |         await openWorkspaceSection(page, "solve");
  793 |         await activateWithKeyboard(page, page.getByTestId("run-mechanics-preview"));
  794 |         await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  795 |         await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=830");
  796 |         await measure("solved");
  797 |         const verifySolvedDockControls = async (phase: string) => {
  798 |           await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  799 |           for (const testId of ["solver-mode-sparse", "solver-mode-dense", "run-mechanics-preview"]) {
  800 |             const control = page.getByTestId(testId);
  801 |             await expect(control).toBeEnabled();
  802 |             await control.scrollIntoViewIfNeeded();
  803 |             await control.focus();
  804 |             await expect(control).toBeFocused();
  805 |             await expectCenterUnobscured(control);
  806 |             const readControlWitness = () => control.evaluate((element) => {
  807 |               const body = element.closest(".workspace-dock-body")!;
  808 |               const box = body.getBoundingClientRect();
  809 |               const target = element.getBoundingClientRect();
  810 |               const hit = document.elementFromPoint(target.x + target.width / 2, target.y + target.height / 2);
  811 |               return { target: target.toJSON(), body: box.toJSON(), scrollTop: body.scrollTop,
  812 |                 clip: { top: box.top + body.clientTop, bottom: box.top + body.clientTop + body.clientHeight },
  813 |                 focused: document.activeElement === element,
  814 |                 hitInsideBody: hit !== null && body.contains(hit),
  815 |                 owned: hit !== null && (hit === element || element.contains(hit)) };
  816 |             });
  817 |             const initial = await readControlWitness();
  818 |             let witness = initial;
  819 |             let wheel: { deltaY: number; dispatched: boolean; beforeScrollTop: number;
  820 |               afterScrollTop: number | null } | null = null;
  821 |             try {
  822 |               if (initial.target.top < initial.clip.top && initial.owned) {
  823 |                 const center = { x: initial.target.x + initial.target.width / 2,
  824 |                   y: initial.target.y + initial.target.height / 2 };
  825 |                 await page.mouse.move(center.x, center.y);
  826 |                 const pointerOwned = await control.evaluate((element, point) => {
  827 |                   const hit = document.elementFromPoint(point.x, point.y);
  828 |                   return hit !== null && element.closest(".workspace-dock-body")!.contains(hit) &&
  829 |                     (hit === element || element.contains(hit));
  830 |                 }, center);
  831 |                 expect(pointerOwned).toBe(true);
  832 |                 const beforeWheel = await readControlWitness();
  833 |                 expect(beforeWheel.owned).toBe(true);
  834 |                 expect(beforeWheel.hitInsideBody).toBe(true);
  835 |                 expect(center.y).toBeGreaterThanOrEqual(beforeWheel.clip.top);
  836 |                 expect(center.y).toBeLessThanOrEqual(beforeWheel.clip.bottom);
  837 |                 wheel = { deltaY: -24, dispatched: false, beforeScrollTop: beforeWheel.scrollTop,
  838 |                   afterScrollTop: null };
```