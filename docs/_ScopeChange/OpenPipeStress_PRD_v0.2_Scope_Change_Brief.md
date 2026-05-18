# Scope-Change Brief: `OpenPipeStress_PRD_v0.2.md` Compared to `PRD.md`

**Purpose:** This document describes how `OpenPipeStress_PRD_v0.2.md` differs from the earlier uploaded `PRD.md`, from the perspective of future software-development agents changing the scope of work for OpenPipeStress.

---

## Executive Summary

The new PRD does **not** remove the piping stress analytical engine. It preserves that ambition. The major change is that the product is no longer scoped as a standalone “free and open-source piping stress analysis platform,” as in the original PRD, but as an **analysis-grade piping design engine and stress-model authoring environment** that uses a full internal solver for design iteration and produces handoff-ready model data for downstream professional validation.

The original PRD positioned the product around global piping flexibility and stress analysis. The new PRD positions it around design knowledge, schema-backed physical modeling, GUI/agent-assisted iteration, state comparison, and prover-tool handoff.

For development planning, the main instruction is:

> Keep building the serious piping stress engine, but move the software architecture outward from “solver + GUI + reports” to “schema-backed design model + solver + GUI + model states + comparison + handoff workflow.”

---

# 1. Product Identity Changed

## Original PRD

The original PRD describes OpenPipeStress as a:

> “Free and open-source piping stress analysis platform.”

Its product vision is to become the leading transparent, auditable, code-neutral, open-source platform for piping flexibility and stress analysis. The core concept is “open mechanics in the solver; user-owned code knowledge in private data and rule packs.”

## New PRD

The new PRD describes OpenPipeStress as an:

> “Analysis-grade piping design engine and stress-model authoring environment.”

It explicitly says the product is **not intended to become an industry-standard professional stress-analysis platform** or replace authenticated commercial software. Instead, it operates earlier in the design workflow, where routes, supports, constraints, equipment interfaces, and design assumptions are still evolving.

## Developer Impact

Future development should not treat the product as merely an open CAESAR II / AutoPIPE substitute. The implementation should support a broader design-authoring workflow:

```text
design knowledge
→ physical model
→ analytical model
→ internal solve
→ route/support iteration
→ saved model states
→ comparison
→ handoff package
→ external validation by professional tool
```

The solver remains core, but it is no longer the whole product.

---

# 2. The Authority Boundary Is Now More Explicit

## Original PRD

The original PRD already included non-goals and disclaimers: it would not claim ASME certification, would not replace the responsible engineer, and would not provide automatic legal interpretation of piping codes.

## New PRD

The new PRD makes this distinction central to the product boundary:

> The product boundary is not defined by solver capability. OpenPipeStress shall have a full analytical engine. The boundary is defined by claim authority.

The new governing distinction is:

```text
OpenPipeStress computes and helps design.
The external prover tool validates for reliance.
The responsible engineer accepts.
```

## Developer Impact

Do not weaken the solver because the tool is “non-authoritative.” The solver should still be correct, deterministic, benchmarked, and useful.

But UI labels, reports, exports, comparison screens, agent output, and documentation must avoid language that implies final approval, certification, code compliance for reliance, or professional acceptance.

---

# 3. A Schema-Backed Physical Model Is Now the Source of Truth

## Original PRD

The original PRD required project files, a model tree, a property editor, GUI modeling, and round-trippable data, but it mostly described the model from the perspective of a stress-analysis input file.

## New PRD

The new PRD makes the **schema-backed physical model** the central source of truth. It distinguishes three model layers:

```text
Physical model
Analytical model
Handoff model
```

The physical model is richer than the analytical model. It includes geometry, components, supports, constraints, equipment interfaces, provenance, unresolved assumptions, and design intent. The analytical model is the idealized solver-ready representation. The handoff model is the target-specific export representation.

## Developer Impact

Future agents should implement model architecture as layered data, not as one flat solver input schema.

Required architecture shift:

```text
Old emphasis:
  project file → solver model → results/report

New emphasis:
  physical model → analytical transform → solver run → result state
                 → handoff transform → downstream package
```

The physical model should support design entities that may not directly appear in the solver, such as routing corridors, no-go volumes, supportable zones, design knowledge records, unresolved assumptions, and future agent operation rationale.

---

# 4. Design Knowledge and Constraints Were Added as First-Class Product Scope

## Original PRD

The original PRD focused on materials, components, load cases, private rule packs, solver inputs, and reports. It did not define a broad design knowledge layer for routing, supportability, equipment interfaces, accessibility, constructability, or project constraints.

## New PRD

The new PRD adds a full **Design Knowledge** concept. It includes line lists, P&ID-derived connectivity, equipment/nozzle locations, pipe specifications, routing corridors, no-go volumes, rack elevations, supportable structural zones, valve access, slope/drain/vent requirements, insulation/clearance envelopes, construction constraints, owner preferences, and stress-review triggers.

It also adds a **constraint engine** for connectivity, endpoint matching, supportable span ranges, clearance envelopes, no-go volumes, slope requirements, drain/vent logic, valve access zones, supportable structure zones, and missing required data.

## Developer Impact

Add new domain modules beyond the stress solver:

```text
design_knowledge/
constraints/
routing_context/
support_zones/
equipment_interfaces/
clearance_volumes/
provenance/
```

The GUI should allow users to enter, inspect, and validate these constraints. Future route/support generation should consume this layer rather than inventing geometry directly.

---

# 5. GUI Scope Expanded from Stress Modeler to Design-Authoring Environment

## Original PRD

The original GUI requirements included a 3D centerline viewport, model tree, property inspector, material and component library editors, load-case manager, rule-pack manager, solver console, results browser, and report generator.

## New PRD

The new GUI scope adds:

- operation history;
- design knowledge panel;
- constraint/warning panel;
- model state and analysis run browser;
- comparison browser;
- comparison overlays;
- routing corridors;
- no-go volumes;
- supportable zones;
- model-state overlay comparisons.

## Developer Impact

The GUI should not be designed only as a stress-input editor. It should be designed as an interactive model authoring environment.

The viewport needs to support both engineering-analysis visualization and design-context visualization:

```text
Analysis overlays:
  deformed shape
  reactions
  displacements
  stresses
  terminal loads

Design overlays:
  route corridors
  forbidden zones
  support zones
  clearance envelopes
  changed entities
  comparison overlays
```

This likely affects scene graph design, entity selection, property binding, undo/redo, operation history, and model-state visualization.

---

# 6. Agent-Assisted Model Operations Are Now in Scope

## Original PRD

The original PRD did not meaningfully include agent-assisted design. It focused on human GUI workflow, rule packs, solver, validation, and reports.

## New PRD

The new PRD adds an agent-enhanced workflow. Agents may propose route/support/model changes, but those proposals must become structured model operations, pass schema and constraint validation, be shown as diffs, and require user acceptance unless configured otherwise.

## Developer Impact

Do not let future agent features mutate the model through free-form text or hidden side effects.

Implement an operation pipeline:

```text
agent/user instruction
→ proposed model operation
→ schema validation
→ constraint validation
→ diff preview
→ user accept/reject
→ committed model state
```

This means future development should introduce a formal operation model early, even before advanced agents exist.

Example operation categories:

```text
add_route_segment
move_node
insert_bend
add_support
change_support_type
modify_pipe_section
add_no_go_volume
assign_material
revise_load_case
```

---

# 7. Model States and Analysis Runs Are New Core Entities

## Original PRD

The original PRD required create/open/save/version projects and reproducible reports, but it did not define immutable model states or analysis runs as first-class entities.

## New PRD

The new PRD defines:

- **Model State:** a named, immutable snapshot of a model at a point in time;
- **Analysis Run:** a solver execution attached to a specific model state;
- **Comparison:** a deterministic diff between two model states and/or two analysis runs.

Each analysis run must bind to exact model state, solver version, settings, units, load cases, and rule-pack/library references.

## Developer Impact

The persistence model needs to support immutable snapshots and run records, not just mutable project files.

Add concepts like:

```text
Project
  ModelState[]
    PhysicalModel
    AnalyticalInputBasis
    Assumptions
    Warnings
    Hash

  AnalysisRun[]
    model_state_id
    solver_version
    settings
    load_cases
    diagnostics
    results
    results_hash
```

This also affects report generation, reproducibility, comparison, undo/redo, export, and validation.

---

# 8. A Generic Comparison Tool Was Added

## Original PRD

The original PRD included validation benchmarks, report reproducibility, and possible commercial software comparison as an open question, but it did not include a product feature for comparing two model states or analysis result sets.

## New PRD

The new PRD adds a generic deterministic comparison tool. It compares two model states and optionally two analysis runs using stable IDs, manual mappings, unit-normalized quantities, and tolerance profiles. It is explicitly **not** a comprehensive external-prover result ingestion platform in the MVP.

## Developer Impact

Add a `compare` subsystem with these responsibilities:

```text
state diff:
  added / removed / changed entities
  geometry changes
  topology changes
  support changes
  load changes
  material changes
  solver-setting changes
  warning/assumption changes

result diff:
  displacement deltas
  rotation deltas
  force/moment deltas
  support reaction deltas
  equipment terminal load deltas
  stress/result deltas
  diagnostics deltas
```

Entity mapping must support at least:

```text
stable ID match
manual user mapping
unmatched / added / deleted classification
```

Future external result comparison should be accommodated architecturally, but not implemented as an MVP dependency.

---

# 9. External Prover Workflow Added, but Heavy Integration Deferred

## Original PRD

Import/export was a secondary goal or “Could” requirement. CAD/BIM/CAE exchange and commercial software comparisons were future or open-ended topics.

## New PRD

The new PRD makes downstream handoff central. It defines an **external prover tool** as an accepted professional stress-analysis platform used outside OpenPipeStress to validate results for project reliance. It also defines a handoff package containing canonical model data, units manifest, entity IDs, library references, rule references, unresolved assumptions, warnings, hashes, and target mapping metadata.

However, the MVP explicitly does **not** include comprehensive external commercial tool result ingestion, forced prover status lifecycle, or automatic professional acceptance records.

## Developer Impact

Implement export/handoff infrastructure before building target-specific commercial-tool parsers.

Near-term scope:

```text
canonical schema export
handoff manifest
model hash
units manifest
entity ID manifest
library/rule references
warnings
unresolved assumptions
target mapping metadata where known
unsupported-target flags
```

Deferred scope:

```text
CAESAR II output parser
AutoPIPE output parser
automatic prover validation status
formal approval lifecycle
commercial-tool result reconciliation
```

Future agents should not overbuild external integration prematurely.

---

# 10. Formal Prover-Status Lifecycle Was Intentionally Not Added

## Original PRD

The original PRD had no prover-status lifecycle. It did have report signoff, disclaimers, and professional limitations.

## New PRD

The new PRD explicitly says the MVP shall **not** force a formal prover-status vocabulary or lifecycle. Instead, users can use model names, state names, tags, notes, external reference fields, attachments/links, and comparison reports.

## Developer Impact

Do not build a rigid approval workflow in the MVP.

Implement flexible metadata instead:

```text
state.name
state.tags[]
state.notes[]
state.external_references[]
comparison.name
report.notes
```

Avoid hard-coded statuses like:

```text
PROVER_VALIDATED
APPROVED
CERTIFIED
CODE_COMPLIANT
READY_FOR_CONSTRUCTION
```

Those labels would create professional-boundary risk and are contrary to the chosen scope.

---

# 11. Requirements Were Reorganized into More Implementation-Specific Domains

## Original PRD

The original PRD used a single functional requirements table with FR-001 through FR-025. It covered project files, units, modeling, solver, rule packs, GUI, reports, components, nonlinear supports, import/export, dynamic modules, and local FEA export.

## New PRD

The new PRD reorganizes requirements into domain-specific groups:

```text
FR-MOD   model and schema
FR-KNOW  design knowledge and constraints
FR-GUI   graphical interface
FR-SOL   analytical engine
FR-RULE  rule packs and private data
FR-CMP   model state / analysis run / comparison
FR-HAND  handoff and export
FR-AGENT agent-assisted design
FR-REP   reporting
```

## Developer Impact

Future work breakdown should follow these subsystem boundaries. The new PRD is better suited for modular architecture and backlog creation.

A likely repository/module structure should now include more than the original `core`, `rules`, `gui`, `reports`, and `validation` directories. It should also include concepts such as:

```text
schema/
physical_model/
analytical_model/
model_transform/
design_knowledge/
constraints/
operations/
states/
runs/
comparison/
handoff/
agents/
```

---

# 12. Analytical Solver Scope Mostly Remains Intact

## What Stayed the Same

The new PRD retains the core solver commitments from the original PRD:

- six-degree-of-freedom nodes;
- 3D frame/line-element analysis;
- straight pipe elements;
- rigid elements;
- linear supports/restraints;
- load cases;
- weight, thermal, pressure metadata, imposed displacement;
- force/moment recovery;
- fundamental stress recovery;
- bends, branches, valves, flanges, reducers, expansion joints;
- nonlinear restraints;
- singularity and ill-conditioning diagnostics.

## What Changed

The solver is now explicitly embedded in a broader design workflow. The new PRD describes the analytical model as being derived from the physical model, and results are attached to analysis runs tied to immutable model states.

## Developer Impact

Do not interpret v0.2 as reducing solver seriousness.

The correct implementation priority is:

```text
solver correctness remains mandatory
+
solver inputs/outputs must be state-bound, traceable, comparable, and exportable
```

The analytical engine should be developed with stable serialization, result schemas, reproducibility, comparison, and handoff in mind from the beginning.

---

# 13. Dynamic Analysis Was Deprioritized

## Original PRD

The original post-MVP scope included dynamic modules: modal, response spectrum, harmonic, and time-history support. The functional requirements table also included dynamic analysis as a “Could” requirement.

## New PRD

The new PRD does not emphasize dynamic analysis in the main roadmap. It says later modules may add dynamics, but the release milestones are now focused on schema, solver, GUI, states/runs/comparison, rule packs/private libraries, piping components/nonlinear supports, design knowledge/handoff, and agent-assisted design.

## Developer Impact

Dynamic analysis should be deferred unless specifically reprioritized.

Near-term development should favor:

```text
state/run infrastructure
comparison
physical-to-analytical transformation
handoff manifests
design constraints
GUI model authoring
```

over:

```text
modal analysis
response spectrum
harmonic response
time history
```

---

# 14. Release Roadmap Changed Significantly

## Original Roadmap

The original release path was:

```text
R0 Architecture Prototype
R1 Core Solver MVP
R2 GUI MVP
R3 Rule-Pack and Private Libraries
R4 Piping Components and Nonlinear Supports
R5 Engineering Beta
```

## New Roadmap

The new roadmap is:

```text
R0 Schema and Solver Prototype
R1 Core Analytical Engine MVP
R2 GUI and Physical Model MVP
R3 States, Runs, and Generic Comparison
R4 Rule Packs and Private Libraries
R5 Piping Components and Nonlinear Supports
R6 Design Knowledge and Handoff Beta
R7 Agent-Assisted Design and Candidate Generation
```

## Developer Impact

Two new major releases were inserted:

```text
R3: states/runs/comparison
R6: design knowledge/handoff
R7: agents/candidate generation
```

Rule packs move later relative to comparison. The project should not rush directly from solver/GUI into rule packs and components without establishing model-state persistence and comparison infrastructure.

---

# 15. MVP Scope Expanded in Some Places and Narrowed in Others

## Added to MVP

The new MVP adds:

- canonical project schema;
- local-first storage;
- named immutable model states;
- analysis runs tied to states;
- basic model-state comparison;
- basic analysis-run comparison;
- export of canonical schema-compliant model package;
- IP/data-boundary notices;
- professional-boundary notices.

## Still in MVP

The following remain in MVP:

- 3D six-degree-of-freedom nodes;
- straight pipe/beam elements;
- rigid elements;
- anchors and linear restraints;
- weight loads;
- thermal loads;
- pressure metadata;
- imposed displacements;
- linear static solver;
- force/moment recovery;
- fundamental stress recovery;
- basic GUI modeler;
- private rule-pack schema with non-code examples;
- missing-data blockers;
- report generation;
- validation suite.

## Explicitly Excluded from MVP

The new PRD explicitly excludes:

- comprehensive external commercial tool result ingestion;
- forced prover-status lifecycle;
- protected standards data;
- automatic professional acceptance records;
- advanced nonlinear/dynamic modules unless otherwise prioritized.

## Developer Impact

The MVP is now more architecture-heavy. It needs durable data modeling and reproducibility earlier. The main added implementation burden is not numerical; it is model lifecycle infrastructure.

---

# 16. Reporting Changed from “Calculation Report” to Broader Audit/Report Package

## Original PRD

The original PRD focused on calculation reports with model summaries, load cases, rule-pack checksums, warnings, results, and signoff blocks.

## New PRD

The new PRD expands reports to include:

- model summary reports;
- analysis result reports;
- warning and assumption reports;
- user-rule check reports;
- model state comparison reports;
- analysis run comparison reports;
- handoff package manifests.

It also includes a stronger report notice identifying the software as decision-support software and stating that it does not certify, seal, approve, authenticate, or determine code compliance for professional reliance.

## Developer Impact

Reports should be generated from state/run/comparison/handoff records, not only from the current mutable project model.

Report generator inputs should include:

```text
model_state_id
analysis_run_id
comparison_id optional
handoff_package_id optional
rule_pack checksum
library references
warnings
assumptions
hashes
```

---

# 17. Validation Scope Now Includes Reproducibility and Comparison Determinism

## Original PRD

The original PRD included solver benchmarks, stress recovery benchmarks, regression testing, and a validation manual.

## New PRD

The new PRD keeps those but adds tests for:

- schema round-trip;
- model state hashing;
- analysis run reproducibility;
- comparison output determinism;
- report reproducibility.

It also reframes external professional tool comparisons as optional validation evidence, not automatic professional acceptance.

## Developer Impact

CI/test planning must cover persistence and lifecycle behavior, not just solver math.

Add regression tests such as:

```text
same model state + same solver version → same result hash
same pair of states + same mapping/tolerances → same comparison output
schema serialize/deserialize → no loss of typed data
report generation from same state/run → reproducible report metadata
```

---

# 18. IP/Data Governance Remained Mostly Unchanged but Was Integrated More Strongly

## Original PRD

The original PRD already had strong IP/data restrictions: no ASME text, tables, B31J content, B16/B36 tables, protected examples, or commercial software material without permission.

## New PRD

The new PRD retains those requirements and aligns them with the separate IP/Data Boundary Policy, which states that the public repository may contain open solver algorithms, schemas, blank templates, invented example data, permissively licensed data, validation benchmarks, and private-library import mechanisms, but must not contain protected standards data, proprietary vendor catalogs without rights, commercial software examples without permission, private rule packs, owner standards, or protected code-specific acceptance criteria.

## Developer Impact

No major scope reversal here. Continue implementing private-by-default rule packs, libraries, provenance metadata, and protected-content quarantine workflows.

The difference is that provenance and private-data handling now also apply to:

```text
design knowledge
constraints
handoff packages
comparison reports
agent rationale
external references
```

not only materials, components, and rule packs.

---

# 19. Personas Shifted Toward Designers and Agent-Enhanced Users

## Original PRD

The original personas were:

- piping stress engineer;
- owner-operator reviewer;
- engineering company administrator;
- open-source contributor;
- educator/researcher.

## New PRD

The new personas include:

- piping designer / layout engineer;
- piping stress engineer;
- owner-operator reviewer;
- engineering company administrator;
- open-source contributor;
- agent-enhanced user.

The educator/researcher persona is no longer a primary persona, though educational/research examples remain a secondary goal.

## Developer Impact

UX should support design iteration, not just stress-calculation report preparation.

Prioritize workflows like:

```text
create route
place supports
inspect constraints
run analysis
modify support
save new state
compare before/after
export handoff
```

not only:

```text
build stress model
run code check
issue calculation report
```

---

# 20. The New PRD Removed or Deemphasized Some Original Sections

The new PRD does not carry over the original sections on:

- detailed platform/technical architecture directory layout;
- performance requirements;
- accessibility and usability requirements;
- explicit source notes and external ASME reference pages;
- detailed technology selection criteria.

This does not necessarily mean those requirements are invalid. It means v0.2 focused on product scope and workflow realignment.

## Developer Impact

Future agents should not assume performance, accessibility, or architecture concerns are intentionally abandoned. They should be reintroduced in a future technical architecture document or v0.3 PRD.

Recommended follow-up artifacts:

```text
TECHNICAL_ARCHITECTURE.md
DATA_MODEL_SPEC.md
COMPARISON_ENGINE_SPEC.md
HANDOFF_PACKAGE_SPEC.md
GUI_UX_REQUIREMENTS.md
PERFORMANCE_REQUIREMENTS.md
ACCESSIBILITY_REQUIREMENTS.md
AGENT_OPERATION_SPEC.md
```

---

# 21. Practical Backlog Changes for Developers

## Add These New Epics

### Epic 1: Canonical Physical Model Schema

Implement a schema that can represent:

```text
physical entities
analytical entities
constraints
design knowledge
provenance
warnings
assumptions
external references
stable IDs
```

### Epic 2: Physical-to-Analytical Transformation

Implement deterministic transformation from physical model to solver model, including warnings when physical design data cannot be represented analytically.

### Epic 3: Model States and Analysis Runs

Implement immutable snapshots, result records, hashes, and reproducibility metadata.

### Epic 4: Generic Comparison Engine

Implement deterministic diffs for model states and analysis runs, including stable-ID mapping, manual mapping, tolerance profiles, and tabular/graphical output.

### Epic 5: Handoff Package Generator

Implement canonical model export, manifests, hashes, warnings, unresolved assumptions, entity ID mappings, and target mapping metadata.

### Epic 6: Design Knowledge and Constraint Engine

Implement routing corridors, no-go volumes, support zones, clearance checks, equipment interfaces, and explicit design-knowledge records.

### Epic 7: Agent Operation Framework

Implement structured operations, schema validation, diff preview, user acceptance, rationale capture, and audit trail.

---

# 22. Defer These Until Later

Future agents should avoid pulling these into the MVP unless the project owner explicitly reprioritizes them:

```text
commercial stress software output parsers
automatic external prover comparison
formal prover lifecycle/status enforcement
automatic professional acceptance records
dynamic analysis modules
comprehensive CAD/BIM integration
advanced route optimization
automatic code-compliance claims
cloud-dependent workflows
```

The new PRD explicitly defers comprehensive external-prover result ingestion and forced prover status, while preserving future external result states as an architectural possibility.

---

# 23. Keep These from the Original PRD

Future agents should **not** discard the following original scope. The new PRD intentionally preserves it:

```text
full analytical piping stress engine
3D frame/line-element solver
six-degree-of-freedom nodes
pipe, bend, branch, rigid component, reducer, expansion joint concepts
linear and nonlinear supports
fundamental stress recovery
load cases and combinations
user-supplied rule packs
private material/component libraries
unit safety
no silent engineering defaults
IP/data separation
validation benchmarks
auditable reports
local-first operation
```

The project still follows the original intent of open mechanics and private code data, but now adds design-engine scope, state comparison, handoff, and external professional validation boundaries.

---

# 24. One-Sentence Guidance for Future Agents

When changing development scope, future agents should treat `OpenPipeStress_PRD_v0.2.md` as shifting the project from:

> **an open piping stress analysis platform**

to:

> **an analysis-grade piping design engine with a full stress-analysis core, schema-backed physical model, GUI/agent-assisted iteration, immutable model states, deterministic comparison, and handoff packages for downstream professional validation.**

The solver remains essential; what changed is the surrounding product architecture and the authority boundary.
