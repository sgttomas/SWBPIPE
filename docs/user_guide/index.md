---
doc_id: OPS-USER-GUIDE-SKELETON
doc_kind: documentation.user_guide
status: draft
created: 2026-05-09
deliverable_id: DEL-11-01
package_id: PKG-11
scope_item: SOW-033
---

# OpenPipeStress User Guide Skeleton

This guide is the initial user-facing structure for OpenPipeStress. It is
grounded in the current repository surfaces: schema contracts, bounded core
modules, invented examples, policy documents, and draft workflow contracts.

OpenPipeStress is decision-support software for open, auditable piping
mechanics. Users supply project data, private libraries, rule packs, owner
requirements, protected or licensed design-basis values, and professional
judgment. Public guide content uses invented examples or placeholders only.

## 1. Current Scope And Authority Boundary

Use OpenPipeStress to organize a 3D centerline/frame model, run open-mechanics
calculations where implemented, evaluate user-owned rule packs where inputs are
available, review diagnostics, and assemble auditable report records.

Keep these responsibilities separate:

| Responsibility | Current guide meaning |
|---|---|
| Mechanics solve | Software computation of displacements, rotations, forces, moments, reactions, stresses, and diagnostics from the recorded model basis. |
| User rule check | Software computation using user-supplied or lawfully imported private rule-pack data. |
| Human review | A competent person reviews the model, inputs, rule basis, diagnostics, reports, assumptions, and limitations before professional reliance. |
| Public examples | Invented or public-permissive demonstration content only. Public examples are not design bases. |

The software must not be described as certifying, sealing, authenticating, or
declaring engineering code compliance for a project. Any human acceptance
record, if used in a future workflow, is external to automatic solver and
rule-pack output and must bind to specific reviewed hashes.

## 2. Current Repository Surfaces

The guide should refer to these current surfaces when explaining behavior:

| Area | Current surfaces | User-facing note |
|---|---|---|
| Model and units | `schemas/model.schema.yaml`, `schemas/units.schema.yaml`, `schemas/project_persistence.schema.yaml` | Project, model, unit, provenance, persistence, migration, and hash fields are schema-first. SCA-003 selects a local SQLite-backed project store/index; canonical JSON/JCS-compatible payload bytes remain the domain and interchange truth. |
| Analysis status | `schemas/analysis_status.schema.yaml`, `schemas/analysis_boundary.schema.yaml`, `docs/architecture/analysis_status_semantics.md` | Mechanics, user-rule, and human-review statuses remain separate. |
| Solver mechanics | `core/solver/frame_kernel`, `core/solver/straight_pipe`, `core/solver/linear_supports`, `core/solver/nonlinear_supports`, `core/solver/diagnostics`, `core/solver/performance_harness` | Current modules are bounded mechanics and diagnostics surfaces. Production sparse-solver choices, final tolerances, and full workflow integration remain `TBD`. |
| Loads and stress recovery | `core/loads/primitive_loads`, `core/loads/user_loads`, `core/loads/load_case_algebra`, `core/loads/stress_recovery` | Loads and stress recovery use explicit unit-aware inputs. Code-specific combinations and acceptability checks are user/rule-pack data. |
| Libraries and provenance | `schemas/material.schema.yaml`, `schemas/section.schema.yaml`, `schemas/component.schema.yaml`, `core/library_import/provenance_checker.py`, `core/section_properties/calculator.py` | Materials, sections, and components carry provenance and review status. Public defaults do not include protected tables or proprietary catalog values. |
| Rule packs | `schemas/rule_pack.schema.yaml`, `core/rules/expression_evaluator`, `core/rules/completeness_checker`, `core/rules/rule_pack_lifecycle` | Rule packs are user-owned or invented examples. Required inputs, unit checks, provenance, checksums, and privacy state are explicit. |
| GUI workflow contracts | `schemas/viewport_editor.schema.yaml`, `core/gui/viewport_editor`, `core/gui/model_tree`, `core/gui/editors`, `core/gui/warnings`, `core/gui/solve_execution`, `core/gui/results_viewer` | These are current contract/support surfaces. A finished Tauri/React/Vite application shell, package manifest, runtime navigation, and screenshots remain `TBD`. |
| Reports and exports | `schemas/report_generator.schema.yaml`, `schemas/report_sections.schema.yaml`, `schemas/results.schema.yaml`, `core/reporting/audit_manifest`, `core/reporting/report_sections`, `core/reporting/result_export`, `core/reporting/report_generator`, `core/reporting/protected_content_linter` | Report records preserve manifests, hashes, warnings, provenance, rule-pack references, limitations, and notices. Final styling/layout and public export formats remain `TBD`. |
| Privacy and export controls | `docs/security/local_first_storage_policy.md`, `docs/security/redaction_export_controls.md`, `docs/security/telemetry_policy.md`, `core/security/redaction` | Private data is local/user-controlled by default. Project storage is local-only and offline-capable. Telemetry is off by default. Redaction/export behavior is metadata-driven. |
| Interop and local analysis | `schemas/adapter_framework.schema.yaml`, `api/api_boundary_contract.yaml`, `schemas/local_fea_handoff.schema.yaml`, `docs/local_analysis/local_fea_handoff_guidance.md`, `core/adapters/framework`, `core/handoff/*` | Adapter transport, external formats, plugin loader behavior, and local FEA package format remain `TBD`. |

## 3. Setup And Project Storage

End-user installation, binary packaging, desktop shell packaging, operating
system storage roots, and release channels remain `TBD`. Do not invent setup
commands from this guide. Until packaging is decided, use this guide as a map
of the documented workflow and current repository contracts.

For project storage:

- keep the SCA-003 local SQLite-backed project store/index in a
  user-controlled private location;
- treat canonical JSON/JCS-compatible payload bytes as the domain and
  interchange truth; SQLite tables and indexes are storage/projection details,
  not a public SQL contract;
- keep private material, section, component, and owner libraries outside public
  repository paths unless reviewed for public redistribution;
- keep private rule packs in user-controlled private paths unless intentionally
  contributed with documented rights;
- keep generated reports, screenshots, exports, diagnostics, and support
  bundles private by default;
- do not put secrets, credentials, private license material, private paths, or
  project values into public examples or public issue attachments.

The current symbolic path classes are documented in
[`docs/security/local_first_storage_policy.md`](../security/local_first_storage_policy.md).

## 4. Creating A Project

A project record should make identity, units, privacy posture, model references,
rule-pack references, report settings, diagnostics, and hashes explicit. The
canonical project/model structure is schema-first and should be validated
before solve or report workflows consume it.

Start each project with these slots:

| Slot | Required posture |
|---|---|
| Project identity | Project ID, model ID, and descriptive labels. Avoid private client or facility names in public examples. |
| Units | Unit system and unit-bearing quantities. Missing or incompatible units are findings, not defaults. |
| Provenance | Source, license or redistribution status, contributor, contributor certification, and review status for reliance-affecting data. |
| Privacy classification | Public example, private project data, private library data, owner data, or other explicit class. |
| Rule-pack references | ID, version, checksum, source notice, public/private marker, and required-input links where a user rule check is used. |
| Report settings | Manifest, warnings, assumptions, limitation, and notice slots. Final layout remains `TBD`. |

Use invented examples only for public learning. Current public example surfaces
include `examples/models/invented/` and `examples/rule_packs/invented_demo.yaml`.

## 5. Model And Library Data

The global model is a 3D centerline/frame model. A typical model needs:

- nodes with unit-aware coordinates and degree-of-freedom state;
- elements between stable node references;
- pipe runs, bend or branch symbols, rigid components, and specialty component
  references where applicable;
- material and section references with provenance;
- support/restraint definitions with explicit directions and properties;
- primitive load cases, user loads, imposed displacements, and combinations;
- result stations, diagnostics, assumptions, traceability links, and hashes.

Library data remains user-supplied or provenance-gated. Public guide content
must not publish protected standards text, standards tables, copied formulas,
material allowables, SIF/flexibility-factor tables, protected dimensional or
rating tables, proprietary vendor catalog values, owner standards, or private
project values.

When a required value is missing:

- solve-required missing data should surface as `MODEL_INCOMPLETE` or a
  `SOLVE_BLOCKING` diagnostic;
- rule-check-required missing data should surface as
  `RULE_INPUTS_INCOMPLETE` or a `RULE_CHECK_BLOCKING` diagnostic;
- missing source, redistribution, or review evidence should surface as a
  provenance or IP-boundary warning.

## 6. Modeling Workflow

The current user workflow is a skeleton for a future GUI and headless flow:

1. Select or create a local project store through application-service
   operations. SCA-003 selects a local SQLite-backed store/index; operating
   system roots and portable export/copy behavior remain `TBD`.
2. Define units and privacy posture before entering quantities.
3. Create the centerline: nodes, pipe runs, bends, branches, and component
   symbols.
4. Attach material, section, component, and support/restraint references.
5. Add load cases, user loads, imposed displacements, and combinations with
   explicit basis notes.
6. Validate model completeness, units, provenance, and privacy classification.
7. Record unresolved assumptions instead of converting them to hidden defaults.

The current viewport/editor contract represents camera, hover, selection,
drag, snap, view primitives, diagnostics, and command intents. Durable edits
should pass through application-service command boundaries rather than bypass
schema, unit, provenance, privacy, diagnostic, report, or human-review controls.

## 7. Solving Mechanics And Reading Status

A mechanics solve starts from a validated analytical model and explicit loads.
Current solver surfaces cover frame-kernel mechanics, straight-pipe element
adaptation, support/restraint behavior, nonlinear support active-state
classification, load application, load-case algebra, stress recovery, and
diagnostics.

Use the status vocabulary exactly:

| Status | User-facing meaning |
|---|---|
| `MODEL_INCOMPLETE` | Required physical data for solving is missing. Fix the model or preserve the finding. |
| `MECHANICS_SOLVED` | Numerical mechanics completed for the stated model and evidence set. It does not mean a user rule check is complete. |
| `RULE_INPUTS_INCOMPLETE` | Mechanics may be solved, but a rule pack lacks required user/code data, provenance, or private inputs. |
| `USER_RULE_CHECKED` | A user-defined rule pack evaluated available results and inputs. |
| `USER_RULE_FAILED` | A user-defined rule pack produced at least one failing result. |
| `HUMAN_REVIEW_REQUIRED` | Professional use still requires competent human review. |
| `HUMAN_APPROVED_FOR_PROJECT` | Reserved for an external human acceptance record bound to the exact reviewed hashes and scope. It is not emitted by the solver or rule-pack evaluator and is not software approval, certification, sealing, authentication, or code-compliance determination. |

Solver diagnostics should distinguish singular or invalid topology/restraints,
numeric invalidity, conditioning, nonconvergence, unsupported solver settings,
and unresolved `TBD` decisions. Production sparse solver selection, release
tolerances, final result-envelope integration details, and full solve UX remain
`TBD` where not already accepted.

## 8. Running User Rule Checks

Rule checks are not built-in protected standards checks. They are user-defined
computations over solver results and user-supplied rule-pack data.

A rule-pack workflow should:

- load or reference a rule pack by identity, version, checksum, source notice,
  privacy class, redistribution status, and review state;
- check required inputs for value, unit, dimension, provenance, redistribution,
  and review status;
- evaluate only through the sandboxed, deterministic, unit-aware rule boundary;
- preserve missing-input and provenance findings instead of inserting defaults;
- report rule outcomes separately from mechanics solve status and human review.

The invented demonstration rule pack is a software example only. Replace it
with project-specific, lawfully supplied, reviewed private rule-pack content
before real project use.

## 9. Reviewing Results

The results-review structure should cover:

- displacements and rotations;
- forces, moments, and reactions;
- stress components and stress summaries from open mechanics;
- user-rule ratios or findings where a user rule pack was supplied;
- units, dimensions, load case or combination basis, and result stations;
- diagnostics, warnings, assumptions, limitations, and unresolved `TBD`s;
- provenance and hash references for result and model basis.

Stress recovery in the public project covers open mechanics quantities. Code
category mappings, allowable comparisons, SIF/flexibility-factor data, owner
criteria, and project acceptability thresholds are user/rule-pack content, not
public defaults.

## 10. Reports And Audit Records

Reports are structured review artifacts. A report should include or reference:

| Report slot | Expected content |
|---|---|
| Software and solver basis | Software version or commit basis, solver version stamp, and schema versions where available. |
| Model and input manifest | Model hash, input manifest, load cases, combinations, units, and relevant assumptions. |
| Provenance summary | Material, section, component, rule-pack, owner-data, and source notes with review state. |
| Results | Displacements, rotations, forces, moments, reactions, stresses, and user-rule results where available. |
| Diagnostics and warnings | Missing data, unit issues, provenance warnings, nonlinear findings, IP-boundary warnings, and unresolved `TBD`s. |
| Rule-pack references | Rule-pack ID, version, checksum, source notice, public/private marker, completeness status, and redaction status. |
| Limitations and notices | Model scope, unsupported cases, validation status, data boundaries, and human-review-needed language. |

Public report templates and public examples must not embed protected standards
content, private rule-pack payloads, private project values, proprietary values,
or real secrets. A clean protected-content linter run is review evidence only.
It is not legal clearance or professional acceptance.

## 11. Review Workflow

Before relying on a report or export, users should review:

1. Project identity, units, model scope, and privacy classification.
2. Geometry, supports/restraints, loads, combinations, and assumptions.
3. Material, section, component, owner, and rule-pack provenance.
4. Solve diagnostics, missing-data findings, nonlinear findings, and warnings.
5. User rule-pack required inputs, checksum, source notice, and outcomes.
6. Report manifest, hashes, limitations, unresolved `TBD`s, and redaction state.
7. External human review records, if any, with hash-bound scope and limitations.

Any changed model, rule pack, result, input manifest, or report basis invalidates
prior hash-bound human acceptance references for the changed content.

## 12. Export, Privacy, And Local Analysis

Export and sharing workflows are review-gated by default. Public/shared exports
should include only public metadata, invented public examples, or content with
documented redistribution rights and review disposition. Private values should
be redacted, omitted, or blocked according to export context and metadata.

Telemetry is off by default. Import/export formats, public API transport,
redaction workflow details, plugin loader mechanics, and local FEA handoff
package format remain `TBD` unless a later accepted deliverable resolves them.

Local shell/solid FEA is a specialized handoff path, not the default global
analysis method. Handoff packages should preserve units, provenance, target
mapping, unsupported-behavior findings, diagnostics, report boundaries, privacy
classification, and limitations.

## 13. Troubleshooting And Warnings

Use warnings to preserve uncertainty:

| Warning or finding | Typical cause | User response |
|---|---|---|
| `SOLVE_BLOCKING` | Required physical input is missing or invalid. | Add source-backed model data or keep the model in an incomplete state. |
| `RULE_CHECK_BLOCKING` | Mechanics can solve, but rule-pack data is missing. | Supply lawful private rule-pack inputs or record the rule check as incomplete. |
| `PROVENANCE_WARNING` | A value has missing or weak source evidence. | Add source, license/redistribution, contributor, and review information. |
| `ASSUMPTION_WARNING` | A user/model assumption needs review. | Record owner, scope, affected objects, downstream effect, and review state. |
| `NONLINEAR_WARNING` | Active-state or convergence uncertainty exists. | Review support definitions, load steps, convergence facts, and limitations. |
| `IP_BOUNDARY_WARNING` | Protected, proprietary, private, or unclear data may be in a public/report path. | Stop publication/export, quarantine if needed, and request human review. |

Warnings should be carried into reports and exports when they affect result
interpretation, rule-check readiness, privacy, or review.

## 14. Known Limitations And Current TBDs

The following remain explicit limitations or open decisions:

- end-user install/package steps and release channels;
- exact dependency versions and final GUI application scaffold;
- production sparse numerical library, tolerance policy, and release thresholds;
- final rule expression grammar/library and private rule-pack storage workflow;
- operating-system storage roots, product/DB migration tooling, and portable
  project export/copy workflow;
- public API transport, endpoint syntax, adapter formats, and plugin loader;
- final report styling/layout, report preview/export runtime, and redaction UX;
- local FEA handoff package format and external-tool execution behavior;
- CI provider, coverage thresholds, release signing, and maintainer policy
  details;
- license and release authority decisions still owned by the human project
  authority.

Unknowns must remain `TBD` until resolved by the appropriate accepted
deliverable or human governance record.

## 15. Glossary

| Term | Meaning |
|---|---|
| Open Mechanics | Public mechanics implemented by the solver: geometry, stiffness, loads, stress recovery, units, diagnostics, and reports. |
| Protected Standards Data | Standards-body text, tables, figures, protected examples, copied code formulas, allowables, SIF/flexibility tables, and protected dimensional tables. |
| User-Supplied Code Data | Licensed or project-specific values, formulas, allowables, SIFs, flexibility factors, load combinations, owner requirements, and interpretations entered by the user. |
| Centerline Model | A 3D line-element representation of piping centerlines and components for global analysis. |
| Rule-Pack Check | A user-defined expression or comparison applied to solver outputs and user-supplied inputs. |
| Diagnostic | A structured finding for missing data, units, provenance, solver state, rule readiness, IP boundary, privacy, report completeness, or unsupported behavior. |
| Provenance | Source, license or redistribution status, contributor certification, and review disposition for reliance-affecting data. |
| Human Acceptance Record | External hash-bound review record, if used. It is not emitted by the solver or rule-pack evaluator. |
