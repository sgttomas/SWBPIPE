# RF successor V3 independent review

Verdict: `CHANGES_REQUIRED`

The narrow V3 Rust correction passes. Decoded patch `9daf0cfb0ca9795731f691cce0f80c11a093fe5f9d93875dd34dd51ae0249e8a` applies in memory to exactly the four V2 paths. Its Rust post-image is `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5`; code outside `valid_invented_model_exposes_nonlinear_support_loop_evidence` is byte-identical. The affected test contains exactly one assertion each for friction `0.489527`, current normal `48.952719`, and `round6(0.01 * normal_evidence.value)`. Stale Rust literals `0.490101` and `48.952652` are absent, while the loop, state, sign, metadata, and diagnostic assertions remain. The fixture, Python, and desktop-service patch sections are byte-identical to V2 and reproduce V2 post-images `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`, `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735`, and `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f`.

The complete candidate still requires two corrections carried from the V2 substantive review:

1. Its active-oracle scope omits `apps/desktop/src/App.test.tsx` literals `4.927109`→`4.927112` at line 6980 and `3.977299`→`3.977301` at lines 11403/11827, plus `apps/desktop/e2e/r2-smoke.spec.ts` literal `4.927109`→`4.927112` at line 589. These are exact values of changed generated fixture rows. The candidate therefore needs six paths, not four.
2. Exact rational interval arithmetic gives historical-no-spring L-200 normal publication candidates `{24.476359, 24.476360}`. The precision-50 Decimal derivation incorrectly excludes the valid boundary vertex anchored by probes `+49.885` and `-49.8805`. Friction remains uniquely `0.244764`, and observed normal `24.476359` remains valid; only the independent uniqueness claim and bounds must be corrected.

Reusable V2 evidence remains accepted: all 280 affine rows satisfy the declared branch/state/convergence conditions with full Dense/Sparse payload equality; the generated runtime fixture has exactly two changed summary values and 338 changed result values, the same 830 ordered IDs, and unchanged 31 diagnostics; its inventory is exact. Solver and public product API bytes match the frozen inputs.

The generated fixture is runtime reference data used by browser and native-invocation-failure fallback. A frontend production build does not establish packaged-native fallback identity. Prior native R2 evidence applies historically to the unchanged successful backend path; packaged-native asset identity/smoke, if required, is a separate fresh gate.

Bindings: V3 brief `17d85f915b47b94493d80d640026cfadea7e7d5ce3b34c43c049dd342cd62c58`; correction `c0c8e2b2db5863f01dfb0d7480ab5db68327ab45be9ec214df6930081b999391`; envelope `ab4bde7f07d6f6bab9be44f5f7f8b19f2eaf8037d69f59ff6cd5333bce6257c8`; decoded patch `9daf0cfb0ca9795731f691cce0f80c11a093fe5f9d93875dd34dd51ae0249e8a`; verification `045ac51f390a9bb433dcb33aba6926fcb3c10bdf4b9504202af1911b04d067df`; F4 manifest `cfe530319d3ee9f4b9c471517a47e0981036b2a045919c7a69d9b9a0b34d17d8`.

No live file was changed, and no Owner, lifecycle, release, or acceptance act is authorized.
