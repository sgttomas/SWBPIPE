---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-01
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-005,SOW-035]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-01 3D frame stiffness kernel

## Purpose and Objective Traceability

This deliverable is the solver's kernel. It turns a piping model, represented as a 3D centerline frame with six degrees of freedom per node, into a global stiffness system, reduces it for restraints and prescribed displacements, and solves it. Every solve in the product goes through it, so the correctness, reproducibility and speed of every published number start here.

It serves SOW-005 and SOW-035 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-01), and PRD §12.1 and §12.2 with FR-SOL-001 and FR-SOL-002. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It reaches the release target through DEL-04-07, whose one route solves the reference system and passes its closed-form references (criteria 1 and 2), and through DEL-04-05, which measures the 2,000-node rack model against the stated times (criterion 9).

- **OUT-001** — A global 3D centerline/frame stiffness kernel for practical piping models, with six degrees of freedom (Ux, Uy, Uz, Rx, Ry, Rz) at every node, local-to-global transforms, global assembly, restraint and prescribed-displacement reduction, and a deterministic linear solve in a sparse and a dense mode.

## Deliverable Definition — Ontology

**Terms the kernel implements.**

- *Frame model.* Nodes on the pipe centerline joined by two-node elements. Each node has six degrees of freedom in the fixed public order `[ux, uy, uz, rx, ry, rz]`; an element's node `i` occupies indices 0 to 5 and node `j` indices 6 to 11. Consumers bind to this order through the kernel's mapping.
- *Element-to-global transform.* A validated right-handed local triad per element. The transform between local and global quantities is the triad's direction cosines.
- *Global stiffness.* The symmetric stiffness of the whole frame, assembled from its elements' stiffnesses in global coordinates.
- *Restraint and prescribed-displacement reduction.* Removal of restrained degrees of freedom and substitution of prescribed displacements, with their force adjustment, before the solve; reactions are recovered at the reduced degrees of freedom.
- *Solve modes.* The sparse mode is the default. The dense mode is selectable and is the sparse mode's parity path.

**Boundary.** Frame assembly and the linear solve are this deliverable's. DEL-04-07 composes the product solve and the nonlinear orchestration. DEL-04-04 keeps the active-set classifier. The kernel does not implement straight pipe element mechanics (DEL-04-02), support families (DEL-04-03, DEL-04-04), load cases (DEL-05-01, DEL-05-05), stress recovery (DEL-05-03), the diagnostics layer (DEL-04-06), rule-pack evaluation, GUI behaviour, reports or packaging. The connector and tie-rod elements are DEL-03-06's definitions and are examined under its criteria; their code is held in the kernel's crate.

**Not decided.** Which deliverable's criteria govern the following, all of which the kernel's code carries: the curved-bend element that places user-entered flexibility factors in the global stiffness (DEC-070) with its benchmark; the restraint basis for an authored skew direction; the solve integrity check with its established recovery and global end actions, and the readers' check of global end rows; the enclosure of the normal-stress maximum along a realized arc (DEL-04-07); the exact accumulator that DEL-05-02's combinations use; the correctly rounded norms; the read-only factor and re-formation seams DEL-04-09 uses; and the retained-precision lanes and exact boundary DEL-04-08 reads. This scope states no criterion for them.

## Completion and Reliance Basis — Epistemology

A kernel result is a numerical statement about the discrete linear model it was given. It bounds no modelling, input or physical error.

### Requirements

- **REQ-001** — The global analysis system is a 3D centerline/frame model (SOW-005; PRD §12.1; `docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-002** — Every active node has the six degrees of freedom Ux, Uy, Uz, Rx, Ry and Rz, in the fixed order (SOW-005; PRD FR-SOL-001).
- **REQ-003** — The kernel assembles the global stiffness of the frame system (SOW-005; PRD §12.2).
- **REQ-004** — The kernel transforms between local and global frame representations by the direction cosines of validated local axes (SOW-005; PRD §12.2, FR-SOL-002).
- **REQ-005** — The kernel applies the restraints and prescribed displacements it is given. Which restraints a model may carry, and what they mean, is DEL-04-03's and DEL-04-04's (PRD §12.2).
- **REQ-006** — The kernel solves through a sparse interface built for sparse numerical performance and reproducible results on practical piping models (SOW-035). No kernel-level time or memory limit is decided: the release limits are exit criterion 9's, measured by DEL-04-05.
- **REQ-007** — Kernel inputs and outputs are unit-aware and dimensionally checked (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-008** — A missing or invalid solve-required value is an explicit finding. The kernel supplies no default (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-009** — Kernel outputs are mechanics only. They carry no rule-pack acceptability, compliance or approval meaning (`docs/CONTRACT.md` OPS-K-MECH-2; PRD §21.2).
- **REQ-010** — Kernel output is deterministic: a fixed model, solver version and settings give bit-identical results on every run, so every kernel check is deterministic (`docs/CONTRACT.md` OPS-K-SOLVER-1; SOW-035; PRD §12.2).
- **REQ-011** — A kernel finding that crosses a service boundary keeps its code, class, severity, source, affected object, message, remediation and provenance (the decomposition's architecture basis for DEL-00-03 and DEL-00-06).
- **REQ-012** — Kernel sources, tests and fixtures contain no protected standards text, tables, figures, formulas or examples and no proprietary data (`docs/CONTRACT.md` OPS-K-IP-1; PRD §20.1).
- **REQ-013** — Each sparse factorization reports its ordering, stored profile size, pivot extremes and count of non-positive pivots, which are what singularity and ill-conditioning are detected from (DEC-023; PRD §12.2).

### Acceptance criteria

- **AC-001** — Every active node carries the six degrees of freedom Ux to Rz, node and element indices follow the fixed order, and a consumer that binds through the kernel's mapping reads the same indices.
- **AC-002** — Element and global stiffness of beams and frames, including members rotated to any orientation, and the displacements solved from them agree with closed-form frame mechanics at the analytic tolerance (1e-9 relative, with its governed absolute floor near zero): cantilever under tip force and tip moment, inclined member and torsion-only member.
- **AC-003** — Restraint and prescribed-displacement reduction reproduces closed-form beam cases: restrained degrees of freedom are exactly zero, each prescribed displacement is reproduced at its degree of freedom, including an imposed anchor displacement, and reactions balance the applied loads.
- **AC-004** — The sparse and dense modes agree at the analytic tolerance on the references of AC-002 and AC-003, and repeated runs of the same model, solver version and settings are bit-identical in each mode.
- **AC-005** — A missing, non-finite, degenerate or out-of-range solve-required input, and a singular system, are each refused with an explicit finding that names the object or degree of freedom. No solve proceeds on a substituted value.
- **AC-006** — No kernel output, message or identifier shown to a user states or implies rule acceptability, code compliance, certification or approval.
- **AC-007** — Kernel sources, tests and fixtures contain nothing that REQ-012 excludes.
- **AC-008** — A kernel finding raised in a desktop solve reaches the desktop's result with all eight fields of REQ-011 intact.
- **AC-009** — A quantity whose dimension does not match its use is rejected with a finding that names it. The kernel converts and assumes no value.
- **AC-010** — A straight member the model selects as Timoshenko has the stiffness of the exact static two-node Timoshenko element with shear area κ_E·A, and reduces to the Euler–Bernoulli element as the shear term vanishes.
- **AC-011** — Each sparse factorization reports its ordering, stored profile size, pivot extremes and non-positive pivot count; on systems constructed with a known inertia, the non-positive pivot count equals the number of non-positive eigenvalues.

## Production and Verification Method — Praxeology

**Method choices that define the kernel.**

- *Sparse strategy.* An in-repository sparse direct solver: profile-reducing ordering, skyline (profile) storage of the symmetric stiffness and LDLᵀ factorization, single-threaded in a fixed operation order, with no numerical dependency added, and each factorization reports its ordering, stored profile size, pivot extremes and count of non-positive pivots (DEC-023). The dense mode is kept as its parity path.
- *Default mode.* Sparse is the default, dense is selectable, and a fall-back from sparse to dense is stated in the result (DEC-053).
- *Transforms.* Direction cosines from validated local axes.
- *Formulation.* The frame is the two-node frame. Straight members are Euler–Bernoulli or, where the model document selects it, the exact static Timoshenko element with the energy-matched annular shear factor; realized arcs are Euler–Bernoulli. The shear factor's definition and the selection rule are DEL-05-03's.
- *Tolerances.* Numerical verification tolerances are held by reference class and quantity kind as relative and absolute pairs, seeded at 1e-9 relative for the analytic class. A fixture may only tighten a governed value (DEC-026).

**How each criterion is examined.**

- **VER-001** — AC-001: an executable check observes the degree-of-freedom indices and element maps against the fixed order.
- **VER-002** — AC-002, AC-003 and AC-010: kernel tests compare stiffness, displacements and reactions with closed-form beam and frame values produced by a reference generator under `validation/references/` that imports no product code; for AC-010 the reference is the exact Timoshenko element in closed form.
- **VER-003** — AC-004 and AC-011: an executable check observes sparse and dense results side by side on those references and compares repeated runs bit for bit; for AC-011 it observes each factorization's reported ordering, profile size, pivot extremes and non-positive pivot count on symmetric systems constructed with a known inertia.
- **VER-004** — AC-005 and AC-009: negative tests observe each invalid, missing, singular or mis-dimensioned input refused with its named finding and no substituted value.
- **VER-005** — AC-006 and AC-007: a reviewer who did not write the code reads kernel outputs and texts against `docs/claims_registry.md`, and the protected-content scan runs over the kernel's sources, tests and fixtures.
- **VER-006** — AC-008: an executable check follows a kernel finding through a desktop solve to the displayed diagnostic, field by field.

## Governing Values and Decisions — Axiology

- **AX-001** — *Correct before fast.* Better is a smaller deviation from the independent closed-form references. This is never bought by loosening a tolerance or its floor, or by altering a frozen reference.
- **AX-002** — *Fast on practical models.* Better is less time and memory on practical piping models, measured by DEL-04-05. This is never bought by giving up determinism, the explicit refusal of missing or invalid input, or the separation of mechanics from rule-pack and compliance meaning.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-005 OBJ-003 | REQ-001 REQ-002 | AC-001 | VER-001 | The criterion states the expected indices (PRD FR-SOL-001) |
| OUT-001 | SOW-005 OBJ-003 | REQ-003 REQ-004 | AC-002 | VER-002 | Outside source: closed-form beam and frame mechanics, through a reference generator that imports no product code. The tolerance is an owner ruling (DEC-026) |
| OUT-001 | SOW-005 OBJ-003 | REQ-005 | AC-003 | VER-002 | Outside source: closed-form beam cases and equilibrium, through a reference generator that imports no product code. The tolerance is an owner ruling (DEC-026) |
| OUT-001 | SOW-035 OBJ-003 | REQ-006 REQ-010 | AC-004 | VER-003 | The product's own other mode and its own earlier output, which only guard against change. Correctness rests on AC-002 and AC-003; on parity alone there are no independent grounds |
| OUT-001 | SOW-005 OBJ-003 | REQ-008 | AC-005 | VER-004 | Owner ruling: `docs/CONTRACT.md` OPS-K-DATA-2 and PRD §5.5 state the expected refusal |
| OUT-001 | SOW-005 OBJ-003 | REQ-009 | AC-006 | VER-005 | Owner ruling: PRD §21.2 and `docs/claims_registry.md` |
| OUT-001 | SOW-005 OBJ-003 | REQ-012 | AC-007 | VER-005 | Protected-content scan and owner review under `docs/IP_AND_DATA_BOUNDARY.md` |
| OUT-001 | SOW-005 OBJ-003 | REQ-011 | AC-008 | VER-006 | The criterion states the expected fields (decomposition architecture basis) |
| OUT-001 | SOW-005 OBJ-003 | REQ-007 | AC-009 | VER-004 | Owner ruling: `docs/CONTRACT.md` OPS-K-UNIT-1 and PRD §5.6 state the expected refusal |
| OUT-001 | SOW-005 OBJ-003 | REQ-003 | AC-010 | VER-002 | Outside source: the exact static Timoshenko element in closed form, through a reference that imports no product code. That straight members may be Timoshenko is an owner ruling |
| OUT-001 | SOW-035 OBJ-003 | REQ-013 | AC-011 | VER-003 | Outside source: the inertia of each constructed system is known from its construction (Sylvester's law of inertia), and the ordering and profile size follow from its sparsity pattern. That the factorization reports them is an owner ruling (DEC-023) |
