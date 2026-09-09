# F4-I3 six-path application brief

Status: `SEALED_FOR_DISPATCH`

## Identity and method

- Parent: `/root/friction_execution`, WORKING_ITEMS Agent 1, instance F4.
- Executor: bounded Agent 2, `gpt-5.6-sol`, high reasoning, no delegation.
- Method: minimum exact application and validation of an already reviewed candidate; do not reopen the numerical decision.
- Working root: `{WORKING_ROOT}` = the current `chirality-physics-ui-implementation-20260908` checkout at branch `codex/piping-physics-ui-implementation-20260908`, pre-application HEAD `7b73460c5e2d85a9f050344069d211fea4af7b3e`.

## Authority and immutable inputs

- Owner approval: `{RUN_ROOT}/post_g0/OWNER_APPROVAL_V1.json`, SHA-256 `cdd26a7a0347331e8ff99a1d621a912fac0737f8c2fcfba358130855e3c46fd6`.
- Approved proposal: `{RUN_ROOT}/post_g0/PROPOSED_OWNER_ACT_V2.md`, SHA-256 `9dfa3140d7762514479eb3983178298977bb1f5fedfdcf91253547a42b62a400`.
- Root-dispatched application amendment: `{RUN_ROOT}/post_g0/F4_SIX_PATH_APPLICATION_LAUNCH_AMENDMENT_V1.md`, SHA-256 `8da19cbb8c259ad89ef0280f399c425b399bb89081d8b8db286c2463e1c1f504`.
- Work graph: `{RUN_ROOT}/post_g0/WORK_GRAPH_V3.json`, SHA-256 `d8d55111332b464735854478d744326d53da0f3388f3fbebf7cee8d89f0aa6be`.
- RF V4 technical PASS: REVIEW `102bb9938617b4c8ca88e032c022bb8f121d9817543bf2fcadd4098cf43f482a`, RETURN `e6b4d07d8d91efdddd948308243836e11903f8f5767199896adaf986a51a578d`, MANIFEST `082b45c0cacfc19435c1687e78c80a5e39531e6f0a9d9d27c46f646d2903ee7b`, STATUS `47562640aba4e6f79ba6fa21a1768f74e3980d2028f38f50125bed21dc69c30c`.
- Candidate envelope: `{DEL_RUN}/post_sweep_g0/successor_v4/PROPOSED_ACTIVE_ORACLE_UPDATE_V4.patch.b64.json`, SHA-256 `5103bf895c4792989d4180a2667ec06adefe45c0247abdfe13f0a95df5c49928`; decoded patch SHA-256 `c31a705a39c4629aacf6a79871c1497a8b8d3976e389478f79e41f4f7752c334`.

`{RUN_ROOT}` means `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION`. `{DEL_RUN}` means `projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908`. Resolve all paths inside `{WORKING_ROOT}`.

## Exact write fence

Apply edits only to these six paths and require these exact preimage/post-image pairs:

| Path | Preimage SHA-256 | Post-image SHA-256 |
| --- | --- | --- |
| `projects/chirality-piping/core/product_physics/src/lib.rs` | `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903` | `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5` |
| `projects/chirality-piping/fixtures/product_preview/invented_mechanics_result.json` | `f0a0ddcdc896df844e303b8278e0bbced295166dedf0f674a26014e5fe35bdee` | `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13` |
| `projects/chirality-piping/tests/product_preview/test_product_preview_service.py` | `467350784ed4217a00ff0b36e37a087fc68d3c45aa930a599a2469e6cb0ecc27` | `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735` |
| `projects/chirality-piping/apps/desktop/src/services/previewService.test.ts` | `c10969ff404ec652395a85ac2b817a314543ec5e3cc68878409ae58a851351e7` | `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f` |
| `projects/chirality-piping/apps/desktop/src/App.test.tsx` | `629037c503c6cf9b32ef3cc6b59e3c04234ec2c8ad61879e6ba73ef43cd3a1be` | `5fa1e1e0439690db4692033eda0ae66c98c79fb050ce2cd1214fb74c417d855c` |
| `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` | `4cd16a28a710789d07f3ee0ebd89c946896ef8d51bc2877cc64865da1dc0b430` | `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17` |

Evidence writes are limited to `{RUN_ROOT}/instances/F4/post_sweep_g0/application_v1/children/I3/**` and `{DEL_RUN}/post_sweep_g0/application_v1/child_I3/**`. Create `RETURN.md` and `STATUS.json` in the first location. Store command outputs and complete application evidence in the second. No other write is permitted.

## Execution

1. Rehash the authority inputs, envelope, decoded patch, and all six preimages. Stop without mutation on any mismatch.
2. Decode and apply exactly the reviewed patch once. Rehash all six post-images and verify the applied scoped diff is byte-identical to the decoded patch. Do not hand-edit or regenerate a different candidate.
3. Run only these validation categories, using a private Cargo target outside the repository and the reusable Python interpreter named by `projects/chirality-piping/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/HOST_PATHS.json`:
   - exact Rust witness `tests::valid_invented_model_exposes_nonlinear_support_loop_evidence`;
   - complete `open_pipe_stress_product_physics` crate tests with its manifest, locked/offline where supported;
   - focused Python file `tests/product_preview/test_product_preview_service.py`;
   - focused desktop service test `previewService.test.ts`;
   - the two affected App tests named `carries queued editor intents into the report packet as review-only operation context` and `shows computed mechanics diagnostics in results, knowledge, and review-only proposal context`;
   - focused browser test `R2 desktop preview smoke covers solve, results, report, and viewport overlay`;
   - registered generator parity by running the package's `generate:product-preview-mechanics` producer to a disposable file without overwriting the live fixture, then requiring byte equality and SHA-256 `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`.
4. Preserve exact commands, exits, concise outputs, and any failure. Do not weaken, tune, retry around, or reinterpret a failed assertion. A tooling/environment failure may be diagnosed only within the same checks and write fence.
5. Freeze a complete binding for the six applied files, the unchanged nonlinear solver producer, the fixture generator source and registration, and all original reviewed source/test members from the final RI source inventory. Record source/test/fixture classifications and aggregate serialization. Confirm no path outside the six-file fence changed during this child run.

## Exclusions and return

No production logic, API, solver history, threshold, pressure, units, dependencies, DAG, lifecycle, receipt, native build, full DEC-025 sweep, practitioner suite, Git commit, push, PR, CI, publication, or merge. Do not delete or rewrite preliminary/failed evidence. Do not claim the prior native bundle covers the changed runtime fixture.

Return exact applied hashes, scoped diff hash, generator parity, every validation result, complete binding and aggregate hashes, changed-path proof, residual risks, and blockers. Stop after the frozen return for manager validation and fresh root-routed RF review.
