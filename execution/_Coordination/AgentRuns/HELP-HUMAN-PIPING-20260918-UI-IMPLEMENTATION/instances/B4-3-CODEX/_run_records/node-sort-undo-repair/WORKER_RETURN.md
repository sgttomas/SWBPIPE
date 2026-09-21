# Node review persistent-editor repair return

Authorized minimal correction complete and frozen for backcheck. Production delta is exactly one `persistentEditor` opt-in on the existing **Node review** EngineeringTable consumer in ModelTree.tsx. Node direct, shared core, VirtualList, Materials, projection, services/controllers/engine/native/CI and sort semantics are unchanged.

Base `3ba8b70f9cf3b89ac77a77a4a967f372df27865d` retains both diagnosis runs and original failing test. Final three-path binding `SOURCE_FREEZE_2.json`, SHA256 `859e89298a5886b5fdf91f3dc547c1d9e81d4e550ed58ea76ef48e2481a1cc7c`. Exact paths: ModelTree.tsx, existing e2e/b4-table-editing.spec.ts, approved additional test consumer src/features/toolkit/DisplayIntegration.test.tsx. Final source check has zero mismatches.

Retained Codex harness-native TASK `/root/b3_manager/b4_3_materials`, Astra/low, parent WORKING_ITEMS `/root/b3_manager` Astra/high. No descendants, Git mutation, production scope expansion, native/CUA, build or full suite. Earlier diagnosis/Materials/native evidence unchanged.

## Why the correction addresses the measured cause

Before: actual key9 took staged1→19 and moved the active Node row past15. Same input/focus/caret survived but its ancestor was removed/reinserted; browser text Undo stayed19. After the Node-review opt-in, the existing host leaves the actual input in a stationary table-owned DOM position while its visual box follows the sorted cell. Both profiles now show:

- owning row wrapper removed1/added1 (actual crossing retained);
- input host removed0/added0;
- same actual focused input, caret[2,2] after key9;
- actual ControlOrMeta+z restores1, caret[1,1], original row order;
- canonical X2 and identical model hash, disabled model Undo/Redo, Edited count0; test-owned drafts cleared.

This is observed before/after causal coverage, not merely a passing test. `OBSERVATIONS.json` extracts raw `BROWSER_1.json` attachments. The old failure remains at diagnostic checkpoint3ba8 and in original traces.

Observer-only test adaptation: lookup moved from `input.closest(virtual row)` to the same specifically identified Node owning row within the Node review table, because the input is now outside that row. The passive observer still counts that actual row's removal/addition and now separately counts all removed/added subtrees containing the actual input. The decisive keys, baseline/crossing values, movement precondition, input identity/focus/caret, Undo, canonical/hash/history assertions remain unchanged. No focus/fill rescue after decisive keys.

## Focused checks

- Initial four-file Node/core/display run passed117 tests (`unit-1.log`). This exercises existing raw drafts, Keep/Cancel, unit-stale editor, Queue-time current basis, filtered Queue/all-draft Clear, direct and review transitions and shared core behavior.
- Approved DisplayIntegration test adaptation binds exactly one aria-owns owner for the specific draft.id in Node review, verifies requested owning row and X-column header plus exact accessible name, and preserves all original converted-unit/rawdraft/model/hash assertions. Its strengthened run passed51 tests (`display-owner-check-2.log`). Other Node unit files required no edits or geometry fixture.
- Strengthened accessible-name assertion initially failed because JSDOM has no layout and the persistent host remained visibility:hidden. A scoped fixture now allocates only that test's exact Node-review root/grid/body/live anchor and grid client extents; other elements use original geometry, hidden ancestors use original geometry, and spies restore in finally. No broad global visible geometry or production condition. The initial failure is retained (`display-owner-check.log`). One intermediate TypeScript `this` annotation error in that test fixture is also retained; explicit HTMLElement annotation repaired it without changing runtime/oracles.
- Final TypeScript passed (`ts-final-3.log`); diff whitespace check passed.
- Ten exact collected source-browser cases passed in both configured profiles, zero retries/skips,22.4s (`BROWSER_1.log/json`): crossing/Undo2, review character-start label/X4 including Enter/dblclick and Keep/Cancel, connected raw Queue/Clear/family/filter retention2, and retained-editor hidden-family/virtual/filter/page-resize2.

New retained-host scenario keeps the review editor open at its existing footer focus boundary, then proves hidden Node layers do not intercept actual Materials label editing/Cancel or take focus on return. It exercises1000-node virtualization, pinned editor/filter threshold, header/footer hit ownership and Libraries/inert→resize→Close; input remains identical/aligned with unchanged raw text, does not steal focus, and Clear removes only the test-owned draft. Model history remains unchanged.

Reassessment: production bytes remained the same one-line opt-in throughout unit checks; only Display test strengthening followed the117 run, and its entire51-test file reran. No single fresh four-file117 run at the final test bytes is claimed. The exact ten browser cases ran against final freeze2.

## Boundaries and handoff

Shared lock/source5174, one worker, pinned Chromium148.0.7778.96; actual identity and measurement attachments retained in JSON. Exact command/collection method is in VERIFICATION_METHOD.json. Final listener5174 and shared lock absent. Source remains immutable for manager/ROOT backcheck.

No native proof or full-suite/CI/qualification/acceptance claim. Parent owns checkpoint; independent backcheck, separately granted Node native witness and final coherent sweep/CI remain required. Reversal removes only this Node-review opt-in after adopting another remedy that passes the same invariant; deferred sort remains an alternative with different view/navigation semantics, not an implemented second policy. No Node direct expansion follows.
