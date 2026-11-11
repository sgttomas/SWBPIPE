---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-07
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-005,SOW-011,SOW-012,SOW-052,SOW-053]
package_objective_refs: [OBJ-003,OBJ-012]
---

# Scope of Work — DEL-04-07 Product solve integration and nonlinear orchestration

## Purpose and Objective Traceability

This deliverable is the product's solve: the one entry that takes a model document and returns published mechanics results. It reads the model, brings every quantity to consistent units, maps pipes, components, supports and loads onto the solver's elements and load terms, runs each load case as a linear solve or as the nonlinear iteration, and publishes displacements, member actions and support loads with their units, diagnostics and the identity of the input they came from. The desktop's Run command and the headless runner both call it. It adds no mechanics of its own: its work is that the primitives are composed correctly, that nothing is silently dropped or assumed on the way, and that one route serves every model.

It serves SOW-005, SOW-011, SOW-012, SOW-052 and SOW-053 under OBJ-003 and OBJ-012, as their product integration only (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-07), and PRD §12.1, §12.2 and §12.8 with FR-SOL-006 and FR-SOL-014. In `docs/RELEASE_TARGET_v1.0.md` it owns "one route" of exit criterion 1: the invented reference system solves on the exact pressure route in the desktop with no admitted family refused. It carries the owner's rulings that the exact route is extended and the pressure-free route gets no pressure, and that the Bourdon effect is a Should option, off by default and clearly labelled.

- **OUT-001** — The product solve entry: composition of the model, unit, boundary, section, load and kernel contracts into one solve entry for the desktop and the headless runner, in both solver modes and on both routes, with the published mechanics result and its identity.
- **OUT-002** — The assembled nonlinear iteration.
- **OUT-003** — The product's section and mass adapter.
- **OUT-004** — Diagnostics, units and provenance carried through composition to the result.

## Deliverable Definition — Ontology

**Terms the solve implements.**

- *Solve request.* A model document with its materials and the cases to solve, in one solver mode. A result binds the exact request it was computed from.
- *Routes.* A document without a pressure contract takes the pressure-free route (PF) and carries no pressure. A document with the pressure contract `3.0.0/exact_pressure_v3` takes the exact route (EX). There is one current pressure contract; any other is refused by name, a zero-pressure document included.
- *Pressure on EX.* Pressure is authored in regions. A region is one chain of members between two terminals, each terminal either transferring its end load to the pipe wall or separately supported or compensated. Pressure enters a case as loads only: the Poisson shortening of the wall, the terminal end loads, and the remainders at bends and joints. A node where terminals of two or more regions meet, each transferring to the wall, is a pressure junction; its end loads sum to the fluid force on the fitting, −Σ p·A·s over the incident members, whatever the fitting's angles or bores.
- *What pressure does not include.* Steady-flow and transient fluid loads, pressure stiffening of flexibility and stress-intensification factors, and ovalization are excluded and disclosed on results. Users enter their own factors.
- *Action.* What a load case applies: a primitive load, an equivalent-static load, a non-empty pressure-region list, a boundary motion, a member state of non-zero free strain, or a preload present in every case (a constant-effort support, a tie rod with a non-zero reference extension, or a connector with a non-zero reference load).
- *Nonlinear iteration.* For a model with nonlinear supports: assemble, solve, classify the supports, update the active set, and repeat until the classifier reports convergence or the iteration cap is reached. A sliding friction support's bounded force is applied in the system from the previous iterate's normal reaction.
- *Memory budget.* One solve may use at most a per-invocation budget: 12 GiB on the 16 GiB base machine, with larger qualified budgets on larger machines. All solves in the desktop share one pool, nominally 75 % of physical memory. A larger pool needs the user's explicit acknowledgement and never exceeds physical memory.
- *Section and mass adapter.* The product's own computation of a pipe's metal area, second moments, torsion constant and mass per length from the user's outside diameter, wall, mill tolerance, corrosion allowance and densities.

**Boundary.** Mathematical primitives, convergence policy and engineering acceptance stay with their owners; a product caller or a file's location does not move them.

- DEL-04-01: frame assembly and linear solving. Its code also carries the element families and the solve integrity check this deliverable composes (see `deliverable.yaml`).
- DEL-04-02: the straight pipe element. DEL-04-03: linear supports and imposed movement. DEL-04-04: the per-iteration classification and the friction law.
- DEL-04-06: what a diagnostic says. DEL-05-04: analysis status semantics.
- DEL-05-01 and DEL-05-05: load values and member loads. DEL-05-02: combinations. DEL-05-03: stress recovery and the two section bases. DEL-03-08: the section and mass calculation this adapter is held to.
- DEL-03-03 to DEL-03-06: component definitions. DEL-02-01: the model document. DEL-02-02: units.
- DEL-04-08: retained-precision publication. DEL-04-09, DEL-04-10 and DEL-04-11: modal, response-spectrum and hanger-sizing entries, which sit beside this one and use its built model.
- DEL-07-07: the desktop's solve job, which calls this entry in a worker.

## Completion and Reliance Basis — Epistemology

A published result is a numerical statement about the discrete model in the request. Agreement between entries or solver modes shows consistent composition; it is not numerical validation.

### Requirements

- **CLM-001** — *One entry.* The accepted model, unit, boundary, section, load and kernel contracts are composed into solve entries for the desktop and the headless runner. A result preserves the identity of its input, and every missing or unsupported input is an explicit, named diagnostic. Every support type and load type the model admits (the supports, springs and imposed displacements of SOW-011, the nonlinear supports of SOW-012, and the concentrated forces, concentrated moments and distributed loads of SOW-052) is applied on both routes or refused by name, never dropped silently. The same model, solver version and settings give byte-identical results and diagnostics through the desktop and the headless entry (PRD §12.2, §12.8).
- **CLM-002** — *Nonlinear iteration.* The assembled nonlinear iteration runs with frame assembly and linear solving left to DEL-04-01 and per-iteration classification left to DEL-04-04, under the governed convergence tolerances and iteration caps, and reports non-convergence (DEC-044, DEC-046).
- **CLM-003** — *Section and mass adapter.* The product's section and mass conversion is held to DEL-03-08's contracts, with the effect of corrosion allowance and mill tolerance assessed explicitly, and an unsupported section input refused or disclosed.
- **CLM-004** — *Nothing lost in composition.* Typed diagnostics, units and provenance are preserved across composition and into the result. A singular, ill-conditioned or non-converged solve publishes the corresponding diagnostic and no unqualified result (SOW-053). A constant-effort support that a solve consumed publishes the force the solve applied, on its positive global degree of freedom, with zero in the other components. A constant-effort support the solve does not consume applies and publishes no force and carries a named warning stating the unmet condition.
- **REQ-005** — *One route for every model.* The exact route admits every family the pressure-free route admits, so that a model with pressure needs no other route: straight pipe, realized bends, tees with user modifiers, concentric and eccentric reducers, valves and flanges, expansion joints with and without tie rods, linear restraints and springs, variable springs, constant-effort supports, gap, lift-off and friction supports, and imposed displacements. The pressure-free route gets no pressure. The exact route refuses no family the pressure-free route admits; each refusal that remains is named and listed as a release limitation (release target exit criterion 3).
- **REQ-006** — *Pressure boundary.* Only the current pressure contract is solved. On the exact route only a realized bend carrying the user's flexibility factor carries pressure; a geometry-only bend is refused there with a message to realize it. A terminal that is separately supported or compensated cannot be part of a pressure junction and is refused by name. Unequal pressures at a junction, and a pipe at a junction outside every region, are admitted as a closure at the fitting, each with its own warning. The exclusions above are stated on the results they affect.
- **REQ-007** — *No silent zero.* A load case whose stored loads are not applied, and which applies no other action, is refused by name. A model in which no case applies an action is refused by name. An empty case among loaded cases is published with a named warning saying that its result is all zero. A reference state of a load-state document is published as zero without a finding. No case's all-zero result is published without one of these.
- **REQ-008** — *Memory budget.* A solve is admitted only within the per-invocation budget and the shared pool, and a solve that would exceed them is refused by name instead of run. A larger budget admits no larger retained-precision domain (DEL-04-08/REQ-004).
- **REQ-009** — *Bourdon effect (Should).* On the exact route a model may select the Bourdon effect, the opening of a realized bend under pressure. It is off by default and clearly labelled where it is set and on results computed with it. With it off, a result discloses that Bourdon opening is not modelled.

**Undecided.** The refusal of a compensated terminal at a junction is a standing refusal of a required capability. It is listed as LIM-002 in `docs/RELEASE_LIMITATIONS_v1.0.md`; whether it is acceptable is the owner's decision.

### Acceptance criteria

- **AC-001** — The same request through the desktop entry and the headless entry gives byte-identical published results and diagnostics in each solver mode. On the benchmark models of PRD §22.2 those results agree with their independent references at the analytic tolerance. An incomplete, invalid or unsupported model is refused on every entry with the same named diagnostic and affected object, and no entry reaches the kernel past a primitive's validation.
- **AC-002** — On models with gap, one-way, lift-off and friction supports, on both routes, the iteration ends in the support states of the independent references, with displacements and reactions within the nonlinear tolerance of their class. Every exit without convergence publishes a non-convergence failure and no result marked solved. The governed tolerances and caps are used unchanged.
- **AC-003** — For annular sections over the range of diameter, wall, mill tolerance and corrosion allowance, the adapter's area, second moments, torsion constant and mass per length equal the closed-form annulus values and DEL-03-08's calculator for the same inputs. Any remaining difference between the adapter and the calculator is stated, with its size, not declared equivalent. A section input the adapter does not support is refused or disclosed.
- **AC-004** — Each diagnostic a primitive raises reaches the result with its code, severity, source and affected references unchanged, in both solver modes and on both routes. A singular, ill-conditioned or non-converged solve publishes its diagnostic and no result marked solved. Every published quantity carries its unit. A result names the request it was computed from, and a consumed constant-effort support's published action equals the force the solve applied. A constant-effort support the solve does not consume applies and publishes no force and carries the named warning with its unmet condition.
- **AC-005** — The invented reference system solves on the exact route in the desktop with no admitted family refused, and each family of REQ-005 reproduces its independent reference on the exact route at the tolerance of its reference class (the analytic tolerance for linear families, the nonlinear class tolerance for nonlinear supports).
- **AC-006** — Each refusal and warning of REQ-006 is raised with its code and names the case, node, region or member. A junction's resultant equals −Σ p·A·s for tees, reducers and bore changes, and equals the fitting load for a closure between two pressures. Each exclusion appears on the results it affects.
- **AC-007** — On both routes, a load-state case that stores loads and applies none of them is refused by name. A model whose cases all apply nothing is refused by name. An empty case among loaded cases carries the no-action warning naming the case. No other all-zero case result is published.
- **AC-008** — A solve whose priced requirement exceeds the per-invocation budget, or for which the pool has no room, is refused by name before it runs. A completed solve's peak footprint is within the budget; the pool defaults to 75 % of physical memory, a larger pool is admitted only with the user's acknowledgement, and no pool exceeds physical memory or bypasses a headroom refusal.
- **AC-009** — With the Bourdon option off, results are unchanged and state the exclusion. With it on, a pressurized bend's end rotation matches an independent reference, and the option is labelled on the input and on the results.

## Production and Verification Method — Praxeology

**Method choices that define the solve.**

- The nonlinear method is an active-set fixed-point iteration around assemble, solve, classify and update. Relaxation and anti-chatter measures are implementation choices inside the iteration; tolerances and caps are not (DEC-044, DEC-046).
- Convergence uses the governed per-class record of relative tolerance, absolute floor and iteration cap. An unmeasured entry keeps the tolerance-policy-undecided warning active, and a fixture override may only tighten (DEC-046).
- Sparse is the default solver mode and dense scrutiny is selectable. Both go through the same composition.
- Pressure enters as case-constant loads, so the nonlinear iteration takes it exactly as it takes thermal loads, and nothing in the iteration depends on displacement through pressure.
- A pressure junction is recognised from the regions that meet at a node. The model document has no junction token.
- A load term the integrity check cannot re-form is not published as established: the case fails closed with a named reason.
- Derived analyses (modal, response spectrum, hanger sizing) read the built model; they do not re-implement composition.

**How each criterion is examined.**

- **VER-001** — Comparison with named references, with an executable check: the same accepted and invalid models are traced through each entry; the check observes identical results, input identity and refusals, and no bypass of primitive validation. The references are `validation/references/w9_benchmarks` and the other benchmark packages of PRD §22.2, generated without product code.
- **VER-002** — Comparison with named references, with an executable check: linear and nonlinear switching, converged and non-converged cases and the classifier hand-offs are exercised against `validation/references/rt_a3_nonlinear`, `rt_a3_s6`, `kd5_nonlinear` and `w16_nonlinear_motion`; the check observes the non-convergence failure on every non-converged exit and the governed cap and tolerances.
- **VER-003** — Comparison with named references: the adapter against closed-form annulus geometry and against DEL-03-08's calculator, for geometry, corrosion allowance, mill tolerance, density and mass, with any discrepancy recorded.
- **VER-004** — Executable check: observes valid and invalid inputs and their diagnostics through each mode to the result, the unit on every published quantity, and the published constant-effort action against `validation/references/rt_a3_constant_effort`.
- **VER-005** — Comparison with named references: the reference system (`validation/references/reference_system`) is solved in the desktop on the exact route, and each family of REQ-005 is solved on the exact route against an independent reference generated without product code, including `validation/references/rt_a3_tee`, `rt_a3_reducer`, `rt_a3_reducer_2b`, `rt_a3_constant_effort`, `rt_a3_nonlinear`, `rt_a3_s6`, `t4_i7`, `t4_i8`, `t4_u4a`, `t4_u4b`, `t4_u5a` and `t4_u5b`, and a reference of the same kind for valves and flanges, the eccentric reducer, the variable spring and an imposed displacement.
- **VER-006** — Executable check: observes each pressure refusal and warning with its named objects, each no-action refusal and warning with a reference state published beside a loaded case, each memory refusal before a solve starts, the pool's default, acknowledged and refused sizes, and a completed solve's peak footprint.
- **VER-007** — Comparison with a named reference: a closed-form end rotation of a pressurized bend, derived without product code, for the Bourdon option.

## Governing Values and Decisions — Axiology

- **AX-001** — *One route.* Better is fewer families refused on the exact route; the target is none that the reference system needs. Admission is never bought by solving a family approximately in place of its definition, or by putting pressure on the pressure-free route.
- **AX-002** — *A silent wrong result is the worst failure.* Better is fewer paths on which a load, a member or a diagnostic can be dropped without a named refusal. Convenience, speed and a shorter result never justify an unannounced zero or default.
- **AX-003** — *Compose, do not re-derive.* Better is less mechanics in this deliverable. A formula duplicated here from a primitive is a defect, even where it is faster.
- **AX-004** — *Within the machine.* Better is a larger model solved within the budget. This is never bought by exceeding the budget unannounced or by weakening a numerical check.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 OBJ-012 | CLM-001 | AC-001 | VER-001 | Outside source: the PRD §22.2 benchmark references, generated without product code. Agreement between entries and modes is the product's own output and only guards against change; on that agreement alone there are no independent grounds |
| OUT-002 | OBJ-003 OBJ-012 | CLM-002 | AC-002 | VER-002 | Outside source: `validation/references/rt_a3_nonlinear`, `rt_a3_s6`, `kd5_nonlinear`, `w16_nonlinear_motion`; owner rulings DEC-044 and DEC-046 for method and limits |
| OUT-003 | OBJ-003 OBJ-012 | CLM-003 | AC-003 | VER-003 | Outside source: closed-form annulus geometry. The comparison with DEL-03-08's calculator is between two product implementations and only guards against drift |
| OUT-004 | OBJ-003 OBJ-012 | CLM-004 | AC-004 | VER-004 | The criterion states the expected fields and units; diagnostics unchanged across routes compare the product with its own output and only guard against change; outside source `validation/references/rt_a3_constant_effort` for the published action |
| OUT-001 | OBJ-003 OBJ-012 | REQ-005 | AC-005 | VER-005 | Outside source: the family references named in VER-005, generated without product code; owner ruling for the reference system's content (exit criterion 1) |
| OUT-001 | OBJ-003 OBJ-012 | REQ-006 | AC-006 | VER-005 VER-006 | Outside source: statics of the fluid force for the junction resultant (`validation/references/rt_a3_tee`, `rt_a3_reducer`, `rt_a3_reducer_2b`, generated without product code); the refusals and warnings are those REQ-006 names |
| OUT-001 | OBJ-003 OBJ-012 | REQ-007 | AC-007 | VER-006 | Owner ruling: PRD §5.5; the criterion states the expected refusals and warning |
| OUT-001 | OBJ-003 OBJ-012 | REQ-008 | AC-008 | VER-006 | Owner ruling: the 12 GiB per-invocation budget on the 16 GiB base machine, the nominal 75 % pool and the acknowledged larger pool |
| OUT-001 | OBJ-003 OBJ-012 | REQ-009 | AC-009 | VER-007 | Outside source: a closed form for the pressurized bend, to be derived without product code; owner ruling for off by default and the label |
