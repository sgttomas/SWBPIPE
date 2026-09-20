# Sealed compact select implementation

TASK Type 2, GPT-6 Astra/low, fresh delegated-harness-native child of B3-CODEX
WORKING_ITEMS under ROOT HELP_HUMAN. No delegation. Use explicit WORKTREE root
supplied at launch. Parent owns integration/index; unrestricted filesystem scopes
are instructions, not sandbox guarantees. Product basis2882acab94120cfa6c3c115ddb416a368cebd9cd.
Read root AGENTS, agents/AGENT_TASK.md, Piping AGENTS/loop. Read full ROOT
B3_NATIVE_SELECT_DECISION.md at supplied origin, SHA256
9ff7e7f325def1ed4750c14c3b865a6875bc988368024c288cbb23b54f6d8563.
Record actual basis/origin/model/parentage/hash receipt in own evidence.

Implement ONLY new src/features/workspace/CompactSelect.tsx and its meaningful
CompactSelect.test.tsx in apps/desktop. Own evidence under B3-CODEX/_run_records/
compact-select. No other source/test/CSS files, no native/probe changes, no Git
mutation or dependencies. Manager integrates exact affected consumers/styles and
navigation tests concurrently. Do not run browser/native or rebuild the native
bundle: original uninstrumented2882 witness currently owns foreground/CUA and ROOT
fidelity reviewer owns headless5183/5184. Unit tests/typecheck are allowed.

Agreed public interface: exported CompactSelect and CompactSelectOption type;
options readonly {value:string;label:string;disabled?:boolean}[], value:string,
onValueChange:(value:string)=>void; support id, aria-label, aria-labelledby,
aria-describedby, data-testid, disabled, className and title. No forged native
ChangeEvent or hidden surrogate select. Preserve unknown current value visibly;
never normalize it automatically. Support inherited disabled fieldset behavior.
The callback preserves the existing form draft route; opening/navigation/cancel
must never call it, and same-value commit closes without a duplicate callback.

Compact select-only trigger and web listbox, no native AppKit popup. Use existing
VirtualTargetPicker/VirtualList navigation/visibility and ToolkitPalette consumed
Escape/focus-return patterns as references, not expanded searchable UI. Closed
footprint matches current select; manager supplies scoped CSS. Agree class hooks
compact-select, compact-select-trigger, compact-select-popup, compact-select-option
and active/selected state via data attributes. Component owns popup position/height
bounded to viewport and scrolling reachability; explain hooks to manager promptly.
Portal may avoid inspector ancestor clipping, with correct label/control/active
option relationships and local React event cancellation. No new layout allocation.

Required semantics: pointer/Space/Enter/arrows open, Home/End, typeahead, disabled
control/options, invalid/unavailable/empty values, selected vs active option.
While open Escape synchronously preventDefault+stopPropagation, closes without
commit, and returns focus to trigger. While closed Escape is untouched for existing
narrow/global shell handler. Same-value pointer/Enter commit closes; changed commit
calls existing value callback exactly once. Tab commits active option then proceeds
with natural focus; outside focus commits active option without stealing focus;
Escape always cancels. These follow the W3C select-only combobox reference
https://www.w3.org/WAI/ARIA/apg/patterns/combobox/examples/combobox-select-only/
(reference only; do not copy illustrative code wholesale). Pointer-trigger closing
cancels. Keep focus on trigger during list navigation, expose active option and
scroll it into view. Unmount, hidden/inert ancestor, stage/view change and another
control opening cannot leave stale popup/focus; identify any required manager
integration hook before widening scope. No timers/polling to infer host popup state.
A typeahead buffer timeout is ordinary interaction behavior, not host correlation.

Meaningful tests: original open→Escape preserves surrounding inspector callback;
second closed Escape reaches it; same/changed commit, disabled/unknown values,
rapid input, duplicate React/window consumers, Tab/outside dismissal/focus behavior,
multiple controls, unmount/stale state, portal containment and inherited fieldset
state. jsdom is not native/AX acceptance; manager owns browser/native regression.
Retain all failing and final raw outputs with new filenames. Send interface/status
promptly, no periodic empty updates. Return exact changed files/hashes, test counts
commands, implementation contracts/limits and source identity; no acceptance claim.
