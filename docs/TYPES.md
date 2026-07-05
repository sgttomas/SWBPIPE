---
doc_id: OPS-TYPES
doc_kind: governance.types
status: draft
created: 2026-04-30
refs:
  - rel: governed_by
    to: OPS-CONTRACT
---

# TYPES — OpenPipeStress Domain Vocabulary and Identity

## 1. Project hierarchy

OpenPipeStress agentic development uses a flat hierarchy:

```text
execution/_Decomposition/SOFTWARE_DECOMP.md
└── PKG-XX Package
    └── DEL-XX-YY Deliverable
```

There are no phases, sub-packages, or task sub-levels inside deliverables. Delivery sequencing may be managed separately, but package identity is a domain partition, not a schedule phase.

## 2. Stable identifiers

| Entity | Format | Example | Assigned by |
|---|---|---|---|
| Scope Item | `SOW-NNN` | `SOW-003` | SOFTWARE_DECOMP |
| Objective | `OBJ-NNN` | `OBJ-001` | SOFTWARE_DECOMP |
| Package | `PKG-XX` | `PKG-04` | SOFTWARE_DECOMP |
| Deliverable | `DEL-XX-YY` | `DEL-04-02` | SOFTWARE_DECOMP |
| Decision | `DEC-NNN` | `DEC-001` | Human / Type 1 agent draft |
| Open Issue | `OI-NNN` | `OI-004` | Type 1 agent draft / human |
| Risk | `RISK-NNN` | `RISK-002` | Type 1 agent draft / human |

Deliverable IDs are coupled to package IDs: `DEL-04-02` belongs to `PKG-04`. IDs persist across renames.

## 3. Software deliverable types

| Type | Meaning |
|---|---|
| `DOC_UPDATE` | Documentation, policy, guide, or report-template work. |
| `DATA_MODEL_CHANGE` | Schema, type, serialization, or domain object change. |
| `API_CONTRACT` | Public API, plugin, import/export, or data contract. |
| `BACKEND_FEATURE_SLICE` | Bounded backend/core behavior implementation. |
| `UX_UI_SLICE` | Bounded GUI feature or view. |
| `TEST_SUITE` | Verification, regression, validation, or lint suite. |
| `CI_CD_CHANGE` | Build, packaging, release, or pipeline change. |
| `SECURITY_CONTROL` | Privacy, redaction, storage, telemetry, or security control. |

## 4. Analysis-status vocabulary

| Status | Meaning | Authority level |
|---|---|---|
| `MODEL_INCOMPLETE` | Required physical data for solving is missing. | Software finding. |
| `MECHANICS_SOLVED` | Numerical mechanics solve completed. | Solver result only. |
| `RULE_INPUTS_INCOMPLETE` | Mechanics may be solved but rule-pack required values are missing. | Rule-pack finding. |
| `USER_RULE_CHECKED` | User-defined rule pack evaluated the result. | Software computation using user data. |
| `USER_RULE_FAILED` | User-defined rule pack produced a failing result. | Software computation using user data. |
| `HUMAN_REVIEW_REQUIRED` | Result requires competent engineering review before reliance. | Always true for professional use. |
| `HUMAN_APPROVED_FOR_PROJECT` | A human has recorded project-specific acceptance outside the solver core. | Human record only. |

Automatic software-emitted statuses are limited to `MODEL_INCOMPLETE`,
`MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`,
`USER_RULE_FAILED`, and `HUMAN_REVIEW_REQUIRED`. The software must not use
`HUMAN_APPROVED_FOR_PROJECT`, `CODE_COMPLIANT`, `CERTIFIED`, `SEALED`,
`APPROVED`, or equivalent professional/code-compliance language as an
automatic status.

The code-neutral analysis boundary is the data-model surface that keeps these
statuses from collapsing into one another. `schemas/analysis_boundary.schema.yaml`
requires an explicit authority model: mechanics statuses come from solver
results, user-rule statuses come from user-supplied rule-pack computation, and
human acceptance is an external hash-bound project record only.

## 5. Epistemic labels

| Label | Meaning |
|---|---|
| `FACT` | Directly observed in a cited source or model artifact. |
| `ASSUMPTION` | Reasonable inference requiring human validation. |
| `PROPOSAL` | Suggested design/development move requiring decision. |
| `TBD` | Unknown or unwarranted; must be resolved before reliance. |

## 5.1 Dependency-register vocabulary

Current dependency registers use the canonical v3.1 core enum values defined by
the shared Chirality dependency schema. For write-form rows, `DependencyType`
is limited to `PREREQUISITE`, `INTERFACE`, `HANDOVER`, `CONSTRAINT`,
`ENABLES`, and `OTHER`; `Status` is limited to `ACTIVE` and `RETIRED`.
Candidate or non-gating graph questions are recorded in a candidate worklist or
review packet, not as `Status=CANDIDATE`.

Legacy values from `DAG-001` through `DAG-006`, including project-specific
labels such as `ARCHITECTURE_BASIS`, `DOMAIN_MODEL`, `GUI_PREDECESSOR`,
`RUNNER_CONTRACT`, `INFERRED_DIRECT`, `UNKNOWN`, and `AnchorType=DELIVERABLE`,
are historical read-compatibility inputs only. New or refreshed dependency
artifacts must emit canonical enum values and preserve any legacy label in
notes or a non-core extension field.

## 6. Piping/software vocabulary

|Canonical Term|Definition|Notes|
|---|---|---|
|Open Mechanics|Public analytical mechanics implemented by the solver: geometry, stiffness, loads, stress recovery, units, reports.|May be shipped publicly.|
|Protected Standards Data|Code text, tables, figures, code-derived formulas, allowables, SIF/flexibility tables, protected dimensional tables.|Must not be shipped publicly unless explicit redistribution rights exist.|
|User-Supplied Code Data|Values, formulas, allowables, SIFs, flexibility factors, load combinations, and interpretations entered by the user.|May be stored in private project/rule-pack files.|
|Mechanics Solve|A solved numerical model with displacements, reactions, forces, moments, and mechanical stresses.|Not equivalent to a code check.|
|Rule-Pack Check|A user-defined expression/comparison applied to solver outputs.|Not equivalent to professional authentication.|
|Professional Approval|Human acceptance for reliance by a competent/responsible professional.|Outside software authority.|
|Centerline Model|A 3D line-element representation of the piping centerline and components.|Default global analysis model.|
|Frame Kernel|The code-neutral 3D frame mechanics core that owns DOF ordering, element stiffness, transforms, assembly, boundary-condition reduction, and a bounded solve interface.|Consumes unit-checked numeric inputs; does not perform rule checks or professional approval.|
|Solver Diagnostic|A deterministic mechanics-solver finding for singularity, invalid topology/restraints, numeric invalidity, conditioning, nonconvergence, or unresolved solver configuration.|Not a rule-pack result, code-compliance statement, or professional acceptance.|
|Local FEA Handoff|A transfer package for local shell/solid analysis.|Specialized workflow, not the default global solver.|
|Plugin Manifest|A declared extension metadata record that requests entrypoints, permissions, provenance, privacy posture, checksums, sandbox requirements, and no-bypass constraints.|A manifest grants nothing by itself.|
|Extension Point|A governed boundary where a plugin or adapter may request to import, export, validate, report, emit diagnostics, or expose user-owned rule-pack metadata.|Exact loader and transport remain `TBD`.|
|Domain Surface|The model, persistence, unit, analysis-boundary, rule-pack metadata, result, report, or diagnostic surface touched by an extension entrypoint.|Must be declared in the plugin manifest.|
|No-Bypass Constraint|A mandatory control that plugins and adapters must preserve, including units, provenance, privacy, protected-content, diagnostics, hashes, rule sandboxing, reports, solver boundaries, and human acceptance boundaries.|Cannot be waived by plugin code.|

## 7. Data provenance labels

| Label | Meaning |
|---|---|
| `USER_SUPPLIED_LICENSED_CODE` | User entered value from a licensed code/standard. |
| `USER_SUPPLIED_OWNER_SPEC` | User entered value from owner/company design basis. |
| `MANUFACTURER_PUBLIC_PERMISSIVE` | Manufacturer data with documented redistribution rights. |
| `MANUFACTURER_PRIVATE` | Manufacturer/vendor data for private project use only. |
| `PUBLIC_DOMAIN_OR_ORIGINAL` | Public-domain/original/invented data safe for public examples. |
| `UNKNOWN_SOURCE` | Source not documented; cannot be relied on without review. |
| `PROTECTED_CONTENT_SUSPECTED` | Quarantine and escalate before public use. |

## 8. Canonical domain object registry

The canonical machine-readable domain model is `schemas/model.schema.yaml`. That schema uses JSON Schema 2020-12, is written as strict JSON syntax in a `.yaml` path, and is intentionally code-neutral: it defines public object structure, identifiers, references, unit-bearing values, provenance, diagnostics, results, and report settings, but it does not bundle protected standards data, proprietary catalog values, code-specific allowables, SIF/flexibility tables, or compliance rules.

PKG-02 owns the canonical semantic dimension vocabulary used by both
`schemas/units.schema.yaml` `DimensionId` and `schemas/model.schema.yaml`
`Quantity.dimension`: `dimensionless`, `length`, `mass`, `time`,
`temperature`, `temperature_interval`, `angle`, `rotation`, `force`,
`force_per_length`, `moment`, `pressure`, `stress`, `area`, `volume`,
`density`, `linear_stiffness`, `rotational_stiffness`, `displacement`,
`velocity`, `acceleration`, `thermal_conductivity`, `specific_heat`,
`thermal_expansion_coefficient`, `second_moment_area`, `section_modulus`,
`mass_per_length`, `volume_per_length`, `slope`, and `TBD`.

Retired aliases are not accepted canonical enum values: use
`temperature_interval` instead of `temperature_difference`, use
`second_moment_area` instead of `area_moment`, and classify generic stiffness as
`linear_stiffness` or `rotational_stiffness`. Unit catalogs, conversion
constants, tolerances, code-specific values, release claims, and professional
claims remain out of scope for this vocabulary.

| Object | Registry meaning | Boundary note |
|---|---|---|
| `Project` | Project container for units, privacy/storage posture, models, rule-pack references, report settings, reports, diagnostics, and hashes. | Project-private engineering data remains user controlled unless intentionally contributed with documented rights. |
| `Model` | Physical or derived analytical model container for nodes, elements, components, materials, sections, supports, load cases, combinations, results, diagnostics, unresolved assumptions, traceability links, and references to design knowledge, constraints, operations, states/runs, comparisons, handoff packages, and external metadata. | The `physical_source_of_truth` role anchors editable design data; downstream PKG-13 through PKG-16 contracts own the detailed records and must not bypass canonical units, provenance, diagnostics, or professional-boundary controls. |
| `ModelRole` | Schema enum distinguishing a physical source-of-truth model from an analytical solver model, derived view, or unresolved `TBD` role. | Role metadata is not a lifecycle or professional-approval status. |
| `DesignKnowledgeEnvelope` | Schema-first records for user-supplied endpoints, line data, routing corridors, zones, equipment interfaces, access/slope/drain/vent requirements, owner/project metadata, source notes, assumptions, diagnostics, and provenance. | Public examples must be invented or cleared; the contract does not bundle owner standards, protected code criteria, proprietary project data, or hidden engineering defaults. |
| `ModelStateRecord` | Named immutable model snapshot record with tags, notes, external references, assumptions, warnings, analysis-status references, deterministic hashes, provenance, and read-only snapshot semantics. | States are review/reproducibility records, not formal prover approval, certification, sealing, authentication, code-compliance, or professional-acceptance states. |
| `Node` | Addressable point with unit-aware coordinates and six-degree-of-freedom state fields. | Missing imposed values or constraints are findings, not silent defaults. |
| `Element` | Analytical member connecting node references with material, section, optional component, governed local-coordinate, and result-station hooks. | Straight-pipe local-coordinate metadata requires a `y_reference` orientation basis and may carry provenance; `local_x_axis` is not part of the canonical schema. Mechanics implementation remains separate from schema structure. |
| `FrameKernel` | Rust mechanics module for two-node 3D frame stiffness, local/global transforms, dense assembly, boundary-condition reduction, and dense solve verification. | Sparse skyline evidence is bound downstream under `DEC-050`; `DEC-053` promotes sparse interactive as the default preview/render path while dense scrutiny remains explicit. Canonical calculation units, conversion constants, downstream adapters, release thresholds, and code-specific checks are out of scope. |
| `StraightPipeElement` | Rust mechanics module that adapts explicit straight-pipe section and mass properties into the frame-kernel element boundary and recovers local mechanical element forces. | Pipe dimensions, material values, units, provenance, and weight/load application are governed upstream or downstream; public defaults and code checks are out of scope. |
| `LinearSupport` | Rust mechanics-boundary module for anchors, guides, line stops, vertical supports, linear springs, and imposed displacement data mapped to frame-kernel DOFs. | Nonlinear gap/lift-off/one-way/friction behavior, support catalog defaults, coordinate-frame policy, sparse-solver integration, and compliance checks are out of scope. |
| `NonlinearSupportActiveSet` | Rust mechanics module that classifies one-way, gap/lift-off/contact, and friction-limited support states during nonlinear support iterations. | It records active-set changes and nonconvergence diagnostics but does not assemble the global nonlinear solve, define production tolerances, choose sparse solvers, or perform code/rule checks. |
| `NonlinearIntegrationLoop` | Rust mechanics integration module that wraps frame assembly/linear solve around the nonlinear active-set classifier for assembled active-set iterations. | Current slice exposes `DEC-053` solver modes: sparse interactive is the default linearized active-set path, dense scrutiny remains explicitly selectable, and sparse fallback to dense must be diagnostic/result-basis visible. Code/rule checks and release performance claims remain out of scope. |
| `SolverDiagnostics` | Rust mechanics-diagnostics module that maps frame-kernel errors and solver state facts to deterministic diagnostic codes, severities, sources, and statuses. | Nonlinear support cases, sparse solver performance policy, and release-quality tolerance thresholds remain downstream work. |
| `SolverPerformanceHarness` | Rust observer/regression module that runs invented or public-permissive frame fixtures repeatedly and records matrix metrics, residuals, repeatability deltas, conditioning observations, provenance, assumptions, and limitations. | Measures dense and DEC-023 sparse skyline paths as evidence; `DEC-053` adds a bounded 9-record sparse default-promotion observation packet with timing/RSS/hardware metadata, practical-size bands, and true condition numbers. Large-model suitability, hosted CI activation, release thresholds, professional claims, and code-compliance claims remain out of scope. |
| `PrimitiveLoadCase` | Rust mechanics module for weight, pressure, thermal, imposed-displacement, hydrotest, wind, seismic, and occasional primitive load categories. | It prepares explicit nodal, element-uniform, and imposed-displacement solver-boundary contributions; code-specific combinations, wind/seismic procedures, stress recovery, rule checks, and compliance claims are out of scope. |
| `LoadCaseAlgebra` | Rust mechanics-boundary module for user-defined linear combinations, result-state subtraction, and range envelopes over compatible load/result quantities. | It preserves unit/dimension intent and analysis statuses; code-specific public combination defaults, rule expression parsing, stress recovery, and compliance claims are out of scope. |
| `UserLoadApplication` | Rust mechanics-boundary module for concentrated forces, concentrated moments, and uniform distributed user loads. | It prepares explicit solver-boundary contributions and result-recovery hooks; code-specific load combinations, default factors, wind/seismic procedures, protected standards content, rule checks, GUI/report/API/CLI behavior, and compliance claims are out of scope. |
| `StressRecovery` | Rust mechanics module that recovers axial, bending, torsional, and pressure membrane stress components from explicit resultants and section/pressure inputs. | It does not provide design-code equations, allowables, stress indices, SIF/flexibility factors, protected standards content, public pipe tables, rule checks, or compliance claims. |
| `RulePack` | User-owned schema-governed artifact defining required inputs, declarative formula slots, user-supplied value slots, user-rule check definitions, provenance, redistribution status, diagnostics, checksums, and professional-boundary flags. | The schema defines structure only; expression grammar, evaluator execution, private storage, public examples, and final result-envelope integration remain downstream work. |
| `ExpressionEvaluator` | Rust rule-pack module that evaluates explicit declarative expression trees with variable bindings, dimension metadata, comparisons, and deterministic findings. | It does not parse arbitrary text, execute host-language code, access files/network/processes, select a final expression grammar/library, implement private rule-pack lifecycle, or make code-compliance/professional claims. |
| `RulePackLifecycle` | Rust rule-pack module that records lifecycle metadata, privacy/redistribution status, protected-content review state, checksum records, and audit-manifest references for user-owned rule packs. | It hashes caller-supplied canonical payload bytes and emits lifecycle findings; it does not store private rule packs, canonicalize JSON, implement encryption/access control, expose private formulas, or make code-compliance/professional claims. |
| `RulePackCompletenessCheck` | Rust rule-pack module that compares required-input declarations with supplied input evidence for values, units, dimensions, provenance, redistribution status, and review state. | It emits deterministic rule-check-blocking findings and `RULE_INPUTS_INCOMPLETE` readiness without evaluating formulas, storing private data, supplying code-specific values, or making code-compliance/professional claims. |
| `MechanicsBenchmark` | Rust validation fixture and hand-calculation record for open mechanics behavior such as cantilever response, frame assembly, thermal growth, imposed displacement, and stiffness transforms. | Fixtures must be original/public/permissive with provenance; tolerance policy, release gates, CI thresholds, code-specific acceptance, and professional reliance remain separate human-governed decisions. |
| `StressRecoveryBenchmark` | Rust validation fixture and hand-calculation record for code-neutral stress recovery behavior such as axial normal stress, bending normal stress, torsional shear stress, pressure membrane stress, and mechanics-only stress range. | Fixtures must be original/public/permissive with provenance; stress range is not a fatigue check, allowable comparison, code compliance decision, or professional approval. |
| `Component` | Piping-specific object such as bend, elbow, branch, reducer, valve, flange, expansion joint, rigid, specialty, or other user-defined component. | User modifiers and code/manufacturer values require provenance and may be private. |
| `Material` | User/private or permissively sourced material record with unit-bearing properties, allowable slots, completeness findings, and provenance. | Public schema does not provide protected material allowables, code tables, or proprietary catalog values. |
| `Section` | Pipe/section record with unit-bearing properties and provenance. | Protected dimensional tables and proprietary catalog data are not public defaults. |
| `SectionLibrary` | Governed section records with user-entered dimensions, derived-property slots, completeness findings, provenance, and review status. | Public fixtures remain schema-shape-only or invented unless public-permissive source review is complete. |
| `SectionPropertyCalculation` | Derived pipe section and mass-property result calculated from user-entered OD, wall, corrosion, density, contents, and insulation inputs. | The calculator does not supply dimensional tables, unit conversion constants, material defaults, catalog values, or protected/code-specific data. |
| `ComponentLibrary` | Governed component records for bends, branches, reducers, valves, flanges, expansion joints, rigid parts, specialty items, and user-defined components. | SIFs, flexibility factors, manufacturer values, weights, COGs, stiffnesses, and movement limits require provenance and may be private. |
| `LibraryImportProvenanceCheck` | Import-boundary validation result for material, section, and component library payloads. | It records accept/reject/review/quarantine outcomes without making legal conclusions or publishing private/protected values. |
| `BendElbowComponent` | Bend or elbow component record with user-entered centerline geometry, orientation metadata, source references, and user-supplied SIF/flexibility slots. | Public records define slots and diagnostics only; protected modifier tables and proprietary catalog values are not public defaults. |
| `BranchConnectionComponent` | Branch connection component record with user-entered run/header geometry, connection angle/type slots, reinforcement references, and user-supplied SIF/flexibility slots. | Public records define slots and diagnostics only; protected branch tables, code-specific modifier values, reinforcement values, and proprietary catalog values are not public defaults. |
| `RigidComponent` | Valve, flange, reducer, rigid, or specialty component record with user-entered dimensions, connection references, weight, center-of-gravity, stiffness behavior, and source references. | Public records define slots and diagnostics only; protected dimensional/rating tables, manufacturer values, stiffness values, and proprietary catalog geometry are not public defaults. |
| `ExpansionJointComponent` | Expansion joint record with user/manufacturer-supplied stiffness, effective-area, movement-limit, hardware, and manufacturer-reference slots. | Public records define slots and diagnostics only; manufacturer values, proprietary catalog data, private library values, and invented engineering defaults are not public defaults. |
| `Support` | Restraint/support record with target reference, directions, unit-bearing properties, and active-state result hook. | Nonlinear solution behavior and convergence semantics are solver-owned. |
| `ViewportEditorSession` | Schema-first viewport/editor contract for transient camera, hover, selection, drag, snap, view primitives, diagnostics, and application-service command intents over centerline model objects. | Tauri/React/Vite/Three.js runtime integration, package manifests, state-management library choice, model tree/property inspector, results viewer, and solve UX remain downstream work; viewport edits are command intents, not direct persisted-project mutations. |
| `ViewportCommandIntent` | GUI-generated intent for creating nodes, connecting pipe runs, inserting bend/branch/component symbols, and changing selection through an application-service boundary. | Intents are unit-aware validation requests and undo/redo candidates where reversible; they do not supply protected/code-specific data or professional acceptance. |
| `LoadCase` | Primitive loading record with strict typed load-record entries, target references, units, dimensions, and provenance. | Canonical load records are limited to `nodal_force`, `nodal_moment`, `element_point_force`, and `element_uniform_distributed_force`; code-specific load requirements are user/rule-pack supplied. |
| `Combination` | Algebraic grouping of load-case factors with an explicit basis. | No public code-specific combination defaults are implied. |
| `RulePackRef` | Reference to a user/public rule-pack by identity, version, checksum, source notice, redistribution status, and required-input links. | Rule formulas, protected interpretations, and evaluator implementation are outside this schema. |
| `Quantity` | Unit-bearing value record with magnitude, unit, dimension, and provenance. | Dimensionless values must be explicit; missing or incompatible units are findings, not hidden defaults. |
| `Provenance` | Source, license, contributor, redistribution, and review-status record used by reliance-affecting public/private data. | Unknown or suspected protected sources must remain reviewable and cannot be treated as accepted public data. |
| `Reference` | Typed stable-object reference used instead of positional coupling where editing, solving, reporting, or audit depends on identity. | External references remain explicit and do not bypass schema validation. |
| `Assumption` | Explicit unresolved or resolved model assumption with affected references and provenance. | Assumptions are review evidence and cannot silently become defaults or professional acceptance. |
| `TraceabilityLink` | Directed source/target trace record for physical-to-analytical transformation, operation application, state snapshots, analysis runs, comparisons, handoff records, or external references. | Traceability records diagnostics and provenance; they do not prove external validation, certification, or code compliance. |
| `Diagnostic` | Result-envelope warning/finding record with code, class, severity, source, affected object, message, remediation, and provenance. | Diagnostics expose missing data and boundary risks without becoming compliance claims. |
| `Checksum` | Hash metadata record with algorithm, canonicalization, payload reference, and value. | JSON payload hashes use the JCS-compatible basis where applicable; SCA-003 local SQLite storage is a payload/index substrate and not the authoritative hash basis. |
| `ProjectPersistenceEnvelope` | Versioned create/open/save envelope with schema version, project payload, migration status, hash metadata, diagnostics, round-trip manifest, validation profile, and service-operation contract records. | Delegates model structure to `schemas/model.schema.yaml`; SCA-003 selects a local SQLite-backed project store/index behind application-service boundaries while canonical JSON/JCS-compatible payload bytes remain the domain and interchange truth. |
| `PersistenceOperation` | Application-service contract record for create, open, save, validate, version-check, and migrate operations. | Operations must not bypass schema, unit, provenance, rule-pack reference, private-data, protected-content, or professional-boundary checks. |
| `RoundTripManifest` | Persistence comparison record naming semantic equality categories, serialization basis, volatile fields, and normalization rules. | Round trips preserve engineering-relevant data and do not insert silent defaults. |
| `HumanAcceptanceRef` | External human-review or project-acceptance reference bound to specific hashes. | Invalidates on bound-hash changes and is not an automatic software approval, certification, sealing, authentication, or code-compliance status. |
| `Result` | Mechanical, diagnostic, stress, reaction, or user-rule-check result envelope with analysis statuses and unit-aware values. | Result statuses do not mean automatic code compliance. |
| `AuditManifest` | Reproducibility record for model/input hashes, solver version stamp, unit-system reference, rule-pack checksum refs, asset hashes, privacy/redaction metadata, and manifest findings. | It records evidence for review without storing private payloads, authenticating engineering work, or making professional/code-compliance claims. |
| `ReportSettings` | Report configuration for manifests, provenance summaries, notices, result references, and rule-pack references. | Report rendering and professional acceptance records remain separate workflows. |
| `Report` | Auditable report-facing record for manifests, hashes, statuses, diagnostics, rule-pack refs, provenance summary, and professional-boundary notice. | Reports are decision support and must not claim certification, sealing, or professional approval by the software. |
| `CalculationReportEnvelope` | Schema-first calculation-report assembly contract for model input summary, load cases, result exports, audit manifests, report sections, rule-pack refs, diagnostics, template slots, limitations, and professional-boundary notices. | GUI/CLI/API/adapter runtime behavior, protected-content linting, private-data redaction/export controls, release-template integration, and final styling/layout remain `TBD`; reports preserve references, provenance, privacy boundaries, units through source envelopes, and no professional/code-compliance claims. |
| `ReportProtectedContentLintRun` | Schema-first linter configuration, target, finding, summary, and provenance contract for public report templates, report examples, and report fixtures. | It is deterministic heuristic review evidence only; it uses invented synthetic fixtures, skips private user surfaces by default, does not depend on `DEL-11-04` educational examples, and does not provide legal clearance, security sufficiency, professional approval, certification, sealing, endorsement, authentication, or code-compliance proof. |
| `ReportSectionEnvelope` | Schema-first records for report-facing warnings, analysis-status disclosures, assumptions, limitations, user-supplied values, provenance notes, unresolved TBDs, and human-review-needed findings. | Full report rendering, final template layout, GUI/CLI/API/adapter behavior, private redaction/export controls, and release-template integration remain `TBD`; sections preserve private/public data boundaries and no professional/code-compliance claims. |
| `ResultExportEnvelope` | Schema-first JSON result export contract for review, regression comparison, reports, headless automation, and governed downstream tooling. | Additional export formats, public transport, local FEA package format, GUI/report/CLI rendering, adapter behavior, and redaction workflow remain `TBD`; exports preserve units, diagnostics, provenance, hashes, status boundaries, and no professional/code-compliance claims. |
| `HeadlessRunnerEnvelope` | Schema-first request/result contract for non-GUI solve execution, benchmark automation, regression execution, audit-manifest capture, and result-export handoff through the `DEC-065` local `openpipestress-runner` policy. | Final local CLI syntax, package-script posture, process invocation, network prohibition, and stdout/default-plus-explicit-output filesystem policy are settled by `DEC-065`; CI provider, release matrix, public transport, external formats, operating-system storage roots, portable project export/copy behavior, GUI/report runtime, adapter behavior, and local FEA packaging remain `TBD`; runner outputs preserve units, diagnostics, provenance, hashes, result-export compatibility, local-first privacy, and no professional/code-compliance claims. |
| `AdapterFrameworkEnvelope` | Schema-first format-neutral contract for import/export adapter declarations, validation plans, diagnostics, provenance, privacy, checksum/audit refs, result-envelope compatibility, and no-bypass controls. | Concrete external formats, public transport, endpoint syntax, plugin runtime/loading/signing, package scripts, CI provider, release matrix, storage-root selection, portable project export/copy behavior, local FEA package format, redaction workflow, and real parse/write behavior remain `TBD`; adapters cannot bypass units, provenance, privacy, protected-content screening, diagnostics, rule sandboxing, report controls, human-review boundaries, or SCA-003 application-service persistence boundaries with direct SQL. |

Companion boundary schemas:

| Schema | Purpose | Boundary note |
|---|---|---|
| `schemas/section.schema.yaml` | Defines section-library metadata, pipe/section dimension slots, derived-property slots, completeness findings, provenance, and review status. | Protected dimensional tables and proprietary catalog values are not public defaults. |
| `schemas/component.schema.yaml` | Defines component-library metadata, user-entered dimension/weight/COG/stiffness/modifier slots, completeness findings, provenance, and review status. | Protected SIF/flexibility tables, code-specific values, proprietary catalog values, and private library data are user-supplied or separately reviewed. |
| `schemas/design_knowledge.schema.json` | Defines schema-first design knowledge records for endpoints, line data, corridors, zones, equipment interfaces, requirements, owner/project metadata, source notes, assumptions, diagnostics, and provenance. | User/project supplied only; public examples must be invented or cleared and must not bundle protected owner, standards, proprietary, or private project data. |
| `schemas/model_state.schema.json` | Defines schema-first immutable model state records with state metadata, external references, assumptions, warnings, hashes, provenance, and read-only snapshot semantics. | Human acceptance remains external and hash-bound; model states are not automatic professional approval or code-compliance statuses. |
| `schemas/analysis_boundary.schema.yaml` | Separates mechanics solve authority, user-rule-check authority, missing-input findings, diagnostics, and external human acceptance references. | Human acceptance is reference-only and hash-bound; it is not emitted by solver or rule-pack computation. |
| `schemas/analysis_status.schema.yaml` | Defines result-envelope status semantics and external human acceptance records. | `HUMAN_APPROVED_FOR_PROJECT` is not an automatic software status. |
| `schemas/project_persistence.schema.yaml` | Defines the versioned project persistence envelope, round-trip manifest, migration status, hash metadata, validation profile, service operations, private-data markers, and external human acceptance references. | Public fixtures must not contain protected standards data, private rule content, proprietary values, or automatic compliance/professional-approval claims. |
| `schemas/rule_pack.schema.yaml` | Defines rule-pack metadata, public/private classification, required inputs, declarative formula slots, user-supplied value slots, check definitions, diagnostics, provenance, redistribution status, checksums, and professional-boundary controls. | Rule-pack content is user-owned or invented/public-reviewed; the schema does not bundle protected values, arbitrary executable code, evaluator behavior, or software-generated professional/code-compliance claims. |
| `schemas/report_generator.schema.yaml` | Defines schema-first calculation-report envelopes, template slots, rendered-section metadata, source envelope references, audit/hash refs, rule-pack refs, diagnostics, provenance, privacy classification, and unresolved runtime/reporting TBDs. | It assembles governed report inputs for deterministic tests only; GUI/CLI/API/adapter runtime behavior, protected-content linting, private redaction/export controls, release-template integration, and final styling/layout are separate work. |
| `schemas/report_protected_content_linter.schema.yaml` | Defines schema-first protected-content linter configuration, public-surface targets, findings, source locations, review routes, dispositions, summaries, and provenance. | It is heuristic review evidence only; fixtures use invented synthetic markers, private surfaces are skipped by default, `DAG-001-E0621` remains non-gating, and CI/release policy, redaction/export controls, quarantine file movement, legal review, and professional acceptance remain separate work. |
| `schemas/report_sections.schema.yaml` | Defines schema-first report-section records for diagnostics, analysis-status disclosures, provenance notes, user-supplied values, assumptions, limitations, unresolved TBDs, and professional-boundary controls. | It does not render final reports or choose final layout; public report sections must not copy protected/private content or declare professional approval or code compliance. |
| `schemas/results.schema.yaml` | Defines schema-first JSON result export envelopes for result identity, model/run references, unit-aware values, diagnostics, provenance, reproducibility refs, analysis status, rule-pack refs, and downstream-use declarations. | Additional export formats are `TBD`; result exports do not copy protected/private rule content and do not declare professional approval or code compliance. |
| `schemas/viewport_editor.schema.yaml` | Defines schema-first viewport/editor status, camera/transient state, view primitives, selection, diagnostics, provenance, professional-boundary controls, and command intents for centerline editing. | It does not create a frontend app shell, render Three.js scenes, mutate durable model state directly, choose GUI dependency versions, or implement adjacent GUI slices. |
| `schemas/headless_runner.schema.yaml` | Defines the schema-first headless runner request/result envelope for operation identity, project/model/unit/load/input-manifest references, requested outputs, job progress/cancellation, diagnostics, result-export refs, audit-manifest refs, privacy, provenance, professional-boundary controls, and the `DEC-065` local CLI/process policy tokens. | It does not choose CI provider, release matrix, public transport, external adapter formats, local FEA package format, operating-system storage roots, portable project export/copy behavior, hosted/runtime service behavior, or release packaging. |
| `schemas/adapter_framework.schema.yaml` | Defines schema-first format-neutral adapter framework envelopes for adapter capability declarations, validation plans, operation results, diagnostics, provenance, privacy, checksum/audit refs, result-envelope compatibility, and no-bypass controls. | It does not choose concrete external formats, public transport, endpoint syntax, adapter execution/loading model, plugin runtime, package scripts, CI provider, release matrix, storage roots, portable project export/copy behavior, local FEA package format, redaction workflow, or real external file parsing; it does not authorize direct SQL access. |

## 9. Lifecycle states for development deliverables

```text
OPEN → INITIALIZED → IN_PROGRESS → CHECKING → ISSUED
```

`ISSUED` means accepted as a development artifact by the human project authority. It does not mean professional engineering authentication of any piping calculation.
