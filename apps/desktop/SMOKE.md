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
