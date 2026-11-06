# Results navigator with keyboard stepping

This design covers survey item A1: the Results table shows one kind of row at a time and steps,
by key or by button, to the next or previous table kind, load case or combination, and support
or element. The current row's page and scroll follow it, and the session's selection follows it,
so the viewport, the tree and the inspector show the row's entity. Paths are relative to
`projects/chirality-piping/`. The standard claim fence applies (F-PIP-2). The navigator presents
existing rows. It forms, orders and qualifies no value.

## 1. Basis and scope

HELP_HUMAN adopted A1 under the owner's 2026-10-27 delegation (`loop/LOOP_INIT.md`). The CAEPIPE
survey (`execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md`, A1) recommends it for
review speed on models with many cases: reading one support across every case becomes one key
press per case. It serves FR-GUI-001 (the viewport shows the selection) and FR-GUI-002 (the tree
shows the selected entity). This deliverable owns the viewer slice. GUI state (DEL-00-05) is not
changed: the navigator's position is view state of the Results panel, as the panel's filters and
page already are.

The following are in scope: the navigation model, the steppers, the key chords, page and scroll
following, and selection following through the existing `onSelectResult`. The following are out
of scope:

- **Stepping inside the other result panels.** The Support Load Summary, the Stress and Ratio
  Table, the Displacement Extremes and the Weight and Centre of Gravity panels keep their own
  controls. They follow the session's selection as they already do, so a navigator step that
  selects a support also moves the Support Load Summary to it (`focusSupportId`), marks the node
  in Displacement Extremes, and marks the row in the Stress and Ratio Table.
- **A menu-bar route.** The visible route for every step is a button in the navigator (§3). A
  View menu entry would need a native menu item in `src-tauri/src/lib.rs` and is a remaining item
  (§9).
- **Camera framing.** The viewport highlights the selection. It does not zoom to it; Fit Selection
  stays the user's command (§4).

## 2. What is stepped

The navigator sits in the Results panel (`apps/desktop/src/features/results/ResultsPanel.tsx`),
below the result filters, as three steppers and one live line. The model is
`apps/desktop/src/features/results/resultsNavigator.ts`.

| Axis | List | First position | Source |
|---|---|---|---|
| Table kind | "All kinds", then each row family the result has rows of, in the family filter's order: Displacement, Rotation, Reaction, Force, Moment, Stress, Ratio, Other | All kinds | The reader's row family (`semanticLabels`), as the family filter buttons count it |
| Load case or combination | "All cases", then every case the result's rows state: load cases, then combinations, then other bases, each in first-appearance order, then "No case stated" for rows with no `basis_ref` | All cases | Each row's own `basis_ref` |
| Support or element | Every entity of the rows the other filters admit, once, in result order | none until chosen | Each row's own `entity_ref` |

- **Table kind** is the existing family filter. Stepping it presses the family button, so the
  table shows that kind's group only: one table at a time. Element forces are the Force and
  Moment kinds; summaries and non-physical rows are Other; Ratio is offered only when the result
  states ratio rows.
- **Case** is a new filter of the table, beside the frame filter. Its list is taken from every row
  of the result, not from the filtered rows, so it does not change with the text, kind or frame
  filters, and case *n* is the same case in every table. A case select offers the same list for a
  direct choice. A combination whose rows state the range-envelope basis is labelled "(range
  envelope)".
- **Support or element** is the current row's entity. Its list is taken from the rows that the
  text, kind, frame and case filters admit, before "Selected entity only", so the list still has
  every entity while that filter is on. The current row is the session's selected result row
  when it belongs to the entity and is listed, else the entity's first listed row.
- **Where the entity comes from.** The navigator's entity is the one the last step chose. A result
  row selected anywhere moves it to that row's entity, and a new session selection moves it to the
  selected entity. A new run clears it.

**A result held natively (C1, DEL-04-05).** The same navigator steps the same three lists in
`NativeResultsPanel.tsx`. Kinds and cases come from S (the ports of `countResultFamilies` and
`navigatorCases`). The support-or-element list comes from the native query
`query_preview_mechanics_job_navigator` over the rows the same filters admit: each distinct
`entity_ref` once in first-publication order, with its first row's position among those rows and
its ordinal, so a step selects the entity's first published row (its own id, by ordinal) and opens
its native page without the webview holding the rows; a probe answers whether the selected row is
listed when the page shown does not tell. The list scales with the entities. A step made while the
list is read waits for it; a reply for a step, filter or result that is no longer current is
discarded. A filtered `entity_ref` that is not a string is refused by name
(`RESULT-DETAIL-NAVIGATOR-ENTITY-TYPE`) and the entity stepper says the list could not be read,
where the full-row list would hold that value; kinds and cases still step.

Kind and case steps keep the entity. When the entity has rows in the new table, its first row
becomes current and is selected (§4). When it has none, it stays the navigator's entity, the
selection is not changed, and the stepper reads "⟨entity⟩: no rows in this table". So "one
support, all cases" is: choose Reaction, step to the support, then step the case.

## 3. Keys, buttons and focus

| Step | Previous | Next | Button |
|---|---|---|---|
| Table kind | Alt+Shift+ArrowLeft | Alt+Shift+ArrowRight | "Previous table kind", "Next table kind" |
| Load case or combination | Alt+ArrowUp | Alt+ArrowDown | "Previous load case or combination", "Next load case or combination" |
| Support or element | Alt+ArrowLeft | Alt+ArrowRight | "Previous support or element", "Next support or element" |

On macOS, Alt is Option. Each button carries its chord in `aria-keyshortcuts` and its title. The
button and the chord run the same step; the tests compare the two routes step by step.

- **Scope.** The keys act only while focus is inside the Results panel. They do nothing in a text
  field, a select, a contenteditable region or an element with role combobox, listbox or menu
  (the result filter, the rows-per-page select and the case select keep their own keys), and
  nothing when another handler has already taken the key.
- **No clash.** Alt+ArrowLeft and Alt+ArrowRight are already the Support Load Summary's support
  step inside that panel; the navigator's entity step uses the same chord for the same meaning,
  and the two panels are siblings, so a key reaches only one of them. No other handler in the
  desktop uses Alt with the arrows: the shell's accelerators need Cmd or Ctrl (⌘1 to ⌘3, ⌘I, ⌘Z),
  the viewport's H and I act only in the viewport and take no modifier, the context menu takes
  every key only while it is open, and the native menu binds no Alt chord.
- **Groups and labels.** The navigator is a region "Results navigator". Each stepper is a group
  labelled "Table", "Case" or "Support or element", with its position as text ("Reaction (3 of
  7)", "Load case L-100 (2 of 4)", "S-120 (2 of 5)"). The live line has role status.
- **Focus.** A button step leaves focus on the button. At an end, the button is marked
  `aria-disabled` and does nothing, but it stays focusable, so focus is not lost to the document
  when the end is reached. A key step from a result row moves focus to the new current row, since
  the row being left may leave the page. When the destination has no current row (an empty
  table, or a table without the entity), focus moves instead to that step's own button, which
  stays mounted, so the keys keep working. A key step from anywhere else leaves focus where it is.

## 4. Following the current row

A step that finds a current row does exactly what a click on that row does, and nothing more:

- it calls the panel's `onSelectResult` with the row's id. In the app this is the session's
  `handleSelectResult`: the row becomes the selected review target, and its entity, where the
  model resolves it, becomes the session's selection. The viewport highlights the selection, the
  tree reveals it and the inspector shows it, through the selection they already read (B8);
- the table moves to the row's page and scrolls the row into view (`scrollIntoView`, nearest
  edge), once the row is laid out (`surfaceReveal.ts`). With "Selected entity only" on, the page
  is the first, as for any new selected entity.

The navigator writes no model data, queues no operation and records no undo entry; selection is
not model history. It never moves the camera. In a reopened historical run there is no session
model, so the panel's own selected row follows and no entity is selected.

## 5. Ends and empty tables

- **Ends.** Every axis stops at its first and last position. It does not wrap, so the user can
  tell that the last case has been read. The step at an end does nothing to the table or the
  selection, and the live line says "Last load case or combination reached." (or the first, or
  the table kind, or the support or element). From no chosen entity, Next goes to the first
  entity and Previous to the last.
- **Empty tables.** A kind and case with no rows under the current filters is not skipped. The
  table shows its existing empty statement, and the live line says "No rows of ⟨kind⟩ are stated
  for ⟨case⟩ under the current filters.". The entity stepper then has nothing to step, and both
  of its buttons are unavailable.
- **No result rows.** Every stepper has nothing to step.

## 6. Results that are refused, incomplete or withheld

The navigator never decides what may be shown. It lists every row the table lists, under the
same filters, and every case any row states. No list leaves out a row, a case or an entity for its
numerical standing, its health or its rule outcome.

- **Numerical health.** Rows keep their health badges. Where the table badges rows (the generic
  route), a case with a stated numerical-health state other than "checks passed" is named with
  that state in the case stepper and the case select, for example "Load case L-1 (numerical
  health: sensitive)", so a stepped-to case never reads as passed. Withheld combinations retain
  the table's absence of a state and badge. The health
  header, the output sentence and the per-case detail are unchanged above the navigator.
- **Standing.** The standing line (`numerical-result-standing`, from `resultStanding.ts` and the
  retained and source-route readers) stays above the navigator for every route, including
  "Unsupported numerical contract; historical values only. Needs recompute.". The navigator steps
  an unsupported result's rows as the table shows them.
- **Output policy.** The navigator is a view, not an output surface: it downloads, copies,
  packages and hands off nothing, so it has no `outputPolicy.ts` entry and no surface's refusal
  changes.
- **Rule checks.** The user's rule-check outcomes, including incomplete ones
  (RULE_INPUTS_INCOMPLETE), are listed by the Stress and Ratio Table, pinned first and never hidden
  (`stress-ratio-table.md` §5, §7). The navigator does not filter, sort or hide that table; a step
  that selects a result row only marks it there. The governing-ratio state and the mechanics gap
  ledger stay in the panel.
- **Range envelopes.** A range-envelope case is stepped like any other and labelled as one.

## 7. Claims fence

The navigator moves a view over emitted rows. It records and implies no acceptance, issuance,
compliance or adequacy, compares nothing with a limit, and selects no governing case.

## 8. Tests

- `apps/desktop/src/features/results/resultsNavigator.test.ts` covers the model: stepping and its
  ends without wrap; from no position; an empty list; case order, labels, range envelopes and rows
  with no case; entity order; the key chords, and the chords that are not steps; and the editing
  targets that keep their keys.
- `apps/desktop/src/features/results/ResultsPanel.navigator.test.tsx` covers the panel on the
  product's preview-physics-2 output for the invented unicode-ids vector: the labelled groups,
  buttons and chords; one support's reactions read case by case by key, with the selection, the
  table's case and focus following; key and button parity over a mixed sequence; ends that stop,
  say so and keep focus; focus kept in the panel when a key step from a row reaches an empty
  table or a table without the entity; an empty table; an entity with no rows in the next table; the page and
  scroll following a far row; keys left to the filter field and selects; no result change and no
  callback but the row selection; a sensitive case named and its badges kept; a withheld
  combination's rows stepped with no badge; and an unsupported result stepped under its unchanged
  standing.
- `apps/desktop/src/App.resultsNavigator.test.tsx` covers the app's wiring on a replayed solve of
  the invented preview model, now held natively: a button step and a key step each select the
  row's node in the session (the selection readout), focus follows the row, a case step keeps the
  node on its first row of that case, and the undo control stays disabled with no model edit. It
  waits for the native entity list after a kind step.
- `apps/desktop/src/features/results/NativeResultsPanel.native.test.tsx` covers the native path
  with the actual Rust replies: the entity list, first positions and ordinals equal
  `navigatorEntities` and `firstRowOf` over the full rows under each family, case, frame, text and
  kind filter, with probes; every entity of an invented paged source stepped to its first row on
  its own page across several pages, the end stopping without wrap, the chord from a row moving
  focus, kind and case chords; a stale step's reply and a reply after disposal discarded; and a
  lone surrogate in the search shown as the typed refusal on the rows and the entity list.
  `result_detail_tests.rs` pins the command over every corpus source and filter member, its
  probes, its refusals and its leases.

## 9. Remaining items

- **A View › Results menu route** for the six steps, with a native menu item.
- **The deflected-shape case (A5)** could follow the navigator's case.
- **Stepping inside the Stress and Ratio Table** (by rule category) and the Displacement Extremes
  (by node) from the same keys.
- **Model labels for cases.** The panel has no model, so cases are named by their ids.
