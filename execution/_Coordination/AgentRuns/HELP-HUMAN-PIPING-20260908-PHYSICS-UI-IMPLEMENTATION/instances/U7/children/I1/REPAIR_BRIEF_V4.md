# U7-R2 Sealed Repair Brief V4

## Portable path anchors

- `REPO_ROOT={git rev-parse --show-toplevel}`
- `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`
- `USER_APPLICATION_SUPPORT` is resolved by the consuming platform.


## Identity, authority, and input

- Parent: `/root/native_authoring`, WORKING_ITEMS U7.
- Executor: reactivated bounded Agent 2 `/root/native_authoring/u7_i1`.
- Model/reasoning: `gpt-5.6-sol`, high. Delegation forbidden.
- Lane: `{REPO_ROOT}`.
- Frozen input manifest: SHA-256 `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446`.
- Terminal RU review: `instances/RU/REVIEW.md`, SHA-256 `51a2123ed8ada427eaa34bac2aaccd9e92372499fc738b8663bc2ef7b8b9bbc0`.
- Terminal RU return: SHA-256 `481edae2ba1fe5a0227cffab231daca0d59086b769f3c55ebdd7bc784a69b69f`.
- Findings: `RU-F1` through `RU-F5`, verdict `CHANGES_REQUIRED`.
- Root explicitly released this bounded repair after verifying all RU packet and inventory hashes.

Read the terminal RU review in full before editing. Preserve frozen V1 manifest/diff/proofs and all RU originals byte-identically.

## Objective

Repair exactly RU-F1 through RU-F5 within the existing accepted U7 behavior. Make no service, schema, API, threshold, default substitution, engineering, or scope choice.

1. **RU-F1 busy and stale publication:** Disable all route draft-mutating controls, including Cancel, endpoint modes/picks, inputs, Continue, Add, and Apply as applicable, for the full Add/Apply flight. Do not invent an enabled in-flight cancellation feature. Every still-permitted selection/model/cancel/invalidation path must invalidate the App-owned direct review/request before a delayed response can publish model, receipt, retained context, or checkpoint.
2. **RU-F2 complete apply-result binding:** Before any state publication, validate the apply response against the frozen submission and basis using the exact fields already exposed by `OperationOutcome`, `OperationBatchOutcome`, and current consumed interfaces. Single response checks include complete acceptance, operation/change/target/change-kind identity, exact diff, model-basis/hash binding, applied hash/model, validation state, and warning/blocking rejection. Batch checks include complete acceptance, batch ID/mode/status/disposition, initial model hash, structurally identical ordered submitted operations, exact ordered step identities/diffs/diagnostics, applied hash/model, and top-level warning/blocking rejection. Missing, malformed, warning-bearing, or mismatched evidence fails closed and requires a new Add; publish nothing first. Reuse pure helpers where appropriate, without editing a service/type file.
3. **RU-F3 App continuation:** Preserve Continue and set the next route's `from` to the successfully committed endpoint across the component's own accepted App model update, while still resetting for external model replacement, create/open, undo/redo, and Cancel.
4. **RU-F4 reservation and collision:** Reserve a pending target only when runtime operation kind, target object type, and nonempty ref form a valid supported discriminator. Ignore malformed entries only for reservation while preserving their untrusted stored/requeued bytes and fail-closed service behavior. Reject a new endpoint and pipe that share an ID before emitting the atomic batch.
5. **RU-F5 explicit provenance:** Initialize node, new-endpoint, and pipe provenance blank. Add stays disabled until the user enters it. Preserve the exact entered provenance through intent, review, and Apply; pointer drafting may set coordinates only.

Structural JSON equality already passes RU probes; preserve it. Preserve exact batch order, one checkpoint, zero coordinates, existing material ID, explicit units/dimensions/y-reference, node-only incomplete save/reopen, inline inspector Apply, compact 1024×768 layout, persistent canvas, and all seven consumed-interface contracts.

## Required genuine tests

Add tests inside the existing approved test files:

- App-level successful existing-end Add/Apply with Continue: next route remains armed from the applied end; material/dimensions/orientation/provenance remain; consumed ID/label/end clear; exactly one model publication, complete receipt, and checkpoint.
- App-level delayed Add and delayed Apply: busy route controls are disabled. Exercise every still-permitted selection/model/cancel/invalidation path and prove no stale review/model/receipt/retained context/checkpoint publication.
- App-level adversarial single and batch responses: incomplete acceptance; model-basis/hash mismatch; operation/batch/submitted-step/diff mismatch; top-level and step warning; missing applied hash/model as applicable. Each fails closed before any publication. Complete response emits exactly one fully bound receipt/checkpoint.
- Pure route plus App-path coverage for missing `target.object_type`, malformed reservation metadata, valid reservations, and intra-route node/pipe ID collision.
- Untouched blank provenance rejects Add for node and route; explicit entered provenance survives the accepted flow.

Do not weaken or replace existing assertions. A private RU reproduction is diagnosis only; required closures live in approved repository tests.

## Write fence

Only these nine product paths:

1. `projects/chirality-piping/apps/desktop/src/App.tsx`
2. `projects/chirality-piping/apps/desktop/src/App.test.tsx`
3. `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx`
4. `projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.ts`
5. `projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.test.ts`
6. `projects/chirality-piping/apps/desktop/src/features/model-tree/PropertyInspector.tsx`
7. `projects/chirality-piping/apps/desktop/src/features/model-tree/typedInspector.test.tsx`
8. `projects/chirality-piping/apps/desktop/src/styles.css`
9. `projects/chirality-piping/apps/desktop/e2e/linear-authoring.spec.ts`

Own run records only under `instances/U7/children/I1/**`. No other source/evidence path.

## Checks and return

Run new failing regressions first. Then run targeted route and affected App tests, full `src/App.test.tsx`, affected inspector tests, and the bounded Chromium desktop Playwright spec on final bytes. Use existing built WASM artifacts only. No Rust, wasm/native build, native launch, full harness/sweep, commit, or push. Stop after required checks pass; no optional polishing.

Return `REPAIR_RETURN_V2.md` and `REPAIR_STATUS_V2.md` with exact RU-F1..F5 closure mapping, tests/counts, changed paths, final SHA-256 values, containment/LF/whitespace checks, unresolved blockers, and browser/native disposition. Include enough exact behavior/test mapping for manager successor manifest, full base diff, V1 delta, updated seven-row proof, and same-RU backcheck.
