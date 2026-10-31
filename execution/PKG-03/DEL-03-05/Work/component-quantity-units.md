# Component creation quantity units (G6)

Serves RT-G2 and RT-G3. Agent 0 authorized G6 as the first follow-up authoring
lever on 2026-10-31. The inspector and viewport creation forms previously used
one length unit for rigid-body length, both end sizes and the COG vector. The
reference-system run sheet therefore needed six later end-size edits to preserve
the fixture's entered units.

For a valve, flange or reducer, each form now offers **Body length unit**,
**End A size unit**, **End B size unit** and **COG unit** beside those values.
Weight retains its existing unit selector. The COG's X/Y/Z components share
their vector's unit; the owner's R10 origin and global-axis convention remain.
The controls use the existing accessible compact selectors and their keyboard
routes. Each unit starts from the project's length key, or stays unselected
(`TBD`) if that key is absent. No engineering value is supplied. Choosing one
unit neither changes another unit nor converts or changes an entered value.

The transient `ComponentDraft` carries the three additional choices. Each
existing geometry quantity receives its own entered unit in the
`insert_component_symbol` intent. Draft checks require all four units; review
metadata describes each quantity's catalog check, and a missing-project-length
notice remains while any is unselected. The operation engine remains the
authority for unit dimensions and validation. The existing explicit Review and
Apply route persists the entered quantities; the engine's existing conversions
govern mechanics consumption. No model-document member, command, quantity
convention or solver calculation changes.

Evidence: `componentIntent.test.ts` covers independent units on valves, flanges
and reducers, incomplete choices and missing-project-unit notices.
`rigidQuantityUnits.test.tsx` enters RS-F1 through both actual forms with 0.15 m
body length, 270 mm end sizes and COG (-0.075, 0, 0) m, checks the emitted record
against the maintained reference fixture, applies it through the Wasm operation
engine and checks browser save/reopen. The maintained catalog fixture supplies
unit choices; no native App invocation or native persistence claim follows.

## Reference-system step model

The DEL-09-01 run sheet's six end-size task edits accounted for 24 entries and
six apply cycles. Creation now adds six unit selections instead. Its valve and
flange step changes from 84 entries / nine cycles to 66 entries / three cycles.
Retaining the sheet's 5.5 s per form entry and 0.75 min per later field edit:
10 - 6 × 0.75 + 6 × 5.5/60 = 6.05 min, rounded to 6 min under the existing
quarter-minute convention. The sheet total changes from 687 entries / 38 cycles /
64.5 min to 669 entries / 32 cycles / 60.5 min (about 61 minutes), before counting
G9 batching. These are estimates, not a timed walkthrough. Criterion 7's value
and the walkthrough remain with the owner and Agent 0.
