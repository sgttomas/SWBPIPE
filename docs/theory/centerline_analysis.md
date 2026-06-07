---
doc_id: OPS-THEORY-CENTERLINE-ANALYSIS
doc_kind: theory.note
status: draft
created: 2026-05-03
updated: 2026-06-07
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
in the source notes. Public-source support added for this revision is
claim-limited: open mechanics and finite-element sources support general
statics, frame, and FEA context, not code-specific piping practice. External
public sources for the detailed history of classical piping flexibility
practice remain `TBD`.

OpenPipeStress outputs are software evidence for review. A mechanics solve,
user-rule check, report, or agent draft is not professional acceptance for a
project-specific piping design.

## The Centerline Idea

For OpenPipeStress, the useful centerline idea is a practical abstraction: a
piping system can be represented by its connected centerline, with components,
supports, anchors, loads, and restraints attached to that line. Instead of
making every global study a wall-by-wall continuum model, the global model asks
how the connected run moves, what reactions develop, and what force and moment
resultants are recovered at engineering stations.

Open educational statics sources support the ordinary mechanics vocabulary used
here: forces, moments, equilibrium, internal reactions, and internal resultants.
Public NASA and MIT finite-element sources support the broader structural
analysis context in which a structure is idealized into connected elements or
model parts, loads are applied to the model, and displacements or resultants are
recovered. Those sources do not establish a detailed history of piping
flexibility practice; that historical lineage remains `TBD-public-history`.

For this note, the important modeling move is not a particular software
implementation or a protected method. It is the separation between:

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

This is a conceptual bridge from the general finite-element/frame context into
the OpenPipeStress architecture. NASA NTRS material for NASTRAN and MIT OCW
finite-element course material support the general idea of finite-element
structural modeling and finite-element procedures. OpenPipeStress-specific
degree-of-freedom ordering, element contracts, diagnostics, result envelopes,
and source controls remain governed by the project documents named in the
source notes.

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

The source support for this section is conceptual. It relies on public statics
and finite-element sources for ordinary mechanics vocabulary and structural
modeling context, while project governance defines which OpenPipeStress
results, diagnostics, and provenance records are exposed to users.

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

Open educational and public-domain mechanics sources may support the concepts
behind a fixture family, but they do not authorize copying textbook examples,
source formulas, tables, or values into public validation files. Public fixtures
still need original, invented, public-domain, or otherwise permissively cleared
provenance.

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

The public sources cited here support local FEA only as a broad finite-element
analysis context. They do not provide OpenPipeStress acceptance criteria,
mesh-quality policy, local-model validation, external-solver endorsement, or
professional approval.

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

Source inventory status for this draft:

| Source ID | Status | Locator and source section | License / redistribution status | Claim scope | Review notes |
|---|---|---|---|---|---|
| `SRC-OPS-GOV` | `ACCEPTED` | `docs/DIRECTIVE.md`; `docs/CONTRACT.md`; `docs/TYPES.md`; `docs/SPEC.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/PROFESSIONAL_BOUNDARY.md`; `docs/VALIDATION_STRATEGY.md`; `docs/architecture/code_neutral_analysis_boundary.md`; `docs/architecture/analysis_status_semantics.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7`. | Project-authored repository governance and architecture basis. | OpenPipeStress scope, authority boundaries, source/provenance controls, protected-content exclusions, centerline/frame architecture, rule-pack separation, validation family separation, and professional-boundary wording. | Accepted for project-specific claims only. Does not supply external piping-history evidence. |
| `SRC-NASA-NTRS-NASTRAN-USER` | `ACCEPTED` | [NASA NTRS, "The NASTRAN user's manual," Document ID `19840013831`](https://ntrs.nasa.gov/citations/19840013831), metadata/abstract. | NTRS marks distribution as public and copyright as "Work of the US Gov. Public Use Permitted." | General finite-element structural-analysis context: structural models can be idealized with connected finite elements/grid points, with loads applied and displacements calculated. | Conceptual support only. No cards, procedures, formulas, examples, tables, or NASTRAN-specific implementation details are copied or adopted. Not used as piping-code history. |
| `SRC-MIT-OCW-FEA` | `ACCEPTED` | [MIT OCW, "Finite Element Procedures for Solids and Structures"](https://ocw.mit.edu/courses/res-2-002-finite-element-procedures-for-solids-and-structures-spring-2010/), course overview; [MIT OCW Privacy and Terms of Use](https://ocw.mit.edu/pages/privacy-and-terms-of-use/), Creative Commons license section. | MIT OCW states CC BY-NC-SA 4.0 for OCW materials unless otherwise marked. | General finite-element analysis context, assumptions, and linear/nonlinear procedure framing for engineering structures. | Citation support only. No course text, videos, images, formulas, examples, or third-party course media are reused. |
| `SRC-ENG-STATICS` | `ACCEPTED` | [Open Textbook Library, "Engineering Statics: Open and Interactive"](https://open.umn.edu/opentextbooks/textbooks/1047), book metadata and conditions of use. | Open Textbook Library lists CC BY-NC-SA conditions of use. | General statics vocabulary: forces, vectors, equilibrium, internal reactions, internal forces, and geometrical properties. | Citation support only. No examples, tables, steel-section properties, formulas, or values are copied. |
| `TBD-public-history` | `TBD` | Public/permissive source for historical development of piping flexibility analysis terminology. | `TBD`. | Historical lineage claims about classical piping flexibility practice. | Do not invent or overstate history. Current prose avoids detailed historical claims. |
| `TBD-open-frame-reference` | `TBD` | Public/permissive structural-analysis source for formula-level 3D frame element details if future theory prose adds formulas or derivations. | `TBD`. | Formula-level or derivation-level frame theory beyond the conceptual FEA/frame context cited above. | Current prose stays conceptual; no formulas or source examples are introduced. |
| `TBD-local-fea-reference` | `TBD` | Public/permissive source for local shell/solid handoff practice if future prose expands beyond project boundary language. | `TBD`. | Practice-specific guidance for local FEA handoff beyond the general FEA concept. | Current prose stays at boundary level and does not claim local FEA acceptance or professional adequacy. |

Rejected or excluded source classes for this draft:

- Protected standards text, figures, examples, code formulas, material
  allowables, SIF/flexibility tables, protected dimensional tables, and
  code-specific values.
- Proprietary commercial software examples, vendor benchmark data, private
  owner data, private rule-pack values, and uncleared manufacturer data.
- Any source whose license, redistribution status, or protected-content status
  is unclear enough that it cannot be cited in public documentation.
