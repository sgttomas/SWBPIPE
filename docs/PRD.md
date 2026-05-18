# Product Requirements Document: OpenPipeStress

**Working title:** OpenPipeStress  
**Product type:** Free and open-source piping stress analysis platform  
**Document version:** 0.1 Draft  
**Date:** 2026-04-30  
**Status:** Product definition / pre-architecture PRD  
**Intended audience:** Piping stress engineers, software architects, numerical-method developers, open-source contributors, owner-operators, engineering managers, and technical reviewers

---

## 1. Executive Summary

OpenPipeStress is a proposed free and open-source software platform for global piping flexibility and stress analysis. The product will implement open, classical, and modern mechanical analysis methods for piping systems while requiring the responsible engineer to provide all code-specific, edition-specific, material, allowable-stress, component, stress-intensification, flexibility-factor, and proprietary design data.

The core product concept is:

> **Open mechanics in the solver; user-owned code knowledge in private data and rule packs.**

The software will provide a modern graphical interface, a transparent 3D line-element analytical engine, a user-defined rule-pack system, private material and component libraries, auditable reports, and a rigorous verification and validation program. It will not distribute ASME code text, ASME tables, ASME formulas copied from standards, ASME material allowable tables, B31J data, B16/B36 dimensional tables, or commercial software examples.

The product is designed to support engineering workflows for piping systems governed by ASME B31-family codes and other piping design codes, but it will not represent itself as an official ASME product, an ASME-certified tool, or a substitute for a licensed code, responsible engineer, or project design basis.

---

## 2. Background and Product Rationale

### 2.1 Technical Background

Modern piping stress analysis is fundamentally a global flexibility and stress-recovery problem. The primary model is a pipe centerline represented by line elements, with piping components represented by elastic members, rigid members, local flexibility modifiers, stress-intensification factors, and other engineering corrections. Three-dimensional solid or shell finite element analysis is reserved for selected local details, special components, or cases where the standard global pipe stress model is insufficient.

The user-supplied reference, **M. W. Kellogg Company, _Design of Piping Systems_ (1956)**, establishes much of the historical and theoretical basis for this model. Its structure aligns with the architecture proposed here:

- Chapter 2, beginning on page 31 of the scan, frames design assumptions, stress evaluation, and design limits.
- Chapter 3, beginning on page 53, addresses local components such as pipe bends, miters, branch connections, flanged connections, and equipment interfaces.
- Chapter 4, beginning on page 90, presents simplified flexibility-analysis methods.
- Chapter 5, beginning on page 115, presents the general analytical method for flexibility analysis, including simultaneous equations, curved members, multiple planes, restrained ends, branched systems, and intermediate restraints.

The proposed product updates this lineage using modern numerical methods: sparse 3D frame finite elements, unit-aware data models, nonlinear support solving, deterministic regression tests, and auditable digital reporting.

### 2.2 Product Opportunity

Commercial piping stress programs are powerful but expensive, opaque, and often difficult to audit at the numerical-method level. Existing open-source structural tools do not generally provide a dedicated piping stress workflow with piping-specific components, load cases, rule-pack logic, private code data management, stress-result review, and calculation-report discipline.

OpenPipeStress will fill that gap by providing:

- A credible open analytical core.
- A piping-specific modeling workflow.
- A graphical interface suitable for engineering use.
- A strict separation between open theory and copyrighted/proprietary code data.
- A validation-oriented development process.
- Private project libraries and rule packs for licensed engineers and companies.

---

## 3. Product Vision

OpenPipeStress will become the leading transparent, auditable, code-neutral, open-source platform for piping flexibility and stress analysis.

The product will allow a qualified user to model a piping system, define materials and components, apply loads and restraints, solve the global flexibility problem, recover forces/moments/displacements/stresses, and evaluate those results against user-supplied design rules and allowables.

The product will emphasize:

1. **Engineering transparency:** Calculations, assumptions, input values, and rule checks must be visible and auditable.
2. **Copyright-respecting architecture:** Code-specific values and protected data are entered by the user or imported from lawful private sources.
3. **Modernized classical theory:** The solver implements the best practical version of line-element piping analysis rather than attempting to replace pipe stress analysis with routine 3D solids analysis.
4. **Extensibility:** The architecture supports multiple codes, company standards, component libraries, and analysis modules without embedding proprietary standards.
5. **Validation:** The software is built around testable mechanics, regression cases, and published validation packages.

---

## 4. Product Goals

### 4.1 Primary Goals

- Provide a free and open-source piping stress analysis platform with a dedicated GUI.
- Implement a robust 3D centerline/beam-based global piping solver.
- Support straight pipe, bends, branches, reducers, valves, flanges, expansion joints, rigid elements, and common support/restraint types.
- Allow the user to define materials, component properties, code allowables, SIFs, flexibility factors, stress indices, and load combinations.
- Provide a private rule-pack mechanism for code-specific stress checks.
- Prevent silent use of missing code or component data.
- Generate auditable reports that identify all assumptions, sources, missing inputs, warnings, results, and rule-pack checksums.
- Maintain a rigorous verification and validation suite.
- Avoid distributing copyrighted or proprietary standards content.

### 4.2 Secondary Goals

- Support open exchange formats for integration with CAD, BIM, CAE, and plant design tools.
- Support educational and research use through non-code example models using invented or public-domain data.
- Provide optional local-analysis handoff to external FEA tools for nozzles, trunnions, lugs, branch details, or special components.
- Allow manufacturers to publish permissively licensed component data packages.
- Allow owner-operators and EPCs to maintain private rule packs and libraries.

---

## 5. Non-Goals

OpenPipeStress will **not**:

- Ship ASME B31.1, B31.3, B31J, B16, B36, or other proprietary standards text, tables, figures, examples, or copyrighted code content.
- Ship ASME material allowable-stress tables.
- Ship B31J stress-intensification-factor or flexibility-factor equations/tables copied from the standard.
- Ship B16/B36 component dimension tables unless the data is independently lawful for redistribution.
- Claim to be ASME-certified, ASME-approved, or endorsed by ASME.
- Replace the responsible engineer, licensed code edition, project design basis, owner specification, or engineering judgment.
- Provide automatic legal interpretation of piping codes.
- Treat global piping stress analysis as routine 3D solid finite element analysis.
- Certify fabrication, examination, inspection, testing, or construction compliance.
- Guarantee suitability for nuclear, aerospace, offshore, cryogenic, toxic, lethal-service, or high-consequence applications without additional qualified engineering review.

---

## 6. Design Principles

### 6.1 Open Mechanics, Private Code Data

The open-source core performs mechanics:

- Geometry.
- Section properties.
- Stiffness assembly.
- Load application.
- Thermal strain.
- Displacement solving.
- Force and moment recovery.
- Stress recovery from fundamental mechanics.
- Load-case algebra.

The user provides code-specific data:

- Allowable stresses.
- Code stress equations.
- Code load combinations.
- Code edition settings.
- Material tables.
- SIFs.
- Flexibility factors.
- Component dimensions.
- Manufacturer data.
- Owner-specific acceptance criteria.

### 6.2 No Silent Engineering Defaults

Any missing code-required value must produce a visible warning or blocking error. The solver may perform a mechanics-only solve with incomplete code-check data, but it must not report a code pass/fail status unless all required rule-pack inputs are present.

### 6.3 Data Provenance by Design

Every material, component, SIF, flexibility factor, load, allowable, and rule-pack value must support source/provenance fields.

Example source categories:

- User licensed code.
- Owner specification.
- Manufacturer catalog.
- Vendor calculation.
- Test report.
- Engineering calculation.
- Public-domain source.
- Invented example value.
- Unknown/unverified.

### 6.4 Global Line-Element Model First

The primary analysis model is a 3D line-element representation of the piping centerline. Local shell or solid analysis is treated as an optional specialized workflow, not as the default analysis basis.

### 6.5 Auditability Over Black-Box Automation

The software must make it easier for an engineer to understand and defend a calculation. It must not hide input assumptions, substitutions, or code-relevant decisions.

### 6.6 Unit Safety

All input, internal calculation, output, rule evaluation, and import/export operations must be unit-aware. Unit conversions must be deterministic and reportable.

---

## 7. Target Users and Personas

### 7.1 Piping Stress Engineer

Needs to create piping flexibility models, apply project data, solve load cases, review stresses and reactions, and issue calculation reports.

Key needs:

- Fast model creation.
- Familiar piping components.
- Load-case and support workflows.
- Reliable stress/reaction reports.
- Ability to enter licensed code data manually.
- Audit trail for all project inputs.

### 7.2 Owner-Operator Reviewer

Needs to review whether an EPC or vendor model is complete, traceable, and consistent with owner requirements.

Key needs:

- Clear assumptions.
- Rule-pack identification.
- Data provenance.
- Missing-data warnings.
- Report reproducibility.

### 7.3 Engineering Company Administrator

Needs to maintain private company rule packs, component libraries, default project templates, and approved calculation-report formats.

Key needs:

- Private libraries.
- Access control.
- Versioned templates.
- Rule-pack checksums.
- No accidental redistribution of protected code data.

### 7.4 Open-Source Contributor

Needs to contribute solver improvements, GUI features, validation cases, documentation, and permissively licensed data.

Key needs:

- Clear architecture.
- Contribution policy.
- IP review gates.
- Automated tests.
- Public validation cases.

### 7.5 Educator or Researcher

Needs transparent examples for teaching piping flexibility and stress analysis without relying on proprietary commercial software.

Key needs:

- Non-code example models.
- Mechanics-based validation problems.
- Clear theory documentation.
- Inspectable solver outputs.

---

## 8. Core User Journeys

### 8.1 Create and Analyze a New Piping System

1. User creates a new project and selects a unit system.
2. User defines project design basis metadata.
3. User creates or imports a private rule pack.
4. User defines materials and temperature-dependent properties.
5. User defines pipe sections and component properties.
6. User builds the 3D piping centerline in the GUI.
7. User inserts bends, branches, valves, flanges, reducers, expansion joints, and supports.
8. User assigns pressures, temperatures, weights, contents, insulation, and imposed displacements.
9. User defines primitive load cases and combination cases.
10. User solves the model.
11. User reviews displacements, rotations, forces, moments, support reactions, and stresses.
12. User runs private rule-pack checks.
13. User resolves warnings and missing data.
14. User generates an auditable calculation report.

### 8.2 Build a Private Code Rule Pack

1. User opens the rule-pack editor.
2. User selects a blank schema or company template.
3. User defines stress categories, load combinations, required variables, allowables, and expressions using their licensed/proprietary sources.
4. User marks the rule pack as private.
5. Software validates unit consistency and required input mappings.
6. Software generates a checksum and version identifier.
7. Rule pack becomes available for private project use only.

### 8.3 Import Private Component Data

1. User imports a private component library file.
2. Software checks schema, units, required fields, and provenance.
3. Software flags records with missing source or missing required dimensions.
4. User assigns components to model elements.
5. Report records component library name, version, source notes, and checksum.

### 8.4 Mechanics-Only Educational Example

1. User opens a public example model with invented material and allowable values.
2. User solves a simple U-loop or frame benchmark.
3. User compares displacement and force results with hand-calculation references.
4. No code-compliance claim is made.

---

## 9. Scope and Release Strategy

### 9.1 MVP Scope

The first minimum viable product must establish the analysis foundation and data-separation model.

MVP must include:

- 3D nodes with six degrees of freedom.
- Straight pipe/beam elements.
- Rigid elements.
- Linear anchors and restraints.
- Weight loads.
- Thermal loads.
- Internal pressure metadata and basic pressure-derived stress recovery.
- Imposed displacements.
- Linear static solver.
- Element force and moment recovery.
- Fundamental stress recovery.
- YAML/JSON project files.
- CSV/tabular results.
- Basic GUI modeler.
- Rule-pack schema with placeholder/example non-code rules.
- Blocking warnings for missing required rule-pack inputs.
- Initial validation suite.
- Calculation report export.

### 9.2 Post-MVP Scope

Post-MVP development should add:

- Bend elements with user-entered SIFs/flexibility factors.
- Branch connection objects with user-entered local modifiers.
- Reducers, flanges, valves, expansion joints, and spring hangers.
- Nonlinear supports: gaps, one-way supports, lift-off, friction, and sliding restraints.
- Load-case algebra and range cases.
- Dynamic modules: modal, response spectrum, harmonic, and time-history support.
- Local FEA export workflows.
- Open validation manual.
- Private library management.
- CAD/BIM import/export.

---

## 10. Functional Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-001 | Create, open, save, and version projects | Must | Project file round-trips without loss of model, units, loads, rule-pack references, or provenance metadata |
| FR-002 | Support explicit unit systems and conversions | Must | All numerical fields have units; incompatible units are rejected; reports show units used |
| FR-003 | Provide 3D node and element modeling | Must | User can create and edit nodes, straight pipe elements, and rigid elements in 3D |
| FR-004 | Support six degrees of freedom per structural node | Must | Solver supports translation and rotation in X/Y/Z at every active node |
| FR-005 | Provide pipe section property calculations | Must | Area, moment of inertia, section modulus, and mass properties are calculated from user-entered OD/wall/density data |
| FR-006 | Allow user-defined materials | Must | Materials include density, elastic modulus, Poisson's ratio, thermal expansion data, and user-entered allowables where needed |
| FR-007 | Support basic load cases | Must | Weight, temperature, pressure metadata, concentrated loads, and imposed displacements can be defined and solved |
| FR-008 | Solve linear static global flexibility problems | Must | Solver passes benchmark tests for beams, frames, thermal cases, and imposed displacements |
| FR-009 | Recover element forces and moments | Must | Local and global force/moment results are available at element ends |
| FR-010 | Recover fundamental stresses | Must | Axial, bending, torsional, and pressure-derived stress quantities can be displayed independently of code checks |
| FR-011 | Provide rule-pack schema | Must | Users can define private stress checks, variables, expressions, allowables, and load-combination mappings |
| FR-012 | Block incomplete code checks | Must | Software refuses to report pass/fail when required rule-pack values are missing |
| FR-013 | Provide graphical 3D modeler | Must | User can create/edit pipe geometry, supports, components, and load assignments visually |
| FR-014 | Provide model tree and property editor | Must | Every model entity can be selected and edited through structured property panels |
| FR-015 | Provide results visualization | Must | Deformed shape, support reactions, stress tables, and governing ratios are viewable in the GUI |
| FR-016 | Generate calculation reports | Must | Report includes input summary, assumptions, rule-pack checksum, warnings, results, and provenance |
| FR-017 | Support bend objects | Should | Bends accept radius, angle, orientation, user-entered SIFs, and user-entered flexibility factors |
| FR-018 | Support branch connection objects | Should | Branches accept geometry and user-entered local modifiers for header/branch checks |
| FR-019 | Support valves/flanges/reducers | Should | Components can be represented as rigid or semi-rigid elements with user-entered dimensions, weights, and source notes |
| FR-020 | Support expansion joints | Should | Expansion joints accept user-entered stiffnesses, effective pressure area, movement limits, and manufacturer provenance |
| FR-021 | Support nonlinear restraints | Should | Gaps, lift-off, one-way supports, and friction converge for validation cases |
| FR-022 | Support private material/component libraries | Should | User can load private libraries that are not redistributed with the open-source package |
| FR-023 | Support import/export | Could | Project and result data can be exchanged using open formats |
| FR-024 | Support dynamic analysis modules | Could | Modal and response-spectrum modules pass validation benchmarks |
| FR-025 | Support local FEA export | Could | Selected local geometry and loads can be exported for external shell/solid analysis |

---

## 11. Analytical Engine Requirements

### 11.1 Core Analysis Model

The solver shall use a 3D line-element model of the piping centerline. Each structural node shall support six degrees of freedom:

```text
Ux, Uy, Uz, Rx, Ry, Rz
```

Each element shall assemble a local stiffness contribution into the global stiffness matrix. The solver shall support local-to-global coordinate transformations, element-end force recovery, and result extraction in both local and global coordinate systems.

### 11.2 Solver Formulation

The initial solver shall implement linear elastic, small-displacement, static 3D frame analysis. Later versions may add geometric nonlinearity and advanced material behavior as optional modules.

Minimum solver capabilities:

- Sparse global stiffness matrix assembly.
- Boundary condition enforcement.
- Imposed displacement handling.
- Thermal initial strain handling.
- Distributed and concentrated load handling.
- Rigid-link/rigid-element support.
- Deterministic solution results.
- Singularity and ill-conditioning detection.
- Unit-aware result storage.

### 11.3 Element Types

#### 11.3.1 Straight Pipe Element

The straight pipe element shall support:

- Axial stiffness.
- Bending stiffness about two local transverse axes.
- Torsional stiffness.
- User-entered or calculated section properties.
- Distributed weight.
- Thermal strain.
- Pressure metadata.
- Stress recovery stations at element ends and optional intermediate points.

#### 11.3.2 Bend Element / Bend Macro-Element

The bend object shall support:

- Bend radius.
- Bend angle.
- Bend plane orientation.
- User-entered in-plane SIF.
- User-entered out-of-plane SIF.
- User-entered torsional SIF, where applicable.
- User-entered in-plane flexibility factor.
- User-entered out-of-plane flexibility factor.
- User-entered pressure-stiffening or pressure-modifier settings, where applicable.
- Optional segmented visualization.

The software shall not supply protected B31J or code-derived bend values in the public distribution.

#### 11.3.3 Branch Connection Object

Branch connection objects shall support:

- Header element association.
- Branch element association.
- Intersection node.
- Branch angle.
- Header and branch section assignments.
- Reinforcement metadata.
- User-entered header-side SIFs.
- User-entered branch-side SIFs.
- User-entered local flexibility modifiers.
- User-entered source/provenance for each local modifier.

#### 11.3.4 Rigid Component Element

Rigid components shall support:

- Length.
- Weight.
- Center of gravity.
- Rotational inertia metadata.
- Local orientation.
- Optional semi-rigid stiffness values.
- Source/provenance.

Applicable components include valves, flanges, strainers, instruments, specialty items, and equipment connection stubs.

#### 11.3.5 Reducer Element

Reducers shall support:

- Concentric/eccentric geometry.
- Upstream/downstream section assignments.
- Length.
- Weight.
- Offset vector for eccentric reducers.
- User-entered local stress modifiers where needed.

#### 11.3.6 Expansion Joint Element

Expansion joints shall support:

- Axial stiffness.
- Lateral stiffness.
- Angular stiffness.
- Torsional stiffness.
- Effective pressure area.
- Pressure thrust.
- Movement limits.
- Tie rod/hinge/gimbal/limit rod configuration metadata.
- Manufacturer data provenance.

### 11.4 Support and Restraint Types

The software shall support these support/restraint types:

- Anchor.
- Guide.
- Line stop.
- Vertical support.
- Directional restraint.
- Translational spring.
- Rotational spring.
- Constant-effort support.
- Variable spring hanger.
- Rigid strut.
- Gap support.
- One-way support.
- Friction support.
- Sliding support.

MVP may include only anchors and linear restraints. Nonlinear supports should be implemented after the linear solver is validated.

### 11.5 Load Types

Primitive load types shall include:

- Dead weight of pipe.
- Contents weight.
- Insulation/refractory/lining weight.
- Component weight.
- Internal pressure metadata.
- Temperature change or total thermal expansion data.
- Concentrated force.
- Concentrated moment.
- Imposed displacement.
- Imposed rotation.
- Wind load.
- Seismic static-equivalent load.
- Hydrotest load.
- User-defined occasional load.

### 11.6 Load Cases and Combinations

The software shall distinguish:

1. **Primitive analysis cases:** Directly solved load cases such as weight, pressure, temperature, imposed displacement, wind, seismic, hydrotest, and user-defined loads.
2. **Combination cases:** Algebraic combinations or differences of solved cases.
3. **Rule-check cases:** User-defined mapping of combinations to stress categories and allowable limits.

The public software shall include a generic load-case builder but shall not ship protected code-specific load-combination requirements.

### 11.7 Fundamental Stress Recovery

The solver shall recover fundamental mechanics quantities before code evaluation:

- Axial force.
- Shear forces.
- Torsional moment.
- Local bending moments.
- Resultant bending moment.
- Axial stress.
- Bending stress.
- Torsional shear stress.
- Pressure hoop stress, if user selects a pressure-stress method.
- Pressure longitudinal stress, if user selects a pressure-stress method.
- Principal stress, optional.
- von Mises equivalent stress, optional.
- Tresca equivalent stress, optional.

Code-specific stress formulas shall be handled in user rule packs.

### 11.8 Numerical Quality Requirements

The engine shall:

- Use deterministic algorithms where practical.
- Report solver residuals.
- Detect unconstrained rigid body modes.
- Detect duplicate or near-duplicate nodes.
- Detect zero-length or invalid elements.
- Detect inconsistent units.
- Store input and output values with sufficient precision for engineering review.
- Provide reproducible results for a given project file and solver version.

---

## 12. Rule-Pack Requirements

### 12.1 Purpose

The rule-pack system is the key mechanism for code-neutral software. It allows users to define code-specific or company-specific checks without the public software distributing protected code content.

### 12.2 Rule-Pack Capabilities

A rule pack shall define:

- Rule-pack name.
- Version.
- Code family or project standard, free text.
- Code edition, free text.
- Source/provenance statement.
- Redistribution status.
- Required material inputs.
- Required component inputs.
- Required project inputs.
- Stress categories.
- Load-case mappings.
- Expressions.
- Allowables.
- Warnings.
- Blocking conditions.
- Unit expectations.

### 12.3 Rule-Pack Expression Evaluator

The expression evaluator shall be:

- Unit-aware.
- Sandboxed.
- Deterministic.
- Non-Turing-complete, if practical.
- Free of arbitrary filesystem or network access.
- Capable of basic algebra, conditionals, min/max, interpolation, and table lookup using user-provided tables.

### 12.4 Rule-Pack Data Separation

Public examples may include invented demonstration values. Public examples must not include protected ASME content or copied code formulas.

Private user rule packs may include the user's licensed or proprietary information, subject to the user's legal rights and organization policies. The application shall mark such rule packs as private by default.

### 12.5 Example Rule-Pack Skeleton

The public repository may include examples like the following, using placeholders rather than code-specific formulas:

```yaml
rule_pack:
  name: "Example non-code piping rule pack"
  version: "0.1"
  source: "Invented demonstration values only"
  redistribution_allowed: true

required_inputs:
  material:
    - elastic_modulus_cold
    - elastic_modulus_hot
    - thermal_expansion_range
    - user_allowable_sustained
    - user_allowable_displacement
  component:
    - user_in_plane_sif
    - user_out_plane_sif
    - user_flexibility_factor
  project:
    - design_pressure
    - operating_temperature
    - ambient_temperature

checks:
  sustained_example:
    combination: "USER_DEFINED_SUSTAINED_CASE"
    expression: "USER_DEFINED_STRESS_EXPRESSION"
    allowable: "user_allowable_sustained"
    result_label: "Example sustained ratio"

  displacement_example:
    combination: "USER_DEFINED_DISPLACEMENT_RANGE"
    expression: "USER_DEFINED_RANGE_EXPRESSION"
    allowable: "user_allowable_displacement"
    result_label: "Example displacement ratio"
```

---

## 13. Material and Component Library Requirements

### 13.1 Material Libraries

Material records shall support:

- Material name.
- Specification/grade, free text.
- Density.
- Poisson's ratio.
- Elastic modulus versus temperature.
- Thermal expansion coefficient or total expansion strain versus temperature.
- Allowable stress versus temperature, user-entered.
- Yield strength versus temperature, user-entered where needed.
- Ultimate strength versus temperature, user-entered where needed.
- Weld/joint/quality factors, user-entered where needed.
- Corrosion allowance.
- Mill tolerance.
- Source/provenance for each property family.
- Verification status.

The public repository shall not ship ASME material allowable tables.

### 13.2 Pipe Section Libraries

Pipe section records shall support:

- Nominal size label, free text.
- Outside diameter.
- Wall thickness.
- Schedule or designation, free text.
- Corrosion allowance.
- Mill tolerance.
- Lining/refractory/insulation metadata.
- Source/provenance.

The public repository shall not ship copyrighted B36 tables unless the data is lawful for redistribution.

### 13.3 Component Libraries

Component records shall support:

- Component type.
- Manufacturer.
- Model.
- Size/rating label.
- End connection.
- Length or face-to-face dimension.
- Weight.
- Center of gravity.
- Stiffness behavior.
- Effective pressure area, if applicable.
- Movement limits, if applicable.
- Source/provenance.
- License or redistribution status.

### 13.4 Public vs Private Data

OpenPipeStress shall distinguish:

- **Public bundled data:** Only invented, public-domain, independently generated, or permissively licensed data.
- **Private user data:** User-entered standards data, owner data, vendor catalogs, material tables, rule packs, and component libraries.
- **Manufacturer-published data packs:** Permissively licensed component libraries supplied by vendors or contributors with documented redistribution rights.

### 13.5 Data Import Warnings

The software shall flag imported data records if:

- Required fields are missing.
- Units are missing or inconsistent.
- Source/provenance is missing.
- Redistribution status is unclear.
- Data appears to be a protected standards table.
- Values are outside user-defined reasonableness checks.

---

## 14. GUI Requirements

### 14.1 Main Interface Layout

The GUI shall include:

- 3D centerline viewport.
- Model tree.
- Property inspector.
- Material library editor.
- Component library editor.
- Load-case manager.
- Rule-pack manager.
- Solver console.
- Results browser.
- Report generator.

### 14.2 3D Viewport

The 3D viewport shall display:

- Pipe centerlines.
- Bend arcs.
- Branch symbols.
- Valves.
- Flanges.
- Reducers.
- Expansion joints.
- Anchors.
- Guides.
- Line stops.
- Springs.
- Gaps.
- Node labels.
- Element labels.
- Load vectors.
- Deformed shapes.
- Reaction arrows.
- Stress-ratio color maps.

### 14.3 Property Inspector

Selecting a model entity shall show editable properties, including:

- Geometry.
- Material.
- Section.
- Component attributes.
- Loads.
- Support settings.
- SIFs/flexibility factors.
- Source/provenance.
- Missing-data status.
- Rule-pack dependencies.

### 14.4 Missing-Data UX

The GUI shall distinguish:

- **Blocking for solve:** Missing data prevents numerical solution.
- **Blocking for code check:** Mechanics solve can run, but code pass/fail cannot be reported.
- **Warning:** Data exists but is incomplete, unverified, or unusual.
- **Informational:** Assumption or note recorded for audit.

Example warnings:

```text
Cannot perform rule check: material allowable stress at operating temperature is missing.
```

```text
Cannot perform bend stress check: out-of-plane SIF is required by the selected rule pack but has not been entered.
```

```text
Mechanics solve allowed: valve V-101 has no source note for weight. Results are marked unverified.
```

### 14.5 Rule-Pack Editor

The rule-pack editor shall provide:

- Required-input definition.
- Variable browser.
- Unit checking.
- Expression editor.
- Load-combination mapping.
- Allowable definition.
- Private/public status.
- Source/provenance fields.
- Validation diagnostics.
- Checksum generation.

The editor shall not provide protected code equations as prefilled templates.

### 14.6 Component Wizard

The component wizard shall help users enter valves, flanges, reducers, expansion joints, and specialty items. It shall request required dimensions and component properties without supplying protected standards tables.

Example valve wizard fields:

- Component ID.
- Valve type.
- End connection.
- Size label.
- Pressure/rating label.
- Face-to-face or end-to-end length.
- Weight.
- Center of gravity.
- Rigid or semi-rigid behavior.
- Source/provenance.

---

## 15. Reporting and Audit Trail

### 15.1 Report Requirements

Calculation reports shall include:

- Project name.
- Software version.
- Solver version.
- Git commit hash or release ID.
- Date/time.
- Units.
- Coordinate system.
- Model summary.
- Node table.
- Element table.
- Material table.
- Component table.
- Support/restraint table.
- Load cases.
- Load combinations.
- Rule-pack name, version, and checksum.
- Statement that code-specific data was user supplied.
- Source/provenance summary.
- Missing-data warnings.
- Solver diagnostics.
- Displacements.
- Rotations.
- Element forces.
- Element moments.
- Support reactions.
- Equipment terminal loads.
- Fundamental stress results.
- Rule-check results.
- Governing locations.
- User signoff block.

### 15.2 Report Prohibitions

Reports generated from public templates shall not reproduce protected ASME code text, tables, examples, or proprietary standards content. Users may maintain private report templates at their own discretion and responsibility.

### 15.3 Reproducibility

Reports shall include sufficient metadata to reproduce results from the same project file, solver version, and private rule/data libraries.

---

## 16. Verification and Validation Requirements

### 16.1 Verification Philosophy

Verification shall prove that the equations are implemented correctly. Validation shall demonstrate that the modeling approach produces credible engineering results within stated assumptions.

### 16.2 Required Solver Benchmarks

The validation suite shall include mechanics-based examples such as:

- Cantilever beam with tip force.
- Cantilever beam with tip moment.
- Fixed-fixed beam with thermal expansion.
- Simply supported pipe under uniform weight.
- L-shaped thermal expansion frame.
- U-loop thermal expansion benchmark.
- Torsion-only element.
- Rigid-link benchmark.
- Inclined-member coordinate transformation benchmark.
- Imposed anchor displacement benchmark.
- Linear spring support benchmark.
- Gapped support benchmark.
- Friction support benchmark.
- Branch assembly benchmark.

### 16.3 Stress Recovery Benchmarks

Stress recovery tests shall include:

- Axial stress from axial force.
- Bending stress from moment.
- Torsional shear from torque.
- Pressure-derived hoop stress.
- Pressure-derived longitudinal stress.
- Stress range from two solved states.
- User-entered SIF multiplier behavior.
- User-entered flexibility-factor behavior.

### 16.4 Regression Testing

Every release shall run automated tests for:

- Unit conversion.
- Matrix assembly.
- Coordinate transformation.
- Boundary conditions.
- Load application.
- Thermal strain.
- Force recovery.
- Stress recovery.
- Rule-pack evaluation.
- Nonlinear support convergence, where applicable.
- Report reproducibility.

### 16.5 Validation Manual

The project shall maintain a validation manual documenting:

- Test purpose.
- Input model.
- Hand calculation or independent reference.
- Expected result.
- Software result.
- Tolerance.
- Pass/fail status.
- Solver version.

Validation examples shall not be copied from protected standards or commercial software manuals unless written permission permits redistribution.

---

## 17. Data Governance, IP, and Compliance Requirements

### 17.1 Public Repository Restrictions

The public repository shall reject contributions containing:

- ASME code text.
- ASME material allowable tables.
- ASME B31J tables or equations copied from the standard.
- ASME B16/B36 dimensional tables copied from standards.
- Screenshots or scans of protected standards.
- Commercial software examples or manuals without permission.
- Vendor catalog data without redistribution permission.
- Any data contribution without provenance.

### 17.2 Contributor Certification

Contributors shall certify that:

```text
I did not copy this contribution from ASME, API, MSS, ISO, EN, CSA, ASTM, a commercial software manual, or any other copyrighted standard or proprietary source unless the material is licensed for redistribution in this project.
```

### 17.3 Private Data Handling

Private rule packs and libraries shall be:

- Stored outside public example directories by default.
- Marked private in metadata.
- Excluded from telemetry.
- Excluded from public bug reports unless the user explicitly attaches them.
- Identified in reports by name/version/checksum without exposing protected content unless the user requests full reporting.

### 17.4 Disclaimers

The application shall include clear user-facing language:

```text
This software performs piping flexibility and stress analysis using open mechanical methods. It does not include ASME code text, ASME tables, ASME material allowables, or proprietary code data. Code-specific values and rules must be supplied and verified by the responsible engineer.
```

```text
This software is not affiliated with, endorsed by, certified by, or approved by ASME.
```

### 17.5 Legal Review Requirement

Before public release, legal counsel should review:

- Project name and branding.
- Documentation language.
- Disclaimers.
- Contributor license agreement or developer certificate of origin.
- Data-contribution policy.
- Example libraries.
- Rule-pack examples.

---

## 18. Security and Privacy Requirements

### 18.1 Local-First Design

The default application shall operate locally. No cloud service shall be required for modeling, solving, rule checking, or reporting.

### 18.2 Telemetry

Telemetry shall be disabled by default. If telemetry is added, it must be opt-in and must never transmit private rule packs, component libraries, material data, project files, or calculation results without explicit user action.

### 18.3 Private Data Protection

The product should support:

- Private library directories.
- Redaction of private data from bug reports.
- Optional encrypted storage for private libraries.
- Checksums for rule packs and libraries.
- Clear warnings before exporting private data.

---

## 19. Platform and Technical Architecture

### 19.1 Proposed Architecture

```text
open-pipe-stress/
  core/
    geometry/
    units/
    materials/
    sections/
    elements/
    supports/
    loads/
    solver/
    nonlinear/
    stress/
    results/

  rules/
    schema/
    evaluator/
    examples_non_code/

  gui/
    viewport/
    model_tree/
    property_editor/
    load_cases/
    rule_pack_editor/
    results_viewer/
    report_viewer/

  data/
    schemas/
    empty_templates/
    public_domain_examples/

  reports/
    templates/
    audit/
    plots/

  validation/
    hand_calcs/
    benchmarks/
    regression_cases/

  docs/
    theory/
    user_guide/
    developer_guide/
    ip_policy/
    validation_manual/
```

### 19.2 Technology Selection Criteria

The final language and framework are open decisions. Candidate implementation approaches should be evaluated by:

- Numerical performance.
- Sparse matrix support.
- Cross-platform GUI support.
- Maintainability.
- Strong typing and unit safety.
- Ability to expose scripting or plugin APIs safely.
- Open-source dependency licenses.
- Ease of packaging for Windows, Linux, and macOS.

Potential architectures include:

- C++ or Rust core with Qt GUI.
- Python GUI with compiled numerical core.
- Rust core with webview/native GUI.
- Hybrid core library plus multiple front ends.

### 19.3 Public API

The core solver should expose an API that allows:

- Model creation.
- Load-case definition.
- Solve execution.
- Result extraction.
- Rule-pack evaluation.
- Report generation.
- Validation-test execution.

---

## 20. Performance Requirements

Initial performance targets:

- Solve small tutorial models interactively.
- Solve medium models with hundreds of nodes in seconds on a standard engineering workstation.
- Solve large models with thousands of nodes using sparse matrix methods.
- Keep GUI responsive during solve through background execution.
- Provide progress, cancellation, and diagnostic logs for long solves.

Performance must not compromise auditability or unit safety.

---

## 21. Accessibility and Usability Requirements

The GUI should support:

- Keyboard navigation for major panels.
- Clear icon labels and tooltips.
- High-contrast result visualization options.
- Search/filter in large model trees.
- Copy/export from result tables.
- Undo/redo for modeling actions.
- Project templates.
- Inline validation messages.
- Clear separation between solve warnings and code-check warnings.

---

## 22. Release Milestones

### 22.1 Release R0: Architecture Prototype

Deliverables:

- Project data schema.
- Unit system.
- Minimal solver prototype.
- Straight pipe/beam element.
- One or two verification cases.
- Command-line input/output.

Exit criteria:

- Cantilever and fixed-fixed thermal benchmarks pass.
- Project schema round-trips.

### 22.2 Release R1: Core Solver MVP

Deliverables:

- 3D static frame solver.
- Weight/thermal/imposed displacement loads.
- Basic stress recovery.
- CSV results.
- Validation test suite.

Exit criteria:

- Required benchmark suite passes within documented tolerances.
- Solver detects singular models.

### 22.3 Release R2: GUI MVP

Deliverables:

- 3D modeler.
- Model tree.
- Property editor.
- Load case manager.
- Results tables.
- Deformed shape plot.
- Basic report generator.

Exit criteria:

- User can create, solve, and report a small piping model without editing raw files.

### 22.4 Release R3: Rule-Pack and Private Libraries

Deliverables:

- Rule-pack schema.
- Expression evaluator.
- Private material libraries.
- Private component libraries.
- Missing-data blockers.
- Rule-check result tables.

Exit criteria:

- User can define a private non-code rule pack and run checks.
- Software blocks pass/fail when required inputs are missing.

### 22.5 Release R4: Piping Components and Nonlinear Supports

Deliverables:

- Bend objects.
- Branch objects.
- Rigid valves/flanges/reducers.
- Expansion joints.
- Spring hangers.
- Gaps/lift-off/friction.

Exit criteria:

- Nonlinear support validation cases converge.
- Component provenance appears in reports.

### 22.6 Release R5: Engineering Beta

Deliverables:

- Validation manual.
- Full report package.
- IP contribution process.
- Public issue templates.
- Private-data redaction workflow.
- Signed releases.

Exit criteria:

- External engineers can reproduce validation examples.
- Public repository contains no known protected standards data.

---

## 23. Success Metrics

### 23.1 Technical Metrics

- Number of verification benchmarks passed.
- Regression-test coverage.
- Maximum deviation from hand benchmarks.
- Solver stability for large models.
- Number of unresolved critical solver defects.
- Reproducibility of reports across platforms.

### 23.2 Product Metrics

- Time to create and solve a simple U-loop model.
- Number of missing-data errors caught before report issue.
- Number of usable private-library imports.
- User satisfaction from stress engineers.
- Documentation completeness.
- Validation manual completeness.

### 23.3 Governance Metrics

- Number of data contributions with complete provenance.
- Number of rejected IP-risk contributions.
- Time to review contributions.
- Number of public examples verified as non-code/non-proprietary.

---

## 24. Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Accidental inclusion of copyrighted standards content | High | Strict contribution policy, IP review, no populated code tables, provenance required |
| Users mistake software for official code compliance | High | Clear disclaimers, blocking warnings, report language, no ASME branding |
| Solver errors | High | Verification suite, validation manual, regression tests, peer review |
| Unit conversion errors | High | Unit-aware data model and expression evaluator |
| Nonlinear support convergence failures | Medium/High | Active-set diagnostics, convergence criteria, user warnings, validation cases |
| Missing component data | Medium/High | Blocking warnings and source/provenance requirements |
| Overreliance on generic defaults | High | No silent defaults for code-relevant data |
| GUI complexity delays release | Medium | CLI-first solver MVP, then GUI layers |
| Private data leakage in bug reports | High | Redaction tools and local-first design |
| Open-source liability concerns | Medium/High | Disclaimers, engineering review requirements, legal review |

---

## 25. Open Questions

1. What open-source license should be used: Apache-2.0, GPLv3, LGPL, MPL, or dual license?
2. Should the core solver be written in Rust, C++, Python with compiled libraries, or another language?
3. Should the GUI be Qt, web-based, or another cross-platform framework?
4. What should the project be named to avoid implying ASME affiliation?
5. Should a contributor license agreement or developer certificate of origin be required?
6. What are acceptable public sources for any bundled dimensional data?
7. Should private rule packs be encrypted by default?
8. How should the project define engineering-beta versus production-ready?
9. What tolerance bands should be used for each validation benchmark?
10. How should commercial software comparison results be handled if users want to contribute them?

---

## 26. Glossary

**Allowable stress:** A user-supplied stress limit or stress-value table used by a rule pack.

**Bend flexibility factor:** A user-supplied modifier used to represent increased flexibility of curved piping components relative to straight pipe behavior.

**Code-neutral:** The software provides mechanics and workflow without embedding proprietary or edition-specific code requirements.

**Global piping analysis:** A centerline/beam-style analysis of the piping system as a connected structural network.

**Local analysis:** A detailed shell or solid analysis of a component or attachment, usually performed outside the global piping model.

**Primitive load case:** A directly solved load case, such as weight, temperature, pressure metadata, wind, seismic, or imposed displacement.

**Rule pack:** A user-defined set of required inputs, load combinations, stress expressions, allowables, and pass/fail checks.

**SIF:** Stress intensification factor; a user-supplied local stress modifier applied during stress recovery or rule checking.

**Source/provenance:** Metadata identifying where a value came from and whether it is verified, private, public, invented, licensed, or otherwise documented.

---

## 27. Appendix A: Example Project Data Model

```yaml
project:
  name: "Example OpenPipeStress Project"
  units: "inch_lbf_F"
  coordinate_system:
    vertical_axis: "Y"
  design_basis:
    description: "Demonstration only; no code compliance claim"
    rule_pack: "example_non_code_rule_pack"

materials:
  - id: "MAT-001"
    name: "Example Carbon Steel"
    source: "Invented demonstration material"
    density: "0.283 lb/in^3"
    poisson_ratio: 0.30
    elastic_modulus:
      - temperature: "70 degF"
        value: "29e6 psi"
        source: "invented example"
    thermal_expansion:
      - from_temperature: "70 degF"
        to_temperature: "400 degF"
        value: "user supplied"
        source: "required user input"

sections:
  - id: "PIPE-001"
    label: "User-defined pipe section"
    od: "10.75 in"
    wall: "0.365 in"
    source: "user supplied"

nodes:
  - id: "N1"
    xyz: ["0 in", "0 in", "0 in"]
  - id: "N2"
    xyz: ["120 in", "0 in", "0 in"]

elements:
  - id: "E1"
    type: "straight_pipe"
    node_i: "N1"
    node_j: "N2"
    material: "MAT-001"
    section: "PIPE-001"

supports:
  - id: "S1"
    node: "N1"
    type: "anchor"

load_cases:
  - id: "W"
    type: "weight"
  - id: "T1"
    type: "temperature"
    temperature: "400 degF"
```

---

## 28. Appendix B: Example Report Notice

```text
This calculation was performed using OpenPipeStress, an open-source piping flexibility and stress-analysis platform. The software performs mechanical analysis using open analytical methods. Code-specific requirements, material allowables, component dimensions, stress intensification factors, flexibility factors, and acceptance criteria were supplied by the user through private project data and rule packs. This software does not include ASME code text, ASME tables, ASME material allowable tables, or proprietary standards content and is not affiliated with or endorsed by ASME.
```

---

## 29. Source Notes and External References

### 29.1 Foundational Technical Reference

- M. W. Kellogg Company, _Design of Piping Systems_, 1956. User-supplied scanned file: `MWK 1956.pdf`.

### 29.2 Public ASME Reference Pages Used for Product Boundary Context

These pages are referenced only to define product-boundary context and copyright-sensitive areas. They are not used to reproduce code rules, tables, formulas, or proprietary standards content.

- ASME, “Use of ASME Copyrighted Information”: https://www.asme.org/codes-standards/find-codes-standards/use-of-asme-copyrighted-information
- ASME, “B31J - Stress Intensification Factors (i-Factors), Flexibility Factors (k-Factors), and Their Determination for Metallic Piping Components”: https://www.asme.org/codes-standards/find-codes-standards/b31j-stress-intensification-factors-flexibility-factors-determination-metallic-piping-components
- ASME, “B31.1 - Power Piping”: https://www.asme.org/codes-standards/find-codes-standards/b31-1-power-piping
- ASME, “B31.3 - Process Piping”: https://www.asme.org/codes-standards/find-codes-standards/b313-2018-process-piping
