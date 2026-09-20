# CompactSelect bounded return

Implemented compact select-only trigger/body portal and component tests. No acceptance claim.
Parent: B3-CODEX WORKING_ITEMS; TASK Type 2, GPT-6 Astra/low, delegated-harness-native; no delegation or Git mutation. Actual source origin/hash receipt in basis.json. Only the two authorized new product files and this evidence directory were written.

## Product bytes

- `projects/chirality-piping/apps/desktop/src/features/workspace/CompactSelect.tsx` — `a8bddb710c9c7c33cc1cffe3af2df813150aa80d4fe0b9b76471442e78e97d8c`
- `projects/chirality-piping/apps/desktop/src/features/workspace/CompactSelect.test.tsx` — `87abddde4ea6b49e1db85471a3d2f95cfc565dc939cc64ab4bb879d42f432817`

## Contract and integration

Exports CompactSelect, CompactSelectOption and CompactSelectScope. Agreed value/options/onValueChange and labeling/test/title/disabled/className props. Scope context cancels on scopeKey change without remounting drafts or returning focus. Manager integrates App scope key, consumer callbacks and CSS. Body portal requires primitive tokens or explicit popup font/colors because app-shell aliases are not inherited. Hooks: compact-select wrapper, compact-select-trigger (className appended here), compact-select-value (text span for ellipsis), compact-select-popup, compact-select-option, compact-select-empty. Options expose data-active/data-selected; trigger and options expose data-value.

No mutation on opening/navigation/Escape/trigger cancellation. Same-value commits close without callback. Tab/outside pointer/focus commits active once without preventing natural destination focus. First Escape is synchronously prevented/stopped at React trigger; second closed Escape remains untouched. Enabled navigation, typeahead, current unavailable value visibility, empty options, fieldset semantics, viewport positioning/scroll reachability, ancestor hidden/inert/disabled cancellation and component unmount and window-blur cancellation covered. Scoped ancestor DOM MutationObserver only; no native observer, host polling, surrogate select or forged ChangeEvent.

## Verification

- `npm --prefix projects/chirality-piping/apps/desktop test -- src/features/workspace/CompactSelect.test.tsx`: final 15/15 tests pass; raw unit-04-final.txt. Earlier 12/12 and 14/14 runs retained in unit-01.txt and unit-02.txt.
- `npm --prefix projects/chirality-piping/apps/desktop exec -- tsc --project projects/chirality-piping/apps/desktop/tsconfig.json --noEmit`: exit 0, empty raw output typecheck-03-final.txt.
- Initial direct apps/desktop/node_modules/.bin/tsc attempt failed because dependencies are hoisted; raw failure typecheck-01.txt retained. Corrected invocation above succeeded.

## Limits

jsdom and TypeScript evidence only. No browser/native/AX/visual acceptance, no running bundle rebuild. Manager owns required uninstrumented native, browser/dist, geometry/fidelity, integration and independent-review checks. Native focus traversal itself is not synthesized by jsdom: test verifies unprevented Tab plus outside focus destination. Scope provider must encompass controls and change its key for shell context changes; this preserves mounted component/draft identity. Keyboard consumption assumes existing shell bubbling React/window routes, as in ToolkitPalette; a capturing ancestor listener that acts before the control is outside this local cancellation contract.
