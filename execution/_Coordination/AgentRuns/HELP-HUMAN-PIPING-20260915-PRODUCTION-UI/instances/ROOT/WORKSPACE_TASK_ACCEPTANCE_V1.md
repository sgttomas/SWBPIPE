# Workspace task acceptance V1

Derivative verification instructions for the approved plan and D-68. These checks do not close the separately held PDU-045/PDU-046 independent-usability work. The touched-control contrast/keyboard/target-size criteria are already selected by the owner in the approved plan; do not describe those adopted criteria as still awaiting a target selection.

## Production tasks

| Task | Observable result |
|---|---|
| Open a project, inspect and navigate | Canvas is the dominant working surface. The model tree, selected properties, camera controls, project units and active construction constraints are identifiable without opening technical evidence. Fit follows the actual visible authored geometry. |
| Select, inspect and filter | A single click selects one typed entity; modifiers build an ordered set. Tree keyboard focus is visually distinct from selection. Filter/collapse changes displayed range order, preserves valid selection and cannot silently target a draft. Multi-selection shows aggregate inspection and specific single-target editing reasons. |
| Perform an existing engineering edit | Command search and category navigation reach the same typed operation. The task identifies its frozen target and units. Entered values survive panel navigation; Add, Cancel, Review and Apply are explicit. Pending review visibly opens the dock. Switching selection cannot silently change the edit target. |
| Use a selected-pipe target set | Transform and self-weight copy the current validated pipes only when their explicit action is invoked. Later selection changes preserve the copied draft targets. Queued filtered grid edits clear only the drafts included in that queue. |
| Work with visibility and dimensions | Hidden entities stay selected and marked in the tree. Attachment ownership follows the accepted mask rules. Measurement reads authored endpoints and unit conversion. Invalid Actual OD displays the centerline and an explanation. No view command changes the engineering model hash. |
| Inspect saved and new results | Historical designation and integrity findings remain visible. Historical data cannot produce Current overlays, checks, comparison readiness or report readiness. A fresh successful solve restores Current only against its exact input basis. |

## Appearance and responsive witnesses

Inspect both Light and Dark at Comfortable and Compact density, each at 1024×768, 1280×800 and 1440×920: twelve actual production states. Separately exercise System theme following a changed OS preference and preference persistence after reload. At and above 1280 px both rails are available; below that, independent drawers preserve canvas access and the task action footer. A review becoming pending opens visibly. Capture ordinary and busy/disabled/focused/selected/diagnostic states, not only the empty shell. Root must view representative captures before accepting the visual result.

## Keyboard and assistive semantics

Use actual keyboard navigation for command search, category navigation, tree movement/selection, task inputs, review/apply/cancel, viewport presets/fit, visibility and measurement. A resizable panel needs a keyboard route (for example a focusable separator with arrow-key resizing and declared bounds) and a discoverable accessible name. Drawers must restore focus to their opener, keep focused controls visible, and keep inactive controls outside the tab sequence.

For the virtualized multi-select tree, assert the named tree, multiple-selection semantics, distinct focus, correct expansion states, and declared level/position/set-size for rows absent from the DOM. Moving focus to an off-window item scrolls and presents that item without mounting every selected row. The adopted behavior follows the [W3C tree pattern](https://www.w3.org/WAI/ARIA/apg/patterns/treeview/).

Touched controls use the adopted text and graphical contrast ratios and applicable minimum target dimensions. Check actual effective target boxes in both densities, including icon buttons and splitter alternatives; the relevant criterion permits explicitly applicable alternatives or spacing rather than treating an icon's glyph size as its hit box. Source: [WCAG target-size guidance](https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html).

## Claim calibration

Automated structure, contrast and task witnesses are project verification. They do not certify accessibility or establish independent practitioner usability. Source-mode/jsdom tests, production-browser tests, actual native GUI gestures, stored-byte comparisons, analytical results and performance measures retain separate evidence labels. No failure or unavailable baseline metric is converted to a candidate PASS.
