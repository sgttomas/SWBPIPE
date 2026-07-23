---
doc_id: R17-DEL1005-N4-COLLATERAL-PROOF
doc_kind: coordination.collateral_proof
status: pass
created: 2026-07-23
frozen_git_basis: 1f2ecc1d06375c01a409041b8380e4d65b2a9f9a
---

# Frozen collateral proof

Byte comparison against the frozen Git basis confirms these excluded
collateral surfaces are unchanged:

| Surface | SHA-256 |
|---|---|
| `core/runner/headless/src/benchmark_binding.rs` | `1dadf7636f99b9a1931e76daf28bfcf3c49e502cecef9b51c4fd351711050d39` |
| `schemas/headless_runner.schema.yaml` | `deaa474eebb07752c0e09878809fbd9a54e9d25052f9a6f0b8bb5f720ce7a182` |
| `apps/desktop/src/App.tsx` | `9db3ac5dc41861c0fc29cbcff245d5e6946adae0ca9e96318f77c5fd6667d5a6` |
| `apps/desktop/src/features/report/RenderedReportPanel.tsx` | `846accdb86db0045e34d64a57233c74ca4cde2eabb63f7062a9fba9d356d931f` |
| `apps/desktop/src/features/report/renderableReportInput.ts` | `041621201bc438c0323cd0219e2cbd862cc801128186da051f698d5e9e046826` |
| `apps/desktop/src/features/report/reportPackageRequest.ts` | `173d832c58b6f79d4f07ea9819196d8e552fcaeccef6cac6932f33727fcf7d6c` |
| `apps/desktop/src/features/report/reportRedactionProjector.ts` | `743acc2aaa9655266f2bd6c9c83961e08849372cfe4d29f7d5db5a88b9ecf504` |
| `apps/desktop/src/features/report/stateComparisonHandoffSections.ts` | `197c62814de74f629e5d6d8e2a91d4b759f2aa0b885eda9da3bc44f08b601c8d` |
| `apps/desktop/src/services/reportPackageSaveService.ts` | `396f6343a7d59a693f40b523f72fca2543dd046bcbcac814f404b370752b2dec` |
| `apps/desktop/src-tauri/src/atomic_report_package_save.rs` | `9130f8bc06ea1437127de2e4ef59b6265ecd250f83f580f5d1c27500f7a46b0b` |
| `apps/desktop/src-tauri/src/lib.rs` | `d22904928064e011092f6e428c05f124896551c2a5a0869b72e8a553a3a359f7` |
| `apps/desktop/src-tauri/Cargo.toml` | `6357871abf8a1d95a9e2ec1aca28e190d356ef6197c8f1f4080086131740f7c1` |
| `core/reporting/report_package/tests/container.rs` | `3ea39a03f3c721e09409fd1d5a09cad75d6ad90d89f76d39828b75ed359d6ff4` |

All five frozen R12 `del1005_payload_binding_*` inputs and all five generated
outputs are also byte-identical to the frozen basis. Their hashes are:

```text
inputs:
aa3b929c1640cbba91625d1520ff3eaaf9d27a19aced5aae2b88497fe5f5a1c9
061f93c1280de67437158ccbcf8124468c3d8e0784c05085345539ed356e5085
f030431555bc424bcdecc5500e5c8315edac92d23f6dc6f1c48cd13c48764ed4
008740d68b856d27ffe5cbd43505cacc748ff9b349eb6d2773bb31c22bd4074e
19f5e715f8d86b8632dbd97b82a5294085436b6fc024ced874a3e0ce839e9b94

generated:
8feb3d25e50e78dcd7fcc85e2253021610faa971a37837eefb63df5cea456d68
9596c052c76a178e13bcf29faa5841848df6d9453983b184ebff3fd5fc2449a4
813a702b74be74a88755626d5b4530716d4fd5e27a1b988e48f3da3be3306728
2f89adce9e4d6250280cee347822a567f4405eafbb8bc666483c6ce4cbd87593
61cba4f28bcf109510489125b1de11e44796a25285ca64bf9c0714e870d9f518
```

The only `core/reporting/report_package/src/lib.rs` change is exporting
`pub mod wire;`; the assembler implementation and fixed member vocabulary are
unchanged. The stable five-verb mapping is covered by the passing runner unit
test, and all pre-existing benchmark/regression tests pass.
