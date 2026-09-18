# Revision pass — preparation, questions for the owner, and what waits on D-71

Status: ROOT record, 2026-09-18, successor ROOT. The program owes one revision pass from sealed briefs: design system V1.2, UX specification V1.1, regenerated frames. This record assembles its inputs. **No revision brief is sealed yet**, because the pass applies the owner's actual D-71 rulings and none has been given, and because the eight screen questions below have no recorded owner answer (ROOT searched the run, the direction record and the decisions directory; the only occurrences are the questions themselves). One pass, not two: splitting it would regenerate eighteen frames twice.

## 1. Inputs already decided (need nothing further)

| # | Input | Source |
|---|---|---|
| R-1 | UX specification §10.9: the Both-view canvas floor of 320 px and table floor of 480 px are reconciled to the design system's 220 px canvas minimum (`layout.canvas.min`) and its narrow-case order (agent column to its strip first, then the slide-over for that window size only). | Brief index, DESIGN-SYSTEM-02 acceptance; design system V1.1 §0, §9 rows 1 and 48 |
| R-2 | UX specification cites design system V1 by name and section; every citation is reconciled to V1.2's names and numbers. | Brief index, UX-SPEC-01 acceptance |
| R-3 | The chip after a failed run follows the run record; no frame draws "Model incomplete" after a non-convergence. | Brief index, UX-SPEC-01 acceptance; already drawn so in MOCKS-02 |
| R-4 | The Type column offers the engine's element kinds only; the design system's table of the file's kinds is aligned. | Same |
| R-5 | Product copy uses Canadian English and the product's existing spelling "Analyze". | Same; design system V1.1 §9 row 49 |
| R-6 | The dark edge line's 1.10:1 on the brightest dark result step is an input to the contrast open item; V1.2 may consider an edge token that switches by fill lightness. | Brief index, DESIGN-SYSTEM-02 acceptance |
| R-7 | D-71 item 6, the historical-run wording: the rendered text is used ("Historical saved run · Run a fresh solve to establish current results."; the popover carries the second sentence). The packet states that silence lets this stand because no act is needed. | D-71 item 6 and on-ruling mechanism |
| R-8 | D-71 item 9, the export's name: named by what it produces, as the packet words it, with the rendered flag "Compatibility with CAEPIPE: not claimed; no evidence recorded". Same standing as R-7. | D-71 item 9 and on-ruling mechanism |
| R-9 | Presentation must not give a Historical record a current-model overlay or a readiness claim: the results header band, the legend and the canvas colour state are specified for Current and for Historical separately. | Successor activation, implementation-handoff constraints |

## 2. The eight screen questions, with ROOT's recommendation

These are the owner's to answer. Each recommendation is what the brief will say if the owner agrees; a different answer changes only that line.

| Q | Question | Recommendation | Why |
|---|---|---|---|
| Q-15 | In Both view with the inspector docked, ⌘⇧G would take the canvas under 220 px. | The column opens and the inspector becomes the slide-over for as long as the column is open at that width; it re-docks when the column closes. Nothing is refused and nothing closes. | The engineer's last explicit act wins; it mirrors row 48, where opening the inspector sends the column to its strip; the selection's context is never lost. |
| Q-16 | The run log popover covers the tab strip's end and the HUD's first buttons. | Hang it right-aligned under the Run button, over the canvas's top edge; it closes on Escape or the first click outside it. | Ordinary popover behaviour; it leaves the table header, where the engineer is working, uncovered. |
| Q-17 | Where "Compare with…" lives. | An entry in the iteration combobox's menu starts a comparison; the compared-with line appears only then, and is itself a combobox to change or clear it. | No fourth header button; the control is absent when nothing is compared, present and editable when something is. |
| Q-18 | The collapsed accepted row cannot hold old and new value in 316 px. | The collapsed line shows the new value, the row count and "accepted 16:31 · Undo"; the old value stays in the diff table above and in the tooltip. One line, no wrap. | A collapsed row is a receipt; the diff is one click away and a wrapped row breaks the queue's rhythm. |
| Q-19 | Export… on the accent makes two accent buttons on the Review page. | Export… is a plain button like Report preview; the specimen's Review header is corrected. | One accent per window, and it is Run; Export… is not the page's primary act, writing is. |
| Q-20 | After docking, the canvas shows the model cropped until the engineer presses F. | If the camera has not moved since the last Fit, docking and undocking refit; otherwise the camera is kept as decision 1 says. No toast. | Keeps decision 1's rule for a camera the engineer placed, and removes the crop in the common case where the engineer never placed it. It is a proposed semantic addition (a "fitted" camera state) and is flagged as one for implementation. |
| Q-21 | No chip after a failed run leaves the status bar's left end empty. | Accept the empty end until the engine answers specification §11 question 11; do not persist "Model incomplete". | A chip states a status the run record carries; showing one the record does not carry would be the interface asserting a status. The rail caption and the banner already carry the state. |
| Q-22 | Eight filter chips wrap to two rows in the 320 px comment column. | Four chips (All, Open, Resolved, Mine) and one Kind menu (Checks, Open issues, Drafts, and the fourth kind under whatever name D-71 item 8 gives it), with the count on the menu when a kind is chosen. | State and kind are two different filters; one row; zero-count kinds stop taking space. |

## 3. The six gaps, as ROOT will direct them in the design-system brief

| Gap | Direction to the child |
|---|---|
| G-7 | Add the three Review icons (Report preview, Export, Snapshot) to the specimen's sprite and draw the Review header with them. |
| G-8 | Specify a toast component in §5: anatomy, placement clear of the status bar and the agent strip, duration, one at a time, never for a state a permanent surface already carries. |
| G-9 | Conditional on D-71 item 2. If option A is ruled, specify the caption band under the results header (one line, secondary text style, never truncated before the sentence ends). If B, no component and the gap closes unused. |
| G-10 | Specify the narrow-canvas HUD: the wrap width, buttons per row and the place of the mock line, from 220 px to the width at which it is one row. |
| G-11 | Specify the issues drawer row's overflow: the entity and the link never truncate; the message truncates with an ellipsis and a tooltip; row height fixed. |
| G-12 | Specify the run log's title line and its three row glyphs by name from the marks vocabulary, so the frames stop reading "a glyph". |

## 4. What waits on the owner's D-71 rulings

| Item | Revision consequence if ruled as recommended | If the other option, or silence |
|---|---|---|
| 1, maturity line | Status bar carries the sentence permanently at its right end; About repeats it; frames drawn as the item 1 aid frame. | Popover and About only, as V1.1 draws; under silence the conditional placement stays and the registry clause is unresolved. |
| 2, acceptance sentence | Results header carries short variant 2 as a visible caption; M-02's home moves from the disclosure to the caption; G-9's band is specified. | Stays in the disclosure. |
| 3, product name | No design change: every frame already carries the new name. The rename's governed acts are separate (scope-change bundle for 3(i), registry act for 3(ii), packaging act for 3(iii)). | Under C the frames would revert to the old name. |
| 4, status labels | "User rules checked" and "User rule failed" become the hyphenated forms; labels exist only from the registered table; token reachable in place. | Labels stay design-system microcopy. |
| 5, hanger libraries | The hanger-selection surface gains the `BS-IP` short variant in the specification. | No change. |
| 7, Checked mark | Wording fixed as the packet gives it; specification §6 states the storage boundary (interface state, not model payload; never exported). | Under B the mark leaves the first version until `PB-TBD-002` closes. |
| 8, agent feedback | Agent-authored "Note" becomes "Evidence summary" on cards and in Q-22's Kind menu. | Card labels stay as drawn. |

Items 6 and 9 are R-7 and R-8 above.

## 5. Order of work once the owner has answered

1. DESIGN-SYSTEM-03 (Fable): V1.2 from §1, §2, §3 and the rulings; change log continued from row 50.
2. UX-SPEC-02 (Fable), after V1.2 is accepted: V1.1 with R-1 to R-5, R-7 to R-9, the answers and the rulings; the operations map re-checked against `HEAD`, since PRs #793 to #795 landed after the map's citations were taken and cited lines may have moved.
3. MOCKS-03 (Fable), after both: the eighteen frames regenerated; the two decision-aid frames retired or folded in according to the rulings.
4. ROOT reviews each return against its brief, inspects frames by eye, retains with hashes.

Each brief is sealed, hashed and recorded with its launch time before launch, as the brief index requires.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
