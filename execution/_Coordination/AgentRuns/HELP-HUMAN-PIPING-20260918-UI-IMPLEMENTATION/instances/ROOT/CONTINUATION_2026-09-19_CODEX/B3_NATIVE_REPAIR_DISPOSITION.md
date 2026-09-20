# B3 native keyboard repair disposition

ROOT engineering disposition after isolated probe, not an owner product ruling.
Probe return: B3-CODEX/_run_records/native-probe/RETURN.md on shell lane,
SHA-256 b4381b46a97710573876f91d40bd64cbf533a80309edbb8732a4e5d1c5c334bd.

## N1 — predefined native text Undo/Redo

Adopt predefined native Undo/Redo responders in Edit, retaining explicit model
menu actions and the existing editable-aware model shortcut guards. Probe A/B
changed only predefined responders; it restored native text undo/redo with stable
input identity and no app default prevention. Neutral model shortcuts passed
before and after populated text undo/redo stacks, plus model pointer actions.

Reconfirm another text field, undo exhaustion, neutral focus before/after populated
undo and redo stacks, and pointer model actions in the uninstrumented repaired
app. This refines B3_UNDO_OWNERSHIP_DECISION: model shortcuts still use their
guarded route; predefined OS text responders supply normal editing behaviour.

## N2 — observed select-popup lifecycle

At Space keyup the actual select was :open; by Escape keydown the host had
already closed it and defaultPrevented was false. Implement a small guard that
remembers genuinely observed open state across that host transition, yields
only that cancellation across both global/narrow paths, clears on commit/focus
exit/pointer lifecycle/cancellation, and permits subsequent closed-select Escape
to close the Both inspector. No blanket select suppression or continuous polling.

Validate pointer and Space opening, same/changed selection commit, outside focus,
reopen, first/second Escape, neighboring controls, and global/drawer paths. The
module/probe repair assignment is Astra/low; manager owns consuming integration.
The old failed pointer reproduction and successful repeated pointer probes stay
as evidence; a pointer-only pass does not establish repair of the Space failure.

## Native geometry and remaining boundary

Public Rust APIs measured physical2560×1600 at scale2 after a real corner drag,
or1280×800 logical native window; a second shrink attempt produced no smaller
size. Startup Rust logical height920 and DOM innerHeight888 differ by32.
Record native/window and DOM content dimensions separately. This is not800 CSS
content proof or D-72 conformity. Carry the measured host-chrome difference into
the later second-profile package rather than silently changing ruled dimensions.

No probe/debug code enters production. Final uninstrumented behaviour, independent
review, full affected checks and remaining B3 gates are still required.

## N2 follow-up — failed guard and bounded native correlation prototype

The observed-open DOM guard failed same-value Return and pointer commits: neither
produced a DOM commit event, so the next ordinary Escape was incorrectly consumed.
The failed candidate and raw evidence are retained; the guard was removed from
production. Product checkpoint `2882acab9` contains only the N1 predefined
responders on top of the reviewed page-close repair.

The isolated public-menu probe found that tested same-value commits emit AppKit
action notifications after end-tracking, while cancellations do not. Its return
is `B3-CODEX/_run_records/public-menu-signal-probe/RETURN.md`, SHA-256
`7d2cfac5e2810d22bfd37e9ab82ae3acf65f63f1dae92338b34f081f8f641943`.
This is diagnostic evidence, not a production bridge or repair pass. The original
Escape-keydown failure did not reproduce in that probe.

ROOT authorized one bounded Astra/low correlated-popup prototype in the isolated
checkout. Bind only a genuinely open DOM popup to its native generation, then
use a main-thread snapshot/barrier at ambiguous Escape; do not rely on push-event
arrival order or decide on end-tracking alone. Share one event decision across
handlers and reject deferred actions after input/focus/view/window generation
changes. Test silent same-value commits, cancellation, quick/reordered responses,
duplicate consumers and subsequent Escape. No time-delay guesses, polling,
private APIs, global monitors or new permissions. If real correlation cannot be
established, stop with a concrete interaction tradeoff for ROOT/owner disposition.
No production adoption is authorized by this prototype assignment. It is B3
native repair, not Chirality Runtime integration.
