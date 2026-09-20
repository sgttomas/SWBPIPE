# CompactSelect independent-review repair — same bounded TASK

ROOT authorized repair of two confirmed source P2 findings on frozen9d0b/81db.
Same existing Astra/low CompactSelect TASK, no delegation, same two product write
files only: CompactSelect.tsx and CompactSelect.test.tsx. Manager writes support
consumer regressions and corrects two Shared section e2e value assertions. Native
worker is still checking unaffected9d0b binary; no rebuild or UI/browser work here.
Preserve original return and logs; corrected evidence uses new filenames/return.

Finding1: show() and active correction visually pick first enabled option for an
invalid/disabled current value. Tab/outside close(true) then commits it without an
explicit choice. Actual Support family undefined/null/unsupported becomes Anchor on
open→Tab or click Queue. Opening, passive dismissal and catalog arrival must never
infer engineering data. Preserve exact source until explicit option choice or actual
navigation followed by commit. Pointer/Enter selection of an enabled option is an
explicit choice; mere fallback highlight from opening is not. First-Escape and
subsequent closed-Escape contract remains. Valid selected-value behavior stays intact.

Finding2: pending selection is an index; latest.options[index] can name another value
after asynchronous replacement/reorder. Track pending selection by OPTION VALUE and
bind it to its opening controlled-value/options basis. Preserve or cancel pending
choice on reorder/removal/disable; never silently retarget. Controlled-value change
while open cannot leave a stale draft to overwrite new canonical value. A static
'dirty' flag alone is insufficient. Option label/disabled/value changes and unavailable
current values must be handled deliberately with no onValueChange on passive fallback.

Tests must reproduce no-navigation Tab/outside for invalid, disabled and unavailable
current values; deliberate navigation+commit and pointer/Enter choice; same-value
commit; options reorder, selected/active removal/disable and controlled-value update
while open; no stale choice/callback, retained Escape/focus/scope semantics. Manager
runs representative SupportConfiguration no-navigation Tab/outside Queue regressions.
Do not change the contract merely to pass tests, introduce hidden native surrogate,
new observer bridge, polling, dependencies, or broader consumer edits. No test timeout
or oracle weakening. Report any scope/contract conflict before expanding.

Return corrected files/hashes/raw failure+pass evidence, exact semantics and limits.
Original component return remains immutable. Browser/native and independent affected
backchecks remain manager/ROOT gates; no qualification from component tests alone.
