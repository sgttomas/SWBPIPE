# Eight-file affected consumer run

Seven suites passed. The App suite passed 218 of 219 cases, giving 297 passing cases and one failure overall. The remaining failure is an old canonical export version assertion in the long diagnostics/export test: the genuine current source emits the selected 0.3 derivative while the test still expects 0.2. The following old derivative/stress-neutral assertions require applicability review before a focused rerun.

The driver stopped after Vitest failure; TypeScript did not run. All recorded source/manifests/locks and the eight current WASM assets remained unchanged. The earlier eleven-failure run remains preserved separately. See `_run_records/STAGES.json`, `INPUTS.json`, `vitest.log`, `WASM_REUSE_BASIS.json` and the executed driver for exact bindings.

This outcome validates those passing cases only. It does not establish the unresolved long test, TypeScript, the complete ordinary cut, native workflows or engineering correctness.
