# Proposed Owner act V2: bounded six-path G0 oracle correction

Status: `PROPOSED — OWNER APPROVAL REQUIRED — CANDIDATE UNAPPLIED`

The accepted bounded F4/UI integration remains valid at its recorded cut. Its Step 1 source exception expired at that integration decision. The later clean release sweep failed DEC-025 G0 in `tests::valid_invented_model_exposes_nonlinear_support_loop_evidence`: observed friction `0.489527` versus stale expected `0.490101`; G1–G4 did not run. The failure and publication hold remain recorded in [`DEC025_G0_FAILURE_HOLD_V1.json`](../dispositions/DEC025_G0_FAILURE_HOLD_V1.json), SHA-256 `f91a721992b62659e768c6e8a867cf8c9e34586fac9132374a319a4b53fdc505`.

F4 prepared an unapplied six-path candidate from tested commit `7b73460c5e2d85a9f050344069d211fea4af7b3e`. Its manifest is [`MANIFEST_V4.json`](../instances/F4/post_sweep_g0/successor_v4/MANIFEST_V4.json), SHA-256 `4cd095a4a7e133a5b0dfdc290e41dea903cad6e166e2bb17a115d7988e7646b8`; decoded patch SHA-256 is `c31a705a39c4629aacf6a79871c1497a8b8d3976e389478f79e41f4f7752c334`. Fresh independent RF review returned `PASS`: [`REVIEW.md`](../instances/RF/post_sweep_g0/successor_v4/REVIEW.md) `102bb9938617b4c8ca88e032c022bb8f121d9817543bf2fcadd4098cf43f482a`, [`RETURN.md`](../instances/RF/post_sweep_g0/successor_v4/RETURN.md) `e6b4d07d8d91efdddd948308243836e11903f8f5767199896adaf986a51a578d`, [`MANIFEST.json`](../instances/RF/post_sweep_g0/successor_v4/MANIFEST.json) `082b45c0cacfc19435c1687e78c80a5e39531e6f0a9d9d27c46f646d2903ee7b`, and [`STATUS.json`](../instances/RF/post_sweep_g0/successor_v4/STATUS.json) `47562640aba4e6f79ba6fa21a1768f74e3980d2028f38f50125bed21dc69c30c`.

## Exact authorized fence

Approval would authorize applying only the reviewed candidate post-images below:

| Path | Candidate SHA-256 | Authorized change |
| --- | --- | --- |
| `projects/chirality-piping/core/product_physics/src/lib.rs` | `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5` | Existing test block only: current-normal fixture expectations and the `round6(μN)` relationship; no production Rust logic. |
| `projects/chirality-piping/fixtures/product_preview/invented_mechanics_result.json` | `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13` | Regenerated runtime reference fixture: two summary values and 338 result values; all 830 ordered result identities and 31 diagnostics are preserved. |
| `projects/chirality-piping/tests/product_preview/test_product_preview_service.py` | `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735` | Narrow generated-fixture expectation update. |
| `projects/chirality-piping/apps/desktop/src/services/previewService.test.ts` | `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f` | Narrow generated-fixture expectation update. |
| `projects/chirality-piping/apps/desktop/src/App.test.tsx` | `5fa1e1e0439690db4692033eda0ae66c98c79fb050ce2cd1214fb74c417d855c` | Exactly three fixture-derived literal replacements: one `4.927109`→`4.927112` and two `3.977299`→`3.977301`. |
| `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` | `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17` | Exactly one fixture-derived literal replacement: `4.927109`→`4.927112`. |

The reviewed numerics set current friction to `0.489527`, current normal to `48.952719`, and retains the explicit `μ = 0.01` relation. Exact rational review also records the historical no-spring L-200 observed normal as `24.476359`, within the independently permitted six-decimal cells `{24.476359, 24.476360}`, with unique friction `0.244764`.

`App.test.tsx` is one of the original ten reviewed source/test members, so its hash is intentionally replaced by the exact three-literal delta above. Production logic remains unchanged. Every other original reviewed member outside this six-path fence must remain unchanged, and the applied tranche must produce a new complete source/test/fixture binding.

This act does not adopt or alter pressure choices, public APIs, solver history behavior, thresholds, units, dependencies, the project DAG, lifecycle state, or engineering acceptance. It does not waive the failed sweep or authorize any additional path.

## Required gates after application

1. Verify candidate preimages, apply only decoded patch `c31a705a39c4629aacf6a79871c1497a8b8d3976e389478f79e41f4f7752c334`, and reproduce all six post-image hashes.
2. Run the focused Rust, Python, service, App, and browser fixture assertions; run the full `product_physics` crate checks; and prove exact registered-generator parity with the runtime fixture.
3. Have RF independently backcheck the applied diff, complete binding, and validation results.
4. Produce a fresh packaged-native build with bound source, fixture, frontend distribution, and bundle identities, then perform one bounded default-fixture native smoke/solve in an isolated store. The earlier R2 authored-model walkthrough remains historical evidence for the unchanged successful backend path. Browser fallback tests and bundle asset identity cover the changed fallback-data path; the native smoke need not claim an unsupported IPC-failure path.
5. Create the authorized source-freeze and tested commits before running the clean complete five-surface DEC-025 sweep. After that sweep and the practitioner suite pass, append and validate Receipt 137 against the exact tested commit and sweep, complete required self-check validation, and create the evidence-only commit. Push and open the PR to obtain exact-head CI. Merge only after every required gate, including exact-head CI, passes.

If approved, this new exception authorizes only the six paths above through corrected full-release verification and root acceptance of the corrected-source fan-in. It expires at that acceptance. It may not silently expand. The user's standing Git authority permits the prerequisite source-freeze and tested commits, the later evidence-only commit, push, and PR creation in the stated order; it permits merge only after all required gates pass.

## Requested Owner act

> I approve the bounded six-path G0 oracle correction in this `PROPOSED_OWNER_ACT_V2.md`, including its exact path fence, required validation and native gates, and expiry at accepted corrected-source fan-in.
