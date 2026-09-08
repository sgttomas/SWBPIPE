# U7-R2 Repair Brief V3 Draft

## Portable path anchors

- `REPO_ROOT={git rev-parse --show-toplevel}`
- `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`
- `USER_APPLICATION_SUPPORT` is resolved by the consuming platform.


Status: `NOT_DISPATCHABLE`; succeeds `REPAIR_BRIEF_V2_DRAFT.md`. Final RU bindings and root release are absent.

## Intended executor and boundary

Bounded Agent 2 using `gpt-5.6-sol` with high reasoning and no delegation. Use only the existing nine U7 product paths and the executor's own run records in `{REPO_ROOT}`. Frozen input manifest SHA-256: `9f71f16fe91112a42ed01fc69aee040665c4c1663d3e9d5a02d8df00747f4446`. RU terminal packet/hash/finding IDs: `TBD_PENDING_RU_TERMINAL`. Root repair release: `NOT_GRANTED`.

## Intended objective

Repair only terminal RU findings while preserving atomic route Apply, stale/revision/hash protections, structural equality, incomplete node-only save/reopen, units/material/provenance contracts, inspector behavior, compact 1024×768 layout, and the persistent canvas.

Required test-first cases, subject to exact terminal RU wording:

1. App-level successful existing-end Apply with Continue retains the next start at the applied end, preserves material/dimensions/orientation/provenance, clears consumed identity/end fields, and publishes one complete operation/receipt/checkpoint.
2. App-level pending Add/Apply disables implicated draft-mutating controls. Every still-permitted cancellation/invalidation route makes the callback fail closed with no stale model, receipt, retained context, or checkpoint publication. Do not create a new enabled in-flight Cancel feature.
3. App-level single Apply with incomplete terminal-defined acceptance or model-basis evidence fails closed before any publication. A complete outcome produces exactly one receipt with fields bound to the submitted intent and service response. Do not infer the exact required evidence until the terminal RU packet is sealed.
4. Pure route and App-level inputs reject a new node ID equal to the new pipe ID and emit no review/batch.
5. Reservation ignores null, non-object, incomplete target, missing `target.object_type`, invalid ref, and irrelevant operation metadata only for collision detection; valid create/connect/insert operations still reserve IDs; retained untrusted evidence remains byte-preserved and service-rejected.
6. Node and pipe provenance initialize blank. Add is disabled until the user explicitly enters provenance; the entered string is preserved through intent, review, and Apply.

No service/type/Toolbar/existing-form/Rust/Tauri/DEL-07-02/schema/threshold/lifecycle/dependency/global-window edits. No new default provenance. No weakening of warnings, diffs, IDs, batch order, or acceptance checks.

Run new failing regressions first, then affected final App/route/inspector/Playwright checks justified by actual deltas. No Rust, wasm/native build, full harness, full sweep, commit, or push. Freeze after required checks pass and return exact hashes, successor diff, counts, containment, blocker and native-witness disposition.
