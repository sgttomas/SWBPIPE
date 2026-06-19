# Plan Completion Log

Archive ledger for landed items from the active completion plan (currently
[PLAN_2026-06-17_prd_completion.md](PLAN_2026-06-17_prd_completion.md), which
supersedes [PLAN_2026-06-10_prd_completion.md](PLAN_2026-06-10_prd_completion.md); plus
the accepted hardening-lane plan
[PLAN_2026-06-11_operation_seam_unification.md](PLAN_2026-06-11_operation_seam_unification.md)). When
a plan item lands, its row in the plan is compressed to one line and the full
narrative moves here, newest entry first. This file is history, not
authority: lifecycle state lives in deliverable `_STATUS.md` files, evidence
lives in `_run_records/**`, and rulings live in the decision register and
`SOFTWARE_DECOMP.md` decision log. Nothing here is a release, professional,
certification, or code-compliance claim.

---

## 2026-06-19 - C5.7R packaged re-pass kit Inc 7 (`TP-R3UX-PACKAGEKIT-002`)

Rebuilt the macOS `.app` after C5.7R Inc 0-6 and prepared the human TP-MAC-189
packaged re-pass handoff. The bundle was produced at
`apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`
from commit `60fb533fe`.

Validation: `npm run tauri -- build --bundles app` passed; the packaged
executable boot probe ran for 8 seconds with process state `SN`,
`stdout_bytes=0`, `stderr_bytes=0`, and clean termination. The Inc 6/7
closeout evidence also includes dist Playwright 1/1 and DEC-025 sweep
`SWEEP_20260619T144814Z_48083bd29407-dirty.json` passing overall.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-274; DEL-07-06 run record
`WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-PACKAGEKIT-002.md`.

Residuals: the human TP-MAC-189 packaged re-pass is now the next gated item.
F-4/A3 remain open until that human record passes or records the next bounded
repair.

Boundary: package/evidence/handoff only. No lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-19 - C5.7R layout grid Inc 6 (`TP-R3UX-GRIDMODE-001`)

Landed the committed bulk-tabular follow-through from the workspace redesign:
the Model pane now supports Tree and Grid modes, with Grid mode exposing
spreadsheet-style editable rows for nodes, pipes, supports, materials,
sections, components, load cases, and combinations.

Changed cells fan out to ordinary structured `EditorOperationIntent`s through
the existing review/apply seam. Unit-bearing grid edits carry explicit unit
metadata; relationship cells that are not part of this slice remain read-only.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-273; DEL-07-06 run record
`WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-GRIDMODE-001.md`.

Validation: focused App Vitest passed 57/57; full desktop Vitest passed
18/18 files and 400/400 tests; desktop production build passed with the
existing Vite chunk-size warning; dev Playwright passed 18/18; dist
Playwright passed 1/1. The DEC-025 evidence sweep passed overall:
`validation/evidence/sweeps/SWEEP_20260619T144814Z_48083bd29407-dirty.json`.

Residuals: Inc 7 packaged build/pass-prep remains before the next human
TP-MAC-189 packaged re-pass; F-4/A3 remain open until the human pass is
recorded.

Boundary: frontend authoring ergonomics and tests only. No solver, schema,
evaluator, persistence, backend API, unit storage, protected-content,
private-data, network/telemetry, lifecycle, release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-19 - C5.7R workspace redesign Inc 0-5 (`TP-R3UX-WORKSPACEREDESIGN-001`)

Landed the minimum workspace redesign required before the first C5.7 packaged
human-pass reattempt: one workflow ribbon replaces the prior journey rail,
workspace tabs, and guided journey stack; storage/boundary/evidence detail is
collapsed into an Audit drawer; diagnostics and missing-input blockers are
collapsed into an Issues drawer; the viewport is promoted to the central hero
canvas; and the inspector now exposes display-only dual-unit context,
provenance, and required-field flags.

The app shell now renders a slim title bar, six-stop ribbon, tree/canvas/
inspector workspace, command/selection bar, and global status bar. The
Operation Apply panel no longer defaults to raw route/unit/session strings;
it uses compact chips and human-readable receipts while retaining governed
operation packet assertions.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-272; DEL-07-06 run record
`WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-WORKSPACEREDESIGN-001.md`.

Validation: `npm test --workspace apps/desktop` passed 18/18 test files and
399/399 tests; `npm run test:e2e --workspace apps/desktop` passed 18/18;
`npm run test:e2e:dist --workspace apps/desktop` passed 1/1; `npm run build
--workspace apps/desktop` passed with the existing Vite chunk-size warning.
The DEC-025 evidence sweep passed overall:
`validation/evidence/sweeps/SWEEP_20260619T072613Z_5dbd406023ba-dirty.json`.

Residuals: Inc 6 bulk tabular grid remains a committed C5.7R follow-through
item; the C5.7 packaged human pass must still be rebuilt and re-run before
F-4/A3 can close.

Boundary: frontend shell/presentation/test coverage only. No solver, schema,
evaluator, persistence, backend API, unit storage, protected-content,
private-data, network/telemetry, lifecycle, release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail export-review non-unit boundary classification (`TP-UNITS-BTAIL-EXPORTREVNONUNITBOUNDARY-001`)

Landed one bounded Phase B-tail Export Safety Review matrix cleanup while C5.7
remains human-execution gated. The two remaining non-unit-bearing Export
Review rows now carry explicit reasons instead of relying only on the generic
fallback label.

Export Safety Review now records `telemetry_boundary_review` and
`build_package_readiness` as
`not_unit_bearing_metadata_or_boundary_review`; the unit-evidence-required set
is unchanged. Solved queued-intent coverage remains `covered=26/27`;
proposal-path coverage remains 27/27 once `agent_proposal_review` is
available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-271; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVNONUNITBOUNDARY-001.md`;
DEL-12-03, DEL-10-04, and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1. Full App, full
desktop Vitest, build, Playwright, and DEC-025 sweep evidence are recorded in
closeout artifacts for this tranche.

Boundary: export-review inventory classification only. No telemetry runtime
behavior, network behavior, payload construction, build script, CI provider,
release matrix, signing, packaging, target writer, manifest-level unit
conversion, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review report-lint unit evidence (`TP-UNITS-BTAIL-EXPORTREVREPORTLINTUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review unit-evidence matrix
slice while C5.7 remains human-execution gated. The Report Content Lint packet
already exposes public-surface unit-policy inventory evidence; this tranche
makes that existing evidence discoverable through the Export Safety Review
matrix.

Export Safety Review now classifies `report_protected_content_lint` as
unit-evidence-required and covered by target-panel/export-packet evidence.
Solved queued-intent coverage is now `covered=26/27`; proposal-path coverage
is 27/27 once `agent_proposal_review` is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-270; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVREPORTLINTUNITS-001.md`;
DEL-08-05 and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1. Full App, full
desktop Vitest, build, Playwright, and DEC-025 sweep evidence are recorded in
closeout artifacts for this tranche.

Boundary: export-review inventory evidence only. No report-linter
protected-content semantics, legal clearance, redaction certification,
release authorization, runtime redaction rule, target writer,
manifest-level unit conversion, protected standards content, private payload,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review validation-evidence unit evidence (`TP-UNITS-BTAIL-EXPORTREVVALIDATIONEVIDUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review unit-evidence matrix
slice while C5.7 remains human-execution gated. The Validation Evidence panel
already exposes unit-policy evidence for project unit context, unit-bearing
record count, and the validation manual unit/schema section; this tranche
makes that existing evidence discoverable through the Export Safety Review
matrix.

Export Safety Review now classifies `validation_release_evidence_review` as
unit-evidence-required and covered by target-panel/export-packet evidence.
Solved queued-intent coverage is now `covered=25/26`; proposal-path coverage
is 26/26 once `agent_proposal_review` is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-269; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVVALIDATIONEVIDUNITS-001.md`;
DEL-09-04, DEL-09-05, and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1. Full App, full
desktop Vitest, build, Playwright, and DEC-025 sweep evidence are recorded in
closeout artifacts for this tranche.

Boundary: export-review inventory evidence only. No validation manual content,
release threshold, release authorization, validation-evidence storage
decision, target writer, manifest-level unit conversion, protected standards
content, private payload, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review design-workspace unit evidence (`TP-UNITS-BTAIL-EXPORTREVDESIGNWORKSPACEUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review unit-evidence matrix
slice while C5.7 remains human-execution gated. The Design Workspace panel
already exposes unit-policy evidence for composed model, result, analysis-run,
and comparison context; this tranche makes that existing evidence discoverable
through the Export Safety Review matrix.

Export Safety Review now classifies
`design_authoring_comparison_workspace` as unit-evidence-required and covered
by target-panel/export-packet evidence. Solved queued-intent coverage is now
`covered=24/25`; proposal-path coverage is 25/25 once
`agent_proposal_review` is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-268; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVDESIGNWORKSPACEUNITS-001.md`;
DEL-07-08 and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1. Full App, full
desktop Vitest, build, Playwright, and DEC-025 sweep evidence are recorded in
closeout artifacts for this tranche.

Boundary: export-review inventory evidence only. No design-workspace
composition behavior, comparison delta math, tolerance profile, operation
application, accepted model-state mutation, target writer, manifest-level unit
conversion, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review accessibility unit evidence (`TP-UNITS-BTAIL-EXPORTREVA11YUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review unit-evidence matrix
slice while C5.7 remains human-execution gated. The Accessibility Baseline
panel already exposes unit-visibility evidence for unit-bearing review
surfaces; this tranche makes that existing evidence discoverable through the
Export Safety Review matrix.

Export Safety Review now classifies
`accessibility_usability_baseline_review` as unit-evidence-required and
covered by target-panel/export-packet evidence. Solved queued-intent coverage
is now `covered=23/24`; proposal-path coverage is 24/24 once
`agent_proposal_review` is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-267; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVA11YUNITS-001.md`;
DEL-07-06 and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1. Full App, full
desktop Vitest, build, Playwright, and DEC-025 sweep evidence are recorded in
closeout artifacts for this tranche.

Boundary: export-review inventory evidence only. No accessibility finding
count, conformance target selection, runtime accessibility evaluation, color
signaling policy, target writer, manifest-level unit conversion, protected
standards content, private payload, lifecycle transition, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review secret/private unit evidence (`TP-UNITS-BTAIL-EXPORTREVSECRETPRIVATEUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review unit-evidence matrix
slice while C5.7 remains human-execution gated. The Secret and Private
Libraries panel already exposes metadata-only unit policy evidence for
unit-bearing private material, component, and rule-pack references; this
tranche makes that existing evidence discoverable through the Export Safety
Review matrix.

Export Safety Review now classifies `secret_private_library_boundary_review`
as unit-evidence-required and covered by target-panel/export-packet evidence.
Solved queued-intent coverage is now `covered=22/23`; proposal-path coverage
is 23/23 once `agent_proposal_review` is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-266; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVSECRETPRIVATEUNITS-001.md`;
DEL-12-04 and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1. Full App, full
desktop Vitest, build, Playwright, and DEC-025 sweep evidence are recorded in
closeout artifacts for this tranche.

Boundary: export-review inventory evidence only. No secret/private-library
payload handling, credential-value handling, encryption/key-management
decision, cloud or network behavior, external secret-manager behavior, target
writer, manifest-level unit conversion, protected standards content, private
payload, lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review editor-contract unit evidence (`TP-UNITS-BTAIL-EXPORTREVEDITORCONTRACTUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review unit-evidence matrix
slice while C5.7 remains human-execution gated. The Editor Contract panel
already exposes the DEL-02-02 unit contract; this tranche makes that existing
evidence discoverable through the Export Safety Review matrix.

Export Safety Review now classifies `editor_contract_review` as
unit-evidence-required and covered by target-panel/export-packet evidence.
Solved queued-intent coverage is now `covered=21/22`; proposal-path coverage
is 22/22 once `agent_proposal_review` is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-265; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVEDITORCONTRACTUNITS-001.md`;
DEL-07-03 and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1. Full App, full
desktop Vitest, build, Playwright, and DEC-025 sweep evidence are recorded in
closeout artifacts for this tranche.

Boundary: export-review inventory evidence only. No editor validation
behavior, durable mutation, private rule-pack payload handling,
private-library payload handling, target writer, manifest-level unit
conversion, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review missing-data unit evidence (`TP-UNITS-BTAIL-EXPORTREVMISSINGDATAUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review unit-evidence matrix
slice while C5.7 remains human-execution gated. The Missing Data Blocking
panel already exposes explicit unit-input policy evidence; this tranche makes
that existing evidence discoverable through the Export Safety Review matrix.

Export Safety Review now classifies `missing_data_warning_blocking_review` as
unit-evidence-required and covered by target-panel/export-packet evidence.
Solved queued-intent coverage is now `covered=20/21`; proposal-path coverage
is 21/21 once `agent_proposal_review` is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-264; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVMISSINGDATAUNITS-001.md`;
DEL-07-04 and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1; full App test file
passed 56/56; full desktop Vitest passed 18/18 files and 399/399 tests;
desktop build passed with the existing Vite large-chunk warning; focused R2
Playwright passed 2/2; full single-worker Playwright passed 18/18.

Boundary: export-review inventory evidence only. No missing-data blocking
behavior, remediation flow, accepted model mutation, rule-check execution,
runtime redaction rule, target writer, manifest-level unit conversion,
protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review external-prover unit evidence (`TP-UNITS-BTAIL-EXPORTREVEXTPROVERUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review unit-evidence matrix
slice while C5.7 remains human-execution gated. The external-prover boundary
panel already exposes DEC-018 unit-policy evidence; this tranche makes that
existing evidence discoverable through the Export Safety Review matrix.

Export Safety Review now classifies `external_prover_boundary_metadata` as
unit-evidence-required and covered by target-panel/export-packet evidence.
Solved queued-intent coverage is now `covered=19/20`; proposal-path coverage
is 20/20 once `agent_proposal_review` is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-263; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVEXTPROVERUNITS-001.md`;
DEL-15-04 and DEL-02-02 supporting run records with the same id.

Validation: focused App workspace-render test passed 1/1; full App test file
passed 56/56 after updating the stale visible coverage assertion; full
desktop Vitest passed 18/18 files and 399/399 tests; desktop build passed
with the existing Vite large-chunk warning; focused R2 Playwright passed 2/2;
full single-worker Playwright passed 18/18.

Boundary: export-review inventory evidence only. No external solver/prover
invocation, target parser, commercial-result ingestion, target writer,
manifest-level unit conversion, redaction behavior, public transport
commitment, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail product-preview fixture unit-policy inventory (`TP-UNITS-BTAIL-FIXTURELINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The invented product-preview fixture
bundle is unit-bearing public example data; this tranche makes its unit
evidence discoverable through the public unit-policy inventory.

Report Content Lint now inventories `fixtures/product_preview` as
`product-preview-fixture-unit-policy`, raising unit-policy target count from
43 to 44 while conversion-witness targets remain two. The fixture target text
records explicit model quantities, rule-pack unit refs, mechanics-result
units, and the active model unit system without changing any fixture payloads.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-262; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-FIXTURELINTUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: full App test file passed 56/56; full desktop Vitest passed 18/18
files and 399/399 tests; desktop build passed with the existing Vite
large-chunk warning; focused R2 Playwright passed 2/2; full single-worker
Playwright passed 18/18.

Boundary: report-lint inventory evidence only. No fixture data,
report-linter protected-content semantics, runtime redaction rules, target
writer behavior, unit conversion, protected standards content, private
payload, lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail security threat-model unit-policy inventory (`TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001`)

Landed one bounded Phase B-tail Security Threat Model unit-policy inventory
slice while C5.7 remains human-execution gated. The security threat-model
review already records `unit_checks` as a no-bypass control; this tranche
makes that unit-policy evidence visible and discoverable through report/export
inventories.

The Security Threat Model panel now displays explicit unit-check no-bypass
evidence for unit-bearing export and handoff workflows:
`unit_checks=true`, six workflow refs, `conversion=false`, and
`certification=false`. Report Content Lint now inventories
`security-threat-model-unit-policy`, raising unit-policy target count from 42
to 43 while conversion-witness targets remain two. Export Safety Review now
marks `security_threat_model_review` as unit-evidence-required, raising solved
queued-intent coverage to `covered=18/19`; proposal-path coverage is 19/19
once the proposal row is available.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-261; DEL-12-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-SECTHREATUNITPOLICY-001.md`;
DEL-08-05, DEL-12-02, and DEL-02-02 supporting run records with the same id.

Validation: full App test file passed 56/56 after stale export-review
coverage expectations were updated; full desktop Vitest passed 18/18 files
and 399/399 tests; desktop build passed with the existing Vite large-chunk
warning; focused R2 Playwright passed 2/2 after stale report-lint target-count
assertions were updated; full single-worker Playwright passed 18/18.

Boundary: security/export/report inventory evidence only. No threat-model
semantics, telemetry authorization, runtime redaction rules, target writer
behavior, manifest-level unit conversion, protected standards content, private
payload, lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, security sufficiency claim, or
code-compliance claim changed.

---

## 2026-06-18 - B-tail agent proposal unit-policy inventory (`TP-UNITS-BTAIL-AGENTPROPOSALUNITS-001`)

Landed one bounded Phase B-tail Agent Proposal unit-policy inventory slice
while C5.7 remains human-execution gated. The Agent Proposal panel is a public
review-only operation surface; this tranche makes its metadata-only
unit-validation status visible and discoverable through report/export
inventories.

The Agent Proposal panel now displays `proposal.validation.unit_validation`;
mechanics-derived proposals and the invented proposal fixture set
`unit_validation=not_required_metadata_review_only`. Report Content Lint now
includes `apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx`
and `agent-proposal-unit-policy`, raising unit-policy target count from 41 to
42 while conversion-witness targets remain two. Export Safety Review now
includes `agent_proposal_review`, pending until a proposal exists and available
after proposal generation; report persistence export inventory now expects 29
rows.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-260; DEL-16-04 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-AGENTPROPOSALUNITS-001.md`;
DEL-16-02, DEL-16-03, DEL-08-05, DEL-12-02, DEL-08-04, and DEL-02-02
supporting run records with the same id.

Validation: focused App proposal test passed 1/1; focused queued-intent export
review test passed 1/1; full App test file passed 56/56; full desktop Vitest
passed 18/18 files and 399/399 tests. Desktop production build passed with
the existing Vite large-chunk warning. Focused R2 Playwright passed 2/2 after
stale target-count assertions were updated; full single-worker Playwright
passed 18/18.

Boundary: proposal/export/report inventory evidence only. No proposal
application, accepted model-state mutation, operation validation semantics,
acceptance semantics, report-linter protected-content semantics, redaction
rules, target writer behavior, unit conversion, protected standards content,
private payload, lifecycle transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail property inspector report-lint unit inventory (`TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Property Inspector already exposes
edit, create, and delete operation unit-validation surfaces; this tranche makes
that public unit-policy surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/model-tree/PropertyInspector.tsx`, target
`target:desktop-property-inspector-template`, and target ref
`property-inspector-unit-validation-surface`. Unit-policy target count
increases from 39 to 40, while target-format conversion-witness target count
remains two. The lint still performs no conversion and makes no target-format
compatibility assertion. Static report-lint target count is 44; solved report
packets include 45 targets after appending the generated preview-report JSON
target.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-257; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001.md`;
DEL-07-02, DEL-16-02, and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render selected test passed 1/1;
focused Property Inspector create-intent App tests passed 4/4 selected tests;
focused Playwright R2 smoke passed 2/2 configured project tests; full desktop
Vitest passed 18/18 files and 399/399 tests. Desktop production build passed
with the existing Vite large-chunk warning. Single-worker R2/R3 Playwright
smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Property Inspector
unit-validation surfaces only. No Property Inspector behavior,
operation validation, operation application, diff preview, report-linter
protected-content semantics, legal clearance, redaction controls, target writer
compatibility, unit-conversion API, DEC-018 catalog constant, schema dimension
enum, protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail load manager report-lint unit inventory (`TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Load Case Manager already exposes
load-case, primitive-load, and combination operation unit-validation surfaces;
this tranche makes that public unit-policy surface discoverable through the
Report Content Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`, target
`target:desktop-load-case-manager-template`, and target ref
`load-manager-unit-validation-surface`. Unit-policy target count increases
from 38 to 39, while target-format conversion-witness target count remains two.
The lint still performs no conversion and makes no target-format compatibility
assertion. Static report-lint target count is 43; solved report packets
include 44 targets after appending the generated preview-report JSON target.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-256; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001.md`;
DEL-07-02, DEL-05-01, DEL-05-02, DEL-16-02, and DEL-02-02 supporting run
records with the same id.

Validation: focused App Vitest workspace-render selected test passed 1/1;
focused load/unit App tests passed 26/26 selected tests; focused Playwright R2
smoke passed 2/2 configured project tests; full desktop Vitest passed 18/18
files and 399/399 tests. Desktop production build passed with the existing
Vite large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18
tests. `git diff --check` passed.

Boundary: report-lint inventory over existing Load Case Manager
unit-validation surfaces only. No load-case schema, primitive-load or
combination behavior, operation validation, operation application,
report-linter protected-content semantics, legal clearance, redaction controls,
target writer compatibility, unit-conversion API, DEC-018 catalog constant,
schema dimension enum, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail library manager report-lint unit inventory (`TP-UNITS-BTAIL-LIBRARYLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Library Manager already exposes
material, section, and component draft unit-helper surfaces; this tranche
makes that public unit-policy surface discoverable through the Report Content
Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/library/LibraryManagerPanel.tsx`, target
`target:desktop-library-manager-template`, and target ref
`library-unit-helper-surfaces`. Unit-policy target count increases from 37 to
38, while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion. Static report-lint target count is 42; solved report packets include
43 targets after appending the generated preview-report JSON target.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-255; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LIBRARYLINTUNITS-001.md`;
DEL-07-03, DEL-03-01, DEL-03-02, and DEL-02-02 supporting run records with
the same id.

Validation: focused App Vitest workspace-render selected test passed 1/1;
focused LibraryManagerPanel Vitest passed 9/9 selected tests; focused
Playwright R2/library-manager smoke passed 4/4 configured project tests; full
desktop Vitest passed 18/18 files and 399/399 tests. Desktop production build
passed with the existing Vite large-chunk warning. Single-worker R2/R3
Playwright smoke passed 18/18 tests. `git diff --check` passed.

Boundary: report-lint inventory over existing Library Manager unit-helper
surfaces only. No library schema, library import storage, private-library
payload handling, report-linter protected-content semantics, legal clearance,
redaction controls, target writer compatibility, unit-conversion API,
DEC-018 catalog constant, schema dimension enum, protected standards content,
private payload, lifecycle transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail rule-pack expression report-lint unit inventory (`TP-UNITS-BTAIL-RULEPACKEXPRLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Rule Pack Expression Composer already
exposes structured-AST expression unit-reference policy; this tranche makes
that public unit-policy surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/rule-packs/ExpressionComposer.tsx`, target
`target:desktop-rule-pack-expression-template`, and target ref
`rule-pack-expression-unit-policy`. Unit-policy target count increases from
36 to 37, while target-format conversion-witness target count remains two. The
lint still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-254; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULEPACKEXPRLINTUNITS-001.md`;
DEL-07-03, DEL-06-02, and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project round-trip
selected tests passed 1/1 each; focused ExpressionComposer/RulePackManagerPanel
Vitest passed 6/6 selected tests; focused Playwright R2/rule-pack smoke passed
4/4 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Rule Pack Expression Composer
unit-reference policy context only. No rule-pack schema, expression grammar,
writable text parser/syntax, evaluator behavior, backend validation,
persistence, report-linter protected-content semantics, legal clearance,
redaction controls, target writer compatibility, unit-conversion API,
DEC-018 catalog constant, schema dimension enum, protected standards content,
private payload, lifecycle transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail rule-pack declarations report-lint unit inventory (`TP-UNITS-BTAIL-RULEPACKDECLLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Rule Pack Declarations editor already
exposes required-input and value-slot unit-reference policy; this tranche
makes that public unit-policy surface discoverable through the Report Content
Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx`, target
`target:desktop-rule-pack-declarations-template`, and target ref
`rule-pack-declarations-unit-policy`. Unit-policy target count increases from
35 to 36, while target-format conversion-witness target count remains two. The
lint still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-253; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULEPACKDECLLINTUNITS-001.md`;
DEL-07-03, DEL-06-01, and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project round-trip
selected tests passed 1/1 each; focused RulePackManagerPanel/DeclarationsEditor
Vitest passed 5/5 selected tests; focused Playwright R2/rule-pack smoke passed
4/4 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Rule Pack Declarations
unit-reference policy context only. No rule-pack schema, expression grammar,
parser/text syntax, evaluator behavior, backend validation, persistence,
report-linter protected-content semantics, legal clearance, redaction
controls, target writer compatibility, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private payload,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail diagnostic report-lint unit inventory (`TP-UNITS-BTAIL-DIAGNOSTICLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Diagnostics panel already exposes
linked result unit context for selected diagnostics; this tranche makes that
public unit-policy surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/diagnostics/DiagnosticsPanel.tsx`, target
`target:desktop-diagnostics-template`, and target ref
`diagnostic-unit-context`. Unit-policy target count increases from 34 to 35,
while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-252; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-DIAGNOSTICLINTUNITS-001.md`;
DEL-07-07 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render, diagnostic detail, and local
project round-trip selected tests passed 1/1 each; focused Playwright R2 smoke
passed 2/2 configured project tests and focused diagnostic Playwright passed
2/2. Full desktop Vitest passed 18/18 files and 399/399 tests. Desktop
production build passed with the existing Vite large-chunk warning.
Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Diagnostics linked-result unit
context only. No diagnostic schema, diagnostic interpretation behavior,
solver behavior, result values, report-linter protected-content semantics,
legal clearance, redaction controls, target writer compatibility,
unit-conversion API, DEC-018 catalog constant, schema dimension enum,
protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail run-rule-check report-lint unit inventory (`TP-UNITS-BTAIL-RULECHECKLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Run Rule Checks panel already exposes
runtime value-binding unit policy context; this tranche makes that public
unit-policy surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx`, target
`target:desktop-rule-check-run-template`, and target ref
`rule-check-unit-binding-policy`. Unit-policy target count increases from 33
to 34, while target-format conversion-witness target count remains two. The
lint still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-251; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RULECHECKLINTUNITS-001.md`;
DEL-06-03 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused RuleCheckRunPanel Vitest
passed 18/18; focused Playwright R2/run-check smoke passed 4/4 configured
project tests; full desktop Vitest passed 18/18 files and 399/399 tests.
Desktop production build passed with the existing Vite large-chunk warning.
Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Run Rule Checks unit-binding
policy context only. No rule-pack schema, expression grammar,
parser/text-syntax, backend completeness/evaluator behavior, rule-pack
persistence, solver behavior, report-linter protected-content semantics,
legal clearance, redaction controls, target writer compatibility,
unit-conversion API, DEC-018 catalog constant, schema dimension enum,
protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail comparison report-lint unit inventory (`TP-UNITS-BTAIL-COMPARISONLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Comparison workspace already exposes
equal-explicit-unit matching policy context; this tranche makes that public
unit-policy surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/comparison/ComparisonPanel.tsx`, target
`target:desktop-comparison-workspace-template`, and target ref
`comparison-unit-policy`. Unit-policy target count increases from 32 to 33,
while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-250; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-COMPARISONLINTUNITS-001.md`;
DEL-14-04 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Comparison workspace
unit-matching policy context only. No comparison delta math, result mapping,
tolerance profile, default tolerance, solver behavior, report-linter
protected-content semantics, legal clearance, redaction controls, target
writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
dimension enum, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail rendered-report report-lint unit inventory (`TP-UNITS-BTAIL-RENDEREDREPORTLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Rendered Report panel already exposes
renderable-report unit-basis context; this tranche makes that public
unit-policy surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/report/RenderedReportPanel.tsx`, target
`target:desktop-rendered-report-template`, and target ref
`rendered-report-unit-basis`. Unit-policy target count increases from 31 to
32, while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-249; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RENDEREDREPORTLINTUNITS-001.md`;
DEL-08-01 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Rendered Report unit-basis
context only. No report renderer route, canonical hash policy, export-gate
policy, report content, print/PDF behavior, report-linter protected-content
semantics, legal clearance, redaction controls, target writer compatibility,
unit-conversion API, DEC-018 catalog constant, schema dimension enum,
protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail results-viewer report-lint unit inventory (`TP-UNITS-BTAIL-RESULTVIEWLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Results viewer already exposes solved
result-unit policy context; this tranche makes that public unit-policy surface
discoverable through the Report Content Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/results/ResultsPanel.tsx`, target
`target:desktop-results-viewer-template`, and target ref
`result-unit-policy`. Unit-policy target count increases from 30 to 31, while
target-format conversion-witness target count remains two. The lint still
performs no conversion and makes no target-format compatibility assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-248; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RESULTVIEWLINTUNITS-001.md`;
DEL-07-05 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Results viewer result-unit
policy context only. No result math, result filtering, pagination semantics,
selected-result interpretation, solver behavior, comparison delta math,
tolerance profile, default tolerance, report-linter protected-content
semantics, legal clearance, redaction controls, target writer compatibility,
unit-conversion API, DEC-018 catalog constant, schema dimension enum,
protected standards content, private payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail apply-operations report-lint unit inventory (`TP-UNITS-BTAIL-OPAPPLYLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Apply Operations panel already
exposes queued-operation and receipt unit-policy context; this tranche makes
that public unit-policy surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/operations/OperationApplyPanel.tsx`, target
`target:desktop-operation-apply-template`, and target ref
`operation-apply-unit-policy`. Unit-policy target count increases from 29 to
30, while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-247; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-OPAPPLYLINTUNITS-001.md`;
DEL-16-03 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Apply Operations unit-policy
context only. No operation application, acceptance semantics, durable audit
persistence, receipt schema, operation validation, report-linter
protected-content semantics, legal clearance, redaction controls, target
writer compatibility, unit-conversion API, DEC-018 catalog constant, schema
dimension enum, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail design-knowledge report-lint unit inventory (`TP-UNITS-BTAIL-KNOWLEDGELINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Design Knowledge panel already
exposes computed result-unit context for local invented mechanics-derived
knowledge records; this tranche makes that public unit-policy surface
discoverable through the Report Content Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/knowledge/KnowledgePanel.tsx`, target
`target:desktop-design-knowledge-template`, and target ref
`knowledge-unit-context`. Unit-policy target count increases from 28 to 29,
while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-246; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-KNOWLEDGELINTUNITS-001.md`;
DEL-13-01 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Design Knowledge computed-unit
context only. No design-knowledge schema, provenance model, result values,
solver behavior, report-linter protected-content semantics, legal clearance,
redaction controls, target writer compatibility, unit-conversion API,
DEC-018 catalog constant, schema dimension enum, protected standards content,
private payload, lifecycle transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail run-audit report-lint unit inventory (`TP-UNITS-BTAIL-RUNAUDITLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Run Audit panel already exposes
solved analysis-run unit traceability; this tranche makes that public
unit-policy surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/run-audit/RunAuditPanel.tsx`, target
`target:desktop-run-audit-template`, and target ref `run-audit-units`.
Unit-policy target count increases from 27 to 28, while target-format
conversion-witness target count remains two. The lint still performs no
conversion and makes no target-format compatibility assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-245; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RUNAUDITLINTUNITS-001.md`;
DEL-14-02 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Run Audit unit traceability
only. No analysis-run schema, hash policy, solver behavior, result values,
report-linter protected-content semantics, legal clearance, redaction
controls, target writer compatibility, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private payload,
lifecycle state transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail solve-job report-lint unit inventory (`TP-UNITS-BTAIL-SOLVEJOBLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Solve execution panel already exposes
visible and exported DEC-018 solve-job unit-policy evidence; this tranche
makes that public unit-policy surface discoverable through the Report Content
Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/solve/SolvePanel.tsx`, target
`target:desktop-solve-job-template`, and target ref
`solve-job-unit-policy`. Unit-policy target count increases from 26 to 27,
while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-244; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-SOLVEJOBLINTUNITS-001.md`;
DEL-07-07 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests; full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Solve execution solve-job
unit-policy evidence only. No solve execution behavior, analysis-run hash
generation, cancellation/progress semantics, backend job behavior,
report-linter protected-content semantics, legal clearance, redaction
controls, target writer compatibility, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail operation-ledger report-lint unit inventory (`TP-UNITS-BTAIL-OPLEDGERLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Operation Review Ledger already
exposes visible and exported DEC-018 unit-policy evidence for review-only
operation records; this tranche makes that public unit-policy surface
discoverable through the Report Content Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/operations/OperationLedgerPanel.tsx`, target
`target:desktop-operation-ledger-template`, and target ref
`operation-ledger-unit-policy`. Unit-policy target count increases from 25 to
26, while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-243; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-OPLEDGERLINTUNITS-001.md`;
DEL-16-03 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests. An initial full desktop Vitest attempt surfaced
an order-sensitive `App.test.tsx` Operation Ledger unit-validation status
mismatch; the selected test then passed in isolation, and the full desktop
Vitest rerun passed 18/18 files and 399/399 tests. Desktop production build
passed with the existing Vite large-chunk warning. Single-worker R2/R3
Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Operation Review Ledger
unit-policy evidence only. No operation application, acceptance semantics,
durable audit persistence, receipt schema, report-linter protected-content
semantics, legal clearance, redaction controls, target writer compatibility,
unit-conversion API, DEC-018 catalog constant, schema dimension enum,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail adapter-framework report-lint unit inventory (`TP-UNITS-BTAIL-ADAPTERFRAMEWORKLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Adapter Framework panel already
exposes DEC-018 unit-policy evidence for the format-neutral adapter
declaration; this tranche makes that public unit-policy surface discoverable
through the Report Content Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx`,
target `target:desktop-adapter-framework-template`, and target ref
`adapter-framework-units`. Unit-policy target count increases from 24 to 25,
while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-242; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-ADAPTERFRAMEWORKLINTUNITS-001.md`;
DEL-10-02 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests. An initial full desktop Vitest attempt surfaced
a transient `DeclarationsEditor.test.tsx` timing failure; the selected test
then passed in isolation, and the full desktop Vitest rerun passed 18/18 files
and 399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Adapter Framework unit-policy
evidence only. No adapter-framework packet semantics, schema, concrete
external format list, public transport, plugin runtime, permission
persistence, package scripts, CI/release matrix, report-linter
protected-content semantics, legal clearance, unit-conversion API, DEC-018
catalog constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail headless-runner report-lint unit inventory (`TP-UNITS-BTAIL-HEADLESSRUNNERLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Headless Runner panel already exposes
DEC-018 unit-system disclosure and per-result unit preservation witnesses for
the schema-first result handoff preview; this tranche makes that public
unit-witness surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx`, target
`target:desktop-headless-runner-template`, and target ref
`headless-runner-unit-witnesses`. Unit-policy target count increases from 23
to 24, while target-format conversion-witness target count remains two. The
lint still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-241; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-HEADLESSRUNNERLINTUNITS-001.md`;
DEL-10-05 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests. Full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Headless Runner unit-witness
evidence only. No headless-runner packet semantics, schema, final CLI syntax,
package scripts, process policy, network policy, filesystem mutation policy,
report-linter protected-content semantics, legal clearance, unit-conversion
API, DEC-018 catalog constant, schema dimension enum, protected standards
content, private payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-18 - B-tail handoff report-lint unit inventory (`TP-UNITS-BTAIL-HANDOFFLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Handoff Package panel already exposes
DEC-018 unit-system disclosure and per-result unit preservation witnesses for
the local review handoff package; this tranche makes that public unit-witness
surface discoverable through the Report Content Lint public-surface
inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/handoff/HandoffPanel.tsx`, target
`target:desktop-handoff-package-template`, and target ref
`handoff-unit-witnesses`. Unit-policy target count increases from 22 to 23,
while target-format conversion-witness target count remains two. The lint
still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-240; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-HANDOFFLINTUNITS-001.md`;
DEL-15-01 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests. Full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Handoff Package unit-witness
evidence only. No handoff packet semantics, handoff schema, target mapping,
external prover workflow, export-review manifest semantics, report-linter
protected-content semantics, legal clearance, unit-conversion API, DEC-018
catalog constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail result-export report-lint unit inventory (`TP-UNITS-BTAIL-RESULTEXPORTLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Result Export panel already exposes
schema-first local JSON result envelopes with explicit result units and
per-result unit preservation witnesses; this tranche makes that public
unit-witness surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/result-export/ResultExportPanel.tsx`, target
`target:desktop-result-export-template`, and target ref
`result-export-unit-witnesses`. Unit-policy target count increases from 21
to 22, while target-format conversion-witness target count remains two. The
lint still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-239; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-RESULTEXPORTLINTUNITS-001.md`;
DEL-08-04 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and local project
round-trip selected tests passed 1/1 each; focused Playwright R2 smoke passed
2/2 configured project tests. Full desktop Vitest passed 18/18 files and
399/399 tests. Desktop production build passed with the existing Vite
large-chunk warning. Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Result Export unit-witness
evidence only. No result-export packet semantics, result envelope schema,
mechanics result generation, export-review manifest semantics,
report-linter protected-content semantics, legal clearance, unit-conversion
API, DEC-018 catalog constant, schema dimension enum, protected standards
content, private payload, lifecycle state transition, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-18 - B-tail operation-diff report-lint unit inventory (`TP-UNITS-BTAIL-DIFFPREVIEWLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Operation Diff Preview panel already
exposed DEC-018 unit-system disclosure and per-change unit preservation
witnesses; this tranche makes that public unit-witness surface discoverable
through the Report Content Lint public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx`, target
`target:desktop-operation-diff-template`, and target ref
`operation-diff-unit-witnesses`. Unit-policy target count increases from 20
to 21, while target-format conversion-witness target count remains two. The
lint still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-238; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-DIFFPREVIEWLINTUNITS-001.md`;
DEL-16-02 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest workspace-render and queued-intent selected
tests passed 1/1 each; focused Playwright R2 desktop preview smoke passed
1/1; full desktop Vitest passed 18/18 files and 399/399 tests. Desktop
production build passed with the existing Vite large-chunk warning.
Single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Operation Diff Preview
unit-witness evidence only. No diff-preview packet semantics, operation
validation, operation application, accepted-model mutation behavior,
report-linter protected-content semantics, legal clearance, unit-conversion
API, DEC-018 catalog constant, schema dimension enum, protected standards
content, private payload, lifecycle state transition, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-18 - B-tail project-validation report-lint unit inventory (`TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint inventory slice while
C5.7 remains human-execution gated. The Project Validation Preflight panel
already exposed DEC-018 `unit_policy_evidence`; this tranche makes that
public unit-bearing surface discoverable through the Report Content Lint
public-surface inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx`,
target `target:desktop-project-validation-template`, and target ref
`project-validation-unit-policy`. Unit-policy target count increases from 19
to 20, while target-format conversion-witness target count remains two. The
lint still performs no conversion and makes no target-format compatibility
assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-237; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-PROJECTVALIDATIONLINTUNITS-001.md`;
DEL-02-05 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest initially caught the missing explicit lint
target entry, then passed the workspace-render selected test after repair;
focused Playwright R2 desktop preview smoke passed 1/1; full desktop Vitest
initially caught one over-broad export-review metadata count assertion, then
passed 18/18 files and 399/399 tests after repair. Desktop production build
passed with the existing Vite large-chunk warning. Single-worker R2/R3
Playwright smoke passed 18/18 tests.

Boundary: report-lint inventory over existing Project Validation Preflight
unit-policy evidence only. No validation-preflight packet semantics, project
persistence semantics, report-linter protected-content semantics, legal
clearance, unit-conversion API, DEC-018 catalog constant, schema dimension
enum, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail export-review storage/preflight unit evidence (`TP-UNITS-BTAIL-EXPORTREVIEWSTORAGEUNITS-001`)

Landed one bounded Phase B-tail Export Safety Review matrix cleanup while
C5.7 remains human-execution gated. The export-review manifest already listed
the Project Storage Audit and Project Validation Preflight export records, and
those packets already carried DEC-018 unit-policy evidence; this tranche now
classifies those two records as unit-evidence-required instead of
metadata-only.

The Export Safety Review `unit_policy_summary` still performs no conversion
and makes no target-writer compatibility claim. Its 27-row
`unit_evidence_matrix` now records 16 unit-bearing covered export records,
including `project_storage_audit` and `project_validation_preflight`, and the
visible `export-review-units` row reports `covered=16/16`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-236; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVIEWSTORAGEUNITS-001.md`;
DEL-02-05 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest passed the workspace-render and local
create/save/open selected tests; full desktop Vitest passed 18/18 files and
399/399 tests; desktop production build passed with the existing Vite
large-chunk warning; single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: export-review metadata inventory only. No project persistence
semantics, validation-preflight semantics, target-specific writer, manifest
unit conversion, runtime redaction rule, public transport commitment,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-18 - B-tail native package unit inventory coverage (`TP-UNITS-BTAIL-NATIVEPACKAGEUNITS-001`)

Landed one bounded Phase B-tail Native JSON package unit-inventory slice while
C5.7 remains human-execution gated. The Native JSON package panel already
exposed project unit declarations, model quantity witnesses, result quantity
witnesses, and `conversion=false`; this tranche makes that public
unit-preservation surface discoverable through the Report Content Lint
unit-policy inventory.

The Report Content Lint explicit public-surface inventory now includes
`apps/desktop/src/features/native-package/NativePackagePanel.tsx` and target
ref `native-package-unit-witnesses`. Unit-policy target count increases from
18 to 19, while target-format conversion-witness target count remains two
(PCF and CAEPIPE MBF). The lint still performs no conversion and makes no
target-format compatibility assertion.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-235; DEL-17-03 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-NATIVEPACKAGEUNITS-001.md`;
DEL-08-05 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest initially caught stale count assertions and
then passed the workspace-render, local create/save/open, computed-diagnostics,
and queued-editor-intent selected tests. Full desktop Vitest initially caught
one over-broad export-review manifest count update and then passed 18/18 files
and 399/399 tests after repair. Desktop production build passed with the
existing Vite large-chunk warning. Single-worker R2/R3 Playwright smoke passed
18/18 tests.

Boundary: no native JSON packet schema, package member set, package hash
basis, target writer, target compatibility posture, report-linter
protected-content semantics, legal clearance, unit-conversion API, DEC-018
catalog constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-18 - B-tail project-storage unit policy evidence (`TP-UNITS-BTAIL-STORAGEUNITS-001`)

Landed one bounded Phase B-tail Project Storage Audit unit-policy evidence
slice while C5.7 remains human-execution gated. The DEL-02-05 Project Storage
Audit panel now exposes structured unit-policy evidence for existing local
project unit round-trip metadata.

The tranche adds `unit_policy_evidence` to the local storage audit packet and
extends `project-storage-unit-round-trip` to report sorted model units and
`conversion=false` alongside the existing round-trip status, checked-ref
count, and signature. The packet records DEC-018 and DEL-02-02/DEL-02-05
basis refs, entered-unit preservation, and a no-conversion policy. The Report
Content Lint inventory now includes the Project Storage Audit unit-policy
surface, increasing unit-policy targets from 17 to 18 while keeping target
conversion witnesses at two.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-234; DEL-02-05 primary run record
`WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-STORAGEUNITS-001.md`;
DEL-02-02 and DEL-08-05 supporting run records with the same id.

Validation: focused App Vitest passed the initial storage, local
create/save/open round-trip, and solved report-lint selected tests; focused
Playwright passed 2/2 Chromium desktop tests; `git diff --check` passed; full
desktop Vitest passed 18/18 files and 399/399 tests; desktop production build
passed with the existing Vite large-chunk warning; single-worker R2/R3
Playwright smoke passed 18/18 tests.

Boundary: no local persistence semantics, migration policy, project-envelope
schema, model hash canonicalization, report-linter protected-content
semantics, unit-conversion API, DEC-018 catalog constant, schema dimension
enum, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail solve-job unit policy visibility (`TP-UNITS-BTAIL-SOLVEJOBUNITS-001`)

Landed one bounded Phase B-tail Solve Job unit-policy visibility slice while
C5.7 remains human-execution gated. The DEL-07-07 Solve execution panel now
exposes the model/result unit basis in the solve-job audit row and downloaded
solve-job JSON.

The tranche adds `solve-job-unit-policy` to the Solve panel and
`unit_policy_evidence` to the solve-job audit packet. The row and packet
report sorted model units, solved result units or `results=none`, result row
count, DEC-018 and DEL-02-02/DEL-07-07 basis refs,
`storage_convention=entered_units_preserved`, and `conversion=false`.
Solve execution, solver backend behavior, analysis-run hashes, and result
values are unchanged.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-233; DEL-07-07 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SOLVEJOBUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed the not-started and solved-state
selected tests; focused Playwright passed 2/2 tests; `git diff --check`
passed; full desktop Vitest passed 18/18 files and 399/399 tests; desktop
production build passed with the existing Vite large-chunk warning;
single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: no solve execution behavior, solver backend seam, result values,
analysis-run hashes, cancellation behavior, unit-conversion API, DEC-018
catalog constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-17 - B-tail rendered-report unit basis visibility (`TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001`)

Landed one bounded Phase B-tail Rendered Report unit-basis visibility slice
while C5.7 remains human-execution gated. The DEL-08-01 Rendered Report panel
now exposes the render-input unit basis before invoking the desktop-only
renderer route.

The tranche adds `rendered-report-unit-basis` to the panel. The row reports
the DEC-018 unit-system reference, sorted model units, solved result units or
`results=none`, `conversion=false`, and `source=renderable_report_input`.
It mirrors the existing `buildRenderableReportInput` unit display summary;
the Rust renderer, report schema, canonical hash behavior, save/print gate,
and report-time conversion policy are unchanged.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-232; DEL-08-01 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RENDEREDREPORTUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused rendered-report Vitest passed 8/8 tests; focused
Playwright passed 2/2 Chromium desktop tests; `git diff --check` passed; full
desktop Vitest passed 18/18 files and 399/399 tests; desktop production build
passed with the existing Vite large-chunk warning; single-worker R2/R3
Playwright smoke passed 18/18 tests.

Boundary: no renderer command, report schema, canonical hash behavior,
save/print gate, report-time conversion, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-17 - B-tail report-lint unit policy inventory (`TP-UNITS-BTAIL-REPORTLINTUNITS-001`)

Landed one bounded Phase B-tail Report Content Lint unit-policy inventory
slice while C5.7 remains human-execution gated. The DEL-08-05 report linter
now exposes how its explicit public report/export target list intersects with
unit-policy and target-format conversion-witness surfaces.

The tranche adds `report-lint-unit-policy` to the Report Content Lint panel
and `unit_policy_evidence` to the downloaded lint JSON. The row and packet
report 17 public unit-policy targets, two target-format conversion-witness
surfaces, `lint_conversion=false`, and no target compatibility assertion by
the linter. This records inventory evidence only; the underlying
protected-content heuristic and redaction/legal boundaries are unchanged.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-231; DEL-08-05 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-REPORTLINTUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 selected test; focused Playwright
passed 1/1 Chromium desktop test; `git diff --check` passed; full desktop
Vitest passed 18/18 files and 399/399 tests; desktop production build passed
with the existing Vite large-chunk warning; single-worker R2/R3 Playwright
smoke passed 18/18 tests.

Boundary: no linter protected-content semantics, legal clearance, redaction
controls, target writer compatibility, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-17 - B-tail operation ledger unit policy visibility (`TP-UNITS-BTAIL-OPLEDGERUNITS-001`)

Landed one bounded Phase B-tail Operation Review Ledger unit-policy visibility
slice while C5.7 remains human-execution gated. The DEL-16-03 operation
ledger now exposes how queued GUI operation intents and agent proposal review
records carry unit-policy evidence before any explicit user acceptance.

The tranche adds `operation-ledger-unit-policy` to the ledger panel and
`unit_policy_evidence` to the downloaded ledger JSON. GUI intent records
count unit-bearing and dimensionless changes from explicit operation metadata;
agent proposal records without unit metadata remain dimensionless/no
unit-validation evidence. The row and packet report validation statuses,
`receipt_units=not_serialized_in_review_ledger`, and `conversion=false`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-230; DEL-16-03 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-OPLEDGERUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 3/3 selected tests; focused Playwright
passed 1/1 Chromium desktop test; `git diff --check` passed; full desktop
Vitest passed 18/18 files and 399/399 tests; desktop production build passed
with the existing Vite large-chunk warning; single-worker R2/R3 Playwright
smoke passed 18/18 tests.

Boundary: no operation application, acceptance semantics, durable audit
persistence, receipt schema, solver behavior, unit-conversion API, DEC-018
catalog constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-17 - B-tail operation apply unit policy visibility (`TP-UNITS-BTAIL-OPAPPLYUNITS-001`)

Landed one bounded Phase B-tail Apply Operations unit-policy visibility slice
while C5.7 remains human-execution gated. The DEL-16-02 Apply Operations panel
now exposes how queued intents, operation outcomes, and applied receipts carry
unit-policy evidence at the final user-apply surface.

The tranche adds `operation-apply-unit-policy` to the Apply Operations panel.
Before applying the load-magnitude edit, the browser route reports
`queued_unit_bearing=1`, `queued_dimensionless=0`,
`outcome_unit_validations=none`, `applied_receipts=0`,
`receipt_units=not_serialized`, and `conversion=false`. After applying through
the local WASM route, it reports `queued_unit_bearing=0`,
`outcome_unit_validations=passed`, `applied_receipts=1`,
`receipt_units=not_serialized`, and `conversion=false`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-229; DEL-16-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-OPAPPLYUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 selected test; focused Playwright
passed 1/1 Chromium desktop test; `git diff --check` passed; full desktop
Vitest passed 18/18 files and 399/399 tests; desktop production build passed
with the existing Vite large-chunk warning; single-worker R2/R3 Playwright
smoke passed 18/18 tests.

Boundary: no operation-applier semantics, backend validation, receipt schema,
solver behavior, unit-conversion API, DEC-018 catalog constant, schema
dimension enum, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail rule-check run binding unit policy visibility (`TP-UNITS-BTAIL-RULECHECKBINDUNITS-001`)

Landed one bounded Phase B-tail run-rule-check binding unit-policy slice while
C5.7 remains human-execution gated. The DEL-06-03 Run Rule Checks binding plan
now exposes visible unit-binding policy evidence for runtime value, slot,
solver-result, and private-library binding routes.

The tranche adds `rule-check-unit-binding-policy` to the run-check binding
plan. The browser demo-pack route reports `value_inputs=1`, `value_slots=1`,
`solver_selectors=1`, `solver_result_refs=0`, `private_library_refs=0`,
`catalog=browser_manual_text_no_fallback`, and `conversion=false`. The
desktop-mode mocked test verifies the DEC-018 catalog route as
`catalog=dec018_catalog(entries=4)`; a private-library input case verifies
`private_library_refs=1` and `catalog=not_required`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-228; DEL-06-03 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RULECHECKBINDUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused RuleCheckRunPanel Vitest passed 18/18 tests; focused
Playwright passed 1/1 Chromium desktop test; full desktop Vitest passed 18/18
files and 399/399 tests; desktop production build passed with the existing
Vite large-chunk warning; single-worker R2/R3 Playwright smoke passed 18/18
tests.

Boundary: no rule-pack schema, expression grammar, parser/text syntax,
backend completeness/evaluator behavior, rule-pack persistence, solver
behavior, unit-conversion API, DEC-018 catalog constant, schema dimension
enum, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail editor-contract unit contract visibility (`TP-UNITS-BTAIL-EDITORCONTRACTUNITS-001`)

Landed one bounded Phase B-tail editor-contract unit-contract slice while C5.7
remains human-execution gated. The DEL-07-03 Editor Contract review panel now
exposes the DEL-02-02 unit contract already carried in its exported packet.

The tranche adds a visible `editor-contract-unit-contract` row reporting
`contract=DEL-02-02`, the dimension schema ref
`schemas/units.schema.yaml#/$defs/DimensionId`, the
`unit_bearing_values_require_explicit_unit_metadata` policy, and
`missing=diagnostic_blocking`. The exported packet is unchanged in shape for
downstream consumers; the visible UI now witnesses the same contract.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-227; DEL-07-03 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-EDITORCONTRACTUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 selected test; focused Playwright
passed 1/1 Chromium desktop test; full desktop Vitest passed 18/18 files and
399/399 tests; desktop production build passed with the existing Vite
large-chunk warning; single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: no editor persistence, accepted model-state mutation,
operation-applier behavior, solver behavior, unit-conversion API, DEC-018
catalog constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-17 - B-tail secret/private-library unit policy visibility (`TP-UNITS-BTAIL-SECRETPRIVATEUNITPOLICY-001`)

Landed one bounded Phase B-tail secret/private-library unit-policy slice while
C5.7 remains human-execution gated. The DEL-12-04 Secret & Private Libraries
panel now exposes how unit-bearing private material/rule references are handled
when private payloads are withheld.

The tranche adds a `unit_policy` section to the exported secret/private packet
and a visible `secret-private-library-unit-policy` row. It records two
metadata-only private unit references, explicit unit metadata required at use,
`unit_payload_included=false`, `conversion_performed=false`, and
`repository_default_private_write=false`. Per-reference statuses distinguish
private material values from private rule input values without reading or
serializing those private payloads.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-226; DEL-12-04 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SECRETPRIVATEUNITPOLICY-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 selected test after the first
focused run caught and fixed a helper destructuring miss; focused Playwright
passed 1/1 Chromium desktop test; full desktop Vitest passed 18/18 files and
399/399 tests; desktop production build passed with the existing Vite
large-chunk warning; single-worker R2/R3 Playwright smoke passed 18/18 tests.

Boundary: no private library payload read or write, credential handling,
storage-root finalization, unit-conversion API, DEC-018 catalog constant,
schema dimension enum, protected standards content, private payload, lifecycle
state transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail viewport placeholder unit validation (`TP-UNITS-BTAIL-VIEWPORTPLACEHOLDERUNITS-001`)

Landed one bounded Phase B-tail viewport placeholder unit-validation slice
while C5.7 remains human-execution gated. The DEL-07-01 one-click viewport
gesture placeholders for `create_node` and `connect_pipe_run` now record
length unit-dimension validation evidence instead of `unit_validation=not_run`.

The tranche preserves review-only `pending_service_validation`, direct model
mutation disallowance, and service-validation requirements. It changes the
placeholder intent metadata to carry `unit=m`, `dimension=length`, and
`unit_validation=length=<status>` through the same unit route used by explicit
viewport geometry. Browser preview records
`length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`.
The component-symbol placeholder remains `not_required_dimensionless`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-225; DEL-07-01 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTPLACEHOLDERUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 selected test; focused Playwright
passed 2/2 tests; full desktop Vitest passed 18/18 files and 399/399 tests;
single-worker R2/R3 Playwright smoke passed 18/18 tests; desktop production
build passed with the existing Vite large-chunk warning. A DEC-025 sweep
attempt exposed a timing-sensitive broad-smoke viewport animation sample; the
R2 desktop preview smoke passed 1/1 after replacing the single sample with a
bounded polling assertion. One exploratory full Playwright run without
`--workers=1` printed all 18 passing tests but was interrupted after a delayed
exit and is not counted as validation evidence.
DEC-025 sweep evidence is recorded separately at closeout, not as a lifecycle,
release, professional, certification, sealing, authentication, or
code-compliance claim.

Boundary: no accepted model-state mutation, operation-applier validation
semantics, solver behavior, component schema, unit-conversion API, DEC-018
catalog constant, schema dimension enum, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-17 - B-tail diagnostic linked-result unit context (`TP-UNITS-BTAIL-DIAGNOSTICUNITS-001`)

Landed one bounded Phase B-tail diagnostic review-surface slice while C5.7
remains human-execution gated. The DEL-07-07 Diagnostic Detail panel now
records explicit unit context for result rows linked to the selected
diagnostic.

The tranche adds first-class `unit` and `unit_source` fields to diagnostic
linked-result interpretation records and renders
`data-testid="diagnostic-unit-context"` with linked result count, result unit
symbols in linked-result order, `source=result_envelope`, and
`conversion=false`. For `HIGH_DISPLACEMENT_REVIEW`, the selected node-level
diagnostic links 21 result rows with units `mm,rad`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-224; DEL-07-07 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-DIAGNOSTICUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 selected test; focused Playwright
passed 2/2 tests; full desktop Vitest passed 18/18 files and 399/399 tests;
R2/R3 Playwright smoke passed 18/18 tests; desktop production build passed
with the existing Vite large-chunk warning. DEC-025 sweep evidence is
recorded separately at closeout, not as a lifecycle, release, professional,
certification, sealing, authentication, or code-compliance claim.

Boundary: no diagnostic schema, solver behavior, result value, unit-conversion
API, DEC-018 catalog constant, schema dimension enum, protected standards
content, private payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-17 - B-tail design-knowledge computed-unit context (`TP-UNITS-BTAIL-KNOWLEDGEUNITS-001`)

Landed one bounded Phase B-tail design-knowledge review-surface slice while
C5.7 remains human-execution gated. The DEL-13-01 Design Knowledge panel now
records explicit unit context for computed result-derived knowledge records.

The tranche adds `data-testid="knowledge-unit-context"` with the count of
computed unit-bearing knowledge result refs, result unit symbols in the
established Results-panel display order, `source=computed_preview_result`, and
`conversion=false`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-223; DEL-13-01 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-KNOWLEDGEUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 file and 56/56 tests; focused R2
Playwright smoke passed 2/2; full desktop Vitest passed 18/18 files and
399/399 tests; desktop production build passed with the existing Vite
large-chunk warning. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: no design-knowledge schema, solver behavior, result value,
unit-conversion API, DEC-018 catalog constant, schema dimension enum,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail run-audit unit traceability evidence (`TP-UNITS-BTAIL-RUNAUDITUNITS-001`)

Landed one bounded Phase B-tail analysis-run audit evidence slice while C5.7
remains human-execution gated. The DEL-14-02 Run Audit panel now records unit
traceability for solved runs from the already-bound model and result envelope.

The tranche adds `data-testid="run-audit-units"` with sorted model unit
declarations, result row unit symbols in the established Results-panel display
order, row count, `source=result_envelope`, and `conversion=false`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-222; DEL-14-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RUNAUDITUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 file and 56/56 tests; focused R2
Playwright smoke passed 2/2; full desktop Vitest passed 18/18 files and
399/399 tests; desktop production build passed with the existing Vite
large-chunk warning. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: no analysis-run schema, hash canonicalization, solver behavior,
result value, unit-conversion API, DEC-018 catalog constant, schema dimension
enum, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail accessibility-baseline unit-visibility evidence (`TP-UNITS-BTAIL-A11YUNITVISIBILITY-001`)

Landed one bounded Phase B-tail/C5 usability-support slice while C5.7 remains
human-execution gated. The DEL-07-06 Accessibility Baseline panel and exported
packet now record explicit unit-visibility evidence for unit-bearing review
surfaces.

The tranche adds `unit_visibility_evidence` with DEC-018/DEL-02-02/
DEL-07-05/DEL-07-06 basis refs, sorted project units, a review-surface unit
label visibility policy, no color-only unit signaling,
`default_units_inferred=false`, and `conversion_performed=false`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-221; DEL-07-06 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-A11YUNITVISIBILITY-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 file and 56/56 tests; focused R2
Playwright from-blank journey smoke passed 2/2; full desktop Vitest passed
18/18 files and 399/399 tests; desktop production build passed with the
existing Vite large-chunk warning. DEC-025 sweep evidence is recorded
separately at closeout, not as a lifecycle, release, professional,
certification, sealing, authentication, or code-compliance claim.

Boundary: no final accessibility conformance target, desktop
accessibility-tree audit claim, solver behavior, rule-check behavior,
unit-conversion API, DEC-018 catalog constant, protected standards content,
private payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-17 - B-tail missing-data unit-input policy visibility (`TP-UNITS-BTAIL-MISSINGDATAUNITPOLICY-001`)

Landed one bounded Phase B-tail missing-data/unit-policy slice while C5.7
remains human-execution gated. The DEL-07-04 Missing Data Blocking panel and
exported warning packet now record explicit unit-input policy evidence for
unit-bearing missing inputs.

The tranche adds `unit_input_policy_evidence` with DEC-018/DEL-02-02/
DEL-07-04 basis refs, sorted project units, an explicit-unit requirement for
unit-bearing inputs, `default_units_inferred=false`,
`silent_unit_defaults_used=false`, `auto_fill_missing_units=false`, and
`conversion_performed=false`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-220; DEL-07-04 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-MISSINGDATAUNITPOLICY-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1 file and 56/56 tests; focused R2
Playwright from-blank journey smoke passed 2/2; full desktop Vitest passed
18/18 files and 399/399 tests; desktop production build passed with the
existing Vite large-chunk warning. DEC-025 sweep evidence is recorded
separately at closeout, not as a lifecycle, release, professional,
certification, sealing, authentication, or code-compliance claim.

Boundary: no solver behavior, rule-check behavior, unit-conversion API,
DEC-018 catalog constant, schema dimension enum, protected standards content,
private payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-17 - B-tail validation-evidence unit-policy visibility (`TP-UNITS-BTAIL-VALIDATIONEVIDUNITS-001`)

Landed one bounded Phase B-tail validation-evidence/unit-policy slice while
C5.7 remains human-execution gated. The DEL-09-05 Validation Evidence panel
and exported packet now record DEC-018 project-unit context for the
manual/evidence inventory without asserting release thresholds.

The tranche adds `unit_policy_evidence` with
`unit-system:dec-018-si-dual-display`, entered-unit preservation, sorted
project units, 18 unit-bearing model records, the
`unit_and_schema_verification` manual section,
`conversion_policy=validation_evidence_inventory_records_unit_context_without_conversion`,
`conversion_performed=false`, and `release_gate_threshold_policy=TBD`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-219; DEL-09-05 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VALIDATIONEVIDUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 1/1; focused R2 Playwright from-blank
journey smoke passed 2/2; full desktop Vitest passed 18/18 files and 399/399
tests; desktop production build passed with the existing Vite large-chunk
warning. DEC-025 sweep evidence is recorded separately at closeout, not as a
lifecycle, release, professional, certification, sealing, authentication, or
code-compliance claim.

Boundary: no release threshold, release authorization, persistence behavior,
solver behavior, unit-conversion API, DEC-018 catalog constant, protected
standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail viewport component-symbol unit-validation evidence (`TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001`)

Landed one bounded Phase B-tail viewport component-symbol unit-validation
slice while C5.7 remains human-execution gated. The reference-only viewport
`insert_component_symbol` placeholder now records
`unit_validation=not_required_dimensionless`.

The tranche preserved the prior geometry boundary at the time: explicit
viewport node and pipe authoring paths already carried length unit-validation
evidence. Generic `create_node` and `connect_pipe_run` placeholders were later
covered by TP-MAC-225.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-218; DEL-07-01 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTCOMPUNITVALID-001.md`;
DEL-16-02 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest passed 1/1; focused R2/R3 Playwright smoke
passed 16/16; full desktop Vitest passed 18/18 files and 399/399 tests;
desktop production build passed with the existing Vite large-chunk warning.
DEC-025 sweep evidence is recorded separately at closeout, not as lifecycle,
release, professional, certification, sealing, authentication, or
code-compliance claims.

Boundary: no operation application behavior, accepted model-state mutation,
durable persistence, schema enum, unit-conversion API, DEC-018 catalog
constant, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail load-manager dimensionless unit-validation evidence (`TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001`)

Landed one bounded Phase B-tail Load Cases manager unit-validation slice while
C5.7 remains human-execution gated. Empty load-case shell creation, load-case
metadata edits, whole load-case deletion, combination creation, combination
basis/factor edits, combination term creation/deletion, and whole-combination
deletion now record `unit_validation=not_required_dimensionless` in the
existing operation-intent validation field.

The tranche documents existing non-unit-bearing behavior: the affected
operations already declare `unit=none` and `dimension=dimensionless`. Unit-
bearing primitive load create/edit paths remain routed through the existing
unit/dimension validation helper.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-217; DEL-05-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001.md`;
DEL-07-02, DEL-16-02, and DEL-02-02 supporting run records with the same id.

Validation: focused manager App Vitest passed 18/18; full desktop Vitest
passed 18/18 files and 399/399 tests; focused R2/R3 Playwright smoke passed
14/14; desktop production build passed with the existing Vite large-chunk
warning. DEC-025 sweep evidence is recorded separately at closeout, not as a
lifecycle, release, professional, certification, sealing, authentication, or
code-compliance claim.

Boundary: no load-case algebra solver behavior, operation application
behavior, accepted model-state mutation, durable persistence, schema enum,
unit-conversion API, DEC-018 catalog constant, protected standards content,
private payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

---

## 2026-06-17 - B-tail delete-intent unit-validation evidence (`TP-UNITS-BTAIL-DELETEINTENTUNITVALID-001`)

Landed one bounded Phase B-tail Property Inspector/delete-intent
unit-validation slice while C5.7 remains human-execution gated. Explicit
support, node, and pipe delete intent previews now record unit validation as
`not_required_dimensionless` instead of leaving `unit_validation=not_run`.

The tranche documents the existing behavior that these delete intents do not
introduce new unit-bearing numeric quantities. Schema and reference checks
remain in the existing validate/apply path; delete operation behavior,
accepted model-state mutation semantics, and solver behavior are unchanged.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-216; DEL-07-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-DELETEINTENTUNITVALID-001.md`;
DEL-16-02 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest passed 56/56; focused R2/R3 Playwright smoke
file passed 14/14; full desktop Vitest passed 18/18 files and 399/399 tests;
desktop production build passed with the existing Vite large-chunk warning;
`git diff --check` passed. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: no delete operation behavior, reference validation, operation
application semantics, accepted model-state mutation, solver behavior, unit
conversion API, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail result viewer unit policy evidence (`TP-UNITS-BTAIL-RESULTVIEWUNITS-001`)

Landed one bounded Phase B-tail results-viewer/unit-evidence slice while C5.7
remains human-execution gated. The DEL-07-05 Results viewer now visibly
records the DEC-018 result-unit policy for the solved preview result envelope
before filtering and pagination.

The tranche adds `result-unit-policy` to the result controls with
`unit-system:dec-018-si-dual-display` policy basis, entered-unit preservation,
source result units `MPa,N,N*m,mm,rad`, 737 source result rows,
`conversion_policy=result_view_preserves_result_row_units_without_conversion`,
and `conversion_performed=false`. The result table continues to display each
row's source unit; no result math, filtering, pagination, solver behavior, or
tolerance behavior changed.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-215; DEL-07-05 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RESULTVIEWUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 56/56; focused R2/R3 Playwright smoke
file passed 14/14; full desktop Vitest passed 18/18 files and 399/399 tests
on rerun after one unrelated transient support-label cross-test failure;
desktop production build passed with the existing Vite large-chunk warning;
`git diff --check` passed. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: no result math, result filtering, pagination semantics, solver
behavior, comparison delta math, tolerance profile, default tolerance, unit
conversion API, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail project validation unit policy evidence (`TP-UNITS-BTAIL-PROJECTVALIDATIONUNITS-001`)

Landed one bounded Phase B-tail project-validation/unit-evidence slice while
C5.7 remains human-execution gated. The DEL-02-05 Project Validation Preflight
packet now records explicit DEC-018 unit-policy evidence for the existing
unit round-trip metadata check.

The tranche adds `unit_policy_evidence` to the validation preflight export
with `unit-system:dec-018-si-dual-display`, entered-unit preservation, sorted
model units, the model unit-bearing record count, persisted round-trip
status/signature when a local snapshot exists,
`conversion_policy=project_validation_records_unit_round_trip_metadata_without_conversion`,
`conversion_performed=false`, and DEC-018/DEL-02-02/DEL-02-05 basis refs. The
rendered project-validation panel now exposes `project-validation-unit-policy`
so the same policy is visible in the browser.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-214; DEL-02-05 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PROJECTVALIDATIONUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 56/56; focused R2/R3 Playwright smoke
file passed 14/14; full desktop Vitest passed 18/18 files and 399/399 tests;
desktop production build passed with the existing Vite large-chunk warning;
`git diff --check` passed. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: no persistence semantics, migration policy, hash canonicalization,
schema versioning, unit conversion API, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

---

## 2026-06-17 - B-tail design workspace unit policy evidence (`TP-UNITS-BTAIL-DESIGNWORKSPACEUNITS-001`)

Landed one bounded Phase B-tail design-workspace/unit-evidence slice while
C5.7 remains human-execution gated. The DEL-07-08 design-authoring workspace
packet now records explicit unit-policy evidence for the composed model,
result, analysis-run, and comparison context exposed by the Results section.

The tranche adds `unit_policy_evidence` to the workspace export with
`unit-system:dec-018-si-dual-display`, entered-unit preservation, sorted model
units, result units, comparison units, the embedded comparison
unit-policy-evidence ref, `conversion_performed=false`,
`tolerance_profile_ref=TBD`, and DEC-018/DEC-026/DEL-02-02/DEL-14-04 basis
refs. The rendered workspace now exposes `design-workspace-units` so the same
policy is visible in the browser.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-213; DEL-07-08 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-DESIGNWORKSPACEUNITS-001.md`;
DEL-02-02 and DEL-14-04 supporting run records with the same id.

Validation: focused App Vitest passed 56/56; focused R2/R3 Playwright smoke
file passed 14/14; full desktop Vitest passed 18/18 files and 399/399 tests;
desktop production build passed with the existing Vite large-chunk warning;
`git diff --check` passed. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: no model mutation, comparison delta math, tolerance profile, default
tolerance, solver behavior, external validation decision, conversion API,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

---

## 2026-06-17 - B-tail comparison workspace unit policy evidence (`TP-UNITS-BTAIL-COMPARISONGUIUNITS-001`)

Landed one bounded Phase B-tail comparison-workspace/unit-evidence slice while
C5.7 remains human-execution gated. The desktop comparison packet now records
explicit unit-policy evidence for mapped result deltas, and the comparison
panel exposes that policy beside the tolerance boundary.

The tranche records the existing invariant that comparison rows are mapped
only when stable result references match and the left/right result units are
equal. The packet now carries `unit_policy_evidence` with DEC-018/DEC-026/
DEL-14-05 basis refs, matched result units, unmatched row counts,
`conversion_policy=comparison_workspace_preserves_result_units_without_conversion`,
`conversion_performed=false`, and `tolerance_profile_ref=TBD`. No delta math,
tolerance profile, or classification behavior changed.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-212; DEL-14-04 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-COMPARISONGUIUNITS-001.md`;
DEL-14-05 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest passed 56/56; focused R2/R3 Playwright smoke
file passed 14/14; full desktop Vitest passed 18/18 files and 399/399 tests;
desktop production build passed with the existing Vite large-chunk warning;
`git diff --check` passed. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: comparison workspace evidence only. No comparison delta math,
tolerance profile, default tolerance, solver convergence policy, external
validation decision, conversion API, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

## 2026-06-17 - B-tail external-prover unit policy evidence (`TP-UNITS-BTAIL-EXTERNALPROVERUNITS-001`)

Landed one bounded Phase B-tail target-format/unit-evidence slice while C5.7
remains human-execution gated. The desktop external-prover boundary metadata
panel now exposes explicit DEC-018 unit-policy evidence for the metadata-only
external review context.

The tranche reuses the shared export unit disclosure helper so the exported
metadata JSON records `unit-system:dec-018-si-dual-display`,
`entered_units_preserved`, source model units, result units when an analysis
run exists, empty target export units, and
`conversion_policy=external_prover_metadata_records_units_without_target_conversion`
with `conversion_performed=false`. The visible
`external-prover-unit-policy` row reports the same source/result unit
disclosure, conversion status, policy, and witness count.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-211; DEL-15-04 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-EXTERNALPROVERUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 55/55; full desktop Vitest passed 18/18
files and 398/398 tests; focused R2/R3 Playwright smoke file passed 14/14;
desktop production build passed with the existing Vite large-chunk warning;
`git diff --check` passed. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: external-prover metadata evidence only. No external solver/prover
invocation, target parser, commercial-result ingestion, target writer,
conversion API, target compatibility claim, protected standards content,
private payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

## 2026-06-17 - B-tail primitive-load delete unit-validation evidence (`TP-UNITS-BTAIL-PRIMDELETEUNITVALID-001`)

Landed one bounded Phase B-tail load-case/unit-validation slice while C5.7
remains human-execution gated. Explicit primitive-load delete intents now
carry the existing primitive's unit-dimension validation evidence in the
visible delete preview instead of `unit_validation=not_run`.

The tranche reuses the existing DEC-018 unit-catalog route already used by
primitive-load create and magnitude-edit intents. Browser preview records
`model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`;
desktop/Tauri routes can record DEC-018 accepted, mismatch, loading, or
unreviewed statuses through the same shared helper. The operation semantics
remain a delete of the selected indexed primitive; no conversion or load-engine
behavior changed.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-210; DEL-05-01 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PRIMDELETEUNITVALID-001.md`;
DEL-02-02 and DEL-05-05 supporting run records with the same id.

Validation: focused App Vitest passed 55/55; full desktop Vitest passed 18/18
files and 398/398 tests; focused R2/R3 Playwright smoke file passed 14/14;
desktop production build passed with the existing Vite large-chunk warning;
`git diff --check` passed. DEC-025 sweep evidence is recorded separately at
closeout, not as a lifecycle, release, professional, certification, sealing,
authentication, or code-compliance claim.

Boundary: primitive-load delete intent evidence only. No DEC-018 catalog
constant change, schema dimension enum change, unit conversion API change,
operation-applier validation semantics, solver behavior, load engine behavior,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail viewport geometry unit-validation evidence (`TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001`)

Landed one bounded Phase B-tail viewport-geometry/unit-validation slice while
C5.7 remains human-execution gated. Explicit viewport node and straight-pipe
geometry intents now carry length unit-dimension validation evidence in their
queued operation records instead of `unit_validation=not_run`.

The tranche reuses the existing DEC-018 unit-catalog route already displayed
by the viewport forms. Browser preview records
`length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`;
desktop/Tauri routes can record DEC-018 accepted, mismatch, loading, or
unreviewed statuses through the same shared helper. Generic one-click viewport
gesture placeholders were later covered by TP-MAC-225.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-209; DEL-07-01 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VIEWPORTGEOMUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 55/55; focused R2/R3 Playwright smoke
file passed 14/14; full desktop Vitest passed 18/18 files and 398/398 tests;
desktop production build passed with the existing Vite large-chunk warning.
DEC-025 sweep evidence is recorded separately at closeout, not as a lifecycle,
release, professional, certification, sealing, authentication, or
code-compliance claim.

Boundary: viewport intent evidence only. No DEC-018 catalog constant change,
schema dimension enum change, unit conversion API change, operation-applier
validation semantics, solver behavior, protected standards content, private
payload, lifecycle state transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

## 2026-06-17 - B-tail PCF conversion witness visibility (`TP-UNITS-BTAIL-PCFCONVWITNESSVISIBLE-001`)

Landed one bounded Phase B-tail PCF/export-evidence slice while C5.7 remains
human-execution gated. The Conservative PCF Export panel now shows the existing
PCF conversion-witness sidecar count, scope, and target length unit in visible
browser UI.

The tranche does not change the PCF package schema or conversion behavior. It
surfaces the existing `conversion_witnesses.json` evidence line in the panel
and extends App and Playwright assertions so the visible desktop smoke proves
`count=23` and `target_length=MM`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-208; DEL-17-07 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PCFCONVWITNESSVISIBLE-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 55/55; focused R2/R3 Playwright smoke
file passed 14/14; full desktop Vitest passed 18/18 files and 398/398 tests;
desktop production build passed with the existing Vite large-chunk warning.

Boundary: PCF desktop visibility/evidence only. No PCF package schema change,
conversion formula change, target compatibility claim, downstream import
evidence, solver-validation claim, unit conversion API, protected standards
content, private payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

## 2026-06-17 - B-tail rule-pack unit policy evidence (`TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001`)

Landed one bounded Phase B-tail rule-pack editor/unit-policy slice while C5.7
remains human-execution gated. The Rule-Pack Manager now shows visible unit
policy and unit-dimension validation status for declaration `quantity_intent`
unit refs and expression literal/table unit refs.

Browser preview preserves the existing manual stored-unit text entry route and
records `catalog_route=browser_preview_manual_entry` with `conversion=false`.
Desktop/Tauri catalog routes continue using the reviewed DEC-018 selectors and
can record accepted dimension matches or explicit mismatch/unreviewed/loading
statuses.

The tranche preserves the rule-pack editor boundary. It does not change the
rule-pack schema, evaluator normalization, parser/text syntax, backend
validation/persistence behavior, or DEC-018 catalog constants.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-207; DEL-07-03 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RULEPACKUNITPOLICY-001.md`;
DEL-02-02, DEL-06-01, and DEL-06-02 supporting run records with the same id.

Validation: focused rule-pack/unit Vitest passed 67/67; focused R2/R3
Playwright smoke file passed 14/14; full desktop Vitest passed 18/18 files and
398/398 tests; desktop production build passed with the existing Vite
large-chunk warning.

Boundary: rule-pack editor unit-policy metadata only. No DEC-018 catalog
constant change, schema dimension enum change, rule-pack schema change,
evaluator normalization change, parser/text-syntax change, backend
validation/persistence behavior change, protected standards content, private
value payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

## 2026-06-17 - B-tail property-inspector unit validation evidence (`TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001`)

Landed one bounded Phase B-tail Property Inspector/unit-validation slice while
C5.7 remains human-execution gated. The desktop Property Inspector now records
unit-dimension validation status on unit-bearing modify intents and explicit
create-section/create-material/create-support intents.

Unit-bearing Property Inspector intents no longer report
`unit_validation=not_run`. Browser preview records
`model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`
because the DEC-018 catalog command is desktop-only; desktop/Tauri catalog
routes can record `dec018_catalog_dimension_match` for accepted matches or
explicit mismatch/unreviewed/loading statuses.

The tranche preserves the Property Inspector and operation-review boundary. It
does not change solver behavior, apply operations automatically, add a unit
conversion API, or change catalog constants/schema enums.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-206; DEL-07-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-PROPERTYINTENTUNITVALID-001.md`;
DEL-02-02 and DEL-16-02 supporting run records with the same id.

Validation: focused App Vitest passed 55/55; focused R2/R3 Playwright smoke
file passed 14/14; full desktop Vitest passed 18/18 files and 397/397 tests;
desktop production build passed with the existing Vite large-chunk warning.

Boundary: Property Inspector operation-intent metadata only. No DEC-018
catalog constant change, schema dimension enum change, unit conversion API
change, solver behavior, operation application semantics, accepted model-state
mutation, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail load-case primitive unit validation evidence (`TP-UNITS-BTAIL-LOADCASEUNITVALID-001`)

Landed one bounded Phase B-tail load-case/unit-validation slice while C5.7
remains human-execution gated. The desktop Load Cases manager now records
unit-dimension validation status on unit-bearing primitive-load create and
magnitude-edit operation intents.

Primitive create/edit intents no longer report `unit_validation=not_run` for
unit-bearing quantities. Browser preview records
`model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview`
because the DEC-018 catalog command is desktop-only; desktop/Tauri catalog
routes can record `dec018_catalog_dimension_match` for accepted matches or
explicit mismatch/unreviewed statuses. The visible create and edit previews
display the validation status next to the already explicit unit and dimension.

The tranche preserves the load-case and operation-review boundary. It does not
change solver behavior, apply operations automatically, add a unit conversion
API, or change catalog constants/schema enums.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-205; DEL-05-01 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOADCASEUNITVALID-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused App Vitest passed 55/55; focused R2 Playwright smoke
passed 2/2; full desktop Vitest passed 18/18 files and 397/397 tests; desktop
production build passed with the existing Vite large-chunk warning.

Boundary: load-case operation-intent metadata only. No DEC-018 catalog
constant change, schema dimension enum change, unit conversion API change,
solver behavior, operation application semantics, protected standards content,
private payload, lifecycle state transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

## 2026-06-17 - B-tail export review unit policy inventory (`TP-UNITS-BTAIL-EXPORTREVIEWUNITS-001`)

Landed one bounded Phase B-tail export-review/unit-evidence inventory slice
while C5.7 remains human-execution gated. The desktop Export Safety Review
manifest now emits a top-level DEC-018 `unit_policy_summary` for its local
export records.

The manifest records `unit-system:dec-018-si-dual-display`,
`entered_units_preserved`,
`conversion_policy=export_review_manifest_inventory_only_no_target_conversion`,
and `conversion_performed=false`. A 27-row `unit_evidence_matrix` separates
unit-bearing export records from metadata-only storage/security/readiness
reviews. After mechanics preview, fourteen unit-bearing records are covered
and the visible UI shows `export-review-units` with `covered=14/14`.

The tranche preserves the export-review boundary. It does not add a
target-specific writer, perform manifest-level unit conversion, alter runtime
redaction behavior, create public transport, or make release/professional/code
claims.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-204; DEL-12-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-EXPORTREVIEWUNITS-001.md`;
DEL-02-02 and DEL-08-04 supporting run records with the same id.

Validation: focused App Vitest passed 55/55; focused R2 Playwright smoke
passed 2/2; full desktop Vitest passed 18/18 files and 397/397 tests; desktop
production build passed with the existing Vite large-chunk warning.

Boundary: export-review metadata inventory only. No DEC-018 catalog constant
change, schema dimension enum change, unit conversion API change,
target-specific writer, public transport commitment, runtime redaction rule
change, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail adapter framework unit policy evidence (`TP-UNITS-BTAIL-ADAPTERFRAMEWORKUNITS-001`)

Landed one bounded Phase B-tail adapter-framework/unit-evidence slice while
C5.7 remains human-execution gated. The desktop Adapter Framework packet now
emits explicit DEC-018 unit-policy evidence for the format-neutral adapter
contract.

The packet records `unit-system:dec-018-si-dual-display`,
`entered_units_preserved`, source/result unit disclosure,
`conversion_policy=adapter_framework_declares_unit_validation_no_format_conversion`,
and `conversion_performed=false`. The visible UI shows
`adapter-framework-units` with `witnesses=1` and the framework-level unit
validation policy.

The tranche preserves the adapter-framework boundary. It does not choose a
concrete external format, add a target-specific writer, add a unit conversion
API, add a runtime loader/public endpoint, make a target support claim, or
make target compatibility/professional/code-compliance claims.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-203; DEL-10-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-ADAPTERFRAMEWORKUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: adapter framework contract test passed; focused App Vitest passed
55/55; focused R2 Playwright smoke passed 2/2; full desktop Vitest passed
18/18 files and 397/397 tests; desktop production build passed with the
existing Vite large-chunk warning.

Boundary: adapter-framework unit metadata only. No DEC-018 catalog constant
change, schema dimension enum change, unit conversion API change, concrete
external format, target-specific writer, external execution, protected
standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail operation diff preview unit witnesses (`TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001`)

Landed one bounded Phase B-tail operation-diff/unit-evidence slice while C5.7
remains human-execution gated. The desktop Operation Diff Preview packet now
emits explicit DEC-018 unit-system disclosure and per-unit-bearing-diff-row
unit-preservation witnesses for queued local operation previews.

The diff packet records `unit-system:dec-018-si-dual-display`, target diff rows
as `per_change_declared_unit`, and `conversion_performed=false`. Each
unit-bearing diff row gets a preservation witness for before/after value text,
unit, and dimension. The visible UI shows `diff-preview-units` and
`diff-preview-unit-witnesses` with `count=1` and `conversion=false` in the
focused material-modulus edit fixture.

The tranche preserves the operation-review boundary. It does not apply queued
operations, mutate accepted model state, add durable operation acceptance
persistence, change the operation schema, or make professional/code-compliance
claims.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-202; DEL-16-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-OPDIFFUNITWITNESS-001.md`;
DEL-16-03 and DEL-02-02 supporting run records with the same id.

Validation: focused App Vitest passed 55/55; focused R2 Playwright smoke
passed 2/2; full desktop Vitest passed 18/18 files and 397/397 tests; desktop
production build passed with the existing Vite large-chunk warning; `git diff
--check` passed. Playwright smoke was not extended with a witness-specific
assertion because the broad R2 smoke does not retain a queued diff row without
altering later flow; an attempted queue-and-clear path hung and was removed.

Boundary: operation diff unit metadata only. No operation schema change,
operation application, accepted model-state mutation, durable acceptance
persistence, unit conversion API, protected standards content, private payload,
lifecycle state transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail CAEPIPE external parser unit witnesses (`TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001`)

Landed one bounded Phase B-tail CAEPIPE external parser/unit-evidence slice
while C5.7 remains human-execution gated. The desktop CAEPIPE External Harness
panel now emits explicit DEC-018 unit-system disclosure and per-parser-row
unit-preservation witnesses for the invented public CSV parser fixture.

The parser package records `unit-system:dec-018-si-dual-display`, target units
for node displacements and element forces, and `conversion_performed=false`.
Each parsed CSV row gets a preservation witness for source value, unit, and
inferred row dimension. The visible UI shows `caepipe-external-units` and
`caepipe-external-unit-witnesses` with `count=3` and `conversion=false`.

The tranche preserves the parser-only external harness boundary. It does not
attempt external CAEPIPE execution, require an executable, license, or path,
assert CAEPIPE compatibility, validate solver behavior, or make professional
or code-compliance claims.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-201; DEL-17-05 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001.md`;
DEL-02-02 and DEL-08-04 supporting run records with the same id.

Validation: focused App Vitest passed 55/55; focused R2 Playwright smoke passed
2/2; full desktop Vitest passed 18/18 files and 397/397 tests; desktop
production build passed with the existing Vite large-chunk warning; `git diff
--check` passed.

Boundary: CAEPIPE external parser unit metadata only. No external CAEPIPE
execution, executable/license/path requirement, target compatibility claim,
solver validation claim, schema contract change, unit conversion API,
protected standards content, private payload, lifecycle state transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail headless runner unit witnesses (`TP-UNITS-BTAIL-HEADLESSRUNNERUNITWITNESS-001`)

Landed one bounded Phase B-tail headless-runner/unit-evidence slice while C5.7
remains human-execution gated. The desktop Headless Runner envelope now emits
explicit DEC-018 unit-system disclosure and per-result unit-preservation
witnesses for the schema-first local result-handoff preview.

The runner result records `unit-system:dec-018-si-dual-display`,
entered-unit storage convention, source model units, result units, and
`conversion_performed=false`. Each finite result row gets a preservation
witness for source value, unit, and inferred result dimension. The visible UI
shows `headless-runner-units` and `headless-runner-unit-witnesses` with
`count=737` and `conversion=false` for the invented preview fixture after
mechanics preview; the pre-run preview state remains explicit with zero
witnesses and no result units.

The tranche preserves the headless-runner TBD boundary. It does not choose
final CLI syntax, package scripts, process/network/filesystem policy, public
transport, CI/release matrix, runtime process launching, or a schema/public
runtime contract beyond the existing desktop preview JSON.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-200; DEL-10-05 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-HEADLESSRUNNERUNITWITNESS-001.md`;
DEL-02-02 and DEL-08-04 supporting run records with the same id.

Validation: focused App Vitest passed 55/55; focused R2 Playwright smoke passed
2/2; full desktop Vitest passed 18/18 files and 397/397 tests; desktop
production build passed with the existing Vite large-chunk warning.

Boundary: headless-runner result-handoff unit metadata only. No final CLI
syntax, package script, process/network/filesystem policy, public transport,
CI/release matrix, runtime process launcher, schema contract change, unit
conversion API, protected standards content, private payload, lifecycle state
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail handoff package unit witnesses (`TP-UNITS-BTAIL-HANDOFFUNITWITNESS-001`)

Landed one bounded Phase B-tail handoff/unit-evidence slice while C5.7 remains
human-execution gated. The desktop Handoff Package panel now emits explicit
DEC-018 unit-system disclosure and per-result unit-preservation witnesses in
the local review handoff package.

The package records `unit-system:dec-018-si-dual-display`, entered-unit storage
convention, source model units, result units, target handoff units, and
`conversion_performed=false`. Each finite result row gets a preservation
witness for source value, unit, and inferred result dimension. The visible UI
shows `handoff-units` and `handoff-unit-witnesses` with `count=737` and
`conversion=false` for the invented preview fixture.

The target profile remains `native_open_json_preview` and
`stable_ids_only_not_target_specific`; target field coverage remains `TBD` and
external validation remains user-controlled. The tranche does not promote D-21,
assert target compatibility, or add an external prover execution path.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-199; DEL-15-01 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-HANDOFFUNITWITNESS-001.md`;
DEL-02-02 and DEL-17-03 supporting run records with the same id.

Validation: focused App Vitest passed 55/55; focused R2 Playwright smoke passed
2/2; full desktop Vitest passed 18/18 files and 397/397 tests; desktop
production build passed with the existing Vite large-chunk warning.

Boundary: handoff unit metadata only. No target-specific mapping, external
prover execution, downstream compatibility claim, D-21 scope promotion, schema
contract change, unit conversion API, protected standards content, private
payload, lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail section-library quantity unit helper (`TP-UNITS-BTAIL-SECLIBQTYUNITS-001`)

Landed one bounded Phase B-tail app unit-entry slice while C5.7 remains
human-execution gated. The Private Library Manager now exposes a section
quantity unit helper for section-library drafts. The helper drafts one private
section `dimensions[]` or `properties[]` quantity with explicit magnitude,
unit, dimension, provenance, and review status before the existing local-only
import validation/store actions run.

Browser preview remains honest: it does not synthesize a fallback catalog and
keeps the selected schema-native default unit as the single explicit option
with the desktop-only unit-catalog diagnostic. In Tauri/desktop mode, the
helper uses the reviewed DEC-018 unit catalog and filters section quantity
options by compatible dimensions, including area-compatible `m^2`/`in^2` for
`cross_section_area` while excluding incompatible length units.

The drafted section quantity is private-only:
`value_status=private_user_supplied`; property slots keep
`calculation_status=not_calculated`, so the helper does not claim the section
property was calculated or reviewed. It does not introduce public section
values, a section-property calculator, validation/storage policy changes, or
catalog/schema enum changes.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-198; DEL-07-03 and DEL-03-02 primary
run records
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SECLIBQTYUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused LibraryManagerPanel Vitest passed 15/15; full desktop
Vitest passed 18/18 files and 397/397 tests; desktop production build passed
with the existing Vite large-chunk warning; focused Playwright library-manager
smoke passed 2/2.

Boundary: section-library draft authoring only. No section-property
calculator, public section values, validation rule, local-store policy, schema
enum change, DEC-018 catalog constant change, protected standards content,
private payload, network/telemetry path, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail material-library property unit helper (`TP-UNITS-BTAIL-MATLIBFIELDUNITS-001`)

Landed one bounded Phase B-tail app unit-entry slice while C5.7 remains
human-execution gated. The Private Library Manager now exposes a material
property unit helper for material-library drafts. The helper drafts one private
`properties[]` quantity with explicit magnitude, schema-native unit ref,
dimension id, unit-required flag, and missing-unit behavior before the existing
local-only import validation/store actions run.

Browser preview remains honest: it does not synthesize a fallback catalog and
keeps the schema-native default unit ref for the selected property as the
single explicit option with the desktop-only unit-catalog diagnostic. In
Tauri/desktop mode, the helper uses the reviewed DEC-018 unit catalog and
filters material-property options by compatible dimensions, including
stress-compatible `Pa`/`MPa` for `elastic_modulus` while excluding incompatible
length units.

The drafted material property is private-only:
`value_status=private_user_supplied`, `required_for=mechanics_solve`,
`quantity_kind=unit_bearing`, `unit_required=true`, and
`missing_unit_behavior=diagnostic_blocking`. It does not introduce public
material values, engineering allowables, validation/storage policy changes, or
catalog/schema enum changes.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-197; DEL-07-03 and DEL-03-01 primary
run records
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-MATLIBFIELDUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused LibraryManagerPanel Vitest passed 13/13; full desktop
Vitest passed 18/18 files and 395/395 tests; desktop production build passed
with the existing Vite large-chunk warning; focused Playwright library-manager
smoke passed 2/2.

Boundary: material-library draft authoring only. No material engineering
allowables, public material values, validation rule, local-store policy,
schema enum change, DEC-018 catalog constant change, protected standards
content, private payload, network/telemetry path, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail export adapter SDK unit policy evidence (`TP-UNITS-BTAIL-ADAPTERSDKUNITS-001`)

Landed one bounded Phase B-tail target-format/unit-evidence slice while C5.7
remains human-execution gated. The DEL-17-09 export adapter SDK admission
package now carries a structured `unit_policy_evidence` member and
`unit_policy_evidence.json` manifest entry.

The evidence records the DEC-018 unit-system reference, entered-units-preserved
storage convention, source/result/target unit disclosure, explicit no-conversion
policy, `conversion_performed=false`, target refs, and a witness policy for
candidate targets that does not claim target-writer conversion, target support,
compatibility, release readiness, or professional reliance.

Desktop preview now shows the adapter-SDK unit evidence line and exports the
same packet member. Browser evidence covers both desktop and compact viewport
variants through the R2 smoke.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-196; DEL-17-09 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-ADAPTERSDKUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused export-adapter SDK Python test passed; focused App Vitest
passed 55/55; full desktop Vitest passed 18/18 files and 393/393 tests; desktop
production build passed with the existing Vite large-chunk warning; focused R2
Playwright smoke passed 2/2.

Boundary: adapter-admission metadata only. No runtime loader, public endpoint,
target-specific writer, conversion API, target support or compatibility claim,
external execution, protected standards content, private payload, lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail component-library field unit helper (`TP-UNITS-BTAIL-COMPLIBFIELDUNITS-001`)

Landed one bounded Phase B-tail app unit-entry slice while C5.7 remains
human-execution gated. The Private Library Manager now exposes a component
field unit helper for component-library drafts. The helper drafts one private
`fields[]` quantity with explicit magnitude, unit, and dimension metadata
before the existing local-only import validation/store actions run.

Browser preview remains honest: it does not synthesize a fallback catalog and
keeps the stored `N/m` unit as the single explicit option with the
desktop-only unit-catalog diagnostic. In Tauri/desktop mode, the helper uses
the reviewed DEC-018 unit catalog and filters `linear_stiffness` options to
compatible force-per-length units, including `N/m` and `lbf/in`, while
excluding incompatible length units.

The drafted component field is private-only:
`public_repository_value_policy=private_user_supplied_only`,
`value_status=private_user_supplied`, and
`dimension=linear_stiffness`. It does not introduce component mechanics,
public component values, SIF/flexibility tables, or validation/storage policy
changes.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-195; DEL-07-03 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-COMPLIBFIELDUNITS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: focused LibraryManagerPanel Vitest passed 11/11; full desktop
Vitest passed 18/18 files and 393/393 tests; desktop production build passed
with the existing Vite large-chunk warning; focused Playwright library-manager
smoke passed 2/2.

Boundary: component-library draft authoring only. No component mechanics,
code-specific SIF/flexibility data, public component catalog, validation rule,
local-store policy, protected standards content, private payload,
network/telemetry path, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

## 2026-06-17 - B-tail comparison mixed-unit tolerance corpus (`TP-UNITS-BTAIL-COMPTOLCORPUS-001`)

Landed one bounded Phase B-tail DEC-026 corpus slice while C5.7 remains
human-execution gated. The analysis-run comparison engine now recognizes
caller-supplied relative+absolute tolerance pairs when a governed profile
supplies both values, while retaining the existing scalar tolerance path for
older fixtures.

The comparison tolerance schema now defines optional
`relative_tolerance_value`, `absolute_tolerance_value`, and
`tolerance_pair_policy=relative_plus_absolute_floor` on tolerance rules. The
numeric pair path is still structurally limited to externally governed or
project-specific review status values and does not add defaults.

Corpus evidence covers mixed stress and force units: stress normalized from
`kPa` to `Pa` is classified by the relative allowance, near-zero force
normalized from `lbf` to `N` is classified by the absolute floor, and removing
the required conversion blocks with `ARC-UNIT-CONVERSION-UNSUPPORTED`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-194; DEL-14-04 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-COMPTOLCORPUS-001.md`;
DEL-14-05 and DEL-02-02 supporting run records with the same id.

Validation: focused analysis-run comparison and comparison-contract scripts
passed; adjacent pytest subset passed 23/23; full Python suite passed 360/360;
`git diff --check` passed.

Boundary: governed-profile comparison evidence only. No default tolerance,
release threshold, solver convergence policy, external validation decision,
protected standards content, private payload, network/telemetry path,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail result export unit witnesses (`TP-UNITS-BTAIL-RESULTEXPORTUNITWITNESS-001`)

Landed one bounded Phase B-tail target-format/unit-witness slice while C5.7
remains human-execution gated. The schema-first result export preview now
emits `unit_witness_policy` plus `unit_preservation_witnesses[]` for every
exported result row in the invented mechanics fixture. Each witness records
source result ref, source field path, source value/unit/dimension, target
result row path, target value/unit/dimension, unit-system ref, and
`conversion_performed=false`.

The strict result-export schema now defines optional `UnitPreservationWitness`
and `UnitPreservationQuantity` vocabulary under `ResultEnvelope`, without
changing existing accepted fixtures or reopening cross-deliverable trace-chain
ownership. The desktop panel displays the witness count and no-conversion
policy for the invented preview fixture.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-193; DEL-08-04 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-RESULTEXPORTUNITWITNESS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: result schema check passed; focused App Vitest passed 55/55; full
desktop Vitest passed 18/18 files and 391/391 tests; desktop production build
passed with the existing Vite large-chunk warning; `git diff --check` passed.

Boundary: result-envelope unit metadata only. No unit conversion, tolerance
policy, solver behavior, public transport commitment, trace-chain ownership
change, protected standards content, private payload, network/telemetry path,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail local FEA handoff unit witnesses (`TP-UNITS-BTAIL-LOCALFEAUNITWITNESS-001`)

Landed one bounded Phase B-tail target-format/unit-witness slice while C5.7
remains human-execution gated. The local FEA handoff package now emits
`unit_witness_policy` plus `unit_preservation_witnesses[]` for transfer-basis
displacement, force, and moment result refs. Each witness records source result
ref, source field path, source value/unit/dimension, target transfer-basis
path, unit-system ref, and `conversion_performed=false`.

The strict local FEA handoff schema now defines `UnitPreservationWitness` and
`UnitPreservationQuantity`, and the desktop panel displays the witness count
and no-conversion policy for the invented preview fixture.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-192; DEL-10-03 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOCALFEAUNITWITNESS-001.md`;
DEL-02-02 supporting run record with the same id.

Validation: local FEA schema pytest passed; focused local FEA/App Vitest
passed 58/58; full desktop Vitest passed 18/18 files and 391/391 tests;
desktop production build passed with the existing Vite large-chunk warning.

Boundary: target-neutral handoff metadata only. No mesh, external solver,
concrete local FEA exchange format, target solver adapter, protected standards
content, private payload, network/telemetry path, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## 2026-06-17 - B-tail support linear-stiffness unit authoring (`TP-UNITS-BTAIL-SUPPORTUNITS-001`)

Landed one bounded Phase B-tail unit I/O slice while C5.7 remains
human-execution gated. The Property Inspector support creation panel now has a
linear-stiffness unit selector and value input. When left blank, support
creation keeps the existing dimensionless restraint-only payload. When filled,
the queued `create_support` operation includes
`properties.linear_stiffness: {value, unit}` with
`dimension=linear_stiffness` and preserves the entered unit through local
session apply.

The Rust operation applier now accepts support creation in two explicit modes:
dimensionless support restraint creation, or positive DEC-018-compatible
linear stiffness creation. Invalid stiffness metadata blocks with explicit
diagnostics rather than falling back. The selected support details panel shows
the preserved stiffness value/unit after apply.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-191; DEL-07-02 primary run record
`WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-SUPPORTUNITS-001.md`; DEL-02-02
supporting run record with the same id.

Validation: `npm run -w apps/desktop build:wasm`; `cargo fmt --manifest-path
core/model_operations/operation_applier/Cargo.toml --check`; `cargo test
--manifest-path core/model_operations/operation_applier/Cargo.toml`; focused
`npm run -w apps/desktop test -- --run src/App.test.tsx` passed 55/55; full
desktop Vitest passed 18/18 files and 390/390 tests; desktop production build
passed with the existing Vite large-chunk warning.

Boundary: app-integration and operation-contract support for explicit
user-entered support stiffness only. No protected content, private data,
network/telemetry path, persistence semantics, release tolerance policy,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - C5 packaged A12 + R3 successor journey kit (`TP-R3UX-PACKAGEKIT-001`)

Prepared the C5.6 packaged journey successor kit for human execution. A fresh
macOS `.app` bundle was built from commit `c013b49b8` using
`cd apps/desktop && npm run tauri -- build --bundles app` after `npm ci` in
the fresh worktree. The build regenerated the WASM operation engine, completed
the Vite production build with the existing chunk-size warning, compiled the
Tauri release app, and produced
`apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`
at 12M.

Boot evidence: the bundled executable launched for 8 seconds, stayed alive as
`openpipestress-desktop`, spawned WebKit GPU, Networking, and WebContent
processes, wrote zero bytes to stdout/stderr, and terminated cleanly.
Post-evidence desktop Vitest passed **18/18 files, 390/390 tests**.

SMOKE TP-MAC-188 now contains the prepared C5.7 human checklist covering both
the A12 guided authoring journey and the R3 private-library/rule-pack guided
journey. The checklist explicitly preserves the R3 missing-input blocker:
pass/fail stays blocked until required user inputs are complete, and no
software code-compliance status may appear.

Evidence: DEL-07-06 run record
`WORKING_ITEMS_RUN_2026-06-17_TP-R3UX-PACKAGEKIT-001.md`; SMOKE TP-MAC-188.

Boundary: package/evidence/checklist preparation only. This does not close
F-4 or the A3 authoring-usability finding; those require the C5.7 human
packaged pass/fail record. No protected content, private project data,
network/telemetry posture change, lifecycle state change, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim.

## 2026-06-16 — B2/B3 rule-check run-time unit binding controls (`TP-UNITS-B2-RULECHECKRUNUNITS-001`)

Adds catalog-aware unit controls to the C4 run-check panel for runtime
user-supplied value bindings and value-slot limit bindings. Desktop/Tauri
runs load the reviewed DEC-018 catalog only when the loaded rule pack has
runtime value units, filter accepted unit options by declared dimension, and
keep any out-of-catalog stored unit visible as the current value.

Browser preview deliberately keeps the previous manual text-entry route for
the same test ids and reports that no fallback catalog was synthesized. The
binding payload sent to the runner remains the explicit `{value, unit,
dimension}` shape; this tranche changes the GUI unit control and witness
surface, not the schema or evaluator semantics.

Evidence: focused `RuleCheckRunPanel` Vitest passed **18/18**; full desktop
Vitest passed **389/389**; desktop build passed with the existing Vite
chunk-size warning; focused run-check Playwright passed **2/2**; full desktop
Playwright passed **10/10** with `--workers=1`; in-app Browser verified the
browser fallback status, text-input tags, `demo_unit`/`ratio` values, and
model-metadata basis text; five-surface DEC-025 dirty-tree sweep passed and
wrote
`validation/evidence/sweeps/SWEEP_20260616T034816Z_9be2d805ab17-dirty.json`.
Run records:
`DEL-07-04 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-RULECHECKRUNUNITS-001.md`
plus supporting DEL-06-02 and DEL-02-02 records; SMOKE TP-MAC-182.

Boundary: GUI run-time binding unit controls only; no schema, evaluator,
rule-pack parser, writable expression text syntax, protected content, private
project data, network/telemetry path, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

## 2026-06-16 — B2/B3 viewport draft length-unit controls (`TP-UNITS-B2-VIEWPORTDRAFTUNITS-001`)

Adds explicit length-unit controls to the viewport create-node and
straight-pipe draft forms. The forms now show the selected coordinate/pipe
geometry unit basis before queueing structured operation intents, and browser
preview exposes its model-metadata fallback instead of implying a loaded
catalog.

The operation seam now accepts compatible DEC-018 length units for explicit
node creation and straight-pipe creation. Node coordinates normalize back to
the model document's stored `project.units.length` scalar basis; pipe OD/wall
inputs are dimension-checked, wall thickness is compared after conversion,
and the entered section units remain visible in the created pipe segment.

Evidence: operation-applier focused explicit tests passed **28/28**; full
operation-applier cargo suite passed **60 unit** tests plus canonical hash and
contract corpus checks; focused App Vitest passed **54/54**; full desktop
Vitest passed **388/388**; desktop build passed with the existing Vite
chunk-size warning; focused R2 Playwright smoke passed **2/2**; full desktop
Playwright passed **10/10** with `--workers=1`; in-app Browser verified the
viewport model-metadata unit fallback and enabled node draft; five-surface
DEC-025 dirty-tree sweep passed and wrote
`validation/evidence/sweeps/SWEEP_20260616T033649Z_40c8d2530ca8-dirty.json`.
Run records:
`DEL-07-01 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-VIEWPORTDRAFTUNITS-001.md`
plus supporting DEL-16-02 and DEL-02-02 records; SMOKE TP-MAC-181.

Boundary: viewport draft and operation-seam unit evidence only; no
project-unit mutation, hidden unit fallback, protected content, private
project data, network/telemetry path, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

## 2026-06-16 — C2 read-only AST-to-text expression preview (`TP-C2-ASTTEXTVIEW-001`)

Adds the optional `DEC-037`-permitted one-way AST-to-text preview to the C2
rule-pack expression composer. The selected formula now renders a labeled
read-only text view beside the structured editor, so users can inspect the
current expression without switching to raw JSON.

The preview is deliberately display-only: it contains no editable controls, is
never parsed, and does not change the canonical authored member. The
checksum-bound rule-pack expression remains the structured DEC-022 AST.
Unsupported and refusal-marker nodes render as explicit bracketed placeholders
instead of being silently interpreted.

Evidence: focused `ExpressionComposer` Vitest passed **21/21**; focused
rule-pack Playwright passed **2/2**; full desktop Vitest passed **388/388**;
desktop build passed with the existing Vite chunk-size warning; full desktop
Playwright passed **10/10** with `--workers=1`; in-app Browser verified the
preview text, `DEC-037` boundary copy, and zero editable controls inside the
preview. DEC-025 dirty-tree sweep passed and wrote
`validation/evidence/sweeps/SWEEP_20260616T031013Z_b431a1676620-dirty.json`.
Run records:
`DEL-07-03 .../WORKING_ITEMS_RUN_2026-06-16_TP-C2-ASTTEXTVIEW-001.md`
and the supporting DEL-06-01 record; SMOKE TP-MAC-180.

Boundary: frontend display-only rule-pack editor evidence only; no schema,
evaluator, parser, writable text syntax, protected content, private project
data, network/telemetry path, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-16 — B2/B3 native JSON package unit preservation witnesses (`TP-UNITS-B2B3-NATIVEUNITWITNESS-001`)

Adds target-format unit evidence to the desktop native JSON package review
packet. The package now declares a
`maps/unit_preservation_witnesses.json` member, a
`unit_witness_policy`, and a `unit_preservation` sidecar with the accepted
DEC-018 unit-system reference. Because native JSON is a project-owned
unit-preserving format, the packet records `conversion_performed=false`.

The sidecar records 6 project unit declarations, 18 model quantity witnesses,
and 739 result quantity witnesses from the invented preview fixture. Witnesses
bind source refs to target package member paths and prove value+unit
preservation for model quantities and result-envelope rows; regression
coverage checks concrete pipe outside-diameter and axial-force witnesses.
The visible Native JSON package panel reports the witness counts and
conversion posture.

Evidence: focused App Vitest passed **54/54**; full desktop Vitest passed
**386/386**; desktop build passed with the existing Vite chunk-size warning;
full desktop Playwright passed **10/10**; in-app Browser verified the solve
completed with `result_rows=737` and the export panel showed
`project_units=6`, `model_quantities=18`, `result_quantities=739`, and
`conversion=false`; five-surface DEC-025 dirty-tree sweep passed and wrote
`validation/evidence/sweeps/SWEEP_20260616T025334Z_92ba64e2b4e3-dirty.json`.
Run records:
`DEL-17-03 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-NATIVEUNITWITNESS-001.md`
and the supporting DEL-02-02 record; SMOKE TP-MAC-179.

Boundary: native JSON browser-preview evidence only; no Python native export
schema, writer, fixture, parser, public API, downstream target compatibility
claim, protected content, private project data, network/telemetry path,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

## 2026-06-16 — B2/B3 review-geometry coordinate unit witnesses (`TP-UNITS-B2B3-REVIEWGEOMUNITWITNESS-001`)

Adds target-format unit evidence to the desktop review-geometry glTF JSON
preview. The packet now includes `unit_system_disclosure` with the accepted
DEC-018 unit-system reference, source model units, target glTF coordinate unit
`m`, `conversion_performed=false`, and the existing
`preview_z_up_to_gltf_y_up_rotation_x_minus_90` axis transform.

The export now carries a `coordinate_unit_witnesses` sidecar member. Witnesses
are generated in the same vertex order as emitted glTF positions and cover
pipe endpoints, node markers, support markers, and component markers. Each
record binds source value/unit/dimension/axis to target glTF
value/unit/dimension/axis plus vertex coordinate location and basis refs. The
visible Review Geometry Export panel reports the witness count and target unit.

Evidence: focused App Vitest passed **54/54**; full desktop Vitest passed
**386/386**; desktop build passed with the existing Vite chunk-size warning;
full desktop Playwright passed **10/10** after rerunning an initial Chrome
launch SIGKILL before app startup; in-app Browser verification was attempted
but blocked by its URL policy before localhost navigation; five-surface
DEC-025 sweep passed and wrote
`validation/evidence/sweeps/SWEEP_20260616T013908Z_673a4de9628e-dirty.json`.
Run records:
`DEL-17-08 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-REVIEWGEOMUNITWITNESS-001.md`
and the supporting DEL-02-02 record; SMOKE TP-MAC-178.

Boundary: review-geometry browser-preview evidence only; no binary GLB
package, viewer compatibility claim, solver-geometry equivalence,
target-compatibility claim, protected content, private project data,
network/telemetry path, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## 2026-06-16 — A3 authoring journey status rail (`TP-APP-R3-A3-JOURNEYPATH-001`)

Adds a compact authoring-journey status rail to the desktop shell to reduce
the R3-carried authoring usability residual from TP-MAC-141. The rail is
derived from existing session/model state and reports Model, Apply, Loads,
Solve, Rules, Report, and Project status. Each item is a live navigation
control into the existing workspace section, so the user can move through the
same authoring path without hunting through the lower dock.

The desktop layout flex ratio was adjusted so the lower dock receives more
usable height at the compact 1280x800 lane while the persistent model tree,
3D viewport, and property inspector remain visible. The rail is bounded and
wraps compactly instead of expanding the page.

Evidence: focused App + dead-control Vitest passed **55/55**; desktop
production build passed with the existing Vite chunk-size warning; focused
compact Playwright smoke passed **1/1**; in-app Browser at 1280x800 verified
Loads rail navigation with `bodyOverflow=0`, `viewportOverflow=0`,
`itemOverflow=0`, `dockBodyHeight=168`, and `journeyHeight=66`; full desktop
Vitest passed **386/386**; full desktop Playwright passed **10/10** across
1440x920 and 1280x800. Run record:
`DEL-07-06 .../WORKING_ITEMS_RUN_2026-06-16_TP-APP-R3-A3-JOURNEYPATH-001.md`;
SMOKE TP-MAC-177.

Boundary: desktop-shell usability/evidence only; no schema, solver,
rule-check math, persistence semantics, protected content, private project
data, network/telemetry path, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
F-4 remains open until the human packaged GUI journey is completed.

## 2026-06-16 — B2/B3 rule-check mixed-unit normalization (`TP-UNITS-B2B3-RULECHECKNORM-001`)

Adds DEC-018 compatible-unit normalization at the rule-check runner boundary.
Before this slice, the C4 rule-check path carried explicit unit strings into
the exact-unit expression evaluator, so a pack whose declaration said `Pa`
could not accept compatible runtime values entered as `MPa` or `kPa`.

`core/rules/rule_check_runner` now depends on the accepted `core/units` crate.
For each required input and value-slot limit, the runner converts compatible
catalog units to the rule-pack declaration's unit before formula evaluation and
acceptability comparison. Exact non-catalog demonstration units still pass only
when the entered and declared strings match. Incompatible or unknown
substitutions block at `RULE_INPUTS_INCOMPLETE` with a `UnitMismatch` finding;
they are never coerced or evaluated.

The desktop Tauri `run_rule_checks_core` regression re-stamps an invented
example pack whose stress declarations use `Pa`, then proves `0.05 MPa` actual
and `100 kPa` limit normalize to `50_000 Pa` and `100_000 Pa` and produce the
same ratio result (`0.5`) through the app command wrapper.

Validation: `cargo test --manifest-path core/rules/rule_check_runner/Cargo.toml`
passed (13 unit tests, 4 acceptability-relation integration tests, 3 invented
demo integration tests); `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
passed (62 tests); desktop Vitest passed (18 files, 386 tests); desktop
production build passed with the existing Vite chunk-size warning.

Evidence: run records
`WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-RULECHECKNORM-001.md` (DEL-06-02
primary and DEL-02-02 supporting); SMOKE TP-MAC-176; completion plan rows B2
and B3 updated.

Boundary review: no grammar change, schema change, text parser, protected
standards content, private project data, network/telemetry path, release-
readiness claim, professional approval, certification, sealing, authentication,
or code-compliance claim changed.

## 2026-06-16 — B2/B3 stress-neutral unit preservation witnesses (`TP-UNITS-B2B3-STRESSNEUTRALUNITWITNESS-001`)

Adds auditable per-row unit-preservation witnesses to the desktop
stress-neutral CSV/JSON package. Stress-neutral export is intentionally not a
target-format conversion path; it preserves source result values, units, and
dimensions so spreadsheet review and downstream adapter prototypes can inspect
the same unit-bearing rows.

`StressNeutralExportPanel` now emits `unit_preservation_witnesses` in the
downloaded JSON package, lists `unit_preservation_witnesses.json` in the
manifest, and exposes a visible witness-count line in the panel. Each witness
names the source result row and field, source value/unit/dimension, target
stress-neutral row and field, target value/unit/dimension, DEC-018/DEL-02-02
and DEL-17-06 basis refs, and preview provenance. The validation report now
checks that every result row has a witness and that source/target quantities
match with `conversion_performed=false`.

Evidence: focused `App.test.tsx` Vitest **54/54**; full desktop Vitest
**386/386**; desktop production build clean with the existing Vite chunk-size
warning; targeted Playwright R2 smoke spec **10/10** after wasm rebuild. Run
records:
`DEL-17-06 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-STRESSNEUTRALUNITWITNESS-001.md`;
`DEL-02-02 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-STRESSNEUTRALUNITWITNESS-001.md`;
SMOKE TP-MAC-175.

Boundary: desktop stress-neutral package surface only; no Python/schema export
package contract change, no vendor format claim, no target compatibility
claim, no comparison pass/fail claim, no solver validation, no protected
standards content, no private data, and no release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claim.

Residual: B2 still owns broader app unit entry/pickers outside the named
covered forms and remaining target-format conversion witnesses outside the PCF
and CAEPIPE smoke-package boundaries. B3 still owns broader mixed-unit
round-trip, conversion-witness, incompatible-unit rejection, and D-04/DEC-026
tolerance corpus outside the named witnesses.

---

## 2026-06-16 — B2/B3 CAEPIPE MBF conversion witnesses (`TP-UNITS-B2B3-CAEPIPECONVWITNESS-001`)

Adds auditable target-format conversion witnesses to the desktop CAEPIPE MBF
smoke-subset export package. The earlier export-disclosure tranche stated the
package's source and target units, but the downloaded desktop package still
lacked per-field source-to-target evidence for a non-PCF target-format
conversion path.

`CaepipeMbfExportPanel` now emits `conversion_witnesses` in the downloaded
JSON package, lists `conversion_witnesses.json` in the manifest, and exposes a
visible witness-count line in the panel. Each witness names the source node
and coordinate field, source value/unit/dimension, target MBF payload
field/value/unit (`mm`), the conversion factor, DEC-018/DEL-02-02 basis refs,
and preview provenance. The MBF text now emits millimeter node coordinates
with an explicit `UNIT,mm,...` record while preserving the smoke-subset
profile, loss report, sidecar stable-ID map, and no-compatibility boundary.

Evidence: focused `App.test.tsx` Vitest **54/54**; full desktop Vitest
**386/386**; desktop production build clean with the existing Vite chunk-size
warning; targeted Playwright R2 smoke spec **10/10** after wasm rebuild. Run
records:
`DEL-17-04 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-CAEPIPECONVWITNESS-001.md`;
`DEL-02-02 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-CAEPIPECONVWITNESS-001.md`;
SMOKE TP-MAC-174.

Boundary: desktop CAEPIPE MBF smoke package surface only; no Python/schema
export package contract change, no CAEPIPE target version/profile closure, no
direct MBF stable-ID carrier claim, no downstream import compatibility claim,
no external CAEPIPE execution, no solver validation, no hidden target default,
no protected standards content, no private data, and no release-readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim.

Residual: B2 still owns broader app unit entry/pickers outside the named
covered forms and remaining target-format conversion witnesses outside the PCF
and CAEPIPE smoke-package boundaries. B3 still owns broader mixed-unit
round-trip, conversion-witness, incompatible-unit rejection, and D-04/DEC-026
tolerance corpus outside the named witnesses.

---

## 2026-06-16 — B2 local project unit round-trip evidence (`TP-UNITS-B2-IMPORTRT-001`)

Adds explicit unit metadata round-trip evidence to local project
create/save/open summaries. Before this slice, the project validation manifest
could report that the active model contained unit metadata, and the persistence
service preserved the model payload, but no persisted/opened local project
summary exposed a deterministic unit signature that tests and humans could
compare after a project round trip.

`projectService.ts` and the Tauri SQLite backend now compute
`unit_round_trip_status`, `unit_round_trip_checked_ref_count`, and
`unit_round_trip_signature` from explicit unit refs in `project.units`,
materials, sections, pipe segment sections, and primitive-load magnitudes.
The Project Storage Audit and Project Validation Preflight panels show a
dedicated unit round-trip evidence row and export the same fields in local JSON
packets. Missing unit-bearing values produce
`unit_metadata_missing_review_required`; preserved local-project unit metadata
produces `unit_metadata_preserved_in_local_project_envelope`.

Evidence: focused `projectService.test.ts` + `App.test.tsx` Vitest **61/61**;
full desktop Vitest **386/386**; desktop production build clean with the
existing Vite chunk-size warning; `cargo fmt --manifest-path
apps/desktop/src-tauri/Cargo.toml --check -p openpipestress-desktop`;
Tauri focused store regression
`local_project_store_uses_sqlite_fts5_and_round_trips_model_snapshot` passed;
in-app Browser fallback verified `New blank` -> `Save local` -> `Open local`
shows `unit_metadata_preserved_in_local_project_envelope` and six
blank-project unit refs in both visible rows. Initial direct Playwright
execution found a missing local Chromium cache and browser-install CDN
timeouts, but the later DEC-025 sweep ran the updated Playwright smoke
successfully: **10/10** dev-server lane plus **1/1** production-dist lane. Run
records:
`DEL-02-05 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-IMPORTRT-001.md`;
`DEL-02-02 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-IMPORTRT-001.md`;
SMOKE TP-MAC-173.

Boundary: local persistence and validation evidence only; no unit conversion,
no import target compatibility claim, no solver behavior change, no hidden
defaults, no protected standards content, no private data committed to the
repository, and no release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim.

Residual: B2 still owns broader app unit entry/pickers outside the named
covered forms and non-PCF target-format conversion witnesses. B3 still owns
broader mixed-unit round-trip, conversion-witness, incompatible-unit rejection,
and D-04/DEC-026 tolerance corpus outside the named witnesses.

---

## 2026-06-16 — B2/B3 PCF conversion witnesses (`TP-UNITS-B2B3-PCFCONVWITNESS-001`)

Adds auditable target-format conversion witnesses to the desktop conservative
PCF export package. The earlier export-disclosure tranche stated that the PCF
text uses millimeter coordinate and pipe-geometry fields, but the downloaded
desktop package did not include a per-field source-to-target record proving the
conversion.

`PcfExportPanel` now emits `conversion_witnesses` in the downloaded JSON
package and lists `conversion_witnesses.json` in the manifest. Each witness
names the source object and field path, source value/unit/dimension, target
PCF field/value/unit (`MM`), the conversion factor, DEC-018/DEL-02-02 basis
refs, and preview provenance. The validation report now checks that each
converted length field has a witness and that all witness target units are
millimeters. The existing PCF text remains conservative and blocked for target
compatibility because nominal size/profile and downstream behavior remain TBD.

Evidence: focused `App.test.tsx` Vitest **54/54**; full desktop Vitest
**386/386**; desktop production build clean with the existing Vite chunk-size
warning; focused Playwright R2 smoke **2/2**. Run records:
`DEL-17-07 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-PCFCONVWITNESS-001.md`;
`DEL-02-02 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2B3-PCFCONVWITNESS-001.md`;
SMOKE TP-MAC-172.

Boundary: desktop PCF package surface only; no Python/schema export package
contract change, no downstream import compatibility claim, no solver-deck
validation, no hidden target default, no protected standards content, no
private data, and no release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim.

Residual: B2 still owns import round-trip unit I/O and broader app unit
entry/pickers outside the named covered forms. B3 still owns broader
mixed-unit round-trip, conversion-witness, incompatible-unit rejection, and
D-04/DEC-026 tolerance corpus outside the product-physics, operation,
rule-pack, node-coordinate, and PCF export witness boundaries.

---

## 2026-06-16 — B2 load-case inspector primitive unit payloads (`TP-UNITS-B2-LOADINSPECTORUNITS-001`)

Extends the B2 unit-aware editor pattern to the Property Inspector's load-case
primitive magnitude view. Before this slice, the Load Cases manager could edit
existing primitive-load magnitudes with explicit `{ value, unit }` payloads, but
the model-tree Property Inspector still rendered the selected load case's first
primitive magnitude as a scalar-only field. That left one visible app path for
primitive-load magnitude editing outside the explicit unit payload pattern.

`PropertyInspector` now marks `primitive_loads.0.magnitude.value` as a
unit-editable quantity when a load case is selected. The emitted operation
intent remains `update_load` and now carries the selected primitive's dimension
and unit in both intent metadata and the JSON after-value payload. This is a
frontend-only binding to the existing operation seam; no backend operation
contract, schema, persistence, solver, report, import/export, or rule-pack
behavior changed.

Evidence: focused `App.test.tsx` Vitest **54/54**; full desktop Vitest
**386/386**; desktop production build clean with the existing Vite chunk-size
warning; focused Playwright R2 smoke **2/2** across configured desktop/compact
Chromium projects after adding a browser preview assertion for the
load-inspector primitive unit payload. Run record:
`DEL-07-02 .../WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-LOADINSPECTORUNITS-001.md`;
SMOKE TP-MAC-171.

Boundary: frontend-only; no hidden unit fallback, no protected standards
content, no private data, no network or telemetry path, and no
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim changed.

Residual: B2 still has broader unit entry/pickers outside the material,
section, node-coordinate, primitive-load manager/inspector, and rule-pack
declaration/expression surfaces; import round-trip unit I/O; and target-format
conversion witnesses beyond disclosure. B3 still owns the larger mixed-unit
round-trip, incompatible-unit rejection, and D-04/DEC-026 tolerance corpus
outside the named witnesses.

---

## 2026-06-15 — B2 node coordinate unit payloads (`TP-UNITS-B2-NODECOORDUNITS-001`)

Extends the B2 unit-aware editor pattern to node coordinate edits. Before this
slice, `PropertyInspector` displayed node `position.x/y/z` values with the
model length unit in metadata, but the edit intent path still emitted a bare
scalar and offered no explicit unit field. That left node coordinates outside
the already covered material/section/primitive-load and rule-pack unit-entry
surfaces.

`PropertyInspector` now marks node coordinates as unit-editable length fields
and emits explicit `{ value, unit }` payloads. The `operation_applier` now
accepts project-unit quantity edits in either legacy scalar form or explicit
payload form. For bare project-unit fields, accepted DEC-018 length units are
converted to the model document's stored `project.units.length` basis before
writing the scalar coordinate; incompatible units remain blocked with
`OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`. The shared operation contract corpus
unit-mismatch case was updated from `ksi` (now an accepted DEC-018 stress unit)
to a true incompatible length/stress mismatch.

Evidence: operation-applier cargo suite **58 unit + canonical hash + 66-case
contract corpus**; full desktop Vitest **385/385**; desktop production build
clean with the existing Vite chunk-size warning; focused Playwright R2 smoke
**2/2** across configured desktop/compact Chromium projects after adding a
browser preview assertion for the node-coordinate unit payload. Run record:
`DEL-16-02 .../WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-NODECOORDUNITS-001.md`;
SMOKE TP-MAC-170.

Boundary: no schema, persistence, solver, rule-pack, private-data,
protected-content, network, telemetry, release-readiness, professional
approval, certification, sealing, authentication, or code-compliance claim
changed. Browser preview still does not synthesize a DEC-018 fallback catalog.

Residual: B2 still has broader unit entry/pickers outside the material,
section, node-coordinate, primitive-load, and rule-pack declaration/expression
forms; import round-trip unit I/O; and target-format conversion witnesses
beyond disclosure. B3 still owns the larger mixed-unit round-trip,
incompatible-unit rejection, and D-04/DEC-026 tolerance corpus outside the
named operation-seam witnesses.

---

## 2026-06-16 — C5 R3 rule-pack/private-library guided flow (`TP-R3UX-R3FLOW-001`)

Adds the R3-specific guided mode inside the guided workbench. The mode routes a
human through private local library import, private non-code rule-pack draft and
validation, checksum/save request, mechanics solve, rule-check binding review,
and rule-check run request.

The guide is UI-only session state layered over existing panels. It records
visible user actions from `LibraryManagerPanel`, `RulePackManagerPanel`, and
`RuleCheckRunPanel`; it does not change the rule-pack schema, evaluator
grammar, solver, persistence, backend commands, checksum semantics, or desktop
store behavior. The R3 path exposes stable IDs including `r3-guided-flow`,
`r3-flow-step-*`, `r3-flow-missing-input-blocker`, and the existing
`r3-exit-journey-status`.

The final browser-preview path reaches all guided steps while still reporting
that pass/fail remains blocked until the desktop checker returns complete
inputs. This preserves the PRD 22.4 rule: missing required inputs block
pass/fail, never silently default or synthesize a code-compliance result.

Evidence: full desktop Vitest **18/18 files, 386/386 tests**, permanent
dead-control audit, desktop production build, full Playwright e2e **14/14**
across 1440x920 and 1280x800, and in-app browser screenshots/probe JSON under
DEL-07-06 `_run_records/assets/`; SMOKE TP-MAC-187; run record
`WORKING_ITEMS_RUN_2026-06-16_TP-R3UX-R3FLOW-001.md`.

Boundary: frontend guided-workbench usability only; no private data committed,
no protected content, no network/telemetry, no lifecycle state change, and no
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim. C5.6 remains the next ordinary C5
tranche: packaged journey successor kit (`TP-R3UX-PACKAGEKIT-001`).

---

## 2026-06-16 — C5 A12 authoring journey redesign (`TP-R3UX-A12FLOW-001`)

Adds the A12-specific guided authoring path inside the guided workbench. The
panel derives its checklist from the current model/session state rather than
hard-coded A12 payload values: blank document, nodes, material, section, pipe,
support, load case, primitive load, combination, solve, report, and
save/reopen.

The A12 panel now exposes a clear next action, current queue status naming the
created object when available, selected-step state for controls that share the
same work surface, and an inline `Apply queued` affordance. That affordance
calls the existing structured operation apply handler, records the existing
operation receipt, preserves undo/result-reset behavior, and leaves Operation
Apply as the receipt/audit surface.

Evidence: focused guided-workbench Vitest, permanent dead-control audit, full
desktop Vitest **18/18 files, 386/386 tests**, desktop production build, full
Playwright e2e **12/12** across 1440x920 and 1280x800, and in-app browser
screenshots/probe JSON under DEL-07-06 `_run_records/assets/`; SMOKE
TP-MAC-186; run record
`WORKING_ITEMS_RUN_2026-06-16_TP-R3UX-A12FLOW-001.md`.

Boundary: frontend guided-authoring usability only; no rule-pack schema,
evaluator grammar, solver, persistence, backend API, project-store semantics,
rule-pack checksum, lifecycle, release-readiness, professional approval,
certification, sealing, authentication, or code-compliance change. C5.5 remains
the next ordinary C5 tranche for the R3 rule-pack/private-library guided flow.

---

## 2026-06-16 — C5 guided workbench shell (`TP-R3UX-SHELL-001`)

Adds the first implementation slice of the C5 guided-workbench redesign. The
desktop shell keeps the persistent spatial core (model tree, 3D centerline
viewport, property inspector) and inserts a default guided lower workspace:
task rail, current-step panel, compact queue/status facts, and compact R3 exit
journey status.

The evidence-heavy surfaces remain available but no longer dominate the
default lower workspace. `EditorContractPanel`, `DiffPreviewPanel`,
`OperationLedgerPanel`, and `AgentProposalPanel` are mounted behind a
review/apply detail drawer in the existing operation-apply section. Existing
panel test IDs remain present after opening details, and new stable IDs cover
the C5 shell (`guided-workbench`, `journey-step-*`,
`review-apply-drawer`, `review-apply-drawer-toggle`,
`r3-exit-journey-status`).

Evidence: focused guided-workbench Vitest, dead-control audit, focused
Playwright e2e at 1440x920 and 1280x800, in-app browser screenshots and probe
JSON under DEL-07-06 `_run_records/assets/`, SMOKE TP-MAC-185, and DEL-07-06
run record `WORKING_ITEMS_RUN_2026-06-16_TP-R3UX-SHELL-001.md`.

Boundary: frontend shell/usability only; no schema, evaluator grammar, solver,
persistence, backend API, rule-pack checksum, lifecycle, release-readiness,
professional approval, certification, sealing, authentication, or
code-compliance change. C5.4 remains the next ordinary C5 tranche for the A12
authoring journey.

---

## 2026-06-16 — C5 R3 exit readiness plan revision (`TP-R3EXIT-PLAN-001`)

Adds Phase C5 to the active completion plan after landed C4 and before Phase D.
C5 is now the ordinary R3 in-stage program for guided-workbench redesign,
packaged A12/R3 human-journey evidence, and R3 exit evidence assembly. It
exists to close the two `DEC-035` blockers before any R3 exit review: F-4
(completed human packaged-GUI journey) and the authoring-journey usability
finding.

Updated routing surfaces: `plans/PLAN_2026-06-10_prd_completion.md`,
`docs/PLAN.md`, `execution/_Coordination/_COORDINATION.md`, and
`execution/_Coordination/NEXT_INSTANCE_PROMPT.md`. C1-C4 remain landed
implementation evidence; Phase D remains behind C5 unless the human explicitly
overrides.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-183 and DEL-07-06 run record
`WORKING_ITEMS_RUN_2026-06-16_TP-R3EXIT-PLAN-001_TP-R3UX-DISCOVERY-001.md`.

Boundary: plan/evidence routing only; no app behavior, schema, solver,
rule-pack grammar, parser, persistence, package, lifecycle, release-readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim.

---

## 2026-06-16 — C5 R3 authoring-usability baseline (`TP-R3UX-DISCOVERY-001`)

Records the C5 starting baseline for the guided-workbench redesign. Evidence
sources: TP-MAC-141 human attempts 1-3, the human-provided 2026-06-16
screenshot and user-perspective evaluation, current 1440x920 and 1280x800
screenshots, viewport overflow/console probe, and the dead-control audit.

Findings: the post-TP-MAC-146 shell is structurally stable, with no horizontal
page overflow, no horizontally clipped visible primary controls, and zero
console errors at both viewport sizes. The existing dead-control audit passed
1 file / 1 test. The remaining user-facing blocker is not mechanical
correctness but work-surface clarity: the default view still exposes internal
state and evidence panels before the user knows the next task.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-184; screenshots and probe JSON under
DEL-07-06 `_run_records/assets/`; run record
`WORKING_ITEMS_RUN_2026-06-16_TP-R3EXIT-PLAN-001_TP-R3UX-DISCOVERY-001.md`.

Boundary: discovery/evidence only; no product behavior change, no lifecycle
state change, and no release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim. Next implementation tranche:
`TP-R3UX-SHELL-001` / target SMOKE TP-MAC-185.

---

---

## 2026-06-15 — B2 rule-pack expression unit selectors (`TP-UNITS-B2-RULEEXPRUNITS-001`)

Extends the DEC-018 unit-catalog UI pattern into the C2 rule-pack expression
composer. Before this slice, expression literal `quantity.unit_ref` and table
`argument_unit_ref` / `result_unit_ref` fields were still plain text fields in
the desktop runtime, even after the prior declaration-unit selector tranche had
closed `quantity_intent.unit_ref` for required inputs and value slots.

`ExpressionComposer` now loads the reviewed desktop unit catalog only when the
Tauri runtime is present, then renders catalog-backed selectors for expression
literal and table unit fields. The selectors filter accepted catalog entries by
the currently authored dimension, use the existing equivalent-dimension rules
(`stress` through pressure units), preserve any stored out-of-catalog unit as
the current value instead of snapping it, and never mutates a dimension/unit pair
silently. Browser preview keeps the previous free-text field and does not
synthesize a fallback catalog.

Evidence: focused `ExpressionComposer` Vitest **19/19**; full desktop Vitest
**384/384**; desktop production build clean with the existing Vite chunk-size
warning; Playwright e2e **10/10** across configured desktop/compact Chromium
projects after extending the rule-pack manager journey with browser-mode manual
expression literal/table unit entry assertions. Run records:
`DEL-02-02 .../WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-RULEEXPRUNITS-001.md`
and
`DEL-06-02 .../WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-RULEEXPRUNITS-001.md`;
SMOKE TP-MAC-169.

Boundary: frontend-only; no schema, backend command, evaluator, expression
grammar, parser/text-syntax, rule-check status, checksum, or persistence change.
No protected standards content, private value, fallback browser catalog,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

Residual: B2 still has broader app unit entry/pickers outside the already
covered material/section/primitive-load and rule-pack declaration/expression
forms, import round-trip unit I/O, and target-format conversion witnesses beyond
disclosure. B3 still owns the larger conversion-witness, incompatible-unit, and
tolerance corpus beyond the named rule-pack picker witnesses.

---

## 2026-06-15 — B2 rule-pack declaration unit selectors (`TP-UNITS-B2-RULEPACKUNITS-001`)

Extends the DEC-018 unit-catalog UI pattern into the C2 rule-pack declarations
form-builder. Before this slice, `required_inputs[].quantity_intent.unit_ref`
and `value_slots[].quantity_intent.unit_ref` were plain text fields even in the
desktop runtime, so Phase B2 still named "rule-pack unit I/O" as open after the
material/section/load/editor unit work.

`DeclarationsEditor` now loads the reviewed desktop unit catalog only when the
Tauri runtime is present, then renders catalog-backed selectors for
declaration `unit_ref` fields. The selector filters accepted catalog entries by
the selected declaration dimension, uses the existing equivalent-dimension
rules (`stress` through pressure units), preserves any stored out-of-catalog
unit as the current value instead of snapping it, and never mutates the
dimension/unit pair silently. Browser preview keeps the previous free-text
field and does not synthesize a fallback catalog.

Evidence: focused `DeclarationsEditor` Vitest **29/29**; full desktop Vitest
**381/381**; desktop production build clean with the existing Vite chunk-size
warning; Playwright e2e **10/10** across configured desktop/compact Chromium
projects after adding a browser-mode manual-unit fallback assertion to the
rule-pack manager journey. Run record:
`DEL-02-02 .../WORKING_ITEMS_RUN_2026-06-15_TP-UNITS-B2-RULEPACKUNITS-001.md`;
SMOKE TP-MAC-168.

Boundary: frontend-only; no schema, backend command, evaluator, expression
grammar, or persistence change. No protected standards content, private value,
fallback browser catalog, release-readiness, professional approval,
certification, sealing, authentication, or code-compliance claim.

Residual: B2 still has rule-pack expression literal/table unit I/O plus broader
unit-entry/import/export conversion work outside the already-covered forms; B3
still owns the larger conversion-witness, incompatible-unit, and tolerance
corpus beyond the declaration-picker witnesses.

---

## 2026-06-15 — C4 run-panel preview for authored solver-result references (`TP-C4-SOLVERREFPICKER-001`)

Closes the remaining C4 follow-up named after `TP-C4-SOLVERREFAUTHOR-001`: a
run-panel resolution preview for `solver_result_ref`, mirroring the earlier
`TP-C3-LIBREFPICKER-001` private-library preview. The optional
`solver_result_ref` member and backend resolver were already landed and ratified
(`TP-C4-SOLVERREF-001`, `DEC-039`); this slice makes the GUI run panel surface
whether the authored reference is usable against the current solved envelope.

`deriveRuleCheckBindingPlan` now preserves optional
`solver_result_ref: {result_id}` on `solver_result` inputs and exposes a pure
`classifySolverResultReference` helper. `RuleCheckRunPanel` now treats an
authored `solver_result_ref` as canonical: it hides the legacy run-panel
selector for that input, omits caller-supplied selector bindings for it, and
renders a read-only "Preview result row" action. The preview classifies the
authored `result_id` as `resolves`, `result_missing`, or `no_result_rows`, lists
available result rows when a solve exists, and states that unresolved references
block at `RULE_INPUTS_INCOMPLETE` rather than receiving a fallback selector.
Packs without `solver_result_ref` keep the prior manual selector behavior.

Evidence: focused Vitest (`RuleCheckRunPanel`, `ruleCheckService`) **27/27**;
full desktop Vitest **378/378**; desktop production build clean; focused
Playwright run-rule-checks smoke **2/2** across the configured desktop/compact
viewports. The first Playwright attempt expected `resolves` in browser preview,
but the browser smoke state had no solved rows, so the product correctly emitted
`no_result_rows`; the spec was corrected to pin that honest state while
component tests cover the resolving solved-envelope path. Run record:
`DEL-06-02 .../WORKING_ITEMS_RUN_2026-06-15_TP-C4-SOLVERREFPICKER-001.md`;
SMOKE TP-MAC-167.

Boundary: frontend-only; no schema/backend change; no private value embedded in
the pack; local-only; status-vocabulary-only; no release, professional,
certification, sealing, authentication, approval, or code-compliance claim.

---

## 2026-06-14 — C4 solver-result-reference GUI authoring in the declarations form-builder (`TP-C4-SOLVERREFAUTHOR-001`)

Lands the **authoring half** of the C4 solver-result binding — the first of
C4's two named non-GUI follow-ups ("GUI authoring of `solver_result_ref` in the
C2 declarations form-builder, mirrors `TP-C3-LIBREFAUTHOR-001`"). The
backend-resolution half landed in `TP-C4-SOLVERREF-001` (the member was ratified
at `schema_version` 0.4.0 by `DEC-039`); before this slice, a `solver_result`
required input's authored `solver_result_ref` could only be set by hand-editing
raw document JSON. The rule-pack declarations form-builder
(`DeclarationsEditor.tsx`) now authors it through a structured control.

GUI: when a required input's `source_kind` is set to `solver_result`, a
reference sub-form appears with a single `result_id` text field, a note, and a
"Remove solver-result reference" control. Switching to `solver_result` seeds a
complete reference (`result_id` → visible uppercase `"TBD"` placeholder), so an
unfilled reference matches no result row and the input blocks at
`RULE_INPUTS_INCOMPLETE` — never a partial shape, never a silent pass. Mirrors
the `library_value_ref` pattern, with one semantic adaptation: because
`solver_result` (unlike `private_library_value`) has a real run-panel
caller-supplied fallback, the note states that an authored reference is the
**canonical** binding and **supersedes** the run-panel per-input selector (per
the backend `resolve_authored_solver_result_bindings` / PRD §12.5), and the
"Remove" control is the visible escape hatch back to the run-panel binding path.
A reference left after the source_kind is changed away stays visible and
removable; seeding a solver reference does not also seed a library reference.
Still form-only (no writable expression text — D-02b `DEC-037` Option O-C,
AST-only).

Boundary: frontend-only; no schema/backend change (the optional member, its
run-time resolution, and its ratification all preceded this slice). The
reference is carried in the pack; no solver result value is embedded.

Evidence: desktop Vitest **372/372** (+5: default-ref shape; seed-on-switch with
no library-seed; first-edit completion; lossless `result_id` edit; stale-ref
visible+removable); `npm run build` clean; Playwright rule-pack journey **2/2**
targeted + **10/10** under the sweep (two viewports; extended in
`e2e/r2-smoke.spec.ts`); cargo crate sweep ok + pytest 359. **Committed (TP-C4-SOLVERREFAUTHOR-001
commit, local `main`); push pending** — the commit-bound DEC-025 five-surface gate did not
produce a single all-green run this session due to sustained external machine
load causing rotating infrastructure timeouts (App.test.tsx per-test caps, then
a Playwright worker-teardown force-kill after all 10 tests passed); every
surface passed individually. Run record:
`DEL-06-02 .../WORKING_ITEMS_RUN_2026-06-14_TP-C4-SOLVERREFAUTHOR-001.md`
(Closeout status / Gate section); SMOKE TP-MAC-166. Residual: the run-panel
resolution preview for `solver_result_ref` (mirrors `TP-C3-LIBREFPICKER-001`) —
C4's last named non-GUI follow-up.

---

## 2026-06-14 — Ratify the two C4 additive schema members + bump rule-pack `schema_version` 0.3.0 → 0.4.0 (`TP-C4-RATIFY-001`, `DEC-039`)

Human project authority ruling ("I approve adding the new schema members")
ratifies both PROPOSAL additive members — `acceptability_relation`
(`TP-C4-ACCEPTREL-001`, on `CheckDefinition`) and `solver_result_ref`
(`TP-C4-SOLVERREF-001`, on `RequiredInput`) — as permanent parts of
`schemas/rule_pack.schema.yaml`. Recorded as `DEC-039` in `SOFTWARE_DECOMP.md`
§12. The members were already in the schema; this is the bounded ratification
follow-up (lifecycle PROPOSAL → permanent) plus the `DEC-033`/`DEC-038`
additive-minor version bump.

Both ratified in one event → a single minor step: rule-pack `schema_version`
**0.3.0 → 0.4.0** (each member is an `additionalProperties:false` addition a
strict pre-member validator would reject — the minor-bump signal — but they
ship together, so one minor release, not 0.4.0-then-0.5.0). `grammar_version`
stays 1.0.0 (frozen, `DEC-022`); `metadata.rule_pack_version` unchanged (no
content change; the demo uses neither optional member).

Version bump + checksum re-stamp only: the canonical demo pack
(`examples/rule_packs/invented_demo.yaml`), its preview fixture twin
(`fixtures/product_preview/invented_demo_rule_pack.json`), and the new-draft
editor template (`rulePackService.ts`) declare 0.4.0; the demo/fixture
`rule_pack_checksum` re-stamped to `c2b4ddce…`; the draft-version Vitest
assertion updated. Transitively, `examples/models/invented/fake_rule_pack_toy_model.json`
hash-references the demo-pack file bytes, so its rule-pack-ref raw-byte digest
and recomputed JCS project hash were re-synced (same step as `DEC-038`; caught by
the full pytest surface, `test_invented_example_models.py`). (`lib.rs:4780`
`schema_version: "0.3.0"` is a project/model document version in a DEC-019 test —
not a rule pack — and is untouched.)

Evidence: pytest `test_rule_pack_schema.py` 5 (JCS checksum-parity recompute
matches); `rule_pack_document` cargo 10 + `rule_check_runner` cargo 18 (demo
golden checksum/validation + invented-demo runs); targeted desktop Vitest 95;
green five-surface DEC-025 sweep. Run record:
`DEL-06-02 .../WORKING_ITEMS_RUN_2026-06-14_TP-C4-RATIFY-001.md`; SMOKE
TP-MAC-165. Closes the "PROPOSAL awaiting ratification" residuals on both C4
additive members.

---

## 2026-06-14 — C4 solver-result-selector: additive `solver_result_ref` member + authored-ref resolution (`TP-C4-SOLVERREF-001`)

Lands the `solver-result-selector` half of the C4 "Remaining scope (non-GUI):
the future additive `acceptability_relation` / solver-result-selector schema
members" — the backend-resolution slice. Before this, a `solver_result`
required input's binding to a solved result row was **caller-supplied only**:
the runner module doc said it plainly ("The schema carries no selector tying a
`solver_result` required input to a solved result row, so the binding is
caller-supplied"), and the desktop command resolved a run-time
`[{input_id, result_id}]` selector against the solved envelope's `results[]`.
The rule pack itself could not declare which result row each solver input reads
— exactly the authored, in-pack reference `library_value_ref` already
established for private-library values.

Additive, optional, backward-compatible PROPOSAL member (the established
`library_value_ref`→`DEC-038` pattern):

- **Schema** (`schemas/rule_pack.schema.yaml`): optional `solver_result_ref`
  (`$defs.SolverResultRef`, `additionalProperties:false`, required
  `{result_id: Id}`) on `RequiredInput`, mirroring `LibraryValueRef`. `result_id`
  reuses the existing stable, deterministic envelope row id (e.g.
  `result:stress:demo`, `result:disp:node-N-130:ux`; the `Id` pattern admits the
  colon-delimited form) — no new addressing model. Not in `required` → packs
  without it stay valid; the pytest schema-conformance suite passes (the demo
  fixture omits it).
- **Desktop command** (`apps/desktop/src-tauri/src/lib.rs`): a shared
  `solver_result_row_value(envelope, result_id)` lookup (the caller-supplied
  `resolve_solver_result_bindings` refactored onto it, behaviour unchanged) and a
  new `resolve_authored_solver_result_bindings(document, envelope)` that resolves
  each `solver_result` input carrying a `solver_result_ref` and returns the
  bindings plus the set of input ids that carry a reference.
  `run_rule_checks_core` resolves authored references first, then extends with
  caller-supplied selectors **only for inputs without an authored reference**.
- **Semantics**: the authored reference is canonical — the caller-supplied
  selector for an authored input is dropped (no run-time override; matches the
  `library_value_ref` ruling). An unresolvable reference (missing row, or no
  numeric value / unit) blocks the input at `RULE_INPUTS_INCOMPLETE` — never a
  silent pass, never a caller rescue (no-silent-defaults, CONTRACT). A pack with
  no reference behaves exactly as before.
- **Runner unchanged / stays pure**: the `rule_check_runner` crate still consumes
  pre-resolved `SolverResultBinding`s; resolution lives in the impure desktop
  command, exactly as the library path resolves from the local store.

Evidence: src-tauri `cargo test` **61** (57 baseline + 4 new: two helper-level,
two end-to-end — authored ref alone passes with no caller selector; authored ref
to a missing row blocks over a would-resolve caller selector); `cargo fmt
--check` clean; pytest `test_rule_pack_schema.py` **5**; runner `cargo test`
**18** unchanged (runner untouched). No TypeScript changed → Vitest/build
unaffected; backend-only slice with no user-visible behaviour change, so no
Playwright/Vitest extension is owed under the H4 posture. Run record:
`DEL-06-02 .../WORKING_ITEMS_RUN_2026-06-14_TP-C4-SOLVERREF-001.md`; SMOKE
TP-MAC-164.

Residual: **schema ratification** (PROPOSAL awaiting a human `DEC`; companion to
`DEC-038`; bumps rule-pack `schema_version` per `DEC-033`, after the
`acceptability_relation` ratification). Non-GUI follow-ups: GUI authoring of
`solver_result_ref` (mirrors `TP-C3-LIBREFAUTHOR-001`) and a run-panel
resolution preview (mirrors `TP-C3-LIBREFPICKER-001`). With this, C4's named
non-GUI schema members are landed.

---

## 2026-06-14 — C4 acceptability relation beyond `<=`: additive `acceptability_relation` member (`TP-C4-ACCEPTREL-001`)

Lands the `acceptability_relation` half of the C4 "Remaining scope (non-GUI):
the future additive `acceptability_relation` / solver-result-selector schema
members". Before this, rule checks could only ever express `≤`: the runner
hard-coded `ComparisonOperator::LessThanOrEqual` for the top-level
computed-vs-limit acceptability comparison, so a pack could not say a check
passes when the computed quantity is `≥` / `<` / `>` its user-supplied limit
(a minimum-thickness or margin check). The frozen grammar (DEC-022) already
carried all six comparison operators and `rule_pack_document` already had the
string↔operator codec — only the runner's acceptability step was fixed.

Additive, optional, backward-compatible PROPOSAL member (the established
`library_value_ref`→`DEC-038` pattern):

- **Schema** (`schemas/rule_pack.schema.yaml`): optional `acceptability_relation`
  on `CheckDefinition`, enum of the four ordering relations (`less_than`,
  `less_than_or_equal`, `greater_than`, `greater_than_or_equal`). Not in
  `required` → packs without it stay valid; the pytest schema-conformance suite
  passes because the demo/example fixtures omit it.
- **Runner** (`core/rules/rule_check_runner`): `resolve_acceptability_relation`
  (absent/empty → `LessThanOrEqual`; the four tokens → their operator; any other
  explicit token → `Err`) drives the synthesized `Compare(formula, <relation>,
  limit)` and the outcome label. An unrecognized explicit token pushes a blocking
  `RULE_EVALUATOR_ERROR` finding and blocks at `RULE_INPUTS_INCOMPLETE` — never a
  silent `≤` (no-silent-defaults, CONTRACT). Equality acceptance (`equal`/
  `not_equal`) is a deliberate non-goal of the member (float-fragile; a
  boolean-predicate formula can express equality).
- **GUI** (`CheckDefinitionsEditor` + `rulePackService` draft template): a
  relation `EnumSelect`; the draft template authors the explicit
  `less_than_or_equal` default so new packs never rely on the runner's absent
  fallback; out-of-vocabulary stored tokens surface as `(current) …`.

Default rationale: absent → `less_than_or_equal` is a backward-compatibility
default for a *missing* member (preserving every pre-member pack), not a silent
fill of a blank — distinct from the no-silent-defaults rule, which the
unrecognized-token-blocks behaviour upholds.

Evidence: runner `cargo test` 18 (11 unit + 4 new relation integration + 3 demo,
demo unchanged → backward compat); `cargo fmt --check` clean; pytest
`test_rule_pack_schema.py` 5; desktop Vitest 367 (+2 net); `npm run build`
clean; `npx playwright test -g "rule-pack manager"` 2/2 (both viewports; the
r2-smoke spec now drives the relation selector and asserts the rewritten
document). Run record:
`DEL-06-02 .../WORKING_ITEMS_RUN_2026-06-14_TP-C4-ACCEPTREL-001.md`; SMOKE
TP-MAC-163.

Residual: **schema ratification** (PROPOSAL awaiting a human `DEC`; bumps
rule-pack `schema_version` 0.3.0 → 0.4.0 per `DEC-033`); the
`solver-result-selector` member is still a named C4 follow-up.

---

## 2026-06-14 — C4 app-side residual closed: wire the GUI rule-check aggregate into the app-held analysis-run envelope (`TP-C4-APPAGG-001`)

The C4 "Remaining scope (non-GUI): app-side wiring of the GUI run aggregate into
an app-held/exported envelope" residual — the app-side analog of the headless
assembly residual `TP-C4-AGGSTATUS-001` closed the same day. The GUI worst-of
rule-check `aggregate_status` (`RULE_INPUTS_INCOMPLETE` / `USER_RULE_CHECKED` /
`USER_RULE_FAILED`) lived only in the Run Rule Checks panel's ephemeral state.
The app-held analysis-run envelope (DEL-14-02, built by
`previewService.buildAnalysisRunPreview`) composed its `analysis_status` from
`result.status.rule_check`, which a plain solve always leaves at
`RULE_INPUTS_INCOMPLETE` (the solve runs no user rule checks) — so the app-held
envelope, and its `ResultExportPanel` / `ReportPanel` consumers that read
`run.analysis_status`, never reflected an actual rule-check run.

Frontend-only wiring change, no schema/vocabulary change anywhere (`analysis_status`
is already a `string[]` carrying the three rule-check strings):

- New pure, exported `appliedRuleCheckStatus(solveRuleCheck, aggregate?)`
  (mirrors the headless `analysis_status_for_rule_check`): a recognized aggregate
  supersedes the solve `rule_check`; absent/unrecognized falls back to the solve
  `rule_check` — no silent coercion, no false pass.
- `buildAnalysisRunPreview(result, ruleCheckAggregate?)` takes an optional
  aggregate (omitting it reproduces prior behavior byte-for-byte). A recognized
  aggregate is composed into the analysis-run **record's own** status — the one
  bound by the `analysis_run_record` hash and surfaced in `analysis_status`.
  The embedded `result_envelope` hash still binds the **raw solve**
  (`canonicalJson(result)`, unchanged), so the hash-bound solve envelope is
  **never mutated** (two hash scopes, each honest about what it binds — exactly
  the `TP-C4-AGGSTATUS-001` "drive into the record before its checksum binds"
  pattern).
- `RuleCheckRunPanel` gains an optional `onAggregateChange(aggregate | null)`
  callback (backward compatible): the aggregate on a successful desktop run,
  `null` on new pack / browser-only seam / run error. `App` holds the lifted
  aggregate and rebuilds the app-held `analysisRun` from `(result, aggregate)`,
  resetting it wherever the analysis-run is cleared/rebuilt (fresh solve, model
  edit, computed-state clear, blank-create, project open) so it never goes stale.

Evidence: desktop Vitest **365** (+8: 6 previewService — `appliedRuleCheckStatus`
recognized/fallback/no-false-pass; `buildAnalysisRunPreview`
default/recognized-with-byte-stable-result-envelope-hash/USER_RULE_CHECKED/
unrecognized-reproduces-no-aggregate; 2 RuleCheckRunPanel — lifts the aggregate
on a desktop run, clears on new pack and the desktop-only browser seam); `tsc -b`
clean; `npm run build` clean; Playwright e2e 10/10 ×2 viewports (unchanged — no
browser-visible regression). UI evidence posture (H4 exception): no new e2e
spec — the aggregate→envelope flow is desktop-only (`run_rule_checks` needs the
Tauri backend; browser preview produces no aggregate and leaves the envelope
solve-only), so the browser harness cannot exercise it; both composition halves
are Vitest-covered instead (same posture as `TP-C4-AGGSTATUS-001`). Five-surface
DEC-025 sweep PASS. Status-vocabulary-only; conservative fallback prevents any
false pass; `HUMAN_REVIEW_REQUIRED` still always present; raw solve-envelope hash
never mutated; deliverables stay `CHECKING`; no lifecycle/release/professional/
code-compliance claim. Run record:
`WORKING_ITEMS_RUN_2026-06-14_TP-C4-APPAGG-001.md` (DEL-14-02 primary; coupled
DEL-06-02 aggregate source, DEL-08-04 result-export consumer). Remaining C4
(non-GUI): the future additive `acceptability_relation` / solver-result-selector
schema members (likely ratification-gated). Pre-existing test-flake note: the
heaviest full-App round-trip test (`App.test.tsx`) straddles the 10s global
`testTimeout` under full-suite CPU load (passes with headroom in isolation /
at a larger timeout); proven independent of this tranche; flagged as a hardening
candidate, not altered here.

---

## 2026-06-14 — C4 non-GUI residual closed: drive the rule-check aggregate into the result assembly (`TP-C4-AGGSTATUS-001`)

The C4 "Remaining scope (non-GUI): driving `aggregate_status` into the solve
envelope / `result_export`" residual. The `core/rules/rule_check_runner`
orchestrator computes a worst-of `aggregate_status` (`RULE_INPUTS_INCOMPLETE` /
`USER_RULE_CHECKED` / `USER_RULE_FAILED`), but that aggregate lived only in the
GUI run panel. The one concrete non-GUI place that composes a result's
`analysis_status` from a solve — the `core/runner/headless` bridge
`run_preview_in_memory` — hardcoded the rule-check half to `RuleInputsIncomplete`
(it keyed off the product-physics envelope's `status.rule_check`, which a plain
solve always leaves at `RULE_INPUTS_INCOMPLETE`).

Single-crate wiring change (`core/runner/headless`, DEL-10-05; no schema or
vocabulary change anywhere — `result_export`'s `AnalysisStatus` already carried
all three rule-check statuses, so the gap was purely the assembly). Added a pure
`analysis_status_for_rule_check(&str) -> Option<AnalysisStatus>` mapping the three
rule-check vocabulary strings (`None` otherwise — no silent coercion). Refactored
`run_preview_in_memory` to delegate to a new
`run_preview_in_memory_with_rule_check(request, preview_request, Option<&str>)`
(old signature preserved → backward compatible): a recognized aggregate is driven
into the carried `MechanicsEnvelope.status.rule_check` **before its checksum
binds** (so the hash-bound envelope honestly carries the outcome) and the runner
`analysis_status` is derived from the envelope (single source of truth); an
unrecognized non-`None` aggregate is a blocking diagnostic
(`HEADLESS_RUNNER_RULE_CHECK_STATUS_INVALID`), never silently dropped, leaving the
envelope conservatively at `RULE_INPUTS_INCOMPLETE` (no false pass). The bridge
takes the aggregate as its vocabulary `&str`, so the crate gains no new
dependency on `rule_check_runner` (the worst-of aggregation already lives there).

Evidence: `core/runner/headless` cargo **16** (+5: vocabulary mapping incl.
non-matches; user-rule-failed and user-rule-checked driven into the envelope +
`analysis_status`; explicit-incomplete ≡ no-aggregate default; unrecognized-
aggregate blocking diagnostic with the envelope left conservative); `cargo fmt
--check` clean; five-surface DEC-025 sweep PASS (the cargo crate sweep re-runs all
crates; pytest / desktop Vitest+wasm / Playwright ×2 / production build are
unaffected — the crate is not consumed by the frontend/wasm/pytest build graphs).
No SMOKE/Playwright/Vitest entry: the headless runner is a non-GUI core crate not
consumed by the desktop app, so there is no user-visible desktop behavior to
smoke (the crate-only posture `TP-UNITS-B1-CATALOG-001`/`TP-D03-SPARSE-001` used).
Status-vocabulary-only; `HUMAN_REVIEW_REQUIRED` preservation unchanged; no
lifecycle/release/professional/code-compliance claim. Run record:
`WORKING_ITEMS_RUN_2026-06-14_TP-C4-AGGSTATUS-001.md` (DEL-10-05 primary; coupled
DEL-06-02 rule-check aggregate source, DEL-08-04 result-export vocabulary).
Remaining C4 (non-GUI): the future additive `acceptability_relation` /
solver-result-selector schema members; and, separately, app-side wiring of the
GUI run aggregate into an app-held/exported envelope.

---

## 2026-06-14 — C3 residual closed: richer library/record/slot resolution-preview picker in the run panel (`TP-C3-LIBREFPICKER-001`)

The last consistently-named Phase C **C3** residual — "a richer C4 run-panel
library/record/slot picker (the panel surfaces the reference read-only)", carried
by all three prior library-reference slices (`TP-C3C4-LIBREF-001`,
`TP-C3-LIBREFAUTHOR-001`, `TP-C3-LIBREFSECCOMP-001`). Before this, the Run Rule
Checks panel showed each `private_library_value` input's authored
`library_value_ref` as a line of read-only text; a user could not tell from the
panel whether the reference would resolve in their local store, nor discover the
valid record/slot ids, until after a run (`bound_inputs` supplied/MISSING).

Frontend only (no schema/runner/Rust change). New pure helpers in
`libraryImportService.ts`: `indexLibraryRecordsSlots(document, kind)` and
`classifyLibraryReference(records, recordId, slotId)`, whose per-kind dispatch
mirrors the desktop resolver (`src-tauri` `extract_library_slot_value` /
`find_library_slot_value`) read verbatim — material
`material_records[material_id].allowables[allowable_id]`; section
`section_records[section_id].dimensions[dimension_id]|properties[property_id]`;
component `component_records[component_id].fields[field_id]`. In
`RuleCheckRunPanel.tsx`, each referenced library input gains a "Preview
resolution" button that calls the existing desktop store commands
(`list_local_libraries` → find the library + gather available-of-kind;
`open_local_library` → index) and renders a resolution badge (resolves /
library_missing / record_missing / slot_missing / unavailable / unsupported_kind
/ error) plus a read-only browse of available libraries → records → slot ids with
the authored record/slot marked "(referenced)".

Design posture: **read-only to the pack** — the picker never mutates the pack,
never overrides the authored reference at run time (the backend run still
resolves from the pack reference + projectId), and never renders the private
value (only structural ids + status; the value is read only at run time, never
embedded — IP boundary). Run-time *override*/selection in the run panel was
deliberately **not** built: it would duplicate the `DeclarationsEditor` authoring
surface and change the "reference authored in the pack" model, which is a human
design ruling — recorded as a non-goal, not an open residual. Desktop-only: the
local SQLite library store is reachable only via the Tauri path; browser preview
reports the honest store-unavailable seam.

Evidence: desktop Vitest **357** (+12: 7 pure-helper unit tests across all three
kinds incl. section's two slot arrays + de-dup/skip-malformed + classify
resolves/record_missing/slot_missing; 5 panel tests — button presence, resolving
+ browse, library_missing-with-available-list short-circuit, record_missing vs
slot_missing, browser-preview seam); `tsc -b` clean; `npm run build` clean;
`npx playwright test` 10/10 (two viewports — the `r2-smoke.spec.ts` rule-check
test now pastes a `private_library_value` pack and asserts the "Preview
resolution" control + its desktop-only store seam). H4 posture: the
browser-reachable slice is automated in Playwright (the H4 default); the
desktop-store resolution/browse is desktop-only and exercised by the Vitest
desktop-mode mocked-`invoke` suite (the documented exception, same posture as
`TP-C3C4-LIBREF-001`/`TP-C3-LIBREFSECCOMP-001`). Five-surface DEC-025 sweep PASS.
Status-vocabulary-only; no lifecycle/release/professional/code-compliance claim.
Run record: `WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFPICKER-001.md` (DEL-06-02
primary; DEL-03-07 library coupling); SMOKE TP-MAC-161. **C3's named residuals
are now closed.**

---

## 2026-06-14 — DEC-038: ratify `library_value_ref` + bump rule-pack schema_version to 0.3.0 (`TP-C3-LIBREFRATIFY-001`)

The human project authority ratified the additive `library_value_ref` rule-pack
schema member (added as PROPOSAL by `TP-C3C4-LIBREF-001`, companion to DEC-031)
as a permanent part of `schemas/rule_pack.schema.yaml`, and ruled that the
DEC-033 additive-minor versioning policy applies to the rule-pack `schema_version`
too. Because `RequiredInput` is `additionalProperties:false`, a strict validator
predating the member rejects a pack that uses it (forward-incompatible while new
code still reads old packs) — the textbook minor-bump signal — so the rule-pack
schema version moves **0.2.0 → 0.3.0**. Ruling recorded as `DEC-038`; this
tranche is the bounded implementation the ruling authorized.

Implementation: bumped the declared `schema_version` (top-level + the `metadata`
echo) 0.2.0 → 0.3.0 in the canonical demo/example packs
(`examples/rule_packs/invented_demo.yaml` and the byte-identical
`fixtures/product_preview/invented_demo_rule_pack.json`) and re-stamped their
`rule_pack_checksum` (`9910cec… → 60c7ba2…`, recomputed via
`compute_rule_pack_checksum`, RFC 8785 canonical bytes minus the `checksums`
member); bumped the new-draft editor template (`rulePackService.ts`
`buildDraftRulePackDocument`) so new private drafts declare 0.3.0; updated the
one Vitest assertion. The author content version (`metadata.rule_pack_version`)
is unchanged (the demo rules did not change), and `grammar_version` stays 1.0.0
(frozen, DEC-022). The unrelated **model-document** schema version (also 0.2.0,
DEC-033, `projectService.ts`) was deliberately not touched. The invented model
example `examples/models/invented/fake_rule_pack_toy_model.json`, which
hash-references the rule-pack file bytes, had its rule-pack-ref digest and JCS
project hash re-stamped accordingly (caught by the sweep's pytest surface, not
the targeted schema test).

Evidence: the Rust `example_pack_stamped_checksum_matches_recomputation` test
re-passes against the new stamp; five-surface DEC-025 sweep PASS (the bumped
packs still schema-conform — pytest; the wasm/Vitest/Playwright surfaces consume
the re-stamped fixture; production build). Status-vocabulary-only; no
lifecycle/release/professional/code-compliance claim. Run record:
`WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFRATIFY-001.md` (DEL-06-02 primary;
DEL-06-01 schema); SMOKE TP-MAC-160. Decision record: `DEC-038` in
`SOFTWARE_DECOMP.md` §12.

---

## 2026-06-14 — C3 slice landed: resolve section/component library references at run time (`TP-C3-LIBREFSECCOMP-001`)

The C3 residual "section/component slot resolution" — the first-listed residual
carried by both `TP-C3C4-LIBREF-001` (run-time resolution half) and
`TP-C3-LIBREFAUTHOR-001` (authoring half). Before this, a rule-pack
`private_library_value` input could **author** a `library_value_ref` for any of
the three library kinds (the `DeclarationsEditor` selector landed in
`TP-C3-LIBREFAUTHOR-001`), but the run-time resolver `extract_library_slot_value`
resolved **material allowable slots only**, so a `section`- or `component`-kind
reference never resolved and the input silently blocked.

Backend only (`apps/desktop/src-tauri/src/lib.rs`; no schema/runner/frontend
change — the runner's `LibraryValueBinding` is kind-agnostic and the store layer
is already keyed by `(project_id, library_kind, library_id)`):
`extract_library_slot_value` now dispatches by `library_kind` to the
record/slot/value shapes read verbatim from the authoritative library schemas —
`material` → `material_records[material_id].allowables[allowable_id]`
(`value.unit_ref.ref_id`); `section` →
`section_records[section_id].dimensions[dimension_id]` **or**
`…properties[property_id]` (`value.unit`); `component` →
`component_records[component_id].fields[field_id]` (`value.unit`). Material values
reference a unit (`unit_ref.ref_id`) while section/component values carry a plain
`unit` string; the two are mutually exclusive per the schemas
(`additionalProperties: false`), so the resolver reads `unit_ref.ref_id` first
and falls back to `unit`. Material behaviour is unchanged. Two small helpers
factored out (`find_library_slot_value`, `parse_quantity_magnitude`). Unknown
kind / missing record/slot/value / non-numeric magnitude → omitted, so the input
blocks (never a silent pass). IP boundary unchanged: the rule pack carries the
reference only; every kind's private value is read at run time and never
embedded.

Evidence: src-tauri cargo **57** (+4: section dimension+property, component
field, unknown-kind, and a section store-resolution integration test;
`extract_library_slot_value_reads_material_allowable` renamed and its stale
"section is a follow-up" comment corrected); `cargo fmt --check` clean;
five-surface DEC-025 sweep PASS (cargo crate sweep re-runs all crates;
pytest/Vitest+wasm/Playwright ×2/production build unaffected — the change is
isolated to the Tauri-only `src-tauri` crate, outside the frontend/wasm/pytest
build graphs). No Playwright spec extension: library resolution is
desktop-store-only (not browser-reachable; the browser keeps its honest
store-unavailable seam), so the behaviour lives at and is tested by the Rust
command/store path — the same posture `TP-C3C4-LIBREF-001` used for the material
resolution it shipped. Status-vocabulary-only; private values never embedded or
committed; no lifecycle/release/professional/code-compliance claim. Run record:
`WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFSECCOMP-001.md` (DEL-06-02 primary;
DEL-03-07 library coupling); SMOKE TP-MAC-159.

Residuals: a richer C4 run-panel library/record/slot picker (the panel surfaces
the reference read-only); human ratification of the additive `library_value_ref`
schema member (PROPOSAL, companion to DEC-031).

---

## 2026-06-14 — C3 authoring slice landed: author `library_value_ref` in the rule-pack editor (`TP-C3-LIBREFAUTHOR-001`)

The C2-authoring half of the rule-pack ↔ private-library round-trip, completing
the C3 residual hand-off from `TP-C3C4-LIBREF-001` (which landed the run-time
resolution half). Before this, a `private_library_value` required input's
`library_value_ref` could only be set by hand-editing raw document JSON; now the
declarations form-builder authors it through structured controls.

GUI (`apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx`): when a
required input's `source_kind` is `private_library_value`, a reference sub-form
appears — a `library_kind` select (material/section/component, the verbatim
schema vocabulary, no "TBD" member) plus `library_id`/`record_id`/`slot_id` text
fields, an IP-boundary note, and a "Remove library reference" control. Switching
to `private_library_value` seeds a complete four-member reference (kind → first
kind; ids → visible uppercase `"TBD"` placeholders), so an unfilled reference
resolves to nothing and the input blocks — never a partial schema shape, never a
silent pass (CONTRACT no-silent-defaults). A reference left after the source_kind
changes away stays visible and removable (never silently hidden); every other
member of the input round-trips verbatim. Frontend-only: no schema/Rust/Python
change (the optional member and its resolution landed in `TP-C3C4-LIBREF-001`;
the Value-based `validate_rule_pack_document` tolerates it). Not gated by D-02b
(structured form authoring, not writable expression text syntax).

Evidence: desktop Vitest 345 (+5: LIBRARY_KINDS vocab, default-ref shape,
seed-on-switch, first-edit completion, stale-ref visible+removable); `npm run
build` clean; `npx playwright test` 10/10 (two viewports — the rule-pack manager
e2e test now authors a `library_value_ref` and asserts it in the canonical draft
JSON). Status-vocabulary-only; reference-only, private value never embedded or
committed; no lifecycle/release/professional/code-compliance claim. Run record:
`WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFAUTHOR-001.md` (DEL-06-02 primary;
DEL-03-07 library coupling); SMOKE TP-MAC-158.

Residuals: section/component slot resolution (material allowables resolve today);
a richer C4 run-panel library/record/slot picker (the panel surfaces the
reference read-only); human ratification of the additive `library_value_ref`
schema member (PROPOSAL).

---

## 2026-06-14 — C3↔C4 library-reference resolution landed (`TP-C3C4-LIBREF-001`)

The coupled C3 ("rule-pack ↔ library reference wiring") and C4
("`private_library_value`-sourced input resolution") residuals, as a
backend-resolution slice (the reference mechanism + run-time resolution;
C2-authoring and a richer C4 picker are follow-ups). Before this, a rule-pack
required input could declare `source_kind: private_library_value` but had no way
to say which library value it meant, so the C4 runner treated it as unsupplied.

Schema (additive, **PROPOSAL** awaiting human ratification — companion to
DEC-031; DEC-033 additive-minor policy): optional `library_value_ref`
(`library_kind`/`library_id`/`record_id`/`slot_id`) on `RequiredInput`. IP
boundary: the rule pack carries the reference only; the private value is read at
run time from the local store and never embedded in the pack.

Runner (`core/rules/rule_check_runner`): `LibraryValueBinding` + `library_values`;
`private_library_value` inputs bind from a resolved library value (note cites
the reference) or stay unsupplied + note (never a silent pass). Desktop command:
`run_rule_checks` refactored into a store-free `run_rule_checks_core` + a wrapper
that resolves `library_value_ref` from the local private-library store (material
allowable slots) into bindings; unresolvable references are omitted so the check
blocks; `project_id` derived from the explicit arg / model / envelope. Frontend:
the run panel passes `projectId` and surfaces each library input's reference with
an honest store-resolution / never-embedded note.

Evidence: runner cargo 11 unit + 3 integration; src-tauri cargo 53 (+5);
`pytest tests/test_rule_pack_schema.py` 5; desktop Vitest 340 (+1); `npm run
build` clean; `npx playwright test` 10/10 (two viewports); `cargo fmt --check`
clean. Status-vocabulary-only; private values never embedded or committed; no
lifecycle/release/professional/code-compliance claim. Run record:
`WORKING_ITEMS_RUN_2026-06-14_TP-C3C4-LIBREF-001.md` (DEL-06-02 primary;
DEL-06-01 schema; DEL-03-07 library coupling); SMOKE TP-MAC-157.

Residuals: C2 authoring of `library_value_ref` in the declarations form-builder;
section/component slot resolution (material allowables resolve today); a richer
C4 GUI library picker; human ratification of the additive schema member.

## 2026-06-14 — C4 GUI slice landed: run rule checks from the GUI (`TP-C4-CHECKGUI-001`)

The Phase C4 GUI residual that makes the R3 exit criterion (PRD §22.4)
GUI-true. The C4 backend (`TP-C4-CHECKRUN-001`) made rule checks runnable but
had no GUI surface — no frontend route for `run_rule_checks` and no panel that
runs checks (the pre-existing `RuleCheckPanel.tsx` is a *completeness review*).
This slice adds the app-side surface only; no backend, schema, or example-pack
change.

New `apps/desktop/src/services/ruleCheckService.ts`: typed `runRuleChecks(...)`
route over the existing `run_rule_checks` Tauri command, with the honest
desktop-only unavailable seam in browser preview (no synthesized fallback,
mirroring the rule-pack/library services); result types mirroring the runner
crate; a pure, unit-tested `deriveRuleCheckBindingPlan(document)` that
partitions a pack's `required_inputs` by `source_kind` and collects
`value_slots`; and `loadDemoRuleCheckPack()`.

New `RuleCheckRunPanel` (`apps/desktop/src/features/rule-check/`), mounted in
the Solve workspace section beside the completeness panel. Pack source is the
bundled invented demo pack
(`fixtures/product_preview/invented_demo_rule_pack.json`, byte-parallel to the
backend example `examples/rule_packs/invented_demo.yaml`), a saved local-store
pack (reuses `listLocalRulePacks`/`openLocalRulePack` — the author→save→run
journey), or pasted JSON. Binding controls are derived from the loaded pack: a
solved-result-row select per `solver_result` input, value+unit entry per
user-supplied input and value slot (dimension from the pack), and a deferred
note per `private_library_value` input (C3 residual — treated as unsupplied,
blocks). The panel renders the worst-of aggregate status (pass/fail/blocked
label + `data-status`), per-check outcomes (status, computed/limit quantity,
acceptability relation, supplied/MISSING bound inputs, completeness + evaluator
findings, diagnostic codes), and the professional-boundary notice. Unbound or
missing inputs block the check at `RULE_INPUTS_INCOMPLETE` — pass/fail is never
reported on missing inputs.

Evidence: desktop Vitest **339/339** (+13: 7 service + 6 panel); `npm run
build` clean; `npx playwright test` **10 passed** (5 specs × two viewports,
incl. the new run-checks spec); live-browser preview confirmed the panel + demo
binding controls. The runner is Tauri-only; pass/fail/blocked outcomes are
covered by the src-tauri Rust command tests and the Vitest desktop-mode mocked
panel suite, and the browser e2e asserts the honest desktop-only run seam.
Status-vocabulary-only; no lifecycle/release/professional/code-compliance claim.
Run record: DEL-06-02 (primary) + DEL-07-04
`WORKING_ITEMS_RUN_2026-06-14_TP-C4-CHECKGUI-001.md`; SMOKE TP-MAC-156. D-02b
(`AWAITING_RULING`) does not block — checks run on the authored declarative AST.

Residual: C4's non-GUI hand-offs remain open (private-library input resolution
with the C3 slice; driving `aggregate_status` into the solve envelope /
`result_export`; additive `acceptability_relation` / solver-result-selector
schema members). With the GUI landed, C4's R3-exit-blocking GUI scope is
complete; the named R3-exit blocking residuals (F-4 packaged human journey, A3
authoring-usability) are unchanged.

## 2026-06-13 — C4 backend slice landed: rule-check orchestration + Tauri command (`TP-C4-CHECKRUN-001`)

First Phase C item **C4** slice and the one that makes rule checks *runnable*:
the rule-check engine crates (`expression_evaluator`, `completeness_checker`,
`rule_pack_document` codec) were built and tested but **orphaned** — no Tauri
command ran a rule pack against a solved model, and `run_preview_mechanics`
hard-codes `rule_check: "RULE_INPUTS_INCOMPLETE"`. This slice wires them into a
runnable path toward the PRD §22.4 / R3 exit criterion ("user defines a private
non-code rule pack and runs checks; software blocks pass/fail on missing
inputs").

New crate `core/rules/rule_check_runner` (`open_pipe_stress_rule_check_runner`)
exposes `run_rule_checks(&RuleCheckRunInput) -> RuleCheckRunResult`. Per check,
in document order: build completeness inputs and run `check_completeness`
(blocked → `RULE_INPUTS_INCOMPLETE`, formula never evaluated — the "blocks
pass/fail on missing inputs" guarantee); else `decode_expression` the
frozen-grammar formula AST, bind the resolved values, `evaluate`; resolve the
user-supplied value-slot limit; synthesize `Compare(formula, <=, limit)` and
evaluate it through the same evaluator (unit/dimension safety in one place);
map the boolean to `USER_RULE_CHECKED` / `USER_RULE_FAILED`; enforce the check's
declared `result_statuses`; aggregate worst-of (`USER_RULE_FAILED` >
`RULE_INPUTS_INCOMPLETE` > `USER_RULE_CHECKED`). A boolean-valued formula is
used directly as the predicate. New `run_rule_checks` Tauri command validates
the pack, obtains the solved envelope (supplied or via `run_preview_mechanics`),
resolves caller-supplied `{input_id, result_id}` selectors against the
envelope's `results[]`, and returns the serialized run result.

The two composition semantics — `<=` acceptability and the caller-supplied
solver-result binding — are **C4-defined wiring**, explicitly routed to C4 by
the DEL-06-01 `TP-C2-SCHEMA-001` run record (§"Residuals and hand-offs"; the
schema deliberately left them open) and grounded in the PRD §12.5
ratio-vs-allowable shape. No human decision was taken; D-02b (writable
expression text syntax) is `AWAITING_RULING` and does not gate running the
already-authored AST.

Evidence: `rule_check_runner` 9 unit + 3 integration (the integration suite
runs the real `examples/rule_packs/invented_demo.yaml`: ratio 0.5 →
`USER_RULE_CHECKED`, 1.5 → `USER_RULE_FAILED`, dropped solver binding →
`RULE_INPUTS_INCOMPLETE`); desktop backend 48 (43 prior + 5 new command tests),
no regression; `expression_evaluator` 32 / `completeness_checker` 12 /
`rule_pack_document` 10 regression green; `cargo fmt --check` clean. No UI/TS
change → no Vitest/Playwright/SMOKE.md entry this slice. Run records:
DEL-06-02 (primary), DEL-06-03 (pointer).

Residual hand-offs (remain in the C4 row): GUI "Run checks" action + per-check
results panel (`TP-C4-CHECKGUI-001`, makes the exit criterion GUI-true);
`private_library_value` resolution (C3 residual); driving `aggregate_status`
into `MechanicsEnvelope.status.rule_check` / `result_export`; future additive
`acceptability_relation` and solver-result-selector schema members.

## 2026-06-13 - C3 GUI slice landed: private library manager panel (`TP-C3-LIBGUI-001`)

Fourth Phase C item **C3** sub-slice and the one that makes C3 user-facing: the
import-wizard **GUI** the foundation/seam/store slices handed off. Selected per
the `_COORDINATION.md` loop as the earliest unblocked R3/Phase C dependency-spine
item (C2 done → C3 → C4); not blocked by any human decision (`D-02b` gates only
C2 expression text syntax). Replicates the C2 rule-pack **manager GUI**
(`RulePackManagerPanel`, `TP-C2-EDITOR-001`).

New `LibraryManagerPanel` (`apps/desktop/src/features/library/`) and a
**"Libraries" workspace section** in `App.tsx`, placed immediately before the
rule-pack manager (journey order operations → loads → libraries → rule-packs →
solve) because both are private local-only asset managers and rule packs
reference imported library allowables. Controls: library-kind
(material/section/component) and intended-visibility selectors (the latter
labeled *validation preview*, with an explicit note that **save always persists
to the private local store**); a built-in **invented private starting template**
per kind (`buildInventedLibraryImportTemplate`, mirroring the desktop backend's
own canonical accepted payload so the happy path validates clean); an
import-document JSON textarea; validate / save / discard; and a project-scoped
list with per-entry open/delete. **No backend change** — it surfaces the prior
slices' `validate_library_import` / `save_local_library` / `open_local_library`
/ `list_local_libraries` / `delete_local_library` commands verbatim.

**PRD §13.5 display + DEC-036 honesty.** Findings render split into a *Blocking /
quarantine* group (blocks acceptance) and an *Advisory* group (review before
acceptance), via `partitionLibraryImportFindings`. Save shows `stored=<bool>`;
when a blocked/suspected-protected import is refused, the status carries an
explicit `DEC-036 refuse-to-store` note and the blocking validation rides through
to the §13.5 display. The boundary note states local-only, never
committed/transmitted/bundled, refuse-to-store, and software-findings-only.

User-visible desktop surface ⇒ per the H4 posture the default UI evidence is the
Playwright e2e spec extension (real browser, both viewports: Libraries nav +
invented-sample load + honest desktop-only seam) plus the new Vitest component
suite; the §13.5-partition and refuse-to-store paths are exercised via mocked
`invoke` because that backend is desktop-only (the documented reason browser e2e
does not cover accept/store — those are covered by the src-tauri Rust tests).
Live-browser confirmed via the preview tools. Validation: desktop Vitest 326/326
(16 files, 9 new); `npm run build` clean; `npm run test:e2e -- --grep "library
manager"` 2 passed; DEC-025 five-surface sweep at HEAD. No lifecycle change;
DEL-03-07 stays CHECKING; no professional/certification/code-compliance claim.

Remaining C3: rule-pack ↔ library reference wiring (couples C2/C3; its own
slice); then C4 end-to-end rule checks on authored models consuming imported
libraries.

Evidence: DEL-03-07 run record
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-07_Public-private library import provenance checker/_run_records/WORKING_ITEMS_RUN_2026-06-13_TP-C3-LIBGUI-001.md`;
`apps/desktop/SMOKE.md` TP-MAC-155.

## 2026-06-13 - C3 store slice landed: local-only private-library persistence (`TP-C3-LIBSTORE-001`)

Third Phase C item **C3** sub-slice: local-only persistence (CRUD) for imported
material/section/component libraries, the store half mirroring the C2
`local_rule_packs` store (`TP-C2-RPLIFE-001`). Human-approved continuation of
the C3 spine after the foundation crate and the validation seam.

Store **v11 migration** (`STORE_SCHEMA_TARGET_VERSION` 10 → 11) creates
`local_libraries` (project-scoped, keyed `(project_id, library_kind,
library_id)`, local SQLite only; the three migration-ledger evidence tests were
updated for the new step). New Tauri commands `save_local_library` /
`open_local_library` / `list_local_libraries` / `delete_local_library` (in
`apps/desktop/src-tauri/src/lib.rs`) and the matching typed
`libraryImportService.ts` routes with the shared browser-unavailable seam.

**Persistence policy (private-by-default, accepted-only).** `save_local_library`
re-validates each document at the import boundary with private visibility and
stores **only an accepted (`PRIVATE_LOCAL_ONLY`) import** (the named gate
`library_import_is_storable`); a suspected-protected (`QUARANTINE`) or blocked
(`REJECTED`) import is refused — `stored:false` with its findings, nothing
written — so suspected protected content never reaches the store (IP boundary).
This deliberately differs from the rule-pack "drafts always saveable" store
because a library *import* carries external-IP risk the DEL-03-07 checker exists
to gate. The human project authority **accepted this refuse-to-store default on
2026-06-13 (`DEC-036`)**; a stored-audit-trail variant is not pursued unless a
later ruling reopens it (run record §Persistence policy).

No GUI, no file parsing, no wizard in this slice (next C3 slices). Backend store
seam ⇒ per the H4 posture the owed evidence is the Rust store/gate tests + the
Vitest service tests; the wizard GUI's Playwright/Vitest evidence rides the next
C3 slice. Validation: src-tauri `cargo test` 43/43 (3 new store/gate tests +
3 updated migration tests), `cargo fmt --check` clean; desktop Vitest 317/317
(15 files, 4 new); `npm run build` clean; DEC-025 five-surface sweep at HEAD. No
lifecycle change; DEL-03-07 stays CHECKING; no professional/certification/
code-compliance claim; private libraries never committed or transmitted.

Evidence: DEL-03-07 run record
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-07_Public-private library import provenance checker/_run_records/WORKING_ITEMS_RUN_2026-06-13_TP-C3-LIBSTORE-001.md`;
`apps/desktop/SMOKE.md` TP-MAC-154.

## 2026-06-13 - C3 seam slice landed: `validate_library_import` command + typed service (`TP-C3-IMPORTCMD-001`)

Second Phase C item **C3** sub-slice: the **desktop seam** exposing the C3
foundation crate (`open_pipe_stress_library_import_document`, the DEL-03-07
port landed at `TP-C3-IMPORTVALIDATE-001`) through the app boundary. Selected
as the earliest unblocked R3/Phase C dependency-spine item; the prior slice's
run record named it verbatim as "C3-next (seam)". Mirrors the C2 backend-seam
ordering (`TP-C2-RPLIFE-001`) and the B2 units seam.

New Tauri command `validate_library_import(payload, libraryKind,
intendedVisibility)` (`apps/desktop/src-tauri/src/lib.rs` + Cargo dep) wraps the
crate's `validate_library_import_tokens`, rejecting unsupported
`library_kind`/`intended_visibility` tokens rather than guessing, and projects
the `ImportValidationResult` into a stable envelope (`outcome`, `accepted`,
`has_blocking_findings`, ordered `findings`, the PKG-02 `import_boundary`
`diagnostics` projection, and a software-findings-only
`professional_boundary_notice`). New typed frontend seam
`apps/desktop/src/services/libraryImportService.ts`: `validateLibraryImport`
returns a discriminated route — browser preview returns an explicit
`LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY` diagnostic and never calls `invoke` (no
synthesized fallback) — plus `partitionLibraryImportFindings`, which splits
findings along the PRD §13.5 blocking-vs-advisory axis once at the seam.

No persistence, no file parsing, no UI surface in this slice — validation is a
pure function over an already-parsed payload (those are the next C3 slices).
Backend seam ⇒ per the H4 posture the owed evidence is the Rust command tests
plus the Vitest service test; the import-wizard GUI's Playwright/Vitest evidence
rides the next C3 slice (as `TP-C2-RPLIFE-001` did for the rule-pack seam).
Validation: src-tauri `cargo test` 40/40 (3 new command tests), `cargo fmt
--check` clean; desktop Vitest 313/313 (15 files, 4 new); `npm run build` clean;
DEC-025 five-surface sweep at HEAD. Pre-existing src-tauri `clippy -D warnings`
debt (10 errors on clean HEAD; never a gate — the sweep runs `cargo test`) left
untouched and recorded in the run record. No lifecycle change; DEL-03-07 stays
CHECKING; no professional/certification/code-compliance claim; no protected or
private data.

Evidence: DEL-03-07 run record
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-07_Public-private library import provenance checker/_run_records/WORKING_ITEMS_RUN_2026-06-13_TP-C3-IMPORTCMD-001.md`;
`apps/desktop/SMOKE.md` TP-MAC-153.

## 2026-06-13 - C3 foundation slice landed: library-import provenance crate (`TP-C3-IMPORTVALIDATE-001`)

First bounded increment of completion-plan Phase C item **C3** (private library
management GUI), selected as the earliest unblocked R3/Phase C dependency-spine
item after C2 landed. C3 is large (~6 sub-slices); this slice builds the runtime
validation foundation every downstream C3 surface needs.

New standalone crate `core/library_import/library_import_document`
(`open_pipe_stress_library_import_document`) is the **runtime Rust port** of the
DEL-03-07 import-provenance contract authored in
`core/library_import/provenance_checker.py`. The Python module is the design
authority but cannot run in the Tauri/Rust desktop runtime, so absorbing
DEL-03-07's mature CHECKING design into the application means porting it — exactly
as `open_pipe_stress_rule_pack_document` underpins the C2 editor and the B1 units
crate landed crate-first. Public API: `validate_library_import(payload, kind,
visibility)` and the token-parsing seam helper `validate_library_import_tokens`
(rejects unsupported tokens, never guesses); `LibraryKind`/`IntendedVisibility`
enums; `ImportFinding`/`ImportValidationResult` with a PKG-02 import-boundary
diagnostic-envelope projection. Ported one-for-one: the 7 required provenance
fields; missing-metadata / missing-record-set / non-object-record blocks;
missing/incomplete provenance; protected-suspected quarantine; rejected-source
block; the public-disposition ladder (private-data-public block → missing
redistribution rights → unaccepted redistribution → review required); nested
unit-bearing-value unit-metadata and value-provenance checks; and the
severity-precedence outcome (`QUARANTINE > REJECTED > REVIEW_REQUIRED >
PRIVATE_LOCAL_ONLY / ACCEPTED_PUBLIC`). Python truthiness and the
`item.get(x) or provenance.get(x)` status resolution are reproduced explicitly.

Cross-language parity is the headline guarantee: `tests/provenance_parity.rs`
mirrors all seven `tests/test_library_import_provenance.py` cases over the **same**
invented fixtures, so the runtime port and the authored contract must move
together. Plus 11 lib unit tests cover branches the shared fixtures don't reach
(rejected source, privacy-class public block, item-over-provenance precedence,
token rejection, incomplete-field sorting, nested-value flags, diagnostic shape).

No `apps/desktop` change in this slice — no Tauri command, service, persistence,
or UI yet (the next C3 slices). No other crate depends on it, so no other Rust/
Python/TS surface is affected. Validation: `cargo test` 18/18; `cargo clippy -D
warnings` clean; Python oracle `pytest tests/test_library_import_provenance.py`
still 7/7. The new manifest is auto-discovered by the DEC-025 sweep's
`cargo_crate_sweep` surface. No lifecycle change; DEL-03-07 stays CHECKING; no
professional/certification/code-compliance claim; no protected or private data.

Regression-gate note (loop step 3.2): the desktop Vitest baseline looked broken
on a direct `npm test` (34 fails) but was traced fully to a missing
`build:wasm:desktop` prebuild (DEC-020 wasm engine) plus `App.test.tsx`
concurrent-load timeouts; with the wasm engine built and the suite run unloaded
it is green (51/52, the lone holdout a save/open round-trip with a hardcoded
10 000 ms budget that passes in isolation), matching every recent sweep's
`desktop_vitest => pass` at the C2 HEAD. No regression.

Evidence: DEL-03-07 run record
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-07_Public-private library import provenance checker/_run_records/WORKING_ITEMS_RUN_2026-06-13_TP-C3-IMPORTVALIDATE-001.md`;
crate README `core/library_import/library_import_document/README.md`.

## 2026-06-13 - C2 editor GUI slice 5 landed: check-definitions form builder; C2 form-builder series complete (`TP-C2-CHECKDEF-001`)

Adds the last rule-pack document-structure form builder: a structured editor for
`check_definitions`, the member that binds the declared inputs/slots and a
formula into an acceptability check. New
`apps/desktop/src/features/rule-packs/CheckDefinitionsEditor.tsx` adds / removes
/ edits each check through form controls — text `check_id`/`name`/`description`;
reference pickers for `formula_ref` (over declared formula ids) and
`required_input_refs` / `value_slot_refs` (add/remove lists over declared
input/slot ids, each held at the `minItems:1` floor); a closed-set
`acceptability_basis` select; a `result_statuses` checkbox multi-select (the six
`AnalysisStatus` tokens, floored at one); and the eight `diagnostic_policy`
condition→code selects. **Add check** seeds a schema-valid default
(`CheckDefinition` shape verified by `toEqual`) with a fresh unique id and refs
bound to the first declared input/slot/formula; **Remove** is floored at
`minItems:1`. The shared field primitives `Field`/`TextField`/`EnumSelect` were
exported from `DeclarationsEditor` for reuse (the codebase already shares
`DimensionSelect` this way). The editor mounts after the expression composer in
`RulePackManagerPanel`; all three editors read the same draft document. An
out-of-vocabulary stored enum surfaces as a "(current) X" option and an
unresolved/`TBD` reference as an "(unresolved) X" option — never silently
snapped (CONTRACT no-silent-defaults). Lossless: a patched check keeps every
member it carried (provenance, description, any `Reference.version`), a
diagnostic-policy edit preserves the other seven bindings, and untouched sibling
checks round-trip verbatim.

With this slice the **C2 rule-pack-editor form-builder series is complete**:
every document-structure authoring member — `required_inputs`, `value_slots`,
the full grammar-v1.0.0 expression AST (incl. table-backed nodes), and
`check_definitions` — now has a structured form builder. The advanced metadata
members (diagnostics, classification, checksums, provenance,
professional_boundary, open_decisions) remain raw-JSON-editable by design, a
bounded recorded residual rather than a form-builder gap. The next Phase C
dependency-spine items are C3 (private-library management GUI) and C4
(engine-side end-to-end rule checks on solved user models).

D-02b gate held: a check binds *references* and selects closed-vocabulary tokens
through pickers/checkboxes — no writable expression text syntax and no text
rendering of any AST; the typed AST stays the sole edited, checksum-bound
expression form (DEC-022). No invented engineering/standards value; the software
authors the check *shape* only and never decides pass/fail acceptance (the
engine independently blocks incomplete checks — C4). Private rule packs stay
local-only.

Validation: rule-packs Vitest **67/67** (was 43), full desktop Vitest
**308/308** (14 files), `tsc -b` clean, Playwright `-g "rule-pack manager"`
**2/2** (both viewports, check editor driven from blank). An independent
four-lens adversarial review (schema-conformance, governance/D-02b/IP, React
correctness, test honesty) returned **no BLOCKER / no SHOULD-FIX**, with a
re-run JSON-Schema validator confirming the default shapes and enum sets; its one
NIT (no test pinned the result-status floor for an out-of-vocabulary last token)
was folded in as an added Vitest case. Run record:
`execution/PKG-07_.../DEL-07-03_.../_run_records/WORKING_ITEMS_RUN_2026-06-13_TP-C2-CHECKDEF-001.md`;
SMOKE TP-MAC-152. DEL-07-03 remains CHECKING; no lifecycle/release/professional
claim.

## 2026-06-13 - C2 editor GUI slice 4 landed: required-input / value-slot declaration form builders (`TP-C2-DECLEDITOR-001`)

Adds a structured editor for the rule pack's variable declarations — its
`required_inputs` and user-supplied `value_slots`, the named variables a
formula's `variable_ref` binds to. New
`apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx` adds / removes /
edits each entry through form controls: text `input_id`/`name`/`unit_ref`,
closed-set selects for `source_kind` / `slot_kind` / `value_status` /
`required_for` / `completeness_status`, and the shared `DimensionSelect` for
the `quantity_intent` dimension. **Add** seeds a schema-valid default
(`RequiredInput` / `UserSuppliedValueSlot` shapes verified by `toEqual`) with a
fresh unique id; **Remove** is blocked at the `minItems:1` schema floor for
both arrays. The editor mounts ahead of the expression composer in
`RulePackManagerPanel`; both read the same draft document, so a variable added
through the editor is immediately bindable in the composer (no raw JSON). The
enum vocabularies are exported consts copied verbatim from the schema and an
out-of-vocabulary stored token surfaces as a "(current) X" option, never
silently snapped (CONTRACT no-silent-defaults). Lossless: a patched entry keeps
every member it carried (provenance, the const-true relaxation flags,
missing-value diagnostic), a nested `quantity_intent` edit preserves
`unit_required`/`dimension_check_required`, and untouched siblings round-trip
verbatim. `draftPlaceholderProvenance()` was extracted from
`rulePackService.ts` (byte-identical) so the template and the declaration
defaults share one private-by-default provenance source. No numeric allowable
value is invented — `UserSuppliedValueSlot` carries a dimension/unit intent and
a `value_status` (`not_provided` default), not a number.

Authority-vs-plan correction: the slice-2/3 residual wording "required-input /
value-slot / **load-combination** form builders" was a misnomer — the
authoritative `schemas/rule_pack.schema.yaml` top-level object is
`additionalProperties:false` and has **no `load_combinations` member** (load
combinations are a model concept, Phase A4). The rule pack's third
user-authored document-structure member is `check_definitions`. Per the
`_COORDINATION.md` state-tracking rule, the plan C2 row and this log are
corrected to name `check_definitions` as the remaining C2 form-builder
sub-surface; the schema (authority) is unchanged.

D-02b gate held: purely structured form controls — the editor authors variable
*declarations*, with no writable expression text syntax and no text rendering
of any AST; the typed AST stays the sole edited, checksum-bound expression form
(DEC-022). Private rule packs stay local-only; no protected tables/constants.

Validation: rule-packs Vitest **43/43** (was 26), full desktop Vitest
**285/285**, `tsc -b` clean, Playwright `-g "rule-pack manager"` **2/2** (both
viewports, declarations editor driven from blank). An independent four-lens
adversarial review (schema-conformance, governance/D-02b/IP, React correctness,
test honesty) returned a SHIP verdict; its one should-fix was folded in before
commit — the shared `DimensionSelect` silently display-snapped an
out-of-vocabulary stored dimension to `"TBD"` instead of surfacing it, so it was
given the same "(current)" escape hatch as the new `EnumSelect` (also tightening
the composer's literal/table dimension fields), with a guarding Vitest case
added. Run record:
`execution/PKG-07_.../DEL-07-03_.../_run_records/WORKING_ITEMS_RUN_2026-06-13_TP-C2-DECLEDITOR-001.md`;
SMOKE TP-MAC-151. DEL-07-03 remains CHECKING; no lifecycle/release/professional
claim. Residual: the `check_definitions` form builder (and the C4 end-to-end
rule-check path) remain the next C2/Phase C work.

## 2026-06-13 - C2 editor GUI slice 3 landed: table-node structured sub-editor (`TP-C2-TABLENODE-001`)

Completes structural authorability of the frozen grammar v1.0.0 in the
rule-pack expression composer. `interpolate` and `lookup` nodes — preserved
read-only by slice 2 — are now first-class authorable node types in
`apps/desktop/src/features/rule-packs/ExpressionComposer.tsx`. Both are added
to `EDITABLE_NODE_TYPES` (so they are reachable from every node-type selector
and `defaultExpressionNode` seeds a schema-valid default table: two
strictly-increasing rows, uppercase `"TBD"` placeholders, plus the recursive
`argument` child; `lookup` also seeds `mode: "exact"`). A structured table
sub-editor authors `table_id`, the argument/result `dimension` (via a shared
`DimensionSelect`) and `unit_ref`, the `{argument, result}` rows (**Add row**
appends last-argument + 1 to keep strict monotonicity; **Remove row** is
blocked at the single-row schema floor), the `lookup` `mode` selector, and the
recursive editor for the table's `argument` expression. With this, no grammar
node type is left to raw-JSON editing; only the refusal markers
(`unsupported_form`/`unsafe_host_access`) and unrecognized tags remain
preserved read-only. Lossless: row edits patch only the touched row, so
untouched rows and sibling subtrees round-trip verbatim.

Regression repair folded in: slice 2 defaulted a literal's dimension to the
lowercase `"tbd"` token and listed it in the shared `DIMENSIONS` vocabulary.
That token is not in the document codec (`core/rules/rule_pack_document`
`decode_quantity` → "unknown dimension token 'tbd'") nor the schema
`DimensionId` enum — both use uppercase `"TBD"` (the token the draft builder
and the evaluator's `Dimension::Tbd => "TBD"` already use). A composer-authored
default literal therefore produced a document that fails backend validation/
save; the slice-2 Vitest missed it because it asserted the produced string,
never a Rust round-trip. Fixed `DIMENSIONS`, the literal default, and the
fallback to `"TBD"`, and added a unit guard.

D-02b gate held: purely structured form controls — no writable expression text
syntax and no text rendering of the AST; the typed AST stays the sole edited,
checksum-bound form (DEC-022). No protected tables/constants; private rule
packs stay local-only.

Validation: rule-packs Vitest 26/26 (was 21), `tsc -b` clean, Playwright
`-g "rule-pack manager"` 2/2 (both viewports, table sub-editor driven from
blank). A five-lens pre-commit review caught one schema-conformance blocker
(the slice-2 default literal emitted `unit_required`/`dimension_check_required`,
which the schema's `additionalProperties:false` `ExpressionQuantity` forbids) —
fixed by emitting only `{value, dimension, unit_ref}`; details in the run
record. The full Vitest run on a busy host showed the documented load-induced
`App.test.tsx` per-test timeout flake (all 34 failures timeouts in that
untouched file); confirmed tranche-independent — once load cleared
`App.test.tsx` passed 52/52 on both my tree and pristine HEAD, and no failing
test renders the changed code. Detail and the gate-determinism follow-up are
in the run record. Evidence: `apps/desktop/SMOKE.md` TP-MAC-150;
DEL-07-03 run record `WORKING_ITEMS_RUN_2026-06-13_TP-C2-TABLENODE-001.md`.
Residual: the required-input/value-slot/load-combination form builders remain
the last C2 authoring sub-surface; C4 (engine-side rule checks on solved user
models) is a separate Phase C item.

## 2026-06-13 - C2 editor GUI slice 2 landed: structured AST expression composer (`TP-C2-COMPOSER-001`)

The PRD §14.5 "Expression editor". New
`apps/desktop/src/features/rule-packs/ExpressionComposer.tsx` is a recursive
form/tree editor that builds the frozen grammar v1.0.0 typed expression AST
(DEC-022) for a rule pack's selected formula, replacing the slice-1 raw
declarative-AST JSON editing of expressions. Structured controls cover the
full non-table node set (literal, variable_ref, unary, binary, compare,
logical, select, aggregate — with add/remove aggregate operands and a
last-operand-removal block); a variable picker / read-only browser is sourced
from the pack's declared `required_inputs`/`value_slots` (PRD §14.5 "Variable
browser"). The composer derives its tree from, and re-serializes edits back
into, the canonical document JSON the validate/checksum/save flow already
reads (single source of truth). `RulePackManagerPanel` also gains the
in-request busy guard that closes the slice-1 clobber residual: the textarea,
composer, and all actions disable with a stated reason while a backend
request is awaiting.

D-02b gate held: the composer is purely structured — no writable expression
text syntax and no text rendering of the AST (read-only rendering is itself
an open D-02b §3 Q5 question). The typed AST stays the sole edited and
checksum-bound form. Lossless preservation: table-backed (interpolate/lookup)
and unrecognized nodes render read-only with no node-type selector and
round-trip byte-for-byte — never silently dropped.

A four-lens pre-commit adversarial review (react-correctness,
governance/D-02b boundary, AST-encoding correctness, evidence-honesty) found
no implementation defects; three test-honesty findings were fixed before
commit (vacuous table-preservation `toMatchObject` → full `toEqual`; added a
test that an unrecognized node survives a sibling edit; shallow e2e
assertion → full composed-structure match).

Validation: targeted Vitest `src/features/rule-packs` 21/21; Playwright
`-g "rule-pack manager"` 2/2 (two-viewport, composer driven from blank);
`tsc -b` clean. The full-suite `App.test.tsx` timeout flakiness is
pre-existing and host-environmental (reproduces on pristine HEAD with this
tranche's panel change reverted; varying failing set), unrelated to this
slice. Residuals: table-node structured sub-editor; required-input/
value-slot/load-combination form builders; engine-side rule evaluation
(C4). Evidence: DEL-07-03 run record
`WORKING_ITEMS_RUN_2026-06-13_TP-C2-COMPOSER-001.md`; SMOKE TP-MAC-149.

## 2026-06-12 - C2 editor GUI slice 1 landed: rule-pack manager (`TP-C2-EDITOR-001`)

The first runtime rule-pack editor surface (PRD §14.5 / §14.1 "Rule-pack
manager"), consuming the TP-C2-RPLIFE-001 backend commands. New "Rule Packs"
workspace section (placed between Load Cases and Solve), `RulePackManagerPanel`,
and a `rulePackService` frontend route. Covers pack management + document-level
authoring: new private-by-default draft (privacy_class=private_user_data,
redistribution_status=private_only per PRD §12.4), list/open/save/delete against
the local-only store, validate, and compute-and-stamp checksum, with validation
findings and the computed checksum rendered. Expressions are authored in the
document's native declarative-AST JSON (DEC-022); no expression text syntax
ships (D-02b AWAITING_RULING), and the draft template is placeholder/invented
only — no protected equations or standards values (PRD §14.5). Honest browser
seam (RULE-PACK-BACKEND-DESKTOP-ONLY), honest desktop error surfacing
(RULE-PACK-BACKEND-ERROR), and a permanent boundary note. A three-lens
pre-commit review surfaced four findings; three were fixed (unhandled desktop
rejections, stale list on project switch, two vacuous e2e assertions) and one
(mid-request textarea clobber) was recorded as a residual. Validation: Vitest
251/251 (8 new; dead-control audit green), Playwright dev 6/6 (two viewports),
dist 1/1, tsc -b + vite build clean. Residuals routed to the next C2 slices:
structured AST expression composer; required-input/value-slot/load-combination
form builders; in-request busy guard. Engine-side rule evaluation on solved user
models (USER_RULE_CHECKED/USER_RULE_FAILED end-to-end) is C4.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-148; DEL-07-03 run record
`WORKING_ITEMS_RUN_2026-06-12_TP-C2-EDITOR-001.md`. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim is created.

## 2026-06-12 - C2 backend seam landed: rule-pack validation, JCS checksum, local store (`TP-C2-RPLIFE-001`)

The first production wiring of the previously-orphan `core/rules` crates.
New crate `core/rules/rule_pack_document`: a production codec between the
rule-pack document AST encoding and the frozen DEC-022 grammar (all 69
conformance-corpus expressions round-trip; decode errors carry JSON-path
breadcrumbs for editor diagnostics), grammar-version-bound JCS checksum
computation over the document minus its `checksums` member (RFC 8785 via
`core/serialization/canonical_json` + the DEL-06-04 binding constructor;
`payload_excludes` recorded, with a matching additive schema member), and
document-level validation composing shape, grammar, expression-decode,
checksum-match, and `validate_lifecycle` findings into one envelope with a
fixed professional-boundary notice. Desktop seam: six new Tauri commands
(`validate_rule_pack`, `compute_rule_pack_document_checksum`, and
save/open/list/delete local rule-pack persistence) over a v10 store
migration adding the project-scoped `local_rule_packs` table — drafts with
blocking findings remain saveable; user packs stay in local SQLite only,
never committed or transmitted (OPS-K-PRIV-1). The DEL-06-05 example pack
gained a real stamped checksum pinned by a Rust golden test and a Python
JCS recomputation (cross-engine parity witness); the DEL-11-04 toy model
was restamped accordingly. Validation: rule_pack_document 10/10, src-tauri
37/37 (4 new tests; store-migration evidence re-pinned for v10), pytest
359/359, fmt clean. Residuals: editor GUI consumes these commands next
(Playwright/Vitest evidence rides that tranche per H4); check-evaluation
composition and load-case mapping vocabulary stay routed to the C4
lead-up; cross-project library storage stays C3 scope.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-147; DEL-06-04 run record
`WORKING_ITEMS_RUN_2026-06-12_TP-C2-RPLIFE-001.md`. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim is created.

## 2026-06-12 - C2 lead-up landed: D-02b packet + DEC-022 schema absorption (`TP-C2-SCHEMA-001`)

Two C2 lead-up actions per the Phase C stage gate. First, the **D-02b decision
packet** (writable rule-expression text syntax) was prepared and registered
AWAITING_RULING — options O-A..O-D with a PROPOSAL-labeled recommendation
(O-C: defer the writable syntax behind a named usability-evidence trigger;
permit a display-only deterministic AST rendering); no writable text input
ships until the human rules. Second, the named C1 residual closed:
`schemas/rule_pack.schema.yaml` absorbed DEC-022 — required top-level
`grammar_version` (semver, sits inside the JCS-hashed `rule_pack_checksum`
payload), `ExpressionNode` declarative-AST encoding mirroring the conformance
corpus (evaluator refusal markers and literal relaxation flags deliberately
not authorable in pack documents), `UserTableValue` for interpolate/lookup
(D-02 §3 Q6), shared `DimensionId`, `grammar_status:
frozen_open_pipe_stress_declared_expression`, and conditional gates pinning
declarative-AST formulas to the frozen language. The DEL-06-05 example pack
became a real evaluable instance (grammar_version 1.0.0, declarative_ast
divide of the two demo quantities deriving the declared dimensionless ratio;
versions bumped to 0.2.0; expression_grammar/evaluator_library open-decisions
flipped to accepted with DEC-022 pointers), and the DEL-11-04 toy model's
rule-pack ref was restamped (version, raw-bytes sha256, project JCS hash).
Validation: full root pytest 358/0 (extended schema tests: seven new
negative JSON Schema cases). Residuals routed: checksum value stamping +
example-AST Rust cross-check → C2 backend seam tranche; check-evaluation
composition and load-case mapping vocabulary (surfaced TBDs, not resolved) →
C4 lead-up.

Evidence: DEL-06-01 run record
`WORKING_ITEMS_RUN_2026-06-12_TP-C2-SCHEMA-001.md`; decision packet
`execution/_Coordination/_DECISIONS/D-02b_rule_expression_text_syntax.md`.
No lifecycle state, release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - A3 sub-slice landed: workspace IA and usability repair (`TP-APP-R2-UXSHELL-001`)

Response to the human packaged-pass attempt-2 usability verdict (TP-MAC-141).
The shell moved from an accreted vertical panel stack to the PRD §14.1
workspace: persistent model tree | 3D viewport | property inspector core
(DEL-07-02 RQ-001/002), with the remaining surfaces behind always-visible
section tabs in A12-journey order (Operation Apply with live queue badge,
Load Cases, Solve, Results, Report, Project, Exports, Audit & Boundaries;
vocabulary from DEL-07-06 Guidance and the checklist's own terms). Inactive
sections stay mounted (drafts/queue state survive navigation); unimplemented
PRD surfaces remain absent — no placeholder controls; boundary strip stays
permanently visible (DEL-07-05 principle). Scroll/overflow fixed
structurally and browser-verified at 1440×920 and 1280×800 with a DOM
overflow probe. New permanent dead-control audit
(`App.deadControls.test.tsx`): every rendered button must produce an
observable change or carry an accessible disabled reason — caught 4
controls, each fixed or reason-labeled; all data-testids preserved.
Validation: Vitest 242/242, dev e2e 4/4 (two-viewport matrix, navigation
driven through visible controls), dist e2e 1/1, build green. Residuals:
1280×800 dock height cosmetic; WCAG target TBD (DEL-07-06-CF-001); canvas
gestures remain separate A3 scope. Nav-pattern choice recorded as PROPOSAL
(kits silent on concrete pattern).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-146; DEL-07-06 run record
`TASK_RUN_2026-06-12_1756.md`.

## 2026-06-12 - Regression repair: packaged builds ship the wasm engine (`TP-APP-R2-WASMPKG-001`)

The first human TP-MAC-141 packaged pass failed at `New blank` with
`WASM-ENGINE-ASSET-ABSENT` — the F-4 seam doing its job. Root cause: frontend
hashing requires the wasm engine in every mode (H1/F-5a, no fallback), but
the generated assets rode a vite-ignored dynamic import only the dev server
resolves; `dist/` never shipped them and `tauri build` never built them.
Fix: assets emit into the Vite publicDir (`public/wasm-engine/`, atomic
rename preserved), the loader imports a fully-qualified root URL valid in
dev/preview/`tauri://` lanes plus node fs candidates,
`beforeBuildCommand` chains the wasm build (self-sufficient `tauri build`),
a `wasm-engine-dist-guard` vite plugin fails production builds loudly on
absence (demonstrated), and a new production-dist Playwright lane
(`test:e2e:dist`) replays the regression against `vite preview` — run by the
DEC-025 sweep as a second command inside the existing e2e surface (five
surfaces unchanged). Validation: Vitest 241/241, dev e2e 2/2, dist e2e 1/1,
pytest 358/358, build green, rebuilt bundle boot-checked. Honest residual:
the `tauri://` protocol itself is exercised only by the human packaged pass.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-144 and the TP-MAC-141 attempt-1
failure record; DEL-00-08 run record `TASK_RUN_2026-06-12_1355.md`.

## 2026-06-12 - DEC-033 implemented: model document 0.2.0 (`TP-APP-R2-DOCVER-020-001`)

`SUPPORTED_MODEL_SCHEMA_VERSION` 0.1.0 → 0.2.0 on both evaluation surfaces
(src-tauri authority + projectService mirror, pending H2 unification) with
published no-op transform `model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop`.
Blank documents author 0.2.0; 0.1.0 documents migrate in memory with the
migration id in the DEC-019 ledger; browser preview keeps stored snapshot
bytes unchanged for migrated documents. Bundled preview fixtures deliberately
stay 0.1.0-era and now exercise the real migration path; contract corpus
byte-identical. Validation: src-tauri 33/33, applier 58+1+2/0, Vitest
241/241, pytest 358/358, build green. Residuals routed to the H2 row (panel
version-literal display staleness; migrated-bytes hash-integrity edge).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-145; DEL-02-05 run record
`TASK_RUN_2026-06-12_1407.md`.

## 2026-06-12 - A4 sub-slice landed: subtraction/range combination authoring (`TP-APP-R2-COMBEXPR-001`)

Closed combination basis set landed end-to-end mirroring the
`core/loads/load_case_algebra` contract vocabulary: `mechanics` (unchanged),
`result_state_subtraction` (`minuend_id`/`subtrahend_id`), `range_envelope`
(`operand_ids` + `mode` in `min|max|min_abs|max_abs`), carried as strictly
additive optional model fields (schema_version held at 0.1.0; regenerable
mechanics fixture byte-identical). Seam: per-basis create validation with
named blocking codes (`OP-COMBINATION-BASIS-UNSUPPORTED`,
`OP-CREATE-COMBINATION-PAYLOAD-INVALID`, `OP-COMBINATION-RANGE-MODE-UNKNOWN`,
`OP-COMBINATION-OPERAND-DUPLICATE`, `OP-COMBINATION-OPERAND-LOAD-NOT-FOUND`,
`OP-COMBINATION-TERM-BASIS-UNSUPPORTED`); load-case delete counts
subtraction/range references. Solve: deterministic evaluation reusing
`load_case_algebra` with pre-solve named blocks
(`LOAD_COMBINATION_SHAPE_INVALID`, `_RANGE_MODE_UNKNOWN`, `_OPERANDS_EMPTY`)
— no silent zeros or skips. UI: basis selector with conditional per-basis
create fields; the combination basis editor moved from free text to the
closed-set select with cross-shape edits blocked
(`OP-COMBINATION-BASIS-SHAPE-MISMATCH`) — a recorded behavior change (the
prior free-text edit was an implementation choice, not a DEC-recorded
decision). Corpus cases 58–65 blessed and re-run through both engines;
README inventory marks them pending human review (DEC-030 covers cases
01–57 only). Disclosed write-scope deviation ratified at fan-in:
`core/product_physics/src/validation.rs` (the pre-existing basis gate)
required extension beyond the brief's named files. Validation: applier
58+1+2 / 0 (65 corpus cases), product_physics 31/0, load_case_algebra 18/0,
Vitest 239/0, e2e 2/2 (authors a subtraction combination through visible
controls in real Chromium), pytest 358/0, build green, fmt clean.
Open review items: corpus 58–65 human review; DEC-019 whether additive
optional fields warrant a 0.2.0 document-version bump + no-op migration
entry (held additive at 0.1.0).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-143;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/TASK_RUN_2026-06-12_1138.md`.

## 2026-06-12 - A6 sub-slice landed: true directional deformed shape (`TP-APP-R2-DEFORMEDDIR-001`)

`core/product_physics::solve_load_case` now emits signed per-node
displacement component rows for all six DOF (ids
`result:disp:<node>:{ux,uy,uz,rx,ry,rz}`, kinds
`global_nodal_displacement_{x,y,z}` / `global_nodal_rotation_{x,y,z}`, mm/rad,
metadata per the established force-row pattern) after the existing
`displacement_magnitude` rows; existing rows byte-identical; `rad` joined the
combination algebra dimensions so combination rotation rows evaluate instead
of warning. `PipeViewport.buildDeformationOverlay` moves nodes along the true
(ux,uy,uz) unit vector scaled by the retained normalized display offset;
boundary discloses `vector_direction=global_cartesian_displacement_components`
(previously `TBD`), with an honest disclosed magnitude-only fallback when
component rows are absent. Canned browser fixture regenerated through the
documented workflow and independently verified purely additive (647 rows
preserved in order, 90 component rows added); first Playwright e2e asserts
the directional disclosure after solve in real Chromium (H4 default posture,
no exception). Validation: product_physics 28/28, src-tauri 32/32, Vitest
220/220, e2e 2/2, pytest 358/358, desktop build green. Residuals stay in the
A6 row; two convention calls recorded for review in the run record (basis
token `solved_from_global_linear_system` not yet in the results-schema enum;
`rad` combination-dimension mapping).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-142;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-05_Results viewer/_run_records/TASK_RUN_2026-06-12_1110.md`.

## 2026-06-12 - Evidence-sweep git-state hardening (`TP-R2VERIFY-FIX-002`)

Repair of verification finding F-2
([VERIFICATION_2026-06-12_r2_exit_chain.md](VERIFICATION_2026-06-12_r2_exit_chain.md)):
the DEC-025 sweep summary could record a clean-looking git block when the git
capture itself failed, because `_capture()` converts any nonzero git exit into
`None` and the state collector defaulted that to "no dirty paths".
`collect_git_state` now records `status_capture_failed` explicitly,
`working_tree_dirty` becomes `null` (never `false`) when the capture fails,
`summary_filename` marks such summaries `-gitunverified`, and `main --execute`
exits nonzero with a named message because an unbound summary does not satisfy
the DEC-025 commit-binding contract. Summary `schema_version` bumped 1 → 2 for
the new member. Tests: 3 new/updated focused cases; `tests/test_evidence_sweep.py`
16/16 green.

Evidence: `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_run_records/WORKING_ITEMS_RUN_2026-06-12_sweep_git_state_hardening.md`.
No lifecycle state, release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - Authored-load preview-category mapping disclosure (`TP-R2VERIFY-FIX-001`)

Repair of verification finding F-1
([VERIFICATION_2026-06-12_r2_exit_chain.md](VERIFICATION_2026-06-12_r2_exit_chain.md)):
the A12 adapter silently coerced operation-authored load categories
(`concentrated_force`/`concentrated_moment` → `occasional`,
`distributed_force` → `weight`) at the product-physics boundary. The mapping
now surfaces as a named per-load `warning` diagnostic
(`LOAD_CATEGORY_PREVIEW_MAPPED`) stating that the preview classification is
not a user-selected engineering classification; native preview categories do
not emit it; solve status is unchanged. The extended regression pins the
diagnostic code, severity, affected refs, and the native-category negative
case. `core/product_physics` 25/25 green.

Evidence: `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-12_category_mapping_disclosure.md`.
No lifecycle state, release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - R2 exit-chain verification snapshot (`TP-INTEGRATED-VERIFY-002`)

The completion plan's Phase A exit-evidence row called for a derivative
verification snapshot (TP-INTEGRATED-VERIFY successor) for human review; it
now exists:
[VERIFICATION_2026-06-12_r2_exit_chain.md](VERIFICATION_2026-06-12_r2_exit_chain.md).
An independent WORKING_ITEMS session re-ran the R2-chain surfaces (Tauri
backend 32/32 including the A12 rehearsal and saved-project regressions;
product_physics 25/25; units 13/13; Playwright 2/2 including the from-blank
GUI journey), built and booted the packaged desktop binary
(`tauri build --debug --no-bundle`; SMOKE TP-MAC-140), and audited the landed
A9-A12/A8 work against PRD §22.3 verbatim. Verdict: the R2 exit criterion is
demonstrated in substance across two complementary verified surfaces; the
packaged-runtime GUI journey remains the one named gap (finding F-4, recorded
with its macOS automation blockers per the H4 exception). Findings F-1/F-2
were repaired same-session; F-3 routed to the next PKG-17 tranche. The
snapshot feeds the `D-14` stage-advancement decision packet.

Evidence: `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-12_r2_exit_chain_verification.md`;
`apps/desktop/SMOKE.md` TP-MAC-140. No lifecycle state, stage advancement,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim is created.

## 2026-06-12 - B2 export unit-system disclosure (`TP-UNITS-B2-EXPORTDISCLOSURE-001`)

PCF, CAEPIPE MBF, and stress-neutral export packages now carry explicit
DEC-018 unit-system disclosure instead of relying only on target text units or
per-row result units. The desktop export panels show a `Units` row with source
model units, target export units, result units where present, and whether
export-time conversion was performed. Downloaded JSON packages include a
`unit_system_disclosure` member with
`unit-system:dec-018-si-dual-display`, `entered_units_preserved`, model units,
target export units, result units, conversion policy/scope, DEC-018/DEL-02-02
basis refs, and protected/private-content false flags.

The schema-owned export builders now require and checksum the same member:
`core/handoff/pcf_export/package.py`,
`core/handoff/caepipe_mbf/package.py`, and
`core/handoff/stress_neutral/package.py` emit `unit_system_disclosure.json` in
their manifest package-member lists. The strict JSON schemas were widened to
make this member required, and invented fixtures were regenerated through the
deterministic builders.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
`apps/desktop/SMOKE.md` TP-MAC-139. Validation: focused export-package tests
passed 32/32; repository Python tests passed 356/356; full desktop Vitest
passed 216/216; desktop build passed with the pre-existing Vite chunk-size
warning; Playwright R2 smoke passed 2/2 after wasm engine build.

Residual hand-offs: B2 still owns import round-trip unit I/O, target-format
conversion witnesses beyond package disclosure, broader app unit entry/pickers
outside the already covered forms, and rule-pack unit I/O. No lifecycle state,
release-readiness, target compatibility, solver-deck validation, professional
approval, certification, sealing, authentication, protected-content,
private-data, or code-compliance claim is created.

## 2026-06-12 - B2 Property Inspector quantity unit edits (`TP-UNITS-B2-INSPECTOREDITUNITS-001`)

Existing Property Inspector material and pipe-section quantity edits now carry
unit intent instead of changing only `.value`. Material elastic modulus, shear
modulus, thermal expansion coefficient, pipe outside diameter, and pipe wall
thickness expose a `Unit` selector beside the proposed value. Browser preview
continues to show a single model-metadata option when the desktop catalog route
is unavailable; desktop/Tauri mode can use accepted DEC-018 catalog options.
Node coordinate edits remain value-only because project length-unit mutation is
a separate project-unit-system concern.

The Rust/wasm operation seam now treats `{ value, unit }` as a generic
sibling-unit quantity edit payload. Compatible entered units are accepted only
when the same validated edit also writes the sibling `.unit` field, so alternate
units cannot pass as a value-only partial update. Native regressions cover an
existing material modulus edited to `MPa`, an existing pipe outside diameter
edited to `mm`, and incompatible `mm` rejection for a stress quantity.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-02_Pipe section and component library schema/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`apps/desktop/SMOKE.md` TP-MAC-138. Validation:
operation-applier cargo suites passed 54 unit tests plus canonical-hash and
contract-corpus tests; Tauri Rust tests passed 32/32; focused desktop Vitest
passed 165/165; full desktop Vitest passed 216/216; desktop build passed with
the pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2 after
wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside material/section
and primitive-load create/edit forms, imports/exports, and rule-pack unit I/O
remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage outside the witnesses named above. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, protected-content, private-data, or code-compliance claim is
created.

## 2026-06-12 - B2 primitive-load magnitude unit edits (`TP-UNITS-B2-PRIMEDITUNITS-001`)

Existing primitive-load magnitude edits now carry unit intent instead of only
editing `.value`. The Load Cases manager exposes a `Magnitude unit` selector
for the selected primitive load, labels the magnitude field with the active
unit basis, and queues an atomic value+unit payload. Browser preview remains
model-metadata-only when the desktop catalog route is unavailable; desktop mode
can use accepted DEC-018 options.

The Rust/wasm operation seam keeps legacy numeric-string primitive magnitude
edits valid, but also accepts the new `{ value, unit }` payload for
`primitive_loads.N.magnitude.value`. Compatible entered units are preserved by
writing the sibling `.unit` field in the same apply step. A native regression
covers an existing force-per-length primitive edited to `lbf/ft`, and rejects
`mm` for that dimension.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
`apps/desktop/SMOKE.md` TP-MAC-137. Validation:
operation-applier cargo suites passed 53 unit tests plus canonical-hash and
contract-corpus tests; Tauri Rust tests passed 32/32; focused desktop Vitest
passed 165/165; full desktop Vitest passed 216/216; desktop build passed with
the pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2 after
wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside material/section
and primitive-load create/edit forms, imports/exports, and rule-pack unit I/O
remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage outside the witnesses named above. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, protected-content, private-data, or code-compliance claim is
created.

## 2026-06-12 - B2 primitive-load unit selectors (`TP-UNITS-B2-LOADPICKERS-001`)

The Load Cases manager primitive-load creation form now has visible unit
selection for the entered magnitude. Browser preview remains honest with a
single model-metadata option; desktop/Tauri mode loads accepted DEC-018 unit
catalog entries for the selected primitive-load dimension and labels
canonical/display options explicitly. Category and direction changes reset the
selector to the model's default unit for that load family.

The structured operation seam now accepts DEC-018-compatible entered units for
created primitive loads while preserving the entered unit in the applied
session model. Native regressions cover a concentrated force entered in `lbf`,
a pressure load entered in `kPa`, and incompatible `mm` rejection for a force
load. Existing primitive-load magnitude edits remain out of scope because
changing a numeric `.value` without a sibling `.unit` update path would create
ambiguous partial edits.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
`apps/desktop/SMOKE.md` TP-MAC-136. Validation:
operation-applier cargo suites passed 52 unit tests plus canonical-hash and
contract-corpus tests; Tauri Rust tests passed 32/32; focused desktop Vitest
passed 165/165; full desktop Vitest passed 216/216; desktop build passed with
the pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2
after wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside material/section
and primitive-load create forms, imports/exports, rule-pack unit I/O, and
existing primitive-load magnitude-edit unit handling remain B2 work. B3 still
owns broader mixed-unit round-trip, conversion-witness, incompatible-unit
rejection, and D-04/DEC-026 tolerance coverage outside the witnesses named
above. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2 rendered report body unit disclosure (`TP-UNITS-B2-REPORTBODY-001`)

The hash-bound rendered report body now carries explicit unit evidence instead
of relying only on the Report Packet JSON disclosure. The report-generator
model-input schema has a backward-compatible optional `unit_display_summary`
with entered-unit storage convention, sorted model unit map, distinct result
units, report display policy, and report-time conversion flag. The React
report adapter populates that summary from the current session model and
result rows, and the Rust renderer displays it in the `Model Input Summary`
section.

The shared invented report fixture and Tauri report-command tests now prove
the rendered HTML includes `Unit storage convention`, `Model units`, `Result
units`, and `Report-time conversion`. The A12 from-blank rehearsal helper
overrides the fixture summary with the actual authored model units and solved
result units, so the full author -> solve -> render regression also carries
this B2 evidence.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_report_body_unit_disclosure.md`;
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_run_records/WORKING_ITEMS_RUN_2026-06-12_report_body_unit_disclosure.md`;
`apps/desktop/SMOKE.md` TP-MAC-135. Validation: report-generator schema
contract test passed; report-generator cargo tests passed 10/10;
report-renderer cargo tests passed 8/8; Tauri Rust tests passed 32/32;
focused report/App Vitest passed 53/53; full desktop Vitest passed 216/216;
desktop build passed with the pre-existing Vite chunk-size warning;
Playwright R2 smoke passed 2/2 after wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside material/section
create forms, imports/exports, and rule-pack unit I/O remain B2 work. B3
still owns broader mixed-unit round-trip, conversion-witness,
incompatible-unit rejection, and D-04/DEC-026 tolerance coverage outside the
witnesses already landed. No lifecycle state, release-readiness,
professional approval, certification, sealing, authentication,
protected-content, private-data, or code-compliance claim is created.

## 2026-06-12 - B2 Property Inspector unit selectors (`TP-UNITS-B2-UNITPICKERS-001`)

The Property Inspector material and pipe-section creation forms now have the
first B2 visible unit entry controls. Section creation has a `Length unit`
selector, material creation has `Modulus unit` and `Thermal expansion unit`
selectors, and the existing field labels/status panel follow the selected
unit basis. Browser preview remains honest with one-option selectors from
model metadata and no synthesized fallback catalog; desktop/Tauri mode draws
accepted options from the DEC-018 catalog route.

The structured operation seam now accepts DEC-018-compatible entered units for
create-section length quantities and create-material stress /
thermal-expansion quantities, preserving entered units in the applied session
model. Native Rust regressions cover `MPa` material moduli, `1/K` thermal
expansion, `mm` section geometry, and incompatible material-unit rejection.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
`apps/desktop/SMOKE.md` TP-MAC-134. Validation:
operation-applier cargo suites passed 51 unit tests plus canonical-hash and
contract-corpus tests; Tauri Rust tests passed 32/32; focused desktop Vitest
passed 165/165; full desktop Vitest passed 216/216; desktop build passed with
the pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2
after wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside these material and
section create forms, report renderer body expansion beyond packet
disclosure, imports/exports, and rule-pack unit I/O remain B2 work. B3 still
owns broader mixed-unit round-trip, conversion-witness, incompatible-unit
rejection, and D-04/DEC-026 tolerance coverage outside the witnesses named
above. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2 solver-boundary unit normalization (`TP-UNITS-B2-SOLVERNORM-001`)

The preview mechanics adapter now uses the accepted DEC-018 unit catalog at
the solver boundary. `core/product_physics` depends on `core/units`, validates
unit-bearing inputs by dimension compatibility instead of exact canonical
strings, and normalizes compatible quantities to SI-canonical values before
building the frame model, supports, sections, thermal loads, pressure thrust,
and primitive loads.

The product-physics regression suite now includes a mixed-unit equivalence
witness: material moduli entered in `MPa`, pipe dimensions in `mm`, and
pressure loads in `kPa` solve to the same rounded result surface as the
original SI fixture. Incompatible material and load units still block with
`UNIT_INPUT_INVALID`; unexpected conversion failures surface as
`UNIT_CONVERSION_UNAVAILABLE`.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_solver_boundary_unit_normalization.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-12_solver_boundary_unit_normalization.md`;
`apps/desktop/SMOKE.md` TP-MAC-133. Validation: product_physics cargo tests
passed 25/25; Tauri Rust tests passed 32/32; headless runner cargo tests
passed 11/11; focused desktop Vitest passed 56/56; Playwright R2 smoke passed
2/2 after wasm engine build; full desktop Vitest passed 216/216; desktop build
passed with the pre-existing Vite chunk-size warning.

Residual hand-offs: broader visible unit entry/pickers, report renderer body
expansion beyond packet disclosure, imports/exports, and rule-pack unit I/O
remain B2 work. B3 still owns the broader conversion-witness and
D-04/DEC-026 tolerance corpus beyond this product-physics boundary witness.
No lifecycle state, release-readiness, professional approval, certification,
sealing, authentication, protected-content, private-data, or code-compliance
claim is created.

## 2026-06-12 - B2 report unit-system disclosure (`TP-UNITS-B2-REPORTUNITS-001`)

The Report Packet panel and JSON export now disclose the unit system used by
the current model/result packet. The visible report line names
`unit-system:dec-018-si-dual-display`, lists model units, lists distinct
result-row units, and records `conversion=false`. The exported report JSON
adds `unit_system_disclosure` with the model unit map, result-unit inventory,
entered-unit preservation posture, and no report-time conversion claim.

The rendered-report adapter also replaces the old placeholder
`preview-display-label-set` with `unit-system:dec-018-si-dual-display` in the
existing strict `model_input_summary.unit_system_ref` field. The
report-generator schema was not widened.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_report_unit_system_disclosure.md`;
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_run_records/WORKING_ITEMS_RUN_2026-06-12_report_unit_system_disclosure.md`;
`apps/desktop/SMOKE.md` TP-MAC-132. Validation: focused report/App Vitest
passed 53/53; Playwright R2 smoke passed 2/2 after wasm engine build; full
desktop Vitest passed 216/216; desktop build passed with the pre-existing
Vite chunk-size warning.

Residual hand-offs: broader visible unit entry/pickers, solver-boundary
normalization, report renderer body expansion beyond packet disclosure,
imports/exports, and rule-pack unit I/O remain B2 work. B3 still owns broader
mixed-unit round-trip, conversion-witness, incompatible-unit rejection, and
D-04/DEC-026 tolerance coverage. No lifecycle state, release-readiness,
professional approval, certification, sealing, authentication,
protected-content, private-data, or code-compliance claim is created.

## 2026-06-12 - B2 Property Inspector unit basis labels (`TP-UNITS-B2-INSPECTORLABELS-001`)

The desktop Property Inspector now has the first visible B2 unit display
retrofit. `unitCatalogService.ts` exposes catalog lookup/display helpers that
match DEC-018 entries by symbol and dimension, including display-only
equivalents needed by current fields such as `stress` through pressure units.
The helpers do not normalize values; browser-preview unavailability and
catalog misses are explicit states.

The Property Inspector loads the unit-catalog route once, renders a compact
`Unit basis` panel, and annotates material and pipe-section creation labels
with their unit source. Browser preview shows `m, model metadata`, `Pa, model
metadata`, and `1/degC, model metadata`; desktop/Tauri mode can display the
DEC-018 catalog basis from the `get_unit_catalog` command.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_basis_labels.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_basis_labels.md`;
`apps/desktop/SMOKE.md` TP-MAC-131. Validation: focused
unit-catalog/App Vitest passed 48/48; Playwright R2 smoke passed 2/2 after
wasm engine build; full desktop Vitest passed 216/216; desktop build passed
with the pre-existing Vite chunk-size warning.

Residual hand-offs: broader visible unit entry/pickers, solver-boundary
normalization, report unit-system disclosures, imports/exports, and rule-pack
unit I/O remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2 frontend unit catalog service (`TP-UNITS-B2-FRONTENDSVC-001`)

The desktop frontend now has a typed service route to the reviewed unit
catalog. `apps/desktop/src/services/unitCatalogService.ts` defines the catalog
payload shape, `loadUnitCatalog`, and `acceptedUnits`. In desktop/Tauri mode
the service invokes `get_unit_catalog`; in browser preview mode it returns an
explicit `UNIT-CATALOG-DESKTOP-ONLY` unavailable route instead of inventing a
fallback catalog.

Vitest coverage pins both routes. The Tauri-route test verifies DEC-018
metadata, entered-unit preservation, factor/offset text, conventional-public
provenance, and boundary flags. This prepares visible unit-picker/display work
without changing authoring forms in this tranche.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_frontend_unit_catalog_service.md`;
`apps/desktop/SMOKE.md` TP-MAC-130. Validation:
`npm test --workspace apps/desktop -- unitCatalogService` passed with 2
tests; `npm test --workspace apps/desktop` passed with 215 tests across 9
files; `npm run build --workspace apps/desktop` passed with the pre-existing
Vite chunk-size warning.

Residual hand-offs: visible app unit pickers/displays, solver-boundary
normalization, report unit-system disclosures, imports/exports, and rule-pack
unit I/O remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2 desktop unit catalog command (`TP-UNITS-B2-CATALOGCMD-001`)

The desktop backend now has a tested B2 binding to the DEC-018 units crate.
`openpipestress-desktop` depends on `open_pipe_stress_units` and exposes a
`get_unit_catalog` Tauri command. The command returns schema-facing catalog
records with stable `unit:*` ids, symbols, dimension ids, canonical flags,
transform kinds, factor representation, optional offset representation,
provenance, review status, and boundary flags. This gives the app a durable
source for future unit picker/display controls instead of copying unit labels
into each authoring surface.

The units crate now also exposes stable catalog IDs and string values for
transform kind, conversion provenance, and review status. Focused Tauri
coverage pins the important B2 metadata: `DEC-018`, entered-unit preservation,
the inch factor text, Fahrenheit offset text, lbf conventional-public-constant
provenance, project-governed semantic binding, and no protected/private data
or professional/code-compliance claim.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_desktop_unit_catalog_binding.md`;
`apps/desktop/SMOKE.md` TP-MAC-129. Validation:
`cargo fmt --manifest-path core/units/Cargo.toml --check` passed;
`cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`
passed; `cargo test --manifest-path core/units/Cargo.toml` passed with 13
unit tests and 0 doctests; focused Tauri command test passed 1/1; full
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 32
unit tests and 0 doctests; `python3 tests/test_units_schema.py` passed.

Residual hand-offs: visible app unit pickers/displays, solver-boundary
normalization, report unit-system disclosures, imports/exports, and rule-pack
unit I/O remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2/B3 units schema-crate contract rider (`TP-UNITS-B2B3-CONTRACT-001`)

The units lane now has an executable schema/crate contract for the two
2026-06-12 findings routed after B1. `core/units` exposes
`factor_representation`, optional `offset_representation`, explicit
`ConversionProvenance`, and `ReviewStatus` on each `UnitDefinition`, so B2
schema/app/report bindings have a crate-side place to preserve per-constant
derivation text and review state. Provenance is no longer derived as a
SI-canonical vs display-unit binary: lbf/psi-family conversions are marked as
`ConventionalPublicConstant`, and project-owned semantic canonical bindings
are marked as `ProjectGovernedDecision`.

B3 also gained the requested executable drift guard. A new Rust regression
parses `schemas/units.schema.yaml` and asserts set equality between the schema
`DimensionId` enum and the crate `DIMENSIONS` vocabulary, preventing the
current hand-checked identifier match from silently drifting.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_units_schema_crate_contract.md`.
Validation: `cargo fmt --manifest-path core/units/Cargo.toml --check`
passed; `cargo test --manifest-path core/units/Cargo.toml` passed with 13
unit tests and 0 doctests; `python3 tests/test_units_schema.py` passed;
`python3 -m pytest tests/test_units_schema.py` passed with 3 tests.

Residual hand-offs: this is not full B2 closure. B2 still must bind unit-aware
I/O through schemas, desktop fields, solver-boundary normalization, reports,
imports, exports, and rule-pack evaluation. B3 still must add broader
mixed-unit round-trip, conversion-witness, incompatible-unit rejection, and
D-04/DEC-026 tolerance corpus coverage. No lifecycle state, release-readiness,
professional approval, certification, sealing, authentication,
protected-content, private-data, or code-compliance claim is created.

## 2026-06-12 - A8 saved-project backend smoke (`TP-APP-R2-SAVEDPROJECT-SMOKE-001`)

The A8 saved-project residual now has a Tauri/backend regression. The new
test `r2_from_blank_saved_project_opens_solves_and_renders_report` reuses the
A12 rehearsal helpers, applies the from-blank script, saves the authored model
into the desktop local SQLite project store, reopens it by project id, solves
the loaded model through `run_preview_mechanics`, and renders through the A7
`render_calculation_report` command. The test verifies the store did not
silently persist mechanics or analysis-run payloads before solve, then records
the solved run/report path after reopening.

Evidence:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-12_saved_project_backend_smoke.md`;
`apps/desktop/SMOKE.md` TP-MAC-128. Validation: focused Tauri regression 1/1;
full Tauri Rust suite 31/31.

Residual hand-off: A8 still has broader SMOKE checklist parity if the project
wants every manual smoke row represented by automated assertions. No lifecycle
state, release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim is created.

## 2026-06-12 - A8 from-blank GUI journey automation (`TP-APP-R2-FROMBLANK-E2E-001`)

The A8 Playwright harness now automates the A12 from-blank rehearsal script
through visible browser GUI controls. The new e2e test reads
`fixtures/product_preview/r2_from_blank_rehearsal.json`, clicks `New blank`,
fills the viewport/property-inspector/load-manager forms for the two nodes,
material, standalone section, pipe run, support, load case, primitive load,
and mechanics combination, and applies all nine operations through the Apply
Operations panel. Each applied receipt is asserted as
`route=local_wasm_engine` with `professional_approval=false`.

The test also records the current browser harness boundary. Browser mode does
not publish solved rows for an edited model; it emits
`BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`. Browser mode also keeps the
hash-bound report renderer desktop-only and emits
`REPORT-RENDERER-DESKTOP-ONLY` instead of using a fallback renderer. The true
backend create -> solve -> render proof remains covered by the A12 Tauri
regression; A8 still needs the packaged Tauri saved-project smoke to bind the
GUI-authored project path to the backend desktop command seams.

Evidence:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-12_from_blank_gui_journey.md`;
`apps/desktop/SMOKE.md` TP-MAC-127. Validation: Playwright e2e 2/2 after
wasm engine build; desktop Vitest 213/213; desktop production build green
with the pre-existing Vite chunk-size warning.

Residual hand-offs: packaged Tauri/backend saved-project solve + render smoke
and broader SMOKE checklist parity remain in A8. Phase B2/B3 riders from the
2026-06-12 human findings are routed in the completion plan: preserve
per-constant unit derivation/provenance text, add executable schema-to-crate
unit vocabulary parity, and keep future process records in deliverable-local
evidence folders.

## 2026-06-12 - A12 from-blank R2 exit rehearsal (`TP-APP-R2-FROMBLANK-REHEARSAL-001`)

A12 is landed as a backend-backed rehearsal script and regression for the PRD
R2 exit chain. The new invented fixture
`fixtures/product_preview/r2_from_blank_rehearsal.json` starts from a blank
local model and lists structured authoring steps for two nodes, one material,
one standalone section, one straight pipe run, one anchor support, one load
case, one primitive load, and one mechanics combination. The Tauri regression
`r2_from_blank_rehearsal_authors_solves_and_renders_report` consumes that
fixture, converts each step to a structured operation envelope, applies the
steps through `apply_model_operation`, solves the authored model through
`run_preview_mechanics`, and renders the deterministic A7 HTML report through
`render_calculation_report`.

The rehearsal exposed and closed a real A4-to-A5 integration gap:
operation-authored primitive-load categories were stored as
`concentrated_force`, `concentrated_moment`, and `distributed_force`, while the
product-physics adapter only accepted the older preview labels. The adapter
now maps concentrated force/moment to the existing equivalent-static
`Occasional` mechanics category and distributed force to `Weight`, with direct
product-physics regression coverage.

Evidence:
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-12_from_blank_rehearsal.md`;
`apps/desktop/SMOKE.md` TP-MAC-126. Validation: rehearsal fixture JSON parses;
product_physics cargo tests 24/24; Tauri Rust tests 30/30; desktop Vitest
213/213; desktop production build green; Playwright R2 smoke 1/1 after wasm
engine build.

Residual hand-off: A8 must automate this A12 script as the GUI/e2e journey
evidence backbone. Browser fixture mode still honestly refuses solved rows for
edited models without the Tauri backend, so A12 does not claim browser-mode
edited-model solving. No lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claim is
created.

## 2026-06-12 - A11 final sub-slice: node deletion authoring (`TP-APP-R2-DELNODE-001`)

The structured operation seam now accepts `delete_node` as a delete-kind
operation. `core/model_operations/operation_applier` validates whole-node
deletion intents with no direct mutation, requires `object_type=Node`,
`field_path=nodes`, `after=not_present`, current node label/x/y/z
before-state display, unit `none`, and dimension `dimensionless`. Apply
removes exactly one unreferenced node and returns a new model document.
Deletion is refused while any pipe endpoint, support, component symbol, or
primitive nodal load still references the node, with
`OP-NODE-DELETE-REFERENCED`; there is no hidden cascade into dependent model
entities. The cross-engine contract corpus added
`case_56_accept_delete_node.json` and
`case_57_block_delete_node_referenced.json`; both native Rust and browser
wasm lanes require accepted `delete_node` and referenced-node blocking
coverage.

The desktop Property Inspector now exposes a `Queue delete node` action for
the selected node. The action queues a review-only structured operation with
the selected node summary as the before-state guard. Applying the queued
operation uses the existing Apply Operations panel, records the same
local-session acceptance receipt, clears stale solve/report state, removes
the node row, and falls back to the project row after the deleted selection
disappears.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_node_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_node_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_node_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-125. Validation: operation_applier cargo
suites 49 unit + canonical hash + 57-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 117/117; focused App Vitest
2/2; full desktop Vitest 213/213; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1 with node delete preview coverage; local
in-app browser create-then-delete node smoke at `http://127.0.0.1:5173/` with
zero console errors.

A11 is complete: support deletion, primitive-load deletion, full-combination
deletion, load-case deletion, pipe-run deletion, node deletion, and the
previously landed combination-term deletion all route through the structured
operation seam with accepted/refusal corpus coverage. Residual hand-offs:
A12 must rehearse the full from-blank create -> solve -> report path, and the
A8 journey automation should become the R2 exit-evidence backbone immediately
after that rehearsal. No lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claim is
created.

## 2026-06-12 - A11 fifth sub-slice: pipe-run deletion authoring (`TP-APP-R2-DELPIPE-001`)

The structured operation seam now accepts `delete_pipe_run` as a delete-kind
operation. `core/model_operations/operation_applier` validates whole pipe-run
deletion intents with no direct mutation, requires `object_type=Element`,
`field_path=pipe_segments`, `after=not_present`, current pipe
label/from/to/material before-state display, unit `none`, and dimension
`dimensionless`. Apply removes exactly one unreferenced pipe segment and
returns a new model document. Deletion is refused while any primitive load
still references the pipe, with `OP-PIPE-DELETE-REFERENCED`; there is no
hidden primitive-load cascade. The cross-engine contract corpus added
`case_54_accept_delete_pipe_run.json` and
`case_55_block_delete_pipe_run_referenced.json`; both native Rust and browser
wasm lanes require accepted `delete_pipe_run` and referenced-pipe blocking
coverage.

The desktop Property Inspector now exposes a `Queue delete pipe` action for
the selected pipe. The action queues a review-only structured operation with
the selected pipe summary as the before-state guard. Applying the queued
operation uses the existing Apply Operations panel, records the same
local-session acceptance receipt, clears stale solve/report state, removes
the pipe row, and falls back to the project row after the deleted selection
disappears.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_pipe_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_pipe_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_pipe_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-124. Validation: operation_applier cargo
suites 47 unit + canonical hash + 55-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 113/113; focused App Vitest
2/2; full desktop Vitest 207/207; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1 with pipe delete preview coverage; local
in-app browser pipe-delete smoke at `http://127.0.0.1:5173/` with zero
console errors.

Residual hand-offs: A11 remains open for node entity deletion only. A12 must
still rehearse the full from-blank create -> solve -> report path, and the A8
journey automation should become the R2 exit-evidence backbone immediately
after that rehearsal. No lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claim is
created.

## 2026-06-12 - B1 unit catalog and conversion crate (`TP-UNITS-B1-CATALOG-001`)

Phase B1 is now landed crate-side. `core/units` has its own Rust crate,
`open_pipe_stress_units`, with a closed DEC-018 catalog surface: canonical
dimension identifiers and exponent-vector algebra, SI-canonical units with
common SI/US display units, exact public definitional conversion constants,
finite-value checks, incompatible-dimension rejection, affine absolute
temperature conversion, separate interval-temperature conversion, and explicit
gauge/absolute pressure conversion that requires a caller-supplied pressure
reference with provenance whenever the pressure kind changes.

During fan-in, the generic quantity conversion API was tightened so
temperature and pressure cannot silently bypass their special semantics through
`QuantityKind::UnitBearing`; those dimensions now require explicit
absolute/interval or absolute/relative quantity semantics. The unit README was
also updated from its pre-DEC-018 posture so it records the accepted B1 basis
and preserves the B2/B3 handoffs instead of listing ruled items as still open.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_unit_catalog_conversion_crate.md`;
`core/units/_run_records/TASK_RUN_2026-06-12_0136.md`. Validation:
`cargo fmt --manifest-path core/units/Cargo.toml --check` passed;
`cargo test --manifest-path core/units/Cargo.toml` passed with 11 unit tests
and 0 doctests; `python3 tests/test_units_schema.py` passed;
`python3 -m pytest tests/test_units_schema.py` passed with 3 tests.

Residual hand-offs: B2 must wire this crate into schema field bindings,
desktop unit entry/display, solver-boundary normalization, reports, imports,
exports, and rule-pack evaluation; B3 must add mixed-unit round-trip tests,
conversion witnesses, rejection coverage, and D-04/DEC-026 tolerance evidence.
Angle/rotation behavior beyond cataloged `rad`/`deg` conversion remains a
future bounded decision/implementation topic. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, protected-content, private-data, or code-compliance claim is
created.

## 2026-06-12 - A11 fourth sub-slice: load-case deletion authoring (`TP-APP-R2-DELLOADCASE-001`)

The structured operation seam now accepts `delete_load_case` as a delete-kind
operation. `core/model_operations/operation_applier` validates whole-load-case
deletion intents with no direct mutation, requires `object_type=Load`,
`field_path=load_cases`, `after=not_present`, current load-case
id/label/kind/status/primitive-count before-state display, unit `none`, and
dimension `dimensionless`. Apply removes exactly one unreferenced load case
and returns a new model document. Deletion is refused while any combination
term still references the load case, with
`OP-LOAD-CASE-DELETE-REFERENCED`; there is no hidden combination-term or
primitive-load cascade. The cross-engine contract corpus added
`case_52_accept_delete_load_case.json` and
`case_53_block_delete_load_case_referenced.json`; both native Rust and
browser wasm lanes require accepted `delete_load_case` and referenced-load
blocking coverage.

The desktop Load Cases manager now exposes a `Queue delete case` action for
the selected load case. The action queues a review-only structured operation
with the selected load-case summary as the before-state guard. Applying the
queued operation uses the existing Apply Operations panel, records the same
local-session acceptance receipt, clears stale solve/report state, and updates
the load-case list/counts without deleting combinations through a hidden
cascade.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_load_case_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_load_case_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_load_case_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-123. Validation: operation_applier cargo
suites 45 unit + canonical hash + 53-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 109/109; focused App Vitest
2/2; full desktop Vitest 201/201; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1 with load-case delete preview coverage;
local in-app browser create-then-delete load-case smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 remains open for node and pipe-run entity deletion.
A12 must still rehearse the full from-blank create -> solve -> report path,
and the A8 journey automation should become the R2 exit-evidence backbone
immediately after that rehearsal. No lifecycle state, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim is created.

## 2026-06-12 - A11 third sub-slice: full-combination deletion authoring (`TP-APP-R2-DELCOMBINATION-001`)

The structured operation seam now accepts `delete_combination` as a
delete-kind operation. `core/model_operations/operation_applier` validates
whole-combination deletion intents with no direct mutation, requires
`field_path=combinations`, `after=not_present`, current combination
id/label/basis/terms before-state display, unit `none`, and dimension
`dimensionless`. Apply removes exactly one combination and returns a new model
document. The cross-engine contract corpus added
`case_51_accept_delete_combination.json`; both native Rust and browser wasm
lanes require accepted `delete_combination` coverage.

The desktop Load Cases manager now exposes a `Queue delete combo` action for
the selected combination. The action queues a review-only structured
operation with the selected combination summary as the before-state guard.
Applying the queued operation uses the existing Apply Operations panel,
records the same local-session acceptance receipt, clears stale solve/report
state, and updates the combination list/counts without deleting load cases,
primitive loads, or combination terms through a hidden cascade.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_combination_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_combination_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_combination_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-122. Validation: operation_applier cargo
suites 44 unit + canonical hash + 51-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 105/105; focused App Vitest
1/1; full desktop Vitest 195/195; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1 with whole-combination delete preview
coverage; local in-app browser full-combination delete smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 remains open for node, pipe run, and load case entity
deletion; `delete_combination_term` remains the existing term-level deletion
operation. A12 must still rehearse the full from-blank create -> solve ->
report path. No lifecycle state, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - A11 second sub-slice: primitive-load deletion authoring (`TP-APP-R2-DELPRIMLOAD-001`)

The structured operation seam now accepts `delete_primitive_load` as a
delete-kind operation. `core/model_operations/operation_applier` validates
indexed primitive-load deletion intents with no direct mutation, requires
`field_path=primitive_loads.N`, `after=not_present`, current primitive
before-state display, and matching primitive unit/dimension metadata. Apply
removes exactly one primitive load from the owning load case and returns a
new model document. The cross-engine contract corpus added
`case_50_accept_delete_primitive_load.json`; both native Rust and browser wasm
lanes require accepted `delete_primitive_load` coverage.

The desktop Load Cases manager now exposes a `Queue delete` action for the
selected primitive load. The action queues a review-only structured operation
with the selected primitive id/category/target/direction/magnitude as the
before-state guard. Applying the queued operation uses the existing Apply
Operations panel, records the same local-session acceptance receipt, clears
stale solve/report state, and updates the primitive-load list/counts without
inventing any replacement load.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_primitive_load_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_primitive_load_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_primitive_load_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-121. Validation: operation_applier cargo
suites 43 unit + canonical hash + 50-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 103/103; focused App Vitest
1/1; full desktop Vitest 192/192; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1; local Chrome primitive-load delete
smoke at `http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 remains open for node, pipe run, load case, and full
combination entity deletion; `delete_combination_term` remains the existing
term-level deletion operation. A12 must still rehearse the full from-blank
create -> solve -> report path. No lifecycle state, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim is created.

## 2026-06-12 - A11 first sub-slice: support deletion authoring (`TP-APP-R2-DELSUPPORT-001`)

The structured operation seam now accepts `delete_support` as a delete-kind
operation. `core/model_operations/operation_applier` validates explicit
support deletion intents with no direct mutation, requires `after=not_present`,
unit `none`, dimension `dimensionless`, current support label before-state
matching, and reference-integrity blocking before any model document is
returned. Deletion is refused with `OP-SUPPORT-DELETE-REFERENCED` when an
imposed-displacement primitive load still targets the support. The
cross-engine contract corpus added `case_48_accept_delete_support.json` and
`case_49_block_delete_support_referenced.json`; both native Rust and browser
wasm lanes require accepted `delete_support` coverage and the referenced
support blocking code.

The desktop Property Inspector now exposes a `Delete support` action only
when a support row is selected. The action queues a review-only structured
operation, applies through the existing Apply Operations panel, records the
same local-session acceptance receipt, clears stale solve/report state, and
falls back to the project row after the deleted support disappears. A second
app regression creates an imposed-displacement primitive load and verifies
that deleting its target support remains blocked with diagnostics and no
model mutation.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_support_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_support_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_support_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-120. Validation: operation_applier cargo
suites 42 unit + canonical hash + 49-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 101/101; focused App Vitest
2/2; full desktop Vitest 189/189; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1; local Chrome support-delete smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 remains open for node, pipe run, load case, primitive
load, and full combination entity deletion; `delete_combination_term` remains
the existing term-level deletion operation. A12 must still rehearse the full
from-blank create -> solve -> report path. No lifecycle state, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claim is created.

## 2026-06-12 - A10 third sub-slice: section creation authoring (`TP-APP-R2-CREATESECTION-001`)

The structured operation seam now accepts `create_section` as a create-kind
operation. `core/model_operations/operation_applier` validates explicit pipe
section payloads with no direct mutation, no engineering defaults, no unit
conversion, duplicate-id blocking, project length-unit alignment for outside
diameter and wall thickness, and a physical geometry guard that requires wall
thickness to be less than the outside-diameter radius. The cross-engine
contract corpus added `case_47_accept_create_section.json`; both native Rust
and browser wasm lanes require `create_section` in the accepted-operation
coverage floor.

The desktop model tree and Property Inspector now expose a local preview
`sections` collection and a compact section creation form: section id, name,
type, outside diameter, wall thickness, provenance, and a `Queue section`
action that emits a review-only structured operation. Applying the queued
intent uses the same operation-apply panel, records the local-session
acceptance receipt, selects the newly created section in the model tree, and
clears stale solve/report state like other applied model edits.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_section_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_section_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_section_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-119. Validation: operation_applier cargo
suites 40 unit + canonical hash + 47-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 97/97; focused App Vitest 1/1;
blank-project service Vitest 5/5; full desktop Vitest 183/183; desktop
production build green; Tauri Rust tests 29/29; Playwright R2 smoke 1/1;
local Chrome smoke at `http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 deletion coverage remains open, and A12 must still
rehearse the full from-blank create -> solve -> report path. Optional later
pipe-form reuse of standalone section refs is not claimed by this tranche.
No lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - A10 second sub-slice: material creation authoring (`TP-APP-R2-CREATEMATERIAL-001`)

The structured operation seam now accepts `create_material` as a create-kind
operation. `core/model_operations/operation_applier` validates explicit
material payloads with no direct mutation, no engineering defaults, no unit
conversion, duplicate-id blocking, project pressure-unit alignment for
elastic/shear quantities, and optional thermal-expansion quantity validation
against the project temperature unit. The cross-engine contract corpus added
`case_46_accept_create_material.json`; both native Rust and browser wasm
lanes require `create_material` in the accepted-operation coverage floor.

The desktop Property Inspector now exposes a compact material creation form:
material id, label, elastic modulus, shear modulus, optional thermal
expansion, provenance, and a `Queue material` action that emits a review-only
structured operation. Applying the queued intent uses the same operation-apply
panel, records the local-session acceptance receipt, selects the newly created
material in the model tree, and clears stale solve/report state like other
applied model edits.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_material_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_material_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_material_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-118. Validation: operation_applier cargo
suites 38 unit + canonical hash + 46-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 95/95; focused App Vitest 1/1;
full desktop Vitest 180/180; desktop production build green; Tauri Rust tests
29/29; Playwright R2 smoke 1/1; local Chrome smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A10 still needs section creation before pipe creation can
be fully fixture-independent; A11 deletion coverage is still open. No
lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 — A10 first sub-slice: support creation authoring (`TP-APP-R2-CREATESUPPORT-001`)

The structured operation seam now accepts `create_support` as a create-kind
operation. `core/model_operations/operation_applier` validates explicit
support payloads with no direct mutation, no engineering defaults, no unit
conversion, duplicate-id blocking, node-reference blocking, and the existing
UX/UY/UZ/RX/RY/RZ restraint-token vocabulary. The cross-engine contract
corpus added `case_45_accept_create_support.json`; both native Rust and
browser wasm lanes require `create_support` in the accepted-operation coverage
floor.

The desktop Property Inspector now exposes a compact support creation form:
support id, label, existing node selector, restraint checkboxes, provenance,
and a `Queue support` action that emits a review-only structured operation.
Applying the queued intent uses the same operation-apply panel, records the
local-session acceptance receipt, selects the newly created support in the
model tree, and clears stale solve/report state like other applied model
edits.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_support_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_support_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_support_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-117. Validation: operation_applier cargo
suites 36 unit + canonical hash + 45-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 93/93; focused App Vitest 1/1;
full desktop Vitest 177/177; desktop production build green; Tauri Rust tests
29/29; Playwright R2 smoke 1/1; local Chrome smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A10 still needs material creation and section creation
before pipe creation can be fully fixture-independent; A11 deletion coverage
is still open. No lifecycle state, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claim is created.

## 2026-06-12 — A9 landed: blank local model authoring target (`TP-APP-R2-BLANK-001`)

The desktop app now has an explicit `New blank` local project action for the
R2 from-scratch path. The action builds a user-created blank model document
(`project:blank-local-*`) with no fixture nodes, pipe segments, supports,
materials, load cases, combinations, or mechanics results; persists it through
the existing browser-memory/Tauri local project boundary; makes it the active
authoring target; resets queued operations, undo/redo, proposals, result
state, and analysis-run state; and records `create_blank` in the storage and
validation evidence panels. The blank document carries visible
`MODEL_INCOMPLETE`, `RULE_INPUTS_INCOMPLETE`, and `NOT_PROVIDED` status plus
a blocking `BLANK_PROJECT_AUTHORING_TARGET` diagnostic, so no hidden fixture
entities or engineering defaults are inserted.

Backend evidence pins the Tauri solve path: running `run_preview_mechanics`
on a blank supplied payload returns `MODEL_INCOMPLETE`, zero result rows, and
the explicit missing-input diagnostics `NODE_INPUT_MISSING`,
`PIPE_INPUT_MISSING`, and `LOAD_INPUT_MISSING`. Browser preview mode preserves
the existing edited/user-created-model boundary: a blank solve attempt
completes as a zero-row `MODEL_INCOMPLETE` envelope with
`BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, and the report packet shows
0 selected result refs instead of reusing solved fixture rows.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-12_blank_project_authoring_path.md`;
`apps/desktop/SMOKE.md` TP-MAC-116. Validation: projectService focused
Vitest 5/5; App focused Vitest 32/32; full desktop Vitest 174/174; Tauri Rust
tests 29/29; production build green; Playwright R2 smoke 1/1; live in-app
browser smoke at `http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A10 must add support/material/section creation operation
kinds and UI forms before a blank model can become solvable from scratch;
A11 must add deletion coverage; packaged-Tauri saved-project blank smoke
remains as an A8/A5 evidence expansion. No lifecycle state, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim is created.

## 2026-06-11 — H5: RFC 8785 canonical number rendering in `canonical_json` (`TP-H5-JCSRENDER-001`)

The engine's canonical JSON is true RFC 8785 (JCS), closing the DEC-010
"JCS-compatible hashing" alignment gap H1 measured. New shared crate
`core/serialization/canonical_json`: ECMAScript `Number::toString` number
rendering with ryu digit selection (std's formatter breaks exact
shortest-representation ties differently from ECMAScript — measured, 7 in
20k random doubles; ryu matched node on 112,220 fuzz vectors), UTF-16
code-unit key sort, `JSON.stringify`-identical escaping; one documented
divergence (raw-text integers beyond 2^53 keep exact i64/u64 precision —
outside the I-JSON envelope, unreachable via JS transport).
`operation_applier::canonical_json` re-exports it;
`BACKEND_CANONICALIZATION` and every engine-hash label across the
frontend, schema enum, and headless runner unified to `rfc8785_jcs`.

Both `TP-H1-HASHUNIFY-001` refutations are resolved: JS transport no
longer shifts canonical bytes, so the corpus ECMA harness renderer twins
and their number-range constraint are deleted, and backend hash VALUES
(`backend_model_hash`, `applied_model_backend_hash`) joined the
full-byte-equality compared surface in every lane. `fixtures/canonical_hash/`
extended 14 → 20 cases (notation boundaries, tie-to-even, −0, beyond-2^53,
UTF-16 sort; floor 12 → 20) and all 44 contract-corpus cases re-blessed.
Headless runner checksums render through the shared crate (TASK subscope;
its previously aspirational `"JCS"` label is now true); audit_manifest
checked — caller-supplied number strings, doc pointer added, no behavior
change. Residual hand-offs (vocabulary-only label sites in
`primitive_loads`/`result_export`/fixtures; Python persistence truth-label
wording; possible future claimed-vs-backend equality evaluation) recorded
in the run record.

Evidence: `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/_run_records/WORKING_ITEMS_RUN_2026-06-11_h5_rfc8785_rendering.md`
(TASK sub-record in DEL-10-05 `_run_records/`); suites: canonical_json 8,
applier 34+20+44×2, Vitest 172/172, headless 11, audit_manifest 13,
src-tauri 28, pytest 353; DEC-025 sweep at the closing commit.

## 2026-06-11 — H1: canonical-hash unification through the wasm engine (`TP-H1-HASHUNIFY-001`)

Frontend hashing is single-sourced: `hashService.ts` now calls the wasm
exports of the engine's `canonical_json`/`sha256_hex` (new
`canonical_json_string` / `canonical_sha256_hex` in `wasm_api`, throwing
the named input diagnostic on malformed JSON), and the TS canonicalization
plus the A7 report-input WebCrypto helper (with its silent `"TBD"` soft
fallback) are deleted — no fallback hashing path exists in the frontend. A
new Rust-blessed parity corpus (`fixtures/canonical_hash/`, 14 cases,
12-case floor) feeds identical raw JSON text to both lanes and pins the
engine's canonical form and number normalization.

The allowlist-tightening rider was executed as a measurement and the
hypothesis from the T4 caveat was **partially refuted with evidence**:
`backend_canonicalization` (invariant label) joined the blessed corpus
projection across all 44 re-blessed cases, and the native runner now
asserts engine self-consistency (`applied_model_backend_hash` =
`canonical_json`+`sha256_hex` of the returned document); but the
`backend_model_hash` value exclusions stand, for measured transport
reasons now documented in the corpus README — `JSON.stringify` renders
`200.0` as `200` so input-text hashes legitimately differ across lanes,
and the engine's applied-model hash covers in-process text reproducible
only Rust-side (recorded consequence: receipt-hash verification must
re-ask the engine; no frontend recomputation is possible). The corpus's
ECMA harness renderer survives in both runners as explicitly harness-only
code with this rationale attached.

Evidence: operation_applier cargo suites green; desktop Vitest 166/166
(was 140); DEC-025 five-surface sweep overall-pass. Engine slice landed as
`6e36f5da4` mid-tranche (collision defense during the shared-worktree
incident); the adapter/corpus slice closes with this entry. Run record:
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/_run_records/WORKING_ITEMS_RUN_2026-06-11_h1_hash_unification.md`;
SMOKE `TP-MAC-114`.

## ## 2026-06-11 — C1 landed: frozen expression grammar v1.0.0 + conformance corpus (`TP-C1-GRAMMAR-001`)

Implementation of the D-02 ruling (`DEC-022`, packet Option A) by a bounded
TASK worker in `core/rules`: the typed expression AST extended to the full
PRD §12.3 set (And/Or/Not, eager Select with blocking unselected-branch
diagnostics, n-ary Min/Max and Abs over same-dimension same-unit
quantities, piecewise-linear Interpolate and exact/step Lookup over
strictly-monotone user tables with blocking out-of-range — no
extrapolation or clamping); the two in-code dimensional TBDs resolved via
an enumerated commutative 18-relation dimension-product table with
ambiguous/unrepresentable results blocking; `GRAMMAR_VERSION = "1.0.0"`
declared-version gate; `grammar_version` bound inside the JCS-hashed
rule-pack checksum (lifecycle record field, blocking findings,
byte-containment evidence check, binding-enforcing constructor). New
blessed conformance corpus `fixtures/rule_expressions/` (69 golden cases +
checksum-binding golden hashes, all synthetic) executed as crate tests
with a runner-enforced coverage floor. Evaluator 31+1 corpus test (69
cases), lifecycle 12, completeness_checker 12, pytest 353 — all green;
zero new dependencies. Residuals and three labeled ASSUMPTIONs (dimension
table contents, derived unit-ref convention, Select branch-compatibility
rule) flagged for human review in the run record.

Evidence:
`execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/_run_records/TASK_RUN_2026-06-11_TP-C1-GRAMMAR-001.md`.

## 2026-06-11 — D7 first slice landed: in-repo sparse skyline direct solver (`TP-D03-SPARSE-001`)

Implementation of the D-03 ruling (`DEC-023`, packet Option C) by a bounded
TASK worker in `core/solver`: new `sparse_direct` crate with deterministic
reverse Cuthill–McKee ordering (all ties broken by ascending
degree/original-index, permutations pinned by test), skyline profile
storage of the symmetric stiffness, in-repo LDLᵀ profile factorization and
solve, single-threaded fixed operation order, zero new dependencies;
diagnostics crate gains NonPositivePivot and sparse-error/factorization
mappings with deterministic ordering; performance harness measures the
sparse path alongside dense (profile/bandwidth reduction, pivot extrema,
parity delta, residual, repeat determinism) with no thresholds asserted
(D-04 governs). Parity vs dense at the DEC-026 analytic seed (1.0e-9
relative) on chain/grid models up to 360 reduced DOFs plus a dyadic-exact
hand-checked case. sparse_direct 18, diagnostics 24, performance_harness
18, frame_kernel (untouched) 34 — all green. Remaining D7 scope stays in
the row: live solve-path adoption (integration plan in the run record) and
profile-direct assembly.

Evidence:
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/_run_records/TASK_RUN_2026-06-11_TP-D03-SPARSE-001.md`.

## 2026-06-11 — A7 landed: rendered FR-016 calculation report (`TP-APP-R2-REPORTRENDER-001`)

Implementation of the D-10 ruling (`DEC-021`, packet Option B): new
`core/reporting/report_renderer` crate renders the validated
`CalculationReport` + `ReportSections` + caller result rows into a
deterministic, self-contained, scriptless single-file HTML document;
SHA-256 of the bytes is the canonical hash-bound evidence; three-point
protected-content lint gating (template surface, pre-render section text,
post-render document text) plus blocking validation diagnostics refuse
export with a visible banner; `derived_print_view` emits the labeled
non-hash-bound print/PDF view naming the canonical hash. Feature-gated
serde derives on report_generator/report_sections pin the schema-contract
spellings, proven against the canonical fixture. Desktop seam: 15th Tauri
command `render_calculation_report`, render service with an explicit
desktop-only browser route (no fallback), session-envelope adapter with
explicit TBD markers, and a Rendered Report panel whose save/print actions
are refused while export is blocked. Residuals in the A7 row: browser-mode
render seam (A8 decision), rule-pack refs population (Phase C),
report-hash persistence follow-up.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-113;
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_run_records/WORKING_ITEMS_RUN_2026-06-11_TP-APP-R2-REPORTRENDER-001.md`;
renderer crate 8/8, Vitest 148/148, src-tauri command test, production
build green; commit-bound sweep summary at the tranche push.

## 2026-06-11 — DEC-025 evidence sweep + F-4 atomic wasm build landed (`TP-SWEEP-001`)

Implementation of the D-05 ruling (`DEC-025`, packet Option D): new
deterministic entrypoint `tools/release/run_evidence_sweep.py` runs the five
evidence surfaces sequentially in F-4-safe order (cargo crate sweep, pytest,
desktop Vitest with the wasm engine built first, Playwright e2e, desktop
production build), fail-fast with `not_run` accounting, and writes a
commit-bound machine-readable summary to `validation/evidence/sweeps/`. The
sweep is the required pre-push/fan-in merge gate for parallel agent branches
(`docs/BUILD_AND_RELEASE.md` §5.1; gate pattern: commit → sweep at clean
HEAD → evidence-only closeout commit → push). F-4 rider: the wasm build now
writes glue to a sibling staging dir and renames it into place; staging dirs
gitignored. Docs: BUILD_AND_RELEASE §2/§3/§5.1/§7/§9 and
RELEASE_QUALITY_GATES §10 record the ruling; the CI-provider TBD closes with
the `D-05b` follow-up pointer. 11 focused tests in
`tests/test_evidence_sweep.py`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-112; validation-run summary
`validation/evidence/sweeps/SWEEP_20260612T031241Z_0f402fc48424-dirty.json`
(all five surfaces pass, exit 0) plus the clean gate-run summary committed
at closeout;
`execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_run_records/WORKING_ITEMS_RUN_2026-06-11_TP-SWEEP-001_evidence_sweep.md`.

## 2026-06-11 — Seam-verification regression repair: F-1 Playwright budget, F-2 NUL bytes (`TP-SEAM-FIX-001`)

Human-instructed repair tranche for the open findings in the independent
post-closure verification
([VERIFICATION_2026-06-11_operation_seam_unification.md](VERIFICATION_2026-06-11_operation_seam_unification.md)).
F-1: the default `test:e2e` failed at the 30s per-test budget; measurement
showed the ~10× runtime growth is the Playwright trace recorder
(`retain-on-failure` records every run and discards on pass — 8.8s trace
off vs 21.6s warm / 37.4s cold with trace on, ~21s of it in trace
finalization at `browser.close`), not the wasm engine (engine-ready wait
<1s, artifact ~330 kB); the T4 apply-flow steps run against the heaviest
DOM state, multiplying per-action snapshot cost. Fixed by raising the
budget to 120s with the measurements recorded in the config comment;
tracing kept; spec not split. F-2: the two literal NUL separators in
`operationContractCorpus.test.ts` replaced with string escapes, making the
file text again for git diff review. F-3 (corpus fixture review
disposition) remains with the human; F-4 stands as an operational note for
`D-05`. Full evidence set re-run sequentially: e2e 1/1 at 18.8s under the
default command, Vitest 140/140, cargo sweep exit 0, pytest 342/342,
production build green. Run record:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-11_seam_verification_regression_repair.md`;
SMOKE entry `TP-MAC-111`.

## 2026-06-11 — Operation-seam plan T4 and plan closure: wasm route swap, TS engine deleted (`TP-SEAM-SWAP-001`)

The structured editor-operation seam is now implemented exactly once. In
every environment — packaged Tauri, Vite dev browser, jsdom Vitest,
Playwright Chrome — validation and apply outcomes are produced by
`core/model_operations/operation_applier` (native via the Tauri commands,
wasm32 otherwise). The TypeScript engine and its private semantic tables
(field rules, canonical dimension set, restraint tokens, deferred fields,
collections) no longer exist in the tree.

`operationService.ts` shrank from a 2,280-line engine to a 112-line thin
routing adapter (Tauri `invoke` else wasm; wasm input-error envelope
surfaces as an explicit thrown error; honest engine-status reporting). Its
17 engine tests (911 lines) are superseded by the contract corpus + 6
adapter tests; net deletion −3,114/+200 lines. Browser receipts now read
`route=local_wasm_engine`; the apply panel reports engine readiness via
`operation-engine-status` with the named `WASM-ENGINE-ASSET-ABSENT`
diagnostic and exact build command when the artifact is missing. The corpus
TS lane retired with the engine: the Vitest runner keeps two lanes (public
adapter seam, wasm engine direct) against the Rust-blessed reference as the
permanent native↔wasm parity + regression surface. Vitest setup pre-warms
the engine; `test:e2e` builds the artifact first; the Playwright spec waits
on engine-ready and ends with a real-browser apply asserting the wasm route
receipt.

Evidence: cargo profile sweep exit 0 (25 manifests, zero failures); pytest
342/342; Vitest 140/140; Playwright 1/1; production build green (index
chunk at the ~577 kB baseline — the engine loads dynamically).
`apps/desktop/SMOKE.md` TP-MAC-110; primary run record at DEL-16-02
`_run_records/WORKING_ITEMS_RUN_2026-06-11_t4_wasm_engine_swap.md` (fan-out
records at DEL-16-03, DEL-07-02, DEL-00-08).

**Plan closure.** With T4 landed, all §4 exit criteria of
[PLAN_2026-06-11_operation_seam_unification.md](PLAN_2026-06-11_operation_seam_unification.md)
are met: one engine; corpus executing in both `cargo test` and `npm test`
with hash parity; D-13 RULED → `DEC-020` with ADR-0001 accepted on the
stood-up ADR surface; the `LocalFeaHandoffPanel` fallback replaced; all
test surfaces green with evidence recorded. The §5 freeze rule is lifted;
the plan's Active Surface pointer in `_COORDINATION.md` was removed per the
acceptance authorization. Deferred items (plan §9) — including the R2
from-scratch authoring set and Phase B B2, both deliberately sequenced
after this unification — roll forward to completion-plan selection, which
now resumes.

## 2026-06-11 — Operation-seam plan T3: wasm build enablement (`TP-SEAM-WASM-001`)

The wasm32 build of `core/model_operations/operation_applier` is now a
working browser-loadable engine with proven parity. Landed across two
sessions (scaffolding committed as `wip(seam-T3)` at handoff, then resumed):
feature-gated `wasm` wasm-bindgen JSON-string exports
(`validate_operation_json` / `apply_operation_json`; native build and suite
unaffected); `apps/desktop/scripts/build-wasm-engine.mjs` +
`build:wasm` / `build:wasm:desktop` npm scripts (prerequisite checks with
exact remediation commands — wasm32 target, wasm-bindgen CLI pinned to the
crate's =0.2.123 — and gitignored `__generated__/` output, never committed);
a loading shim whose absent-artifact failure is the named diagnostic
`WASM-ENGINE-ASSET-ABSENT` with the build command (no silent fallback, no
fallback engine); and the corpus wasm lane in
`operationContractCorpus.test.ts`.

The plan §3 first-task spike — the 44-case corpus through the wasm engine
under Vitest/jsdom — succeeded; the §6 stop rule did not fire and no TS
fallback was written. Two environment-fit fixes only, both artifact
location/typing (engine semantics untouched): Node-branch path probing for
the wasm bytes (Vite's test transform rewrites `import.meta.url` to a
root-relative file URL, breaking module-relative resolution; loud failure
now lists every probed path) and a `Uint8Array<ArrayBuffer>` copy for
`BufferSource` typing (also fixed a latent `tsc -b` failure in the WIP
shim, caught by this tranche's production-build evidence).

Result: three-way native↔wasm↔TS parity — identical semantic outcomes and
identical corpus-harness canonical sha256 hashes — so hash evidence does not
fork by environment. Wasm-lane extras: honest `local_wasm_engine` route
assertion; structured input-error envelope
(`WASM-ENGINE-INPUT-JSON-INVALID`) instead of a trap.
`docs/BUILD_AND_RELEASE.md` §3 records the engine identity, toolchain
prerequisites, and build step.

Evidence: `cargo test` 36/36 (native unchanged); `npm run build:wasm:desktop`
OK; `npm test --workspace apps/desktop` 151/151 (46 wasm-lane); desktop
production build green (index chunk at the ~577 kB baseline — the engine is
dynamically loaded, not bundled). Primary record:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-02_Repository and module boundary architecture/_run_records/WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_completion.md`
(WIP handoff record superseded in the same folder; fan-out records at
DEL-00-08 and DEL-16-02).

## 2026-06-11 — Operation-seam plan T1: cross-engine contract corpus + handoff rider (`TP-SEAM-CORPUS-001`)

The editor-operation seam now has an executable cross-engine contract: 44
invented cases at `fixtures/model_operations/contract_corpus/` run inside
both engines' normal suites — a Rust integration test in the
`operation_applier` crate (the contract reference, with a `CORPUS_BLESS=1`
regeneration mode) and a Vitest spec driving `operationService` browser mode
— asserting identical semantic outcomes (validation states, diff-preview
rows, order-insensitive diagnostic records), parsed-JSON deep equality of
applied documents, and one shared corpus-harness canonical sha256 per
applied document. Engine-identity fields are excluded only via a documented
per-field allowlist. The coverage floor is enforced programmatically in both
runners: all 10 operation kinds accepted, the named block classes (19
distinct blocking codes), and the dynamic paths
`primitive_loads.N.magnitude.value` / `terms.N.factor` accepted and blocked.
Notable finding: the TypeScript mirror reproduced all 44 Rust-reference
outcomes on first execution — zero alignment fixes, zero quarantined cases —
which retroactively strengthens all existing browser-based evidence.

The boundary-hygiene rider removed the `LocalFeaHandoffPanel` fixture-ID
fallback (`pipe:P-120` / `node:N-140`): missing result-summary refs now emit
the explicit `LOCAL-FEA-RESULT-SUMMARY-REF-MISSING` finding with an honestly
degraded region id and no invented references (two new panel tests).

Execution note: the bounded TASK worker for this tranche was interrupted by
an environment connection loss after authoring its artifacts; WORKING_ITEMS
independently re-verified everything (cargo 36/36; Vitest 105/105;
Playwright 1/1 unchanged; desktop production build green) and completed the
closeout. Residual: corpus README `review_status` pending human review.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-109 and
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-11_seam_contract_corpus_fanin.md`
(interrupted worker record `TASK_RUN_2026-06-11_1414.md` finalized FAILED in
the same folder; fan-out records at DEL-16-03 and DEL-00-08; rider record at
DEL-10-03).

## 2026-06-11 — Operation-seam plan T2: ADR surface + D-13 ruling transcription (`TP-SEAM-DECISION-001`)

The ADR surface anticipated by DEL-00-01 now exists: `docs/architecture/adr/`
with `template.md`, `index.md`, and
`ADR-0001_operation_seam_engine_unification.md` (status accepted, dated
2026-06-11) recording the D-13 ruling — wasm32 `operation_applier` as the
sole browser-mode engine, the TS mirror's validation/apply logic retired at
plan T4, four alternatives rejected with reasons, consequences, and
reconsideration triggers. The pre-write gate check confirmed the surface was
unbuilt by explicit deferral (PKG-00 lock review), not resolved elsewhere.
Transcription obligations completed: `DEC-020` appended to
`SOFTWARE_DECOMP.md` §12 and the D-13 row appended to
`_DECISIONS/_REGISTER.md` directly in state RULED, with the accepted plan
itself as the packet. Cross-consistency of ADR ↔ DEC-020 ↔ D-13 ↔ plan §2
verified; no code changes.

Evidence:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-01_Architecture decision record baseline/_run_records/TASK_RUN_2026-06-11_1408.md`.

## 2026-06-11 — A6 second sub-slice: results family selector (`TP-APP-R2-RESULTFAMILY-001`)

The results panel now exposes direct result-family filters with visible counts
for the solved preview envelope: displacement, reaction, force, moment, and
stress. Selecting a family resets pagination and constrains the existing
table groups, while the existing free-text filter still composes with the
selected family. The support-reaction rows and stress rows are therefore
selectable as dedicated table views without requiring text-search strings.

The fixture-backed app test asserts the family counts
(`15/9/180/180/263`), reaction-only pagination (`9 of 647`), stress-only
pagination (`263 of 647`), active `aria-pressed` state, and representative
reaction/stress rows. The Playwright R2 smoke covers the reaction family path
after solving. In-app browser smoke at `http://127.0.0.1:5175/` confirmed the
solved app showed `result_rows=647`, reaction/stress family selection,
visible non-overflowing selector buttons, all-family restoration, and zero
browser console errors.

Residuals remain in A6: true directional deformed shape once displacement
vectors exist, governing-ratio views once ratio rows exist in result
envelopes, and richer result-selection coupling.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-108 and
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-05_Results viewer/_run_records/WORKING_ITEMS_RUN_2026-06-11_results_family_selector.md`.

## 2026-06-11 — A5 second sub-slice: persisted edited-load solve regression (`TP-APP-R2-PERSISTEDSOLVE-001`)

The Tauri backend test suite now proves an edited load-data model can move
through the app's structured operation and local persistence seams before
being solved from the restored model payload. The regression applies an
explicit `update_load` operation to `load:L-100` primitive
`primitive_loads.1.magnitude.value`, persists the edited model into the local
SQLite project store, reloads it by project id, and solves the restored
payload through `solve_preview_mechanics`.

The test asserts the saved/reopened model retains the edited force magnitude,
the restored solve is bound to `project:edited-load-roundtrip`, mechanics
status is `MECHANICS_SOLVED`, and `result:disp:node-N-140` changes relative
to the original fixture solve. This is backend evidence for saved edited-load
data reaching the solve boundary; it does not claim that the later packaged
Tauri GUI smoke is complete.

Residuals remain in A5: full packaged-Tauri GUI smoke over a saved edited
project snapshot, UI polish for incomplete-model diagnostics, and broader
persisted solve coverage as new authoring surfaces grow.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-107;
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-11_persisted_edited_load_solve_regression.md`,
the same-named record under
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/`,
and the same-named record under
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_run_records/`.

## 2026-06-11 — A4 thirteenth sub-slice: combination creation editor (`TP-APP-R2-COMBCREATE-001`)

The Load Cases manager now exposes explicit creation for mechanics-basis load
combinations. The create form captures a new combination id, label, mechanics
basis, one existing load-case term, a finite dimensionless factor, provenance,
and rationale, then queues a structured `create_combination` operation with
`field_path=combinations`, `before=not_present`, unit `none`, and dimension
`dimensionless`.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`create_combination` payloads. Accepted intents must create a new
`Combination`, use basis `mechanics`, include at least one explicit term,
reference existing load cases, carry finite factors, avoid duplicate initial
operands, preserve structured-operation audit boundaries, and emit no
professional approval claim. Duplicate ids, missing load cases, empty terms,
invalid payloads, non-mechanics basis values, and invalid unit/dimension
metadata are blocked.

The app-level regression queues `op:load-manager-create-combination:C-300`,
applies it through `OperationApplyPanel`, verifies the manager summary changes
to two combinations, checks the new `combination:C-300` row with
`load:L-100 x 1`, selects the row into the property inspector, and confirms
zero pending operations with solve state reset to `not_started`. The
Playwright R2 smoke verifies the rendered create-combination preview without
applying so the solve/results/report path remains on the unchanged fixture
model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-create-combination:C-300` and confirmed two combinations,
`combination:C-300` with `basis=mechanics` and `load:L-100 x 1`, property
inspector selection, zero pending operations, `applied_operations=1`, solve
state `not_started`, and no browser console errors.

Residuals remain in A4: subtraction/range expression authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-106;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 twelfth sub-slice: combination term deletion editor (`TP-APP-R2-COMBTERMDELETE-001`)

The Load Cases manager now exposes explicit deletion for a selected existing
combination term. The delete control appears in the selected-term editor,
captures a rationale, and queues a structured `delete_combination_term`
operation with `field_path=terms.N`, `before=<load_case> x <factor>`,
`after=not_present`, unit `none`, and dimension `dimensionless`. The operation
removes one indexed term only; whole-term replacement, code/rule combinations,
and broader algebra authoring remain deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`delete_combination_term` payloads. Accepted intents must target an existing
`Combination`, address an existing term index, carry a before-value matching
the current term display, preserve structured-operation audit boundaries, and
emit no professional approval claim. Stale before-values, out-of-range
indices, invalid unit/dimension metadata, and missing combination `terms`
arrays are blocked.

The app-level regression selects `combination:C-OPER-ALT` term 1, queues
`op:load-manager-combination:C-OPER-ALT-term-1-delete`, applies it through
`OperationApplyPanel`, and verifies `load:L-200 x 0.5` is removed while
`load:L-100 x 1` remains. The Playwright R2 smoke verifies the rendered delete
preview without applying so the solve/results/report path remains on the
unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-combination:C-OPER-ALT-term-1-delete` and confirmed
`load:L-200 x 0.5` was no longer visible in the combination row, zero pending
operations, `applied_operations=1`, solve state `not_started`, and no browser
console errors.

Residuals remain in A4: broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-105;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_deletion_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 eleventh sub-slice: combination term creation editor (`TP-APP-R2-COMBTERMCREATE-001`)

The Load Cases manager now exposes explicit child-term creation for existing
load combinations. The create form selects an existing `Combination`, an
existing `LoadCase`, a finite dimensionless factor, and a rationale, then
queues a structured `create_combination_term` operation with
`field_path=terms`, `before=not_present`, unit `none`, and dimension
`dimensionless`. The operation appends one term only; whole-term replacement,
term deletion, code/rule combinations, and broader algebra authoring remain
deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`create_combination_term` payloads. Accepted intents must target an existing
`Combination`, reference an existing `LoadCase`, carry JSON
`{ "load_case": string, "factor": number }`, preserve structured-operation
audit boundaries, and emit no professional approval claim. Missing load cases,
invalid payloads, non-dimensionless metadata, and missing combination `terms`
arrays are blocked.

The app-level regression creates `load:L-300`, then appends it to
`combination:C-OPER-ALT` as `load:L-300 x 0.25` through
`OperationApplyPanel`, proving existing `load:L-100 x 1` and
`load:L-200 x 0.5` terms are preserved. The Playwright R2 smoke verifies the
rendered create-term preview without applying so the solve/results/report path
remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied the default
create-load-case intent, selected `load:L-300` in the combination-term form,
applied `op:load-manager-combination:C-OPER-ALT-term-2-create`, and confirmed
`load:L-300 x 1`, zero pending operations, `applied_operations=2`, solve
state `not_started`, and no browser console errors. The smoke avoided text
entry because the Browser plugin's virtual clipboard remained unavailable for
`fill`.

Residuals remain in A4: combination term deletion, broader algebra authoring,
Phase B unit picker/display retirement, and packaged-Tauri saved-project smoke
over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-104;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 tenth sub-slice: combination basis editor (`TP-APP-R2-COMBBASIS-001`)

The Load Cases manager now exposes explicit basis editing for existing load
combinations. The selected-combination editor captures a replacement
`Combination.basis` text value and rationale, then queues a structured
`update_load` operation with `field_path=basis`, unit `none`, and dimension
`dimensionless`. The operation is limited to an existing combination record;
whole-term replacement, term creation/deletion, code/rule combinations, and
broader algebra authoring remain deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now treat
`Combination.basis` as an editable text field while retaining the explicit
deferred finding for whole `Combination.terms` edits. Accepted basis edits
must target an existing `Combination`, carry a current before-value, provide a
non-empty replacement value, and route through structured operations only. The
tests prove the edit changes only `basis` and preserves `terms` and
`provenance`.

The app-level test selects `combination:C-OPER-ALT`, changes basis from
`mechanics` to `mechanics_user_review`, queues
`op:load-manager-combination:C-OPER-ALT-basis`, applies it through
`OperationApplyPanel`, verifies the manager row and property inspector, and
confirms zero pending operations with solve state reset to `not_started`. The
Playwright R2 smoke checks the rendered basis preview without applying so the
solve/results/report path remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` keyed
`mechanics_user_review` into the basis field, applied
`op:load-manager-combination:C-OPER-ALT-basis`, and confirmed
`basis=mechanics_user_review`, zero pending operations,
`applied_operations=1`, and solve state `not_started`. The smoke used
single-key presses because the Browser plugin's virtual clipboard remained
unavailable for `fill`/`type`.

Residuals remain in A4: combination term creation/deletion, broader algebra
authoring, Phase B unit picker/display retirement, and packaged-Tauri
saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-103;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_basis_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 ninth sub-slice: imposed-displacement primitive-load creation editor (`TP-APP-R2-IMPOSED-001`)

The Load Cases manager now exposes explicit imposed-displacement primitive
load creation for existing load cases. The create form selects
`imposed_displacement`, captures the target load case, primitive-load id,
existing support target, support DOF, magnitude, and provenance, then queues a
structured `create_primitive_load` operation. Translational DOFs
`UX|UY|UZ` use the project length unit and dimension `displacement`.
Rotational DOFs `RX|RY|RZ` use the project angle unit and dimension
`rotation`. The invented preview model now records `project.units.angle =
"rad"` so rotational imposed-displacement previews and validation consume
explicit project metadata.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`imposed_displacement` primitive-load creation alongside the previously
landed concentrated-force, distributed-force, concentrated-moment, pressure,
and thermal paths. Accepted imposed-displacement intents must target an
existing `Load` with `field_path=primitive_loads`, `before=not_present`, a
globally unique primitive-load id, category `imposed_displacement`, target
`{ type: "support", support: <existing support id>, dof: <matching DOF> }`,
finite magnitude in the expected project unit, matching dimension, and
non-empty provenance. Duplicate primitive IDs, missing support targets,
invalid DOFs, target/DOF mismatches, and missing project unit metadata are
blocked.

The app-level test creates `load:L-100-I300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new support-targeted row
`load:L-100-I300; support:support:S-100; UZ; dimension=displacement`,
confirms the local receipt, and confirms stale solve state is reset. The
operation-service and Rust tests also apply a rotational `RX` payload with
dimension `rotation` and unit `rad`. The Playwright R2 smoke checks both the
translational preview and the rotational `RX` unit/dimension preview without
queueing so the solve/results/report path remains on the unchanged fixture
model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-I300-primitive` from a clean session
using the default finite magnitude `250 m`. The smoke confirmed `2 load
cases; 8 primitive loads; 1 combinations`, manager row
`load:L-100-I300; support:support:S-100; UZ; dimension=displacement`, zero
pending operations, `persistence=session_state_only_not_yet_saved`,
`professional_approval=false`, and solve state `not_started`. The browser
plugin could not type a replacement magnitude because its virtual clipboard
was unavailable; the negative translational magnitude path is covered by
Vitest.

Residuals remain in A4: combination basis editing, combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-102;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_imposed_displacement_load_creation_editor.md`,
the same-named record under
`DEL-04-03_Linear support and restraint models/_run_records/`, the
same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 eighth sub-slice: pressure and thermal primitive-load creation editor (`TP-APP-R2-PRESSTEMP-001`)

The Load Cases manager now exposes explicit pressure and thermal primitive
load creation for existing load cases. The create form selects `pressure` or
`thermal`, captures the target load case, primitive-load id, existing pipe
target, global direction, magnitude, and provenance, then queues a structured
`create_primitive_load` operation. Pressure payloads are limited to target
`{ type: "element", pipe: <existing pipe id> }`, dimension `pressure`, and
the project pressure unit (`Pa` for the invented preview model). Thermal
payloads use the same element target contract with dimension
`temperature_interval` and the project temperature interval unit (`degC` for
the invented preview model). No gauge/absolute pressure conversion,
reference-pressure default, thermal absolute-temperature conversion, imposed
displacement, hidden default, unit conversion, or code-specific combination is
inferred.

The invented preview model now records `project.units.pressure = "Pa"` so
pressure creation consumes explicit project unit metadata rather than an
implicit pressure fallback. This remains a single-unit technical-preview
posture: D-01 has accepted future unit semantics, but Phase B unit conversion,
quantity-kind UI, reference-pressure handling, and unit picker/display
retirement remain outside this tranche.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`pressure` and `thermal` primitive-load creation alongside the previously
landed concentrated-force, distributed-force, and concentrated-moment paths.
Accepted pressure/thermal intents must target an existing `Load` with
`field_path=primitive_loads`, `before=not_present`, the corresponding project
unit metadata, matching dimension, a globally unique primitive-load id,
category `pressure` or `thermal`, an existing pipe target, global direction,
finite magnitude in the expected unit, and non-empty provenance. Duplicate
primitive IDs and missing pipe targets are blocked.

The app-level tests create `load:L-100-P300` and `load:L-100-T300` through
the manager, apply each through `OperationApplyPanel`, verify the manager
summary and `load:L-100` primitive count, check the new element-targeted rows,
confirm the local receipt, and confirm stale solve state is reset. The
Playwright R2 smoke checks both rendered create previews without queueing so
the solve/results/report path remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-P300-primitive` and
`op:load-manager-load:L-100-load:L-100-T300-primitive` in clean sessions. The
smoke confirmed `2 load cases; 8 primitive loads; 1 combinations`, pressure
row `load:L-100-P300; element:pipe:P-100; global_x; dimension=pressure`,
thermal row `load:L-100-T300; element:pipe:P-100; global_z;
dimension=temperature_interval`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: imposed-displacement authoring breadth, combination
basis editing, combination term creation/deletion, broader algebra authoring,
Phase B unit picker/display retirement, and packaged-Tauri saved-project smoke
over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-101;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_pressure_thermal_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 seventh sub-slice: concentrated moment primitive-load creation editor (`TP-APP-R2-MOMENTCREATE-001`)

The Load Cases manager now exposes explicit concentrated nodal-moment
primitive load creation for existing load cases. The create form selects
`concentrated_moment`, captures the target load case, primitive-load id,
existing node target, rotational direction, magnitude, and provenance, then
queues a structured `create_primitive_load` operation. The payload is limited
to target `{ type: "node", node: <existing node id> }`, direction
`rotation_x|rotation_y|rotation_z`, dimension `moment`, and the project
force*length unit basis (`N*m` for the invented preview model). No
pressure/temperature primitives, imposed displacements, hidden defaults, unit
conversion, or code-specific combinations are inferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`concentrated_moment` primitive-load creation alongside the previously landed
`concentrated_force` and `distributed_force` paths. Accepted concentrated
moment intents must target an existing `Load` with `field_path=primitive_loads`,
`before=not_present`, project force and length unit metadata, dimension
`moment`, a globally unique primitive-load id, category
`concentrated_moment`, an existing node target, rotational direction, finite
magnitude in the expected unit, and non-empty provenance. Duplicate primitive
IDs and missing node targets are blocked.

The app-level test creates `load:L-100-M300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-M300; node:node:N-100; rotation_z; dimension=moment`, confirms
the local receipt, and confirms stale solve state is reset. The Playwright R2
smoke checks the rendered concentrated-moment create preview without queueing
so the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-M300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the concentrated-moment manager
row with `250 N*m`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: pressure/temperature primitive creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-100;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_moment_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 sixth sub-slice: distributed primitive-load creation editor (`TP-APP-R2-DISTLOAD-001`)

The Load Cases manager now exposes explicit distributed element-force
primitive load creation for existing load cases. The create form selects
`distributed_force`, captures the target load case, primitive-load id, existing
pipe target, global direction, magnitude, and provenance, then queues a
structured `create_primitive_load` operation. The payload is limited to target
`{ type: "element", pipe: <existing pipe id> }`, dimension
`force_per_length`, and the project force/length unit basis (`N/m` for the
invented preview model). No concentrated moments, pressure or temperature
primitives, imposed displacements, hidden defaults, unit conversion, or
code-specific combinations are inferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`distributed_force` primitive-load creation alongside the previously landed
`concentrated_force` path. Accepted distributed intents must target an
existing `Load` with `field_path=primitive_loads`, `before=not_present`,
project force and length unit metadata, dimension `force_per_length`, a
globally unique primitive-load id, category `distributed_force`, an existing
pipe target, direction `global_x|global_y|global_z`, finite magnitude in the
expected unit, and non-empty provenance. Duplicate primitive IDs and missing
pipe targets are blocked.

The app-level test creates `load:L-100-D300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-D300; element:pipe:P-100; global_y; dimension=force_per_length`,
confirms the local receipt, and confirms stale solve state is reset. The
Playwright R2 smoke checks the rendered distributed create preview without
queueing so the solve/results/report path remains on the unchanged fixture
model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-D300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the distributed-force manager row
with `250 N/m`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: concentrated moments, pressure/temperature primitive
creation, imposed-displacement authoring breadth, combination basis editing,
combination term creation/deletion, broader algebra authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-99;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_distributed_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 fifth sub-slice: concentrated primitive-load creation editor (`TP-APP-R2-PRIMCREATE-001`)

The Load Cases manager now exposes explicit concentrated nodal-force primitive
load creation for existing load cases. The form captures target load case,
primitive-load id, existing node target, global direction, magnitude, and
provenance, then queues a structured `create_primitive_load` operation. The
payload is limited to category `concentrated_force`, dimension `force`, and the
project force unit; no distributed loads, concentrated moments, pressure or
temperature primitives, imposed displacements, hidden defaults, unit
conversion, or code-specific combinations are inferred.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply `create_primitive_load`
intents. Accepted intents must target an existing `Load` with
`field_path=primitive_loads`, `before=not_present`, project force unit,
dimension `force`, a globally unique primitive-load id, category
`concentrated_force`, an existing node target, direction
`global_x|global_y|global_z`, finite magnitude, and non-empty provenance.
Duplicate primitive IDs and missing node targets are blocked.

The app-level test creates `load:L-100-F300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-F300; node:node:N-100; global_y; dimension=force`, confirms the
local receipt, and confirms stale solve state is reset. The Playwright R2 smoke
checks the rendered create preview without queueing so the solve/results/report
path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-F300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the concentrated-force manager row
with `250 N`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: distributed primitive-load creation, concentrated
moments, pressure/temperature primitive creation, imposed-displacement
authoring breadth, combination basis editing, combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-98;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_primitive_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 fourth sub-slice: empty load-case creation editor (`TP-APP-R2-LOADCREATE-001`)

The Load Cases manager now exposes explicit empty load-case creation for the
invented preview model. The create form captures a user-supplied load-case id,
label, kind, status, and provenance, previews the structured create operation,
and queues only a load-case shell with `primitive_loads=0`. It does not create
primitive loads, imposed displacements, code combinations, hidden defaults, or
derived engineering values.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply `create_load_case`
intents. Accepted intents must target `Load` with `field_path=load_cases`,
`before=not_present`, unit `none`, dimension `dimensionless`, a non-duplicate
load-case id, matching JSON payload id, non-empty label/kind/status/
provenance, and absent or empty `primitive_loads`. Non-empty primitive payloads
are blocked so future primitive-load creation remains an explicit A4 tranche.

The app-level test creates `load:L-300` through the manager, applies it through
`OperationApplyPanel`, verifies the manager summary and row
`load:L-300; primitive_user_load; draft; primitives=0`, checks the property
inspector, confirms the local receipt, and confirms stale solve state is reset.
The Playwright R2 smoke checks the rendered create preview without queueing so
the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-create-load:L-300` and confirmed `3 load cases; 7 primitive
loads; 1 combinations`, the empty-shell manager row, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: arbitrary primitive-load creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-97;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 third sub-slice: combination term-factor editor (`TP-APP-R2-COMBFACTOR-001`)

The Load Cases manager now exposes existing combination term-factor editing
for the invented mechanics preview combination. The editor selects an
existing `Combination.terms.N.factor` row, queues a structured `update_load`
intent with explicit before/after scalar values, unit `none`, dimension
`dimensionless`, local-session audit boundaries, and no professional approval
claim. Whole `Combination.terms` replacement, combination `basis` editing,
term creation/deletion, code/rule combinations, and broader algebra authoring
remain explicitly out of scope.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply the dynamic
`terms.N.factor` path as a dimensionless numeric field. The app-level test
applies `combination:C-OPER-ALT` term 1 from `0.5` to `0.75` through
`OperationApplyPanel`, verifies the manager row, property inspector,
pending-operation count, applied-operation count, and stale-solve reset. The
Rust test proves `terms.1.factor` applies without mutating the input model and
that whole `terms` editing remains an explicit deferred finding. The
Playwright R2 smoke previews the rendered factor editor without queueing so
the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-combination:C-OPER-ALT-term-1-factor` and confirmed
`load:L-200 x 0.75`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: load-case creation, arbitrary primitive-load
creation, imposed-displacement authoring breadth, combination basis editing,
combination term creation/deletion, broader algebra authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-96;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_factor_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 second sub-slice: load-case metadata editor (`TP-APP-R2-LOADMETA-001`)

The Load Cases manager now exposes selected load-case metadata editing for
`status` and `kind`. The editor queues structured `update_load` intents for
`Load.status` or `Load.kind` with explicit before/after values, unit `none`,
dimension `dimensionless`, local-session audit boundaries, and no
professional approval claim. The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
those two load metadata fields; combination `basis`/`terms` editing remains
explicitly deferred.

The app-level test applies `load:L-100` status from `preview_only` to `TBD`
through `OperationApplyPanel`, verifies the manager row, property inspector,
pending-operation count, applied-operation count, and stale-solve reset, then
verifies the `kind` editor previews `primitive_user_load -> TBD` without
applying a second operation. The Playwright R2 smoke checks the rendered
status/kind metadata controls without queueing so the solve/results/report
path remains on the unchanged fixture model. An in-app browser smoke at
`http://127.0.0.1:5175/` applied the rendered status edit and confirmed the
row showed `status=TBD`, pending operations returned to zero, and solve state
remained `not_started`.

Residuals remain in A4: load-case creation, arbitrary primitive-load creation,
imposed-displacement authoring breadth, full combination editing/algebra
authoring, Phase B unit picker/display retirement, and packaged-Tauri
saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-95;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_metadata_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 seventh sub-slice: viewport pipe endpoint picking (`TP-APP-R2-PIPEPICK-001`)

The viewport pipe form now has explicit endpoint-pick controls. Arming
`Pick` for the `from` endpoint and selecting a rendered viewport node fills
the pipe form's `from` field, advances to `to` picking, and still updates the
normal viewport/model-tree selection. Selecting a second rendered node fills
the `to` field and clears pick mode. Picking only supplies node references;
material, section geometry, non-zero `y_reference`, and provenance remain
explicit user-entered fields before Queue pipe can be enabled.

The app-level test picks `node:N-100` and `node:N-140` from viewport targets,
fills the remaining explicit pipe fields, queues and applies
`pipe:P-151` through `OperationApplyPanel`, and verifies the created pipe is
selected in the model tree, viewport layer, and inspector. The Playwright R2
smoke covers the rendered endpoint-pick controls before the unchanged
solve/results/report flow. An in-app browser smoke at
`http://127.0.0.1:5175/` confirmed `from=node:N-100`,
`to=node:N-140`, pick-mode advancement/clearing, and Queue pipe remaining
disabled until the remaining explicit fields are supplied.

Residuals remain in A3: canvas gesture capture beyond node drafting and
endpoint picking, component/rigid authoring, and broader editor coverage as
new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-94;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_pipe_endpoint_picking.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and the
same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 sixth sub-slice: canvas node drafting (`TP-APP-R2-CANVASNODE-001`)

The viewport canvas now captures a primary pointer gesture into the explicit
node form before any operation is queued. In WebGL mode, the handler raycasts
from the Three.js camera to the `y=0` drafting plane; in fallback/test mode,
it maps the pointer into the same bounded model-drafting plane. The captured
draft fills a visible editable node id (`node:V-001` style), label, and finite
x/y/z coordinates. The existing Queue node and Apply Operations path remains
the only mutation path.

The app-level test applies a canvas-drafted node through the structured
operation seam and verifies model-tree/inspector selection. The Playwright R2
smoke now exercises the real browser canvas click path without queueing the
draft, so the solve/report smoke remains on the unchanged fixture model. An
in-app browser smoke at `http://127.0.0.1:5175/` confirmed a canvas click
drafted `node:V-001`, finite coordinates, `y=0`, and enabled Queue node.

Residuals remain in A3: canvas gesture capture beyond node drafting
(pipe/connectivity and component/rigid authoring), rigid/component authoring,
and broader editor coverage as new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-93;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_canvas_node_drafting.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and the
same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 fifth sub-slice: explicit straight-pipe creation (`TP-APP-R2-CONNECTPIPE-001`)

The viewport editor now has an explicit straight-pipe form for user-entered
pipe id, label, endpoint nodes, material, outside diameter, wall thickness,
non-zero local `y_reference`, and provenance. The form queues a structured
`connect_pipe_run` intent with `field_path=pipe_segments` and applies it
through the same operation review/acceptance path as other local-session
edits. Legacy one-click pipe-run viewport gestures remain blocked because
they still carry only underspecified `viewport.connect_pipe_run` data.

The Rust `core/model_operations/operation_applier` crate and browser local
operation mirror both validate the explicit pipe payload: matching id,
`before=not_present`, project length unit, `dimension=length`, non-duplicate
pipe id, existing endpoint nodes/material, positive OD/wall quantities, and
non-zero `y_reference`. Applying the intent appends a new `pipe_segments`
record to the returned session model without mutating the input model in
place. The app test confirms `pipe:P-150` is created, selected in the model
tree and viewport selection layer, visible in the property inspector, and
recorded with local-session acceptance only.

Residuals remain in A3: true canvas raycast/gesture geometry capture,
rigid/component authoring, and broader editor coverage as new authoring
surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-92;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_explicit_straight_pipe_connectivity.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 first sub-slice: load-case primitive magnitude manager (`TP-APP-R2-LOADMGR-001`)

The desktop app now has a right-rail Load Cases manager over the invented
preview model. It surfaces load-case counts, primitive-load rows, combination
terms, and a focused primitive-load magnitude editor. Selecting
`load:L-100-P` exposes `primitive_loads.2.magnitude.value`; changing the
magnitude queues `op:load-manager-load:L-100-load:L-100-P-magnitude` as a
structured `update_load` intent. Applying the queued operation uses the
existing OperationApplyPanel, records local-session acceptance, clears stale
solve results, and leaves persistence to the Save local path.

Residuals remain in A4: load-case creation, load status/kind editing,
arbitrary primitive-load creation, imposed-displacement authoring breadth,
full combination editing/algebra authoring, unit picker/display retirement
after Phase B, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-91;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_manager_primitive_magnitude.md`,
the same-named record under
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/`,
and the same-named record under
`DEL-05-02_Load-case algebra engine/_run_records/`.

## 2026-06-11 — A8 first sub-slice: Playwright R2 smoke harness (`TP-APP-R2-PLAYWRIGHT-001`)

The desktop workspace now has a Playwright harness and root script
`npm run test:e2e:desktop`. The first smoke test runs the technical-preview
fixture through initial shell checks, local-only boundary checks, a nonblank
and animated Three.js viewport assertion, mechanics preview solve,
`result_rows=647`, viewport displacement-overlay availability, result
filtering/detail inspection for `pipe:P-120`, and deterministic report-packet
export checks.

Vitest is scoped to `src/**/*.test.{ts,tsx}` so the unit suite and Playwright
suite remain separate. Playwright local output directories are ignored.

Residuals remain in A8: authored create/edit -> solve -> report automation,
full manual SMOKE checklist parity, packaged Tauri saved-project solve smoke,
and CI browser provisioning policy.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-90;
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-11_playwright_r2_smoke_harness.md`.

## 2026-06-11 — A6 first sub-slice: viewport displacement overlay (`TP-APP-R2-DEFORMEDVIEW-001`)

The Three.js viewport now consumes the current mechanics result and renders a
review-only shape overlay after a solved preview run. The first slice uses
available `displacement_magnitude` rows by node, draws a teal overlaid
centerline/marker set, and exposes a toolbar status with node count, maximum
reported displacement magnitude, and an explicit boundary:
`scale=normalized_display_offset_not_physical_length`,
`vector_direction=TBD`, and `professional_claim=false`.

When no result is present the overlay status is `not started`; when the
current mechanics result is incomplete, such as the browser edited-model guard,
the overlay status is `blocked` and no deformed overlay is rendered.

Residuals remain in A6: true directional deformed shape once displacement
vectors exist, support-reaction visualization, stress/governing-ratio views,
and richer result-selection coupling.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-89;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_deformation_overlay.md`
and the same-named record under
`DEL-07-05_Results viewer/_run_records/`.

## 2026-06-11 — A5 first sub-slice: model-bound solve guard (`TP-APP-R2-SOLVEBOUND-001`)

The preview mechanics service now refuses to reuse bundled solved-result rows
for an edited model in browser fixture mode. Edited browser-session models
return a `MODEL_INCOMPLETE` mechanics result with zero result rows and an
explicit `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL` blocking
diagnostic. The unchanged fixture model still returns the bundled solved
fixture for browser preview workflows.

The Tauri backend path was verified as model-bound: direct
`run_preview_mechanics(Some(model))` and the solve-job registry both solve the
supplied edited model payload and publish result envelopes bound to the edited
`project.id`, not the bundled fixture id.

Residuals remain in the A5 row: full packaged-Tauri GUI smoke over a saved
edited project snapshot, richer incomplete-model UI copy, and broader
persisted non-fixture solve coverage as authoring surfaces grow.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-88;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-11_edited_model_solve_binding.md`,
the same-named record under
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_run_records/`,
and the same-named record under
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/_run_records/`.

## 2026-06-10 — A3 fourth sub-slice: session undo/redo checkpoints (`TP-APP-R2-UNDOREDO-001`)

The Apply Operations panel now exposes local-session Undo/Redo controls for
applied structured operations. Applying an operation records a checkpoint for
the previous session model and selection; undo restores that checkpoint,
moves the current model to redo, and clears stale solve results; redo restores
the undone model and clears stale solve results again. The history summary is
explicitly labeled local-session-only and saved-project-mutated=false.

Residuals remain in the A3 row: true canvas raycast/gesture geometry capture,
straight-pipe connectivity creation, and broader editor coverage as new
authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-87;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-10_session_undo_redo_checkpoints.md`
and the same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-10 — A3 third sub-slice: explicit node create operation (`TP-APP-R2-CREATENODE-001`)

The viewport editor now has an explicit node-geometry form for user-entered
node id, label, and finite x/y/z coordinates in the project length unit. The
form queues a structured `create_node` intent and applies it through the
existing operation seam; the browser local engine and Rust applier accept only
explicit node payloads, reject duplicate ids, and preserve the no-silent
conversion/default posture. The applied target becomes the active model-tree
and inspector selection. During browser smoke this tranche also fixed
viewport/operation-panel hit-test layout issues so the new form and apply
buttons are actionable in the live app.

Residuals remain in the A3 row: true canvas raycast/gesture geometry capture,
straight-pipe connectivity creation, undo/redo, and broader editor coverage
as new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-86;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-10_explicit_node_create_operation.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-10 — A3 second sub-slice: property-inspector inline validation (`TP-APP-R2-INLINEVALID-001`)

The property inspector now exposes validate-only feedback for draft editor
intents before queue/apply. The UI calls the existing structured-operation
validation seam and displays application status, schema/unit/before-state
states, diff rows, diagnostics, and a no-mutation/professional-boundary note.
This landed as an A3 editor UX sub-slice; broader editor coverage, undo/redo,
and true geometry-capture workflows remain in the A3 row.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-85;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`
and
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`.

## 2026-06-10 — A3 first sub-slice: viewport selection binding (`TP-APP-R2-VIEWSELECT-001`)

Viewport entity selection controls for loaded nodes, straight pipes,
supports, and component markers now drive shared selection, model-tree
active state, property-inspector binding, and viewport active highlight.
Residuals (remain in the A3 row): true canvas raycast/gesture geometry
capture, node/straight-pipe creation tools with explicit
coordinates/connectivity, undo/redo, and inline validation messages.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-84;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-10_viewport_selection_binding.md`
and the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`.

## 2026-06-10 — A2 landed: model-document persistence (`TP-APP-R2-PERSIST-001`)

Implemented under `DEC-019` (D-08 ruling): in-document semver authority,
application-service transform chain (migrate-in-memory-on-open /
persist-on-save), refusal semantics for newer/unsupported documents, store
v9 evidence-only migration ledger with pre/post hashes, and
validation-preflight evidence replacing the prior TBD marker. Open residuals
(remain in the A2 row): compatibility-window size (human ruling), explicit
"Migrate project" operation, sibling JSON-slot coverage.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-83;
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-10_model_document_schema_migration.md`
(with same-day `TASK_RUN_2026-06-10_*.md` records in the same folder).

## 2026-06-10 — A1 landed: apply-operation command path (`TP-APP-R2-EDITLOOP-001`)

New `core/model_operations/operation_applier` crate plus the
`apply_model_operation` and `validate_model_operation` Tauri commands
(desktop bridge now 14 commands, two on the mutating path) and an Apply
Operations panel. Inspector modify intents apply to the session model;
viewport gesture intents block pending A3 geometry capture; unit conversion
blocks pending D-01/Phase B.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-82;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-10_apply_operation_command_path.md`.

## 2026-06-10 — Decision packets D-01 and D-08 prepared and ruled

Both packets drafted and ruled same day: `D-01` → `DEC-018` (SI-canonical
with dual display catalog, as the packet proposed; Phase B unblocked);
`D-08` → `DEC-019` (per-document semver transform chain,
migrate-in-memory-on-open / persist-on-save, as the packet proposed; A2
unblocked). `D-10` packet drafted the same day, `AWAITING_RULING`. Packets
and state: `execution/_Coordination/_DECISIONS/_REGISTER.md`; rulings:
`SOFTWARE_DECOMP.md` §12. This completed items 1–2 of the plan's original
"first three tranches" sequence; item 3 (A2) landed the same day, above.
