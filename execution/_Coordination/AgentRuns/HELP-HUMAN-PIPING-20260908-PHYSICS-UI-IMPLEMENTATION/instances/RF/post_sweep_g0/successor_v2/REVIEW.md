# RF successor V2 independent review

Verdict: `CHANGES_REQUIRED`

The V2 numeric and fixture evidence is substantially sound. All 280 public-API affine rows report `MECHANICS_SOLVED`; every L-100/L-200 row retains friction state 3, one-way state 1, two iterations, and convergence; complete Dense/Sparse payloads, including diagnostics, match. In-memory application reproduces all four declared candidate post-image hashes. The generated runtime fixture has the same 830 ordered result IDs and 31 diagnostics, and its complete recursive diff is exactly the structured inventory: two summary `value` leaves plus 338 result `value` leaves, with no other changed field.

Required corrections:

1. The V2 Rust hunk removes the old friction assertion but adds no replacement assertion, retains stale current-normal literal `48.952652`, and contains neither `0.489527` nor `48.952719` nor the required `round6(0.01 * normal)` relation. This contradicts `DIAGNOSIS_V2.md` and leaves the earlier branch-proof gap open.
2. The proposed four-path inventory omits active fixture-derived goldens. `apps/desktop/src/App.test.tsx` requires `4.927109`→`4.927112` at line 6980 and `3.977299`→`3.977301` at lines 11403 and 11827; `apps/desktop/e2e/r2-smoke.spec.ts` requires `4.927109`→`4.927112` at line 589. These values correspond to changed generated fixture rows and would remain stale after applying V2.
3. The historical-no-spring L-200 uniqueness claim is too strong. Exact rational interval arithmetic retains a valid fourth vertex anchored by the published probe intervals at `q=49.885` (`N=18.656087±0.0000005`) and `q=-49.8805` (`N=30.353500±0.0000005`). It gives an upper fixed-point normal of approximately `24.476359504314274`, so the independently possible six-decimal normal publications are `24.476359` and `24.476360`. The V2 Decimal calculation at precision 50 drops this boundary vertex. The friction publication remains uniquely `0.244764`, and the observed target normal `24.476359` remains compatible; the diagnosis and bounds record must calibrate the normal as observational rather than independently unique.

The fixture is runtime reference data. It is dynamically imported for browser runs of the canonical/no model and for native invocation failure fallback. A frontend production build can verify the web asset import, but it is not a packaged-native build. Prior native R2 evidence remains historical evidence for the unchanged successful backend path only; if packaged-native fallback asset identity or smoke is required, it must be a separate fresh gate.

Reviewed bindings: V2 brief `d2f536c8540e7bdf0be3c136df976592dbe998afbe382ae7db6e1469e7273402`; diagnosis `9c7814a14198b7840aa228abd03c701d1de542ffdcb28ff3c0da0480cf1d3bb3`; affine rows `c556a9db4ab4f15e2603ce3c50793b96dc5a7aaa8f2a08f3386707757e6ff351`; bounds `4172c62c88ec1eece8fa1784d24ad1b0bd362c20a54ace3fbae84cdc67f66e8a`; inventory `d04ce5db7266969e0ab46ef7ceabdd43e32d8a3928e13cd172a26c45b6938d87`; patch envelope `faa878e6278c5f5aac405817dca56655b1b9a07e231ee8324852e939e82595eb`; decoded patch `9210169b6968d7f319506cfb53b8d6bc71132b713fba8fa52736b2ee4bc8e8d7`; manifest `728c5e89053428e9a3a6004b248d5476ab1e68b20f32d7bc7085cef0c07195e3`.

No live source, fixture, test, build, Git, lifecycle, or release state was changed. No Owner act is ready on V2.
