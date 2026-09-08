# U7-R3 sealed residual repair return

Status: `REPAIR_COMPLETE_PENDING_SAME_RU_BACKCHECK`

- Brief SHA-256 verified: `3732433d8673b6136ce6592c2b12ad3c6ed2cb91263a4322688bdb5bb4968856`.
- Same-RU backcheck review SHA-256 verified and read in full: `54dd95c5f4410974060430221c16bdc3bd83bbb39964ea0622fc64fdd03f8f18`.

## Residual closure

| Finding | Closure | Evidence |
|---|---|---|
| `RU-BC-F1` | Apply binding now requires producer schema `0.1.0`, exact single deliverables, complete professional/audit boundaries, and complete `info` diagnostics. Batch binding separately requires the engine-created initial hash to carry `payload_ref=model:local_batch_input` and the submitted initial hash to be structurally identical to the complete frozen basis hash. | Valid single/batch fixtures are actual `applyModelOperation` / `applyOperationBatch` outcomes created from the loaded model and computed basis hash, then cloned before adversarial mutation. Route tests reject wrong engine/echoed payload refs, malformed info, incomplete boundaries, schema, identities, diffs, validation, acceptance, hashes, model, and warning diagnostics. App cases independently reject engine payload-ref, echoed payload-ref, and malformed-info mutations with zero model, receipt, retained context, or checkpoint publication. |
| `RU-BC-F2` | App carries the accepted frozen review ID as an explicit commit token only for the direct Apply model commit. `PipeViewport` continues a route only when that token equals its pending continuation token; pipe ID/from/to content no longer establishes ownership. Every other model commit clears the token. | Own direct Apply preserves Continue and the committed endpoint. Undo and redo clear continuation. A delayed Apply followed by opening a different project containing a coincident pipe clears continuation; the later producer-valid response publishes nothing. |

`RU-F1`, `RU-F4`, and `RU-F5` remain closed: busy/invalidation and stale-response protection is unchanged; malformed restored records remain visible but do not reserve IDs unless their discriminator is complete; explicit provenance remains required and pointer placement does not synthesize it.

## Test-first and final evidence

- Producer-derived receipt regression run: **15 passed / 2 failed**; the two complete-envelope tests exposed permissive binding. Final route test: `npx vitest run src/features/viewport/routeDraft.test.ts` — **17/17 passed**.
- Initial App continuation regression: **26 passed / 1 failed**; the coincident external model incorrectly preserved Continue under content matching.
- Fixed affected App set: **28/28 passed**; focused own/external continuation: **2/2 passed**.
- Final adversarial publication assertions, including zero retained context: **26/26 passed**.
- Final exact-cut full App: `npx vitest run src/App.test.tsx` — **162/162 passed**, 237.84 s.
- Final exact-cut desktop build: `npm run build` — **passed**, `tsc -b && vite build`, 1,698 modules, 1.79 s Vite build. Existing chunk-size advisory was non-failing.
- Final exact-cut browser: `PLAYWRIGHT_WORKERS=1 npx playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop` — **1/1 passed**, 13.7 s, including 1024×768 canvas/card and Add/Apply journey.
- `git diff --check` over the nine-path fence passed before the final two assertion-only lines; those added lines use LF endings and contain no trailing whitespace. R3 changed only five paths inside the nine-path fence and these two own run records.

## Final nine-path hashes

| Path | SHA-256 | R3 disposition |
|---|---|---|
| `apps/desktop/src/App.tsx` | `a9217c959bfc2979e21cde5d5baf865f75258f03b902ecdd2c0fd545e5bfbde9` | explicit direct-draft commit token |
| `apps/desktop/src/App.test.tsx` | `629037c503c6cf9b32ef3cc6b59e3c04234ec2c8ad61879e6ba73ef43cd3a1be` | producer mutations, coincident-open, undo/redo, zero-context assertions |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `ac3789a6b4e8c452fe398d062183b25d884f55f5763af877ac6f1b3d4cb9f2ad` | token-owned continuation; content matcher removed |
| `apps/desktop/src/features/viewport/routeDraft.ts` | `cb258db1ab0750b891a023c0723cfbfdb55a898bb4930d4aaf4137cc6992c7a4` | strict producer receipt/hash/diagnostic/boundary binding |
| `apps/desktop/src/features/viewport/routeDraft.test.ts` | `9de95189460a18bc67e231308c641978f38ee1d971f9e6c6343e0b5969d94633` | producer-derived valid outcomes and mutations |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `a6b63807e99e36a1194e3d01d1b91ce604e3b0478f6b29d9255971c1fb9c3a78` | unchanged from R2 |
| `apps/desktop/src/features/model-tree/typedInspector.test.tsx` | `5f42c1d77a479c9cfb00cd9f98d901b7953c4bc7f900dc094e54e2a99c615712` | unchanged from R2 |
| `apps/desktop/src/styles.css` | `cefbc458a1f2189268e467608dc9be0e24d6c132234d28586551cfb7a599ca26` | unchanged from R2 |
| `apps/desktop/e2e/linear-authoring.spec.ts` | `338625b6d893a2aa47804327cec57127464f4bc600c30556d4ff1946b6faa305` | unchanged from R2 |

The historical 1024×768 screenshot remains browser visual fan-in from the earlier cut and is not rebound to R3. Packaged/native GUI proof remains separate for parent/root; it is not claimed here. No service, shared type, Rust, WASM, native, or out-of-fence product source was edited. No Rust/WASM/native/full-harness/full-sweep command, delegation, commit, or push occurred.
