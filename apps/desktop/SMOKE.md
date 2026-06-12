# Desktop Physics Workflow Smoke

This smoke check covers the TP-MAC-02 desktop path over invented local preview
data. It is intentionally limited to workflow wiring and product-boundary
signals; it is not a validation, certification, compliance, or professional
approval check.

## Start

```bash
npm run dev --workspace apps/desktop
```

Open the Vite URL printed by the command, normally
`http://127.0.0.1:5173/`.

## Required Checks

1. Initial load shows `data-testid="desktop-preview-shell"` and the header
   `OpenPipeStress Technical Preview`.
2. `data-testid="solve-panel"` shows initial status values from the fixture,
   including `data-testid="status-mechanics"` with
   `ready for preview diagnostics`.
3. Click `data-testid="run-mechanics-preview"`.
4. Results render `data-testid="results-panel"` with grouped sections:
   `result-group-displacement`, `result-group-reaction`,
   `result-group-force`, `result-group-moment`, and `result-group-stress`.
5. Results include `data-testid="result-row-result:force:pipe-P-120:axial"`
   and `data-testid="result-row-result:force:pipe-P-120:axial:end-j"`.
6. Click `data-testid="result-row-result:force:pipe-P-120:axial"` and confirm
   `data-testid="result-detail-panel"` shows `axial_force`,
   `element_local`, `end_i`, `recovered_from_local_element_stiffness`,
   the sign convention, `pipe:P-120`, `DEL-14-02`, and
   `run:preview-linear-static-001`.
7. Confirm `data-testid="endpoint-pair-table"` shows both
   `result:force:pipe-P-120:axial` and
   `result:force:pipe-P-120:axial:end-j`.
8. Click `data-testid="result-row-result:force:pipe-P-120:axial:end-j"` and
   confirm the detail panel shows `end_j` and the j-end sign convention.
9. Results include `data-testid="result-row-result:stress:pipe-P-120"` and
   `data-testid="result-row-result:stress:pipe-P-120:end-j:torsional-shear"`.
10. Click
    `data-testid="result-row-result:stress:pipe-P-120:end-j:torsional-shear"`
    and confirm `data-testid="result-detail-panel"` shows
    `torsional_shear_stress`, `element_local`, `end_j`,
    `recovered_from_open_mechanics_stress_components`, `pipe:P-120`, and the
    paired `result:stress:pipe-P-120:end-i:torsional-shear`.
11. Confirm the model context selects `pipe:P-120` and the property inspector
   (`aria-label="Property inspector"`) shows `Rack span`.
12. Confirm results include quarter-station and shear rows:
    `result:force:pipe-P-120:shear-y`,
    `result:force:pipe-P-120:shear-y:end-j`,
    `result:force:pipe-P-120:quarter-1:shear-y`,
    `result:force:pipe-P-120:midspan:shear-z`,
    `result:force:pipe-P-120:quarter-3:shear-z`, and
    `result:moment:pipe-P-120:quarter-1:bending-z`.
13. Click `data-testid="result-row-result:force:pipe-P-120:midspan:axial"`
    and confirm `data-testid="result-detail-panel"` shows `midspan`,
    `interpolated_from_endpoint_resultants`, and no endpoint-pair table.
14. Click `data-testid="result-row-result:force:pipe-P-120:quarter-1:shear-y"`
    and confirm the detail panel shows `shear_force_y`, `quarter_1`,
    `interpolated_from_endpoint_resultants`, and no endpoint-pair table.
15. Click `data-testid="result-row-result:force:pipe-P-120:shear-y"` and
    confirm endpoint-pair display still applies to endpoint shear force rows
    and includes `result:force:pipe-P-120:shear-y:end-j`.
16. Results include
    `data-testid="result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"`.
    Click it and confirm `data-testid="result-detail-panel"` shows
    `explicit_user_linear_combination`, `combination:combination:C-OPER-ALT`,
    source result refs for `result:force:pipe-P-120:axial` and
    `result:loadcase:load-L-200:force:pipe-P-120:axial`, and no compliance
    or professional approval claim.
17. Confirm results include
    `result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y`
    and `result:stress:pipe-P-120:quarter-1:torsional-shear`.
18. Confirm `data-testid="mechanics-gap-ledger"` includes
   `data-testid="gap:endpoint-j-recovery"` with status `implemented`; this is
   not a compliance failure.
19. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:endpoint-stress-components"` with status `implemented`;
    this is not a compliance failure.
20. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:station-recovery"` with status `implemented`, with
    fixed station-grid recovery implemented and arbitrary station sweeps, exact
    internal diagrams, and shear stress recovery deferred.
21. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:thermal-behavior"` with status `implemented`, with
    broader thermal combinations and temperature-dependent properties deferred.
22. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:load-combinations"` with status `implemented`, with
    code/rule combinations still deferred.
23. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:pressure-frame-load"` with status `implemented`, with
    broader pressure behavior still deferred.
24. Confirm results include
    `result:stress:pipe-P-120:end-i:pressure-hoop`, do not include
    `pressure-longitudinal` or shear stress rows such as
    `result:stress:pipe-P-120:quarter-1:shear-y`, and diagnostics do not include
    `PRESSURE_LOAD_NOT_APPLIED_TO_FRAME_VECTOR`.
25. Diagnostics render `data-testid="diagnostics-panel"` and include
   `diagnostic-HIGH_DISPLACEMENT_REVIEW` and
   `diagnostic-RULE_CHECK_INPUTS_MISSING`.
26. Click `data-testid="diagnostic-HIGH_DISPLACEMENT_REVIEW"` and confirm
   `data-testid="diagnostic-detail-panel"` shows
   `diagnostic:physics:high-displacement-review`,
   `result:disp:node-N-140`, `node:N-140`, linked result value context, and the
   review-only professional boundary.
27. Confirm the model context selects `node:N-140` and the property inspector
   (`aria-label="Property inspector"`) shows `Terminal tie-in`.
28. Knowledge renders `data-testid="knowledge-panel"` and includes computed
   context for `result:disp:node-N-140` and
   `result:force:pipe-P-120:axial`.
29. Report packet renders `data-testid="report-panel"` and
   `data-testid="report-packet-body"` with selected result refs including
   `result:disp:node-N-140`, `result:force:pipe-P-120:axial`, and
   `result:force:pipe-P-120:axial:end-j`,
   `result:force:pipe-P-120:midspan:axial`,
   `result:force:pipe-P-120:quarter-1:shear-y`,
   `result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial`,
   `result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y`,
   `result:stress:pipe-P-120:end-j:torsional-shear`, and
   `result:stress:pipe-P-120:quarter-1:torsional-shear`.
30. Report packet includes `data-testid="report-analysis-run"` with `DEL-14-02`
   and `run:preview-linear-static-001`; `data-testid="report-packet-body"`
   includes the result-value hash and `result_envelope` audit context.
31. Report packet includes `data-testid="report-load-basis-refs"` with
    `load:L-100`, `load:L-200`, and `combination:C-OPER-ALT`.
32. Click `data-testid="generate-review-proposal"`.
33. Proposal renders `data-testid="proposal-body"` and references the selected
   review target `result:stress:pipe-P-120:end-j:torsional-shear` if the
   endpoint stress row remains selected.
34. After selecting `diagnostic-HIGH_DISPLACEMENT_REVIEW`, generating a proposal
   references `diagnostic:physics:high-displacement-review` and uses diagnostic
   review wording.
35. `data-testid="accept-proposal-disabled"` is disabled; the proposal must
    remain review-only and must not mutate accepted model state.
36. Footer retains the technical-preview boundary text and does not claim
    production readiness, release readiness, certification, sealing, code
    compliance, or licensed engineering reliance.

## Current Status

The selectors above are present in the React UI and covered by Vitest where
practical. A live browser run passed on 2026-05-10 using the in-app browser at
`http://127.0.0.1:5173/`.

TP-MAC-03 result-interpretation smoke also passed on 2026-05-10. The run
confirmed axial force result selection, detail metadata display, `pipe:P-120`
model/viewport context selection, selected-result review proposal text,
disabled accept control, and endpoint-j recovery listed as a deferred mechanics
gap.

TP-MAC-03 diagnostic-interpretation smoke also passed on 2026-05-10. The run
confirmed `HIGH_DISPLACEMENT_REVIEW` selection, affected result/model refs,
linked result value context, `node:N-140` model/viewport selection, selected
diagnostic review proposal text, and disabled accept control.

TP-MAC-04 endpoint-result smoke passed on 2026-05-10. The run confirmed
end-j axial result selection, end-j metadata and j-end sign convention, endpoint
pair display, endpoint-j recovery marked implemented in the mechanics gap
ledger, report-packet inclusion of the end-j axial result ref, selected-result
review proposal text, and disabled accept control.

TP-MAC-05 endpoint-stress smoke passed on 2026-05-10. The run confirmed
end-j torsional shear stress result selection, stress metadata and endpoint
pair display, endpoint stress components marked implemented in the mechanics
gap ledger, report-packet inclusion of the end-j torsional shear stress ref,
selected-result review proposal text, and disabled accept control.

TP-RUN-01 preview-runtime smoke passed on 2026-05-10. The run confirmed the
React-loaded preview model is passed into the desktop runtime solve path while
the fixture-backed browser path remains intact, endpoint force and endpoint
stress result interpretation still works, report-packet audit context remains
visible, selected diagnostic proposals remain review-only, and the disabled
accept control does not mutate accepted model state.

TP-MAC-08 load-combination smoke passed on 2026-05-10. The run confirmed the
explicit mechanics-basis combination row renders, the detail panel shows
`explicit_user_linear_combination`, `combination:C-OPER-ALT`, and source result
refs for both load cases, the mechanics gap ledger marks explicit load
combinations implemented, and the report packet load-basis refs include
`load:L-100`, `load:L-200`, and `combination:C-OPER-ALT`.

TP-MAC-09 pressure-thrust smoke passed on 2026-05-10. The run confirmed the
mechanics gap ledger marks pressure-to-frame conversion implemented for the
bounded closed-end straight-pipe thrust slice, pressure hoop rows render,
pressure-longitudinal rows are suppressed, the prior pressure-not-applied
diagnostic is absent, and report refs still include the explicit mechanics
combination row.

TP-MAC-10 station-grid smoke passed on 2026-05-10. The run confirmed
quarter-station rows render, local shear force rows render, endpoint-pair
display remains endpoint-only, station-grid gap ledger text marks exact
diagrams, arbitrary station sweeps, and shear stress recovery deferred,
pressure hoop rows remain present, pressure-longitudinal rows remain
suppressed for thrust-active cases, shear stress rows are absent, and report
refs include quarter-station force, stress, and combination rows.

TP-MAC-11 model-workspace entity coverage smoke passed on 2026-06-07. The run
confirmed the model tree renders `Materials`, `Components`, `Load Cases`, and
`Combinations`; `material:invented-carbon-steel`, `component:C-110`,
`load:L-100`, and `combination:C-OPER-ALT` are uniquely selectable; the
property inspector shows the invented fixture ID, material properties,
kind/status/basis, load categories, load targets, combination terms, and
provenance; the technical-preview footer remains visible; and browser console
errors were absent.

TP-MAC-12 local-project-control fallback smoke passed on 2026-06-07. The run
confirmed the non-Tauri browser-preview path labels storage as a local
browser-session preview store, reports `network=false`, `daemon=false`,
`telemetry=false`, and `FTS5=false`, and round-trips Create local -> Save local
-> Open local over the invented preview model without external file copies,
repository-default writes, or browser console errors. The Tauri runtime path
remains the app-local SQLite project-store authority.

TP-MAC-13 preview-boundary strip smoke passed on 2026-06-07. The run confirmed
the app renders a visible boundary strip sourced from the invented preview
model's `data_boundary` fields: public example data, protected-content policy,
private-data policy, and professional-boundary policy. The footer retained
technical-preview/no-reliance wording, the local project toolbar retained
`network=false` and `telemetry=false`, and browser console errors were absent.

TP-MAC-14 report-export smoke passed on 2026-06-07 using the in-app browser at
`http://127.0.0.1:5174/`. The run confirmed `report-export-link` renders as a
local `data:application/json` download named
`openpipestress-preview-report-run-preview-linear-static-001.json`; the parsed
JSON packet reports `openpipestress.technical_preview.report_packet_export`,
`local_browser_download_preview`, `DEL-14-02`,
`run:preview-linear-static-001`, 11 selected result refs, 6 diagnostic refs,
load-basis refs for `load:L-100`, `load:L-200`, and
`combination:C-OPER-ALT`, and the `result_envelope` hash scope. The packet
excluded private payloads, protected content, and release or professional
claims, retained the human-review boundary, and browser console errors were
absent.

TP-MAC-15 proposal-boundary smoke passed on 2026-06-07 using the in-app
browser at `http://127.0.0.1:5174/`. The run confirmed mechanics execution
enables the review-only proposal path; the generated proposal exposes
`op:review-computed-diagnostic`, `attach_design_knowledge`,
`draft_user_review_required`, affected entity `result:force:pipe-P-120:axial`,
validation statuses including
`warning_computed_context_requires_human_review`,
`generated_from_computed_context`, and `not_applied`, audit-boundary flags for
required user acceptance, no accepted-model-state mutation, and review-only
acceptance recording, and professional-boundary flags with human review
required and software approval/compliance claims false. The accept action
remained disabled, the technical-preview footer boundary remained visible, and
browser console errors were absent.

TP-MAC-16 solve-readiness smoke passed on 2026-06-07 using the in-app browser
at `http://127.0.0.1:5174/`. The initial run state showed preview mechanics
not started, rule inputs incomplete with mechanics results still reviewable
only, 1 diagnostic, 1 warning, 0 blocking/error findings, and no professional
acceptance record. After `Run mechanics preview`, readiness updated to 647
computed result rows with mechanics solved, rule inputs still incomplete and
review-only, 7 diagnostics, 7 warnings, 0 blocking/error findings, and the
same human-review/no-professional-acceptance boundary. Results rendered, the
technical-preview footer boundary remained visible, and browser console errors
were absent.

TP-MAC-17 result-filter smoke passed on 2026-06-07 using the in-app browser at
`http://127.0.0.1:5174/`. The run confirmed mechanics execution renders
`result-filter-summary` as `647 of 647 results visible`; filtering by
`torsional-shear` narrows the visible set to `60 of 647 results visible`,
keeps the stress group and the end-j torsional shear row visible, and hides
the nonmatching force group. Clearing the filter restores `647 of 647 results
visible`, restores the force group, disables the clear control, retains the
technical-preview footer boundary, and produces no browser console errors.

TP-MAC-18 diagnostic-filter smoke passed on 2026-06-07 using the in-app
browser at `http://127.0.0.1:5174/`. The run confirmed mechanics execution
renders `diagnostic-filter-summary` as `9 of 9 diagnostics visible` with
`8 Warnings`, `1 Info`, `0 Errors`, and `0 Blocking`; filtering by
`HIGH_DISPLACEMENT_REVIEW` narrows the visible set to
`1 of 9 diagnostics visible`, keeps the high-displacement diagnostic visible,
and hides the nonmatching `RULE_CHECK_NOT_PERFORMED` diagnostic. Selecting the
filtered diagnostic retained affected refs for `result:disp:node-N-140` and
`node:N-140` and selected the `Terminal tie-in` context. Clearing the filter
restores `9 of 9 diagnostics visible`, restores `RULE_CHECK_NOT_PERFORMED`,
retains the technical-preview footer boundary, and produces no browser console
errors.

TP-MAC-19 report-diagnostic-scope smoke passed on 2026-06-07 using the in-app
browser at `http://127.0.0.1:5174/`. The run confirmed the report packet
exports the same full diagnostic review inventory used by the diagnostics
panel: model, design knowledge, and computed mechanics findings. The report
export summary showed `11 refs; 9 diagnostics; no private payload`; the report
diagnostic severity line showed `8 warnings; 1 info; 0 errors; 0 blocking`;
the parsed local JSON contained 9 diagnostic refs, including
`RULE_INPUTS_MISSING`, `SUPPORT_STIFFNESS_UNRESOLVED`, and
`diagnostic:physics:high-displacement-review`; `diagnostic_summary.total` was
9 with warning count 8 and info count 1. Private payload, protected content,
and release/professional claim flags remained false; the technical-preview
footer boundary remained visible; and browser console errors were absent.

TP-MAC-20 proposal-report-linkage smoke passed on 2026-06-07 using the in-app
browser at `http://127.0.0.1:5174/`. The run executed mechanics, selected
`diagnostic:physics:high-displacement-review`, generated the review-only
proposal, and confirmed the report packet rendered the selected review target,
`op:review-computed-diagnostic`, `attach_design_knowledge`,
`draft_user_review_required`, and `not_applied`. The report proposal-boundary
line showed review-only status, required user acceptance, no accepted-model
state mutation, and no compliance or professional approval claim. The parsed
local JSON export included `selected_review_target` for the diagnostic,
`proposal_ref` as `proposal:physics-diagnostic-review`, structured
`proposal_operation` fields for affected entity, validation, audit boundary,
and professional boundary, `diagnostic_summary.total` of 9, and false flags
for private payload, protected content, and release/professional claims.
Browser console errors were absent.

TP-MAC-21 state-run-audit smoke passed on 2026-06-07 using the in-app browser
at `http://127.0.0.1:5174/`. The pre-run app rendered the run-audit panel with
an empty state explaining that immutable model-state and analysis-run audit
references are generated by the mechanics preview. After `Run mechanics
preview`, the panel showed model state
`state:project:invented-loop-01:preview`, analysis run
`run:preview-linear-static-001`, statuses `HUMAN_REVIEW_REQUIRED`,
`MECHANICS_SOLVED`, and `RULE_INPUTS_INCOMPLETE`, 647 result rows, 647 result
value hashes, hash scopes `analysis_run_record` and `result_envelope`, input
manifest ref `result-envelope:run:preview-linear-static-001`, unresolved
reproducibility TBDs for physical project container and release-grade solver
build provenance, read-only/new-run-required immutability, and the
human-review/no-compliance-certification-sealing-authentication-approval
boundary. The parsed local report JSON included the same `run_audit` payload
and false private-payload, protected-content, and release/professional-claim
flags. Browser console errors were absent.

TP-MAC-22 proposal-runtime-target-parity smoke passed on 2026-06-07 using the
in-app browser at `http://127.0.0.1:5174/` plus focused Tauri unit coverage.
The Tauri command test
`agent_proposal_preserves_selected_review_target_without_mutation_or_claims`
confirmed `sample_agent_proposal` accepts the selected review target and emits
`diagnostic:physics:high-displacement-review` as the affected entity/change
target while keeping `not_applied`, `accepted_model_mutated=false`, accepted
model mutation false, and professional compliance/approval claims false. The
browser fallback smoke selected the same diagnostic, generated the review-only
proposal, and confirmed the proposal panel/report export preserved the
selected diagnostic target, proposal affected entity, audit mutation=false,
professional compliance claim=false, and false private-payload,
protected-content, and release/professional-claim flags. Browser console
errors were absent.

TP-MAC-23 result-pagination smoke passed on 2026-06-07 using the in-app
browser at `http://127.0.0.1:5174/`. After `Run mechanics preview`, the
results panel showed `647 of 647 results match filter` and page summary
`Showing 1 to 50 of 647 matching results; page 1 of 13`, with exactly 50
rendered result rows. Advancing to the next page showed
`Showing 51 to 100 of 647 matching results; page 2 of 13` and rendered the
expected `result:force:pipe-P-110:axial` row. Filtering by
`stress:pipe-P-120:end-j:torsional-shear` reset the page state, narrowed the
view to 3 rendered rows, selected the off-page stress result, and preserved
the selected review target in the report panel. The parsed local report JSON
still carried 11 selected result refs, a 647-row run-audit count, and false
private-payload, protected-content, and release/professional-claim flags.
Browser console errors were absent.

TP-MAC-24 model-tree-filter smoke passed on 2026-06-07 using the in-app
browser at `http://127.0.0.1:5174/`. The model tree initially reported
`19 of 19 model entities visible`. Filtering by `component:C-110` narrowed
the tree to one component row, hid unrelated material/load groups, and selected
`Invented elbow marker` in the inspector. Filtering by `carbon-steel-like`
narrowed the tree to the material row while preserving the prior component
selection until the material row was selected. Filtering by `no-match-token`
showed the empty state without clearing the material inspector selection, and
clearing the filter restored all 19 entities including Load Cases. Browser
console errors were absent.

TP-MAC-25 comparison-workspace smoke passed on 2026-06-07 using the in-app
browser at `http://127.0.0.1:5174/`. After `Run mechanics preview`, the
comparison workspace reported `load:L-100; 217 rows` against
`combination:C-OPER-ALT; 213 rows`, with `213 comparable pairs`,
`4 reference-only`, `0 target-only`, and tolerance status
`not_tolerance_checked; profile=TBD`. The mapping basis preserved stable result
IDs plus explicit `source_result_refs`. Selecting the
`result:combination:combination-C-OPER-ALT:moment:pipe-P-120:bending-z`
comparison row selected `Rack span` and recorded the selected review target in
the report packet. The parsed local report JSON carried comparison summary
fields with 213 mapped pairs, 4 unmatched reference rows, tolerance status
`not_tolerance_checked`, and false private-payload, protected-content,
release/professional-claim, and comparison compliance-claim flags. Browser
console errors were absent.

TP-MAC-26 editor-intent smoke passed on 2026-06-07 using the in-app browser at
`http://127.0.0.1:5174/`. The run selected
`material:invented-carbon-steel`, changed the review-only editor intent field
to `elastic_modulus.value`, entered a changed preview value, and queued
`op:editor-intent-material:invented-carbon-steel-elastic_modulus.value` as
`editor-intent-1`. The intent preview exposed `modify`, `proposed`,
`schema_validation=not_run`, `constraint_validation=not_run`,
`unit_validation=not_run`, `diff_preview_status=not_generated`, and
`application_status=not_applied`; the audit boundary preserved
`structured_operations_only`, `direct_model_mutation_allowed=false`,
`requires_user_acceptance=true`, and `mutates_accepted_model_state=false`.
Professional-boundary flags retained human-review-required posture and false
software compliance/approval claims. Browser console errors were absent.

TP-MAC-27 editor-intent-report-export smoke passed on 2026-06-07
America/Edmonton using the in-app browser at `http://127.0.0.1:5174/`
with timestamp marker `2026-06-08T03:45:05.491Z`. The run selected
`material:invented-carbon-steel`, queued
`op:editor-intent-material:invented-carbon-steel-elastic_modulus.value` as
`editor-intent-1`, ran mechanics preview, and confirmed the report packet
surfaced `1 queued`, the latest operation ID, `not_applied`, and the
review-only boundary stating user acceptance is required, accepted model state
is not mutated, and no compliance or professional approval claim is made. The
parsed local report JSON carried `editor_intent_refs`,
`editor_intent_summary.queued_count=1`, application status `not_applied`,
`mutates_accepted_model_state=false`, `direct_model_mutation_allowed=false`,
and the full exported editor operation intent with false private-payload,
protected-content, and release/professional-claim flags. Fresh browser console
errors after the timestamp marker were absent.

TP-MAC-28 local-editor-intent-persistence smoke passed on 2026-06-07
America/Edmonton using the in-app browser at `http://127.0.0.1:5174/`
with timestamp marker `2026-06-08T03:50:54.585Z`. The run began with
`0 pending operations; applied=false`, queued
`op:editor-intent-material:invented-carbon-steel-elastic_modulus.value` as
`editor-intent-1`, and confirmed the local project toolbar reported
`1 pending operation; applied=false` after Create local, Save local, and Open
local. Re-selecting `material:invented-carbon-steel` after open restored the
queued intent with `application_status=not_applied` and
`mutates_accepted_model_state=false`. After running mechanics preview, the
report packet still showed `1 queued`, the review-only/no-mutation/no-claim
editor intent boundary, and a parsed local JSON export with
`editor_intent_summary.queued_count=1`, status `not_applied`,
`direct_model_mutation_allowed=false`, false private-payload/protected-content
flags, and `release_or_professional_claim=false`. Fresh browser console errors
after the timestamp marker were absent.

TP-MAC-29 local-handoff-package smoke passed on 2026-06-07 America/Edmonton
using the in-app browser at `http://127.0.0.1:5174/` with timestamp marker
`2026-06-08T03:57:25.488Z`. The run began with the empty handoff package
state, queued
`op:editor-intent-material:invented-carbon-steel-elastic_modulus.value` as
`editor-intent-1`, ran mechanics preview, and confirmed the handoff panel
reported `19 entities`, `9 diagnostics`, `3 loss notes`, and
`1 pending operation`. The visible package rows preserved
`native_open_json_preview`, `stable_ids_only_not_target_specific`, 647 result
refs, the target-specific mapping loss note, and the boundary text for local
review handoff only, no private payload, no protected content, and no release
or professional claim. The parsed local package JSON carried
`document_kind=openpipestress.technical_preview.handoff_package`,
`export_scope=local_browser_download_preview`, deliverable refs
`DEL-15-01`, `DEL-15-02`, `DEL-15-03`, and `DEL-17-03`, model-state/run refs,
19 entity refs, 647 result refs, comparison summary, diagnostic refs,
target-specific unresolved assumptions, the queued editor intent with
`not_applied` status, and false private-payload, protected-content,
release/professional-claim, compliance-claim, and authentication-claim flags.
Fresh browser console errors after the timestamp marker were absent.

TP-MAC-30 operation-review-ledger smoke passed on 2026-06-07
America/Edmonton using the in-app browser at `http://127.0.0.1:5174/`
with timestamp marker `2026-06-08T04:05:32.577Z`. The run confirmed the empty
operation ledger state, selected `material:invented-carbon-steel`, queued
`op:editor-intent-material:invented-carbon-steel-elastic_modulus.value` as a
GUI editor intent, and verified the local ledger JSON reported
`document_kind=openpipestress.technical_preview.operation_review_ledger`, one
`gui_editor_intent_queue` record, `held_for_user_acceptance`, no explicit user
acceptance, affected entity `material:invented-carbon-steel`,
`application_status=not_applied`, accepted model state unchanged, and false
release/professional-claim flags. After running mechanics preview and
generating `proposal:physics-diagnostic-review`, the ledger reported
`2 review records`, `2 held`, `0 accepted`, and `0 rejected`. The parsed
combined export carried deliverable refs `DEL-16-01` through `DEL-16-04`,
scope items `SOW-069` and `SOW-070`, model-state/run refs, both record sources
(`gui_editor_intent_queue` and `agent_proposal`), proposal operation
`op:review-computed-diagnostic`, accepted model state unchanged, and false
private-payload, protected-content, release/professional-claim,
compliance-claim, and authentication-claim flags. Fresh browser console errors
after the timestamp marker were absent.

TP-MAC-31 export-safety-review smoke passed on 2026-06-07 America/Edmonton
using the in-app browser at `http://127.0.0.1:5174/` with timestamp marker
`2026-06-08T04:19:04.682Z`. The run confirmed the new Export Safety Review
panel initially reported `0 of 3 local exports ready`, protected-content
blocking enabled, and no release/professional claim. After selecting
`material:invented-carbon-steel`, queueing
`op:editor-intent-material:invented-carbon-steel-elastic_modulus.value`, and
running mechanics preview, the panel reported `3 of 3 local exports ready`,
state binding to `run:preview-linear-static-001`, and available records for
the report packet, handoff package, and operation review ledger. The parsed
local manifest JSON carried
`document_kind=openpipestress.technical_preview.export_review_manifest`,
deliverable refs including `DEL-12-02`, `DEL-08-04`, and `DEL-17-03`, scope
items `SOW-040` and `SOW-046`, `available_count=3`,
`operation_record_count=1`, 647 result refs for each export, protected-content
blocking, source-model-mutation disabled, private-data policy
`no_private_project_data`, and false private-payload, protected-content,
release/professional-claim, and compliance-claim flags. Fresh browser console
errors after the timestamp marker were absent.

TP-MAC-32 rule-check-completeness smoke passed on 2026-06-07
America/Edmonton using the in-app browser at `http://127.0.0.1:5174/`
with timestamp marker `2026-06-08T04:23:33.773Z`. The run confirmed the new
Rule-Check Completeness panel initially reported `4 review findings`,
`rule_check_blocked=true`, and `mechanics_reviewable=false`, with
`RULE_CHECK_BLOCKING` for missing user/private rule inputs and
`PROVENANCE_WARNING` for component flexibility-factor provenance. After
running mechanics preview, the panel reported `5 review findings`,
`mechanics_reviewable=true`, `RULE_INPUTS_INCOMPLETE`, and
`MECHANICS_SOLVED`, adding an `ASSUMPTION_WARNING` for missing professional
acceptance. The parsed local completeness JSON carried
`document_kind=openpipestress.technical_preview.rule_completeness_review`,
deliverable refs `DEL-06-03` and `DEL-07-04`, scope items `SOW-004` and
`SOW-022`, run ref `run:preview-linear-static-001`, warning classes
`RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, and `ASSUMPTION_WARNING`,
`silent_defaults_used=false`, `bundled_code_values_used=false`,
`compliance_claim_made=false`, and false private-payload, protected-content,
release/professional-claim, and compliance-claim flags. Fresh browser console
errors after the timestamp marker were absent.

TP-MAC-33 operation-diff-preview smoke passed on 2026-06-07
America/Edmonton using the in-app browser at `http://127.0.0.1:5174/`
with timestamp marker `2026-06-08T04:28:45.194Z`. The run confirmed the new
Operation Diff Preview panel initially reported no queued operation diffs and
explicitly stated that the panel does not apply operations. After selecting
`material:invented-carbon-steel` and queueing
`op:editor-intent-material:invented-carbon-steel-elastic_modulus.value`, the
panel reported `1 operations`, `1 diff rows`, `accepted_state_mutated=false`,
`0 hash-bound rows`, and `1 held`, with a local before/after row from
`200000000000` to `210000000000 Pa`. After running mechanics preview, the
parsed local diff JSON carried
`document_kind=openpipestress.technical_preview.operation_diff_preview`,
deliverable ref `DEL-16-02`, scope item `SOW-069`, model-state/run refs
`state:project:invented-loop-01:preview` and
`run:preview-linear-static-001`, `operation_count=1`, `diff_row_count=1`,
`accepted_model_state_mutated=false`, and `hash_bound_diff_preview_count=0`.
After selecting `result:stress:pipe-P-120:end-j:torsional-shear` and
generating `proposal:physics-diagnostic-review`, the combined parsed diff JSON
reported `operation_count=2`, `diff_row_count=2`, `held_for_user_acceptance`
for both previews, record sources `gui_editor_intent_queue` and
`agent_proposal`, proposal operation `op:review-computed-diagnostic`,
`generated_from_computed_context`, `not_applied`, no accepted-state mutation,
and false private-payload, protected-content, release/professional-claim,
compliance-claim, certification-claim, sealing-claim, approval-claim, and
authentication-claim flags. Fresh browser console errors after the timestamp
marker were absent.

TP-MAC-34 project-storage-audit smoke passed on 2026-06-07 America/Edmonton
using the in-app browser at `http://127.0.0.1:5174/` with timestamp marker
`2026-06-08T04:35:39.400Z`. The run confirmed the new Project Storage Audit
panel initially reported `operation=not_started`, `pending operations=0`,
`accepted_state_mutated=false`, browser memory preview storage,
`mode=not_persisted_this_session`, `network=false`, `daemon=false`,
`telemetry=false`, `repository_default_private_write=false`, no private
payload, no protected content, and no release/professional claim. The initial
parsed local storage JSON carried
`document_kind=openpipestress.technical_preview.local_project_persistence_audit`,
deliverable refs `DEL-02-05` and `DEL-12-01`, scope item `SOW-050`, no pending
operations, no accepted-state mutation, no repository-default private write,
and false private-payload, protected-content, and release/professional-claim
flags. After selecting `material:invented-carbon-steel` and queueing
`op:editor-intent-material:invented-carbon-steel-elastic_modulus.value`, the
toolbar and audit panel reported `1 pending operation; applied=false`. Create
local, Save local, and Open local then preserved the queued intent and reported
`operation=open`, `browser_memory_preview`, `copied_external_files=false`, and
`accepted_state_mutated=false`. The final parsed storage JSON carried
deliverable refs `DEL-02-05` and `DEL-12-02`, scope items `SOW-050` and
`SOW-040`, project ref `project:invented-loop-01`, `pending_operation_count=1`,
`applied_operation_count=0`, no network/daemon/telemetry requirement, the
queued editor intent with `not_applied` status, `local_only_project_store=true`,
no repository-default private write, no external file copy, no accepted-state
mutation, false private-payload, false protected-content,
false release/professional-claim, false compliance-claim, and false
authentication-claim flags. Fresh browser console errors after the timestamp
marker were absent.

TP-MAC-35 solve-job-audit smoke passed on 2026-06-07 America/Edmonton using
the in-app browser at `http://127.0.0.1:5174/` with timestamp marker
`2026-06-08T04:42:11.724Z`. The run confirmed the new Solve Job Audit packet
initially reported `state=not_started`, `events=1`, `result_rows=0`,
`cancellation_requested=false`, progress basis
`preview_service_event_state_only_no_percent_stream`,
`percentages_synthesized=false`, `backend_percent_stream_available=false`,
`token=TBD`, no direct solver-process mutation, and no cancellation-success
claim. After running mechanics preview, the panel reported `state=completed`,
`events=3`, `result_rows=647`, `cancellation_requested=false`,
`enabled=false`, `token=TBD`, `success_claimed=false`, model-state ref
`state:project:invented-loop-01:preview`, analysis-run ref
`run:preview-linear-static-001`, and `hashes=647`. The parsed local solve-job
JSON carried
`document_kind=openpipestress.technical_preview.solve_job_audit`, deliverable
refs `DEL-07-07`, `DEL-14-02`, and `DEL-04-06`, scope items `SOW-055`,
`SOW-072`, and `SOW-053`, `event_count=3`, `result_row_count=647`,
`diagnostic_count=7`, event states `queued`, `running`, and `completed`,
statuses `HUMAN_REVIEW_REQUIRED`, `MECHANICS_SOLVED`, and
`RULE_INPUTS_INCOMPLETE`, `result_hash_count=647`, hash scopes
`analysis_run_record` and `result_envelope`, false private-payload, false
protected-content, false release/professional-claim, false compliance-claim,
false certification-claim, false sealing-claim, false approval-claim, and
false authentication-claim flags. Fresh browser console errors after the
timestamp marker were absent.

TP-MAC-50 caepipe-mbf-export-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
smoke marker `caepipe-mbf-smoke-1780922356401`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, and `python3 tests/test_caepipe_mbf_export_package.py`
before browser smoke.

The initial browser state confirmed the new CAEPIPE MBF Export panel reported
`available`, `nodes=5`, `pipes=4`, `supports=3`,
`validation=boundary_checked`, and `losses=6`. The parsed CAEPIPE MBF package
JSON carried
`document_kind=openpipestress.technical_preview.caepipe_mbf_export_package`,
`DEL-17-04`, profile `ops.caepipe_mbf.smoke_tbd`,
`target_family=caepipe_mbf`, `target_version_basis=TBD-17-01-001`,
`record_subset_basis=TBD-17-01-002`,
`stable_id_policy=sidecar_mapping_until_direct_mbf_carrier_confirmed`, carried
TBD refs `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`, 5 nodes, 4
pipe elements, 3 supports, 2 load cases, 14 sidecar stable-ID entries, all six
loss categories (`exported`, `omitted`, `approximated`, `delegated`,
`unsupported`, and `tbd`), zero diagnostics, `validation_status=boundary_checked`,
and false CAEPIPE-compatibility, solver-validation, code-compliance, and
professional-reliance claim flags. The parsed MBF text download contained
`PIPE,P003,N003,N004`, `LOAD_CASES`, and terminal `END`. Export Safety Review
initially reported `10 of 16 local exports ready`, and Report Content Lint
reported `targets=9`, `findings=0`, and `blocking=0`.

After running the preview mechanics path, the CAEPIPE MBF package bound to
`analysis_run_ref=run:preview-linear-static-001` and
`model_state_ref=state:project:invented-loop-01:preview` while retaining
`validation=boundary_checked`, 14 stable-ID entries, 6 loss entries, and zero
diagnostics. Export Safety Review reported `15 of 16 local exports ready`
because no operation review record was queued; the parsed export review
manifest carried `export_count=16`, `available_count=15`, and
`caepipe_mbf_export:available`. The parsed report JSON persistence inventory
carried `expected_export_count=16`, `available_count=15`, and
`caepipe_mbf_export:available`; Report Content Lint reported `targets=10`,
`findings=0`, and `blocking=0`. Captured browser console error entries were
absent.

TP-MAC-36 result-export-audit smoke passed on 2026-06-07 America/Edmonton
using the in-app browser at `http://127.0.0.1:5174/` with timestamp marker
`2026-06-08T04:58:58.888Z`. The run confirmed the new Result Export panel
initially reported an empty schema-first local result envelope state and the
Export Safety Review panel reported `0 of 4 local exports ready`. After
running mechanics preview, the Result Export panel reported `available`,
`rows=647`, `sets=1`, and `diagnostics=7`, with format
`schema_first_json_result_envelope`, `additional_formats=TBD`, model/run
binding to `project:invented-loop-01` and `run:preview-linear-static-001`,
explicit unit/dimension coverage including `length` and `stress`,
`deterministic_ordering=true`, and `run_hashes=2`. The parsed local result
envelope carried `deliverable_id=DEL-08-04`, `package_id=PKG-08`,
`scope_item=SOW-046`, objectives `OBJ-007` and `OBJ-009`, one `mechanics`
result set with 647 values, seven diagnostics, nonempty load-basis refs,
analysis statuses `HUMAN_REVIEW_REQUIRED`, `MECHANICS_SOLVED`, and
`RULE_INPUTS_INCOMPLETE`, a redacted not-loaded user rule-pack ref with
`missing_required_inputs`, and `additional_export_formats=TBD` for downstream
use. The Export Safety Review manifest reported four local export slots and
three available exports without queued operations, including the available
`result_envelope` export. False private-payload, protected-content,
release/professional-claim, compliance-claim, certification-claim,
sealing-claim, approval-claim, and authentication-claim boundaries were
preserved. Fresh browser console errors after the timestamp marker were
absent.

TP-MAC-37 native-json-package smoke passed on 2026-06-07 America/Edmonton
using the in-app browser at
`http://127.0.0.1:5173/?smoke=tp-mac-37-1780895252691` with timestamp marker
`2026-06-08T05:07:32.691Z`. The initial state confirmed the Native JSON
Package panel reported an empty local review-manifest state and Export Safety
Review reported `0 of 5 local exports ready`. After running mechanics preview,
the Native JSON Package panel reported `ready`, `members=9`, `entities=19`,
`results=647`, and `operations=0`. The visible package profile preserved
`native_open_json_preview`, `physical_container=TBD`, and
`public_transport=TBD`; the validation line reported
`review_manifest_complete`, `deterministic_paths=true`, and
`model_hash=TBD_model_hash_not_available`; the loss report line reported one
`TBD` and one unsupported target-specific behavior; and the storage line
reported `network=false`, `telemetry=false`, and
`repository_default_private_write=false`. The parsed local package JSON carried
`document_kind=openpipestress.technical_preview.native_json_package_review`,
deliverable refs `DEL-17-02`, `DEL-17-03`, `DEL-02-05`, `DEL-12-01`,
`DEL-08-04`, and `DEL-14-02`, scope items `SOW-030`, `SOW-074`, `SOW-050`,
`SOW-029`, `SOW-046`, and `SOW-072`, nine package member paths including
`manifest.json`, `maps/stable_id_map.json`, and
`results/result_envelope_ref.json`, `runtime_timestamp_fields_in_hash_inputs=false`,
19 entity refs including `project:invented-loop-01`, 647 result refs including
`result:force:pipe-P-120:axial`, zero operation refs, seven diagnostics, two
run hash refs, `TBD_model_hash_not_available`, and
`TBD_canonical_package_hash_service_not_available`. Export Safety Review
reported five local export slots and four available exports without queued
operations, including the available `native_json_package` export. False
private-payload, protected-content, release/professional-claim,
compatibility-claim, code-compliance-claim, and software compliance-claim
boundaries were preserved. Fresh browser console errors after the timestamp
marker were absent.

TP-MAC-38 native-package-proposal-refs smoke passed on 2026-06-07
America/Edmonton using the in-app browser at
`http://127.0.0.1:5173/?smoke=tp-mac-38-1780895522105` with timestamp marker
`2026-06-08T05:12:02.105Z`. The run started from the Native JSON Package
empty state and `0 of 5 local exports ready`, ran mechanics preview, selected
`result:stress:pipe-P-120:end-j:torsional-shear`, generated the deterministic
review proposal, and confirmed the Native JSON Package panel updated from
`operations=0` to `operations=1`. The parsed local native package JSON carried
`operation_ref_count=1`, operation ref `op:review-computed-diagnostic`,
proposal ref `proposal:physics-diagnostic-review`, `record_count=1`,
`editor_intent_count=0`, `proposal_count=1`,
`held_for_user_acceptance_count=1`, `accepted_count=0`, `rejected_count=0`,
selected review target `result:stress:pipe-P-120:end-j:torsional-shear`,
`accepted_model_state_mutated=false`, `operation_application_status=not_applied`,
`requires_user_acceptance=true`, `preview_records_do_not_apply_operations=true`,
and `direct_model_mutation_allowed=false`. The native validation checks
included `review-only operation refs declared when present`. Export Safety
Review reported `5 of 5 local exports ready`, with the `native_json_package`
and `operation_review_ledger` exports available and `operation_record_count=1`.
False private-payload, protected-content, release/professional-claim,
software compliance-claim, and software approval-claim boundaries were
preserved. Fresh browser console errors after the timestamp marker were
absent.

TP-MAC-39 project-validation-preflight smoke passed on 2026-06-07
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-39-1780896094525` with timestamp marker
`2026-06-08T05:21:34.525Z`. The run started from
`validation=preview_not_persisted`, selected the invented material, queued one
review-only elastic-modulus editor intent, and completed create, save, and open
through the local browser-preview project controls. The Project Validation
Preflight panel then reported `validation=preview_current`,
`version=supported_current_schema`, `migration=current`, and
`round_trip=semantic_categories_declared`; its operation line reported
`validate=preflight_generated_preview_current`,
`version_check=supported_current_schema`,
`migrate=not_run_migration_framework_tbd`, and `pending operations=1`. The
parsed validation JSON carried
`document_kind=openpipestress.technical_preview.project_validation_preflight`,
`storage_mode=browser_memory_preview`, `last_operation=open`,
`pending_operation_count=1`, `accepted_model_state_mutated=false`, six
round-trip categories, ready preview round-trip status for model content, unit
metadata, load payloads, and provenance metadata,
`rule_pack_references:not_present_in_invented_fixture`, and
`reproducibility_metadata:hash_basis_declared_hash_service_tbd`. Local-only
storage, no repository-default private writes, no external file copy, no
network, no telemetry, false private-payload, protected-content,
release/professional-claim, and software compliance-claim boundaries were
preserved. Fresh browser console errors after the timestamp marker were absent.

TP-MAC-40 export-review-persistence-inventory smoke passed on 2026-06-07
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-40-1780896608564` with timestamp marker
`2026-06-08T05:30:08.564Z`. The run confirmed Export Safety Review initially
reported `2 of 7 local exports ready`, with `project_storage_audit` and
`project_validation_preflight` both available before a mechanics run. After
running the preview mechanics path, the solve job reported `state=completed`,
`events=3`, and `result_rows=647`; Export Safety Review then reported
`6 of 7 local exports ready`. The parsed manifest carried `export_count=7`,
`available_count=6`, `diagnostics_reviewed=9`, `operation_record_count=0`,
and export readiness entries for `project_storage_audit:available`,
`project_validation_preflight:available`, `result_envelope:available`,
`native_json_package:available`, `report_packet:available`,
`handoff_package:available`, and
`operation_review_ledger:empty_operation_queue`. The manifest included
`DEL-02-05`, `DEL-12-01`, and `DEL-12-02` plus `SOW-050`, `SOW-041`,
`SOW-029`, and `SOW-040`; the storage audit entry remained
`last_operation=not_started`, `storage_mode=not_persisted_this_session`, and
`copied_external_files=false`, while the validation preflight entry remained
`validation_status=preview_not_persisted`,
`version_check_status=supported_current_schema`, and
`round_trip_status=semantic_categories_declared`. False private-payload,
protected-content, release/professional-claim, and software compliance-claim
boundaries were preserved. Fresh browser console errors after the timestamp
marker were absent.

TP-MAC-41 report-persistence-evidence smoke passed on 2026-06-07
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-41-1780896920000` with timestamp marker
`2026-06-08T05:35:20.000Z`. The run loaded the desktop preview with local
browser-preview storage reporting `network=false`, `daemon=false`, and
`telemetry=false`, then ran the preview mechanics path. The solve job reported
`state=completed`, `events=3`, and `result_rows=647`; Export Safety Review
reported `6 of 7 local exports ready`. The Report Packet panel reported
`storage=not_persisted_this_session`, `validation=preview_not_persisted`,
`round_trip=semantic_categories_declared`, `6 of 7 local exports ready`,
`storage=available`, `validation=available`, `network=false`,
`telemetry=false`, `private/protected payload=false`, and
`accepted_state_mutated=false`. The parsed local report JSON carried
`document_kind=openpipestress.technical_preview.report_packet_export`,
deliverable refs `DEL-08-01`, `DEL-08-03`, `DEL-08-04`, `DEL-08-06`,
`DEL-02-05`, `DEL-12-01`, and `DEL-12-02`; its
`persistence_evidence` carried
`openpipestress.technical_preview.report_persistence_export_context`,
`openpipestress.technical_preview.local_project_persistence_audit`,
`storage_mode=not_persisted_this_session`,
`openpipestress.technical_preview.project_validation_preflight`,
`validation_status=preview_not_persisted`,
`round_trip_status=semantic_categories_declared`, `expected_export_count=7`,
`available_count=6`, and
`operation_review_ledger=empty_operation_queue`. False network, telemetry,
private-payload, protected-content, release/professional-claim, and accepted
state mutation boundaries were preserved. Fresh browser console errors after
the timestamp marker were absent.

TP-MAC-42 report-content-lint-preview smoke passed on 2026-06-07
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-42-1780897360326` with timestamp marker
`2026-06-08T05:42:40.326Z`. The initial state confirmed the new Report
Content Lint panel reported `available`, `targets=3`, `findings=0`,
`blocking=0`, `explicit_public_surfaces`,
`private=skip_unless_explicitly_authorized`, `clearance=false`,
`heuristic_only=true`, `legal=false`, `professional=false`, `ci=TBD`, and
`redaction=TBD`; Export Safety Review initially reported
`3 of 8 local exports ready`. The parsed lint JSON carried
`deliverable_id=DEL-08-05`, `scope_item=SOW-043`, objectives `OBJ-002` and
`OBJ-007`, `heuristic_only=true`, `legal_clearance=false`,
`professional_approval=false`, `ci_release_policy=TBD`, `target_count=3`,
`finding_count=0`, `blocking_finding_count=0`,
`clean_scan_is_clearance=false`, and
`private_surface_default=skip_unless_explicitly_authorized`. After running the
preview mechanics path, the solve job reported `state=completed`, `events=3`,
and `result_rows=647`; the lint panel reported `targets=4`, `findings=0`,
and `blocking=0`, with target IDs `target:desktop-report-template`,
`target:desktop-export-review-template`, `target:invented-preview-fixture`,
and `target:report-packet-preview-json`, and `private_targets=0`. Export
Safety Review reported `7 of 8 local exports ready`; the parsed export review
manifest carried `export_count=8`, `available_count=7`, and an available
`report_protected_content_lint` export with
`document_kind=openpipestress.technical_preview.report_protected_content_lint_run`,
deliverable refs `DEL-08-05`, `DEL-08-01`, `DEL-08-03`, `DEL-08-06`, and
`DEL-12-02`, `target_count=4`, `finding_count=0`,
`blocking_finding_count=0`, `clean_scan_is_clearance=false`, and false
private-payload, protected-content, and release/professional-claim flags. The
parsed report JSON persistence inventory carried `expected_export_count=8`,
`available_count=7`, and
`report_protected_content_lint:available`. Fresh browser console errors after
the timestamp marker were absent.

TP-MAC-43 headless-runner-envelope-preview smoke passed on 2026-06-07
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-43-1780897981511` with timestamp marker
`2026-06-08T05:54:35Z`. The initial state confirmed the new Headless Runner
panel reported `available`, `job=TBD`, `outputs=4`, `result_refs=1`,
`schema_first_headless_runner_contract`, `cli=TBD`, `scripts=TBD`,
`process=TBD`, `network=TBD`, `filesystem=TBD`, `network_required=false`,
`direct_sql=false`, and no compliance, certification, sealing,
authentication, or approval claim. The parsed runner JSON carried
`deliverable_id=DEL-10-05`, scope items `SOW-054` and `SOW-032`, objectives
`OBJ-008`, `OBJ-009`, and `OBJ-012`, `operation=solve`, requested outputs
`result_envelope`, `audit_manifest`, `diagnostics`, and
`regression_record`, `job_state=TBD`, analysis statuses `MODEL_INCOMPLETE`
and `HUMAN_REVIEW_REQUIRED`, one placeholder result ref, one `TBD` checksum,
`network_required=false`, `direct_sql=false`, `process_invocation=TBD`,
`telemetry=false`, and `software_makes_compliance_claim=false`. Export Safety
Review initially reported `4 of 9 local exports ready` with
`headless_runner_envelope` included in the nine-slot manifest.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, and `result_rows=647`; the Headless Runner
panel reported `job=COMPLETED`, `outputs=4`, `result_refs=647`, job progress
`3/3`, `cancel=false`, `schema_first_json_result_envelope`,
`audit=audit-manifest:run:preview-linear-static-001:preview`, and
`checksums=2`. The parsed runner JSON carried
`run_id=run:preview-linear-static-001`, `job_state=COMPLETED`, result envelope
ref `result-envelope:run:preview-linear-static-001`, 647 result refs, two
checksums, seven diagnostics, analysis statuses `HUMAN_REVIEW_REQUIRED`,
`MECHANICS_SOLVED`, and `RULE_INPUTS_INCOMPLETE`, `network_required=false`,
`direct_sql=false`, `process_invocation=TBD`, `telemetry=false`, and false
compliance/authentication claims. Export Safety Review reported `8 of 9 local
exports ready`; the parsed export review manifest carried `export_count=9`,
`available_count=8`, an available `headless_runner_envelope` export with
`document_kind=openpipestress.technical_preview.headless_runner_envelope`,
`result_ref_count=647`, `runner_job_state=COMPLETED`, and final CLI/process/
network/filesystem fields still `TBD`. The parsed report JSON persistence
inventory carried `expected_export_count=9`, `available_count=8`, and
`headless_runner_envelope:available`; report lint remained at `targets=4`,
`findings=0`, `clean_scan_is_clearance=false`. Fresh browser console errors
after the timestamp marker were absent.

TP-MAC-44 adapter-framework-envelope-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-44-1780918779041` with timestamp marker
`2026-06-08T11:40:33Z`. The initial state confirmed the new Adapter Framework
panel reported `available`, `capabilities=4`, `parse=not_parsed_by_framework`,
`diagnostics=1`, `schema_first_format_neutral_adapter_framework`,
`formats=TBD`, `transport=TBD`, capabilities `export_model`,
`export_results`, `validate_payload`, and `contribution_review`,
`schema=required`, `units=required`, `protected=required`,
`export=required_before_shared_payload`, `direct_sql=false`,
`network=false`, `code_compliance_claim=false`, `plugin=TBD`,
`endpoint=TBD`, `package_scripts=TBD`, `local_first=true`,
`telemetry=false`, `private_redacted=true`, and no compliance, certification,
sealing, approval, or security-certification claim. The parsed adapter JSON
carried `deliverable_id=DEL-10-02`, `scope_item=SOW-030`,
`objective=OBJ-009`, `interface_kind=schema_first_format_neutral_adapter_framework`,
`external_format_list=TBD`, `public_transport_protocol=TBD`,
`plugin_runtime=TBD`, four declared capabilities, one initial diagnostic, one
`TBD` checksum, no direct SQL, no network access, no code-compliance claim,
`telemetry_allowed=false`, and
`software_makes_security_certification_claim=false`. Export Safety Review
initially reported `5 of 10 local exports ready` with
`adapter_framework_envelope` included in the ten-slot manifest.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, and `result_rows=647`; the Adapter Framework
panel reported `diagnostics=7`, retained `parse=not_parsed_by_framework`,
and retained required schema/unit/protected-content/export-review validation.
The parsed adapter JSON carried
`operation_id=adapter-operation:desktop-preview:run-preview-linear-static-001`,
`operation_class=export`, `parse_status=not_parsed_by_framework`, seven
diagnostics, two checksums, audit manifest ref
`audit-manifest:run:preview-linear-static-001:preview`, result envelope ref
`result-envelope:run:preview-linear-static-001`, no direct SQL, no network
access, no arbitrary-code execution, `private_payload_redacted=true`, and
false compliance and security-certification claims. Export Safety Review
reported `9 of 10 local exports ready`; the parsed export review manifest
carried `export_count=10`, `available_count=9`, an available
`adapter_framework_envelope` export with
`document_kind=openpipestress.technical_preview.adapter_framework_envelope`,
deliverable refs `DEL-10-02`, `DEL-02-04`, `DEL-08-04`, `DEL-12-01`, and
`DEL-12-02`, `result_ref_count=647`, `external_format_list=TBD`,
`public_transport_protocol=TBD`, and `plugin_runtime=TBD`. The parsed report
JSON persistence inventory carried `expected_export_count=10`,
`available_count=9`, and `adapter_framework_envelope:available`. Fresh
browser console errors after the timestamp marker were absent.

TP-MAC-45 external-prover-boundary-metadata-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-45-1780919355205` with timestamp marker
`2026-06-08T11:49:15.202Z`. The initial state confirmed the new External
Prover Boundary panel reported `available`,
`metadata=non_authoritative_workflow_metadata`, `refs=1`, and
`diagnostics=4`; the parsed metadata JSON carried `DEL-15-04`, `SOW-075`,
objectives `OBJ-017` and `OBJ-018`, tag `run-context-pending`, external ref
`external:desktop-preview-metadata-only`, `handoff:TBD`, `state:TBD`, one
attachment reference with `payload_embedded=false`, three unsupported-target
flags, and false external-tool invocation, commercial-result ingestion,
software external-validation record, and software compliance claim flags.
Export Safety Review initially reported `6 of 11 local exports ready`, with
`external_prover_boundary_metadata:available`, and Report Content Lint reported
`targets=4`, `findings=0`, `blocking=0`.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, and `result_rows=647`; the external-prover
metadata JSON carried tag `mechanics-context-bound`, handoff ref
`handoff:project:invented-loop-01:run:preview-linear-static-001`, model-state
ref `state:project:invented-loop-01:preview`, `sha256` hash references, the
same three unsupported-target flags, and false external-tool invocation,
commercial-result ingestion, and software-created professional-reliance record
flags. Export Safety Review reported `10 of 11 local exports ready`; the parsed
manifest carried `export_count=11`, `available_count=10`, and an available
`external_prover_boundary_metadata` export with `DEL-15-04`, `DEL-15-01`,
`DEL-15-02`, `DEL-15-03`, `DEL-17-05`, one external reference, one attachment,
three unsupported-target flags, and false private-payload, protected-content,
release/professional-claim, external-tool invocation, commercial-result
ingestion, and software external-validation record flags. The parsed report
JSON persistence inventory carried `expected_export_count=11`,
`available_count=10`, and `external_prover_boundary_metadata:available`; Report
Content Lint reported `targets=5`, `findings=0`, and `blocking=0`. Fresh browser
console errors after the timestamp marker were absent.

TP-MAC-46 review-geometry-export-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-46-1780920002685` with timestamp marker
`2026-06-08T12:00:02.685Z`. The initial state confirmed the new Review
Geometry Export panel reported `available`, `format=glTF_2_0_json_preview`,
`segments=4`, `nodes=5`, and `stable_ids=19`; the parsed review geometry JSON
carried `document_kind=openpipestress.technical_preview.review_geometry_export`,
`DEL-17-08`, scope items `SOW-030` and `SOW-074`, objectives `OBJ-009` and
`OBJ-017`, status `visual_review_geometry_only`, glTF `asset.version=2.0`,
line primitive mode `1`, point primitive mode `0`, 216 bytes of embedded
buffer data, 19 sidecar stable-ID rows, `pipe:P-120` as emitted,
`support:S-120` as approximated, a `TBD` GLB/viewer-behavior loss entry, and
false private-payload, protected-content, solver-geometry-equivalence,
professional-validation, and target-compatibility claim flags. Export Safety
Review initially reported `7 of 12 local exports ready`, with
`review_geometry_export:available`, and Report Content Lint reported
`targets=5`, `findings=0`, `blocking=0`.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, and `result_rows=647`; the review geometry JSON
bound to `analysis_run_ref=run:preview-linear-static-001`,
`model_state_ref=state:project:invented-loop-01:preview`, and
`result_ref=run:preview-linear-static-001`, while retaining 216 bytes of
embedded buffer data, 19 stable-ID rows, and false private/protected payload,
solver-geometry-equivalence, and professional-validation flags. Export Safety
Review reported `11 of 12 local exports ready`; the parsed manifest carried
`export_count=12`, `available_count=11`, and an available
`review_geometry_export` record with source refs `run:preview-linear-static-001`,
`state:project:invented-loop-01:preview`, and `project:invented-loop-01`. The
parsed report JSON persistence inventory carried `expected_export_count=12`,
`available_count=11`, `review_geometry_export:available`, and
`operation_review_ledger:empty_operation_queue`; Report Content Lint reported
`targets=6`, `findings=0`, and `blocking=0`. Fresh browser console errors after
the timestamp marker were absent.

TP-MAC-47 stress-neutral-csv-json-export-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T12:10:39.510Z`. The initial state confirmed the
Stress-neutral CSV/JSON panel was present but pending a mechanics run, Report
Content Lint reported `targets=6`, `findings=0`, and `blocking=0`, and Export
Safety Review reported `7 of 13 local exports ready`.

After running the preview mechanics path, the stress-neutral panel reported
`available`, `rows=647`, `csv_columns=11`, and `diagnostics=2`. The parsed JSON
download carried
`document_kind=openpipestress.technical_preview.stress_neutral_csv_json_package`,
`DEL-17-06`, `PKG-17`, 647 result rows, 647 stable-ID mappings, 11 CSV columns,
`validation_status=passed`,
`schema_validation_status=desktop_preview_shape_aligned_not_runtime_json_schema_validated`,
`canonical_package_hash_status=TBD_browser_preview_does_not_emit_canonical_package_hash`,
three loss-report entries, diagnostics `SN-DESKTOP-PREVIEW-HASH-TBD` and
`SN-DESKTOP-PREVIEW-COMPARISON-TBD`, and false private-payload,
protected-content, vendor-format, solver-validation, code-compliance, and
professional-reliance claim flags. The parsed CSV download had header
`result_id,canonical_ref,row_kind,result_family,load_case_ref,station_ref,component_ref,value,unit,dimension,correlation_status`,
648 lines including the header, and contained `result:force:pipe-P-120:axial`.
Export Safety Review reported `12 of 13 local exports ready`, and Report
Content Lint reported `targets=7`, `findings=0`, and `blocking=0`. Fresh browser
console errors after the timestamp marker were absent.

TP-MAC-48 build-package-readiness-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T12:19:11.798Z`. The local validation run for this
tranche passed `npm test --workspace apps/desktop`, `npm run build --workspace
apps/desktop`, and
`python3 tools/release/check_release_readiness.py --profile skeleton --execute`
before browser smoke.

The initial browser state confirmed the new Build/Package Readiness panel
reported `available`, `scripts=8`, `tauri=present`, `bundle_active=false`, and
`diagnostics=4`. The parsed readiness JSON carried
`document_kind=openpipestress.technical_preview.build_package_readiness`,
`DEL-10-04`, `PKG-10`, scope item `SOW-032`, root scripts `dev:desktop`,
`build:desktop`, `test:desktop`, and `generate:product-preview-mechanics`,
desktop scripts `dev`, `build`, `test`, and `tauri`, Tauri
`before_build_command=npm run build`, readiness profiles `skeleton`, `python`,
`security`, `cargo`, and `all`, `browser_panel_runs_tool=false`, unresolved
`ci_provider=TBD`, `release_matrix=TBD`, and `signing_notarization=TBD`, and
false network, telemetry, release-publication, installer/binary generation,
code-compliance, professional-reliance, private-payload, protected-content,
and release/professional-claim flags. Export Safety Review initially reported
`8 of 14 local exports ready`, and Report Content Lint reported `targets=7`,
`findings=0`, and `blocking=0`.

After running the preview mechanics path, the result export reported
`rows=647`; Build/Package Readiness remained available with `scripts=8`,
`tauri=present`, and `bundle_active=false`; Export Safety Review reported
`13 of 14 local exports ready` because no operation review record was queued;
and Report Content Lint reported `targets=8`, `findings=0`, and `blocking=0`.
Fresh browser console errors after the timestamp marker were absent.

TP-MAC-49 conservative-pcf-export-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T12:28:39.901Z`. The local validation run for this
tranche passed `npm test --workspace apps/desktop`, `npm run build --workspace
apps/desktop`, and `python3 tests/test_pcf_export_package.py` before browser
smoke.

The initial browser state confirmed the new Conservative PCF Export panel
reported `available`, `segments=4`, `nodes=5`,
`validation=blocked_missing_explicit_pcf_target_fields`, and `losses=6`.
The parsed PCF package JSON carried
`document_kind=openpipestress.technical_preview.conservative_pcf_export_package`,
`DEL-17-07`, profile `ops.pcf.conservative_subset`, `target_family=pcf`,
`target_profile_version_basis=TBD`, 5 nodes, 4 pipe segments, 4 sidecar
stable-ID entries, all six loss categories (`exported`, `omitted`,
`approximated`, `delegated`, `unsupported`, and `tbd`), diagnostics
`PCF-NOMINAL-SIZE-TBD`, `PCF-SUPPORT-SEMANTICS-UNSUPPORTED`,
`PCF-COMPONENT-COVERAGE-APPROXIMATED`, and `PCF-TARGET-PROFILE-TBD`, and false
private-payload, protected-content, target-compatibility, solver-validation,
code-compliance, and professional-reliance claim flags. The parsed PCF text
download contained `COMPONENT-IDENTIFIER OPS-PIPE-P-120`,
`TBD_SOURCE_REQUIRED`, and terminal `END-ISOGEN`. Export Safety Review initially
reported `9 of 15 local exports ready`, and Report Content Lint reported
`targets=8`, `findings=0`, and `blocking=0`.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, and `result_rows=647`; the PCF package bound to
`analysis_run_ref=run:preview-linear-static-001` and
`model_state_ref=state:project:invented-loop-01:preview` while retaining
`validation=blocked_missing_explicit_pcf_target_fields`, 4 stable-ID entries,
and 6 loss entries. Export Safety Review reported `14 of 15 local exports
ready` because no operation review record was queued; the parsed export review
manifest carried `export_count=15`, `available_count=14`, and
`conservative_pcf_export:available`. The parsed report JSON persistence
inventory carried `expected_export_count=15`, `available_count=14`, and
`conservative_pcf_export:available`; Report Content Lint reported `targets=9`,
`findings=0`, and `blocking=0`. Fresh browser console errors after the
timestamp marker were absent.

TP-MAC-51 export-adapter-sdk-registry-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T12:49:12.722Z`. The local validation run for this
tranche passed `npm test --workspace apps/desktop`, `npm run build --workspace
apps/desktop`, `git diff --check`, and a touched-file ASCII scan before
closeout.

The initial browser state confirmed the new Export Adapter SDK panel reported
`available`, `targets=5`, `capabilities=4`, `validation=boundary_checked`, and
`diagnostics=0`. The parsed registry JSON carried
`document_kind=openpipestress.technical_preview.export_adapter_sdk_registry`,
`DEL-17-09`, `PKG-17`, registry
`ops.export_adapter_sdk.registry_preview`, `target_count=5`,
`admitted_support_claim_count=0`, `plugin_runtime=TBD`,
`permission_taxonomy=TBD`, denied filesystem and network grants, zero
diagnostics, and false target-compatibility, solver-validation,
code-compliance, and professional-reliance claim flags. Export Safety Review
initially reported `11 of 17 local exports ready`, with
`export_adapter_sdk_registry:available`, and Report Content Lint reported
`targets=10`, `findings=0`, and `blocking=0`.

After running the preview mechanics path, the Export Adapter SDK registry bound
to `analysis_run_ref=run:preview-linear-static-001` and
`model_state_ref=state:project:invented-loop-01:preview` while retaining
`validation=boundary_checked`, 5 registry targets, 0 admitted support claims,
0 diagnostics, and the same no-compatibility, no-solver-validation,
no-code-compliance, and no-professional-reliance boundaries. Export Safety
Review reported `16 of 17 local exports ready` because no operation review
record was queued; the parsed export review manifest carried
`export_count=17`, `available_count=16`, and
`export_adapter_sdk_registry:available`. The parsed report JSON persistence
inventory carried `expected_export_count=17`, `available_count=16`, and
`export_adapter_sdk_registry:available`; Report Content Lint reported
`targets=11`, `findings=0`, and `blocking=0`. Captured browser console error
entries were absent.

TP-MAC-52 caepipe-external-harness-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T12:58:19.278Z`. The local validation run for this
tranche passed `npm test --workspace apps/desktop`, `npm run build --workspace
apps/desktop`, `python3 tests/test_caepipe_external_run_package.py`,
`git diff --check`, and a touched-file ASCII scan before closeout.

The initial browser state confirmed the new CAEPIPE External Harness panel
reported `available`, `status=parser_only_evidence`, `rows=3`, `sections=2`,
`external_invoked=false`, and `diagnostics=0`. The parsed harness JSON carried
`document_kind=openpipestress.technical_preview.caepipe_external_run_package`,
`DEL-17-05`, `PKG-17`, MBF reference
`caepipe-mbf:desktop-preview-del-17-04:project-invented-loop-01`, absent
executable configuration, false license and environment acknowledgements,
`invocation=TBD-17-05-invocation-profile`, `mode=parser_only_not_invoked`,
`attempted=false`, 2 parser coverage sections, 3 parsed CSV rows, 0 diagnostics,
`validation_status=boundary_checked`, and false CAEPIPE-compatibility,
solver-validation, code-compliance, professional-reliance, private-payload, and
protected-content claim flags. The CSV download contained
`NODE_DISPLACEMENTS`, `ELEMENT_FORCES`, and canonical ID map correlation rows.
Export Safety Review initially reported `12 of 18 local exports ready`, and
Report Content Lint reported `targets=11`, `findings=0`, and `blocking=0`.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, and `result_rows=647`; the CAEPIPE external
harness bound to `analysis_run_ref=run:preview-linear-static-001` and
`model_state_ref=state:project:invented-loop-01:preview` while retaining
`status=parser_only_evidence`, `external_invoked=false`, and 0 diagnostics.
Export Safety Review reported `17 of 18 local exports ready` because no
operation review record was queued; the parsed export review manifest carried
`available_count=17`, `DEL-17-05`, and
`caepipe_external_run_evidence:available` with parser row count 3, parser
section count 2, canonical ID map correlation, and false compatibility and
code-compliance claim flags. The parsed report JSON persistence inventory
carried `expected_export_count=18`, `available_count=17`, and
`caepipe_external_run_evidence:available`; the operation review ledger remained
empty; Report Content Lint reported `targets=12`, `findings=0`, and
`blocking=0`. Captured browser console error entries were absent.

TP-MAC-53 local-fea-handoff-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T13:14:15.629Z`. The local validation run for this
tranche passed `npm test --workspace apps/desktop`, `npm run build --workspace
apps/desktop`, `python3 tests/test_local_fea_handoff_contract.py`,
`git diff --check`, and a touched-file ASCII scan before closeout.

The initial browser state confirmed the new Local FEA Handoff panel remained
pending until a mechanics run, with target-neutral local shell/solid FEA
wording and explicit advisory/TBD framing. Export Safety Review initially
reported `12 of 19 local exports ready`; the parsed local FEA export-review
record was `pending_mechanics_run`, carried `DEL-10-03`, `SOW-031`, and
`SOW-049`, and retained `concrete_export_format=TBD`,
`target_solver_adapter=TBD`, `mesh_generation=TBD`, and
`external_solver_invocation=TBD`. Report Content Lint reported `targets=12`,
`findings=0`, and `blocking=0`.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. The Local FEA Handoff panel reported
`available`, `labels=4`, `flags=5`, `diagnostics=4`, `mesh=false`, and
`solver=false`, bound to `state:project:invented-loop-01:preview` and
`result-envelope:run:preview-linear-static-001`, selected
`pipe:P-120` and `pipe:P-130` by `diagnostic_suggested`, and used
`transfer_method_label=result_reference_only` with 3 load references. The
parsed handoff JSON carried `DEL-10-03`, `PKG-10`, `OBJ-009`, `SOW-031`,
`SOW-049`, `package_kind=local_shell_solid_fea_handoff`,
`global_model_kind=centerline_frame_global_analysis`, 5 result IDs, 4 guidance
labels, 5 unsupported behavior flags including
`mesh_generation_not_performed`, `target_format_not_selected`, and
`external_solver_not_invoked`, and 4 diagnostics including
`LOCAL-FEA-EXTERNAL-SOLVER-NOT-INVOKED`. Privacy and professional boundaries
remained false for telemetry, private payload embedding, compliance,
certification, sealing, approval, and authentication claims, with human review
required.

Export Safety Review reported `18 of 19 local exports ready` because no
operation review record was queued, with `local_fea_handoff_package:available`
and 19 total export records. The report UI readiness line reported
`18 of 19 local exports ready`, `storage=available`, and
`validation=available`, while the report packet download included `DEL-10-03`
and retained false private/protected payload flags. Report Content Lint reported
`targets=13`, `findings=0`, and `blocking=0`, with
`target:desktop-local-fea-handoff-template` present. Captured browser console
error entries were absent.

TP-MAC-54 editor-contract-review-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T08:40:24-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `python3 tests/test_gui_editors_contract.py`,
`git diff --check`, and a touched-file ASCII scan before closeout.

The initial browser state confirmed the new Editor Contract panel reported
`available`, `editors=4`, `surfaces=6`, `diagnostics=4`, and
`private_payload=false`. The surface coverage included material review editors,
component review editors, load-case and support operation-intent routes,
rule-pack reference metadata, and private-library reference slots without
private payload copying. The parsed editor-contract JSON carried `DEL-07-03`,
`PKG-07`, `SOW-021`, `OBJ-006`,
`document_kind=openpipestress.technical_preview.editor_contract_review`, 4
editors, 6 surfaces, 17 fields, 2 ready editors, 2 blocked editors, 4
diagnostics, and 0 queued intents. The rule-pack reference remained
`rule-pack:user-supplied:not-loaded` with checksum
`TBD_private_rule_pack_not_loaded`, private/protected payload redaction, and
missing required rule inputs. Source-of-truth, privacy, and professional
boundaries remained false for direct model mutation, accepted-state mutation,
private payload inclusion, protected content inclusion, release claims,
code-compliance claims, approval, authentication, certification, and sealing.

Export Safety Review initially reported `13 of 20 local exports ready`, with
`editor_contract_review:available`, `queued_intent_count=0`, 4 editor records,
6 editor surfaces, 2 ready editors, 2 blocked editors, 4 diagnostics, private
library reference-only status, and false direct/accepted model mutation flags.
Report Content Lint initially reported `targets=13`, `findings=0`, and
`blocking=0`, with `target:desktop-editor-contract-template` present.

After selecting the invented material row, changing the label through the
review-only editor intent form, and queueing it, the Editor Contract panel
reported `queued=1`. The editor-contract JSON carried
`queued_operation_intent_refs=["op:editor-intent-material:invented-carbon-steel-label"]`,
while the export-review editor record carried `queued_intent_count=1` and
source ref `editor-intent-1`. After running the preview mechanics path, the
solve job reported `state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. Export Safety Review reported `20 of 20 local
exports ready`; the report packet persistence inventory carried
`expected_export_count=20`, `available_count=20`, and
`editor_contract_review:available`; the report editor intent summary carried
`queued_count=1`, `mutates_accepted_model_state=false`, and
`direct_model_mutation_allowed=false`; and Report Content Lint reported
`targets=14`, `findings=0`, and `blocking=0`. Captured browser console error
entries were absent.

TP-MAC-55 missing-data-blocking-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T08:51:04-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `python3 tests/test_missing_data_warning_ux.py`,
`git diff --check`, and a touched-file ASCII scan before closeout.

The initial browser state confirmed the new Missing Data Blocking panel
reported `available`, `classes=6`, `active=5`, `solve_blocked=false`,
`rule_blocked=true`, and `private_payload=false`. Class coverage included
`SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`,
`ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING`; the
inactive preview classes remained available rather than hidden. The status
separation line preserved `mechanics=ready_for_preview_diagnostics`,
`rule_check=not_performed_user_rule_inputs_missing`, and
`mechanics_reviewable=false`. The parsed warning JSON carried `DEL-07-04`,
`PKG-07`, `SOW-022`, `OBJ-006`, `OBJ-011`, 6 warning classes, 5 active warning
records, 0 solve-blocking records, 2 rule-check-blocking records, explicit
GUI and local warning-class status maps, text fields for class, severity,
affected refs, message, and remediation, and `color_only=false`.

The initial warning packet and export-review record retained
`silent_defaults_used=false`, `auto_fill_missing_data=false`,
`direct_model_mutation_allowed=false`, `accepted_model_state_mutated=false`,
`private_payload_included=false`, `protected_content_included=false`, and no
release, professional, approval, authentication, certification, sealing, or
code-compliance claim. Export Safety Review initially reported `14 of 21 local
exports ready` with `missing_data_warning_blocking_review:available`; Report
Content Lint initially reported `targets=14`, `findings=0`, and `blocking=0`,
with `target:desktop-missing-data-template` present.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. The Missing Data Blocking panel preserved 5
active warning records and updated status separation to
`mechanics=MECHANICS_SOLVED`, `rule_check=RULE_INPUTS_INCOMPLETE`,
`mechanics_reviewable=true`, and `mechanics_qualified=true`, keeping
`solve=0` and `rule=2` in the blocking summary. Export Safety Review reported
`20 of 21 local exports ready` because no operation review record was queued;
the report packet persistence inventory carried `expected_export_count=21`,
`available_count=20`, and `missing_data_warning_blocking_review:available`;
and Report Content Lint reported `targets=15`, `findings=0`, and `blocking=0`.
Captured browser console error entries were absent.

TP-MAC-56 telemetry-boundary-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T10:11:52-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/security/test_telemetry_policy.py`, `git diff --check`, and a
touched-file ASCII scan before closeout.

The initial browser state confirmed the new Telemetry Boundary panel reported
`disabled=true`, `config=absent_or_preview_config_resolves_disabled`,
`blocked=4`, `payload=false`, and `network=false`. The parsed telemetry JSON
carried `DEL-12-03`, `PKG-12`, `SOW-037`, `OBJ-010`,
`document_kind=openpipestress.technical_preview.telemetry_boundary_review`, 4
attempted events, 0 allowed events, 4 blocked events, explicit opt-in false,
allowlist approval false, product config schema `TBD`, payload construction
false, network transport false, endpoint false, vendor false, upload queue
false, telemetry persistence false, private payload false, protected content
false, and security-certification claim false. The no-bypass surface list
included reports, plugins, adapters, import/export, private libraries, CLI, and
diagnostics.

Export Safety Review initially reported `15 of 22 local exports ready` with
`telemetry_boundary_review:available`; the export-review record preserved
telemetry disabled, 4 blocked event attempts, payload construction false,
network initialization false, endpoint/vendor/persistence false, and no
security certification claim. Report Content Lint initially reported
`targets=15`, `findings=0`, and `blocking=0`, with
`target:desktop-telemetry-boundary-template` present.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. The Telemetry Boundary panel remained
`disabled=true`, kept 4 blocked event attempts, and kept payload/network false.
Export Safety Review reported `21 of 22 local exports ready` because no
operation review record was queued; the report packet persistence inventory
carried `expected_export_count=22`, `available_count=21`, and
`telemetry_boundary_review:available`; the report readiness line included
`telemetry=available`; and Report Content Lint reported `targets=16`,
`findings=0`, and `blocking=0`. Captured browser console error entries were
absent.

TP-MAC-57 secret-private-library-boundary-preview smoke passed on 2026-06-08
America/Edmonton using local Chrome/Playwright against
`http://127.0.0.1:5175/` with timestamp marker
`2026-06-08T10:24:39-0600 MDT`. The local validation run for this tranche
passed `npm test --workspace apps/desktop`, `npm run build --workspace
apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/security/test_secret_private_library_handling.py`, browser initial and
solved-state packet checks, zero browser console errors, `git diff --check`,
and a touched-file ASCII scan before closeout.

The initial browser state confirmed the new Secret & Private Libraries panel
reported `records=4`, `metadata_only=true`, `public_fixture_blocked=4`,
`secrets=false`, and `private_payload=false`. The parsed secret/private JSON
carried `DEL-12-04`, `PKG-12`, `SOW-040`, `SOW-029`, `OBJ-010`,
`document_kind=openpipestress.technical_preview.secret_private_library_boundary_review`,
4 references, 2 private library/rule-pack references, 1 symbolic private-path
reference, 1 credential reference, public fixture block count 4, public report
metadata-only count 4, local-private metadata-only count 4, secret material
false, direct SQL false, storage bypass false, private payload false,
protected content false, exact secret provider `TBD`, encrypted storage
default `TBD`, and security-certification claim false.

Export Safety Review initially reported `16 of 23 local exports ready` with
`secret_private_library_boundary_review:available`; Report Content Lint
initially reported `targets=16`, `findings=0`, and `blocking=0`, with
`target:desktop-secret-private-library-template` present. The favicon data URL
prevented the browser from issuing a missing `/favicon.ico` resource error.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. The Secret & Private Libraries panel retained
the same metadata-only/no-secret boundary. Export Safety Review reported
`22 of 23 local exports ready` because no operation review record was queued;
the report packet persistence inventory carried `expected_export_count=23`,
`available_count=22`, and
`secret_private_library_boundary_review:available`; the report readiness line
included `secrets=available`; and Report Content Lint reported `targets=17`,
`findings=0`, and `blocking=0`. Captured browser console error entries were
absent.

TP-MAC-58 security-threat-model-preview smoke passed on 2026-06-08
America/Edmonton using local Chrome/Playwright against
`http://127.0.0.1:5175/` with timestamp marker
`2026-06-08T10:37:18-0600 MDT`. The local validation run for this tranche
passed `npm test --workspace apps/desktop`, `npm run build --workspace
apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_security_threat_model.py`, browser initial and solved-state packet
checks, zero browser console errors, `git diff --check`, and a touched-file
ASCII scan before closeout.

The initial browser state confirmed the new Threat Model Review panel reported
`threats=19`, `high=14`, `local_first=true`, `private_payload=false`, and
`security_certification=false`. The parsed threat-model JSON carried
`DEL-12-05`, `PKG-12`, `SOW-040`, `OBJ-010`,
`document_kind=openpipestress.technical_preview.security_threat_model_review`,
19 threat rows, 14 high-risk rows, 5 medium/high rows, 10 asset classes, 10
trust boundaries, 6 export workflow reviews, 14 open decisions, 13 `TBD`
decisions, no direct SQL access, no plugin manifest runtime grant, private
payload false, protected content false, and no security-certification claim.

Export Safety Review initially reported `17 of 24 local exports ready` with
`security_threat_model_review:available`; Report Content Lint initially
reported `targets=17`, `findings=0`, and `blocking=0`, with
`target:desktop-security-threat-model-template` present.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. Export Safety Review reported `23 of 24 local
exports ready` because no operation review record was queued; the report
packet persistence inventory carried `expected_export_count=24`,
`available_count=23`, and `security_threat_model_review:available`; the report
readiness line included `threats=available`; and Report Content Lint reported
`targets=18`, `findings=0`, and `blocking=0`. Captured browser console error
entries were absent.

TP-MAC-59 accessibility-usability-baseline-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T10:48:45-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_accessibility_usability_baseline.py`, browser initial and
solved-state packet checks, zero browser console errors, `git diff --check`,
and a touched-file ASCII scan before closeout.

The initial browser state confirmed the new Accessibility Baseline panel
reported `surfaces=6`, `findings=69`, `warnings=10`,
`target=TBD_by_human_project_authority`, `conformance_claim=false`,
`color_only=false`, and `private_payload=false`. The parsed accessibility JSON
carried `DEL-07-06`, `PKG-07`, `SOW-036`, `OBJ-006`,
`document_kind=openpipestress.technical_preview.accessibility_usability_baseline_review`,
6 source surfaces, 69 findings, 57 pass records, 10 warning records, 1 fail
record, 1 blocking record, target `TBD_by_human_project_authority`, desktop
runtime evaluation `not_performed`, private payload false, protected content
false, color-only status signaling false, and no accessibility conformance,
release, code-compliance, or professional reliance claim.

Export Safety Review initially reported `18 of 25 local exports ready` with
`accessibility_usability_baseline_review:available`; Report Content Lint
initially reported `targets=18`, `findings=0`, and `blocking=0`, with
`target:desktop-accessibility-baseline-template` present.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. Export Safety Review reported `24 of 25 local
exports ready` because no operation review record was queued; the report
packet persistence inventory carried `expected_export_count=25`,
`available_count=24`, and
`accessibility_usability_baseline_review:available`; the report readiness line
included `accessibility=available`; and Report Content Lint reported
`targets=19`, `findings=0`, and `blocking=0`. Captured browser console error
entries were absent.

TP-MAC-60 design-authoring-workspace-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T11:02:04-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_design_authoring_comparison_workspace.py`, browser initial and
solved-state packet checks, zero browser console errors, `git diff --check`,
and a touched-file ASCII scan before closeout.

The initial browser state confirmed the new Design-authoring Workspace panel
reported `knowledge=3`, `states=0`, `runs=0`, `comparisons=0`, and
`operations=0`. The core-contract line reported `records=2`, `warnings=1`,
`states=2`, `runs=2`, and `overlays=5`. The parsed workspace JSON carried
`DEL-07-08`, `PKG-07`, `SOW-076`, `OBJ-015`, `OBJ-016`,
`document_kind=openpipestress.technical_preview.design_authoring_comparison_workspace`,
current knowledge record count 3, pending state/run browser, pending
comparison tables, accepted model mutation false, private payload false,
protected content false, and no release or professional claim.

Export Safety Review initially reported `19 of 26 local exports ready` with
`design_authoring_comparison_workspace:available`; Report Content Lint
initially reported `targets=19`, `findings=0`, and `blocking=0`, with
`target:desktop-design-workspace-template` present.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. The Design-authoring Workspace panel reported
`knowledge=3`, `states=1`, `runs=1`, `comparisons=213`, and `operations=0`;
the current browser line carried `analysis_run=run:preview-linear-static-001`
and `result_rows=647`; and the parsed workspace JSON reported available
state/run browsing, available comparison tables, comparison
`comparison:run:preview-linear-static-001:load-L-100-to-combination-C-OPER-ALT`,
accepted model mutation false, private payload false, protected content false,
and no release or professional claim. Export Safety Review reported `25 of 26
local exports ready` because no operation review record was queued; the report
packet persistence inventory carried `expected_export_count=26`,
`available_count=25`, and
`design_authoring_comparison_workspace:available`; the report readiness line
included `workspace=available`; and Report Content Lint reported `targets=20`,
`findings=0`, and `blocking=0`. Captured browser console error entries were
absent.

TP-MAC-61 validation-release-evidence-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T11:12:56-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_release_readiness_script.py`, browser initial and solved-state
packet checks, zero browser console errors, `git diff --check`, and a
touched-file ASCII scan before closeout.

The initial browser state confirmed the new Validation Evidence panel reported
`manual=10`, `evidence=8`, `profiles=5`, and `open_tbd=5`. The manual line
reported 10 sections and states `PLANNED`, `DRAFT_EVIDENCE`,
`MAINTAINER_REVIEWED`, `BLOCKED`, and `TBD`; the inventory line reported
`areas=8`, `gui=PLANNED`, and `bundles=TBD`; the release-check line reported
profiles `skeleton`, `python`, `security`, `cargo`, and `all`, with
`skeleton_checks=2` and `dry_run_default=true`; and the release-gate line
reported solver/rule-engine/GUI/report-template/mixed gate families with
`thresholds=TBD` and `release_authorized=false`.

The parsed validation-evidence JSON carried
`document_kind=openpipestress.technical_preview.validation_release_evidence_review`,
`DEL-09-04`, `DEL-09-05`, `DEL-10-04`, `DEL-09-01`, `DEL-09-02`,
`DEL-09-03`, `DEL-08-05`, scope items `SOW-026`, `SOW-027`, `SOW-032`, and
`SOW-043`, 10 manual sections, 8 evidence areas, 5 release profiles, 7
required release paths, 2 skeleton checks, 5 open validation decisions, and 5
release-gate families. It recorded
`latest_dag_dependency_edges=execution/_DAG/DAG-006/DependencyEdges.csv`,
`browser_panel_runs_tool=false`, `release_publication_authorized=false`,
`private_payload_included=false`, `protected_content_included=false`, and
`release_or_professional_claim=false`.

Export Safety Review initially reported `20 of 27 local exports ready` with
`validation_release_evidence_review:available`; Report Content Lint initially
reported `targets=20`, `findings=0`, and `blocking=0`, with
`target:desktop-validation-evidence-template` present.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. Result Export reported `rows=647`.
Export Safety Review reported `26 of 27 local exports ready` because no
operation review record was queued; the report packet persistence inventory
carried `expected_export_count=27`, `available_count=26`, and
`validation_release_evidence_review:available`; the report readiness line
included `evidence=available`; and Report Content Lint reported `targets=21`,
`findings=0`, and `blocking=0`. Captured browser console error entries were
absent.

TP-MAC-62 viewport-operation-queue-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T11:31:14-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_viewport_editor_contract.py tests/test_gui_editors_contract.py
tests/test_model_operation_schema.py tests/test_operation_validation_preview.py
tests/test_operation_audit_trail.py
tests/test_design_authoring_comparison_workspace.py`, direct
`PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_operation_schema.py`,
browser queue and solved-state packet checks, responsive canvas dimension
checks, zero browser console errors, `git diff --check`, and a touched-file
ASCII scan before closeout.

The initial browser state confirmed the viewport editor intent actions were
available with one `Node intent`, one `Pipe-run intent`, and one `Component
intent` control. The local project review context reported `0 pending
operations`, the editor-contract panel reported `editors=4`, `surfaces=7`,
and `diagnostics=4`, the editor coverage included
`viewport_gestures:operation_intent_editor_available`, Report Content Lint
reported `targets=21`, and the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-62`.

After queuing the three viewport intents, the local project review context
reported `3 pending operations`. The viewport cards recorded
`create_node`, `connect_pipe_run`, and `insert_component_symbol` as
`pending_service_validation`, `unit_aware_domain_validation_required`, and
`does_not_mutate_persisted_project_payload`, with queue IDs
`editor-intent-1`, `editor-intent-2`, and `editor-intent-3`. The parsed
editor-contract JSON reported `queued_intent_count=3`, `surface_count=7`, and
operation refs `op:viewport-intent-create_node-001`,
`op:viewport-intent-connect_pipe_run-002`, and
`op:viewport-intent-insert_component_symbol-003`.

The operation diff preview reported `3 operations`, `3 diff rows`, and
`3 held`, with every preview carrying `accepted_model_state_mutated=false`.
The operation review ledger reported 3 records, 3
`held_for_user_acceptance` decisions, source
`apps/desktop/src/features/viewport/PipeViewport.tsx`, actor role
`viewport_editor`, `accepted_model_state_unchanged=true`, and no release or
professional claim. Export Safety Review reported `21 of 27 local exports
ready` before solve, with `operation_review_ledger:available`, `DEL-07-01`,
`SOW-020`, and `operation_record_count=3`.

After running the preview mechanics path, the solve job reported
`state=completed`, `events=3`, `result_rows=647`, and
`cancellation_requested=false`. The report packet reported `27 of 27` local
exports ready, `operation_review_ledger:available`, `queued_count=3`, latest
operation `op:viewport-intent-insert_component_symbol-003`, latest source
role `viewport_editor`, accepted-state mutation false, and no release or
professional claim. Report Content Lint reported `targets=22`, `findings=0`,
and `blocking=0`, with `target:desktop-viewport-editor-template` present.
Export Safety Review reported `27 of 27` exports ready and
`operation_record_count=3`. Captured browser console error entries were
absent.

Responsive viewport checks confirmed a rendered Three.js canvas with no WebGL
fallback on both the default desktop-sized viewport and a 390 by 844
mobile-sized viewport. The desktop canvas measured 578 by 325 CSS pixels, and
the mobile-sized canvas measured 458 by 258 CSS pixels after the viewport
canvas min-height/aspect-ratio fix.

TP-MAC-63 operation-review-queue-clear smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T11:40:56-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_model_operation_schema.py tests/test_operation_validation_preview.py
tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py
tests/test_design_authoring_comparison_workspace.py`, direct
`PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_operation_schema.py`,
browser review-queue clear checks, zero browser console errors,
`git diff --check`, and a touched-file ASCII scan before closeout.

The browser state confirmed the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-63`. After queuing the three viewport
intents, the local project review context reported `3 pending operations`,
the operation review ledger reported `3 review records`, the operation diff
preview reported `3 operations` and `3 diff rows`, and Export Safety Review
reported `operation_record_count=3` with `operation_review_ledger:available`.

After choosing `Clear pending operations`, the viewport intent list returned
to its no-direct-mutation empty state, local project review context reported
`0 pending operations`, the operation ledger reported no structured
operations queued, the diff preview reported no operation diffs, and Export
Safety Review reported `operation_record_count=0` with
`operation_review_ledger:empty_operation_queue`.

After running the preview mechanics path, filtering to
`result:stress:pipe-P-120:end-j:torsional-shear`, and generating the
review-only proposal, the operation ledger reported `1 review record`, the
diff preview reported `1 operations`, the report export carried
`proposal:physics-diagnostic-review` and operation
`op:review-computed-diagnostic`, the native package carried one operation
ref, and Export Safety Review reported one operation record.

After choosing `Clear pending operations` again, the proposal body was removed,
the operation ledger and diff preview returned to empty-queue states, the
report export carried `proposal_ref=not generated` and
`proposal_operation=null`, the selected review target remained
`result:stress:pipe-P-120:end-j:torsional-shear`, the native package reported
`operation_ref_count=0` and `proposal_count=0`, and Export Safety Review
reported `operation_record_count=0` with
`operation_review_ledger:empty_operation_queue`. Captured browser console
error entries were absent.

TP-MAC-64 unified-review-operation-count-preview smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T11:46:50-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_model_operation_schema.py tests/test_operation_validation_preview.py
tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py
tests/test_design_authoring_comparison_workspace.py`, direct
`PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_operation_schema.py`,
browser unified review-operation count checks, zero browser console errors,
`git diff --check`, and a touched-file ASCII scan before closeout.

The initial browser state confirmed the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-64`. The local project review context
reported `0 pending operations; applied=false; editor_intents=0;
agent_proposals=0`, and both Project Storage Audit and Project Validation
Preflight JSON reported `pending_operation_count=0` and
`proposal_operation_count=0`.

After running the preview mechanics path, filtering to
`result:stress:pipe-P-120:end-j:torsional-shear`, and generating the
review-only proposal, the local project review context reported
`1 pending operation; applied=false; editor_intents=0; agent_proposals=1`.
Project Storage Audit reported `pending operations=1` and `proposals=1`, with
JSON `editor_intent_count=0`, `proposal_operation_count=1`, and proposal ref
`proposal:physics-diagnostic-review`. Project Validation Preflight reported
the same pending/proposal split. The report packet persistence evidence carried
`pending_operation_count=1` and `proposal_operation_count=1`, and Export Safety
Review reported `operation_record_count=1` while its `project_storage_audit`
and `project_validation_preflight` records each carried one pending proposal
operation.

After choosing `Clear pending operations`, the proposal body was removed, the
local project review context returned to `0 pending operations; applied=false;
editor_intents=0; agent_proposals=0`, Project Storage Audit and Project
Validation Preflight returned to `pending_operation_count=0` and
`proposal_operation_count=0`, proposal refs were empty, and Export Safety
Review reported `operation_review_ledger:empty_operation_queue`. Captured
browser console error entries were absent.

TP-MAC-65 proposal-local-project-roundtrip smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T11:57:00-0600 MDT`. The local validation run for
this tranche passed `npm test --workspace apps/desktop`, `npm run build
--workspace apps/desktop`, `cargo test --manifest-path
apps/desktop/src-tauri/Cargo.toml`, `PYTHONDONTWRITEBYTECODE=1 python3 -m
pytest tests/test_model_operation_schema.py tests/test_operation_validation_preview.py
tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py
tests/test_design_authoring_comparison_workspace.py`, and direct
`PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_operation_schema.py`
before closeout.

The initial browser state confirmed the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-65`. After running preview mechanics,
selecting `result:stress:pipe-P-120:end-j:torsional-shear`, generating
`proposal:physics-diagnostic-review`, and choosing `Save local`, the local
project review context reported `1 pending operation; applied=false;
editor_intents=0; agent_proposals=1`. Project Storage Audit reported
`operation=save`, `pending operations=1`, `proposals=1`, and snapshot
`persisted_proposals=1`; its JSON carried `pending_operation_count=1`,
`proposal_operation_count=1`, `persisted_proposal_count=1`,
`project_summary.proposal_count=1`, and proposal ref
`proposal:physics-diagnostic-review`. Project Validation Preflight carried the
same proposal and persisted-proposal counts, and Export Safety Review reported
`operation_record_count=1`.

After choosing `Open local`, the proposal body still rendered
`proposal:physics-diagnostic-review`, the affected entity remained
`result:stress:pipe-P-120:end-j:torsional-shear`, and the mechanics job state
returned to `state=not_started` without clearing the pending proposal. The
local project review context still reported `1 pending operation; applied=false;
editor_intents=0; agent_proposals=1`. Project Storage Audit reported
`operation=open`, `pending operations=1`, `proposals=1`, and
`persisted_proposals=1`; Project Validation Preflight reported
`pending operations=1`, `proposals=1`, and `persisted proposals=1`. The
Operation Review Ledger reported `1 review record` with state binding
`not generated`, and Export Safety Review kept `operation_record_count=1`
with `project_storage_audit`, `project_validation_preflight`, and
`operation_review_ledger` all carrying the expected proposal/persisted counts
or available readiness. Captured browser console error entries were absent.

TP-MAC-66 selected-review-target-local-roundtrip smoke passed on 2026-06-08
America/Edmonton using the in-app browser at `http://127.0.0.1:5175/` with
timestamp marker `2026-06-08T12:06:06-0600 MDT`. The local validation run for
this tranche passed focused
`npm test --workspace apps/desktop -- --run App.test.tsx -t "round trips
review-only proposal operations through local save and open"`, full
`npm test --workspace apps/desktop`, `npm run build --workspace apps/desktop`,
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`,
`PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_model_operation_schema.py tests/test_operation_validation_preview.py
tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py
tests/test_design_authoring_comparison_workspace.py`, and direct
`PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_operation_schema.py`
before browser smoke.

The initial browser state confirmed the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-66`. After running preview mechanics,
selecting `result:stress:pipe-P-120:end-j:torsional-shear`, generating
`proposal:physics-diagnostic-review`, and choosing `Save local`, Project
Storage Audit and Project Validation Preflight both reported persisted review
target count `1` in the visible text and exported JSON. After choosing
`Open local`, the proposal body still rendered, the selected review target
still showed `result: result:stress:pipe-P-120:end-j:torsional-shear`, the
proposal affected entity still referenced the same result, and the mechanics
job state returned to `state=not_started`. Operation Review Ledger exported
`selected_review_target={target_type: "result", id:
"result:stress:pipe-P-120:end-j:torsional-shear"}` at both ledger and record
level. Export Safety Review carried the same selected review target and
`persisted_selected_review_target_count=1` on the project storage and project
validation records. Captured browser console error entries were absent.

TP-MAC-67 persisted-editor-intent-count-local-roundtrip smoke passed on
2026-06-08 America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/` with timestamp marker `2026-06-08T12:15:22-0600 MDT`.
The local validation run for this tranche passed focused
`npm test --workspace apps/desktop -- --run App.test.tsx -t "round trips local
create, save, and open project controls without external file copies"`, full
`npm test --workspace apps/desktop`, `npm run build --workspace apps/desktop`,
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`,
`PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_model_operation_schema.py tests/test_operation_validation_preview.py
tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py
tests/test_design_authoring_comparison_workspace.py
tests/security/test_local_first_storage_policy.py`, and direct
`PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_operation_schema.py`
before browser smoke.

The initial browser state confirmed the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-67`. After selecting
`material:invented-carbon-steel`, queueing a review-only
`elastic_modulus.value` editor intent, and choosing `Create local`, `Save
local`, then `Open local`, the local project review context reported `1
pending operation; applied=false; editor_intents=1`. Project Storage Audit
visible text reported `operation=open` and `persisted_editor_intents=1`; its
JSON carried `summary.editor_intent_count=1`,
`summary.persisted_editor_intent_count=1`, and
`project_summary.editor_intent_count=1` with `local_only_project_store=true`
and `external_file_copy_performed=false`. Project Validation Preflight visible
text reported `persisted editor intents=1`; its JSON carried the same
`editor_intent_count`, `persisted_editor_intent_count`, and
`project_summary.editor_intent_count` values. Export Safety Review carried
`persisted_editor_intent_count=1` on both the `project_storage_audit` and
`project_validation_preflight` records. Re-selecting the material after open
showed the restored `editor-intent-1` queue entry. Captured browser console
error entries were absent.

TP-MAC-68 persisted-selected-review-target-ref-local-roundtrip smoke passed on
2026-06-08 America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/` with timestamp marker `2026-06-08T12:23:50-0600 MDT`.
The local validation run for this tranche passed focused
`npm test --workspace apps/desktop -- --run App.test.tsx -t "round trips
review-only proposal operations through local save and open"`, full
`npm test --workspace apps/desktop`, `npm run build --workspace apps/desktop`,
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`,
`PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_model_operation_schema.py tests/test_operation_validation_preview.py
tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py
tests/test_design_authoring_comparison_workspace.py
tests/security/test_local_first_storage_policy.py`, and direct
`PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_operation_schema.py`
before browser smoke.

The initial browser state confirmed the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-68`. After running preview mechanics,
selecting `result:stress:pipe-P-120:end-j:torsional-shear`, generating
`proposal:physics-diagnostic-review`, and choosing `Save local`, Project
Storage Audit and Project Validation Preflight visible text both carried
`result: result:stress:pipe-P-120:end-j:torsional-shear` as the persisted
selected review target ref. Their JSON packets carried
`persisted_selected_review_target_count=1`; both
`persisted_selected_review_target_ref` and
`project_summary.selected_review_target_ref` equaled
`result: result:stress:pipe-P-120:end-j:torsional-shear`. The saved report
packet persistence evidence carried the same target ref on both the storage
audit and validation preflight records. After choosing `Open local`, Project
Storage Audit and Project Validation Preflight still reported `operation=open`
and the same persisted selected-review-target ref, and Export Safety Review
carried it on both the `project_storage_audit` and
`project_validation_preflight` records while the restored
`selected_review_target` object retained
`result:stress:pipe-P-120:end-j:torsional-shear`. Private payload, protected
content, release, and professional-claim flags stayed false where checked.
Captured browser console error entries were absent.

TP-MAC-69 native-package-persisted-review-context smoke passed on 2026-06-08
America/Edmonton using the in-app browser at
`http://127.0.0.1:5175/?smoke=tp-mac-69-1780943398019` with timestamp marker
`2026-06-08T18:29:57.988Z`. The local validation run for this tranche passed
focused `npm test --workspace apps/desktop -- --run App.test.tsx -t "round
trips review-only proposal operations through local save and open|renders the
engineering workspace from invented local fixtures"`, full
`npm test --workspace apps/desktop`, `npm run build --workspace apps/desktop`,
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`,
`PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/test_model_operation_schema.py tests/test_operation_validation_preview.py
tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py
tests/test_design_authoring_comparison_workspace.py
tests/security/test_local_first_storage_policy.py`, and direct
`PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_operation_schema.py`.

The initial browser state confirmed the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-69`. After running preview mechanics,
selecting `result:stress:pipe-P-120:end-j:torsional-shear`, generating
`proposal:physics-diagnostic-review`, and choosing `Save local`, the Native
JSON Package panel reported `ready`, `members=9`, `entities=19`,
`results=647`, and `operations=1`. Its visible persisted review-context line
reported `editor_intents=0`, `proposals=1`, `selected_targets=1`, and
`selected_ref=result: result:stress:pipe-P-120:end-j:torsional-shear`. The
parsed native package JSON carried the same persisted review-context fields in
`source_project.storage_summary`, `operation_review.persisted_review_context`,
and `generation_context`, and included the validation check `persisted local
project review-context summary carried when available`. The package preserved
`selected_review_target` as the selected stress result, kept
`accepted_model_state_mutated=false`, `network_required=false`,
`telemetry_enabled=false`, and retained false private-payload,
protected-content, release/professional-claim, code-compliance-claim, and
software-approval-claim flags. Captured browser console error entries after
the timestamp marker were absent.

TP-MAC-70 persisted-mechanics-run-local-roundtrip smoke passed on 2026-06-09
America/Edmonton using the in-app browser at `http://127.0.0.1:5173/` with
timestamp marker `2026-06-10T03:00:29.813Z`. The local validation run for this
tranche passed focused and full `npm test --workspace apps/desktop`,
`npm run build --workspace apps/desktop`,
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`, and
`PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
tests/security/test_local_first_storage_policy.py
tests/test_model_operation_schema.py`.

The initial browser state confirmed the validation-evidence packet recorded
`current_tranche_smoke_record=TP-MAC-70`. After running preview mechanics,
selecting `result:stress:pipe-P-120:end-j:torsional-shear`, generating
`proposal:physics-diagnostic-review`, and choosing `Save local`, the Project
Storage Audit packet carried `persisted_mechanics_result_count=1`,
`persisted_analysis_run_count=1`, and
`persisted_analysis_run_ref=run:preview-linear-static-001` on
`project_summary`, alongside the persisted selected-review-target ref
`result: result:stress:pipe-P-120:end-j:torsional-shear`. After choosing
`Open local`, the saved mechanics run was restored instead of cleared: the
solve panel reported `state=completed`, `events=1`, `result_rows=647`, and the
progress basis `restored_persisted_run_record_no_new_solve_executed`,
declaring that the run record was restored from the local project store and
that no new solve executed in this session. The restored results re-rendered
all 647 result rows including
`result:stress:pipe-P-120:end-j:torsional-shear` and the displacement result
group after clearing the result filter, the operation review ledger state
binding reported `state:project:invented-loop-01:preview;
run:preview-linear-static-001`, the review context still reported `1 pending
operation; applied=false; editor_intents=0; agent_proposals=1`, and the
Project Storage Audit packet carried the same persisted mechanics-run fields
with `operation=open`. The restored run is review context only; it is not a
new computation, release claim, professional approval, certification,
sealing, authentication, or code-compliance claim. Captured browser console
error entries after the timestamp marker were absent.

TP-MAC-71 persisted-mechanics-run-panel-exposure smoke passed on 2026-06-09
America/Edmonton using the in-app browser at `http://127.0.0.1:5173/` with
timestamp marker `2026-06-10T03:14:16.045Z`. The local validation run for this
tranche passed `npm test --workspace apps/desktop`,
`npm run build --workspace apps/desktop`,
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib`, and the
full `python3 -m pytest -q tests` suite (340 passed), which includes
`tests/security/test_local_first_storage_policy.py` and
`tests/test_model_operation_schema.py`.

The validation-evidence packet captured during the session recorded
`current_tranche_smoke_record=TP-MAC-71`. The initial browser state confirmed
that before any persistence the
Project Storage Audit visible snapshot line and the Project Validation
Preflight visible persistence-operations line both rendered the new
persisted-run fields at their defaults: `persisted_mechanics_results=0`,
`persisted_analysis_runs=0`, and `persisted_analysis_run_ref=not_persisted`.
After running preview mechanics (`state=completed`, `result_rows=647`) and
choosing `Save local`, both visible lines reported
`persisted_mechanics_results=1`, `persisted_analysis_runs=1`, and
`persisted_analysis_run_ref=run:preview-linear-static-001`. After choosing
`Open local`, the restored run kept the solve panel at `state=completed`,
`events=1`, `result_rows=647` with the progress basis
`restored_persisted_run_record_no_new_solve_executed`, and the same
persisted-run values remained visible in both panel lines and in the Project
Storage Audit packet summary. The Export Safety Review manifest entries for
`project_storage_audit` and `project_validation_preflight` and the report
persistence-evidence `storage_audit` and `validation_preflight` blocks each
carried `persisted_mechanics_result_count=1`, `persisted_analysis_run_count=1`,
and `persisted_analysis_run_ref=run:preview-linear-static-001`. The exposed
persisted-run context is local technical-preview review evidence only; it is
not a release claim, professional approval, certification, sealing,
authentication, or code-compliance claim. Captured browser console error and
warning entries after the timestamp marker were absent.

TP-MAC-72 native-package-persisted-run-exposure smoke passed on 2026-06-09
America/Edmonton using the in-app browser at `http://127.0.0.1:5173/` with
timestamp marker `2026-06-10T03:38:25.337Z`. The local validation run for this
tranche passed `npm test --workspace apps/desktop`,
`npm run build --workspace apps/desktop`,
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib`, and the
full `python3 -m pytest -q tests` suite (340 passed).

The validation-evidence packet captured during the session recorded
`current_tranche_smoke_record=TP-MAC-72`. Before running preview mechanics the
Native JSON Package panel rendered its empty state. After running preview
mechanics (`state=completed`, `result_rows=647`) and before any persistence,
the visible `Persisted review context` line of the Native JSON Package panel
rendered the new persisted-run fields at their defaults —
`mechanics_results=0`, `analysis_runs=0`, `run_ref=not_persisted` — and the
native package packet carried the same defaults on both
`source_project.storage_summary` and `generation_context`
(`persisted_mechanics_result_count=0`, `persisted_analysis_run_count=0`,
`persisted_analysis_run_ref=not_persisted`). After choosing `Save local`, the
visible line reported `mechanics_results=1`, `analysis_runs=1`, and
`run_ref=run:preview-linear-static-001`, and the packet carried
`persisted_mechanics_result_count=1`, `persisted_analysis_run_count=1`, and
`persisted_analysis_run_ref=run:preview-linear-static-001` on both
`source_project.storage_summary` and `generation_context`. After choosing
`Open local`, the restored run kept the progress basis
`restored_persisted_run_record_no_new_solve_executed` and the same
persisted-run values remained in the visible line and in both packet
locations, with the packet boundary flags
`release_or_professional_claim=false`, `private_payload_included=false`, and
`protected_content_included=false`. The exposed persisted-run context is
local technical-preview review evidence only; it is not a release claim,
professional approval, certification, sealing, authentication, or
code-compliance claim. Captured browser console error and warning entries
after the timestamp marker were absent.

TP-MAC-73 local-project-index-listing smoke passed on 2026-06-09
America/Edmonton using the in-app browser at `http://127.0.0.1:5173/` with
timestamp marker `2026-06-10T03:52:40.229Z`. The local validation run for this
tranche passed `npm test --workspace apps/desktop`,
`npm run build --workspace apps/desktop`,
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib`
(including the new `list_projects` ordering test), and the full
`python3 -m pytest -q tests` suite (340 passed).

The validation-evidence packet captured during the session recorded
`current_tranche_smoke_record=TP-MAC-73`. Before any project operation the
Project Storage Audit panel rendered the new `Project index` line at its
defaults — `state=not_requested; listed_projects=0; refs=none` — and the
storage-audit packet carried `project_index_state=not_requested`,
`listed_project_count=0`, and empty `project_index`/`project_index_refs`
arrays. Choosing the new `List local` control against the empty store reported
`Listed 0 local project snapshots from the local store index.` with
`state=listed; listed_projects=0; refs=none`. After `Save local` followed by
`List local`, the visible line reported
`state=listed; listed_projects=1; refs=project:invented-loop-01`, and the
packet recorded `last_operation=list`, `project_index_state=listed`,
`listed_project_count=1`, `project_index_refs=["project:invented-loop-01"]`,
and an index entry with `project_id=project:invented-loop-01`,
`project_name=Invented Utility Loop Preview`,
`storage_mode=browser_memory_preview`, and positive
`created_at_unix`/`updated_at_unix` values. A subsequent `Open local`
round-trip succeeded with the listed index intact, and the packet boundary
flags remained `release_or_professional_claim=false`,
`private_payload_included=false`, `protected_content_included=false`,
`network_required=false`, and `telemetry_enabled=false`. The project index is
a local technical-preview store listing only; it is not a release claim,
professional approval, certification, sealing, authentication, or
code-compliance claim. Captured browser console error and warning entries
after the timestamp marker were absent.

## TP-MAC-74 — open-by-id-project-picker

- Date: 2026-06-09 (America/Edmonton)
- Browser smoke marker: `2026-06-10T04:06:53.239Z`
- Scope: open a specific listed project snapshot by id — `open_local_project`
  by-id selection covered in Rust tests, `openLocalProject(projectId)` service
  parameter with browser-memory id matching, per-entry "Open … (id)" picker row
  rendered from the listed project index, and `open_by_id` operation exposure
  in the Project Storage Audit packet.
- Validation commands:
  - `npm test --workspace apps/desktop` → 10/10 passed
  - `npm run build --workspace apps/desktop` → built
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` → 3/3
    passed (index test now also asserts open-by-id selects the older snapshot
    while open-most-recent returns the newer one)
  - `python3 -m pytest -q tests` → 340/340 passed
- Browser smoke (Claude Preview, port 5173):
  - Initial state: `Project index state=not_requested; listed_projects=0;
    refs=none`; no picker row rendered.
  - Save local → List local: message `Listed 1 local project snapshot from the
    local store index.`; index line `state=listed; listed_projects=1;
    refs=project:invented-loop-01`; picker row visible with button
    `Open Invented Utility Loop Preview (project:invented-loop-01)`.
  - Open by id: message `Opened local browser-preview project snapshot by id
    project:invented-loop-01.`; storage audit summary `operation=open_by_id`;
    packet `last_operation=open_by_id`, index state/refs retained.
  - Validation evidence packet `current_tranche_smoke_record=TP-MAC-74`.
  - Boundary flags: network=false, daemon=false, telemetry=false,
    private_payload=false, release_or_professional_claim=false.
  - Console: zero errors, zero warnings.
- Boundary: local technical-preview evidence only — invented data, local-only
  storage; no release, professional, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-75 — canonical-model-hash-service

- Date: 2026-06-09 (America/Edmonton)
- Browser smoke marker: `2026-06-10T04:17:23.439Z`
- Scope: close the `model_hash_status` TBD in the native JSON package review —
  new `hashService` computing SHA-256 over JCS-like canonical (sorted-key)
  model JSON via Web Crypto, model-hash state in App passed into
  `NativePackagePanel`, computed hash carried in the validation report, the
  manifest hash basis, and the `model/project.json` member record. The
  canonical *package* hash service remains `TBD` and is unchanged.
- Validation commands:
  - `npm test --workspace apps/desktop` → 12/12 passed (new
    `hashService.test.ts` covers canonical key-order invariance and
    deterministic sha256 evidence fields)
  - `npm run build --workspace apps/desktop` → built
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` → 3/3
    passed
  - `python3 -m pytest -q tests` → 340/340 passed
- Browser smoke (Claude Preview, port 5173):
  - Run mechanics preview → native package validation line
    `model_hash=computed_local_preview_sha256`.
  - Packet: `validation_report.model_hash_status=computed_local_preview_sha256`;
    `model_hash.value=sha256:f596e0f4…98a701e` (64-hex format verified);
    canonicalization `jcs_like_sorted_object_keys`; payload ref
    `project:invented-loop-01`; manifest `source_model_version_or_hash_basis`
    and `model/project.json` member `hash_status` carry the same value;
    `package_hash_status` still `TBD_canonical_package_hash_service_not_available`.
  - Validation evidence packet `current_tranche_smoke_record=TP-MAC-75`.
  - Boundary flags: private_payload=false, protected_content=false,
    release_or_professional_claim=false.
  - Console: zero errors, zero warnings.
- Boundary: the model hash is local technical-preview evidence for review
  reproducibility only — not a release artifact hash, acceptance record, or
  certification/sealing/authentication/code-compliance claim.

## TP-MAC-76 canonical-package-hash-service (2026-06-09)

- Scope: compute a canonical package hash for the native JSON package review packet. `hashService` gains a shared sha256 digest helper and `computePackageHash(packageId, canonicalPayload)`; `NativePackagePanel` hashes the JCS-like canonical JSON of the built review packet (hash carrier fields excluded from the basis) and carries the evidence in `validation_report.package_hash_status` / `validation_report.package_hash`. Loss-report TBD entry trimmed to physical container + transport. Other export panels' `canonical_package_hash_status` TBDs (ExportReviewPanel, ExportAdapterSdkPanel, StressNeutral, PCF, CAEPIPE MBF) intentionally out of scope.
- Validation commands: `npm test --workspace apps/desktop` (13/13), `npm run build --workspace apps/desktop` (pass), `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` (3/3), `python3 -m pytest -q tests` (340/340).
- Browser smoke (Claude Preview, port 5173, marker `2026-06-10T04:29:05.907Z`): ran mechanics preview; native package validation line shows `model_hash=computed_local_preview_sha256; package_hash=computed_local_preview_sha256`; exported packet carries `package_hash.value=sha256:42aa719dcb48731468fce5f5061c2cfa9d692d50757e2d6718c588f9566f0e49` with `payload_scope=package_review_payload`, `payload_excludes=validation_report_package_hash_fields`, `payload_ref=native-json-preview:project:invented-loop-01`, distinct from `model_hash.value=sha256:f596e0f4…98a701e`; loss-report TBD refs now `physical_project_container, public_transport_protocol` only; validation evidence smoke record `TP-MAC-76`.
- Boundary flags observed false: `private_payload_included`, `protected_content_included`, `release_or_professional_claim`, `network_required`, `telemetry_enabled`; boundary line renders the local-review-only disclaimer. Zero console errors/warnings.
- Boundary: this hash is local technical-preview review-reproducibility evidence only — not a release artifact hash, acceptance record, certification, sealing, authentication, or code-compliance claim.

## TP-MAC-77 export-panel-canonical-package-hash (2026-06-09)

- Scope: extend the canonical package hash service to the export panels. New shared `usePackageHash` hook + `withCanonicalPackageHash` overlay (`src/services/usePackageHash.ts`); `NativePackagePanel` refactored onto the hook with no hash-basis change. `StressNeutralExportPanel`, `PcfExportPanel`, `CaepipeMbfExportPanel`, and `ExportAdapterSdkPanel` now compute sha256 over the JCS-like canonical JSON of their own built packet (carrier fields `manifest.canonical_package_hash*` / `validation_report.hash_validation_status` excluded from the basis, evidence `payload_excludes=manifest_and_validation_report_package_hash_carrier_fields`) and carry status `computed_local_preview_sha256` plus `hash_validation_status=package_hash_computed_local_preview_not_independently_validated`. ExportReview descriptor strings and the SDK stress-neutral target `unresolved_tbd_refs` updated to match. Per-member/artifact `hash_status` and `source_model_hash_status` TBDs intentionally left in place.
- Validation commands: `npm test --workspace apps/desktop` (13/13), `npm run build --workspace apps/desktop` (pass), `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` (3/3), `python3 -m pytest -q tests` (340/340).
- Browser smoke (Claude Preview, port 5173, marker `2026-06-10T04:42:13.521Z`): after a mechanics run all five panels show `package_hash=computed_local_preview_sha256`; four distinct export-panel hash values observed (stress-neutral `sha256:a29898a3…`, PCF `sha256:7e3613a0…`, CAEPIPE MBF `sha256:4189b878…`, SDK `sha256:0aaf50ce…`); native package hash unchanged from TP-MAC-76 (`sha256:42aa719d…`), confirming the hook refactor preserved the hash basis; SDK stress-neutral target `unresolved_tbd_refs=[]`; ExportReview descriptors show `computed_local_preview_sha256_by_target_panel`; validation evidence smoke record `TP-MAC-77`. Zero console errors/warnings.
- Boundary: hashes are local technical-preview review-reproducibility evidence only — not release artifact hashes, acceptance records, certification, sealing, authentication, or code-compliance claims; no network/telemetry writes.

## TP-MAC-78 local-project-model-hash-persistence (2026-06-09)

- Scope: persist the canonical model hash with local project snapshots and
  verify it on open. Rust store (`src-tauri/src/lib.rs`) gains a
  `model_hash_json` column (with `ensure_column` migration), a
  `normalized_model_hash` helper, `model_hash` passthrough on
  `create_local_project` (camelCase invoke arg `modelHash`) and
  `save_local_project` (`request.model_hash`), envelope `model_hash`, and
  summary fields `persisted_model_hash_count` / `persisted_model_hash_ref`;
  the hash is stored verbatim (frontend WebCrypto JCS-like canonicalization is
  the single hashing authority; no Rust-side hashing) and excluded from FTS.
  Frontend (`projectService.ts`, `App.tsx`, `ProjectStorageAuditPanel.tsx`,
  `types.ts`) passes the computed model hash into create/save (browser-memory
  fallback contract-identical), recomputes the hash from the restored model on
  open, and derives `ModelHashIntegrityEvidence`
  (`verified_match` / `mismatch_review_required` / `not_persisted` /
  `hash_recompute_unavailable`, basis
  `recomputed_on_open_from_restored_model`), cleared on create/save/open so no
  stale verification is shown. New audit-panel lines
  `model-hash-persistence` and `model-hash-integrity`.
- Validation commands: `npm test --workspace apps/desktop` (13/13),
  `npm run build --workspace apps/desktop` (pass),
  `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` (3/3),
  `python3 -m pytest -q tests` (340/340).
- Browser smoke (Claude Preview, port 5173, markers
  `2026-06-10T05:07:30.814Z` start, `2026-06-10T05:08:14.703Z` verified):
  before any store operation `model-hash-persistence` shows
  `persisted_model_hashes=0; persisted_model_hash_ref=not_persisted` and
  `model-hash-integrity` reports no open-verification this session; after
  mechanics run + Create local, persistence shows count 1 with
  `sha256:f596e0f4…` while integrity still reports no open-verification;
  after Save local + Open local, integrity shows
  `integrity_status=verified_match` with identical persisted and recomputed
  `sha256:f596e0f4…` values and
  `verification_basis=recomputed_on_open_from_restored_model`; validation
  evidence packet `gui_validation_context.current_tranche_smoke_record`
  is `TP-MAC-78`. Zero console errors/warnings.
- Boundary: the persisted hash and open-verification status are local
  technical-preview review-reproducibility evidence only — a review-only
  integrity signal for human review, not release artifact hashing, acceptance,
  certification, sealing, authentication, or code-compliance claims; no
  network/telemetry writes.

## TP-MAC-79 validation-preflight-model-hash-evidence (2026-06-10)

- Scope: close the stale canonical-hash TBDs in the Project Validation
  Preflight panel now that the canonical model-hash service exists
  (TP-MAC-75) and the hash is persisted with local project snapshots and
  verified on open (TP-MAC-78). `ProjectValidationPanel` gains `modelHash` /
  `modelHashIntegrity` props (threaded from existing App state; no new hash
  computation); `validation_profile.hash_service_status` becomes
  `canonical_model_hash_service_available_model_payload_scope` with a new
  explicit `project_envelope_hash_status=
  model_payload_scope_only_full_project_envelope_hash_tbd` so the unresolved
  full-envelope hash stays declared; the `reproducibility_metadata` round-trip
  category and a new summary/visible `Model hash evidence` line report the
  staged status (`model_hash_computed_not_persisted` →
  `model_hash_persisted_open_verification_not_run` →
  `model_hash_verified_on_open` / `model_hash_mismatch_review_required` /
  `model_hash_recompute_unavailable_review_required`, or
  `model_hash_service_unavailable_in_this_runtime`); the packet carries the
  `model_hash` evidence object and `model_hash_integrity`; the static
  `PROJECT-VALIDATION-HASH-SERVICE-TBD` warning is replaced by staged
  diagnostics (`PROJECT-VALIDATION-MODEL-HASH-REVIEW-ONLY` info,
  `PROJECT-VALIDATION-MODEL-HASH-MISMATCH` warning, or
  `PROJECT-VALIDATION-MODEL-HASH-SERVICE-UNAVAILABLE` warning) with new
  diagnostic class `REPRODUCIBILITY`.
- Validation commands: `npm test --workspace apps/desktop` (13/13),
  `npm run build --workspace apps/desktop` (pass),
  `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` (3/3),
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q tests` (340/340).
- Browser smoke (Claude Preview, port 5173, marker
  `2026-06-10T23:51:19.823Z`): initial state shows
  `model_hash=model_hash_computed_not_persisted; persisted_model_hashes=0;
  persisted_model_hash_ref=not_persisted;
  integrity=open_verification_not_run_this_session` and round-trip
  `reproducibility=model_hash_computed_not_persisted`; after Create local →
  Save local → Open local, the model-hash line shows
  `model_hash=model_hash_verified_on_open; persisted_model_hashes=1;
  persisted_model_hash_ref=sha256:f596e0f4…98a701e; integrity=verified_match`,
  the round-trip line shows `reproducibility=model_hash_verified_on_open`,
  and the storage-audit integrity line shows the same persisted/recomputed
  values with basis `recomputed_on_open_from_restored_model`. The exported
  validation packet carries `hash_service_status=
  canonical_model_hash_service_available_model_payload_scope`,
  `project_envelope_hash_status=
  model_payload_scope_only_full_project_envelope_hash_tbd`,
  `summary.model_hash_status=model_hash_verified_on_open`,
  `summary.persisted_model_hash_count=1`,
  `summary.model_hash_integrity_status=verified_match`, the full
  `model_hash_integrity` object, reproducibility category status
  `model_hash_verified_on_open`, and diagnostics
  `PROJECT-VALIDATION-PREFLIGHT-ONLY:info` plus
  `PROJECT-VALIDATION-MODEL-HASH-REVIEW-ONLY:info:REPRODUCIBILITY`; boundary
  flags `private_payload_included`, `protected_content_included`,
  `release_or_professional_claim`, `network_required`, and
  `telemetry_enabled` all false. Validation evidence packet
  `gui_validation_context.current_tranche_smoke_record` is `TP-MAC-79`.
  Zero console errors/warnings.
- Boundary: the model-hash preflight evidence is a local technical-preview
  review-reproducibility signal scoped to the model payload only — not a
  release artifact hash, acceptance record, certification, sealing,
  authentication, or code-compliance claim; the full project-envelope hash
  remains an explicit TBD; no network/telemetry writes.

## TP-MAC-80 versioned-store-schema-migration-ledger (2026-06-10)

- Scope: replace the local project store's ad-hoc, unversioned schema setup
  and hard-coded `migration_status="current"` with a versioned SQLite
  `user_version` migration ledger
  (`versioned_sqlite_user_version_migration_ledger`,
  `STORE_SCHEMA_TARGET_VERSION=7`: v1 base project/FTS tables, v2–v7 the six
  snapshot JSON columns), closing the Project Validation Preflight
  `migrate=not_run_migration_framework_tbd` seam per the DEL-00-04
  persistence/schema-versioning architecture (REQ-04-01/REQ-04-02). Every
  store open applies pending migrations idempotently (legacy pre-ledger
  stores reconcile from v0 without data loss) and reports evidence:
  `StorageCapability` and `LocalProjectSummary` gain `migration_framework`,
  `migration_status`
  (`current_store_schema_v7_no_pending_migrations` /
  `migrated_on_open_store_schema_v{before}_to_v{after}`),
  `store_schema_version`, `store_schema_target_version`, and
  `migrations_applied_on_open`. The validation preflight packet gains a
  `store_migration` block (with `evidence_source`,
  `migration_scope=local_store_schema_only_not_model_document_schema`,
  `model_document_migration_status=model_document_migrations_not_defined_tbd`,
  `destructive_migration_performed=false`), profile fields
  `store_migration_framework_status` and `model_document_migration_status`,
  a visible `Store migration evidence` line
  (`data-testid="project-validation-store-migration"`), a staged `migrate`
  service operation, and staged info diagnostics
  (`PROJECT-VALIDATION-STORE-MIGRATED-ON-OPEN` /
  `PROJECT-VALIDATION-STORE-MIGRATION-LEDGER-REVIEW-ONLY`, class
  `MIGRATION`). The browser memory fallback reports honest parity values
  (`browser_memory_preview_no_sqlite_migration_ledger`,
  `browser_memory_snapshot_no_sql_store_migrations_applicable`, versions 0).
  Model *document* migrations remain an explicit TBD; only the local store
  schema ledger is claimed.
- Validation commands: `npm test --workspace apps/desktop` (13/13),
  `npm run build --workspace apps/desktop` (pass),
  `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` (5/5,
  including new `store_migration_ledger_reports_fresh_open_and_idempotent_
  reopen_evidence` and
  `store_migration_ledger_reconciles_legacy_store_and_preserves_rows`),
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q tests` (340/340).
- Browser smoke (Claude Preview, port 5173, marker
  `2026-06-11T01:27:57.689Z`): initial state shows
  `framework=browser_memory_preview_no_sqlite_migration_ledger;
  store_schema_version=0; target=0; applied_on_open=0;
  status=browser_memory_snapshot_no_sql_store_migrations_applicable` with
  `migrate=not_run_no_local_snapshot_this_session` and summary
  `migration=not_persisted_this_session`; after Create local → Save local →
  Open local, the summary shows
  `migration=browser_memory_snapshot_no_sql_store_migrations_applicable`,
  the migrate operation reports the same staged status with
  `result_available=true`, the exported packet carries the full
  `store_migration` block (`evidence_source=local_project_summary`,
  `migrations_applied_on_open=[]`,
  `destructive_migration_performed=false`), profile
  `store_migration_framework_status=
  browser_memory_preview_no_sqlite_migration_ledger` and
  `model_document_migration_status=model_document_migrations_not_defined_tbd`,
  and diagnostics include
  `PROJECT-VALIDATION-STORE-MIGRATION-LEDGER-REVIEW-ONLY:MIGRATION:info`
  (and not `PROJECT-VALIDATION-STORE-MIGRATED-ON-OPEN`, since the browser
  fallback applies no SQL migrations). The TP-MAC-78/79 model-hash chain
  still verifies (`model_hash=model_hash_verified_on_open;
  integrity=verified_match`). Boundary flags `private_payload_included`,
  `protected_content_included`, `release_or_professional_claim`,
  `network_required`, `telemetry_enabled`,
  `accepted_model_state_mutated`, and `repository_default_private_write`
  all false; `local_only_project_store=true`. Validation evidence packet
  `gui_validation_context.current_tranche_smoke_record` is `TP-MAC-80`.
  Zero console errors/warnings. SQLite-runtime migration behavior
  (fresh v0→v7 open, idempotent reopen, legacy reconciliation with row
  preservation) is covered by the cargo lib tests rather than the browser
  smoke.
- Boundary: store migration evidence is local technical-preview store
  maintenance evidence scoped to the SQLite store schema ledger only — not a
  model document migration framework, release artifact, acceptance record,
  certification, sealing, authentication, or code-compliance claim; no
  destructive migrations; no network/telemetry writes.

## TP-MAC-81 full-project-envelope-hash-persisted-and-verified (2026-06-10)

- Scope: extended the canonical reproducibility hash chain from model-payload
  scope to the full persisted project envelope, closing the validation-profile
  `project_envelope_hash_status=
  model_payload_scope_only_full_project_envelope_hash_tbd` seam. The Tauri
  store gains ledger migration v8 (`store-v8-project-envelope-hash-column`,
  `STORE_SCHEMA_TARGET_VERSION=8`) adding `project_envelope_hash_json`;
  existing v7 stores stage `migrated_on_open_store_schema_v7_to_v8` on next
  open (covered by a new cargo test). `computeProjectEnvelopeHash` hashes the
  canonical JSON of `{model, editor_intents, proposal,
  selected_review_target, mechanics_result, analysis_run, model_hash}` —
  excluding the volatile storage summary and the envelope-hash carrier field
  itself so the hash can be recomputed from a restored envelope. The hash is
  computed at create/save, persisted through the envelope and
  `LocalProjectSummary` (`persisted_project_envelope_hash_count/ref`), and
  verified on open with `ProjectEnvelopeHashIntegrityEvidence`
  (`verification_basis=recomputed_on_open_from_restored_envelope_payload`).
  The Project Validation Preflight adds an `Envelope hash evidence` line
  (`data-testid="project-validation-envelope-hash"`), packet blocks
  `project_envelope_hash` / `project_envelope_hash_integrity`, profile
  `project_envelope_hash_scope=
  persisted_envelope_payload_excluding_storage_summary_and_hash_carrier`,
  and staged REPRODUCIBILITY diagnostics
  (`PROJECT-VALIDATION-ENVELOPE-HASH-NOT-COMPUTED` /
  `-REVIEW-ONLY` / `-MISMATCH`).
- Validation: `npm test --workspace apps/desktop` (13/13),
  `npm run build --workspace apps/desktop` (built),
  `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib` (6/6,
  including new
  `store_migration_ledger_stages_v7_store_to_v8_applying_only_pending_migration`),
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q tests` (340/340).
- Browser smoke (Claude Preview, fresh server, port 5173, marker
  `2026-06-11T02:11:26.627Z`): initial state shows
  `envelope_hash=project_envelope_hash_not_computed_no_save_this_session;
  persisted_envelope_hashes=0; integrity=
  open_verification_not_run_this_session` with diagnostic
  `PROJECT-VALIDATION-ENVELOPE-HASH-NOT-COMPUTED:REPRODUCIBILITY:info`;
  after Run mechanics preview → Create local → Save local → Open local, the
  envelope-hash line reports
  `envelope_hash=project_envelope_hash_verified_on_open;
  persisted_envelope_hashes=1; integrity=verified_match`, the exported
  packet carries `project_envelope_hash`
  (`payload_scope=project_envelope_payload`, `payload_excludes=
  storage_summary_and_envelope_hash_carrier_fields`,
  `value=sha256:4fa9a9ae4fd76200f4a1724befca579c28627bd21d080a8c9d49854dc24d9d6e`)
  and `project_envelope_hash_integrity` with persisted == recomputed and
  `verification_basis=recomputed_on_open_from_restored_envelope_payload`,
  over an envelope that includes one persisted mechanics result and one
  persisted analysis run; diagnostics include
  `PROJECT-VALIDATION-ENVELOPE-HASH-REVIEW-ONLY:REPRODUCIBILITY:info` and
  not `PROJECT-VALIDATION-ENVELOPE-HASH-MISMATCH`. The TP-MAC-78/79
  model-hash chain still verifies (`model_hash=model_hash_verified_on_open;
  persisted_model_hash_ref=
  sha256:f596e0f453ffefec7f78fa4e18964931e5a19820d606036481a9ef59598a701e;
  integrity=verified_match`). Boundary flags `private_payload_included`,
  `protected_content_included`, `release_or_professional_claim`,
  `network_required`, `telemetry_enabled`, `accepted_model_state_mutated`,
  and `repository_default_private_write` all false;
  `local_only_project_store=true`. Validation evidence packet
  `gui_validation_context.current_tranche_smoke_record` is `TP-MAC-81`.
  Zero console errors/warnings on the fresh server (an earlier HMR session
  while the tranche was being edited produced transient
  `ProjectValidationPanel` error-boundary warnings; the server was restarted
  and the final code smoke-tested clean). SQLite-runtime persistence of the
  new column and the v7→v8 staged migration are covered by the cargo lib
  tests rather than the browser smoke.
- Boundary: the project-envelope hash is a local technical-preview
  review-reproducibility signal over the persisted envelope payload only —
  not an acceptance, certification, sealing, authentication, release, or
  code-compliance record; model *document* schema migrations remain TBD; no
  destructive migrations; no network/telemetry writes.

## TP-MAC-82 apply-operation-command-path (2026-06-10)

- Tranche: `TP-APP-R2-EDITLOOP-001` (completion plan Phase A1) — structured
  apply-operation seam. New core crate
  `core/model_operations/operation_applier` implements PKG-16
  validate → diff → apply semantics for inspector `set_field` /
  `update_load` / `update_support` intents; new Tauri commands
  `validate_model_operation` and `apply_model_operation` expose it; the new
  Apply Operations panel validates/applies queued intents; applying replaces
  the session model document, clears stale solve results, and records an
  applied-operation receipt. Browser fixture mode runs an equivalently
  scoped local engine honestly labeled
  `application_route=browser_fixture_local_apply` (Tauri route reports
  `tauri_backend_apply`).
- Local validation before browser smoke: `cargo test` in
  `core/model_operations/operation_applier` (19/19, including no-input-
  mutation, stale-before blocking, unit-mismatch blocking without
  conversion, project-unit resolution for node positions, restraint
  vocabulary, deferred-field findings, viewport geometry findings,
  determinism, and professional-boundary checks); `cargo test` in
  `apps/desktop/src-tauri` (14/14, including applying an inspector intent to
  the bundled fixture model and re-solving the applied model through
  `run_preview_mechanics`); `npm test --workspace apps/desktop` (21/21);
  `npm run build --workspace apps/desktop`; `python3 -m pytest -q tests`
  (342/342, untouched surfaces).
- Browser smoke (Claude Preview, fresh vite server on port 5173, browser
  fixture mode): selected `material:invented-carbon-steel`, queued
  `op:editor-intent-material:invented-carbon-steel-elastic_modulus.value`
  (`200000000000` → `195000000000` Pa) as `editor-intent-1`; Validate
  reported `validate_only; application_status=not_applied; schema=passed;
  unit=passed; before_state=passed; route=browser_fixture_local_apply`;
  Apply reported the applied message, drained the queue
  (`0 queued; 1 applied this session`), updated the inspector to
  `195000000000 Pa`, reset the solve job to `state=not_started; events=1`
  with the model-changed event (`previous mechanics results were cleared`),
  and recorded the receipt
  `route=browser_fixture_local_apply;
  acceptance=user_initiated_apply_in_local_session;
  persistence=session_state_only_not_yet_saved;
  professional_approval=false`. Run mechanics preview then completed against
  the edited model (`state=completed; events=3; result_rows=647`) and Save
  local stored the edited snapshot (`Saved local browser-preview project
  snapshot without external file copies`). The toolbar review context now
  reports `applied_operations=N` instead of a hardcoded `applied=false`.
  Console errors absent. Vitest covers the same journey plus stale-intent
  blocking (`OP-STALE-BEFORE-VALUE`) and viewport-gesture blocking
  (`OP-GEOMETRY-INPUT-INCOMPLETE`).
- Boundary: applying a structured operation is a user-initiated
  local-session acceptance over the session model document only — receipts
  state `acceptance_is_professional_approval=false`; no unit conversion is
  performed (mismatches block with `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`
  pending D-01/Phase B); no geometry values are invented for viewport
  gesture intents (blocked pending A3); applied receipts are session-only
  until the A2 model-document persistence tranche (D-08); claimed UI model
  hashes are echoed beside backend hashes without cross-canonicalization
  equality claims; stale results are cleared rather than displayed against
  an edited model; no compliance, certification, sealing, authentication,
  approval, release, or code-compliance claims.

## TP-MAC-83 model-document-schema-migration-seam (2026-06-10)

- Tranche: `TP-APP-R2-PERSIST-001` (completion plan Phase A2) implementing the
  human-ruled D-08 policy (`DEC-019` in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12): two-track versioning
  with the in-document semver `schema_version` as the sole model-document
  version authority; an ordered transform-chain registry in the application
  service (`apps/desktop/src-tauri/src/model_document_migration.rs`,
  framework `application_service_separate_db_and_product_schema`);
  migrate-in-memory-on-open / persist-on-save; `newer_than_supported` and
  `unsupported_schema` documents refused for editing with structured
  diagnostics (no coercion, no down-migration); and a per-document
  evidence-only migration ledger (store schema v9 column
  `model_migration_ledger_json`) whose records carry pre- and post-migration
  model hashes. The validation preflight's hardcoded
  `model_document_migrations_not_defined_tbd` marker is replaced by real
  DEC-019 evidence (new `project-validation-model-document-migration` line
  and packet block). The published transform chain is empty — `0.1.0` is the
  only model-document schema version ever published — so chain mechanics are
  proven by injected-chain tests, not invented historical versions.
- Local validation before browser smoke: `cargo test` in
  `apps/desktop/src-tauri` (24/24: 7 new migration-module tests covering
  current/newer/invalid/chain/gap/failure/ledger-record semantics; 3 new
  persist-path tests covering refusal, current-document persistence, and
  open-time-migrated save appending a ledger record with pre/post hashes;
  store ledger tests updated for the v9 staged migration with target
  version 9); `npm test --workspace apps/desktop` (25/25, including new
  projectService browser-engine tests and the R2 flow asserting the new
  validation line); `npm run build --workspace apps/desktop`;
  `python3 -m pytest -q tests` (342/342).
- Browser smoke (Claude Preview, fresh vite server on port 5173, browser
  fixture mode): initial load shows
  `status=current; source=0.1.0; target=0.1.0;
  framework=application_service_separate_db_and_product_schema;
  persistence=no_persistence_operation_this_session; ledger_records=0`
  (session-document local evaluation — no hardcoded TBD); after Create
  local → Save local → Open local the line reports
  `persistence=stored_document_current; ledger_records=0` from the
  persistence envelope. Console errors and warnings absent.
- Boundary: model-document migration evidence is a local technical-preview
  reproducibility signal only; stored bytes are never rewritten as a side
  effect of open; no destructive rewrites; no down-migration; refused
  documents are findings, not silent coercions; the backward-compatibility
  window size remains `TBD — human ruling` per the DEC-019 packet note; no
  release, professional, certification, sealing, authentication, approval,
  or code-compliance claims.

## TP-MAC-84 viewport-selection-binding (2026-06-10)

- Tranche: `TP-APP-R2-VIEWSELECT-001` (completion plan Phase A3 sub-slice)
  implementing selectable viewport entity controls over the existing Three.js
  centerline view. The viewport now exposes DOM-accessible selection targets
  for the invented fixture's rendered nodes, straight pipe segments,
  supports, and component markers; selecting one updates the shared React
  selection state, the model tree active row, the property inspector, and the
  viewport active highlight. The controls are generated from loaded model data
  and do not create or apply operation intents by themselves.
- Local validation: `npm test --workspace apps/desktop` passed with 26/26
  Vitest tests, including `selects rendered viewport entities into the shared
  model inspector`; `npm run build --workspace apps/desktop` passed through
  `tsc -b` and Vite production build. Browser smoke against a fresh Vite
  server on `http://127.0.0.1:5174/` confirmed 14 viewport selection targets;
  selecting `pipe:P-120` updated the inspector to `Rack span`, set the model
  tree row class to `tree-row active`, set the viewport target
  `aria-pressed=true`, and updated the toolbar to `Selected: pipe:P-120`.
  Browser console warnings/errors were absent. No Rust or Python source
  changed in this tranche.
- Boundary: this is UI selection and inspection only. Durable model mutation
  still routes through queued structured operations and the existing explicit
  apply path; viewport geometry-creation intents remain blocked from apply
  until later A3 geometry capture supplies explicit coordinates/connectivity.
  No unit conversion, code-specific default, protected standards data,
  private project data, telemetry, network path, release claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  is introduced.

## TP-MAC-85 property-inspector-inline-validation (2026-06-10)

- Tranche: `TP-APP-R2-INLINEVALID-001` (completion plan Phase A3 sub-slice)
  adding validate-only inline feedback to the property inspector's draft
  editor intent. The inspector now calls the existing structured-operation
  validation seam before queue/apply and displays application status, schema
  state, unit state, before-state state, diff rows, diagnostics, and an
  explicit no-mutation/professional-boundary note beside the draft edit.
- Local validation: `npm test --workspace apps/desktop` passed with 26/26
  Vitest tests, including inline validation of an invented material elastic
  modulus edit; `npm run build --workspace apps/desktop` passed through
  `tsc -b` and Vite production build. Browser smoke against a fresh Vite
  server on `http://127.0.0.1:5174/` selected
  `material:invented-carbon-steel`, validated
  `elastic_modulus.value` from `200000000000` to `210000000000` Pa, and
  observed `validate_only; application_status=not_applied; schema=passed;
  unit=passed; before_state=passed`, the expected diff row, and
  `validate-only; no accepted model mutation; no professional approval`.
  Timestamp-filtered browser logs after the final reload contained no new
  warnings/errors. No Rust or Python source changed in this tranche.
- Boundary: this is validation preview only. It does not apply operations,
  mutate accepted or persisted model state, perform unit conversion, supply
  code-specific defaults, include protected standards data, include private
  project data, access network/cloud/telemetry, or make release,
  professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-86 explicit-node-create-operation (2026-06-10)

- Tranche: `TP-APP-R2-CREATENODE-001` (completion plan Phase A3 sub-slice)
  adding explicit form-based node creation through the structured operation
  seam. The viewport editor now captures user-entered node id, label, and
  finite x/y/z coordinates in the project length unit, queues a
  `create_node` intent with `field_path=nodes`, and applies it through the
  existing validate/diff/apply path. The browser local engine and Rust
  operation applier now accept only explicit node payloads, reject duplicate
  node ids, require matching length-unit metadata, and keep underspecified
  viewport gesture intents blocked rather than inventing geometry.
- Local validation: `cargo test --manifest-path
  core/model_operations/operation_applier/Cargo.toml` passed with 20/20
  tests, including explicit create-node apply and underspecified gesture
  blocking; `npm test --workspace apps/desktop` passed with 28/28 Vitest
  tests, including the full viewport queue/apply flow; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production
  build; targeted `git diff --check` on touched files passed. Browser smoke
  against `http://127.0.0.1:5174/` entered `node:N-150`, `User preview
  node`, and coordinates `8.4, 2.4, 2.8`; queue/apply produced
  `Applied op:viewport-create-node-node:N-150-001`, active tree row
  `tree-row-node:N-150`, inspector position `8.4, 2.4, 2.8 m`, review
  context `applied_operations=1`, receipt
  `acceptance=user_initiated_apply_in_local_session`,
  `persistence=session_state_only_not_yet_saved`, and
  `professional_approval=false`. Viewport targets increased from 14 to 15.
  Timestamp-filtered browser warnings/errors after the final reload were
  absent.
- Boundary: this applies only to the in-session model document until the user
  saves the local project. It does not perform unit conversion, infer
  missing coordinates, create pipe connectivity, implement canvas raycast
  geometry capture, implement undo/redo, include protected standards data,
  include private project data, access network/cloud/telemetry, or make
  release, professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-87 session-undo-redo-checkpoints (2026-06-10)

- Tranche: `TP-APP-R2-UNDOREDO-001` (completion plan Phase A3 sub-slice)
  adding local-session undo/redo checkpoints for applied structured
  operations. The Apply Operations panel now exposes Undo/Redo controls and a
  history summary (`undo=N; redo=N; local_session_only=true;
  saved_project_mutated=false`). Applying an operation pushes the prior
  session model/selection onto the undo stack and clears redo; undo restores
  the prior session model, moves the current model to redo, clears stale solve
  results, and does not mutate saved project data; redo restores the undone
  session model and clears stale solve results again.
- Local validation: `npm test --workspace apps/desktop` passed with 28/28
  Vitest tests, including apply → undo → redo of explicit viewport node
  creation; `npm run build --workspace apps/desktop` passed through
  `tsc -b` and Vite production build; targeted `git diff --check` on touched
  files passed. Browser smoke against `http://127.0.0.1:5174/` created
  `node:N-155` at `9.1, 2.6, 3.3`, observed `undo=1; redo=0`, clicked Undo
  and confirmed the node row disappeared with `undo=0; redo=1` and
  `state=not_started`, then clicked Redo and confirmed the active node row
  and inspector position returned with `undo=1; redo=0`. Timestamp-filtered
  browser warnings/errors after the final reload were absent.
- Boundary: undo/redo is an in-memory session checkpoint mechanism only. It
  does not rewrite durable project snapshots, erase the audit receipt ledger,
  persist model changes, perform unit conversion, infer engineering values,
  access protected/private data, access network/cloud/telemetry, or make
  release, professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-88 edited-model-solve-binding (2026-06-11)

- Tranche: `TP-APP-R2-SOLVEBOUND-001` (completion plan Phase A5 first
  sub-slice) adding an honest edited-model solve boundary. Browser fixture
  mode now refuses to reuse bundled solved-result rows for edited session
  models; it returns a result envelope bound to the edited `project.id` with
  `MODEL_INCOMPLETE`, zero result rows, and blocking diagnostic
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`. The unchanged bundled
  model still returns the solved fixture. Tauri unit coverage verifies that
  both `run_preview_mechanics(Some(model))` and the backend solve-job seam use
  the supplied edited model payload and publish solved results bound to the
  edited `project.id`.
- Local validation: `npm test --workspace apps/desktop` passed with 31/31
  Vitest tests; `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  passed with 26/26 Rust tests; `npm run build --workspace apps/desktop`
  passed through `tsc -b` and Vite production build; `git diff --check -- .
  ':!init/init-prompt.md'` passed for the touched scope. Browser smoke
  against `http://127.0.0.1:5174/` selected
  `material:invented-carbon-steel`, applied `elastic_modulus.value =
  195000000000`, ran mechanics preview, and confirmed `status-mechanics =
  MODEL INCOMPLETE`, `0 computed result rows; model incomplete`,
  `state=completed; result_rows=0`, diagnostic
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, and solve-job JSON
  `result_row_count=0` with diagnostics
  `RULE_INPUTS_MISSING` and
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`. Browser console output
  contained only Vite debug messages and the React DevTools info line; no
  warnings/errors/page errors were observed.
- Boundary: this is not a browser solver implementation. Edited-model solving
  is proven through the Tauri backend command/job path; browser fixture mode
  now blocks stale fixture reuse. No protected/private data, code-specific
  defaults, network/cloud/telemetry, release claim, professional approval,
  certification, sealing, authentication, or code-compliance claim was added.

## TP-MAC-89 viewport-deformation-overlay (2026-06-11)

- Tranche: `TP-APP-R2-DEFORMEDVIEW-001` (completion plan Phase A6 first
  sub-slice) adding a Three.js viewport overlay from solved preview mechanics
  result rows. The overlay consumes node `displacement_magnitude` rows,
  renders an overlaid centerline/marker set, and reports
  `available; nodes=5; max=33.211157 mm` with boundary
  `scale=normalized_display_offset_not_physical_length;
  vector_direction=TBD; unit_basis=mm; professional_claim=false`. Initial
  state reports `not started`; incomplete mechanics results report `blocked`
  and render no overlay.
- Local validation: `npm test --workspace apps/desktop` passed with 31/31
  Vitest tests; `npm run build --workspace apps/desktop` passed through
  `tsc -b` and Vite production build; `git diff --check -- .
  ':!init/init-prompt.md'` passed for the touched scope.
- Browser smoke against `http://127.0.0.1:5174/` used system Chrome through
  Playwright. Desktop viewport `1280x900`: canvas screenshot was `578x320`,
  opaque pixels `184960`, sampled unique colors reached `1000`, and a
  700 ms animation diff changed `5567` pixels. Mobile viewport `390x844`:
  canvas screenshot was `458x240`, opaque pixels `109920`, sampled unique
  colors reached `1000`, and the animation diff changed `6480` pixels. Both
  runs updated overlay status to `available; nodes=5; max=33.211157 mm`;
  layout checks confirmed `frameBottom == intentsTop` and
  `intentsBottom < bottomPanelsTop`; browser console output contained only
  Vite debug messages and the React DevTools info line.
- Boundary: this overlay is a normalized display aid from displacement
  magnitudes only. It is not a physical directional deformed-shape solver, not
  a validation plot, not a professional acceptance record, and not a release,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-90 playwright-r2-smoke-harness (2026-06-11)

- Tranche: `TP-APP-R2-PLAYWRIGHT-001` (completion plan Phase A8 first
  sub-slice) adding a Playwright harness for the desktop technical preview.
  The root script `npm run test:e2e:desktop` delegates to the desktop
  workspace and runs `apps/desktop/e2e/r2-smoke.spec.ts` against a Vite dev
  server on `127.0.0.1:5174`.
- Automated coverage: initial shell and local-only boundary signals,
  nonblank/changing Three.js viewport canvas, mechanics preview solve,
  `result_rows=647`, viewport displacement-overlay availability, result
  filtering and result-detail metadata for `pipe:P-120`, and deterministic
  report-packet export fields including false private/protected/release
  flags.
- Local validation: `npm run test:e2e:desktop` passed with 1/1 Playwright
  tests using the `chromium-desktop` project; `npm test --workspace
  apps/desktop` passed with 31/31 Vitest tests after scoping Vitest to
  `src/**/*.test.{ts,tsx}`; `npm run build --workspace apps/desktop` passed
  through `tsc -b` and Vite production build; `git diff --check -- .
  ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this first harness slice is a regression spine over the existing
  invented preview fixture. It does not yet automate authored model creation
  from a blank project, packaged Tauri saved-project solve, full manual SMOKE
  checklist parity, CI browser provisioning, protected/private data handling,
  release readiness, professional approval, certification, sealing,
  authentication, or code-compliance claims.

## TP-MAC-91 load-case-manager-primitive-magnitude (2026-06-11)

- Tranche: `TP-APP-R2-LOADMGR-001` (completion plan Phase A4 first
  sub-slice) adding a right-rail Load Cases manager for the invented desktop
  preview model. The panel surfaces `2 load cases`, `7 primitive loads`, and
  `1 combinations`; selecting `load:L-100-P` exposes
  `primitive_loads.2.magnitude.value` with pressure magnitude `1200000 Pa`.
- Edit path: changing the selected primitive magnitude queues
  `op:load-manager-load:L-100-load:L-100-P-magnitude` as a structured
  `update_load` operation. Applying it uses the existing OperationApplyPanel,
  records `session_state_only_not_yet_saved`, clears stale solve results, and
  does not mutate a saved project until the user saves locally.
- Local validation: `npm test --workspace apps/desktop` passed with 32/32
  Vitest tests, including manager queue/apply coverage; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed with
  1/1 Playwright tests and now asserts manager summary plus primitive
  pressure-row selection; `git diff --check -- . ':!init/init-prompt.md'`
  passed for the touched scope.
- Boundary: this first A4 slice edits existing primitive load magnitudes only.
  It does not add load-case creation, load status/kind editing, arbitrary
  primitive-load creation, imposed-displacement authoring breadth,
  rule-code/default combinations, protected standards content, private data,
  release readiness, professional approval, certification, sealing,
  authentication, or code-compliance claims.

## TP-MAC-92 explicit-straight-pipe-connectivity (2026-06-11)

- Tranche: `TP-APP-R2-CONNECTPIPE-001` (completion plan Phase A3 fifth
  sub-slice) adding explicit straight-pipe creation from the viewport editor.
  The form captures pipe id, label, endpoint nodes, material, outside
  diameter, wall thickness, non-zero local `y_reference`, and provenance, then
  queues a structured `connect_pipe_run` operation for `pipe_segments`.
- Edit path: applying
  `op:viewport-connect-pipe-pipe:P-150-001` appends the explicit pipe payload
  to the session model through the OperationApplyPanel. The created pipe row
  becomes active in the model tree and viewport selection layer; the property
  inspector shows the user-entered label, OD, wall thickness, material, and
  provenance. Legacy one-click pipe-run intents remain blocked rather than
  inventing connectivity or section geometry.
- Local validation: `cargo test --manifest-path
  core/model_operations/operation_applier/Cargo.toml` passed with 22/22 Rust
  tests, including explicit pipe apply and underspecified pipe gesture
  blocking; `npm test --workspace apps/desktop` passed with 33/33 Vitest
  tests; `npm run build --workspace apps/desktop` passed through `tsc -b` and
  Vite production build with the existing chunk-size warning; `npm run
  test:e2e:desktop` passed with 1/1 Playwright tests.
- Boundary: this A3 slice accepts only explicit straight-pipe payloads with
  existing endpoint/material references and project-length-unit section
  quantities. It does not implement canvas raycast capture, hidden geometry
  defaults, unit conversion, durable persistence, protected standards content,
  private data, release readiness, professional approval, certification,
  sealing, authentication, or code-compliance claims.

## TP-MAC-93 viewport-canvas-node-drafting (2026-06-11)

- Tranche: `TP-APP-R2-CANVASNODE-001` (completion plan Phase A3 sixth
  sub-slice) adding canvas pointer capture for explicit node drafting. A
  primary click on the viewport fills the existing explicit node form with a
  visible generated node id/label and finite x/y/z coordinates from the
  `y=0` drafting plane. Queue/Apply remain explicit user actions through the
  structured operation path.
- Automated coverage: Vitest creates a canvas-drafted `node:V-001`, queues
  and applies it through `OperationApplyPanel`, then verifies the active model
  tree row and property inspector position. Playwright clicks the real
  WebGL canvas, verifies the form draft and enabled Queue node button, and
  leaves the draft unqueued so the existing solve/results/report smoke still
  runs on the unchanged fixture model.
- Local validation: `npm test --workspace apps/desktop` passed with 34/34
  Vitest tests; `npm run build --workspace apps/desktop` passed through
  `tsc -b` and Vite production build with the existing chunk-size warning;
  `npm run test:e2e:desktop` passed with 1/1 Playwright tests; in-app browser
  smoke at `http://127.0.0.1:5175/` confirmed one visible canvas, drafted
  `node:V-001`, finite coordinates, `y=0`, and Queue node enabled.
- Boundary: this slice drafts explicit node geometry only. It does not queue
  or apply without user action, mutate saved project data, implement pipe or
  component gesture capture, infer engineering values, perform unit
  conversion, add protected standards content, handle private data, or make
  release, professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-94 viewport-pipe-endpoint-picking (2026-06-11)

- Tranche: `TP-APP-R2-PIPEPICK-001` (completion plan Phase A3 seventh
  sub-slice) adding endpoint-pick controls to the viewport straight-pipe
  form. Arming `Pick` for `from` and selecting a rendered node fills the
  `from` field and advances to `to`; selecting a second rendered node fills
  `to` and clears pick mode.
- Edit path: endpoint picking only supplies existing node references. Pipe id,
  label, material, outside diameter, wall thickness, non-zero `y_reference`,
  and provenance remain explicit inputs before Queue pipe can enable. Vitest
  queues/applies picked-endpoint `pipe:P-151` through the existing
  OperationApplyPanel and verifies the created pipe selection.
- Local validation: `npm test --workspace apps/desktop` passed with 35/35
  Vitest tests; `npm run build --workspace apps/desktop` passed through
  `tsc -b` and Vite production build with the existing chunk-size warning;
  `npm run test:e2e:desktop` passed with 1/1 Playwright tests; in-app browser
  smoke at `http://127.0.0.1:5175/` confirmed `from=node:N-100`,
  `to=node:N-140`, pick-mode advancement/clearing, and Queue pipe disabled
  until the remaining explicit fields are supplied; `git diff --check -- .
  ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not infer pipe section/material/provenance values,
  auto-queue or auto-apply operations, mutate saved project data, perform unit
  conversion, add protected standards content, handle private data, or make
  release, professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-95 load-case-metadata-editor (2026-06-11)

- Tranche: `TP-APP-R2-LOADMETA-001` (completion plan Phase A4 second
  sub-slice) adding selected load-case `status` and `kind` editing to the
  Load Cases manager. The editor queues structured `update_load` operations
  for `Load.status` or `Load.kind` with explicit before/after values, unit
  `none`, dimension `dimensionless`, and the existing local-session
  acceptance boundary.
- Edit path: applying `op:load-manager-load:L-100-status` updates
  `load:L-100` from `preview_only` to `TBD` through `OperationApplyPanel`,
  resets stale solve results, leaves persistence at
  `session_state_only_not_yet_saved`, and records no professional approval.
  The `kind` editor previews `primitive_user_load -> TBD`; combination
  `basis` and `terms` editing remain explicitly deferred.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 23/23 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 36/36 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered status edit and confirmed the
  row showed `status=TBD`, pending operations returned to zero, and solve
  state remained `not_started`; `git diff --check -- . ':!init/init-prompt.md'`
  passed for the touched scope.
- Boundary: this slice does not add load-case creation, arbitrary
  primitive-load creation, imposed-displacement authoring, combination
  editing, unit conversion, saved-project mutation without user Save,
  protected standards content, private data, or release, professional
  approval, certification, sealing, authentication, or code-compliance claims.

## TP-MAC-96 combination-term-factor-editor (2026-06-11)

- Tranche: `TP-APP-R2-COMBFACTOR-001` (completion plan Phase A4 third
  sub-slice) adding existing combination term-factor editing to the Load
  Cases manager. The editor queues structured `update_load` operations for
  `Combination.terms.N.factor` with explicit before/after scalar values, unit
  `none`, dimension `dimensionless`, and the existing local-session
  acceptance boundary.
- Edit path: applying
  `op:load-manager-combination:C-OPER-ALT-term-1-factor` updates the existing
  `combination:C-OPER-ALT` second term from `load:L-200 x 0.5` to
  `load:L-200 x 0.75` through `OperationApplyPanel`, resets stale solve
  results, leaves persistence at `session_state_only_not_yet_saved`, and
  records no professional approval. Whole `terms` replacement, `basis`
  editing, term creation/deletion, code/rule combinations, and broader
  algebra authoring remain explicitly deferred.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 24/24 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 37/37 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered combination-factor edit and
  confirmed `load:L-200 x 0.75`, pending operations returned to zero,
  persistence remained `session_state_only_not_yet_saved`, professional
  approval remained false, and solve state remained `not_started`; `git diff
  --check -- . ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not add load-case creation, arbitrary
  primitive-load creation, imposed-displacement authoring, combination basis
  editing, combination term creation/deletion, code/rule combinations,
  broader algebra authoring, unit conversion, saved-project mutation without
  user Save, protected standards content, private data, or release,
  professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-97 load-case-creation-editor (2026-06-11)

- Tranche: `TP-APP-R2-LOADCREATE-001` (completion plan Phase A4 fourth
  sub-slice) adding explicit empty load-case creation to the Load Cases
  manager. The create form captures id, label, kind, status, and provenance,
  then queues a structured `create_load_case` operation for a shell whose
  `primitive_loads` collection is empty.
- Edit path: applying `op:load-manager-create-load:L-300` creates
  `load:L-300` as `primitive_user_load`, `draft`, `primitive_loads=0`
  through `OperationApplyPanel`, resets stale solve results, leaves
  persistence at `session_state_only_not_yet_saved`, and records no
  professional approval. Non-empty primitive payloads are rejected so future
  primitive-load creation remains explicit work.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 25/25 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 39/39 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered empty load-case create intent
  and confirmed `3 load cases; 7 primitive loads; 1 combinations`,
  `load:L-300; primitive_user_load; draft; primitives=0`, persistence
  remained `session_state_only_not_yet_saved`, professional approval remained
  false, and solve state remained `not_started`; `git diff --check -- .
  ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not add arbitrary primitive-load creation,
  imposed-displacement authoring, combination basis editing, combination term
  creation/deletion, code/rule combinations, broader algebra authoring, unit
  conversion, saved-project mutation without user Save, protected standards
  content, private data, or release, professional approval, certification,
  sealing, authentication, or code-compliance claims.

## TP-MAC-98 primitive-load-creation-editor (2026-06-11)

- Tranche: `TP-APP-R2-PRIMCREATE-001` (completion plan Phase A4 fifth
  sub-slice) adding explicit concentrated nodal-force primitive creation to
  the Load Cases manager. The create form captures load case, primitive id,
  existing node target, global direction, magnitude, and provenance, then
  queues a structured `create_primitive_load` operation.
- Edit path: applying
  `op:load-manager-load:L-100-load:L-100-F300-primitive` creates
  `load:L-100-F300` under `load:L-100` as `concentrated_force`,
  `target=node:N-100`, `global_y`, `dimension=force`, with magnitude in the
  project force unit. Duplicate primitive ids and missing node targets are
  blocked by validation. Distributed loads, concentrated moments,
  pressure/temperature primitive creation, and imposed displacements remain
  separate A4 residuals.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 26/26 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 41/41 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered primitive-load create intent
  and confirmed `2 load cases; 8 primitive loads; 1 combinations`,
  `load:L-100-F300; node:node:N-100; global_y; dimension=force`,
  persistence remained `session_state_only_not_yet_saved`, professional
  approval remained false, and solve state remained `not_started`; `git diff
  --check -- . ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not add distributed primitive-load creation,
  concentrated moments, pressure/temperature primitive creation,
  imposed-displacement authoring, combination basis editing, combination term
  creation/deletion, code/rule combinations, broader algebra authoring, unit
  conversion, saved-project mutation without user Save, protected standards
  content, private data, or release, professional approval, certification,
  sealing, authentication, or code-compliance claims.

## TP-MAC-99 distributed-primitive-load-creation-editor (2026-06-11)

- Tranche: `TP-APP-R2-DISTLOAD-001` (completion plan Phase A4 sixth
  sub-slice) adding explicit distributed element-force primitive creation to
  the Load Cases manager. The create form selects `distributed_force`,
  captures load case, primitive id, existing pipe target, global direction,
  magnitude, and provenance, then queues a structured `create_primitive_load`
  operation.
- Edit path: applying
  `op:load-manager-load:L-100-load:L-100-D300-primitive` creates
  `load:L-100-D300` under `load:L-100` as `distributed_force`,
  `target=pipe:P-100`, `global_y`, `dimension=force_per_length`, with
  magnitude in the project force/length unit (`N/m`). Duplicate primitive ids
  and missing pipe targets are blocked by validation. Concentrated moments,
  pressure/temperature primitive creation, and imposed displacements remain
  separate A4 residuals.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 27/27 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 43/43 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered distributed primitive-load
  create intent and confirmed `2 load cases; 8 primitive loads; 1
  combinations`, `load:L-100-D300; element:pipe:P-100; global_y;
  dimension=force_per_length`, persistence remained
  `session_state_only_not_yet_saved`, professional approval remained false,
  and solve state remained `not_started`; `git diff --check -- .
  ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not add concentrated moments,
  pressure/temperature primitive creation, imposed-displacement authoring,
  combination basis editing, combination term creation/deletion, code/rule
  combinations, broader algebra authoring, unit conversion, saved-project
  mutation without user Save, protected standards content, private data, or
  release, professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-100 concentrated-moment-load-creation-editor (2026-06-11)

- Tranche: `TP-APP-R2-MOMENTCREATE-001` (completion plan Phase A4 seventh
  sub-slice) adding explicit concentrated nodal-moment primitive creation to
  the Load Cases manager. The create form selects `concentrated_moment`,
  captures load case, primitive id, existing node target, rotational
  direction, magnitude, and provenance, then queues a structured
  `create_primitive_load` operation.
- Edit path: applying
  `op:load-manager-load:L-100-load:L-100-M300-primitive` creates
  `load:L-100-M300` under `load:L-100` as `concentrated_moment`,
  `target=node:N-100`, `rotation_z`, `dimension=moment`, with magnitude in
  the project force*length unit (`N*m`). Duplicate primitive ids and missing
  node targets are blocked by validation. Pressure/temperature primitive
  creation and imposed displacements remain separate A4 residuals.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 28/28 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 45/45 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered concentrated-moment
  primitive-load create intent and confirmed `2 load cases; 8 primitive
  loads; 1 combinations`, `load:L-100-M300; node:node:N-100; rotation_z;
  dimension=moment`, persistence remained `session_state_only_not_yet_saved`,
  professional approval remained false, and solve state remained
  `not_started`; `git diff --check -- . ':!init/init-prompt.md'` passed for
  the touched scope.
- Boundary: this slice does not add pressure/temperature primitive creation,
  imposed-displacement authoring, combination basis editing, combination term
  creation/deletion, code/rule combinations, broader algebra authoring, unit
  conversion, saved-project mutation without user Save, protected standards
  content, private data, or release, professional approval, certification,
  sealing, authentication, or code-compliance claims.

## TP-MAC-101 pressure-thermal-load-creation-editor (2026-06-11)

- Tranche: `TP-APP-R2-PRESSTEMP-001` (completion plan Phase A4 eighth
  sub-slice) adding explicit pressure and thermal primitive creation to the
  Load Cases manager. The create form selects `pressure` or `thermal`,
  captures load case, primitive id, existing pipe target, global direction,
  magnitude, and provenance, then queues a structured `create_primitive_load`
  operation.
- Edit path: applying
  `op:load-manager-load:L-100-load:L-100-P300-primitive` creates
  `load:L-100-P300` under `load:L-100` as `pressure`,
  `target=pipe:P-100`, `global_x`, `dimension=pressure`, with magnitude in
  the project pressure unit (`Pa`). Applying
  `op:load-manager-load:L-100-load:L-100-T300-primitive` creates
  `load:L-100-T300` as `thermal`, `target=pipe:P-100`, `global_z`,
  `dimension=temperature_interval`, with magnitude in the project temperature
  interval unit (`degC`). Duplicate primitive ids and missing pipe targets are
  blocked by validation.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 29/29 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 48/48 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered pressure and thermal
  primitive-load create intents in clean sessions and confirmed `2 load
  cases; 8 primitive loads; 1 combinations`,
  `load:L-100-P300; element:pipe:P-100; global_x; dimension=pressure`,
  `load:L-100-T300; element:pipe:P-100; global_z;
  dimension=temperature_interval`, persistence remained
  `session_state_only_not_yet_saved`, professional approval remained false,
  and solve state remained `not_started`; `git diff --check -- .
  ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not add gauge/absolute pressure conversion,
  reference-pressure defaults, thermal absolute-temperature conversion,
  imposed-displacement authoring, combination basis editing, combination term
  creation/deletion, code/rule combinations, broader algebra authoring, unit
  conversion, saved-project mutation without user Save, protected standards
  content, private data, or release, professional approval, certification,
  sealing, authentication, or code-compliance claims.

## TP-MAC-102 imposed-displacement-load-creation-editor (2026-06-11)

- Tranche: `TP-APP-R2-IMPOSED-001` (completion plan Phase A4 ninth sub-slice)
  adding explicit support-target imposed-displacement primitive creation to
  the Load Cases manager. The create form selects `imposed_displacement`,
  captures load case, primitive id, existing support target, support DOF,
  magnitude, and provenance, then queues a structured `create_primitive_load`
  operation.
- Edit path: applying
  `op:load-manager-load:L-100-load:L-100-I300-primitive` creates
  `load:L-100-I300` under `load:L-100` as `imposed_displacement`,
  `target=support:S-100`, `UZ`, `dimension=displacement`, with magnitude in
  the project length unit (`m`). Rotational DOF previews use the project angle
  unit (`rad`) and dimension `rotation`. Duplicate primitive ids, invalid
  DOFs, DOF/target mismatches, missing support targets, and missing unit
  metadata are blocked by validation.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 30/30 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 50/50 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered imposed-displacement
  primitive-load create intent with the default finite magnitude and confirmed
  `2 load cases; 8 primitive loads; 1 combinations`,
  `load:L-100-I300; support:support:S-100; UZ; dimension=displacement`,
  persistence remained `session_state_only_not_yet_saved`, professional
  approval remained false, and solve state remained `not_started`; `git diff
  --check -- . ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not add support coordinate policy, stiffness or
  default restraint behavior, solver boundary behavior, combination basis
  editing, combination term creation/deletion, code/rule combinations,
  broader algebra authoring, unit conversion, saved-project mutation without
  user Save, protected standards content, private data, or release,
  professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-103 combination-basis-editor (2026-06-11)

- Tranche: `TP-APP-R2-COMBBASIS-001` (completion plan Phase A4 tenth
  sub-slice) adding explicit basis editing for existing load combinations in
  the Load Cases manager. The selected-combination editor captures a
  replacement basis text value and rationale, then queues a structured
  `update_load` operation.
- Edit path: applying `op:load-manager-combination:C-OPER-ALT-basis` updates
  `combination:C-OPER-ALT` from `basis=mechanics` to
  `basis=mechanics_user_review` with unit `none` and dimension
  `dimensionless`. Existing combination terms and provenance are preserved.
  Whole `terms` replacement, term creation/deletion, code/rule combinations,
  and broader algebra authoring remain deferred by validation.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 31/31 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 52/52 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the rendered combination-basis edit and
  confirmed `basis=mechanics_user_review`, zero pending operations,
  `applied_operations=1`, and solve state `not_started`; `git diff --check --
  . ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not add whole-term replacement, combination term
  creation/deletion, public code-specific combination factors, protected
  standards content, rule-pack evaluator behavior, stress recovery, allowables,
  SIF/flexibility tables, broader algebra authoring, unit conversion,
  saved-project mutation without user Save, protected/private data handling,
  release readiness, professional approval, certification, sealing,
  authentication, or code-compliance claims.

## TP-MAC-104 combination-term-creation-editor (2026-06-11)

- Tranche: `TP-APP-R2-COMBTERMCREATE-001` (completion plan Phase A4 eleventh
  sub-slice) adding explicit child-term creation for existing load
  combinations in the Load Cases manager. The create form selects an existing
  combination, existing load case, finite dimensionless factor, and rationale,
  then queues a structured `create_combination_term` operation.
- Edit path: applying `op:load-manager-combination:C-OPER-ALT-term-2-create`
  appends one `{ load_case, factor }` term to `combination:C-OPER-ALT` with
  unit `none` and dimension `dimensionless`. The app regression creates
  `load:L-300`, appends it as `load:L-300 x 0.25`, and verifies existing terms
  `load:L-100 x 1` and `load:L-200 x 0.5` are preserved. Missing referenced
  load cases and invalid term payloads are blocked by validation.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 32/32 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 54/54 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied `load:L-300` creation, selected it in the
  combination-term form, applied
  `op:load-manager-combination:C-OPER-ALT-term-2-create`, and confirmed
  `load:L-300 x 1`, zero pending operations, `applied_operations=2`, solve
  state `not_started`, and no browser console errors; `git diff --check -- .
  ':!init/init-prompt.md'` passed for the touched scope.
- Boundary: this slice does not add whole-term replacement, combination term
  deletion, code/rule combinations, broader algebra authoring, unit
  conversion, saved-project mutation without user Save, protected standards
  content, private data, release readiness, professional approval,
  certification, sealing, authentication, or code-compliance claims.

## TP-MAC-105 combination-term-deletion-editor (2026-06-11)

- Tranche: `TP-APP-R2-COMBTERMDELETE-001` (completion plan Phase A4 twelfth
  sub-slice) adding explicit indexed child-term deletion for existing load
  combinations in the Load Cases manager. The selected-term editor captures a
  rationale, then queues a structured `delete_combination_term` operation.
- Edit path: applying
  `op:load-manager-combination:C-OPER-ALT-term-1-delete` removes indexed term
  `terms.1` (`load:L-200 x 0.5`) from `combination:C-OPER-ALT` with unit
  `none`, dimension `dimensionless`, and `after=not_present`.
  `load:L-100 x 1` remains. Stale before-values, out-of-range indices,
  invalid metadata, and missing combination `terms` arrays are blocked by
  validation.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 33/33 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 56/56 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the delete intent and confirmed
  `load:L-200 x 0.5` was no longer visible in the combination row,
  `load:L-100 x 1` remained, zero pending operations,
  `applied_operations=1`, solve state `not_started`, and no browser console
  errors; `git diff --check -- . ':!init/init-prompt.md'` passed for the
  touched scope.
- Boundary: this slice does not add whole-term replacement, code/rule
  combinations, broader algebra authoring, unit conversion, saved-project
  mutation without user Save, protected standards content, private data,
  release readiness, professional approval, certification, sealing,
  authentication, or code-compliance claims.

## TP-MAC-106 combination-creation-editor (2026-06-11)

- Tranche: `TP-APP-R2-COMBCREATE-001` (completion plan Phase A4 thirteenth
  sub-slice) adding explicit mechanics-basis combination creation to the Load
  Cases manager. The create form captures a new combination id, label, basis
  `mechanics`, existing load case, finite dimensionless factor, provenance,
  and rationale, then queues a structured `create_combination` operation.
- Edit path: applying `op:load-manager-create-combination:C-300` creates
  `combination:C-300` with `basis=mechanics` and initial term
  `load:L-100 x 1`. Duplicate combination ids, missing referenced load cases,
  empty terms, non-finite factors, duplicate initial operands, non-mechanics
  basis values, and invalid unit/dimension metadata are blocked by validation.
- Local validation: `cargo fmt --manifest-path
  core/model_operations/operation_applier/Cargo.toml --check` passed; `cargo
  test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed with 34/34 Rust tests; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests; `npm test
  --workspace apps/desktop` passed with 58/58 Vitest tests; `npm run build
  --workspace apps/desktop` passed through `tsc -b` and Vite production build
  with the existing chunk-size warning; `npm run test:e2e:desktop` passed
  with 1/1 Playwright tests; in-app browser smoke at
  `http://127.0.0.1:5175/` applied the create-combination intent and
  confirmed two combinations, `combination:C-300` with `basis=mechanics` and
  `load:L-100 x 1`, property inspector selection, zero pending operations,
  `applied_operations=1`, solve state `not_started`, and no browser console
  errors; `git diff --check -- . ':!init/init-prompt.md'` passed for the
  touched scope.
- Boundary: this slice does not add subtraction/range expression authoring,
  code/rule combinations, public code-specific factors, protected standards
  content, rule-pack evaluator behavior, unit conversion, saved-project
  mutation without user Save, private data, release readiness, professional
  approval, certification, sealing, authentication, or code-compliance claims.

## TP-MAC-107 persisted-edited-load-solve-regression (2026-06-11)

- Tranche: `TP-APP-R2-PERSISTEDSOLVE-001` (completion plan Phase A5 second
  sub-slice) adding a Tauri backend regression for saved edited-load data
  reaching the solve boundary after local persistence.
- Regression path: `apps/desktop/src-tauri/src/lib.rs` applies
  `op:test-load-L-100-Y-magnitude` through the structured operation seam,
  changing `load:L-100` `primitive_loads.1.magnitude.value` from `350` to
  `425` N, persists the edited model to an in-memory SQLite store with the
  current model-document schema status, reloads
  `project:edited-load-roundtrip`, and solves the restored payload through
  `solve_preview_mechanics`.
- Evidence checks: the restored model retains the `425` N magnitude, the
  restored solve reports `model_ref=project:edited-load-roundtrip`,
  mechanics status `MECHANICS_SOLVED`, non-empty result rows, and
  `result:disp:node-N-140` differs from the baseline fixture solve.
- Local validation: `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
  --check` passed; `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
  saved_edited_load_model_round_trips_and_solves_from_restored_payload`
  passed with 1/1 targeted test; `cargo test --manifest-path
  apps/desktop/src-tauri/Cargo.toml` passed with 27/27 Rust tests. No
  browser smoke was run because this tranche changes backend regression
  coverage only, not UI runtime behavior.
- Boundary: this slice does not complete packaged-Tauri GUI smoke, add a
  browser solver, change solver numerics, perform network/cloud/telemetry
  work, write repository-default private data, introduce protected standards
  content, or create release readiness, professional approval,
  certification, sealing, authentication, or code-compliance claims.

## TP-MAC-108 results-family-selector (2026-06-11)

- Tranche: `TP-APP-R2-RESULTFAMILY-001` (completion plan Phase A6 second
  sub-slice) adding direct result-family selectors to the desktop results
  panel.
- Result selector checks: after solving the invented preview model, the
  results panel reports `647 of 647 results match filter`; family counts show
  `Displacement=15`, `Reaction=9`, `Force=180`, `Moment=180`, and
  `Stress=263`. Selecting `Reaction` reports `9 of 647 results match filter`,
  `Showing 1 to 9 of 9 matching results; page 1 of 1`, and shows
  `result:reaction:support-S-120`. Selecting `Stress` reports
  `263 of 647 results match filter`, `Showing 1 to 50 of 263 matching
  results; page 1 of 6`, and shows
  `result:stress:pipe-P-120:end-i:torsional-shear`. Returning to `All`
  restores `647 of 647 results match filter`.
- Local validation: `npm test --workspace apps/desktop` passed with 58/58
  Vitest tests; `npm run build --workspace apps/desktop` passed through
  `tsc -b` and Vite production build with the existing chunk-size warning;
  `npm run test:e2e:desktop` passed with 1/1 Playwright tests; in-app browser
  smoke at `http://127.0.0.1:5175/` confirmed solved result-family selection,
  non-overflowing family selector buttons, all-family restoration, and zero
  browser console errors.
- Boundary: this slice does not add governing-ratio result generation, rule
  checks, protected standards data, private data, release readiness,
  professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-109 operation-contract-corpus (2026-06-11)

- Tranche: `TP-SEAM-CORPUS-001` (operation-seam unification plan T1 plus its
  boundary-hygiene rider) adding the cross-engine operation contract corpus
  and removing the local-FEA handoff fixture-ID fallback.
- Contract corpus checks: 44 invented cases at
  `fixtures/model_operations/contract_corpus/` execute through both engines —
  the Rust contract reference via
  `core/model_operations/operation_applier/tests/contract_corpus.rs` under
  plain `cargo test`, and the browser mirror via
  `apps/desktop/src/services/operationContractCorpus.test.ts` under plain
  `npm test` — asserting identical semantic outcomes (validation states,
  diff-preview rows, order-insensitive diagnostic records), parsed-JSON deep
  equality of applied documents, and one shared corpus-harness canonical
  sha256 per applied document (the hash-stability guarantee). The coverage
  floor is enforced programmatically in both runners: all 10 operation kinds
  accepted, the named block classes (19 distinct blocking codes recorded),
  and dynamic paths `primitive_loads.N.magnitude.value` / `terms.N.factor`
  each accepted and blocked. The TypeScript engine reproduced all 44
  recorded Rust outcomes with zero alignment fixes (`operationService.ts`
  unchanged).
- Handoff rider checks: `LocalFeaHandoffPanel` no longer substitutes
  `pipe:P-120` / `node:N-140` when result-summary location refs are missing;
  it emits the explicit `LOCAL-FEA-RESULT-SUMMARY-REF-MISSING` finding with
  an honestly degraded region id, and keeps real references plus the
  standing four findings when refs are present (two Vitest cases).
- Local validation: `cargo test` (operation_applier) passed 36/36 (34 lib +
  2 corpus tests); `npm test --workspace apps/desktop` passed 105/105 Vitest
  tests across 7 files; `npm run test:e2e:desktop` passed 1/1 Playwright
  tests unchanged; `npm run build --workspace apps/desktop` passed through
  `tsc -b` and Vite production build with the existing chunk-size warning.
- Boundary: all corpus data is invented with `PUBLIC_DOMAIN_OR_ORIGINAL`
  provenance recorded in the corpus README (`review_status` pending human
  review); no protected standards content, code-specific values, network
  surfaces, release readiness, professional approval, certification,
  sealing, authentication, or code-compliance claims. The plan §5 freeze
  rule was honored: no new operation kinds or field rules entered the TS
  engine.

## TP-MAC-110 wasm-engine-swap (2026-06-11)

- Tranche: `TP-SEAM-WASM-001` (T3) + `TP-SEAM-SWAP-001` (T4) of the
  operation-seam unification plan (`DEC-020` / ADR-0001): the wasm32 build
  of `core/model_operations/operation_applier` is now the sole browser-mode
  operation engine, and the TypeScript engine no longer exists.
- Engine swap checks: `operationService.ts` reduced from 2,280 lines (full
  validation/diff/apply engine with private field-rule, dimension, and
  restraint tables) to a 112-line thin routing adapter — Tauri present →
  `invoke` (authoritative, unchanged); otherwise the wasm engine. Net
  deletion across the engine and its superseded 17-test engine spec: −3,114
  lines, +200 lines (adapter + 6 thin adapter tests covering routing, the
  wasm input-error envelope as an explicit thrown error, and engine-status
  reporting). Intent builders, types, and UI are retained.
- Honest receipts: browser-mode `application_route` is now
  `local_wasm_engine` (type union updated; UI receipt line and App test
  assertion updated). The Apply Operations panel reports
  `engine_route`/`engine_state` via the new `operation-engine-status`
  testid; an absent wasm artifact reports `unavailable` with the
  `WASM-ENGINE-ASSET-ABSENT` diagnostic and exact build command — no
  fallback engine exists.
- Corpus continuity: the 44-case contract corpus now runs two browser lanes
  — through the public adapter seam (wasm-backed) and through the wasm
  engine directly — against the Rust-blessed native reference, preserving
  semantic and canonical-hash parity as the permanent native↔wasm
  regression surface. Vitest setup pre-warms the engine; `test:e2e` builds
  the wasm artifact first.
- Playwright extension: the smoke spec waits on the engine-ready status,
  then applies the prepared explicit viewport node intent in a real Chrome
  browser and asserts the `route=local_wasm_engine` receipt and the
  by-design clearing of stale solve results.
- Local validation: cargo profile sweep
  (`tools/release/check_release_readiness.py --profile cargo --execute`)
  passed across all 25 discovered crate manifests (51 green test-result
  lines, exit 0); `python3 -m pytest -q tests` passed 342/342 (unaffected);
  `npm test --workspace apps/desktop` passed 140/140 across 7 files (was
  151: −17 superseded engine tests, +6 adapter tests); `npm run
  test:e2e:desktop` passed 1/1 including the new engine-route assertions;
  `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build (index chunk at the existing ~577 kB baseline with the
  standing chunk-size warning; the wasm engine loads dynamically and is not
  bundled).
- Boundary review: wasm runs in-process (local-only; no cloud, daemon,
  network, or telemetry surface); invented corpus and fixture data only; no
  protected standards content; no release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claims. The
  plan §5 freeze rule was honored through the swap and is lifted at this
  landing: browser-mode operation work now lands in the single Rust engine.

## TP-MAC-111 seam-verification regression repair (2026-06-11)

- Tranche `TP-SEAM-FIX-001`: human-instructed repair of findings F-1/F-2
  from `plans/VERIFICATION_2026-06-11_operation_seam_unification.md`
  (post-closure verification of the operation-seam plan). F-3 (corpus
  fixture review disposition) remains a human action; F-4 stands as an
  operational note for `D-05`.
- F-1 (Playwright 30s timeout exceeded): root cause measured — the trace
  recorder (`trace: "retain-on-failure"`), not the wasm engine. Spec runs
  8.8s with trace off vs 21.6s warm / 37.4s cold with trace on;
  `DEBUG=pw:api` attributed ~21s of the cold run to trace finalization in
  `browser.close`, with the engine-ready wait under 1s. Fixed by raising
  the per-test budget to 120s in `playwright.config.ts` (comment records
  the measurements); tracing retained for failure diagnosis; spec not
  split (its ordering encodes the apply-clears-solve-results design).
- F-2 (raw NUL bytes): the two literal `0x00` separators in
  `operationContractCorpus.test.ts` `diagnosticSortKey` are now `\u0000`
  escapes; the file contains zero NUL bytes, so git diffs of future
  changes render as reviewable text.
- Local validation (sequential): `npm run test:e2e` 1/1 at 18.8s under the
  default command and config; Vitest 140/140; cargo profile sweep exit 0
  with zero failing test-result lines; pytest 342/342; desktop production
  build green.
- Boundary review: test-infrastructure-only; no engine, fixture, or UI
  behavior change; local-only; no protected content; no release-readiness,
  professional approval, certification, sealing, authentication, or
  code-compliance claims.

## TP-MAC-112 DEC-025 evidence sweep + F-4 atomic wasm build (2026-06-11)

- Tranche `TP-SWEEP-001`: implementation of the `DEC-025` ruling (D-05
  Option D) — the five-surface local evidence sweep codified as one
  deterministic entrypoint, `tools/release/run_evidence_sweep.py`, plus
  the F-4 atomic temp-write-and-rename fix in
  `apps/desktop/scripts/build-wasm-engine.mjs`.
- The sweep runs sequentially in F-4-safe order: cargo crate sweep,
  pytest, desktop Vitest (wasm engine built first), Playwright e2e,
  desktop production build. Fail-fast; unexecuted surfaces are recorded
  `not_run`. Each execute run writes a commit-bound JSON summary to
  `validation/evidence/sweeps/` (exit 0 only when all five pass). This
  sweep is the required pre-push/fan-in merge gate for parallel agent
  development branches (`docs/BUILD_AND_RELEASE.md` §5.1).
- F-4: wasm-bindgen glue is generated into a sibling `.tmp-<pid>` staging
  dir and renamed into place, so concurrent readers never observe a
  half-written `__generated__/` artifact set; stale staging dirs are
  cleaned on the next build.
- Local validation (single sweep run, working tree, exit 0): cargo crate
  sweep pass; pytest pass (includes 11 new
  `tests/test_evidence_sweep.py` tests); Vitest pass; Playwright e2e 1/1
  (17.5s); production build green (index chunk at the standing ~536 kB
  baseline with the chunk-size warning). Summary artifact:
  `validation/evidence/sweeps/SWEEP_20260612T031241Z_0f402fc48424-dirty.json`.
  The post-commit gate run at the clean tranche HEAD adds the clean
  summary alongside it.
- Boundary review: local-only evidence tooling; no hosted CI workflow,
  network, or telemetry surface; invented data only; no protected
  standards content; no release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claims.

## TP-MAC-113 rendered-report-fr016 (2026-06-11)

- Tranche `TP-APP-R2-REPORTRENDER-001` (completion plan A7, per `DEC-021`
  / D-10 Option B): the FR-016 calculation report now renders as a
  deterministic, self-contained, scriptless single-file HTML document
  from the new `core/reporting/report_renderer` crate, SHA-256 hash-bound
  and gated by the protected-content linter at three points (template
  surface, pre-render section text, post-render document text); blocking
  findings or blocking validation diagnostics refuse export with a
  visible `EXPORT BLOCKED` banner and named reasons.
- Desktop seam: new `render_calculation_report` Tauri command; new
  Rendered Report panel (render → canonical hash + gate state → save
  canonical HTML / print derived view). The print/PDF path emits the
  derived view that names the canonical hash and is labeled non-evidence.
  Browser preview reports the explicit `REPORT-RENDERER-DESKTOP-ONLY`
  route — no fallback renderer exists.
- Adapter composes the renderer input from session envelopes (run-record
  and result-envelope hashes, load cases, diagnostics, reproducibility
  TBDs) with explicit `TBD` markers (e.g. persistence ref before the
  project is saved) and locally computed canonical-JSON SHA-256 values.
- Local validation: renderer crate 8/8; report_generator 10/10 and
  report_sections 13/13 (serde feature added, default-off); src-tauri
  command test 1/1; Vitest 148/148 (8 new); desktop production build
  green; pytest report contracts 22/22. Full five-surface sweep evidence
  recorded at the tranche push (commit-bound summary under
  `validation/evidence/sweeps/`).
- Boundary review: local-only rendering, no network/scripts/external
  refs; invented or user-local session data only; empty human signoff
  fields; no release-readiness, professional approval, certification,
  sealing, authentication, or code-compliance claims.

## TP-MAC-114 H1 canonical-hash unification (2026-06-11)

- Tranche `TP-H1-HASHUNIFY-001` (completion-plan hardening lane H1;
  verification F-5a): frontend hashing now routes through the wasm build
  of the engine's `canonical_json`/`sha256_hex` via new wasm exports
  (`canonical_json_string`, `canonical_sha256_hex`). `hashService.ts`
  holds no canonicalization or digest code; the A7 report-input local
  WebCrypto helper (with its silent `"TBD"` soft fallback) is deleted; no
  fallback hashing path exists.
- New Rust-blessed parity corpus `fixtures/canonical_hash/` (14 cases,
  12-case floor, `CANONICAL_HASH_BLESS=1` regeneration): native and wasm
  lanes assert identical canonical text and sha256 on identical raw input
  text, pinning the engine's number normalization (`1e9` ->
  `1000000000.0`).
- Allowlist rider measured: `backend_canonicalization` joined the blessed
  corpus projection (44 cases re-blessed); the native runner gained an
  engine self-consistency hash assertion. The `backend_model_hash` value
  exclusions stand for a measured reason now documented in the corpus
  README: JS transport renders `200.0` as `200`, and the engine's
  applied-model hash covers in-process text reproducible only Rust-side.
- Local validation: operation_applier cargo suites green (unit + corpus +
  parity); desktop Vitest 166/166 (was 140); DEC-025 five-surface sweep
  overall: pass (cargo sweep, pytest 342, Vitest, Playwright e2e,
  production build; summary artifact in `validation/evidence/sweeps/`).
- Boundary review: local-only; invented values; no protected content; no
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claims.

## TP-MAC-115 H5 RFC 8785 canonical rendering (2026-06-11)

- Tranche `TP-H5-JCSRENDER-001` (completion-plan hardening lane H5,
  human-directed; DEC-010 JCS alignment): the engine's `canonical_json`
  is true RFC 8785 (JCS) via the new shared
  `core/serialization/canonical_json` crate — ECMAScript
  `Number::toString` rendering (ryu digit selection; ties to even),
  UTF-16 code-unit key sort, `JSON.stringify` escaping. All three
  JSON-text producers (JS transport, raw-file serde parse, engine-internal
  rendering) now emit identical canonical bytes for identical values.
- Both `TP-H1-HASHUNIFY-001` measured exclusions are retired: the corpus
  ECMA harness renderer twins and the number-range constraint are
  deleted; `backend_model_hash` and `applied_model_backend_hash` values
  are byte-equality-compared against blessed expectations in every lane.
  `fixtures/canonical_hash/` extended 14 -> 20 cases (notation
  boundaries, shortest-tie-to-even, −0, beyond-2^53 integer pin, UTF-16
  key sort); all 44 contract cases re-blessed. Canonicalization labels
  unified to `rfc8785_jcs` (engine constant, hashService/types,
  previewService, report input, headless runner + schema enum).
- No interactive desktop behavior changed; per the H4 evidence posture
  the user-visible deltas (hash values and the `rfc8785_jcs` label inside
  evidence packets) are pinned by unit suites and the cross-lane corpus,
  so no Playwright spec extension was made — existing e2e specs assert
  behavior, not hash vocabulary.
- Local validation: canonical_json crate 8 tests (RFC 8785 Appendix B
  vectors node-verified; one-shot 112,220-vector node fuzz clean);
  operation_applier suites green at re-blessed expectations; desktop
  Vitest 172/172; headless 11, audit_manifest 13, src-tauri 28, pytest
  353. DEC-025 five-surface sweep at `0b7a24c93`: overall pass (cargo
  sweep across 28 manifests incl. the new crate, pytest, Vitest with
  fresh wasm artifact, Playwright e2e, production build; summary
  `validation/evidence/sweeps/SWEEP_20260612T053738Z_0b7a24c93e2c-dirty.json`
  — dirty delta is solely the human-owned `init/init-prompt.md`).
- Boundary review: local-only; invented values; no protected content; no
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claims.

## TP-MAC-116 blank-project authoring target (2026-06-12)

- Tranche `TP-APP-R2-BLANK-001` (completion-plan Phase A9): the local project
  toolbar now exposes `New blank`, creating a user-owned local blank model
  document as the active authoring target instead of cloning the bundled
  invented preview fixture.
- Live in-app browser smoke at `http://127.0.0.1:5173/` verified the blank
  action produced `project:blank-local-*`, project name `Blank Local Model`,
  `MODEL INCOMPLETE`, `RULE INPUTS INCOMPLETE`, `NOT PROVIDED`, one visible
  project entity in the model tree, and boundary-strip text
  `blank_user_created_local_document_no_bundled_engineering_values`,
  `no_protected_standards_content_inserted`, and
  `local_user_document_not_committed_to_repository`.
- Storage and validation evidence after creation showed
  `operation=create_blank`, `pending operations=0`, browser-memory local
  storage, `persisted_mechanics_results=0`, no external file copies,
  `network=false`, and no repository-default private-data write.
- Browser-mode solve over the blank authoring target completed with
  `state=completed`, `result_rows=0`, `MODEL INCOMPLETE`, blocking diagnostic
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, result table summary
  `0 of 0 results match filter`, and report export summary
  `0 refs; 4 diagnostics; no private payload`.
- Console review during the live smoke reported zero browser console errors.
- Automated evidence: projectService Vitest 5/5; App Vitest 32/32 focused and
  full desktop Vitest 174/174; Tauri Rust tests 29/29 including blank supplied
  payload returning `NODE_INPUT_MISSING`, `PIPE_INPUT_MISSING`, and
  `LOAD_INPUT_MISSING`; production build green; Playwright R2 smoke 1/1.
- Boundary review: local-only; no cloud, daemon, network, telemetry, protected
  content, repository-default private-data write, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-117 support creation authoring (`TP-APP-R2-CREATESUPPORT-001`, 2026-06-12)

- Tranche `TP-APP-R2-CREATESUPPORT-001` (completion-plan Phase A10 first
  sub-slice): the structured operation seam now accepts `create_support`
  intents, validates explicit support payloads, blocks duplicate support ids,
  blocks missing node references, and reuses the UX/UY/UZ/RX/RY/RZ restraint
  vocabulary. The contract corpus now has 45 cases and requires an accepted
  `create_support` apply in both Rust and browser wasm lanes.
- Property Inspector smoke at `http://127.0.0.1:5173/` selected
  `node:N-100`, authored `support:S-151` with label `Browser guide support`,
  node target `node:N-100`, restraints `UX, UY, UZ, RX`, and provenance
  `user_entered_local_preview`, then queued and applied the intent through the
  Apply Operations panel.
- Live browser evidence showed the created support row active in the model
  tree, the inspector showing `node:N-100` and `UX, UY, UZ, RX`, review context
  `0 pending operations; applied_operations=1`, acceptance
  `user_initiated_apply_in_local_session`, persistence
  `session_state_only_not_yet_saved`, and `professional_approval=false`.
- Console review during the live Chrome smoke reported zero browser console
  errors.
- Automated evidence: operation_applier cargo suites green (36 unit tests,
  canonical hash parity, 45-case corpus); desktop operationContractCorpus
  Vitest 93/93; focused App Vitest 1/1; full desktop Vitest 177/177; desktop
  production build green; Tauri Rust tests 29/29; Playwright R2 smoke 1/1.
- Boundary review: local-only; invented values; no cloud, daemon, network,
  telemetry, protected content, repository-default private-data write,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-118 material creation authoring (`TP-APP-R2-CREATEMATERIAL-001`, 2026-06-12)

- Tranche `TP-APP-R2-CREATEMATERIAL-001` (completion-plan Phase A10 second
  sub-slice): the structured operation seam now accepts `create_material`
  intents, validates explicit material payloads, blocks duplicate material
  ids, requires elastic/shear quantities in the project pressure unit, and
  accepts an optional thermal-expansion coefficient in `1/{temperature_unit}`.
  The contract corpus now has 46 cases and requires an accepted
  `create_material` apply in both Rust and browser wasm lanes.
- Property Inspector smoke at `http://127.0.0.1:5173/` authored
  `material:M-301` with label `Browser alloy material`, elastic modulus
  `125000000000 Pa`, shear modulus `48000000000 Pa`, thermal expansion
  `0.00001 1/degC`, and provenance `user_entered_local_preview`, then queued
  and applied the intent through the Apply Operations panel.
- Live browser evidence showed the created material row active in the model
  tree, the inspector showing the elastic, shear, and thermal-expansion
  quantities, review context `0 pending operations; applied_operations=1`,
  acceptance `user_initiated_apply_in_local_session`, persistence
  `session_state_only_not_yet_saved`, and `professional_approval=false`.
- Console review during the live Chrome smoke reported zero browser console
  errors.
- Automated evidence: operation_applier cargo suites green (38 unit tests,
  canonical hash parity, 46-case corpus); desktop operationContractCorpus
  Vitest 95/95; focused App Vitest 1/1; full desktop Vitest 180/180; desktop
  production build green; Tauri Rust tests 29/29; Playwright R2 smoke 1/1.
- Boundary review: local-only; invented values; no cloud, daemon, network,
  telemetry, protected content, repository-default private-data write,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-119 section creation authoring (`TP-APP-R2-CREATESECTION-001`, 2026-06-12)

- Tranche `TP-APP-R2-CREATESECTION-001` (completion-plan Phase A10 third
  sub-slice): the structured operation seam now accepts `create_section`
  intents, validates explicit pipe-section payloads, blocks duplicate section
  ids, requires outside-diameter and wall-thickness quantities in the project
  length unit, and blocks wall thickness at or beyond the outside-diameter
  radius. The contract corpus now has 47 cases and requires an accepted
  `create_section` apply in both Rust and browser wasm lanes.
- Property Inspector smoke at `http://127.0.0.1:5173/` authored
  `section:S-301` with name `Browser pipe section`, type `pipe`, outside
  diameter `0.114 m`, wall thickness `0.006 m`, and provenance
  `user_entered_local_preview`, then queued and applied the intent through the
  Apply Operations panel.
- Live browser evidence showed the created section row active in the model
  tree, the inspector showing the section name and OD/wall quantities, review
  context `0 pending operations; applied_operations=1`, acceptance
  `user_initiated_apply_in_local_session`, persistence
  `session_state_only_not_yet_saved`, and `professional_approval=false`.
- Console review during the live Chrome smoke reported zero browser console
  errors.
- Automated evidence: operation_applier cargo suites green (40 unit tests,
  canonical hash parity, 47-case corpus); desktop operationContractCorpus
  Vitest 97/97; focused App Vitest 1/1; blank-project service Vitest 5/5; full
  desktop Vitest 183/183; desktop production build green; Tauri Rust tests
  29/29; Playwright R2 smoke 1/1.
- Boundary review: local-only; invented values; standalone section creation
  only; no cloud, daemon, network, telemetry, protected content,
  repository-default private-data write, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-120 support deletion authoring (`TP-APP-R2-DELSUPPORT-001`, 2026-06-12)

- Tranche `TP-APP-R2-DELSUPPORT-001` (completion-plan Phase A11 first
  sub-slice): the structured operation seam now accepts `delete_support`
  intents, validates explicit support deletion metadata, removes only the
  selected unreferenced support, and blocks deletion with
  `OP-SUPPORT-DELETE-REFERENCED` when an imposed-displacement primitive load
  still targets the support. The contract corpus now has 49 cases and
  requires accepted `delete_support` plus the referenced-support block in
  both Rust and browser wasm lanes.
- Property Inspector smoke at `http://127.0.0.1:5173/` selected
  `support:S-120`, queued `op:delete-support-support:S-120`, and applied the
  intent through the Apply Operations panel.
- Live browser evidence showed `support:S-120` removed from the model tree,
  the project row active as the post-delete selection fallback, review context
  `0 pending operations; applied_operations=1`, acceptance
  `user_initiated_apply_in_local_session`, persistence
  `session_state_only_not_yet_saved`, and `professional_approval=false`.
- Console review during the live Chrome smoke reported zero browser console
  errors.
- Automated evidence: operation_applier cargo suites green (42 unit tests,
  canonical hash parity, 49-case corpus); desktop operationContractCorpus
  Vitest 101/101; focused App Vitest 2/2; full desktop Vitest 189/189;
  desktop production build green; Tauri Rust tests 29/29; Playwright R2 smoke
  1/1.
- Boundary review: local-only; invented values; support deletion only; no
  cloud, daemon, network, telemetry, protected content, repository-default
  private-data write, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-121 primitive-load deletion authoring (`TP-APP-R2-DELPRIMLOAD-001`, 2026-06-12)

- Tranche `TP-APP-R2-DELPRIMLOAD-001` (completion-plan Phase A11 second
  sub-slice): the structured operation seam now accepts
  `delete_primitive_load` intents, validates indexed primitive-load deletion
  metadata, and removes only the selected primitive load from the owning load
  case. The contract corpus now has 50 cases and requires accepted
  `delete_primitive_load` in both Rust and browser wasm lanes.
- Load Cases manager smoke at `http://127.0.0.1:5173/` selected
  `load:L-100-Y`, queued
  `op:load-manager-load:L-100-load:L-100-Y-delete`, and applied the intent
  through the Apply Operations panel.
- Live browser evidence showed `load:L-100-Y` removed from the primitive-load
  list, load-case summary changed to `2 load cases; 6 primitive loads; 1
  combinations`, `load:L-100` primitive count changed to `3`, review context
  `0 pending operations; applied_operations=1`, acceptance
  `user_initiated_apply_in_local_session`, persistence
  `session_state_only_not_yet_saved`, and `professional_approval=false`.
- Console review during the live Chrome smoke reported zero browser console
  errors.
- Automated evidence: operation_applier cargo suites green (43 unit tests,
  canonical hash parity, 50-case corpus); desktop operationContractCorpus
  Vitest 103/103; focused App Vitest 1/1; full desktop Vitest 192/192;
  desktop production build green; Tauri Rust tests 29/29; Playwright R2 smoke
  1/1.
- Boundary review: local-only; invented values; primitive-load deletion only;
  no cloud, daemon, network, telemetry, protected content, repository-default
  private-data write, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-122 full-combination deletion authoring (`TP-APP-R2-DELCOMBINATION-001`, 2026-06-12)

- Tranche `TP-APP-R2-DELCOMBINATION-001` (completion-plan Phase A11 third
  sub-slice): the structured operation seam now accepts `delete_combination`
  intents, validates whole-combination deletion metadata, and removes only
  the selected combination from the model. The contract corpus now has 51
  cases and requires accepted `delete_combination` in both Rust and browser
  wasm lanes.
- Load Cases manager smoke at `http://127.0.0.1:5173/` selected
  `combination:C-OPER-ALT`, queued
  `op:load-manager-combination:C-OPER-ALT-delete`, and applied the intent
  through the Apply Operations panel.
- Live browser evidence showed `combination:C-OPER-ALT` removed from the
  combination list, load-case summary changed to `2 load cases; 7 primitive
  loads; 0 combinations`, review context `0 pending operations;
  applied_operations=1`, acceptance `user_initiated_apply_in_local_session`,
  persistence `session_state_only_not_yet_saved`, and
  `professional_approval=false`.
- Console review during the live in-app browser smoke reported zero browser
  console errors.
- Automated evidence: operation_applier cargo suites green (44 unit tests,
  canonical hash parity, 51-case corpus); desktop operationContractCorpus
  Vitest 105/105; focused App Vitest 1/1; full desktop Vitest 195/195;
  desktop production build green; Tauri Rust tests 29/29; Playwright R2 smoke
  1/1 with whole-combination delete preview coverage.
- Boundary review: local-only; invented values; whole-combination deletion
  only; no cloud, daemon, network, telemetry, protected content,
  repository-default private-data write, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-123 load-case deletion authoring (`TP-APP-R2-DELLOADCASE-001`, 2026-06-12)

- Tranche `TP-APP-R2-DELLOADCASE-001` (completion-plan Phase A11 fourth
  sub-slice): the structured operation seam now accepts `delete_load_case`
  intents, validates whole-load-case deletion metadata, refuses deletion while
  any combination term still references the load case, and removes only the
  selected unreferenced load case from the model. The contract corpus now has
  53 cases and requires accepted `delete_load_case` plus blocking
  `OP-LOAD-CASE-DELETE-REFERENCED` coverage in both Rust and browser wasm
  lanes.
- Load Cases manager smoke at `http://127.0.0.1:5173/` created
  `load:L-300`, selected that unreferenced load case, queued
  `op:load-manager-load:L-300-delete`, and applied the intent through the
  Apply Operations panel.
- Live browser evidence showed the load-case summary move from `2 load
  cases; 7 primitive loads; 1 combinations` to `3 load cases; 7 primitive
  loads; 1 combinations`, then back to `2 load cases; 7 primitive loads; 1
  combinations`; review context `0 pending operations; applied_operations=2`;
  acceptance `user_initiated_apply_in_local_session`; persistence
  `session_state_only_not_yet_saved`; and `professional_approval=false`.
- Console review during the live in-app browser smoke reported zero browser
  console errors.
- Automated evidence: operation_applier cargo suites green (45 unit tests,
  canonical hash parity, 53-case corpus); desktop operationContractCorpus
  Vitest 109/109; focused App Vitest 2/2; full desktop Vitest 201/201;
  desktop production build green; Tauri Rust tests 29/29; Playwright R2 smoke
  1/1 with load-case delete preview coverage.
- Boundary review: local-only; invented values; whole-load-case deletion only
  for unreferenced load cases; no hidden combination-term cascade; no cloud,
  daemon, network, telemetry, protected content, repository-default
  private-data write, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-124 pipe-run deletion authoring (`TP-APP-R2-DELPIPE-001`, 2026-06-12)

- Tranche `TP-APP-R2-DELPIPE-001` (completion-plan Phase A11 fifth
  sub-slice): the structured operation seam now accepts `delete_pipe_run`
  intents, validates whole pipe-run deletion metadata, refuses deletion while
  any primitive load still references the pipe, and removes only the selected
  unreferenced pipe from the model. The contract corpus now has 55 cases and
  requires accepted `delete_pipe_run` plus blocking
  `OP-PIPE-DELETE-REFERENCED` coverage in both Rust and browser wasm lanes.
- Property Inspector smoke at `http://127.0.0.1:5173/` selected
  `pipe:P-130`, queued `op:delete-pipe-pipe:P-130`, and applied the intent
  through the Apply Operations panel.
- Live browser evidence showed `pipe:P-130` removed from the model tree, the
  project row active as the post-delete selection fallback, review context
  `0 pending operations; applied_operations=1`, acceptance
  `user_initiated_apply_in_local_session`, persistence
  `session_state_only_not_yet_saved`, and `professional_approval=false`.
- Console review during the live in-app browser smoke reported zero browser
  console errors.
- Automated evidence: operation_applier cargo suites green (47 unit tests,
  canonical hash parity, 55-case corpus); desktop operationContractCorpus
  Vitest 113/113; focused App Vitest 2/2; full desktop Vitest 207/207;
  desktop production build green; Tauri Rust tests 29/29; Playwright R2 smoke
  1/1 with pipe delete preview coverage.
- Boundary review: local-only; invented values; whole-pipe deletion only for
  unreferenced pipes; no hidden primitive-load cascade; no cloud, daemon,
  network, telemetry, protected content, repository-default private-data
  write, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim.

## TP-MAC-125 node deletion authoring (`TP-APP-R2-DELNODE-001`, 2026-06-12)

- Tranche `TP-APP-R2-DELNODE-001` (completion-plan Phase A11 final
  sub-slice): the structured operation seam now accepts `delete_node`
  intents, validates whole node deletion metadata, refuses deletion while any
  pipe endpoint, support, component symbol, or primitive load still references
  the node, and removes only the selected unreferenced node from the model.
  The contract corpus now has 57 cases and requires accepted `delete_node`
  plus blocking `OP-NODE-DELETE-REFERENCED` coverage in both Rust and browser
  wasm lanes.
- Property Inspector smoke at `http://127.0.0.1:5173/` created
  `node:N-160`, queued `op:delete-node-node:N-160`, and applied the intent
  through the Apply Operations panel.
- Live browser evidence showed `node:N-160` removed from the model tree, the
  project row active as the post-delete selection fallback, review context
  `0 pending operations; applied_operations=2`, acceptance
  `user_initiated_apply_in_local_session`, persistence
  `session_state_only_not_yet_saved`, and `professional_approval=false`.
- Console review during the live in-app browser smoke reported zero browser
  console errors.
- Automated evidence: operation_applier cargo suites green (49 unit tests,
  canonical hash parity, 57-case corpus); desktop operationContractCorpus
  Vitest 117/117; focused App Vitest 2/2; full desktop Vitest 213/213;
  desktop production build green; Tauri Rust tests 29/29; Playwright R2 smoke
  1/1 with node delete preview coverage.
- Boundary review: local-only; invented values; whole-node deletion only for
  unreferenced nodes; no hidden pipe/support/component/load cascade; no cloud,
  daemon, network, telemetry, protected content, repository-default
  private-data write, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-126 from-blank R2 exit rehearsal (`TP-APP-R2-FROMBLANK-REHEARSAL-001`, 2026-06-12)

- Tranche `TP-APP-R2-FROMBLANK-REHEARSAL-001` (completion-plan A12): added
  invented script fixture
  `fixtures/product_preview/r2_from_blank_rehearsal.json` and Tauri
  regression `r2_from_blank_rehearsal_authors_solves_and_renders_report`.
  The script starts from a blank local model and applies structured operation
  steps for nodes, material, standalone section, pipe run, support, load case,
  primitive load, and mechanics combination.
- Backend rehearsal evidence: every fixture step applies through
  `apply_model_operation` with `applied_to_session_model` and no professional
  approval claim; the authored model solves through `run_preview_mechanics`
  with `MECHANICS_SOLVED`, emits combination-basis result rows, and renders
  through the A7 `render_calculation_report` command with export gates
  unblocked and a 64-character canonical HTML SHA-256.
- Integration repair found by the rehearsal: product physics now maps
  operation-authored primitive-load categories `concentrated_force`,
  `concentrated_moment`, and `distributed_force` into the existing equivalent
  static preview mechanics categories, so A4-created primitives can feed the
  A5 backend solve path.
- Automated evidence: `python3 -m json.tool` on the rehearsal fixture;
  product_physics cargo tests 24/24; Tauri Rust tests 30/30; desktop Vitest
  213/213; desktop production build green; Playwright R2 smoke 1/1 after
  wasm engine build.
- Residual A8 hand-off: browser fixture mode still intentionally refuses to
  publish solved rows for edited models without the Tauri backend. The next
  harness tranche should automate this A12 script as the GUI/e2e journey
  backbone. Boundary review: invented public example only; no cloud, daemon,
  network, telemetry, protected content, repository-default private-data
  write, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim.

## TP-MAC-127 from-blank GUI journey automation (`TP-APP-R2-FROMBLANK-E2E-001`, 2026-06-12)

- Tranche `TP-APP-R2-FROMBLANK-E2E-001` (completion-plan A8): extended the
  Playwright harness with a second browser e2e test that reads
  `fixtures/product_preview/r2_from_blank_rehearsal.json`, clicks `New blank`,
  and authors the A12 script through visible viewport, property-inspector, and
  load-manager controls.
- Automated coverage: two nodes, one material, one standalone section, one
  pipe run, one support, one load case, one primitive load, and one mechanics
  combination are queued and applied through the Apply Operations panel. Each
  applied operation asserts `route=local_wasm_engine` and
  `professional_approval=false`.
- Browser boundary evidence: after the authored model is run in browser mode,
  the harness asserts `result_rows=0` and
  `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, then clicks rendered
  report and asserts `REPORT-RENDERER-DESKTOP-ONLY`. This is an honest
  browser harness boundary, not a backend solve/render claim.
- Automated evidence: `npm run test:e2e --workspace apps/desktop` passed with
  2/2 Playwright tests after wasm engine build; `npm test --workspace
  apps/desktop` passed with 213/213 Vitest tests; `npm run build --workspace
  apps/desktop` passed with the pre-existing Vite chunk-size warning.
- Residual A8 hand-off: packaged Tauri/backend saved-project solve + render
  smoke remains the next harness slice for true GUI-authored backend
  completion. Boundary review: invented public example only; no cloud,
  daemon, network, telemetry, protected content, repository-default
  private-data write, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-128 saved-project backend smoke (`TP-APP-R2-SAVEDPROJECT-SMOKE-001`, 2026-06-12)

- Tranche `TP-APP-R2-SAVEDPROJECT-SMOKE-001` (completion-plan A8): added
  Tauri regression `r2_from_blank_saved_project_opens_solves_and_renders_report`.
- Automated coverage: applies the A12 from-blank rehearsal steps, saves the
  authored model into the desktop local SQLite project store, reopens it by
  project id, confirms mechanics/result payloads were not silently persisted
  before solve, solves the loaded model through `run_preview_mechanics`, and
  renders through `render_calculation_report`.
- Automated evidence: focused Tauri regression passed 1/1; full
  `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with
  31/31 Rust tests.
- Residual A8 hand-off: broader SMOKE checklist parity remains if every
  manual smoke row needs automated assertion coverage. Boundary review:
  invented public example only; in-memory local SQLite test store; no cloud,
  daemon, network, telemetry, protected content, repository-default
  private-data write, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-129 desktop unit catalog command (`TP-UNITS-B2-CATALOGCMD-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-CATALOGCMD-001` (completion-plan B2): added desktop
  backend command `get_unit_catalog`, backed by `core/units`, as the
  application binding point for DEC-018 unit metadata.
- Command payload coverage: stable unit ids, symbols, dimensions, canonical
  flags, transform kinds, factor representation text, optional offset
  representation text, provenance, review status, and boundary flags.
- Automated evidence: focused Tauri command test passed 1/1; full
  `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with
  32/32 Rust tests; `cargo test --manifest-path core/units/Cargo.toml` passed
  with 13/13 Rust tests; `python3 tests/test_units_schema.py` passed.
- No browser smoke was run because this tranche adds a backend command
  contract, not visible unit picker/display UI. Boundary review: no protected
  content, private project data, unit defaulting, solver/report normalization,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-130 frontend unit catalog service (`TP-UNITS-B2-FRONTENDSVC-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-FRONTENDSVC-001` (completion-plan B2): added typed
  frontend service `unitCatalogService.ts` for the desktop `get_unit_catalog`
  command.
- Desktop route: invokes `get_unit_catalog` and returns DEC-018 catalog
  metadata for future unit picker/display controls. Browser preview route:
  returns explicit `UNIT-CATALOG-DESKTOP-ONLY` unavailability and does not
  synthesize a fallback unit catalog.
- Automated evidence: focused `unitCatalogService` Vitest passed 2/2; full
  `npm test --workspace apps/desktop` passed with 215/215 tests; production
  build passed with the pre-existing Vite chunk-size warning.
- Boundary review: no visible unit picker/display retrofit, browser fallback
  catalog, unit defaulting, solver/report normalization, protected content,
  private project data, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-131 Property Inspector unit basis labels (`TP-UNITS-B2-INSPECTORLABELS-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-INSPECTORLABELS-001` (completion-plan B2): connected
  the Property Inspector's material and pipe-section creation labels to
  catalog-aware unit-basis display helpers.
- Visible browser-preview behavior: the new `Unit basis` panel reports
  `browser preview uses model metadata; no fallback catalog`, and the field
  labels show `m, model metadata`, `Pa, model metadata`, and `1/degC, model
  metadata`.
- Desktop/Tauri path remains backed by the `get_unit_catalog` command from
  TP-MAC-129; browser mode still does not synthesize a fallback catalog.
- Automated evidence: focused unit-catalog/App Vitest passed 48/48; full
  desktop Vitest passed 216/216; desktop production build passed with the
  pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2 after
  wasm engine build and asserted the visible unit-basis status.
- Boundary review: no unit picker, unit conversion, solver/report
  normalization, import/export unit handling, rule-pack unit I/O, protected
  content, private project data, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim.

## TP-MAC-132 Report packet unit-system disclosure (`TP-UNITS-B2-REPORTUNITS-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-REPORTUNITS-001` (completion-plan B2): added visible
  unit-system disclosure to the Report Packet panel and JSON export.
- Visible report behavior: `data-testid="report-unit-system"` names
  `unit-system:dec-018-si-dual-display`, lists model units such as
  `length=m`, lists distinct result-row units, and records
  `conversion=false`.
- Export behavior: the report JSON carries `unit_system_disclosure` with
  model units, result units, entered-unit preservation, and
  `conversion_performed=false`.
- Rendered-report adapter behavior: the strict `model_input_summary` keeps
  the existing `unit_system_ref` field and now points it at the DEC-018 unit
  system id; no report-generator schema expansion was made.
- Automated evidence: focused report/App Vitest passed 53/53; full desktop
  Vitest passed 216/216; desktop production build passed with the
  pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2 after
  wasm engine build and asserted the report unit-system disclosure.
- Boundary review: no unit conversion, solver-boundary normalization, unit
  picker, import/export unit conversion, rule-pack unit I/O, browser fallback
  catalog, protected content, private project data, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-133 solver-boundary unit normalization (`TP-UNITS-B2-SOLVERNORM-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-SOLVERNORM-001` (completion-plan B2): connected
  `core/product_physics` to `core/units` so the preview mechanics boundary
  accepts DEC-018-compatible unit symbols and normalizes to SI-canonical
  solver quantities before assembly.
- Automated product-physics evidence: a mixed-unit preview model with `MPa`
  material moduli, `mm` pipe dimensions, and `kPa` pressure loads solves to
  the same rounded result surface as the SI fixture; incompatible material and
  load units still block with `UNIT_INPUT_INVALID`.
- Automated app evidence: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  passed 32/32; focused desktop Vitest passed 56/56; Playwright R2 smoke
  passed 2/2 after wasm engine build; full desktop Vitest passed 216/216;
  desktop production build passed with the pre-existing Vite chunk-size
  warning.
- Boundary review: no visible unit picker, report-time conversion,
  import/export conversion, rule-pack unit I/O, browser fallback catalog,
  protected content, private project data, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-134 Property Inspector unit selectors (`TP-UNITS-B2-UNITPICKERS-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-UNITPICKERS-001` (completion-plan B2): added visible
  unit selectors to the Property Inspector material and pipe-section creation
  forms.
- Visible authoring behavior: section creation now has a `Length unit`
  selector; material creation now has `Modulus unit` and `Thermal expansion
  unit` selectors. Browser preview keeps one-option selectors from model
  metadata and does not synthesize a fallback catalog; desktop/Tauri mode uses
  accepted DEC-018 catalog entries.
- Operation-seam behavior: `core/model_operations/operation_applier` now
  accepts DEC-018-compatible entered units for create-section length
  quantities and create-material stress/thermal-expansion quantities, while
  preserving entered units in the applied session model. Incompatible
  dimensions still block.
- Automated evidence: operation-applier cargo suites passed 51 unit tests plus
  canonical-hash and contract-corpus tests; Tauri Rust tests passed 32/32;
  focused desktop Vitest passed 165/165; full desktop Vitest passed 216/216;
  desktop production build passed with the pre-existing Vite chunk-size
  warning; Playwright R2 smoke passed 2/2 after wasm engine build.
- Boundary review: no project-wide unit-system picker, existing field-edit
  unit picker, node/pipe/load unit picker, import/export unit conversion,
  rule-pack unit I/O, browser fallback catalog, protected content, private
  project data, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-135 rendered report body unit disclosure (`TP-UNITS-B2-REPORTBODY-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-REPORTBODY-001` (completion-plan B2): expanded the
  hash-bound rendered report body beyond the Report Packet disclosure so the
  `Model Input Summary` section displays unit evidence.
- Renderer input behavior: `model_input_summary.unit_display_summary` is a
  backward-compatible optional schema field containing
  `entered_units_preserved`, sorted model units, distinct result-row units,
  display policy, and `conversion_performed=false`.
- Rendered HTML behavior: the Rust report renderer prints `Unit storage
  convention`, `Model units`, `Result units`, `Quantity display policy`, and
  `Report-time conversion`; shared fixture and Tauri command coverage assert
  those rows.
- A12 regression behavior: the from-blank author-solve-render helper replaces
  the shared fixture summary with the authored model units and solved result
  units before rendering.
- Automated evidence: report-generator schema contract test passed;
  report-generator cargo tests passed 10/10; report-renderer cargo tests
  passed 8/8; Tauri Rust tests passed 32/32; focused report/App Vitest passed
  53/53; full desktop Vitest passed 216/216; desktop production build passed
  with the pre-existing Vite chunk-size warning; Playwright R2 smoke passed
  2/2 after wasm engine build.
- Boundary review: no report-time conversion, project-wide unit-system
  picker, import/export unit conversion, rule-pack unit I/O, browser fallback
  catalog, protected content, private project data, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-136 primitive-load unit selectors (`TP-UNITS-B2-LOADPICKERS-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-LOADPICKERS-001` (completion-plan B2): added visible
  unit selection to the Load Cases manager primitive-load creation form.
- Visible authoring behavior: `Magnitude unit` follows the selected load
  category/direction. Browser preview keeps one model-metadata option;
  desktop/Tauri mode uses accepted DEC-018 catalog options for the selected
  primitive-load dimension.
- Operation-seam behavior: `core/model_operations/operation_applier` accepts
  DEC-018-compatible entered primitive-load units and preserves them in the
  applied session model. Regressions cover `lbf` concentrated force, `kPa`
  pressure, and incompatible `mm` rejection for force.
- Automated evidence: operation-applier cargo suites passed 52 unit tests
  plus canonical-hash and contract-corpus tests; Tauri Rust tests passed
  32/32; focused desktop Vitest passed 165/165; full desktop Vitest passed
  216/216; desktop production build passed with the pre-existing Vite
  chunk-size warning; Playwright R2 smoke passed 2/2 after wasm engine build.
- Boundary review: no unit conversion at edit time, existing primitive-load
  magnitude-edit unit handling, project-wide unit-system picker, import/export
  unit conversion, rule-pack unit I/O, browser fallback catalog, protected
  content, private project data, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim.

## TP-MAC-137 primitive-load magnitude unit edits (`TP-UNITS-B2-PRIMEDITUNITS-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-PRIMEDITUNITS-001` (completion-plan B2): added
  unit-aware editing for existing primitive-load magnitudes in the Load Cases
  manager.
- Visible authoring behavior: the selected primitive-load editor now exposes
  `Magnitude unit`, labels the value input with the active unit basis, and
  queues an atomic `{ value, unit }` payload for
  `primitive_loads.N.magnitude.value`.
- Operation-seam behavior: legacy numeric-string magnitude edits remain valid;
  the new payload can also update the sibling `.unit` field in the same
  applied session-model change. Regressions cover `lbf/ft` force-per-length
  editing and incompatible `mm` rejection.
- Automated evidence: operation-applier cargo suites passed 53 unit tests
  plus canonical-hash and contract-corpus tests; Tauri Rust tests passed
  32/32; focused desktop Vitest passed 165/165; full desktop Vitest passed
  216/216; desktop production build passed with the pre-existing Vite
  chunk-size warning; Playwright R2 smoke passed 2/2 after wasm engine build.
- Boundary review: no report-time conversion, project-wide unit-system picker,
  import/export unit conversion, rule-pack unit I/O, browser fallback catalog,
  protected content, private project data, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-138 Property Inspector quantity unit edits (`TP-UNITS-B2-INSPECTOREDITUNITS-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-INSPECTOREDITUNITS-001` (completion-plan B2): added
  unit-aware editing for existing material and pipe-section quantities in the
  Property Inspector.
- Visible authoring behavior: editable material modulus, material thermal
  expansion, pipe outside diameter, and pipe wall thickness fields now expose a
  `Unit` selector and label the proposed value with the active unit basis.
  Browser preview remains model-metadata-only; desktop/Tauri mode can show
  accepted DEC-018 catalog options.
- Operation-seam behavior: sibling-unit quantity edits queue atomic
  `{ value, unit }` payloads. The Rust/wasm seam writes both `.value` and the
  sibling `.unit` field only after validating the entered unit against the
  declared dimension. Native regressions cover material `MPa`, pipe `mm`, and
  incompatible material-unit rejection.
- Automated evidence: operation-applier cargo suites passed 54 unit tests
  plus canonical-hash and contract-corpus tests; Tauri Rust tests passed
  32/32; focused desktop Vitest passed 165/165; full desktop Vitest passed
  216/216; desktop production build passed with the pre-existing Vite
  chunk-size warning; Playwright R2 smoke passed 2/2 after wasm engine build.
- Boundary review: no project-wide unit-system picker, project unit mutation,
  report-time conversion, import/export unit conversion, rule-pack unit I/O,
  browser fallback catalog, protected content, private project data,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim.
