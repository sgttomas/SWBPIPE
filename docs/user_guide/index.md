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

SWBPIPE is under development toward the [v1.0 release target](../RELEASE_TARGET_v1.0.md).
This guide retains versioned implementation descriptions, including the 0.1.0
desktop basis below; it is not evidence of a published release or completion of
the target. Consult the [release limitations](../RELEASE_LIMITATIONS_v1.0.md)
and each section's source basis when assessing a workflow.

This guide describes the macOS desktop app and the repository
surfaces behind it: schema contracts, core modules, invented examples and
policy documents. [Current Desktop App](#current-desktop-app-010) lists what
the app does at its stated revision and what it refuses. Current commitments
are in the PRD, its adopted companions and local deliverable ScopeOfWork files;
`Work/` and `deliverable.yaml` record technical detail and dependency conditions.

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

- **Projects.** Use File › New Blank Project (an empty model) or Open Model
  Document… to start. Save Local Project writes the project to the local
  store (§3.2), and Open Local Project… reopens it; ⌘S is Save Local Project.
  Save as New Local Project stores the model on screen as a local project of
  its own; it does not start an empty model. Quitting (⌘Q) or closing the
  window while the model has unsaved edits (the toolbar shows "Edited") asks
  first: Save Local Project and quit, discard the edits and quit, or Cancel.
  While a dialog is open, ⌘Q and the close button do nothing; finish or
  cancel the dialog first. If a save is still running when you quit again,
  you choose between waiting for it and quitting without saving. Quitting
  from the Dock or at logout does not ask yet.
- **Authoring.** Place nodes and pipes in the 3D viewport. Assign materials and
  sections, and add bends, expansion joints, supports, load cases and
  combinations. Each edit is a validated operation that you can review and
  undo (Edit › Undo Model Edit, Redo Model Edit). Undo keeps the last 25
  edits of this session; it is not kept when you quit or open another project.
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
  package review manifest and a preview handoff JSON, among others. The MBF
  text is a placeholder record subset that CAEPIPE does not import (§12).
  Outputs that must be byte-exact for a downstream tool (stress-neutral CSV,
  glTF review geometry, PCF text and MBF text) are withheld whenever
  redaction would alter them. Session geometry and result values are
  classified private, so these downloads are withheld in practice: the panel
  builds them but offers no download link.

**Projects saved before model document 0.9.0.** Model document 0.9.0 adds optional operating-condition sets (`condition_sets`, a pipe segment's `condition_set_ref` and a load case's `condition_use`), a single-DOF support's `direction`, a result-state subtraction's `difference_basis` and `subtrahend_support_state`, the `absolute_sum` and `srss` combination bases with combinations as operands (these now solve, §6.2), a load case's hydrostatic `test_condition`, the `snubber` support family and response `spectra` (DEL-02-01 Design `model-document-0.9.0.md` §1-§2). A document may carry these members and open, edit and save; until the slices that give them meaning land, a solve refuses each one it cannot yet read by name, and nothing is dropped. The pressure-free solve now resolves a case's condition temperature, added weight and wind exposure (§6.1). The desktop opens only the current model-document version, so a project or saved model state stored at 0.8.0 is refused by name when you open it, and nothing is migrated. A 0.8.0 document whose `schema_version` alone is changed to 0.9.0 has the same meaning in 0.9.0; making that change is your re-authoring step, and the stored-document export still writes a refused project's bytes for it. Recorded runs keep their own format and existing read and compare behaviour; this change adds no model-document-version gate for recorded runs. Existing integrity checks and retained-precision/source-reader restrictions still apply. Run history is listed for the active project's id, so the runs of an older project are reached only through a current (0.9.0) project with the same project id. Comparing an older run with a newer one keeps the usual eligibility rules.

**Projects saved before model document 0.8.0.** Model document 0.8.0 adds a reducer's optional eccentric offset and the optional `model_settings.shear_model` beam-theory choice (§1.1 of DEL-02-01 Design `model-document-0.8.0.md`). The desktop opens only the current model-document version, so a project or saved model state stored at 0.7.0 is refused by name when you open it, and nothing is migrated. A 0.7.0 document whose `schema_version` alone is changed to 0.8.0 has the same meaning in 0.8.0; making that change is your re-authoring step. Recorded runs keep their own format and existing read and compare behaviour; this change adds no model-document-version gate for recorded runs. Existing integrity checks and retained-precision/source-reader restrictions still apply. Run history is listed for the active project's id, so the runs of an older project are reached only through a current (0.8.0) project with the same project id. Comparing an older run with a newer one keeps the usual eligibility rules.

**What the app refuses or does not offer today:**

- **Reports are local-private only.**
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
  - On an exact-route result (pressure-2 or pressure-source-1) that Export
    Result JSON admits, **Render report** adds the producer's limitations,
    the stress headline of each section basis, the numerical standing and,
    where present, the resolved load/reference states. The report JSON packet
    still refuses exact-route results with
    `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`.
  - Save Report Package… saves a local private package for a pressure-free
    result, or for an exact-route result that Export Result JSON admits,
    after its own local-private selection.

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
| Hanger sizing | `core/analysis/hanger_sizing`, `schemas/hanger_sizing_result.schema.json`, `schemas/hanger.schema.yaml`, `core/runner/headless/src/bin/openpipestress-runner.rs` (`size-hangers`), `docs/theory/hanger_sizing.md` | Sizes named supports from a private hanger library on the pressure-free route and publishes a closed, local-private result document through the headless runner only ([§7.1](#71-hanger-sizing-with-the-headless-runner-pressure-free-route)). There is no desktop display, CSV table or recorded-run save yet, and the exact route is refused. |
| Reports and result envelopes | `schemas/report_generator.schema.yaml`, `schemas/report_sections.schema.yaml`, `schemas/results.schema.yaml`, `core/reporting/audit_manifest`, `core/reporting/report_sections`, `core/reporting/result_export`, `core/reporting/report_generator`, `core/reporting/report_renderer`, `core/reporting/protected_content_linter` | Report records preserve manifests, hashes, warnings, provenance, rule-pack references, limitations, and notices. A deterministic hash-bound single-file HTML renderer, a desktop preview and a native report-package save path exist in code, and the app renders a local private report for a pressure-free result after the local-private selection; a public render stays blocked, eligible exact-route results can be rendered and packaged, and the report JSON packet still refuses them ([Current Desktop App](#current-desktop-app-010)). Final styling/layout, deterministic hash-bound PDF policy, portable replay, and release-template integration remain `TBD`. |
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

1. Start a project: File › New Blank Project (an empty model) or Open Model
   Document…. The project lives in the local SQLite store (§3.2). Storage
   roots for other operating systems and a portable project export/copy
   remain `TBD`.
2. Choose the units. New Blank Project opens a units dialog: SI (the
   default), US customary, or a custom unit per key (length, force, moment,
   pressure, stress, temperature, angle) from the unit catalog. Values are
   stored in the units entered, and the length unit is fixed once the project
   has nodes. The display units can be switched.
   After Create, the new blank opens once it has been stored, usually at
   once. If you change the model on screen before then (an applied edit,
   undo or redo), your change is kept and the blank is not opened. The status
   line then shows `NEW-BLANK-SUPERSEDED` with the blank's name and id. The
   blank is stored: open it from List Local Projects. Selecting or typing
   into a form does not stop the blank from opening.
   New fields start in the project's units:
   - A load case's solve temperature starts in the temperature unit (a legacy
     `C` is shown as `degC`).
   - Seismic gravity starts in `<length>/s^2`: `m/s^2` in a metre project.
     The unit catalog has no `mm/s^2`, `in/s^2` or `ft/s^2` yet, so in other
     projects the field starts unselected with a notice naming the symbol.
     Choose `m/s^2` there.
   - Component stiffness and area start in units composed from the keys:
     `<force>/<length>`, `<moment>/<angle>` and `<length>^2`. For example, SI
     gives `N/m`, `N*m/rad` and `m^2`. A composition the catalog lacks starts
     unselected with a notice.
   - A field whose key is missing is refused by name, and nothing is assumed.
     Choose its unit in the property inspector, or set the key in Project
     units on the Project page. The grid has no unit selector: it refuses such
     a cell and points to the inspector.
   - Stored values keep their units. Changing a project unit changes only
     where new entries start.
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

### 6.1 Operating-condition sets (model document 0.9.0)

An operating-condition set states a line's temperatures (and other values) once
and lets load cases use them by key. The design is DEL-02-01 Work
`model-document-0.9.0.md` §1.

**What you enter (in the model document).** Each `condition_sets[]` record has an
`id`, a non-blank `provenance` and, as needed: `conditions[]` (each with a `key`
such as `OPE1` or `HYDRO`, and an absolute `temperature`, `pressure` or
`contents_density`); an `installation_temperature` (the stress-free
temperature); an `added_weight` (`magnitude` as a force per length and the
`direction` it acts along, `global_x`, `global_y` or `global_z`); and `exposure`
marks (`wind`, `snow_ice`, `wave`). Each pipe names its set in
`condition_set_ref`. A load case's `condition_use` names what it takes from the
sets: `temperature` (a key), `added_weight: true`, `wind_exposure: true`
(and `pressure` or `contents_density`, see below). Every value is yours, in a
catalog unit; the product supplies no temperature, weight, exposure, wind input
or set.

**What the pressure-free solve does with it.** Before anything else in the
solve, each case that uses conditions is resolved into ordinary per-case
records, which then solve exactly as if you had typed them:

- `temperature`: for every pipe, in pipe order, a thermal primitive in kelvin
  with ΔT = condition temperature − the set's installation temperature;
- `added_weight`: for every pipe whose set has an added weight, a weight
  primitive with that magnitude and direction, added to the case's own weights
  (a set without an added weight adds none);
- `wind_exposure`: the case's exposed pipes are the pipes whose set is marked
  `wind: true`; the case's own `equivalent_static.wind` supplies the pressure,
  shape factor and direction.

Generated records have ids `load:condition-temperature:…` and
`load:condition-added-weight:…` (length-prefixed case and pipe ids) and a
provenance naming the set, key and method; an authored id equal to one is the
usual `DUPLICATE_ID`. The case's elastic basis is unchanged: condition
temperatures drive thermal strain only.

**The temperature difference.** Each temperature is read as the decimal you
typed, converted to kelvin exactly and subtracted exactly, then rounded once
(§1.5.1). 20.3 degC − 20.1 degC gives exactly the binary64 nearest 0.2. A
temperature change typed directly keeps its existing conversion; for degF and
degR this can differ in the last bit from the condition route (a typed 7 degF
gives 3.8888888888888893 K, 77 degF − 70 degF from conditions gives
3.888888888888889 K). Absolute temperatures must lie within |value| ≤ 10⁶ in
their own unit with a decimal exponent of at least −24.

**Refusals by name** (the case, pipe, set, key and quantity are named; no pipe is
skipped and no value assumed): a pipe without a set
(`OPERATING_CONDITION_SET_MISSING`), a set without the key
(`OPERATING_CONDITION_KEY_MISSING`), a condition without a temperature
(`OPERATING_CONDITION_VALUE_MISSING`), a set without an installation temperature
(`OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING`), a case that also
carries a thermal primitive (`LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT`) or its
own exposed pipes or spans, an authored empty list included (`LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT`), and a
wind-exposure use without `equivalent_static.wind`
(`LOAD_CASE_CONDITION_WIND_INPUT_MISSING`). If no pipe is wind-exposed, the
existing `EQUIVALENT_STATIC_INPUT_MISSING` names the empty exposure. A set and
its values are checked whether or not a case uses them.

**Not yet available.** `contents_density` uses are refused
(`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`) until their consumer is designed;
a `pressure` use on a pressure-free document is refused
(`LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT`); every use on a
document with a pressure contract is refused
(`LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED`) until the exact route's resolution
(S4). A hydrostatic `test_condition` and a skewed support `direction` keep their
own refusals. The retained Direct and headless routes and the independent
source readers still decline condition sets, references and uses by name; the
ordinary solve publishes. `design`, `peak`, `snow_ice`, `wave` and
`contents_density` are carried but not used by the solve in 0.9.0.

**Editing through the operations (S3a).** The engine's structured operations
author condition sets, pipe assignments and case uses, one record or one member
at a time, alone or in an atomic batch (all applied, or nothing). The desktop's
condition-set manager, pipe field and load-manager Conditions control are a
later slice (S3b); until then these operations are reached through the
operation seam.

- `create_condition_set`, `update_condition_set` and `delete_condition_set` write
  one whole set as you entered it. A set's shape, units, temperature range,
  provenance and key uniqueness are checked when the operation is proposed, with
  the solve's own codes and messages. A set that any pipe still names is not
  deleted (`OP-CONDITION-SET-IN-USE`, naming each pipe); clear or change those
  pipes first.
- `assign_condition_set` sets a pipe's `condition_set_ref`, or clears it. A name
  that is no set is refused (`OPERATING_CONDITION_SET_UNRESOLVED`).
  `connect_pipe_run` does not take a set; assign it as its own step.
- `set_load_case_condition_use` and `clear_load_case_condition_use` state or
  remove a case's use, and `create_load_case` may carry one. On a pressure-free
  document `temperature`, `added_weight: true` and `wind_exposure: true` are
  admitted. `contents_density` (`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`) and
  `pressure` (`LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT`) are
  refused, and so is every use on a document with a pressure contract
  (`LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED`).

You may state a case's use before every pipe has its set. After each of these
operations the engine runs the solve's own resolution on the result and reports
what the solve would refuse. A pipe without a set, a missing key, value or
installation temperature, a case that also carries a thermal primitive or its own
exposed pipes, a wind-exposure use without `equivalent_static.wind`, and a wind
exposure that resolves to no pipe are shown as warnings, with the solve's code
and message. The edit is applied, and the solve still refuses that case by name
until you supply the input.

Anything else the solve refuses blocks the edit, with one exception: a refusal
the document already had before the edit, and that the edit cannot affect. A
snubber or a skewed support elsewhere in the model, for example, does not stop
you from creating a set, assigning one or stating a case's use. Such a refusal
is shown as information, with the solve's code and message and a note that the
document already carried it, and the edit is applied. It still blocks when the
edit reaches it: when you edit that case itself, when it is a case that uses
conditions and you change any set or a pipe's assignment, or when the edit
makes the refusal appear more often. A refusal the edit introduces, one that
names no particular member, and a use the route refuses always block. The solve
still refuses the document by name until the member is resolved; an applied edit
never means the model solves.

Nothing is defaulted at any step: no set is created or suggested, no pipe is
given a set it was not assigned, and no temperature, weight, exposure or wind
input is filled in. A hydrostatic `test_condition`, a support `direction` and
a combination's `difference_basis` and `subtrahend_support_state` stay refused
at authoring; absolute sums, SRSS and combination operands are authored (§6.2).
These are mechanics inputs only: no code operating, design or occasional load
definition is implied, and the results carry no code-compliance claim.

The current viewport/editor contract represents camera, hover, selection,
drag, snap, view primitives, diagnostics, and command intents. Durable edits
should pass through application-service command boundaries rather than bypass
schema, unit, provenance, privacy, diagnostic, report, or human-review controls.

### 6.2 Load combinations: absolute sums, SRSS and combinations of combinations

A combination is a row operation over solved results; it never adds a solve. Model
document 0.9.0 has five bases (DEL-05-02 Work `absolute-sum-srss-combinations.md`;
theory note `docs/theory/load_combinations.md`):

| Basis | Inputs | Each published row |
|---|---|---|
| `mechanics` | terms (factor and operand) | Σ fᵢ·xᵢ, signed |
| `result_state_subtraction` | `minuend_id`, `subtrahend_id` | x_m − x_s, signed; not an equilibrium state |
| `range_envelope` | `operand_ids`, `mode` | the selected operand row; not one state |
| `absolute_sum` | terms | Σ \|fᵢ·xᵢ\|, unsigned |
| `srss` | terms | √Σ (fᵢ·xᵢ)², unsigned |

**Inputs.** A term names exactly one of `load_case` and `combination`, with a
finite factor; no operand may repeat. A subtraction's or envelope's id names the
load case when one has that id, and a combination otherwise; to name a combination
that shares a load case's id, use a term's `combination`. Combinations may name
combinations declared later in the list, but never in a cycle
(`LOAD_COMBINATION_OPERAND_CYCLE`). A declared operating comparison
(`difference_basis`) names load cases only.

**Method.** The absolute sum is the exact sum of |factor × value|, rounded once.
The SRSS rounds each factor × value once and takes the correctly rounded square
root of the exact sum of their squares. Both are unsigned, never −0.0, and do not
depend on term order. A value outside the binary64 range is withheld by name
(`LOAD_COMBINATION_ROW_WITHHELD`, "load-case algebra refused"). Displacement and
support-force and -moment magnitudes are the norm of the combination's own
components: for an SRSS that is the SRSS of the operands' magnitudes; for an
absolute sum it bounds every signed combination's magnitude from above. Support
groups publish whole or not at all. Combinations are evaluated in dependency order
(earliest in the list first among those ready), and the results follow that order.

**Assumptions and gates.** A combination counts each fixed load (a consuming
constant-effort support's force, a connector's or tie rod's installed reference)
κ times, its fixed-load count, computed exactly from the operand tree:
- a `mechanics` combination in such a model publishes only when κ is within 2⁻⁴⁶
  of 1 (`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` or
  `COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE` otherwise; an operand with an
  undefined κ withholds it too);
- an absolute sum or SRSS in such a model publishes only when every operand's κ is
  exactly 0 (`CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND`,
  `INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND`). Author each seismic
  operand as a difference, (base + EQx) − base, which has κ = 0;
- every load case a combination reaches must share one modulus basis (and, on the
  exact route, one resolved stiffness);
- in a model with nonlinear supports a `mechanics` combination is withheld
  (`NONLINEAR_COMBINATION_REQUIRES_SOLVE`) unless it is one solved load case with
  factor exactly 1 plus one or more combinations with κ exactly 0 (for example
  sustained + SRSS of differences). Its rows say "a solved state plus derived
  differences or bounds; not an equilibrium state", and each operand's standing is
  recorded;
- a combination naming a withheld combination is withheld whole
  (`LOAD_COMBINATION_OPERAND_WITHHELD`).

**Limitations.** Combination rows are derived inspection results: they carry no
numerical check of their own and never enter headlines, maxima or numerical
eligibility. Unsigned rows carry no sign or direction. The desktop shows them in the
Results table, the navigator and the support and displacement summaries labelled
"absolute sum, unsigned" or "SRSS, unsigned", keeps them out of maxima and minima,
and never draws them as a deformed shape or a colour state. The retained routes and
the CAEPIPE SI writer decline these bases and combination operands by name. The
load manager's editors for them are later work. Which combinations a design code
requires, with which factors, is your rule-pack data: SWBPIPE forms the
combinations you author and does not determine code compliance.

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

### 7.1 Hanger sizing with the headless runner (pressure-free route)

Hanger sizing chooses variable spring hangers and constant-effort supports for
the supports you name, from your own private hanger library. Today it runs only
from the command line, through the `openpipestress-runner size-hangers` verb.
The desktop app does not offer it yet. This section describes that verb. The
method is explained in the [hanger sizing theory note](../theory/hanger_sizing.md),
and the governing design is DEL-04-11
`execution/PKG-04/DEL-04-11/Work/hanger-sizing.md` (§2–§8, with the publication
contract and the headless entry in §8.3).

**What the verb does.**

- It works on the **pressure-free route only**. A model that carries a
  `pressure_contract` is refused with `HANGER_SIZING_EXACT_ROUTE_PENDING`.
- It sizes each candidate support you name against the records of one hanger
  library, then publishes one result document, `sized` or `refused`.
- It never changes your model. Every solve it runs uses a modified copy of the
  model that is never saved. It offers no way to apply a selection.
- Preload is not applied. The model's own solve does not yet apply a variable
  spring's cold load as a preload (DEL-04-11 Design §11). If you enter a
  selected spring into the model yourself, the model's own cases will not
  reproduce the loads the sizing verification reports, and each entered spring
  load is warned as not applied (`SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`).

**Getting the runner.** Build `openpipestress-runner` from a checkout of the
repository as the [headless runner reproduction guide](../validation_manual/headless_runner_reproduction.md)
describes. Below, `openpipestress-runner` stands for the built binary.

**The input.** The input is one JSON document with exactly three members. Each
is required, and no other member is admitted:

```json
{"request": {"model": …, "materials": […]}, "sizing": <sizing request>, "library": <hanger library>}
```

- `request` is the solve request: your model document and its materials
  array. Its hash identifies the exact model and materials the result was
  sized from.
- `library` is one hanger library document (`schemas/hanger.schema.yaml`,
  format `1.0.0` or `1.1.0`). It must pass the library import boundary as a
  private hanger library and carry a `hanger_library.library_id`.
- `sizing` is the sizing request, described next.

Nothing is defaulted. The runner supplies no value for any member, and each
member must be present even when it is an empty list. In the sizing request:

| Member | What you state |
|---|---|
| `candidates` | A non-empty list of `{support, family_policy}`, each support once. `support` is the id of a support in your model. `family_policy` is `variable` (variable spring records only), `constant` (constant-effort records only) or `variable_else_constant` (constant-effort records only when no variable record fits). |
| `weight_case` | The load case whose reactions give each candidate's hot-load basis W. Sizing does not check that it holds only weight: every load it carries enters W. |
| `weight_support_state` | Which nonlinear-support state W is formed in. Only `own_solve`, the weight case's own converged state, is admitted. |
| `design_operating_case` | The operating load case in which a hot-designed spring carries W. |
| `check_operating_cases` | Further operating load cases checked against the same setting. It may be an empty list, but it must be present. |
| `method.load_design` | `hot` (the spring carries W in the design operating case) or `cold` (it carries W in the cold condition). |
| `method.variation_reference` | The load each case's variation is a ratio of: `operating_load` (that case's load) or `design_load` (W). |
| `method.variation_limit` | The largest variation you accept, as a positive finite ratio (for example `0.25` for 25 %). |
| `method.travel_allowance_m` | A length in metres, at least zero, added at each end of the travel span. Zero is used only when you enter it. |
| `method.selection_rule` | `smallest_capacity` or `least_variation`, for choosing among the records that fit. |
| `solver_mode` | `sparse_interactive` or `dense_scrutiny`. Every solve the sizing runs uses it. |

Every named case must be a load case of the model, not a combination. A case
with a `test_condition` is refused, and a case may be named in only one role. A
candidate must restrain exactly one translational DOF (UX, UY or UZ), with no
skew direction, must not be nonlinear, a snubber or an anchor, must share its
node and DOF with no other support, and must not be referenced by another model
object. Its present family and hanger data are not read
([theory note: Candidates](../theory/hanger_sizing.md#candidates)).

**How the sizing works, briefly.** The theory note gives each step in full.

- **The installed origin.** Every position is measured from the model's drawn
  geometry, taken as the installed position at which each spring is set. A
  candidate's force acts along the positive axis of its DOF; nothing infers
  "up". A variable spring set to F_cold at the installed position carries
  F(u) = F_cold − k·u. Settled-position and erection-offset settings are not
  offered ([the installed position](../theory/hanger_sizing.md#the-installed-position-and-the-spring-law)).
- **Hot-load basis and positions.** W is each candidate's reaction in the
  weight case with every candidate held rigid. The cold-condition and
  operating positions come from solves with every candidate replaced by its
  force W. A W that is not positive refuses the request
  ([hot-load basis](../theory/hanger_sizing.md#hot-load-basis)).
- **Cold and hot load designs.** Hot design sets F_cold so that the spring
  carries W in the design operating case; cold design sets it so that the
  spring carries W in the cold condition
  ([variable spring loads](../theory/hanger_sizing.md#variable-spring-loads)).
- **Travel and variation tests.** The travel span runs from the least to the
  greatest of the installed position, the cold position and every operating
  position, and the required travel is the span plus the allowance at each
  end. A variable record must carry a stiffness on the candidate's DOF, a
  working load range and a travel range; its loads must be positive, its load
  envelope within its working load range, the required travel within its
  travel range, and every variation within your limit. A constant-effort
  record is tested on its data, on W within its working load range and on
  travel. Records are tested in a fixed order, and the first failure is the
  record's reason. For a candidate with no fit or a fallback, `failure_counts`
  gives the number of records that failed each test
  ([travel](../theory/hanger_sizing.md#travel-span-and-allowance),
  [fit tests](../theory/hanger_sizing.md#fit-tests-and-selection)).
- **Selection and the fallback.** Among the records that fit,
  `smallest_capacity` picks the smallest working-load maximum, then the
  smallest largest variation over the operating cases, then the smallest
  `hanger_id`. `least_variation` picks the smallest largest variation, then the
  smallest maximum, then the smallest `hanger_id`. Under `variable_else_constant`,
  constant-effort records are tested only when no variable record fits, and
  the candidate's outcome is then `selected_by_fallback`. A candidate with no
  fitting record is `no_record_fits`, and nothing is put in its place.
- **Library records.** Only a format `1.1.0` record with a
  `working_load_range` can be selected. A `1.0.0` record always fails the first
  test (no working load range)
  ([library records](../theory/hanger_sizing.md#library-records)).

**Verification and what `sized` means.** After selection, the sizing solves
the weight and operating cases again with the selections in place: a selected
variable record as a spring of its rate plus the force F_cold, a constant-effort
selection as the force W, and a candidate with no fit also as the force W
([verification](../theory/hanger_sizing.md#verification)).

- `status: "sized"` means the sizing algorithm completed. It does not mean that
  every candidate has a record, and it is not an engineering acceptance.
- `result.verification_standing` is `unconditional` when every candidate has a
  selection. It is `conditional` exactly when some candidate's outcome is
  `no_record_fits`: the verification then assumed that candidate to be the
  force W, which applying the selected records alone does not reproduce. Each
  candidate's `verification_assumption` states the role the verification gave
  it.
- Each candidate's `verification.failed_tests` lists the tests that fail with
  the verified values. The selection stands as sized: nothing is re-sized
  automatically, so re-running after changes is your choice.
- `result.disclosures` names nonlinear supports whose state differs between
  the weight solve and the design operating solve
  (`HANGER_SIZING_SUPPORT_STATE_DIFFERS`), or between the weight solve and the
  cold-condition solve (`HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS`). Judge
  whether W is representative when either appears.
- Loads and positions inherit every premise of the solves that produced them.
  The result claims no hanger's suitability for a service and no code
  compliance; its fixed `limitations` text says so.

**Local-private output.** A sizing result names records of your private library,
so the whole document is local-private output, whether sized or refused. The
runner prints a local-private control record whose `payload` member is the
result document.

- Without `--explicit-local-private-intent`, the output is blocked: the control
  record carries `"payload": null` and one `LOCAL_PRIVATE_INTENT_REQUIRED`
  finding, nothing is written to `--output`, and the exit status is 1.
- With the flag, the document is the payload, the record carries one
  `PRIVATE_LOCAL_ALLOWED_WITH_WARNING` warning, and `--output` receives the same
  text the runner prints.

Keep these outputs out of public locations, as for any private project data
(§3.2).

**Exit statuses and limits.**

| Status | Meaning |
|---|---|
| 0 | Sized, with the local-private flag given. |
| 1 | Refused, or blocked because the flag was not given. A refused document is still written to `--output` when the flag is given. |
| 2 | The invocation or input is unusable and no document is formed: for example an unsupported argument, `--solver-mode` (the request's `solver_mode` is the only source of the mode), an input file that cannot be read, or an input over 64 MiB. |

The input is limited to 64 MiB (67,108,864 bytes), checked before it is parsed.
A larger input is reported on standard error as `HANGER_SIZING_INPUT_TOO_LARGE`
with status 2. This is the runner's own limit, not an engineering value.

**Refusals.** A refused document has no table. Its `refusals` list gives each
finding as `{code, names, message}`, with `names` naming the paths, cases,
supports or solves concerned. The checks run in stages: the input text, then
the input's shape, then the library and the sizing request, then the model's
cases and candidates, then the solves. Each stage lists every finding it makes
together, and a later stage runs only when the earlier ones pass, so fix every
finding listed and run again. The codes you are most likely to meet:

| Code | Cause | What to do |
|---|---|---|
| `HANGER_SIZING_INPUT_NOT_ADMITTED` | The input is not JSON, repeats a key, or holds a number outside the canonical profile (not finite, or an integer beyond ±(2⁵³ − 1)). `names` gives the kind and the path; the value is never quoted. | Correct the input at that path. |
| `HANGER_SIZING_INPUT_SHAPE_INVALID` | The input is not exactly `{request: {model, materials}, sizing, library}`. | Add the missing members and remove any others named. |
| `HANGER_SIZING_LIBRARY_NOT_ADMITTED` | The library fails the import boundary, or carries no `library_id`. The message lists the boundary's finding codes and paths. | Correct the library document. |
| `HANGER_SIZING_REQUEST_INVALID` | A sizing-request member is missing, unknown or of the wrong kind. `names` gives each path. | Supply every member, using the allowed values above. |
| `HANGER_SIZING_EXACT_ROUTE_PENDING` | The model carries a `pressure_contract`. | Sizing is available on the pressure-free route only. |
| `HANGER_SIZING_VARIATION_LIMIT_INVALID`, `HANGER_SIZING_TRAVEL_ALLOWANCE_INVALID` | The limit is not a positive finite ratio, or the allowance is negative or not finite. | Enter a valid value. |
| `HANGER_SIZING_CASE_UNKNOWN`, `HANGER_SIZING_TEST_CASE_UNSUPPORTED`, `HANGER_SIZING_CASE_ROLES_OVERLAP` | A named case is not a load case of the model, carries a `test_condition`, or is named in more than one role. | Name each load case once, without test-condition cases. |
| `HANGER_SIZING_CANDIDATE_UNKNOWN`, `_DUPLICATE`, `_DOF_UNSUPPORTED`, `_SKEW_UNSUPPORTED`, `_FAMILY_UNSUPPORTED`, `_DOF_SHARED`, `_REFERENCED` (each prefixed `HANGER_SIZING_CANDIDATE`) | A named support does not exist, is named twice, or does not meet the candidate rules above. | Correct the support in the model, or remove it from `candidates`. |
| `HANGER_SIZING_ID_COLLISION` | The model already uses an id `load:hanger-sizing:<support>`, which sizing reserves for a candidate's force. | Rename the model object. |
| `HANGER_SIZING_LOAD_NOT_POSITIVE` | A candidate's rigid reaction W is not positive along its DOF. Sizing never flips a sign. Every such candidate is named. | Remove the named candidates and run again. |
| `HANGER_SIZING_DERIVED_SOLVE_FAILED` | One of the sizing's solves was refused, published a blocking diagnostic or lacked a needed result. `names` gives the solve's label (such as `S_W`), and the message carries the solve's diagnostics. A singular system after a candidate is replaced by a force shows that the candidate carried stability, not only load. | Read the solve's diagnostics and correct the model or the candidate list. |

**Examples.** These use the maintained invented fixtures in
`fixtures/results/`; run them from the repository root.

A sized request, written to a file of your choice:

```bash
openpipestress-runner size-hangers \
  --input fixtures/results/hanger_sizing_input_two_span.json \
  --output two_span_result.json \
  --explicit-local-private-intent
```

The exit status is 0. The output's `payload` equals
`fixtures/results/hanger_sizing_result_two_span.json`. It sizes one candidate,
`support:B`, with the `variable` policy, hot design, `variation_limit` 0.25 and
`travel_allowance_m` 0.002, over the weight case `load:W` and the operating
cases `load:O1` and `load:O2`. The candidate's `w_n` is 5000.0, its outcome is
`selected`, with record `VS-200-L` and `f_cold_n` 5197.458880641297; the
`verification_standing` is `unconditional`, its verification has no failed
tests, and `disclosures` is empty. `result.solves` lists the seven solves
(`S_W`, `S_C`, `S_O1`, `S_O2`, `V_C`, `V_O1`, `V_O2`), each with its derived
document's hash and the state of the model's nonlinear support `support:C-lift`.

The same run without the flag exits 1, prints a control record with
`"payload": null`, and writes no file. The input can also be given on standard
input (`--input -`, or no `--input`).

A refused request:

```bash
openpipestress-runner size-hangers \
  --input fixtures/results/hanger_sizing_input_refused.json \
  --output refused_result.json \
  --explicit-local-private-intent
```

The exit status is 1. The output's `payload` equals
`fixtures/results/hanger_sizing_result_refused.json`: `status` is `refused`,
and its one refusal is `HANGER_SIZING_CANDIDATE_UNKNOWN`, naming
`support:missing`, a candidate the model does not contain.

**Checking a published document.** The runner has no `read` route for sizing
documents. The two readers that check them, the Rust reader
(`core/analysis/hanger_sizing/src/reader.rs`) and an independent, test-only
Python verifier (`tests/verifiers/hanger_sizing_evidence.py`), are exercised
by the repository's maintained tests and are not user commands. Each reports a
standing as passed, failed or not checked; none states that the mechanics are
verified:

- *closure*: the document matches the closed schema
  `schemas/hanger_sizing_result.schema.json`, with its cross-field rules and
  fixed texts;
- *binding*: each input supplied hashes to the hash the document records;
- *arithmetic consistent with published positions*: the loads, tests,
  selections and verification are recomputed from the published W and
  positions. A document whose positions were invented consistently would still
  pass;
- *derivation* and *reproduction* (Rust reader only): the derived documents
  are rebuilt, and the whole sizing re-run from the bound inputs.
  Reproduction is the only standing that ties the positions to the bound
  model, and it is claimed only for the same engine build on the same
  platform.

When you read a document yourself, check that `status`, `route` and
`inputs.sizing_request` are what you submitted, that `inputs.library.library_id`
names your library, and then each candidate's `outcome`, `selected`,
`verification.failed_tests` and the `verification_standing`. The hashes in
`inputs` are SHA-256 digests of the inputs' canonical text
(`openpipestress_jcs_ijson_v1`). They identify the inputs; they do not
authenticate where the document came from.

**Not available yet.**

- No desktop request form, hanger table display or application of a
  selection.
- No CSV hanger table.
- No native save of a sizing result into a recorded analysis run. A sizing
  result is not a recordable run.
- No sizing on the exact pressure route (`HANGER_SIZING_EXACT_ROUTE_PENDING`).
- No variable spring preload in the model's own solve.
- No forming W in the design operating case's support state (only `own_solve`).

The method is the mechanics and arithmetic stated in the design and the theory
note; it uses no supplier catalogue, supplier selection procedure or standards
text. The variation limit, travel allowance and selection rule are your inputs,
and SWBPIPE ships no value for any of them. Engineering judgement on the inputs, the cases, the
library and the result remains yours, and competent human review is required
before professional reliance (§1, [`docs/PROFESSIONAL_BOUNDARY.md`](../PROFESSIONAL_BOUNDARY.md)).

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

The desktop app renders a report only as a local private document, after the
local-private selection, for a pressure-free result or an exact-route result
that Export Result JSON admits, and the report package is saved locally for
the same results (see
[Current Desktop App](#current-desktop-app-010)). The table below is the
intended report content. The rendered report now carries rule-pack
references, user-rule check outcomes, software and solver versions, the
generation time and the analysis settings a verified run manifest binds;
it also records the provenance of materials, sections, components and supports and lists the library records the model references, with the library store's findings on them.

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

The desktop `.mbf` text is not yet a CAEPIPE model: it writes a placeholder
record subset (`UNIT`, `NODE`, `PIPE`, `SUPPORT`, `LOADCASE` lines) that CAEPIPE
does not import, so do not hand it to CAEPIPE. A real SI MBF writer for a pilot
subset exists in `core/handoff/caepipe_mbf/si_writer.py` for the developer
correlation pilot (DEL-17-05 `Work/caepipe-correlation-pilot.md`); the desktop
export does not use it yet.

Export evidence must not bundle commercial solvers, bypass licenses, reverse
engineer protected formats, or embed proprietary examples. Export packages are
handoff evidence for external validation, not a validation outcome; validation
occurs in the user's accepted professional tools. Public API transport, endpoint
syntax, target field coverage, source confirmations, plugin loader mechanics,
redaction workflow details, and local FEA handoff package format remain `TBD`
unless a later accepted deliverable resolves them.

Units in the PCF, `.mbf` and local FEA exports:

- **Missing project unit.** An export refuses by name when a project unit it
  writes is missing, and offers no download:
  - `.mbf`: length, force or temperature (`MBF-PROJECT-UNIT-MISSING`);
  - PCF: length (`PCF-PROJECT-UNIT-MISSING`);
  - local FEA: length, temperature, or force or stress where no result row of
    that kind is referenced (`LOCAL-FEA-PROJECT-UNIT-MISSING`).

  Set the key in Project units on the Project page. A missing length in a model
  with nodes cannot be set; re-author the model.
- **Lengths in millimetres.** PCF and `.mbf` lengths convert exactly to
  millimetres from `m`, `mm`, `in` (25.4 mm) and `ft` (304.8 mm). Node
  coordinates convert from the project length unit. PCF OD and wall thickness
  convert from their own units.
  - A unit without a routine exact conversion is refused by name:
    `MBF-LENGTH-UNIT-UNSUPPORTED` or `PCF-LENGTH-UNIT-UNSUPPORTED`.
  - So is a value that is not finite: `MBF-LENGTH-OUT-OF-RANGE` or
    `PCF-LENGTH-OUT-OF-RANGE`.
  - In either case the download is withheld.
- **Local FEA result units.** The local FEA handoff declares its stress and
  force units from the result rows they label. The governing stress maximum
  sets the stress unit; in the demo, that is `Pa`. It refers only to rows in
  that unit, and converts none.
  - A row in another unit is left out and named in a
    `LOCAL-FEA-RESULT-ROW-UNIT-OMITTED` warning. Read that row from the result
    in its own unit.
  - If no row governs and the rows disagree, the handoff is refused
    (`LOCAL-FEA-RESULT-UNIT-INCONSISTENT`).

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
  documents, and the report JSON packet only for pressure-free results, no model states, run history or comparison, no rigid elements, no model export
  to a file, and no design-knowledge entry or constraint validation;
- hanger sizing only through the headless runner on the pressure-free route,
  with no desktop display, CSV table, recorded-run save, exact-route sizing or
  variable spring preload in the model's own solve (§7.1);
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
