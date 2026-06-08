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
