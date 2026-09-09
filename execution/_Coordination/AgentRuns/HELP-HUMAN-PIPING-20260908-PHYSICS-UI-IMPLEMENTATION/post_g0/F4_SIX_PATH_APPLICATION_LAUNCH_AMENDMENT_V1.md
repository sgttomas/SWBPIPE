# F4 six-path application launch amendment V1

Status: `SEALED — OWNER AUTHORIZED — ROOT DISPATCH PENDING`

Owner approval is recorded in [`OWNER_APPROVAL_V1.json`](OWNER_APPROVAL_V1.json), SHA-256 `cdd26a7a0347331e8ff99a1d621a912fac0737f8c2fcfba358130855e3c46fd6`. It adopts [`PROPOSED_OWNER_ACT_V2.md`](PROPOSED_OWNER_ACT_V2.md), SHA-256 `9dfa3140d7762514479eb3983178298977bb1f5fedfdcf91253547a42b62a400`. This amendment activates only that exact six-path correction. F4 remains the manager and may dispatch one bounded Agent 2 using `gpt-5.6-sol` at high reasoning, with no delegation. Actual execution begins only after root dispatch.

## Candidate and write fence

Use F4 V4 manifest [`MANIFEST_V4.json`](../instances/F4/post_sweep_g0/successor_v4/MANIFEST_V4.json), SHA-256 `4cd095a4a7e133a5b0dfdc290e41dea903cad6e166e2bb17a115d7988e7646b8`. Decode the candidate only from `projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/post_sweep_g0/successor_v4/PROPOSED_ACTIVE_ORACLE_UPDATE_V4.patch.b64.json`, SHA-256 `5103bf895c4792989d4180a2667ec06adefe45c0247abdfe13f0a95df5c49928`, and require decoded SHA-256 `c31a705a39c4629aacf6a79871c1497a8b8d3976e389478f79e41f4f7752c334`. Verify each live preimage before mutation and each post-image after application.

| Exact writable path | Required preimage SHA-256 | Required post-image SHA-256 |
| --- | --- | --- |
| `projects/chirality-piping/core/product_physics/src/lib.rs` | `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903` | `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5` |
| `projects/chirality-piping/fixtures/product_preview/invented_mechanics_result.json` | `f0a0ddcdc896df844e303b8278e0bbced295166dedf0f674a26014e5fe35bdee` | `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13` |
| `projects/chirality-piping/tests/product_preview/test_product_preview_service.py` | `467350784ed4217a00ff0b36e37a087fc68d3c45aa930a599a2469e6cb0ecc27` | `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735` |
| `projects/chirality-piping/apps/desktop/src/services/previewService.test.ts` | `c10969ff404ec652395a85ac2b817a314543ec5e3cc68878409ae58a851351e7` | `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f` |
| `projects/chirality-piping/apps/desktop/src/App.test.tsx` | `629037c503c6cf9b32ef3cc6b59e3c04234ec2c8ad61879e6ba73ef43cd3a1be` | `5fa1e1e0439690db4692033eda0ae66c98c79fb050ce2cd1214fb74c417d855c` |
| `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` | `4cd16a28a710789d07f3ee0ebd89c946896ef8d51bc2877cc64865da1dc0b430` | `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17` |

No other source, test, fixture, dependency, DAG, lifecycle, receipt, or Git write is authorized to F4 or its child. Preserve production logic and every original reviewed member outside this exact fence.

## Application and validation return

Apply the decoded patch once. Run the focused Rust, Python, preview-service, App, and browser assertions; the complete `product_physics` crate tests; and exact registered-generator parity against the runtime fixture. Record exact commands, exits, hashes, and failures. Freeze a complete six-path applied manifest, diff, generator-parity evidence, validation, and manager return under F4's existing post-G0 instance/run-record scopes. Do not suppress or reinterpret failures.

Return control to root for a fresh RF read-only applied-diff backcheck. F4 and its child must stop before RF begins. F4 does not perform native build/witness, DEC-025, Receipt 137, CI, publication, or merge. The approved exception remains limited to these six paths and expires only at root's accepted corrected-source fan-in after all prescribed release verification.
