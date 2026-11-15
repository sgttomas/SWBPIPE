---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-01
package_id: PKG-05
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-013]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-05-01 Primitive load case engine

## Purpose and Objective Traceability

This deliverable is what a load case is made of. It defines the eight kinds of primitive load an engineer applies to a piping model (weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic and occasional) and turns each load of a case into what the solver consumes: forces and moments at nodes, loads along members, free strain in members and prescribed movement at supports. It also forms the loads that follow from the model itself: self-weight from the pipes' mass and a stated gravity, static-equivalent seismic load from the same mass and the user's g-factors, wind load from the user's pressure and exposed spans, and the temperature changes and added weights a case takes from its operating conditions. It defines the hydrostatic test case. Every value is the user's. Nothing is defaulted, and no code combination, factor or coefficient is supplied.

It serves SOW-013 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-05, row DEL-05-01: primitive categories, code combinations remain external), and PRD §12.5 and §12.6 with FR-SOL-006. In `docs/RELEASE_TARGET_v1.0.md`, exit criterion 1 names it with DEL-05-02 for loads, cases and combinations: on the invented reference system the sustained, operating, hydrotest, wind, three-axis static seismic and seismic anchor movement cases solve on the exact route in one model, with no admitted family refused. Under criterion 2 each of those cases passes closed-form or hand-calculation references made by an independent generator. It carries two requirements of the product: the hydrostatic test is a flagged test condition with test pressure, test contents density and spring supports held, marked as a test case in results and reports; and static-equivalent seismic and wind loads are generated from the user's inputs and the model's own mass and geometry only.

- **OUT-001** — The primitive load case engine: the load categories and records, the preparation of each load into solver contributions with its findings, static-equivalent seismic and wind generation, operating-condition resolution, and the hydrostatic test condition.
- **OUT-002** — Self-weight generation: the weight loads of selected pipes and components from their mass and an explicit gravity, as a plan of structured operations with its generation evidence.

## Deliverable Definition — Ontology

**Terms the engine implements.**

- *Primitive load.* One load of a load case: an identity, a category, a target (a node, a pipe, or a support with a degree of freedom), a direction, a dimension, a magnitude with its unit, for a member load an optional span given as two fractions of the member's length, and its provenance.
- *Category.* One of weight, pressure, thermal, imposed displacement, hydrotest, wind, seismic and occasional. A category names what the load is for the engineer; it selects no factor and no rule.
- *Load case.* A named set of primitive loads, with an optional static-equivalent generation record, an optional use of operating conditions and an optional test condition. A case is solved directly; combining solved cases is DEL-05-02's.
- *Contribution.* What a load gives the solve: a force or moment on a degree of freedom of a node; a uniform load per length on a member, over its whole span or the stated part; a free strain α·ΔT on a member; or a prescribed displacement or rotation at a support's degree of freedom.
- *Static-equivalent generation.* Loads formed for a case from the user's inputs. Seismic: on every member, a load per length equal to the user's g-factor for each global axis, times gravity, times the member's mass per length. Wind: on each span the user marks, over the fraction marked, a load per length equal to the user's pressure times the user's shape factor times the projected diameter (outside diameter plus twice the insulation thickness), in the user's global direction.
- *Mass basis.* The mass per length a generated load uses is the route's own: mass per length as DEL-03-08 defines it, with each part taken on the section DEL-05-03's section bases give it on that route. The basis is named `source_od_effective_wall_areas/v1` on the pressure-free route and `source_od_nominal_wall_pressure_bore_areas/v1` on the exact route. One model can therefore carry different weight and seismic loads on the two routes, and each generated load's evidence names its basis.
- *Self-weight plan.* A proposal, never an application: for the pipes the user selects and a gravity the user states (magnitude, unit and global axis), the operations that would create a weight case. A pipe's load is its mass per length times the gravity (`pipe_mass_per_length_times_explicit_axis_acceleration/v2`). A pipe that a rigid-family component maps is given the component's own weight instead: the force W along gravity at the component's node and the moments of that force about the node from the centre-of-gravity vector (`component_weight_at_center_of_gravity_global_offset/v1`).
- *Operating-condition use.* A case may take its temperature, pressure, wind exposure, added weight or contents density from the operating-condition set assigned to each pipe, in place of loads of its own. Resolution turns that use into ordinary loads of the case before anything else reads the case.
- *Test condition.* A case's declaration that it is a hydrostatic test, with its choice of held supports: `springs_and_constant_effort` or `none`. Both members are authored; neither defaults the other, and a case label alone has no mechanical effect.
- *Held support.* In a test case that holds supports, each spring, variable spring and constant-effort support is replaced on the degrees of freedom it acts on by a rigid restraint at zero displacement from the authored geometry. Neither its stiffness nor its force acts. Its reaction is the replacement restraint's, published under the support's own identity.
- *Finding.* A load or case the engine cannot prepare, reported with a code, the identity of the load or case, a message and a remediation.
- *Boundary records.* The engine's load-dimension names (`LoadDimension`, `CanonicalDimension`) and its storage-neutral boundary records (`BoundaryRecordRef`, and `BoundaryQuantityRecord` with its `QuantityUnitMetadata` and `CanonicalSchemaBinding`), which carry a quantity's dimension, unit, schema binding, payload references and provenance. The load-case algebra, the stress recovery and the user-load crate build their operands and records on them, and the user-load crate also takes the engine's prepared axial-effect contributions (`PrimitiveAxialEffectContribution`).

### CLM-035 — Self-weight generation boundary

This deliverable keeps the primitive-load and self-weight generation mathematics. The interaction with a self-weight plan in the desktop is DEL-07-03's; the mass contracts are DEL-03-08's; which section each part of the mass is taken on is DEL-05-03's; a rigid component's weight and centre of gravity are DEL-03-05's definition; product integration, with the product's computation of mass per length, is DEL-04-07's. A generated change reaches the model only through the structured-operation route of PKG-16, on the user's acceptance.

**Boundary.** The engine says what a case applies. The following belong elsewhere:

- the equivalent nodal loads of a member load and the end forces of a free strain: DEL-04-02;
- how a prescribed movement is solved, and the support families: DEL-04-03, DEL-04-04;
- what pressure contributes to a solve (pressure regions on the exact route; the pressure-free route carries none), each pipe's mass per length, and the composition of every load into the product solve on both routes: DEL-04-07;
- the replacement of held supports in the assembled system, and their reactions: DEL-04-07 (DEL-04-07/REQ-011);
- concentrated and distributed loads as user loads: DEL-05-05. Combinations of cases, including what an occasional or a test case means inside one: DEL-05-02;
- modal and response-spectrum analysis: DEL-04-09, DEL-04-10. Time-history and other dynamic loading are outside the product;
- the model-document members for loads, conditions and the test condition: DEL-02-01. Units and their conversion: DEL-02-02;
- showing a test case's mark in a report: DEL-08-01. Showing it in the results viewer is required, and no deliverable currently commits to providing it; REQ-014 gives the result record the mark the viewer would show.

Wind coefficients, exposure rules, seismic response parameters, code load cases and the mapping of an occasional event to a rule are the user's or the user's rule pack's.

**Not decided.**

- How a case's pressure is resolved from an operating condition. A case that asks for it is refused by name.
- Whether an authored temperature change in degF or degR is converted with correct rounding. It would move existing results.
- Which deliverable states the load and reference state of a document: a case's reference configuration, its fit and thermal strains by member, its per-support participation and its ledger of load sources. This scope states none of it.
- For a case that is not a hydrostatic test, whether and how its contents density from its operating condition replaces the section's contents density in its self-weight. A case that asks for it is refused by name.

**Withheld in v1.0.** A combination that includes a hydrostatic test case is withheld by name. v1.0 does not define what such a combination means, and exit criterion 1 asks for the test case, not for a combination of it. The refusal is a release limitation (exit criterion 3; LIM-003 in `docs/RELEASE_LIMITATIONS_v1.0.md`).

**Selected and sequenced.** The work that brings a test case's selected contents density into its self-weight is selected. SWBPIPE's Agent 0 released the hold that stood on it (2026-11-12). It follows the change that makes pressure elongation a user option on the exact route (DEL-04-07), because both change the same solve sources and every change to them is priced again (DEL-04-08/REQ-004). REQ-014 and AC-010 state the objective.

## Completion and Reliance Basis — Epistemology

A prepared load is a statement about the user's inputs. Its correctness is examined against closed forms and references made without product code. Nothing here says a load is the right one for a design.

### Requirements

- **REQ-001** — A load keeps its category from authoring to the published result: weight, pressure, thermal, imposed displacement, hydrotest, wind, seismic or occasional (SOW-013; PRD §12.5).
- **REQ-002** — Loads act on the 3D centerline model: on the six degrees of freedom of a node, along a member, or on a support's degree of freedom (PRD §12.1; `docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-003** — The engine holds no code-specific load combination, allowable, factor, coefficient, procedure generator or environmental default, and no output of it states or implies approval or compliance (the decomposition's PKG-05 exclusion; PRD §12.6, §21.2; `docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-IP-1, OPS-K-MECH-2).
- **REQ-004** — Every load magnitude carries its dimension and unit. The dimension vocabulary is the unit catalog's; a dimension that does not fit the load's category and target, or a name the catalog has retired, is rejected (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-005** — A load or case that lacks a value the solve needs (a target, a magnitude, a unit, a direction, a span, a property) or carries an invalid one is a finding. Nothing is defaulted, and a case with a blocking finding gives the solve no partial set of loads (PRD §5.5, §12.8; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-006** — Each finding becomes a diagnostic record with code, class, blocking severity, source, affected object, message, remediation and provenance (decomposition AB-00-06).
- **REQ-007** — A primitive load-case record binds one category to the model load case's identity, its provenance, the reference and hash of its payload, and its loads' identities in sorted order. A record that mixes categories is refused; combining categories is DEL-05-02's (SOW-013; SOW-014).
- **REQ-008** — *Static-equivalent generation.* For wind, seismic and occasional cases the engine accepts explicit equivalent loads, and generates seismic and wind loads as defined above from the user's inputs and the model's mass and geometry, on both routes. Each generated load names its inputs and its mass basis. A missing input blocks the case by name and generates nothing. No dynamic procedure, response parameter or coefficient is generated (PRD §12.5).
- **REQ-009** — Preparation is deterministic: the same case gives the same contributions and findings, in the same order, bit for bit, on every run (PRD §12.2; `docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-010** — Loads, load-case records and diagnostic records keep the provenance that separates a value the user or a lawful source supplied from one that is unresolved. An unresolved value is never read as a value (PRD §20.2; `docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-DATA-2).
- **REQ-011** — Where a member load is lumped to its end nodes, only an explicit load per length in a global translational direction is lumped, from the member's supplied span and end nodes. Pressure, thermal, rotational and acceleration inputs are never lumped.
- **REQ-012** — The thermal load of a straight member is the free strain α·ΔT, equal to the axial force E·A·α·ΔT when fully restrained, and a pressure end thrust is p·A_i, each from the member's supplied properties only. Neither forms a pressure stress or draws on a material or section catalogue (the decomposition's PKG-05 exclusion).
- **REQ-013** — *Operating-condition resolution.* A case's condition use resolves, before validation, into ordinary loads of the case: per pipe, a thermal temperature change equal to the exact difference of the condition temperature and the set's installation temperature, rounded once to binary64 kelvin; per pipe, the set's added weight, in addition to the case's own weight loads; and the pipes exposed to wind. The resolved loads come in a fixed order with deterministic identities. A case takes its temperature, pressure and wind exposure from conditions or from its own records, never both. A missing set, condition or value refuses the case by name. An authored temperature change keeps the unit catalog's interval conversion, so for degF and degR the two sources can differ in the last bit, and the user guide says so.
- **REQ-014** — *Hydrostatic test.* A case that declares the test condition is solved on the exact route as a test: its pressure is the case's own authored pressure, its contents density is that of the condition the case selects, and with `springs_and_constant_effort` its supports are held as defined above, while with `none` they act as in any case. The case's result record states that it is a hydrostatic test and which supports were held. The declaration, the pressure, the contents selection and the held choice survive preview, apply, undo, save and reopen unchanged. A test case on the pressure-free route is refused by name, never solved as an ordinary case (release target exit criterion 1).
- **REQ-015** — *Self-weight.* A self-weight plan is generated only from explicit inputs: the selected pipes, their sections and densities, and a gravity with magnitude, unit and axis. Its evidence names, per pipe, the method, the mass basis and the generated load per length, and per component the weight, node and centre of gravity. A component whose weight cannot be applied is named in a warning and its pipe keeps its distributed weight. Generating or inspecting a plan changes nothing in the model, and inspection of applied self-weight says whether each generated load still matches the model it came from (CLM-035).

### Acceptance criteria

- **AC-001** — Each of the eight categories can be authored on a case and is applied in the solve at its authored target, direction and magnitude on the exact route; on the pressure-free route only a load that carries pressure is refused, by name. In closed-form cases displacements and reactions agree at the analytic tolerance (1e-9 relative, with a stated absolute floor near zero): a nodal force and a nodal moment on a cantilever, a uniform load on a simply supported span, a temperature change in a fixed-fixed member, and an imposed displacement at a support. The published applied load keeps its category.
- **AC-002** — For selected pipes and an explicit gravity, each pipe's generated load per length equals its mass per length on the route's basis times the gravity, and a mapped component's generated force and moments equal W along gravity and r × F about its node, with that pipe's distributed weight omitted. The plan's evidence names each method, basis and input. A missing density, dimension or gravity blocks the plan by name. The model's canonical hash is the same before and after generating and inspecting.
- **AC-003** — A load with a missing target, magnitude, unit or direction, a non-finite value, a target the model does not have, a dimension that does not fit its category and target, a retired dimension name or an inverted span is reported as a finding with its code and the load's identity, and its case gives the solve no loads. No finding is replaced by a default.
- **AC-004** — A record that mixes categories, repeats a load identity, has no loads or lacks its case metadata is refused, and an accepted record lists its loads in sorted order. Each finding's diagnostic record carries all eight fields, and a record without provenance is refused.
- **AC-005** — Repeated preparation of the same case gives bit-identical contributions and findings in the same order.
- **AC-006** — The engine's sources, tests and fixtures contain no code-specific combination, allowable, factor, coefficient or environmental default and only invented values, and no message or record states or implies approval or compliance.
- **AC-007** — A generated seismic load equals g-factor × gravity × mass per length on every member for each axis, and a generated wind load equals pressure × shape factor × projected diameter on each marked span and fraction. A model so loaded reproduces its independent reference on both routes, including wind on part of a Timoshenko member. A missing input blocks the case with a named diagnostic and no load. On the reference system the wind case and the three seismic cases solve on the exact route in the same document as the other cases.
- **AC-008** — An explicit load per length in a global direction lumps to end-node forces in equilibrium with it; a pressure, thermal, rotational or acceleration input is refused for lumping. A member's thermal axial force is E·A·α·ΔT and a pressure thrust is p·A_i for the supplied values, and a missing or non-positive property is a finding.
- **AC-009** — A resolved temperature change has the bits of the exact difference rounded once to kelvin, for temperatures entered in K, degC, degF and degR. A document whose cases use conditions solves as the document with those loads authored. A case that names a missing set, condition or value, or that gives a quantity both ways, is refused by name. The one-bit difference between a typed and a resolved degF temperature change is reproduced and stated in the user guide.
- **AC-010** — A hydrostatic test case with supports held gives the closed-form reactions of the rigid replacement, with no spring or constant-effort force acting, and with `none` gives the reactions of the active supports. The case's authored pressure acts as the test pressure: on a straight test span the bore hoop stress equals the Lamé value for that pressure. Its weight includes contents at the selected test density, once. Its result states the test and the held choice. The declaration, pressure, contents selection and held choice are unchanged after preview, apply, undo, save and reopen, and after save and reopen the same input gives the same result. A test case on the pressure-free route is refused by name. On the reference system the hydrostatic test case solves on the exact route beside the static seismic cases.
- **AC-011** — On the reference system the sustained, operating and seismic anchor movement cases solve on the exact route in the same document as the other cases, and their reactions reproduce the system's independent weight references.

## Production and Verification Method — Praxeology

**Method choices that define the engine.**

- Loads are records, not expressions. A case's loads are a closed list, and a generated or resolved load becomes an ordinary record of the case with an identity that says how it was formed.
- Generation and resolution run before validation and before the solve, so the solve, its integrity checks and its published evidence see one kind of load.
- A held support's reference is the authored geometry, not a plant coordinate, a previously solved state or a datum inferred from the support's entered loads.
- Contents density is a mass input. It reaches a solve only through the one self-weight calculation, never as a second weight generator.
- A member load reaches the solver as the element's work-consistent equivalent nodal loads (DEL-04-02). Lumping is a separate, limited conversion.

**How each criterion is examined.**

- **VER-001** — Comparison with named references (closed-form beam mechanics and the hand calculations under `validation/hand_calcs/mechanics/`, with the benchmark cases of PRD §22.2): each category's load is applied in a closed-form case and displacements, reactions and the published applied load are compared; the thermal axial force and pressure thrust are compared with E·A·α·ΔT and p·A_i.
- **VER-002** — Comparison with named references, with an executable check: generated pipe and component weights are compared with annulus areas times densities times gravity and with W and r × F computed outside the product (`validation/references/w15_component_weight`, the weight totals of `validation/references/reference_system`); the check observes the plan's evidence, each refusal for a missing input, and the model's hash before and after.
- **VER-003** — Executable check: observes each finding of AC-003 and AC-004 with its code and the load's identity, the absence of loads for a blocked case, the sorted record, the diagnostic fields, and bit-identical output on repeated preparation.
- **VER-004** — Judgment by a named role (an independent reviewer who did not write the code): examines the engine's sources, tests, fixtures and messages against PRD §20.1, §21.2 and `docs/claims_registry.md`, with the protected-content scan's result for the same files.
- **VER-005** — Comparison with named references: seismic and wind generation against `validation/hand_calcs/mechanics/tp_pmm_p3_occloadgen_equivalent_static.md` and `tp_pmm_p3_subspan_wind_exposure.md`, and solved models against `validation/references/w16_exact_equivalent_static` and `validation/references/w16_timoshenko_partial_wind`, on both routes and in both solver modes; an executable check observes each missing-input refusal and the reference system's wind and seismic cases.
- **VER-006** — Comparison with a named reference (`validation/references/rt_g5_s2_conditions`, exact rational arithmetic): the bits of each resolved temperature change, the resolved loads and their order, each refusal, and the typed against resolved degF difference; an executable check observes that a document with condition use and its authored twin publish the same results.
- **VER-007** — Comparison with named references (closed forms: the reactions (3/8, 10/8, 3/8)·w·a of two equal spans with the middle support rigid; the spring-supported twin R = δ₀/(f + 1/k); the Lamé bore hoop stress for the test pressure), with an executable check that observes the test mark, the held choice, the single inclusion of contents weight, the result after save and reopen, the unchanged test condition after preview, apply and undo, the pressure-free refusal and the reference system's test case.
- **VER-008** — Comparison with a named reference (`validation/references/reference_system`, closed-form weight totals and moments whose generator imports no product code), with an executable check: observes that the reference system's sustained, operating and seismic anchor movement cases solve on the exact route in one document, and compares their reaction resultants with the reference.

## Governing Values and Decisions — Axiology

- **AX-001** — *Nothing assumed.* Better is fewer values the engine could supply by itself; the target is none. No magnitude, gravity, density, factor, coefficient or direction is supplied to make a case solvable.
- **AX-002** — *More of what users author is applied.* Better is fewer named refusals of load cases on the exact route, measured on the reference system, where the target is none. A refusal is never removed by applying a load approximately, on a basis its evidence does not name, or twice.
- **AX-003** — *Closer to the independent reference.* Better is a generated or resolved load that agrees with its closed form more tightly; a tolerance is only tightened. Agreement is never bought by tuning a load to a comparison tool's output.
- **AX-004** — *A load is mechanics.* Where a category or a generated load could be read as a code load case, the reading that claims less is the better one. Convenience for the user never justifies a built-in factor or combination.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-013 OBJ-003 | REQ-001 REQ-002 | AC-001 | VER-001 | Outside source: closed-form beam mechanics and the hand calculations under `validation/hand_calcs/mechanics/`, which import no product code |
| OUT-002 | SOW-013 OBJ-003 | CLM-035 REQ-015 | AC-002 | VER-002 | Outside source: annulus areas, densities and gravity computed outside the product (`validation/references/w15_component_weight`, `validation/references/reference_system`). The unchanged hash compares the model with itself and only guards against change |
| OUT-001 | SOW-013 OBJ-003 | REQ-004 REQ-005 REQ-010 | AC-003 | VER-003 | The criterion states the expected finding for each defect; PRD §5.5 and §5.6 say that none is defaulted |
| OUT-001 | SOW-013 OBJ-003 | REQ-006 REQ-007 | AC-004 | VER-003 | The criterion states the expected refusals and fields; decomposition AB-00-06 lists the diagnostic fields |
| OUT-001 | SOW-013 OBJ-003 | REQ-009 | AC-005 | VER-003 | The product's own earlier output: it guards against change and shows nothing about correctness |
| OUT-001 | SOW-013 OBJ-003 | REQ-003 | AC-006 | VER-004 | PRD §20.1 and §21.2 list what must not appear; the reviewer compares the files with them |
| OUT-001 | SOW-013 OBJ-003 | REQ-008 | AC-007 | VER-005 | Outside source: the two hand calculations and `validation/references/w16_exact_equivalent_static` and `w16_timoshenko_partial_wind`, generated without product code. Exit criterion 1 for the reference system |
| OUT-001 | SOW-013 OBJ-003 | REQ-011 REQ-012 | AC-008 | VER-001 | Outside source: statics of a uniform load on a span, and the closed forms E·A·α·ΔT and p·A_i |
| OUT-001 | SOW-013 OBJ-003 | REQ-013 | AC-009 | VER-006 | Outside source: `validation/references/rt_g5_s2_conditions`, exact rational arithmetic that reads no product code. The document against its authored twin is the product's own output and only guards the equivalence |
| OUT-001 | SOW-013 OBJ-003 | REQ-014 | AC-010 | VER-007 | Outside source: the closed forms named in VER-007. Exit criterion 1 for the reference system. The unchanged test condition after preview, apply, undo, save and reopen compares the product with its own input and only guards against change |
| OUT-001 | SOW-013 OBJ-003 | REQ-001 REQ-002 REQ-013 | AC-011 | VER-008 | Exit criterion 1 says which cases the reference system solves on the exact route in one document. Outside source: the reference system's independent references (`validation/references/reference_system`), generated without product code |
