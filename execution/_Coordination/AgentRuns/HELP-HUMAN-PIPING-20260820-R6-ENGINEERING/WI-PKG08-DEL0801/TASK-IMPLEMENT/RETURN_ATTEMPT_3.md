# N2-I1 implementation return — Amendment 2 / attempt 3

Status: `SUCCESS / INTEGRATED REVIEW V4 FINDING REMEDIATED / ACCEPTED FOR FRESH FULL-N2 REVIEW`

`component:C-140` provenance is now removed before mechanics, input-manifest, and analysis-run construction. Every dependent evidence object is rebuilt from that same mutated model. Before report-package assembly, the test asserts the manifest's complete `model_basis.model_payload` equals the report model and explicitly contains `component:C-140` with `provenance: ""`.

Final full-N2 product/test freeze from original basis `357a58b56726feba49507534159c3fbc4656b818`:

- `apps/desktop/src/features/report/reportPackageRequest.test.ts` — `afb037581feb58e251c3a8b821b37d93ceca7f8f776b655407c4b67e253c7358`
- `core/reporting/report_package/src/wire.rs` — `4a04fbde7b50515b00d55dde7438497fa7c5fe1d433bb7fc7d9ff5ee771489e1`
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json` — `3b20700718c3e5ee87ddf242637f4bb997f38617115bb7ccc2cba89cbf51d110`

Full diff size: 303 additions, 2 deletions across the three paths. Manager rerun: focused Vitest PASS 6/6; report-package Cargo PASS 19/19 plus doc tests; Cargo fmt PASS. Child containment and basis-scoped diff check PASS. Production and fixture content are unchanged by Amendment 2.

Residuals/blockers: none for implementation. Fresh full-N2 review is released.
