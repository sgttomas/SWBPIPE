# U7-R3 Sealed Residual Repair Brief V5

## Portable path anchors

- `REPO_ROOT={git rev-parse --show-toplevel}`
- `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`
- `USER_APPLICATION_SUPPORT` is resolved by the consuming platform.


## Identity and frozen basis

- Executor: reactivated bounded Agent 2 `/root/native_authoring/u7_i1`, `gpt-5.6-sol`, high reasoning, no delegation.
- Parent: `/root/native_authoring`, WORKING_ITEMS U7; manager is product-read-only.
- Lane: `{REPO_ROOT}`.
- Frozen R2 manifest SHA-256: `72d1cb5f31d7d81e92f05edd6f0b9e8b74c102cc27c898b7139e99644fb529f1`.
- Same-RU backcheck review SHA-256: `54dd95c5f4410974060430221c16bdc3bd83bbb39964ea0622fc64fdd03f8f18`.
- Same-RU return SHA-256: `044f422f3ac5fcb9cfe92f7bcc2e78c79b2ebe6ff93a9c96d836a3a0d1eaf2b4`.
- Open residuals: `RU-BC-F1` and `RU-BC-F2` only. `RU-F1`, `RU-F4`, and `RU-F5` remain closed.

Read the full backcheck review before editing. Preserve all V1, R2, and RU artifacts byte-identically.

## Objective

Repair only the two residuals using the actual existing producer contracts.

### RU-BC-F1 response binding

Distinguish the two batch hash sources:

- require the engine-created `initial_model_hash.payload_ref` to equal the actual producer contract `model:local_batch_input`, while matching the expected algorithm, canonicalization, payload scope, value, and status;
- require `submitted_initial_model_hash` to be structurally identical to the caller's complete frozen `basisHash`, including its echoed `payload_ref`.

Validate every accepted diagnostic against the existing `OperationOutcomeDiagnostic` producer shape. A non-warning diagnostic is acceptable only when its actual severity and required ID, code, message, remediation, affected-ref array, and source are valid; malformed `info` or unknown severity fails closed. Validate the receipt/boundary members required by the actual producer contract. Do not treat `schema_version="1"`, empty audit/professional boundary objects, or hand-built incomplete helpers as valid-envelope evidence.

Build valid single and batch test fixtures from actual producer-derived outcomes before adversarial mutation. Add App publication tests that independently mutate engine-created `payload_ref`, echoed frozen `payload_ref`, and a non-warning diagnostic shape, and require zero model, receipt, retained context, or checkpoint publication. Preserve every R2 rejection and complete happy path.

### RU-BC-F2 continuation ownership

Replace content/hash/ID matching as proof of own commit. Continuation may survive only when an explicit token/event produced by the accepted direct Apply request is carried through the actual App commit and returned/rendered with that committed model transition. Coincident IDs, contents, project hashes, or endpoints alone must not establish ownership.

Add genuine App tests for:

- valid own direct Apply/commit still preserving Continue and the committed endpoint;
- delayed Apply followed by external open/replacement with a different project identity but coincident pipe ID/from/to, then a valid delayed response: callback publishes nothing and continuation clears;
- external create/open/undo/redo clearing pending continuation even when route content coincides, as applicable to the existing reachable controls.

Do not weaken closed RU-F1 cancellation/busy/invalidation protection.

## Write fence and preserved behavior

Only the existing nine U7 product paths and own `instances/U7/children/I1/**` records. No service, schema, shared type, Toolbar, existing-form, Rust/Tauri, DEL-07-02, threshold, default, lifecycle, dependency, global window, selective-solve, or model-slicing changes.

Preserve exact atomic order, complete receipt binding, structural equality, one checkpoint, explicit blank provenance, runtime reservation/collision guards, zero coordinates, existing material/unit/y-reference contracts, incomplete node-only persistence, inspector behavior, compact 1024×768 layout, and persistent canvas.

## Checks and return

Run new residual tests red-to-green, then the affected route/App tests. Freeze one source cut before final full `src/App.test.tsx`, desktop `npm run build`, and bounded Chromium desktop Playwright. Run route/inspector checks if affected or required to preserve prior closure. No Rust, WASM/native build, native launch, full harness/sweep, commit, push, or optional polish.

If a later strictly erased TypeScript fix is needed, preserve the exact pre-fix source snapshot and prove emitted JavaScript identity before reusing runtime evidence. Otherwise rerun affected checks.

Return `REPAIR_RETURN_V3.md` and `REPAIR_STATUS_V3.md` with exact residual closure, producer-derived fixture basis, tests/counts, changed paths/hashes, containment/whitespace, carried closures, and native disposition.
