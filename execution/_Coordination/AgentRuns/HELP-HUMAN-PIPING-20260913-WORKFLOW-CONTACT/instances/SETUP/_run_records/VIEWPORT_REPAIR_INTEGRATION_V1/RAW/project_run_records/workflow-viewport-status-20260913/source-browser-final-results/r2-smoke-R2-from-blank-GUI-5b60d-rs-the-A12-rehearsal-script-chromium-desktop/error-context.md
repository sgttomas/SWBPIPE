# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: r2-smoke.spec.ts >> R2 from-blank GUI journey authors the A12 rehearsal script
- Location: e2e/r2-smoke.spec.ts:860:1

# Error details

```
Error: expect(received).toBeTruthy()

Received: null
```

# Page snapshot

```yaml
- main [ref=e3]:
  - generic [ref=e4]:
    - generic [ref=e5]:
      - heading "OpenPipeStress" [level=1] [ref=e6]
      - paragraph [ref=e7]: Blank Local Model
    - generic "Local project controls" [ref=e8]:
      - group [ref=e9]:
        - generic "Units" [ref=e10] [cursor=pointer]
        - option "Entered" [selected]
        - option "SI"
        - option "US"
      - button "Create local" [ref=e11] [cursor=pointer]:
        - img [ref=e12]
        - text: Create local
      - button "New blank" [ref=e16] [cursor=pointer]:
        - img [ref=e17]
        - text: New blank
      - button "Open local" [ref=e20] [cursor=pointer]:
        - img [ref=e21]
        - text: Open local
      - button "List local" [ref=e23] [cursor=pointer]:
        - img [ref=e24]
        - text: List local
      - button "Save local" [ref=e25] [cursor=pointer]:
        - img [ref=e26]
        - text: Save local
  - group "Project summary" [ref=e30]:
    - generic "Details" [ref=e31] [cursor=pointer]:
      - status [ref=e32]: Opened local browser-preview project snapshot.
      - generic [ref=e33]: Details
  - navigation "Application menu" [ref=e34]:
    - button "File" [ref=e36] [cursor=pointer]
    - button "Edit" [ref=e38] [cursor=pointer]
    - button "View" [ref=e40] [cursor=pointer]
    - button "Insert" [ref=e42] [cursor=pointer]
    - button "Analyze" [ref=e44] [cursor=pointer]
  - generic [ref=e45]:
    - navigation "Workspace tasks" [ref=e46]:
      - button "Model" [ref=e47] [cursor=pointer]:
        - img [ref=e48]
        - text: Model
      - button "Loads" [ref=e51] [cursor=pointer]:
        - img [ref=e52]
        - text: Loads
      - button "Analyze" [pressed] [ref=e56] [cursor=pointer]:
        - img [ref=e57]
        - text: Analyze
      - button "Results" [ref=e59] [cursor=pointer]:
        - img [ref=e60]
        - text: Results
      - button "Rules" [ref=e62] [cursor=pointer]:
        - img [ref=e63]
        - text: Rules
      - button "Report" [ref=e66] [cursor=pointer]:
        - img [ref=e67]
        - text: Report
    - group "Editing tools" [ref=e70]:
      - button "Select" [ref=e71] [cursor=pointer]:
        - img [ref=e72]
        - text: Select
      - region "Human toolkit" [ref=e74]:
        - button "Toolkit" [ref=e75] [cursor=pointer]:
          - img [ref=e76]
          - text: Toolkit
      - button "Undo model edit" [disabled] [ref=e79]:
        - img [ref=e80]
      - button "Redo model edit" [disabled] [ref=e83]:
        - img [ref=e84]
      - button "Review" [ref=e87] [cursor=pointer]:
        - img [ref=e88]
        - text: Review
  - generic [ref=e92]:
    - region "Modeling workspace" [ref=e93]:
      - generic [ref=e94]:
        - button "Collapse model tree" [expanded] [ref=e95] [cursor=pointer]:
          - img [ref=e96]
          - generic [ref=e98]: Model
          - generic [ref=e99]: ‹
        - generic "Model tree" [ref=e100]:
          - generic [ref=e101]: Model
          - region "Layout grid mode" [ref=e102]:
            - button "Tree" [pressed] [ref=e103]:
              - img [ref=e104]
              - text: Tree
            - button "Grid" [ref=e107]:
              - img [ref=e108]
              - text: Grid
          - region "Model tree filtering" [ref=e110]:
            - generic [ref=e111]:
              - img [ref=e112]
              - generic [ref=e115]: Filter model
              - searchbox "Filter model tree" [ref=e116]
            - generic [ref=e117]: 9 of 9 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e118]:
              - img [ref=e119]
          - button "Blank Local Model" [pressed] [ref=e122] [cursor=pointer]:
            - img [ref=e123]
            - strong [ref=e128]: Blank Local Model
          - generic [ref=e129]:
            - heading "Materials" [level=3] [ref=e130]
            - button "R2 invented carbon-steel-like material material:r2-carbon-steel" [ref=e131] [cursor=pointer]:
              - img [ref=e132]
              - generic [ref=e135]:
                - strong [ref=e136]: R2 invented carbon-steel-like material
                - generic [ref=e137]: material:r2-carbon-steel
          - generic [ref=e138]:
            - heading "Sections" [level=3] [ref=e139]
            - button "R2 invented pipe section section:r2-pipe" [ref=e140] [cursor=pointer]:
              - img [ref=e141]
              - generic [ref=e143]:
                - strong [ref=e144]: R2 invented pipe section
                - generic [ref=e145]: section:r2-pipe
          - generic [ref=e146]:
            - heading "Nodes" [level=3] [ref=e147]
            - button "R2 anchored node node:R2-100" [ref=e148] [cursor=pointer]:
              - img [ref=e149]
              - generic [ref=e152]:
                - strong [ref=e153]: R2 anchored node
                - generic [ref=e154]: node:R2-100
            - button "R2 loaded node node:R2-110" [ref=e155] [cursor=pointer]:
              - img [ref=e156]
              - generic [ref=e159]:
                - strong [ref=e160]: R2 loaded node
                - generic [ref=e161]: node:R2-110
          - generic [ref=e162]:
            - heading "Pipes" [level=3] [ref=e163]
            - button "R2 invented straight pipe pipe:R2-100" [ref=e164] [cursor=pointer]:
              - img [ref=e165]
              - generic [ref=e169]:
                - strong [ref=e170]: R2 invented straight pipe
                - generic [ref=e171]: pipe:R2-100
          - generic [ref=e172]:
            - heading "Supports" [level=3] [ref=e173]
            - button "R2 user anchor support:R2-anchor" [ref=e174] [cursor=pointer]:
              - img [ref=e175]
              - generic [ref=e178]:
                - strong [ref=e179]: R2 user anchor
                - generic [ref=e180]: support:R2-anchor
          - generic [ref=e181]:
            - heading "Load Cases" [level=3] [ref=e182]
            - button "R2 invented operating load load:R2-L-100" [ref=e183] [cursor=pointer]:
              - img [ref=e184]
              - generic [ref=e186]:
                - strong [ref=e187]: R2 invented operating load
                - generic [ref=e188]: load:R2-L-100
          - generic [ref=e189]:
            - heading "Combinations" [level=3] [ref=e190]
            - button "R2 invented operating combination combination:R2-C-100" [ref=e191] [cursor=pointer]:
              - img [ref=e192]
              - generic [ref=e196]:
                - strong [ref=e197]: R2 invented operating combination
                - generic [ref=e198]: combination:R2-C-100
      - generic [ref=e200]:
        - generic [ref=e201]:
          - generic [ref=e202]: 3D Centerline
          - group "Viewport deformation overlay status" [ref=e203]:
            - generic "Deformation · blocked" [ref=e204] [cursor=pointer]
          - group "Viewport display toggles" [ref=e205]:
            - button "Labels" [pressed] [ref=e206]
            - button "Loads" [pressed] [ref=e207]
            - button "Grid" [pressed] [ref=e208]
          - generic [ref=e209]: "Selected: project:blank-local-20260913t215903z"
        - generic [ref=e210]:
          - generic "Three.js pipe centerline viewport" [ref=e211]
          - generic "Viewport entity selection":
            - button "Select R2 anchored node in viewport" [ref=e213] [cursor=pointer]:
              - img [ref=e214]
              - generic [ref=e217]: R2-100
            - button "Select R2 loaded node in viewport" [ref=e218] [cursor=pointer]:
              - img [ref=e219]
              - generic [ref=e222]: R2-110
            - button "Select R2 invented straight pipe in viewport" [ref=e223] [cursor=pointer]:
              - img [ref=e224]
              - generic [ref=e228]: R2-100
            - button "Select R2 user anchor in viewport" [ref=e229] [cursor=pointer]:
              - img [ref=e230]
              - generic [ref=e233]: R2-anchor
          - generic "Orientation gizmo" [ref=e234]
          - generic "View controls" [ref=e237]:
            - button "Front" [ref=e238] [cursor=pointer]
            - button "Top" [ref=e239] [cursor=pointer]
            - button "Iso" [pressed] [ref=e240] [cursor=pointer]
          - generic [ref=e242]: 1 m
        - region "Command and selection bar" [ref=e243]:
          - generic "Object creation tools" [ref=e244]:
            - button "Node" [ref=e245] [cursor=pointer]:
              - img [ref=e246]
              - text: Node
            - button "Pipe" [ref=e248] [cursor=pointer]:
              - img [ref=e249]
              - text: Pipe
            - button "Support" [pressed] [ref=e253] [cursor=pointer]:
              - img [ref=e254]
              - text: Support
            - button "Component" [ref=e257] [cursor=pointer]:
              - img [ref=e258]
              - text: Component
            - button "Load" [ref=e261] [cursor=pointer]:
              - img [ref=e262]
              - text: Load
          - 'generic "Support tool armed: select a node, then complete Create support in the Inspector." [ref=e264]': Support tool armed
          - group [ref=e265]:
            - generic "Selection & navigation" [ref=e266] [cursor=pointer]
      - generic [ref=e267]:
        - button "Collapse inspector" [expanded] [ref=e268] [cursor=pointer]:
          - img [ref=e269]
          - generic [ref=e271]: Properties
          - generic [ref=e272]: ›
        - generic "Property inspector" [ref=e273]:
          - heading "project:blank-local-20260913t215903z" [level=2] [ref=e274]
          - group [ref=e275]:
            - generic "All properties" [ref=e276] [cursor=pointer]
          - region "Editor operation intent" [ref=e277]:
            - paragraph [ref=e278]: Select a model entity to edit its properties.
          - group [ref=e279]:
            - generic "Sources and units" [ref=e280] [cursor=pointer]
          - group [ref=e281]:
            - generic "New support configuration" [ref=e282] [cursor=pointer]
            - option "Not provided" [selected]
            - option "node:R2-100"
            - option "node:R2-110"
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
          - group [ref=e283]:
            - generic "New section" [ref=e284] [cursor=pointer]
            - region "Create section intent" [ref=e285]:
              - generic [ref=e286]:
                - generic [ref=e287]:
                  - generic [ref=e288]: Section ID
                  - textbox "New section ID" [ref=e289]: section:S-1
                - generic [ref=e290]:
                  - generic [ref=e291]: Name
                  - textbox "New section name" [ref=e292]: Section S-1
                - generic [ref=e293]:
                  - generic [ref=e294]: Type
                  - combobox "New section type" [ref=e295]:
                    - option "pipe" [selected]
                - generic [ref=e296]:
                  - generic [ref=e297]: Length unit
                  - combobox "New section length unit" [ref=e298]:
                    - option "m, model metadata" [selected]
                - generic [ref=e299]:
                  - generic [ref=e300]: Outside diameter (m, model metadata)
                  - textbox "New section outside diameter" [ref=e301]
                - generic [ref=e302]:
                  - generic [ref=e303]: Wall thickness (m, model metadata)
                  - textbox "New section wall thickness" [ref=e304]
                - generic [ref=e305]:
                  - generic [ref=e306]: Provenance
                  - textbox "New section provenance" [ref=e307]: user_entered_local_preview
                - button "Queue section" [disabled] [ref=e308]:
                  - img [ref=e309]
                  - text: Queue section
          - group [ref=e311]:
            - generic "New material" [ref=e312] [cursor=pointer]
            - region "Create material intent" [ref=e313]:
              - generic [ref=e314]:
                - generic [ref=e315]:
                  - generic [ref=e316]: Material ID
                  - textbox "New material ID" [ref=e317]: material:M-1
                - generic [ref=e318]:
                  - generic [ref=e319]: Label
                  - textbox "New material label" [ref=e320]: Material M-1
                - generic [ref=e321]:
                  - generic [ref=e322]: Modulus unit
                  - combobox "New material modulus unit" [ref=e323]:
                    - option "Pa, model metadata" [selected]
                - generic [ref=e324]:
                  - generic [ref=e325]: Elastic modulus (Pa, model metadata)
                  - textbox "New material elastic modulus" [ref=e326]
                - generic [ref=e327]:
                  - generic [ref=e328]: Shear modulus (Pa, model metadata)
                  - textbox "New material shear modulus" [ref=e329]
                - generic [ref=e330]:
                  - generic [ref=e331]: Thermal expansion unit
                  - combobox "New material thermal expansion unit" [ref=e332]:
                    - option "1/C, model metadata" [selected]
                - generic [ref=e333]:
                  - generic [ref=e334]: Thermal expansion (1/C, model metadata)
                  - textbox "New material thermal expansion" [ref=e335]
                - generic [ref=e336]:
                  - generic [ref=e337]: Provenance
                  - textbox "New material provenance" [ref=e338]: user_entered_local_preview
                - button "Queue material" [disabled] [ref=e339]:
                  - img [ref=e340]
                  - text: Queue material
          - group [ref=e342]:
            - generic "New support" [ref=e343] [cursor=pointer]
            - region "Create support intent" [ref=e344]:
              - generic [ref=e345]:
                - generic [ref=e346]:
                  - generic [ref=e347]: Support ID
                  - textbox "New support ID" [ref=e348]: support:S-1
                - generic [ref=e349]:
                  - generic [ref=e350]: Label
                  - textbox "New support label" [ref=e351]: Support S-1
                - generic [ref=e352]:
                  - generic [ref=e353]: Node
                  - combobox "New support node" [ref=e354]:
                    - option "R2 anchored node (node:R2-100)" [selected]
                    - option "R2 loaded node (node:R2-110)"
                - generic "New support restraints" [ref=e355]:
                  - generic [ref=e356]:
                    - checkbox "UX" [checked] [ref=e357]
                    - generic [ref=e358]: UX
                  - generic [ref=e359]:
                    - checkbox "UY" [checked] [ref=e360]
                    - generic [ref=e361]: UY
                  - generic [ref=e362]:
                    - checkbox "UZ" [checked] [ref=e363]
                    - generic [ref=e364]: UZ
                  - generic [ref=e365]:
                    - checkbox "RX" [ref=e366]
                    - generic [ref=e367]: RX
                  - generic [ref=e368]:
                    - checkbox "RY" [ref=e369]
                    - generic [ref=e370]: RY
                  - generic [ref=e371]:
                    - checkbox "RZ" [ref=e372]
                    - generic [ref=e373]: RZ
                - generic [ref=e374]:
                  - generic [ref=e375]: Linear stiffness unit
                  - combobox "New support linear stiffness unit" [ref=e376]:
                    - option "N/m, model metadata" [selected]
                - generic [ref=e377]:
                  - generic [ref=e378]: Linear stiffness (N/m, model metadata)
                  - textbox "New support linear stiffness" [ref=e379]
                - generic [ref=e380]:
                  - generic [ref=e381]: Provenance
                  - textbox "New support provenance" [ref=e382]: user_entered_local_preview
                - button "Queue support" [ref=e383] [cursor=pointer]:
                  - img [ref=e384]
                  - text: Queue support
              - article [ref=e386]:
                - generic [ref=e387]:
                  - generic [ref=e388]:
                    - generic [ref=e389]: Operation
                    - strong [ref=e390]: op:create-support-support:S-1; create; proposed
                  - generic [ref=e391]:
                    - generic [ref=e392]: Target
                    - strong [ref=e393]: Support; support:S-1
                  - generic [ref=e394]:
                    - generic [ref=e395]: Change
                    - strong [ref=e396]: "create_support; Explicit support; supports; before=not_present; after={\"id\":\"support:S-1\",\"label\":\"Support S-1\",\"node\":\"node:R2-100\",\"restraints\":[\"UX\",\"UY\",\"UZ\"],\"provenance\":\"user_entered_local_preview\"}"
                  - generic [ref=e397]:
                    - generic [ref=e398]: Unit basis
                    - strong [ref=e399]: dimensionless; unit=none; explicit user-entered support node and restraint tokens
                  - generic [ref=e400]:
                    - generic [ref=e401]: Validation
                    - strong [ref=e402]: not_run; not_run; not_required_dimensionless; not_generated; not_applied
                  - generic [ref=e403]:
                    - generic [ref=e404]: Audit boundary
                    - strong [ref=e405]: Routed through the structured operations only; no direct model mutation; requires your acceptance; does not change the accepted model until applied
                  - generic [ref=e406]:
                    - generic [ref=e407]: Professional boundary
                    - strong [ref=e408]: Requires human review; no compliance claim; no approval claim
                  - generic [ref=e409]:
                    - generic [ref=e410]: Rationale
                    - strong [ref=e411]: explicit user-entered support for project:blank-local-20260913t215903z; requires service validation before durable model change.
          - group [ref=e412]:
            - generic "New component" [ref=e413] [cursor=pointer]
            - option "bend" [selected]
            - option "tee"
            - option "reducer"
            - option "valve"
            - option "flange"
            - option "expansion_joint"
            - option "R2 anchored node (node:R2-100)" [selected]
            - option "R2 loaded node (node:R2-110)"
            - option "Select connected pipe"
            - option "R2 invented straight pipe (pipe:R2-100)" [selected]
            - option "m, model metadata" [selected]
            - option "rad, model metadata" [selected]
    - region "Workspace sections" [ref=e414]:
      - generic [ref=e415]:
        - heading "Analyze" [level=2] [ref=e416]
        - button "Close" [ref=e417] [cursor=pointer]:
          - img [ref=e418]
          - text: Close
      - region "Solve section" [ref=e422]:
        - region "Solve execution" [ref=e423]:
          - generic [ref=e424]: Execution
          - region "Solve readiness summary" [ref=e425]:
            - generic [ref=e426]:
              - img [ref=e427]
              - generic [ref=e430]: Mechanics readiness
              - strong [ref=e431]: 0 computed result rows; model incomplete
            - generic [ref=e432]:
              - img [ref=e433]
              - generic [ref=e435]: Rule-check readiness
              - strong [ref=e436]: rule inputs incomplete; mechanics results remain reviewable only; rule inputs incomplete
            - generic [ref=e437]:
              - img [ref=e438]
              - generic [ref=e440]: Diagnostics
              - strong [ref=e441]: 2 diagnostics; 0 warnings; 2 blocking/error
            - generic [ref=e442]:
              - img [ref=e443]
              - generic [ref=e445]: Professional boundary
              - strong [ref=e446]: no professional acceptance record; human review remains required; acceptance stays with the responsible engineer
          - generic [ref=e447]:
            - generic [ref=e448]:
              - generic [ref=e449]:
                - checkbox "Include known private values in this local export" [ref=e450]
                - text: Include known private values in this local export
              - generic [ref=e451]: decisions=127; findings=127; blocked=true
              - generic "solve-job-export-link redaction decisions" [ref=e452]: path=$.__route_key__schema_version; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__document_kind; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__export_scope; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[3]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__project_ref; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__job_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__job_state; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__solver_mode; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__event_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__result_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__diagnostic_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__cancellation_requested; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__cancellation_status; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__running; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__solver_mode; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__sparse_interactive_default; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__dense_scrutiny_explicit; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__progress_basis; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__percentages_synthesized; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__backend_percent_stream_available; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__latest_event_state; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__request_control_visible; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__request_enabled; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__requested; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__status; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__backend_job_seam; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__backend_job_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__backend_cancellation_token; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__cancellation_scope; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__mutates_solver_process_directly; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__cancellation_success_claimed; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__model_state_ref.__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__model_state_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_run_ref.__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_run_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__run_kind; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_status[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_status[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_status[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__result_hash_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__hash_scopes[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__hash_scopes[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__input_manifest_refs[0].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__input_manifest_refs[0].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__evidence_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__storage_convention; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__solve_job_unit_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__force; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__length; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__moment; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__pressure; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__stress; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__temperature; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__analysis_run_ref.__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__analysis_run_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_performed; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__object_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__event_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__state; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__message; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__result_available; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__diagnostic_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__result_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__event_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__state; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__message; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__result_available; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__diagnostic_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__result_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__analysis_status[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__analysis_status[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__event_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__state; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__message; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__result_available; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__diagnostic_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__result_row_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__analysis_status[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__analysis_status[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__analysis_status[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__affected_refs[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__code; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__message; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__severity; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__source; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__code; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__severity; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__source; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__affected_refs[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__message; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__error_message; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__private_data_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__professional_boundary; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__protected_source_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__public_examples_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__private_payload_included; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__protected_content_included; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__release_or_professional_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__human_review_required; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_compliance_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_certification_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_sealing_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_approval_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_authentication_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN
              - generic "solve-job-export-link redaction findings" [ref=e453]: path=$.__route_key__schema_version; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__document_kind; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__export_scope; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[3]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__project_ref; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__job_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__job_state; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__solver_mode; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__event_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__result_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__diagnostic_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__cancellation_requested; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__cancellation_status; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__running; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__solver_mode; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__sparse_interactive_default; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__dense_scrutiny_explicit; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__progress_basis; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__percentages_synthesized; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__backend_percent_stream_available; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__progress_contract.__route_key__latest_event_state; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__request_control_visible; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__request_enabled; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__requested; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__status; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__backend_job_seam; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__backend_job_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__backend_cancellation_token; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__cancellation_scope; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__mutates_solver_process_directly; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__cancellation.__route_key__cancellation_success_claimed; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__model_state_ref.__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__model_state_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_run_ref.__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_run_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__run_kind; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_status[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_status[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__analysis_status[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__result_hash_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__hash_scopes[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__hash_scopes[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__input_manifest_refs[0].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__input_manifest_refs[0].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__evidence_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__storage_convention; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__solve_job_unit_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__force; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__length; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__moment; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__pressure; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__stress; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__temperature; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__result_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__analysis_run_ref.__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__analysis_run_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_performed; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__object_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__event_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__state; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__message; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__result_available; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__diagnostic_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[0].__route_key__result_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__event_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__state; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__message; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__result_available; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__diagnostic_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__result_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__analysis_status[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[1].__route_key__analysis_status[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__event_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__state; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__message; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__result_available; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__diagnostic_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__result_row_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__analysis_status[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__analysis_status[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__events[2].__route_key__analysis_status[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__affected_refs[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__code; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__message; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__severity; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[0].__route_key__source; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__code; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__severity; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__source; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__affected_refs[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostics[1].__route_key__message; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__error_message; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__private_data_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__professional_boundary; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__protected_source_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__public_examples_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__private_payload_included; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__protected_content_included; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__release_or_professional_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__human_review_required; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_compliance_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_certification_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_sealing_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_approval_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_authentication_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN
              - generic [ref=e454]:
                - img [ref=e455]
                - text: Solve job JSON
            - generic: state=completed; events=3; result_rows=0; cancellation_requested=false
          - generic [ref=e458]:
            - generic [ref=e459]:
              - generic [ref=e460]: Solver mode
              - strong [ref=e461]: selected=sparse_interactive; sparse_default=true; dense_scrutiny=false
            - generic [ref=e462]:
              - generic [ref=e463]: Progress
              - strong [ref=e464]: completed; preview_service_event_state_only_no_percent_stream; percentages_synthesized=false
            - generic [ref=e465]:
              - generic [ref=e466]: Cancellation
              - strong [ref=e467]: control_visible=true; enabled=false; requested=false; seam=browser_fixture_no_backend_job; token=unavailable_no_backend_job_browser_fixture_mode; success_claimed=false
            - generic [ref=e468]:
              - generic [ref=e469]: Result binding
              - strong [ref=e470]: state:project:blank-local-20260913t215903z:preview; run:preview-linear-static-browser-blocked:project-blank-local-20260913t215903z; result rows=0; hashes=0
            - generic [ref=e471]:
              - generic [ref=e472]: Unit policy
              - strong [ref=e473]: model=force=N,length=m,moment=N*m,pressure=Pa,stress=Pa,temperature=C; results=none; rows=0; conversion=false
            - generic [ref=e474]:
              - generic [ref=e475]: Boundary
              - strong [ref=e476]: private payload=false; protected content=false; release/professional claim=false; human review required
          - group "Solver mode" [ref=e477]:
            - button "Sparse interactive" [pressed] [ref=e478] [cursor=pointer]:
              - img [ref=e479]
              - text: Sparse interactive
            - button "Dense scrutiny" [ref=e481] [cursor=pointer]:
              - img [ref=e482]
              - text: Dense scrutiny
          - button "Run mechanics preview" [ref=e485] [cursor=pointer]:
            - img [ref=e486]
            - text: Run mechanics preview
          - button "Cancel preview" [disabled] [ref=e488] [cursor=pointer]:
            - img [ref=e489]
            - text: Cancel preview
        - region "Rule-check completeness" [ref=e491]:
          - generic [ref=e492]:
            - img [ref=e493]
            - text: Rule-Check Completeness
          - generic [ref=e495]:
            - generic [ref=e496]:
              - generic [ref=e497]:
                - checkbox "Include known private values in this local export" [ref=e498]
                - text: Include known private values in this local export
              - generic [ref=e499]: decisions=87; findings=87; blocked=true
              - generic "rule-check-export-link redaction decisions" [ref=e500]: path=$.__route_key__schema_version; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__document_kind; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__export_scope; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[3]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[1]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[2]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[3]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__project_ref; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__run_ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__model_ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mechanics_status; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__rule_check_status; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_acceptance_status; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__finding_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__rule_check_blocked; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__rule_check_blocking_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__provenance_warning_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__ip_boundary_warning_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__mechanics_results_reviewable; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__silent_defaults_used; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__bundled_code_values_used; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__summary.__route_key__compliance_claim_made; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__evidence_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__ref_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__ref_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__storage_convention; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__rule_completeness_unit_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__force; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__length; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__moment; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__pressure; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__stress; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__temperature; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_bearing_record_count; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__rule_input_unit_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_mismatch_diagnostic_code; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_performed; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref_type; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__protected_content_included; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__private_payload_included; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__finding_id; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__warning_class; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__severity; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__affected_refs[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__source_refs[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__message; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__remediation; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__private_data_required; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__protected_content_required; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__findings[0].__route_key__mechanics_solve_blocking; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__rule_check_blocking; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostic_refs[0]; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__private_data_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__professional_boundary; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__protected_source_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__public_examples_policy; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__private_payload_included; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__protected_content_included; classification=private_project_data; action=block_export; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__release_or_professional_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__human_review_required; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_compliance_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_certification_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_sealing_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_approval_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_authentication_claim; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN
              - generic "rule-check-export-link redaction findings" [ref=e501]: path=$.__route_key__schema_version; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__document_kind; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__export_scope; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__deliverable_refs[3]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__scope_items[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[1]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[2]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__objectives[3]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__project_ref; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__run_ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__model_ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__mechanics_status; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__rule_check_status; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_acceptance_status; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__finding_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__rule_check_blocked; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__rule_check_blocking_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__provenance_warning_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__ip_boundary_warning_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__mechanics_results_reviewable; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__silent_defaults_used; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__summary.__route_key__bundled_code_values_used; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__summary.__route_key__compliance_claim_made; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__evidence_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_system_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__ref_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__ref_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__source_model_ref.__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__storage_convention; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__rule_completeness_unit_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__force; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__length; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__moment; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__pressure; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__stress; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__model_units.__route_key__temperature; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_bearing_record_count; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__rule_input_unit_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__unit_mismatch_diagnostic_code; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__conversion_performed; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[0].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[1].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[2].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref_type; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__decision_basis_refs[3].__route_key__ref; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__unit_policy_evidence.__route_key__protected_content_included; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__unit_policy_evidence.__route_key__private_payload_included; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__finding_id; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__warning_class; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__severity; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__affected_refs[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__source_refs[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__message; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__remediation; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__private_data_required; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__protected_content_required; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__findings[0].__route_key__mechanics_solve_blocking; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__findings[0].__route_key__rule_check_blocking; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__diagnostic_refs[0]; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__private_data_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__professional_boundary; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__protected_source_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__data_boundary.__route_key__public_examples_policy; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__private_payload_included; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__protected_content_included; class=PRIVATE_DATA_WARNING; severity=BLOCKING; reason=LOCAL_PRIVATE_INTENT_REQUIRED path=$.__route_key__release_or_professional_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__human_review_required; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_compliance_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_certification_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_sealing_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_approval_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN path=$.__route_key__professional_boundary.__route_key__software_makes_authentication_claim; class=PROVENANCE_WARNING; severity=WARNING; reason=REDISTRIBUTION_STATUS_UNKNOWN
              - generic [ref=e502]:
                - img [ref=e503]
                - text: Local completeness JSON
            - generic: 1 review findings; rule_check_blocked=true; mechanics_reviewable=false
          - generic [ref=e506]:
            - generic [ref=e507]:
              - generic [ref=e508]: Rule status
              - strong [ref=e509]: RULE_INPUTS_INCOMPLETE
            - generic [ref=e510]:
              - generic [ref=e511]: Mechanics status
              - strong [ref=e512]: MODEL_INCOMPLETE
            - generic [ref=e513]:
              - generic [ref=e514]: Unit policy
              - strong [ref=e515]: model=force=N,length=m,moment=N*m,pressure=Pa,stress=Pa,temperature=C; records=5; rule_input_units=explicit_or_blocking; diagnostic=RULE_UNIT_MISMATCH; conversion=false
            - generic [ref=e516]:
              - generic [ref=e517]: Boundary
              - strong [ref=e518]: private rule data absent; silent defaults used=false; bundled code values=false; no compliance claim
            - article [ref=e520]:
              - strong [ref=e521]: RULE_CHECK_BLOCKING
              - generic [ref=e522]: warning; rule_blocking=true; private_data=true
              - paragraph [ref=e523]: User-supplied rule-pack inputs are absent, so rule-check status stays incomplete while mechanics output remains separate. Provide a private rule pack and project-specific criteria with provenance; the public preview does not bundle code defaults.
          - generic [ref=e524]: Completeness findings explain missing user/private rule-check data. Mechanics preview output is decision-support information; acceptance and professional judgment remain with the responsible engineer.
        - region "Run rule checks" [ref=e525]:
          - generic [ref=e526]:
            - img [ref=e527]
            - text: Run Rule Checks (user-defined, private)
          - generic [ref=e530]:
            - generic [ref=e531]: "Project scope: project:blank-local-20260913t215903z."
            - generic [ref=e532]: No solved mechanics result in this session; run a solve first so solver_result inputs can bind to result rows.
          - generic [ref=e533]:
            - button "Load demo rule pack" [ref=e534]
            - button "Refresh saved packs" [ref=e535]
          - generic [ref=e537]: Saved rule-pack list not refreshed yet.
          - generic [ref=e538]:
            - generic [ref=e539]: Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)
            - textbox "Rule-pack document JSON (load the demo or a saved pack, or paste a private pack. The binding controls below are derived from this document.)" [ref=e540]:
              - /placeholder: No rule-pack document. Load the demo, open a saved pack, or paste one.
          - generic [ref=e541]:
            - button "Run checks" [disabled] [ref=e542]:
              - img [ref=e543]
              - text: Run checks
            - generic [ref=e545]: No rule-check run in this session.
          - generic [ref=e546]:
            - img [ref=e547]
            - text: Rule checks run locally over user-supplied data only. The software emits user-defined check statuses (USER_RULE_CHECKED / USER_RULE_FAILED / RULE_INPUTS_INCOMPLETE). Human review remains required; acceptance stays with the responsible engineer.
        - region "Design knowledge" [ref=e549]:
          - generic [ref=e550]:
            - img [ref=e551]
            - text: Design Knowledge
          - generic [ref=e553]:
            - article [ref=e554]:
              - generic [ref=e555]:
                - strong [ref=e556]: Pump nozzle endpoint
                - generic [ref=e557]: endpoint · review_required
              - paragraph [ref=e558]: Invented endpoint record used to show how user-supplied design context appears in the preview.
              - generic [ref=e559]: invented_example
            - article [ref=e560]:
              - generic [ref=e561]:
                - strong [ref=e562]: Rack corridor placeholder
                - generic [ref=e563]: routing_corridor · review_required
              - paragraph [ref=e564]: Invented corridor note; no owner standard or protected spacing criterion is bundled.
              - generic [ref=e565]: invented_example
            - article [ref=e566]:
              - generic [ref=e567]:
                - strong [ref=e568]: Support stiffness dimensions unresolved
                - generic [ref=e569]: assumption · unresolved
              - paragraph [ref=e570]: Linear and rotational support behavior is represented as a preview diagnostic until user-supplied stiffness values and canonical dimensions are provided.
              - generic [ref=e571]: invented_example
  - region "Workspace status" [ref=e572]:
    - generic "Analysis statuses" [ref=e573]:
      - group [ref=e574]:
        - generic "Mechanics model incomplete" [ref=e575] [cursor=pointer]:
          - strong [ref=e576]: Mechanics
          - text: model incomplete
      - group [ref=e577]:
        - generic "Rule check Inputs needed" [ref=e578] [cursor=pointer]:
          - strong [ref=e579]: Rule check
          - text: Inputs needed
      - group [ref=e580]:
        - generic "Professional Review required" [ref=e581] [cursor=pointer]:
          - strong [ref=e582]: Professional
          - text: Review required
    - generic [ref=e583]:
      - button "Local · no network · no telemetry Local preview store" [ref=e584] [cursor=pointer]:
        - text: Local · no network · no telemetry
        - generic [ref=e585]: Local preview store
      - button "⚑ 6 Issues" [ref=e586] [cursor=pointer]
  - generic [ref=e587]: Technical preview — not a released product. Acceptance and professional judgment remain with the responsible engineer.
```

# Test source

```ts
  951  |     reviewSequence: 3,
  952  |     appliedSequence: 5,
  953  |     publishedTestId: "tree-row-pipe:R2-100",
  954  |     publishedText: pipe.label
  955  |   });
  956  | 
  957  |   const support = stepPayload("create_support", "support:R2-anchor");
  958  |   await page.getByTestId("command-support").click();
  959  |   await expect(page.getByTestId("create-support-id")).toBeFocused();
  960  |   await expect(page.getByTestId("create-support-id")).toBeVisible();
  961  |   await page.getByTestId("create-support-id").fill(support.id);
  962  |   await page.getByTestId("create-support-label").fill(support.label);
  963  |   await page.getByTestId("create-support-node").selectOption(support.node);
  964  |   for (const restraint of ["RX", "RY", "RZ"]) {
  965  |     await page.getByTestId(`create-support-restraint-${restraint}`).setChecked(true);
  966  |   }
  967  |   await page.getByTestId("create-support-provenance").fill(support.provenance);
  968  |   await page.getByTestId("queue-create-support-intent").click();
  969  |   await applyQueuedIntent(page, 3, 6, support.id);
  970  | 
  971  |   const loadCase = stepPayload("create_load_case", "load:R2-L-100");
  972  |   await openWorkspaceSection(page, "loads");
  973  |   await page.getByTestId("load-manager-create-load-id").fill(loadCase.id);
  974  |   await page.getByTestId("load-manager-create-load-label").fill(loadCase.label);
  975  |   await page.getByTestId("load-manager-create-load-kind").fill(loadCase.kind);
  976  |   await page.getByTestId("load-manager-create-load-status").fill(loadCase.status);
  977  |   await page.getByTestId("load-manager-create-load-provenance").fill(loadCase.provenance);
  978  |   await page.getByTestId("queue-create-load-case-intent").click();
  979  |   await applyQueuedIntent(page, 4, 7, loadCase.id);
  980  | 
  981  |   const primitive = stepPayload("create_primitive_load", "load:R2-L-100-FY");
  982  |   await openWorkspaceSection(page, "loads");
  983  |   await page.getByTestId("load-manager-create-primitive-load-case").selectOption(loadCase.id);
  984  |   await page.getByTestId("load-manager-create-primitive-category").selectOption(primitive.category);
  985  |   await page.getByTestId("load-manager-create-primitive-id").fill(primitive.id);
  986  |   await page.getByTestId("load-manager-create-primitive-node").selectOption(primitive.target.node);
  987  |   await page.getByTestId("load-manager-create-primitive-direction").selectOption(primitive.direction);
  988  |   await page.getByTestId("load-manager-create-primitive-magnitude").fill(String(primitive.magnitude.value));
  989  |   await page.getByTestId("load-manager-create-primitive-provenance").fill(primitive.provenance);
  990  |   await page.getByTestId("queue-create-primitive-intent").click();
  991  |   await applyQueuedIntent(page, 5, 8, primitive.id);
  992  | 
  993  |   const combination = stepPayload("create_combination", "combination:R2-C-100");
  994  |   await openWorkspaceSection(page, "loads");
  995  |   await page.getByTestId("load-manager-create-combination-id").fill(combination.id);
  996  |   await page.getByTestId("load-manager-create-combination-label").fill(combination.label);
  997  |   await page.getByTestId("load-manager-create-combination-load-case").selectOption(combination.terms[0].load_case);
  998  |   await page.getByTestId("load-manager-create-combination-factor").fill(String(combination.terms[0].factor));
  999  |   await page.getByTestId("load-manager-create-combination-provenance").fill(combination.provenance);
  1000 |   await page.getByTestId("load-manager-create-combination-rationale").fill("A8 GUI replay of the A12 invented rehearsal.");
  1001 |   await page.getByTestId("queue-create-combination-intent").click();
  1002 |   await applyQueuedIntent(page, 6, 9, combination.id);
  1003 | 
  1004 |   await openWorkspaceSection(page, "loads");
  1005 |   await expect(page.getByTestId("load-case-manager-summary")).toContainText(
  1006 |     "1 load cases; 1 primitive loads; 1 combinations"
  1007 |   );
  1008 |   await openWorkspaceSection(page, "project");
  1009 |   await page.getByRole("button", { name: "Save local" }).click();
  1010 |   await expect(page.getByTestId("local-project-message")).toContainText(
  1011 |     "Saved local browser-preview project snapshot without external file copies."
  1012 |   );
  1013 |   await page.getByRole("button", { name: "Open local" }).click();
  1014 |   await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project snapshot.");
  1015 |   await expect(page.getByTestId("project-storage-unit-round-trip")).toContainText(
  1016 |     "status=unit_metadata_preserved_in_local_project_envelope"
  1017 |   );
  1018 |   await expect(page.getByTestId("project-storage-unit-round-trip")).toContainText("project.units.length=m");
  1019 |   await expect(page.getByTestId("project-storage-unit-round-trip")).toContainText("conversion=false");
  1020 |   await expect(page.getByTestId("project-validation-unit-round-trip")).toContainText(
  1021 |     "status=unit_metadata_preserved_in_local_project_envelope"
  1022 |   );
  1023 |   await expect(page.getByTestId("project-validation-unit-policy")).toContainText(
  1024 |     "round_trip=unit_metadata_preserved_in_local_project_envelope"
  1025 |   );
  1026 |   await expect(page.getByTestId("project-validation-unit-policy")).toContainText("conversion=false");
  1027 |   await page.getByTestId("audit-drawer-toggle").click();
  1028 |   const auditDrawer = page.getByTestId("audit-boundary-drawer");
  1029 |   await expect(auditDrawer).toBeVisible();
  1030 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText("model=");
  1031 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText("force=N");
  1032 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText("length=m");
  1033 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText(
  1034 |     "manual=unit_and_schema_verification"
  1035 |   );
  1036 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText("conversion=false");
  1037 |   await expect(auditDrawer.getByTestId("accessibility-baseline-unit-visibility")).toContainText(
  1038 |     "policy=unit_bearing_values_keep_visible_unit_labels_in_review_surfaces"
  1039 |   );
  1040 |   await expect(auditDrawer.getByTestId("accessibility-baseline-unit-visibility")).toContainText("length=m");
  1041 |   await expect(auditDrawer.getByTestId("accessibility-baseline-unit-visibility")).toContainText("conversion=false");
  1042 |   await auditDrawer.getByRole("button", { name: /Close/i }).click();
  1043 |   await openWorkspaceSection(page, "solve");
  1044 |   await page.getByTestId("run-mechanics-preview").click();
  1045 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  1046 |   await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=0");
  1047 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("results=none");
  1048 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("rows=0");
  1049 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("conversion=false");
  1050 |   const solveJobHref = await page.getByTestId("solve-job-export-link").getAttribute("href");
> 1051 |   expect(solveJobHref).toBeTruthy();
       |                        ^ Error: expect(received).toBeTruthy()
  1052 |   const solveJobPacket = JSON.parse(decodeURIComponent(solveJobHref!.split(",", 2)[1]));
  1053 |   expect(solveJobPacket.unit_policy_evidence.unit_system_ref.ref).toBe("unit-system:dec-018-si-dual-display");
  1054 |   expect(solveJobPacket.unit_policy_evidence.model_units.length).toBe("m");
  1055 |   expect(solveJobPacket.unit_policy_evidence.result_units).toEqual([]);
  1056 |   expect(solveJobPacket.unit_policy_evidence.result_row_count).toBe(0);
  1057 |   expect(solveJobPacket.unit_policy_evidence.conversion_performed).toBe(false);
  1058 |   await page.getByTestId("issues-drawer-toggle").click();
  1059 |   await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toContainText(
  1060 |     "BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL"
  1061 |   );
  1062 |   await page.getByTestId("issues-home").getByRole("button", { name: /Close/i }).click();
  1063 |   await openWorkspaceSection(page, "report");
  1064 |   await expect(page.getByTestId("rendered-report-render")).toBeDisabled();
  1065 |   await expect(page.getByTestId("rendered-report-precondition")).toBeVisible();
  1066 |   await expect(page.getByTestId("rendered-report-unit-basis")).toHaveCount(0);
  1067 |   await expectWorkspaceStatusClearOfTarget(page, "rendered-report-render");
  1068 |   await expect(page.getByTestId("rendered-report-route")).toHaveCount(0);
  1069 |   await expect(page.getByTestId("rendered-report-preview")).toHaveCount(0);
  1070 | });
  1071 | 
  1072 | test("diagnostic detail exposes linked result unit context", async ({ page }) => {
  1073 |   await page.goto("/");
  1074 |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  1075 |   await openWorkspaceSection(page, "solve");
  1076 | 
  1077 |   await page.getByTestId("run-mechanics-preview").click();
  1078 |   await page.getByTestId("issues-drawer-toggle").click();
  1079 |   await page.getByTestId("diagnostic-filter-input").fill("result:stress:pipe-P-130");
  1080 |   const diagnosticButton = page.getByTestId("diagnostic-COMBINATION_STRESS_SUMMARY_SKIPPED");
  1081 |   await expect(diagnosticButton).toBeVisible();
  1082 |   await diagnosticButton.click();
  1083 | 
  1084 |   await expect(page.getByTestId("selected-diagnostic-linked-results")).toContainText("result:stress:pipe-P-130");
  1085 |   await expect(page.getByTestId("diagnostic-unit-context")).toContainText("linked_results=1");
  1086 |   await expect(page.getByTestId("diagnostic-unit-context")).toContainText("units=MPa");
  1087 |   await expect(page.getByTestId("diagnostic-unit-context")).toContainText("source=result_envelope");
  1088 |   await expect(page.getByTestId("diagnostic-unit-context")).toContainText("conversion=false");
  1089 | });
  1090 | 
  1091 | // Phase C2 slice 1 (TP-C2-EDITOR-001): the rule-pack manager authors a
  1092 | // private draft in memory and reports the honest desktop-only seam for
  1093 | // validation, checksum, persistence, and listing in browser mode — the
  1094 | // same recorded boundary pattern as report rendering and the unit catalog.
  1095 | test("rule-pack manager drafts privately and reports the desktop-only backend seam", async ({ page }) => {
  1096 |   await page.goto("/");
  1097 |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  1098 |   await ensureEngineReady(page);
  1099 | 
  1100 |   await openWorkspaceSection(page, "rule-packs");
  1101 |   await expect(page.getByTestId("rule-pack-scope-status")).toContainText("local SQLite only");
  1102 |   await expect(page.getByTestId("rule-pack-boundary-note")).toContainText("DEC-037");
  1103 | 
  1104 |   await page.getByTestId("rule-pack-new-draft").click();
  1105 |   const draftText = await page.getByTestId("rule-pack-draft-json").inputValue();
  1106 |   const draft = JSON.parse(draftText);
  1107 |   expect(draft.rule_pack_kind).toBe("open_pipe_stress_rule_pack");
  1108 |   expect(draft.grammar_version).toBe("1.0.0");
  1109 |   expect(draft.classification.privacy_class).toBe("private_user_data");
  1110 |   expect(draft.classification.redistribution_status).toBe("private_only");
  1111 |   await expect(page.getByTestId("rule-pack-action-status")).toContainText("private_user_data");
  1112 | 
  1113 |   // Slice 2 (TP-C2-COMPOSER-001) + DEC-037 follow-up: the structured AST
  1114 |   // composer rewrites the selected formula's expression through visible
  1115 |   // controls. It may render display-only text, but ships no writable text
  1116 |   // syntax and no parser. Switching the root node type rewrites the canonical
  1117 |   // document JSON the validate/save flow reads.
  1118 |   await expect(page.getByTestId("rule-pack-expression-composer")).toBeVisible();
  1119 |   await expect(page.getByTestId("rule-pack-variable-browser")).toContainText("user_required_input_1");
  1120 | 	  await expect(page.getByTestId("rule-pack-expression-text-preview")).toContainText(
  1121 | 	    "Read-only AST-to-text preview"
  1122 | 	  );
  1123 | 	  await expect(page.getByTestId("rule-pack-expression-text-preview")).toContainText(
  1124 | 	    "user_required_input_1"
  1125 | 	  );
  1126 |   expect(draft.formula_declarations[0].declaration_payload.expression_ast.node).toBe("variable_ref");
  1127 | 
  1128 |   // Slice 4 (TP-C2-DECLEDITOR-001): the declarations editor authors the
  1129 |   // required_inputs / value_slots the composer's variable_ref binds to. Add a
  1130 |   // required input from blank; the canonical document JSON grows and the
  1131 |   // composer's variable picker reflects the new id — no raw JSON. Still
  1132 |   // structured-only (D-02b).
  1133 |   await expect(page.getByTestId("rule-pack-declarations-editor")).toBeVisible();
  1134 |   await page.getByTestId("rule-pack-input-add").click();
  1135 |   // TP-UNITS-B2-RULEPACKUNITS-001: browser preview cannot call the desktop
  1136 |   // get_unit_catalog command, so declaration unit refs stay editable as stored
  1137 |   // unit text here. The desktop catalog-picker path is covered by mocked-Tauri
  1138 |   // Vitest; this e2e assertion protects the no-fallback/manual-entry route.
  1139 |   await page.getByTestId("rule-pack-input-dimension").last().selectOption("stress");
  1140 |   await page.getByTestId("rule-pack-input-unit").last().fill("MPa");
  1141 |   await expect(page.getByTestId("rule-pack-declarations-unit-policy")).toContainText(
  1142 |     "catalog_route=browser_preview_manual_entry"
  1143 |   );
  1144 |   await expect(page.getByTestId("rule-pack-declarations-unit-policy")).toContainText(
  1145 |     "required_input:user_required_input_2=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview(unit=MPa;dimension=stress)"
  1146 |   );
  1147 |   const declText = await page.getByTestId("rule-pack-draft-json").inputValue();
  1148 |   expect(JSON.parse(declText).required_inputs).toHaveLength(2);
  1149 |   expect(JSON.parse(declText).required_inputs[1].quantity_intent).toMatchObject({
  1150 |     dimension: "stress",
  1151 |     unit_ref: "MPa"
```