# Key-results review panel after each solve (A6)

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). The panel puts an existing
view of the result on screen with one button or one key. It forms no result, no ratio, no limit and
no status, and it makes no professional claim. Its open state and its "Open after every solve"
choice are session view state. They are not model data, not written to the model document, not
persisted in the UI preferences and not exported. Paths are relative to `projects/chirality-piping/`.

## 1. Basis and scope

HELP_HUMAN adopted survey item A6 (`execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md`)
under the owner's 2026-10-27 delegation (`loop/LOOP_INIT.md`). It serves the iteration loop of PRD
§10.2 step 5 and §10.3 step 1: after a solve, the user looks at ratios, deflections, support loads
and support states, then returns to editing. It touches FR-GUI-001 (the viewport shows deformed
shapes and result overlays) and FR-GUI-006 (an incomplete rule input stays visibly incomplete). This
deliverable owns the viewer slice. It composes A1 (`results-navigator.md`), A2
(`support-load-summary.md`), A3 (`stress-ratio-table.md`), A4 (`result-contour.md`) and A5
(`deflected-shape-and-element-axes.md`). It does not rebuild them.

The sources are:

- `apps/desktop/src/features/results/keyResults.ts`: what is offered, the keys and the auto-open
  rule. These are pure functions.
- `KeyResultsPanel.tsx`: the panel.
- `useKeyResultsSession.ts`: the open state, the auto-open effect and the action wiring.
- `features/viewport/viewportSelection.ts` and `PipeViewport.tsx`: two view commands.
- `ResultsPanel.tsx`: the `viewRequest` prop, its exact kind filter and its withheld marks.
- `SupportLoadSummaryPanel.tsx`: the `focusRequest` prop.
- `App.tsx`: mounting, the Results section's opener and the browser View menu item.

## 2. What is offered, and the state each action sets

The panel is a raised card at the workspace's top right, below the toolbar. It is a labelled region
"Key results" and shows the run id. Each action is one button and one letter key. A select before a
button chooses its target. Its first entry is preselected, so one click or one key shows a view.
Choosing in a select sets nothing by itself.

| Action | Key | Target choice | What it sets | Existing route it matches |
|---|---|---|---|---|
| Show ratio contour | R | Stress category of the user's rule-check run | Viewport `show-result-colour` command: result colour on, quantity `rule-ratio:<category>`, case | The Result colour toggle, then the legend's quantity and case selectors (A4) |
| Show deflected shape | D | A load case or combination of the result | Viewport `show-deflected-shape` command: the deformation case, with the factor back to its default and the phase at 1 | The deformation popover's case selector (A5) |
| Show support load summary | S | An anchor or equipment connection | The session's selection of that support (`handleSelectEntity`), a new A2 focus request, and the Results section | A click on the support; the A2 panel follows `focusSupportId`, and each new `focusRequest` shows it again, even when the user stepped A2 to another support meanwhile |
| Show contact and lift-off state | C | none | The results table's exact kind filter `nonlinear_support_active_set_state_code` (stated above the table, with "Show all kinds"), every other filter at its default, and a withheld mark on each row the standing withholds; the Results section | The table's family and text filters, narrowed to one kind (A1 then steps it case by case) |
| Results table | T | none | The Results section, with the table scrolled into view | The Results stage tab |
| Stress and ratio table | A | none | The Results section, with the A3 panel scrolled into view | as above |
| Displacement extremes | X | none | The Results section, with the A8 panel scrolled into view | as above |
| Back to editing | E | none | The Model stage (`view.stage.model`), and the panel closes | The rail's Model button and View › Model |

- **The ratio contour's case** is the first case, in the viewport's case order (load cases, then
  combinations), that has an outcome of the category the contour can place. The legend's case
  selector then steps the others. The category list, its labels and the quantity id are the ones
  A3 and A4 already derive (`buildStressRatioTable`, `ruleRatioColourQuantities`). The panel calls
  the same builders with the same inputs.
- **The deflected-shape cases** are the viewport's own case list (`resultOverlayCases`): every load
  case and combination the result states, without range envelopes. The product does not decide
  which cases are "main". The user picks the case. The popover's normalized "governing row per
  node" drawing stays its default and is not offered here.
- **Anchors and equipment connections.** A support is listed when its family is `anchor`, or when
  the A2 summary gives it an equipment boundary association or an equipment interface at its node.
  The list is in model order. Other supports remain one step away in the A2 panel. The list reads
  declarations, not restraints: a support fixed in all six DOFs with no `family` is not listed. With
  none listed the panel says "No support is declared as an anchor or equipment connection.", which is
  true of such a model. DEL-07-02's "New support" form suggests `anchor` from six checked rigid DOFs
  with no stiffness, visibly and changeably, and writes the family only when the user queues the
  form. It lists every family; spring and the hanger families, which the applier refuses from that
  form, and nonlinear, which the applier admits without a nonlinear law but the solve would lower
  to a rigid restraint, are disabled with a reason naming Support configuration. No open, load or migration path
  adds a family to an existing support
  (`App.supportFamilyOpen.test.tsx`).
- **Support contact and lift-off.** The product emits one `nonlinear_support_active_set_state_code`
  row per nonlinear support and case (DEL-04-04's classifier through DEL-04-07's loop). Each row
  states its own meaning: "0=inactive, 1=active, 2=sticking, 3=sliding". The panel only counts the
  rows and cases. The results table shows exactly the rows of that kind (a row that merely mentions
  the kind in its id or metadata is not one), with their standing badges, and the A1 navigator
  steps them case by case. A linear result has none, and the action says so.
- **Revealing a panel** scrolls it into view (`scrollIntoView`, nearest edge), once laid out
  (`surfaceReveal.ts`). Focus stays on the key-results button that was pressed, so the next key
  works at once. The status line (role status) says what was shown. Tab reaches the revealed panel.
- **Table view.** The viewport actions still set their state. The status line adds that the canvas
  is hidden in Table view.
- **A rule-check route.** When the model references a rule pack but no run of this result is held,
  a "Rule checks…" button opens the Rule Checks page (`analyze.rule-checks`). It sets no view.

Nothing else changes. Every action writes no model data, queues no operation and records no undo
entry. Selection is not model history. No action moves the camera.

## 3. Opening, and the auto-open rule

**Rule.** The panel opens by itself once per app session: when the session's own solve first
publishes a result with mechanics solved. After that it opens on demand only. The "Open after every
solve (this session)" checkbox makes every such result open it. The checkbox is off by default and is
not persisted. A reopened or bundled run, a cleared result, a refused or failed solve, and a re-render
of the same result open nothing (`keyResultsAutoOpen`).

**Reason.**

- The first solve is when the user most needs to learn where the results are, so the panel opens
  then.
- Later solves happen inside an editing loop. A card that reappears after every solve would cover
  the canvas each time the user has just closed it. The user who wants it every time sets the
  checkbox.
- The UI preference record (`uiPreferences.ts`) is the only existing persistence for view choices.
  Adding a field changes its stored format and its migration tests for a choice that costs one
  click to make again. So the choice stays in the session (§8).

**Focus.** Opening, automatic or on demand, never moves focus. The panel renders in place, so a
field being edited keeps focus and its text. Closing moves focus only if focus was inside the panel.
It moves back to where it was when the panel was opened on demand, else to the rail's button for the
current stage. "Back to editing" sends it to the rail's Model button.

**On demand.** The panel opens from:

- a "Key results" button at the top of the Results section, in both runtimes. It is a toggle and
  carries `aria-pressed`;
- View › Key Results in the browser preview's menu bar.

The native menu has no item yet (§8). Escape or the close button closes it.

## 4. Standing, refusal, absence and incompleteness

The panel decides nothing about what may be shown. Every value stays in the view it opens, under
that view's rules.

- **Categories and ratios come only from the user's rule-check run of this result.** App passes
  the session's run only while the shown result is the current solved result, the A3 and A4 rule.
  The panel's ratio section says exactly one of these:
  - **Absent:** the model references no rule pack and no run is held. "No rule pack…
    Stress categories and ratios come only from your rule pack, so none is shown." No category, no
    ratio, no zero.
  - **Not run:** the model references N packs but no run of this result is held. A "Rule checks…"
    route is offered.
  - **Not this result:** the run binds rows this result does not hold. Nothing from it is shown.
  - **No outcomes:** the run has no outcomes, for example a refused pack. The run's own status is
    shown.
  - **Used:** the pack id and outcome count, the run's aggregate status with its registered label
    (for example "Rule pack · Rule inputs incomplete"), and the list of categories.
- **Incomplete is shown as incomplete.** When any outcome is `RULE_INPUTS_INCOMPLETE`, the panel
  says how many, and that they form no ratio, colour nothing and are never shown as passing.
  - Each category's option names its outcome count, its incomplete count and its withheld count.
  - A category the pack declares with no outcome in this run is listed by name.
  - A category whose outcomes are all incomplete is still offered. The contour then shows its
    legend with the incomplete count and colours nothing (A4 §6).
- **A category the contour cannot draw** is offered with the reason, and its action does nothing
  but state that reason. Either none of its outcomes is located on a single pipe, or all of them are
  on range envelopes. The stress and ratio table lists it.
- **Numerical standing.** The panel states the result's standing as the results panel does: the
  numerical health headline and output sentence, or "Withheld: the result's numerical standing does
  not admit it.". The view actions stay available. The viewport, A2 and A3 withhold the values
  themselves (`overlayMayShowRow`, `resultStanding.ts`), so their withheld views show "held" or the
  withheld reason, never a value.
- **The results table is the exception.** It lists every emitted row with its stated value, as it
  always has, and badges a row only by its case's numerical-health state; a combination without a
  health state gets no badge. So "Show contact and lift-off state" also marks each support-state row
  that the overlays' rule withholds, with the reason A2 and A3 give for its case
  (`withheldSupportStateRows`, `caseWithheldReason`): for example "Withheld: numerical health
  sensitive." or "Withheld: this case has no numerical health state.". The mark is the request's,
  so it holds while the kind filter does. The stated value stays in the row beside the mark: the
  table does not hide values, and the panel neither replaces nor forms one. Rows reached through
  the table's own filters rather than through the panel carry only the table's own badges.
- **No result, a reopened run, mechanics not solved.** The panel says which. The four view actions
  are marked unavailable with that reason. The tables and "Back to editing" stay available.
- **Output policy.** The panel is a view, not an output surface. It has no `outputPolicy.ts`
  entry. It downloads, copies, packages and hands off nothing.

## 5. Keyboard and accessibility

- The keys R, D, S, C, T, A, X and E act while focus is inside the panel. Either case is accepted,
  with no Alt, Ctrl or Meta. They do nothing in a select, a text field, a checkbox or a
  combobox (`isNavigatorExcludedTarget`), so a select's type-ahead keeps its letters. Escape closes
  the panel.
- The keys are not global, so they cannot clash with the shell's ⌘ accelerators, the viewport's H
  and I, or the navigator's Alt+Arrow chords.
- Each button carries its key in `aria-keyshortcuts` and its title, and shows it in a `kbd`. The
  button and the key run one function (`act`), and the tests compare the two routes.
- An unavailable action is `aria-disabled`, not disabled. It stays focusable, and its reason is
  linked by `aria-describedby`. Pressing it only announces the reason, so focus is never lost.
- The groups are labelled "Ratio contours", "Deflected shape", "Supports" and "Tables and
  editing". The status line is a polite live region.
- The card uses the raised-surface tokens of both themes. A disabled action is dimmed and keeps its
  text.

## 6. Composition and render cost

The panel derives once per identity of (`model`, `result`, `standing`, `ruleCheckRun`). It calls
`buildStressRatioTable`, `buildSupportLoadSummaries` and `resultOverlayCases` once each, and counts
the support-state rows. That is the same order of work as the A2 and A3 panels. It is mounted only
while open.

## 7. Tests

- `apps/desktop/src/features/results/keyResults.test.ts` covers the model:
  - an absent pack, a pack not run, a run of another result, and a run with no outcomes;
  - categories with their counts, first drawable case, aggregate status and incompleteness;
  - a declared category without outcomes;
  - categories that cannot be placed, or are on envelopes only;
  - withheld counts under a sensitive case;
  - the case list without envelopes;
  - anchors and equipment connections;
  - support-state counts;
  - standing texts;
  - the no-result, reopened and unsolved states;
  - determinism and unchanged inputs;
  - the key map;
  - the auto-open rule.
- `KeyResultsPanel.test.tsx` covers the panel:
  - for each of the eight actions, the button and the key each call exactly one setter, with the
    same arguments and the same status;
  - chosen targets, and focus kept on the button;
  - keys left to selects, the checkbox and modifier chords;
  - Escape and the close button;
  - `aria-keyshortcuts` and the region's name;
  - the Table-view note;
  - absent and incomplete packs, with no ratio, zero or pass stated for an absent pack;
  - an all-incomplete category;
  - a category the contour cannot place;
  - no support-state rows;
  - no result;
  - a withheld result.
- `apps/desktop/src/features/viewport/PipeViewport.keyResults.test.tsx` covers the two view
  commands:
  - each sets the state its manual route sets (the toggle, the quantity and case selectors, the
    deformation case);
  - an incomplete case's legend is unavailable;
  - no operation is queued;
  - the model and result are unchanged.
- `ResultsPanel.keyResults.test.tsx` covers the table request:
  - exactly the support-state rows of every case are shown, with no selection, and a row whose id
    and metadata contain the kind is not;
  - withheld rows of a sensitive case and of a combination without a health state are marked with
    their reasons, and no other row is;
  - a request is applied once, so the user's own choice then stands;
  - a request for another run is ignored.
- `useKeyResultsSession.test.tsx` covers the wiring: each "Show support load summary" is a new A2
  request, for the same support too, and the contact request carries the exact kind and the
  withheld rows.
- `SupportLoadSummaryPanel.focusRequest.test.tsx` covers A2: after the user steps away, a new
  request shows the requested support again, and a re-render with the same request does not.
- `apps/desktop/src/App.keyResults.test.tsx` covers the app on a replayed solve of the invented
  preview model:
  - the first solve opens the panel while focus and text stay in the palette's search field;
  - Escape from inside closes it, with focus on the current rail stage;
  - a second solve does not reopen it;
  - the Results section's button opens it without moving focus;
  - the D key sets the viewport's deformation case;
  - the table action enters the Results stage with focus kept in the panel;
  - E returns to the Model stage with focus on its rail button;
  - R states the absent pack and leaves result colour off, C states that the linear result has no
    support state, and S selects the anchor, enters Results and shows it in A2;
  - "Open after every solve" reopens the panel on the next solve;
  - View › Key Results opens and closes it in the browser menu, with no result yet;
  - the undo control stays disabled and the model unedited.

The A2, A3, A4, A5 and A1 suites are unchanged and still pass.

## 8. Excluded, and later additions

- **The hanger table (C4)** is not built, so the panel offers nothing for it and shows no
  placeholder. When C4 exists, it becomes one more "Tables" action that reveals it.
- **Frequencies (F1).** The modal view (DEL-04-09) is not shipped. When it is, a "Modes" action can
  reveal its modes table and set the mode-shape view's mode. Until then nothing is shown.
- **A native menu item and a global accelerator** for View › Key Results need a native menu item
  in `src-tauri/src/lib.rs`. The browser menu has the item. The native app opens the panel from the
  Results section's button.
- **The solve-progress offer (D6).** Survey item D6 offers to open this panel when a solve ends. It
  is that item's own scope.
- **Persisting "Open after every solve"** would need a UI-preference field and its migration (§3).
- **A key-results headline**, the worst ratio or the largest movement read through the A2, A3 and
  A8 composition contracts, is not shown. Each would need its exclusions beside it, and the panel
  is the route to the views that already state them.
- **A camera move to the revealed support** stays the user's Fit Selection.

This note describes the design and code only. It records no backcheck or acceptance.
