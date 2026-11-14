---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-05
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-035]
package_objective_refs: [OBJ-003,OBJ-008]
---

# Scope of Work — DEL-04-05 Sparse solver performance harness

## Purpose and Objective Traceability

This deliverable measures the solver. On invented piping models of practical size it records how long a solve takes, how much memory it uses, where the time and memory go, and whether repeated runs give the same result. It is how the project knows, without changing the solver, whether an engineer can solve a real rack model in reasonable time on an ordinary machine, and which stage to work on when it cannot.

It serves SOW-035 under OBJ-003 and OBJ-008 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-05), and PRD §12.2 (deterministic output), §12.8 (reproducible results), §20.1 and §20.2 (public data boundary and provenance) and §25.1 (time to solve representative models). In `docs/RELEASE_TARGET_v1.0.md` it is the owning deliverable of exit criterion 9: on a 2,000-node rack model, on the 16 GiB base machine, measured by this harness, linear static in at most 10 s and nonlinear operating in at most 60 s. Both values are provisional: they are Agent 0's targets, set from intended use and representative examination under the grant in `AGENTS.md` ("Posture for this phase"), and a failure alone does not justify relaxing one.

- **OUT-001** — The performance harness: the reference rack models, the timing and memory measurements of the product's solve from request to displayed result, the kernel-level sparse observations, and the records that state what was measured, on what, and under which limits.

## Deliverable Definition — Ontology

**Terms the harness implements.**

- *Reference rack.* A deterministic, invented pipe rack of an exact node count, generated from bays of straight pipe, realized bends, branches, rigid components and supports. It has three profiles: linear static on the exact route, with its static cases and their combinations; nonlinear operating on the exact route, with lift-off, friction and gap supports; and the nonlinear profile's pressure-free twin. A rack is identified by the hash of its model document. A rack that omits or simplifies a form its target rack carries is *interim*: the generator's ledger names each such form, its interim form and what it waits for. A measurement on an interim rack can show exit criterion 9 not met and never shows it met. As generated now, the linear rack omits the tee and the reducer at each branch, anchor movement and its difference, the hydrotest's test condition, the seismic SRSS and absolute-sum combinations and the two occasional sums built on them, the expansion difference's recorded basis and the operating condition set. The nonlinear racks omit the tee, the branch reducer, the friction shoe whose normal comes from a one-way rest and the operating condition set; their result is published unqualified while a gap state has no confirmation, so their members are not certified; and the product refuses them from 1,775 nodes at its nonlinear admission cap.
- *The clock.* For exit criterion 9, the time from the run request in the desktop until the result is verified and first painted. "Linear static" is every case and combination of the linear rack; "nonlinear operating" is the operating case of the nonlinear rack. Each stage is timed separately, so that a shortfall names its stage: model read and parse, the solve and publication, serialization, the result's verification, transfer to the display, and first paint. A record states two figures and never only their sum: the engine's time, which is the solve worker's stages, and the time from the worker's result to first paint. On a 16 GiB base machine, an engine time above the corresponding exit criterion 9 value shows the criterion not met; only a measurement of the whole clock on that base machine can show it met.
- *Verified before first paint.* Agent 0 decided this on 2026-11-14, under the grant in `AGENTS.md`. Before first paint, every required verification predicate has an outcome established for the held result and its applicable inputs. A later consumer may reuse that outcome only with matching dependencies and must still apply its own admission and presentation rules.
  - *Dependencies.* An outcome's dependencies are:
    - the reader operation;
    - its model-free or model-bound mode;
    - the invocation, where it applies;
    - the current or recorded semantic table and its digest;
    - any separately supplied evidence that affects the verdict.

    Each call compares the actual dependency digests and recomputes when they differ. Model-bound reuse therefore does not wait on a proof that two model texts are identical.
  - *Outside the rule.* Disposal, changed registration, cancellation and memory admission are outside the rule. A restored result does not inherit a fresh solve's registration because its bytes match. A parse kept for sharing keeps its memory charge for as long as it lives.
  - *A fact is shared, not its consequence.* For example, duplicated ids are a finding in standing and a refusal in building the run record.
  - *Nothing is dropped.* No check is dropped, deferred past first paint or weakened. These three stay required until their coverage is established:
    - The run record's per-row interpretation (`interpret` in `analysis_run_v03_build.rs`) tells unknown signatures apart from unit, basis and dimension contradictions under the record's table. It may be replaced only by a result shown to be equivalent per row under that table, with its refusal precedence preserved.
    - The checked number profile needs an authentic outcome tied to the held source, not the attestation's presence. The assembled record's own added members still need their profile checks.
    - The route header may share the parse and the route facts, but is never evidence that the full reading passed. The check that the header agrees with the dispatch stays.

    Until a reuse is built, the clock includes every check the desktop actually runs before first paint, and ends at first paint.
- *The base machine.* A machine with 16 GiB of memory, on which one solve may use at most the product's per-invocation budget of 12 GiB. A run whose peak footprint passes the budget is classed as exceeding it and is not a timed result. Exit criterion 9's base machine is a 16 GiB Apple-silicon notebook, identified by the owner on 2026-11-13 as that machine. Its processor and macOS version are recorded with the first measurement made on it. Criterion 9's absolute timings are made on it and arranged when one is needed. A time taken on a development host is not criterion 9's measurement.
- *Observation.* A recorded measurement with its model hash, build identity, solver mode, host and conditions. An observation asserts nothing; limits come only from exit criterion 9 and the generated-grid screen of REQ-003.
- *Sparse observation.* For a kernel solve: the ordering, the stored profile size, the pivot extremes and their ratio, the count of non-positive pivots, the residual, the agreement between sparse and dense, and the repeat-run difference.

**Boundary.** The harness observes; it contains no solver logic and changes no product result or behaviour. The solve it measures is DEL-04-07's entry over DEL-04-01's kernel; the desktop path it times is DEL-07-07's; conditioning and failure records are DEL-04-06's. Work to make the product meet criterion 9 is done in those deliverables against this harness's measurements.

## Completion and Reliance Basis — Epistemology

A measurement on one host is evidence about that host. Whether exit criterion 9 is met is decided only from a run on a 16 GiB base machine, and setting the values is Agent 0's decision.

*The headless baseline* is a method and a comparison set on the development host where it is made. The method is the timing driver's engine stages on two exact-route workloads: one case of the 2,000-node linear rack, and the invented reference system. Each workload runs five times, the two alternating, as the harness README's headless baseline describes. A second worker repeats the set on the same host, and the two sets are judged for agreement under the thresholds the measurer's own contract states. It shows how the engine's time and memory compare across revisions and candidates on that host. It is not exit criterion 9's measurement: it measures neither the whole clock, nor the whole rack, nor the base machine.

A baseline bundle's read-back is the shell steps, the runner's `--read-back` command and the worker's own checks, all three, as the harness README's headless baseline describes them. It verifies the files read against an inventory written after the run, whose root digest is stated outside the bundle, and checks that the records agree with one another. It does not establish that the files were unchanged before the inventory was written. It does not detect a change made after it has read a file. It does not show that a record is true.

*Measured and not met, as of 2026-11-13.* These figures were taken on the development host, an Apple M5 Max with 128 GiB of memory on macOS 27.2, which is not a base machine, at the engine of commit 16039fa62. Main has since taken pull request 217, under which the rack models, which name no pressure-elongation case, solve without it; main's engine is not the one measured. The figures are from the timing driver's existing whole-text route (B in the measurement plan), for the linear rack on the exact route, in sparse mode. They are driver observations; the desktop's whole clock and the desktop worker's peak memory were not measured. The figures are uncorrected for other load recorded on the host during the measurement window. Below 2,000 nodes each figure is the median of four runs.

| Nodes | Entry stage (s) | Lifetime-maximum footprint at the entry stage's end (GB) |
|---|---|---|
| 250 | 18.9 | 1.56 |
| 500 | 51.0 | 3.14 |
| 1,000 | 139.0 | 6.23 |
| 2,000 | 457.9 and 479.5, one run each | 12.38 |

The entry stage's time grows between 2.7 and 3.5-fold with each doubling of the node count. Its footprint grows in proportion, about 6.2 MB per node.

At 2,000 nodes the entry stage's lifetime-maximum footprint, 12.38 GB, is under the 12 GiB budget (12.88 GB). After the entry stage, the existing route's footprint passed the budget, and the footprint watch stopped it at 13.07 GB, after the entry-stage line and before any serialize-stage line. With a 24 GiB budget allowed, the same route completed, with a lifetime-maximum footprint of 13.83 GB. Its resident maximum was 17.90 GB, beside the 16 GiB of a base machine's memory. That reading is an observation, not the criterion's memory term, which is the per-invocation footprint budget.

The entry stage alone took 457.9 and 479.5 s at 2,000 nodes, on this host at that revision. Criterion 9's value for linear static is 10 s, for the whole clock. Exit criterion 9 remains unmet because its required 2,000-node nonlinear profile is refused, as stated below. These development-host observations do not decide the linear-static profile's compliance on a base machine; no base-machine measurement is supplied here.

Independently of any timing, the product refuses the nonlinear operating rack from 1,775 nodes (Reference rack, above). Criterion 9's nonlinear profile therefore cannot be run at 2,000 nodes, a blocker for the criterion whatever the times. The refusal and the unmet target are each listed in the release limitations, as LIM-014 and LIM-015 (AX-003).

### Requirements

- **REQ-001** — The harness contains no solver logic and changes no product result or behaviour. What it measures is the product's own entry. The native-smoke performance kit and result injection are absent from product builds. The in-product solve-timeline marks change no behaviour, keep only timings, counts and codes, and can be read only where the desktop's UI diagnostics global is installed.
- **REQ-002** — Harness runs are comparable: the same model, build, solver mode and settings give the same result, and every run records what identifies it (SOW-035; `docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-003** — The harness records time, memory and conditioning observations and sets no release limit. The limits it applies are exit criterion 9's and, on the invented generated-grid observation set only, the bounded sparse-suitability screen that set is recorded under: sparse–dense relative difference at most 1e-9, residual at most 1e-6, repeat difference 0, no non-positive pivot, and pivot-ratio estimate at most 1e16. The screen is not a conditioning or release limit, and the pivot ratio is a proxy, not a condition number.
- **REQ-004** — Rack models and fixtures are invented, or public-domain or permissively licensed data from lawful sources. Each records the provenance PRD §20.2 lists, and none contains protected standards content, vendor data or commercial benchmark data (PRD §20.1; `docs/CONTRACT.md` OPS-K-IP-1).
- **REQ-005** — Each record carries the solver version and build identity, the model hash, the host, the diagnostics the solve raised, and a statement of what the measurement does not reproduce.
- **REQ-006** — Rack models state their units and pass the product's own unit and dimension checks as any model does. Which dimensional checks a rack must pass beyond the product's own model checks is not decided.
- **REQ-007** — Harness records are mechanics and performance observations only; none states or implies approval, certification or compliance (PRD §21.2).
- **REQ-008** — The harness measures exit criterion 9 as its clock, racks and base machine are defined above, for the linear static and nonlinear operating profiles, and gives a timing profile for each further analysis that names this harness: hanger sizing (DEL-04-11/AC-011) and the modal scale benchmarks (DEL-04-09/AC-004).

**Undecided.**

- Exit criterion 9's two values become final when Agent 0 decides them. Whether a stated projection from another host's speed may stand in for a base-machine run is Agent 0's choice; none is applied.
- Limits for conditioning, and for time and memory normalized across machines, are not set.
- Whether hosted CI records sparse performance observations is not decided.

### Acceptance criteria

- **AC-001** — On the 2,000-node reference racks, on a 16 GiB base machine, the measured clock is at most 10 s for linear static and at most 60 s for nonlinear operating, with every solve within the 12 GiB per-invocation budget. The record gives each stage's time and peak memory.
- **AC-002** — The bytes the timing driver records for a solve are the bytes the product's own entry writes for the same request, and a product build contains neither the native-smoke injection nor the performance kit, and exposes no timeline reading outside the builds that install the diagnostics global.
- **AC-003** — A rack generated twice for the same node count and profile has the same hash, and two runs of the same rack, build, mode and settings give identical result bytes and row counts.
- **AC-004** — Every record contains the fields of REQ-005, and a run stopped for exceeding the budget or a timeout is recorded as such with the stage it reached, never as a timed result.
- **AC-005** — For each kernel observation the record gives the sparse observation's quantities, and no test or record asserts a time, memory or conditioning limit other than exit criterion 9's and the generated-grid screen of REQ-003.
- **AC-006** — The rack documents open and solve in the product as authored models do, state their units, and contain only invented or lawfully sourced data with its provenance recorded.
- **AC-007** — A timing profile exists for hanger sizing and for the modal scale benchmarks, each recorded like the criterion 9 profiles.
- **AC-008** — No harness record or message states or implies acceptability, approval or compliance.

## Production and Verification Method — Praxeology

**Method choices that define the harness.**

- The engine is timed by a driver that performs the desktop solve worker's own steps on the product's public entry; the desktop is timed in the running application, through marks that change no behaviour.
- One process runs at a time, in an exclusive window, under a footprint watch at the per-invocation budget. Attribution runs, which sample stacks to tie memory to code, are never used as timings.
- Where nothing may be installed on the base machine, the timing driver supervises its own runs using the runner's footprint-watch and classification rules, writes corresponding records that name its readings, and can additionally use the driver's lifetime-maximum footprint read at exit, including after a kill.
- Peak physical footprint, not resident set size, judges the budget, because it is what the product itself enforces.
- No projection is applied to times measured on a host that is not a base machine.
- Result bytes are not kept unless asked; records are.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the measured clock and stage records of the criterion 9 profiles on a base machine): the run's record against the criterion's values. On another host it observes only a proxy for the base machine's time, and says so.
- **VER-002** — Executable check: observes the driver's written bytes against the product entry's serialization, and scans product builds for the native-smoke injection, the performance kit and any timeline reading outside the builds that install the diagnostics global.
- **VER-003** — Executable check: observes rack hashes on regeneration, repeat-run result identity, and each record's fields, including the record of a stopped run.
- **VER-004** — Executable check: observes that the rack documents open and solve in the product and state their units.
- **VER-005** — Judgment by an independent reviewer: examines the fixtures' provenance and the records' wording against `docs/IP_AND_DATA_BOUNDARY.md` and `docs/claims_registry.md`, and confirms that no limit other than exit criterion 9's and the generated-grid screen is asserted.

## Governing Values and Decisions — Axiology

- **AX-001** — *Measure the product, not a stand-in.* Better is a measurement closer to what a user's desktop does, end to end. This is never bought by changing product behaviour to make it measurable, or by timing a path the product does not take.
- **AX-002** — *A shortfall must point somewhere.* Better is a record that attributes more of the time and memory to a stage. Attribution never alters a timing: sampled runs are kept apart from timed runs.
- **AX-003** — *No invented limit.* A value that neither exit criterion 9 nor the generated-grid screen sets is recorded as an observation. An unmet target is listed in the release limitations, not relaxed and not hidden.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-035 OBJ-003 OBJ-008 | REQ-008 | AC-001 | VER-001 | Exit criterion 9 of `docs/RELEASE_TARGET_v1.0.md`, whose values are provisional; a run on another host is a proxy only |
| OUT-001 | SOW-035 OBJ-003 OBJ-008 | REQ-001 | AC-002 | VER-002 | The criterion determines the expected identity and absence |
| OUT-001 | SOW-035 OBJ-003 OBJ-008 | REQ-002 REQ-005 | AC-003 AC-004 | VER-003 | The product's own earlier output; guards against change only. The required fields are determined by the criterion |
| OUT-001 | SOW-035 OBJ-003 OBJ-008 | REQ-004 REQ-006 | AC-006 | VER-004 VER-005 | The criterion determines the expected behaviour; PRD §20 and `docs/IP_AND_DATA_BOUNDARY.md`, read by the reviewer, for the data and its provenance |
| OUT-001 | SOW-035 OBJ-003 OBJ-008 | REQ-003 | AC-005 | VER-003 VER-005 | Exit criterion 9 and the generated-grid screen of REQ-003; the reviewer confirms that no other limit is asserted |
| OUT-001 | SOW-035 OBJ-003 OBJ-008 | REQ-008 | AC-007 | VER-003 | The criterion determines the expected profiles (DEL-04-11/AC-011; DEL-04-09/AC-004); no limit is asserted |
| OUT-001 | SOW-035 OBJ-003 OBJ-008 | REQ-007 | AC-008 | VER-005 | The reviewer's reading against PRD §21.2 and `docs/claims_registry.md` |
