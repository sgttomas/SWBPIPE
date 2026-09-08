# U7-R2 sealed repair return

Status: `REPAIR_COMPLETE_PENDING_SAME_RU_BACKCHECK`

Brief SHA-256 verified: `5f8b1a712508ffb362eec3c33c5d008dd5fdf8475800014a1c79412cba596bfe`.

Terminal RU review SHA-256 verified and read in full: `51a2123ed8ada427eaa34bac2aaccd9e92372499fc738b8663bc2ef7b8b9bbc0`.

## RU finding closure

| Finding | Final behavior | Exact repository evidence |
|---|---|---|
| `RU-F1` | One disabled fieldset covers node, endpoint-mode, endpoint-pick, pipe, Continue, Cancel, Add, and Apply controls for the whole Add/Apply flight. Creation commands and viewport target buttons are also disabled; canvas draft mutation returns immediately while busy. User selection that remains available through the model tree synchronously invalidates the App-owned epoch and registered review before a delayed response can publish. Model replacement still invalidates by revision/epoch. | App tests **disables every route control during delayed Add and discards the result after permitted selection invalidation** and **disables the route for delayed Apply and rejects its valid response after permitted selection invalidation**. Both resolve service-valid delayed responses and require zero model/receipt/context/checkpoint publication. |
| `RU-F2` | `applyResultMatchesSubmission` fails closed on malformed or incomplete apply envelopes. Single apply binds complete acceptance, mode, route, operation/change/kind/target identity, exact diff, successful validation, model-basis value/canonicalization/binding, clean diagnostics, applied model/hash, and boundaries. Batch apply additionally binds batch identity/status/disposition, semantic initial/submitted hash evidence, initial/input backend hashes, exact submitted operation order and values, trust marker, every ordered step identity/diff/validation/diagnostic, and top-level diagnostics. App recomputes the applied-model hash and canonical batch hash before any state setter. | Route tests **accepts only a completely bound successful single apply envelope** and **accepts only a completely bound ordered atomic batch apply envelope**. App parameterized adversaries cover incomplete acceptance; basis/initial/applied/batch hash mismatch; operation/batch/submission/step/diff mismatch; incomplete validation; top/step warning; missing applied hash/model. Happy paths require one bound receipt and one checkpoint. |
| `RU-F3` | A pending route continuation snapshot is captured before Apply and matched only to the component's own committed pipe in the next model. The accepted endpoint becomes the next `from`; Continue, material, unit, OD, wall, y-reference, and provenance remain; consumed ID/label/to and new-end fields clear. External model replacement and Cancel still reset. | App test **keeps an accepted App route armed from its committed end with exact entered geometry and provenance** plus existing cancellation/model-revision tests. |
| `RU-F4` | Reservation ignores malformed members unless operation kind, supported target object type, and nonempty ref are all present; stored/requeued untrusted bytes are untouched. A new endpoint and pipe with the same ID are rejected before batch emission. | Route tests **does not reserve a ref from an incomplete target discriminator**, **ignores malformed restored members only for reservation while preserving valid reservations**, and **rejects a new endpoint and pipe that share one entity ID**. Existing App malformed-null save/open/requeue tests remain green. |
| `RU-F5` | Node, new-endpoint, and pipe provenance initialize blank. Pointer placement fills only ID/label/coordinates and leaves provenance blank. Add remains disabled until explicit provenance is entered; entered provenance is carried through intent, review, apply, tree/inspector, and persistence. | Route test **requires explicitly entered node and pipe provenance**; App tests **starts node and route provenance blank and keeps Add disabled until explicit entry** and **captures viewport pointer geometry into an explicit node draft before apply**. All prior happy paths now enter and assert explicit provenance. |

## Updated seven-row consumer proof

| Dependency | Current Apply behavior | Exact tests/current behavior |
|---|---|---|
| `DAG-002-E0486` | Stable node/pipe/load/component IDs and schema paths cross the service boundary; new-end publication remains exactly `[create_node, connect_pipe_run]`. | Route exact single/batch/order/diff tests; App ordered-batch one-checkpoint test; typed inspector exact inline Apply test. |
| `DAG-002-E0487` | Frozen revision/epoch/hash and complete returned evidence gate publication. Delayed Add/Apply selection and model invalidation publish nothing. Accepted node/load state round-trips while transient review/history do not become durable truth. | Two delayed direct-route App tests, stale-review test, incomplete-node save/open test, malformed-null save/open/requeue test, and bounded Playwright save/open checks. |
| `DAG-002-E0488` | Pipe payload preserves material, OD, wall, length unit, y-reference, and explicit provenance without inventing a section. Component and section assignment flows remain service-routed. | Route exact payload test; App straight-pipe/expansion-joint/tee tests; Playwright shared-section assignment. |
| `DEP-007-02-004` | Explicit units and dimensions are preserved; missing/TBD units fail construction. | Route payload/rejection tests; App reviewed `[m]` diffs; typed inspector exact `350 -> {value:500,unit:"N"}` force intent. |
| `DEP-007-02-005` | Node/pipe provenance must be explicit and material must resolve to an existing ID; no pointer or form default asserts user provenance. | Route missing-provenance/material tests; App blank/default/pointer tests and explicit `explicit_straight_pipe_provenance` inspector assertion; Playwright explicit provenance. |
| `DEP-007-02-006` | Inline Apply forwards the exact selected user rule field without inventing code values or mutating the source model before service application. | Typed inspector rule-field Apply test; full App rule-pack missing-input/reference assertions. |
| `DEP-007-02-007` | Apply never copies or invents private rule-pack payload/checksum data; the reference-only/redacted contract remains. | Full App private-checksum/redaction assertions and typed inspector exact public modifier forwarding. |

## Test and check evidence

- Initial test-first route run: `15 tests | 2 failed | 13 passed`; exact failures were missing-object-type reservation and intra-route shared ID.
- Final route: `npx vitest run src/features/viewport/routeDraft.test.ts` — **17 passed**, 0 failed, 0.372 s.
- Required affected App repair set: `npx vitest run src/App.test.tsx -t "native straight-route Add and Apply"` — **31 passed**, 0 failed, 126 skipped, 31.58 s before the final build-only type repair; a second run after the first type repair was **31 passed**, 31.42 s.
- First broad App run after repair tests: **151 passed / 6 failed**. Exact six failures were:
  1. `queues and applies explicit node deletion through the structured operation seam` — prerequisite created-node fixture omitted node provenance.
  2. `queues and applies explicit viewport node geometry through the structured operation seam` — omitted node provenance.
  3. `queues and applies explicit straight pipe connectivity through the structured operation seam` — omitted pipe provenance.
  4. `picks straight pipe endpoints from viewport node targets before apply` — omitted pipe provenance.
  5. `creates an expansion joint on the exact selected pipe at a three-incident node and preserves every entered value` — setup pipe omitted provenance.
  6. `clears a queued tee at a three-incident node then applies the next exact user-selected roles` — setup pipe omitted provenance.
  Deletion/component assertions were preserved; only genuine happy-path provenance was supplied. Pointer drafting was separately corrected to leave provenance blank.
- Exact six plus pointer regression: **7 passed**, 0 failed, 150 skipped, 21.99 s.
- Final full App runtime cut: `npx vitest run src/App.test.tsx` — **157 passed**, 0 failed, 212.96 s.
- Final typed inspector: `npx vitest run src/features/model-tree/typedInspector.test.tsx` — **9 passed**, 0 failed, 0.931 s.
- Required desktop build first exposed five TypeScript nullable-use errors after the runtime gate. The final repair consists only of non-null assertions at already-gated uses; `!` is erased by TypeScript and does not change emitted runtime JavaScript. Therefore the 157/157 full-App runtime result remains exact for emitted behavior under root direction. The 31-test affected set also passed with equivalent redundant local narrowing before it was reduced to erased assertions.
- Final `npm run build` — **passed**, `tsc -b && vite build`, 1698 modules, 4.93 s. The existing chunk-size advisory is non-failing.
- Final `PLAYWRIGHT_WORKERS=1 npx playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop` — **1 passed**, 0 failed, 12.8 s. It exercises the 1024×768 assertion, zero body horizontal overflow, persistent canvas >300×250, in-bounds authoring card, exact Add/Apply journey, one atomic route, undo/redo, and save/open.
- `git diff --check` over the tracked fence: exit 0.
- All nine paths: zero CR bytes, zero trailing-whitespace lines, final LF present.

## R1-to-R2 delta and final nine-path hashes

| Path | R1 SHA-256 | R2 SHA-256 | R2 disposition |
|---|---|---|---|
| `src/App.tsx` | `58a2c502ac62cd94744ae24d61123da751d53b25be8d8604174a2112d55e1ee7` | `106453b26388f832b2f0cc5683e9e50bf7c2ec9fc50e0576abfd0a3ae2036155` | App request invalidation and complete apply binding |
| `src/App.test.tsx` | `3f7471c594d0551724b7ba7bfeaea2508c7abe23d4f88c678c2d25a2fbeab5b7` | `5204a848a44b75f918108a70dfae075e5c4cbc6f434403fa10ec3da6ecbd05e8` | required App regressions and explicit happy-path provenance |
| `src/features/viewport/PipeViewport.tsx` | `e28eedb99d0a6ec1f59ac1e80c119608d92c107ad3b3371619661c8eaa831db2` | `395d81f29f2a3e827e111dd5077bee30b62d7970b49ec5233abb12877b23dfdf` | busy ownership, continuation, blank provenance/pointer |
| `src/features/viewport/routeDraft.ts` | `9ee118453f0c0a9ceda55403b40412fa5bd9582f652b4d325729bd2753520dda` | `5f03c2bb8af223dca4ddf4e058c7b80676a957ea247fcea59d423fad2ad8884d` | complete response binder and reservation/collision gates |
| `src/features/viewport/routeDraft.test.ts` | `9be8876089630acd2db7bb723252f5c5548a6b11c3c176254ef0c198f215d85f` | `84bca45138fafd18d09601ae5af8f1a5e683026f954c636dcccfc396d5b07bee` | pure RU-F2/F4/F5 regressions |
| `src/features/model-tree/PropertyInspector.tsx` | `a6b63807e99e36a1194e3d01d1b91ce604e3b0478f6b29d9255971c1fb9c3a78` | same | unchanged in R2 |
| `src/features/model-tree/typedInspector.test.tsx` | `5f42c1d77a479c9cfb00cd9f98d901b7953c4bc7f900dc094e54e2a99c615712` | same | unchanged in R2 |
| `src/styles.css` | `f7a4d3fea4781e4a7cc5d6b6072360a5c95d60cbc1748a17bab898c6380b1820` | `cefbc458a1f2189268e467608dc9be0e24d6c132234d28586551cfb7a599ca26` | disabled fieldset reset without layout expansion |
| `e2e/linear-authoring.spec.ts` | `338625b6d893a2aa47804327cec57127464f4bc600c30556d4ff1946b6faa305` | same | unchanged in R2; final browser run green |

The six changed product paths are all inside the nine-path fence. Four fence paths are unchanged from R1. Concurrent Rust, decision-register, Change, and sibling run-record changes visible in the shared lane were not read as repair inputs, edited, reverted, staged, or included.

## Screenshot and native disposition

The historical `COMPACT_ROUTE_REVIEW_1024x768.png` remains pre-remediation browser visual fan-in only, with prior recorded SHA-256 `eeebc72db4f1899c216328aefef9bc75d175ebff6a3840b5d5f472e4d0ecc107`. No final-cut screenshot was requested by this successor brief. The final bounded Playwright DOM/layout witness is green. Packaged/native GUI proof remains explicitly separate for parent/root and is not claimed here.

## Handoff

Repair is complete with no unresolved implementation or test blocker. Same-RU backcheck remains the next governed step. No service/type/Rust/WASM/native source or build was touched; no Rust/WASM/native/full-harness/full-sweep command, commit, push, delegation, or optional polishing occurred.
