# V1-C — Test coverage, reachability, and sensitivity audit

## Basis

Frozen source `2be412ccea62bdc4bd96deb082c46d7a792076ea`; accepted parent basis decomposition 0.12 / SCA-009 / DAG-010 and the approved physics-audit plan. This is derivative audit evidence, not scope truth, release acceptance, or engineering validation. TASK/evaluation-protocol Agent 2; no children; actual model not exposed, recorded UNKNOWN. Role non-delegation is instruction+config asserted.

## Method

Inventoried all 38 non-generated project Cargo manifests, 608 ordinary Rust test functions across the numerical/core runner and three benchmark crates, and 710 Python test definitions. These are source-definition counts, not independent cases, passed tests, or runtime branch coverage. Parametrization explains why suite execution counts differ. Preserved full Rust test bodies for review and 168 source/test/policy hashes. Focused semantic assertion and call-reachability review covered each family in COVERAGE.csv and all seven mandatory witness leads. This does not claim a line-by-line semantic review of all 608 bodies or all 710 Python definitions.

With the parent-released build slot, executed four serial copies of the complete product `--lib` suite. Each copy retained the same assertions, dependencies and fixtures. Only dependency/include path relocation and the documented production mutation changed. Expectations were frozen in MUTATION_EXPECTATIONS_V2.json before execution. No source, fixture, expected value or tolerance was changed in the actual project.

## Coverage

COVERAGE.csv records 24 family dispositions and representative checks; WITNESS_COVERAGE.csv maps W1–W7. TEST_ASSERTIONS.json, PYTHON_TEST_INVENTORY.json and CARGO_MANIFEST_INVENTORY.json preserve inventories. These distinguish unit arithmetic, numerical references, shared-path differential checks, metadata/contract checks, and policy-scoped observations.

Strong existing coverage includes small dense/sparse references, frame transform and prescribed-reduction checks, curved-bend rigid-body/energy/equilibrium identities, thermal fixed-end correction, canonical straight-pipe station load accumulation, alternate-unit conversion, constant-effort superposition, and explicit invalid-input cases. These do not automatically cover product adapter consumption of those kernels.

Examples of necessary calibration:

- `mechanics/src/lib.rs:4263` imposed-displacement benchmark prepares a boundary and checks `k*d`; it does not solve an assembled spring/prescribed model. Other lower-level tests check actual force reduction.
- `mechanics/src/lib.rs:963` thermal observation computes `E*A*alpha*dT` directly. Actual thermal integration is tested elsewhere, including product `fixed_fixed_thermal_load_applies_axial_fixed_end_correction`.
- TP-PHYS-014 expected-value construction calls its solver, but `validate_tp_phys_014_canonical_analytical_payload` at 5883 checks fixed numeric loads, reactions, displacements and station values. Its benchmark test calls that predicate. It is therefore incorrect to label the whole case purely self-comparing based on its constructor alone.
- `tests/test_calculation_witness.py:66` compares production section calculation with independently interpreted Decimal witness values and result-envelope quantities. The test name's binding language is not evidence of circular implementation lineage; reference manager V1-R owns detailed independence assessment.
- Most nonlinear product checks inspect support-specific rows, states and policy metadata. Ordinary nodal/force/stress tests inspect another surface. A shared selected-solution equality assertion is not supplied by those separate assertions.

## Executed sensitivity results

| Copied production variant | Complete product unit result | Meaning |
|---|---|---|
| Unmodified baseline, path relocation only | 127 passed | Valid baseline and executable copied test setup |
| W3: omit rotational reaction terms from published force-resultant norm | 127 passed | The suite did not distinguish these two force-resultant constructions |
| W5: zero every default-wrapper published row whose metadata basis is `interpolated_from_endpoint_resultants` | 127 passed | The suite did not detect corruption of these interpolated station values |
| Positive control: zero all default force, moment and stress result values | 114 passed, 13 failed | Numerical assertions exist and detect broad value corruption; this is not a metadata-only suite |

Exact outputs are encoded without alteration in MUTATION_baseline.json, MUTATION_W3_force_only_reaction.json, MUTATION_W5_zero_interpolated_stations.json and MUTATION_positive_zero_force_stress.json. MUTATION_PER_TEST.csv records all 508 individual outcomes. MUTATION_PATCHES.json preserves relocation and mutation diffs plus input/output hashes. SOURCE_UNCHANGED.json confirms all 168 captured subject files were unchanged afterward.

The W5 mutation acts at the public default wrapper, after solve and combination generation. It corrupts every matching published default-path row and leaves direct `with_mode` callers untouched. Its scope must not be generalized to all lower-level recovery, all combination paths, WASM, GUI or the full repository suite. The W3 direction could form part of a correction but this experiment does not itself validate the intended reaction definition. Neither mutation result independently reproduces the original physics defect; each proves a specific regression-sensitivity gap.

The existing product force/stress exposure tests explicitly assert `interpolated_from_endpoint_resultants` metadata and largely check IDs, units and labels. Such tests correctly serve their contract purpose; the missing complementary numerical checks are the issue. Kernel station recovery tests cannot compensate when the native product uses a different recovery route.

## Validated-return inventory

No children were dispatched. Parent owns the unmodified full Rust/Python suite runs and other managers own independent physics witnesses. This packet makes no claim that their pending returns have passed or been accepted.

## Findings

- V1-C-01: reproduced product-suite insensitivity to W5 published station-value corruption.
- V1-C-02: reproduced product-suite insensitivity to W3 reaction-norm definition change.
- V1-C-03: confirmed separation between kernel benchmark reachability and selected product behavior; missing complementary end-to-end checks for selected nonlinear solution, mixed spring transfer, spring/active support stabilization and changed-model identity remain audit gaps.

FINDINGS.csv carries classification, confidence-calibrated status, evidence and repair owners. Severity describes the consequence of missing regression protection, not the numerical severity of another manager's unreviewed defect claim.

## Conflicts / unknowns

No exhaustive mutation score, compiler coverage instrumentation, fuzz coverage or whole-repository mutation run was performed. Numerical correctness of the identified physics leads is assigned to numerical/integration specialists. Broad generated rotations, translations, permutations, extreme scales and arbitrary support combinations are not established by this inventory. The absent named tests are a search/review result, not a theorem that no indirect assertion could ever detect the behavior.

User-stiffness finite-length rigid-motion behavior may depend on the intended physical contract; do not implement a new formulation on the strength of an absent test. General convergence criteria and arbitrary nonlinear load-history assumptions remain Owner decisions. Existing thresholds were neither adopted outside scope nor relaxed.

## Recommendations

After full-baseline acceptance, couple accepted physics repairs with independent numerical regressions at the actual product entrypoint. Add dimension-separated reaction checks, straight-pipe station equilibrium under distributed/partial loads, and equality of ordinary outputs to the selected converged solution. Include mixed spring/nonlinear stability and distinct model/settings provenance cases. Retain metadata tests and lower-level benchmarks. Freeze independent expected values before implementation; require the original defects and targeted corruption to be detected without changing comparison criteria.

## Decision queue

No new Owner decision is required to record these coverage findings. Parent must route any physical-formulation, public-contract or convergence-policy changes arising from the substantive audit. Test strengthening inside existing accepted mechanics can be briefed to the owning WORKING_ITEMS manager only after whole-baseline review.

## Handoff summary

Coverage and targeted sensitivity work complete with explicit limitations; no engine-correctness closure. Parent must validate this packet, integrate independent physics witnesses and complete fresh baseline review before repair. Source changes affecting tested routes require rerunning the original fixtures, full product suite and relevant independent benchmark checks. No production repair, lifecycle status, authority pointer, Git state or running application was changed.
