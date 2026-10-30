# Post-Closure Verification — Operation-Seam Unification Plan (2026-06-11)

**Epistemic status:** independent verification record (assessment, non-governing). An independent session re-ran all test surfaces and audited [PLAN_2026-06-11_operation_seam_unification.md](PLAN_2026-06-11_operation_seam_unification.md) T1–T4 against their tranche contracts and §4 exit criteria after the executing agent reported completion. No lifecycle, release, professional, certification, or code-compliance claim.

## Verdict

**Accepted in substance.** All four tranches conform to the plan; the engine is unified (no TS engine remnants found by search); the ADR-0001 ↔ DEC-020 ↔ D-13 chain is recorded and cross-consistent; the corpus, provenance, route receipts, explicit `WASM-ENGINE-ASSET-ABSENT` diagnostic, gitignored wasm artifacts, pointer add/remove lifecycle, and the T1 rider all check out. T1's result that the TS mirror needed **zero alignment fixes** against the 44 Rust-reference outcomes confirms unification landed before drift materialized.

## Independently re-run evidence (this verification session)

| Surface | Result |
|---|---|
| Cargo sweep (25 manifests) | 446 passed / 0 failed (incl. corpus runner with programmatic coverage-floor enforcement) |
| Desktop Vitest | 140/140 (7 files; wasm engine in jsdom confirmed) |
| Python | 342/342 |
| Desktop production build | green, index chunk at baseline |
| Playwright e2e | **FAILS at the configured 30s timeout** (33.4s, reproduced twice, once fully isolated); **passes at 43.9s** with `npx playwright test --timeout=90000` |

## Open findings (input to the next tranche)

1. **F-1 — Playwright timeout margin (regression; blocks the default `npm run test:e2e:desktop`).** The T4 spec extension is logically correct (engine-ready wait; wasm-route apply assertion correctly sequenced last because applying clears solve results), but spec runtime grew from ~4s pre-seam to ~34–44s while `apps/desktop/playwright.config.ts` still sets `timeout: 30_000`. Fix: raise the test timeout (~120s) or split the spec; additionally investigate *why* runtime grew ~10× (the engine-ready wait alone should not cost ~30s). The T4 "Playwright 1/1" evidence was presumably a just-under-budget run; it is not reproducible at the default budget.
2. **F-2 — Raw NUL bytes in `apps/desktop/src/services/operationContractCorpus.test.ts`** (two literal `0x00` bytes used as dedup-key separators, offsets ≈3977+). Git classifies the file as binary, making its diffs unreviewable. Replace the literal bytes with the `\u0000` string escape; behavior is unchanged.
3. **F-3 — Corpus review disposition (human).** `fixtures/model_operations/contract_corpus/README.md` records `review_status: pending human review` — the maintainer disposition on the invented fixture set remains with the human project authority (T1's recorded residual).
4. **F-5 — Residual TS mirrors of Rust logic outside the seam plan's scope (added on follow-up semantic audit, 2026-06-11).** The operation engine is fully unified (semantic audit: no `OP-*` diagnostic synthesis, no engine tables, no validation verdicts written in TS outside tests; UI writes only honest pre-validation `not_run`/`not_applied` initial states and declarative intent metadata the engine re-validates). However, two pre-existing, smaller mirrors remain: (a) `apps/desktop/src/services/projectService.ts` `evaluateModelDocumentLocal` (~60 lines) — a self-described "browser-preview mirror of the backend DEC-019 evaluation" duplicating semver compare and refusal semantics from `apps/desktop/src-tauri/src/model_document_migration.rs`; deliberately a stricter refuse-non-current subset, lower stakes (browser store is preview memory), but the same defect class with no cross-engine test; (b) `apps/desktop/src/services/hashService.ts` (~93 lines) — sorted-keys canonical JSON hashing in TS paralleling the Rust canonicalization; partially alarmed by the corpus canonical-hash parity check on the operation path. Disposition: candidates for the same wasm-channel unification now that the T3 toolchain exists (the DEC-019 evaluation would need relocation from the Tauri crate into a wasm-compilable crate) — route to the revised development plan, not the F-1/F-2 repair tranche.
5. **F-4 — Operational note (no code change required now).** Do not run the cargo sweep and the desktop suites concurrently: one concurrent Vitest run flaked under core saturation, and `build:wasm` rewrites `src/services/wasmEngine/__generated__/` in place while Vitest/dev-server may read it. Sequential evidence runs (the loop's normal pattern) are fine. If parallel CI lanes arrive with `D-05`, give the wasm build an atomic temp-write-and-rename.

## Recommended next tranche

One bounded regression-repair tranche (loop repair lane, ahead of new scope per the loop's own ordering): fix F-1 and F-2, re-run the full evidence set, record SMOKE/run-record evidence. F-3 is a human action; F-4 is a standing note for `D-05` preparation. After the repair lands, ordinary selection resumes per the seam plan's §9 deferral list and the completion plan, pending the revised development plan.
