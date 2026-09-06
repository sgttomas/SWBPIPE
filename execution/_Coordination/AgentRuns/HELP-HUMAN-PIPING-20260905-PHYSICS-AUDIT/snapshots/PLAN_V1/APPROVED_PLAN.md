# Comprehensive Rust Physics Audit and Bounded Repair Program

Provenance: faithful parent-transmitted semantic copy of the user-approved proposed plan. This is not claimed to be recovered raw UI bytes. Original layout is not an immutable authority requirement. The exact current Owner utterance supplied by parent is “Implement the proposed plan.” The plan is accepted execution scope through that user direction; no agent adopts it on the Owner's behalf. Exact selected choices are retained in USER_CHOICES.json.

## 1. Objective, scope, governing decisions

Determine whether the implemented pipeline faithfully transforms an authored piping model into coherent, reproducible mechanical results. Audit numerical kernels AND actual application consumption, then repair confirmed defects within existing requirements.

Questions: correct units, geometry, materials, loads and boundaries; declared element, assembly and solver mechanics; a clearly identified nonlinear solution consistently driving recovery/publication; correct force, moment, stress, combinations, extrema, diagnostics and identities; independent test verification including the native route; which capabilities are verified, defective, disconnected, unsupported or uncertain.

The Owner established: full implemented numerical pipeline; complete independently reviewed entire baseline before production repairs; serious findings immediately without stopping unrelated audit; follow with bounded fixes; internal designs including solution selection/adapters allowed consistent with requirements; escalate physical assumptions, engineering acceptance/convergence thresholds, public compatibility and scope amendment; vetted project, independently reviewed analytical and authoritative open references, with commercial references optional.

Include units, consumed section/material calculations, compilation, elements, supports, loads, solvers, recovery, combinations and publication; native/headless/WASM only where actual capabilities exist. Persistence/reporting is limited to numerical provenance/result association. Exclude comprehensive storage redesign, rule-engine audit, UI, new physics and professional acceptance. No production API, schema, numerical-policy or source changes during baseline.

## 2. Organization and sequencing

HELP_HUMAN is Agent 0. Three EVALUATION Agent 1 managers: numerical mechanics with three TASK specialists (elements/assembly; linear numerics; nonlinear supports/convergence); product integration with two (input compilation/load-support transfer; recovery/combinations/publication); verification evidence with two (independent references; benchmark coverage/sensitivity/reproducibility). Maximum initial root + three managers + seven specialists = eleven. Specialists do not delegate. Each receives frozen basis/files/tools/isolation/checks/returns.

### Phase A — frame and freeze

CHANGE creates a clean audit branch from verified origin/main containing PR724, records exact commit and inspects intervening Piping changes. Resolve current accepted decomposition, scopes, DAG, numerical decisions and deliverable status (inspected basis 0.12/SCA009/DAG010). Inventory all scoped production modules/entrypoints/tests/benchmarks/calculations/policies. Capability/reachability matrix includes requirement and owner; implementation and callers; kernel/adapter/native coverage; supported families and limits; references, tolerances and decisions. Seal protocol and briefs before fan-out. Classify disconnected lower-level capabilities explicitly.

### Phase B — full frozen baseline

Three managers work in parallel on the frozen basis. Review all scoped production logic with coverage. Reference expected behavior without using production outputs to set expected values/tolerances. Existing suites run before independent witnesses. New harnesses/experiments are quarantined in evaluation directories; no production fixture or expected-value edits. Use isolated targets and serialize heavy/shared resources. Preserve failures and contradictions.

### Phase C — validate whole baseline

Managers validate children; Agent 0 performs fan-in. A fresh independent consolidated-report review examines coverage, reference independence, critical reproductions and repair fences; it challenges overstated defects and verified claims. Freeze the WHOLE baseline before any production repair. Audit complete is distinct from engine correct; defects and unknowns can remain.

### Phase D — bounded repair

Package WORKING_ITEMS repairs require a confirmed finding and requirement, minimal reproducer or failing check, source ownership and compatibility, implementation objective, closure test, fresh review and post-verification.

### Phase E — verify and publish

Rerun original witnesses on repaired source while preserving before/after. Run integration checks; provide reviewed repair PR and CHANGE Git closeout. No self-merge.

## 3. Technical coverage

### Inputs

All normalized coordinates/scalars/vectors; m/mm/in equivalence; translation and large origin; mixed units; absolute temperature versus delta; E/G/thermal property selection and interpolation; section/effective wall; missing, invalid and nonfinite input, duplicates and unresolved input; source preservation and idempotence.

### Elements

Axial, both bending axes and torsion; straight, curved, rigid, user stiffness and mixed elements; degrees of freedom, local frames, reversal, symmetry, rigid-body modes, connectivity, assembly, work/energy and formulation assumptions.

### Supports

Rigid, translational/rotational springs, constant effort and imposed support behavior where implemented; redundant/conflicting/disconnected/mechanism cases; spring and nonlinear stability; restraint-count heuristics.

### Loads

Nodal forces/moments; local/global distributed and partial-span loads; self-weight/mass; thermal; pressure thrust/membrane; implemented equivalent-static wind/seismic. Check signs, application, resultant force AND moment, nodal work, duplicates and omissions.

### Linear numerics

Dense/sparse assembly and solution; ordering, factorization, reduction, prescribed conditions, singularity/conditioning, fallback, zero load, full restraint, stiffness scale, nonfinite handling and determinism. Independently assess residual/backward error.

### Nonlinear numerics

Gap open/close; one-way activation/release; lift-off; stick/slip; both directions; zero-friction normal force; mixed cases; initial states; limits and cycling; state-stable but unbalanced cases; equilibrium, complementarity, friction bounds and exact policy scope.

### Recovery

Local/global and end-sign conventions; interior stations/extrema/discontinuities; curved and fixed-end effects; thermal/pressure/modifiers; force/moment dimensions; precision before rounding.

### Combinations and publication

Linear superposition only where valid; incompatible nonlinear/modulus bases; range subtraction; missing, partial or failed results; selected-solution identity; stale/model/settings/case/run association; native/headless behavior; metadata, serialization and export.

### Rust robustness

Indices/dimensions, panics, overflow/underflow, nonfinite handling, error suppression/defaults, shared mutable state and unsafe/FFI if present. Distinguish numerical findings from robustness findings.

### Seven mandatory initial witnesses

1. Replay the original unchanged repaired m/mm/in fixtures, then translated/rotated cases and other dimensions.
2. Trace nonlinear versus ordinary final displacement/reaction through ordinary nodes, forces, stresses, maxima and combinations.
3. Force-resultant accounting that incorporates rotational reactions.
4. Transfer of linear spring stiffness into nonlinear analysis.
5. Straight-pipe interior stations/extrema against independent distributed-load calculations.
6. Springs/active nonlinear supports stabilizing models rejected by preliminary checks.
7. Distinct model/settings requests with ambiguous or reused result identity.

Items 3–6 are investigation targets until reproduced against a supported contract, not confirmed numerical defects from static inspection alone.

### Methods

Use dimensioned analytical independent references with assumptions; conservation of force/moment, compatibility and appropriate work/energy; metamorphic units, rigid translation, rotated loads/supports, permutations and linear scaling where valid; refinement where convergence is meaningful. Dense/sparse and shared native/headless differential comparisons corroborate rather than independently prove correctness. Controlled mutations in isolated copies test detection of omitted loads/units/signs/selected vectors. Deterministic seeded, bounded, well-posed generated cases preserve seeds and minimized counterexamples. Do not indiscriminately apply linear superposition or path independence to nonlinear behavior.

### References and tolerances

Inspect hand-calculation independence rather than trusting a label; record shared code/formulas/constants/lineage. Freeze expected results and criteria before comparisons. Apply approved tolerances only within their scope. New cases independently document error estimates and proposed criteria. Missing acceptance basis yields observed error and a decision, never invented PASS. Fixture convergence policy is never promoted to production policy. Extracted piping-design equations are NOT physics references. New external references need authoritative provenance, applicability and precise citations; distinguish analytical from experimental evidence.

## 4. Repair program

The full reviewed-baseline gate applies. Default order: inputs → element/load/support assembly → solver/convergence reporting → internal selected solution → recovery/stress/combinations → identity/diagnostic publication. Disjoint fixes may run in parallel; shared product adapters are serialized with a single owner. Repair coupled defects together, not by patching display rows.

Internal types for coherent displacements/reactions/convergence/active state/basis are permitted when consistent with requirements; document and review all consumers. Owner gates apply to new formulation, friction load history, extrapolation, production convergence/engineering criteria, public schema, migration compatibility, adoption of unsupported behavior or scope change. Hold only affected work. No silent fallback, tolerance loosening or redefining expected results.

Preserve original baseline evidence. Regression must detect the original defect; corrected behavior must match accepted references; check neighboring invariants and mixed cases. Require fresh review over 100% frozen production diff, relevant crates/benchmarks, clean registered DEC-025 before publication, native rebuild and isolated actual native checks for integration. Hashes bind reviews, tests and builds. Distinguish source commit from later evidence commit. Do not change user projects or the running app.

## 5. Deliverables and completion criteria

Use a unique execution/_Evaluation audit directory and AgentRuns controls, with separate baseline and post-repair snapshots. Deliver:

- Protocol.
- Coverage matrix: requirements → capability → implementation → entrypoints → references → checks → verdict.
- Reference register: provenance/derivation/independence/assumptions/applicability/tolerance.
- Schema-compliant findings with expected/actual behavior, configuration, impact, owner, reproducer and closure test.
- Baseline report distinguishing verified behavior, defects, unsupported behavior, conflicts and unknowns.
- Reproduction bundle: fixtures/harnesses/commands/environment/raw evidence/seeds/hashes/rerun instructions.
- Ordered repair briefs and decision queue.
- Post-repair before/after evidence and dispositions.
- Final handoff: completed scope, residual decisions, reruns, derivative status and next actions.

Severity is separate from confidence; distinguish hypothesis, reproduced and corrected. NO aggregate numerical-quality score.

Baseline complete means: every scoped module/family has a coverage disposition; every supported family has its existing evidence assessed and planned representative checks executed OR a specific unresolved gap; all seven witness outcomes are recorded; serious findings are independently reproduced OR explicitly unconfirmed; conflicts, unavailable references and missing acceptance bases remain visible; fresh review accepts completeness and calibration.

Repair complete means: all accepted bounded repairs pass closure tests, review and integration; residuals have owners and concrete prerequisites; verified and unresolved scope is stated exactly; reviewed PR and reproducible evidence are available. Software/mechanics evidence is NOT blanket validation, all-model correctness or engineering-hold closure.
