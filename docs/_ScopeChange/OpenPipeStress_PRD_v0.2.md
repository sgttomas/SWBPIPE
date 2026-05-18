# Product Requirements Document: OpenPipeStress

**Working title:** OpenPipeStress  
**Product type:** Analysis-grade piping design engine and stress-model authoring environment  
**Document version:** 0.2 Draft  
**Date:** 2026-05-02  
**Status:** Replacement PRD / product definition  
**Intended audience:** Piping designers, piping stress engineers, software architects, numerical-method developers, open-source contributors, owner-operators, EPC engineering managers, and technical reviewers

---

## 1. Executive Summary

OpenPipeStress is a proposed free and open-source, local-first, analysis-grade piping design engine with an integrated piping flexibility and stress-analysis core. It enables engineers and designers to create, modify, analyze, compare, and export piping route and support concepts in a schema-backed graphical environment.

The product is not intended to become an industry-standard professional stress-analysis platform or to replace authenticated commercial stress-analysis software. Instead, it is intended to operate earlier in the design workflow, where routes, supports, constraints, equipment interfaces, and design assumptions are still evolving.

OpenPipeStress shall maintain a robust physical model, apply user-supplied design knowledge and constraints, solve the piping flexibility/stress model using transparent analytical methods, and produce schema-compliant model data suitable for downstream 3D modeling and professional piping stress-analysis validation.

The software shall include a serious analytical engine. Its results must be as correct, deterministic, auditable, and useful as practical. However, those results are design-engine results. They may support engineering iteration, screening, comparison, and model authoring, but they are not professional reliance results. Final project reliance requires validation in accepted industry software and competent human review.

The governing product distinction is:

> **OpenPipeStress computes and helps design. The external prover tool validates for reliance. The responsible engineer accepts.**

The governing product philosophy is:

> **Open source the mechanics. Make design intent executable. Export to the prover. Let the responsible engineer accept.**

---

## 2. Source Basis and Supersession

This PRD supersedes the earlier product framing that described OpenPipeStress primarily as a free and open-source piping stress-analysis platform. It retains the valuable portions of that prior direction: the open analytical core, 3D line-element solver, rule-pack architecture, private data separation, unit safety, validation discipline, reporting, and professional boundary.

This PRD also incorporates the project intent that the software should implement open piping flexibility and stress-analysis mechanics while avoiding redistribution of protected standards data, proprietary code rules, copyrighted examples, and private company data.

This PRD retains and depends on the project governance boundaries defined by the IP/Data Boundary Policy and the Professional Boundary and Product Claims Policy.

---

## 3. Product Vision

OpenPipeStress will be a transparent, auditable, analysis-grade piping design environment that lets users build and iterate piping route/support concepts before those concepts are finalized in professional 3D modeling and professional piping stress-analysis software.

The product shall allow a qualified user to:

1. express design knowledge and constraints;
2. build a schema-backed physical piping model;
3. create or modify the model through a graphical user interface;
4. optionally use agents to propose model operations;
5. run a full internal piping flexibility/stress analysis;
6. review displacements, rotations, forces, moments, stresses, support reactions, and terminal loads;
7. evaluate results using user-supplied rule packs and private project data;
8. save named model states and analysis runs;
9. compare model states and analysis results deterministically;
10. export schema-compliant handoff data to downstream modeling and professional stress-analysis workflows.

OpenPipeStress shall be useful because its analytical results are right, not because it claims authority. The product shall prioritize correctness, transparency, reproducibility, and handoff quality while avoiding claims that it certifies, authenticates, seals, or approves engineering work.

---

## 4. Product Positioning

### 4.1 What the Product Is

OpenPipeStress is:

- an analysis-grade piping design engine;
- a schema-backed physical modeling environment;
- a graphical piping route/support authoring tool;
- a full piping flexibility and stress-analysis engine for design iteration;
- a user-rule and private-data evaluation environment;
- a model state, analysis run, and comparison management tool;
- a handoff generator for downstream modeling and professional stress-analysis validation.

### 4.2 What the Product Is Not

OpenPipeStress is not:

- an authenticated replacement for accepted commercial piping stress-analysis software;
- a standards-body-approved or standards-body-certified application;
- a source of final professional code-compliance determinations;
- a substitute for licensed standards, project design basis, owner requirements, or engineering judgment;
- a system that distributes protected standards content, proprietary rule data, or commercial software examples;
- a comprehensive external-prover result ingestion platform in the MVP;
- a lifecycle/status-control system that forces formal prover states on the user.

### 4.3 Product Boundary

The product boundary is not defined by solver capability. OpenPipeStress shall have a full analytical engine. The boundary is defined by claim authority.

The software may say:

- mechanics solved;
- internal analytical result produced;
- user-rule check evaluated;
- warnings and assumptions recorded;
- model state saved;
- analysis run saved;
- model/result states compared;
- handoff package generated.

The software must not say or imply:

- professional approval granted;
- design certified;
- design sealed;
- code compliance established for reliance;
- external prover validation completed unless that is a user-supplied external record and not a solver-generated conclusion.

---

## 5. Design Principles

### 5.1 Full Analytical Engine, Non-Authoritative Result

OpenPipeStress shall implement a serious piping flexibility and stress-analysis engine. The analytical result must be useful enough to guide route selection, support placement, equipment-load reduction, expansion-loop sizing, model cleanup, and design iteration.

However, internal results are not authoritative for professional reliance. The intended validation pathway is export to an accepted professional stress-analysis tool and review by a competent engineer.

### 5.2 Schema-Backed Physical Model Is the Source of Truth

The source of truth shall be the deterministic model graph and its schema-compliant serialized state, not free-form text, agent messages, report prose, or an external file alone.

All geometry, supports, components, loads, materials, libraries, rule references, constraints, warnings, assumptions, and analysis settings shall be traceable to model entities.

### 5.3 GUI and Agent Actions Become Model Operations

The user may modify the model manually through the GUI. Agents may also propose changes. In both cases, changes shall be represented as structured model operations, validated against the schema, shown to the user, and applied only through the model engine.

Agent output shall never become accepted engineering work by itself.

### 5.4 Open Mechanics, Private Code Data

The public project shall include open mechanics, schemas, solvers, validation cases, and empty or invented examples. Code-specific and project-specific values shall be supplied by the user or the user's organization through private data and rule packs.

### 5.5 No Silent Engineering Defaults

The product shall not silently insert code-relevant data, component values, SIFs, flexibility factors, allowables, owner requirements, or support assumptions. Missing data must produce visible warnings or blocking errors according to severity.

### 5.6 Unit Safety

All numerical values shall be unit-aware. The product shall reject incompatible units, preserve unit metadata, and report unit conversions deterministically.

### 5.7 State-Based Auditability

The product shall support named, immutable model states and analysis runs. Every result set shall bind to the model state, solver version, analysis settings, load cases, rule-pack references, and library versions used to produce it.

### 5.8 Handoff Quality Over Platform Lock-In

The product shall make it easy to export a clean, traceable, schema-compliant model basis for downstream 3D modeling and professional stress-analysis validation.

### 5.9 Professional Boundary by Design

Reports, UI labels, agent output, examples, and documentation shall preserve the distinction between software-computed results and professional engineering acceptance.

---

## 6. Product Goals

### 6.1 Primary Goals

1. Provide a schema-backed piping design environment for route, component, support, load, and constraint modeling.
2. Provide a full 3D centerline/beam-based piping flexibility and stress-analysis engine suitable for serious design iteration.
3. Provide a graphical interface for selecting, modifying, inspecting, and comparing model assets.
4. Support user-supplied design knowledge, constraints, private component data, private material data, and private rule packs.
5. Allow rapid iteration of route and support concepts while preserving a robust physical model.
6. Support optional agent-assisted model operations without allowing agents to bypass schema validation or user review.
7. Produce fundamental mechanical results: displacements, rotations, forces, moments, reactions, terminal loads, and stress quantities.
8. Evaluate user-supplied rule packs without shipping protected standards content.
9. Save named model states and analysis runs.
10. Provide a deterministic comparison tool for two model states and/or two analysis runs.
11. Produce schema-compliant handoff packages for the user's downstream modeling engine and, later, selected external exchange formats.
12. Generate auditable reports that identify assumptions, warnings, data sources, versions, model states, analysis runs, and rule-pack references.
13. Maintain strict IP/data boundaries and professional-claim boundaries.
14. Build confidence through verification, validation, regression tests, and transparent documentation.

### 6.2 Secondary Goals

1. Support import/export pathways for plant CAD, BIM, CAE, and professional stress-analysis tools.
2. Support optional external-result states in the future so that prover-tool results can be compared with internal results when the user supplies compatible structured data.
3. Support candidate route/support generation and ranking based on constraints and analytical consequences.
4. Support educational and research examples using invented, public-domain, or permissively licensed data.
5. Support manufacturer or owner data packs when redistribution rights are documented.
6. Support local analysis handoff to external FEA tools for nozzles, lugs, trunnions, branches, or other local details.

---

## 7. Non-Goals

OpenPipeStress shall not:

1. Claim to be an industry-standard professional stress-analysis prover tool.
2. Claim ASME, API, ISO, EN, CSA, or other standards-body approval, certification, endorsement, or official interpretation.
3. Ship protected standards text, tables, figures, examples, proprietary formulas, material allowable tables, code-specific load combinations, or copyrighted commercial software examples.
4. Automatically determine code compliance for professional reliance.
5. Replace the responsible engineer, owner specification, project design basis, licensed standards, or professional judgment.
6. Force a formal prover-status lifecycle in the MVP.
7. Implement comprehensive parsing or ingestion of external commercial stress-analysis outputs in the MVP.
8. Depend on cloud services for modeling, solving, rule checking, reporting, or comparison.
9. Treat global piping stress analysis as routine 3D solid finite element analysis.
10. Certify fabrication, construction, inspection, examination, testing, or operating fitness.

---

## 8. Core Product Concepts

### 8.1 Design Knowledge

Design knowledge is user-supplied or organization-supplied information that informs model generation and evaluation. It may include:

- line list data;
- P&ID-derived connectivity;
- equipment and nozzle locations;
- equipment movement or nozzle-load sensitivity metadata;
- pipe specifications;
- design and operating pressures/temperatures;
- routing corridors;
- no-go volumes;
- pipe rack elevations;
- supportable structural zones;
- valve accessibility requirements;
- slope, drain, and vent requirements;
- insulation and clearance envelopes;
- construction and modularization constraints;
- owner preferences;
- company support standards;
- stress-review triggers;
- private rule-pack references.

### 8.2 Physical Model

The physical model is the user's editable design model. It includes geometry, components, supports, equipment interfaces, loads, materials, constraints, libraries, provenance, and unresolved assumptions.

The physical model is richer than the analytical model. It may contain design intent and constructability information that is not directly solved by the structural solver.

### 8.3 Analytical Model

The analytical model is the idealized piping flexibility/stress model produced from the physical model. It includes nodes, elements, restraints, loads, load cases, material properties, section properties, SIF/flexibility data, and other quantities required for solving.

### 8.4 Handoff Model

The handoff model is a target-specific representation exported from the physical and analytical models. It is used to feed the user's downstream modeling engine and, later, external professional stress-analysis tools.

### 8.5 Rule Pack

A rule pack is a user-supplied or organization-supplied artifact that defines required inputs, stress categories, load-case mappings, expressions, allowables, checks, warnings, and blocking conditions.

The public project shall provide schemas and invented examples, not protected code rules.

### 8.6 Model State

A model state is a named, immutable snapshot of a model at a point in time. It records the physical model, analysis inputs, references, tags, notes, unresolved assumptions, and model hash.

### 8.7 Analysis Run

An analysis run is a solver execution attached to a specific model state. It records solver version, settings, load cases, diagnostics, rule-pack references, library references, and results.

### 8.8 Comparison

A comparison is a deterministic diff between two model states and/or two analysis runs. It uses stable IDs, user-defined mappings, unit-normalized quantities, and tolerance profiles.

The comparison tool is generic. It is not, in the MVP, a comprehensive integration with an external prover tool.

### 8.9 External Prover Tool

An external prover tool is an accepted industry-standard professional stress-analysis platform used by the responsible engineer or organization to validate a design for project reliance.

OpenPipeStress shall help produce handoff-ready data for such tools, but it shall not claim that internal results alone replace them.

---

## 9. Target Users and Personas

### 9.1 Piping Designer / Layout Engineer

Needs to explore route and support concepts quickly while maintaining a physically coherent model.

Key needs:

- graphical route editing;
- support placement;
- constraint visualization;
- accessibility and clearance checks;
- route/support variants;
- fast analytical feedback;
- handoff-ready data.

### 9.2 Piping Stress Engineer

Needs a serious analytical engine for design iteration before validating final results in a professional tool.

Key needs:

- 3D centerline model;
- reliable loads, supports, and components;
- displacements, forces, moments, reactions, terminal loads, and stress results;
- user-defined rule packs;
- warning and missing-data discipline;
- model-state comparison;
- export/handoff support.

### 9.3 Owner-Operator Reviewer

Needs traceable, reviewable design basis and model evolution.

Key needs:

- assumptions and provenance;
- model states;
- comparison reports;
- rule-pack checksums;
- clear boundaries between design-engine outputs and professional acceptance.

### 9.4 Engineering Company Administrator

Needs to manage private data and templates.

Key needs:

- private material libraries;
- private component libraries;
- private rule packs;
- project templates;
- report templates;
- access control and redaction;
- contribution and IP boundaries.

### 9.5 Open-Source Contributor

Needs to contribute mechanics, schemas, GUI features, validation cases, and documentation without introducing protected content.

Key needs:

- clear architecture;
- tests;
- contribution policy;
- IP review gates;
- public examples using lawful data only.

### 9.6 Agent-Enhanced User

Needs to interact with model-generation or route/support proposal agents while retaining deterministic model control.

Key needs:

- agent proposals as structured operations;
- visible diffs;
- user review gates;
- schema validation;
- undo/redo;
- audit trail.

---

## 10. Core User Journeys

### 10.1 Create a Route and Support Concept

1. User creates a project and selects units.
2. User imports or enters design knowledge: endpoints, line metadata, pipe spec references, design temperature/pressure metadata, routing corridors, no-go volumes, support zones, and equipment interfaces.
3. User creates a physical model manually or through assisted generation.
4. User places pipe route segments, bends, components, branches, and supports.
5. Software validates schema, connectivity, clearances, missing inputs, and physical plausibility.
6. User saves a named model state.

### 10.2 Run Internal Analytical Design Evaluation

1. User derives or updates the analytical model from the physical model.
2. User defines load cases, combinations, and analysis settings.
3. User supplies required material, component, SIF, flexibility, and rule-pack data.
4. User runs the internal solver.
5. Software reports displacements, rotations, forces, moments, reactions, terminal loads, fundamental stresses, user-rule checks, warnings, and diagnostics.
6. User saves the analysis run.

### 10.3 Iterate with GUI or Agent Assistance

1. User identifies an issue: excessive displacement, reaction, terminal load, stress indicator, clearance conflict, missing support, poor routing, or external review feedback.
2. User edits the model through the GUI or requests an agent proposal.
3. Proposed changes are represented as structured model operations.
4. Software validates the operation and displays the diff.
5. User accepts, rejects, or modifies the proposal.
6. User saves a new model state and reruns analysis.

### 10.4 Validate Externally and Reflect Changes Internally

1. User exports a handoff package or recreates the model in an external professional stress-analysis tool.
2. User interprets the external tool output outside OpenPipeStress.
3. User determines what needs to change.
4. User modifies the OpenPipeStress model through the GUI.
5. User saves a new model state.
6. User runs the internal analysis again.
7. User compares the pre-review and post-review states/runs using the generic comparison tool.

### 10.5 Compare Two Model States or Analysis Runs

1. User selects Model State A and Model State B.
2. User optionally selects Analysis Run A and Analysis Run B.
3. Software maps entities using stable IDs.
4. User manually maps, unmaps, or marks unmatched entities as needed.
5. Software compares inputs and results using unit normalization and tolerance profiles.
6. User reviews tabular and graphical differences.
7. User exports a comparison report.

### 10.6 Build a Private Rule Pack

1. User opens the rule-pack editor.
2. User defines required inputs, load-case mappings, expressions, allowables, and warnings using licensed or private sources.
3. Software validates units, references, and required fields.
4. Software marks the rule pack private by default.
5. User applies the rule pack to project analysis runs.

---

## 11. Functional Requirements

### 11.1 Model and Schema Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-MOD-001 | Provide canonical project schema | Must | Project files serialize and deserialize without loss of model data, units, references, tags, notes, or warnings. |
| FR-MOD-002 | Support schema-backed physical model | Must | Lines, nodes, elements, supports, components, loads, materials, constraints, and equipment interfaces are represented as typed entities. |
| FR-MOD-003 | Preserve stable entity IDs | Must | Entity IDs persist across edits unless user intentionally creates new entities. |
| FR-MOD-004 | Support user-visible names, tags, and notes | Must | Users can label states/models using flexible naming conventions without enforced prover status. |
| FR-MOD-005 | Support provenance fields | Must | Materials, components, SIFs, flexibility factors, allowables, rules, and library records can record source and verification metadata. |
| FR-MOD-006 | Support unresolved assumptions | Must | Users can record assumptions and open issues at project, line, component, support, state, and run levels. |
| FR-MOD-007 | Support physical-to-analytical transformation | Must | The product can derive a solver-ready analytical model from the physical model and record transformation warnings. |

### 11.2 Design Knowledge and Constraint Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-KNOW-001 | Import or enter design knowledge | Must | Users can define endpoints, line data, route constraints, support zones, and design metadata. |
| FR-KNOW-002 | Validate connectivity | Must | Software detects disconnected routes, invalid branch topology, and unsupported model topology. |
| FR-KNOW-003 | Validate constraints | Should | Software can flag no-go volume conflicts, clearance issues, slope conflicts, and support-zone violations when relevant data exists. |
| FR-KNOW-004 | Maintain constraint provenance | Should | Each constraint can identify its source: user, project standard, owner requirement, imported data, or agent proposal. |
| FR-KNOW-005 | Generate candidate route/support operations | Could | Software can propose route/support alternatives using recorded constraints and model context. |

### 11.3 GUI Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-GUI-001 | Provide 3D model viewport | Must | Users can view and select piping centerlines, components, supports, labels, loads, deformed shapes, and result overlays. |
| FR-GUI-002 | Provide model tree | Must | Users can navigate lines, nodes, elements, components, supports, load cases, states, and analysis runs. |
| FR-GUI-003 | Provide property inspector | Must | Selecting an entity shows editable typed properties, units, provenance, warnings, and dependencies. |
| FR-GUI-004 | Support direct model editing | Must | Users can add, modify, delete, move, and reconnect model entities while preserving schema validation. |
| FR-GUI-005 | Support undo/redo | Should | User can undo and redo model operations with deterministic state recovery. |
| FR-GUI-006 | Show validation messages | Must | GUI distinguishes solve blockers, rule-check blockers, warnings, and informational notes. |
| FR-GUI-007 | Show comparison views | Should | GUI can show side-by-side tables, changed entities, result deltas, and graphical overlays for comparisons. |

### 11.4 Analytical Engine Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-SOL-001 | Support six-degree-of-freedom nodes | Must | Solver supports Ux, Uy, Uz, Rx, Ry, and Rz at every active node. |
| FR-SOL-002 | Implement 3D frame/line-element analysis | Must | Solver passes benchmark tests for beams, frames, coordinate transformations, thermal loads, and imposed displacements. |
| FR-SOL-003 | Support straight pipe elements | Must | Straight pipe elements support axial, bending, torsional, weight, thermal, and stress-recovery behavior. |
| FR-SOL-004 | Support rigid elements | Must | Rigid components and offsets can transfer forces and moments correctly in validation cases. |
| FR-SOL-005 | Support linear supports/restraints | Must | Anchors, guides, directional restraints, and linear springs can be modeled and solved. |
| FR-SOL-006 | Support load cases | Must | Weight, thermal, pressure metadata, concentrated loads, moments, and imposed displacements can be defined and solved. |
| FR-SOL-007 | Recover forces and moments | Must | Element-end forces/moments are available in local and global coordinates. |
| FR-SOL-008 | Recover fundamental stresses | Must | Axial, bending, torsional, pressure-derived, and combined stress quantities are recoverable before rule evaluation. |
| FR-SOL-009 | Support load-case algebra | Must | Users can define combination/range cases from primitive solved cases. |
| FR-SOL-010 | Support bends and elbows | Should | Bend objects accept radius, angle, orientation, user-supplied SIFs, and flexibility factors. |
| FR-SOL-011 | Support branches | Should | Branch objects accept topology, header/branch geometry, and user-supplied local modifiers. |
| FR-SOL-012 | Support valves, flanges, reducers, and specialty components | Should | Components can be modeled as rigid or semi-rigid entities with user-supplied dimensions, weights, and source notes. |
| FR-SOL-013 | Support expansion joints | Should | Expansion joints accept stiffnesses, effective pressure area, movement limits, and manufacturer provenance. |
| FR-SOL-014 | Support nonlinear restraints | Should | Gaps, one-way supports, lift-off, and friction converge on validation cases and report diagnostics. |
| FR-SOL-015 | Report numerical diagnostics | Must | Solver reports singularities, ill-conditioning, residuals, convergence issues, and invalid elements. |

### 11.5 Rule-Pack and Private Data Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-RULE-001 | Provide rule-pack schema | Must | Users can define private required inputs, stress categories, expressions, allowables, load mappings, and blocking conditions. |
| FR-RULE-002 | Provide unit-aware expression evaluator | Must | Expressions reject incompatible units and run without arbitrary filesystem/network access. |
| FR-RULE-003 | Block incomplete rule checks | Must | Software refuses to report pass/fail when required rule-pack values are missing. |
| FR-RULE-004 | Mark private rule packs by default | Must | User-created rule packs are excluded from public examples, telemetry, and issue bundles unless explicitly exported. |
| FR-RULE-005 | Support private material libraries | Must | Users can define and import materials with units, temperature-dependent properties, allowables, and provenance. |
| FR-RULE-006 | Support private component libraries | Should | Users can define and import components with dimensions, weights, stiffnesses, metadata, and provenance. |
| FR-RULE-007 | Provide invented public examples only | Must | Bundled examples contain invented, public-domain, or permissively licensed values only. |

### 11.6 Model State, Analysis Run, and Comparison Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-CMP-001 | Save named immutable model states | Must | User can save, reopen, tag, and compare model snapshots without mutation. |
| FR-CMP-002 | Save analysis runs against model states | Must | Each result set links to exact model state, solver version, settings, units, load cases, and rule-pack/library references. |
| FR-CMP-003 | Compare two model states | Must | Tool reports added, removed, unchanged, and modified entities. |
| FR-CMP-004 | Compare two analysis runs | Must | Tool reports result deltas for mapped nodes, elements, supports, terminals, and stress/result locations. |
| FR-CMP-005 | Provide mapping table editor | Should | User can map, unmap, and mark unmatched entities intentionally unmatched. |
| FR-CMP-006 | Use stable IDs automatically | Must | Stable IDs are used as the default mapping when available. |
| FR-CMP-007 | Support manual mappings | Should | User can map entities that changed IDs or topology between states. |
| FR-CMP-008 | Support tolerance profiles | Should | User can define absolute and relative tolerances by result category. |
| FR-CMP-009 | Show tabular and graphical diffs | Should | Tool displays changed inputs, changed settings, changed results, and graphical overlays. |
| FR-CMP-010 | Export comparison report | Should | Comparison can be exported as CSV, JSON, and report section. |
| FR-CMP-011 | Allow future external result states | Could | Architecture can represent imported or manually entered external results as comparable result states without changing core compare logic. |

### 11.7 Handoff and Export Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-HAND-001 | Export schema-compliant model package | Must | User can export canonical model data for the downstream modeling engine. |
| FR-HAND-002 | Include handoff manifest | Must | Export includes model hash, units, entity IDs, library references, rule references, warnings, and unresolved assumptions. |
| FR-HAND-003 | Include target mapping metadata | Should | Export records how internal entities map to target model fields. |
| FR-HAND-004 | Identify unsupported target behavior | Should | Export flags items that cannot be represented exactly in the target schema. |
| FR-HAND-005 | Support future professional-tool exchange formats | Could | Architecture allows later PCF or target-specific exporters where legally and technically appropriate. |

### 11.8 Agent-Assisted Design Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-AGENT-001 | Represent agent output as proposed operations | Should | Agent proposals are structured operations, not direct unsupervised model mutations. |
| FR-AGENT-002 | Validate agent operations | Should | Proposed operations must pass schema and constraint validation before application. |
| FR-AGENT-003 | Require user acceptance | Should | Agent-proposed changes are shown as diffs and require user acceptance unless the user configures another workflow. |
| FR-AGENT-004 | Record rationale and assumptions | Should | Accepted operations record rationale, constraints considered, and unresolved assumptions. |
| FR-AGENT-005 | Preserve professional boundary | Must | Agent output cannot claim engineering acceptance, certification, or code compliance for reliance. |

### 11.9 Reporting Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---:|---|
| FR-REP-001 | Generate design/analysis report | Must | Report includes inputs, versions, states, runs, assumptions, warnings, results, and rule-pack references. |
| FR-REP-002 | Include professional-boundary notice | Must | Report distinguishes mechanics results, user-rule checks, and human review. |
| FR-REP-003 | Include provenance summary | Must | Report identifies sources or source notes for user-supplied data where available. |
| FR-REP-004 | Include comparison report sections | Should | User can include model/result comparison summaries in reports. |
| FR-REP-005 | Avoid protected content in public templates | Must | Public templates do not embed standards text, protected tables, proprietary formulas, or private rule data. |

---

## 12. Analytical Engine Requirements

### 12.1 Core Analysis Model

The solver shall use a 3D line-element representation of the piping centerline. Each active structural node shall support six degrees of freedom:

```text
Ux, Uy, Uz, Rx, Ry, Rz
```

The initial formulation shall be linear elastic, small-displacement, static 3D frame analysis. Later modules may add geometric nonlinearity, dynamics, or specialized component behavior.

### 12.2 Solver Capabilities

Minimum capabilities:

- sparse global stiffness matrix assembly;
- local-to-global coordinate transformations;
- boundary-condition enforcement;
- imposed displacement handling;
- thermal initial strain handling;
- distributed and concentrated load handling;
- rigid link / rigid element support;
- element-end force and moment recovery;
- local and global result extraction;
- unit-aware result storage;
- singularity detection;
- ill-conditioning detection;
- deterministic output for a fixed model, solver version, and settings.

### 12.3 Element Types

#### Straight Pipe Elements

Straight pipe elements shall support:

- axial stiffness;
- bending stiffness about two local transverse axes;
- torsional stiffness;
- distributed pipe, contents, insulation, and component weight;
- thermal strain;
- pressure metadata;
- local coordinate systems;
- stress recovery at ends and optional stations.

#### Bend / Elbow Objects

Bend objects shall support:

- bend radius;
- bend angle;
- bend plane orientation;
- optional segmented visualization;
- user-supplied in-plane SIF;
- user-supplied out-of-plane SIF;
- user-supplied torsional SIF where applicable;
- user-supplied flexibility factors;
- user-supplied source/provenance for each local modifier.

The public project shall not ship protected B31J or standards-derived SIF/flexibility data.

#### Branch Connection Objects

Branch objects shall support:

- header association;
- branch association;
- intersection node;
- branch angle;
- reinforcement metadata;
- user-supplied header-side modifiers;
- user-supplied branch-side modifiers;
- provenance for local data.

#### Rigid and Semi-Rigid Components

Components such as valves, flanges, strainers, instruments, specialty items, and equipment stubs shall support:

- length or offsets;
- weight;
- center of gravity;
- orientation;
- rigid or semi-rigid behavior;
- optional stiffness values;
- source/provenance.

#### Reducers

Reducers shall support:

- concentric or eccentric geometry;
- upstream/downstream section assignments;
- length;
- weight;
- eccentric offset vector;
- optional user-supplied local modifiers.

#### Expansion Joints

Expansion joints shall support:

- axial stiffness;
- lateral stiffness;
- angular stiffness;
- torsional stiffness;
- effective pressure area;
- pressure thrust;
- movement limits;
- tie rod, hinge, gimbal, or limit rod metadata;
- manufacturer data provenance.

### 12.4 Support and Restraint Types

The product shall support, over time:

- anchor;
- guide;
- line stop;
- vertical support;
- directional restraint;
- translational spring;
- rotational spring;
- constant-effort support;
- variable spring hanger;
- rigid strut;
- gap support;
- one-way support;
- lift-off;
- friction support;
- sliding support.

The MVP may start with anchors, guides, vertical supports, directional restraints, and linear springs.

### 12.5 Load Types

The product shall support:

- pipe dead weight;
- contents weight;
- insulation, lining, refractory, and coating weight;
- component weight;
- internal pressure metadata;
- temperature and thermal expansion;
- concentrated forces;
- concentrated moments;
- imposed displacements;
- imposed rotations;
- equipment/nozzle movement;
- hydrotest load;
- wind load;
- seismic static-equivalent load;
- user-defined occasional load.

### 12.6 Load Cases and Combination Cases

The software shall distinguish:

1. **Primitive analysis cases** solved directly by the analytical engine.
2. **Combination or range cases** produced by algebraic operations on solved cases.
3. **Rule-check cases** defined by user rule packs.

The public software may provide a generic load-case builder but shall not ship protected code-specific load combinations.

### 12.7 Fundamental Stress Recovery

Before any code-specific evaluation, the software shall recover fundamental mechanics quantities including:

- axial force;
- shear forces;
- torsional moment;
- local bending moments;
- resultant bending moment;
- axial stress;
- bending stress;
- torsional shear stress;
- pressure-derived hoop stress if selected;
- pressure-derived longitudinal stress if selected;
- optional principal stress;
- optional von Mises equivalent stress;
- optional Tresca equivalent stress.

Code-specific stress formulas shall be evaluated only through user rule packs.

### 12.8 Numerical Quality Requirements

The analytical engine shall:

- detect unconstrained rigid-body modes;
- detect duplicate or near-duplicate nodes;
- detect zero-length or invalid elements;
- detect missing physical data required for solve;
- report solver residuals and convergence diagnostics;
- store inputs and outputs with sufficient precision for engineering review;
- produce reproducible results for a fixed model state and solver version.

---

## 13. Design Knowledge, Constraint, and Candidate Generation Requirements

### 13.1 Design Knowledge Layer

The product shall support design knowledge as explicit data rather than hidden assumptions.

Design knowledge records may include:

- source;
- scope;
- applicable lines/equipment/zones;
- unit system;
- required fields;
- confidence or verification status;
- private/public status;
- unresolved assumptions.

### 13.2 Constraint Engine

The constraint engine shall evaluate model consistency and project constraints, including where data exists:

- route connectivity;
- endpoint matching;
- supportable span ranges;
- clearance envelopes;
- no-go volumes;
- slope requirements;
- drain/vent logic;
- valve access zones;
- supportable structure zones;
- equipment interface consistency;
- unresolved or missing required data.

Constraint failures shall be shown as validation messages, not hidden in agent text or reports alone.

### 13.3 Candidate Generation

In future releases, the product may propose route/support alternatives. Candidate generation shall be constrained by the physical model, schema, available design knowledge, and analytical results.

Candidate generation shall produce model operations such as:

```yaml
operation:
  type: add_support
  target_line: L-1001
  proposed_node: N-024
  support_type: guide
  rationale: "Reduce lateral displacement in long thermal run"
  constraints_considered:
    - rack_support_zone_A
    - clearance_zone_CZ-07
  unresolved_assumptions:
    - final guide location requires structural support confirmation
```

---

## 14. GUI Requirements

### 14.1 Main Interface

The GUI shall include:

- 3D viewport;
- model tree;
- property inspector;
- operation history;
- design knowledge panel;
- constraint/warning panel;
- material library editor;
- component library editor;
- load-case manager;
- rule-pack manager;
- solver console;
- results browser;
- model state and analysis run browser;
- comparison browser;
- report generator.

### 14.2 3D Viewport

The viewport shall display:

- pipe centerlines;
- bend arcs;
- branches;
- valves;
- flanges;
- reducers;
- expansion joints;
- equipment interfaces;
- supports and restraints;
- load vectors;
- route corridors;
- no-go volumes;
- supportable zones;
- node/element labels;
- deformed shapes;
- reaction arrows;
- stress/result overlays;
- model-state overlay comparisons where available.

### 14.3 Property Inspector

The property inspector shall show:

- entity type;
- geometry;
- units;
- material;
- section;
- component properties;
- loads;
- support settings;
- SIF/flexibility inputs;
- source/provenance;
- validation status;
- dependent rule-pack inputs;
- linked assumptions and notes.

### 14.4 Warning and Blocking UX

The GUI shall distinguish:

- **Blocking for model validity:** schema or topology invalid;
- **Blocking for solve:** physical data required for mechanics missing;
- **Blocking for user-rule check:** mechanics can solve, but required rule-pack data is missing;
- **Warning:** data exists but is incomplete, unverified, unusual, or outside configured ranges;
- **Informational:** assumption or note recorded for review.

### 14.5 Comparison UI

The comparison UI shall show:

- selected states/runs;
- mapping table;
- unmatched entities;
- added/deleted/modified entities;
- changed solver settings;
- changed rule-pack/library references;
- result deltas;
- out-of-tolerance items;
- graphical overlays;
- export options.

---

## 15. Model State, Analysis Run, and Comparison Management

### 15.1 Model States

A model state shall be immutable after creation. Users may create new states from existing states, but historical states shall remain reproducible.

A model state shall record:

- state ID;
- user-visible name;
- optional tags;
- optional notes;
- project/model hash;
- timestamp;
- units;
- entity graph;
- materials and library references;
- components and library references;
- load definitions;
- constraints;
- warnings;
- unresolved assumptions;
- external references entered by the user.

### 15.2 Analysis Runs

An analysis run shall record:

- run ID;
- associated model state ID;
- solver version;
- build/release identifier;
- analysis settings;
- active load cases;
- combination cases;
- rule-pack references;
- material/component library references;
- solve diagnostics;
- results;
- warnings;
- timestamp.

### 15.3 Generic Comparison Tool

The comparison tool shall compare two model states and optionally two analysis runs. It shall not require or enforce a formal prover lifecycle.

The tool shall compare model inputs such as:

- geometry;
- topology;
- nodes;
- elements;
- branches;
- bends;
- components;
- supports;
- loads;
- materials;
- sections;
- rule-pack references;
- solver settings;
- constraints;
- warnings;
- unresolved assumptions.

The tool shall compare analysis outputs such as:

- node displacements;
- node rotations;
- element forces;
- element moments;
- support reactions;
- equipment terminal loads;
- stress quantities;
- governing locations;
- user-rule check outputs where applicable;
- solve diagnostics.

### 15.4 Entity Mapping

The comparison tool shall support mapping for:

- nodes;
- elements;
- supports/restraints;
- components;
- equipment terminals;
- load cases;
- combination cases;
- result stations;
- stress/result quantities.

Mapping hierarchy:

1. stable ID match;
2. user-defined manual mapping;
3. unmatched / added / deleted classification;
4. optional future geometric/topological suggested matching.

### 15.5 Tolerance Profiles

Users shall be able to define absolute and relative tolerances by result category. Example categories:

- displacement;
- rotation;
- force;
- moment;
- reaction;
- stress;
- stress ratio;
- temperature;
- pressure;
- support gap or movement.

### 15.6 Future External Result State

The architecture should allow a future external result state object. This would let users manually enter or import selected results from a professional prover tool and compare them using the same generic comparison engine.

This is not an MVP requirement.

---

## 16. Handoff and External Prover Workflow

### 16.1 Handoff Package

The product shall produce a handoff package that includes:

- canonical internal model file;
- target modeling-engine input file where supported;
- units manifest;
- entity ID manifest;
- material/component reference manifest;
- load-case manifest;
- support/restraint manifest;
- rule-pack reference manifest;
- unresolved assumptions;
- warnings;
- known target limitations;
- model hash;
- export hash;
- optional comparison references.

### 16.2 External Prover Workflow

The intended workflow is:

```text
OpenPipeStress model state
  -> internal analysis run
  -> handoff/export package
  -> external professional stress-analysis tool
  -> user interprets external results
  -> user modifies OpenPipeStress model through GUI
  -> new model state
  -> new internal analysis run
  -> generic comparison of states/runs
```

The software shall support this workflow without requiring comprehensive external output ingestion in the MVP.

### 16.3 Naming, Tags, and Notes Instead of Forced Prover Status

The MVP shall not enforce a formal prover-status vocabulary or lifecycle. Instead, users shall be able to use:

- model names;
- state names;
- tags;
- notes;
- external reference fields;
- attachments or links where supported;
- comparison reports.

Example user naming conventions:

```text
P-1001_RouteA_State001_initial
P-1001_RouteA_State002_after_external_review
P-1001_RouteB_State001_supports_revised
P-1001_RouteA_State003_ready_for_handoff
```

---

## 17. Rule-Pack Requirements

### 17.1 Purpose

The rule-pack system lets users evaluate internal analysis results against user-supplied, project-specific, company-specific, or code-specific criteria without the public project distributing protected standards content.

### 17.2 Rule-Pack Contents

A rule pack shall support:

- name;
- version;
- private/public status;
- design basis text fields;
- source/provenance statement;
- required material inputs;
- required component inputs;
- required model inputs;
- required load cases;
- stress categories;
- load-case mappings;
- expressions;
- allowables;
- warnings;
- blocking conditions;
- unit expectations;
- checksum.

### 17.3 Expression Evaluator

The expression evaluator shall be:

- unit-aware;
- deterministic;
- sandboxed;
- free of arbitrary network/filesystem access;
- able to evaluate algebra, conditionals, interpolation, min/max, and table lookup using user-provided data.

### 17.4 Public vs Private Rule Packs

Public example rule packs may include invented, non-code examples only.

Private rule packs may represent licensed codes, owner standards, or company practices, but the user is responsible for lawful use and redistribution.

### 17.5 Missing Data Behavior

The software may run a mechanics-only solve if sufficient physical data exists. It shall not report user-rule pass/fail results unless all rule-required values are present.

---

## 18. Material and Component Library Requirements

### 18.1 Material Libraries

Material records shall support:

- material name;
- specification/grade label;
- density;
- Poisson's ratio;
- elastic modulus versus temperature;
- thermal expansion data;
- allowable stress data where user-supplied;
- yield/ultimate strength data where user-supplied;
- corrosion allowance;
- mill tolerance;
- source/provenance;
- verification status;
- private/public status.

### 18.2 Pipe Section Libraries

Pipe section records shall support:

- size label;
- outside diameter;
- wall thickness;
- schedule/designation text;
- corrosion allowance;
- mill tolerance;
- lining, insulation, or coating metadata;
- source/provenance.

The public project shall not ship protected dimensional tables unless redistribution rights are documented.

### 18.3 Component Libraries

Component records shall support:

- type;
- manufacturer;
- model;
- size/rating label;
- end connection;
- face-to-face or end-to-end length;
- weight;
- center of gravity;
- stiffness behavior;
- effective pressure area if applicable;
- movement limits if applicable;
- source/provenance;
- redistribution status.

### 18.4 Import Warnings

Imported data shall be flagged if:

- required fields are missing;
- units are missing or inconsistent;
- provenance is missing;
- redistribution status is unclear;
- values appear to be protected standards data;
- values are outside user-defined reasonableness ranges.

---

## 19. Reporting and Audit Requirements

### 19.1 Report Types

The product shall support:

- model summary report;
- analysis result report;
- warning and assumption report;
- user-rule check report;
- model state comparison report;
- analysis run comparison report;
- handoff package manifest.

### 19.2 Required Report Content

Reports shall include, where applicable:

- project name;
- software version;
- solver version;
- build/release identifier;
- date/time;
- units;
- coordinate system;
- model state ID/name/hash;
- analysis run ID/name;
- material/component library references;
- rule-pack name/version/checksum;
- source/provenance summary;
- warnings;
- unresolved assumptions;
- load cases;
- solver settings;
- solver diagnostics;
- displacements;
- rotations;
- forces and moments;
- support reactions;
- equipment terminal loads;
- fundamental stresses;
- user-rule checks;
- comparison results where selected;
- review/signoff block.

### 19.3 Required Notice

Reports shall include a notice substantially equivalent to:

```text
OpenPipeStress is decision-support software for piping design, flexibility, and stress-analysis workflows. It computes mechanical results from recorded user inputs and may evaluate user-supplied rule packs. It does not certify, seal, approve, authenticate, or determine code compliance for professional reliance. Code-specific and project-specific data are supplied by the user or user-controlled private sources. Competent human review and, where required, validation in accepted professional tools remain the responsibility of the user and project authority.
```

### 19.4 Report Prohibitions

Public report templates shall not include protected standards text, protected tables, proprietary formulas, private rule-pack content, or private project data.

---

## 20. IP, Data, Security, and Privacy Requirements

### 20.1 Public Repository Data Boundary

The public repository may contain:

- open solver algorithms;
- geometry, units, loads, stress-recovery, and report schemas;
- blank templates;
- invented example data;
- public-domain or permissively licensed data with provenance;
- validation benchmarks from lawful sources;
- private-library import mechanisms.

The public repository must not contain:

- protected standards text, tables, examples, figures, or commentary;
- material allowable tables copied from standards;
- protected SIF/flexibility content;
- protected dimensional or rating tables;
- proprietary vendor catalogs without permission;
- commercial software examples or report templates without permission;
- private company data;
- code-specific acceptance criteria unless invented or cleared for public redistribution.

### 20.2 Provenance Requirements

Every public data record shall include:

- source name;
- source location or identifier;
- source license;
- contributor;
- contributor certification;
- redistribution status;
- review status.

Unclear or suspicious data shall be quarantined and not used as public fixture data.

### 20.3 Private Data Handling

Private rule packs, material libraries, component catalogs, owner standards, and project design bases shall be stored in user-controlled paths and excluded from telemetry, public examples, and issue bundles by default.

### 20.4 Local-First Operation

Modeling, solving, rule evaluation, reporting, comparison, and export shall work locally without a cloud service.

### 20.5 Telemetry

Telemetry shall be disabled by default. If added, telemetry shall be opt-in and shall not transmit private model, result, material, component, rule-pack, or project data without explicit user action.

---

## 21. Professional Boundary Requirements

### 21.1 Permitted Claims

The product may claim:

- mechanics results computed from recorded inputs;
- user-rule checks computed from user-supplied rules and data;
- diagnostics, warnings, assumptions, and limitations;
- provenance, version, checksum, and hash records;
- validation and regression evidence for software behavior;
- report generation for competent human review;
- local-first private-data handling where implemented.

### 21.2 Prohibited Claims

The product must not claim or imply that it:

- certifies engineering work;
- seals engineering work;
- approves engineering work;
- authenticates engineering work;
- declares code compliance for professional reliance;
- replaces the engineer of record;
- replaces project-specific professional review;
- provides standards-body approval or official interpretation;
- makes public example data suitable for project use.

### 21.3 Human Acceptance

Any human acceptance record, if stored in future releases, shall be separate from solver outputs, rule-pack outputs, comparison outputs, and report generation.

MVP shall not require a formal acceptance workflow.

---

## 22. Verification and Validation Requirements

### 22.1 Verification Philosophy

Verification proves the equations are implemented correctly. Validation demonstrates that the modeling approach produces credible engineering results within stated assumptions.

OpenPipeStress shall earn trust through transparent benchmarks, regression tests, and documented tolerances, not through unsupported equivalence claims.

### 22.2 Required Solver Benchmarks

The validation suite shall include:

- cantilever beam with tip force;
- cantilever beam with tip moment;
- fixed-fixed thermal beam;
- simply supported pipe under weight;
- inclined-member coordinate transformation;
- L-shaped thermal expansion frame;
- U-loop thermal expansion case;
- torsion-only element;
- rigid link case;
- imposed anchor displacement case;
- linear spring support case;
- gapped support case;
- friction support case;
- branch assembly case.

### 22.3 Stress Recovery Benchmarks

The validation suite shall include:

- axial stress from axial force;
- bending stress from moment;
- torsional shear from torque;
- pressure-derived hoop stress;
- pressure-derived longitudinal stress;
- stress range from two states;
- user-entered SIF multiplier behavior;
- user-entered flexibility-factor behavior.

### 22.4 Regression Testing

Every release shall run automated tests for:

- unit conversion;
- schema round-trip;
- matrix assembly;
- coordinate transformation;
- boundary conditions;
- load application;
- thermal strain;
- force recovery;
- stress recovery;
- rule-pack evaluation;
- nonlinear convergence where applicable;
- model state hashing;
- analysis run reproducibility;
- comparison output determinism;
- report reproducibility.

### 22.5 Prover Correlation as Future/Optional Validation Evidence

The product may support project-private or lawfully publishable comparisons against external professional tools. Such comparisons shall identify:

- internal model state hash;
- exported model hash;
- target tool and version where disclosed;
- import/export assumptions;
- manual changes;
- load-case mapping;
- support/restraint mapping;
- result comparison basis;
- tolerance basis;
- discrepancy disposition.

These comparisons are validation evidence, not automatic professional acceptance.

---

## 23. MVP Scope

The MVP shall establish the canonical schema, physical model, analytical core, GUI basics, rule-pack/data boundary, model states, analysis runs, and deterministic comparison.

MVP must include:

- canonical project schema;
- local-first storage;
- unit system;
- 3D nodes with six degrees of freedom;
- straight pipe/beam elements;
- rigid elements;
- anchors and linear restraints;
- weight loads;
- thermal loads;
- pressure metadata;
- imposed displacements;
- linear static solver;
- element force/moment recovery;
- fundamental stress recovery;
- basic GUI modeler;
- model tree and property inspector;
- material and section entry;
- private rule-pack schema with non-code examples;
- missing-data blockers;
- named immutable model states;
- analysis runs tied to states;
- basic model-state comparison;
- basic analysis-run comparison;
- export of canonical schema-compliant model package;
- basic report generation;
- initial validation suite;
- IP/data-boundary notices;
- professional-boundary notices.

MVP shall not include:

- comprehensive external commercial tool result ingestion;
- forced prover-status lifecycle;
- protected standards data;
- automatic professional acceptance records;
- advanced nonlinear/dynamic modules unless otherwise prioritized.

---

## 24. Release Milestones

### R0: Schema and Solver Prototype

Deliverables:

- canonical model schema;
- unit system;
- straight pipe/beam element;
- minimal linear solver;
- command-line input/output;
- cantilever and fixed-fixed thermal benchmarks.

Exit criteria:

- schema round-trips;
- benchmark results match documented references within tolerance;
- singular model detection implemented.

### R1: Core Analytical Engine MVP

Deliverables:

- 3D frame solver;
- six-degree-of-freedom nodes;
- weight, thermal, concentrated load, and imposed displacement cases;
- rigid elements;
- element force/moment recovery;
- fundamental stress recovery;
- validation test suite.

Exit criteria:

- benchmark suite passes;
- solver diagnostics visible;
- results reproducible for fixed input.

### R2: GUI and Physical Model MVP

Deliverables:

- 3D modeler;
- model tree;
- property inspector;
- material/section editors;
- support editor;
- load-case manager;
- result tables;
- deformed shape visualization;
- warning panel.

Exit criteria:

- user can create, solve, review, and save a small model without editing raw files.

### R3: States, Runs, and Generic Comparison

Deliverables:

- named immutable model states;
- analysis runs bound to model states;
- model-state diff;
- analysis-run diff;
- entity mapping editor;
- tolerance profiles;
- comparison export.

Exit criteria:

- user can compare two states and two runs deterministically;
- comparison identifies changed inputs and changed results.

### R4: Rule Packs and Private Libraries

Deliverables:

- private rule-pack schema;
- sandboxed unit-aware expression evaluator;
- private material libraries;
- private component libraries;
- missing-data blockers;
- rule-check results;
- report integration.

Exit criteria:

- user can create a private non-code rule pack and run checks;
- software blocks pass/fail when required values are missing.

### R5: Piping Components and Nonlinear Supports

Deliverables:

- bend objects;
- branch objects;
- valves/flanges/reducers;
- expansion joints;
- spring hangers;
- gaps, lift-off, one-way supports, and friction;
- nonlinear validation cases.

Exit criteria:

- component data appears in reports with provenance;
- nonlinear cases converge or report actionable diagnostics.

### R6: Design Knowledge and Handoff Beta

Deliverables:

- design knowledge import/entry;
- routing corridors and support zones;
- constraint validation;
- handoff package generator;
- target mapping metadata;
- comparison/report enhancements;
- validation manual.

Exit criteria:

- user can produce a schema-compliant handoff package for downstream modeling;
- external reviewers can reproduce validation examples.

### R7: Agent-Assisted Design and Candidate Generation

Deliverables:

- structured operation proposal format;
- agent proposal review UI;
- route/support candidate generation;
- candidate comparison;
- rationale and assumption capture.

Exit criteria:

- agents can propose schema-valid operations;
- user can accept/reject proposals;
- accepted operations are auditable and reversible.

---

## 25. Success Metrics

### 25.1 Technical Metrics

- number of verification benchmarks passed;
- regression-test coverage;
- solver deviation from hand/reference benchmarks;
- unit-conversion test coverage;
- reproducibility across platforms;
- number of critical solver defects open;
- comparison determinism across repeated runs;
- time to solve representative models.

### 25.2 Product Metrics

- time to create and solve a simple route/support model;
- time to save, modify, rerun, and compare two states;
- number of missing-data blockers caught before report issue;
- user satisfaction from designers and stress engineers;
- number of successful handoff package exports;
- number of GUI operations covered by undo/redo;
- documentation completeness.

### 25.3 Governance Metrics

- number of public data contributions with complete provenance;
- number of rejected or quarantined protected-content submissions;
- number of report templates reviewed for professional-boundary language;
- number of releases with complete validation evidence;
- private-data leakage incidents.

---

## 26. Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---:|---|
| Users mistake internal results for professional reliance results | High | Strong UI/report notices, professional-boundary policy, no certification language. |
| Solver errors affect design decisions | High | Verification suite, regression tests, peer review, validation manual, transparent diagnostics. |
| Accidental protected-content inclusion | High | IP/data policy, provenance requirements, quarantine rule, contribution review. |
| Rule packs contain missing or incorrect user data | High | Required-input checks, no silent defaults, provenance warnings, checksums. |
| Comparison tool is interpreted as external validation | Medium/High | Label comparison as diagnostic/audit feature; avoid automatic acceptance/status language. |
| Agent proposes unsafe or invalid changes | Medium/High | Structured operations, schema validation, user acceptance, warnings, audit trail. |
| Export/handoff loses critical assumptions | High | Handoff manifest, model hashes, warnings, unsupported-target flags. |
| External prover workflows become too complex too early | Medium | MVP uses generic state/run comparison; external result states deferred. |
| GUI complexity delays solver validation | Medium | CLI/schema/solver first, GUI layered after core functions. |
| Private data leakage in reports or bug bundles | High | Local-first design, redaction, private-by-default data paths, telemetry off by default. |
| Unit conversion errors | High | Unit-aware model, expression evaluator, tests, report units. |

---

## 27. Open Questions

1. What final project name best captures design-engine + stress-core positioning without implying professional authentication?
2. What open-source license should be used?
3. What language/runtime should implement the core solver?
4. What GUI technology should be used?
5. What schema format should be canonical: JSON Schema, YAML with schema, Protocol Buffers, or another typed format?
6. What should the first target handoff schema be for the separate modeling engine?
7. Which exchange formats should be prioritized after the internal modeling-engine handoff?
8. What tolerance profiles should ship as invented/non-code examples?
9. How should stable entity IDs survive route edits, branch insertion, segmentation, and component replacement?
10. What data should be included in a minimal comparison report?
11. How should private rule packs be stored and optionally encrypted?
12. What validation benchmarks are sufficient for engineering beta?
13. How should future external result states be represented without overfitting to a specific commercial tool?
14. What contribution review workflow is required before public release?

---

## 28. Glossary

**Analysis run:** A recorded solver execution tied to a specific model state, solver version, settings, load cases, and rule-pack/library references.

**Analytical model:** The idealized line-element model solved by the internal piping flexibility/stress engine.

**Design engine:** The software environment that applies design knowledge, physical modeling, constraints, analysis, comparison, and handoff workflows before professional validation.

**Design knowledge:** User-supplied project, owner, routing, support, equipment, accessibility, constructability, and rule information that informs modeling and evaluation.

**External prover tool:** An accepted professional stress-analysis platform used outside OpenPipeStress to validate results for project reliance.

**Handoff package:** Exported schema-compliant model data and manifests intended for downstream modeling or professional stress-analysis workflows.

**Model state:** A named, immutable snapshot of the model and its associated inputs, notes, warnings, assumptions, tags, and references.

**Physical model:** The rich editable piping design model, including geometry, components, supports, constraints, equipment interfaces, provenance, and assumptions.

**Rule pack:** A user-supplied artifact defining required inputs, load mappings, stress expressions, allowables, warnings, and checks.

**User-rule check:** Evaluation of recorded results against user-supplied rules and data. It is not professional approval.

---

## 29. Appendix A: Example Model State Record

```yaml
model_state:
  id: STATE-0018
  name: P-1001_RouteA_State002_after_external_review
  tags:
    - RouteA
    - after_external_review
  created_at: 2026-05-02T10:15:00-07:00
  created_by: user
  parent_state: STATE-0012
  units: inch_lbf_F
  model_hash: sha256:...
  notes:
    - "Guide added near rack bent 4 after external stress review."
  external_references:
    - type: user_note
      label: "External stress review comments"
      value: "Project file reference by user"
  unresolved_assumptions:
    - "Final structural attachment details pending."
```

---

## 30. Appendix B: Example Analysis Run Record

```yaml
analysis_run:
  id: RUN-0052
  model_state: STATE-0018
  solver_version: 0.2.0-dev
  build_id: git:abcdef123
  units: inch_lbf_F
  load_cases:
    - W
    - T1
    - D1
  combination_cases:
    - OPERATING_1
    - EXPANSION_RANGE_1
  rule_packs:
    - id: RP-PRIVATE-001
      checksum: sha256:...
  diagnostics:
    singular_modes: 0
    max_residual: "1.2e-8 lbf"
  results_hash: sha256:...
```

---

## 31. Appendix C: Example Comparison Record

```yaml
comparison:
  id: CMP-0007
  name: RouteA_initial_vs_after_external_review
  model_state_a: STATE-0012
  model_state_b: STATE-0018
  analysis_run_a: RUN-0041
  analysis_run_b: RUN-0052
  mapping_table: MAP-0009
  tolerance_profile: TOL-DEFAULT-001
  created_at: 2026-05-02T10:45:00-07:00
  summary:
    added_supports: 1
    modified_elements: 3
    unmatched_nodes: 0
    out_of_tolerance_results: 4
```

---

## 32. Appendix D: Example Report Notice

```text
OpenPipeStress is decision-support software for piping design, flexibility, and stress-analysis workflows. It computes mechanical results from recorded user inputs and may evaluate user-supplied rule packs. It does not certify, seal, approve, authenticate, or determine code compliance for professional reliance. Code-specific and project-specific data are supplied by the user or user-controlled private sources. Competent human review and, where required, validation in accepted professional tools remain the responsibility of the user and project authority.
```
