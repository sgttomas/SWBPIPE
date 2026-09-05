# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: r2-smoke.spec.ts >> R2 desktop preview smoke covers solve, results, report, and viewport overlay
- Location: e2e/r2-smoke.spec.ts:180:1

# Error details

```
Error: result-row-result:force:pipe-P-120:axial must not be covered by workspace status

expect(received).toBe(expected) // Object.is equality

Expected: false
Received: true
```

# Page snapshot

```yaml
- main [ref=e3]:
  - generic [ref=e4]:
    - generic [ref=e5]:
      - heading "OpenPipeStress" [level=1] [ref=e6]
      - paragraph [ref=e7]: Invented Utility Loop Preview
    - generic "Local project controls" [ref=e8]:
      - generic [ref=e10]:
        - text: Display units
        - combobox "Display units" [ref=e11]:
          - option "Entered" [selected]
          - option "SI"
          - option "US"
        - paragraph [ref=e12]: Readouts use the selected units where conversion is available. Unavailable quantities retain entered values. Editing uses entered units.
      - generic [ref=e13]: Invented Utility Loop Preview
      - button "Create local" [ref=e14] [cursor=pointer]:
        - img [ref=e15]
        - text: Create local
      - button "New blank" [ref=e19] [cursor=pointer]:
        - img [ref=e20]
        - text: New blank
      - button "Open local" [ref=e23] [cursor=pointer]:
        - img [ref=e24]
        - text: Open local
      - button "List local" [ref=e26] [cursor=pointer]:
        - img [ref=e27]
        - text: List local
      - button "Save local" [ref=e28] [cursor=pointer]:
        - img [ref=e29]
        - text: Save local
  - region "Project summary" [ref=e33]:
    - generic [ref=e34]: Local project store not opened.
    - generic [ref=e35]: "Review context: 0 pending operations; applied_operations=0; editor_intents=0; agent_proposals=0."
  - navigation "Application menu" [ref=e36]:
    - button "File" [ref=e38] [cursor=pointer]
    - button "Edit" [ref=e40] [cursor=pointer]
    - button "View" [ref=e42] [cursor=pointer]
    - button "Insert" [ref=e44] [cursor=pointer]
    - button "Analyze" [ref=e46] [cursor=pointer]
  - generic [ref=e47]:
    - region "Modeling workspace" [ref=e48]:
      - generic [ref=e49]:
        - button "Collapse model tree" [expanded] [ref=e50] [cursor=pointer]:
          - generic [ref=e51]: Model Tree
          - generic [ref=e52]: ‹
        - generic "Model tree" [ref=e53]:
          - generic [ref=e54]: Model
          - region "Layout grid mode" [ref=e55]:
            - button "Tree" [pressed] [ref=e56]:
              - img [ref=e57]
              - text: Tree
            - button "Grid" [ref=e60]:
              - img [ref=e61]
              - text: Grid
          - region "Model tree filtering" [ref=e63]:
            - generic [ref=e64]:
              - img [ref=e65]
              - generic [ref=e68]: Filter model
              - searchbox "Filter model tree" [ref=e69]
            - generic [ref=e70]: 26 of 26 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e71]:
              - img [ref=e72]
          - button "Invented Utility Loop Preview" [ref=e75] [cursor=pointer]:
            - img [ref=e76]
            - strong [ref=e81]: Invented Utility Loop Preview
          - generic [ref=e82]:
            - heading "Materials" [level=3] [ref=e83]
            - button "Invented carbon-steel-like material material:invented-carbon-steel" [ref=e84] [cursor=pointer]:
              - img [ref=e85]
              - generic [ref=e88]:
                - strong [ref=e89]: Invented carbon-steel-like material
                - generic [ref=e90]: material:invented-carbon-steel
          - generic [ref=e91]:
            - heading "Nodes" [level=3] [ref=e92]
            - button "Pump nozzle node:N-100" [ref=e93] [cursor=pointer]:
              - img [ref=e94]
              - generic [ref=e97]:
                - strong [ref=e98]: Pump nozzle
                - generic [ref=e99]: node:N-100
            - button "Low point elbow node:N-110" [ref=e100] [cursor=pointer]:
              - img [ref=e101]
              - generic [ref=e104]:
                - strong [ref=e105]: Low point elbow
                - generic [ref=e106]: node:N-110
            - button "Riser elbow node:N-120" [ref=e107] [cursor=pointer]:
              - img [ref=e108]
              - generic [ref=e111]:
                - strong [ref=e112]: Riser elbow
                - generic [ref=e113]: node:N-120
            - button "Rack turn node:N-130" [ref=e114] [cursor=pointer]:
              - img [ref=e115]
              - generic [ref=e118]:
                - strong [ref=e119]: Rack turn
                - generic [ref=e120]: node:N-130
            - button "Terminal tie-in node:N-140" [pressed] [ref=e121] [cursor=pointer]:
              - img [ref=e122]
              - generic [ref=e125]:
                - strong [ref=e126]: Terminal tie-in
                - generic [ref=e127]: node:N-140
          - generic [ref=e128]:
            - heading "Pipe Segments" [level=3] [ref=e129]
            - button "Pump discharge run pipe:P-100" [ref=e130] [cursor=pointer]:
              - img [ref=e131]
              - generic [ref=e135]:
                - strong [ref=e136]: Pump discharge run
                - generic [ref=e137]: pipe:P-100
            - button "Vertical riser pipe:P-110" [ref=e138] [cursor=pointer]:
              - img [ref=e139]
              - generic [ref=e143]:
                - strong [ref=e144]: Vertical riser
                - generic [ref=e145]: pipe:P-110
            - button "Rack span pipe:P-120" [ref=e146] [cursor=pointer]:
              - img [ref=e147]
              - generic [ref=e151]:
                - strong [ref=e152]: Rack span
                - generic [ref=e153]: pipe:P-120
            - button "Tie-in rise pipe:P-130" [ref=e154] [cursor=pointer]:
              - img [ref=e155]
              - generic [ref=e159]:
                - strong [ref=e160]: Tie-in rise
                - generic [ref=e161]: pipe:P-130
          - generic [ref=e162]:
            - heading "Supports" [level=3] [ref=e163]
            - button "Anchor at pump nozzle support:S-100" [ref=e164] [cursor=pointer]:
              - img [ref=e165]
              - generic [ref=e168]:
                - strong [ref=e169]: Anchor at pump nozzle
                - generic [ref=e170]: support:S-100
            - button "Guide on riser support:S-120" [ref=e171] [cursor=pointer]:
              - img [ref=e172]
              - generic [ref=e175]:
                - strong [ref=e176]: Guide on riser
                - generic [ref=e177]: support:S-120
            - button "Rack shoe support:S-130" [ref=e178] [cursor=pointer]:
              - img [ref=e179]
              - generic [ref=e182]:
                - strong [ref=e183]: Rack shoe
                - generic [ref=e184]: support:S-130
            - button "Preview one-way terminal stop support:NL-140" [ref=e185] [cursor=pointer]:
              - img [ref=e186]
              - generic [ref=e189]:
                - strong [ref=e190]: Preview one-way terminal stop
                - generic [ref=e191]: support:NL-140
            - button "Preview sliding-friction rack shoe support:NL-130-FRIC" [ref=e192] [cursor=pointer]:
              - img [ref=e193]
              - generic [ref=e196]:
                - strong [ref=e197]: Preview sliding-friction rack shoe
                - generic [ref=e198]: support:NL-130-FRIC
            - button "Invented variable spring hanger support:SH-140" [ref=e199] [cursor=pointer]:
              - img [ref=e200]
              - generic [ref=e203]:
                - strong [ref=e204]: Invented variable spring hanger
                - generic [ref=e205]: support:SH-140
            - button "Invented constant-effort support support:CE-120" [ref=e206] [cursor=pointer]:
              - img [ref=e207]
              - generic [ref=e210]:
                - strong [ref=e211]: Invented constant-effort support
                - generic [ref=e212]: support:CE-120
          - generic [ref=e213]:
            - heading "Components" [level=3] [ref=e214]
            - button "Invented elbow marker component:C-110" [ref=e215] [cursor=pointer]:
              - img [ref=e216]
              - generic [ref=e219]:
                - strong [ref=e220]: Invented elbow marker
                - generic [ref=e221]: component:C-110
            - button "Invented branch connection marker component:C-120" [ref=e222] [cursor=pointer]:
              - img [ref=e223]
              - generic [ref=e226]:
                - strong [ref=e227]: Invented branch connection marker
                - generic [ref=e228]: component:C-120
            - button "Invented semi-rigid valve marker component:C-130" [ref=e229] [cursor=pointer]:
              - img [ref=e230]
              - generic [ref=e233]:
                - strong [ref=e234]: Invented semi-rigid valve marker
                - generic [ref=e235]: component:C-130
            - button "Invented tie-in marker component:C-140" [ref=e236] [cursor=pointer]:
              - img [ref=e237]
              - generic [ref=e240]:
                - strong [ref=e241]: Invented tie-in marker
                - generic [ref=e242]: component:C-140
            - button "Invented expansion joint marker component:C-150" [ref=e243] [cursor=pointer]:
              - img [ref=e244]
              - generic [ref=e247]:
                - strong [ref=e248]: Invented expansion joint marker
                - generic [ref=e249]: component:C-150
          - generic [ref=e250]:
            - heading "Load Cases" [level=3] [ref=e251]
            - button "Invented operating gravity and pressure preview load:L-100" [ref=e252] [cursor=pointer]:
              - img [ref=e253]
              - generic [ref=e255]:
                - strong [ref=e256]: Invented operating gravity and pressure preview
                - generic [ref=e257]: load:L-100
            - button "Invented alternate gravity and pressure preview load:L-200" [ref=e258] [cursor=pointer]:
              - img [ref=e259]
              - generic [ref=e261]:
                - strong [ref=e262]: Invented alternate gravity and pressure preview
                - generic [ref=e263]: load:L-200
          - generic [ref=e264]:
            - heading "Combinations" [level=3] [ref=e265]
            - button "Invented explicit operating plus alternate preview combination:C-OPER-ALT" [ref=e266] [cursor=pointer]:
              - img [ref=e267]
              - generic [ref=e271]:
                - strong [ref=e272]: Invented explicit operating plus alternate preview
                - generic [ref=e273]: combination:C-OPER-ALT
      - generic [ref=e274]:
        - region "Human toolkit" [ref=e275]:
          - button "Toolkit" [ref=e276]
        - generic [ref=e277]:
          - generic [ref=e278]:
            - generic [ref=e279]: 3D Centerline
            - generic "Viewport deformation overlay status" [ref=e280]:
              - strong [ref=e281]:
                - text: available; nodes=5; max=
                - generic [ref=e282]: 4.567557 mm
              - generic [ref=e283]: scale=normalized_display_offset_not_physical_length; vector_direction=global_cartesian_displacement_components; unit_basis=mm; professional_claim=false
            - group "Viewport display toggles" [ref=e284]:
              - button "Labels" [pressed] [ref=e285]
              - button "Loads" [pressed] [ref=e286]
              - button "Grid" [pressed] [ref=e287]
            - generic [ref=e288]: "Selected: node:N-140"
          - generic [ref=e289]:
            - generic "Three.js pipe centerline viewport" [ref=e290]
            - generic "Viewport entity selection":
              - button "Select Pump nozzle in viewport" [ref=e292] [cursor=pointer]:
                - img [ref=e293]
                - generic [ref=e296]: N-100
              - button "Select Low point elbow in viewport" [ref=e297] [cursor=pointer]:
                - img [ref=e298]
                - generic [ref=e301]: N-110
              - button "Select Riser elbow in viewport" [ref=e302] [cursor=pointer]:
                - img [ref=e303]
                - generic [ref=e306]: N-120
              - button "Select Rack turn in viewport" [ref=e307] [cursor=pointer]:
                - img [ref=e308]
                - generic [ref=e311]: N-130
              - button "Select Terminal tie-in in viewport" [pressed] [ref=e312] [cursor=pointer]:
                - img [ref=e313]
                - generic [ref=e316]: N-140
              - button "Select Pump discharge run in viewport" [ref=e317] [cursor=pointer]:
                - img [ref=e318]
                - generic [ref=e322]: P-100
              - button "Select Vertical riser in viewport" [ref=e323] [cursor=pointer]:
                - img [ref=e324]
                - generic [ref=e328]: P-110
              - button "Select Rack span in viewport" [ref=e329] [cursor=pointer]:
                - img [ref=e330]
                - generic [ref=e334]: P-120
              - button "Select Tie-in rise in viewport" [ref=e335] [cursor=pointer]:
                - img [ref=e336]
                - generic [ref=e340]: P-130
              - button "Select Anchor at pump nozzle in viewport" [ref=e341] [cursor=pointer]:
                - img [ref=e342]
                - generic [ref=e345]: S-100
              - button "Select Guide on riser in viewport" [ref=e346] [cursor=pointer]:
                - img [ref=e347]
                - generic [ref=e350]: S-120
              - button "Select Rack shoe in viewport" [ref=e351] [cursor=pointer]:
                - img [ref=e352]
                - generic [ref=e355]: S-130
              - button "Select Preview one-way terminal stop in viewport" [ref=e356] [cursor=pointer]:
                - img [ref=e357]
                - generic [ref=e360]: NL-140
              - button "Select Preview sliding-friction rack shoe in viewport" [ref=e361] [cursor=pointer]:
                - img [ref=e362]
                - generic [ref=e365]: NL-130-FRIC
              - button "Select Invented variable spring hanger in viewport" [ref=e366] [cursor=pointer]:
                - img [ref=e367]
                - generic [ref=e370]: SH-140
              - button "Select Invented constant-effort support in viewport" [ref=e371] [cursor=pointer]:
                - img [ref=e372]
                - generic [ref=e375]: CE-120
              - button "Select Invented elbow marker in viewport" [ref=e376] [cursor=pointer]:
                - img [ref=e377]
                - generic [ref=e380]: C-110
              - button "Select Invented branch connection marker in viewport" [ref=e381] [cursor=pointer]:
                - img [ref=e382]
                - generic [ref=e385]: C-120
              - button "Select Invented semi-rigid valve marker in viewport" [ref=e386] [cursor=pointer]:
                - img [ref=e387]
                - generic [ref=e390]: C-130
              - button "Select Invented tie-in marker in viewport" [ref=e391] [cursor=pointer]:
                - img [ref=e392]
                - generic [ref=e395]: C-140
              - button "Select Invented expansion joint marker in viewport" [ref=e396] [cursor=pointer]:
                - img [ref=e397]
                - generic [ref=e400]: C-150
            - generic "Orientation gizmo" [ref=e401]
            - generic "View controls" [ref=e404]:
              - button "Front" [ref=e405] [cursor=pointer]
              - button "Top" [ref=e406] [cursor=pointer]
              - button "Iso" [pressed] [ref=e407] [cursor=pointer]
            - generic [ref=e409]: 1 m
          - region "Command and selection bar" [ref=e410]:
            - generic "Object creation tools" [ref=e411]:
              - button "Node" [ref=e412] [cursor=pointer]:
                - img [ref=e413]
                - text: Node
              - button "Pipe" [pressed] [ref=e415] [cursor=pointer]:
                - img [ref=e416]
                - text: Pipe
              - button "Support" [ref=e420] [cursor=pointer]:
                - img [ref=e421]
                - text: Support
              - button "Component" [ref=e424] [cursor=pointer]:
                - img [ref=e425]
                - text: Component
              - button "Load" [ref=e428] [cursor=pointer]:
                - img [ref=e429]
                - text: Load
            - button "Queue preview" [ref=e431] [cursor=pointer]
            - generic [ref=e432]: "Pipe tool armed: pick from/to nodes or complete the pipe form."
            - generic [ref=e433]: "Selected node: node:N-140; 0 queued"
            - generic [ref=e434]: Drag to orbit · scroll to zoom · right-drag to pan
          - region "Viewport editor intents" [ref=e435]:
            - generic [ref=e436]:
              - generic "Explicit straight pipe connectivity" [ref=e437]:
                - generic [ref=e438]:
                  - generic [ref=e439]: Pipe ID
                  - textbox "New pipe ID" [ref=e440]:
                    - /placeholder: pipe:P-2
                - generic [ref=e441]:
                  - generic [ref=e442]: Label
                  - textbox "New pipe label" [ref=e443]:
                    - /placeholder: Pipe label
                - generic [ref=e444]:
                  - generic [ref=e445]:
                    - generic [ref=e446]: From
                    - button "Pick" [ref=e447] [cursor=pointer]:
                      - img [ref=e448]
                      - text: Pick
                  - combobox "New pipe from node" [ref=e451]:
                    - option "From node"
                    - option "Pump nozzle (node:N-100)" [selected]
                    - option "Low point elbow (node:N-110)"
                    - option "Riser elbow (node:N-120)"
                    - option "Rack turn (node:N-130)"
                    - option "Terminal tie-in (node:N-140)"
                - generic [ref=e452]:
                  - generic [ref=e453]:
                    - generic [ref=e454]: To
                    - button "Pick" [ref=e455] [cursor=pointer]:
                      - img [ref=e456]
                      - text: Pick
                  - combobox "New pipe to node" [ref=e459]:
                    - option "To node"
                    - option "Pump nozzle (node:N-100)"
                    - option "Low point elbow (node:N-110)"
                    - option "Riser elbow (node:N-120)"
                    - option "Rack turn (node:N-130)"
                    - option "Terminal tie-in (node:N-140)" [selected]
                - generic [ref=e460]:
                  - generic [ref=e461]: Material
                  - combobox "New pipe material" [ref=e462]:
                    - option "Material" [selected]
                    - option "Invented carbon-steel-like material (material:invented-carbon-steel)"
                - generic [ref=e463]:
                  - generic [ref=e464]: OD
                  - textbox "New pipe outside diameter" [ref=e465]:
                    - /placeholder: "0.114"
                - generic [ref=e466]:
                  - generic [ref=e467]: Wall
                  - textbox "New pipe wall thickness" [ref=e468]:
                    - /placeholder: "0.006"
                - generic [ref=e469]:
                  - generic [ref=e470]: Length unit
                  - combobox "New pipe length unit" [ref=e471]:
                    - option "m" [selected]
                - generic [ref=e472]: "Pipe geometry: m, model metadata"
                - generic [ref=e473]:
                  - generic [ref=e474]: Yref X
                  - textbox "New pipe y-reference X" [ref=e475]:
                    - /placeholder: "0"
                - generic [ref=e476]:
                  - generic [ref=e477]: Yref Y
                  - textbox "New pipe y-reference Y" [ref=e478]:
                    - /placeholder: "0"
                - generic [ref=e479]:
                  - generic [ref=e480]: Yref Z
                  - textbox "New pipe y-reference Z" [ref=e481]:
                    - /placeholder: "1"
                - generic [ref=e482]:
                  - generic [ref=e483]: Provenance
                  - textbox "New pipe provenance" [ref=e484]: user_entered_local_preview
                - generic [ref=e485]:
                  - checkbox "Continue from end after queue; keep the entered material, dimensions, orientation and provenance" [ref=e486]
                  - text: Continue from end after queue; keep the entered material, dimensions, orientation and provenance
                - button "Cancel pipe draft" [ref=e487] [cursor=pointer]
                - button "Queue pipe" [disabled] [ref=e488]:
                  - img [ref=e489]
                  - text: Queue pipe
              - generic [ref=e493]: browser preview uses model metadata for viewport length units
            - paragraph [ref=e495]: Editor gestures create pending service-validation intents; they do not mutate persisted project data directly.
      - generic [ref=e496]:
        - button "Collapse inspector" [expanded] [ref=e497] [cursor=pointer]:
          - generic [ref=e498]: Inspector
          - generic [ref=e499]: ›
        - generic "Property inspector" [ref=e500]:
          - generic [ref=e501]: Properties
          - heading "Terminal tie-in" [level=2] [ref=e502]
          - generic [ref=e503]:
            - generic [ref=e504]:
              - term [ref=e505]: ID
              - definition [ref=e506]: node:N-140
            - generic [ref=e507]:
              - term [ref=e508]: Label
              - definition [ref=e509]: Terminal tie-in
            - generic [ref=e510]:
              - term [ref=e511]: Position
              - definition [ref=e512]:
                - generic [ref=e513]:
                  - text: x=
                  - generic [ref=e514]: 7.6 m
                  - text: ","
                - generic [ref=e515]:
                  - text: y=
                  - generic [ref=e516]: 2.4 m
                  - text: ","
                - generic [ref=e517]:
                  - text: z=
                  - generic [ref=e518]: 2.2 m
            - generic [ref=e519]:
              - term [ref=e520]: Provenance
              - definition [ref=e521]: invented_example
          - region "Inspector review context" [ref=e522]:
            - region "Display quantity" [ref=e523]:
              - generic [ref=e524]: Display quantity
              - strong [ref=e525]:
                - generic [ref=e526]: Terminal tie-in none
              - generic [ref=e527]: "Entered: Terminal tie-in none"
              - generic [ref=e528]: "Display basis: none, model metadata"
              - generic [ref=e529]: Display only — storage stays entered-units-preserved.
            - region "Provenance" [ref=e530]:
              - generic [ref=e531]: Provenance
              - generic [ref=e533]:
                - term [ref=e534]: Source
                - definition [ref=e535]: invented_example
            - region "Required flags" [ref=e536]:
              - generic [ref=e537]: Required
              - generic [ref=e538]: No missing selected fields detected.
          - region "Unit catalog status" [ref=e539]:
            - heading "Unit basis" [level=3] [ref=e540]
            - generic [ref=e541]:
              - generic [ref=e542]:
                - generic [ref=e543]: Catalog
                - strong [ref=e544]: browser preview uses model metadata; no fallback catalog
              - generic [ref=e545]:
                - generic [ref=e546]: Field units
                - strong [ref=e547]: m, model metadata; Pa, model metadata; N/m, model metadata; 1/degC, model metadata
          - region "Editor operation intent" [ref=e548]:
            - heading "Review-only edit intent" [level=3] [ref=e549]
            - generic [ref=e550]:
              - generic [ref=e551]:
                - generic [ref=e552]: Field
                - combobox "Intent field" [ref=e553]:
                  - option "Label" [selected]
                  - option "X position"
                  - option "Y position"
                  - option "Z position"
                  - option "Provenance"
              - generic [ref=e554]:
                - generic [ref=e555]: Proposed value
                - textbox "Proposed editor value" [ref=e556]: Terminal tie-in
              - generic [ref=e557]:
                - generic [ref=e558]: Rationale
                - textbox "Editor intent rationale" [ref=e559]: user_entered_preview_change
              - button "Queue review intent" [disabled] [ref=e560]:
                - img [ref=e561]
                - text: Queue review intent
              - button "Validate intent" [disabled] [ref=e562]:
                - img [ref=e563]
                - text: Validate intent
            - article [ref=e567]:
              - generic [ref=e568]:
                - generic [ref=e569]:
                  - generic [ref=e570]: Operation
                  - strong [ref=e571]: op:editor-intent-node:N-140-label; modify; proposed
                - generic [ref=e572]:
                  - generic [ref=e573]: Target
                  - strong [ref=e574]: Node; node:N-140
                - generic [ref=e575]:
                  - generic [ref=e576]: Change
                  - strong [ref=e577]: set_field; Label; label; before=Terminal tie-in; after=Terminal tie-in
                - generic [ref=e578]:
                  - generic [ref=e579]: Unit basis
                  - strong [ref=e580]: dimensionless; unit=none; node label only
                - generic [ref=e581]:
                  - generic [ref=e582]: Validation
                  - strong [ref=e583]: not_run; not_run; not_required_dimensionless; not_generated; not_applied
                - generic [ref=e584]:
                  - generic [ref=e585]: Audit boundary
                  - strong [ref=e586]: Routed through the structured operations only; no direct model mutation; requires your acceptance; does not change the accepted model until applied
                - generic [ref=e587]:
                  - generic [ref=e588]: Professional boundary
                  - strong [ref=e589]: Requires human review; no compliance claim; no approval claim
                - generic [ref=e590]:
                  - generic [ref=e591]: Rationale
                  - strong [ref=e592]: user_entered_preview_change
            - paragraph [ref=e593]: Validate this draft intent to preview structured-operation findings before queuing or applying it.
            - paragraph [ref=e594]: No transient editor intents queued.
          - region "Support configuration" [ref=e595]:
            - heading "Support configuration" [level=3] [ref=e596]
            - paragraph [ref=e597]: Choose the support family and enter its physical inputs. Family names do not supply restraints or engineering values. Missing inputs remain missing; validation determines readiness.
            - group "Create support" [ref=e598]:
              - generic [ref=e599]: Create support
              - generic [ref=e600]:
                - text: Support ID
                - textbox "Support ID" [ref=e601]
              - generic [ref=e602]:
                - text: Support label
                - textbox "Support label" [ref=e603]
              - generic [ref=e604]:
                - text: Support node
                - combobox "Support node" [ref=e605]:
                  - option "Not provided"
                  - option "node:N-100"
                  - option "node:N-110"
                  - option "node:N-120"
                  - option "node:N-130"
                  - option "node:N-140" [selected]
              - generic [ref=e606]:
                - text: Support family
                - combobox "Support family" [ref=e607]:
                  - option "Not provided (preserved)" [disabled] [selected]
                  - option "Anchor"
                  - option "Guide"
                  - option "Line stop"
                  - option "Vertical support"
                  - option "Spring"
                  - option "Variable spring hanger"
                  - option "Spring hanger"
                  - option "Constant-effort support"
                  - option "Nonlinear support"
              - group "Restrained degrees of freedom" [ref=e608]:
                - generic [ref=e609]: Restrained degrees of freedom
                - generic [ref=e610]:
                  - checkbox "Restrain UX" [ref=e611]
                  - text: UX
                - generic [ref=e612]:
                  - checkbox "Restrain UY" [ref=e613]
                  - text: UY
                - generic [ref=e614]:
                  - checkbox "Restrain UZ" [ref=e615]
                  - text: UZ
                - generic [ref=e616]:
                  - checkbox "Restrain RX" [ref=e617]
                  - text: RX
                - generic [ref=e618]:
                  - checkbox "Restrain RY" [ref=e619]
                  - text: RY
                - generic [ref=e620]:
                  - checkbox "Restrain RZ" [ref=e621]
                  - text: RZ
              - generic [ref=e622]:
                - text: Support provenance
                - textbox "Support provenance" [ref=e623]
              - button "Add support stiffness" [ref=e624]
              - generic [ref=e625]:
                - checkbox "Include hanger data" [ref=e626]
                - text: Include hanger data
              - generic [ref=e627]:
                - checkbox "Include nonlinear behavior" [ref=e628]
                - text: Include nonlinear behavior
              - button "Queue support creation" [ref=e629]
          - region "Create section intent" [ref=e630]:
            - heading "Create section" [level=3] [ref=e631]
            - generic [ref=e632]:
              - generic [ref=e633]:
                - generic [ref=e634]: Section ID
                - textbox "New section ID" [ref=e635]: section:S-1
              - generic [ref=e636]:
                - generic [ref=e637]: Name
                - textbox "New section name" [ref=e638]: Section S-1
              - generic [ref=e639]:
                - generic [ref=e640]: Type
                - combobox "New section type" [ref=e641]:
                  - option "pipe" [selected]
              - generic [ref=e642]:
                - generic [ref=e643]: Length unit
                - combobox "New section length unit" [ref=e644]:
                  - option "m, model metadata" [selected]
              - generic [ref=e645]:
                - generic [ref=e646]: Outside diameter (m, model metadata)
                - textbox "New section outside diameter" [ref=e647]
              - generic [ref=e648]:
                - generic [ref=e649]: Wall thickness (m, model metadata)
                - textbox "New section wall thickness" [ref=e650]
              - generic [ref=e651]:
                - generic [ref=e652]: Provenance
                - textbox "New section provenance" [ref=e653]: user_entered_local_preview
              - button "Queue section" [disabled] [ref=e654]:
                - img [ref=e655]
                - text: Queue section
          - region "Create material intent" [ref=e657]:
            - heading "Create material" [level=3] [ref=e658]
            - generic [ref=e659]:
              - generic [ref=e660]:
                - generic [ref=e661]: Material ID
                - textbox "New material ID" [ref=e662]: material:M-1
              - generic [ref=e663]:
                - generic [ref=e664]: Label
                - textbox "New material label" [ref=e665]: Material M-1
              - generic [ref=e666]:
                - generic [ref=e667]: Modulus unit
                - combobox "New material modulus unit" [ref=e668]:
                  - option "Pa, model metadata" [selected]
              - generic [ref=e669]:
                - generic [ref=e670]: Elastic modulus (Pa, model metadata)
                - textbox "New material elastic modulus" [ref=e671]
              - generic [ref=e672]:
                - generic [ref=e673]: Shear modulus (Pa, model metadata)
                - textbox "New material shear modulus" [ref=e674]
              - generic [ref=e675]:
                - generic [ref=e676]: Thermal expansion unit
                - combobox "New material thermal expansion unit" [ref=e677]:
                  - option "1/degC, model metadata" [selected]
              - generic [ref=e678]:
                - generic [ref=e679]: Thermal expansion (1/degC, model metadata)
                - textbox "New material thermal expansion" [ref=e680]
              - generic [ref=e681]:
                - generic [ref=e682]: Provenance
                - textbox "New material provenance" [ref=e683]: user_entered_local_preview
              - button "Queue material" [disabled] [ref=e684]:
                - img [ref=e685]
                - text: Queue material
          - region "Create support intent" [ref=e687]:
            - heading "Create support" [level=3] [ref=e688]
            - generic [ref=e689]:
              - generic [ref=e690]:
                - generic [ref=e691]: Support ID
                - textbox "New support ID" [ref=e692]: support:S-1
              - generic [ref=e693]:
                - generic [ref=e694]: Label
                - textbox "New support label" [ref=e695]: Support S-1
              - generic [ref=e696]:
                - generic [ref=e697]: Node
                - combobox "New support node" [ref=e698]:
                  - option "Pump nozzle (node:N-100)"
                  - option "Low point elbow (node:N-110)"
                  - option "Riser elbow (node:N-120)"
                  - option "Rack turn (node:N-130)"
                  - option "Terminal tie-in (node:N-140)" [selected]
              - generic "New support restraints" [ref=e699]:
                - generic [ref=e700]:
                  - checkbox "UX" [checked] [ref=e701]
                  - generic [ref=e702]: UX
                - generic [ref=e703]:
                  - checkbox "UY" [checked] [ref=e704]
                  - generic [ref=e705]: UY
                - generic [ref=e706]:
                  - checkbox "UZ" [checked] [ref=e707]
                  - generic [ref=e708]: UZ
                - generic [ref=e709]:
                  - checkbox "RX" [ref=e710]
                  - generic [ref=e711]: RX
                - generic [ref=e712]:
                  - checkbox "RY" [ref=e713]
                  - generic [ref=e714]: RY
                - generic [ref=e715]:
                  - checkbox "RZ" [ref=e716]
                  - generic [ref=e717]: RZ
              - generic [ref=e718]:
                - generic [ref=e719]: Linear stiffness unit
                - combobox "New support linear stiffness unit" [ref=e720]:
                  - option "N/m, model metadata" [selected]
              - generic [ref=e721]:
                - generic [ref=e722]: Linear stiffness (N/m, model metadata)
                - textbox "New support linear stiffness" [ref=e723]
              - generic [ref=e724]:
                - generic [ref=e725]: Provenance
                - textbox "New support provenance" [ref=e726]: user_entered_local_preview
              - button "Queue support" [ref=e727] [cursor=pointer]:
                - img [ref=e728]
                - text: Queue support
            - article [ref=e730]:
              - generic [ref=e731]:
                - generic [ref=e732]:
                  - generic [ref=e733]: Operation
                  - strong [ref=e734]: op:create-support-support:S-1; create; proposed
                - generic [ref=e735]:
                  - generic [ref=e736]: Target
                  - strong [ref=e737]: Support; support:S-1
                - generic [ref=e738]:
                  - generic [ref=e739]: Change
                  - strong [ref=e740]: "create_support; Explicit support; supports; before=not_present; after={\"id\":\"support:S-1\",\"label\":\"Support S-1\",\"node\":\"node:N-140\",\"restraints\":[\"UX\",\"UY\",\"UZ\"],\"provenance\":\"user_entered_local_preview\"}"
                - generic [ref=e741]:
                  - generic [ref=e742]: Unit basis
                  - strong [ref=e743]: dimensionless; unit=none; explicit user-entered support node and restraint tokens
                - generic [ref=e744]:
                  - generic [ref=e745]: Validation
                  - strong [ref=e746]: not_run; not_run; not_required_dimensionless; not_generated; not_applied
                - generic [ref=e747]:
                  - generic [ref=e748]: Audit boundary
                  - strong [ref=e749]: Routed through the structured operations only; no direct model mutation; requires your acceptance; does not change the accepted model until applied
                - generic [ref=e750]:
                  - generic [ref=e751]: Professional boundary
                  - strong [ref=e752]: Requires human review; no compliance claim; no approval claim
                - generic [ref=e753]:
                  - generic [ref=e754]: Rationale
                  - strong [ref=e755]: explicit user-entered support for project:invented-loop-01; requires service validation before durable model change.
          - region "Create component intent" [ref=e756]:
            - heading "Create component" [level=3] [ref=e757]
            - generic [ref=e758]:
              - generic [ref=e759]:
                - generic [ref=e760]: Component ID
                - textbox "New component ID" [ref=e761]: component:C-1
              - generic [ref=e762]:
                - generic [ref=e763]: Label
                - textbox "New component label" [ref=e764]: Bend C-1
              - generic [ref=e765]:
                - generic [ref=e766]: Kind
                - combobox "New component kind" [ref=e767]:
                  - option "bend" [selected]
                  - option "tee"
                  - option "reducer"
                  - option "valve"
                  - option "flange"
                  - option "expansion_joint"
              - generic [ref=e768]:
                - generic [ref=e769]: Node
                - combobox "New component node" [ref=e770]:
                  - option "Pump nozzle (node:N-100)"
                  - option "Low point elbow (node:N-110)"
                  - option "Riser elbow (node:N-120)"
                  - option "Rack turn (node:N-130)"
                  - option "Terminal tie-in (node:N-140)" [selected]
              - generic [ref=e771]:
                - generic [ref=e772]: Realized pipe
                - combobox "New component realized pipe" [ref=e773]:
                  - option "Select connected pipe"
                  - option "Tie-in rise (pipe:P-130)" [selected]
              - generic [ref=e774]:
                - generic [ref=e775]: Radius
                - textbox "New component radius" [ref=e776]
              - generic [ref=e777]:
                - generic [ref=e778]: Radius unit
                - combobox "New component radius unit" [ref=e779]:
                  - option "m, model metadata" [selected]
              - generic [ref=e780]:
                - generic [ref=e781]: Angle
                - textbox "New component angle" [ref=e782]
              - generic [ref=e783]:
                - generic [ref=e784]: Angle unit
                - combobox "New component angle unit" [ref=e785]:
                  - option "rad, model metadata" [selected]
              - generic [ref=e786]:
                - generic [ref=e787]: Plane orientation
                - textbox "New component plane orientation" [ref=e788]
              - generic [ref=e789]:
                - generic [ref=e790]: Geometry source
                - textbox "New component geometry source" [ref=e791]: user_entered_component_form
              - generic [ref=e792]:
                - generic [ref=e793]: Provenance
                - textbox "New component provenance" [ref=e794]: user_entered_local_preview
              - button "Queue bend" [disabled] [ref=e795]:
                - img [ref=e796]
                - text: Queue bend
          - region "Delete node intent" [ref=e798]:
            - heading "Delete node" [level=3] [ref=e799]
            - button "Queue delete node" [ref=e801] [cursor=pointer]:
              - img [ref=e802]
              - text: Queue delete node
            - article [ref=e805]:
              - generic [ref=e806]:
                - generic [ref=e807]:
                  - generic [ref=e808]: Operation
                  - strong [ref=e809]: op:delete-node-node:N-140; delete; proposed
                - generic [ref=e810]:
                  - generic [ref=e811]: Target
                  - strong [ref=e812]: Node; node:N-140
                - generic [ref=e813]:
                  - generic [ref=e814]: Change
                  - strong [ref=e815]: delete_node; Explicit node deletion; nodes; before=Terminal tie-in; x=7.6; y=2.4; z=2.2; after=not_present
                - generic [ref=e816]:
                  - generic [ref=e817]: Unit basis
                  - strong [ref=e818]: dimensionless; unit=none; explicit user-entered node deletion; endpoint and load reference integrity required
                - generic [ref=e819]:
                  - generic [ref=e820]: Validation
                  - strong [ref=e821]: not_run; not_run; not_required_dimensionless; not_generated; not_applied
                - generic [ref=e822]:
                  - generic [ref=e823]: Audit boundary
                  - strong [ref=e824]: Routed through the structured operations only; no direct model mutation; requires your acceptance; does not change the accepted model until applied
                - generic [ref=e825]:
                  - generic [ref=e826]: Professional boundary
                  - strong [ref=e827]: Requires human review; no compliance claim; no approval claim
                - generic [ref=e828]:
                  - generic [ref=e829]: Rationale
                  - strong [ref=e830]: explicit user-entered node deletion for project:invented-loop-01; requires reference validation before durable model change.
    - region "Workspace sections" [ref=e831]:
      - generic [ref=e832]:
        - heading "Results" [level=2] [ref=e833]
        - button "Close panel" [ref=e834] [cursor=pointer]
      - region "Results section" [ref=e836]:
        - region "Results" [ref=e837]:
          - generic [ref=e838]: Results
          - region "Result filtering" [ref=e839]:
            - generic [ref=e840]:
              - generic [ref=e841]: Source result units
              - strong [ref=e842]: MPa, N, N*m, mm, rad, boolean, count, m, mode_code, N*m/rad, N/m, state_code · 830 rows · entered units preserved; displayed quantities follow the display preference where available
            - group "Result family filter" [ref=e843]:
              - button "All 830" [pressed] [ref=e844] [cursor=pointer]:
                - generic [ref=e845]: All
                - strong [ref=e846]: "830"
              - button "Displacement 111" [ref=e847] [cursor=pointer]:
                - generic [ref=e848]: Displacement
                - strong [ref=e849]: "111"
              - button "Reaction 33" [ref=e850] [cursor=pointer]:
                - generic [ref=e851]: Reaction
                - strong [ref=e852]: "33"
              - button "Force 182" [ref=e853] [cursor=pointer]:
                - generic [ref=e854]: Force
                - strong [ref=e855]: "182"
              - button "Moment 182" [ref=e856] [cursor=pointer]:
                - generic [ref=e857]: Moment
                - strong [ref=e858]: "182"
              - button "Stress 290" [ref=e859] [cursor=pointer]:
                - generic [ref=e860]: Stress
                - strong [ref=e861]: "290"
              - button "Other 32" [ref=e862] [cursor=pointer]:
                - generic [ref=e863]: Other
                - strong [ref=e864]: "32"
            - generic [ref=e865]:
              - generic [ref=e866]:
                - img [ref=e867]
                - generic [ref=e870]: Filter results
                - searchbox "Filter result rows" [active] [ref=e871]: pipe-P-120
              - generic [ref=e872]: 170 of 830 results match filter
              - button "Clear result filter" [ref=e873] [cursor=pointer]:
                - img [ref=e874]
            - generic "Result page controls" [ref=e877]:
              - generic [ref=e878]:
                - generic [ref=e879]: Rows
                - combobox "Rows per result page" [ref=e880]:
                  - option "50" [selected]
                  - option "100"
                  - option "200"
              - generic [ref=e881]: Showing 1 to 50 of 170 matching results; page 1 of 4
              - generic [ref=e882]:
                - button "Previous result page" [disabled] [ref=e883]:
                  - img [ref=e884]
                  - text: Previous
                - button "Next result page" [ref=e886] [cursor=pointer]:
                  - text: Next
                  - img [ref=e887]
          - region "Result detail" [ref=e889]:
            - heading "Result Detail" [level=3] [ref=e890]
            - paragraph [ref=e891]: Select a computed result row to inspect recovery context and review boundary.
          - region "Governing ratio state" [ref=e892]:
            - heading "Governing Ratio" [level=3] [ref=e893]
            - paragraph [ref=e894]: unavailable; no user-rule ratio rows are present in this result envelope; mechanics results remain reviewable but no governing rule-check ratio is inferred
            - generic [ref=e895]: This view reports supplied ratio rows only; it does not synthesize allowables or code criteria. Acceptance and professional judgment remain with the responsible engineer.
          - generic [ref=e896]:
            - region "Force results" [ref=e897]:
              - heading "Force" [level=3] [ref=e898]
              - table [ref=e899]:
                - rowgroup [ref=e900]:
                  - row "ID Entity Location Value Entered source" [ref=e901]:
                    - columnheader "ID" [ref=e902]
                    - columnheader "Entity" [ref=e903]
                    - columnheader "Location" [ref=e904]
                    - columnheader "Value" [ref=e905]
                    - columnheader "Entered source" [ref=e906]
                - rowgroup [ref=e907]:
                  - 'row "result:force:pipe-P-120:axial pipe:P-120 end_i 0 N Entered: 0 N" [ref=e908] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:axial" [ref=e909]
                    - cell "pipe:P-120" [ref=e910]
                    - cell "end_i" [ref=e911]
                    - cell "0 N" [ref=e912]:
                      - generic [ref=e913]: 0 N
                    - 'cell "Entered: 0 N" [ref=e914]':
                      - generic [ref=e915]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:axial:end-j pipe:P-120 end_j 0 N Entered: 0 N" [ref=e916] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:axial:end-j" [ref=e917]
                    - cell "pipe:P-120" [ref=e918]
                    - cell "end_j" [ref=e919]
                    - cell "0 N" [ref=e920]:
                      - generic [ref=e921]: 0 N
                    - 'cell "Entered: 0 N" [ref=e922]':
                      - generic [ref=e923]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:shear-y pipe:P-120 end_i 272.514977 N Entered: 272.514977 N" [ref=e924] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:shear-y" [ref=e925]
                    - cell "pipe:P-120" [ref=e926]
                    - cell "end_i" [ref=e927]
                    - cell "272.514977 N" [ref=e928]:
                      - generic [ref=e929]: 272.514977 N
                    - 'cell "Entered: 272.514977 N" [ref=e930]':
                      - generic [ref=e931]: "Entered: 272.514977 N"
                  - 'row "result:force:pipe-P-120:shear-y:end-j pipe:P-120 end_j -272.514977 N Entered: -272.514977 N" [ref=e932] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:shear-y:end-j" [ref=e933]
                    - cell "pipe:P-120" [ref=e934]
                    - cell "end_j" [ref=e935]
                    - cell "-272.514977 N" [ref=e936]:
                      - generic [ref=e937]: "-272.514977 N"
                    - 'cell "Entered: -272.514977 N" [ref=e938]':
                      - generic [ref=e939]: "Entered: -272.514977 N"
                  - 'row "result:force:pipe-P-120:shear-z pipe:P-120 end_i 0 N Entered: 0 N" [ref=e940] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:shear-z" [ref=e941]
                    - cell "pipe:P-120" [ref=e942]
                    - cell "end_i" [ref=e943]
                    - cell "0 N" [ref=e944]:
                      - generic [ref=e945]: 0 N
                    - 'cell "Entered: 0 N" [ref=e946]':
                      - generic [ref=e947]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:shear-z:end-j pipe:P-120 end_j 0 N Entered: 0 N" [ref=e948] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:shear-z:end-j" [ref=e949]
                    - cell "pipe:P-120" [ref=e950]
                    - cell "end_j" [ref=e951]
                    - cell "0 N" [ref=e952]:
                      - generic [ref=e953]: 0 N
                    - 'cell "Entered: 0 N" [ref=e954]':
                      - generic [ref=e955]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:quarter-1:axial pipe:P-120 quarter_1 0 N Entered: 0 N" [ref=e956] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:quarter-1:axial" [ref=e957]
                    - cell "pipe:P-120" [ref=e958]
                    - cell "quarter_1" [ref=e959]
                    - cell "0 N" [ref=e960]:
                      - generic [ref=e961]: 0 N
                    - 'cell "Entered: 0 N" [ref=e962]':
                      - generic [ref=e963]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:quarter-1:shear-y pipe:P-120 quarter_1 136.257488 N Entered: 136.257488 N" [ref=e964] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:quarter-1:shear-y" [ref=e965]
                    - cell "pipe:P-120" [ref=e966]
                    - cell "quarter_1" [ref=e967]
                    - cell "136.257488 N" [ref=e968]:
                      - generic [ref=e969]: 136.257488 N
                    - 'cell "Entered: 136.257488 N" [ref=e970]':
                      - generic [ref=e971]: "Entered: 136.257488 N"
                  - 'row "result:force:pipe-P-120:quarter-1:shear-z pipe:P-120 quarter_1 0 N Entered: 0 N" [ref=e972] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:quarter-1:shear-z" [ref=e973]
                    - cell "pipe:P-120" [ref=e974]
                    - cell "quarter_1" [ref=e975]
                    - cell "0 N" [ref=e976]:
                      - generic [ref=e977]: 0 N
                    - 'cell "Entered: 0 N" [ref=e978]':
                      - generic [ref=e979]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:midspan:axial pipe:P-120 midspan 0 N Entered: 0 N" [ref=e980] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:midspan:axial" [ref=e981]
                    - cell "pipe:P-120" [ref=e982]
                    - cell "midspan" [ref=e983]
                    - cell "0 N" [ref=e984]:
                      - generic [ref=e985]: 0 N
                    - 'cell "Entered: 0 N" [ref=e986]':
                      - generic [ref=e987]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:midspan:shear-y pipe:P-120 midspan 0 N Entered: 0 N" [ref=e988] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:midspan:shear-y" [ref=e989]
                    - cell "pipe:P-120" [ref=e990]
                    - cell "midspan" [ref=e991]
                    - cell "0 N" [ref=e992]:
                      - generic [ref=e993]: 0 N
                    - 'cell "Entered: 0 N" [ref=e994]':
                      - generic [ref=e995]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:midspan:shear-z pipe:P-120 midspan 0 N Entered: 0 N" [ref=e996] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:midspan:shear-z" [ref=e997]
                    - cell "pipe:P-120" [ref=e998]
                    - cell "midspan" [ref=e999]
                    - cell "0 N" [ref=e1000]:
                      - generic [ref=e1001]: 0 N
                    - 'cell "Entered: 0 N" [ref=e1002]':
                      - generic [ref=e1003]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:quarter-3:axial pipe:P-120 quarter_3 0 N Entered: 0 N" [ref=e1004] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:quarter-3:axial" [ref=e1005]
                    - cell "pipe:P-120" [ref=e1006]
                    - cell "quarter_3" [ref=e1007]
                    - cell "0 N" [ref=e1008]:
                      - generic [ref=e1009]: 0 N
                    - 'cell "Entered: 0 N" [ref=e1010]':
                      - generic [ref=e1011]: "Entered: 0 N"
                  - 'row "result:force:pipe-P-120:quarter-3:shear-y pipe:P-120 quarter_3 -136.257488 N Entered: -136.257488 N" [ref=e1012] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:quarter-3:shear-y" [ref=e1013]
                    - cell "pipe:P-120" [ref=e1014]
                    - cell "quarter_3" [ref=e1015]
                    - cell "-136.257488 N" [ref=e1016]:
                      - generic [ref=e1017]: "-136.257488 N"
                    - 'cell "Entered: -136.257488 N" [ref=e1018]':
                      - generic [ref=e1019]: "Entered: -136.257488 N"
                  - 'row "result:force:pipe-P-120:quarter-3:shear-z pipe:P-120 quarter_3 0 N Entered: 0 N" [ref=e1020] [cursor=pointer]':
                    - cell "result:force:pipe-P-120:quarter-3:shear-z" [ref=e1021]
                    - cell "pipe:P-120" [ref=e1022]
                    - cell "quarter_3" [ref=e1023]
                    - cell "0 N" [ref=e1024]:
                      - generic [ref=e1025]: 0 N
                    - 'cell "Entered: 0 N" [ref=e1026]':
                      - generic [ref=e1027]: "Entered: 0 N"
            - region "Moment results" [ref=e1028]:
              - heading "Moment" [level=3] [ref=e1029]
              - table [ref=e1030]:
                - rowgroup [ref=e1031]:
                  - row "ID Entity Location Value Entered source" [ref=e1032]:
                    - columnheader "ID" [ref=e1033]
                    - columnheader "Entity" [ref=e1034]
                    - columnheader "Location" [ref=e1035]
                    - columnheader "Value" [ref=e1036]
                    - columnheader "Entered source" [ref=e1037]
                - rowgroup [ref=e1038]:
                  - 'row "result:moment:pipe-P-120:torsion pipe:P-120 end_i 111.679634 N*m Entered: 111.679634 N*m" [ref=e1039] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:torsion" [ref=e1040]
                    - cell "pipe:P-120" [ref=e1041]
                    - cell "end_i" [ref=e1042]
                    - cell "111.679634 N*m" [ref=e1043]:
                      - generic [ref=e1044]: 111.679634 N*m
                    - 'cell "Entered: 111.679634 N*m" [ref=e1045]':
                      - generic [ref=e1046]: "Entered: 111.679634 N*m"
                  - 'row "result:moment:pipe-P-120:torsion:end-j pipe:P-120 end_j -111.679634 N*m Entered: -111.679634 N*m" [ref=e1047] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:torsion:end-j" [ref=e1048]
                    - cell "pipe:P-120" [ref=e1049]
                    - cell "end_j" [ref=e1050]
                    - cell "-111.679634 N*m" [ref=e1051]:
                      - generic [ref=e1052]: "-111.679634 N*m"
                    - 'cell "Entered: -111.679634 N*m" [ref=e1053]':
                      - generic [ref=e1054]: "Entered: -111.679634 N*m"
                  - 'row "result:moment:pipe-P-120:bending-y pipe:P-120 end_i 0 N*m Entered: 0 N*m" [ref=e1055] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:bending-y" [ref=e1056]
                    - cell "pipe:P-120" [ref=e1057]
                    - cell "end_i" [ref=e1058]
                    - cell "0 N*m" [ref=e1059]:
                      - generic [ref=e1060]: 0 N*m
                    - 'cell "Entered: 0 N*m" [ref=e1061]':
                      - generic [ref=e1062]: "Entered: 0 N*m"
                  - 'row "result:moment:pipe-P-120:bending-y:end-j pipe:P-120 end_j 0 N*m Entered: 0 N*m" [ref=e1063] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:bending-y:end-j" [ref=e1064]
                    - cell "pipe:P-120" [ref=e1065]
                    - cell "end_j" [ref=e1066]
                    - cell "0 N*m" [ref=e1067]:
                      - generic [ref=e1068]: 0 N*m
                    - 'cell "Entered: 0 N*m" [ref=e1069]':
                      - generic [ref=e1070]: "Entered: 0 N*m"
                  - 'row "result:moment:pipe-P-120:bending-z pipe:P-120 end_i 370.59479 N*m Entered: 370.59479 N*m" [ref=e1071] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:bending-z" [ref=e1072]
                    - cell "pipe:P-120" [ref=e1073]
                    - cell "end_i" [ref=e1074]
                    - cell "370.59479 N*m" [ref=e1075]:
                      - generic [ref=e1076]: 370.59479 N*m
                    - 'cell "Entered: 370.59479 N*m" [ref=e1077]':
                      - generic [ref=e1078]: "Entered: 370.59479 N*m"
                  - 'row "result:moment:pipe-P-120:bending-z:end-j pipe:P-120 end_j 828.471107 N*m Entered: 828.471107 N*m" [ref=e1079] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:bending-z:end-j" [ref=e1080]
                    - cell "pipe:P-120" [ref=e1081]
                    - cell "end_j" [ref=e1082]
                    - cell "828.471107 N*m" [ref=e1083]:
                      - generic [ref=e1084]: 828.471107 N*m
                    - 'cell "Entered: 828.471107 N*m" [ref=e1085]':
                      - generic [ref=e1086]: "Entered: 828.471107 N*m"
                  - 'row "result:moment:pipe-P-120:quarter-1:torsion pipe:P-120 quarter_1 55.839817 N*m Entered: 55.839817 N*m" [ref=e1087] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:quarter-1:torsion" [ref=e1088]
                    - cell "pipe:P-120" [ref=e1089]
                    - cell "quarter_1" [ref=e1090]
                    - cell "55.839817 N*m" [ref=e1091]:
                      - generic [ref=e1092]: 55.839817 N*m
                    - 'cell "Entered: 55.839817 N*m" [ref=e1093]':
                      - generic [ref=e1094]: "Entered: 55.839817 N*m"
                  - 'row "result:moment:pipe-P-120:quarter-1:bending-y pipe:P-120 quarter_1 0 N*m Entered: 0 N*m" [ref=e1095] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:quarter-1:bending-y" [ref=e1096]
                    - cell "pipe:P-120" [ref=e1097]
                    - cell "quarter_1" [ref=e1098]
                    - cell "0 N*m" [ref=e1099]:
                      - generic [ref=e1100]: 0 N*m
                    - 'cell "Entered: 0 N*m" [ref=e1101]':
                      - generic [ref=e1102]: "Entered: 0 N*m"
                  - 'row "result:moment:pipe-P-120:quarter-1:bending-z pipe:P-120 quarter_1 485.063869 N*m Entered: 485.063869 N*m" [ref=e1103] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:quarter-1:bending-z" [ref=e1104]
                    - cell "pipe:P-120" [ref=e1105]
                    - cell "quarter_1" [ref=e1106]
                    - cell "485.063869 N*m" [ref=e1107]:
                      - generic [ref=e1108]: 485.063869 N*m
                    - 'cell "Entered: 485.063869 N*m" [ref=e1109]':
                      - generic [ref=e1110]: "Entered: 485.063869 N*m"
                  - 'row "result:moment:pipe-P-120:midspan:torsion pipe:P-120 midspan 0 N*m Entered: 0 N*m" [ref=e1111] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:midspan:torsion" [ref=e1112]
                    - cell "pipe:P-120" [ref=e1113]
                    - cell "midspan" [ref=e1114]
                    - cell "0 N*m" [ref=e1115]:
                      - generic [ref=e1116]: 0 N*m
                    - 'cell "Entered: 0 N*m" [ref=e1117]':
                      - generic [ref=e1118]: "Entered: 0 N*m"
                  - 'row "result:moment:pipe-P-120:midspan:bending-y pipe:P-120 midspan 0 N*m Entered: 0 N*m" [ref=e1119] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:midspan:bending-y" [ref=e1120]
                    - cell "pipe:P-120" [ref=e1121]
                    - cell "midspan" [ref=e1122]
                    - cell "0 N*m" [ref=e1123]:
                      - generic [ref=e1124]: 0 N*m
                    - 'cell "Entered: 0 N*m" [ref=e1125]':
                      - generic [ref=e1126]: "Entered: 0 N*m"
                  - 'row "result:moment:pipe-P-120:midspan:bending-z pipe:P-120 midspan 599.532948 N*m Entered: 599.532948 N*m" [ref=e1127] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:midspan:bending-z" [ref=e1128]
                    - cell "pipe:P-120" [ref=e1129]
                    - cell "midspan" [ref=e1130]
                    - cell "599.532948 N*m" [ref=e1131]:
                      - generic [ref=e1132]: 599.532948 N*m
                    - 'cell "Entered: 599.532948 N*m" [ref=e1133]':
                      - generic [ref=e1134]: "Entered: 599.532948 N*m"
                  - 'row "result:moment:pipe-P-120:quarter-3:torsion pipe:P-120 quarter_3 -55.839817 N*m Entered: -55.839817 N*m" [ref=e1135] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:quarter-3:torsion" [ref=e1136]
                    - cell "pipe:P-120" [ref=e1137]
                    - cell "quarter_3" [ref=e1138]
                    - cell "-55.839817 N*m" [ref=e1139]:
                      - generic [ref=e1140]: "-55.839817 N*m"
                    - 'cell "Entered: -55.839817 N*m" [ref=e1141]':
                      - generic [ref=e1142]: "Entered: -55.839817 N*m"
                  - 'row "result:moment:pipe-P-120:quarter-3:bending-y pipe:P-120 quarter_3 0 N*m Entered: 0 N*m" [ref=e1143] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:quarter-3:bending-y" [ref=e1144]
                    - cell "pipe:P-120" [ref=e1145]
                    - cell "quarter_3" [ref=e1146]
                    - cell "0 N*m" [ref=e1147]:
                      - generic [ref=e1148]: 0 N*m
                    - 'cell "Entered: 0 N*m" [ref=e1149]':
                      - generic [ref=e1150]: "Entered: 0 N*m"
                  - 'row "result:moment:pipe-P-120:quarter-3:bending-z pipe:P-120 quarter_3 714.002028 N*m Entered: 714.002028 N*m" [ref=e1151] [cursor=pointer]':
                    - cell "result:moment:pipe-P-120:quarter-3:bending-z" [ref=e1152]
                    - cell "pipe:P-120" [ref=e1153]
                    - cell "quarter_3" [ref=e1154]
                    - cell "714.002028 N*m" [ref=e1155]:
                      - generic [ref=e1156]: 714.002028 N*m
                    - 'cell "Entered: 714.002028 N*m" [ref=e1157]':
                      - generic [ref=e1158]: "Entered: 714.002028 N*m"
            - region "Stress results" [ref=e1159]:
              - heading "Stress" [level=3] [ref=e1160]
              - table [ref=e1161]:
                - rowgroup [ref=e1162]:
                  - row "ID Entity Location Value Entered source" [ref=e1163]:
                    - columnheader "ID" [ref=e1164]
                    - columnheader "Entity" [ref=e1165]
                    - columnheader "Location" [ref=e1166]
                    - columnheader "Value" [ref=e1167]
                    - columnheader "Entered source" [ref=e1168]
                - rowgroup [ref=e1169]:
                  - 'row "result:stress:pipe-P-120:end-i:axial-normal pipe:P-120 end_i 0 MPa Entered: 0 MPa" [ref=e1170] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-i:axial-normal" [ref=e1171]
                    - cell "pipe:P-120" [ref=e1172]
                    - cell "end_i" [ref=e1173]
                    - cell "0 MPa" [ref=e1174]:
                      - generic [ref=e1175]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1176]':
                      - generic [ref=e1177]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:end-i:bending-normal-y pipe:P-120 end_i 0 MPa Entered: 0 MPa" [ref=e1178] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-i:bending-normal-y" [ref=e1179]
                    - cell "pipe:P-120" [ref=e1180]
                    - cell "end_i" [ref=e1181]
                    - cell "0 MPa" [ref=e1182]:
                      - generic [ref=e1183]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1184]':
                      - generic [ref=e1185]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:end-i:bending-normal-z pipe:P-120 end_i 2.708463 MPa Entered: 2.708463 MPa" [ref=e1186] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-i:bending-normal-z" [ref=e1187]
                    - cell "pipe:P-120" [ref=e1188]
                    - cell "end_i" [ref=e1189]
                    - cell "2.708463 MPa" [ref=e1190]:
                      - generic [ref=e1191]: 2.708463 MPa
                    - 'cell "Entered: 2.708463 MPa" [ref=e1192]':
                      - generic [ref=e1193]: "Entered: 2.708463 MPa"
                  - 'row "result:stress:pipe-P-120:end-i:torsional-shear pipe:P-120 end_i 0.408101 MPa Entered: 0.408101 MPa" [ref=e1194] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-i:torsional-shear" [ref=e1195]
                    - cell "pipe:P-120" [ref=e1196]
                    - cell "end_i" [ref=e1197]
                    - cell "0.408101 MPa" [ref=e1198]:
                      - generic [ref=e1199]: 0.408101 MPa
                    - 'cell "Entered: 0.408101 MPa" [ref=e1200]':
                      - generic [ref=e1201]: "Entered: 0.408101 MPa"
                  - 'row "result:stress:pipe-P-120:end-i:pressure-hoop pipe:P-120 end_i 13.8 MPa Entered: 13.8 MPa" [ref=e1202] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-i:pressure-hoop" [ref=e1203]
                    - cell "pipe:P-120" [ref=e1204]
                    - cell "end_i" [ref=e1205]
                    - cell "13.8 MPa" [ref=e1206]:
                      - generic [ref=e1207]: 13.8 MPa
                    - 'cell "Entered: 13.8 MPa" [ref=e1208]':
                      - generic [ref=e1209]: "Entered: 13.8 MPa"
                  - 'row "result:stress:pipe-P-120:end-j:axial-normal pipe:P-120 end_j 0 MPa Entered: 0 MPa" [ref=e1210] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-j:axial-normal" [ref=e1211]
                    - cell "pipe:P-120" [ref=e1212]
                    - cell "end_j" [ref=e1213]
                    - cell "0 MPa" [ref=e1214]:
                      - generic [ref=e1215]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1216]':
                      - generic [ref=e1217]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:end-j:bending-normal-y pipe:P-120 end_j 0 MPa Entered: 0 MPa" [ref=e1218] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-j:bending-normal-y" [ref=e1219]
                    - cell "pipe:P-120" [ref=e1220]
                    - cell "end_j" [ref=e1221]
                    - cell "0 MPa" [ref=e1222]:
                      - generic [ref=e1223]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1224]':
                      - generic [ref=e1225]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:end-j:bending-normal-z pipe:P-120 end_j 6.054815 MPa Entered: 6.054815 MPa" [ref=e1226] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-j:bending-normal-z" [ref=e1227]
                    - cell "pipe:P-120" [ref=e1228]
                    - cell "end_j" [ref=e1229]
                    - cell "6.054815 MPa" [ref=e1230]:
                      - generic [ref=e1231]: 6.054815 MPa
                    - 'cell "Entered: 6.054815 MPa" [ref=e1232]':
                      - generic [ref=e1233]: "Entered: 6.054815 MPa"
                  - 'row "result:stress:pipe-P-120:end-j:torsional-shear pipe:P-120 end_j -0.408101 MPa Entered: -0.408101 MPa" [ref=e1234] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-j:torsional-shear" [ref=e1235]
                    - cell "pipe:P-120" [ref=e1236]
                    - cell "end_j" [ref=e1237]
                    - cell "-0.408101 MPa" [ref=e1238]:
                      - generic [ref=e1239]: "-0.408101 MPa"
                    - 'cell "Entered: -0.408101 MPa" [ref=e1240]':
                      - generic [ref=e1241]: "Entered: -0.408101 MPa"
                  - 'row "result:stress:pipe-P-120:end-j:pressure-hoop pipe:P-120 end_j 13.8 MPa Entered: 13.8 MPa" [ref=e1242] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:end-j:pressure-hoop" [ref=e1243]
                    - cell "pipe:P-120" [ref=e1244]
                    - cell "end_j" [ref=e1245]
                    - cell "13.8 MPa" [ref=e1246]:
                      - generic [ref=e1247]: 13.8 MPa
                    - 'cell "Entered: 13.8 MPa" [ref=e1248]':
                      - generic [ref=e1249]: "Entered: 13.8 MPa"
                  - 'row "result:stress:pipe-P-120:quarter-1:axial-normal pipe:P-120 quarter_1 0 MPa Entered: 0 MPa" [ref=e1250] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:quarter-1:axial-normal" [ref=e1251]
                    - cell "pipe:P-120" [ref=e1252]
                    - cell "quarter_1" [ref=e1253]
                    - cell "0 MPa" [ref=e1254]:
                      - generic [ref=e1255]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1256]':
                      - generic [ref=e1257]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:quarter-1:bending-normal-y pipe:P-120 quarter_1 0 MPa Entered: 0 MPa" [ref=e1258] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:quarter-1:bending-normal-y" [ref=e1259]
                    - cell "pipe:P-120" [ref=e1260]
                    - cell "quarter_1" [ref=e1261]
                    - cell "0 MPa" [ref=e1262]:
                      - generic [ref=e1263]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1264]':
                      - generic [ref=e1265]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:quarter-1:bending-normal-z pipe:P-120 quarter_1 3.545051 MPa Entered: 3.545051 MPa" [ref=e1266] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:quarter-1:bending-normal-z" [ref=e1267]
                    - cell "pipe:P-120" [ref=e1268]
                    - cell "quarter_1" [ref=e1269]
                    - cell "3.545051 MPa" [ref=e1270]:
                      - generic [ref=e1271]: 3.545051 MPa
                    - 'cell "Entered: 3.545051 MPa" [ref=e1272]':
                      - generic [ref=e1273]: "Entered: 3.545051 MPa"
                  - 'row "result:stress:pipe-P-120:quarter-1:torsional-shear pipe:P-120 quarter_1 0.20405 MPa Entered: 0.20405 MPa" [ref=e1274] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:quarter-1:torsional-shear" [ref=e1275]
                    - cell "pipe:P-120" [ref=e1276]
                    - cell "quarter_1" [ref=e1277]
                    - cell "0.20405 MPa" [ref=e1278]:
                      - generic [ref=e1279]: 0.20405 MPa
                    - 'cell "Entered: 0.20405 MPa" [ref=e1280]':
                      - generic [ref=e1281]: "Entered: 0.20405 MPa"
                  - 'row "result:stress:pipe-P-120:quarter-1:pressure-hoop pipe:P-120 quarter_1 13.8 MPa Entered: 13.8 MPa" [ref=e1282] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:quarter-1:pressure-hoop" [ref=e1283]
                    - cell "pipe:P-120" [ref=e1284]
                    - cell "quarter_1" [ref=e1285]
                    - cell "13.8 MPa" [ref=e1286]:
                      - generic [ref=e1287]: 13.8 MPa
                    - 'cell "Entered: 13.8 MPa" [ref=e1288]':
                      - generic [ref=e1289]: "Entered: 13.8 MPa"
                  - 'row "result:stress:pipe-P-120:midspan:axial-normal pipe:P-120 midspan 0 MPa Entered: 0 MPa" [ref=e1290] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:midspan:axial-normal" [ref=e1291]
                    - cell "pipe:P-120" [ref=e1292]
                    - cell "midspan" [ref=e1293]
                    - cell "0 MPa" [ref=e1294]:
                      - generic [ref=e1295]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1296]':
                      - generic [ref=e1297]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:midspan:bending-normal-y pipe:P-120 midspan 0 MPa Entered: 0 MPa" [ref=e1298] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:midspan:bending-normal-y" [ref=e1299]
                    - cell "pipe:P-120" [ref=e1300]
                    - cell "midspan" [ref=e1301]
                    - cell "0 MPa" [ref=e1302]:
                      - generic [ref=e1303]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1304]':
                      - generic [ref=e1305]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:midspan:bending-normal-z pipe:P-120 midspan 4.381639 MPa Entered: 4.381639 MPa" [ref=e1306] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:midspan:bending-normal-z" [ref=e1307]
                    - cell "pipe:P-120" [ref=e1308]
                    - cell "midspan" [ref=e1309]
                    - cell "4.381639 MPa" [ref=e1310]:
                      - generic [ref=e1311]: 4.381639 MPa
                    - 'cell "Entered: 4.381639 MPa" [ref=e1312]':
                      - generic [ref=e1313]: "Entered: 4.381639 MPa"
                  - 'row "result:stress:pipe-P-120:midspan:torsional-shear pipe:P-120 midspan 0 MPa Entered: 0 MPa" [ref=e1314] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:midspan:torsional-shear" [ref=e1315]
                    - cell "pipe:P-120" [ref=e1316]
                    - cell "midspan" [ref=e1317]
                    - cell "0 MPa" [ref=e1318]:
                      - generic [ref=e1319]: 0 MPa
                    - 'cell "Entered: 0 MPa" [ref=e1320]':
                      - generic [ref=e1321]: "Entered: 0 MPa"
                  - 'row "result:stress:pipe-P-120:midspan:pressure-hoop pipe:P-120 midspan 13.8 MPa Entered: 13.8 MPa" [ref=e1322] [cursor=pointer]':
                    - cell "result:stress:pipe-P-120:midspan:pressure-hoop" [ref=e1323]
                    - cell "pipe:P-120" [ref=e1324]
                    - cell "midspan" [ref=e1325]
                    - cell "13.8 MPa" [ref=e1326]:
                      - generic [ref=e1327]: 13.8 MPa
                    - 'cell "Entered: 13.8 MPa" [ref=e1328]':
                      - generic [ref=e1329]: "Entered: 13.8 MPa"
            - region "Mechanics gap ledger" [ref=e1330]:
              - heading "Mechanics Gap Ledger" [level=3] [ref=e1331]
              - generic [ref=e1332]:
                - article [ref=e1333]:
                  - strong [ref=e1334]: Endpoint-j local force/moment recovery
                  - generic [ref=e1335]: implemented
                  - paragraph [ref=e1336]: End-i and end-j local force/moment preview results are emitted and paired in result detail; arbitrary station sweeps remain deferred.
                - article [ref=e1337]:
                  - strong [ref=e1338]: Endpoint stress component recovery
                  - generic [ref=e1339]: implemented
                  - paragraph [ref=e1340]: End-i and end-j open-mechanics stress component preview results are emitted and paired when stress recovery succeeds.
                - article [ref=e1341]:
                  - strong [ref=e1342]: Intermediate station result recovery
                  - generic [ref=e1343]: implemented
                  - paragraph [ref=e1344]: Fixed station-grid preview force, moment, shear force, and stress rows are emitted from interpolated endpoint resultants; arbitrary station sweeps, exact internal diagrams, and shear stress recovery remain deferred.
                - article [ref=e1345]:
                  - strong [ref=e1346]: Pressure-to-frame load conversion
                  - generic [ref=e1347]: implemented
                  - paragraph [ref=e1348]: Closed-end pressure thrust for straight preview pipes is applied to frame resultants; broader pressure behavior remains deferred.
                - article [ref=e1349]:
                  - strong [ref=e1350]: Thermal behavior
                  - generic [ref=e1351]: implemented
                  - paragraph [ref=e1352]: Uniform axial temperature-change loads for straight preview pipes are implemented with explicit material expansion input; temperature-dependent properties and thermal combinations remain deferred.
                - article [ref=e1353]:
                  - strong [ref=e1354]: Support stiffness completeness
                  - generic [ref=e1355]: deferred
                  - paragraph [ref=e1356]: Linear support restraints are preview inputs; richer stiffness and nonlinear behavior require explicit data.
                - article [ref=e1357]:
                  - strong [ref=e1358]: Load combinations
                  - generic [ref=e1359]: implemented
                  - paragraph [ref=e1360]: Explicit mechanics-basis user load combinations are emitted from matching scalar load-case result rows; code/rule combinations remain deferred/private.
                - article [ref=e1361]:
                  - strong [ref=e1362]: Protected rule/code checks
                  - generic [ref=e1363]: requires private inputs
                  - paragraph [ref=e1364]: Private criteria, allowables, SIF/flexibility tables, and code checks are not bundled publicly.
        - region "Comparison workspace" [ref=e1365]:
          - generic [ref=e1366]:
            - img [ref=e1367]
            - text: Comparison
          - generic [ref=e1372]:
            - generic [ref=e1373]:
              - generic [ref=e1374]: Reference
              - strong [ref=e1375]: load:L-100; 279 rows
            - generic [ref=e1376]:
              - generic [ref=e1377]: Target
              - strong [ref=e1378]: combination:C-OPER-ALT; 261 rows
            - generic [ref=e1379]:
              - generic [ref=e1380]: Mapped
              - strong [ref=e1381]: 261 comparable pairs; 18 reference-only; 0 target-only
            - generic [ref=e1382]:
              - generic [ref=e1383]: Tolerance
              - strong [ref=e1384]: not_tolerance_checked; profile=TBD
            - generic [ref=e1385]:
              - generic [ref=e1386]: Source unit evidence
              - strong [ref=e1387]: units=MPa,N,N*m,mm,rad; matching=equal_explicit_units; conversion=false; tolerance=not_tolerance_checked
            - generic [ref=e1388]:
              - generic [ref=e1389]: Boundary
              - strong [ref=e1390]: review-only comparison; acceptance and professional judgment remain with the responsible engineer
          - paragraph [ref=e1391]: stable result IDs plus explicit source_result_refs from the preview mechanics result envelope
          - generic [ref=e1392]:
            - generic [ref=e1393]:
              - generic [ref=e1394]: Target result
              - generic [ref=e1395]: Entity
              - generic [ref=e1396]: Location
              - generic [ref=e1397]: Delta
            - 'button "result:combination:combination-C-OPER-ALT:pressure-thrust:component-C-150 not tolerance checked component:C-150 expansion_joint_pressure_thrust; pipe:P-130 5400 N Reference: 21600 N Target: 27000 N" [ref=e1399] [cursor=pointer]':
              - generic [ref=e1400]:
                - text: result:combination:combination-C-OPER-ALT:pressure-thrust:component-C-150
                - generic [ref=e1401]: not tolerance checked
              - generic [ref=e1402]: component:C-150
              - generic [ref=e1403]: expansion_joint_pressure_thrust; pipe:P-130
              - generic [ref=e1404]:
                - generic [ref=e1406]: 5400 N
                - generic [ref=e1407]:
                  - text: "Reference:"
                  - generic [ref=e1408]: 21600 N
                - generic [ref=e1409]:
                  - text: "Target:"
                  - generic [ref=e1410]: 27000 N
            - 'button "result:combination:combination-C-OPER-ALT:moment:pipe-P-120:bending-z:end-j not tolerance checked pipe:P-120 bending_moment_z; end_j 207.350325 N*m Reference: 828.471107 N*m Target: 1035.821432 N*m" [ref=e1412] [cursor=pointer]':
              - generic [ref=e1413]:
                - text: result:combination:combination-C-OPER-ALT:moment:pipe-P-120:bending-z:end-j
                - generic [ref=e1414]: not tolerance checked
              - generic [ref=e1415]: pipe:P-120
              - generic [ref=e1416]: bending_moment_z; end_j
              - generic [ref=e1417]:
                - generic [ref=e1419]: 207.350325 N*m
                - generic [ref=e1420]:
                  - text: "Reference:"
                  - generic [ref=e1421]: 828.471107 N*m
                - generic [ref=e1422]:
                  - text: "Target:"
                  - generic [ref=e1423]: 1035.821432 N*m
            - 'button "result:combination:combination-C-OPER-ALT:reaction:support-S-120 not tolerance checked support:S-120 reaction_resultant; summary 192.51394500000004 N Reference: 774.961882 N Target: 967.475827 N" [ref=e1425] [cursor=pointer]':
              - generic [ref=e1426]:
                - text: result:combination:combination-C-OPER-ALT:reaction:support-S-120
                - generic [ref=e1427]: not tolerance checked
              - generic [ref=e1428]: support:S-120
              - generic [ref=e1429]: reaction_resultant; summary
              - generic [ref=e1430]:
                - generic [ref=e1432]: 192.51394500000004 N
                - generic [ref=e1433]:
                  - text: "Reference:"
                  - generic [ref=e1434]: 774.961882 N
                - generic [ref=e1435]:
                  - text: "Target:"
                  - generic [ref=e1436]: 967.475827 N
            - 'button "result:combination:combination-C-OPER-ALT:moment:pipe-P-120:quarter-3:bending-z not tolerance checked pipe:P-120 bending_moment_z; quarter_3 178.52549999999997 N*m Reference: 714.002028 N*m Target: 892.527528 N*m" [ref=e1438] [cursor=pointer]':
              - generic [ref=e1439]:
                - text: result:combination:combination-C-OPER-ALT:moment:pipe-P-120:quarter-3:bending-z
                - generic [ref=e1440]: not tolerance checked
              - generic [ref=e1441]: pipe:P-120
              - generic [ref=e1442]: bending_moment_z; quarter_3
              - generic [ref=e1443]:
                - generic [ref=e1445]: 178.52549999999997 N*m
                - generic [ref=e1446]:
                  - text: "Reference:"
                  - generic [ref=e1447]: 714.002028 N*m
                - generic [ref=e1448]:
                  - text: "Target:"
                  - generic [ref=e1449]: 892.527528 N*m
          - generic [ref=e1450]: 1 comparison diagnostic; top 4 deltas shown from 261 mapped pairs.
        - region "Design-authoring workspace" [ref=e1451]:
          - generic [ref=e1452]:
            - img [ref=e1453]
            - text: Design Workspace
          - generic [ref=e1458]:
            - generic [ref=e1459]:
              - generic [ref=e1460]:
                - checkbox "Include known private values in this local export" [ref=e1461]
                - text: Include known private values in this local export
              - generic [ref=e1462]: decisions=127; findings=127; blocked=true
              - generic "design-workspace-export-link redaction decisions" [ref=e1463]: path=$.__route_key__schema_version; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__document_kind; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__export_scope; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__package_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_item; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__project_ref; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__source_basis.__route_key__core_module; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__guard_tests; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__deliverable_context; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__source_basis.__route_key__app_surfaces[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[3]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[4]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[5]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[6]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[7]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_design_knowledge_record_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_design_knowledge_diagnostic_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_model_diagnostic_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_result_diagnostic_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_total_diagnostic_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_model_state_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_analysis_run_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_result_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_comparison_pair_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_unmatched_comparison_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__operation_review_record_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__selected_review_target_present; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__design_knowledge_record_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__records_with_unresolved_tbd; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__constraint_warning_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__constraint_validation_has_blocking_findings; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__model_state_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__analysis_run_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__model_comparison_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__unmatched_model_comparison_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__analysis_result_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__graphical_overlay_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__operation_diff_review_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__route_state; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__deterministic_workspace_hash_present; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__workspace_hash_scope; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__design_knowledge_panel; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__constraint_warning_panel; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__state_run_browser; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__comparison_tables; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__graphical_overlays; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__operation_diff_review; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__analysis_run_ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__model_state_ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__comparison_ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__review_state_routing.__route_key__selected_review_target_ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__review_state_routing.__route_key__route_state; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__review_state_routing.__route_key__result_selection_routes_to_model_context; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__review_state_routing.__route_key__diagnostic_selection_routes_to_model_context; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__mutation_boundary.__route_key__workspace_mutates_accepted_model_state; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__hidden_accepted_model_mutation; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__accepted_model_state_mutated; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__workspace_application_status; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__gui_originated_changes_route; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__accepted_operation_requires_explicit_user_acceptance_record; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__evidence_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__analysis_run_ref.__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__analysis_run_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_ref.__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__storage_convention; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__workspace_unit_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__angle; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__force; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__length; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__pressure; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__stress; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__temperature; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[3]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[4]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[5]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[6]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[7]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[8]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[9]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[10]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[11]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[3]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[4]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_unit_policy_ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_performed; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__tolerance_profile_ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__tolerance_status; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__protected_content_included; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__private_payload_included; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__public_examples_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__protected_source_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__private_data_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__professional_boundary; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__private_payload_included; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__protected_content_included; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__release_or_professional_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__human_review_required; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_compliance_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_certification_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_sealing_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_approval_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_authentication_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN
              - generic "design-workspace-export-link redaction findings" [ref=e1464]: path=$.__route_key__schema_version; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__document_kind; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__export_scope; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__package_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_item; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__project_ref; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__source_basis.__route_key__core_module; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__guard_tests; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__deliverable_context; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__source_basis.__route_key__app_surfaces[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[3]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[4]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[5]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[6]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__source_basis.__route_key__app_surfaces[7]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_design_knowledge_record_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_design_knowledge_diagnostic_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_model_diagnostic_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_result_diagnostic_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_total_diagnostic_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_model_state_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_analysis_run_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_result_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_comparison_pair_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__current_unmatched_comparison_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__operation_review_record_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__selected_review_target_present; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__design_knowledge_record_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__records_with_unresolved_tbd; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__constraint_warning_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__constraint_validation_has_blocking_findings; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__model_state_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__analysis_run_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__model_comparison_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__unmatched_model_comparison_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__analysis_result_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__graphical_overlay_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__operation_diff_review_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__route_state; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__deterministic_workspace_hash_present; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__core_contract_evidence.__route_key__workspace_hash_scope; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__design_knowledge_panel; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__constraint_warning_panel; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__state_run_browser; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__comparison_tables; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__graphical_overlays; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__operation_diff_review; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__analysis_run_ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__model_state_ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__current_workspace_state.__route_key__comparison_ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__review_state_routing.__route_key__selected_review_target_ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__review_state_routing.__route_key__route_state; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__review_state_routing.__route_key__result_selection_routes_to_model_context; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__review_state_routing.__route_key__diagnostic_selection_routes_to_model_context; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__mutation_boundary.__route_key__workspace_mutates_accepted_model_state; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__hidden_accepted_model_mutation; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__accepted_model_state_mutated; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__workspace_application_status; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__gui_originated_changes_route; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mutation_boundary.__route_key__accepted_operation_requires_explicit_user_acceptance_record; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__evidence_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__analysis_run_ref.__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__analysis_run_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_ref.__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__storage_convention; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__workspace_unit_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__angle; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__force; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__length; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__pressure; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__stress; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__temperature; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[3]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[4]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[5]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[6]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[7]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[8]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[9]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[10]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_units[11]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[3]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_units[4]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__comparison_unit_policy_ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_performed; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__tolerance_profile_ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__tolerance_status; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__protected_content_included; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__private_payload_included; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__public_examples_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__protected_source_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__private_data_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__professional_boundary; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__private_payload_included; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__protected_content_included; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__release_or_professional_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__human_review_required; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_compliance_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_certification_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_sealing_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_approval_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_authentication_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN
              - generic [ref=e1465]:
                - img [ref=e1466]
                - text: Workspace JSON
            - generic: knowledge=3; states=1; runs=1; comparisons=261; operations=0
          - generic [ref=e1469]:
            - generic [ref=e1470]:
              - generic [ref=e1471]: Core contract
              - strong [ref=e1472]: records=2; warnings=1; states=2; runs=2; overlays=5
            - generic [ref=e1473]:
              - generic [ref=e1474]: Current browser
              - strong [ref=e1475]: analysis_run=run:preview-linear-static-001; comparison=comparison:run:preview-linear-static-001:load-L-100-to-combination-C-OPER-ALT; result_rows=830
            - generic [ref=e1476]:
              - generic [ref=e1477]: Operation review
              - strong [ref=e1478]: records=0; accepted_mutation=false; application=empty_operation_queue
            - generic [ref=e1479]:
              - generic [ref=e1480]: Routing
              - strong [ref=e1481]: selected=none; route=no_selected_review_target
            - generic [ref=e1482]:
              - generic [ref=e1483]: Units
              - strong [ref=e1484]: model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC; results=MPa,N,N*m,N*m/rad,N/m,boolean,count,m,mm,mode_code,rad,state_code; comparison=MPa,N,N*m,mm,rad; conversion=false
            - generic [ref=e1485]:
              - generic [ref=e1486]: Boundary
              - strong [ref=e1487]: private_payload=false; protected=false; professional_claim=false
          - generic [ref=e1488]: Design workspace evidence is a local composition of GUI state, comparison context, and operation-review metadata. It does not apply operations or mutate accepted model state; acceptance and professional judgment remain with the responsible engineer.
  - region "Workspace status" [ref=e1489]:
    - generic "Analysis statuses" [ref=e1490]:
      - generic "MECHANICS_SOLVED" [ref=e1491]:
        - strong [ref=e1492]: Mechanics
        - code [ref=e1493]: MECHANICS_SOLVED
      - generic "RULE_INPUTS_INCOMPLETE" [ref=e1494]:
        - strong [ref=e1495]: Rule check
        - code [ref=e1496]: RULE_INPUTS_INCOMPLETE
      - generic "HUMAN_REVIEW_REQUIRED" [ref=e1497]:
        - strong [ref=e1498]: Professional
        - code [ref=e1499]: HUMAN_REVIEW_REQUIRED
      - generic "seam=browser_fixture_no_backend_job; project=project:invented-loop-01; result_model=project:invented-loop-01; identity=match; rows=830; generation=5; job=job:preview-linear-static:project-invented-loop-01; model_sha256=sha256:b2cec8ee447c44fa7efbdf43c0970a5c690b8c63e24577a105ce12e7f613ccc3; input_manifest_sha256=e5eb1c25d854a98cd230da42c4fde20d617ee1c3a9c772f2cd8320adb732a0a4" [ref=e1500]:
        - strong [ref=e1501]: Solve proof
        - code [ref=e1502]: seam=browser_fixture_no_backend_job; project=project:invented-loop-01; result_model=project:invented-loop-01; identity=match; rows=830; generation=5; job=job:preview-linear-static:project-invented-loop-01; model_sha256=sha256:b2cec8ee447c44fa7efbdf43c0970a5c690b8c63e24577a105ce12e7f613ccc3; input_manifest_sha256=e5eb1c25d854a98cd230da42c4fde20d617ee1c3a9c772f2cd8320adb732a0a4
    - generic [ref=e1503]:
      - button "Local · no network · no telemetry Local preview store" [ref=e1504] [cursor=pointer]:
        - text: Local · no network · no telemetry
        - generic [ref=e1505]: Local preview store
      - button "⚑ 35 Issues" [ref=e1506] [cursor=pointer]
  - generic [ref=e1507]: Technical preview — not a released product. Acceptance and professional judgment remain with the responsible engineer.
```

# Test source

```ts
  1   | import { expect, test, type Page } from "@playwright/test";
  2   | import { readFileSync } from "node:fs";
  3   | import { inflateSync } from "node:zlib";
  4   | 
  5   | type RehearsalStep = {
  6   |   change_kind: string;
  7   |   payload: Record<string, any>;
  8   |   target?: {
  9   |     ref: string;
  10  |   };
  11  | };
  12  | 
  13  | type RehearsalFixture = {
  14  |   steps: RehearsalStep[];
  15  | };
  16  | 
  17  | const rehearsal = JSON.parse(
  18  |   readFileSync(new URL("../../../fixtures/product_preview/r2_from_blank_rehearsal.json", import.meta.url), "utf8")
  19  | ) as RehearsalFixture;
  20  | 
  21  | // TP-R3UX-CADSHELL: the shell keeps the spatial core (model tree | viewport |
  22  | // property inspector) persistent and dominant, with the remaining panels
  23  | // collapsed by default and summoned from the in-DOM View menu (which the native
  24  | // macOS menu mirrors in the Tauri shell). The specs drive that menu exactly as
  25  | // a human following SMOKE.md TP-MAC-189 would. Idempotent: if the requested
  26  | // section is already open, do nothing (re-selecting it would toggle it shut).
  27  | async function openWorkspaceSection(page: Page, sectionId: string): Promise<void> {
  28  |   const section = page.getByTestId(`workspace-section-${sectionId}`);
  29  |   if (await section.isVisible()) {
  30  |     return;
  31  |   }
  32  |   await page.getByTestId("menu-view").click();
  33  |   await page.getByTestId(`menu-item-view.section.${sectionId}`).click();
  34  |   await expect(section).toBeVisible();
  35  | }
  36  | 
  37  | async function ensureTreeExpanded(page: Page): Promise<void> {
  38  |   const toggle = page.getByTestId("toggle-tree");
  39  |   if ((await toggle.getAttribute("aria-expanded")) !== "true") {
  40  |     await toggle.click();
  41  |   }
  42  |   await expect(toggle).toHaveAttribute("aria-expanded", "true");
  43  | }
  44  | 
  45  | async function ensureInspectorExpanded(page: Page): Promise<void> {
  46  |   const toggle = page.getByTestId("toggle-inspector");
  47  |   if ((await toggle.getAttribute("aria-expanded")) !== "true") {
  48  |     await toggle.click();
  49  |   }
  50  |   await expect(toggle).toHaveAttribute("aria-expanded", "true");
  51  | }
  52  | 
  53  | async function ensureCreationToolArmed(page: Page, testId: "command-node" | "command-pipe", label: string): Promise<void> {
  54  |   const button = page.getByTestId(testId);
  55  |   if ((await button.getAttribute("aria-pressed")) !== "true") {
  56  |     await button.click();
  57  |   }
  58  |   await expect(button).toHaveAttribute("aria-pressed", "true");
  59  |   await expect(page.getByTestId("armed-creation-tool")).toContainText(label);
  60  |   await expect(page.getByTestId("viewport-editor-intents")).toHaveClass(/active/);
  61  | }
  62  | 
  63  | async function ensurePipeEndpointPick(page: Page, testId: "viewport-pick-pipe-from" | "viewport-pick-pipe-to"): Promise<void> {
  64  |   const button = page.getByTestId(testId);
  65  |   if ((await button.getAttribute("aria-pressed")) !== "true") {
  66  |     await button.click();
  67  |   }
  68  |   await expect(button).toHaveAttribute("aria-pressed", "true");
  69  | }
  70  | 
  71  | async function expectWorkspaceStatusClearOfTarget(
  72  |   page: Page,
  73  |   targetTestId: string,
  74  | ): Promise<void> {
  75  |   const target = page.getByTestId(targetTestId);
  76  |   await target.scrollIntoViewIfNeeded();
  77  |   const [targetBox, statusBox] = await Promise.all([
  78  |     target.boundingBox(),
  79  |     page.getByTestId("workspace-status-bar").boundingBox(),
  80  |   ]);
  81  |   expect(targetBox).not.toBeNull();
  82  |   expect(statusBox).not.toBeNull();
  83  |   const overlaps =
  84  |     targetBox!.x < statusBox!.x + statusBox!.width &&
  85  |     targetBox!.x + targetBox!.width > statusBox!.x &&
  86  |     targetBox!.y < statusBox!.y + statusBox!.height &&
  87  |     targetBox!.y + targetBox!.height > statusBox!.y;
> 88  |   expect(overlaps, `${targetTestId} must not be covered by workspace status`).toBe(false);
      |                                                                               ^ Error: result-row-result:force:pipe-P-120:axial must not be covered by workspace status
  89  | }
  90  | 
  91  | test("guided workbench shell keeps journey steps, details, and compact status reachable", async ({ page }) => {
  92  |   await page.goto("/");
  93  | 
  94  |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  95  |   await expect(page.getByTestId("app-menu-bar")).toBeVisible();
  96  |   // The dock and detailed rails are collapsed by default so the 3D model plus
  97  |   // the local review-only agent rail dominate the primary screen.
  98  |   await expect(page.getByTestId("workspace-dock")).toHaveClass(/collapsed/);
  99  |   await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  100 |   await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  101 |   await expect(page.getByTestId("agent-workbench-panel")).toBeVisible();
  102 |   await expect(page.getByTestId("agent-focus-selection")).toContainText("project:invented-loop-01");
  103 |   await expect(page.getByTestId("agent-proposal-summary")).toContainText("review_only_local_preview");
  104 |   await expect(page.getByTestId("workspace-status-bar")).toBeVisible();
  105 |   await expect(page.getByTestId("status-pill-professional")).toContainText("HUMAN_REVIEW_REQUIRED");
  106 |   await page.getByTestId("toggle-tree").click();
  107 |   await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "true");
  108 |   await page.getByTestId("layout-mode-grid").click();
  109 |   await expect(page.getByTestId("entity-grid")).toBeVisible();
  110 |   await expect(page.getByTestId("entity-grid-table-nodes")).toBeVisible();
  111 |   await page.getByTestId("entity-grid-row-node:N-100").click();
  112 |   await expect(page.getByTestId("agent-focus-selection")).toContainText("node:N-100");
  113 |   await expect(page.getByLabel("Property inspector")).toContainText("node:N-100");
  114 |   await page.getByTestId("entity-grid-input-node:N-100-x").fill("1.25");
  115 |   await page.getByTestId("entity-grid-input-node:N-100-y").fill("0.5");
  116 |   await expect(page.getByTestId("entity-grid-change-count")).toContainText("2 changed cells");
  117 |   await page.getByTestId("queue-entity-grid-intents").click();
  118 |   await expect(page.getByTestId("entity-grid-queued-message")).toContainText("Queued 2 review intents");
  119 |   await expect(page.getByTestId("operation-apply-row-editor-intent-1")).toContainText(
  120 |     "op:grid-intent-node:N-100-position-x"
  121 |   );
  122 |   await expect(page.getByTestId("operation-apply-row-editor-intent-2")).toContainText(
  123 |     "op:grid-intent-node:N-100-position-y"
  124 |   );
  125 |   await page.getByTestId("layout-mode-tree").click();
  126 |   await expect(page.getByTestId("audit-boundary-drawer")).toHaveCount(0);
  127 |   await page.getByTestId("audit-drawer-toggle").click();
  128 |   await expect(page.getByTestId("audit-boundary-drawer")).toBeVisible();
  129 |   await expect(page.getByTestId("local-project-status")).toContainText("network=false");
  130 |   await expect(page.getByTestId("local-project-status")).toContainText("telemetry=false");
  131 |   await page.getByTestId("audit-boundary-drawer").getByRole("button", { name: /Close/i }).click();
  132 |   await openWorkspaceSection(page, "operations");
  133 |   await expect(page.getByTestId("editor-contract-panel")).toBeVisible();
  134 |   await expect(page.getByTestId("editor-contract-unit-contract")).toContainText("contract=DEL-02-02");
  135 |   await expect(page.getByTestId("editor-contract-unit-contract")).toContainText(
  136 |     "schema=schemas/units.schema.yaml#/$defs/DimensionId"
  137 |   );
  138 |   await expect(page.getByTestId("editor-contract-unit-contract")).toContainText(
  139 |     "policy=unit_bearing_values_require_explicit_unit_metadata"
  140 |   );
  141 |   await expect(page.getByTestId("editor-contract-unit-contract")).toContainText("missing=diagnostic_blocking");
  142 |   await expect(page.getByTestId("diff-preview-panel")).toBeVisible();
  143 | 
  144 |   await openWorkspaceSection(page, "rule-packs");
  145 |   await expect(page.getByTestId("workspace-section-rule-packs")).toBeVisible();
  146 | 
  147 |   const horizontalOverflow = await page.evaluate(
  148 |     () =>
  149 |       document.documentElement.scrollWidth > document.documentElement.clientWidth ||
  150 |       document.body.scrollWidth > document.body.clientWidth
  151 |   );
  152 |   expect(horizontalOverflow).toBe(false);
  153 | });
  154 | 
  155 | test("DEC-077 solve temperature queues an explicit unit-bearing operation", async ({ page }) => {
  156 |   await page.goto("/");
  157 |   await ensureTreeExpanded(page);
  158 |   await ensureInspectorExpanded(page);
  159 |   await page.getByTestId("tree-row-load:L-100").click();
  160 | 
  161 |   const editorIntentPanel = page.getByTestId("editor-intent-panel");
  162 |   await editorIntentPanel
  163 |     .getByTestId("editor-intent-field")
  164 |     .selectOption("modulus_basis_temperature.value");
  165 |   await expect(editorIntentPanel.getByTestId("editor-intent-unit")).toHaveValue("K");
  166 |   await editorIntentPanel.getByTestId("editor-intent-value").fill("400");
  167 |   await expect(editorIntentPanel.getByTestId("editor-intent-validation")).toContainText(
  168 |     "model_metadata_unit_dimension_declared"
  169 |   );
  170 |   await expect(editorIntentPanel.getByTestId("queue-editor-intent")).toBeEnabled();
  171 |   await editorIntentPanel.getByTestId("queue-editor-intent").evaluate((button: HTMLButtonElement) => button.click());
  172 |   await expect(editorIntentPanel.getByTestId("editor-intent-queue")).toContainText(
  173 |     "modulus_basis_temperature.value"
  174 |   );
  175 |   await expect(editorIntentPanel.getByTestId("editor-intent-queue")).toContainText(
  176 |     '{"value":400,"unit":"K"}'
  177 |   );
  178 | });
  179 | 
  180 | test("R2 desktop preview smoke covers solve, results, report, and viewport overlay", async ({ page }) => {
  181 |   await page.goto("/");
  182 | 
  183 |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  184 |   await expect(page.getByRole("heading", { name: "OpenPipeStress" })).toBeVisible();
  185 |   // Engine-ready guard (DEC-020 / ADR-0001): browser mode answers operations
  186 |   // through the wasm32 operation_applier build; wait for init before edits.
  187 |   await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");
  188 |   await expect(page.getByTestId("property-unit-catalog-status")).toContainText(
```