# RF successor V4 independent review

Verdict: `PASS`

No actionable finding remains in the held V4 candidate.

Decoded patch `c31a705a39c4629aacf6a79871c1497a8b8d3976e389478f79e41f4f7752c334` applies in memory to exactly six paths and reproduces all declared post-image hashes. The Rust, generated fixture, Python, and preview-service TypeScript patch sections are byte-identical to V3. The V4 additions are exactly one `4.927109`→`4.927112` and two `3.977299`→`3.977301` replacements in `apps/desktop/src/App.test.tsx`, plus one `4.927109`→`4.927112` replacement in `apps/desktop/e2e/r2-smoke.spec.ts`. An active-source virtual-candidate scan finds no remaining exact old numeric literal from the accepted 340-entry generated-fixture inventory.

Independent exact `Fraction` reconstruction retains four feasible historical-no-spring L-200 vertices. Its six-decimal normal cells are `{24.476359, 24.476360}`, the observed target is `24.476359`, and friction remains uniquely `0.244764`. `AFFINE_BOUNDS_V4.json` records the same numerical result, with `24.47636` as the canonical decimal spelling of `24.476360`. This closes the V3 claim-calibration finding.

Accepted evidence reused: V3 proves the Rust test contains exactly one assertion each for friction `0.489527`, current normal `48.952719`, and `round6(0.01 * normal)` while retaining state, metadata, and diagnostic checks. V2 proves all 280 affine rows maintain the stated branch/state/convergence conditions and full Dense/Sparse payload equality, and that the runtime fixture changes exactly two summary values plus 338 result values while retaining 830 ordered result IDs and 31 diagnostics.

Six candidate post-images:

- `core/product_physics/src/lib.rs`: `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5`
- `fixtures/product_preview/invented_mechanics_result.json`: `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`
- `tests/product_preview/test_product_preview_service.py`: `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735`
- `apps/desktop/src/services/previewService.test.ts`: `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f`
- `apps/desktop/src/App.test.tsx`: `5fa1e1e0439690db4692033eda0ae66c98c79fb050ce2cd1214fb74c417d855c`
- `apps/desktop/e2e/r2-smoke.spec.ts`: `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17`

The fixture is runtime reference data. A new bounded Owner act covering all six paths is required before application. Subsequent acceptance must include the focused assertions, applicable crate/frontend checks, a clean full DEC-025 sweep, and a separate fresh packaged-native build binding the fixture, frontend distribution, and native bundle identities plus one isolated default-fixture native smoke/solve. Browser fallback tests cover the actual fallback path; they should not claim to force unsupported IPC failure. Prior R2 native evidence remains historical evidence only for the unchanged successful backend path.

Bindings: brief `16117f799b8c286636e507652b8ecdf17017d289396f6849793752089abed804`; amendment `7682ebba8f7dc5181c05d8ec1197f038944e68f04c7a7b54a40b788e54a7bf8f`; Fraction derivation `81b0e9a9a616d65c5fbb51a8cb8949198a64bacd13af368b23a4fbafe3455cdd`; Fraction output `4863cb9b4c0f8dda61508b54455e68d7f927fe69494f14e7d0cf5f381c470bf9`; patch envelope `5103bf895c4792989d4180a2667ec06adefe45c0247abdfe13f0a95df5c49928`; verification program `37afc53c5117b31b8981d17b31549facbeaa84f449cbd76aacb17ce753d816cd`; verification result `a291da477ba802a63c8617dc63ccf7b9a7d79f3ec9ceae6cd4e836d2d44a1116`; F4 manifest `4cd095a4a7e133a5b0dfdc290e41dea903cad6e166e2bb17a115d7988e7646b8`.

This is a technical candidate review only. No live source, test, fixture, build, Git, Owner, lifecycle, release, or acceptance state was changed.
