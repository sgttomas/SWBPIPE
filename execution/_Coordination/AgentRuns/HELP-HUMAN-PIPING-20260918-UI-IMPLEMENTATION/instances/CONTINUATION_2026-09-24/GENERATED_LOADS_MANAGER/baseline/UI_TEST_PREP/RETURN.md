# Refresh UI tests prepared

Only new `apps/desktop/src/features/self-weight-authoring/SelfWeightRefresh.test.tsx` authored (paths relative to Piping). Existing create-flow tests and all Rust are untouched. Source/output hashes and actual parentage are in `AUTHORING.json`.

Actual production generation and batch services are used; no engine mocks. Queue callback is observed with `vi.fn`. A local clone of the invented preview fixture receives explicit OD 0.1 m, wall 0.01 m, density 1000 kg/m³ and gravity -7 m/s². The generated case and ordinary manual tip load are applied through structured operations; density and generated-magnitude edits also use actual structured operations.

Tests cover default-create retention; existing-case refresh review before queue; complete source-model hash; atomic apply with independent annular-density intensity at 2000 kg/m³ (absolute tolerance 1e-10 N/m); unchanged IDs/order/count/case metadata/unrelated cases/manual load; postqueue stale hash refusal; default modified-load block; explicit Keep choice preserving all physical JSON bytes and exact -123.456 N/m with manual marker/original provenance/decision/hash; and prepared-review invalidation after model, request epoch, selection, busy, or form-input changes with successful regeneration required.

No builds, tests, browser or native execution occurred. Parent must first build candidate generator and applier WASM under its allocated resource slot, then run the new test and retained workflows tests. Suggested from Piping root after build:

```sh
npm run test --workspace apps/desktop -- src/features/self-weight-authoring/SelfWeightRefresh.test.tsx src/features/offline-proposal-intake/workflows.test.tsx
```

Compilation and runtime outcome remain unverified. Rust connected equilibrium oracle remains parent-owned. This JSDOM/service test is not a native UI witness or practitioner acceptance.
