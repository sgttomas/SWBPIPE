# Read-only bounded repair comparison

Basis: product2882acab94120cfa6c3c115ddb416a368cebd9cd; corrected isolated prototype
return dc5ac88876b025224a28e64b4bee6df457ce3defd7b6ee4dc6f40cc36b188c16.
Manager WORKING_ITEMS, Astra/high. No product writes or new probe selected by this
assessment. ROOT requested one concrete bounded repair choice.

Recommend a compact web-controlled select only for existing B3 inspector/routing
consumers. The selection operation and form drafts remain owned by their existing
callbacks. The popup itself becomes DOM state, so its first Escape can synchronously
consume cancellation before the existing narrow/global shell listeners; a later
closed-control Escape reaches those listeners normally. This is a visible control
implementation change and requires actual keyboard/accessibility/native proof.

## Exact proposed production boundary

Paths below are relative to apps/desktop.

| File | Bounded change |
|---|---|
| New `src/features/workspace/CompactSelect.tsx` | Select-only controlled component: value/options/disabled state and value callback, no model or native bridge knowledge |
| `src/features/model-tree/PropertyInspector.tsx` | Nine native-select source sites, including ComponentUnitSelect helper; retain option labels/values, disabled placeholders and existing handlers |
| `src/features/viewport/PipeViewport.tsx` | Six authoring select source sites including ViewportComponentUnitSelect; exclude the selection-filter control at2250 and every picking/rendering path |
| `src/features/support-configuration/SupportConfigurationForm.tsx` | FamilyField and explicit opt-in at seven TextField choices call sites; preserve unsupported/null/missing source-value labels and values |
| `src/features/toolkit/SectionAssignment.tsx` | One Shared section selector, preserving blank placeholder and current queue handler |
| `src/features/rich-authoring/formSupport.tsx` | Optional compact-choice presentation flag defaulting false; only the named support form opts in, so boundary/geometry/rule-pack and other callers retain existing controls |
| `src/styles.css` | Scoped compact-control/listbox styles matching current closed footprint; popup bounded to viewport and scrollable, without altering pane/drawn-canvas allocations |

SupportConfigurationForm and SectionAssignment have no production import other than
PropertyInspector. MaterialTemperatureForm uses TextField without choices; WindExposure
uses existing VirtualTargetPicker, so neither needs modification. No global select
replacement, App/session listener rewrite, renderer/picking change or new dependency
is proposed. Native Rust/build files would retain only the already adopted responder
change. Global page dropdown Escape ambiguity is an adjacent unproved issue and stays
outside this repair; it must not silently widen the implementation.

## Reuse and interaction contract

VirtualTargetPicker already provides value/options, disabled entries, active-option
navigation and VirtualList visibility. It is permanently expanded/searchable, so it
is not a drop-in replacement. Reuse its list/navigation patterns, not that expanded
presentation. ToolkitPalette demonstrates local consumed Escape and trigger return.
A small list should remain simple; virtualization is only useful if actual option
counts require it, and introducing it must not duplicate selection state.

The [W3C select-only combobox pattern](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/examples/combobox-select-only/)
provides a bounded semantic reference: DOM focus stays on the trigger while active
option is exposed, navigation does not immediately commit, and Escape closes without
changing value. Enter/Space/option pointer activation commits; Tab commits then moves
focus. Implement rather than copy the illustrative example, and verify actual host
accessibility. Typeahead, Home/End/arrows, disabled options and active-option visibility
belong in the compact control's contract.

Same-value commit must close without introducing a model operation. Outside focus,
window exit and rerender/unmount must remove the popup without stealing the new focus.
Exact outside-focus commit/cancel behavior must be declared in the component tests;
recommend the above pattern's commit-on-focus-exit, while Escape always restores the
prior value. Keep a separate active option only while open; the existing value remains
canonical. Unknown current values and disabled preserved placeholders cannot be
silently normalized. Preserve inherited fieldset disabled behavior.

The listbox needs an unclipped placement strategy inside finite inspector scroll
containers. A DOM portal can preserve React child cancellation while positioning the
list within viewport bounds; it must retain labels/controls relationships, trigger
focus, topmost pointer ownership and scroll reachability. No canvas/table padding or
new layout allocation. Collapsed control footprint should match the current select.

## Required verification and maintained-test changes

Add focused component tests and explicit B3 e2e/native cases for Space/pointer opening,
same/changed commit, first/second Escape, rapid input, Tab/outside focus, disabled and
unknown values, scrolling, both consumer paths, and Model docked behavior. Retain
actual AX evidence and focus return. Existing tests that call selectOption/fireEvent
change on migrated controls must use real keyboard/pointer selection, never a hidden
native surrogate or forged event. Relevant current unit entrypoints include
`typedInspector.test.tsx`, `schemaSlotEmission.test.tsx`, `richForms.test.tsx`,
`supportFamilyEngine.test.tsx`, `ExistingToolkitEngine.test.tsx`,
`DisplayIntegration.test.tsx`, viewport authoring tests and App tests. Existing
workspace/authoring/r2 e2e adapters need targeted updates only where consumers migrate;
full source/dist lanes detect missed callers. No protected oracle/Box16/floor weakening.

## Compared with the best tested public-native design

The public generation/action snapshot design retains native appearance and silent
same-value commit knowledge. Its prototype touched Objective-C observer/build plumbing,
Rust main-thread snapshot command, frontend lifecycle helper and both shell listeners.
It additionally needs native-window/control correlation, async ordering/teardown,
platform gating and stale/duplicate-consumer proof. Corrected observer scoping passed
its failed same-value case, but pointer ownership is intermittent across observed
routes and the original cancellation-keydown interception remains unproved. Only
keyup cancellation was delivered in corrected runs. More correlation machinery would
be a new speculative layer, already outside the stopping rule.

The compact component therefore offers the smaller deterministic behavioral boundary,
even though migration touches more form sites/tests. It preserves the required
first/second-Escape interaction rather than accepting blanket select suppression.
It is not a proposal to refactor every app select or to claim accessibility acceptance
from unit tests. ROOT must choose this bounded repair before implementation.
