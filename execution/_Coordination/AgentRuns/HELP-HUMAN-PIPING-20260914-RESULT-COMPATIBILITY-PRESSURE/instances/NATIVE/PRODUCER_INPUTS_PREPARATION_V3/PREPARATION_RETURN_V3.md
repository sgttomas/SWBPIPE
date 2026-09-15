# Native producer inputs preparation return V3

**PREPARED — source-derived inputs only.** This stage read maintained source, accepted V1/V2 evidence and public invented fixtures; parsed and hashed local JSON; and wrote only under `instances/NATIVE/PRODUCER_INPUTS_PREPARATION_V3/`. It did not build or execute product code or tests, control a browser or native app, use Git or network, edit source, or delegate.

The sealed brief is `NATIVE_PRODUCER_INPUTS_PREPARATION_V3.md`, SHA-256 `901e3e48df8d33bbe743f4bba5061c7ed7c907e2afeb2063bf30a08790f37060`.

## Prepared inputs

Three invented two-node PreviewModels keep a full six-DOF root anchor, one straight pipe with explicit orientation, the maintained invented section dimensions, explicit E/G, and one load case. The split isolates solver and reachability conditions:

| Case | Files | Source-derived target |
|---|---|---|
| `hanger_constant_effort` | `models/hanger_constant_effort.model.json`, `requests/hanger_constant_effort.request.json` | 052 constant-effort applied load; 056 variable-spring movement limit; 057 constant-effort movement limit; 058 rotational variable-spring stiffness |
| `rotational_nonlinear` | `models/rotational_nonlinear.model.json`, `requests/rotational_nonlinear.request.json` | 053 rotational final displacement in `rad`; 054 rotational final reaction in `N*m` |
| `zero_pressure_longitudinal` | `models/zero_pressure_longitudinal.model.json`, `requests/zero_pressure_longitudinal.request.json` | 059 longitudinal pressure stress in `MPa`, as a reachability probe |

Each request uses the settled headless shape: the RunnerRequest is at `/request`, and the product `LinearStaticPreviewRequest` is at `/solve/preview_model` with the exact prepared model at `/solve/preview_model/model` and an empty external material list at `/solve/preview_model/materials`. The embedded model parses identically to its standalone model file. The stable output location is `/mechanics_envelope`; result rows are `/mechanics_envelope/results`.

The hanger case follows the existing `cantilever_constant_effort_request` pattern. Its constant-effort support has one translational restraint, `UY`, and a finite positive 375 N load, which is the source condition for `constant_effort_support_applied_load`. The variable spring has an explicit positive RZ stiffness of 100000 N·m/rad, and both hanger types have finite positive movement limits. Root anchoring keeps the cantilever stable; the rotational spring is not used to supply rigid-body stability.

The rotational nonlinear case uses the accepted `one_way` class with explicit `RZ`, active initial state and `positive_reaction` activation sense. A -100 N·m nodal RZ moment exercises the rotational path. Root anchoring keeps the model solvable whether the contact finishes active or inactive. The producer chooses `rad` and `N*m` from `support.dof.is_translational() == false` after a completed nonlinear solve. Active-set convergence and final state remain runtime questions.

The pressure case is deliberately separate and uses a genuine element pressure record: `category=pressure`, `dimension=pressure`, element target and 0 Pa magnitude. Source inspection shows `pressure_for_pipe` returns `Some(0)` because the record is present. The pressure-thrust load also exists but has zero axial load, so `pressure_thrust_for_pipe(...) != 0.0` is false and `include_pressure_longitudinal` is true. Stress recovery should retain a zero-valued pressure component and append the exact row. This is a source-derived hypothesis with no actual producer capture; signature 059 remains unresolved until the final candidate executes it. The private pressure kernel is neither used nor activated.

## Existing inputs retained by reference

`REFERENCED_INPUTS_V3.json` points to, and hashes, the maintained curved-bend, explicit-friction, modulus and combination-modulus PreviewModels from V2. Their raw bytes and historical outputs were not copied. At candidate execution, each parsed PreviewModel can be embedded unchanged into a fresh stable-CLI wrapper under the runtime evidence directory, with the original model hash retained in the receipt.

## Later execution contract

`EXECUTION_MAP_V3.json` freezes the command shape:

```text
<FROZEN_CLI> solve --input <ABS_REQUEST_JSON> --output <ABS_OUTPUT_JSON>
```

A case passes only when the final frozen binary exits successfully, `/mechanics_envelope/status/mechanics` is `MECHANICS_SOLVED`, and an actual row matches all three exact fields: `kind`, `unit`, and `metadata.component`. `EXPECTED_SIGNATURES_V3.json` contains the required triples and source-derived preconditions. `verify_declared_signatures.py` checks those exact declarations in captured CLI outputs; it does not run product code and was not run in this preparation stage.

The execution agent must capture the candidate source hash, binary hash, request hash, output hash, exit status and stderr/stdout transport. It must not convert a prepared expectation into a pass. If the rotational case does not converge, split or adjust the evidence-local contact case without weakening its RZ condition. If the zero-pressure case lacks signature 059, record an unreachable/blocking result and route the source/contract inconsistency; do not force the row, activate private pressure mechanics, or substitute the authored semantic fixture.

## Preserved V1/V2 boundaries

The solver-mode row remains discrete disclosure rather than diagnostic work. The 67-row native cantilever and 830-row compatibility population remain distinct. Manifest hashes keep their established seed scope; dedicated package checksums bind final assembled data. Existing browser tests remain fixture-backed delivery evidence only. Final producer and frontend qualification is candidate-bound and awaits root's serialized build/native execution lease.
