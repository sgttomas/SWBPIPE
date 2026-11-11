---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-00-02
package_id: PKG-00
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-057, SOW-062]
package_objective_refs: [OBJ-013]
---

# Scope of Work — DEL-00-02 Repository and module boundary architecture

## Purpose and Objective Traceability

This deliverable is the layer map of the codebase and the rule for which way dependencies run. It says which layer each module belongs to and what that layer is responsible for, so that mechanics and invariant enforcement stay in the domain core, the GUI and the adapters stay outside it, and no module reaches around the unit, provenance and data-boundary checks. The result it contributes is that any deliverable can tell where its code goes and what that code may depend on, and that a dependency running the wrong way is caught when it is added.

It serves SOW-057 and SOW-062 under OBJ-013 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-00, row DEL-00-02, and architecture-basis row AB-00-02 in §8.1), with `docs/SPEC.md` §1 and §2 and `docs/PRD.md` §5.2 and §5.3. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The layer and module-ownership map, a document under `docs/architecture/`: every module of the repository assigned to one layer, with the layer's responsibility and the deliverables that own code in the module.
- **OUT-002** — The dependency rules, and the check that the repository's module graph obeys them.

## Deliverable Definition — Ontology

**Terms the architecture implements.**

- *Layer.* One of eleven: GUI, application services, domain core, solver, rules, reports, adapters, storage, schemas, validation and tests. Each has one responsibility, which the map states in full. The sources fix this much:
  - the domain core, with the solver, the rules and the reports, owns mechanics and invariant enforcement (`docs/SPEC.md` §1);
  - application services coordinate the GUI, the core, the solver, storage, reports and adapters;
  - the GUI sits outside the core and reaches it through application services;
  - adapters translate data in and out and cannot bypass the core's unit, provenance and data-boundary checks, or governance, validation and diagnostics (`docs/SPEC.md` §1; decomposition AB-00-02).
- *Module.* A crate, a package, or a top-level directory that holds code the product builds, runs or tests with. Each module has one layer, and the map lists every module.
- *Inward.* The direction of a permitted dependency: toward the domain contracts. The GUI, application services and adapters may depend on the core and its contracts. Nothing in the domain core, solver, rules or reports depends on the GUI or on an adapter.
- *Drift.* A choice made inside one module that contradicts the map or the direction: an outward dependency, or a module the map does not place.

**Boundary.** This deliverable defines the layers and the direction; it implements no module. Its own files are the map and the dependency check. A dependency that breaks the rule is repaired in the module that holds it, by the deliverable that owns that module.

- DEL-00-03 owns the shape of the application-service entry (commands, queries, jobs). This scope owns which module may depend on which.
- DEL-00-07 owns which seams are public and the controls applied to data that an adapter or plugin carries across one.
- DEL-00-05 owns what the GUI may hold and how it changes the model.
- DEL-00-01 records the stack the modules are written in; DEL-10-04 owns the build manifests' tooling.
- The content of each module belongs to the deliverable whose `code_paths` claim it.

**Not decided.** The lint or build tool that enforces the direction in each language.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The map states the responsibility of each of the eleven layers and assigns every module to exactly one (decomposition AB-00-02; `docs/SPEC.md` §1).
- **REQ-002** — Dependencies point inward toward the domain contracts, and none runs around the unit, provenance or data-boundary checks (decomposition AB-00-02).
- **REQ-003** — Adapters and plugins may translate data and may not bypass governance, validation or diagnostics (decomposition AB-00-02; `docs/SPEC.md` §1).
- **REQ-004** — The map describes the repository as it is. A layout question that has not been decided is stated as undecided, and the map invents no structure the tree does not have. Folder names may change without a change to a layer's responsibility.
- **REQ-005** — A check detects drift before it is integrated. This requirement is held (PDU-007): it is not treated as met until a formal review has disposed of whether the check and its evidence are sufficient to detect drift. A reviewer who did not write the map or the check may make that review; lifting the hold is the owner's.

### Acceptance criteria

- **AC-001** — The map assigns every crate, package and top-level source directory to exactly one of the eleven layers, states each layer's responsibility and names the deliverables that own code in each module. Every path it names exists and no module of the tree is missing from it. This carries commitment DEL-00-02:1.
- **AC-002** — In the repository's module graph (Cargo path dependencies and TypeScript and Python imports), every dependency between layers points inward as the map states. No domain-core, solver, rules or reports module depends on a GUI or adapter module, and GUI feature modules reach the core only through application-service modules.
- **AC-003** — Every adapter and plugin module is placed in the adapters layer. None carries its own unit, provenance or data-boundary validation in place of the core's, and none writes the model document, the project store or report output except through the module that applies those checks.
- **AC-004** — An undecided layout question appears in the map as one statement of what is undecided, and the map selects nothing that the owner or the owning deliverable has not chosen.
- **AC-005** — A change that adds an outward dependency, or adds a module the map does not place, fails a check that runs with the project's checks. A reviewer who did not write the map can place every existing module and a new one from it without asking. This carries commitment DEL-00-02:2.

## Production and Verification Method — Praxeology

**Method choices that define the architecture.**

- A layer is defined by its responsibility and not by a folder name.
- Dependencies are judged between modules, from the build manifests and the import statements.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-004, judgment by a reviewer who did not write the map: examines the map against the repository tree, the manifests and the deliverables' `code_paths`.
- **VER-002** — AC-002, AC-003 and AC-005, executable check: observes the Cargo path dependencies and the TypeScript and Python imports against the declared layers, and the adapter modules' writes to the model document, the store and report output; a seeded outward dependency and a seeded unplaced module must each fail it.
- **VER-003** — AC-005, judgment by a reviewer who did not write the map: examines whether the map alone is enough to place existing and new modules.

## Governing Values and Decisions — Axiology

- **AX-001** — *Inward only.* Better is fewer dependencies against the direction; the target is none, with no listed exceptions. This is never bought by moving a module into a layer whose responsibility it does not have.
- **AX-002** — *Checked, not remembered.* Better is a larger share of the direction rules enforced by the executable check and a smaller share left to review. The check is never weakened with an ignore list to let a change pass.
- **AX-003** — *The map follows responsibility.* Renaming or moving a folder is free. Changing what a layer is responsible for is an architecture decision and is recorded through DEL-00-01.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-057 SOW-062 OBJ-013 | REQ-001 REQ-004 | AC-001 | VER-001 | Decomposition AB-00-02 and `docs/SPEC.md` §1 name the layers; the comparison is against the repository tree and manifests |
| OUT-001 | SOW-057 OBJ-013 | REQ-004 | AC-004 | VER-001 | The reviewer's comparison of the map's layout statements with the repository tree and with the choices the owner or the owning deliverables have made; SOW-057 lets folder names change without a change of layer responsibility |
| OUT-002 | SOW-057 SOW-062 OBJ-013 | REQ-002 | AC-002 | VER-002 | Decomposition AB-00-02 states the direction; the comparison is against the manifests and imports |
| OUT-002 | SOW-062 OBJ-013 | REQ-003 | AC-003 | VER-002 | Decomposition AB-00-02 and `docs/SPEC.md` §1 state the rule; the comparison is each adapter and plugin module's placement and its writes to the model document, the store and report output |
| OUT-002 | SOW-057 OBJ-013 | REQ-005 | AC-005 | VER-002 VER-003 | Decomposition AB-00-02; the seeded violations state what must fail, and the reviewer did not write the map |
