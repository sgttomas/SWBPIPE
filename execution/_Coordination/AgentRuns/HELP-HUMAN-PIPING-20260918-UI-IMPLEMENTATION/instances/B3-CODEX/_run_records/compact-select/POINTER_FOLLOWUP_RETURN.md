# Pointer identity follow-up

Same TASK/parent/model/scope and instructions as REPAIR_RETURN.md. Manager explicitly requested bounded pointer-backcheck repair; no delegation, Git mutation, native/browser/UI or build. Prior returns and raw logs remain immutable.

Reproduced: before React reconciliation, reordering the shared option catalog then clicking rendered Charlie invoked Delta because pointer activation retained an index. First reverse-order test exposed a lost choice (pointer-unit-before.txt); refined enabled-value reorder decisively reproduced wrong value c→d (pointer-unit-before-02.txt). Both logs retained.

Fixed: shared activateValue records pending identity directly. Pointer handlers supply their rendered option value, never resolve their rendered index against a later catalog. Keyboard navigation resolves its current index immediately in the current catalog and hands its value to the same helper. Existing synchronous semantic-basis/disabled checks still cancel changed catalog meaning. Added keyboard reorder regression confirming navigation starts from pending value under the latest order.

Verification command: `npm --prefix projects/chirality-piping/apps/desktop test -- src/features/workspace/CompactSelect.test.tsx`: 35/35 pass, pointer-unit-final.txt.
TypeScript command: `npm --prefix projects/chirality-piping/apps/desktop exec -- tsc --project projects/chirality-piping/apps/desktop/tsconfig.json --noEmit`: exit 0, empty pointer-typecheck-final.txt.

- `projects/chirality-piping/apps/desktop/src/features/workspace/CompactSelect.tsx` — `a0c160c24da5b472c7e60c028308d4db1bdfc9675b0157e99a12be693f1767a2`
- `projects/chirality-piping/apps/desktop/src/features/workspace/CompactSelect.test.tsx` — `9000a4494ebfa6dcb4fa91911799b549a092be6f37b890daaf87663ef02e0168`

Limits unchanged: jsdom/typecheck only, no native/AX or acceptance claim. Manager owns consumer regressions, actual candidate verification and independent review/backcheck.
