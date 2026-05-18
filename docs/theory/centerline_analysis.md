---
doc_id: OPS-THEORY-CENTERLINE-ANALYSIS
doc_kind: theory.note
status: draft
created: 2026-05-03
deliverable_id: DEL-11-03
package_id: PKG-11
scope_item: SOW-033
objectives:
  - OBJ-001
  - OBJ-003
---

# Classical To Modern Centerline Analysis

## Purpose And Boundary

These notes explain why OpenPipeStress treats routine global piping analysis as
a 3D centerline/frame problem and where that model stops. The intent is
educational: give users and contributors a shared mechanics vocabulary for the
model, solver results, rule-pack checks, validation evidence, and local FEA
handoff boundary.

This document is not a design-code guide. It does not reproduce protected
standards text, protected examples, standards tables, material allowables,
stress-intensification-factor tables, flexibility-factor tables, copied code
formulas, proprietary vendor data, private owner data, or private rule-pack
values. Mechanics statements are project-derived unless a source is identified
in the source notes. External public sources for the detailed history of
classical piping flexibility practice remain `TBD`.

OpenPipeStress outputs are software evidence for review. A mechanics solve,
user-rule check, report, or agent draft is not professional acceptance for a
project-specific piping design.

## The Classical Centerline Idea

Classical piping flexibility work starts from a practical abstraction: a piping
system can be represented by its connected centerline, with components,
supports, anchors, loads, and restraints attached to that line. Instead of
meshing every wall thickness, weld detail, nozzle, lug, shoe, or branch
reinforcement as a local continuum problem, the global model asks how the
connected run moves, what reactions develop, and what force and moment
resultants are recovered at engineering stations.

In that lineage, the important modeling move is not a particular software
implementation. It is the separation between:

- global flexibility of the connected piping network;
- local component behavior represented through user-supplied section,
  stiffness, flexibility, stress, or manufacturer data;
- acceptability checks that depend on the user's selected rules and project
  basis.

OpenPipeStress preserves that separation. The public project may define schema
slots and mechanics interfaces for component modifiers and rule inputs, but it
does not bundle protected code values or proprietary component libraries.
User-supplied SIFs, flexibility factors, stress indices, allowables, load
combinations, component dimensions, and manufacturer stiffnesses remain input
data with provenance, not public defaults.

## Modern 3D Frame Interpretation

OpenPipeStress maps the centerline idea to a 3D frame model. In the project
architecture, the primary global analysis model is a connected line-element
system. Nodes carry translational and rotational degrees of freedom. Elements
connect nodes, carry local axis definitions, contribute stiffness to a global
system, and recover local force and moment resultants after solving.

The current theory vocabulary is:

| Concept | Centerline meaning | OpenPipeStress boundary |
|---|---|---|
| Node | A point on the analytical centerline where elements, supports, loads, or components meet. | Carries explicit mechanical degrees of freedom and must be backed by unit-aware model data. |
| Element | A centerline segment or component representation between nodes. | Provides mechanics behavior such as axial, torsional, bending, thermal, load, and recovery behavior as implemented by assigned solver slices. |
| Local frame | Element-specific coordinate basis used to express element behavior and result recovery. | Must transform consistently to the global model; unresolved conventions remain explicit `TBD`s, not hidden defaults. |
| Global frame | Common coordinate basis for assembly, supports, loads, displacements, reactions, and reporting. | Must remain unit-aware and deterministic for a stated model and solver basis. |
| Support or restraint | A boundary condition or stiffness contribution that limits motion or adds a reaction path. | Missing, invalid, or unsupported support data becomes a diagnostic, not an invented assumption. |
| Load case | A stated mechanical loading basis, such as weight, thermal change, pressure metadata, imposed displacement, or user-supplied loads. | Code-specific load combinations are not public defaults; rule-pack or project inputs provide them. |
| Resultant | A recovered force, moment, displacement, reaction, or mechanics stress component. | Resultants are mechanics outputs; code-category interpretation belongs to user rule packs and human review. |

This frame interpretation lets the project keep global mechanics testable and
auditable. A straight pipe element can be verified against open mechanics
benchmarks without importing protected standards data. Load application,
support behavior, stress recovery, and diagnostics can each be tested as
bounded software surfaces.

## What The Global Model Is Good For

The global centerline model is intended for system-level piping behavior:

- displacement and rotation response of the connected run;
- support, anchor, spring, guide, line-stop, and equipment terminal reactions;
- element force and moment resultants;
- mechanics-only stress recovery from open quantities and user-supplied section
  or pressure basis data;
- deterministic diagnostics for incomplete models, invalid restraints,
  singular systems, nonconvergence, or unresolved assumptions;
- reproducible report evidence for a competent reviewer.

The model is especially useful because it keeps the entire system connected.
Thermal growth, imposed displacements, restraint layout, support stiffness,
weight distribution, and load-case algebra are global effects. A local shell or
solid mesh can answer a different question in more detail, but it does not
replace the need to understand the whole line's load path.

## What The Global Model Does Not Decide

The global model is not a private rule library, a standards database, a
professional approval workflow, or a local-detail substitute. It does not
decide:

- which design code, owner standard, edition, or project basis applies;
- what protected code values, allowables, SIFs, flexibility factors, stress
  indices, component dimensions, or load combinations should be used;
- whether a user has the right to redistribute private or licensed source data;
- whether a local component detail is adequately represented by a line-element
  abstraction;
- whether a report is acceptable for professional reliance.

Those questions require user-supplied data, provenance, rule-pack evaluation,
private project governance, and competent human review.

## Rule-Pack Checks Are A Separate Layer

OpenPipeStress separates mechanics from rule checks. The solver computes
mechanics quantities such as model findings, displacements, rotations,
reactions, forces, moments, stresses, convergence facts, and diagnostics.
Rule-pack evaluation is a separate user-data computation. It may map mechanics
results into user-defined categories, check required inputs, evaluate
user-supplied expressions, and report user-rule outcomes.

This separation matters because a mechanically solved model can still be
incomplete for rule checking. For example, a model may have enough information
to solve displacements and reactions while still missing a user-required
allowable, SIF, flexibility factor, component source, load-combination mapping,
or provenance field. In that case, the appropriate software behavior is an
explicit finding, not a silent default.

## Mechanics Verification And Validation

Mechanics verification asks whether the software solves the stated mechanics
problem correctly within declared tolerances. The current project validation
strategy names frame mechanics, piping loads, stress recovery, nonlinear
supports, rule packs, and reports as separate benchmark families. This is
deliberate: a passing frame benchmark does not prove a user-rule pack is
complete, and a user-rule check does not prove professional acceptance.

For centerline analysis, useful verification evidence includes:

- simple beam and frame fixtures with invented or permissively sourced values;
- transformation checks for inclined members and local/global result recovery;
- thermal-growth and imposed-displacement cases;
- support reaction and singular-model diagnostics;
- stress recovery checks for open mechanics components;
- protected-content and provenance review for public examples and report
  templates.

Release-quality tolerances, final solver numerical library choices, and final
validation gate wording remain governed by future accepted deliverables where
not already recorded.

## Local FEA Handoff Boundary

Local FEA is a handoff path for questions that are not well answered by a
global centerline model alone. Typical local-analysis subjects include nozzles,
trunnions, lugs, branch details, attachments, specialty components, or local
load introduction regions. In those cases, the global model can provide
boundary loads, displacements, reactions, or context, while the local shell or
solid model studies local stress distribution, geometric detail, contact,
load introduction, or component-specific behavior.

The handoff boundary should preserve:

- the model and result basis used to derive local loads or displacements;
- units, coordinate frames, sign conventions, and load-case identity;
- source and provenance for component, material, and rule data;
- warnings, assumptions, incomplete inputs, and limitations;
- hash or version references where the project supports them.

The handoff should not imply that OpenPipeStress has performed the local FEA,
accepted the local model, or made the professional judgment for the project.
External FEA setup, meshing, boundary conditions, material modeling, local
acceptance criteria, and final interpretation remain outside the global solver
authority unless a future governed integration explicitly implements and
validates them.

## Practical Reading Of Results

A centerline result should be read as an answer to a declared model, not as a
property of the physical plant by itself. Review should consider:

- whether the physical centerline, nodes, components, supports, and restraints
  match the intended project abstraction;
- whether units, material properties, section properties, load definitions,
  support stiffnesses, and component inputs are complete and sourced;
- whether missing values are solve-blocking, rule-check-blocking, or review
  limitations;
- whether stress recovery quantities are open mechanics components or
  user-rule categories;
- whether local details require a shell/solid handoff;
- whether the report preserves warnings, provenance, assumptions, hashes, and
  limitations.

The intended workflow is transparent review, not black-box acceptance.

## Source Notes

Project-authored sources used for this draft:

- `docs/CONTRACT.md` - invariant catalog for protected content, mechanics,
  unit, solver, report, and professional-boundary constraints.
- `docs/PRD.md` - product vision, non-goals, global line-element model,
  local-analysis handoff, rule-pack, and analytical-engine requirements.
- `docs/SPEC.md` - solver core, straight-pipe/frame element, support, load,
  stress-recovery, rule-pack, and diagnostics architecture notes.
- `docs/IP_AND_DATA_BOUNDARY.md` - public/private data and protected-content
  boundaries.
- `docs/PROFESSIONAL_BOUNDARY.md` - product-claim and professional-reliance
  boundaries.
- `docs/VALIDATION_STRATEGY.md` - mechanics verification and validation
  family separation.
- `docs/architecture/code_neutral_analysis_boundary.md` - mechanics,
  user-rule, and human-acceptance authority separation.
- `docs/architecture/analysis_status_semantics.md` - status vocabulary for
  mechanics solved, user-rule checked, and human review required.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5` - package,
  objective, scope, and architecture-basis context.

External citation needs:

- `TBD-public-history`: public, redistributable source for the historical
  development of piping flexibility analysis terminology.
- `TBD-open-frame-reference`: public or permissively citable structural
  analysis reference for 3D frame modeling concepts if formula-level theory is
  added later.
- `TBD-local-fea-reference`: public or permissively citable source for local
  shell/solid handoff practice if future notes expand beyond project boundary
  language.
