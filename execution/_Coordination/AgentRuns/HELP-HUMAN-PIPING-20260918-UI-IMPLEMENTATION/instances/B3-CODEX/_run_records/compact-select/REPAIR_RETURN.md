# CompactSelect independent-review repair return

Same bounded TASK Type 2 / GPT-6 Astra low, delegated-harness-native child /root/b3_manager/compact_select of B3-CODEX manager. No delegation. Worktree /Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3. Original RETURN.md and logs preserved. Instructions remain bounded writes rather than host sandbox guarantees.

First read verified sealed briefs/COMPACT_SELECT_REVIEW_REPAIR.md SHA256 `092a8debf8e809a24671ef237f1be4cae3650b1c44c943185662404722778712`. Original instruction source identities remain in basis.json. Review repair authorized through parent; only CompactSelect.tsx, CompactSelect.test.tsx and new own evidence files changed by this TASK.

## Corrected semantics

Opening establishes a provisional active value for accessibility/navigation only. No-navigation Tab, outside pointer and outside focus do not commit fallback values for unknown, empty or disabled sources. Enter/Space on the open control, pointer selection, and deliberate navigation/typeahead followed by commit establish an explicit choice. Unchanged selected value still closes without callback.

Active/pending identity is the option VALUE, not its index. Opening captures the controlled value and an order-independent semantic catalog snapshot (option value, label and disabled status). Pure reorder retains the chosen value and remaps active descendant. Any membership/label/disabled change, catalog arrival, or controlled-value change cancels without commit/focus theft. Commit synchronously rechecks the captured basis, current availability and current chosen value, so reconciliation effects are not the sole protection. The policy deliberately cancels on any semantic catalog change, even to an unrelated option, rather than assuming an old engineering choice remains valid.

No observer bridge, hidden select, polling, dependency, scope expansion or native build. Existing Escape, scope, window-blur and focus semantics retained.

## Changed product hashes

- `projects/chirality-piping/apps/desktop/src/features/workspace/CompactSelect.tsx` — `57132795584e1f105bf3277f58254753f67253bb8773e42452f97d1818d07631`
- `projects/chirality-piping/apps/desktop/src/features/workspace/CompactSelect.test.tsx` — `d5a44f6d2db86ebc7340abbf4e49ba1949c552c69781cc4b70c1fc06c67ce2af`

## Verification

Command from repository root: `npm --prefix projects/chirality-piping/apps/desktop test -- src/features/workspace/CompactSelect.test.tsx`.

- repair-unit-before.txt: 11 failed / 18 passed, before implementation correction. Failures reproduce implicit source normalization, catalog arrival, reordered index targeting and stale basis behavior.
- repair-unit-01.txt: 29/29 pass after correction.
- repair-unit-02-final.txt: 33/33 pass, including additional event-time in-place catalog removal/disable/rename/reorder before any React reconciliation. These directly test the synchronous commit guard.
- `npm --prefix projects/chirality-piping/apps/desktop exec -- tsc --project projects/chirality-piping/apps/desktop/tsconfig.json --noEmit`: exit 0, empty output repair-typecheck-01.txt.

Tests include invalid/empty/disabled-source passive dismissal, intentional Enter/pointer/navigation, current same-value closure, dynamic option reorder/removal/disable/label change, controlled-value change, empty catalog arrival and all original interaction regressions.

## Limits and return path

Component jsdom and TypeScript only. Controlled-value updates tested via React rerender; option guard also tested before reconciliation using in-place catalog mutation. No browser/native/AX/visual acceptance claim. Manager owns SupportConfiguration consumer regressions, actual candidate browser/native verification, independent review/backcheck and source integration. All original holds remain.
