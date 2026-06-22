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
   `diagnostic-COMBINATION_STRESS_SUMMARY_SKIPPED` and
   `diagnostic-RULE_CHECK_INPUTS_MISSING`.
26. Filter diagnostics by `result:stress:pipe-P-130`, click
   `data-testid="diagnostic-COMBINATION_STRESS_SUMMARY_SKIPPED"`, and confirm
   `data-testid="diagnostic-detail-panel"` shows
   `diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED`,
   `result:stress:pipe-P-130`, linked result value context, and the
   review-only professional boundary. Confirm
   `data-testid="diagnostic-unit-context"` shows linked result unit context
   with `units=MPa`, `source=result_envelope`, and `conversion=false`.
27. Confirm the model context selects `pipe:P-130` and the property inspector
   (`aria-label="Property inspector"`) shows `Tie-in rise`.
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

## TP-MAC-139 export unit-system disclosure (`TP-UNITS-B2-EXPORTDISCLOSURE-001`, 2026-06-12)

- Tranche `TP-UNITS-B2-EXPORTDISCLOSURE-001` (completion-plan B2): added
  DEC-018 source-unit disclosure to the desktop PCF, CAEPIPE MBF, and
  stress-neutral export packages.
- Visible export behavior: PCF, MBF, and stress-neutral panels now show a
  `Units` row summarizing source model units, target export units, result
  units where present, and whether export-time conversion was performed.
- Package behavior: downloaded JSON packages now include
  `unit_system_disclosure` with `unit-system:dec-018-si-dual-display`,
  `entered_units_preserved`, source model units, target export units, result
  units, conversion policy/scope, and protected/private-content false flags.
  Manifest package-member lists now include `unit_system_disclosure.json`.
- Schema/core behavior: strict PCF, CAEPIPE MBF, and stress-neutral package
  schemas and Python builders now require and checksum the disclosure member;
  invented fixtures were regenerated through the deterministic builders.
- Automated evidence: focused export-package tests passed 32/32; repository
  Python tests passed 356/356; full desktop Vitest passed 216/216; desktop
  production build passed with the pre-existing Vite chunk-size warning;
  Playwright R2 smoke passed 2/2 after wasm engine build.
- Boundary review: no target compatibility, import round-trip, target
  solver-deck validation, rule-pack unit I/O, protected content, private
  project data, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-140 R2 exit-chain verification + packaged-binary boot smoke (`TP-INTEGRATED-VERIFY-002`, 2026-06-12)

- Tranche `TP-INTEGRATED-VERIFY-002` (completion-plan Phase A exit-evidence
  row): independent verification session re-ran the R2-chain surfaces and
  produced the derivative verification snapshot
  `plans/VERIFICATION_2026-06-12_r2_exit_chain.md` for the D-14
  stage-advancement ruling.
- Re-run evidence: Tauri backend suite 32/32 including the A12 from-blank
  rehearsal and saved-project regressions; product_physics 25/25; core/units
  13/13; Playwright e2e 2/2 (real Chrome, wasm engine rebuilt) including the
  from-blank GUI journey.
- Packaged-binary boot smoke: `tauri build --debug --no-bundle` built the
  desktop binary in 1m05s; the binary launched, registered a Foreground
  window with WebKit renderer/networking processes, produced zero bytes of
  error output, and terminated cleanly.
- H4 evidence-posture exception note: the GUI journey inside the packaged
  webview was NOT automated because `tauri-driver` does not support macOS and
  interactive desktop driving was unavailable in this session (permission
  dialog timed out); the gap stays recorded in the A5/A8 rows as verification
  finding F-4. Every R2 criterion element remains proven on at least one
  automated surface.
- Boundary review: no lifecycle state change, stage advancement, release
  readiness, professional approval, certification, sealing, authentication,
  or code-compliance claim; stage advancement awaits the human D-14 ruling.

## TP-MAC-141 packaged-runtime A12 GUI journey — PREPARED manual checklist, human execution pending (`TP-APP-R2-PACKAGEDSMOKE-KIT-001`, 2026-06-12)

**Status: PREPARED, not performed.** D-14 was ruled Option O-B (`DEC-029`,
2026-06-12): the target stage holds at R2 until packaged-runtime GUI journey
evidence exists, and the recorded closure path is a **human-performed** manual
smoke of the packaged binary against the A12 script, recorded here as a human
entry. This section is the prepared kit for that pass. Its existence does not
close verification finding F-4; only the recorded human pass below does, and
stage advancement remains a separate human ruling.

### Binary provenance

- Built from commit `cc3b16cbd` (sweep-validated code state `78f618e09`;
  the sweep commit added evidence only) with `npm run tauri -- build`
  (release compile, 8m54s) then `npm run tauri -- build --bundles app` in
  `apps/desktop/` — no bundle targets are configured in `tauri.conf.json`,
  so the explicit `--bundles app` flag is required to produce the `.app`
  (12 MB).
- Bundle path:
  `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
- Boot-checked 2026-06-12 (agent, method of TP-MAC-140): the bundled
  executable launched, stayed alive with WebKit renderer processes and zero
  bytes of error output, and terminated cleanly on signal. Boot check only —
  not the GUI journey; the journey is the human checklist below.
- To rebuild fresh at any later HEAD:
  `cd apps/desktop && npm run tauri -- build --bundles app`
  (the checklist is unchanged; record the rebuilt commit in the entry below).

### Checklist (all values are the invented A12 rehearsal payloads from `fixtures/product_preview/r2_from_blank_rehearsal.json`)

Navigation (TP-MAC-146): the model tree, 3D viewport, and Property Inspector
stay on screen; every other surface is behind the workspace section tabs
above the lower dock (`Operation Apply`, `Load Cases`, `Solve`, `Results`,
`Report`, `Project`, `Exports`, `Audit & Boundaries`).

Authoring steps queue an intent, then apply it from the Operation Apply panel
row (Apply button) — open the `Operation Apply` section to reach it. After
each apply, expect the apply summary to show
`0 queued; N applied` with the step's N, and the applied row to show
`route=tauri_backend_apply` and `professional_approval=false`.

1. **Launch** the bundle (double-click or `open "…/OpenPipeStress Technical Preview.app"`).
   Expect the `OpenPipeStress Technical Preview` header, operation engine
   status `engine_route=tauri_backend_apply; engine_state=ready`, and local
   project status containing `network=false` and `telemetry=false`.
2. **New blank.** Click `New blank`. Expect "Created blank local model
   document without fixture entities or external file copies."; then open
   the `Load Cases` section and expect Load Cases
   summary `0 load cases; 0 primitive loads; 0 combinations`.
3. **Node 1** (viewport explicit-node form): id `node:R2-100`, label
   `R2 anchored node`, x/y/z `0 / 0 / 0`. Queue + apply (1 applied).
4. **Node 2**: id `node:R2-110`, label `R2 loaded node`, x/y/z `3 / 0 / 0`.
   Queue + apply (2 applied).
5. **Material** (Property Inspector create-material form): id
   `material:r2-carbon-steel`, label `R2 invented carbon-steel-like material`,
   elastic modulus `200000000000` Pa, shear modulus `77000000000` Pa,
   provenance `invented_a12_rehearsal_user_input`. Queue + apply (3 applied).
6. **Section** (create-section form): id `section:r2-pipe`, name
   `R2 invented pipe section`, outside diameter `0.114` m, wall thickness
   `0.006` m, same provenance. Queue + apply (4 applied).
7. **Pipe** (viewport explicit-pipe form): id `pipe:R2-100`, label
   `R2 invented straight pipe`, from `node:R2-100`, to `node:R2-110`,
   material `material:r2-carbon-steel`, OD `0.114`, wall `0.006`,
   y-reference x/y/z `0 / 0 / 1`, same provenance. Queue + apply (5 applied).
8. **Support** (create-support form): id `support:R2-anchor`, label
   `R2 user anchor`, node `node:R2-100`, all six restraints checked
   (UX, UY, UZ, RX, RY, RZ), same provenance. Queue + apply (6 applied).
9. **Load case** (open the `Load Cases` section; Load Cases manager create form): id `load:R2-L-100`, label
   `R2 invented operating load`, kind `primitive_user_load`, status
   `preview_only`, same provenance. Queue + apply (7 applied).
10. **Primitive load** (in the `Load Cases` section): load case `load:R2-L-100`, category
    `concentrated_force`, id `load:R2-L-100-FY`, node `node:R2-110`,
    direction `global_y`, magnitude `250` N, same provenance.
    Queue + apply (8 applied).
11. **Combination** (in the `Load Cases` section): id `combination:R2-C-100`, label
    `R2 invented operating combination`, load case `load:R2-L-100`, factor
    `1`, same provenance, short rationale text. Queue + apply (9 applied).
12. **Summary check.** In the `Load Cases` section, expect `1 load cases; 1 primitive loads;
    1 combinations`.
13. **Solve.** Open the `Solve` section, then click `Run mechanics preview`. Expect the solve job to reach
    `state=completed` with mechanics status `MECHANICS_SOLVED` and at least
    one result row; grouped results render (displacement/reaction/force/
    moment/stress) in the `Results` section.
14. **Report.** Open the `Report` section, then click `Render report`. Expect a rendered report preview and
    `Canonical HTML SHA-256: <hash>`; record the hash below.
15. **Save.** Click `Save local`. Expect a saved-project confirmation message
    (local store only; nothing leaves the machine).
16. **Reopen.** Click `List local`, then `Open …` for the saved project.
    Expect the authored model to reload with the step-12 summary intact.
17. **Re-solve.** Open the `Solve` section, then click `Run mechanics preview` again. Expect
    `state=completed` + `MECHANICS_SOLVED` again on the reopened model.
18. **Quit.** No network, daemon, telemetry, or repository writes occurred;
    the user project stays in local app storage and is never committed.

### Human execution record (fill in when performed — human entry per DEC-029)

**Attempt 1 — 2026-06-12, human project authority, binary at `cc3b16cbd`:
FAIL at step 2.** Clicking `New blank` produced: "Blank create failed:
Error: WASM-ENGINE-ASSET-ABSENT: generated glue module import failed:
TypeError: 'text/html' is not a valid JavaScript MIME type. — the wasm
operation engine is required in browser mode (DEC-020 / ADR-0001) and no
fallback engine exists." Root cause: the production build never shipped the
generated wasm engine assets (frontend hashing requires them in every mode
per H1/F-5a) and `tauri build` never built them. Repaired by
`TP-APP-R2-WASMPKG-001` (see TP-MAC-144): assets now emitted via the Vite
publicDir into `dist/`, `beforeBuildCommand` chains the wasm build, a
build-time guard fails loudly on absence, and a production-dist Playwright
lane replays this exact regression. The bundle is rebuilt with the fix;
the checklist is unchanged.

- Rebuilt binary provenance: commit `0b674b2e5` (contains
  `TP-APP-R2-WASMPKG-001` and the `DEC-033` document-version bump),
  `npm run tauri -- build --bundles app` (now self-sufficient — it builds
  the wasm engine itself). Same bundle path as above.
- Rebuilt again after the usability repair: commit `26aadc4fe` (code state
  `e05527c2c`, sweep-validated; adds the `TP-APP-R2-UXSHELL-001` workspace
  IA/scroll/dead-control repair), same command, boot-checked clean. Steps
  reference the new section tabs per the navigation wording inserted by
  that tranche.

**Attempt 2 — 2026-06-12, human project authority, binary at `0b674b2e5`
(recorded by the coordinating agent on the human's explicit stated
authority): NOT COMPLETED — abandoned for usability.** Step 1–2 passed on
the repaired binary: launch clean, `New blank` created the blank document
("Created blank local model document without fixture entities or external
file copies."), and model-hash persistence ran (observed status:
`persisted_model_hashes=1;
persisted_model_hash_ref=sha256:cff048ccb87de950e14e6ca83a53a871dbdb76b702515311eed0c36cff6c1bff`).
The authoring journey (steps 3–18) was not completed. Human verdict,
verbatim: "no it's impossible to use, but the button did reset some stuff.
There's a long way to go development wise and the interface is so confusing
and poorly laid out that I can't even get through that checklist. I'm not
telling you that we have to halt progress, either. Following a line of
development according to plan has merit, so long as there aren't blockers
or significant risks." No report was rendered; no report SHA-256 exists for
this attempt. **F-4 remains open.** Finding recorded to the completion
plan's A3 row: the packaged-runtime functional chain is repaired, but
authoring-journey usability now blocks the human pass — the R2 criterion's
"a user can create, solve, and report" is not yet humanly demonstrable
end-to-end, independent of mechanical correctness.

**Attempt 3 — 2026-06-12, human project authority, binary at `26aadc4fe`
(post-UXSHELL rebuild; recorded by the coordinating agent on the human's
explicit stated authority): PARTIAL — steps 1–5 PASS, journey not
completed.** Steps 1 through 5 individually confirmed (launch, New blank,
node 1, node 2, material). Steps 6–18 not executed as prescribed; the human
observed "a lot of seemingly good functionality in terms of buttons doing
expected things" but "would need a lot of help to figure out where to
navigate and enter the information." Human ruling, verbatim: "the app
appears to have consistent functionality with the test that I could figure
out. The rest of the test still fails because the user interface is too
cumbersome and opaque. But there's still much more to do anyways. Unless
there's some significant risk, we should proceed with the next development
phase." No report rendered; no report SHA-256 exists for this attempt.
**F-4 remains open** (no completed journey). Stage consequence: per
`DEC-035` (same day), the target stage advanced to R3 with F-4 and the
authoring-journey usability finding carried as named blocking residuals at
the R3 exit review.

```
performed: NO (attempt 3 partial — steps 1–5 pass; F-4 open, carried as a
  blocking residual at the R3 exit review per DEC-035; re-run when
  authoring-UX iteration makes the journey completable)
date:
performed_by:
binary commit:
outcome (PASS / FAIL per step, with any deviations):
report sha256:
notes:
```

- Boundary review: invented rehearsal values only; user-created local project
  data stays in local storage and is never committed; no lifecycle state
  change, stage advancement, release readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-142 true directional deformed-shape overlay (`TP-APP-R2-DEFORMEDDIR-001`, 2026-06-12)

- Tranche `TP-APP-R2-DEFORMEDDIR-001` (completion-plan A6 residual): the
  deformed-shape overlay is now directionally true end-to-end. The solver
  bridge emits signed per-node displacement component rows for all six DOF,
  and the viewport applies the real global-cartesian displacement direction
  instead of the previous Y-only normalized offset.
- Solver-bridge behavior (`core/product_physics`): after the existing
  `displacement_magnitude` rows, each load case appends
  `result:disp:<node>:{ux,uy,uz,rx,ry,rz}` rows — translations in mm
  (round6 of m×1000), rotations in rad (round6), signed values from the
  6-DOF kernel displacement vector. Rows carry the existing metadata
  vocabulary: component `nodal_displacement_*`/`nodal_rotation_*`,
  coordinate_system `global`, location `node`, basis
  `solved_from_global_linear_system`, explicit global-cartesian
  sign-convention text. Existing row ids, kinds, values, and relative order
  are unchanged (verified: regenerated fixture has 0 changed, 0 removed,
  90 appended rows). `rad` was added to the load-combination dimension map
  so the new rotation rows join the explicit user combination algebra like
  every other emitted scalar row instead of raising spurious
  unsupported-dimension warnings.
- Viewport behavior (`PipeViewport.buildDeformationOverlay`): each node's
  displaced marker now moves along the unit vector of its (ux, uy, uz) rows
  — taken from the same load-case/combination basis as the node's governing
  magnitude row — scaled by the existing normalized display offset. The
  boundary line replaces `vector_direction=TBD` with
  `vector_direction=global_cartesian_displacement_components`; when
  component rows are absent or incomplete the overlay falls back to the
  previous magnitude-only vertical display offset and the boundary line says
  `vector_direction=vertical_display_axis_fallback_component_rows_unavailable`.
  Rotational DOF rows are emitted but intentionally not visualized
  (no curvature rendering claim). The normalized display scale remains
  disclosed as not physical length.
- Canned browser fixture: `invented_mechanics_result.json` regenerated
  through the documented workflow
  (`npm run generate:product-preview-mechanics`); invented values only,
  produced by the same solver emission (647 → 737 rows; displacement family
  15 → 105).
- Automated evidence: product_physics cargo tests passed 28/28 (new:
  six-DOF row pinning with metadata, signed cantilever direction check for
  +Y/−Y tip loads, byte-stable determinism across runs, fixture-consistency
  for component rows); Tauri Rust tests passed 32/32 (no count pins needed
  updating — assertions are minimum-row based); desktop Vitest passed
  220/220 (new: directional offset math against fixture geometry, basis
  mismatch fallback, component-rows-absent fallback, DOM disclosure of the
  fallback status line); desktop production build passed with the
  pre-existing Vite chunk-size warning; repo Python suite passed 358/358
  (fixture schema spot-checks unaffected).
- H4 evidence posture: the browser-mode preview-fixture solve path carries
  the new component rows (regenerated canned fixture), and the first
  Playwright e2e test now asserts the deformation overlay boundary discloses
  `vector_direction=global_cartesian_displacement_components` after solve in
  a real Chromium browser; e2e passed 2/2 after the wasm engine build.
- Residuals: governing-ratio views still pending (no ratio rows exist);
  rotational deformation is not visualized; the results schema's
  `ResultMetadata.basis` enum does not yet list
  `solved_from_global_linear_system` (preview rows are not schema-validated
  today; flagged for the schema owner rather than edited here).
- Boundary review: invented values only; no network, cloud, or telemetry
  surface; no protected standards content or private project data; no
  lifecycle state change, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim. The
  overlay remains a review-only display aid with its non-physical display
  scale explicitly disclosed.

## TP-MAC-143 subtraction/range combination expression authoring (`TP-APP-R2-COMBEXPR-001`, 2026-06-12)

- Tranche `TP-APP-R2-COMBEXPR-001` (completion-plan A4 residual): users can
  now author result-state-subtraction and range-envelope load combinations
  in the Load Cases manager, validated through the structured operation
  seam and evaluated deterministically at solve. The combination basis is a
  closed set mirroring `core/loads/load_case_algebra` vocabulary:
  `mechanics` (linear terms, unchanged), `result_state_subtraction`
  (`minuend_id` − `subtrahend_id`), `range_envelope` (`mode` over
  `operand_ids`, modes `min`/`max`/`min_abs`/`max_abs`). New record fields
  are strictly additive and optional; existing documents and fixtures stay
  byte-valid (the regenerated mechanics preview fixture is byte-identical),
  and the model-document schema version stays 0.1.0.
- Seam behavior (`core/model_operations/operation_applier`, native + wasm —
  sole engine per DEC-020): `create_combination` accepts the closed basis
  set with per-basis payload-shape validation and named blocking
  diagnostics — `OP-COMBINATION-BASIS-UNSUPPORTED` (token outside the
  closed set), `OP-CREATE-COMBINATION-PAYLOAD-INVALID` (wrong per-basis
  payload shape, including cross-basis stray fields and missing/empty
  operand lists), `OP-COMBINATION-RANGE-MODE-UNKNOWN`,
  `OP-COMBINATION-OPERAND-DUPLICATE` (duplicates and self-subtraction),
  `OP-COMBINATION-OPERAND-LOAD-NOT-FOUND`. `create_combination_term` on a
  non-mechanics combination blocks with
  `OP-COMBINATION-TERM-BASIS-UNSUPPORTED`. Load-case deletion now also
  counts `minuend_id`/`subtrahend_id`/`operand_ids` references in
  `OP-LOAD-CASE-DELETE-REFERENCED`.
- Behavior change (basis-edit validation): `Combination.basis` updates were
  free text since `TP-APP-R2-COMBBASIS-001`; they now validate the same
  closed set (`OP-COMBINATION-BASIS-UNSUPPORTED`) and block cross-shape
  changes whose stored payload does not already carry the target basis
  fields (`OP-COMBINATION-BASIS-SHAPE-MISMATCH`) — changing the expression
  shape is delete + recreate, never a half-evaluable record. Legacy
  free-text basis values remain repairable to a shape-matching closed-set
  token. The first Playwright test previously filled
  `mechanics_user_review` and asserted the proposed preview; it now selects
  `result_state_subtraction` from the closed-set selector, and corpus cases
  64/65 pin both blocked edit outcomes cross-engine.
- Solve behavior (`core/product_physics`): per-basis structural validation
  blocks pre-solve with named diagnostics
  (`LOAD_COMBINATION_BASIS_UNSUPPORTED` message now names the closed set;
  new `LOAD_COMBINATION_SHAPE_INVALID`,
  `LOAD_COMBINATION_RANGE_MODE_UNKNOWN`,
  `LOAD_COMBINATION_OPERANDS_EMPTY`; duplicates/self-subtraction reuse
  `LOAD_COMBINATION_DUPLICATE_TERM`; unknown refs reuse
  `LOAD_COMBINATION_LOAD_CASE_UNKNOWN`). Evaluation reuses
  `core/loads/load_case_algebra` (`evaluate_result_state_subtraction`,
  `evaluate_range_envelope`): subtraction = minuend − subtrahend; range =
  mode-selected value across operands with the crate's sorted-operand
  tie-break. Combination rows carry result-metadata basis
  `explicit_user_result_state_subtraction` /
  `explicit_user_range_envelope` (added additively to
  `schemas/results.schema.yaml`), per-basis sign-convention text, and
  deterministic `source_result_refs`. Missing/unsolvable source rows keep
  the existing named-diagnostic-plus-withheld-row pattern
  (`LOAD_COMBINATION_SOURCE_RESULT_MISSING` et al.) — structural problems
  block the solve; evaluation-time gaps name the row and emit no value;
  silent zeros/skips do not exist. The mechanics path is unchanged
  (regenerated fixture byte-identical).
- UI (`LoadCaseManagerPanel`): the create-combination form gains a
  closed-set basis selector with conditional fields per basis (mechanics:
  existing load/factor flow unchanged; subtraction: minuend + subtrahend
  selectors; range: operand multi-select + mode selector), per-basis
  intent payloads, preview text, and readiness validation (distinct
  subtraction operands; unique existing range operands). The selected-
  combination basis editor is now a closed-set selector that still renders
  a legacy stored value honestly. Combination rows display the per-basis
  expression. Intent/preview/queue/apply pattern, ids, and testids are
  unchanged.
- Contract corpus: cases 58–65 added (accepted subtraction create; accepted
  range create pinning `max_abs` as one representative mode — not all four;
  blocked wrong-shape per new basis; blocked unknown mode; blocked missing
  operand ref; blocked free-text basis edit; blocked cross-shape basis
  edit), blessed via `CORPUS_BLESS=1 cargo test --test contract_corpus` and
  re-run through both engines. The README inventory notes cases 58+ are
  pending human review and do not ride the DEC-030 acceptance.
  `schemas/model.schema.yaml` Combination adds the two basis tokens,
  optional `minuend_ref`/`subtrahend_ref`/`operand_refs`/`mode` fields, and
  per-basis conditional requires using the schema's existing if/then
  convention; every previously valid document remains valid.
- Automated evidence: operation_applier cargo tests passed 61/61
  (58 lib + 1 canonical-hash parity + 2 corpus runners over 65 cases);
  product_physics cargo tests passed 31/31 (new: signed subtraction
  determinism, all four range modes, named blocking diagnostics);
  load_case_algebra cargo tests passed 18/18 (new: mode-token round-trip);
  desktop Vitest passed 239/239 (new: subtraction and range create
  draft→preview→queue→apply through the wasm engine, draft validation
  surfacing, honest blocked basis-edit surfacing; corpus runner covers all
  65 cases in both wasm lanes); Tauri Rust tests passed 32/32 (untouched);
  desktop production build passed with the pre-existing chunk-size
  warning; repo Python suite passed 358/358 (schema changes validated);
  `cargo fmt --check` clean on the three touched crates.
- H4 evidence posture: user-visible UI change ⇒ Playwright e2e extended as
  default evidence — the first test authors a `result_state_subtraction`
  combination through the visible create-form controls in real Chromium
  and applies it through the wasm engine (route=local_wasm_engine
  receipt), and exercises the closed-set basis editor proposal; e2e passed
  2/2 after the wasm engine build. New React surface stays within the
  existing panel, covered by Vitest at the slice's existing pattern.
- Residuals: solve evaluation of the new bases is exercised natively
  (product_physics tests); browser-mode solve of an edited model still
  reports `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, so the e2e
  evidence for new bases stops at authoring/apply (pre-existing limit, not
  a new gap). Cross-shape basis conversion is intentionally
  delete-and-recreate. Corpus cases 58–65 await human review.
  (Review recorded 2026-06-12: `DEC-032` accepted cases 58–65.)
- Boundary review: invented values only; basis names stay code-neutral
  mechanics vocabulary with no code-specific load-combination defaults; no
  protected standards content or private project data; no network, cloud,
  or telemetry surface; no lifecycle state change; no release-readiness,
  professional, certification, sealing, authentication, or code-compliance
  claim. Combination results remain review-only preview mechanics
  quantities requiring human engineering review.

## TP-MAC-144 packaged builds ship the wasm engine (`TP-APP-R2-WASMPKG-001`, 2026-06-12, regression repair)

- Defect: the human TP-MAC-141 packaged pass (attempt 1) failed at `New
  blank` with `WASM-ENGINE-ASSET-ABSENT` — frontend hashing requires the
  wasm engine in every mode (H1/F-5a, no fallback), but the generated
  assets were loaded via a vite-ignored dynamic import only the dev server
  resolves; `dist/` never shipped them and `tauri build` never built them,
  so the packaged asset protocol served `index.html` (`text/html` MIME) to
  the module import.
- Fix: `build-wasm-engine.mjs` emits into `apps/desktop/public/wasm-engine/`
  (dev-served at root AND copied verbatim into `dist/`; DEC-025 F-4 atomic
  rename preserved); loader browser lane imports a fully-qualified root URL
  (valid under Vite dev, `vite preview`, and `tauri://localhost/`);
  node/vitest lane probes the new disk candidates; `WASM-ENGINE-ASSET-ABSENT`
  text and remediation unchanged, no fallback engine added.
  `tauri.conf.json` `beforeBuildCommand` chains `npm run build:wasm`, so
  `tauri build` is self-sufficient. New `wasm-engine-dist-guard` vite plugin
  fails the production build loudly when assets are absent (demonstrated
  during validation, then restored).
- New evidence lane: `npm run test:e2e:dist --workspace apps/desktop` runs
  Playwright against the production `dist/` via `vite preview` and replays
  the exact regression (shell loads, engine ready, `New blank` succeeds).
  The DEC-025 sweep runs it as a second command inside the existing
  `desktop_playwright_e2e` surface (five surfaces unchanged).
- Validation: Vitest 241/241; dev e2e 2/2; dist e2e 1/1; pytest 358/358;
  production build green with `dist/wasm-engine/` shipped; rebuilt bundle
  boot-checked clean. Honest residual: the `tauri://` protocol itself is
  exercised only by the human TP-MAC-141 packaged pass.
- Evidence: DEL-00-08 run record `TASK_RUN_2026-06-12_1355.md`.
- Boundary review: no network, cloud, or telemetry surface; no lifecycle
  state change; no release-readiness, professional, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-145 model document version 0.2.0 per DEC-033 (`TP-APP-R2-DOCVER-020-001`, 2026-06-12)

- Implements the `DEC-033` ruling: additive shape changes bump the minor
  document version. `SUPPORTED_MODEL_SCHEMA_VERSION` is 0.2.0 on both
  evaluation surfaces (src-tauri authority and the projectService mirror,
  pending H2 unification), with published no-op transform
  `model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop` (0.2.0 only
  adds optional combination members). Blank documents author 0.2.0; opening
  a 0.1.0 document migrates in memory with the migration id in the DEC-019
  ledger; browser preview keeps stored snapshot bytes unchanged for
  migrated documents (no unevidenced rewrite). Bundled preview fixtures stay
  0.1.0-era and now exercise the real migration path; contract corpus
  byte-identical (the applier does not version-gate documents).
- Validation: src-tauri 33/33; applier 58+1+2/0; Vitest 241/241; pytest
  358/358; build green; fmt clean.
- Residuals routed to the plan's H2 row: panel-level `"0.1.0"` display
  literals are stale for old-version session documents; desktop-save
  hash-integrity edge for migrated bytes.
- Evidence: DEL-02-05 run record `TASK_RUN_2026-06-12_1407.md`.
- Boundary review: no lifecycle state change; no release-readiness,
  professional, certification, sealing, authentication, or code-compliance
  claim.

## TP-MAC-146 desktop shell usability repair: workspace IA, scroll/overflow, dead-control audit (`TP-APP-R2-UXSHELL-001`, 2026-06-12)

- Responds to the TP-MAC-141 attempt-2 abandonment ("no logical layout of
  the panels at all... things are cut off or don't scroll... many things are
  unresponsive"). Three repairs, no behavior or `data-testid` changes:
- **Workspace information architecture** (PRD section 14.1; DEL-07-02 and
  DEL-07-06 kits): the shell keeps a persistent spatial core — model tree |
  3D centerline viewport | property inspector — and moves all other panels
  behind an always-visible workspace navigation in A12 journey order:
  `Operation Apply` (apply queue with live queued-count badge, editor
  contract, diff preview, operation ledger, proposals), `Load Cases`,
  `Solve` (execution, diagnostics, missing-data, rule-check, knowledge),
  `Results` (results browser, comparison, design workspace), `Report`
  (rendered report, report packet, report lint), `Project` (storage audit,
  validation preflight), `Exports` (all export/interop/handoff panels),
  `Audit & Boundaries` (run audit, validation evidence, build readiness,
  telemetry/secret/security/accessibility reviews). Inactive sections stay
  mounted (form drafts and queue state survive navigation) and are hidden
  with `display:none`. PRD 14.1 surfaces without implementations
  (material/component library editors, rule-pack manager) remain absent —
  no placeholder dead buttons. The boundary strip stays always visible.
- **Scroll/overflow repair** (structural, in `styles.css` with a structure
  map documented at the top): each core column is an independently
  scrolling pane; the section dock body scrolls; the operation apply queue
  is height-bounded and scrolls internally; project toolbar and form rows
  wrap instead of clipping; the model-tree filter row reflowed to two rows
  (it overlapped at narrow widths); boundary-strip values carry full-text
  tooltips when ellipsized. Verified in a real browser at 1440x920 (the
  packaged window default) and 1280x800: the only remaining ellipsized
  content is the boundary strip (tooltip-backed); all panel regions scroll
  within their bounds; zero console errors.
- **Dead-control audit** (`src/App.deadControls.test.tsx`, permanent
  regression test): renders the full shell in three states (initial,
  queued, solved), enumerates every button by normalized control class, and
  fails on any enabled control whose click produces no observable DOM
  change or any disabled control with no accessible reason
  (title/aria-label/aria-describedby). Documented exemption: buttons with
  `aria-pressed="true"` (already-active toggles) are idempotent by design.
  Initial audit caught 4 controls; dispositions: `tree-row-project`
  (clicking the already-selected tree row produced no change — rows now
  expose `aria-pressed` selection state, so the active row is an honest
  toggle), `cancel-mechanics-preview`, `generate-review-proposal`, and
  `rendered-report-render` (all legitimately disabled but reason-less —
  each now carries an explicit disabled-reason title naming the enabling
  step). Audit green after dispositions; no control needed to be removed.
- TP-MAC-141 checklist updated with navigation wording only (a navigation
  note plus "open the `<section>` section" inserts on steps 2, 9-12, 13,
  14, 17); payloads, expectations, and human execution records untouched.
- e2e now drives the navigation through visible controls; the dev lane runs
  a viewport matrix (1440x920 packaged default + 1280x800 compact).
- Validation (this tranche): Vitest 10 files / 242 tests passed (241 prior
  + dead-control audit); `npm run build` green; dev e2e 4 passed (2 specs x
  2 viewports); dist e2e 1 passed.
- H4 posture: the changed navigation is exercised end-to-end in a real
  browser (both e2e journeys click the section tabs); jsdom suites cover
  panel behavior; the packaged `tauri://` runtime remains covered only by
  the human TP-MAC-141 pass, which this row exists to unblock.
- Boundary review: layout and navigation changes only — no solver, engine,
  schema, or persistence change; nav labels carry no claims language; the
  professional/boundary strip remains permanently visible; no network,
  daemon, or telemetry surface added; no lifecycle state change, release
  readiness, certification, sealing, authentication, or code-compliance
  claim; whether the layout is now humanly usable is decided by the human
  TP-MAC-141 re-run, not by this entry.

## TP-MAC-147 rule-pack backend seam: document validation, JCS checksum, local store (`TP-C2-RPLIFE-001`, 2026-06-12)

- Phase C2 backend seam (no UI change in this tranche). New crate
  `core/rules/rule_pack_document`: production codec between the rule-pack
  document AST encoding and the frozen DEC-022 grammar (round-trips all 69
  conformance-corpus expressions), grammar-version-bound JCS checksum over
  the document minus its `checksums` member (RFC 8785 via
  `core/serialization/canonical_json` + the DEL-06-04 lifecycle binding
  constructor), and document-level validation composing lifecycle,
  grammar, expression-decode, and checksum-match findings.
- Desktop commands (first consumers of the `core/rules` crates):
  `validate_rule_pack`, `compute_rule_pack_document_checksum`,
  `save_local_rule_pack`, `open_local_rule_pack`, `list_local_rule_packs`,
  `delete_local_rule_pack`. Store v10 migration adds `local_rule_packs`
  (project-scoped, local SQLite only). Drafts with blocking findings remain
  saveable; user packs are never committed to the repository or
  transmitted (OPS-K-PRIV-1, PRD §17.3).
- The DEL-06-05 example pack now carries a real stamped checksum
  (`9910cecaff4e…`), pinned by a Rust golden test and a Python JCS
  recomputation — a cross-engine parity witness across both hash lanes.
- Validation: rule_pack_document 10/10 (6 unit + corpus parity + 3
  example-pack incl. evaluator computing the invented ratio 0.5);
  src-tauri 37/37 (store v10 migration evidence re-pinned, 4 new rule-pack
  tests); pytest 359/359; fmt clean.
- No browser/UI surface changed; the C2 editor GUI tranche consumes these
  commands next (Playwright/Vitest evidence rides that tranche per the H4
  posture).
- Evidence: DEL-06-04 run record
  `WORKING_ITEMS_RUN_2026-06-12_TP-C2-RPLIFE-001.md`.
- Boundary review: validation statuses are software findings only; no
  lifecycle state change; no release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-148 rule-pack manager GUI: private-by-default authoring, validation, checksum (`TP-C2-EDITOR-001`, 2026-06-12)

- Phase C2 slice 1 of the PRD §14.5 rule-pack editor / §14.1 "Rule-pack
  manager". New "Rule Packs" workspace section (between Load Cases and
  Solve), `features/rule-packs/RulePackManagerPanel.tsx`, and the
  `rulePackService.ts` frontend route over the TP-MAC-147 backend commands.
  Covers pack management + document-level authoring: new private draft
  (privacy_class=private_user_data, redistribution_status=private_only per
  PRD §12.4), list/open/save/delete against the local-only store, validate,
  and compute-and-stamp checksum. Expressions are authored in the document's
  native declarative-AST JSON (DEC-022); no expression text syntax ships
  (D-02b AWAITING_RULING). The draft template carries only placeholders and
  invented wording — no protected code equations or standards values
  (PRD §14.5).
- Honest browser-mode seam: every backend action reports
  `RULE-PACK-BACKEND-DESKTOP-ONLY` (the unit-catalog/report precedent);
  drafts stay in session memory until a desktop save. Desktop backend
  rejections surface as `RULE-PACK-BACKEND-ERROR` rather than silent
  unhandled rejections. The stored-pack list resets on project change so it
  never contradicts the scope banner.
- Boundary note kept permanently visible: private packs never committed or
  transmitted; rule-check output is a software computation over user data,
  never a code-compliance, certification, sealing, approval, or professional
  acceptance claim.
- Validation: Vitest 251/251 (11 files; 8 new rule-pack panel/service tests;
  the permanent dead-control audit passes — disabled draft buttons carry
  accessible reasons, enabled buttons produce observable change); Playwright
  dev 6/6 (two-viewport matrix; new rule-pack journey drives the section
  through visible controls); dist 1/1; `tsc -b` + vite build clean.
- Adversarial pre-commit review (three lenses) surfaced four findings; three
  legitimate ones were fixed before commit (unhandled desktop rejections,
  stale list on project switch, two vacuous e2e assertions), one
  (mid-request textarea clobber) recorded as a residual for the composer
  slice.
- Residuals routed to the next C2 slices: structured AST expression composer
  (replacing raw-JSON expression editing); required-input/value-slot/
  load-combination form builders (PRD §14.5 sub-surfaces); in-request busy
  guard; per-finding inline surfacing on the document. Engine-side rule
  evaluation on solved user models is C4.
- Evidence: DEL-07-03 run record
  `WORKING_ITEMS_RUN_2026-06-12_TP-C2-EDITOR-001.md`.
- Boundary review: no lifecycle state change; no release-readiness,
  professional, certification, sealing, authentication, or code-compliance
  claim.

## TP-MAC-149 rule-pack structured AST expression composer (`TP-C2-COMPOSER-001`, 2026-06-13)

- Phase C2 slice 2: the PRD §14.5 "Expression editor". New
  `features/rule-packs/ExpressionComposer.tsx` — a recursive form/tree editor
  that builds the frozen grammar v1.0.0 typed expression AST (DEC-022) for a
  rule pack's selected formula, replacing raw declarative-AST JSON editing.
  Structured controls cover the full non-table node set (literal,
  variable_ref, unary, binary, compare, logical, select, aggregate, with
  add/remove aggregate operands); a variable picker / read-only browser is
  sourced from the pack's `required_inputs`/`value_slots` (PRD §14.5
  "Variable browser"). Edits re-serialize back into the canonical document
  JSON the validate/checksum/save flow already reads.
- D-02b gate held: purely structured controls — **no writable expression
  text syntax and no text rendering of the AST** (read-only rendering is
  itself an open D-02b §3 Q5 question). Lossless preservation: table-backed
  (interpolate/lookup) and unrecognized nodes render read-only with no
  node-type selector and round-trip unchanged — never silently dropped.
- In-request busy guard added (closes the slice-1 residual): the textarea,
  composer, and all actions disable with a stated reason while a backend
  request is awaiting, so an async response (`compute-checksum`/`open`)
  cannot clobber a mid-request edit.
- Manual check: open "Rule Packs", click "New draft rule pack"; the
  Expression composer shows the formula's `variable_ref` root and the
  declared variables; switch the root node type to `compare` and confirm the
  document JSON below updates to the compare AST; switch to `aggregate` and
  add/remove operands (last-operand removal is blocked with a stated reason).
- Validation: targeted Vitest `src/features/rule-packs` 21/21; Playwright
  `-g "rule-pack manager"` 2/2 (chromium-desktop + chromium-compact, composer
  driven from blank); `tsc -b` clean. Full-suite `App.test.tsx` shows
  pre-existing, host-environmental timeout flakiness (reproduces on pristine
  HEAD, varying test set) unrelated to this slice — see the run record.
- Adversarial pre-commit review (four lenses: react-correctness,
  governance/D-02b, AST-encoding, evidence-honesty); three test-honesty
  findings fixed before commit (vacuous table-preservation assertion →
  full `toEqual`; missing unknown-node preservation test added; shallow e2e
  assertion → full structure).
- Residuals routed to the next C2 slices: table-node structured sub-editor
  (interpolate/lookup rows/dimensions/units); required-input/value-slot/
  load-combination form builders. Engine-side rule evaluation on solved user
  models is C4.
- Evidence: DEL-07-03 run record
  `WORKING_ITEMS_RUN_2026-06-13_TP-C2-COMPOSER-001.md`.
- Boundary review: no lifecycle state change; no release-readiness,
  professional, certification, sealing, authentication, or code-compliance
  claim.

## TP-MAC-150 rule-pack table-node structured sub-editor (`TP-C2-TABLENODE-001`, 2026-06-13)

- Phase C2 slice 3: completes structural authorability of the frozen grammar
  v1.0.0 in the expression composer. `interpolate` and `lookup` nodes —
  preserved read-only by slice 2 — are now first-class authorable node types
  in `features/rule-packs/ExpressionComposer.tsx`. A structured table
  sub-editor authors `table_id`, argument/result `dimension` (shared
  `DimensionSelect`) and `unit_ref`, the `{argument, result}` rows (Add row
  appends last-argument + 1 to stay strictly increasing; Remove row is blocked
  at the single-row schema floor), the `lookup` `mode` (exact/step), and the
  recursive editor for the table's `argument` expression. No grammar node type
  is left to raw-JSON editing.
- Regression repair folded in: slice 2 defaulted a literal's dimension to the
  lowercase `"tbd"` token, which fails the document codec (`decode_quantity`
  "unknown dimension token") and the schema `DimensionId` enum — both require
  uppercase `"TBD"`. Fixed the shared `DIMENSIONS` vocabulary, the literal
  default, and the literal-dimension fallback; added a unit guard.
- D-02b gate held: purely structured form controls — **no writable expression
  text syntax and no text rendering of the AST**. The typed AST stays the sole
  edited, checksum-bound form (DEC-022).
- Lossless: row edits patch only the touched row; untouched rows and sibling
  subtrees round-trip verbatim. Read-only preservation now covers only the
  refusal markers (`unsupported_form`/`unsafe_host_access`) and unrecognized
  tags.
- Manual check: open "Rule Packs" → "New draft rule pack"; in the Expression
  composer set the root node type to `interpolate`; a default two-row table
  appears with `TBD` placeholders; edit a row's result and add/remove rows;
  switch the root to `lookup` and pick `step` mode; confirm the document JSON
  below updates to the table-backed AST and the table round-trips when you edit
  the argument child.
- Validation: targeted Vitest `src/features/rule-packs` 26/26 (was 21);
  Playwright `-g "rule-pack manager"` 2/2 (chromium-desktop + chromium-compact,
  table sub-editor driven from blank); `tsc -b` clean. Full-suite Vitest on
  this busy host showed the documented load-induced timeout flakiness — every
  failure a per-test timeout in `App.test.tsx` (a file this tranche does not
  touch), none in the rule-pack surface. Confirmed tranche-independent: once
  host load cleared, `App.test.tsx` passed 52/52 on both my working tree and
  pristine HEAD (my runtime files stashed), and no failing test renders the
  changed code — see the run record's evidence chain.
- Evidence: DEL-07-03 run record
  `WORKING_ITEMS_RUN_2026-06-13_TP-C2-TABLENODE-001.md`.
- Boundary review: no lifecycle state change; no release-readiness,
  professional, certification, sealing, authentication, or code-compliance
  claim.

## TP-MAC-151 rule-pack variable-declaration form builders (`TP-C2-DECLEDITOR-001`, 2026-06-13)

- Phase C2 slice 4: a structured editor for the rule pack's variable
  declarations — its `required_inputs` and user-supplied `value_slots`, the
  named variables a formula's `variable_ref` binds to. New
  `features/rule-packs/DeclarationsEditor.tsx` adds / removes / edits each
  entry through form controls (text ids/names/units, closed-set selects for
  source_kind / slot_kind / value_status / required_for / completeness_status,
  and the shared `DimensionSelect` for the quantity_intent dimension). Add
  seeds a schema-valid default with a fresh unique id; Remove is blocked at the
  `minItems:1` schema floor for both arrays. Until this slice these arrays were
  raw-JSON only and the composer's variable picker had nothing to offer.
- Authority-vs-plan fix: the slice-2/3 residual wording "required-input /
  value-slot / **load-combination** form builders" was a misnomer — the
  authoritative `rule_pack.schema.yaml` is `additionalProperties:false` with
  **no `load_combinations` member** (load combinations are a *model* concept,
  Phase A4). The rule pack's third user-authored member is `check_definitions`;
  the plan/log are corrected to name it (authority unchanged).
- D-02b gate held: purely structured controls — **no writable expression text
  syntax**; the editor authors variable *declarations*, never expression text.
  No invented numeric allowable value (`value_status: not_provided`); no
  silent defaults (out-of-vocabulary stored tokens surface as "(current) X",
  never snapped); private-by-default placeholders.
- Lossless: a patched entry keeps every member it carried (provenance, the
  const-true relaxation flags, missing-value diagnostic); a nested
  `quantity_intent` edit preserves `unit_required`/`dimension_check_required`;
  untouched sibling entries round-trip verbatim.
- Folded-in review fix: gave the shared `DimensionSelect` the same "(current)"
  escape hatch the new `EnumSelect` uses, so an out-of-vocabulary stored
  dimension is surfaced rather than display-snapped to `"TBD"` (also tightens
  the composer's literal/table dimension fields). Caught by an independent
  four-lens adversarial review (schema-conformance, D-02b/IP, react, evidence).
- Manual check: open "Rule Packs" → "New draft rule pack"; in "Variable
  declarations" click "Add required input" (a `user_required_input_2` row
  appears) and edit its dimension to `stress`; confirm the Expression
  composer's variable browser now lists it and the document JSON below shows
  two required inputs; "Add value slot", set slot_kind `allowable` and
  value_status `private_user_supplied`; remove is disabled on the last row of
  each list.
- Validation: targeted Vitest `src/features/rule-packs` 43/43 (was 26);
  full desktop Vitest 285/285 (13 files); `tsc -b` clean; Playwright
  `-g "rule-pack manager"` 2/2 (chromium-desktop + chromium-compact,
  declarations editor driven from blank). No `App.test.tsx` load-flake this run.
- Evidence: DEL-07-03 run record
  `WORKING_ITEMS_RUN_2026-06-13_TP-C2-DECLEDITOR-001.md`.
- Boundary review: no lifecycle state change; no release-readiness,
  professional, certification, sealing, authentication, or code-compliance
  claim.

## TP-MAC-152 rule-pack check-definitions form builder (`TP-C2-CHECKDEF-001`, 2026-06-13)

- Phase C2 slice 5 (final form-builder slice): a structured editor for the rule
  pack's `check_definitions` — the member that binds the declared inputs/slots
  and a formula into an acceptability check. New
  `features/rule-packs/CheckDefinitionsEditor.tsx` adds / removes / edits each
  check: text `check_id`/`name`/`description`; reference pickers for
  `formula_ref` (over declared formula ids) and `required_input_refs` /
  `value_slot_refs` (add/remove lists over declared input/slot ids, held at the
  `minItems:1` floor); a closed-set `acceptability_basis` select; a
  `result_statuses` checkbox multi-select (six `AnalysisStatus` tokens, floored
  at one); and the eight `diagnostic_policy` condition→code selects. Until this
  slice `check_definitions` was the last document-structure authoring member
  edited as raw JSON.
- D-02b gate held: a check binds *references* and selects closed-vocabulary
  tokens through pickers/checkboxes — **no writable expression text syntax**,
  no text rendering of any AST. No invented numeric/standards value; no silent
  defaults (out-of-vocabulary enums surface as "(current) X"; unresolved/`TBD`
  refs surface as "(unresolved) X", never snapped); private-by-default
  placeholder provenance preserved losslessly.
- Lossless: a patched check keeps every member it carried (provenance,
  description, any `Reference.version`); a diagnostic-policy edit preserves the
  other seven bindings; untouched sibling checks round-trip verbatim. Every
  document-structure authoring member now has a form builder; the advanced
  metadata members (diagnostics, classification, checksums, provenance,
  professional_boundary, open_decisions) stay raw-JSON-editable by design.
- Manual check: open "Rule Packs" → "New draft rule pack"; scroll to
  "Acceptability checks"; the template check shows `formula_ref=user_formula_1`
  bound, `required_input_refs`→`user_required_input_1`,
  `value_slot_refs`→`user_limit_slot_1`; toggle a `result_statuses` checkbox and
  confirm the document JSON below updates; "Add check" creates `user_check_2`
  bound to the declared ids; remove is disabled on the last check and the last
  ref of each list and the last result status. Declare a second required input in
  "Variable declarations" and confirm it appears in a check's input-ref picker.
- Validation: targeted Vitest `src/features/rule-packs` 67/67 (was 43); full
  desktop Vitest 308/308 (14 files); `tsc -b` clean; Playwright
  `-g "rule-pack manager"` 2/2 (chromium-desktop + chromium-compact, check
  editor driven from blank). Independent four-lens adversarial review
  (schema-conformance/governance/react/test-honesty) returned no BLOCKER/
  SHOULD-FIX; one NIT (result-status floor for an out-of-vocab last token) folded
  in as an added test.
- Evidence: DEL-07-03 run record
  `WORKING_ITEMS_RUN_2026-06-13_TP-C2-CHECKDEF-001.md`.
- Boundary review: no lifecycle state change; no release-readiness,
  professional, certification, sealing, authentication, or code-compliance
  claim.

## TP-MAC-153 library-import validation seam: `validate_library_import` command + typed service (`TP-C3-IMPORTCMD-001`, 2026-06-13)

- Phase C3 seam slice (no UI change in this tranche). Exposes the C3 foundation
  crate `open_pipe_stress_library_import_document` (the DEL-03-07 port, landed
  `TP-C3-IMPORTVALIDATE-001`) through the desktop boundary so the later import
  wizard can validate an already-parsed material/section/component import
  payload before it is stored in a local-only private library.
- New Tauri command `validate_library_import(payload, libraryKind,
  intendedVisibility)` wrapping the crate's `validate_library_import_tokens`:
  unsupported `library_kind`/`intended_visibility` tokens are **rejected, never
  guessed**. The envelope carries `outcome`, `accepted`, `has_blocking_findings`,
  the ordered `findings` (each with the `blocking`/`quarantine`/`review_required`
  severity), the PKG-02 `import_boundary` `diagnostics` projection, and a fixed
  software-findings-only `professional_boundary_notice`.
- New frontend seam `services/libraryImportService.ts`: `validateLibraryImport`
  returns a discriminated route — browser preview mode returns an explicit
  `LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY` diagnostic and never calls `invoke`
  (no synthesized fallback validator), exactly like the units and rule-pack
  seams. `partitionLibraryImportFindings` splits findings along the PRD §13.5
  blocking-vs-advisory axis once, at the seam (`blocking`/`quarantine` block;
  `review_required` advises).
- No persistence, no file parsing, no UI surface here — validation is a pure
  function over the payload. Imported private libraries are never transmitted or
  committed (OPS-K-PRIV-1, PRD §13.5/§17.3).
- Manual check (desktop): there is no rendered surface yet, so this is verified
  by automated tests, not a live-browser journey. The import wizard + workspace
  section is the next C3 slice; its Playwright/Vitest UI evidence rides that
  tranche per the H4 posture (as `TP-C2-RPLIFE-001` did for the rule-pack
  backend seam).
- Validation: src-tauri `cargo test` 40/40 (3 new command tests:
  accepted-private / blocked-missing-metadata+diagnostics / unsupported-token
  rejection); `cargo fmt --check` clean; desktop Vitest 313/313 (15 files; 4 new
  in `libraryImportService.test.ts`); `npm run build` (`tsc -b && vite build`)
  clean.
- Evidence: DEL-03-07 run record
  `WORKING_ITEMS_RUN_2026-06-13_TP-C3-IMPORTCMD-001.md`.
- Boundary review: validation statuses are software findings only; no lifecycle
  state change (DEL-03-07 stays CHECKING); no release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-154 local-only private-library persistence: store v11 + CRUD commands (`TP-C3-LIBSTORE-001`, 2026-06-13)

- Phase C3 store slice (no UI change in this tranche). Adds local-only
  persistence for imported material/section/component libraries, mirroring the
  C2 `local_rule_packs` store. Store v11 migration creates `local_libraries`
  (project-scoped, keyed `(project_id, library_kind, library_id)`, local SQLite
  only).
- New Tauri commands `save_local_library` / `open_local_library` /
  `list_local_libraries` / `delete_local_library`, plus the matching typed
  `libraryImportService.ts` routes with the shared browser-unavailable seam.
- **Private-by-default, accepted-only.** `save_local_library` re-validates the
  document at the import boundary with private visibility and **stores only an
  accepted (`PRIVATE_LOCAL_ONLY`) import**; a suspected-protected (`QUARANTINE`)
  or blocked (`REJECTED`) import is **refused** — it returns `stored:false` with
  its findings and writes nothing. Suspected protected content never reaches the
  store (IP boundary). Private libraries are never transmitted or committed
  (OPS-K-PRIV-1, PRD §13.5/§17.3). This deliberately differs from the rule-pack
  "drafts always saveable" store (rationale in the run record); a stored
  audit-trail variant is a flagged human-gated follow-up.
- Manual check (desktop): no rendered surface yet — verified by automated tests,
  not a live-browser journey. The import wizard + workspace section + list panel
  is the next C3 slice; its Playwright/Vitest UI evidence rides that tranche per
  the H4 posture.
- Validation: src-tauri `cargo test` 43/43 (3 new store/gate tests; 3
  migration-ledger tests updated for v11); `cargo fmt --check` clean; desktop
  Vitest 317/317 (15 files; 4 new); `npm run build` (`tsc -b && vite build`)
  clean.
- Evidence: DEL-03-07 run record
  `WORKING_ITEMS_RUN_2026-06-13_TP-C3-LIBSTORE-001.md`.
- Boundary review: validation statuses are software findings only; no lifecycle
  state change (DEL-03-07 stays CHECKING); no release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-155 private library manager GUI: import wizard, §13.5 display, refuse-to-store (`TP-C3-LIBGUI-001`, 2026-06-13)

- Phase C3 **GUI slice** — the import-wizard panel + workspace section the three
  prior C3 slices handed off. New `LibraryManagerPanel`
  (`apps/desktop/src/features/library/`) and a "Libraries" workspace section
  (journey order: operations → loads → libraries → rule-packs → solve),
  replicating the C2 `RulePackManagerPanel`. No backend change — it surfaces the
  prior slices' `validate_library_import` / `save_local_library` /
  `open_local_library` / `list_local_libraries` / `delete_local_library`
  commands verbatim.
- Controls: library-kind selector (material/section/component); intended-
  visibility selector labeled **validation preview** (with a note that **save
  always persists to the private local store**); a built-in **invented private
  starting template** per kind; an import-document JSON textarea; validate /
  save / discard; a project-scoped list with per-entry open/delete.
- **PRD §13.5 display.** Validation findings render split into a **Blocking /
  quarantine** group (blocks import acceptance) and an **Advisory** group
  (human review before acceptance, not blocking), via
  `partitionLibraryImportFindings`, each finding showing code / severity / path
  / message / remediation, plus the backend software-findings-only notice.
- **DEC-036 refuse-to-store, surfaced honestly.** Save shows `stored=<bool>`;
  when a blocked/suspected-protected import is refused, the status carries an
  explicit `DEC-036 refuse-to-store` note and the blocking validation rides
  through to the §13.5 display. Imported private libraries are never transmitted,
  committed, or bundled (OPS-K-PRIV-1, PRD §13.5/§17.3).
- Manual check (desktop/browser): live-browser confirmed via the preview tools —
  the Libraries nav activates the section, the panel renders project-scoped
  (`local SQLite only`), and load-sample populates a private-classified material
  library with the private-by-default status. The accept/store outcomes run in
  the Tauri runtime only; in browser preview validate/save/list report the honest
  `LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY` seam (the documented reason the browser
  e2e does not exercise accept/store — those paths are covered by the src-tauri
  Rust tests and the Vitest desktop-mode mocked panel suite).
- Validation: desktop Vitest **326/326** (16 files; 9 new in
  `LibraryManagerPanel.test.tsx`, incl. desktop-mode §13.5-partition and
  refuse-to-store via mocked `invoke`); `npm run build` (`tsc -b && vite build`)
  clean; `npm run test:e2e -- --grep "library manager"` **2 passed**
  (chromium-desktop + chromium-compact). No backend change, so no `cargo`/`pytest`
  surface is owed.
- Evidence: DEL-03-07 run record
  `WORKING_ITEMS_RUN_2026-06-13_TP-C3-LIBGUI-001.md`.
- Boundary review: validation statuses are software findings only; no lifecycle
  state change (DEL-03-07 stays CHECKING); no release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-156 run rule checks from the GUI: per-check pass/fail/blocked + aggregate (`TP-C4-CHECKGUI-001`, 2026-06-14)

- Phase C4 **GUI slice** — makes the R3 exit criterion (PRD §22.4) GUI-true. The
  C4 backend (`run_rule_checks` command + `core/rules/rule_check_runner`) landed
  with `TP-C4-CHECKRUN-001` but had no GUI surface; this slice adds the
  service route + panel. No backend change — it surfaces the existing command
  verbatim.
- New `ruleCheckService.ts` (typed `runRuleChecks` route + result types mirroring
  the runner crate + pure `deriveRuleCheckBindingPlan` + bundled-demo loader) and
  `RuleCheckRunPanel` (`apps/desktop/src/features/rule-check/`), mounted in the
  **Solve** workspace section beside the completeness `RuleCheckPanel`.
- Pack source: the bundled **invented demo pack**
  (`fixtures/product_preview/invented_demo_rule_pack.json`, byte-parallel to the
  backend example `examples/rule_packs/invented_demo.yaml`), a **saved local-store
  pack** (reuses `listLocalRulePacks`/`openLocalRulePack` — the author→save→run
  journey), or **pasted JSON**.
- Binding controls are **derived from the loaded pack**: a solved-result-row
  `<select>` per `solver_result` input; value+unit entry per user-supplied input
  and per value slot (dimension from the pack); a deferred note per
  `private_library_value` input (C3 residual — treated as unsupplied, blocks).
  Unbound/missing inputs block the check at `RULE_INPUTS_INCOMPLETE` — pass/fail
  is never reported on missing inputs.
- Results: the aggregate status (pass/fail/blocked label + `data-status`) and
  per-check outcomes (status, computed/limit quantity, acceptability relation,
  supplied/MISSING bound inputs, completeness + evaluator findings, diagnostic
  codes) plus the professional-boundary notice.
- Manual check (desktop/browser): live-browser confirmed via the preview tools —
  the Solve nav shows the "Run Rule Checks" panel; "Load demo rule pack" derives
  the three binding controls (`demo_actual_quantity` select, `demo_limit_quantity`
  value, `demo_limit_slot` limit). The runner runs in the Tauri runtime only; in
  browser preview running reports the honest `RULE-CHECK-BACKEND-DESKTOP-ONLY`
  seam (the documented reason the browser e2e does not assert pass/fail — those
  outcomes are covered by the src-tauri Rust command tests and the Vitest
  desktop-mode mocked panel suite).
- Validation: desktop Vitest **339/339** (18 files; +13 — 7 in
  `ruleCheckService.test.ts`, 6 in `RuleCheckRunPanel.test.tsx`); `npm run build`
  (`tsc -b && vite build`) clean; `npx playwright test` **10 passed** (5 specs ×
  chromium-desktop + chromium-compact, incl. the new run-checks spec). No backend
  change, so no `cargo`/`pytest` surface is owed.
- Evidence: DEL-06-02 (primary) + DEL-07-04 run record
  `WORKING_ITEMS_RUN_2026-06-14_TP-C4-CHECKGUI-001.md`.
- Boundary review: emits only the three automatic rule-check statuses; no
  lifecycle state change (deliverables stay CHECKING); no release-readiness,
  professional, certification, sealing, authentication, or code-compliance claim.

## TP-MAC-157 rule-pack ↔ private-library reference resolution (`TP-C3C4-LIBREF-001`, 2026-06-14)

- Phase C3↔C4 coupling (backend-resolution slice): a rule-pack
  `private_library_value` required input can now reference a value in a saved
  private library and have it resolved at rule-check run time. Closes the C3
  "rule-pack ↔ library reference wiring" residual and the C4
  `private_library_value` resolution residual together.
- **Schema (additive, PROPOSAL):** optional `library_value_ref`
  (`library_kind`/`library_id`/`record_id`/`slot_id`) on `RequiredInput` in
  `schemas/rule_pack.schema.yaml`. The rule pack carries the **reference only**;
  the private value is read at run time and **never embedded** in the rule pack
  (IP boundary). Awaits human ratification (companion to DEC-031).
- **Runner:** new `LibraryValueBinding` + `library_values`; a
  `private_library_value` input binds from a resolved library value (note cites
  the library reference) or stays unsupplied + note (never a silent pass).
- **Command:** `run_rule_checks` refactored into a store-free
  `run_rule_checks_core` + a wrapper that resolves `library_value_ref` from the
  local private-library store (material allowable slots: `material_records` →
  `allowables` → `value.magnitude`/`unit_ref.ref_id`) into bindings; unresolvable
  references are omitted so the check blocks.
- **GUI:** the run panel passes `projectId`, and the library-input row shows the
  reference (`kind:id → record → slot`) with an honest "resolves from the local
  private library store at run time; value never embedded" note (replacing the
  earlier deferred note). The run result's bound inputs show whether each library
  input resolved.
- Manual check (desktop/browser): the run-checks panel's library-input row
  renders the reference + boundary note (Vitest panel test asserts it); the
  browser e2e run-checks seam is unaffected (the demo pack has no library
  inputs). Pass/fail with a resolved library value is covered by the src-tauri
  Rust command tests and the Vitest desktop-mode panel suite.
- Validation: runner cargo **11 unit + 3 integration**; src-tauri cargo **53**
  (+5 library-resolution); `pytest tests/test_rule_pack_schema.py` **5**;
  desktop Vitest **340** (+1); `npm run build` clean; `npx playwright test`
  **10/10** (two viewports); `cargo fmt --check` clean.
- Evidence: run record `WORKING_ITEMS_RUN_2026-06-14_TP-C3C4-LIBREF-001.md`
  (DEL-06-02 primary; DEL-06-01 schema; DEL-03-07 library coupling).
- Boundary review: private library values resolved at run time, never embedded
  or committed; status-vocabulary-only; deliverables stay CHECKING; no
  release-readiness, professional, certification, sealing, authentication, or
  code-compliance claim. The schema member is a PROPOSAL pending human
  ratification.

## TP-MAC-158 author the library reference in the rule-pack editor (`TP-C3-LIBREFAUTHOR-001`, 2026-06-14)

- Phase C3 residual (C2 authoring half of the rule-pack ↔ private-library
  round-trip): the rule-pack declarations form-builder now authors a
  `private_library_value` required input's `library_value_ref` through
  structured controls, instead of hand-editing raw document JSON. Pairs with
  TP-MAC-157 (which resolves the reference at run time): author here → resolve
  there.
- **GUI (`DeclarationsEditor.tsx`):** when a required input's `source_kind` is
  set to `private_library_value`, a reference sub-form appears with a
  `library_kind` select (material/section/component — the verbatim schema
  vocabulary, no "TBD" member) plus `library_id`/`record_id`/`slot_id` text
  fields, an IP-boundary note ("resolved at check-run time … never embedded in
  the rule pack"), and a "Remove library reference" control. Switching to
  `private_library_value` **seeds a complete four-member reference** (kind →
  first kind; ids → visible uppercase `"TBD"` placeholders), so an unfilled
  reference resolves to nothing and the input blocks — never a partial schema
  shape, never a silent pass. A reference left after the source_kind is changed
  away stays **visible and removable** (never silently hidden). Still
  form-only (no writable expression text — D-02b AWAITING_RULING).
- **Boundary:** frontend-only; no schema/backend change (the optional
  `library_value_ref` member and its run-time resolution landed in
  TP-C3C4-LIBREF-001; the Value-based `validate_rule_pack_document` tolerates
  it). The reference is carried in the pack; the private value is never embedded
  or committed.
- Manual check (browser e2e, both viewports): in the rule-pack manager draft,
  setting the appended input's `source_kind` to `private_library_value` reveals
  the sub-form + boundary note; filling the four controls writes a complete
  `library_value_ref` into the canonical draft JSON
  (`required_inputs[last].library_value_ref`). Asserted in
  `e2e/r2-smoke.spec.ts` (the rule-pack manager test).
- Validation: desktop Vitest **345** (+5: LIBRARY_KINDS vocab, default-ref
  shape, seed-on-switch, first-edit completion, stale-ref visible+removable);
  `npm run build` clean; `npx playwright test` **10/10** (two viewports). No
  Rust/Python/schema surfaces touched.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFAUTHOR-001.md` (DEL-06-02 primary;
  DEL-03-07 library coupling).
- Boundary review: local-only; status-vocabulary-only; deliverables stay
  CHECKING; no release-readiness, professional, certification, sealing,
  authentication, or code-compliance claim. The `library_value_ref` schema
  member it authors remains a PROPOSAL pending human ratification.

## TP-MAC-159 resolve section/component library references at run time (`TP-C3-LIBREFSECCOMP-001`, 2026-06-14)

- Phase C3 residual ("section/component slot resolution"): the rule-check
  run-time resolver now resolves a `private_library_value` input's
  `library_value_ref` for **all three** library kinds. Before this it resolved
  `material` allowable slots only, so a `section`- or `component`-kind reference
  (already authorable since TP-MAC-158) never resolved and the input silently
  blocked. Closes the first-listed residual carried by TP-MAC-157/158.
- **Backend only (`apps/desktop/src-tauri/src/lib.rs`):** `extract_library_slot_value`
  now dispatches by `library_kind` to the authoritative library-schema shapes —
  `material` → `material_records[material_id].allowables[allowable_id]`
  (`value.unit_ref.ref_id`); `section` →
  `section_records[section_id].dimensions[dimension_id]` or
  `…properties[property_id]` (`value.unit`); `component` →
  `component_records[component_id].fields[field_id]` (`value.unit`). Unknown kind
  / missing record/slot/value → omitted, so the input blocks (never a silent
  pass). Material behaviour is unchanged. No schema/runner/frontend change.
- **IP boundary (unchanged):** the rule pack carries the reference only; every
  kind's private value is read at run time from the local store and never
  embedded in the rule pack.
- Manual check posture: no Playwright spec extension — library resolution is
  **desktop-store only** (the local private-library SQLite store is not reachable
  from the browser preview, which keeps its honest store-unavailable seam). The
  behaviour is exercised by the Rust command/store tests (same posture
  TP-MAC-157 used for the material resolution it shipped). No new UI surface:
  the authoring selector (TP-MAC-158) and read-only run-panel reference display
  (TP-MAC-157) already cover section/component; this slice only makes their
  references resolve.
- Validation: src-tauri cargo **57** (+4: section dimension+property,
  component field, unknown-kind, section store-resolution integration);
  `cargo fmt --check` clean; five-surface DEC-025 sweep PASS (cargo crate sweep,
  pytest, desktop Vitest+wasm, Playwright e2e ×2, production build) — see the
  committed sweep summary.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFSECCOMP-001.md` (DEL-06-02 primary;
  DEL-03-07 library coupling).
- Boundary review: local-only; status-vocabulary-only; deliverables stay
  CHECKING; no release-readiness, professional, certification, sealing,
  authentication, or code-compliance claim. The `library_value_ref` schema
  member it reads remains a PROPOSAL pending human ratification.

## TP-MAC-160 ratify `library_value_ref` + bump rule-pack schema_version to 0.3.0 (`TP-C3-LIBREFRATIFY-001`, DEC-038, 2026-06-14)

- Records the human ratification (`DEC-038`) of the additive `library_value_ref`
  rule-pack schema member (added as PROPOSAL by TP-MAC-157) and the coupled
  version bump: the rule-pack `schema_version` moves **0.2.0 → 0.3.0** (DEC-033
  additive-minor policy adopted for rule packs, because `RequiredInput` is
  `additionalProperties:false` so old strict validators reject packs using the
  new member — a minor-bump signal).
- **Packs:** the bundled demo pack
  (`fixtures/product_preview/invented_demo_rule_pack.json` and the byte-identical
  `examples/rule_packs/invented_demo.yaml`) now declares `schema_version` 0.3.0
  (top-level + metadata echo); `rule_pack_checksum` re-stamped
  `9910cec… → 60c7ba2…`. `rule_pack_version` (author content version) and
  `grammar_version` (1.0.0) unchanged.
- **Editor:** a new private draft from "New draft rule pack" now declares
  `schema_version` 0.3.0 (`rulePackService.buildDraftRulePackDocument`).
- Manual check posture: version-string + checksum change only — no behaviour
  change, no new UI surface. Covered by the Vitest draft-template assertion
  (0.3.0) and the Playwright e2e lanes, which load the re-stamped demo fixture;
  the Rust `example_pack_stamped_checksum_matches_recomputation` test re-passes
  against the new stamp.
- The unrelated **model-document** schema version (also 0.2.0, DEC-033) was not
  touched.
- Validation: Rust checksum-recomputation test re-passes; five-surface DEC-025
  sweep PASS (cargo crate sweep, pytest rule-pack-schema, desktop Vitest+wasm,
  Playwright e2e ×2, production build) — see the committed sweep summary.
- Evidence: run record `WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFRATIFY-001.md`
  (DEL-06-02 primary; DEL-06-01 schema); decision `DEC-038` in
  `SOFTWARE_DECOMP.md` §12.
- Boundary review: local-only; status-vocabulary-only; reference-only member
  ratified (private values never embedded); deliverables stay CHECKING; no
  release-readiness, professional, certification, sealing, authentication, or
  code-compliance claim. `DEC-038` is a schema-member ratification + version
  disposition, not a lifecycle/release act.

## TP-MAC-161 richer library/record/slot resolution-preview picker in the run panel (`TP-C3-LIBREFPICKER-001`, 2026-06-14)

- Closes the last consistently-named Phase C **C3** residual — "a richer C4
  run-panel library/record/slot picker (the panel surfaces the reference
  read-only)". The Run Rule Checks panel previously showed each
  `private_library_value` input's authored `library_value_ref` as read-only text;
  the only resolution signal was post-run (`bound_inputs` supplied/MISSING).
- **Frontend only.** Each library input with a reference now has a **"Preview
  resolution"** button. It queries the local private-library store
  (`list_local_libraries` + `open_local_library`, desktop-only), indexes the
  records/slots with a new pure helper that **mirrors the desktop resolver's
  per-kind dispatch exactly** (material `material_records[material_id].allowables`;
  section `section_records[section_id].dimensions|properties`; component
  `component_records[component_id].fields`), and surfaces a resolution badge
  (resolves / library_missing / record_missing / slot_missing) plus a read-only
  browse of the available libraries → records → slot ids, with the authored
  record/slot marked "(referenced)".
- **Read-only to the pack.** The picker never mutates the rule pack, never
  overrides the authored reference at run time (the backend run still resolves
  from the pack reference + projectId), and never renders the private value.
  Run-time *override* was deliberately not built (it would need a human design
  ruling) — recorded as a non-goal, not a residual.
- Manual check posture: the browser-reachable slice — the "Preview resolution"
  control and its honest desktop-only store seam
  (`LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY`) — is automated in the `r2-smoke.spec.ts`
  rule-check e2e (both viewports). The desktop-store resolution/browse itself is
  desktop-only (not browser-reachable) and is exercised by the Vitest
  desktop-mode mocked-`invoke` suite (same posture as TP-MAC-157/159).
- Validation: desktop Vitest **357** (+12: 7 helper unit tests, 5 panel tests);
  `tsc -b` clean; `npm run build` clean; `npx playwright test` 10/10 (two
  viewports); five-surface DEC-025 sweep PASS (cargo crate sweep, pytest, desktop
  Vitest+wasm, Playwright e2e ×2, production build) — see the committed sweep
  summary.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFPICKER-001.md` (DEL-06-02 primary;
  DEL-03-07 library coupling).
- Boundary review: frontend/local-only; status-vocabulary-only; private values
  read only at run time and never embedded/committed/rendered; deliverables stay
  CHECKING; no release-readiness, professional, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-162 wire the GUI rule-check aggregate into the app-held analysis-run envelope (`TP-C4-APPAGG-001`, 2026-06-14)

- Closes the Phase C **C4** residual "app-side wiring of the GUI run aggregate
  into an app-held/exported envelope" — the app-side analog of the non-GUI
  assembly residual `TP-C4-AGGSTATUS-001` closed headless-side. Before this, the
  GUI worst-of rule-check aggregate (`RULE_INPUTS_INCOMPLETE` /
  `USER_RULE_CHECKED` / `USER_RULE_FAILED`) lived only in the Run Rule Checks
  panel; the app-held analysis-run envelope (DEL-14-02) and its
  `ResultExportPanel` / `ReportPanel` consumers (which read `run.analysis_status`)
  still reported the plain solve's `RULE_INPUTS_INCOMPLETE`.
- **Frontend only.** The panel now lifts its aggregate via an optional
  `onAggregateChange` callback; `App` rebuilds the app-held analysis-run envelope
  from `(result, aggregate)`; `buildAnalysisRunPreview` composes a recognized
  aggregate into the analysis-run **record's own** `analysis_status` and its
  `analysis_run_record` hash, while the embedded `result_envelope` hash still
  binds the **raw solve** — so the hash-bound solve envelope is never mutated. An
  absent or unrecognized aggregate falls back to the solve `rule_check` (no
  silent coercion, no false pass).
- Manual/automation posture (H4 exception, no new e2e spec): the end-to-end
  aggregate→envelope flow is **desktop-only** — running rule checks
  (`run_rule_checks`) needs the Tauri backend, and in browser preview a run
  returns the explicit unavailable seam (producing no aggregate, leaving the
  app-held envelope solve-only and browser-visible behavior unchanged). The
  browser Playwright harness therefore cannot exercise the flow. Both composition
  halves are instead covered by Vitest: the panel lift (desktop-mode mocked
  `invoke`) and the envelope threading (`buildAnalysisRunPreview` pure test,
  asserting the `result_envelope` hash stays byte-stable while `analysis_status`
  and the run-record hash reflect the aggregate). Same posture as the
  non-browser-reachable `TP-C4-AGGSTATUS-001` seam.
- Validation: desktop Vitest **365** (+8: 6 previewService, 2 RuleCheckRunPanel);
  `tsc -b` clean; `npm run build` clean; `npx playwright test` 10/10 (two
  viewports, unchanged — confirms no browser-visible regression); five-surface
  DEC-025 sweep PASS (cargo crate sweep, pytest, desktop Vitest+wasm, Playwright
  e2e ×2, production build) — see the committed sweep summary.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-14_TP-C4-APPAGG-001.md` (DEL-14-02 primary; coupled
  DEL-06-02 aggregate source, DEL-08-04 result-export consumer).
- Boundary review: frontend/local-only; status-vocabulary-only; conservative
  fallback prevents any false pass; `HUMAN_REVIEW_REQUIRED` still always present;
  raw solve-envelope hash never mutated; deliverables stay CHECKING; no
  release-readiness, professional, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-163 author a check's acceptability relation beyond `<=` (`TP-C4-ACCEPTREL-001`, 2026-06-14)

- Lands the `acceptability_relation` half of the Phase C **C4** named remaining
  scope ("the future additive `acceptability_relation` / solver-result-selector
  schema members"). Before this, rule checks could only express `≤`: the runner
  hard-coded `LessThanOrEqual`, so a pack could not state that a check passes
  when the computed quantity is `≥` / `<` / `>` its user-supplied limit.
- **Additive, optional, backward-compatible PROPOSAL member.** New optional
  `acceptability_relation` on `CheckDefinition` (four ordering relations only:
  `less_than` / `less_than_or_equal` / `greater_than` / `greater_than_or_equal`),
  mapped to the frozen grammar's existing comparison operators (DEC-022 — no
  grammar change). Absent → `less_than_or_equal` (the demo and every existing
  pack are byte-identical and behave identically). An explicit but unrecognized
  token **blocks** the check (`RULE_EVALUATOR_ERROR` → `RULE_INPUTS_INCOMPLETE`),
  never a silent `≤`. Equality acceptance is a deliberate non-goal. Built as a
  PROPOSAL awaiting human ratification (precedent `DEC-038` / `library_value_ref`);
  ratification bumps rule-pack `schema_version` 0.3.0 → 0.4.0 (`DEC-033`).
- GUI: the C2 check-definitions editor gains a relation selector
  (`rule-pack-check-acceptability-relation`); the draft template authors the
  explicit `less_than_or_equal` default; out-of-vocabulary stored tokens surface
  as `(current) …` (no silent snap). The Run Rule Checks panel already renders
  the relation the backend reports — it now varies with the authored member.
- Automation posture (H4 default): the browser Playwright `rule-pack manager`
  spec is extended to assert the relation selector shows the `less_than_or_equal`
  default and that choosing `greater_than_or_equal` rewrites the canonical
  document JSON — the changed authoring behaviour exercised in a real browser on
  both viewports. Relation *evaluation* semantics (which require the Tauri
  backend) are covered by the Rust runner integration tests.
- Validation: `cargo test` runner crate **18** (11 unit + 4 new relation
  integration + 3 demo, demo unchanged → backward compat); `cargo fmt --check`
  clean; `pytest tests/test_rule_pack_schema.py` **5** (schema valid, fixtures
  conform); desktop Vitest **367** (+2 net); `npm run build` clean (1653
  modules); `npx playwright test -g "rule-pack manager"` **2/2**; five-surface
  DEC-025 sweep — see the committed sweep summary.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-14_TP-C4-ACCEPTREL-001.md` (DEL-06-02 primary;
  coupled DEL-06-01 schema, DEL-07-03 editor).
- Boundary review: local-only pure runner + invented fixtures; status-
  vocabulary-only; no silent default (absent→`≤` is a documented backward-compat
  default; unknown token blocks); deliverables stay CHECKING; no release-
  readiness, professional, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-164 author a `solver_result` input's result-row reference in the pack (`TP-C4-SOLVERREF-001`, 2026-06-14)

- Lands the `solver-result-selector` half of the Phase C **C4** named remaining
  scope ("a selector tying a `solver_result` input to a solved result row,
  retiring the caller-supplied binding"). Before this, a `solver_result` input's
  binding to a solved result row was **caller-supplied only** — the pack could
  not declare which result row each solver input reads. This is the
  **backend-resolution slice**; GUI authoring and a run-panel picker are
  follow-ups.
- **Additive, optional, backward-compatible PROPOSAL member.** New optional
  `solver_result_ref` (`{result_id}`) on `RequiredInput`, mirroring
  `library_value_ref` exactly. `result_id` reuses the existing stable envelope
  row id (e.g. `result:stress:demo`, `result:disp:node-N-130:ux`) — no new
  addressing model. Built as a PROPOSAL awaiting human ratification (precedent
  `DEC-038` / `library_value_ref`); ratification bumps rule-pack `schema_version`
  under the `DEC-033` additive-minor policy.
- **Authored reference is canonical.** A `solver_result` input that carries a
  `solver_result_ref` is governed by it alone — the caller-supplied selector for
  that input is dropped (no run-time override; matches the `library_value_ref`
  ruling). An unresolvable reference (missing row, or no numeric value / unit)
  **blocks** the input at `RULE_INPUTS_INCOMPLETE` — never a silent pass, and
  never a caller rescue. A pack with no reference behaves exactly as before
  (every solver value still comes from the caller-supplied selectors).
- Runner unchanged / stays pure (still consumes pre-resolved
  `SolverResultBinding`s). Resolution lives in the desktop command, exactly as the
  library path resolves from the local store.
- Automation posture (H4 default): backend-only slice, no user-visible behaviour
  change (the GUI does not yet author or consume the member), so no
  Playwright/Vitest extension is owed. Resolution semantics are covered by the
  Rust src-tauri integration tests (no caller selector → authored ref alone
  passes; authored ref to a missing row blocks over a would-resolve caller
  selector).
- Validation: `cargo test` (src-tauri) **61** (57 baseline + 4 new); `cargo fmt
  --check` clean; `pytest tests/test_rule_pack_schema.py` **5** (schema valid,
  fixtures conform); `cargo test` runner crate **18** unchanged (runner not
  touched); no TS changed → Vitest/build unaffected; five-surface DEC-025 sweep —
  see the committed sweep summary.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-14_TP-C4-SOLVERREF-001.md` (DEL-06-02 primary;
  coupled DEL-06-01 schema).
- Boundary review: local-only resolution + invented fixtures; authored reference
  carries only a result-row id, never an embedded solver value; status-
  vocabulary-only; no silent default (unresolvable reference blocks); deliverables
  stay CHECKING; no release-readiness, professional, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-165 ratify the two C4 additive schema members + bump rule-pack schema_version to 0.4.0 (`TP-C4-RATIFY-001`, DEC-039, 2026-06-14)

- Human project authority ruling ("I approve adding the new schema members")
  ratifies both PROPOSAL additive members — `acceptability_relation`
  (`TP-C4-ACCEPTREL-001`) and `solver_result_ref` (`TP-C4-SOLVERREF-001`) — as
  permanent parts of `schemas/rule_pack.schema.yaml`. Recorded as `DEC-039`.
- Both members are already in the schema (no shape change); this is the bounded
  ratification follow-up: lifecycle PROPOSAL → permanent, plus the
  `DEC-033`/`DEC-038` additive-minor version bump. Both members ratified in one
  event → a **single** minor step: rule-pack `schema_version` **0.3.0 → 0.4.0**
  (not two bumps). `grammar_version` stays 1.0.0; author content version
  unchanged.
- Version bump + checksum re-stamp + transitive hash re-sync (no rule/grammar
  content change): the canonical demo pack
  (`examples/rule_packs/invented_demo.yaml`), its preview fixture twin
  (`fixtures/product_preview/invented_demo_rule_pack.json`), and the new-draft
  editor template (`rulePackService.ts`) all declare 0.4.0; the demo/fixture
  `rule_pack_checksum` re-stamped to `c2b4ddce…`; the draft-version Vitest
  assertion updated to 0.4.0. Transitively, the invented model
  `examples/models/invented/fake_rule_pack_toy_model.json` hash-references the
  demo-pack file bytes, so its rule-pack-ref raw-byte digest and recomputed JCS
  project hash were re-synced (same step as `DEC-038`; caught by the full pytest
  surface).
- Validation: `pytest tests/test_rule_pack_schema.py` **5** (JCS checksum-parity
  recompute matches the re-stamped value); `cargo test` `rule_pack_document`
  **10** and `rule_check_runner` **18** (demo golden checksum + validation +
  invented-demo runs recompute and match); targeted desktop Vitest **95**
  (`rule-packs`/`rule-check`/`rulePackService`/`ruleCheckService`; draft now
  0.4.0, run-checks load the re-stamped fixture); five-surface DEC-025 sweep —
  see the committed sweep summary.
- Evidence: run record `WORKING_ITEMS_RUN_2026-06-14_TP-C4-RATIFY-001.md`
  (DEL-06-02 primary; coupled DEL-06-01 schema). `DEC-039` in `SOFTWARE_DECOMP.md`
  §12; completion-plan C4 row + completion log updated.
- Boundary review: metadata-only (a version label + its binding hash); no new
  pack content, rule, or grammar change; deliverables stay CHECKING; no release-
  readiness, professional, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-166 author the solver-result reference in the rule-pack editor (`TP-C4-SOLVERREFAUTHOR-001`, 2026-06-14)

- Phase C4 residual (the C2 authoring half of the solver-result binding): the
  rule-pack declarations form-builder now authors a `solver_result` required
  input's `solver_result_ref` through a structured control, instead of
  hand-editing raw document JSON or relying solely on the run-panel selector.
  Mirrors TP-MAC-158 (`library_value_ref` authoring) and pairs with TP-MAC-164
  (`TP-C4-SOLVERREF-001`, which resolves the reference at run time): author
  here → resolve there.
- **GUI (`DeclarationsEditor.tsx`):** when a required input's `source_kind` is
  set to `solver_result`, a reference sub-form appears with a single `result_id`
  text field, a note ("Binds this input to a specific row of the solved result
  envelope by result id, resolved at check-run time … an authored reference is
  the canonical binding — it supersedes the run panel's per-input result
  selector; remove it to bind from the run panel instead"), and a "Remove
  solver-result reference" control. Switching to `solver_result` **seeds a
  complete reference** (`result_id` → visible uppercase `"TBD"` placeholder), so
  an unfilled reference matches no result row and the input blocks — never a
  partial schema shape, never a silent pass. A reference left after the
  source_kind is changed away stays **visible and removable** (never silently
  hidden). Seeding a solver reference does not also seed a library reference
  (independent kinds). Still form-only (no writable expression text — D-02b
  `DEC-037` Option O-C: AST-only).
- **Boundary:** frontend-only; no schema/backend change (the optional
  `solver_result_ref` member and its run-time resolution landed in
  TP-C4-SOLVERREF-001 and was ratified at `schema_version` 0.4.0 by `DEC-039`;
  the Value-based `validate_rule_pack_document` tolerates it). The reference is
  carried in the pack; no solver result *value* is embedded.
- Manual check (browser e2e, both viewports): in the rule-pack manager draft,
  adding an input and setting its `source_kind` to `solver_result` reveals the
  sub-form + supersede note; filling `result_id` writes a complete
  `solver_result_ref` into the canonical draft JSON
  (`required_inputs[last].solver_result_ref`). Asserted in
  `e2e/r2-smoke.spec.ts` (the rule-pack manager test).
- Validation: desktop Vitest **372/372** (+5: default-ref shape, seed-on-switch
  + no library-seed, first-edit completion, lossless id edit, stale-ref
  visible+removable); `npm run build` clean (tsc typecheck + vite); Playwright
  rule-pack manager journey **2/2** targeted + **10/10** under the sweep (two
  viewports; the new authoring assertions ride the rule-pack manager test).
- **Gate status (PUSH PENDING):** every surface passed individually — cargo
  crate sweep (28 crates ok), pytest **359**, desktop Vitest **372/372**,
  Playwright **10/10**, production build clean — but the combined commit-bound
  five-surface DEC-025 sweep did **not** produce a single all-green run this
  session. Sustained **external** machine load (a concurrent coding agent + git
  + file sync, load avg 6–21 on 8 cores) caused rotating **infrastructure**
  timeouts: App.test.tsx per-test 10–15s caps (unrelated to this slice; pass
  52/52 standalone), then a Playwright worker-teardown force-kill *after all 10
  tests passed*. No test or assertion failed. Gate PASS + `git push` are pending
  a quiet-machine sweep re-run or a human ruling — see the run record.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-14_TP-C4-SOLVERREFAUTHOR-001.md` (DEL-06-02
  primary).
- Boundary review: local-only; status-vocabulary-only; deliverables stay
  CHECKING; no release-readiness, professional, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-167 preview authored solver-result references in the run panel (`TP-C4-SOLVERREFPICKER-001`, 2026-06-15)

- Phase C4 residual closed: `RuleCheckRunPanel` now detects a `solver_result`
  required input with an authored `solver_result_ref`, treats that reference as
  the canonical in-pack binding, and hides the legacy run-panel selector for
  that input. Packs without `solver_result_ref` keep the existing manual result
  row selector path.
- The new "Preview result row" control is read-only with respect to the pack. It
  classifies the authored `result_id` against the current solved envelope as
  `resolves`, `result_missing`, or `no_result_rows`, lists available result rows
  when present, and states that unresolved references block at
  `RULE_INPUTS_INCOMPLETE` instead of inventing a fallback binding.
- Browser smoke: the existing run-rule-checks Playwright path now asserts the
  visible solver-reference preview and the honest `no_result_rows` state in
  browser preview before a solve exists. Component tests pin the resolving and
  missing-row cases against a solved envelope.
- Validation: focused Vitest (`RuleCheckRunPanel`, `ruleCheckService`) **27/27**;
  desktop Vitest **378/378**; `npm run build --workspace apps/desktop` clean;
  focused Playwright run-rule-checks smoke **2/2** after correcting the browser
  expectation to `no_result_rows` (first attempt correctly exposed the mismatch).
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-15_TP-C4-SOLVERREFPICKER-001.md` (DEL-06-02
  primary); completion log updated.
- Boundary review: frontend-only; no schema/backend change; local-only;
  status-vocabulary-only; no private value or protected content embedded; no
  release-readiness, professional, certification, sealing, authentication, or
  code-compliance claim.

## TP-MAC-168 rule-pack declaration unit selectors (`TP-UNITS-B2-RULEPACKUNITS-001`, 2026-06-15)

- Phase B2 rule-pack unit I/O slice: `DeclarationsEditor` now uses the reviewed
  DEC-018 unit catalog for `required_inputs[].quantity_intent.unit_ref` and
  `value_slots[].quantity_intent.unit_ref` when the desktop/Tauri
  `get_unit_catalog` route is available.
- Desktop behavior covered by mocked-Tauri component tests: the selector offers
  dimension-compatible accepted units only (for stress declarations, `Pa` and
  `MPa`; not length/force units), preserves a stored out-of-catalog unit as the
  current value instead of snapping it, and writes the selected unit back while
  preserving the quantity-intent flags.
- Browser behavior covered in Playwright: no fallback catalog is synthesized;
  the rule-pack manager journey keeps manual declaration unit entry available
  and records `dimension=stress`, `unit_ref=MPa` in the canonical draft JSON.
- Validation: focused `DeclarationsEditor` Vitest **29/29**; desktop Vitest
  **381/381**; `npm run build --workspace apps/desktop` clean with the existing
  Vite chunk-size warning; Playwright e2e **10/10** across the configured
  desktop/compact Chromium projects.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-RULEPACKUNITS-001.md` (DEL-02-02
  primary); completion plan/log updated.
- Boundary review: frontend-only; no schema/backend/evaluator/grammar change;
  no protected standards content, private value, browser fallback catalog,
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-169 rule-pack expression unit selectors (`TP-UNITS-B2-RULEEXPRUNITS-001`, 2026-06-15)

- Phase B2/B3 rule-pack unit I/O slice: `ExpressionComposer` now uses the
  reviewed DEC-018 unit catalog for expression literal `quantity.unit_ref`,
  table `argument_unit_ref`, and table `result_unit_ref` when the
  desktop/Tauri `get_unit_catalog` route is available.
- Desktop behavior covered by mocked-Tauri component tests: selectors offer
  dimension-compatible accepted units only (for stress expressions, `Pa` and
  `MPa`; not length/force units), preserve stored out-of-catalog units as the
  current value instead of snapping them, and write selected table/literal units
  back into the canonical AST.
- Browser behavior covered in Playwright: no fallback catalog is synthesized;
  the rule-pack manager journey keeps manual expression unit entry available
  and records literal `dimension=stress`, `unit_ref=MPa` plus table
  `argument_dimension=temperature`, `argument_unit_ref=degC`,
  `result_dimension=stress`, and `result_unit_ref=MPa` in the canonical draft
  JSON.
- Validation: focused `ExpressionComposer` Vitest **19/19**; desktop Vitest
  **384/384**; `npm run build --workspace apps/desktop` clean with the existing
  Vite chunk-size warning; Playwright e2e **10/10** across the configured
  desktop/compact Chromium projects.
- Evidence: run records
  `WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-RULEEXPRUNITS-001.md` (DEL-02-02
  primary and DEL-06-02 companion); completion plan/log updated.
- Boundary review: frontend-only; no schema/backend/evaluator/grammar/parser
  change; no protected standards content, private value, browser fallback
  catalog, release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-170 node coordinate unit payloads (`TP-UNITS-B2-NODECOORDUNITS-001`, 2026-06-15)

- Phase B2/B3 units slice: Property Inspector node coordinate edits
  (`position.x`, `position.y`, `position.z`) now expose the existing unit-aware
  editor control and emit an explicit `{value, unit}` payload instead of a bare
  scalar.
- The operation applier now accepts project-unit quantity edits in scalar or
  payload form, converts compatible DEC-018 length units (`ft`, `mm`, etc.) to
  the model document's stored `project.units.length` value for scalar node
  coordinates, and still blocks incompatible units with
  `OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`.
- Browser evidence: the R2 Playwright smoke selects node `node:N-110`, previews
  a `position.y` edit with `unit=m`, verifies the explicit payload
  `{"value":1.25,"unit":"m"}`, and keeps the existing queue/apply numbering
  unchanged by not queuing this preview-only assertion.
- Validation: operation-applier cargo suite **58 unit + canonical hash + 66
  corpus cases**; desktop Vitest **385/385**; `npm run build --workspace
  apps/desktop` clean with the existing Vite chunk-size warning; focused
  Playwright R2 smoke **2/2** across configured desktop/compact Chromium
  projects.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-NODECOORDUNITS-001.md` (DEL-16-02
  primary); completion plan/log updated.
- Boundary review: no schema, persistence, solver, rule-pack, private-data,
  protected-content, network, telemetry, release-readiness, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-171 load-case inspector primitive unit payloads (`TP-UNITS-B2-LOADINSPECTORUNITS-001`, 2026-06-16)

- Phase B2 units slice: the Property Inspector's selected load-case "First
  primitive magnitude" field now uses the same unit-aware editor control as the
  already-landed material, section, node-coordinate, and load-manager primitive
  magnitude paths.
- Browser evidence: the R2 Playwright smoke selects load case `load:L-100`,
  previews `primitive_loads.0.magnitude.value`, verifies `unit=N/m`, and checks
  the explicit payload `{"value":-225,"unit":"N/m"}` with the
  `update_load` operation intent. The assertion is preview-only and does not
  alter downstream queue/apply numbering in the smoke.
- Validation: focused `App.test.tsx` Vitest **54/54**; desktop Vitest
  **386/386**; `npm run build --workspace apps/desktop` clean with the existing
  Vite chunk-size warning; focused Playwright R2 smoke **2/2** across the
  configured desktop/compact Chromium projects.
- Evidence: run record
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-LOADINSPECTORUNITS-001.md`
  (DEL-07-02 primary); completion plan/log updated.
- Boundary review: frontend-only; no schema, backend operation, persistence,
  solver, rule-pack, private-data, protected-content, network, telemetry,
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-172 PCF conversion witnesses (`TP-UNITS-B2B3-PCFCONVWITNESS-001`, 2026-06-16)

- Phase B2/B3 units slice: the desktop conservative PCF export package now
  embeds `conversion_witnesses` so its millimeter target fields are auditable
  beyond the previously landed unit-system disclosure.
- Package behavior: downloaded PCF JSON includes one witness for each node
  coordinate component and pipe OD/wall field. Each witness records source
  object/field, source value/unit/dimension, target PCF value/unit/field,
  conversion factor, DEC-018/DEL-02-02 basis refs, and preview provenance.
  The manifest now lists `conversion_witnesses.json`, and the validation
  report checks witness count and `MM` target units.
- Regression evidence: `App.test.tsx` decodes the PCF package and verifies the
  `pipe:P-120` outside diameter witness converts `0.168 m` to `168 MM`, that a
  node coordinate witness targets `MM`, that the manifest member is present,
  and that the PCF text carries `OUTSIDE-DIAMETER 168`.
- Validation: focused `App.test.tsx` Vitest **54/54**; desktop Vitest
  **386/386**; `npm run build --workspace apps/desktop` clean with the existing
  Vite chunk-size warning; focused Playwright R2 smoke **2/2** across the
  configured desktop/compact Chromium projects.
- Evidence: run records
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-PCFCONVWITNESS-001.md`
  (DEL-17-07 primary and DEL-02-02 supporting); completion plan/log updated.
- Boundary review: desktop PCF package only; no downstream import
  compatibility, solver-deck validation, target-profile closure, hidden target
  default, protected content, private data, release-readiness, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-173 local project unit round-trip evidence (`TP-UNITS-B2-IMPORTRT-001`, 2026-06-16)

- Phase B2 import/open/save unit I/O slice: local project create/save/open
  summaries now carry deterministic unit round-trip evidence for the restored
  project envelope: `unit_round_trip_status`,
  `unit_round_trip_checked_ref_count`, and `unit_round_trip_signature`.
- Browser and Tauri parity: the browser-preview project service and the
  Tauri SQLite backend both compute the signature from explicit
  `project.units`, material, section, pipe-section, and primitive-load
  magnitude unit refs. Missing unit-bearing values report
  `unit_metadata_missing_review_required`; preserved unit metadata reports
  `unit_metadata_preserved_in_local_project_envelope`.
- UI/export evidence: Project Storage Audit and Project Validation Preflight
  now show a dedicated unit round-trip line and include the same fields in
  their local JSON packets.
- Validation: focused `projectService.test.ts` + `App.test.tsx` Vitest
  **61/61**; desktop Vitest **386/386**; `npm run build --workspace
  apps/desktop` clean with the existing Vite chunk-size warning; Tauri
  focused store regression
  `local_project_store_uses_sqlite_fts5_and_round_trips_model_snapshot`
  passed; rustfmt check passed for `openpipestress-desktop`; in-app Browser
  fallback verified `New blank` -> `Save local` -> `Open local` shows
  `unit_metadata_preserved_in_local_project_envelope` and six blank-project
  unit refs in both storage and validation rows. Initial direct Playwright
  execution found a missing local Chromium cache and timed out during browser
  install, but the later DEC-025 sweep ran the updated Playwright smoke
  successfully (**10/10** dev-server lane plus **1/1** production-dist lane).
- Evidence: run records
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-IMPORTRT-001.md` (DEL-02-05
  primary and DEL-02-02 supporting); completion plan/log updated.
- Boundary review: persistence/evidence only; no unit conversion, import
  target compatibility, solver behavior, protected content, private project
  data, network, telemetry, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-174 CAEPIPE MBF conversion witnesses (`TP-UNITS-B2B3-CAEPIPECONVWITNESS-001`, 2026-06-16)

- Phase B2/B3 units slice: the desktop CAEPIPE MBF smoke-subset export package
  now embeds `conversion_witnesses` for node-coordinate target-format
  conversion beyond the previously landed unit-system disclosure.
- Package behavior: downloaded CAEPIPE MBF JSON includes one witness per node
  coordinate component. Each witness records source node/field, source
  value/unit/dimension, target MBF payload value/unit/field, conversion factor,
  DEC-018/DEL-02-02 basis refs, and preview provenance. The manifest lists
  `conversion_witnesses.json`, and validation checks witness count and `mm`
  target units.
- Export text behavior: MBF node records now use millimeter coordinate values
  with an explicit `UNIT,mm,...` record. The invented preview witness for
  `node:N-120` records `3.2 m -> 3200 mm`, and the MBF text carries
  `NODE,N003,3200,2400,0`.
- Validation: focused `App.test.tsx` Vitest **54/54**; desktop Vitest
  **386/386**; `npm run build` clean with the existing Vite chunk-size
  warning; targeted Playwright R2 smoke spec **10/10** after wasm rebuild.
- Evidence: run records
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-CAEPIPECONVWITNESS-001.md`
  (DEL-17-04 primary and DEL-02-02 supporting); completion plan/log updated.
- Boundary review: desktop CAEPIPE MBF smoke package only; no CAEPIPE target
  version/profile closure, direct MBF stable-ID carrier, downstream import
  compatibility, external CAEPIPE execution, solver validation, protected
  content, private data, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-175 stress-neutral unit preservation witnesses (`TP-UNITS-B2B3-STRESSNEUTRALUNITWITNESS-001`, 2026-06-16)

- Phase B2/B3 units slice: the desktop stress-neutral CSV/JSON package now
  embeds `unit_preservation_witnesses` for every exported result row. The
  package remains unit-preserving: it records source result units and
  dimensions without export-time conversion.
- Package behavior: downloaded stress-neutral JSON includes one witness per
  result row, with source result ref/field, source value/unit/dimension,
  target row ref/field, target value/unit/dimension, `conversion_performed=false`,
  DEC-018/DEL-02-02/DEL-17-06 basis refs, and preview provenance. The manifest
  lists `unit_preservation_witnesses.json`, and validation checks witness
  count plus source/target row agreement.
- Browser evidence: the R2 Playwright smoke checks the visible
  `Unit witnesses` line reports `count=737` and `conversion=false`.
- Validation: focused `App.test.tsx` Vitest **54/54**; desktop Vitest
  **386/386**; `npm run build` clean with the existing Vite chunk-size
  warning; targeted Playwright R2 smoke spec **10/10** after wasm rebuild.
- Evidence: run records
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-STRESSNEUTRALUNITWITNESS-001.md`
  (DEL-17-06 primary and DEL-02-02 supporting); completion plan/log updated.
- Boundary review: desktop stress-neutral review package only; no vendor
  format, target compatibility, comparison pass/fail, solver validation,
  protected content, private data, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-176 rule-check mixed-unit normalization (`TP-UNITS-B2B3-RULECHECKNORM-001`, 2026-06-16)

- Phase B2/B3 units slice: the rule-check runner now normalizes compatible
  DEC-018 catalog units to each rule-pack declaration's unit before formula
  evaluation and top-level acceptability comparison.
- Behavior: exact non-catalog demonstration units still pass only when entered
  and declared strings match; compatible catalog values such as `0.05 MPa`
  against a `Pa` stress declaration convert to `50_000 Pa`; incompatible
  substitutions such as `mm` for stress block at `RULE_INPUTS_INCOMPLETE` with
  `UnitMismatch` rather than evaluating.
- Desktop command evidence: the Tauri `run_rule_checks_core` regression
  re-stamps an invented demo pack whose stress inputs declare `Pa` and verifies
  `0.05 MPa` solver output plus `100 kPa` user limit normalize to `Pa` and
  compute the expected `0.5` ratio through the app command wrapper.
- Validation: rule-check runner cargo tests passed (13 unit + 7 integration
  tests); desktop Tauri cargo tests passed (62/62); desktop Vitest passed
  (18 files, 386 tests); desktop production build passed with the existing Vite
  chunk-size warning.
- Evidence: run records
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-RULECHECKNORM-001.md`
  (DEL-06-02 primary and DEL-02-02 supporting); completion plan/log updated.
- Boundary review: no grammar change, schema change, text parser, protected
  standards content, private project data, network/telemetry path,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-177 authoring journey status rail and compact dock proof (`TP-APP-R3-A3-JOURNEYPATH-001`, 2026-06-16)

- A3/R3 usability follow-up: the desktop shell now exposes a compact
  authoring-journey status rail below the existing workspace section tabs.
  The rail is derived from existing state and reports Model, Apply, Loads,
  Solve, Rules, Report, and Project status; each item navigates to the
  corresponding workspace section.
- Compact layout: the workspace flex ratio now gives the lower dock more
  usable height at the 1280x800 lane while keeping the persistent model tree,
  3D viewport, and property inspector visible. The rail wraps to bounded rows
  at compact width.
- Browser/e2e proof: the R2 smoke spec now asserts the rail text, clicks the
  Loads journey item, and checks real-browser geometry (`bodyOverflow=0`,
  `viewportOverflow=0`, `itemOverflow=0`, dock body height >150px, rail height
  <80px) in the compact lane. In-app Browser at 1280x800 separately measured
  `dockBodyHeight=168` and `journeyHeight=66` after activating Loads.
- Validation: focused App + dead-control Vitest passed (55/55); desktop
  production build passed with the existing Vite chunk-size warning; focused
  compact Playwright smoke passed (1/1); full desktop Vitest passed (18 files,
  386 tests); full desktop Playwright passed (10/10 across 1440x920 and
  1280x800).
- Evidence: DEL-07-06 run record
  `WORKING_ITEMS_RUN_2026-06-16_TP-APP-R3-A3-JOURNEYPATH-001.md`;
  completion plan/log updated.
- Boundary review: shell usability/evidence only; no schema, solver,
  rule-check math, persistence semantics, protected content, private project
  data, network/telemetry behavior, lifecycle state, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed. **F-4 remains open** until the human
  packaged GUI journey is completed.

## TP-MAC-178 review-geometry coordinate unit witnesses (`TP-UNITS-B2B3-REVIEWGEOMUNITWITNESS-001`, 2026-06-16)

- Phase B2/B3 units slice: the desktop review-geometry glTF JSON preview now
  emits `coordinate_unit_witnesses` and a package member
  `coordinate_unit_witnesses.json`.
- Packet behavior: the review-geometry export declares
  `unit-system:dec-018-si-dual-display`, source model units, target glTF
  coordinate unit `m`, `conversion_performed=false`, and the existing
  `preview_z_up_to_gltf_y_up_rotation_x_minus_90` axis transform.
- Witness coverage: 54 coordinate-component records cover pipe endpoints,
  node markers, support markers, and component markers in the same vertex
  order as emitted glTF positions. Regression checks prove `pipe:P-120`
  source Y maps to target glTF Z with the documented sign flip.
- Browser/e2e proof: the R2 Playwright smoke checks the visible Review
  Geometry Export `Unit witnesses` line reports `count=54` and `target=m`.
  In-app Browser verification was attempted but its URL policy blocked
  `http://127.0.0.1:4179/` before navigation; no workaround was used.
- Validation: focused App Vitest passed **54/54**; full desktop Vitest passed
  **386/386**; desktop production build passed with the existing Vite
  chunk-size warning; full desktop Playwright passed **10/10** after rerunning
  an initial Chrome launch SIGKILL before app startup; five-surface DEC-025
  sweep passed and wrote
  `validation/evidence/sweeps/SWEEP_20260616T013908Z_673a4de9628e-dirty.json`.
- Evidence: DEL-17-08 primary run record and DEL-02-02 supporting run record
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-REVIEWGEOMUNITWITNESS-001.md`;
  completion plan/log updated.
- Boundary review: review-geometry browser-preview evidence only; no binary
  GLB package, viewer compatibility claim, solver-geometry equivalence,
  target-compatibility claim, protected content, private data,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-179 native JSON package unit preservation witnesses (`TP-UNITS-B2B3-NATIVEUNITWITNESS-001`, 2026-06-16)

- Phase B2/B3 units slice: the desktop native JSON package review packet now
  declares a `maps/unit_preservation_witnesses.json` package member and
  carries `unit_preservation` evidence for project-owned JSON quantity fields.
- Packet behavior: the native package declares
  `unit-system:dec-018-si-dual-display`, preserves source model/result
  quantity value+unit fields, and records `conversion_performed=false`.
- Witness coverage: 6 project unit declarations, 18 model quantity witnesses,
  and 739 result quantity witnesses are reported in the visible Native JSON
  package panel. Regression checks prove a model pipe outside-diameter witness
  and a result axial-force witness preserve source and target quantities.
- Browser/e2e proof: the R2 Playwright smoke checks the visible Native JSON
  package `Unit witnesses` line reports `project_units=6`,
  `model_quantities=18`, `result_quantities=739`, and `conversion=false`.
  In-app Browser verification completed the solve path and observed
  `state=completed; result_rows=737` plus the same native-package witness
  counts.
- Validation: focused App Vitest passed **54/54**; full desktop Vitest passed
  **386/386**; desktop production build passed with the existing Vite
  chunk-size warning; full desktop Playwright passed **10/10**; five-surface
  DEC-025 dirty-tree sweep passed and wrote
  `validation/evidence/sweeps/SWEEP_20260616T025334Z_92ba64e2b4e3-dirty.json`.
- Evidence: DEL-17-03 primary run record and DEL-02-02 supporting run record
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-NATIVEUNITWITNESS-001.md`;
  completion plan/log updated.
- Boundary review: native JSON browser-preview evidence only; no Python native
  export schema, writer, fixture, parser, public API, downstream target
  compatibility claim, protected content, private data, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-180 rule-pack expression read-only text preview (`TP-C2-ASTTEXTVIEW-001`, 2026-06-16)

- Phase C2 rule-pack editor follow-up under `DEC-037`: the structured
  expression composer now shows a labeled read-only AST-to-text preview for
  the selected formula.
- Boundary behavior: the preview is not editable, has no input/select/textarea
  controls, is never parsed, and the checksum-bound rule-pack member remains
  the structured DEC-022 AST. No writable expression text syntax shipped.
- Browser/e2e proof: the rule-pack Playwright journey asserts the `DEC-037`
  boundary note, the initial preview text for `user_required_input_1`, and the
  updated preview after the AST is changed to a comparison with a `MPa`
  stress literal. In-app Browser verification observed the same preview with
  `editableControlsInPreview=0`.
- Validation: focused `ExpressionComposer` Vitest passed **21/21**; focused
  rule-pack Playwright passed **2/2**; full desktop Vitest passed **388/388**;
  desktop production build passed with the existing Vite chunk-size warning;
  full desktop Playwright passed **10/10** with `--workers=1`; DEC-025
  dirty-tree sweep passed and wrote
  `validation/evidence/sweeps/SWEEP_20260616T031013Z_b431a1676620-dirty.json`.
- Evidence: DEL-07-03 primary run record and DEL-06-01 supporting run record
  `WORKING_ITEMS_RUN_2026-06-16_TP-C2-ASTTEXTVIEW-001.md`; completion
  plan/log updated.
- Boundary review: frontend display-only rule-pack editor evidence; no schema,
  evaluator, parser, writable text syntax, protected content, private data,
  network/telemetry path, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-181 viewport draft length-unit controls (`TP-UNITS-B2-VIEWPORTDRAFTUNITS-001`, 2026-06-16)

- Phase B2/B3 units slice: the viewport create-node and straight-pipe draft
  forms now expose length-unit selectors with visible unit-basis text before
  queueing structured operation intents.
- Viewport behavior: browser preview records the explicit model-metadata
  fallback (`browser preview uses model metadata for viewport length units`);
  Tauri-capable runs load the DEC-018 unit catalog and filter accepted length
  entries. Node draft coordinates carry the selected coordinate length unit;
  pipe geometry drafts carry the selected pipe length unit.
- Operation-seam behavior: `create_node` now normalizes compatible DEC-018
  length units back to the stored `project.units.length` scalar basis, while
  `connect_pipe_run` validates compatible OD/wall units and checks wall
  thickness after conversion without erasing the entered section unit.
- Browser/e2e proof: the R2 Playwright smoke checks the viewport unit status,
  node selector value, node unit-basis text, pipe selector value, and pipe
  unit-basis text in both desktop and compact viewports. In-app Browser
  verification observed the same model-metadata fallback and confirmed a
  filled node draft enabled queueing with unit `m`.
- Validation: operation-applier focused explicit tests passed **28/28**;
  full operation-applier cargo suite passed **60 unit** tests plus canonical
  hash and contract corpus checks; focused App Vitest passed **54/54**; full
  desktop Vitest passed **388/388**; desktop production build passed with the
  existing Vite chunk-size warning; focused R2 Playwright smoke passed **2/2**;
  full desktop Playwright passed **10/10** with `--workers=1`; five-surface
  DEC-025 dirty-tree sweep passed and wrote
  `validation/evidence/sweeps/SWEEP_20260616T033649Z_40c8d2530ca8-dirty.json`.
- Evidence: DEL-07-01 primary run record plus DEL-16-02 and DEL-02-02
  supporting run records
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-VIEWPORTDRAFTUNITS-001.md`;
  completion plan/log updated.
- Boundary review: viewport draft and operation-seam unit evidence only; no
  project-unit mutation, hidden unit fallback, release-readiness claim,
  professional approval, certification, sealing, authentication, code-
  compliance claim, protected content, private project data, network path, or
  telemetry path changed.

## TP-MAC-182 rule-check run-time unit binding controls (`TP-UNITS-B2-RULECHECKRUNUNITS-001`, 2026-06-16)

- Phase B2/B3 units slice on the C4 run-check GUI: runtime
  user-supplied values and value-slot limits in `RuleCheckRunPanel` now use
  catalog-aware unit controls for their binding units.
- Desktop/Tauri behavior: the panel loads the DEC-018 catalog only when a
  loaded rule pack has runtime value bindings, filters accepted unit options by
  the binding dimension, preserves out-of-catalog stored units as current
  values, and shows per-binding unit-basis text.
- Browser behavior: browser preview keeps the existing manual unit text fields
  for the same `data-testid` controls and reports
  `browser preview keeps run-check unit refs as stored manual text; no fallback
  catalog synthesized`.
- Browser/e2e proof: the run-rule-checks Playwright smoke asserts the browser
  fallback status, `demo_unit` user-value unit, `ratio` slot unit, and both
  model-metadata unit-basis witnesses in desktop and compact viewports.
  In-app Browser verification observed the same status and text-input tags.
- Validation: focused `RuleCheckRunPanel` Vitest passed **18/18**; full
  desktop Vitest passed **389/389**; desktop production build passed with the
  existing Vite chunk-size warning; focused run-check Playwright passed
  **2/2**; full desktop Playwright passed **10/10** with `--workers=1`;
  five-surface DEC-025 dirty-tree sweep passed and wrote
  `validation/evidence/sweeps/SWEEP_20260616T034816Z_9be2d805ab17-dirty.json`.
- Evidence: DEL-07-04 primary run record plus DEL-06-02 and DEL-02-02
  supporting run records
  `WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-RULECHECKRUNUNITS-001.md`;
  completion plan/log updated.
- Boundary review: GUI run-time binding unit controls only; no schema,
  evaluator, rule-pack parser, writable expression text syntax, protected
  content, private data, network/telemetry path, release-readiness claim,
  professional approval, certification, sealing, authentication, or code-
  compliance claim changed.

## TP-MAC-183 R3 exit readiness plan revision (`TP-R3EXIT-PLAN-001`, 2026-06-16)

- Establishes completion-plan Phase C5: R3 exit readiness and
  authoring-usability closure, inserted after landed C4 and before Phase D.
- Updates `plans/PLAN_2026-06-10_prd_completion.md`, `docs/PLAN.md`,
  `execution/_Coordination/_COORDINATION.md`, and
  `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` so ordinary R3 work
  continues through C5 before Phase D unless the human explicitly overrides.
- C5 records the guided-workbench direction: keep the persistent model tree,
  viewport, and property inspector, but make the lower workspace task-guided
  and move evidence/debug-heavy surfaces behind details or `Audit &
  Boundaries`.
- C1-C4 remain implementation evidence for PRD §22.4. This row does not begin
  or pass an R3 exit review; F-4 and the authoring-journey usability finding
  remain open until packaged human evidence closes them.
- Evidence: DEL-07-06 run record
  `WORKING_ITEMS_RUN_2026-06-16_TP-R3EXIT-PLAN-001_TP-R3UX-DISCOVERY-001.md`;
  completion log updated.
- Boundary review: plan/evidence routing only; no app behavior, schema,
  solver, rule-pack grammar, parser, persistence, package, lifecycle,
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim.

## TP-MAC-184 R3 authoring-usability failure baseline (`TP-R3UX-DISCOVERY-001`, 2026-06-16)

- Records the C5 starting baseline for the guided-workbench redesign: TP-MAC-141
  attempts 1-3, the human-provided 2026-06-16 screenshot evaluation, and the
  finding that the current shell is credible as an internal technical preview
  but still overloaded for a working engineer user.
- Baseline screenshots captured at 1440x920 and 1280x800:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_run_records/assets/TP-R3UX-DISCOVERY-001_1440x920.png`
  and
  `.../TP-R3UX-DISCOVERY-001_1280x800.png`.
- Viewport probe artifact:
  `.../_run_records/assets/TP-R3UX-DISCOVERY-001_viewport_probe.json`.
  Results: zero console errors, no horizontal page overflow, and no
  horizontally clipped visible primary controls at either viewport.
- Dead-control audit rerun: `npm test --workspace apps/desktop --
  App.deadControls` passed 1 file / 1 test.
- Baseline conclusion: the shell is structurally stable after TP-MAC-146, but
  the default working surface still makes the next action unclear and exposes
  developer/evidence panels too prominently. C5.3 (`TP-R3UX-SHELL-001`) is the
  next implementation tranche.
- Boundary review: discovery/evidence only; no product behavior change, no
  lifecycle state change, and no release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## TP-MAC-185 guided workbench shell (`TP-R3UX-SHELL-001`, 2026-06-16)

- Adds the first C5 guided-workbench shell: the persistent model tree,
  3D centerline viewport, and property inspector remain visible, while the
  default lower workspace now starts with a task rail and current-step panel.
- The rail covers the ordinary journeys: model edits, loads, private
  libraries, rule packs, solve/check, results, report, and save/reopen. New
  stable IDs include `guided-workbench`, `journey-step-*`,
  `journey-current-step`, `review-apply-drawer`,
  `review-apply-drawer-toggle`, and `r3-exit-journey-status`.
- Evidence-heavy surfaces remain available but are no longer the default
  work surface. `Editor Contract`, operation diff internals, operation ledger,
  and agent proposal context are mounted behind the Review evidence detail
  drawer in Operation Apply; existing panel test IDs are preserved after the
  drawer opens.
- In-app browser evidence:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_run_records/assets/TP-R3UX-SHELL-001_1440x920_iab.png`,
  `.../TP-R3UX-SHELL-001_1280x800_iab.png`, and
  `.../TP-R3UX-SHELL-001_iab_probe.json`. Probe results: no horizontal page
  overflow and no clipped guided-workbench primary controls at either
  1440x920 or 1280x800.
- Validation completed before closeout: focused guided-workbench Vitest,
  dead-control audit, focused Playwright guided-workbench e2e at both
  configured viewports, full desktop Vitest, and desktop production build.
- Boundary review: frontend shell/usability only; no schema, evaluator
  grammar, solver, persistence, backend API, rule-pack checksum, lifecycle,
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance change. F-4 and the authoring-usability
  finding remain open pending C5 packaged human journey evidence.

## TP-MAC-186 A12 guided authoring journey (`TP-R3UX-A12FLOW-001`, 2026-06-16)

- Adds an A12-specific guided authoring panel inside the guided workbench for
  blank-model authoring. The checklist covers blank document, nodes, material,
  section, pipe, support, load case, primitive load, combination, solve,
  report, and save/reopen.
- The checklist is derived from the current model/session state, not from
  hard-coded A12 fixture values. It reports the next unmet step, step counts,
  and current queue status; queued creation operations name the created object
  when the operation payload carries an `id`.
- Adds an inline `Apply queued` affordance for A12. It calls the existing
  structured operation apply handler, records the normal operation receipt,
  preserves result-reset/undo behavior, and leaves Operation Apply available as
  the receipt and audit surface.
- A12 step buttons now maintain visible selected-step state when several steps
  route to the same work section, preventing no-op controls and making
  node/material/section/pipe/support/load/solve/report/save-reopen navigation
  explicit.
- In-app browser evidence:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_run_records/assets/TP-R3UX-A12FLOW-001_1440x920_iab_blank.png`,
  `.../TP-R3UX-A12FLOW-001_1280x800_iab_queued.png`,
  `.../TP-R3UX-A12FLOW-001_1280x800_iab_applied.png`, and
  `.../TP-R3UX-A12FLOW-001_iab_probe.json`. Probe results: no horizontal
  page overflow; A12 primary controls and compact inline apply remain inside
  the viewport at 1440x920 and 1280x800.
- Validation completed before closeout: focused guided-workbench Vitest,
  permanent dead-control audit, focused A12 Playwright e2e at both configured
  viewports, full desktop Vitest **18 files / 386 tests**, desktop production
  build, and full Playwright e2e **12/12**.
- Boundary review: frontend guided-authoring usability only; no rule-pack
  schema, evaluator grammar, solver, persistence, backend API, project-store
  semantics, rule-pack checksum, lifecycle, release-readiness, professional
  approval, certification, sealing, authentication, or code-compliance change.
  F-4 and the authoring-usability finding remain open until packaged human
  successor journey evidence is recorded.

## TP-MAC-187 R3 guided rule-pack/private-library flow (`TP-R3UX-R3FLOW-001`, 2026-06-16)

- Adds an R3-specific guided mode beside the A12 mode in the guided workbench.
  The path routes private library import, private non-code rule-pack draft and
  validation, checksum/save request, mechanics solve, rule-check binding
  review, and rule-check run request through existing panels.
- The implementation is UI-only session state. `LibraryManagerPanel`,
  `RulePackManagerPanel`, and `RuleCheckRunPanel` report visible user actions
  to the guided workbench; no rule-pack schema, evaluator grammar, solver,
  persistence, backend API, checksum, or store semantics changed.
- New stable IDs include `guided-journey-tab-r3`, `r3-guided-flow`,
  `r3-flow-next-action`, `r3-flow-progress`, `r3-flow-step-*`,
  `r3-flow-status`, `r3-flow-next-action-button`, and
  `r3-flow-missing-input-blocker`.
- The browser-preview journey intentionally reaches `6/6` only as guided action
  coverage. The visible blocker remains explicit: pass/fail stays blocked until
  the desktop checker returns complete inputs. This preserves PRD 22.4 missing
  input behavior and makes no code-compliance claim.
- In-app browser evidence:
  `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_run_records/assets/TP-R3UX-R3FLOW-001_1440x920_iab_r3_start.png`,
  `.../TP-R3UX-R3FLOW-001_1280x800_iab_r3_run.png`, and
  `.../TP-R3UX-R3FLOW-001_iab_probe.json`. Probe results: no horizontal page
  overflow, no clipped R3 primary controls, `r3Progress=6/6`, and blocker text
  `browser preview keeps pass/fail blocked until the desktop checker returns
  complete inputs`.
- Validation completed before closeout: full desktop Vitest **18 files / 386
  tests**, permanent dead-control audit, desktop production build, full
  Playwright e2e **14/14** across 1440x920 and 1280x800, plus in-app browser
  visual verification.
- Boundary review: frontend guided-workbench usability only; no private data
  committed, no protected content, no network/telemetry posture change, no
  lifecycle state change, and no release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim. F-4 and the
  authoring-usability finding remain open until packaged human successor
  journey evidence is recorded.

## TP-MAC-188 packaged A12 + R3 successor journey kit - PREPARED manual checklist, human execution pending (`TP-R3UX-PACKAGEKIT-001`, 2026-06-17)

**Status: PREPARED, not performed.** This is the C5.6 successor kit to
TP-MAC-141. It builds a fresh packaged `.app`, boot-checks the bundle, and
defines the human-run checklist for C5.7. This section does not close F-4, the
A3 authoring-usability finding, R3 exit review, lifecycle issuance, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance. Only a later recorded human packaged pass/fail entry can close
or redirect those findings.

### Binary provenance

- Built from commit `c013b49b8` with:
  - `npm ci` at the workspace root because this fresh worktree had no
    `node_modules`;
  - `cd apps/desktop && npm run tauri -- build --bundles app`.
- Build evidence: WASM engine generation succeeded, Vite production build
  succeeded with the existing chunk-size warning, Tauri release build finished
  in 1m16s, and Tauri produced one macOS app bundle.
- Bundle path:
  `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
- Bundle size: `12M`.
- Boot check: launched the bundled executable for 8 seconds, observed process
  `openpipestress-desktop` plus WebKit GPU, Networking, and WebContent child
  processes, recorded `stdout_bytes=0` and `stderr_bytes=0`, then terminated
  the process cleanly.
- Validation after evidence edits: `npm test --workspace apps/desktop` passed
  18 files / 390 tests.

### Human successor checklist for C5.7

Run this checklist against the bundle path above or a freshly rebuilt bundle
from the then-current reviewed code state. Record pass/fail by step in the
human execution record below.

1. Launch the `.app`. Expect the technical preview shell, the guided
   workbench, local project status with `network=false` and `telemetry=false`,
   and no release/professional/code-compliance claim.
2. Open the A12 guided authoring tab. Expect the A12 checklist, next-action
   panel, current queue status, and inline `Apply queued` affordance.
3. Click `New blank`. Expect a blank local model document without fixture
   entities or external file copies.
4. Follow the A12 guided steps to create the two nodes, material, section,
   straight pipe, support, load case, primitive load, and combination using the
   invented rehearsal values from
   `fixtures/product_preview/r2_from_blank_rehearsal.json`.
5. Use the guided inline apply action or the Operation Apply receipt surface
   after queued creation steps. Expect applied receipts to remain local and
   `professional_approval=false`.
6. Run mechanics preview. Expect completion with `MECHANICS_SOLVED` and
   result rows visible in the results surface.
7. Render the report. Expect a report preview and a canonical HTML SHA-256.
   Record the hash below.
8. Save locally, list local projects, reopen the saved project, and re-run
   mechanics preview. Expect the authored model and solve behavior to survive
   the local save/reopen path.
9. Switch to the R3 guided rule-pack/private-library tab. Expect the R3 guide,
   progress status, and next-action panel.
10. Load the invented private library template, validate it, and request local
    save. Expect the guide to record the visible action without committing
    private user data to the repository.
11. Create the private non-code rule-pack draft, validate it, calculate its
    checksum, and request local save. The structured composer remains the
    source of truth; writable expression text parsing is still deferred by
    `DEC-037`.
12. Run the R3 rule-check path after mechanics solve. Expect the GUI to show
    missing-input blockers when required values are absent. Pass/fail must stay
    blocked until the checker has complete user-supplied inputs; no software
    code-compliance status may appear.
13. Quit. Confirm no network, daemon, telemetry, repository-default private
    data write, protected standards content, release-readiness claim,
    professional approval, certification, sealing, authentication, or
    code-compliance claim occurred.

### Human execution record (fill in during C5.7)

```
performed: NO (prepared package kit only; F-4 and A3 authoring-usability
  finding remain open until a human completes or fails this successor journey)
date:
performed_by:
binary commit:
bundle path:
outcome (PASS / FAIL per step, with any deviations):
report sha256:
notes:
```

- Boundary review: invented preview and rehearsal data only; user-created
  models and private libraries/rule packs stay in local app storage and are
  never committed by default. This kit prepares human execution only and makes
  no lifecycle, release-readiness, professional approval, certification,
  sealing, authentication, or code-compliance claim.

## TP-MAC-190 R3 exit-chain verification packet - PREPARED (`TP-R3VERIFY-001`, 2026-06-20)

- Scope: C5.8 evidence assembly after the human accepted the replacement C5
  closure criterion. This packet prepares the R3 exit review evidence; it does
  not pass the R3 exit review or advance the stage.
- Human status: F-4/A3 are closed for the replacement C5 criterion by the
  2026-06-20 human ruling. The stale TP-MAC-189 packaged re-pass path remains
  bypassed, not retroactively passed.
- Evidence package: `plans/VERIFICATION_2026-06-20_r3_exit_chain.md`.
- Run record: DEL-07-06
  `_run_records/WORKING_ITEMS_RUN_2026-06-20_TP-R3VERIFY-001.md`.
- Evidence basis: C1-C4 SMOKE TP-MAC-147..167 and TP-MAC-180; C5 SMOKE
  TP-MAC-183..188 and TP-MAC-272..277; latest app-code validation/package
  evidence at commit `3abf5d9bb`, including DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260621T014041Z_9586deb6c15e.json`.
- Boundary review: evidence and coordination only. No app code, schema, solver,
  evaluator, persistence, package artifact, private-data behavior, network or
  telemetry feature, live agent runtime, external SDK/harness consumption,
  autonomous model mutation, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-191 support linear-stiffness unit authoring - PASSED (`TP-UNITS-BTAIL-SUPPORTUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail support unit I/O slice while C5.7 remains
  human-execution gated. The Property Inspector support creation panel now
  exposes a linear-stiffness unit selector and value field. Blank stiffness
  preserves the prior dimensionless support-create behavior; entered stiffness
  queues `properties.linear_stiffness` with `{value, unit}` and
  `dimension=linear_stiffness`.
- Apply path: `core/model_operations/operation_applier` accepts support
  creation with either dimensionless restraint metadata or a positive
  DEC-018-compatible `linear_stiffness` quantity. The browser-mode WASM engine
  was regenerated with `npm run -w apps/desktop build:wasm` before UI
  validation.
- Readback: selected support properties display the preserved
  `Linear stiffness` value/unit after the local session apply route accepts the
  structured operation.
- Validation:
  - `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
  - `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed 61 unit tests plus canonical-hash and contract-corpus tests.
  - `npm run -w apps/desktop test -- --run src/App.test.tsx` passed 55/55 tests after WASM regeneration.
  - `npm test --workspace apps/desktop` passed 18/18 files and 390/390 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.
- Evidence: DEL-07-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SUPPORTUNITS-001.md`; DEL-02-02
  supporting run record with the same id; completion log entry.
- Boundary: invented/local preview operation authoring only. No protected
  standards content, private project data, network/telemetry path, durable
  persistence semantics, solver/release tolerance policy, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-192 local FEA handoff unit witnesses - PASSED (`TP-UNITS-BTAIL-LOCALFEAUNITWITNESS-001`, 2026-06-17)

- Scope: bounded Phase B-tail target-format/unit-witness slice while C5.7
  remains human-execution gated. The local FEA handoff package now emits
  source value/unit/dimension preservation witnesses for transfer-basis
  displacement, force, and moment result refs.
- Packet contract: `handoff_package.unit_witness_policy` is
  `preserve_source_result_units_for_referenced_transfer_results`, and
  `handoff_package.unit_preservation_witnesses[]` records source result ref,
  source field path, source quantity, target transfer-basis path,
  unit-system ref, and `conversion_performed=false`.
- UI check: `data-testid="local-fea-unit-witnesses"` displays
  `count=3`, `policy=preserve_source_result_units`, and `conversion=false`
  for the invented preview fixture after mechanics preview.
- Schema: `schemas/local_fea_handoff.schema.yaml` now strictly defines
  `UnitPreservationWitness` and `UnitPreservationQuantity`.
- Validation:
  - `python3 -m pytest tests/test_local_fea_handoff_contract.py` passed.
  - `npm run -w apps/desktop test -- --run src/features/local-fea-handoff/LocalFeaHandoffPanel.test.tsx src/App.test.tsx` passed 58/58 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 391/391 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.
- Evidence: DEL-10-03 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOCALFEAUNITWITNESS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: target-neutral handoff metadata only. No mesh, external solver,
  concrete local FEA exchange format, target solver adapter, protected
  standards content, private project payload, network/telemetry path,
  lifecycle state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-193 result export unit witnesses - PASSED (`TP-UNITS-BTAIL-RESULTEXPORTUNITWITNESS-001`, 2026-06-17)

- Scope: bounded Phase B-tail target-format/unit-witness slice while C5.7
  remains human-execution gated. The schema-first result export preview now
  emits a deterministic unit-preservation witness for each exported result
  row in the invented mechanics fixture.
- Packet contract: `result_envelope.unit_witness_policy` is
  `preserve_source_result_value_unit_and_dimension_per_exported_result_row`,
  and `result_envelope.unit_preservation_witnesses[]` records source result
  ref, source field path, source value/unit/dimension, target result row path,
  target value/unit/dimension, unit-system ref, and
  `conversion_performed=false`.
- UI check: `data-testid="result-export-unit-witnesses"` displays
  `count=737` and `conversion=false` for the invented preview fixture after
  mechanics preview.
- Schema: `schemas/results.schema.yaml` now defines optional
  `UnitPreservationWitness` and `UnitPreservationQuantity` vocabulary for
  result-envelope unit preservation without changing existing accepted
  fixtures or trace-chain ownership.
- Validation:
  - `python3 tests/test_results_schema.py` passed.
  - `npm --prefix apps/desktop test -- App.test.tsx` passed 55/55 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 391/391 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
- Evidence: DEL-08-04 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RESULTEXPORTUNITWITNESS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: result-envelope unit metadata only. No unit conversion,
  tolerance policy, solver behavior, public transport commitment, trace-chain
  ownership change, protected standards content, private project payload,
  lifecycle state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-194 comparison mixed-unit tolerance corpus - PASSED (`TP-UNITS-BTAIL-COMPTOLCORPUS-001`, 2026-06-17)

- Scope: bounded Phase B-tail DEC-026 corpus slice while C5.7 remains
  human-execution gated. The analysis-run comparison engine now recognizes
  caller-supplied relative+absolute tolerance pairs when a governed profile
  supplies both values, while keeping the existing scalar tolerance path for
  older fixtures.
- Contract: `schemas/comparison_tolerance.schema.json` now defines optional
  `relative_tolerance_value`, `absolute_tolerance_value`, and
  `tolerance_pair_policy=relative_plus_absolute_floor` on tolerance rules.
  Numeric pair values are still restricted to externally governed or
  project-specific review status paths. No schema default value was added.
- Corpus evidence: `tests/test_analysis_run_comparison.py` covers a
  DEC-026-style mixed-unit profile with stress normalized from `kPa` to `Pa`
  and force normalized from `lbf` to `N`; the stress case is classified by the
  relative allowance, and the near-zero force case is classified by the
  absolute floor. Removing a required conversion still blocks with
  `ARC-UNIT-CONVERSION-UNSUPPORTED`.
- Validation:
  - `python3 tests/test_analysis_run_comparison.py` passed.
  - `python3 tests/test_comparison_contracts.py` passed.
  - `python3 -m pytest tests/test_analysis_run_comparison.py tests/test_comparison_contracts.py tests/test_design_authoring_comparison_workspace.py tests/test_state_comparison_handoff_report_sections.py -q` passed 23/23 tests.
  - `python3 -m pytest -q tests` passed 360/360 tests.
  - `git diff --check` passed.
- Evidence: DEL-14-04 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-COMPTOLCORPUS-001.md`;
  DEL-14-05 and DEL-02-02 supporting run records with the same id;
  completion log entry.
- Boundary: governed-profile comparison evidence only. No default tolerance,
  release threshold, solver convergence policy, external validation decision,
  protected standards content, private project payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-195 component library field unit helper - PASSED (`TP-UNITS-BTAIL-COMPLIBFIELDUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail app unit-entry slice while C5.7 remains
  human-execution gated. The Private Library Manager now exposes a component
  field unit helper for component-library drafts, so a user can add one
  private component `fields[]` quantity with explicit magnitude, unit, and
  dimension metadata before running the existing local-only import validation.
- Unit behavior: browser preview does not synthesize a catalog and keeps the
  stored `N/m` field unit as a single explicit option with the
  desktop-only catalog diagnostic. In Tauri/desktop mode, the helper uses the
  DEC-018 unit catalog and filters `linear_stiffness` options to compatible
  force-per-length units such as `N/m` and `lbf/in`, excluding length units.
- Draft payload: applying the helper writes a private `component_records[0]`
  field with `field_kind=linear_stiffness`,
  `public_repository_value_policy=private_user_supplied_only`, and
  `value={magnitude, unit, dimension=linear_stiffness,
  value_status=private_user_supplied}`. Validation and storage are unchanged
  and still route through the desktop-only local library backend.
- Validation:
  - `npm test --workspace apps/desktop -- LibraryManagerPanel.test.tsx`
    passed 11/11 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 393/393 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- -g "library manager loads"`
    passed 2/2 Playwright tests.
- Evidence: DEL-07-03 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-COMPLIBFIELDUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: component-library draft authoring only. No component mechanics,
  code-specific SIF/flexibility data, public component catalog, library
  validation rule, local-store policy, protected standards content, private
  project payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-196 export adapter SDK unit policy evidence - PASSED (`TP-UNITS-BTAIL-ADAPTERSDKUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail target-format/unit-evidence slice while C5.7
  remains human-execution gated. The DEL-17-09 export adapter SDK admission
  package now includes a structured `unit_policy_evidence` payload and a
  `unit_policy_evidence.json` manifest member.
- Unit behavior: the package records `unit-system:dec-018-si-dual-display`,
  `entered_units_preserved`, source/result/target unit disclosure,
  `conversion_policy=no_adapter_sdk_conversion_performed`,
  `conversion_performed=false`, target refs, and a witness policy for candidate
  targets without claiming target-writer conversion.
- UI check: `data-testid="export-adapter-sdk-units"` displays
  `conversion=false`, `witnesses=5`, and the candidate-target no-claim witness
  policy in the desktop preview; the downloaded registry JSON carries the same
  `unit_policy_evidence` fields.
- Validation:
  - `python3 tests/test_export_adapter_sdk.py` passed.
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 393/393 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
    passed 2/2 Playwright tests.
- Evidence: DEL-17-09 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-ADAPTERSDKUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: adapter-admission metadata only. No runtime loader, public endpoint,
  target-specific writer, conversion API, target support claim, compatibility
  claim, external execution, protected standards content, private project
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-203 adapter framework unit policy evidence - PASSED (`TP-UNITS-BTAIL-ADAPTERFRAMEWORKUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail adapter-framework/unit-evidence slice while C5.7
  remains human-execution gated. The DEL-10-02 format-neutral adapter
  framework preview now includes structured `unit_policy_evidence` in its
  exported JSON packet.
- Unit behavior: the packet records `unit-system:dec-018-si-dual-display`,
  `entered_units_preserved`, source/result unit disclosure,
  `conversion_policy=adapter_framework_declares_unit_validation_no_format_conversion`,
  `conversion_performed=false`, and a witness policy for framework-level unit
  validation without claiming target-writer conversion.
- UI check: `data-testid="adapter-framework-units"` displays
  `conversion=false`, `witnesses=1`, and
  `unit_validation_required_before_adapter_payload_exchange` in the desktop
  preview; the downloaded adapter JSON carries the same `unit_policy_evidence`
  fields.
- Validation:
  - `python3 tests/test_adapter_framework_contract.py` passed.
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
    passed 2/2 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-10-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-ADAPTERFRAMEWORKUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: adapter-framework metadata only. No concrete external format,
  target-specific writer, conversion API, runtime loader, public endpoint,
  target support claim, compatibility claim, external execution, protected
  standards content, private project payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-204 export review unit policy inventory - PASSED (`TP-UNITS-BTAIL-EXPORTREVIEWUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail export-review/unit-evidence inventory slice
  while C5.7 remains human-execution gated. The DEL-12-02 Export Safety Review
  manifest now carries top-level `unit_policy_summary` evidence for local
  export records.
- Unit behavior: the manifest records `unit-system:dec-018-si-dual-display`,
  `entered_units_preserved`,
  `conversion_policy=export_review_manifest_inventory_only_no_target_conversion`,
  `conversion_performed=false`, and a 27-row `unit_evidence_matrix`.
  Fourteen unit-bearing export records are inventoried as covered when their
  source export packet is available; metadata-only review records are marked
  `not_unit_bearing_metadata_or_boundary_review`.
- UI check: `data-testid="export-review-units"` displays
  `unit-system:dec-018-si-dual-display`, `covered=14/14`, and
  `conversion=false` after the mechanics preview; the downloaded review JSON
  carries the same unit-policy summary and matrix.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
  - `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
    passed 2/2 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-EXPORTREVIEWUNITS-001.md`;
  DEL-02-02 and DEL-08-04 supporting run records with the same id; completion
  log entry.
- Boundary: export-review metadata inventory only. No target-specific writer,
  conversion API, public transport commitment, runtime redaction rule change,
  protected standards content, private project payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-205 load-case primitive unit validation evidence - PASSED (`TP-UNITS-BTAIL-LOADCASEUNITVALID-001`, 2026-06-17)

- Scope: bounded Phase B-tail load-case/unit-validation slice while C5.7
  remains human-execution gated. The desktop Load Cases manager now records
  unit-dimension validation status on unit-bearing primitive-load create and
  magnitude-edit operation intents.
- Unit behavior: primitive load create/edit intents no longer emit
  `unit_validation=not_run` for unit-bearing quantities. Browser preview
  records
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`;
  desktop/Tauri catalog routes can record `dec018_catalog_dimension_match` for
  accepted DEC-018 unit/dimension matches or explicit mismatch/unreviewed
  statuses.
- UI check: `data-testid="load-manager-create-primitive-preview"` and
  `data-testid="load-manager-edit-preview"` include `unit_validation=...`
  alongside the existing explicit unit and dimension. The R2 smoke checks the
  create-preview status; App Vitest checks create and edit paths.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
    passed 2/2 Playwright tests after the test assertion was moved to the
    exercised create-preview path.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-05-01 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOADCASEUNITVALID-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: load-case operation-intent metadata only. No DEC-018 catalog
  constant change, schema dimension enum change, unit conversion API change,
  solver behavior, operation application semantics, protected standards
  content, private project payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-206 property-inspector unit validation evidence - PASSED (`TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001`, 2026-06-17)

- Scope: bounded Phase B-tail Property Inspector/unit-validation slice while
  C5.7 remains human-execution gated. The desktop Property Inspector now
  records unit-dimension validation status on unit-bearing modify intents and
  explicit create-section/create-material/create-support intents.
- Unit behavior: unit-bearing Property Inspector intents no longer emit
  `unit_validation=not_run`. Browser preview records
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`
  because the reviewed DEC-018 catalog command is desktop-only; desktop/Tauri
  routes can record `dec018_catalog_dimension_match`, mismatch, loading, or
  unreviewed statuses from the catalog basis.
- UI check: `data-testid="editor-intent-validation"` shows the unit-validation
  status in the existing operation preview for node coordinate edits, load
  primitive magnitude edits, section creation, material creation, and support
  creation. Playwright smoke checks the node and load edit preview statuses;
  App Vitest checks the edit and create-intent statuses.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-07-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001.md`;
  DEL-02-02 and DEL-16-02 supporting run records with the same id; completion
  log entry.
- Boundary: Property Inspector operation-intent metadata only. No DEC-018
  catalog constant change, schema dimension enum change, unit conversion API
  change, solver behavior, operation application semantics, accepted
  model-state mutation, protected standards content, private project payload,
  lifecycle state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-207 rule-pack unit policy evidence - PASSED (`TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001`, 2026-06-17)

- Scope: bounded Phase B-tail rule-pack editor/unit-policy slice while C5.7
  remains human-execution gated. The Rule-Pack Manager now shows visible unit
  policy and unit-dimension validation status for declaration `quantity_intent`
  unit refs and expression literal/table unit refs.
- Unit behavior: browser preview keeps the existing manual stored-unit text
  route and explicitly reports `catalog_route=browser_preview_manual_entry`
  with `conversion=false`. Desktop/Tauri catalog routes continue using the
  reviewed DEC-018 catalog selectors and can report accepted dimension matches
  or explicit mismatch/unreviewed/loading statuses.
- UI check: `data-testid="rule-pack-declarations-unit-policy"` summarizes
  required-input and value-slot unit refs. `data-testid="rule-pack-expression-unit-policy"`
  summarizes literal and table unit refs in the active formula. The R2/R3
  Playwright smoke checks required-input, literal, and table result statuses.
- Validation:
  - `npm test --workspace apps/desktop -- unitCatalogService.test.ts DeclarationsEditor.test.tsx ExpressionComposer.test.tsx RulePackManagerPanel.test.tsx` passed 67/67 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-07-03 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001.md`;
  DEL-02-02, DEL-06-01, and DEL-06-02 supporting run records with the same id;
  completion log entry.
- Boundary: rule-pack editor unit-policy metadata only. No DEC-018 catalog
  constant change, schema dimension enum change, rule-pack schema change,
  evaluator normalization change, parser/text-syntax change, backend
  validation/persistence behavior change, protected standards content,
  private value payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-197 material library property unit helper - PASSED (`TP-UNITS-BTAIL-MATLIBFIELDUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail app unit-entry slice while C5.7 remains
  human-execution gated. The Private Library Manager now exposes a material
  property unit helper for material-library drafts, so a user can add one
  private material `properties[]` quantity with explicit magnitude, unit ref,
  dimension id, and missing-unit policy before running the existing local-only
  import validation.
- Unit behavior: browser preview does not synthesize a catalog and keeps the
  schema-native default unit ref for each property kind as a single explicit
  option with the desktop-only catalog diagnostic. In Tauri/desktop mode, the
  helper uses the DEC-018 unit catalog and filters material-property choices by
  compatible dimensions; for example, `elastic_modulus` offers `Pa`/`MPa` and
  excludes length units.
- Draft payload: applying the helper writes a private
  `material_records[0].properties[]` entry with
  `property_kind=elastic_modulus`, `value_status=private_user_supplied`,
  `required_for=mechanics_solve`, and
  `value={magnitude, unit_ref, dimension_id=stress,
  quantity_kind=unit_bearing, unit_required=true,
  missing_unit_behavior=diagnostic_blocking}`. Validation and storage are
  unchanged and still route through the desktop-only local library backend.
- Validation:
  - `npm test --workspace apps/desktop -- LibraryManagerPanel.test.tsx`
    passed 13/13 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 395/395 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- -g "library manager loads"`
    passed 2/2 Playwright tests.
- Evidence: DEL-07-03 and DEL-03-01 primary run records
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-MATLIBFIELDUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: material-library draft authoring only. No material engineering
  allowables, public material values, validation/storage rule change, schema
  enum change, DEC-018 catalog constant change, protected standards content,
  private project payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-198 section library quantity unit helper - PASSED (`TP-UNITS-BTAIL-SECLIBQTYUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail app unit-entry slice while C5.7 remains
  human-execution gated. The Private Library Manager now exposes a section
  quantity unit helper for section-library drafts, so a user can add one
  private section dimension or property quantity with explicit magnitude, unit,
  dimension, provenance, and review status before running the existing
  local-only import validation.
- Unit behavior: browser preview does not synthesize a catalog and keeps the
  selected schema-native default unit as a single explicit option with the
  desktop-only catalog diagnostic. In Tauri/desktop mode, the helper uses the
  DEC-018 unit catalog and filters section quantity choices by compatible
  dimensions; for example, `cross_section_area` offers `m^2`/`in^2` and
  excludes length units.
- Draft payload: applying the helper can write either a private
  `section_records[0].dimensions[]` slot such as
  `dimension_kind=outside_diameter` or a private
  `section_records[0].properties[]` slot such as
  `property_kind=cross_section_area`. Values are recorded as
  `value_status=private_user_supplied`; property slots keep
  `calculation_status=not_calculated` so no reviewed/calculated engineering
  claim is implied. Validation and storage are unchanged and still route
  through the desktop-only local library backend.
- Validation:
  - `npm test --workspace apps/desktop -- LibraryManagerPanel.test.tsx`
    passed 15/15 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- -g "library manager loads"`
    passed 2/2 Playwright tests.
- Evidence: DEL-07-03 and DEL-03-02 primary run records
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SECLIBQTYUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: section-library draft authoring only. No section-property
  calculator, public section values, validation/storage rule change, schema
  enum change, DEC-018 catalog constant change, protected standards content,
  private project payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-199 handoff package unit witnesses - PASSED (`TP-UNITS-BTAIL-HANDOFFUNITWITNESS-001`, 2026-06-17)

- Scope: bounded Phase B-tail handoff/unit-evidence slice while C5.7 remains
  human-execution gated. The desktop Handoff Package panel now emits explicit
  DEC-018 unit-system disclosure and per-result unit-preservation witnesses in
  the local review handoff package.
- Packet contract: `unit_system_disclosure` records
  `unit-system:dec-018-si-dual-display`, entered-unit storage convention,
  source model units, result units, target handoff units, and
  `conversion_performed=false`. `unit_witness_policy` is
  `preserve_source_result_value_unit_and_dimension_per_handoff_result_ref`,
  and `unit_preservation_witnesses[]` preserves finite result-row value, unit,
  and dimension metadata by reference.
- UI check: `data-testid="handoff-units"` displays the source/result unit
  disclosure and `data-testid="handoff-unit-witnesses"` displays `count=737`
  with `conversion=false` for the invented preview fixture after mechanics
  preview.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
    passed 2/2 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-15-01 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-HANDOFFUNITWITNESS-001.md`;
  DEL-02-02 and DEL-17-03 supporting run records with the same id; completion
  log entry.
- Boundary: handoff unit metadata only. No target-specific mapping, external
  prover execution, downstream compatibility claim, D-21 scope promotion,
  schema contract change, unit conversion API, protected standards content,
  private payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-200 headless runner unit witnesses - PASSED (`TP-UNITS-BTAIL-HEADLESSRUNNERUNITWITNESS-001`, 2026-06-17)

- Scope: bounded Phase B-tail headless-runner/unit-evidence slice while C5.7
  remains human-execution gated. The desktop Headless Runner envelope now
  emits explicit DEC-018 unit-system disclosure and per-result
  unit-preservation witnesses for the schema-first local result-handoff
  preview.
- Packet contract: `result.unit_system_disclosure` records
  `unit-system:dec-018-si-dual-display`, entered-unit storage convention,
  source model units, result units, and `conversion_performed=false`.
  `result.unit_witness_policy` is
  `preserve_source_result_value_unit_and_dimension_per_headless_result_handoff_row`,
  and `result.unit_preservation_witnesses[]` preserves finite result-row
  value, unit, and inferred dimension metadata by reference.
- UI check: `data-testid="headless-runner-units"` displays the DEC-018
  disclosure and `data-testid="headless-runner-unit-witnesses"` displays
  `count=737` with `conversion=false` for the invented preview fixture after
  mechanics preview. The pre-run preview path keeps `result_units=[]`,
  `conversion=false`, and zero witnesses explicit.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
    passed 2/2 Playwright tests.
- Evidence: DEL-10-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-HEADLESSRUNNERUNITWITNESS-001.md`;
  DEL-02-02 and DEL-08-04 supporting run records with the same id; completion
  log entry.
- Boundary: headless-runner result-handoff unit metadata only. No final CLI
  syntax, package script, process/network/filesystem policy, public transport,
  CI/release matrix, runtime process launcher, schema contract change, unit
  conversion API, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-201 CAEPIPE external parser unit witnesses - PASSED (`TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001`, 2026-06-17)

- Scope: bounded Phase B-tail CAEPIPE external parser/unit-evidence slice
  while C5.7 remains human-execution gated. The desktop CAEPIPE External
  Harness panel now emits explicit DEC-018 unit-system disclosure and
  per-parser-row unit-preservation witnesses for the invented public CSV
  parser fixture.
- Packet contract: `unit_system_disclosure` records
  `unit-system:dec-018-si-dual-display`, target parser export units for node
  displacements and element forces, and `conversion_performed=false`.
  `unit_witness_policy` is
  `preserve_parser_csv_row_value_unit_and_dimension_per_row`, and
  `unit_preservation_witnesses[]` preserves each parser CSV row value, unit,
  and inferred dimension.
- UI check: `data-testid="caepipe-external-units"` displays the DEC-018
  disclosure and `data-testid="caepipe-external-unit-witnesses"` displays
  `count=3` with `conversion=false`.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
    passed 2/2 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
- Evidence: DEL-17-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001.md`;
  DEL-02-02 and DEL-08-04 supporting run records with the same id; completion
  log entry.
- Boundary: CAEPIPE external parser unit metadata only. No external CAEPIPE
  execution, executable/license/path requirement, target compatibility claim,
  solver validation claim, schema contract change, unit conversion API,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-202 operation diff preview unit witnesses - PASSED (`TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001`, 2026-06-17)

- Scope: bounded Phase B-tail operation-diff/unit-evidence slice while C5.7
  remains human-execution gated. The desktop Operation Diff Preview packet now
  emits explicit DEC-018 unit-system disclosure and per-unit-bearing-diff-row
  preservation witnesses for queued local operation previews.
- Packet contract: `unit_system_disclosure` records
  `unit-system:dec-018-si-dual-display`, target diff rows as
  `per_change_declared_unit`, and `conversion_performed=false`.
  `unit_witness_policy` is
  `preserve_operation_diff_change_value_unit_and_dimension_per_unit_bearing_row`,
  and `unit_preservation_witnesses[]` preserves each unit-bearing before/after
  diff row's value text, unit, and dimension.
- UI check: `data-testid="diff-preview-units"` displays the DEC-018
  disclosure and `data-testid="diff-preview-unit-witnesses"` displays
  `count=1` with `conversion=false` for the focused material-modulus edit
  fixture.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
    passed 2/2 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
  - Playwright smoke was not extended with a witness-specific assertion for
    this slice: the broad R2 smoke previews unit-bearing edits but does not
    retain a queued diff row without changing later smoke flow. An attempted
    queue-and-clear assertion path was removed after hanging. Focused App
    coverage renders the UI and parses the exported packet.
- Evidence: DEL-16-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001.md`;
  DEL-16-03 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: operation diff unit metadata only. No operation schema change,
  operation application, accepted model-state mutation, durable acceptance
  persistence, unit conversion API, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-208 PCF conversion witness visibility - PASSED (`TP-UNITS-BTAIL-PCFCONVWITNESSVISIBLE-001`, 2026-06-17)

- Scope: bounded Phase B-tail PCF/export-evidence slice while C5.7 remains
  human-execution gated. The desktop Conservative PCF Export panel now exposes
  the already-built PCF conversion-witness count and target unit in the visible
  UI, so browser smoke verifies the same witness sidecar that the package JSON
  already carries.
- Unit behavior: no conversion algorithm or package schema changed. The
  existing PCF package still converts source length fields to the conservative
  target millimeter fields and carries `conversion_witnesses.json`; the new
  `data-testid="pcf-export-conversion-witnesses"` line reports
  `count=23`, the node/pipe-field scope, and `target_length=MM`.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-17-07 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PCFCONVWITNESSVISIBLE-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: PCF desktop visibility/evidence only. No PCF package schema
  change, conversion formula change, target compatibility claim, downstream
  import evidence, solver-validation claim, unit conversion API, protected
  standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-209 viewport geometry unit-validation evidence - PASSED (`TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail viewport-geometry/unit-validation slice while
  C5.7 remains human-execution gated. Explicit viewport node and straight-pipe
  geometry intents now record length unit-dimension validation evidence instead
  of `unit_validation=not_run`.
- Unit behavior: the viewport already loaded the DEC-018 unit catalog when
  available and displayed the browser-preview model-metadata fallback. This
  tranche threads that existing route into the queued explicit geometry
  intents: browser preview records
  `length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`;
  desktop/Tauri routes can record DEC-018 accepted/mismatch/loading/unreviewed
  statuses from the same helper. Generic one-click viewport gesture
  placeholders were later covered by TP-MAC-225.
- UI check: `data-testid="viewport-intent-unit-validation-create_node"` and
  `data-testid="viewport-intent-unit-validation-connect_pipe_run"` expose the
  queued validation status before the operation is applied. The A12 from-blank
  Playwright journey checks both node and pipe evidence in desktop and compact
  viewports.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
- Evidence: DEL-07-01 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: viewport intent evidence only. No DEC-018 catalog constant change,
  schema dimension enum change, unit conversion API change, operation-applier
  validation semantics, solver behavior, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-210 primitive-load delete unit-validation evidence - PASSED (`TP-UNITS-BTAIL-PRIMDELETEUNITVALID-001`, 2026-06-17)

- Scope: bounded Phase B-tail load-case/unit-validation slice while C5.7
  remains human-execution gated. Explicit primitive-load delete intents now
  record the existing primitive's unit-dimension validation evidence instead
  of `unit_validation=not_run`.
- Unit behavior: deletion preserves the existing primitive's unit and
  dimension in the operation-intent evidence and runs the same status helper
  used by primitive-load creation and magnitude edits. Browser preview records
  `model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`;
  desktop/Tauri routes can record DEC-018 accepted/mismatch/loading/unreviewed
  statuses from the same helper.
- UI check: `data-testid="load-manager-primitive-delete-preview"` exposes the
  queued validation status before the operation is applied. The focused App
  test checks the visible delete-preview evidence for `load:L-100-Y`.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- Evidence: DEL-05-01 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PRIMDELETEUNITVALID-001.md`;
  DEL-02-02 and DEL-05-05 supporting run records with the same id; completion
  log entry.
- Boundary: primitive-load delete intent evidence only. No DEC-018 catalog
  constant change, schema dimension enum change, unit conversion API change,
  operation-applier validation semantics, solver behavior, load engine
  behavior, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-211 external-prover unit policy evidence - PASSED (`TP-UNITS-BTAIL-EXTERNALPROVERUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail target-format/unit-evidence slice while C5.7
  remains human-execution gated. The DEL-15-04 external-prover boundary
  metadata packet now records explicit DEC-018 unit-policy evidence for the
  metadata-only external review context.
- Unit behavior: the packet reuses the shared export unit disclosure helper to
  record `unit-system:dec-018-si-dual-display`, entered-unit preservation,
  source model units, result units when an analysis run exists, empty target
  export units, and
  `conversion_policy=external_prover_metadata_records_units_without_target_conversion`
  with `conversion_performed=false`.
- UI check: `data-testid="external-prover-unit-policy"` exposes source/result
  unit disclosure, `conversion=false`, the external-prover unit policy, and
  witness count. The exported metadata JSON carries `unit_policy_evidence`
  with `external_prover_scope=metadata_only_external_review_context`,
  `analysis_run_ref`, and the desktop preview external reference.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 398/398 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
- Evidence: DEL-15-04 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-EXTERNALPROVERUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: external-prover metadata evidence only. No external solver/prover
  invocation, target parser, commercial-result ingestion, target writer,
  conversion API, target compatibility claim, protected standards content,
  private payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-212 comparison workspace unit policy evidence - PASSED (`TP-UNITS-BTAIL-COMPARISONGUIUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail comparison-workspace/unit-evidence slice while
  C5.7 remains human-execution gated. The desktop comparison packet now
  records explicit unit-policy evidence for mapped result deltas, and the
  comparison panel exposes that policy beside the tolerance boundary.
- Unit behavior: comparison matching already required equal explicit result
  units before producing a delta. This tranche records that invariant as
  `unit_policy_evidence`, with
  `comparison_unit_policy=compare_only_rows_with_equal_explicit_result_units`,
  `conversion_policy=comparison_workspace_preserves_result_units_without_conversion`,
  matched result units, unmatched row counts, `conversion_performed=false`,
  and `tolerance_profile_ref=TBD`.
- UI check: `data-testid="comparison-unit-policy"` exposes matched result
  units, `matching=equal_explicit_units`, `conversion=false`, and
  `tolerance=not_tolerance_checked`. The focused App test checks both the
  packet-level evidence and the rendered row; R2/R3 Playwright smoke checks
  the visible row in a real browser.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 56/56 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
- Evidence: DEL-14-04 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-COMPARISONGUIUNITS-001.md`;
  DEL-14-05 and DEL-02-02 supporting run records with the same id;
  completion log entry.
- Boundary: comparison workspace evidence only. No comparison delta math,
  tolerance profile, default tolerance, solver convergence policy, external
  validation decision, conversion API, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-213 design workspace unit policy evidence - PASSED (`TP-UNITS-BTAIL-DESIGNWORKSPACEUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail design-workspace/unit-evidence slice while C5.7
  remains human-execution gated. The DEL-07-08 design-authoring workspace
  packet now records explicit unit-policy evidence for the composed model,
  result, analysis-run, and comparison context exposed by the Results section.
- Unit behavior: the workspace packet records
  `unit-system:dec-018-si-dual-display`, entered-unit preservation, sorted
  model units, result units when a run exists, comparison units when a
  comparison exists, the embedded comparison unit-policy evidence ref,
  `conversion_policy=design_workspace_preserves_source_units_without_conversion`,
  `conversion_performed=false`, `tolerance_profile_ref=TBD`, and
  DEC-018/DEC-026/DEL-02-02/DEL-14-04 basis refs.
- UI check: `data-testid="design-workspace-units"` exposes the model unit
  manifest, result units, comparison units, and `conversion=false`. The
  focused App test checks both the unsolved and solved packet/UI states; the
  R2/R3 Playwright smoke checks the solved browser row.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 56/56 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
- Evidence: DEL-07-08 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-DESIGNWORKSPACEUNITS-001.md`;
  DEL-02-02 and DEL-14-04 supporting run records with the same id; completion
  log entry.
- Boundary: design-workspace evidence only. No model mutation, comparison
  delta math, tolerance profile, default tolerance, solver behavior, external
  validation decision, conversion API, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-214 project validation unit policy evidence - PASSED (`TP-UNITS-BTAIL-PROJECTVALIDATIONUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail project-validation/unit-evidence slice while
  C5.7 remains human-execution gated. The DEL-02-05 Project Validation
  Preflight packet now records explicit DEC-018 unit-policy evidence for the
  existing unit round-trip metadata check.
- Unit behavior: the packet records
  `unit-system:dec-018-si-dual-display`, entered-unit preservation, sorted
  model units, the model unit-bearing record count, persisted round-trip
  status/signature when a local project snapshot exists, and
  `conversion_policy=project_validation_records_unit_round_trip_metadata_without_conversion`
  with `conversion_performed=false`.
- UI check: `data-testid="project-validation-unit-policy"` exposes the model
  unit manifest, unit-bearing record count, round-trip status, and
  `conversion=false`. The focused App test checks both not-persisted and
  saved/opened packet states; R2/R3 Playwright smoke checks the saved/opened
  browser row.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 56/56 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
- Evidence: DEL-02-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PROJECTVALIDATIONUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: project-validation evidence only. No persistence semantics,
  migration policy, hash canonicalization, schema versioning, unit conversion
  API, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-215 result viewer unit policy evidence - PASSED (`TP-UNITS-BTAIL-RESULTVIEWUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail results-viewer/unit-evidence slice while C5.7
  remains human-execution gated. The DEL-07-05 Results viewer now visibly
  records the DEC-018 result-unit policy for the solved preview result
  envelope before filtering and pagination.
- Unit behavior: the result controls expose
  `unit-system:dec-018-si-dual-display` by policy, entered-unit preservation,
  sorted source result units `MPa,N,N*m,mm,rad`, 737 source result rows, and
  `conversion_policy=result_view_preserves_result_row_units_without_conversion`
  with `conversion_performed=false`.
- UI check: `data-testid="result-unit-policy"` exposes the source result unit
  manifest, result-row count, entered-unit storage convention, and
  `conversion=false`. The focused App test and R2/R3 Playwright smoke now
  check the browser row after the preview solve.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 56/56 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests
    on rerun after one unrelated transient support-label cross-test failure.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
- Evidence: DEL-07-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RESULTVIEWUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: result-viewer evidence only. No result math, result filtering,
  pagination semantics, solver behavior, comparison delta math, tolerance
  profile, default tolerance, unit conversion API, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-216 delete-intent unit-validation evidence - PASSED (`TP-UNITS-BTAIL-DELETEINTENTUNITVALID-001`, 2026-06-17)

- Scope: bounded Phase B-tail Property Inspector/delete-intent
  unit-validation slice while C5.7 remains human-execution gated. The explicit
  support, node, and pipe delete intent previews now record unit validation as
  `not_required_dimensionless` instead of leaving `unit_validation=not_run`.
- Unit behavior: delete intents do not introduce a new unit-bearing numeric
  quantity. The preview metadata now records that fact explicitly while still
  leaving schema/reference validation to the existing validate/apply path.
- UI check: `data-testid="editor-intent-validation"` reports
  `not_required_dimensionless` for support/node/pipe delete previews. The
  focused App test checks all three delete families, and R2/R3 Playwright
  smoke checks the support-delete browser preview without applying the
  deletion.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 56/56 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `git diff --check` passed.
- Evidence: DEL-07-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-DELETEINTENTUNITVALID-001.md`;
  DEL-16-02 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: delete-intent metadata only. No delete operation behavior,
  reference validation, operation application semantics, accepted model-state
  mutation, solver behavior, unit conversion API, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-217 load-manager dimensionless unit-validation evidence - PASSED (`TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001`, 2026-06-17)

- Scope: bounded Phase B-tail Load Cases manager unit-validation slice while
  C5.7 remains human-execution gated. Non-unit-bearing load-case and
  combination operation intents now record unit validation as
  `not_required_dimensionless` instead of leaving `unit_validation=not_run`.
- Unit behavior: empty load-case shell creation, load-case metadata edits,
  whole load-case deletion, combination creation, combination basis/factor
  edits, combination term creation/deletion, and whole-combination deletion
  already declare `unit=none` and `dimension=dimensionless`; their preview
  metadata now makes that classification explicit.
- UI check: the Load Cases manager preview testids for create-load,
  metadata/delete, create-combination, combination term create/delete,
  combination basis/factor, and combination delete now expose
  `unit_validation=not_required_dimensionless`. Unit-bearing primitive load
  create/edit paths remain routed through the existing unit/dimension
  validation helper.
- Validation:
  - `npm test --workspace apps/desktop -- src/App.test.tsx -t "manager panel"`
    passed 18/18 focused manager tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    14/14 Playwright tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-05-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001.md`;
  DEL-07-02, DEL-16-02, and DEL-02-02 supporting run records with the same
  id; completion log entry.
- Boundary: operation-intent metadata only. No load-case algebra solver
  behavior, operation application behavior, accepted model-state mutation,
  durable persistence, schema enum, unit-conversion API, DEC-018 catalog
  constant, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-218 viewport component-symbol unit-validation evidence - PASSED (`TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001`, 2026-06-17)

- Scope: bounded Phase B-tail viewport component-symbol unit-validation slice
  while C5.7 remains human-execution gated. The reference-only viewport
  `insert_component_symbol` placeholder now records unit validation as
  `not_required_dimensionless`.
- Unit behavior: the component-symbol placeholder introduces no unit-bearing
  numeric quantity. Explicit viewport node/pipe authoring paths already carry
  length unit-validation evidence; generic node/pipe placeholders were later
  covered by TP-MAC-225.
- UI check: `data-testid="viewport-intent-unit-validation-insert_component_symbol"`
  exposes `unit_validation=not_required_dimensionless`. The focused App test
  checks the component-symbol status, and Playwright R2/R3 smoke checks the
  browser row in desktop and compact viewports.
- Validation:
  - `npm test --workspace apps/desktop -- src/App.test.tsx -t "viewport editor intents"`
    passed 1/1 focused App test.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts` passed
    16/16 Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-07-01 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001.md`;
  DEL-16-02 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: operation-intent metadata only. No operation application behavior,
  accepted model-state mutation, durable persistence, schema enum,
  unit-conversion API, DEC-018 catalog constant, protected standards content,
  private payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-225 viewport placeholder unit-validation evidence - PASSED (`TP-UNITS-BTAIL-VIEWPORTPLACEHOLDERUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail viewport placeholder/unit-validation slice while
  C5.7 remains human-execution gated. The generic one-click `create_node` and
  `connect_pipe_run` viewport gesture placeholders now record length
  unit-dimension validation evidence instead of `unit_validation=not_run`.
- Unit behavior: node and pipe placeholders keep the review-only
  `pending_service_validation` and no-mutation posture, but their intent
  metadata now carries `unit=m`, `dimension=length`, and
  `unit_validation=length=<status>`. Browser preview records
  `length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`;
  desktop/Tauri routes can record DEC-018 accepted/mismatch/loading/unreviewed
  statuses from the same helper. The component-symbol placeholder remains
  `not_required_dimensionless`.
- UI check: the focused App test verifies node, pipe, and component placeholder
  statuses after the unit route is loaded. The focused Playwright smoke checks
  the same three placeholders in desktop and compact browser projects.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "records viewport editor intents without direct persisted-project mutation"`
    passed 1/1 selected test.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "viewport gesture placeholders record unit validation"`
    passed 2/2 focused Playwright tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - After the DEC-025 sweep exposed a timing-sensitive viewport animation
    sample in the broad R2 smoke, `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
    passed 1/1 with the polling assertion.
- Command note: an earlier full Playwright run without `--workers=1` printed
  all 18 passing tests but did not exit promptly; it was interrupted and is
  not counted as validation evidence.
- Evidence: DEL-07-01 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTPLACEHOLDERUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: viewport placeholder metadata only. No accepted model-state
  mutation, operation-applier validation semantics, solver behavior, component
  schema, unit conversion API, DEC-018 catalog constant, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-219 validation-evidence unit-policy visibility - PASSED (`TP-UNITS-BTAIL-VALIDATIONEVIDUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail validation-evidence/unit-policy slice while
  C5.7 remains human-execution gated. The Validation Evidence panel and export
  packet now expose DEC-018 project-unit context for the manual/evidence
  inventory without asserting release thresholds.
- Unit behavior: `unit_policy_evidence` records
  `unit-system:dec-018-si-dual-display`, entered-unit preservation, sorted
  project units, 18 unit-bearing model records, the
  `unit_and_schema_verification` manual section, and
  `conversion_performed=false`.
- UI check: `data-testid="validation-evidence-unit-policy"` reports the model
  unit signature, record count, manual section, and no-conversion posture. The
  focused App test checks the rendered line and exported JSON packet; the
  focused Playwright smoke checks the same row in desktop and compact browser
  projects.
- Validation:
  - `npm test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace"`
    passed 1/1 focused App test.
  - `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts:548` passed
    2/2 focused Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-09-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VALIDATIONEVIDUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: validation-evidence review metadata only. No release threshold,
  release authorization, persistence behavior, solver behavior, unit
  conversion API, DEC-018 catalog constant, protected standards content,
  private payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-220 missing-data unit-input policy visibility - PASSED (`TP-UNITS-BTAIL-MISSINGDATAUNITPOLICY-001`, 2026-06-17)

- Scope: bounded Phase B-tail missing-data/unit-policy slice while C5.7
  remains human-execution gated. The Missing Data Blocking panel and export
  packet now expose explicit unit-input policy evidence for unit-bearing
  missing inputs without changing solve, rule-check, or lifecycle state.
- Unit behavior: `unit_input_policy_evidence` records DEC-018/DEL-02-02/
  DEL-07-04 basis refs, sorted project units, explicit-unit requirement for
  unit-bearing inputs, no inferred/default units, no auto-filled missing
  units, and `conversion_performed=false`.
- UI check: `data-testid="missing-data-unit-policy"` reports
  `required=true`, `default_units=false`, `conversion=false`, and the model
  unit signature. The focused App test checks the rendered line and exported
  JSON packet; the focused Playwright smoke checks the same row in desktop
  and compact browser projects before the mechanics run.
- Validation:
  - `npm test --workspace apps/desktop -- --run src/App.test.tsx` passed
    1/1 file and 56/56 tests.
  - `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop -- e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay"`
    passed 2/2 focused Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-07-04 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-MISSINGDATAUNITPOLICY-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: missing-data review metadata only. No solver behavior,
  rule-check behavior, unit conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-221 accessibility-baseline unit-visibility evidence - PASSED (`TP-UNITS-BTAIL-A11YUNITVISIBILITY-001`, 2026-06-17)

- Scope: bounded Phase B-tail/C5 usability-support slice while C5.7 remains
  human-execution gated. The Accessibility Baseline panel and export packet
  now expose explicit unit-visibility evidence for unit-bearing review
  surfaces without selecting a final accessibility target.
- Unit behavior: `unit_visibility_evidence` records DEC-018/DEL-02-02/
  DEL-07-05/DEL-07-06 basis refs, sorted project units, unit-bearing review
  surfaces that keep unit labels visible, no color-only unit signaling, no
  inferred/default units, and `conversion_performed=false`.
- UI check: `data-testid="accessibility-baseline-unit-visibility"` reports
  the unit-visibility policy, model unit signature, result-row visibility
  basis, and no-conversion posture. The focused App test checks the rendered
  line and exported JSON packet; the focused Playwright from-blank journey
  checks the same row in desktop and compact browser projects.
- Validation:
  - `npm test --workspace apps/desktop -- --run src/App.test.tsx` passed
    1/1 file and 56/56 tests.
  - `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop -- e2e/r2-smoke.spec.ts -g "R2 from-blank GUI journey authors the A12 rehearsal script"`
    passed 2/2 focused Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-07-06 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-A11YUNITVISIBILITY-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: accessibility-baseline review metadata only. No final
  accessibility conformance target, desktop accessibility-tree audit claim,
  solver behavior, rule-check behavior, unit conversion API, DEC-018 catalog
  constant, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-222 run-audit unit traceability evidence - PASSED (`TP-UNITS-BTAIL-RUNAUDITUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail analysis-run audit evidence slice while C5.7
  remains human-execution gated. The Run Audit panel now exposes solved-run
  unit traceability from the already-bound model and result envelope.
- Unit behavior: `data-testid="run-audit-units"` reports sorted model unit
  declarations, result row unit symbols in the established Results-panel
  display order, row count, `source=result_envelope`, and
  `conversion=false`.
- UI check: the focused App test verifies the run-audit unit line after
  mechanics preview solve. The focused R2 Playwright journey checks the same
  line in desktop and compact browser projects after solve.
- Validation:
  - `npm test --workspace apps/desktop -- --run src/App.test.tsx` passed
    1/1 file and 56/56 tests.
  - `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop -- e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay"`
    passed 2/2 focused Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-14-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RUNAUDITUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: run-audit UI evidence only. No analysis-run schema, hash
  canonicalization, solver behavior, result value, unit conversion API,
  DEC-018 catalog constant, schema dimension enum, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-223 design-knowledge computed-unit context - PASSED (`TP-UNITS-BTAIL-KNOWLEDGEUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail design-knowledge review-surface slice while
  C5.7 remains human-execution gated. The Design Knowledge panel now exposes
  explicit unit context for computed result-derived knowledge records.
- Unit behavior: `data-testid="knowledge-unit-context"` reports the count of
  computed unit-bearing knowledge result refs, result units in the established
  Results-panel display order, `source=computed_preview_result`, and
  `conversion=false`.
- UI check: the focused App test verifies the Design Knowledge unit-context
  line after mechanics preview solve. The focused R2 Playwright journey checks
  the same line in desktop and compact browser projects after solve.
- Validation:
  - `npm test --workspace apps/desktop -- --run src/App.test.tsx` passed
    1/1 file and 56/56 tests.
  - `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop -- e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay"`
    passed 2/2 focused Playwright tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-13-01 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-KNOWLEDGEUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: design-knowledge review UI evidence only. No design-knowledge
  schema, solver behavior, result value, unit conversion API, DEC-018 catalog
  constant, schema dimension enum, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-224 diagnostic linked-result unit context - PASSED (`TP-UNITS-BTAIL-DIAGNOSTICUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail diagnostic review-surface slice while C5.7
  remains human-execution gated. The Diagnostic Detail panel now exposes
  explicit unit context for result rows linked to a selected diagnostic.
- Unit behavior: `data-testid="diagnostic-unit-context"` reports linked result
  count, first-class result units in linked-result order, `source=result_envelope`,
  and `conversion=false`. For `HIGH_DISPLACEMENT_REVIEW`, the selected
  node-level diagnostic links 21 result rows with units `mm,rad`.
- UI check: the focused App test verifies the Diagnostic Detail unit-context
  line after mechanics preview solve. The R2/R3 Playwright smoke file checks
  the same row in desktop and compact browser projects.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "links selected diagnostics to affected result and model context"`
    passed 1/1 selected test.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "diagnostic detail exposes linked result unit context"`
    passed 2/2 focused Playwright tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npx playwright test e2e/r2-smoke.spec.ts` passed 18/18 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
- Evidence: DEL-07-07 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-DIAGNOSTICUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: diagnostic review UI evidence only. No diagnostic schema, solver
  behavior, result value, unit conversion API, DEC-018 catalog constant,
  schema dimension enum, protected standards content, private payload,
  lifecycle state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-226 secret/private-library unit policy visibility - PASSED (`TP-UNITS-BTAIL-SECRETPRIVATEUNITPOLICY-001`, 2026-06-17)

- Scope: bounded Phase B-tail secret/private-library unit-policy slice while
  C5.7 remains human-execution gated. The Secret & Private Libraries panel now
  exposes how unit-bearing private material/rule references are handled when
  private payloads are withheld.
- Unit behavior: `data-testid="secret-private-library-unit-policy"` reports
  `unit_refs=2`, `required=true`, `payload=false`, and `conversion=false`.
  The exported packet carries `unit_policy` with DEC-018/DEL-02-02/DEL-12-04
  refs and per-reference metadata statuses for the private material library
  and private rule-pack references.
- UI check: the focused App test verifies the visible row and downloaded JSON
  packet. The focused R2 Playwright journey checks the same visible row in a
  real browser.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test after fixing the helper destructuring miss caught
    by the first focused run.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
    passed 1/1 focused Playwright test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-12-04 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SECRETPRIVATEUNITPOLICY-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: secret/private-library metadata UI evidence only. No private
  library payload read or write, credential handling, storage-root
  finalization, unit conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-227 editor-contract unit contract visibility - PASSED (`TP-UNITS-BTAIL-EDITORCONTRACTUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail editor-contract unit-contract slice while C5.7
  remains human-execution gated. The Editor Contract review panel now exposes
  the DEL-02-02 unit contract already carried in its exported packet.
- Unit behavior: `data-testid="editor-contract-unit-contract"` reports
  `contract=DEL-02-02`,
  `schema=schemas/units.schema.yaml#/$defs/DimensionId`,
  `policy=unit_bearing_values_require_explicit_unit_metadata`, and
  `missing=diagnostic_blocking`.
- UI check: the focused App test verifies the visible row and downloaded JSON
  packet. The focused R2 Playwright journey checks the same visible row in a
  real browser.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "guided workbench shell keeps journey steps, details, and compact status reachable" --project=chromium-desktop`
    passed 1/1 focused Chromium desktop test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-07-03 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-EDITORCONTRACTUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: editor-contract UI/packet visibility only. No editor persistence,
  accepted model-state mutation, operation-applier behavior, solver behavior,
  unit conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-228 rule-check run binding unit policy visibility - PASSED (`TP-UNITS-BTAIL-RULECHECKBINDUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail run-rule-check binding unit-policy slice while
  C5.7 remains human-execution gated. The Run Rule Checks binding plan now
  exposes a visible unit-binding policy summary for runtime value, slot,
  solver-result, and private-library binding routes.
- Unit behavior: `data-testid="rule-check-unit-binding-policy"` reports
  `value_inputs=1`, `value_slots=1`, `solver_selectors=1`,
  `solver_result_refs=0`, `private_library_refs=0`,
  `catalog=browser_manual_text_no_fallback`, and `conversion=false` for the
  browser demo-pack route. Desktop-mode Vitest also verifies
  `catalog=dec018_catalog(entries=4)`.
- UI check: the focused RuleCheckRunPanel test verifies browser/manual-text,
  desktop DEC-018 catalog, and private-library-reference cases. The focused
  R2 Playwright run-check journey checks the same visible row in a real
  browser.
- Validation:
  - `npm run test --workspace apps/desktop -- RuleCheckRunPanel.test.tsx`
    passed 18/18 tests.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "run-rule-checks panel loads the demo pack, derives bindings, and reports the desktop-only run seam" --project=chromium-desktop`
    passed 1/1 focused Chromium desktop test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-06-03 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RULECHECKBINDUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: run-check binding-plan UI evidence only. No rule-pack schema,
  expression grammar, parser/text syntax, backend completeness/evaluator
  behavior, rule-pack persistence, solver behavior, unit conversion API,
  DEC-018 catalog constant, schema dimension enum, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-229 operation apply unit policy visibility - PASSED (`TP-UNITS-BTAIL-OPAPPLYUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail Apply Operations unit-policy visibility slice
  while C5.7 remains human-execution gated. The Apply Operations panel now
  exposes a visible summary of queued unit-bearing intents, dimensionless
  intents, operation outcome unit-validation statuses, applied receipt count,
  receipt unit-serialization policy, and conversion posture.
- Unit behavior: `data-testid="operation-apply-unit-policy"` reports
  `queued_unit_bearing=1`, `queued_dimensionless=0`,
  `outcome_unit_validations=none`, `applied_receipts=0`,
  `receipt_units=not_serialized`, and `conversion=false` before applying the
  queued load-magnitude edit. After applying through the local WASM route it
  reports `queued_unit_bearing=0`, `outcome_unit_validations=passed`,
  `applied_receipts=1`, `receipt_units=not_serialized`, and
  `conversion=false`.
- UI check: the focused App test verifies the manager-panel queue/apply path.
  The focused R2 Playwright journey checks the same visible row in a real
  browser and confirms the applied operation route remains
  `local_wasm_engine`.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "queues and applies a load-case primitive magnitude through the manager panel"`
    passed 1/1 selected test.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
    passed 1/1 focused Chromium desktop test.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"`
    passed 1/1 selected test after the scenario-specific export-review lint
    count assertion was restored.
  - `npm run test --workspace apps/desktop` initially caught the over-broad
    export-review manifest count assertion and then passed 18/18 files and
    399/399 tests after repair.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
  - `git diff --check` passed.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-16-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-OPAPPLYUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: Apply Operations UI/review evidence only. No operation-applier
  semantics, backend validation, receipt schema, solver behavior,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-230 operation ledger unit policy visibility - PASSED (`TP-UNITS-BTAIL-OPLEDGERUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail Operation Review Ledger unit-policy visibility
  slice while C5.7 remains human-execution gated. The Operation Review Ledger
  now exposes a visible and exported unit-policy summary for queued GUI
  operation intents and agent proposal review records.
- Unit behavior: `data-testid="operation-ledger-unit-policy"` reports record
  count, unit-bearing change count, dimensionless change count, unit
  validation statuses, `receipt_units=not_serialized_in_review_ledger`, and
  `conversion=false`. The exported ledger JSON carries
  `unit_policy_evidence` with
  `unit_policy=ledger_preserves_operation_unit_metadata_without_conversion`.
- UI check: the focused App tests verify viewport intent, material-edit, and
  agent-proposal ledger paths. The focused R2 Playwright journey checks the
  same visible row in a real browser before apply.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "records viewport editor intents without direct persisted-project mutation|carries queued editor intents into the report packet as review-only operation context|shows computed mechanics diagnostics in results, knowledge, and review-only proposal context"`
    passed 3/3 selected tests.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
    passed 1/1 focused Chromium desktop test.
  - `git diff --check` passed.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-16-03 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-OPLEDGERUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: Operation Review Ledger UI/export evidence only. No operation
  application, acceptance semantics, durable audit persistence, receipt
  schema, solver behavior, unit-conversion API, DEC-018 catalog constant,
  schema dimension enum, protected standards content, private payload,
  lifecycle state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-231 report-lint unit policy inventory - PASSED (`TP-UNITS-BTAIL-REPORTLINTUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail Report Content Lint unit-policy inventory slice
  while C5.7 remains human-execution gated. The report linter now exposes a
  visible and exported inventory of public report/export surfaces that carry
  unit-policy or target-format conversion-witness evidence.
- Unit behavior: `data-testid="report-lint-unit-policy"` reports
  `unit_targets=17`, `conversion_witness_targets=2`, and
  `lint_conversion=false`. The downloaded lint JSON carries
  `unit_policy_evidence` with
  `evidence_kind=public_surface_unit_policy_inventory`,
  `lint_performs_conversion=false`, and
  `lint_asserts_target_format_compatibility=false`.
- UI check: the focused App test verifies the visible row and packet fields
  for the report linter. The focused R2 Playwright journey checks the same
  visible row and exported packet in a real browser.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
    passed 1/1 focused Chromium desktop test.
  - `git diff --check` passed.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-REPORTLINTUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: Report Content Lint UI/export evidence only. No linter
  protected-content semantics, legal clearance, redaction controls, target
  writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-232 rendered-report unit basis visibility - PASSED (`TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail Rendered Report unit-basis visibility slice
  while C5.7 remains human-execution gated. The Rendered Report panel now
  exposes the render-input unit basis before invoking the desktop-only
  renderer route.
- Unit behavior: `data-testid="rendered-report-unit-basis"` reports
  `unit_system=unit-system:dec-018-si-dual-display`, sorted model units,
  solved result units or `results=none`, `conversion=false`, and
  `source=renderable_report_input`. This mirrors the existing
  `buildRenderableReportInput` unit display summary and does not change the
  Rust renderer, report schema, or report-time conversion behavior.
- UI check: the focused rendered-report Vitest verifies the visible row for
  the component route. The focused R2 Playwright checks verify the row for
  both solved preview results and the A12 from-blank browser-preview render
  route.
- Validation:
  - `npm run test --workspace apps/desktop -- src/features/report/renderedReport.test.tsx`
    passed 8/8 tests.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay|R2 from-blank GUI journey authors the A12 rehearsal script" --project=chromium-desktop`
    passed 2/2 focused Chromium desktop tests.
  - `git diff --check` passed.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-01 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: Rendered Report panel UI evidence only. No renderer command,
  report schema, canonical hash behavior, save/print gate, report-time
  conversion, unit-conversion API, DEC-018 catalog constant, schema dimension
  enum, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-233 solve-job unit policy visibility - PASSED (`TP-UNITS-BTAIL-SOLVEJOBUNITS-001`, 2026-06-17)

- Scope: bounded Phase B-tail Solve Job unit-policy visibility slice while
  C5.7 remains human-execution gated. The Solve execution panel now exposes
  model/result unit basis in the solve-job audit row and JSON packet.
- Unit behavior: `data-testid="solve-job-unit-policy"` reports sorted model
  units, solved result units or `results=none`, result row count, and
  `conversion=false`. The downloaded solve-job JSON carries
  `unit_policy_evidence` with DEC-018 and DEL-02-02/DEL-07-07 basis refs,
  `storage_convention=entered_units_preserved`,
  `solve_job_unit_policy=solve_job_audit_records_model_and_result_units_without_conversion`,
  and `conversion_performed=false`.
- UI check: focused App tests verify the row and packet fields before a run
  and after the preview mechanics run. The focused R2 Playwright journey
  verifies the solved row in browser smoke and the from-blank zero-result
  path.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "computed mechanics diagnostics"`
    passed 1/1 selected test.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay"`
    passed 2/2 focused Playwright tests.
  - `git diff --check` passed.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-07-07 primary run record
  `WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SOLVEJOBUNITS-001.md`;
  DEL-02-02 supporting run record with the same id; completion log entry.
- Boundary: Solve Job audit UI/export evidence only. No solve execution
  behavior, solver backend seam, result values, analysis-run hashes,
  cancellation behavior, unit-conversion API, DEC-018 catalog constant,
  schema dimension enum, protected standards content, private payload,
  lifecycle state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-234 project-storage unit policy evidence - PASSED (`TP-UNITS-BTAIL-STORAGEUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Project Storage Audit unit-policy evidence
  slice while C5.7 remains human-execution gated. The Project Storage Audit
  packet now exposes structured unit-policy evidence for the existing local
  project unit round-trip metadata.
- Unit behavior: `data-testid="project-storage-unit-round-trip"` now reports
  sorted model units and `conversion=false` alongside the existing round-trip
  status, checked-ref count, and signature. The downloaded local storage JSON
  carries `unit_policy_evidence` with DEC-018 and DEL-02-02/DEL-02-05 basis
  refs, `storage_convention=entered_units_preserved`,
  `storage_unit_policy=local_project_storage_audit_records_unit_round_trip_status_without_conversion`,
  and `conversion_performed=false`.
- Report-lint inventory: the Report Content Lint public unit-policy inventory
  now includes the Project Storage Audit surface, increasing
  `unit_targets` from 17 to 18 while keeping `conversion_witness_targets=2`
  and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "computed mechanics diagnostics"`
    passed 1/1 selected test.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay|R2 from-blank GUI journey authors the A12 rehearsal script" --project=chromium-desktop`
    passed 2/2 focused Playwright tests.
  - `git diff --check` passed.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-02-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-STORAGEUNITS-001.md`;
  DEL-02-02 and DEL-08-05 supporting run records with the same id; completion
  log entry.
- Boundary: Project Storage Audit UI/export evidence and report-lint
  inventory only. No local persistence semantics, migration policy,
  project-envelope schema, model hash canonicalization, report-linter
  protected-content semantics, unit-conversion API, DEC-018 catalog constant,
  schema dimension enum, protected standards content, private payload,
  lifecycle state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-235 native package unit inventory coverage - PASSED (`TP-UNITS-BTAIL-NATIVEPACKAGEUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Native JSON Package unit-inventory slice while
  C5.7 remains human-execution gated. The Native JSON package already exposes
  project/model/result unit preservation witnesses; this tranche records that
  public unit surface in the Report Content Lint inventory.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/native-package/NativePackagePanel.tsx` in its
  explicit public target roots and target list, and adds
  `native-package-unit-witnesses` to `unit_policy_evidence.target_refs`.
  The visible lint row reports `unit_targets=19`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    initially caught stale target-count assertions, then passed 1/1 selected
    test after the assertions were updated.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "computed mechanics diagnostics"`
    initially caught stale target-count assertions, then passed 1/1 selected
    test after the assertions were updated.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
    passed 1/1 focused Chromium desktop test.
- Evidence: DEL-17-03 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-NATIVEPACKAGEUNITS-001.md`;
  DEL-08-05 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Native JSON Package
  unit-preservation evidence only. No native JSON packet schema, package
  member set, package hash basis, target writer, target compatibility posture,
  report-linter protected-content semantics, legal clearance,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-236 export-review storage/preflight unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVIEWSTORAGEUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The Project Storage Audit and Project
  Validation Preflight packets already expose DEC-018 unit-policy evidence;
  the export-review manifest now classifies those records as
  unit-evidence-required rather than metadata-only.
- App behavior: `data-testid="export-review-units"` now reports
  `covered=16/16` with `unit-system:dec-018-si-dual-display` and
  `conversion=false`. The downloaded export-review JSON includes
  `project_storage_audit` and `project_validation_preflight` in
  `unit_policy_summary.covered_export_ids`, and both rows have
  `unit_evidence_status=covered_by_target_panel_or_export_packet`.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVIEWSTORAGEUNITS-001.md`;
  DEL-02-05 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: export-review metadata inventory only. No project persistence
  semantics, validation-preflight semantics, target-specific writer,
  manifest unit conversion, runtime redaction rule, public transport
  commitment, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-237 project-validation report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Project Validation Preflight panel
  already exposes DEC-018 `unit_policy_evidence`; the report-lint public
  inventory now records that public unit-bearing surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx`
  in its explicit public target roots and target list, and adds
  `project-validation-unit-policy` to `unit_policy_evidence.target_refs`.
  The visible lint row reports `unit_targets=20`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    initially caught the missing explicit lint target entry, then passed 1/1
    selected test after the target list was updated.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
    passed 1/1 focused Chromium desktop test.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"`
    passed 1/1 selected test after an over-broad export-review metadata count
    assertion was restored.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001.md`;
  DEL-02-05 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Project Validation Preflight
  unit-policy evidence only. No validation-preflight packet semantics,
  project persistence semantics, report-linter protected-content semantics,
  legal clearance, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-238 operation-diff report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-DIFFPREVIEWLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Operation Diff Preview panel already
  exposes DEC-018 `unit_system_disclosure` and per-change unit preservation
  witnesses; the report-lint public inventory now records that public
  unit-witness surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx` in its
  explicit public target roots and target list, and adds
  `operation-diff-unit-witnesses` to `unit_policy_evidence.target_refs`. The
  visible lint row reports `unit_targets=21`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"`
    passed 1/1 selected test.
  - `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop`
    passed 1/1 focused Chromium desktop test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-DIFFPREVIEWLINTUNITS-001.md`;
  DEL-16-02 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Operation Diff Preview
  unit-witness evidence only. No diff-preview packet semantics, operation
  validation, operation application, accepted-model mutation behavior,
  report-linter protected-content semantics, legal clearance,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-239 result-export report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-RESULTEXPORTLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Result Export panel already exposes
  schema-first local JSON result envelopes with explicit result units and
  per-result unit preservation witnesses; the report-lint public inventory
  now records that public unit-witness surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/result-export/ResultExportPanel.tsx` in its
  explicit public target roots and target list, and adds
  `result-export-unit-witnesses` to `unit_policy_evidence.target_refs`. The
  visible lint row reports `unit_targets=22`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RESULTEXPORTLINTUNITS-001.md`;
  DEL-08-04 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Result Export unit-witness
  evidence only. No result-export packet semantics, result envelope schema,
  mechanics result generation, export-review manifest semantics,
  report-linter protected-content semantics, legal clearance, unit-conversion
  API, DEC-018 catalog constant, schema dimension enum, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-240 handoff report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-HANDOFFLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Handoff Package panel already exposes
  DEC-018 unit-system disclosure and per-result unit preservation witnesses
  for the local review handoff package; the report-lint public inventory now
  records that public unit-witness surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/handoff/HandoffPanel.tsx` in its explicit public
  target roots and target list, and adds `handoff-unit-witnesses` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=23`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-HANDOFFLINTUNITS-001.md`;
  DEL-15-01 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Handoff Package unit-witness
  evidence only. No handoff packet semantics, handoff schema, target mapping,
  external prover workflow, export-review manifest semantics,
  report-linter protected-content semantics, legal clearance, unit-conversion
  API, DEC-018 catalog constant, schema dimension enum, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-241 headless-runner report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-HEADLESSRUNNERLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Headless Runner panel already exposes
  DEC-018 unit-system disclosure and per-result unit preservation witnesses
  for the schema-first result handoff preview; the report-lint public
  inventory now records that public unit-witness surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx` in its
  explicit public target roots and target list, and adds
  `headless-runner-unit-witnesses` to `unit_policy_evidence.target_refs`. The
  visible lint row reports `unit_targets=24`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and 399/399
    tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-HEADLESSRUNNERLINTUNITS-001.md`;
  DEL-10-05 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Headless Runner
  unit-witness evidence only. No headless-runner packet semantics, schema,
  final CLI syntax, package scripts, process policy, network policy,
  filesystem mutation policy, report-linter protected-content semantics,
  legal clearance, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-242 adapter-framework report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-ADAPTERFRAMEWORKLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Adapter Framework panel already exposes
  DEC-018 unit-policy evidence for the format-neutral adapter declaration; the
  report-lint public inventory now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx` in
  its explicit public target roots and target list, and adds
  `adapter-framework-units` to `unit_policy_evidence.target_refs`. The visible
  lint row reports `unit_targets=25`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` initially surfaced a
    `DeclarationsEditor.test.tsx` timing failure in
    `keeps an out-of-catalog stored unit visible as current instead of snapping it`;
    the selected test then passed in isolation, and the full desktop Vitest
    rerun passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-ADAPTERFRAMEWORKLINTUNITS-001.md`;
  DEL-10-02 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Adapter Framework unit-policy
  evidence only. No adapter-framework packet semantics, schema, concrete
  external format list, public transport, plugin runtime, permission
  persistence, package scripts, CI/release matrix, report-linter
  protected-content semantics, legal clearance, unit-conversion API, DEC-018
  catalog constant, schema dimension enum, protected standards content,
  private payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-243 operation-ledger report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-OPLEDGERLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Operation Review Ledger already exposes
  visible and exported DEC-018 unit-policy evidence for review-only operation
  records; the report-lint public inventory now records that public
  unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/operations/OperationLedgerPanel.tsx` in its
  explicit public target roots and target list, and adds
  `operation-ledger-unit-policy` to `unit_policy_evidence.target_refs`. The
  visible lint row reports `unit_targets=26`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` initially surfaced an
    order-sensitive `App.test.tsx` Operation Ledger unit-validation status
    mismatch; the selected test then passed in isolation, and the full desktop
    Vitest rerun passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-OPLEDGERLINTUNITS-001.md`;
  DEL-16-03 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Operation Review Ledger
  unit-policy evidence only. No operation application, acceptance semantics,
  durable audit persistence, receipt schema, report-linter protected-content
  semantics, legal clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-244 solve-job report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-SOLVEJOBLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Solve execution panel already exposes
  visible and exported DEC-018 solve-job unit-policy evidence; the report-lint
  public inventory now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/solve/SolvePanel.tsx` in its explicit public
  target roots and target list, and adds `solve-job-unit-policy` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=27`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-SOLVEJOBLINTUNITS-001.md`;
  DEL-07-07 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Solve execution solve-job
  unit-policy evidence only. No solve execution behavior, analysis-run hash
  generation, cancellation/progress semantics, backend job behavior,
  report-linter protected-content semantics, legal clearance, redaction
  controls, target writer compatibility, unit-conversion API, DEC-018 catalog
  constant, schema dimension enum, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-245 run-audit report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-RUNAUDITLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Run Audit panel already exposes solved
  analysis-run unit traceability; the report-lint public inventory now records
  that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/run-audit/RunAuditPanel.tsx` in its explicit
  public target roots and target list, and adds `run-audit-units` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=28`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RUNAUDITLINTUNITS-001.md`;
  DEL-14-02 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Run Audit unit traceability
  only. No analysis-run schema, hash policy, solver behavior, result values,
  report-linter protected-content semantics, legal clearance, redaction
  controls, target writer compatibility, unit-conversion API, DEC-018 catalog
  constant, schema dimension enum, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-246 design-knowledge report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-KNOWLEDGELINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Design Knowledge panel already exposes
  computed result-unit context; the report-lint public inventory now records
  that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/knowledge/KnowledgePanel.tsx` in its explicit
  public target roots and target list, and adds `knowledge-unit-context` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=29`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-KNOWLEDGELINTUNITS-001.md`;
  DEL-13-01 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Design Knowledge computed-unit
  context only. No design-knowledge schema, provenance model, result values,
  solver behavior, report-linter protected-content semantics, legal clearance,
  redaction controls, target writer compatibility, unit-conversion API,
  DEC-018 catalog constant, schema dimension enum, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-247 apply-operations report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-OPAPPLYLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Apply Operations panel already exposes
  queued-operation and receipt unit-policy context; the report-lint public
  inventory now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/operations/OperationApplyPanel.tsx` in its
  explicit public target roots and target list, and adds
  `operation-apply-unit-policy` to `unit_policy_evidence.target_refs`. The
  visible lint row reports `unit_targets=30`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-OPAPPLYLINTUNITS-001.md`;
  DEL-16-03 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Apply Operations unit-policy
  context only. No operation application, acceptance semantics, durable audit
  persistence, receipt schema, operation validation, report-linter
  protected-content semantics, legal clearance, redaction controls, target
  writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-248 results-viewer report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-RESULTVIEWLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Results viewer already exposes solved
  result-unit policy context; the report-lint public inventory now records
  that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/results/ResultsPanel.tsx` in its explicit public
  target roots and target list, and adds `result-unit-policy` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=31`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RESULTVIEWLINTUNITS-001.md`;
  DEL-07-05 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Results viewer result-unit
  policy context only. No result math, result filtering, pagination
  semantics, selected-result interpretation, solver behavior, comparison
  delta math, tolerance profile, default tolerance, report-linter
  protected-content semantics, legal clearance, redaction controls, target
  writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-249 rendered-report report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-RENDEREDREPORTLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Rendered Report panel already exposes
  renderable-report unit-basis context; the report-lint public inventory now
  records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/report/RenderedReportPanel.tsx` in its explicit
  public target roots and target list, and adds
  `rendered-report-unit-basis` to `unit_policy_evidence.target_refs`. The
  visible lint row reports `unit_targets=32`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RENDEREDREPORTLINTUNITS-001.md`;
  DEL-08-01 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Rendered Report unit-basis
  context only. No report renderer route, canonical hash policy, export-gate
  policy, report content, print/PDF behavior, report-linter protected-content
  semantics, legal clearance, redaction controls, target writer
  compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-250 comparison report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-COMPARISONLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Comparison workspace already exposes
  equal-explicit-unit matching policy context; the report-lint public
  inventory now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/comparison/ComparisonPanel.tsx` in its explicit
  public target roots and target list, and adds `comparison-unit-policy` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=33`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-COMPARISONLINTUNITS-001.md`;
  DEL-14-04 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Comparison workspace
  unit-matching policy context only. No comparison delta math, result mapping,
  tolerance profile, default tolerance, solver behavior, report-linter
  protected-content semantics, legal clearance, redaction controls, target
  writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-251 run-rule-check report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-RULECHECKLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Run Rule Checks panel already exposes
  runtime value-binding unit policy context; the report-lint public inventory
  now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx` in its
  explicit public target roots and target list, and adds
  `rule-check-unit-binding-policy` to `unit_policy_evidence.target_refs`. The
  visible lint row reports `unit_targets=34`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- RuleCheckRunPanel.test.tsx`
    passed 18/18 tests.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke|run-rule-checks panel loads the demo pack"`
    passed 4/4 focused configured-project tests.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULECHECKLINTUNITS-001.md`;
  DEL-06-03 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Run Rule Checks unit-binding
  policy context only. No rule-pack schema, expression grammar,
  parser/text-syntax, backend completeness/evaluator behavior, rule-pack
  persistence, solver behavior, report-linter protected-content semantics,
  legal clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-252 diagnostic report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-DIAGNOSTICLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Diagnostics panel already exposes linked
  result unit context for selected diagnostics; the report-lint public
  inventory now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/diagnostics/DiagnosticsPanel.tsx` in its
  explicit public target roots and target list, and adds
  `diagnostic-unit-context` to `unit_policy_evidence.target_refs`. The visible
  lint row reports `unit_targets=35`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "links selected diagnostics to affected result and model context"`
    passed 1/1 selected test.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test:e2e --workspace apps/desktop -- --grep "diagnostic detail exposes linked result unit context"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-DIAGNOSTICLINTUNITS-001.md`;
  DEL-07-07 and DEL-02-02 supporting run records with the same id; completion
  log entry.
- Boundary: report-lint inventory over existing Diagnostics linked-result
  unit context only. No diagnostic schema, diagnostic interpretation behavior,
  solver behavior, result values, report-linter protected-content semantics,
  legal clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-253 rule-pack declarations report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-RULEPACKDECLLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Rule Pack Declarations editor already
  exposes required-input and value-slot unit-reference policy; the report-lint
  public inventory now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx` in its
  explicit public target roots and target list, and adds
  `rule-pack-declarations-unit-policy` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=36`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- RulePackManagerPanel.test.tsx DeclarationsEditor.test.tsx -t "unit|declares|DEC-018|rule-pack"`
    passed 5/5 selected tests.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke|rule-pack manager drafts privately"`
    passed 4/4 focused configured-project tests.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULEPACKDECLLINTUNITS-001.md`;
  DEL-07-03, DEL-06-01, and DEL-02-02 supporting run records with the same id;
  completion log entry.
- Boundary: report-lint inventory over existing Rule Pack Declarations
  unit-reference policy context only. No rule-pack schema, expression grammar,
  parser/text syntax, evaluator behavior, backend validation, persistence,
  report-linter protected-content semantics, legal clearance, redaction
  controls, target writer compatibility, unit-conversion API, DEC-018 catalog
  constant, schema dimension enum, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-254 rule-pack expression report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-RULEPACKEXPRLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Rule Pack Expression Composer already
  exposes structured-AST expression unit-reference policy; the report-lint
  public inventory now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/rule-packs/ExpressionComposer.tsx` in its
  explicit public target roots and target list, and adds
  `rule-pack-expression-unit-policy` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=37`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- ExpressionComposer.test.tsx RulePackManagerPanel.test.tsx -t "unit|declares|DEC-018|rule-pack|browser-preview"`
    passed 6/6 selected tests.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke|rule-pack manager drafts privately"`
    passed 4/4 focused configured-project tests.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "round trips local create, save, and open project controls without external file copies"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULEPACKEXPRLINTUNITS-001.md`;
  DEL-07-03, DEL-06-02, and DEL-02-02 supporting run records with the same id;
  completion log entry.
- Boundary: report-lint inventory over existing Rule Pack Expression Composer
  unit-reference policy context only. No rule-pack schema, expression grammar,
  writable text parser/syntax, evaluator behavior, backend validation,
  persistence, report-linter protected-content semantics, legal clearance,
  redaction controls, target writer compatibility, unit-conversion API,
  DEC-018 catalog constant, schema dimension enum, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-255 library manager report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-LIBRARYLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Library Manager already exposes material,
  section, and component draft unit-helper surfaces; the report-lint public
  inventory now records that public unit-policy surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/library/LibraryManagerPanel.tsx` in its explicit
  public target list, and adds `library-unit-helper-surfaces` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=38`, `conversion_witness_targets=2`, and
  `lint_conversion=false`. Static report-lint target count is 42; solved
  report packets include 43 targets after appending the generated preview
  report JSON target.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- LibraryManagerPanel.test.tsx -t "unit|library|desktop"`
    passed 9/9 selected tests.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke|library manager"`
    passed 4/4 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
  - `git diff --check` passed.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LIBRARYLINTUNITS-001.md`;
  DEL-07-03, DEL-03-01, DEL-03-02, and DEL-02-02 supporting run records with
  the same id; completion log entry.
- Boundary: report-lint inventory over existing Library Manager unit-helper
  surfaces only. No library schemas, library import storage, private-library
  payload handling, report-linter protected-content semantics, legal
  clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-256 load manager report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Load Case Manager already exposes
  load-case, primitive-load, and combination operation unit-validation
  surfaces; the report-lint public inventory now records that surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` in its
  explicit public target list, and adds
  `load-manager-unit-validation-surface` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=39`, `conversion_witness_targets=2`, and
  `lint_conversion=false`. Static report-lint target count is 43; solved
  report packets include 44 targets after appending the generated preview
  report JSON target.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "load|primitive|combination|unit_validation"`
    passed 26/26 selected tests.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
  - `git diff --check` passed.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001.md`;
  DEL-07-02, DEL-05-01, DEL-05-02, DEL-16-02, and DEL-02-02 supporting run
  records with the same id; completion log entry.
- Boundary: report-lint inventory over existing Load Case Manager
  unit-validation surfaces only. No load-case schemas, primitive-load or
  combination behavior, operation validation, operation application,
  report-linter protected-content semantics, legal clearance, redaction
  controls, target writer compatibility, unit-conversion API, DEC-018 catalog
  constant, schema dimension enum, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-257 property inspector report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Property Inspector already exposes edit,
  create, and delete operation unit-validation surfaces; the report-lint
  public inventory now records that surface.
- App behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/model-tree/PropertyInspector.tsx` in its explicit
  public target list, and adds
  `property-inspector-unit-validation-surface` to
  `unit_policy_evidence.target_refs`. The visible lint row reports
  `unit_targets=40`, `conversion_witness_targets=2`, and
  `lint_conversion=false`. Static report-lint target count is 44; solved
  report packets include 45 targets after appending the generated preview
  report JSON target.
- Validation:
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- App.test.tsx -t "queues and applies explicit (material|section|support)"`
    passed 4/4 selected tests.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001.md`;
  DEL-07-02, DEL-16-02, and DEL-02-02 supporting run records with the same
  id; completion log entry.
- Boundary: report-lint inventory over existing Property Inspector
  unit-validation surfaces only. No Property Inspector behavior, operation
  validation, operation application, diff preview, report-linter
  protected-content semantics, legal clearance, redaction controls, target
  writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
  dimension enum, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-258 rule completeness report-lint unit inventory - PASSED (`TP-UNITS-BTAIL-RULECOMPLETENESSLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The Rule-Check Completeness panel is an
  exported public review surface for missing rule-check data; it now exposes
  explicit rule-input unit-policy evidence and the report-lint public inventory
  records that surface.
- App behavior: `RuleCheckPanel` now displays and exports
  `unit_policy_evidence` with the DEC-018 unit-system ref, sorted model units,
  unit-bearing record count, explicit rule-input-unit blocking policy,
  `RULE_UNIT_MISMATCH`, and `conversion_performed=false`. `ReportLintPanel`
  now includes `apps/desktop/src/features/rule-check/RuleCheckPanel.tsx` as
  `target:desktop-rule-completeness-template` and adds
  `rule-completeness-unit-policy` to `unit_policy_evidence.target_refs`. The
  visible lint row reports `unit_targets=41`, `conversion_witness_targets=2`,
  and `lint_conversion=false`. Static report-lint target count is 45; solved
  report packets include 46 targets after appending the generated preview
  report JSON target.
- Validation:
  - `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "shows computed mechanics diagnostics in results, knowledge, and review-only proposal context"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULECOMPLETENESSLINTUNITS-001.md`;
  DEL-06-03, DEL-07-04, DEL-05-04, DEL-08-03, and DEL-02-02 supporting run
  records with the same id.
- Boundary: evidence and inventory only. No rule evaluator semantics,
  required-input completeness logic, analysis status semantics, mechanics
  solve behavior, operation application, report-linter protected-content
  semantics, legal clearance, redaction controls, target writer compatibility,
  unit-conversion API, DEC-018 catalog constant, schema dimension enum,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-259 export review rule completeness unit inventory - PASSED (`TP-UNITS-BTAIL-EXPORTREVIEWRULECOMPUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The Rule-Check Completeness review is now a
  unit-bearing exported public surface, so the Export Safety Review and Report
  packet persistence inventories record it explicitly.
- App behavior: `ExportReviewPanel` now includes `rule_completeness_review` as
  an available metadata-only local export row, marks it unit-evidence-required,
  and reports `covered=17/17` with `conversion=false`. The solved export
  review manifest now has 28 export rows. `ReportPanel` persistence export
  evidence now expects 28 local export rows and reports 27 available when the
  operation ledger is empty.
- Validation:
  - `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "shows computed mechanics diagnostics in results, knowledge, and review-only proposal context"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVIEWRULECOMPUNITS-001.md`;
  DEL-06-03, DEL-08-03, DEL-08-04, and DEL-02-02 supporting run records with
  the same id.
- Boundary: export/review inventory only. No rule-completeness semantics, rule
  evaluation, missing-input blocking behavior, runtime redaction rules,
  target-specific writers, manifest-level unit conversion, public transport
  commitments, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-260 agent proposal unit-policy inventory - PASSED (`TP-UNITS-BTAIL-AGENTPROPOSALUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Agent Proposal unit-policy inventory slice while
  C5.7 remains human-execution gated. The Agent Proposal panel is a public
  review-only operation surface; it now carries explicit metadata-only unit
  evidence and the report/export inventories record that surface.
- App behavior: `AgentProposalPanel` now exposes
  `data-testid="proposal-unit-policy"` with
  `unit_validation=not_required_metadata_review_only`, source
  `proposal.validation.unit_validation`, and `conversion=false`. Mechanics
  proposals and the invented proposal fixture carry the same validation
  status. The operation ledger preserves the status while still reporting
  `unit_bearing_changes=0`, held-for-user-acceptance review posture, and
  accepted-state mutation false.
- Inventory behavior: `ReportLintPanel` now includes
  `apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx` as
  `agent-proposal-unit-policy`, raising visible unit-policy targets to 42
  while conversion-witness targets remain two. `ExportReviewPanel` now
  includes `agent_proposal_review` as a metadata-only export row. The row is
  pending until a proposal exists, then available with
  `unit_validation_status=not_required_metadata_review_only`. Solved queued
  export-review evidence reports 29 rows and `covered=17/18`; the proposal
  path reports 29/29 exports available and 18/18 unit-evidence rows present.
- Validation:
  - `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "shows computed mechanics diagnostics in results, knowledge, and review-only proposal context"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- --run src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"`
    passed 1/1 selected test.
  - `npm run test --workspace apps/desktop -- --run src/App.test.tsx`
    passed 56/56 tests.
  - `npm run test --workspace apps/desktop` passed 18/18 files and
    399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 focused configured-project tests after stale report-lint
    target-count assertions were updated.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-16-04 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-AGENTPROPOSALUNITS-001.md`;
  DEL-16-02, DEL-16-03, DEL-08-05, DEL-12-02, DEL-08-04, and DEL-02-02
  supporting run records with the same id.
- Boundary: proposal/export/report inventory evidence only. No proposal
  application, accepted model-state mutation, operation validation semantics,
  acceptance semantics, report-linter protected-content semantics, redaction
  rules, target writer behavior, unit conversion, protected standards content,
  private payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-261 security threat-model unit-policy inventory - PASSED (`TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001`, 2026-06-18)

- Scope: bounded Phase B-tail Security Threat Model unit-policy inventory
  slice while C5.7 remains human-execution gated. The security threat-model
  review already records `unit_checks` as a no-bypass control; this tranche
  makes that unit-policy evidence visible and export-review discoverable.
- App behavior: `SecurityThreatModelPanel` now exposes
  `data-testid="security-threat-model-unit-policy"` with
  `unit_checks=true`, `workflows=6`, `conversion=false`, and
  `certification=false`. The exported threat-model packet carries
  `unit-policy-evidence:security-threat-model-no-bypass`, references DEC-018,
  and does not invoke a target writer.
- Inventory behavior: `ReportLintPanel` now includes
  `security-threat-model-unit-policy`, raising visible unit-policy targets to
  43 while conversion-witness targets remain two. `ExportReviewPanel` now
  marks `security_threat_model_review` as unit-evidence-required, raising
  solved queued-intent coverage to `covered=18/19`; proposal-path coverage is
  19/19 once the proposal row is available.
- Validation:
  - `npm test --workspace apps/desktop -- src/App.test.tsx` passed 56/56
    tests after updating stale export-review coverage expectations.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 after stale report-lint target-count assertions were updated.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-12-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001.md`;
  DEL-08-05, DEL-12-02, and DEL-02-02 supporting run records with the same
  id.
- Boundary: security/export/report inventory evidence only. No security
  sufficiency claim, telemetry authorization, redaction behavior, target
  writer behavior, unit conversion, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-262 product-preview fixture unit-policy inventory - PASSED (`TP-UNITS-BTAIL-FIXTURELINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Report Content Lint inventory slice while C5.7
  remains human-execution gated. The invented product-preview fixture bundle
  is unit-bearing public example data; this tranche makes its unit evidence
  discoverable through the public unit-policy inventory.
- App behavior: `ReportLintPanel` now inventories `fixtures/product_preview`
  as `product-preview-fixture-unit-policy`. The fixture target text records
  explicit model quantities, rule-pack unit refs, mechanics-result units, and
  the active model unit system without changing any fixture payloads.
- Inventory behavior: the visible lint unit-policy row now reports
  `unit_targets=44`, `conversion_witness_targets=2`, and
  `lint_conversion=false`.
- Validation:
  - `npm test --workspace apps/desktop -- src/App.test.tsx` passed 56/56
    tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-08-05 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-FIXTURELINTUNITS-001.md`;
  DEL-02-02 supporting run record with the same id.
- Boundary: report-lint inventory evidence only. No fixture data,
  report-linter protected-content semantics, redaction behavior, target writer
  behavior, unit conversion, protected standards content, private payload,
  lifecycle state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-263 export-review external-prover unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVEXTPROVERUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The DEL-15-04 external-prover boundary panel
  already exposes DEC-018 unit-policy evidence; this tranche classifies the
  `external_prover_boundary_metadata` export row as unit-evidence-required.
- App behavior: `ExportReviewPanel` now includes
  `external_prover_boundary_metadata` in the unit-evidence-required matrix.
  Solved queued-intent Export Review reports `covered=19/20`; the proposal
  path reports 20/20 once `agent_proposal_review` is available. The downloaded
  manifest includes `external_prover_boundary_metadata` in
  `unit_policy_summary.covered_export_ids` with
  `unit_evidence_status=covered_by_target_panel_or_export_packet`.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm test --workspace apps/desktop -- src/App.test.tsx` passed 56/56
    tests after updating the stale visible coverage assertion.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 tests.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVEXTPROVERUNITS-001.md`;
  DEL-15-04 and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory evidence only. No external solver/prover
  invocation, target parser, commercial-result ingestion, target writer,
  manifest-level unit conversion, redaction behavior, protected standards
  content, private payload, lifecycle state transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-264 export-review missing-data unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVMISSINGDATAUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The DEL-07-04 Missing Data Blocking panel
  already exposes explicit unit-input policy evidence; this tranche classifies
  the `missing_data_warning_blocking_review` export row as
  unit-evidence-required.
- App behavior: `ExportReviewPanel` now includes
  `missing_data_warning_blocking_review` in the unit-evidence-required
  matrix. The row records
  `unit_policy_ref=unit-input-policy-evidence:missing-data-warning-blocking-review`,
  `unit_evidence_required=true`, and `conversion_performed=false`. Solved
  queued-intent Export Review reports `covered=20/21`; the proposal path
  reports 21/21 once `agent_proposal_review` is available.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - `npm test --workspace apps/desktop -- src/App.test.tsx` passed 56/56
    tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
  - `npm run build --workspace apps/desktop` passed with the existing Vite
    large-chunk warning.
  - `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"`
    passed 2/2 tests.
  - `npm run test:e2e --workspace apps/desktop -- --workers=1` passed 18/18
    tests.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVMISSINGDATAUNITS-001.md`;
  DEL-07-04 and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory evidence only. No missing-data blocking
  behavior, remediation flow, accepted model mutation, rule-check execution,
  runtime redaction rule, target writer, manifest-level unit conversion,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-265 export-review editor-contract unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVEDITORCONTRACTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The DEL-07-03 Editor Contract panel already
  exposes the DEL-02-02 unit contract; this tranche classifies
  `editor_contract_review` as unit-evidence-required.
- App behavior: `ExportReviewPanel` now includes `editor_contract_review` in
  the unit-evidence-required matrix. The row records
  `unit_policy_ref=DEL-02-02:unit_bearing_values_require_explicit_unit_metadata`,
  `unit_evidence_required=true`, `missing_unit_behavior=diagnostic_blocking`,
  and `conversion_performed=false`. Solved queued-intent Export Review reports
  `covered=21/22`; the proposal path reports 22/22 once
  `agent_proposal_review` is available.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep
    evidence are recorded in closeout artifacts for this tranche.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVEDITORCONTRACTUNITS-001.md`;
  DEL-07-03 and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory evidence only. No editor validation
  behavior, durable mutation, private rule-pack payload handling,
  private-library payload handling, target writer, manifest-level unit
  conversion, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-266 export-review secret/private unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVSECRETPRIVATEUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The DEL-12-04 Secret and Private Libraries
  panel already exposes metadata-only unit policy evidence for unit-bearing
  private material, component, and rule-pack references; this tranche
  classifies `secret_private_library_boundary_review` as
  unit-evidence-required.
- App behavior: `ExportReviewPanel` now includes
  `secret_private_library_boundary_review` in the unit-evidence-required
  matrix. The row records
  `unit_policy_ref=unit-policy:secret-private-library-metadata-only-preview`,
  `unit_evidence_required=true`, `explicit_unit_metadata_required=true`,
  `unit_payload_included=false`, and `conversion_performed=false`. Solved
  queued-intent Export Review reports `covered=22/23`; the proposal path
  reports 23/23 once `agent_proposal_review` is available.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep
    evidence are recorded in closeout artifacts for this tranche.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVSECRETPRIVATEUNITS-001.md`;
  DEL-12-04 and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory evidence only. No secret/private-library
  payload handling, credential-value handling, encryption/key-management
  decision, cloud or network behavior, external secret-manager behavior,
  target writer, manifest-level unit conversion, protected standards content,
  private payload, lifecycle state transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-267 export-review accessibility unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVA11YUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The DEL-07-06 Accessibility Baseline panel
  already exposes unit-visibility evidence for unit-bearing review surfaces;
  this tranche classifies `accessibility_usability_baseline_review` as
  unit-evidence-required.
- App behavior: `ExportReviewPanel` now includes
  `accessibility_usability_baseline_review` in the unit-evidence-required
  matrix. The row records
  `unit_policy_ref=unit-visibility-evidence:accessibility-baseline-preview`,
  `unit_evidence_required=true`, `default_units_inferred=false`, and
  `conversion_performed=false`. Solved queued-intent Export Review reports
  `covered=23/24`; the proposal path reports 24/24 once
  `agent_proposal_review` is available.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep
    evidence are recorded in closeout artifacts for this tranche.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVA11YUNITS-001.md`;
  DEL-07-06 and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory evidence only. No accessibility finding
  count, conformance target selection, runtime accessibility evaluation,
  color signaling policy, target writer, manifest-level unit conversion,
  protected standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-268 export-review design-workspace unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVDESIGNWORKSPACEUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The DEL-07-08 Design Workspace panel already
  exposes unit-policy evidence for composed model/result/analysis-run/
  comparison context; this tranche classifies
  `design_authoring_comparison_workspace` as unit-evidence-required.
- App behavior: `ExportReviewPanel` now includes
  `design_authoring_comparison_workspace` in the unit-evidence-required
  matrix. The row records
  `unit_policy_ref=unit-policy-evidence:design-workspace-preview`,
  `unit_evidence_required=true`, `default_units_inferred=false`, and
  `conversion_performed=false`. Solved queued-intent Export Review reports
  `covered=24/25`; the proposal path reports 25/25 once
  `agent_proposal_review` is available.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep
    evidence are recorded in closeout artifacts for this tranche.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVDESIGNWORKSPACEUNITS-001.md`;
  DEL-07-08 and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory evidence only. No design-workspace
  composition behavior, comparison delta math, tolerance profile, operation
  application, accepted model-state mutation, target writer, manifest-level
  unit conversion, protected standards content, private payload, lifecycle
  state transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.

## TP-MAC-269 export-review validation-evidence unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVVALIDATIONEVIDUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The Validation Evidence panel already
  exposes unit-policy evidence for project unit context and the validation
  manual unit/schema section; this tranche classifies
  `validation_release_evidence_review` as unit-evidence-required.
- App behavior: `ExportReviewPanel` now includes
  `validation_release_evidence_review` in the unit-evidence-required matrix.
  The row records
  `unit_policy_ref=unit-policy-evidence:validation-release-evidence-review`,
  `unit_evidence_required=true`, `default_units_inferred=false`, and
  `conversion_performed=false`. Solved queued-intent Export Review reports
  `covered=25/26`; the proposal path reports 26/26 once
  `agent_proposal_review` is available.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep
    evidence are recorded in closeout artifacts for this tranche.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVVALIDATIONEVIDUNITS-001.md`;
  DEL-09-04, DEL-09-05, and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory evidence only. No validation manual
  content, release threshold, release authorization, validation-evidence
  storage decision, target writer, manifest-level unit conversion, protected
  standards content, private payload, lifecycle state transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-270 export-review report-lint unit evidence - PASSED (`TP-UNITS-BTAIL-EXPORTREVREPORTLINTUNITS-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. The Report Content Lint packet already
  exposes public-surface unit-policy evidence; this tranche classifies
  `report_protected_content_lint` as unit-evidence-required.
- App behavior: `ExportReviewPanel` now includes
  `report_protected_content_lint` in the unit-evidence-required matrix. The
  row records
  `unit_policy_ref=unit-policy-evidence:report-lint-public-surfaces`,
  `unit_evidence_required=true`, `unit_policy_target_count=44`,
  `conversion_witness_target_count=2`, `default_units_inferred=false`, and
  `conversion_performed=false`. Solved queued-intent Export Review reports
  `covered=26/27`; the proposal path reports 27/27 once
  `agent_proposal_review` is available.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep
    evidence are recorded in closeout artifacts for this tranche.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVREPORTLINTUNITS-001.md`;
  DEL-08-05 and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory evidence only. No report-linter
  protected-content semantics, legal clearance, redaction certification,
  release authorization, runtime redaction rule, target writer,
  manifest-level unit conversion, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-271 export-review non-unit boundary classification - PASSED (`TP-UNITS-BTAIL-EXPORTREVNONUNITBOUNDARY-001`, 2026-06-18)

- Scope: bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
  remains human-execution gated. This tranche records explicit non-unit-bearing
  reasons for `telemetry_boundary_review` and `build_package_readiness`.
- App behavior: `ExportReviewPanel` now carries
  `unit_boundary_classification=not_unit_bearing_metadata_or_boundary_review`
  and per-row `unit_boundary_reason` values for the telemetry and build
  readiness rows. The matrix asserts
  `not_unit_bearing_export_ids=["telemetry_boundary_review","build_package_readiness"]`.
  Unit evidence counts remain `covered=26/27` in the solved queued-intent path
  and 27/27 in the proposal path once `agent_proposal_review` is available.
- Validation:
  - `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"`
    passed 1/1 selected test.
  - Full App, full desktop Vitest, build, Playwright, and DEC-025 sweep
    evidence are recorded in closeout artifacts for this tranche.
- Evidence: DEL-12-02 primary run record
  `WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVNONUNITBOUNDARY-001.md`;
  DEL-12-03, DEL-10-04, and DEL-02-02 supporting run records with the same id.
- Boundary: export-review inventory classification only. No telemetry runtime
  behavior, network behavior, payload construction, build script, CI provider,
  release matrix, signing, packaging, target writer, manifest-level unit
  conversion, protected standards content, private payload, lifecycle state
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-272 workspace redesign Inc 0-5 - PASSED (`TP-R3UX-WORKSPACEREDESIGN-001`, 2026-06-19)

- Scope: C5.7R minimum redesign before the next packaged human pass attempt.
  Replaces the three competing navigation systems with one ribbon; demotes
  evidence, telemetry, storage, and boundary strings into the Audit drawer;
  moves diagnostics and missing-input blockers into the Issues drawer; enlarges
  the viewport into the hero canvas; and adds inspector dual-unit/provenance/
  required-field context.
- App behavior: default shell now renders a title bar, ribbon, tree/canvas/
  inspector workspace, status bar, Audit drawer, and Issues drawer. The
  viewport exposes axis triad, view cube controls, scale bar, load glyph
  summary, selection handles, and a command/selection bar. Operation Apply
  raw summaries were replaced with compact chips and human-readable receipts.
- Validation:
  - `npm test --workspace apps/desktop` passed 18/18 test files and 399/399
    tests.
  - `npm run test:e2e --workspace apps/desktop` passed 18/18 Playwright
    checks across desktop and compact Chromium projects.
  - `npm run test:e2e:dist --workspace apps/desktop` passed 1/1 packaged
    production smoke check.
  - `npm run build --workspace apps/desktop` passed, retaining the existing
    Vite chunk-size warning.
  - `python3 tools/release/run_evidence_sweep.py --execute` passed overall;
    summary:
    `validation/evidence/sweeps/SWEEP_20260619T072613Z_5dbd406023ba-dirty.json`.
- Evidence: DEL-07-06 run record
  `WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-WORKSPACEREDESIGN-001.md`.
- Boundary: frontend shell/presentation/test coverage only. No solver,
  schema, evaluator, persistence, backend API, unit storage, protected-content,
  private-data, network/telemetry, lifecycle, release-readiness, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed. Inc 6 bulk tabular grid and the packaged human TP-MAC-189 re-pass
  remain residuals.

## TP-MAC-273 workspace redesign Inc 6 layout grid - PASSED (`TP-R3UX-GRIDMODE-001`, 2026-06-19)

- Scope: C5.7R Inc 6 committed follow-through. Adds the CAEPIPE-style bulk
  tabular Grid mode to the left layout region while preserving Tree mode.
- App behavior: the Model pane now has Tree/Grid segmented controls. Grid
  mode exposes editable tables for nodes, pipes, supports, materials,
  sections, components, load cases, and combinations. Editing cells records
  local drafts; "Queue changed cells" fans each changed cell into an existing
  structured `EditorOperationIntent` with no new backend contract.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 57/57 tests.
  - `npm test --workspace apps/desktop` passed 18/18 files and 400/400 tests.
  - `npm run build --workspace apps/desktop` passed, retaining the existing
    Vite chunk-size warning.
  - `npm run test:e2e --workspace apps/desktop` passed 18/18 Playwright
    checks across desktop and compact Chromium projects.
  - `npm run test:e2e:dist --workspace apps/desktop` passed 1/1 packaged
    production smoke check.
  - `python3 tools/release/run_evidence_sweep.py --execute` passed overall;
    summary:
    `validation/evidence/sweeps/SWEEP_20260619T144814Z_48083bd29407-dirty.json`.
- Evidence: DEL-07-06 run record
  `WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-GRIDMODE-001.md`.
- Boundary: frontend shell/authoring ergonomics/test coverage only. No solver,
  schema, evaluator, persistence, backend API, unit storage, protected-content,
  private-data, network/telemetry, lifecycle, release-readiness, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed. Inc 7 packaged build/pass prep and the human TP-MAC-189 re-pass
  remain residuals.

## TP-MAC-274 C5.7R packaged re-pass kit - PREPARED (`TP-R3UX-PACKAGEKIT-002`, 2026-06-19)

- Scope: C5.7R Inc 7 package build and human-pass handoff after Inc 0-6.
- Build: `cd apps/desktop && npm run tauri -- build --bundles app` passed at
  commit `60fb533fe`, regenerating the WASM operation engine and producing:
  `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
  Bundle size: `12M`.
- Boot probe: launched the bundled executable for 8 seconds, observed process
  state `SN`, recorded `stdout_bytes=0` and `stderr_bytes=0`, and terminated
  cleanly.
- Validation basis: `npm run test:e2e:dist --workspace apps/desktop` passed
  1/1 earlier in this Inc 6/7 closeout; DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260619T144814Z_48083bd29407-dirty.json`
  passed overall.
- Human execution pending: run TP-MAC-189 against the rebuilt bundle and record
  pass/fail per step, performed-by, date, binary commit `60fb533fe`, bundle
  path, report hash when generated, and notes.
- Boundary: package/evidence/handoff only. No lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-275 C5.7R object-creation toolbar and rebuilt app - PASSED (`TP-R3UX-CREATIONTOOLS-001`, 2026-06-20)

- Scope: C5.7R tranche 2c. Insert commands now arm object-creation tools
  instead of only navigating/collapsing panels. The viewport command bar exposes
  Node, Pipe, Support, Component, and Load tools with active state and separates
  arming from queueing.
- App behavior: in-DOM and native Insert commands dispatch through the same
  `runMenuCommand` sink. Node/Pipe/Component preview intents queue only after
  `Queue preview`; explicit node/pipe forms, Inspector support creation, and
  Load Cases creation continue through existing structured-operation surfaces.
  Empty-canvas node coordinate capture now requires the Node tool to be armed;
  Pipe arming starts endpoint-pick mode.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 57/57 tests.
  - `npm test --workspace apps/desktop` passed 19/19 test files and 406/406
    tests.
  - `npm run build --workspace apps/desktop` passed, retaining the existing
    Vite chunk-size warning.
  - `npm run test:e2e --workspace apps/desktop` passed 18/18 Playwright checks
    across desktop and compact Chromium projects.
  - `npm run test:e2e:dist --workspace apps/desktop` passed 1/1 production
    dist smoke check.
  - `cargo check --release` in `apps/desktop/src-tauri` passed.
  - `cd apps/desktop && npm run tauri -- build --bundles app` passed,
    regenerating the WASM operation engine and producing:
    `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
    Bundle size: `12M`.
- Boot probe: launched the bundled executable for 8 seconds, observed process
  state `SN`, recorded `stdout_bytes=0` and `stderr_bytes=0`, and terminated
  cleanly.
- Evidence: DEL-07-06 run record
  `WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-CREATIONTOOLS-001.md`.
- Human execution pending: run TP-MAC-189 against the rebuilt bundle and record
  pass/fail per step, performed-by, date, binary commit, bundle path, report
  hash when generated, and notes. F-4 and A3 remain open until that human
  record passes or records the next bounded repair.
- Boundary: frontend/menu interaction, tests, and package evidence only. No
  solver, schema, evaluator, persistence, backend API, unit storage,
  protected-content, private-data, network/telemetry, lifecycle,
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-276 C5.7 human bypass and agent-first viewport shell - PASSED (`TP-R3UX-AGENTSHELL-001`, 2026-06-20)

- Scope: human-directed bypass/redirect of the pending C5.7 packaged re-pass.
  The prior criterion was not passed. The human ruled that the primary screen
  must become a 3D model and visual-interaction surface, not a heavily
  user-laden workflow/data-entry screen; the missing primary affordance is an
  agent panel because the agent is expected to manage data, workflows, and
  model redesign from user inputs.
- App behavior: the model tree and inspector now default to collapsed rails,
  entity selection no longer forces the inspector open, and the primary
  workspace includes a persistent local `Design Agent` workbench. The agent
  panel exposes selected target, queued/applied operation counts, professional
  boundary, review-only proposal status, mechanics run, proposal generation
  after mechanics results exist, and Operations/Results openers. It is a local
  deterministic workbench over existing app state, not a live agent runtime.
- Layout evidence: the dock-collapsed short-screen layout now preserves the
  primary model/agent surface instead of shrinking the canvas below its fixed
  frame. Visual probe assets are stored under DEL-07-06 `_run_records/assets/`:
  `TP-R3UX-AGENTSHELL-001_1440x920.png`,
  `TP-R3UX-AGENTSHELL-001_1280x800.png`,
  `TP-R3UX-AGENTSHELL-001_1440x920_canvas.png`,
  `TP-R3UX-AGENTSHELL-001_1280x800_canvas.png`, and
  `TP-R3UX-AGENTSHELL-001_probe.json`. The probe recorded no horizontal
  overflow, collapsed tree/inspector rails by default, visible agent panel,
  contained canvas, and nonblank canvas pixel samples at 1440x920 and 1280x800.
- Validation:
  - `npm test --workspace apps/desktop` passed 19/19 test files and 406/406
    tests.
  - `npm run build --workspace apps/desktop` passed, retaining the existing
    Vite chunk-size warning.
  - `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop` passed
    18/18 Playwright checks across desktop and compact Chromium projects.
  - `npm run test:e2e:dist --workspace apps/desktop` passed 1/1 production
    dist smoke check.
  - `cargo check --release` in `apps/desktop/src-tauri` passed.
  - `cd apps/desktop && npm run tauri -- build --bundles app` passed,
    regenerating the WASM operation engine and producing:
    `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
- Boot probe: launched the bundled executable for 8 seconds, observed it still
  running, recorded `stdout_bytes=0` and `stderr_bytes=0`, and terminated it.
- Evidence: DEL-07-06 run record
  `WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-AGENTSHELL-001.md`.
- Human status: TP-MAC-189 / the old C5.7 packaged pass criterion is bypassed
  by human direction, not passed. F-4 and A3 remain open under the prior
  `DEC-035` vocabulary until the human accepts a replacement C5.7 closure
  criterion or records a new gate.
- Boundary: local deterministic review-only UI workbench only. No live
  embedded-agent runtime, external SDK/harness consumption, autonomous accepted
  model-state mutation, solver change, schema change, evaluator grammar
  change, persistence contract change, protected-content source, private-data
  write path, network/telemetry feature, lifecycle transition, R3 exit review,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## TP-MAC-277 primary canvas contextual authoring drawer - PASSED (`TP-R3UX-PRIMARYCANVAS-001`, 2026-06-20)

- Scope: follow-up to the 2026-06-20 human screenshot feedback that the
  agent-first shell was a step in the right direction but still showed too
  much manual data-entry chrome on the primary screen. This tranche removes
  idle Node/Pipe forms from the viewport and makes them contextual to the
  active creation tool.
- App behavior: `Viewport editor intents` is visually collapsed by default when
  no Node/Pipe tool is active and no viewport intents are queued. Arming Node
  reveals only the explicit node geometry drawer. Arming Pipe reveals only the
  straight-pipe connectivity drawer. Component preview, Support, and Load
  continue through their existing review-only/structured surfaces.
- Layout evidence: visual probe assets are stored under DEL-07-06
  `_run_records/assets/`:
  `TP-R3UX-PRIMARYCANVAS-001_2048x1200_idle.png`,
  `TP-R3UX-PRIMARYCANVAS-001_2048x1200_node_armed.png`, and
  `TP-R3UX-PRIMARYCANVAS-001_probe.json`. The probe recorded idle
  `viewport-intents collapsed`, node/pipe forms hidden, no horizontal overflow,
  and a 1592x854 canvas at 2048x1200; after arming Node, only the node form was
  visible.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 57/57 tests.
  - Focused Playwright over shell, broad preview smoke, viewport gesture, and
    from-blank journeys passed 8/8 after one stale endpoint-pick smoke
    assumption was corrected.
  - `npm test --workspace apps/desktop` passed 19/19 test files and 406/406
    tests.
  - `npm run build --workspace apps/desktop` passed, retaining the existing
    Vite chunk-size warning.
  - `PLAYWRIGHT_WORKERS=1 npm run test:e2e --workspace apps/desktop` passed
    18/18 Playwright checks across desktop and compact Chromium projects.
  - `npm run test:e2e:dist --workspace apps/desktop` passed 1/1 production
    dist smoke check.
  - `cd apps/desktop && npm run tauri -- build --bundles app` passed,
    regenerating the WASM operation engine and producing:
    `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
- Boot probe: launched the bundled executable for 8 seconds, observed it still
  running, recorded `stdout_bytes=0` and `stderr_bytes=0`, and terminated it.
- Evidence: DEL-07-06 run record
  `WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-PRIMARYCANVAS-001.md`.
- Boundary: viewport presentation/contextual drawer/tests/evidence only. No
  live embedded-agent runtime, external SDK/harness consumption, autonomous
  accepted model-state mutation, backend operation kind, solver change, schema
  change, evaluator grammar change, persistence contract change,
  protected-content source, private-data write path, network/telemetry feature,
  lifecycle transition, R3 exit review, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-278 R4 D1 bend visibility/provenance slice - PASSED (`TP-R4-D1-BENDVIS-001`, 2026-06-20)

- Scope: first partial R4/D1 app-absorption slice for bend objects under
  `DEC-045`. This makes invented bend geometry and user-entered modifier values
  visible and reviewable in the technical preview without closing D1 mechanics
  or report-provenance work.
- App behavior: `component:C-110` in the invented preview model now carries
  user-entered bend radius, bend angle, bend-plane orientation, geometry source,
  user SIF, user flexibility factor, modifier source, completeness evidence,
  and `mechanics_geometry_only` / `user_rule_pack_inputs_only` labels. The
  viewport renders bend/elbow components as a curved glyph. Model view,
  inspector/provenance rows, grid mode, editor contract, missing-data blockers,
  rule-check diagnostics, validation evidence, and native-package witnesses now
  surface those bend fields.
- Validation:
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 57/57 tests.
  - `npm test --workspace apps/desktop` passed 19/19 test files and 406/406
    tests.
  - `npm run build:desktop` passed, retaining the existing Vite chunk-size
    warning.
  - Full DEC-025 evidence sweep passed:
    `validation/evidence/sweeps/SWEEP_20260621T202442Z_4829dea6c2e0-dirty.json`
    (`overall_status=pass`, 5/5 surfaces).
  - `npm run test:e2e:desktop` passed 18/18 Playwright checks.
- Evidence: DEL-07-06 run record
  `WORKING_ITEMS_RUN_2026-06-20_TP-R4-D1-BENDVIS-001.md`; DEL-03-03 run record
  with the same tranche id; plan log entry in `plans/PLAN_COMPLETION_LOG.md`.
- Residual: D1 remains partially open for any governed mechanics/stress-recovery
  multiplier consumption and downstream report-provenance closure.
- Boundary: invented/user-entered preview values only. No protected standards
  table, code-derived component factor, proprietary catalog value, private data,
  solver/kernel behavior, rule-engine grammar, persistence contract, network
  path, telemetry feature, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-279 R4 D1 bend stress-recovery and report provenance slice - PASSED (`TP-R4-D1-BENDSTRESS-001`, 2026-06-21)

- Scope: second R4/D1 bend-object app-absorption slice under `DEC-045`,
  closing the D1 mechanics/stress-recovery and preview report-provenance
  residual for the invented bend path. This remains multiplier-first
  `mechanics_geometry_only` behavior: all component factors are user-entered
  and no protected/default engineering factors are bundled.
- App/core behavior: `core/product_physics` now parses preview `components`,
  validates bend/elbow geometry and dimensionless user-entered SIF/flexibility
  values, and emits `component_user_stress_multiplier_review` result rows for
  adjacent bend endpoints. `component:C-110` produces rows for `pipe:P-100`
  end-j and `pipe:P-110` end-i; the base pipe stress rows and frame stiffness
  remain unchanged. The regenerated invented mechanics fixture now has 743
  result rows and `COMPONENT_STRESS_MULTIPLIER_APPLIED` info diagnostics.
- Report behavior: the report packet displays component provenance and
  component stress modifier summaries, and the JSON export includes
  `component_provenance`, `component_stress_modifier_evidence`, and
  `component_stress_modifier_count` with no-private/no-protected/no-claim
  boundary flags.
- Validation:
  - `cargo test --manifest-path core/product_physics/Cargo.toml` passed 32/32
    unit tests.
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 57/57 tests.
  - `npm test --workspace apps/desktop` passed 19/19 test files and 406/406
    tests.
  - `npm run build:desktop` passed, retaining the existing Vite chunk-size
    warning.
  - `npm run test:e2e:desktop` passed 18/18 Playwright checks.
- Evidence: DEL-03-03, DEL-05-03, and DEL-08-03 run records
  `WORKING_ITEMS_RUN_2026-06-21_TP-R4-D1-BENDSTRESS-001.md`; plan log entry in
  `plans/PLAN_COMPLETION_LOG.md`.
- Residual: D1 bend-object app absorption is landed for the invented preview
  path. D2 branch objects are the next ordinary unblocked Phase D component
  item. Broader D8/D9 R4 report/validation evidence remains open.
- Boundary: invented/user-entered preview values only. No protected standards
  table, code-derived component factor, proprietary catalog value, private data,
  frame-stiffness behavior change, network path, telemetry feature, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-280 R4 D2 branch connection stress-recovery and report provenance slice - PASSED (`TP-R4-D2-BRANCHSTRESS-001`, 2026-06-21)

- Scope: R4/D2 branch-object app-absorption slice under `DEC-045`, landing the
  invented preview branch geometry/provenance path plus side-specific
  user-entered stress modifier evidence. This remains multiplier-first
  `mechanics_geometry_only` behavior: all branch factors are user-entered and
  no protected/default engineering factors are bundled.
- App/core behavior: `core/product_physics` now parses branch geometry fields,
  branch header/branch pipe mappings, user-entered header and branch SIFs,
  user-entered flexibility factor, and source references. The invented preview
  model carries `component:C-120`; the viewport renders branch components as a
  T-shaped marker. Model view, inspector/provenance rows, grid/search, editor
  contract, missing-data blockers, rule-check diagnostics, validation evidence,
  native-package witnesses, and report exports now surface the branch fields.
- Mechanics/report behavior: `component:C-120` emits
  `component_user_stress_multiplier_review` rows for `pipe:P-110` end-j
  (branch side) and `pipe:P-120` end-i (header side), including both authored
  load cases and combination rows. The regenerated mechanics fixture now has
  749 result rows and 12 component multiplier evidence rows including
  combinations. Report export records include branch geometry source and
  user-entered header/branch SIF provenance.
- Validation:
  - `cargo test --manifest-path core/product_physics/Cargo.toml` passed 33/33
    unit tests.
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 57/57 tests.
  - `npm test --workspace apps/desktop` passed 19/19 test files and 406/406
    tests.
  - `npm run build:desktop` passed, retaining the existing Vite chunk-size
    warning.
  - `npm run test:e2e:desktop` passed 18/18 Playwright checks.
- Evidence: DEL-03-04, DEL-05-03, and DEL-08-03 run records
  `WORKING_ITEMS_RUN_2026-06-21_TP-R4-D2-BRANCHSTRESS-001.md`; plan log entry
  in `plans/PLAN_COMPLETION_LOG.md`.
- Residual: D2 branch-object app absorption is landed for the invented preview
  path. The PRD §16.2 branch-assembly benchmark remains D9 validation
  evidence. D3 rigid/semi-rigid components are the next ordinary unblocked
  Phase D component item.
- Boundary: invented/user-entered preview values only. No protected standards
  table, code-derived branch factor, proprietary catalog value, private data,
  frame-stiffness behavior change, network path, telemetry feature, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-281 R4 D3 rigid/semi-rigid component visibility and provenance slice - PASSED (`TP-R4-D3-RIGIDVIS-001`, 2026-06-21)

- Scope: R4/D3 rigid/semi-rigid component app-absorption slice under
  `DEC-045`, landing invented preview evidence for a valve-family rigid
  component with pipe mapping, user-entered dimensions, weight, center of
  gravity, stiffness/scaling values, and source notes. This is
  `mechanics_geometry_only` evidence: no frame-stiffness or solve-result
  behavior changed.
- App/core behavior: `core/product_physics` now parses and validates D3 rigid
  component geometry/modifier fields and emits `RIGID_COMPONENT_*` diagnostics.
  The invented preview model carries `component:C-130`; the viewport renders
  rigid/semi-rigid components with a distinct compact valve-like glyph. Model
  view, inspector/provenance rows, grid/search, editor contract, missing-data
  blockers, rule-check diagnostics, validation evidence, native-package
  witnesses, and report exports now surface the D3 rigid fields.
- Validation:
  - `cargo test --manifest-path core/product_physics/Cargo.toml` passed 33/33
    unit tests.
  - `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py`
    passed 20/20 tests.
  - `npm test --workspace apps/desktop -- App.test.tsx` passed 57/57 tests.
  - `npm test --workspace apps/desktop` passed 19/19 test files and 406/406
    tests.
  - `npm run build:desktop` passed, retaining the existing Vite chunk-size
    warning.
  - Focused Chromium R2 desktop preview smoke passed 1/1.
  - `npm run test:e2e:desktop` passed 18/18 Playwright checks.
- Evidence: DEL-03-05 and DEL-08-03 run records
  `WORKING_ITEMS_RUN_2026-06-21_TP-R4-D3-RIGIDVIS-001.md`; plan log entry in
  `plans/PLAN_COMPLETION_LOG.md`.
- Residual: D3 rigid/semi-rigid component app absorption is landed for the
  invented preview path. Full rigid macro-element solve behavior remains out of
  scope for this slice; D4 expansion joints are the next ordinary unblocked
  Phase D component item.
- Boundary: invented/user-entered preview values only. No protected standards
  table, code-derived rigid component factor, proprietary catalog value,
  private data, frame-stiffness behavior change, network path, telemetry
  feature, lifecycle transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.

## TP-MAC-282 R4 D4 expansion-joint user-stiffness preview slice - PASSED (`TP-R4-D4-EJSTIFF-001`, 2026-06-21)

- Scope: R4/D4 expansion-joint app-absorption slice under `DEC-045`, landing
  invented preview evidence for an expansion joint with pipe mapping,
  user-entered effective pressure area, movement limit, hardware/manufacturer
  provenance, pressure-thrust reference, and axial/lateral/angular/torsional
  stiffness values. This is dedicated user-stiffness macro-element review
  evidence; it does not claim a full assembled global macro-element solve or
  pressure-thrust load generation.
- App/core behavior: `core/product_physics` now parses and validates D4
  expansion-joint geometry/modifier fields, normalizes area/movement/stiffness
  units, emits `EXPANSION_JOINT_*` diagnostics, and appends four
  `component_user_stiffness_macro_element_review` rows for `component:C-150`.
  The regenerated mechanics fixture now has 753 result rows and
  `summary.component_user_stiffness_macro_element_count = 4`.
- UI/report behavior: the invented preview model carries `component:C-150`;
  the viewport renders expansion joints with a distinct bellows-style marker.
  Model view, inspector/provenance rows, grid/search, validation evidence,
  native-package witnesses, and report exports now surface the D4
  expansion-joint fields and user-stiffness review rows.
- Validation:
  - `cargo test --manifest-path core/product_physics/Cargo.toml` passed 34/34
    unit tests.
  - `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py`
    passed 20/20 tests.
  - `npm --prefix apps/desktop test -- --run src/App.test.tsx` passed 57/57
    tests.
  - `npm --prefix apps/desktop test -- --run` passed 19/19 test files and
    406/406 tests.
  - `npm --prefix apps/desktop run build` passed, retaining the existing Vite
    chunk-size warning.
  - `npm --prefix apps/desktop run build:wasm` passed.
  - From `apps/desktop`, `PLAYWRIGHT_WORKERS=1 npm exec playwright test e2e/r2-smoke.spec.ts --workers=1`
    passed 18/18 Playwright checks.
- Evidence: DEL-03-06 and DEL-08-03 run records
  `WORKING_ITEMS_RUN_2026-06-21_TP-R4-D4-EJSTIFF-001.md`; plan log entry in
  `plans/PLAN_COMPLETION_LOG.md`.
- Residual: D4 expansion-joint app absorption is landed for the invented
  preview path. Full assembled nonlinear solve behavior, pressure-thrust load
  application, and R4 validation benchmarks remain out of scope for this
  slice. The next Phase D item is D5, gated by `D-15`.
- Boundary: invented/user-entered preview values only. No protected standards
  table, code-derived expansion-joint factor, proprietary catalog value,
  private data, pressure-thrust load generation, global nonlinear solve,
  network path, telemetry feature, lifecycle transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## TP-MAC-283 R4 D5 spring-hanger user-data preview slice - PASSED (`TP-R4-D5-HANGERDATA-001`, 2026-06-21)

- Scope: R4/D5 spring-hanger app-absorption slice under `DEC-049`, landing
  invented preview evidence for a variable spring hanger and a constant-effort
  support with user-entered stiffness/load/travel fields, source/manufacturer
  provenance, load-side review reference, and mechanics-consumption notes. This
  is user-data/review evidence only; it does not claim catalog sizing, hidden
  defaults, full constant-effort solve behavior, or professional acceptance.
- App/core behavior: `schemas/model.schema.yaml` now includes hanger-specific
  support slots; the invented preview model carries `support:SH-140` and
  `support:CE-120`. `core/product_physics` validates hanger units and required
  user data, consumes variable spring hanger stiffness through the existing
  linear spring primitive, skips constant-effort supports as review/load-side
  evidence, and emits `spring_hanger_user_input_review` plus
  `constant_effort_user_input_review` result rows.
- UI/report behavior: support family/hanger fields now appear in the model
  tree/grid/search and selected-support properties. Project validation,
  native-package unit preservation, report export packets, and rendered-report
  input provenance include the spring-hanger user-data evidence.
- Validation:
  - `python3 -m json.tool fixtures/product_preview/invented_preview_model.json`
    passed.
  - `npm run generate:product-preview-mechanics` passed.
  - `python3 -m json.tool fixtures/product_preview/invented_mechanics_result.json`
    passed.
  - `cargo fmt --manifest-path core/product_physics/Cargo.toml` passed.
  - `cargo test --manifest-path core/product_physics/Cargo.toml` passed 43/43
    unit tests.
  - `npm test --workspace apps/desktop` passed 19/19 test files and 407/407
    tests.
  - From `apps/desktop`, `npm exec -- playwright test e2e/r2-smoke.spec.ts --workers=1`
    passed 18/18 Playwright checks after updating fixture-count expectations
    for the 800-row D5 mechanics result surface.
  - `npm run build:desktop` passed, retaining the existing Vite chunk-size
    warning.
- Evidence: DEL-04-03 run record
  `WORKING_ITEMS_RUN_2026-06-21_TP-R4-D5-HANGERDATA-001.md`; plan log entry in
  `plans/PLAN_COMPLETION_LOG.md`.
- Residual: D5 is landed for the minimal user-data path. Catalog sizing,
  protected/default support values, hidden manufacturer data, full
  constant-effort solve behavior, and R4 validation/exit evidence remain out of
  scope for this slice. The next Phase D item is D7 sparse evidence-lane
  adoption under `DEC-050`.
- Boundary: invented/user-entered preview values only. No protected standards
  table, code-derived spring-hanger factor, proprietary catalog value, private
  data, constant-effort solve claim, network path, telemetry feature, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## TP-MAC-284 R4 D7 sparse evidence lane - PASSED (`TP-R4-D7-SPARSELIVE-001`, 2026-06-21)

- Scope: R4/D7 sparse evidence-lane slice under `DEC-050`, binding the accepted
  `core/solver/sparse_direct` solver as a live non-blocking evidence observer
  while dense remains the default product solve path and parity oracle.
- App/core behavior: `core/solver/nonlinear_integration` records sparse
  sidecar evidence per iteration, and `core/product_physics` emits two unitless
  `sparse_live_path_dense_parity_relative_delta` rows for the invented preview
  load cases. Sparse evidence rows are not load effects and are excluded from
  combination algebra.
- UI/report behavior: generated mechanics results now contain 802 rows; result
  filtering, unit policy surfaces, native package witnesses, headless/stress
  neutral/handoff witnesses, and Playwright smoke expectations match the
  updated result surface.
- Validation:
  - `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check`
    passed.
  - `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`
    passed 10/10 tests.
  - `cargo fmt --manifest-path core/product_physics/Cargo.toml --check`
    passed.
  - `cargo test --manifest-path core/product_physics/Cargo.toml` passed 43/43
    tests.
  - `cargo fmt --manifest-path core/solver/diagnostics/Cargo.toml --check`
    passed.
  - `cargo test --manifest-path core/solver/diagnostics/Cargo.toml` passed
    24/24 tests.
  - `cargo fmt --manifest-path core/solver/performance_harness/Cargo.toml --check`
    passed.
  - `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
    passed 18/18 tests.
  - `npm test --workspace apps/desktop` passed 19/19 test files and 407/407
    tests.
  - From `apps/desktop`,
    `npm exec -- playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18
    Playwright checks.
  - Full DEC-025 evidence sweep passed:
    `validation/evidence/sweeps/SWEEP_20260621T205711Z_c771567ed6a8-dirty.json`
    (`overall_status=pass`, 5/5 surfaces).
- Evidence: DEL-04-01 and DEL-04-05 run records
  `WORKING_ITEMS_RUN_2026-06-21_TP-R4-D7-SPARSELIVE-001.md`; plan log entry in
  `plans/PLAN_COMPLETION_LOG.md`.
- Residual: D7 is landed for evidence-lane adoption only. Profile-direct sparse
  assembly, sparse-default promotion, timing/memory thresholds, practical-size
  bands, non-seed convergence thresholds, and the remaining D9 validation
  package remain follow-on work.
- Boundary: invented/local evidence only. No protected standards content,
  proprietary benchmark model, private data, network path, telemetry feature,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.
