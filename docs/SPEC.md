---
doc_id: OPS-SPEC
doc_kind: governance.technical_spec
status: draft
created: 2026-04-30
refs:
  - rel: depends_on
    to: OPS-CONTRACT
  - rel: depends_on
    to: OPS-TYPES
  - rel: implements
    to: OPS-PRD
---

# SPEC — OpenPipeStress Technical and Agentic Implementation Specification

## 1. Architectural overview

OpenPipeStress shall be organized as a code-neutral engineering workbench with five separable layers:

```text
GUI / UX Layer
  └── Application Services
        └── Domain Core
              ├── Geometry + Units + Data Schemas
              ├── Solver Core
              ├── Loads + Stress Recovery
              ├── Rule-Pack Evaluator
              └── Reporting/Audit Manifest
        └── Adapters
              ├── Private Libraries
              ├── Import/Export Plugins
              ├── Local FEA Handoff
              └── Storage/Packaging
```

The domain core owns mechanics and invariant enforcement. Adapters may import/export data but may not bypass unit checks, provenance checks, or public/private data boundaries.

## 2. Repository layout target

```text
open-pipe-stress/
├── docs/
├── schemas/
├── core/
│   ├── units/
│   ├── geometry/
│   ├── model/
│   ├── solver/
│   ├── loads/
│   ├── stress/
│   ├── rules/
│   └── reports/
├── gui/
├── adapters/
├── examples/
│   ├── invented_models/
│   └── rule_packs_invented_only/
├── validation/
│   ├── benchmarks/
│   ├── hand_calcs/
│   └── regression/
├── scripts/
└── tests/
```

## 3. Domain objects

Minimum domain objects:

| Object | Purpose | Required properties |
|---|---|---|
| `Project` | Project container and design basis metadata | ID, units, storage policy, rule-pack refs, report settings |
| `Model` | Physical source-of-truth or derived analytical piping model | role, nodes, elements, components, supports, loads, libraries, diagnostics, unresolved assumptions, traceability and downstream contract refs |
| `Node` | 3D coordinate and six-DOF state | coordinates, DOF flags, imposed displacement refs |
| `Element` | Analytical member between nodes | type, section/material refs, local coordinate system, results stations |
| `Component` | Piping-specific object | component type, geometry, user modifiers, source/provenance |
| `Material` | User/private material record | density, modulus, expansion, allowables, source status |
| `Section` | Pipe/section properties | OD, thickness, corrosion basis, section properties, source status |
| `Support` | Restraint/restraint behavior | type, direction, stiffness/gap/friction, active-state results |
| `LoadCase` | Primitive loading | type, magnitude, units, source |
| `Combination` | Algebraic result combination | expression, result basis, category |
| `RulePack` | User-defined code/design-basis check | required inputs, formulas, allowables, status, checksum |
| `Report` | Auditable calculation output | input manifest, warnings, results, rule-pack refs, notices |

### 3.0 Dependency-register type system

Dependency registers are governed coordination artifacts. Current write-form
registers must use the canonical v3.1 enum surface: `DependencyType` is one of
`PREREQUISITE`, `INTERFACE`, `HANDOVER`, `CONSTRAINT`, `ENABLES`, or `OTHER`,
and `Status` is one of `ACTIVE` or `RETIRED`. Candidate dependencies are
non-authoritative worklist items until promoted by a human-gated graph refresh;
they are not emitted as `Status=CANDIDATE`.

`DAG-001` through `DAG-006` remain immutable historical snapshots with legacy
read compatibility. Current successors, beginning with the approved `DAG-007`
rectification package, must validate in canonical mode and may preserve legacy
labels only as provenance notes or extension metadata.

Revision 0.5 treats the schema-backed physical model as the source of truth for
editable piping design data. `schemas/model.schema.yaml` therefore exposes
source-of-truth role metadata, unresolved assumptions, diagnostics/warnings,
traceability links, and typed references to design knowledge, constraints,
equipment interfaces, model operations, immutable states, analysis runs,
comparisons, handoff packages, and external prover metadata. The detailed
records for those surfaces remain owned by PKG-13 through PKG-16 and their
specialized schemas or services.

### 3.1 Design knowledge contract

The design knowledge contract is represented by
`schemas/design_knowledge.schema.json`. It defines user-supplied design
knowledge records for endpoints, line data, routing corridors, zones,
equipment interfaces, access/slope/drain/vent requirements, owner/project
metadata, source notes, and unresolved assumptions.

Design knowledge records are structured project data supplied by users or
their cleared sources. Public examples must be invented or otherwise cleared
for redistribution. Records carry provenance, privacy classification,
redistribution status, review status, and explicit unit metadata for
unit-bearing values. Missing or uncertain data remains a diagnostic or
assumption record, not a silent engineering default.

The schema provides slots for owner/project metadata without bundling owner
standards, protected code criteria, proprietary project data, protected
standards text, protected tables, vendor catalog values, or code-specific
acceptance criteria.

### 3.2 Immutable model state contract

The immutable model state contract is represented by
`schemas/model_state.schema.json`. It defines named model state records with
tags, notes, parent-state references, external references, unresolved
assumptions, warnings, analysis-status references, deterministic hashes, and a
read-only snapshot policy.

Model states are reproducibility and design-iteration records. Changes to a
state payload create a new model state. Hash-bound external review or human
acceptance references, if added by later workflows, invalidate when bound
hashes change. The model state schema uses the accepted JCS-compatible JSON
payload hash basis. Physical MVP project storage follows SCA-003: a local
SQLite-backed project store/index behind application-service boundaries, with
canonical JSON/JCS-compatible payload bytes remaining the domain and
interchange truth.

Model states do not provide formal prover approval states, certification
states, sealing states, authentication states, automatic code-compliance
statuses, or professional acceptance records.

## 4. Unit system and dimensional analysis

The domain core owns the unit contract. Every physical value that crosses a
schema, application-service, solver, import/export, report, or rule-evaluation
boundary must carry explicit unit metadata unless the field is explicitly
classified as dimensionless, ratio, percentage, or coefficient.

The unit contract is represented by `schemas/units.schema.yaml` and the
`core/units` module contract. It defines:

- unit-system records, unit definitions, dimensions, and dimension vectors;
- unit-bearing quantity records with magnitude, unit reference, dimension,
  missing-unit behavior, and provenance;
- conversion declarations with transform kind, provenance, redistribution
  status, and review status;
- dimension-check records and unit diagnostics;
- operation rules for same-dimension operations, derived-dimension operations,
  schema/import/export validation, and rule evaluation;
- test obligations and open decisions that block downstream schema/API freeze
  where decisions remain `TBD`.

The accepted PKG-02 semantic dimension vocabulary is shared by
`schemas/units.schema.yaml` and `schemas/model.schema.yaml`: `dimensionless`,
`length`, `mass`, `time`, `temperature`, `temperature_interval`, `angle`,
`rotation`, `force`, `force_per_length`, `moment`, `pressure`, `stress`,
`area`, `volume`, `density`, `linear_stiffness`, `rotational_stiffness`,
`displacement`, `velocity`, `acceleration`, `thermal_conductivity`,
`specific_heat`, `thermal_expansion_coefficient`, `second_moment_area`,
`section_modulus`, `mass_per_length`, `volume_per_length`, `slope`, and `TBD`.
Retired aliases are
not accepted enum values: `temperature_difference` maps to
`temperature_interval`, `area_moment` maps to `second_moment_area`, and generic
`stiffness` must be classified as `linear_stiffness` or
`rotational_stiffness`.

Dimensionless values are not a fallback for missing units. Missing or ambiguous
unit metadata on a unit-bearing physical value is a diagnostic, and solve- or
rule-check-required missing unit data must not be silently supplied.

Conversion data is governed data. Public unit or conversion records require
provenance, redistribution status, contributor certification, and review
status. Suspected protected standards content, proprietary vendor data, or
undocumented commercial data must be quarantined and escalated under the
project IP/data-boundary policy.

Deterministic conversion tests that require numeric constants remain gated until
the project has accepted the unit catalog, conversion source set, numeric
representation, and tolerance policy. Until accepted, unsupported conversion
semantics such as offset temperature, gauge versus absolute pressure, and
angle/rotation treatment remain explicit `TBD` decisions or blocking
diagnostics.

Unit checks support mechanics and rule evaluation. They do not certify, seal,
approve, authenticate, or declare engineering code compliance for reliance.

## 4.1 Material library schema

Material library records are governed data. The public schema may define
material identity, temperature-dependent property slots, allowable-value slots,
unit metadata, provenance, redistribution status, review status, and
completeness findings. The public repository must not bundle protected material
allowable tables, code-specific allowable values, proprietary catalog data, or
private user material libraries as defaults.

The material-library contract is represented by `schemas/material.schema.yaml`.
It defines:

- material-library metadata, privacy class, and provenance;
- material records with unit-bearing property slots and review status;
- allowable slots for private or reviewed user-rule data without public
  protected/code-specific values;
- completeness rules and diagnostics for missing solve-required or
  rule-check-required material data;
- open decisions for public fixture policy, accepted source catalogs,
  allowable-value storage, and temperature interpolation.

Public material fixtures must be invented non-engineering examples or
permissively sourced records with completed review evidence. Missing or
unreviewed material values remain explicit findings; they are not supplied by
silent defaults.

## 4.2 Section and component library schemas

Pipe section and component library records are governed data. The public schema
may define user-entered dimensions, weights, centers of gravity, component
modifier slots, derived-property slots, provenance, redistribution status,
review status, and completeness findings. The public repository must not bundle
protected dimensional tables, code-specific component modifiers, proprietary
catalog values, or private user component libraries as defaults.

The section and component library contracts are represented by
`schemas/section.schema.yaml` and `schemas/component.schema.yaml`. They define:

- section-library metadata, pipe/section dimension slots, derived-property
  slots, provenance, review status, and missing-data diagnostics;
- component-library metadata, component field slots for dimensions, weights,
  centers of gravity, stiffness, effective area, movement limits, and user
  modifier inputs;
- bend/elbow family contracts for user-entered centerline radius, included
  angle, plane/orientation metadata, source references, and user-supplied SIF
  or flexibility-factor slots;
- branch-connection family contracts for user-entered run/header geometry,
  connection angle/type slots, reinforcement source slots, and user-supplied
  SIF or flexibility-factor slots;
- rigid/semi-rigid component family contracts for valves, flanges, reducers,
  rigid parts, and specialty items using user-entered dimensions, weights,
  centers of gravity, stiffness behavior, and source references;
- expansion-joint family contracts for user/manufacturer-supplied stiffnesses,
  effective area, movement limits, hardware references, and manufacturer
  provenance slots;
- explicit public repository value policies for SIF, flexibility, proprietary
  catalog, and user/private values;
- completeness rules and diagnostics for solve-required, property-calculation,
  rule-check-required, and import/review-required data;
- open decisions for fixture policy, accepted source catalogs, import formats,
  section-property calculation policy, and component value policy.

Public section/component fixtures must be invented non-engineering examples or
schema-shape-only records unless a public-permissive source has completed
review evidence. Missing or unreviewed section/component values remain explicit
findings; they are not supplied by silent defaults.

The pipe section property calculator is represented by
`core/section_properties/calculator.py`. It calculates circular pipe section
properties and mass-per-length values from user-entered dimensions and density
inputs only. It does not provide pipe schedule tables, protected dimensional
tables, material defaults, catalog values, unit conversion constants, or
code-specific values. Unit-bearing inputs must already carry compatible unit and
dimension metadata; otherwise the calculator returns blocking diagnostics such
as `SECTION_UNIT_MISSING`, `SECTION_DIMENSION_INCONSISTENT`, or
`SECTION_CALCULATION_INPUT_INVALID`.

The library import provenance checker is represented by
`core/library_import/provenance_checker.py`. It validates already-parsed
material, section, and component library payloads for required provenance,
redistribution status, review disposition, private-only handling, protected
content quarantine, and unit metadata preservation. It does not parse external
formats and does not make legal conclusions; unresolved rights remain review
findings for the human project authority.

Bend and elbow records define schema slots and mechanics-interface metadata
only. Public repository fixtures may show the shape of bend geometry fields but
must not supply protected bend tables, code-specific SIF or flexibility values,
or proprietary catalog geometry as defaults.

Branch connection records define schema slots and mechanics-interface metadata
only. Public repository fixtures may show the shape of branch geometry and
reinforcement-reference fields but must not supply protected branch tables,
code-specific SIF or flexibility values, reinforcement values, or proprietary
catalog geometry as defaults.

Rigid and semi-rigid component records define schema slots and
mechanics-interface metadata only. Public repository fixtures may show the
shape of valve, flange, reducer, rigid, and specialty-item fields but must not
supply protected dimensional/rating tables, manufacturer weights, centers of
gravity, stiffness values, or proprietary catalog geometry as defaults.

Expansion joint component records define schema slots and mechanics-interface
metadata only. Public repository fixtures may show stiffness, effective-area,
movement-limit, hardware, and manufacturer-reference field shapes but must not
supply manufacturer values, proprietary catalog data, private library values,
or invented engineering defaults.

### 4.3 Code-neutral analysis boundary

The analysis boundary is represented by
`schemas/analysis_boundary.schema.yaml`. It separates three authority domains:

- mechanics solve authority: solver results and diagnostics only;
- user-rule-check authority: software computation using user-supplied rule-pack
  data, rule-pack version, checksum, source notice, redistribution status, and
  provenance;
- human acceptance authority: external project records bound to reviewed
  evidence hashes.

Software-generated statuses may report `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`,
`RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, and
`HUMAN_REVIEW_REQUIRED`. Software must not emit human approval, certification,
sealing, authentication, or code-compliance labels as automatic analysis
statuses.

Missing solve-required inputs and missing rule-check-required inputs are
explicit findings with diagnostics and provenance. They are not defaulted by the
solver, rule-pack evaluator, adapters, reports, or GUI.

### 4.4 Project persistence and round-trip serialization

The project persistence envelope is represented by
`schemas/project_persistence.schema.yaml`. It is a versioned, schema-governed
JSON document contract for create/open/save, version checks, migration status,
canonical hash metadata, private-data markers, diagnostics, and deterministic
round-trip manifests. The envelope delegates model structure to
`schemas/model.schema.yaml` rather than redefining model objects.

SCA-003 selects the MVP physical storage substrate: a local SQLite-backed
project store/index that stores schema-governed canonical
JSON/JCS-compatible payload bytes and private indexed projections behind
application-service boundaries. SQLite tables, FTS5/BM25 sidecars, and other
derived indexes are storage/projection details, not public contracts or domain
truth. Retrieval sidecars are rebuildable and non-authoritative.

Persistence service operations are application-service boundaries. Create,
open, save, validate, version-check, and migrate flows must return structured
outputs and diagnostics; adapters and plugins must not bypass schema
validation, unit metadata checks, provenance checks, rule-pack reference checks,
private-data controls, protected-content screening, professional-boundary
checks, or the SCA-003 prohibition on direct SQL access, raw SQLite handles,
table-name dependencies, or direct project-store mutation.

JSON payload hashes use the accepted JCS-compatible canonical JSON basis where
the payload is JSON. Hash records must identify payload scope, such as project
envelope, project payload, model payload, rule-pack reference, input manifest,
report manifest, or external artifact. Non-JSON and binary payload partitioning
remains `TBD` and must be explicit in hash or round-trip manifests.

Round-trip acceptance compares semantic equality for model content, unit
metadata, loads, rule-pack references, provenance metadata, reproducibility
metadata, and documented volatile-field exclusions. Round trips must not insert
silent engineering defaults for units, provenance, rule-pack values, material
data, component data, SIF/flexibility inputs, allowables, or load basis values.

Optional human acceptance references, when present in persisted projects, are
external and hash-bound. They invalidate on bound-hash changes and do not imply
software certification, sealing, authentication, professional approval, or
automatic code compliance.

The MVP storage posture is local-first and offline-capable: core model
authoring, solve, rule-check, and reporting require no network, and there is no
hosted database, daemon, cloud sync, or telemetry path by default. Per
`D-T0-04` (open residency) / `DEC-051`, the app does not enforce data
residency: the owner may configure a networked model provider (local,
Anthropic, or other) for an embedded agent, and once configured the agent may
transmit the owner's own private model and Class-B data to that provider with
no further app-side guard. Such transmission is never a public commit
(`OPS-K-IP-1/2/3` unaffected) and is distinct from telemetry (off by default,
`OPS-K-PRIV-2`). Operating system roots, application data directories, product
and DB migration mechanics, portable export/copy workflows, encryption, key
management, secret storage, provider/egress configuration surfaces, cloud
exception workflows, and release packaging remain separate `TBD` decisions.

### 4.5 Plugin and extension domain contracts

The plugin manifest contract is represented by
`schemas/plugin_manifest.schema.yaml`. It defines domain-level extension
metadata, entrypoints, permissions, provenance, privacy posture, checksums,
sandbox requirements, no-bypass controls, and professional-boundary constraints.

Plugins and adapters are denied by default. A manifest request is not a runtime
grant. Each entrypoint must identify the domain surface it touches, such as the
canonical model, project persistence, units, analysis-boundary state, rule-pack
metadata, results, reports, or diagnostics.

Plugin and adapter paths must preserve the same controls as first-party code:
schema validation, unit checks, provenance checks, private-data controls,
protected-content screening, diagnostics/result envelopes, persistence hashes,
rule-pack sandboxing, report boundaries, solver boundaries, and external
hash-bound human acceptance records. They must not create software-generated
professional approval records or automatic code-compliance statuses.

The import/export adapter framework is represented by
`schemas/adapter_framework.schema.yaml` and
`core/adapters/framework/adapter_framework.py`. It defines a format-neutral
adapter declaration, validation plan, diagnostics, provenance, privacy,
checksum/audit references, result-envelope compatibility, and no-bypass
controls. It validates already-declared adapter metadata and invented fixtures
only. It does not parse real external files, select import/export formats,
choose public transport or endpoint syntax, implement plugin loading, choose
filesystem roots, mutate project storage, access networks or processes, or
make professional/code-compliance/security claims.

Adapter declarations must distinguish syntactic parse readiness, schema
validation, unit and dimensional validation, provenance completeness,
redistribution review, privacy classification, protected-content screening,
mechanics readiness, rule-check readiness, export review, and human review.
Export operations remain local-first and review-gated by default; unresolved
redaction, concrete format, runtime, packaging, CI, release, local FEA,
portable project export/copy, and storage-root choices remain `TBD`.

## 5. Solver core requirements

### 5.1 Degrees of freedom

Each node carries six degrees of freedom:

```text
Ux, Uy, Uz, Rx, Ry, Rz
```

### 5.2 Straight pipe/frame element

The straight-pipe element shall support axial, torsional, and bending stiffness in local coordinates, local-to-global transformation, thermal strain, distributed loads, and element end-force recovery.

The first frame-kernel slice is `core/solver/frame_kernel`. It defines the
six-degree-of-freedom node ordering, two-node frame element stiffness,
local-axis transformation, dense assembly, boundary-condition reduction, and a
temporary deterministic dense solve interface for small verification cases. The
sparse numerical library, solver tolerances, and canonical calculation unit
basis remain `TBD` until accepted through a later solver/architecture gate.
Frame-kernel inputs are numeric mechanics quantities that must arrive through
unit-aware upstream contracts; missing, non-finite, non-positive, degenerate,
invalid-node, repeated-restraint, or singular-system conditions are explicit
errors, not silent defaults.

### 5.3 Bend and component treatment

The open core shall provide fields and mechanics interfaces for bends, elbows, branches, reducers, valves, flanges, expansion joints, rigid elements, and specialty components. User-supplied SIFs, flexibility factors, stress indices, manufacturer stiffnesses, and local data are inputs, not public defaults.

### 5.3.1 Linear supports and restraints

The linear support and restraint slice is `core/solver/linear_supports`. It
models anchors, guides, line stops, vertical supports, linear springs, and
imposed displacement boundary data as explicit mechanics-boundary inputs for
the frame-kernel six-DOF order. It prepares restrained DOFs, linear spring
entries, imposed displacement entries, and deterministic findings for missing
or invalid solve-required support data.

Support quantities retain dimension intent, including linear stiffness,
rotational stiffness, displacement, and rotation. Unit conversion constants,
canonical calculation units, support coordinate-frame convention,
constraint-elimination or penalty strategy, sparse-solver integration, and
final result-envelope integration remain `TBD` until accepted through later
gates.

The linear support slice does not implement nonlinear gap, lift-off, one-way,
friction, or active-set behavior; that behavior remains assigned to
`DEL-04-04`. It also does not provide public support stiffness defaults,
catalog values, protected standards data, rule-pack checks, load-case algebra,
or professional/code-compliance claims.

### 5.4 Nonlinear supports

Nonlinear supports shall use an active-set or equivalent iterative method. Results must record active/inactive states, gaps, lift-off, friction state, convergence tolerance, iteration count, and non-convergence warnings.

The nonlinear support active-set slice is `core/solver/nonlinear_supports`. It
models one-way supports, gap/lift-off/contact state, and friction stick/slip
classification as explicit mechanics-boundary decisions driven by supplied
trial displacement and reaction data. It records active-set changes, residual
state, convergence, iteration limits, and nonconvergence diagnostics through
the solver diagnostics contract.

The assembled nonlinear active-set loop slice is
`core/solver/nonlinear_integration`, created under `DEC-044`. It wraps the
linear frame assembly/reduction/dense solve path around the active-set
classifier, applies base restraints plus active nonlinear restraints, and stops
when the classifier converges or the explicit iteration cap produces
nonconvergence diagnostics.

These slices do not select the live sparse solve path, define release-quality
numeric convergence values, perform load-case algebra, provide public
support/catalog defaults, run rule-pack checks, or make
professional/code-compliance claims. Sign conventions for activation and gap
closure remain explicit model/solver inputs rather than hidden defaults; final
support coordinate convention, product/app integration, and measured
class-tiered convergence values remain governed follow-on work.

### 5.5 Numerical quality

Solver results must be deterministic for the same model, units, solver version, and rule-pack inputs. Sparse-solver settings, tolerances, and conditioning warnings must be reportable.

The solver diagnostics slice is `core/solver/diagnostics`. It maps
frame-kernel failures into deterministic solver diagnostic records for singular
systems, invalid restraints, invalid topology, invalid numeric inputs,
conditioning warnings/failures, nonconvergence, and explicit sparse-solver or
tolerance-policy `TBD` states. These diagnostics are mechanics-solver findings
only; they do not perform rule-pack checks, professional review, certification,
sealing, or code-compliance assessment.

The sparse-solver performance harness slice is
`core/solver/performance_harness`. It records deterministic repeat-run evidence
for invented/public frame fixtures at the current solve boundary, including
matrix size, nonzero-count, residual, repeatability, provenance, assumptions,
limitations, and conditioning diagnostics. It is an observer/regression surface:
it does not modify solver logic, choose the production sparse numerical
library, set release timing or memory thresholds, import proprietary benchmark
models, or make professional/code-compliance claims.

The straight-pipe element slice is `core/solver/straight_pipe`. It adapts
explicit section properties into the frame-kernel element boundary, exposes
mass/weight-per-length hooks for later load-case work, and recovers local
mechanical element forces from element or model displacement vectors. Inputs
must come from governed unit/provenance/section-property paths; the slice does
not provide public pipe tables, material defaults, code-specific values, stress
checks, load combinations, or professional/code-compliance claims.

## 6. Loads and stress recovery

Primitive loads include weight, pressure, temperature, imposed displacement, hydrotest, wind, seismic, and user occasional loads. Code-specific load combinations are not public defaults; they are user rule-pack or project inputs.

The primitive load case slice is `core/loads/primitive_loads`. It records
weight, pressure, thermal, imposed-displacement, hydrotest, wind, seismic, and
occasional primitive loads as explicit mechanics inputs with dimension intent,
target references, and deterministic findings for missing or invalid
solve-required data. It prepares nodal, element-uniform, and
imposed-displacement contributions for later load-case algebra, stress
recovery, GUI, and headless execution consumers.

This slice does not define code-specific load combinations, wind/seismic design
procedures, pressure stress formulas, public catalog defaults, protected
standards data, rule-pack checks, result envelopes, or professional/code
compliance claims. Wind and seismic loads are currently represented only as
user-supplied equivalent mechanics loads; dynamic or code-procedure generation
remains `TBD`.

The load-case algebra slice is `core/loads/load_case_algebra`. It combines
explicit primitive-load or result-state operands using user-defined factors,
subtraction, and range envelopes while preserving unit/dimension intent and
analysis-status boundaries. Missing operands, incompatible dimensions,
non-finite factors, duplicate operands, unsupported expression shapes, and
human-approval status are deterministic findings, not silent defaults. This
slice does not provide code-specific public combination defaults, a general
rule-pack expression evaluator, stress recovery, protected standards data, or
professional/code-compliance claims.

The concentrated and distributed user-load slice is `core/loads/user_loads`.
It prepares explicit concentrated forces, concentrated moments, and uniform
distributed user loads as solver-boundary contribution records with unit/
dimension intent, target references, provenance references, deterministic
findings, and result-recovery hooks. Missing targets, missing quantities,
out-of-range targets, incompatible dimensions, invalid directions, invalid
distribution spans, and non-positive element lengths are findings, not silent
defaults. This slice does not provide code-specific load combinations, public
default factors, wind/seismic procedures, protected standards data, rule-pack
checks, GUI/report/API/CLI behavior, or professional/code-compliance claims.

The canonical model schema represents user loads as strict typed load records:
`nodal_force` targets a `Node`, uses global `X`/`Y`/`Z`, and carries a `force`
quantity; `nodal_moment` targets a `Node`, uses `RX`/`RY`/`RZ`, and carries a
`moment` quantity; `element_point_force` targets an `Element`, carries a
dimensionless `station_fraction`, uses `X`/`Y`/`Z`, and carries a `force`
quantity; `element_uniform_distributed_force` targets an `Element`, carries a
dimensionless start/end span, uses `X`/`Y`/`Z`, and carries a
`force_per_length` quantity. Straight-pipe elements use governed local
orientation metadata: their `local_coordinate_system` carries a `y_reference`
and may carry provenance. The schema does not add a `local_x_axis` field.

The stress recovery slice is `core/loads/stress_recovery`. It recovers
code-neutral mechanics components from explicit element force resultants,
section properties, and optional pressure basis inputs: axial normal stress,
bending normal stress components, torsional shear stress, and thin-wall
pressure membrane components. Missing resultants, missing section or pressure
inputs, non-finite values, non-positive properties, incomplete mechanics
status, and human-approval status are deterministic findings, not silent
defaults. This slice does not provide design-code stress equations, allowables,
stress indices, SIF/flexibility tables, protected standards content, public
pipe tables, unit conversion constants, rule-pack checks, result export, or
professional/code-compliance claims.

Stress recovery shall calculate open mechanics quantities such as axial stress, bending stress, torsional shear stress, pressure membrane stresses, and resultants. Code-category equations are rule-pack mappings.

The stress recovery benchmark suite is `validation/benchmarks/stress`. It
contains original invented verification fixtures and hand-calculation notes for
axial normal stress, bending normal stress, torsional shear stress, pressure
membrane stress, and mechanics-only stress range behavior. The suite compares
outputs from the governed stress-recovery API without changing production
stress-recovery behavior and without introducing code-specific stress
equations, allowables, protected standards content, or professional/compliance
claims.

## 7. Rule-pack evaluator

Rule packs are private or user-owned design-basis artifacts. The public project ships schemas and invented examples only.

The rule-pack artifact contract is represented by
`schemas/rule_pack.schema.yaml`. It defines structure for:

- rule-pack identity, schema version, rule-pack version, lifecycle/status, and
  source notices;
- public/private classification, redistribution status, protected-content
  review requirement, and public repository policy;
- required input declarations with unit/dimensional intent, provenance
  requirements, completeness status, and missing-value diagnostics;
- declarative formula slots that are data records only, with arbitrary code
  execution disallowed and protected text, tables, or copied formulas excluded;
- user-supplied value slots for allowables, limits, coefficients, thresholds,
  and source references, with provenance, redistribution, review, and
  completeness status;
- user-rule check definitions that may produce `RULE_INPUTS_INCOMPLETE`,
  `USER_RULE_CHECKED`, `USER_RULE_FAILED`, and `HUMAN_REVIEW_REQUIRED`
  statuses, but not human approval or code-compliance statuses;
- checksum metadata using the accepted canonical JSON/JCS-compatible basis for
  JSON rule-pack payloads;
- diagnostics for missing inputs, unit mismatch, provenance gaps,
  redistribution gaps, suspected protected content, evaluator errors, and
  incomplete data;
- professional-boundary flags preventing software-generated certification,
  sealing, compliance, or human-acceptance records.

The schema is a contract for downstream evaluator, completeness-check,
private-lifecycle, GUI-editor, report, and documentation work. It does not
select the expression grammar, implement evaluation, create public example rule
packs, define private storage, choose the checksum library, or integrate final
result envelopes. Those details remain `TBD` until assigned to later
deliverables.

`core/rules/expression_evaluator` implements the first bounded evaluator
surface. It evaluates explicit expression trees only; it does not parse
arbitrary text or use host-language `eval`. The supported surface is deliberately
small: numeric literals, declared variable references, unary negation, basic
arithmetic, and same-dimension comparisons. Multiplication and division are
bounded to dimensionless scaling and same-dimension ratios until a later unit
algebra decision extends them.

`core/rules/rule_pack_lifecycle` implements private rule-pack lifecycle and
checksum evidence. It records rule-pack identity, version, privacy class,
redistribution status, review state, source notice, protected-content review
state, SHA-256 checksum records, JCS-compatible canonicalization metadata for
caller-supplied canonical JSON payload bytes, and audit-manifest references.
It emits deterministic findings for missing source notices, missing or unknown
redistribution state, pending review state, missing or stale checksums,
suspected protected content, attempted public export of private content, and
professional-boundary violations. It does not parse/canonicalize JSON, store
private rule-pack payloads, choose private storage paths, implement encryption
or access control, handle secrets, run GUI/report/API workflows, evaluate rule
expressions, or make professional/code-compliance claims.

`core/rules/completeness_checker` implements required-input completeness
checking for user-owned rule packs. It compares declarative required-input
records against caller-supplied input evidence for values, units, dimensions,
provenance, redistribution status, and review status. Blocking findings map to
`RULE_INPUTS_INCOMPLETE` and `RULE_CHECK_BLOCKING` semantics while preserving
the distinction between mechanics solve status and user-rule-check readiness.
It does not evaluate formulas, parse rule-pack files, store private data,
provide code-specific values, import protected standards content, or make
professional/code-compliance claims.

The evaluator reports deterministic findings for unsafe host-access requests,
unsupported expression forms, missing or duplicate bindings, invalid
references, missing required values, non-finite inputs, division by zero,
dimension mismatches, type mismatches, and human-approval status boundary
violations. It preserves the separation among mechanics solved, rule inputs
incomplete, user-rule checked, user-rule failed, and human review required. It
does not emit certification, sealing, authentication, human approval, or
code-compliance statuses.

## 8. GUI requirements

The GUI shall expose:

- 3D centerline viewport;
- model tree;
- property inspector;
- material/component/rule-pack editors;
- load case and combination manager;
- support/restraint editor;
- results viewer;
- warnings and blocking messages;
- report preview/export.

Missing data warnings must distinguish:

| Warning class | Meaning |
|---|---|
| `SOLVE_BLOCKING` | Required physical input missing. |
| `RULE_CHECK_BLOCKING` | Mechanics can solve, but rule-pack check lacks required user/code data. |
| `PROVENANCE_WARNING` | Value exists but source is missing or weak. |
| `ASSUMPTION_WARNING` | User or model assumption requires review. |
| `NONLINEAR_WARNING` | Convergence or active-state uncertainty exists. |
| `IP_BOUNDARY_WARNING` | Public contribution/report may contain protected or private data. |

The initial 3D viewport/editor contract is represented by
`schemas/viewport_editor.schema.yaml` and
`core/gui/viewport_editor`. It defines transient camera, hover, selection,
drag, snap, view-primitive, diagnostic, and command-intent records for nodes,
pipe runs, bend arcs, branch symbols, and simple component symbols. Durable
model edits are represented as application-service command intents; the
viewport contract does not directly mutate persisted project payloads.

The current viewport/editor implementation is a bounded contract and support
module because the repository has no frontend application scaffold yet. It does
not create a Tauri/React/Vite shell, render Three.js scenes, choose dependency
versions, select a state-management library, access host resources, parse
project files, implement adjacent GUI slices, provide protected engineering
data, or make professional/code-compliance claims. Runtime Three.js
integration, GUI package manifests, Playwright rendering checks, and final app
workflow behavior remain `TBD`.

## 9. Reporting and audit

Reports must include:

- software version and solver version;
- model hash and input manifest;
- unit system;
- material/component/rule-pack provenance summary;
- load cases and combinations;
- displacements, rotations, forces, moments, reactions, stresses;
- warnings, assumptions, missing data;
- rule-pack name/version/checksum;
- statement that code-specific data is user-supplied;
- statement that professional reliance requires competent human review.

Reports must not reproduce protected code text, protected standards tables, or proprietary formulas in public templates/examples.

`core/reporting/audit_manifest` implements the bounded audit-manifest and model
hash surface. It hashes structured canonical JSON values using sorted object
keys, records non-JSON asset hashes as separate manifest entries, captures
solver version stamps, unit-system references, rule-pack checksum references,
privacy/redaction metadata, and deterministic findings for missing manifest
evidence. It does not parse arbitrary project files, open or mutate the SCA-003
SQLite store, choose operating-system storage roots, store private rule-pack
payloads, import protected standards content, authenticate engineering work, or
make professional/code-compliance claims.

Analysis-status envelopes must preserve the project authority boundary:
software may emit `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`,
`RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, and
`HUMAN_REVIEW_REQUIRED`, but it must not emit `HUMAN_APPROVED_FOR_PROJECT` or
any code-compliance, certification, sealing, approval, authentication, or
professional-reliance equivalent as an automatic status. Any human acceptance
record is external, human-actor-owned, and bound to reviewed payload hashes.

The warnings, assumptions, and provenance report-section surface is represented
by `schemas/report_sections.schema.yaml` and the bounded
`core/reporting/report_sections` crate. It defines schema-first records for
report-facing diagnostics, analysis-status disclosures, provenance notes,
user-supplied values, assumptions, limitations, unresolved TBDs, and
human-review-needed findings. It does not implement the full report renderer,
final report template layout, GUI presentation, CLI runtime, API transport,
adapter behavior, private redaction/export controls, or release-template
integration; those remain `TBD` unless authorized by later deliverables.

Report-section records must preserve structured diagnostic fields from solver,
rule, provenance, unit, nonlinear, IP-boundary, and report checks. Numeric
user-supplied values required for mechanics solving or user-rule checking must
carry finite magnitude, unit, and dimension metadata. Provenance notes must
carry source, license, contributor certification, redistribution status, review
status, and privacy classification. Assumptions and limitations must identify
owner/source, affected scope, review status where applicable, and downstream
effect on mechanics solve, user-rule check, report completeness, and human
review. Missing required data remains an explicit finding and is never treated
as an accepted default.

Report sections may reference private rule packs, material libraries, component
libraries, owner requirements, and project values by identifier, checksum,
source note, privacy class, and review state, but public artifacts must not
copy private formulas, protected standards text, protected tables, proprietary
engineering values, private project data, private rule-pack payloads, or real
secrets. Report sections preserve the professional boundary: software output is
decision support and must not declare code compliance, certification, sealing,
approval, authentication, endorsement, or professional reliance.

The result export format is represented by `schemas/results.schema.yaml` and
the bounded `core/reporting/result_export` crate. The baseline is a
schema-first JSON result envelope for review, regression comparison, report
consumption, headless automation, and governed downstream tooling. Additional
export formats, public API transport, local FEA package format, external
adapter formats, GUI rendering, CLI runtime behavior, report rendering,
private redaction workflow, and release comparison thresholds remain `TBD`.

Result export envelopes must identify the result set, model/run basis, solver
version, unit-system reference, load-case or combination basis, diagnostics,
provenance, reproducibility hashes or audit-manifest reference, analysis
statuses, rule-pack references where present, and a professional-boundary
notice. Displacement, rotation, force, moment, reaction, stress, ratio, and
rule-check result values must carry explicit unit and dimensional metadata or
produce blocking diagnostics. Rule-pack references expose identity, version,
checksum, source notice, redistribution/private-public status, completeness
status, and redaction status; exports must not copy private formulas,
protected standards text, protected tables, proprietary values, or private
rule-pack payloads into public artifacts.

The calculation report generator is represented by
`schemas/report_generator.schema.yaml` and the bounded
`core/reporting/report_generator` crate. It assembles report-facing references
to persisted model inputs, load cases or combinations, result export envelopes,
audit manifests, report-section records, rule-pack references, diagnostics,
limitations, and professional-boundary notices into deterministic template
slots and neutral structured sections for tests. It does not read arbitrary
project files, call solver internals, choose final report styling/layout, run
GUI/CLI/API/adapter workflows, implement protected-content linting, implement
private-data redaction/export controls, or make professional/code-compliance
claims.

Calculation reports must preserve stable references, checksums, source notes,
privacy classification, review state, and provenance for input/report/result
payloads. Numeric values included through result/report references must carry
unit and dimensional metadata or explicit diagnostics from their source
envelopes. Missing solve-required or rule-check-required values remain
findings and are never hidden defaults. Report templates and public fixtures
must be original or invented project artifacts; they must not copy protected
standards text, protected tables, protected examples, proprietary formulas,
proprietary engineering values, private project data, private rule-pack
payloads, private library content, or real secrets.

The report protected-content linter is represented by
`schemas/report_protected_content_linter.schema.yaml` and the bounded
`core/reporting/protected_content_linter` crate. It records deterministic
heuristic findings over explicitly configured public report-template,
public-report-example, and public-fixture surfaces. The implementation uses
invented synthetic markers for fixtures and never requires protected examples
or `DEL-11-04` educational example models. It does not scan private user
templates by default, move files into quarantine, implement redaction/export
controls, choose CI or release policy, access host resources, or provide legal
clearance, security sufficiency, professional approval, certification, sealing,
endorsement, authentication, or code-compliance proof.

Protected-content linter findings must preserve target identity, path, line,
column, finding code, class, severity, matched policy, excerpt, remediation,
review route, disposition, and provenance. Clean linter output is review
evidence only and is not legal or professional clearance. Public-safe report
metadata such as rule-pack identity, version, checksum, source notice,
redistribution status, review state, provenance summary, diagnostics, and
professional-boundary notices may be present when they do not embed private
formulas, protected content, proprietary values, private project data, private
rule-pack payloads, private library content, or real secrets.

The headless runner contract is represented by
`schemas/headless_runner.schema.yaml` and the bounded
`core/runner/headless` crate. The baseline is a schema-first request/result
envelope for non-GUI solve execution, benchmark automation, regression
execution, audit-manifest capture, and result-export handoff. It is an
application-service client contract, not a solver shortcut. Runner requests
must identify operation, project, model, unit-system, load-case or combination
basis, input manifest, requested outputs, privacy, provenance, and the
professional-boundary posture. Runner results must identify job progress and
cancellation state, analysis statuses, result-export envelope reference,
audit-manifest reference, checksums, diagnostics, privacy, provenance, and the
professional-boundary posture.

`DEL-10-05` uses the `DEC-065` local `openpipestress-runner` policy for the
final CLI syntax, package-script posture, foreground process invocation,
network prohibition, and stdout/default-plus-explicit-output filesystem
posture. It still deliberately leaves public transport, CI provider, release
matrix, external adapter formats, operating-system storage roots, portable
project export/copy behavior, GUI/report runtime behavior, local FEA package
structure, and release automation details as `TBD`. Runner access to
persisted projects must stay behind application-service boundaries and must
not depend on direct SQL or SQLite table layout. Missing solve-required or
rule-check-required values must produce structured diagnostics rather than
silent defaults. Runner outputs must remain compatible with
`schemas/results.schema.yaml` rather than introducing an ad hoc regression or
automation output shape. Public fixtures or examples for runner tests must use
invented or otherwise permitted data only and must not embed protected
standards text, protected tables, proprietary values, private project data,
private rule-pack payloads, real secrets, or real user paths.

## 10. Verification and validation mechanics

The project shall maintain:

- unit tests for units, schemas, expression evaluation, and status semantics;
- hand-calculation benchmark cases for mechanics;
- regression tests for solver and stress recovery;
- nonlinear support convergence tests;
- report reproducibility tests;
- public validation manual using original/public/permissive examples only.

`validation/benchmarks/mechanics/` is the initial mechanics benchmark crate for
`DEL-09-01`. It records public-original fixture provenance, dimensioned
expected values, unresolved tolerance policy, and focused automated comparisons
for cantilever, frame, thermal-growth, imposed-displacement, and
stiffness-transform families. Hand-calculation notes for those fixtures live in
`validation/hand_calcs/mechanics/`.

## 11. Agentic development mechanics

Downstream implementation should use the decomposition package as the source of work identity:

```text
PKG-XX_<PackageLabel>/
└── 1_Working/
    └── DEL-XX-YY_<DeliverableLabel>/
        ├── _STATUS.md
        ├── _CONTEXT.md
        ├── _REFERENCES.md
        ├── _DEPENDENCIES.md
        ├── Datasheet.md
        ├── Specification.md
        ├── Guidance.md
        └── Procedure.md
```

Each deliverable must be executable by a bounded Type 2 specialist. If a deliverable becomes cross-domain or too large, it must be split or explicitly accepted as a human-approved open issue.

## 12. Acceptance semantics

A software increment may be accepted for development use only when:

1. scope and deliverable ID match the decomposition;
2. tests and/or documentation artifacts listed in the deliverable exist;
3. no protected public data has been introduced;
4. missing data and assumptions are surfaced;
5. solver/rule/report changes pass required validation gates;
6. a human accepts the work at the review gate.
