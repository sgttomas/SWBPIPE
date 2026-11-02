# Tee modifier source in the inspector (G11)

Owner/Agent 0 resumed the authoring lane on 2026-11-02 for G11 only: a person
could enter a tee's SIFs, but the inspector had no route to enter
`modifiers.source_reference`. The existing component-field applier accepts
that member as optional text. The reference system's RS-T1 record therefore
could not be reproduced in full through the GUI, despite having the numerical
SIF entry routes. This is a missing metadata-input capability in RT-G3.

For `kind: tee`, **Modifier source** joins the existing inspector property list
beside the header/branch SIF and flexibility fields. Selecting it shows the
stored source, or the existing `TBD` missing-value marker when absent. Nothing
invents or copies a source from geometry provenance or another modifier. The
person enters the text and queues the displayed `set_field` intent, targeting
the existing `modifiers.source_reference` member, with `author_type: user`.
The usual explicit Apply and engine before-value checks govern the change.
No quantity, k, SIF, or other model member is changed by the source edit.

This adds one tee inspector property. Other component kinds, the combination
renderer, the run sheet, engine, schema, agent catalog and import routes are
unchanged. The engine's non-user modifier refusals and LIM-001 still govern:
agent-authored or host-stamped imported operations cannot write this source,
k or SIFs. The GUI does not reattribute imported content as a person's input.
The existing property selector/text input, keyboard routes, accessible names,
review preview and Apply controls are reused. RT-A3's mechanics are untouched.

Evidence: `apps/desktop/src/features/model-tree/teeModifierSource.test.tsx`
uses real inspector input and queue/Apply controls, the workspace session and
Wasm operation engine, and browser project save/reopen. It starts with the
maintained RS-T1 missing only its source and restores the fixture's complete
tee record; separate checks preserve existing user k/SIFs, reject stale edits,
and retain agent/import refusals for source, SIF and k. Native App verification
and a timed walkthrough remain open.

## Live script and estimate

DEL-09-01's run sheet gains one explicit source edit after the tee SIFs: three
entries (component selection, property selection, text) and one Apply cycle.
At the existing 0.75-minute field-edit estimate, its tee step changes from
21 entries / three cycles / 3 minutes to 24 / four / 3.75 minutes. The live
total becomes 668 entries / 30 per-intent cycles / 60.25 minutes (about 60).
This closes the missing source input; it is not a claimed speed improvement
or timed result. Historical estimates and the #63 numbering identities stay
intact. Optional G9 batching receives no new timing credit.

G11 was catalogued under DEL-03-05 in the reference run sheet. This note records
the bounded GUI implementation under DEL-07-02; it changes no component-family
mechanics ownership, underlying field contract or dependency condition.
