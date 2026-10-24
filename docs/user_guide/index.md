---
doc_id: OPS-USER-GUIDE-SKELETON
doc_kind: documentation.user_guide
status: draft
created: 2026-05-09
deliverable_id: DEL-11-01
package_id: PKG-11
scope_item: SOW-033
---

# SWBPIPE User Guide

This guide describes the SWBPIPE 0.1.0 macOS desktop app and the repository
surfaces behind it: schema contracts, core modules, invented examples and
policy documents. [Current Desktop App](#current-desktop-app-010) lists what
the app does today and what it refuses. Current guide authority is the
accepted decomposition surface at `execution/_Decomposition/SOFTWARE_DECOMP.md` and the approved dependency
graph selected by `execution/_DAG/_LATEST.md`.

SWBPIPE is decision-support software for open, auditable piping
mechanics. Users supply project data, private libraries, rule packs, owner
requirements, protected or licensed design-basis values, and professional
judgment. Public guide content uses invented examples or placeholders only.
The project license is `MIT`, as selected by the human project authority on
2026-09-22 (`D-74`, superseding the 2026-06-03 selection); this is a project governance notice, not legal
advice or an engineering acceptance claim.

## 1. Current Scope And Authority Boundary

Use SWBPIPE to organize a 3D centerline/frame model, run open-mechanics
calculations where implemented, evaluate user-owned rule packs where inputs are
available, review diagnostics, and assemble auditable report records.

Keep these responsibilities separate:

| Responsibility | Current guide meaning |
|---|---|
| Mechanics solve | Software computation of displacements, rotations, forces, moments, reactions, stresses, and diagnostics from the recorded model basis. |
| User rule check | Software computation using user-supplied or lawfully imported private rule-pack data. |
| Human review | A competent person reviews the model, inputs, rule basis, diagnostics, reports, assumptions, and limitations before professional reliance. |
| Public examples | Invented or public-permissive demonstration content only. Public examples are not design bases. |

Any human acceptance record, if used in a future workflow, is external to
automatic solver and rule-pack output and must bind to specific reviewed
hashes.

## Current Desktop App (0.1.0)

SWBPIPE 0.1.0 is a desktop app for Macs with Apple silicon; §3.1 installs it.
It works on local data only. The window has four stages (Model, Loads,
Results, Review) and File, Edit and View menus. This section describes the
app at source commit `62c1efd75b` (2026-10-23).

A model solves on one of two routes:

- the **pressure-free route**, for a model without a pressure contract;
- the **exact pressure route**, for a model with the exact pressure contract
  `3.0.0/exact_pressure_v3`.

**What you can do today**, with invented data or your own private data:

- **Projects.** Use File › Open Model Document…, New Blank Project or New
  Local Project to start. Save Local Project writes the project to the local
  store (§3.2), and Open Local Project… reopens it.
- **Authoring.** Place nodes and pipes in the 3D viewport. Assign materials and
  sections, and add bends, expansion joints, supports, load cases and
  combinations. Each edit is a validated operation that you can review and
  undo (Edit › Undo Model Edit, Redo Model Edit).
- **Solving.** The pressure-free route covers linear and nonlinear supports,
  wind, seismic loads and load combinations. The exact route covers pressure,
  load/reference states and objective-connector joints.
- **Results.** Read displacements, element-local forces, reactions and
  stresses, each with its numerical-integrity standing, and view the deformed
  shape.
- **Rule packs.** Author a private rule pack and run it against results. A
  check whose inputs are missing is reported as `RULE_INPUTS_INCOMPLETE`,
  naming the missing inputs, and is not evaluated.
- **Libraries.** Import private material, section and component libraries.
- **Exports.** Both routes export result JSON and stress-neutral JSON, each
  only with explicit local-private intent. Pressure-free results also export
  the PCF and MBF package documents (redacted for downstream tools), a native
  package review manifest and a preview handoff JSON, among others.
  Outputs that must be byte-exact for a downstream tool (stress-neutral CSV,
  glTF review geometry, PCF text and MBF text) are withheld whenever
  redaction would alter them. Session geometry and result values are
  classified private, so these downloads are withheld in practice: the panel
  builds them but offers no download link.

**What the app refuses or does not offer today:**

- **Reports are local-private only, and pressure-free only.**
  - On a pressure-free result, **Render report** works for your own review
    when you tick "Include known private values in this local rendered
    report". The render then shows a preview and its canonical HTML SHA-256,
    and offers Save canonical HTML (local private) and Print / PDF. The
    document carries a "LOCAL PRIVATE REPORT … not for sharing or
    publication" notice. Without that selection the render runs in the
    public-report redaction context and stays `blocked=true` with no preview,
    because session inputs are classified private.
  - The rendered report carries the rule-pack name, id, version and checksum,
    each user-rule check's id, status and missing-input names, the software
    and solver versions, the generation time, solver mode and settings from
    a run manifest that verifies, and the result coordinate systems. The
    build or release id, the run's creation time and model-level coordinates
    are stated as "not recorded".
  - Every report surface refuses exact-route results with
    `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` (owner ruling 10A, 2026-10-21;
    `execution/PKG-04/DEL-04-07/Design/desktop-exact-outputs.md`).
  - **Save Report Package…** refuses every current result with
    `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` ("The report package is
    unavailable for fresh results.").

  This is PRD FR-REP-001 (Must) and the MVP item "basic report generation".
- **No model states, run history or comparison** between states or runs. A
  project holds one model and one run. Comparison works only within one run.
- **No rigid elements.** Valves, flanges, reducers and rigid or specialty
  components are analysed as plain pipe on the pressure-free route and
  refused on the exact route.
- **No model export to a file.** There is no save-model-to-file command. The
  handoff download is a preview document, not a schema handoff package.
  Pressure models export no handoff.
- **No design-knowledge entry or constraint validation in the app.** The
  Design Knowledge panel shows a compiled-in document.

## Accessibility Intent

Guide instructions must describe available keyboard navigation, labels, focus
behavior, visible units, missing-data diagnostics and usable result alternatives
where the corresponding interface provides them. Keep the DEL-07-06
accessibility/usability contract and DEL-11-01 acceptance criteria visible when
reviewing a user journey. Document known gaps and test evidence at their actual
reach; an intended control or guide statement does not establish accessibility
conformance or settle a human-selected target.

## 2. Current Repository Surfaces

The guide should refer to these current surfaces when explaining behavior:

| Area | Current surfaces | User-facing note |
|---|---|---|
| Model and units | `schemas/model.schema.yaml`, `schemas/units.schema.yaml`, `schemas/project_persistence.schema.yaml` | Project, model, unit, provenance, persistence, migration, and hash fields are schema-first. SCA-003 selects a local SQLite-backed project store/index; canonical JSON/JCS-compatible payload bytes remain the domain and interchange truth. |
| Analysis status | `schemas/analysis_status.schema.yaml`, `schemas/analysis_boundary.schema.yaml`, `docs/architecture/analysis_status_semantics.md` | Mechanics, user-rule, and human-review statuses remain separate. |
| Solver mechanics | `core/solver/frame_kernel`, `core/solver/straight_pipe`, `core/solver/linear_supports`, `core/solver/nonlinear_supports`, `core/solver/diagnostics`, `core/solver/sparse_direct`, `core/solver/performance_harness` | Current modules are bounded mechanics and diagnostics surfaces. The in-repository sparse skyline solver is selected and the bounded interactive preview/render path uses it by default while retaining dense scrutiny. Unmeasured tolerance entries, release-quality performance thresholds, and full workflow integration remain `TBD`. |
| Loads and stress recovery | `core/loads/primitive_loads`, `core/loads/user_loads`, `core/loads/load_case_algebra`, `core/loads/stress_recovery` | Loads and stress recovery use explicit unit-aware inputs. Code-specific combinations and acceptability checks are user/rule-pack data. |
| Libraries and provenance | `schemas/material.schema.yaml`, `schemas/section.schema.yaml`, `schemas/component.schema.yaml`, `core/library_import/provenance_checker.py`, `core/section_properties/calculator.py` | Materials, sections, and components carry provenance and review status. Public defaults do not include protected tables or proprietary catalog values. |
| Rule packs | `schemas/rule_pack.schema.yaml`, `core/rules/expression_evaluator`, `core/rules/completeness_checker`, `core/rules/rule_pack_lifecycle` | Rule packs are user-owned or invented examples. Required inputs, unit checks, provenance, checksums, and privacy state are explicit. |
| GUI workflow contracts | `schemas/viewport_editor.schema.yaml`, `core/gui/viewport_editor`, `core/gui/model_tree`, `core/gui/editors`, `core/gui/warnings`, `core/gui/solve_execution`, `core/gui/results_viewer`, `apps/desktop` | The Tauri/React/Vite macOS desktop app in `apps/desktop` is the user surface ([Current Desktop App](#current-desktop-app-010)); the `core/gui/*` modules are contract/reference modules (Python engines and a standalone Rust crate) that the app does not call or link. Its native smoke (`apps/desktop/native-smoke/`) and browser specs (`apps/desktop/e2e/`) exercise the app; no single run yet takes a model authored in the app through solve, review, save and reopen. Supported-release acceptance and other platforms remain `TBD`. |
| Reports and result envelopes | `schemas/report_generator.schema.yaml`, `schemas/report_sections.schema.yaml`, `schemas/results.schema.yaml`, `core/reporting/audit_manifest`, `core/reporting/report_sections`, `core/reporting/result_export`, `core/reporting/report_generator`, `core/reporting/report_renderer`, `core/reporting/protected_content_linter` | Report records preserve manifests, hashes, warnings, provenance, rule-pack references, limitations, and notices. A deterministic hash-bound single-file HTML renderer, a desktop preview and a native report-package save path exist in code, and the app renders a local private report for a pressure-free result after the local-private selection; a public render stays blocked, exact-route results are refused, and the report package refuses every current result ([Current Desktop App](#current-desktop-app-010)). Final styling/layout, deterministic hash-bound PDF policy, portable replay, and release-template integration remain `TBD`. |
| Privacy and export controls | `docs/security/local_first_storage_policy.md`, `docs/security/redaction_export_controls.md`, `docs/security/telemetry_policy.md`, `core/security/redaction` | Private data is local/user-controlled by default. Project storage is local-only and offline-capable. Telemetry is off by default. Redaction/export behavior is metadata-driven. |
| Export interoperability | `schemas/native_json_export.schema.json`, `schemas/stress_neutral_export.schema.json`, `schemas/pcf_export.schema.json`, `schemas/review_geometry_export.schema.json`, `schemas/export_adapter_sdk.schema.json`, `schemas/target_mapping.schema.json`, `core/handoff/*` | SCA-004 makes native JSON, the export of a `.mbf` model batch file, optional user-owned external-run evidence, stress-neutral CSV/JSON, conservative PCF, glTF review geometry, stable ID maps, loss reports, and adapter SDK surfaces explicit. These are bounded package surfaces: handoff evidence for external validation, not a validation outcome. |
| Interop and local analysis | `schemas/adapter_framework.schema.yaml`, `api/api_boundary_contract.yaml`, `schemas/local_fea_handoff.schema.yaml`, `docs/local_analysis/local_fea_handoff_guidance.md`, `core/adapters/framework`, `core/handoff/*` | Public API transport, endpoint syntax, plugin runtime/loading/signing, concrete target field coverage, and local FEA package format remain `TBD`. Adapters cannot bypass units, provenance, privacy, protected-content screening, diagnostics, rule sandboxing, report controls, or human-review boundaries. |

## 3. Setup And Project Storage

### 3.1 Installing the v0.1 macOS build

The v0.1 packaging policy (`DEC-057`) is one artifact for Apple Silicon Macs
(`aarch64-apple-darwin`): the SWBPIPE `.app` bundle in a zip archive, with a
SHA-256 checksum file beside it. Release publication is the owner's act. The
published location is `[OWNER: public repository name and location (DEC-057)]`.
The steps below apply once the owner has published a release there or given you
the two files directly. This guide is not evidence that a release exists.

The two files for version `0.1.0` are named:

- `SWBPIPE_0.1.0_aarch64-apple-darwin.app.zip`, the application;
- `SWBPIPE_0.1.0_aarch64-apple-darwin.app.zip.sha256`, its checksum, in
  `shasum` format (the 64-character hexadecimal digest, two spaces, the zip's
  file name).

**Requirements.** A Mac with Apple silicon. There is no Intel, Windows or Linux
build for v0.1.

**1. Verify the checksum before opening anything.** Put both files in one
folder, then in Terminal:

```bash
cd ~/Downloads        # the folder holding both files
shasum -a 256 -c SWBPIPE_0.1.0_aarch64-apple-darwin.app.zip.sha256
```

The command prints `SWBPIPE_0.1.0_aarch64-apple-darwin.app.zip: OK` when the
zip matches. Any other output means the file differs from the published one.
Delete it and do not open it. Where the release notes also state the digest,
check that it matches the `.sha256` file too. The release artifact record (see
[`BUILD_AND_RELEASE.md`](../BUILD_AND_RELEASE.md) §8.2) binds this digest to the
source commit and that commit's evidence sweep.

**2. Unzip.** Double-click the zip in Finder, or run:

```bash
ditto -x -k SWBPIPE_0.1.0_aarch64-apple-darwin.app.zip .
```

The archive holds `SWBPIPE.app`. Move it to `/Applications` or another folder
you choose.

**3. Open the app.** The v0.1 build is signed with the developer's Developer
ID certificate and notarized by Apple, with the notarization ticket stapled to
the app (owner decision of 2026-10-23, `D-06b`). Double-click `SWBPIPE.app`.
On the first launch of a downloaded copy, macOS asks you to confirm opening an
app downloaded from the internet and names it as checked by Apple for
malicious software; choose **Open**. If macOS instead reports that it cannot
verify the developer or that the app is damaged, do not open it. Delete it and
download it again.

The signature identifies the developer to macOS. The checksum you verified in
step 1 shows that your zip is byte-for-byte the published one, and the release
artifact record ties that digest to the source commit and its evidence sweep.
Repeat the checksum check for each new download.

**Updating.** Verify and unzip the new version as above, then replace the old
`SWBPIPE.app`. Your project store is kept, because it lives outside the app
bundle (below).

**Removing.** Delete `SWBPIPE.app`. Your project store stays in place until you
delete it yourself.

### 3.2 Where SWBPIPE keeps project data

The desktop app keeps its SCA-003 local project store in the macOS application
data folder for its bundle identifier, `com.chirality.swbpipe`:
`~/Library/Application Support/com.chirality.swbpipe/`. The store is
`openpipestress-projects.sqlite3` in that folder, with SQLite's `-wal` and
`-shm` sidecar files beside it. If a store from an earlier preview build exists
under the former identifier `org.openpipestress.technical-preview`, the app
copies it forward once on first launch and never overwrites an existing store.
Saved project packages, reports and exports go wherever you choose to save them.
Back up the store folder like any other private data. Nothing in it is sent
anywhere: the product is local-only.

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

In the desktop app the workflow is:

1. Start a project: File › Open Model Document…, New Blank Project or New
   Local Project. The project lives in the local SQLite store (§3.2). Storage
   roots for other operating systems and a portable project export/copy
   remain `TBD`.
2. Choose the units. New Blank Project opens a units dialog: SI (the
   default), US customary, or a custom unit per key (length, force, moment,
   pressure, stress, temperature, angle) from the unit catalog. Values are
   stored in the units entered, and the length unit is fixed once the project
   has nodes. The display units can be switched.
3. In the Model stage, create the centerline: place nodes and pipes in the
   viewport, then add bends, expansion joints and other components.
4. Assign materials and sections, and add supports and restraints.
5. In the Loads stage, add load cases and combinations. Imposed
   displacements are accepted on the exact route only; the pressure-free
   route refuses support-targeted loads.
6. Solve. Missing data blocks the solve with named findings in the Missing
   Data Blocking panel; fix them in the model.
7. Note unresolved assumptions in your own project records. The app has no
   slot for tags, notes or unresolved assumptions yet.

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
| `HUMAN_APPROVED_FOR_PROJECT` | Reserved for an external human acceptance record bound to the exact reviewed hashes and scope. It is not emitted by the solver or rule-pack evaluator. |

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

The desktop app renders a report only as a local private document for a
pressure-free result, after the local-private selection; exact-route results
are refused, and the report package refuses every current result (see
[Current Desktop App](#current-desktop-app-010)). The table below is the
intended report content. The rendered report now carries rule-pack
references, user-rule check outcomes, software and solver versions, the
generation time and the analysis settings a verified run manifest binds;
materials, sections and libraries are not yet in it.

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
or real secrets. A clean protected-content linter run is development review
evidence, not legal clearance.

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

Telemetry is off by default. SCA-004 adds explicit export-family surfaces for
native JSON packages, the export of a `.mbf` model batch file, optional
user-owned external-run evidence, stress-neutral CSV/JSON, conservative PCF, glTF review
geometry, stable ID maps, loss reports, and adapter SDK support. These exports
must remain deterministic, provenance-bearing, privacy-aware, and loss-reporting
where target limitations exist.

Export evidence must not bundle commercial solvers, bypass licenses, reverse
engineer protected formats, or embed proprietary examples. Export packages are
handoff evidence for external validation, not a validation outcome; validation
occurs in the user's accepted professional tools. Public API transport, endpoint
syntax, target field coverage, source confirmations, plugin loader mechanics,
redaction workflow details, and local FEA handoff package format remain `TBD`
unless a later accepted deliverable resolves them.

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

- the app refusals and gaps listed under
  [Current Desktop App](#current-desktop-app-010): reports only as local private
  documents and only for pressure-free results, no model states, run history or comparison, no rigid elements, no model export
  to a file, and no design-knowledge entry or constraint validation;
- publication of the v0.1 macOS Apple Silicon `.app` zip, which is the
  owner's act; the published location is still to be named (§3.1);
- exact dependency versions and an end-to-end run of a model authored in the
  app through solve, review, save and reopen;
- unmeasured per-kind and nonlinear-class tolerance entries, release-quality
  performance thresholds, and sparse-default hardening beyond bounded
  interactive preview/render evidence;
- canonical DEC-022 typed-AST rule grammar is fixed; a writable text syntax is
  deferred by DEC-037, while final evaluator/library integration and the
  private rule-pack storage workflow remain `TBD`;
- storage roots for platforms other than macOS (§3.2), product/DB migration
  tooling, and portable project export/copy workflow;
- public API transport, endpoint syntax, plugin loader, and concrete
  target-field coverage for the accepted adapter/export families;
- target field coverage, source confirmations, stable-ID carriage, and loss
  taxonomy for export packages where not already accepted;
- final report styling/layout, deterministic hash-bound PDF policy, portable
  report-package replay, release-template integration, and redaction UX beyond
  the bounded export surface;
- local FEA handoff package format and external-tool execution behavior;
- activation of the accepted public sanitized-export GitHub Actions path,
  numeric coverage floors, and remaining maintainer policy details;
- a contributor legal instrument if external intake is later activated,
  legal-review authority, security contact, release-label vocabulary,
  human-acceptance workflow, and jurisdiction-specific professional-practice
  wording; external contribution intake remains closed and maintainer quorum
  and release authority are currently held by the sole human project
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
| Export Package | A deterministic, provenance-bearing downstream package or sidecar set with explicit stable ID mapping, loss reporting, privacy metadata, diagnostics, and professional-boundary notices. |
| Diagnostic | A structured finding for missing data, units, provenance, solver state, rule readiness, IP boundary, privacy, report completeness, or unsupported behavior. |
| Provenance | Source, license or redistribution status, contributor certification, and review disposition for reliance-affecting data. |
| Human Acceptance Record | External hash-bound review record, if used. It is not emitted by the solver or rule-pack evaluator. |
