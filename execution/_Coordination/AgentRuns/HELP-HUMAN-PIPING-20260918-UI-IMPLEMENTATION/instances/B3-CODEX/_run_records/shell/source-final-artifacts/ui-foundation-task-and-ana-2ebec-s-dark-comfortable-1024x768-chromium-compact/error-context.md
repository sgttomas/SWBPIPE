# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ui-foundation.spec.ts >> task and analysis dock preserve usable canvas dark comfortable 1024x768
- Location: e2e/ui-foundation.spec.ts:671:7

# Error details

```
Error: expect(received).toBeLessThanOrEqual(expected)

Expected: <= 745
Received:    1256.5
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
              - button "Select Terminal tie-in in viewport" [ref=e213] [cursor=pointer]:
                - img [ref=e214]
                - generic [ref=e217]: N-140
              - button "Select Preview one-way terminal stop in viewport" [ref=e218] [cursor=pointer]:
                - img [ref=e219]
                - generic [ref=e222]: NL-140
              - button "Select Anchor at pump nozzle in viewport" [ref=e223] [cursor=pointer]:
                - img [ref=e224]
                - generic [ref=e227]: S-100
            - img "Orientation gizmo showing X, Y, Z axes"
            - generic "View controls" [ref=e228]:
              - button "Front" [ref=e229] [cursor=pointer]
              - button "Top" [ref=e230] [cursor=pointer]
              - button "Isometric" [pressed] [ref=e231] [cursor=pointer]
            - generic:
              - generic: 1 m
          - region "Command and selection bar" [ref=e232]:
            - generic "Object creation tools" [ref=e233]:
              - button "Node" [ref=e234] [cursor=pointer]:
                - img [ref=e235]
                - text: Node
              - button "Pipe" [pressed] [ref=e237] [cursor=pointer]:
                - img [ref=e238]
                - text: Pipe
              - button "Support" [ref=e242] [cursor=pointer]:
                - img [ref=e243]
                - text: Support
              - button "Component" [ref=e246] [cursor=pointer]:
                - img [ref=e247]
                - text: Component
              - button "Load" [ref=e250] [cursor=pointer]:
                - img [ref=e251]
                - text: Load
            - 'generic "Pipe tool armed: pick from/to nodes or complete the pipe form." [ref=e253]': Pipe tool armed
            - group [ref=e254]:
              - generic "Selection & navigation" [ref=e255] [cursor=pointer]
        - generic [ref=e256]:
          - button "Close inspector" [ref=e257]:
            - img [ref=e258]
          - region "Viewport editor intents" [ref=e262]:
            - heading "Create pipe" [level=3] [ref=e263]
            - group [ref=e264]:
              - generic "Explicit straight pipe connectivity" [ref=e265]:
                - generic [ref=e266]:
                  - generic [ref=e267]: Pipe ID
                  - textbox "New pipe ID" [ref=e268]:
                    - /placeholder: pipe:P-2
                - generic [ref=e269]:
                  - generic [ref=e270]: Label
                  - textbox "New pipe label" [ref=e271]:
                    - /placeholder: Pipe label
                    - text: Retained pipe draft
                - generic [ref=e272]:
                  - generic [ref=e273]:
                    - generic [ref=e274]: From
                    - button "Pick" [pressed] [ref=e275] [cursor=pointer]:
                      - img [ref=e276]
                      - text: Pick
                  - generic [ref=e279]:
                    - generic [ref=e280]:
                      - generic [ref=e281]: New pipe from node
                      - combobox "New pipe from node" [expanded] [ref=e282]
                    - generic "New pipe from node current value" [ref=e283]: No target selected
                    - generic [ref=e284]: 5 of 5 targets
                    - listbox "New pipe from node options" [ref=e285]:
                      - generic [ref=e286]:
                        - option "Pump nozzle node:N-100" [ref=e288] [cursor=pointer]:
                          - generic [ref=e289]: Pump nozzle
                          - generic [ref=e290]: node:N-100
                        - option "Low point elbow node:N-110" [ref=e292] [cursor=pointer]:
                          - generic [ref=e293]: Low point elbow
                          - generic [ref=e294]: node:N-110
                        - option "Riser elbow node:N-120" [ref=e296] [cursor=pointer]:
                          - generic [ref=e297]: Riser elbow
                          - generic [ref=e298]: node:N-120
                        - option "Rack turn node:N-130" [ref=e300] [cursor=pointer]:
                          - generic [ref=e301]: Rack turn
                          - generic [ref=e302]: node:N-130
                        - option "Terminal tie-in node:N-140" [ref=e304] [cursor=pointer]:
                          - generic [ref=e305]: Terminal tie-in
                          - generic [ref=e306]: node:N-140
                - group "End mode" [ref=e307]:
                  - generic [ref=e308]: End mode
                  - generic [ref=e309]:
                    - radio "Existing node" [checked] [ref=e310]
                    - text: Existing node
                  - generic [ref=e311]:
                    - radio "New node" [ref=e312]
                    - text: New node
                - generic [ref=e313]:
                  - generic [ref=e314]:
                    - generic [ref=e315]: To
                    - button "Pick" [ref=e316] [cursor=pointer]:
                      - img [ref=e317]
                      - text: Pick
                  - generic [ref=e320]:
                    - generic [ref=e321]:
                      - generic [ref=e322]: New pipe to node
                      - combobox "New pipe to node" [expanded] [ref=e323]
                    - generic "New pipe to node current value" [ref=e324]: No target selected
                    - generic [ref=e325]: 5 of 5 targets
                    - listbox "New pipe to node options" [ref=e326]:
                      - generic [ref=e327]:
                        - option "Pump nozzle node:N-100" [ref=e329] [cursor=pointer]:
                          - generic [ref=e330]: Pump nozzle
                          - generic [ref=e331]: node:N-100
                        - option "Low point elbow node:N-110" [ref=e333] [cursor=pointer]:
                          - generic [ref=e334]: Low point elbow
                          - generic [ref=e335]: node:N-110
                        - option "Riser elbow node:N-120" [ref=e337] [cursor=pointer]:
                          - generic [ref=e338]: Riser elbow
                          - generic [ref=e339]: node:N-120
                        - option "Rack turn node:N-130" [ref=e341] [cursor=pointer]:
                          - generic [ref=e342]: Rack turn
                          - generic [ref=e343]: node:N-130
                        - option "Terminal tie-in node:N-140" [ref=e345] [cursor=pointer]:
                          - generic [ref=e346]: Terminal tie-in
                          - generic [ref=e347]: node:N-140
                - generic [ref=e348]:
                  - generic [ref=e349]:
                    - generic [ref=e350]: New pipe material
                    - combobox "New pipe material" [expanded] [ref=e351]
                  - generic "New pipe material current value" [ref=e352]: No target selected
                  - generic [ref=e353]: 1 of 1 targets
                  - listbox "New pipe material options" [ref=e354]:
                    - option "Invented carbon-steel-like material material:invented-carbon-steel" [ref=e357] [cursor=pointer]:
                      - generic [ref=e358]: Invented carbon-steel-like material
                      - generic [ref=e359]: material:invented-carbon-steel
                - generic [ref=e360]:
                  - generic [ref=e361]: OD
                  - textbox "New pipe outside diameter" [ref=e362]:
                    - /placeholder: "0.114"
                - generic [ref=e363]:
                  - generic [ref=e364]: Wall
                  - textbox "New pipe wall thickness" [ref=e365]:
                    - /placeholder: "0.006"
                - generic [ref=e366]:
                  - generic [ref=e367]: Length unit
                  - combobox "New pipe length unit" [ref=e368]:
                    - option "m" [selected]
                - generic [ref=e369]: "Pipe geometry: m, model metadata"
                - generic [ref=e370]: "Construction plane inactive: existing endpoint uses exact node IDs."
                - generic [ref=e371]: No route ghost is visible.
                - generic [ref=e372]:
                  - generic [ref=e373]: Yref X
                  - textbox "New pipe y-reference X" [ref=e374]:
                    - /placeholder: "0"
                - generic [ref=e375]:
                  - generic [ref=e376]: Yref Y
                  - textbox "New pipe y-reference Y" [ref=e377]:
                    - /placeholder: "0"
                - generic [ref=e378]:
                  - generic [ref=e379]: Yref Z
                  - textbox "New pipe y-reference Z" [ref=e380]:
                    - /placeholder: "1"
                - generic [ref=e381]:
                  - generic [ref=e382]: Provenance
                  - textbox "New pipe provenance" [ref=e383]: layout regression draft
                - generic [ref=e384]:
                  - checkbox "Continue from end after Apply; keep the entered material, dimensions, orientation and provenance" [ref=e385]
                  - text: Continue from end after Apply; keep the entered material, dimensions, orientation and provenance
                - button "Cancel pipe draft" [ref=e386] [cursor=pointer]
                - button "Add route" [disabled] [ref=e387]:
                  - img [ref=e388]
                  - text: Add route
                - generic [ref=e392]: Choose an existing start node.; Choose an existing end node.; Enter a pipe ID.; Choose an existing material ID.; Enter positive outside diameter and wall thickness values.; Enter a finite, nonzero y-reference vector.
              - region "Route review" [ref=e393]:
                - heading "Review and Apply" [level=4] [ref=e394]
                - status [ref=e395]: The affected selection changed. Add again to review the current draft.
                - paragraph [ref=e396]: Add a complete node or route to generate the service validation and exact diff.
                - button "Apply" [disabled] [ref=e397]
              - group [ref=e398]:
                - generic "Unit source" [ref=e399] [cursor=pointer]
            - group [ref=e400]:
              - generic "Pending changes (0)" [ref=e401] [cursor=pointer]
          - region "Property inspector" [ref=e402]:
            - 'heading "Pump nozzle — node: node:N-100" [level=2] [ref=e403]':
              - text: Pump nozzle
              - generic [ref=e404]: "— node: node:N-100"
            - tablist "Inspector views" [ref=e405]:
              - tab "Properties" [ref=e406]
              - tab "Task" [selected] [ref=e407]
            - tabpanel "Editor operation intent" [ref=e408]:
              - generic [ref=e410]: "Draft target: node: node:N-100"
              - heading "Edit name" [level=3] [ref=e411]:
                - img [ref=e412]
                - text: Edit name
              - generic [ref=e415]:
                - generic [ref=e416]:
                  - generic [ref=e417]: Property
                  - combobox "Property to edit" [ref=e418]:
                    - option "Name" [selected]
                    - option "X position"
                    - option "Y position"
                    - option "Z position"
                    - option "Provenance"
                - region "Current value" [ref=e419]:
                  - generic [ref=e420]: Current name
                  - strong [ref=e421]: Pump nozzle
                - generic [ref=e422]:
                  - generic [ref=e423]: New name
                  - textbox "New name" [ref=e424]: Retained inspector task
                - generic "Task actions" [ref=e425]:
                  - button "Add" [ref=e426] [cursor=pointer]:
                    - img [ref=e427]
                    - text: Add
                  - button "Cancel" [ref=e428] [cursor=pointer]
                  - button "Review" [ref=e429] [cursor=pointer]: Review
                  - button "Apply" [ref=e433] [cursor=pointer]:
                    - img [ref=e434]
                    - text: Apply
              - group [ref=e437]:
                - generic "Operation details" [ref=e438] [cursor=pointer]
              - paragraph [ref=e439]: Validate to check this change before applying it.
      - region "Workspace sections" [ref=e440]:
        - generic [ref=e441]:
          - heading "Analyze" [level=2] [ref=e442]
          - button "Close" [ref=e443] [cursor=pointer]:
            - img [ref=e444]
            - text: Close
        - region "Solve section" [ref=e448]:
          - region "Solve execution" [ref=e449]:
            - generic [ref=e450]: Execution
            - region "Solve readiness summary" [ref=e451]:
              - generic [ref=e452]:
                - img [ref=e453]
                - generic [ref=e455]: Mechanics readiness
                - strong [ref=e456]: preview run not started; ready for preview diagnostics
              - generic [ref=e457]:
                - img [ref=e458]
                - generic [ref=e460]: Rule-check readiness
                - strong [ref=e461]: Rule pack · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE); mechanics results remain reviewable only
              - generic [ref=e462]:
                - img [ref=e463]
                - generic [ref=e465]: Diagnostics
                - strong [ref=e466]: 1 diagnostic; 1 warning; 0 blocking/error
              - generic [ref=e467]:
                - img [ref=e468]
                - generic [ref=e470]: Professional boundary
                - strong [ref=e471]: Human · Human review required (HUMAN_REVIEW_REQUIRED)
            - generic [ref=e472]:
              - generic [ref=e473]:
                - generic [ref=e474]:
                  - checkbox "Include known private values in this local export" [ref=e475]
                  - text: Include known private values in this local export
                - generic [ref=e476]: decisions=93; findings=93; blocked=true
                - group [ref=e477]:
                  - generic "Details" [ref=e478] [cursor=pointer]
                - generic [ref=e479]:
                  - img [ref=e480]
                  - text: Solve job JSON
              - generic [ref=e483]: state=not_started; events=1; result_rows=0; cancellation_requested=false
            - generic [ref=e484]:
              - generic [ref=e485]:
                - generic [ref=e486]: Solver mode
                - strong [ref=e487]: selected=sparse_interactive; sparse_default=true; dense_scrutiny=false
              - generic [ref=e488]:
                - generic [ref=e489]: Progress
                - strong [ref=e490]: not_started; preview_service_event_state_only_no_percent_stream; percentages_synthesized=false
              - generic [ref=e491]:
                - generic [ref=e492]: Cancellation
                - strong [ref=e493]: control_visible=true; enabled=false; requested=false; seam=no_job_started; token=none_no_active_backend_job; success_claimed=false
              - generic [ref=e494]:
                - generic [ref=e495]: Result binding
                - strong [ref=e496]: not generated; result rows=0; hashes=0
              - generic [ref=e497]:
                - generic [ref=e498]: Unit policy
                - strong [ref=e499]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; results=none; rows=0; conversion=false
              - generic [ref=e500]:
                - generic [ref=e501]: Boundary
                - strong [ref=e502]: private payload=false; protected content=false; release/professional claim=false; human review required
            - group "Solver mode" [ref=e503]:
              - button "Sparse interactive" [pressed] [ref=e504] [cursor=pointer]:
                - img [ref=e505]
                - text: Sparse interactive
              - button "Dense scrutiny" [ref=e507] [cursor=pointer]:
                - img [ref=e508]
                - text: Dense scrutiny
            - button "Run mechanics preview" [ref=e511] [cursor=pointer]:
              - img [ref=e512]
              - text: Run mechanics preview
            - button "Cancel preview" [disabled] [ref=e514] [cursor=pointer]:
              - img [ref=e515]
              - text: Cancel preview
          - region "Rule-check completeness" [ref=e517]:
            - generic [ref=e518]:
              - img [ref=e519]
              - text: Rule-Check Completeness
            - generic [ref=e521]:
              - generic [ref=e522]:
                - generic [ref=e523]:
                  - checkbox "Include known private values in this local export" [ref=e524]
                  - text: Include known private values in this local export
                - generic [ref=e525]: decisions=122; findings=122; blocked=true
                - group [ref=e526]:
                  - generic "Details" [ref=e527] [cursor=pointer]
                - generic [ref=e528]:
                  - img [ref=e529]
                  - text: Local completeness JSON
              - generic [ref=e532]: 3 review findings; rule_check_blocked=true; mechanics_reviewable=false
            - generic [ref=e533]:
              - generic [ref=e534]:
                - generic [ref=e535]: Rule status
                - strong [ref=e536]: not_performed_user_rule_inputs_missing
              - generic [ref=e537]:
                - generic [ref=e538]: Mechanics status
                - strong [ref=e539]: ready_for_preview_diagnostics
              - generic [ref=e540]:
                - generic [ref=e541]: Unit policy
                - strong [ref=e542]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; records=38; rule_input_units=explicit_or_blocking; diagnostic=RULE_UNIT_MISMATCH; conversion=false
              - generic [ref=e543]:
                - generic [ref=e544]: Boundary
                - strong [ref=e545]: private rule data absent; silent defaults used=false; bundled code values=false; no compliance claim
              - generic [ref=e546]:
                - article [ref=e547]:
                  - strong [ref=e548]: RULE_CHECK_BLOCKING
                  - generic [ref=e549]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e550]: User-supplied rule-pack inputs are absent, so rule-check status stays incomplete while mechanics output remains separate. Provide a private rule pack and project-specific criteria with provenance; the public preview does not bundle code defaults.
                - article [ref=e551]:
                  - strong [ref=e552]: RULE_CHECK_BLOCKING
                  - generic [ref=e553]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e554]: Load cases and combinations are mechanics-basis preview inputs, not code-specific rule combinations. Supply private owner/code combination criteria before reporting user-rule checked status.
                - article [ref=e555]:
                  - strong [ref=e556]: PROVENANCE_WARNING
                  - generic [ref=e557]: warning; rule_blocking=true; private_data=true
                  - paragraph [ref=e558]: Material and pipe-section values are invented public fixture values, not verified project or catalog data. Replace them with user-controlled data and provenance before engineering reliance.
            - generic [ref=e559]: Completeness findings explain missing user/private rule-check data.
          - region "Run rule checks" [ref=e560]:
            - generic [ref=e561]:
              - img [ref=e562]
              - text: Run Rule Checks (user-defined, private)
            - generic [ref=e565]:
              - generic [ref=e566]: "Project scope: project:invented-loop-01."
              - generic [ref=e567]: No solved mechanics result in this session; run a solve first so solver_result inputs can bind to result rows.
            - generic [ref=e568]:
              - button "Load demo rule pack" [ref=e569]
              - button "Refresh saved packs" [ref=e570]
            - generic [ref=e572]: Saved rule-pack list not refreshed yet.
            - generic [ref=e573]:
              - generic [ref=e574]: Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)
              - textbox "Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)" [ref=e575]:
                - /placeholder: No rule-pack document. Load the demo, open a saved pack, or paste one.
            - generic [ref=e576]:
              - button "Run checks" [disabled] [ref=e577]:
                - img [ref=e578]
                - text: Run checks
              - generic [ref=e580]: No rule-check run in this session.
            - generic [ref=e581]:
              - img [ref=e582]
              - text: Rule checks run locally over user-supplied data only. The software emits user-defined check statuses (USER_RULE_CHECKED / USER_RULE_FAILED / RULE_INPUTS_INCOMPLETE).
          - region "Design knowledge" [ref=e584]:
            - generic [ref=e585]:
              - img [ref=e586]
              - text: Design Knowledge
            - generic [ref=e588]:
              - article [ref=e589]:
                - generic [ref=e590]:
                  - strong [ref=e591]: Pump nozzle endpoint
                  - generic [ref=e592]: endpoint · review_required
                - paragraph [ref=e593]: Invented endpoint record used to show how user-supplied design context appears in the preview.
                - generic [ref=e594]: invented_example
              - article [ref=e595]:
                - generic [ref=e596]:
                  - strong [ref=e597]: Rack corridor placeholder
                  - generic [ref=e598]: routing_corridor · review_required
                - paragraph [ref=e599]: Invented corridor note; no owner standard or protected spacing criterion is bundled.
                - generic [ref=e600]: invented_example
              - article [ref=e601]:
                - generic [ref=e602]:
                  - strong [ref=e603]: Support stiffness dimensions unresolved
                  - generic [ref=e604]: assumption · unresolved
                - paragraph [ref=e605]: Linear and rotational support behavior is represented as a preview diagnostic until user-supplied stiffness values and canonical dimensions are provided.
                - generic [ref=e606]: invented_example
    - complementary "Agent" [ref=e607]:
      - button "Agent" [disabled] [ref=e609]:
        - img [ref=e610]
        - generic [ref=e613]: Agent
  - generic "Workspace status" [ref=e614]:
    - generic "Analysis statuses"
    - button "5 Issues" [ref=e615] [cursor=pointer]:
      - img [ref=e616]
      - text: 5 Issues
    - generic "Selection" [ref=e618]: "node: node:N-100"
    - generic "Display units" [ref=e619]: Entered
    - button "About SWBPIPE…" [ref=e620] [cursor=pointer]:
      - img [ref=e621]
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