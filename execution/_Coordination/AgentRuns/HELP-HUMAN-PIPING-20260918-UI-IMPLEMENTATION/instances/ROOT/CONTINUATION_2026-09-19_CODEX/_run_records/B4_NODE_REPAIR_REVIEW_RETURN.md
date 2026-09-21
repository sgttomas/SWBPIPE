**Scoped PASS — Node-review Undo repair backchecked.** No actionable findings or weakened behavioral oracles found.

The production correction is exactly the Node-review `persistentEditor` opt-in. Node direct, shared core, VirtualList, Materials/projection, engine/controller/native, and CI remain unchanged.

The recorded failure and correction align causally:

- Before repair, actual `9` changed staged `1`→`19`, moved its owning row 36px, and preserved input/focus/caret—but text Undo left `19`.
- After repair, both profiles retain actual row movement while recording zero input-host movement. Text Undo restores `1`, caret `[1,1]`, and original order. Canonical X remains `2`, model hash is unchanged, model Undo/Redo remain disabled, and Edited count stays zero.
- The earlier End-key run is correctly retained as a no-crossing setup/control observation.
- Observer adaptation identifies the same typed owning row after the input leaves its subtree. Actual keys and movement/input/focus/caret/Undo/model/history assertions remain intact; no decisive-step focus/fill rescue was added.
- DisplayIntegration now verifies one exact `aria-owns` cell, requested row/X column, and accessible name while preserving conversion, raw-draft, model, and hash assertions. Its geometry fixture is scoped to that review surface and restored in `finally`; it establishes no layout qualification.
- The footer-focus retention stress setup legitimately tests an already-retained editor. It does **not** establish that ordinary family switching always preserves an open editor.

Evidence supports **117 tests in four files**, a later overlapping **51-test DisplayIntegration rerun**, reported final TypeScript success, and **ten browser cases across both profiles**, with no skips/retries. These counts are not additive or a full final-lane run. Initial zero-layout and test-typing failures remain retained.

Verified identities:

- Reviewed delta: `3ba8b70f9cf3b89ac77a77a4a967f372df27865d..15186ef0973c0ba7f654743ed0f19913b0361c57`.
- Final evidence: `9d642e9064e852b1f5db177de13337ad797c38a4`; no desktop delta from reviewed source.
- Desktop tree: `569bb433ce914ef5f96234bc1ddfbb4a82700924`.
- Desktop binary diff SHA256: `8e0df6dc9ce3a6ede7722d9ceec7fc7ac0dd33ecb508ac5b7f7d254d637617f3`.
- All three hashes matched `SOURCE_FREEZE_2.json`, SHA256 `859e89298a5886b5fdf91f3dc547c1d9e81d4e550ed58ea76ef48e2481a1cc7c`.

Complete maintained scope, relative to `projects/chirality-piping/apps/desktop/`:

```text
e2e/b4-table-editing.spec.ts
src/features/model-tree/ModelTree.tsx
src/features/toolkit/DisplayIntegration.test.tsx
```

Diagnostic manifest verified **23 payloads / 8,777,303 bytes**, including both failed traces. Final repair manifest verified **26 payloads / 443,705 bytes**, zero mismatches; SHA256 `1f8ee1f3cfa4c21e5b2aac3cba446793ee67bc3864d9207e34b37b89baeb61d1`. Worker return, manager tail, and ROOT tail hashes matched their supplied bindings.

Actual reviewer: retained independent TASK `/root/b4_3_code_review`, Codex harness-native, gpt-6-astra/xhigh. Read-only source/diff/hash/log/JSON/trace inspection; no writes, tests, builds, UI, network, Git mutations, or descendants.

Node native verification remains outstanding. Materials native evidence remains bound to `75fa863…`. Combined review, clean sweep, actual-head CI, acceptance, and release remain separate gates.
