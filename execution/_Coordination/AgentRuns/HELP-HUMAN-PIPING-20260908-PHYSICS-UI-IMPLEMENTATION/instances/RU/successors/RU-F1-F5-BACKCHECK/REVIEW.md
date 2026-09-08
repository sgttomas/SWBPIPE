# RU successor backcheck

**Verdict: CHANGES_REQUIRED.**

The sealed backcheck brief matches SHA-256 `951051bf9a4d78b21795d05db82dc64a014fdfd35673180ab0d5967a8f9f55ab`. I reviewed all 1,102 lines and all six paths of `U7_V1_TO_SUCCESSOR.delta`, traced the affected current source and consumed contracts, independently rehashed all 26 successor-manifest entries, and exercised the final bytes. `SUCCESSOR_MANIFEST_V2.json` remains `72d1cb5f31d7d81e92f05edd6f0b9e8b74c102cc27c898b7139e99644fb529f1`; the full base diff remains `8c01ab323ef27836ecb1e777366b9166adfe28f9948bd990b01441fc2302f30e`; the V1-to-successor delta remains `09881c4a563bd919a52bd855fc954c798565be9d8817dabfbf36a32793f83863`.

## Actionable production defects

### RU-BC-F1 — Apply still accepts hash evidence with the wrong payload identity and malformed diagnostics

`routeDraft.ts:270,276` sends both batch hash-evidence objects through `modelHashMatches`, but `routeDraft.ts:356-363` checks only that `payload_ref` is some nonempty string. The two fields have different concrete producer contracts: `atomic_batch.rs:18-24,174` constructs `initial_model_hash` with `payload_ref="model:local_batch_input"`, while `atomic_batch.rs:179` returns `submitted_initial_model_hash` as an exact clone of the caller's frozen claim. The binder therefore must not compare the engine-created reference to the UI project ID, but it must require the engine-created reference and the echoed frozen reference appropriate to their distinct origins.

An actual prebuilt-WASM batch outcome had those expected references and passed. Changing the engine-created reference to `model:unrelated-engine-payload` or the echoed frozen reference to `project:unrelated-frozen-claim` independently still returned `true` from `applyResultMatchesSubmission`. This is not the invalid criterion of requiring both references to equal each other.

The same binder treats any diagnostic object with a string severity other than `warning` or `blocking` as clean (`routeDraft.ts:373-379`). An otherwise valid actual outcome with `diagnostics=[{severity:"info"}]` therefore passes despite missing the required diagnostic identity, code, message, remediation, affected references, and source. The repository's purported complete-envelope helper also uses `schema_version="1"` and empty boundary objects (`routeDraft.test.ts:70-100`), so its green result does not prove strict response shape.

`App.tsx:1072-1107` relies on this binder before creating receipts, a checkpoint, and publishing the batch. Its later applied-model and batch-value hash recomputations do not check either payload reference or diagnostic shape. Repair by binding `initial_model_hash.payload_ref` to its engine-owned contract, comparing `submitted_initial_model_hash` structurally with the frozen `basisHash`, and validating every accepted diagnostic and other receipt member to the actual emitted contract. Add adversarial App tests for both distinct references and a malformed non-warning diagnostic. Existing missing-acceptance, warning, diff, order, step, validation, applied-hash, and batch-hash rejection checks are otherwise effective.

### RU-BC-F2 — A coincident external replacement is mistaken for the component's own commit

During Apply, `PipeViewport.tsx:625-638` stores a pending continuation before the App callback returns. Every new model then reaches `PipeViewport.tsx:301-316`; `committedRouteMatches` at `PipeViewport.tsx:1605-1610` classifies the update as the component's own commit using only pipe ID, `from`, and `to`.

The affirmative probe began a delayed valid Apply, then supplied an external replacement model with a different project ID that happened to contain the same frozen pipe ID/endpoints. The App/component invalidation prevented the delayed callback from publishing, but Continue remained checked and `from` advanced to the endpoint. This violates the required external-replacement reset even though RU-F1's no-publication protection holds.

Repair by recognizing continuation only from the actual accepted request/commit event, using request ownership carried through the successful App callback/commit. Matching IDs, matching model content, or a matching model hash cannot distinguish that event from a user opening or replacing a model. External open/create/undo/redo updates must continue to clear pending continuation even when route values coincide. Add a delayed-Apply/open-replacement regression with the same route identity, plus the existing own-commit continuation case.

## Finding closure

| Original finding | Backcheck result | Evidence |
|---|---|---|
| `RU-F1` | **Closed** | One disabled fieldset covers route controls; command buttons and canvas targets also gate on busy. Permitted selection synchronously increments the App epoch. The delayed Add and Apply tests publish no model, receipt, or checkpoint. |
| `RU-F2` | **Still open** | Most binding and warning cases now fail closed, but `RU-BC-F1` proves incorrect hash references and malformed non-warning diagnostics still pass the publication binder. |
| `RU-F3` | **Still open** | Own App publication preserves requested continuation and ordinary Cancel/replacement/history paths clear it, but `RU-BC-F2` proves a colliding external replacement is falsely preserved. |
| `RU-F4` | **Closed** | Runtime reservation requires a supported kind, supported target object type, and nonempty string ref; malformed saved members remain untouched for authoritative rejection; intra-route node/pipe ID reuse is rejected. |
| `RU-F5` | **Closed** | Node, endpoint, pipe, and pointer provenance start empty; Add remains disabled until explicit entry, and the explicit value reaches the route payload. |

Structural equality remains object-key-insensitive and array-order-sensitive. Route, persistence, inspector, and final browser checks support the seven consumer fields; `DAG-002-E0487` cannot be closed while the coincident external-replacement case remains. This is consumer evidence only and is not dependency-row acceptance.

## Evidence gaps and limits

The repository adversarial tables do not mutate either `payload_ref`, do not supply a malformed non-warning diagnostic, and do not open a replacement containing the frozen route identity. Those omissions explain why the focused repository suites remain green.

Seven final-source non-null assertions in `App.tsx` are TypeScript-only. A scoped `transpileModule` comparison confirms equivalent JavaScript with and without those assertions, and the affected 32 App cases were rerun on final bytes. No pre-assertion source snapshot exists in the sealed packet, so the claimed historical timing of the 157-test run is not independently byte-verifiable; the final-byte affected rerun avoids relying on it for this backcheck.

The final Chromium journey passes at exactly 1024×768 with no body horizontal overflow, an in-bounds authoring card, canvas larger than 300×250, reachable Add/Apply controls, exact route creation, undo/redo, section assignment, and save/open. It used the existing prebuilt WASM service. No native-app claim is made.
