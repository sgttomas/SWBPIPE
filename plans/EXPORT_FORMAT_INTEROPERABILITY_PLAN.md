# Export Format Interoperability Plan

## Purpose

This plan defines how an open source piping design prototyping application should handle export formats so it can become useful in the current piping design and pipe stress analysis ecosystem without becoming a code-checking program itself.

The application's job is to create physically plausible, inspectable, well-structured piping models. Downstream tools remain responsible for stress analysis, code compliance, reporting, and professional design acceptance.

The practical export strategy is therefore:

1. Keep a clean internal model that is richer than any one vendor format.
2. Export deterministic solver input packages.
3. Preserve provenance, stable IDs, units, coordinates, and assumptions.
4. Run import-and-analysis harnesses where a licensed downstream tool supports scripted use.
5. Report exactly what was exported, omitted, approximated, or delegated to the downstream tool.

## Market and Strategy Context

The useful market position is not "an open source replacement for commercial pipe stress software." That would put the project directly against mature tools whose value is bound up with code compliance, liability, QA practice, project standards, support contracts, and organizational trust.

The better position is "a local-first open model authoring and solver-handoff workbench."

In that position, the application serves the phase before formal stress verification:

- Rapidly create and revise physically plausible piping layouts.
- Preserve engineering assumptions in a structured model.
- Let humans and agents inspect the same model state.
- Generate deterministic handoff files for established downstream tools.
- Record semantic losses, unsupported features, and target-specific assumptions.
- Help users iterate before they commit time in a commercial stress or plant-design environment.

This framing makes the application complementary to CAEPIPE, CAESAR II, AutoPIPE, Plant 3D, OpenPlant, CADWorx, and related tools rather than adversarial to them. Those tools remain the destination for formal analysis, code-specific checks, project deliverables, and professional review.

The strategic gap is in the pre-solver layer. Existing commercial tools are strong at trusted final analysis and enterprise delivery, but they are not optimized for open-ended local prototyping, agent-readable engineering state, transparent provenance, reproducible export packages, or community-extensible adapters. Open modeling tools exist, but they do not generally solve the full piping-specific path from early layout through stress-model handoff.

This project should therefore compete on:

- Openness
- Local execution
- Inspectability
- Fast iteration
- Stable model identity
- High-quality export reporting
- Scripted validation against external tools
- Extensible adapters rather than closed workflow capture

The initial adoption path is likely to start at the edges: students, small firms, consultants, owners' engineers, open-source engineering users, early design teams, and organizations that need credible first-pass piping alternatives before investing effort in final commercial-tool modeling. Broader professional adoption depends less on replacing incumbents and more on proving that the exported handoff packages are repeatable, intelligible, and useful to people who already live with those incumbents.

## Current Strategic Decision

CAEPIPE should be the first validation target.

That does not mean the project depends on CAEPIPE, bundles CAEPIPE, claims CAEPIPE compatibility in every workflow, or performs CAEPIPE's code-checking role. It means the first serious downstream validation loop should target CAEPIPE because it has a comparatively accessible cost structure and a documented text model input path that can be exercised from scripts.

The primary target format should be CAEPIPE MBF, not PCF.

Reasoning:

- MBF is CAEPIPE's model batch file format and is documented as text input.
- CAEPIPE can import an `.mbf`, analyze it, produce a `.csv` result file, and close when invoked through the executable.
- MBF is closer to CAEPIPE's actual stress model than PCF is.
- PCF remains valuable for plant-design and isometric interoperability, but CAEPIPE's PCF path uses translator behavior, mapping databases, defaults, and approximations that make it less deterministic as the first validation backbone.

The near-term priority becomes:

1. Native open JSON package.
2. CAEPIPE MBF export profile.
3. CAEPIPE scripted validation harness.
4. Stress neutral CSV/JSON package.
5. PCF export for broader interoperability.
6. Geometry exports and additional adapters.

## Non-Goals

The application must not:

- Perform piping-code compliance checks.
- Claim that a generated design is code-compliant.
- Embed proprietary solver logic or copied proprietary examples.
- Reverse engineer protected binary formats.
- Bypass downstream license restrictions.
- Treat successful export as professional engineering acceptance.
- Bundle CAEPIPE or any other commercial solver.

The project may ask the user to provide a licensed local solver path and may run that solver as an external program when the user's license and operating environment permit it.

## System Boundary

The application owns:

- Model topology
- Geometry
- Connectivity
- Components
- Supports and restraints
- User-supplied material and section data
- User-supplied operating/design cases
- Export profiles
- Export reports
- Import/run harnesses
- Result parsing for regression and handoff validation

The downstream solver owns:

- Stress analysis
- Code selection and code-specific calculations
- Allowables and code compliance reports
- Solver-specific interpretation of model options
- Final engineering results
- Professional review workflow

Some solver input fields may name a piping code or analysis option because the target solver requires that input. In this project, those fields must be treated as pass-through target configuration, not as local code-checking logic.

## Reference Basis

The plan is based on publicly available vendor documentation and should be refreshed when the supported CAEPIPE version changes.

- CAEPIPE documents importing model data from `.mbf` text files and states that command-line use of `caepipe.exe sample.mbf` produces `sample.csv` in the same folder.
- CAEPIPE documents export of model data to MBF and PCF, and export/print of model and result data to CSV or text.
- CAEPIPE's PCF translator documentation shows that PCF-to-CAEPIPE conversion can depend on external mapping databases, configurable OD/thickness attributes, support translation behavior, and defaults such as anchoring free ends.
- CAEPIPE's current public product page advertises an evaluation version with a 20-row model limit for a limited time, which is useful for smoke tests but not a substitute for a full validation license.

Primary public references:

- <https://www.sstusa.com/docs/users_manual/import_mbf_print.htm>
- <https://www.sstusa.com/docs/users_manual/export_data_from_caepipe.htm>
- <https://www.sstusa.com/pdfs/PCF.pdf>
- <https://www.sstusa.com/caepipe.php>
- <https://www.sstusa.com/caepipe-enhancements.php>

## Core Principles

- Use documented, intentionally supported exchange formats and public interfaces.
- Keep the internal model richer, cleaner, and more explicit than any export format.
- Treat every export as potentially lossy.
- Make validation stricter than file writing.
- Preserve stable IDs for lines, nodes, components, supports, equipment interfaces, and load/design cases.
- Keep units, coordinate systems, sign conventions, and assumptions explicit.
- Generate human-readable and machine-readable export reports.
- Keep target-specific exporters isolated behind adapter interfaces.
- Record target version, export profile version, and harness version for every generated package.
- Prefer deterministic text formats for validation workflows.
- Allow users to supply local mappings for licensed downstream tools.

## Canonical Internal Model

Before targeting any external format, define a canonical internal piping model. Exporters should consume this internal model rather than reading directly from UI state, solver state, or display geometry.

The internal model should represent at minimum:

- Project metadata
- Coordinate system and vertical axis
- Units and unit family
- Line numbers and line groups
- Nodes and connectivity graph
- Pipe segments
- Bends and elbows
- Tees and branches
- Reducers
- Valves
- Flanges
- Specialty components
- Rigid elements or component weight stand-ins where needed
- Equipment nozzles and terminal connections
- Supports, anchors, guides, stops, springs, snubbers, and restraints
- Materials and material labels
- Nominal sizes, outside diameters, schedules, wall thicknesses, and corrosion allowances where supplied
- Temperatures, pressures, fluid density, insulation, lining, and component weights where supplied
- User-defined load/design cases
- Target-solver pass-through options
- Component metadata and user notes
- Validation messages and unresolved assumptions

This model should have a versioned JSON schema. That schema becomes the stable contract for exporters, tests, sample files, community adapters, and regression fixtures.

## Stable Identity Rules

Stable IDs are not optional. They are how the application proves that exported solver results correspond to the originating model.

Required IDs:

- `project_id`
- `model_id`
- `line_id`
- `node_id`
- `element_id`
- `component_id`
- `support_id`
- `material_id`
- `section_id`
- `case_id`
- `export_id`

Rules:

- IDs must be stable across re-export when the semantic object has not changed.
- Exporters may emit solver-specific names or node numbers, but must retain a mapping back to canonical IDs.
- The export package must include an ID map for every target file.
- Result parsers must report results against canonical IDs where possible.
- If a target format cannot carry the ID directly, the exporter must preserve the mapping in sidecar files.

## Priority Export Targets

### 1. Native Open JSON Package

Create a native open export package first. This is the canonical debug, interchange, archival, and adapter-input format for the project.

Recommended contents:

- `model.json`
- `schema.json`
- `validation_report.md`
- `validation_report.json`
- `export_manifest.json`
- `id_map.json`
- Optional lightweight geometry preview such as `model.glb`

The JSON package should be complete enough to recreate the model inside the prototyping application and to support third-party exporters without requiring the application runtime.

### 2. CAEPIPE MBF

Implement CAEPIPE MBF as the first solver-specific export profile.

This is the main validation target because it supports a direct text-file handoff into CAEPIPE and can be exercised through a repeatable external run harness.

The first MBF exporter should support:

- Heading/title metadata
- CAEPIPE version/profile declaration
- Units and vertical-axis options
- Solver target options as explicit pass-through fields
- Material records
- Pipe section records
- Load records for the initial supported cases
- Layout records for straight pipe, bends, tees, reducers, flanges, valves, and rigid elements where needed
- Anchors
- Guides
- Line stops
- Hangers or spring placeholders only when the semantics are understood
- User-supplied weights and weight densities in the form CAEPIPE expects
- Stable ID comments or sidecar mappings where MBF cannot preserve IDs natively

The first MBF exporter should reject or warn on:

- Unsupported support/restraint behavior
- Missing OD or wall thickness where required
- Ambiguous material properties
- Missing weight for components represented as weight-bearing items
- Unsupported branch/SIF details
- Unknown coordinate-system or vertical-axis assumptions
- Load cases that cannot yet be represented faithfully
- Any target option that would imply local code-checking responsibility

The MBF writer should be deterministic: the same canonical model and export profile should produce byte-stable output except for explicitly declared timestamp fields.

### 3. CAEPIPE Scripted Validation Harness

Build a harness that runs CAEPIPE as an external local executable when the user provides a valid executable path and has the legal right to use it.

Recommended run directory:

```text
case_001/
  model.json
  model.mbf
  model.csv
  export_manifest.json
  export_report.json
  export_report.md
  id_map.json
  caepipe_run.json
  caepipe_stdout.txt
  caepipe_stderr.txt
```

Recommended Windows command shape:

```powershell
& "C:\CAEPIPE\caepipe.exe" "C:\cases\case_001\model.mbf"
if (!(Test-Path "C:\cases\case_001\model.csv")) {
  throw "CAEPIPE did not produce CSV output"
}
```

The actual executable path must be user-configured. On macOS, the application should treat CAEPIPE execution as an external Windows-bound or compatibility-layer workflow unless a supported native path exists. The project should document Parallels, VMware, Wine, or remote Windows runners only as user-owned execution environments, not bundled dependencies.

The harness should verify:

- The MBF file was written.
- CAEPIPE was invoked with the expected file.
- A CSV result file was produced.
- The CSV can be parsed.
- Expected model sections are present in the CSV.
- Expected node/component/support counts survive the run.
- Units and coordinate conventions are consistent with the export profile.
- Results can be correlated to canonical IDs where possible.
- Warnings, missing sections, or parse failures are recorded.

The harness should not judge code compliance. It should judge export fidelity, run repeatability, and downstream acceptance of the model.

### 4. Stress Neutral CSV/JSON Package

Define a stress-oriented neutral package controlled by this project. This is not a vendor format and should not be presented as a replacement for a commercial solver input deck.

Recommended contents:

- Node table
- Element table
- Component table
- Restraint/support table
- Equipment/nozzle table
- Material table
- Section table
- Load/design case table
- Units metadata
- ID map
- Validation report

Recommended formats:

- CSV for inspection and spreadsheet review
- JSON for structured import/export

This package should support review, debugging, community adapter development, and comparison across target exporters.

### 5. PCF

PCF should remain a priority, but not the first validation backbone.

PCF is useful because:

- It is piping-specific.
- It is recognized in many plant design and stress handoff workflows.
- It can represent useful component, line, coordinate, and isometric data.
- It can support eventual export into multiple downstream ecosystems.

PCF is not ideal as the first CAEPIPE validation target because:

- CAEPIPE's PCF conversion path may rely on translator mappings.
- OD and wall thickness may be read from attributes or an external mapping database depending on the source PCF.
- Support behavior can depend on PCF attributes, SKEY values, mapping databases, or defaults.
- Free ends may be anchored by translator behavior because PCF does not fully describe external pipeline/equipment connections.
- Some PCF components are simulated as rigid elements, equivalent pipes, concentrated masses, or other CAEPIPE-side approximations.

The first PCF exporter should support a conservative subset:

- Straight pipe
- Elbows and bends
- Tees
- Reducers
- Flanges
- Valves
- End connections
- Line numbers
- Nominal size
- Outside diameter and wall thickness attributes where supported
- Basic spec/material labels
- Coordinates
- Component identifiers
- Stable ID sidecar mapping

The PCF export report must list support/restraint semantics separately when the target path cannot preserve them reliably.

### 6. Geometry Exchange

Geometry export is useful for visualization, coordination, clash review, and downstream model comparison, but it should not be treated as stress-ready handoff by itself.

Candidate formats:

- GLB/glTF for lightweight 3D review and web visualization
- STEP for mechanical CAD exchange when the modeling kernel supports reliable solids
- IFC for BIM/plant coordination if a suitable piping representation is implemented
- DXF only for limited 2D or simplified handoff cases

Geometry exports should carry stable IDs where the format permits. When a format cannot preserve piping semantics, the export report should state that the result is geometry-only.

## Export Profiles

The application should support named export profiles. A profile declares what the target expects and what the exporter is allowed to emit.

Example profile fields:

- `profile_id`
- `profile_version`
- `target_family`
- `target_software`
- `target_version`
- `format`
- `units`
- `coordinate_system`
- `vertical_axis`
- `supported_components`
- `supported_supports`
- `required_fields`
- `optional_fields`
- `unsupported_behavior`
- `material_mapping`
- `section_mapping`
- `spec_mapping`
- `support_mapping`
- `load_case_mapping`
- `target_options`
- `metadata_policy`
- `validation_policy`

Profiles let the application support different workflows without weakening the internal model.

## CAEPIPE Profile Shape

The first CAEPIPE profile should be explicit and narrow.

Recommended fields:

- `profile_id: caepipe-mbf.v1`
- `format: mbf`
- `target_software: CAEPIPE`
- `target_version: 15.x` or the version under test
- `mbf_version`
- `units`
- `vertical_axis`
- `length_precision`
- `node_numbering_policy`
- `component_weight_policy`
- `material_property_policy`
- `section_property_policy`
- `load_case_policy`
- `support_policy`
- `code_option_policy`
- `unsupported_behavior: error | warn`
- `result_parser_profile`

The `code_option_policy` must be framed as solver configuration. The local application should not derive code rules, calculate allowables, or present compliance conclusions.

## Validation Strategy

Export quality should be judged by validation, not by whether a file was written.

Validation should run in stages:

1. Internal model validation
2. Export target capability validation
3. File generation
4. Re-read validation where possible
5. Target parser validation
6. Optional downstream import/run test where the user has legal access to the target software
7. Result correlation against stable IDs
8. Regression comparison against accepted fixtures

Core checks:

- No disconnected line segments unless explicitly permitted.
- No duplicate stable IDs.
- No zero-length pipe segments.
- No impossible bend geometry.
- No missing units.
- No ambiguous coordinate system.
- No missing line number for exported piping.
- No unsupported component emitted without warning.
- No omitted support/restraint without report entry.
- No missing material, size, OD, or wall thickness when required by target profile.
- No mixed unit ambiguity.
- No unbounded loss of metadata.
- No target-side run accepted without preserving a result artifact and run manifest.

Validation output should include both machine-readable JSON and a concise human-readable Markdown report.

## CAEPIPE Validation Tiers

### Tier 1: Smoke

Purpose: prove the local export path and executable invocation.

Scope:

- Very small models that fit within evaluation-version constraints.
- Straight run.
- One bend.
- One anchor/guide case.
- One simple valve or rigid component if supported.

Pass condition:

- MBF is written.
- CAEPIPE produces CSV.
- CSV parser finds expected sections.
- Export report has no silent omissions.

### Tier 2: Fixture

Purpose: exercise common piping semantics.

Fixtures:

- Anchor-to-anchor straight run
- Pump discharge line
- Elbow loop
- Branch with tee
- Reducer case
- Valve/flange pair
- Guide and line-stop case
- Spring or hanger case only after support semantics are stable

Pass condition:

- Models run repeatedly.
- Counts and mapped IDs remain stable.
- Known approximations are reported.

### Tier 3: Regression

Purpose: compare accepted CAEPIPE outputs over time.

Scope:

- Full licensed CAEPIPE runs for models exceeding evaluation limits.
- Versioned baselines for generated MBF, CAEPIPE CSV, run manifest, and parsed result summaries.

Pass condition:

- Changes are explainable.
- Numeric differences fall within declared tolerances.
- Any target-version change updates the baseline deliberately.

### Tier 4: Round-Trip

Purpose: detect exporter drift.

Scope:

- Export MBF from the app.
- Import/run in CAEPIPE.
- Export MBF back from CAEPIPE when practical.
- Normalize and compare semantic content.

Pass condition:

- Differences are classified as equivalent, expected target normalization, or true loss.

### Tier 5: Manual Engineering Review

Purpose: support real-world validation without pretending it is automated approval.

Scope:

- Preserve `.mbf`, `.mod` if produced by the user workflow, `.csv`, reports, screenshots where useful, and engineer notes.
- Record downstream review status separately from automated pass/fail.

Pass condition:

- The review package is complete enough for a qualified user to reproduce and inspect the handoff.

## Loss and Assumption Reporting

Every export should produce an export report.

The report should include:

- Files written
- Export profile used
- Internal model schema version
- Target software and version
- Units and coordinate system
- Component counts
- Support/restraint counts
- Load/design case counts
- Unsupported components
- Unsupported supports
- Approximations
- Omitted metadata
- Missing user criteria
- Target-side assumptions
- Required downstream review steps
- Warnings and errors
- Stable ID mapping file location

This report is essential. Many engineering exchange failures are semantic losses that appear only after import or analysis.

## Adapter Architecture

Exporters should be plugins or adapters, not hardwired branches in the core model.

Recommended adapter responsibilities:

- Validate target capability.
- Convert canonical model to target representation.
- Write target files.
- Write sidecar ID maps.
- Write export reports.
- Provide optional parser for generated target files.
- Provide optional parser for target results.
- Register fixtures and golden files.

Recommended adapter interface:

```text
adapter.validate(model, profile) -> validation_result
adapter.export(model, profile, output_dir) -> export_package
adapter.read_back(output_dir, profile) -> parsed_export
adapter.run_external(output_dir, profile, runner_config) -> run_result
adapter.parse_results(output_dir, profile) -> parsed_results
adapter.compare(expected, actual, tolerance) -> comparison_report
```

The core application should not know CAEPIPE syntax. It should know only the canonical model and the adapter contract.

## Suggested Repository Layout

```text
exports/
  schemas/
    piping-model.schema.json
    export-profile.schema.json
    validation-report.schema.json
    run-manifest.schema.json
  profiles/
    caepipe-mbf.v1.json
    generic-pcf.v1.json
    neutral-stress.v1.json
  adapters/
    native_json/
    caepipe_mbf/
    pcf/
    neutral_stress/
    gltf/
  fixtures/
    straight_run/
    anchor_anchor/
    elbow_loop/
    pump_discharge/
    branch_with_valve/
    reducer_case/
    support_cases/
  tests/
    test_model_validation.py
    test_native_json_export.py
    test_caepipe_mbf_export.py
    test_caepipe_runner.py
    test_caepipe_csv_parser.py
    test_pcf_export.py
    test_neutral_stress_export.py
```

Recommended external-run output location:

```text
validation_runs/
  caepipe/
    YYYYMMDD_HHMMSS_case_name/
      model.json
      model.mbf
      model.csv
      id_map.json
      export_report.md
      export_report.json
      caepipe_run.json
```

The repository should include small synthetic fixtures and expected text outputs where licensing permits. It should not include proprietary downstream software files, copied vendor examples, or generated results that cannot be redistributed.

## Implementation Phases

### Phase 1: Export Foundation

- Define internal piping model schema.
- Define stable ID rules.
- Add model validation.
- Add native JSON export package.
- Add Markdown and JSON validation reports.
- Add fixture models for common piping arrangements.

Exit criteria:

- A sample model can be exported, re-read, validated, and compared for identity.

### Phase 2: CAEPIPE MBF Export

- Implement CAEPIPE MBF writer for a conservative component subset.
- Add `caepipe-mbf.v1` profile.
- Add deterministic node numbering.
- Add material and section serialization.
- Add support/restraint subset serialization.
- Add MBF golden fixtures.
- Add explicit warnings for unsupported solver fields.

Exit criteria:

- Representative simple systems export to MBF with stable IDs and no silent omissions.

### Phase 3: CAEPIPE Scripted Harness

- Add user-configured CAEPIPE executable path.
- Add local runner configuration.
- Invoke CAEPIPE with generated MBF.
- Detect produced CSV.
- Parse CSV enough to verify sections, counts, and selected result rows.
- Write `caepipe_run.json`.
- Add smoke fixtures that can run within evaluation-version limits.

Exit criteria:

- At least one generated MBF can be run through CAEPIPE and correlated back to canonical IDs.

### Phase 4: Stress Neutral Package

- Implement node/element/component/support/load-case tables.
- Export CSV and JSON variants.
- Add table-level validation.
- Add documentation for how the neutral package maps to common stress-model concepts.

Exit criteria:

- A model can be represented as a reviewable stress-oriented tabular package independent of any vendor.

### Phase 5: PCF Subset Export

- Implement PCF writer for a conservative component subset.
- Add PCF-specific validation profile.
- Add unit tests for component serialization.
- Add golden fixture files.
- Add loss reporting for unsupported items.
- Add CAEPIPE-PCF notes only as a secondary route, not as the primary validation route.

Exit criteria:

- Representative simple systems export to PCF with stable line/component IDs and no silent omissions.

### Phase 6: Geometry Export

- Add GLB/glTF export for review.
- Consider STEP export if the modeling kernel supports reliable solids.
- Add metadata linkage from geometry objects to stable IDs.

Exit criteria:

- A user can inspect the exported geometry and correlate objects back to the internal model.

### Phase 7: Adapter SDK and Additional Targets

- Define exporter plugin interface.
- Define export profile format.
- Add adapter test harness.
- Add sample adapter using the native JSON package as input.
- Evaluate additional documented import paths such as CAESAR II neutral-style handoffs, AutoPIPE-supported exchanges, plant design formats, or user-owned APIs where legally permitted.

Exit criteria:

- Third parties can implement exporters without modifying core model logic.

## Initial Minimal Viable Export

The first useful milestone should be deliberately small:

- One line
- Two terminal nozzles or endpoints
- Straight runs
- One bend
- One valve or rigid stand-in
- One tee or branch if supported in the first profile
- Basic anchors/guides
- User-supplied material/spec labels
- User-supplied section dimensions
- Native JSON package
- CAEPIPE MBF export
- Validation report
- Optional CAEPIPE run producing CSV

This is enough to prove the handoff architecture without getting trapped in the full complexity of plant design interoperability.

## Main Risks

- Exporting geometry without preserving engineering semantics.
- Overpromising compatibility with vendor tools.
- Silent loss of support/restraint meaning.
- Weak unit and coordinate handling.
- Tool-specific logic leaking into the core model.
- Users mistaking prototype exports for verified designs.
- Treating solver configuration as local code checking.
- Depending on an evaluation version beyond its permitted limits.
- Assuming PCF import behavior is deterministic across translator settings.
- Losing ID correlation between app objects and solver result rows.
- Legal exposure from undocumented proprietary formats.

## Recommended Near-Term Decision

Adopt this export priority:

1. Native open JSON package.
2. CAEPIPE MBF exporter.
3. CAEPIPE scripted validation harness.
4. Stress neutral CSV/JSON package.
5. PCF exporter.
6. GLB/glTF visual export.
7. Adapter SDK and additional target exporters after real import testing.

This sequence gives the application a practical path into existing piping workflows while keeping the core project open, inspectable, legally cautious, and technically honest.
