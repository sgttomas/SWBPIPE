# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ui-foundation.spec.ts >> task and analysis dock preserve usable canvas dark compact 1280x800
- Location: e2e/ui-foundation.spec.ts:671:7

# Error details

```
Error: expect(received).toBeLessThanOrEqual(expected)

Expected: <= 777
Received:    1269.890625
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
      - option "System"
      - option "Light"
      - option "Dark" [selected]
      - option "Comfortable"
      - option "Compact" [selected]
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
                - searchbox "Filter model tree" [ref=e159]: node:N-100
              - generic [ref=e160]: 3 of 27 model entities visible
              - button "Clear model tree filter" [ref=e161] [cursor=pointer]:
                - img [ref=e162]
            - tree "Model" [ref=e165]:
              - generic [ref=e166]:
                - treeitem "Nodes" [expanded] [level=1] [ref=e168] [cursor=pointer]:
                  - generic [ref=e169]: ▾
                  - strong [ref=e170]: Nodes
                - treeitem "Pump nozzle node:N-100" [level=2] [selected] [ref=e172] [cursor=pointer]:
                  - img [ref=e173]
                  - generic [ref=e176]:
                    - strong [ref=e177]: Pump nozzle
                    - generic [ref=e178]: node:N-100
                - treeitem "Pipes" [expanded] [level=1] [ref=e180] [cursor=pointer]:
                  - generic [ref=e181]: ▾
                  - strong [ref=e182]: Pipes
                - treeitem "Pump discharge run pipe:P-100" [level=2] [ref=e184] [cursor=pointer]:
                  - img [ref=e185]
                  - generic [ref=e189]:
                    - strong [ref=e190]: Pump discharge run
                    - generic [ref=e191]: pipe:P-100
                - treeitem "Supports" [expanded] [level=1] [ref=e193] [cursor=pointer]:
                  - generic [ref=e194]: ▾
                  - strong [ref=e195]: Supports
                - treeitem "Anchor at pump nozzle support:S-100" [level=2] [ref=e197] [cursor=pointer]:
                  - img [ref=e198]
                  - generic [ref=e201]:
                    - strong [ref=e202]: Anchor at pump nozzle
                    - generic [ref=e203]: support:S-100
        - separator "Resize table and canvas" [ref=e204]
        - generic [ref=e206]:
          - generic [ref=e207]:
            - group "Viewport controls" [ref=e208]:
              - generic [ref=e209]: 3D Centerline
              - group "Viewport deformation overlay status" [ref=e210]:
                - generic "Deformation · unavailable" [ref=e211] [cursor=pointer]
              - group "Viewport display toggles" [ref=e212]:
                - button "Labels" [pressed] [ref=e213]
                - button "Loads" [pressed] [ref=e214]
                - button "Grid" [pressed] [ref=e215]
              - group "Viewport selection tools" [ref=e216]:
                - button "Box Select" [ref=e217]
                - generic [ref=e218]:
                  - generic [ref=e219]: Selection filter
                  - combobox "Selection filter" [ref=e220]:
                    - option "All" [selected]
                    - option "Pipes"
                    - option "Nodes"
                    - option "Supports"
                    - option "Components"
                - button "Hide" [ref=e221]
                - button "Isolate" [ref=e222]
                - button "Show All" [disabled] [ref=e223]
                - button "Fit Model" [ref=e224]
                - button "Fit Visible" [ref=e225]
                - button "Fit Selection" [ref=e226]
              - group "Viewport geometry" [ref=e227]:
                - button "Schematic" [pressed] [ref=e228]
                - button "Actual OD" [ref=e229]
                - button "Measure" [ref=e230]
            - generic "Viewport status" [ref=e231]:
              - 'generic "Selected node: node:N-100" [ref=e232]': "Selected: node:N-100"
              - status "Schematic centerline geometry" [ref=e233]
              - status "View command status" [ref=e234]: No view command dispatched.
          - generic [ref=e235]:
            - generic "Three.js pipe centerline viewport" [ref=e236]
            - generic "Viewport entity selection":
              - button "Select Pump nozzle in viewport" [pressed] [ref=e238] [cursor=pointer]:
                - img [ref=e239]
                - generic [ref=e242]: N-100
              - button "Select Vertical riser in viewport" [ref=e243] [cursor=pointer]:
                - img [ref=e244]
                - generic [ref=e248]: P-110
              - button "Select Invented branch connection marker in viewport" [ref=e249] [cursor=pointer]:
                - img [ref=e250]
                - generic [ref=e253]: C-120
              - button "Select Low point elbow in viewport" [ref=e254] [cursor=pointer]:
                - img [ref=e255]
                - generic [ref=e258]: N-110
              - button "Select Riser elbow in viewport" [ref=e259] [cursor=pointer]:
                - img [ref=e260]
                - generic [ref=e263]: N-120
              - button "Select Rack span in viewport" [ref=e264] [cursor=pointer]:
                - img [ref=e265]
                - generic [ref=e269]: P-120
              - button "Select Tie-in rise in viewport" [ref=e270] [cursor=pointer]:
                - img [ref=e271]
                - generic [ref=e275]: P-130
              - button "Select Invented semi-rigid valve marker in viewport" [ref=e276] [cursor=pointer]:
                - img [ref=e277]
                - generic [ref=e280]: C-130
              - button "Select Rack turn in viewport" [ref=e281] [cursor=pointer]:
                - img [ref=e282]
                - generic [ref=e285]: N-130
              - button "Select Terminal tie-in in viewport" [ref=e286] [cursor=pointer]:
                - img [ref=e287]
                - generic [ref=e290]: N-140
              - button "Select Preview one-way terminal stop in viewport" [ref=e291] [cursor=pointer]:
                - img [ref=e292]
                - generic [ref=e295]: NL-140
              - button "Select Anchor at pump nozzle in viewport" [ref=e296] [cursor=pointer]:
                - img [ref=e297]
                - generic [ref=e300]: S-100
            - img "Orientation gizmo showing X, Y, Z axes"
            - generic "View controls" [ref=e301]:
              - button "Front" [ref=e302] [cursor=pointer]
              - button "Top" [ref=e303] [cursor=pointer]
              - button "Isometric" [pressed] [ref=e304] [cursor=pointer]
            - generic:
              - generic: 1 m
          - region "Command and selection bar" [ref=e305]:
            - generic "Object creation tools" [ref=e306]:
              - button "Node" [ref=e307] [cursor=pointer]:
                - img [ref=e308]
                - text: Node
              - button "Pipe" [pressed] [ref=e310] [cursor=pointer]:
                - img [ref=e311]
                - text: Pipe
              - button "Support" [ref=e315] [cursor=pointer]:
                - img [ref=e316]
                - text: Support
              - button "Component" [ref=e319] [cursor=pointer]:
                - img [ref=e320]
                - text: Component
              - button "Load" [ref=e323] [cursor=pointer]:
                - img [ref=e324]
                - text: Load
            - 'generic "Pipe tool armed: pick from/to nodes or complete the pipe form." [ref=e326]': Pipe tool armed
            - group [ref=e327]:
              - generic "Selection & navigation" [ref=e328] [cursor=pointer]
        - generic [ref=e329]:
          - button "Close inspector" [ref=e330]:
            - img [ref=e331]
          - region "Viewport editor intents" [ref=e335]:
            - heading "Create pipe" [level=3] [ref=e336]
            - group [ref=e337]:
              - generic "Explicit straight pipe connectivity" [ref=e338]:
                - generic [ref=e339]:
                  - generic [ref=e340]: Pipe ID
                  - textbox "New pipe ID" [ref=e341]:
                    - /placeholder: pipe:P-2
                - generic [ref=e342]:
                  - generic [ref=e343]: Label
                  - textbox "New pipe label" [ref=e344]:
                    - /placeholder: Pipe label
                    - text: Retained pipe draft
                - generic [ref=e345]:
                  - generic [ref=e346]:
                    - generic [ref=e347]: From
                    - button "Pick" [pressed] [ref=e348] [cursor=pointer]:
                      - img [ref=e349]
                      - text: Pick
                  - generic [ref=e352]:
                    - generic [ref=e353]:
                      - generic [ref=e354]: New pipe from node
                      - combobox "New pipe from node" [expanded] [ref=e355]
                    - generic "New pipe from node current value" [ref=e356]: No target selected
                    - generic [ref=e357]: 5 of 5 targets
                    - listbox "New pipe from node options" [ref=e358]:
                      - generic [ref=e359]:
                        - option "Pump nozzle node:N-100" [ref=e361] [cursor=pointer]:
                          - generic [ref=e362]: Pump nozzle
                          - generic [ref=e363]: node:N-100
                        - option "Low point elbow node:N-110" [ref=e365] [cursor=pointer]:
                          - generic [ref=e366]: Low point elbow
                          - generic [ref=e367]: node:N-110
                        - option "Riser elbow node:N-120" [ref=e369] [cursor=pointer]:
                          - generic [ref=e370]: Riser elbow
                          - generic [ref=e371]: node:N-120
                        - option "Rack turn node:N-130" [ref=e373] [cursor=pointer]:
                          - generic [ref=e374]: Rack turn
                          - generic [ref=e375]: node:N-130
                        - option "Terminal tie-in node:N-140" [ref=e377] [cursor=pointer]:
                          - generic [ref=e378]: Terminal tie-in
                          - generic [ref=e379]: node:N-140
                - group "End mode" [ref=e380]:
                  - generic [ref=e381]: End mode
                  - generic [ref=e382]:
                    - radio "Existing node" [checked] [ref=e383]
                    - text: Existing node
                  - generic [ref=e384]:
                    - radio "New node" [ref=e385]
                    - text: New node
                - generic [ref=e386]:
                  - generic [ref=e387]:
                    - generic [ref=e388]: To
                    - button "Pick" [ref=e389] [cursor=pointer]:
                      - img [ref=e390]
                      - text: Pick
                  - generic [ref=e393]:
                    - generic [ref=e394]:
                      - generic [ref=e395]: New pipe to node
                      - combobox "New pipe to node" [expanded] [ref=e396]
                    - generic "New pipe to node current value" [ref=e397]: No target selected
                    - generic [ref=e398]: 5 of 5 targets
                    - listbox "New pipe to node options" [ref=e399]:
                      - generic [ref=e400]:
                        - option "Pump nozzle node:N-100" [ref=e402] [cursor=pointer]:
                          - generic [ref=e403]: Pump nozzle
                          - generic [ref=e404]: node:N-100
                        - option "Low point elbow node:N-110" [ref=e406] [cursor=pointer]:
                          - generic [ref=e407]: Low point elbow
                          - generic [ref=e408]: node:N-110
                        - option "Riser elbow node:N-120" [ref=e410] [cursor=pointer]:
                          - generic [ref=e411]: Riser elbow
                          - generic [ref=e412]: node:N-120
                        - option "Rack turn node:N-130" [ref=e414] [cursor=pointer]:
                          - generic [ref=e415]: Rack turn
                          - generic [ref=e416]: node:N-130
                        - option "Terminal tie-in node:N-140" [ref=e418] [cursor=pointer]:
                          - generic [ref=e419]: Terminal tie-in
                          - generic [ref=e420]: node:N-140
                - generic [ref=e421]:
                  - generic [ref=e422]:
                    - generic [ref=e423]: New pipe material
                    - combobox "New pipe material" [expanded] [ref=e424]
                  - generic "New pipe material current value" [ref=e425]: No target selected
                  - generic [ref=e426]: 1 of 1 targets
                  - listbox "New pipe material options" [ref=e427]:
                    - option "Invented carbon-steel-like material material:invented-carbon-steel" [ref=e430] [cursor=pointer]:
                      - generic [ref=e431]: Invented carbon-steel-like material
                      - generic [ref=e432]: material:invented-carbon-steel
                - generic [ref=e433]:
                  - generic [ref=e434]: OD
                  - textbox "New pipe outside diameter" [ref=e435]:
                    - /placeholder: "0.114"
                - generic [ref=e436]:
                  - generic [ref=e437]: Wall
                  - textbox "New pipe wall thickness" [ref=e438]:
                    - /placeholder: "0.006"
                - generic [ref=e439]:
                  - generic [ref=e440]: Length unit
                  - combobox "New pipe length unit" [ref=e441]:
                    - option "m" [selected]
                - generic [ref=e442]: "Pipe geometry: m, model metadata"
                - generic [ref=e443]: "Construction plane inactive: existing endpoint uses exact node IDs."
                - generic [ref=e444]: No route ghost is visible.
                - generic [ref=e445]:
                  - generic [ref=e446]: Yref X
                  - textbox "New pipe y-reference X" [ref=e447]:
                    - /placeholder: "0"
                - generic [ref=e448]:
                  - generic [ref=e449]: Yref Y
                  - textbox "New pipe y-reference Y" [ref=e450]:
                    - /placeholder: "0"
                - generic [ref=e451]:
                  - generic [ref=e452]: Yref Z
                  - textbox "New pipe y-reference Z" [ref=e453]:
                    - /placeholder: "1"
                - generic [ref=e454]:
                  - generic [ref=e455]: Provenance
                  - textbox "New pipe provenance" [ref=e456]: layout regression draft
                - generic [ref=e457]:
                  - checkbox "Continue from end after Apply; keep the entered material, dimensions, orientation and provenance" [ref=e458]
                  - text: Continue from end after Apply; keep the entered material, dimensions, orientation and provenance
                - button "Cancel pipe draft" [ref=e459] [cursor=pointer]
                - button "Add route" [disabled] [ref=e460]:
                  - img [ref=e461]
                  - text: Add route
                - generic [ref=e465]: Choose an existing start node.; Choose an existing end node.; Enter a pipe ID.; Choose an existing material ID.; Enter positive outside diameter and wall thickness values.; Enter a finite, nonzero y-reference vector.
              - region "Route review" [ref=e466]:
                - heading "Review and Apply" [level=4] [ref=e467]
                - status [ref=e468]: The affected selection changed. Add again to review the current draft.
                - paragraph [ref=e469]: Add a complete node or route to generate the service validation and exact diff.
                - button "Apply" [disabled] [ref=e470]
              - group [ref=e471]:
                - generic "Unit source" [ref=e472] [cursor=pointer]
            - group [ref=e473]:
              - generic "Pending changes (0)" [ref=e474] [cursor=pointer]
          - region "Property inspector" [ref=e475]:
            - 'heading "Pump nozzle — node: node:N-100" [level=2] [ref=e476]':
              - text: Pump nozzle
              - generic [ref=e477]: "— node: node:N-100"
            - tablist "Inspector views" [ref=e478]:
              - tab "Properties" [ref=e479]
              - tab "Task" [selected] [ref=e480]
            - tabpanel "Editor operation intent" [ref=e481]:
              - generic [ref=e483]: "Draft target: node: node:N-100"
              - heading "Edit name" [level=3] [ref=e484]:
                - img [ref=e485]
                - text: Edit name
              - generic [ref=e488]:
                - generic [ref=e489]:
                  - generic [ref=e490]: Property
                  - combobox "Property to edit" [ref=e491]:
                    - option "Name" [selected]
                    - option "X position"
                    - option "Y position"
                    - option "Z position"
                    - option "Provenance"
                - region "Current value" [ref=e492]:
                  - generic [ref=e493]: Current name
                  - strong [ref=e494]: Pump nozzle
                - generic [ref=e495]:
                  - generic [ref=e496]: New name
                  - textbox "New name" [ref=e497]: Retained inspector task
                - generic "Task actions" [ref=e498]:
                  - button "Add" [ref=e499] [cursor=pointer]:
                    - img [ref=e500]
                    - text: Add
                  - button "Cancel" [ref=e501] [cursor=pointer]
                  - button "Review" [ref=e502] [cursor=pointer]: Review
                  - button "Apply" [ref=e506] [cursor=pointer]:
                    - img [ref=e507]
                    - text: Apply
              - group [ref=e510]:
                - generic "Operation details" [ref=e511] [cursor=pointer]
              - paragraph [ref=e512]: Validate to check this change before applying it.
      - region "Workspace sections" [ref=e513]:
        - generic [ref=e514]:
          - heading "Analyze" [level=2] [ref=e515]
          - button "Close" [ref=e516] [cursor=pointer]:
            - img [ref=e517]
            - text: Close
        - region "Solve section" [ref=e521]:
          - region "Solve execution" [ref=e522]:
            - generic [ref=e523]: Execution
            - region "Solve readiness summary" [ref=e524]:
              - generic [ref=e525]:
                - img [ref=e526]
                - generic [ref=e528]: Mechanics readiness
                - strong [ref=e529]: preview run not started; ready for preview diagnostics
              - generic [ref=e530]:
                - img [ref=e531]
                - generic [ref=e533]: Rule-check readiness
                - strong [ref=e534]: Rule pack · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE); mechanics results remain reviewable only
              - generic [ref=e535]:
                - img [ref=e536]
                - generic [ref=e538]: Diagnostics
                - strong [ref=e539]: 1 diagnostic; 1 warning; 0 blocking/error
              - generic [ref=e540]:
                - img [ref=e541]
                - generic [ref=e543]: Professional boundary
                - strong [ref=e544]: Human · Human review required (HUMAN_REVIEW_REQUIRED)
            - generic [ref=e545]:
              - generic [ref=e546]:
                - generic [ref=e547]:
                  - checkbox "Include known private values in this local export" [ref=e548]
                  - text: Include known private values in this local export
                - generic [ref=e549]: decisions=93; findings=93; blocked=true
                - group [ref=e550]:
                  - generic "Details" [ref=e551] [cursor=pointer]
                - generic [ref=e552]:
                  - img [ref=e553]
                  - text: Solve job JSON
              - generic [ref=e556]: state=not_started; events=1; result_rows=0; cancellation_requested=false
            - generic [ref=e557]:
              - generic [ref=e558]:
                - generic [ref=e559]: Solver mode
                - strong [ref=e560]: selected=sparse_interactive; sparse_default=true; dense_scrutiny=false
              - generic [ref=e561]:
                - generic [ref=e562]: Progress
                - strong [ref=e563]: not_started; preview_service_event_state_only_no_percent_stream; percentages_synthesized=false
              - generic [ref=e564]:
                - generic [ref=e565]: Cancellation
                - strong [ref=e566]: control_visible=true; enabled=false; requested=false; seam=no_job_started; token=none_no_active_backend_job; success_claimed=false
              - generic [ref=e567]:
                - generic [ref=e568]: Result binding
                - strong [ref=e569]: not generated; result rows=0; hashes=0
              - generic [ref=e570]:
                - generic [ref=e571]: Unit policy
                - strong [ref=e572]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; results=none; rows=0; conversion=false
              - generic [ref=e573]:
                - generic [ref=e574]: Boundary
                - strong [ref=e575]: private payload=false; protected content=false; release/professional claim=false; human review required
            - group "Solver mode" [ref=e576]:
              - button "Sparse interactive" [pressed] [ref=e577] [cursor=pointer]:
                - img [ref=e578]
                - text: Sparse interactive
              - button "Dense scrutiny" [ref=e580] [cursor=pointer]:
                - img [ref=e581]
                - text: Dense scrutiny
            - button "Run mechanics preview" [ref=e584] [cursor=pointer]:
              - img [ref=e585]
              - text: Run mechanics preview
            - button "Cancel preview" [disabled] [ref=e587] [cursor=pointer]:
              - img [ref=e588]
              - text: Cancel preview
          - region "Rule-check completeness" [ref=e590]:
            - generic [ref=e591]:
              - img [ref=e592]
              - text: Rule-Check Completeness
            - generic [ref=e594]:
              - generic [ref=e595]:
                - generic [ref=e596]:
                  - checkbox "Include known private values in this local export" [ref=e597]
                  - text: Include known private values in this local export
                - generic [ref=e598]: decisions=122; findings=122; blocked=true
                - group [ref=e599]:
                  - generic "Details" [ref=e600] [cursor=pointer]
                - generic [ref=e601]:
                  - img [ref=e602]
                  - text: Local completeness JSON
              - generic [ref=e605]: 3 review findings; rule_check_blocked=true; mechanics_reviewable=false
            - generic [ref=e606]:
              - generic [ref=e607]:
                - generic [ref=e608]: Rule status
                - strong [ref=e609]: not_performed_user_rule_inputs_missing
              - generic [ref=e610]:
                - generic [ref=e611]: Mechanics status
                - strong [ref=e612]: ready_for_preview_diagnostics
              - generic [ref=e613]:
                - generic [ref=e614]: Unit policy
                - strong [ref=e615]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; records=38; rule_input_units=explicit_or_blocking; diagnostic=RULE_UNIT_MISMATCH; conversion=false
              - generic [ref=e616]:
                - generic [ref=e617]: Boundary
                - strong [ref=e618]: private rule data absent; silent defaults used=false; bundled code values=false; no compliance claim
              - generic [ref=e619]:
                - article [ref=e620]:
                  - strong [ref=e621]: RULE_CHECK_BLOCKING
                  - generic [ref=e622]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e623]: User-supplied rule-pack inputs are absent, so rule-check status stays incomplete while mechanics output remains separate. Provide a private rule pack and project-specific criteria with provenance; the public preview does not bundle code defaults.
                - article [ref=e624]:
                  - strong [ref=e625]: RULE_CHECK_BLOCKING
                  - generic [ref=e626]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e627]: Load cases and combinations are mechanics-basis preview inputs, not code-specific rule combinations. Supply private owner/code combination criteria before reporting user-rule checked status.
                - article [ref=e628]:
                  - strong [ref=e629]: PROVENANCE_WARNING
                  - generic [ref=e630]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e631]: Material and pipe-section values are invented public fixture values, not verified project or catalog data. Replace them with user-controlled data and provenance before engineering reliance.
            - generic [ref=e632]: Completeness findings explain missing user/private rule-check data.
          - region "Run rule checks" [ref=e633]:
            - generic [ref=e634]:
              - img [ref=e635]
              - text: Run Rule Checks (user-defined, private)
            - generic [ref=e638]:
              - generic [ref=e639]: "Project scope: project:invented-loop-01."
              - generic [ref=e640]: No solved mechanics result in this session; run a solve first so solver_result inputs can bind to result rows.
            - generic [ref=e641]:
              - button "Load demo rule pack" [ref=e642]
              - button "Refresh saved packs" [ref=e643]
            - generic [ref=e645]: Saved rule-pack list not refreshed yet.
            - generic [ref=e646]:
              - generic [ref=e647]: Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)
              - textbox "Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)" [ref=e648]:
                - /placeholder: No rule-pack document. Load the demo, open a saved pack, or paste one.
            - generic [ref=e649]:
              - button "Run checks" [disabled] [ref=e650]:
                - img [ref=e651]
                - text: Run checks
              - generic [ref=e653]: No rule-check run in this session.
            - generic [ref=e654]:
              - img [ref=e655]
              - text: Rule checks run locally over user-supplied data only. The software emits user-defined check statuses (USER_RULE_CHECKED / USER_RULE_FAILED / RULE_INPUTS_INCOMPLETE).
          - region "Design knowledge" [ref=e657]:
            - generic [ref=e658]:
              - img [ref=e659]
              - text: Design Knowledge
            - generic [ref=e661]:
              - article [ref=e662]:
                - generic [ref=e663]:
                  - strong [ref=e664]: Pump nozzle endpoint
                  - generic [ref=e665]: endpoint · review_required
                - paragraph [ref=e666]: Invented endpoint record used to show how user-supplied design context appears in the preview.
                - generic [ref=e667]: invented_example
              - article [ref=e668]:
                - generic [ref=e669]:
                  - strong [ref=e670]: Rack corridor placeholder
                  - generic [ref=e671]: routing_corridor · review_required
                - paragraph [ref=e672]: Invented corridor note; no owner standard or protected spacing criterion is bundled.
                - generic [ref=e673]: invented_example
              - article [ref=e674]:
                - generic [ref=e675]:
                  - strong [ref=e676]: Support stiffness dimensions unresolved
                  - generic [ref=e677]: assumption · unresolved
                - paragraph [ref=e678]: Linear and rotational support behavior is represented as a preview diagnostic until user-supplied stiffness values and canonical dimensions are provided.
                - generic [ref=e679]: invented_example
    - complementary "Agent" [ref=e680]:
      - button "Agent" [disabled] [ref=e682]:
        - img [ref=e683]
        - generic [ref=e686]: Agent
  - generic "Workspace status" [ref=e687]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e688] [cursor=pointer]:
      - img [ref=e689]
      - text: 5 Issues
    - generic "Selection" [ref=e691]: "node: node:N-100"
    - generic "Display units" [ref=e692]: Entered
    - button "About SWBPIPE…" [ref=e693] [cursor=pointer]:
      - img [ref=e694]
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
  729 |           expect(geometry.canvas.width, `${phase} canvas width`).toBeGreaterThanOrEqual(200);
  730 |           expect(geometry.canvas.height, `${phase} canvas height`).toBeGreaterThanOrEqual(200);
  731 |           expect(geometry.canvas.x).toBeGreaterThanOrEqual(0);
  732 |           expect(geometry.canvas.y).toBeGreaterThanOrEqual(0);
  733 |           expect(geometry.canvas.right).toBeLessThanOrEqual(viewport.width);
  734 |           // The open page covers the stage's surfaces exactly; the canvas and the task form keep
  735 |           // their boxes inside those surfaces, under the page.
  736 |           for (const edge of ["x", "y", "width", "height"] as const) expect(geometry.dock[edge], `${phase} page ${edge}`).toBeCloseTo(geometry.surfaces[edge], 0);
  737 |           expect(geometry.canvas.bottom).toBeLessThanOrEqual(geometry.surfaces.bottom + 1);
> 738 |           expect(geometry.task.bottom).toBeLessThanOrEqual(geometry.surfaces.bottom + 1);
      |                                        ^ Error: expect(received).toBeLessThanOrEqual(expected)
  739 |           expect(geometry.dock.bottom).toBeLessThanOrEqual(geometry.status.y);
  740 |           expect(geometry.bodyOverflowX).toBeLessThanOrEqual(0);
  741 |           expect(geometry.bodyOverflowY).toBeLessThanOrEqual(0);
  742 |           if (phase.startsWith("solved")) {
  743 |             expect(geometry.dockBody.usableHeight, `${phase} usable dock body height`).toBeGreaterThan(64);
  744 |           }
  745 |           return geometry.canvas;
  746 |         };
  747 |         const beforeSelection = await measure("before solve");
  748 |         const selected = await selectTreeRow(page, "node", "node:N-110");
  749 |         await expect(selected).toHaveAttribute("aria-selected", "true");
  750 |         // Selecting in the tree closes the page onto the Model stage. Restore the same task and
  751 |         // page layout before comparing rectangles (measure reopens the page).
  752 |         if (viewport.width < 1280) await ensureRail(page, "tree", false);
  753 |         await expect(page.locator(".viewport-toolbar-selection-status")).toHaveText("Selected: node:N-110");
  754 |         expect(await measure("changed primary; task/dock restored")).toEqual(beforeSelection);
  755 |         await closePage();
  756 |         await ensureRail(page, "inspector", true);
  757 |         await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText("node: node:N-100");
  758 |         await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained inspector task");
  759 |         const footer = inspector.getByTestId("task-action-footer");
  760 |         for (const control of await footer.getByRole("button").all()) {
  761 |           await control.scrollIntoViewIfNeeded();
  762 |           await expectCenterUnobscured(control);
  763 |           if (await control.isEnabled()) {
  764 |             await control.focus();
  765 |             await expect(control).toBeFocused();
  766 |           }
  767 |         }
  768 |         if (viewport.width < 1280) {
  769 |           await ensureRail(page, "inspector", false);
  770 |         } else {
  771 |           await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "true");
  772 |           await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
  773 |         }
  774 |         expect(await measure("after footer navigation; task/dock restored")).toEqual(beforeSelection);
  775 |         await closePage();
  776 |         const toolbar = page.getByRole("group", { name: "Viewport controls", exact: true });
  777 |         for (const control of await toolbar.locator("button, select, summary").all()) {
  778 |           await control.scrollIntoViewIfNeeded();
  779 |           await expectCenterUnobscured(control);
  780 |           if (await control.isEnabled()) {
  781 |             await control.focus();
  782 |             await expect(control).toBeFocused();
  783 |           }
  784 |         }
  785 |         await page.getByTestId("cancel-pipe-draft").scrollIntoViewIfNeeded();
  786 |         await page.getByTestId("cancel-pipe-draft").focus();
  787 |         await expect(page.getByTestId("cancel-pipe-draft")).toBeFocused();
  788 |         await expectCenterUnobscured(page.getByTestId("cancel-pipe-draft"));
  789 |         await page.getByTestId("queue-explicit-pipe-intent").scrollIntoViewIfNeeded();
  790 |         await expectCenterUnobscured(page.getByTestId("queue-explicit-pipe-intent"));
  791 |         await openWorkspaceSection(page, "solve");
  792 |         await activateWithKeyboard(page, page.getByTestId("run-mechanics-preview"));
  793 |         await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  794 |         await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=830");
  795 |         await measure("solved");
  796 |         const verifySolvedDockControls = async (phase: string) => {
  797 |           await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  798 |           for (const testId of ["solver-mode-sparse", "solver-mode-dense", "run-mechanics-preview"]) {
  799 |             const control = page.getByTestId(testId);
  800 |             await expect(control).toBeEnabled();
  801 |             await control.scrollIntoViewIfNeeded();
  802 |             await control.focus();
  803 |             await expect(control).toBeFocused();
  804 |             await expectCenterUnobscured(control);
  805 |             const readControlWitness = () => control.evaluate((element) => {
  806 |               const body = element.closest(".workspace-dock-body")!;
  807 |               const box = body.getBoundingClientRect();
  808 |               const target = element.getBoundingClientRect();
  809 |               const hit = document.elementFromPoint(target.x + target.width / 2, target.y + target.height / 2);
  810 |               return { target: target.toJSON(), body: box.toJSON(), scrollTop: body.scrollTop,
  811 |                 clip: { top: box.top + body.clientTop, bottom: box.top + body.clientTop + body.clientHeight },
  812 |                 focused: document.activeElement === element,
  813 |                 hitInsideBody: hit !== null && body.contains(hit),
  814 |                 owned: hit !== null && (hit === element || element.contains(hit)) };
  815 |             });
  816 |             const initial = await readControlWitness();
  817 |             let witness = initial;
  818 |             let wheel: { deltaY: number; dispatched: boolean; beforeScrollTop: number;
  819 |               afterScrollTop: number | null } | null = null;
  820 |             try {
  821 |               if (initial.target.top < initial.clip.top && initial.owned) {
  822 |                 const center = { x: initial.target.x + initial.target.width / 2,
  823 |                   y: initial.target.y + initial.target.height / 2 };
  824 |                 await page.mouse.move(center.x, center.y);
  825 |                 const pointerOwned = await control.evaluate((element, point) => {
  826 |                   const hit = document.elementFromPoint(point.x, point.y);
  827 |                   return hit !== null && element.closest(".workspace-dock-body")!.contains(hit) &&
  828 |                     (hit === element || element.contains(hit));
  829 |                 }, center);
  830 |                 expect(pointerOwned).toBe(true);
  831 |                 const beforeWheel = await readControlWitness();
  832 |                 expect(beforeWheel.owned).toBe(true);
  833 |                 expect(beforeWheel.hitInsideBody).toBe(true);
  834 |                 expect(center.y).toBeGreaterThanOrEqual(beforeWheel.clip.top);
  835 |                 expect(center.y).toBeLessThanOrEqual(beforeWheel.clip.bottom);
  836 |                 wheel = { deltaY: -24, dispatched: false, beforeScrollTop: beforeWheel.scrollTop,
  837 |                   afterScrollTop: null };
  838 |                 await page.mouse.wheel(0, wheel.deltaY);
```