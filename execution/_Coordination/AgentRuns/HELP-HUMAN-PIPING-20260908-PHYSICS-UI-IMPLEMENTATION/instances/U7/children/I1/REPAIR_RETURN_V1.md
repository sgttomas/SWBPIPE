# U7-R1 sealed repair return

Status: `REPAIR_COMPLETE_PENDING_PARENT_VALIDATION`

Brief SHA-256 verified: `61d20df1a043382e57ab6a55bf5e6d5c5a0cb3bca53894586eda98f1eabb99b0`.

The manager candidate hashes in `instances/U7/DIRECT_INTEGRATION_DEVIATION_V1.md` matched the live nine-path candidate exactly before repair. The four App failures are resolved without weakening the frozen route, atomicity, persistence, or untrusted-input requirements.

## Manager candidate dispositions

1. **Exact validation diff and submitted-operation binding — accepted in intent, corrected in implementation.** The candidate correctly required clean validation status, exact step order/IDs/change IDs, exact one-row diffs, and an exact returned submitted-operation list. Its `JSON.stringify(left) === JSON.stringify(right)` operation-list comparison was not valid across the real WASM boundary because the returned JSON object keys were reordered while all values and array order remained identical. Final code compares JSON values structurally: object key order is irrelevant, array order remains exact, and every field/value must match. The regression test now deliberately reorders operation-object keys before accepting the exact list, then mutates a submitted operation and requires rejection.
2. **Queued-batch operations as route ID reservations — accepted in App, corrected at the viewport trust boundary.** `App.tsx` remains byte-identical to the manager candidate and continues to pass all queued batch members to the viewport reservation projection. The candidate dereferenced those members as trusted `EditorOperationIntent` objects, crashing on preserved `null` evidence. Final `PipeViewport.tsx` declares the reservation-only input as `ReadonlyArray<unknown>`, and `routeDraft.ts` reserves only a runtime-valid create/connect/insert target with a nonempty string ref. The stored/requeued batch is not filtered, rewritten, or sanitized.
3. **No-callback atomic-route fallback — accepted.** The candidate correctly refuses to split a new-end `[create_node, connect_pipe_run]` batch into independently queued operations when the review callback is absent. This behavior is retained; the overall `PipeViewport.tsx` hash changed only for the explicit untrusted reservation-input type.
4. **Permanent Playwright source must not write historical evidence — accepted byte-identically.** `linear-authoring.spec.ts` remains at candidate SHA-256 `338625b6d893a2aa47804327cec57127464f4bc600c30556d4ff1946b6faa305`. It no longer writes into the historical I1 run directory. The earlier screenshot remains unchanged and is separately bound below.

## Four failure causes and fixes

The two route failures were:

- `validates and applies a new endpoint as one ordered batch with one checkpoint`
- `publishes neither batch member and creates no checkpoint when Apply is blocked`

Both failed before review creation. Diagnostic inspection of the actual browser/WASM service response confirmed that it returned the full `submitted_operations` list, `submitted_operations_trust=untrusted_submitted_metadata_not_validation_evidence`, and exact per-step diff rows. The service JSON parser reordered object keys, so the candidate's order-sensitive string comparison rejected value-identical submitted operations. Structural JSON equality fixes the false rejection while still binding array order and all operation fields. No test mock or service contract was loosened.

The two adversarial failures were:

- `retains a malformed restored member as untrusted data and rejects it without crashing or accepting`
- `handles offline null through queue, validation, apply, save, reopen and explicit requeue as untrusted context`

Both crashed when the new reservation projection read `operation_kind` from a preserved `null` batch member. Reservation now treats members as unknown and ignores malformed/non-object/non-reservable members only for collision detection. The original `null` remains in retained context, is requeued unchanged, reaches validation/apply as untrusted input, is rejected fail-closed, survives save/open, and never becomes accepted state. A new route unit test covers null, scalar, empty object, invalid target, non-string ref, and irrelevant operation kind, then proves valid batch operations still reserve both IDs.

## Final verification

- Four affected App tests: `4 passed | 128 skipped`, 6.41 s.
- `npx vitest run src/features/viewport/routeDraft.test.ts`: `12 passed`, final-cut result 0.353 s.
- `npx vitest run src/App.test.tsx`: `132 passed`, final-cut result 184.00 s. An earlier repaired-byte full run was also green at 191.91 s before the final trust-boundary type annotation and JSON primitive comparison alignment.
- `npx vitest run src/features/model-tree/typedInspector.test.tsx`: `9 passed`, 0.969 s.
- `PLAYWRIGHT_WORKERS=1 npx playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop`: `1 passed`, 12.1 s.
- `git diff --check` over tracked fence members: exit 0.
- Explicit trailing-whitespace scan over all nine product paths: no findings.

The inspector and Playwright results are reused for the exact cut under root direction. After those green runs, the only final delta was (a) the TypeScript-only `reservedIntents` annotation from `EditorOperationIntent[]` to `ReadonlyArray<unknown>`, which erases at runtime, and (b) `Object.is` to `===` for JSON primitive equality. Final-cut route tests exercise the latter, including the reordered submitted-operation regression, and final-cut full App tests exercise both normal route publication and preserved malformed batch handling. No inspector code, styles, Playwright source, layout behavior, normal browser operation value, or service effect changed after the green inspector/Playwright runs.

## Final nine-path hashes

```text
58a2c502ac62cd94744ae24d61123da751d53b25be8d8604174a2112d55e1ee7  projects/chirality-piping/apps/desktop/src/App.tsx
3f7471c594d0551724b7ba7bfeaea2508c7abe23d4f88c678c2d25a2fbeab5b7  projects/chirality-piping/apps/desktop/src/App.test.tsx
e28eedb99d0a6ec1f59ac1e80c119608d92c107ad3b3371619661c8eaa831db2  projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx
9ee118453f0c0a9ceda55403b40412fa5bd9582f652b4d325729bd2753520dda  projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.ts
9be8876089630acd2db7bb723252f5c5548a6b11c3c176254ef0c198f215d85f  projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.test.ts
a6b63807e99e36a1194e3d01d1b91ce604e3b0478f6b29d9255971c1fb9c3a78  projects/chirality-piping/apps/desktop/src/features/model-tree/PropertyInspector.tsx
5f42c1d77a479c9cfb00cd9f98d901b7953c4bc7f900dc094e54e2a99c615712  projects/chirality-piping/apps/desktop/src/features/model-tree/typedInspector.test.tsx
f7a4d3fea4781e4a7cc5d6b6072360a5c95d60cbc1748a17bab898c6380b1820  projects/chirality-piping/apps/desktop/src/styles.css
338625b6d893a2aa47804327cec57127464f4bc600c30556d4ff1946b6faa305  projects/chirality-piping/apps/desktop/e2e/linear-authoring.spec.ts
```

Only the nine approved product paths are dirty under `apps/desktop`; no additional product path was introduced. R1 product edits affected `PipeViewport.tsx`, `routeDraft.ts`, and `routeDraft.test.ts`; `App.tsx` was temporarily instrumented only to inspect the real returned service envelope and was restored byte-identically to the manager candidate before verification and hashing. Concurrent Rust, decision, Change, and sibling run-record changes in the shared lane were left untouched.

## Pre-remediation screenshot provenance

The retained `COMPACT_ROUTE_REVIEW_1024x768.png` is pre-remediation visual evidence. Its SHA-256 is `eeebc72db4f1899c216328aefef9bc75d175ebff6a3840b5d5f472e4d0ecc107`; filesystem mtime is `2026-09-08T07:17:18-0600` and size is 114,773 bytes. It was captured and inspected against these exact nine source hashes recorded in the prior I1 command output immediately after capture:

```text
7edfdea979cb84a9da46376b0d438864829d3aad710a762eba1190381d6f0703  src/App.tsx
3f7471c594d0551724b7ba7bfeaea2508c7abe23d4f88c678c2d25a2fbeab5b7  src/App.test.tsx
3c543e3d8da104b6e0382699b8e17cd99f31c76f4529a9f85789448740ea3d2b  src/features/viewport/PipeViewport.tsx
ecf4ee08e59daa17f2562499bdb57b6a0ef059e0a65a8fc6b9e2aa437c163a14  src/features/viewport/routeDraft.ts
aeb13edb28ee265ce332f4b6f8adc90304255bba76bc9a54db58d953dcd01ae8  src/features/viewport/routeDraft.test.ts
a6b63807e99e36a1194e3d01d1b91ce604e3b0478f6b29d9255971c1fb9c3a78  src/features/model-tree/PropertyInspector.tsx
5f42c1d77a479c9cfb00cd9f98d901b7953c4bc7f900dc094e54e2a99c615712  src/features/model-tree/typedInspector.test.tsx
f7a4d3fea4781e4a7cc5d6b6072360a5c95d60cbc1748a17bab898c6380b1820  src/styles.css
af7f07e4ef0e4b2d41cf5f72db4f85ce65157d905eb69ca4f5cee2fe022a44a5  e2e/linear-authoring.spec.ts
```

That screenshot is preserved as historical browser visual fan-in only. It is not evidence for the final source hashes, not a packaged/native GUI witness, and not a substitute for final-cut native visual proof.

## Handoff

Exact handoff state: remediation complete; final-cut route and App tests are green; applicable inspector and bounded Playwright evidence is green with exact-cut delta reasoning recorded; exact nine-path containment and whitespace checks are clean; no unresolved blocker. Final-cut packaged/native visual proof remains separate parent/root work. No Rust, WASM build, native build/run, full harness, full sweep, commit, push, or delegation occurred.
