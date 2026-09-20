# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ui-foundation.spec.ts >> task and analysis dock preserve usable canvas dark compact 1024x768
- Location: e2e/ui-foundation.spec.ts:671:7

# Error details

```
Error: expect(received).toBeLessThanOrEqual(expected)

Expected: <= 745
Received:    1237.890625
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
    - heading "SWBPIPE" [level=1] [ref=e17]
    - group "Editing tools" [ref=e18]:
      - button "Undo model edit" [disabled] [ref=e19]:
        - img [ref=e20]
      - button "Redo model edit" [disabled] [ref=e23]:
        - img [ref=e24]
      - button "Select (⎋)" [ref=e27] [cursor=pointer]:
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
      - option "System"
      - option "Light"
      - option "Dark" [selected]
      - option "Comfortable"
      - option "Compact" [selected]
    - region "Human toolkit" [ref=e70]:
      - button "Find modeling commands" [ref=e71] [cursor=pointer]:
        - img [ref=e72]
        - generic [ref=e75]: Search or command…
        - generic "Command K" [ref=e76]: ⌘K
  - generic [ref=e77]:
    - navigation "Stages" [ref=e78]:
      - list [ref=e79]:
        - listitem [ref=e80]:
          - button "Model" [ref=e81] [cursor=pointer]:
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
    - generic [ref=e126]:
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
              - 'generic "Selected node: node:N-100" [ref=e164]': "Selected: node:N-100"
              - status "Schematic centerline geometry" [ref=e165]
              - status "View command status" [ref=e166]: No view command dispatched.
          - generic [ref=e167]:
            - generic "Three.js pipe centerline viewport" [ref=e168]
            - generic "Viewport entity selection":
              - button "Select Pump nozzle in viewport" [pressed] [ref=e170] [cursor=pointer]:
                - img [ref=e171]
                - generic [ref=e174]: N-100
              - button "Select Vertical riser in viewport" [ref=e175] [cursor=pointer]:
                - img [ref=e176]
                - generic [ref=e180]: P-110
              - button "Select Invented branch connection marker in viewport" [ref=e181] [cursor=pointer]:
                - img [ref=e182]
                - generic [ref=e185]: C-120
              - button "Select Low point elbow in viewport" [ref=e186] [cursor=pointer]:
                - img [ref=e187]
                - generic [ref=e190]: N-110
              - button "Select Riser elbow in viewport" [ref=e191] [cursor=pointer]:
                - img [ref=e192]
                - generic [ref=e195]: N-120
              - button "Select Rack span in viewport" [ref=e196] [cursor=pointer]:
                - img [ref=e197]
                - generic [ref=e201]: P-120
              - button "Select Tie-in rise in viewport" [ref=e202] [cursor=pointer]:
                - img [ref=e203]
                - generic [ref=e207]: P-130
              - button "Select Invented semi-rigid valve marker in viewport" [ref=e208] [cursor=pointer]:
                - img [ref=e209]
                - generic [ref=e212]: C-130
              - button "Select Rack turn in viewport" [ref=e213] [cursor=pointer]:
                - img [ref=e214]
                - generic [ref=e217]: N-130
              - button "Select Terminal tie-in in viewport" [ref=e218] [cursor=pointer]:
                - img [ref=e219]
                - generic [ref=e222]: N-140
              - button "Select Preview one-way terminal stop in viewport" [ref=e223] [cursor=pointer]:
                - img [ref=e224]
                - generic [ref=e227]: NL-140
              - button "Select Anchor at pump nozzle in viewport" [ref=e228] [cursor=pointer]:
                - img [ref=e229]
                - generic [ref=e232]: S-100
            - img "Orientation gizmo showing X, Y, Z axes"
            - generic "View controls" [ref=e233]:
              - button "Front" [ref=e234] [cursor=pointer]
              - button "Top" [ref=e235] [cursor=pointer]
              - button "Isometric" [pressed] [ref=e236] [cursor=pointer]
            - generic:
              - generic: 1 m
          - region "Command and selection bar" [ref=e237]:
            - generic "Object creation tools" [ref=e238]:
              - button "Node" [ref=e239] [cursor=pointer]:
                - img [ref=e240]
                - text: Node
              - button "Pipe" [pressed] [ref=e242] [cursor=pointer]:
                - img [ref=e243]
                - text: Pipe
              - button "Support" [ref=e247] [cursor=pointer]:
                - img [ref=e248]
                - text: Support
              - button "Component" [ref=e251] [cursor=pointer]:
                - img [ref=e252]
                - text: Component
              - button "Load" [ref=e255] [cursor=pointer]:
                - img [ref=e256]
                - text: Load
            - 'generic "Pipe tool armed: pick from/to nodes or complete the pipe form." [ref=e258]': Pipe tool armed
            - group [ref=e259]:
              - generic "Selection & navigation" [ref=e260] [cursor=pointer]
        - generic [ref=e261]:
          - button "Close inspector" [ref=e262]:
            - img [ref=e263]
          - region "Viewport editor intents" [ref=e267]:
            - heading "Create pipe" [level=3] [ref=e268]
            - group [ref=e269]:
              - generic "Explicit straight pipe connectivity" [ref=e270]:
                - generic [ref=e271]:
                  - generic [ref=e272]: Pipe ID
                  - textbox "New pipe ID" [ref=e273]:
                    - /placeholder: pipe:P-2
                - generic [ref=e274]:
                  - generic [ref=e275]: Label
                  - textbox "New pipe label" [ref=e276]:
                    - /placeholder: Pipe label
                    - text: Retained pipe draft
                - generic [ref=e277]:
                  - generic [ref=e278]:
                    - generic [ref=e279]: From
                    - button "Pick" [pressed] [ref=e280] [cursor=pointer]:
                      - img [ref=e281]
                      - text: Pick
                  - generic [ref=e284]:
                    - generic [ref=e285]:
                      - generic [ref=e286]: New pipe from node
                      - combobox "New pipe from node" [expanded] [ref=e287]
                    - generic "New pipe from node current value" [ref=e288]: No target selected
                    - generic [ref=e289]: 5 of 5 targets
                    - listbox "New pipe from node options" [ref=e290]:
                      - generic [ref=e291]:
                        - option "Pump nozzle node:N-100" [ref=e293] [cursor=pointer]:
                          - generic [ref=e294]: Pump nozzle
                          - generic [ref=e295]: node:N-100
                        - option "Low point elbow node:N-110" [ref=e297] [cursor=pointer]:
                          - generic [ref=e298]: Low point elbow
                          - generic [ref=e299]: node:N-110
                        - option "Riser elbow node:N-120" [ref=e301] [cursor=pointer]:
                          - generic [ref=e302]: Riser elbow
                          - generic [ref=e303]: node:N-120
                        - option "Rack turn node:N-130" [ref=e305] [cursor=pointer]:
                          - generic [ref=e306]: Rack turn
                          - generic [ref=e307]: node:N-130
                        - option "Terminal tie-in node:N-140" [ref=e309] [cursor=pointer]:
                          - generic [ref=e310]: Terminal tie-in
                          - generic [ref=e311]: node:N-140
                - group "End mode" [ref=e312]:
                  - generic [ref=e313]: End mode
                  - generic [ref=e314]:
                    - radio "Existing node" [checked] [ref=e315]
                    - text: Existing node
                  - generic [ref=e316]:
                    - radio "New node" [ref=e317]
                    - text: New node
                - generic [ref=e318]:
                  - generic [ref=e319]:
                    - generic [ref=e320]: To
                    - button "Pick" [ref=e321] [cursor=pointer]:
                      - img [ref=e322]
                      - text: Pick
                  - generic [ref=e325]:
                    - generic [ref=e326]:
                      - generic [ref=e327]: New pipe to node
                      - combobox "New pipe to node" [expanded] [ref=e328]
                    - generic "New pipe to node current value" [ref=e329]: No target selected
                    - generic [ref=e330]: 5 of 5 targets
                    - listbox "New pipe to node options" [ref=e331]:
                      - generic [ref=e332]:
                        - option "Pump nozzle node:N-100" [ref=e334] [cursor=pointer]:
                          - generic [ref=e335]: Pump nozzle
                          - generic [ref=e336]: node:N-100
                        - option "Low point elbow node:N-110" [ref=e338] [cursor=pointer]:
                          - generic [ref=e339]: Low point elbow
                          - generic [ref=e340]: node:N-110
                        - option "Riser elbow node:N-120" [ref=e342] [cursor=pointer]:
                          - generic [ref=e343]: Riser elbow
                          - generic [ref=e344]: node:N-120
                        - option "Rack turn node:N-130" [ref=e346] [cursor=pointer]:
                          - generic [ref=e347]: Rack turn
                          - generic [ref=e348]: node:N-130
                        - option "Terminal tie-in node:N-140" [ref=e350] [cursor=pointer]:
                          - generic [ref=e351]: Terminal tie-in
                          - generic [ref=e352]: node:N-140
                - generic [ref=e353]:
                  - generic [ref=e354]:
                    - generic [ref=e355]: New pipe material
                    - combobox "New pipe material" [expanded] [ref=e356]
                  - generic "New pipe material current value" [ref=e357]: No target selected
                  - generic [ref=e358]: 1 of 1 targets
                  - listbox "New pipe material options" [ref=e359]:
                    - option "Invented carbon-steel-like material material:invented-carbon-steel" [ref=e362] [cursor=pointer]:
                      - generic [ref=e363]: Invented carbon-steel-like material
                      - generic [ref=e364]: material:invented-carbon-steel
                - generic [ref=e365]:
                  - generic [ref=e366]: OD
                  - textbox "New pipe outside diameter" [ref=e367]:
                    - /placeholder: "0.114"
                - generic [ref=e368]:
                  - generic [ref=e369]: Wall
                  - textbox "New pipe wall thickness" [ref=e370]:
                    - /placeholder: "0.006"
                - generic [ref=e371]:
                  - generic [ref=e372]: Length unit
                  - combobox "New pipe length unit" [ref=e373]:
                    - option "m" [selected]
                - generic [ref=e374]: "Pipe geometry: m, model metadata"
                - generic [ref=e375]: "Construction plane inactive: existing endpoint uses exact node IDs."
                - generic [ref=e376]: No route ghost is visible.
                - generic [ref=e377]:
                  - generic [ref=e378]: Yref X
                  - textbox "New pipe y-reference X" [ref=e379]:
                    - /placeholder: "0"
                - generic [ref=e380]:
                  - generic [ref=e381]: Yref Y
                  - textbox "New pipe y-reference Y" [ref=e382]:
                    - /placeholder: "0"
                - generic [ref=e383]:
                  - generic [ref=e384]: Yref Z
                  - textbox "New pipe y-reference Z" [ref=e385]:
                    - /placeholder: "1"
                - generic [ref=e386]:
                  - generic [ref=e387]: Provenance
                  - textbox "New pipe provenance" [ref=e388]: layout regression draft
                - generic [ref=e389]:
                  - checkbox "Continue from end after Apply; keep the entered material, dimensions, orientation and provenance" [ref=e390]
                  - text: Continue from end after Apply; keep the entered material, dimensions, orientation and provenance
                - button "Cancel pipe draft" [ref=e391] [cursor=pointer]
                - button "Add route" [disabled] [ref=e392]:
                  - img [ref=e393]
                  - text: Add route
                - generic [ref=e397]: Choose an existing start node.; Choose an existing end node.; Enter a pipe ID.; Choose an existing material ID.; Enter positive outside diameter and wall thickness values.; Enter a finite, nonzero y-reference vector.
              - region "Route review" [ref=e398]:
                - heading "Review and Apply" [level=4] [ref=e399]
                - status [ref=e400]: The affected selection changed. Add again to review the current draft.
                - paragraph [ref=e401]: Add a complete node or route to generate the service validation and exact diff.
                - button "Apply" [disabled] [ref=e402]
              - group [ref=e403]:
                - generic "Unit source" [ref=e404] [cursor=pointer]
            - group [ref=e405]:
              - generic "Pending changes (0)" [ref=e406] [cursor=pointer]
          - region "Property inspector" [ref=e407]:
            - 'heading "Pump nozzle — node: node:N-100" [level=2] [ref=e408]':
              - text: Pump nozzle
              - generic [ref=e409]: "— node: node:N-100"
            - tablist "Inspector views" [ref=e410]:
              - tab "Properties" [ref=e411]
              - tab "Task" [selected] [ref=e412]
            - tabpanel "Editor operation intent" [ref=e413]:
              - generic [ref=e415]: "Draft target: node: node:N-100"
              - heading "Edit name" [level=3] [ref=e416]:
                - img [ref=e417]
                - text: Edit name
              - generic [ref=e420]:
                - generic [ref=e421]:
                  - generic [ref=e422]: Property
                  - combobox "Property to edit" [ref=e423]:
                    - option "Name" [selected]
                    - option "X position"
                    - option "Y position"
                    - option "Z position"
                    - option "Provenance"
                - region "Current value" [ref=e424]:
                  - generic [ref=e425]: Current name
                  - strong [ref=e426]: Pump nozzle
                - generic [ref=e427]:
                  - generic [ref=e428]: New name
                  - textbox "New name" [ref=e429]: Retained inspector task
                - generic "Task actions" [ref=e430]:
                  - button "Add" [ref=e431] [cursor=pointer]:
                    - img [ref=e432]
                    - text: Add
                  - button "Cancel" [ref=e433] [cursor=pointer]
                  - button "Review" [ref=e434] [cursor=pointer]: Review
                  - button "Apply" [ref=e438] [cursor=pointer]:
                    - img [ref=e439]
                    - text: Apply
              - group [ref=e442]:
                - generic "Operation details" [ref=e443] [cursor=pointer]
              - paragraph [ref=e444]: Validate to check this change before applying it.
      - region "Workspace sections" [ref=e445]:
        - generic [ref=e446]:
          - heading "Analyze" [level=2] [ref=e447]
          - button "Close" [ref=e448] [cursor=pointer]:
            - img [ref=e449]
            - text: Close
        - region "Solve section" [ref=e453]:
          - region "Solve execution" [ref=e454]:
            - generic [ref=e455]: Execution
            - region "Solve readiness summary" [ref=e456]:
              - generic [ref=e457]:
                - img [ref=e458]
                - generic [ref=e460]: Mechanics readiness
                - strong [ref=e461]: preview run not started; ready for preview diagnostics
              - generic [ref=e462]:
                - img [ref=e463]
                - generic [ref=e465]: Rule-check readiness
                - strong [ref=e466]: Rule pack · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE); mechanics results remain reviewable only
              - generic [ref=e467]:
                - img [ref=e468]
                - generic [ref=e470]: Diagnostics
                - strong [ref=e471]: 1 diagnostic; 1 warning; 0 blocking/error
              - generic [ref=e472]:
                - img [ref=e473]
                - generic [ref=e475]: Professional boundary
                - strong [ref=e476]: Human · Human review required (HUMAN_REVIEW_REQUIRED)
            - generic [ref=e477]:
              - generic [ref=e478]:
                - generic [ref=e479]:
                  - checkbox "Include known private values in this local export" [ref=e480]
                  - text: Include known private values in this local export
                - generic [ref=e481]: decisions=93; findings=93; blocked=true
                - group [ref=e482]:
                  - generic "Details" [ref=e483] [cursor=pointer]
                - generic [ref=e484]:
                  - img [ref=e485]
                  - text: Solve job JSON
              - generic [ref=e488]: state=not_started; events=1; result_rows=0; cancellation_requested=false
            - generic [ref=e489]:
              - generic [ref=e490]:
                - generic [ref=e491]: Solver mode
                - strong [ref=e492]: selected=sparse_interactive; sparse_default=true; dense_scrutiny=false
              - generic [ref=e493]:
                - generic [ref=e494]: Progress
                - strong [ref=e495]: not_started; preview_service_event_state_only_no_percent_stream; percentages_synthesized=false
              - generic [ref=e496]:
                - generic [ref=e497]: Cancellation
                - strong [ref=e498]: control_visible=true; enabled=false; requested=false; seam=no_job_started; token=none_no_active_backend_job; success_claimed=false
              - generic [ref=e499]:
                - generic [ref=e500]: Result binding
                - strong [ref=e501]: not generated; result rows=0; hashes=0
              - generic [ref=e502]:
                - generic [ref=e503]: Unit policy
                - strong [ref=e504]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; results=none; rows=0; conversion=false
              - generic [ref=e505]:
                - generic [ref=e506]: Boundary
                - strong [ref=e507]: private payload=false; protected content=false; release/professional claim=false; human review required
            - group "Solver mode" [ref=e508]:
              - button "Sparse interactive" [pressed] [ref=e509] [cursor=pointer]:
                - img [ref=e510]
                - text: Sparse interactive
              - button "Dense scrutiny" [ref=e512] [cursor=pointer]:
                - img [ref=e513]
                - text: Dense scrutiny
            - button "Run mechanics preview" [ref=e516] [cursor=pointer]:
              - img [ref=e517]
              - text: Run mechanics preview
            - button "Cancel preview" [disabled] [ref=e519] [cursor=pointer]:
              - img [ref=e520]
              - text: Cancel preview
          - region "Rule-check completeness" [ref=e522]:
            - generic [ref=e523]:
              - img [ref=e524]
              - text: Rule-Check Completeness
            - generic [ref=e526]:
              - generic [ref=e527]:
                - generic [ref=e528]:
                  - checkbox "Include known private values in this local export" [ref=e529]
                  - text: Include known private values in this local export
                - generic [ref=e530]: decisions=122; findings=122; blocked=true
                - group [ref=e531]:
                  - generic "Details" [ref=e532] [cursor=pointer]
                - generic [ref=e533]:
                  - img [ref=e534]
                  - text: Local completeness JSON
              - generic [ref=e537]: 3 review findings; rule_check_blocked=true; mechanics_reviewable=false
            - generic [ref=e538]:
              - generic [ref=e539]:
                - generic [ref=e540]: Rule status
                - strong [ref=e541]: not_performed_user_rule_inputs_missing
              - generic [ref=e542]:
                - generic [ref=e543]: Mechanics status
                - strong [ref=e544]: ready_for_preview_diagnostics
              - generic [ref=e545]:
                - generic [ref=e546]: Unit policy
                - strong [ref=e547]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; records=38; rule_input_units=explicit_or_blocking; diagnostic=RULE_UNIT_MISMATCH; conversion=false
              - generic [ref=e548]:
                - generic [ref=e549]: Boundary
                - strong [ref=e550]: private rule data absent; silent defaults used=false; bundled code values=false; no compliance claim
              - generic [ref=e551]:
                - article [ref=e552]:
                  - strong [ref=e553]: RULE_CHECK_BLOCKING
                  - generic [ref=e554]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e555]: User-supplied rule-pack inputs are absent, so rule-check status stays incomplete while mechanics output remains separate. Provide a private rule pack and project-specific criteria with provenance; the public preview does not bundle code defaults.
                - article [ref=e556]:
                  - strong [ref=e557]: RULE_CHECK_BLOCKING
                  - generic [ref=e558]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e559]: Load cases and combinations are mechanics-basis preview inputs, not code-specific rule combinations. Supply private owner/code combination criteria before reporting user-rule checked status.
                - article [ref=e560]:
                  - strong [ref=e561]: PROVENANCE_WARNING
                  - generic [ref=e562]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e563]: Material and pipe-section values are invented public fixture values, not verified project or catalog data. Replace them with user-controlled data and provenance before engineering reliance.
            - generic [ref=e564]: Completeness findings explain missing user/private rule-check data.
          - region "Run rule checks" [ref=e565]:
            - generic [ref=e566]:
              - img [ref=e567]
              - text: Run Rule Checks (user-defined, private)
            - generic [ref=e570]:
              - generic [ref=e571]: "Project scope: project:invented-loop-01."
              - generic [ref=e572]: No solved mechanics result in this session; run a solve first so solver_result inputs can bind to result rows.
            - generic [ref=e573]:
              - button "Load demo rule pack" [ref=e574]
              - button "Refresh saved packs" [ref=e575]
            - generic [ref=e577]: Saved rule-pack list not refreshed yet.
            - generic [ref=e578]:
              - generic [ref=e579]: Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)
              - textbox "Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)" [ref=e580]:
                - /placeholder: No rule-pack document. Load the demo, open a saved pack, or paste one.
            - generic [ref=e581]:
              - button "Run checks" [disabled] [ref=e582]:
                - img [ref=e583]
                - text: Run checks
              - generic [ref=e585]: No rule-check run in this session.
            - generic [ref=e586]:
              - img [ref=e587]
              - text: Rule checks run locally over user-supplied data only. The software emits user-defined check statuses (USER_RULE_CHECKED / USER_RULE_FAILED / RULE_INPUTS_INCOMPLETE).
          - region "Design knowledge" [ref=e589]:
            - generic [ref=e590]:
              - img [ref=e591]
              - text: Design Knowledge
            - generic [ref=e593]:
              - article [ref=e594]:
                - generic [ref=e595]:
                  - strong [ref=e596]: Pump nozzle endpoint
                  - generic [ref=e597]: endpoint · review_required
                - paragraph [ref=e598]: Invented endpoint record used to show how user-supplied design context appears in the preview.
                - generic [ref=e599]: invented_example
              - article [ref=e600]:
                - generic [ref=e601]:
                  - strong [ref=e602]: Rack corridor placeholder
                  - generic [ref=e603]: routing_corridor · review_required
                - paragraph [ref=e604]: Invented corridor note; no owner standard or protected spacing criterion is bundled.
                - generic [ref=e605]: invented_example
              - article [ref=e606]:
                - generic [ref=e607]:
                  - strong [ref=e608]: Support stiffness dimensions unresolved
                  - generic [ref=e609]: assumption · unresolved
                - paragraph [ref=e610]: Linear and rotational support behavior is represented as a preview diagnostic until user-supplied stiffness values and canonical dimensions are provided.
                - generic [ref=e611]: invented_example
    - complementary "Agent" [ref=e612]:
      - button "Agent" [disabled] [ref=e614]:
        - img [ref=e615]
        - generic [ref=e618]: Agent
  - generic "Workspace status" [ref=e619]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e620] [cursor=pointer]:
      - img [ref=e621]
      - text: 5 Issues
    - generic "Selection" [ref=e623]: "node: node:N-100"
    - generic "Display units" [ref=e624]: Entered
    - button "About SWBPIPE…" [ref=e625] [cursor=pointer]:
      - img [ref=e626]
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