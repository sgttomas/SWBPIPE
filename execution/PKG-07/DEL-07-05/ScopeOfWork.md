---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-05
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-023]
package_objective_refs: [OBJ-006, OBJ-007]
---

# Scope of Work — DEL-07-05 Results viewer

## Purpose and Objective Traceability

This deliverable is where the engineer reads a solve. It shows the displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses and ratios of a result as tables and as drawings on the model, each value with its unit, its load case, the frame it is stated in and the limits of the route that produced it. It forms no engineering value of its own: it reads what the solve and the user's rule check published, arranges it, and says plainly when something is absent, unsupported or incomplete. The result it contributes is that an engineer can review a solved system in the desktop, find the governing values, and see what each number does and does not claim.

It serves SOW-023 under OBJ-006 and OBJ-007 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-05), and PRD §14.1, §14.2, §12.7, §12.8, §21.2 and FR-GUI-001. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It supports criterion 1, whose solved results are read here, and criterion 5, whose exported tables are the tables shown here.

- **OUT-001** — The results tables and panels: the result tables, the derived summaries, the navigator, the key-results panel and the numerical-health indicator.
- **OUT-002** — The result drawings in the viewport: deflected shape, rotation glyphs, element axes, colour overlay, contour and reaction arrows, with their legends and controls.

## Deliverable Definition — Ontology

**Terms the viewer implements.**

- *Result.* The published output of one solve of one model: rows by kind and load case, findings, and the identities that tie it to its model and solver. A *row* is one published value set for one entity, case and kind.
- *Product result reader.* The one reader that classifies a result, gives each row its meaning, and returns the result's standing. The viewer shows what the reader returns and re-derives no verdict.
- *Standing.* What the reader says may be relied on about a result: admitted or not, on which route, with which published limitations. An *unsupported* result is one the reader does not admit; its values are shown as historical only.
- *Historical run.* A saved or recorded run reopened in the session. It is evidence of an earlier solve, not the current model's result.
- *Published frame.* The coordinate frame a row states its values in. The viewer shows values in that frame and re-projects none.
- *Range-envelope row.* A row that holds a range across cases and not a state. It has no direction and is never drawn as a vector, a shape or a colour.
- *Deflected shape.* The model drawn with each node moved along its translational displacement. By default the drawing is normalized: the governing row per node, the largest displacement drawn at a fixed display length, stated as not to physical scale. Otherwise one load case or combination is drawn at a magnification factor of the physical displacement. The undeformed model is drawn dimmed behind it.
- *Rotation glyph.* A double-headed arrow at a node's deflected position along the rotation vector formed from the node's three emitted rotation rows, by the right-hand rule, at a normalized length.
- *Element axes.* A triad on each straight member: x from the From node to the To node, y from the member's projected y reference, z = x × y, as the frame kernel defines them.
- *Colour overlay.* One value per member, the largest absolute value of one emitted per-member row kind for one case, drawn as a colour on a stepped scale.
- *Contour.* A member coloured along its length from the values at its stations (i end, first quarter, midspan, third quarter, j end).
- *Reaction arrow.* A support's emitted reaction component drawn at its node: a single-headed arrow arriving at the node for a force, a double-headed arrow for a moment, in the convention of the support acting on the pipe.
- *Derived summary.* A table the viewer forms by selecting and ordering published rows: per-support load summary, displacement extremes, stress and ratio table, weight and centre-of-gravity summary. A summary selects maxima and minima and forms the ratios defined below; it computes no mechanics.
- *Numerical-health indicator.* A badge and headline that restate the solve's own quality verdict in one of five states. It is not a measure of error.
- *View state.* The chosen case, overlay, magnification, threshold, filter, page and navigator position. It belongs to the session and is never written to the model or the result.

**Boundary.** The viewer presents. The following belong elsewhere:

- the result's values and their meaning: the solver deliverables, DEL-05-02 for combinations and the result reader, DEL-05-03 for stresses;
- retained-precision results and their standing: DEL-04-08. The modes table and mode shapes: DEL-04-09. The spectrum results view: DEL-04-10. The hanger-sizing table: DEL-04-11. Each follows REQ-010;
- the rule check's outcomes and the wording of an outcome: DEL-06-03 and DEL-07-04;
- the solve's quality verdict, which the result reader returns, and the conditioning findings and thresholds behind it: DEL-05-02 and DEL-04-06;
- holding a result natively and paging it to the display: DEL-07-07;
- CSV and JSON export of the tables shown here: DEL-08-04. Reports: DEL-08-01;
- comparison of runs and states: DEL-07-08;
- the scene, selection and camera the drawings share: DEL-07-01;
- allowable loads, limits and acceptance of any value: the user and the user's rule pack. The viewer supplies none.

**Not decided.**

- How a ratio is named where the user's rule packs use different terms for it. Until decided, the ratio table shows the pack's own check name and category.
- How equipment loads are aggregated and displayed beyond the per-support summary of REQ-019.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses and ratios are each shown where the result has them. A category the result lacks is stated as absent, never shown as zero.
- **REQ-002** — Every result is read through the product result reader. The viewer reaches around it to no engine internals, re-derives no verdict, and shows a result the reader does not admit as unsupported, with its values marked historical.
- **REQ-003** — Every value is shown with its unit, in the display unit the user chose, converted through the unit catalog without changing the stored value. A value whose unit is missing or incompatible is shown as a finding and not as a number. The same row reads the same in a table, a report and a CSV file, by the shared report display rule.
- **REQ-004** — Mechanics solved, rule inputs incomplete, user-rule checked, user-rule failed and human review required are shown as distinct statuses in their registered display forms. No surface shows code compliance or approval (PRD §21.2).
- **REQ-005** — A ratio is shown only where the user's rule-check run supplies the computed value and the limit. Otherwise it is shown as unavailable with the finding that explains why. The result contract has no governing-ratio family, so the viewer states a governing ratio as unavailable and derives none.
- **REQ-006** — Warnings, assumptions, missing data, solver findings, nonlinear support state, provenance gaps and IP-boundary warnings are visible with the result they qualify, under DEL-07-04's presentation rules.
- **REQ-007** — A displayed result can be traced to its model hash, solver version, rule-pack name, version and checksum, units, warnings, assumptions and limitations.
- **REQ-010** — *What a table states.* Every results table renders from a view model that gives its rows with stable identifiers, the displayed unit and conversion of each numeric column, and the status text of an absent value. The table states its units, the published frame of each row, the limitations of the route that produced it and the result's standing, and can be filtered by frame and to follow the selection; with that filter on and nothing selected, the table shows no rows and says so. Results views owned by other deliverables follow this rule.
- **REQ-011** — *Standing and output.* A standing line states the route and each of its published limitations. An exact-contract result shows one stress headline for each section basis, nominal and reduced, and names no governing basis. A historical run is labelled as historical evidence. Viewing a result grants no output: an output surface the output policy refuses is shown as refused with the refusal's name.
- **REQ-012** — *Navigator.* The user steps through results by table kind (Alt+Shift+Left and Right), by case (Alt+Up and Down) and by entity (Alt+Left and Right), and by labelled Previous and Next buttons that do the same. A step stops at an end and says so. An empty table is stated, not skipped. The keys act only with focus in the results panel and never in a control that edits or chooses (a text field, select, combobox, listbox or menu). A step selects the row as a click does. No row is left out on account of numerical health, and the navigator forms no value.
- **REQ-013** — *Key results.* A key-results panel opens on the session's first solved result without moving focus, and on demand. It offers one button and one key each for: the ratio contour of a stress category of the user's rule-check run, the deflected shape, the support load summary of an anchor or equipment connection, the support contact and lift-off rows, the results, stress-ratio and displacement-extremes tables, and return to editing. An absent rule pack is stated as absent and an incomplete one as incomplete, never as zero or as within the limit.
- **REQ-014** — *Deflected shape and rotation glyphs.* The deflected shape is drawn from emitted translation rows. By default it is normalized from the governing row per node and stated as not to physical scale. The user can instead draw one load case or combination at a magnification factor. The default factor is the 1–2–5 factor nearest, on a logarithmic scale, to the one that draws the largest displacement at one tenth of the largest side of the model's bounding box; where no default can be formed, the view says so and still accepts a factor. The view states the magnification drawn. A factor is drawn only while every offset is finite in 32-bit arithmetic and no larger than 1e15 model length units; otherwise the default factor, or the normalized drawing, is drawn with the reason. A rotation glyph is drawn only from a complete triple of emitted rotation rows in one unit; an incomplete triple, mixed units or a range-envelope row gives no glyph and a stated reason. Animation is off where the user asks for reduced motion.
- **REQ-015** — *Element axes.* Triads are drawn as the frame kernel defines them, or the global axes on request. A triad is withheld, with the reason, for a member with a missing end node or y reference, a frame the kernel refuses, a unit that cannot be converted, or a realized curved bend.
- **REQ-016** — *Colour overlay.* The scale runs from zero to the largest drawn value in seven steps interpolated in OKLCH, with an optional display threshold. The legend gives the quantity, unit, case, five ticks, the minimum and maximum, and the count of members with no value. Range-envelope rows are not drawn, and values in different units are not compared. A row is coloured only where the result's standing admits it and the row's numerical health is "checks passed"; otherwise nothing is coloured and the legend says why.
- **REQ-017** — *Reaction arrows.* Arrows are drawn only from emitted support-reaction component rows stated in the global frame, under the same admission rule as REQ-016. The legend counts the supports with no rows, the rows stated in another frame and the attributions withheld.
- **REQ-018** — *Contour.* Station values are drawn as magnitudes in the section sign convention: the i-end value negated, the j-end value kept. Adjacent stations of opposite sign are not blended. A threshold colours only values strictly above it. A number label shows a stated value only, never an interpolated one.
- **REQ-019** — *Support load summary.* For a chosen support the summary gives the signed maximum and minimum of each reaction component with its governing case, over rows of one solved state. It shows no allowable load.
- **REQ-020** — *Stress and ratio table.* A ratio is formed only from finite values in the same stored unit and dimension with a positive limit: computed over limit for a less-than relation, limit over computed for a greater-than relation, so that a ratio above 1 means past the limit. Incomplete outcomes are listed first and never ranked. A threshold never hides an incomplete, withheld or failed outcome. The status shown is the rule check's own, and an outcome is worded as DEL-07-04/REQ-016 requires.
- **REQ-021** — *Displacement extremes.* The table gives the signed maximum and minimum of each global translation and rotation over the chosen cases, with the governing case, over admitted rows of one solved state, fifty nodes to a page. It checks no clearance or travel.
- **REQ-022** — *Weight and centre of gravity.* The summary covers the whole model, one line or the selected pipes, needs no solve, and reads the self-weight generator's plan without queueing it, under a gravity the user enters. A pipe's weight acts at its span midpoint; a rigid-family component's weight acts at its node plus its centre-of-gravity offset, in place of its mapped member's weight. Bend spans are excluded and a partial summary says what it omits. The panel states that the figure is not a lifting, rigging or structural weight.
- **REQ-023** — *Numerical health.* The indicator has five states with the headlines "Numerical health: checks passed", "sensitive", "quality unresolved", "solve failed" and "not assessed", and a green, red or grey badge with the state in words. It follows the solve's quality verdict and sets no threshold. A combination takes the worst state of its load cases. It reports and never decides whether output is available. The conditioning figure the solve reports is printed in the detail text only, as information. Every use carries the sentence "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right." No text of the indicator uses verified, certified, bound, guaranteed, valid, approved, compliant, correct or reliable.
- **REQ-024** — Every control of the tables and drawings changes view state only. No control changes the model document or the result, and no drawing re-projects a value into another frame.

### Acceptance criteria

- **AC-001** — For an invented solved fixture: every value in every results table equals the published row's value converted by the catalog factor of the displayed unit, each row shows its unit, case and frame, each absent category is stated as absent, and the same rows read the same in the report display.
- **AC-002** — For a result the reader does not admit, a historical run and an exact-contract result: the first is shown as unsupported with historical values, the second as historical evidence, the third with one stress headline per section basis; each status appears in its registered form; and a refused output surface shows the refusal's name.
- **AC-003** — For a fixture whose displacements, rotations, reactions and station values are set by hand: each deflected node is drawn at its position plus the stated factor times its displacement, the default factor is the one REQ-014 defines, the normalized drawing places the largest displacement at the fixed display length, each glyph lies along its rotation vector, each reaction arrow has the direction and head REQ-017 and the ontology define, each contour station has the sign REQ-018 defines, and each legend states the counts the fixture contains. A fixture with a range-envelope row and mixed units draws nothing for them and gives each reason; a factor beyond the limit draws the default factor or the normalized drawing and gives the reason; and a load case whose numerical health is "sensitive" is given neither colour nor arrows, and the legend gives the reason.
- **AC-004** — For the frame kernel's orientation cases: each drawn triad equals the kernel's axes for that case, and each case the kernel refuses is withheld with the reason.
- **AC-005** — For a fixture whose rows and rule-check outcomes are chosen by hand: the support load summary and the displacement extremes give the maxima, minima and governing cases determined by hand; each ratio equals the quotient REQ-020 defines; incomplete outcomes are listed first; and no threshold hides one.
- **AC-006** — For an invented model whose pipe lengths, masses per length, component weights and gravity are set by hand: the weight and centre of gravity shown equal the hand sum of REQ-022's terms, with bend spans and omitted items named.
- **AC-007** — In a session with a solved result: each navigator key and button makes the step REQ-012 defines, stops at each end with a message, states an empty table without skipping it, and leaves the session's selection equal to that of a click on the same row; the keys do nothing while focus is outside the results panel or in a text field, select, combobox, listbox or menu; the key-results panel opens without moving focus and each of its buttons opens the view REQ-013 names.
- **AC-008** — For one supplied quality verdict of each kind, on a load case and on a combination: the indicator shows the state and headline REQ-023 gives, the boundary sentence is present, the badge's state is readable without colour, and no indicator text contains a word REQ-023 excludes.
- **AC-009** — Every results table's view model gives stable row identifiers, each numeric column's displayed unit and conversion, and an absent value's status text, and the table shows its units, frames, limitations and standing. With the follow-selection filter on and nothing selected, the table shows no rows and says so.
- **AC-010** — After every control of every table and drawing has been used, the model document and the result are byte for byte what they were.
- **AC-011** — For an invented solved fixture that carries a warning, an assumption, a missing-data finding, a solver finding, a nonlinear support-state warning, a provenance gap and an IP-boundary warning, and a rule-check run with one complete and one incomplete check: each item appears with the result it qualifies; the result shows the model hash, solver version, rule-pack name, version and checksum, units and limitations the fixture carries; a ratio appears only for the complete check, and the incomplete one shows unavailable with its finding; and the governing ratio is stated as unavailable.

## Production and Verification Method — Praxeology

**Method choices.** The viewer is the desktop's results feature and a set of overlay modules drawn in DEL-07-01's scene. Each derived summary is a pure function from published rows to a view model, so a table, its export and its check read the same rows. Display values pass through one report display rule. Overlay geometry is computed apart from the scene so that it can be checked without a renderer.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the rendered tables against the fixture's published result): reads every displayed value, unit, case and frame and compares them with the published rows converted through the unit catalog, and with the report display of the same rows.
- **VER-002** — Comparison with a named reference (the reader's verdict for each fixture result, and the claims registry's display forms): renders an unadmitted result, a historical run and an exact-contract result and compares the standing, headlines, statuses and refusal names shown with the reader's verdict and the registry.
- **VER-003** — Executable check (observes overlay geometry and the drawn scene against hand-set fixtures): computes each expected position, direction, sign, scale step and legend count from the fixture's numbers by the stated definitions, and compares them with what the overlay modules produce and the scene draws.
- **VER-004** — Comparison with a named reference (the frame kernel's orientation cases): draws the triads for each case and compares their axes with the kernel's.
- **VER-005** — Executable check (observes the derived summaries against hand-determined answers): feeds the hand-chosen rows, outcomes and weights to each summary and compares maxima, minima, governing cases, ratios, order and totals with the answers written beside the fixture.
- **VER-006** — Executable check (observes a desktop session by keyboard and accessibility tree): steps the navigator to each end, opens the key-results panel and each of its views, and reads focus, messages and the session's selection.
- **VER-007** — Executable check (observes the indicator for supplied verdicts): renders each verdict kind for a load case and a combination, reads the state, headline, sentence and badge text, and scans every indicator text for the excluded words.
- **VER-008** — Executable check (observes view models and stored bytes): reads each table's view model for identifiers, units, conversions and absent-value text, then exercises every control and compares the model document and the result with their bytes before.
- **VER-009** — Executable check (observes the results surfaces against a fixture's published result and rule-check run): reads where each finding, identity, ratio and unavailable marking is shown and compares them with the fixture's.

## Governing Values and Decisions — Axiology

- **AX-001** — The viewer adds nothing to a result. Better is a value shown exactly as published with its frame and limits, measured by displayed values equalling published rows, never a value re-projected, interpolated, ranked or completed by the display.
- **AX-002** — Absence is information. Better is a row, an arrow or a ratio withheld with its reason, measured by every withheld item having a stated reason and a count in its legend, never a zero, a blank or a default drawn in its place.
- **AX-003** — A display says what it is. Better is a magnification, a scale, a standing and a health state stated in words beside the drawing, never a picture or a colour the reader could take for a judgment of the design.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-023 OBJ-006 OBJ-007 | REQ-001 REQ-003 | AC-001 | VER-001 | The published result of the fixture is the reference and is not written by this deliverable; unit factors are the unit catalog's (DEL-02-02). This shows the viewer alters nothing; whether the published values are right is the solver deliverables' to show. |
| OUT-001 | SOW-023 OBJ-007 | REQ-002 REQ-004 REQ-011 | AC-002 | VER-002 | The product result reader's verdict (DEL-05-02) and the claims registry's display forms (DEL-01-04) are the references. |
| OUT-002 | SOW-023 OBJ-006 | REQ-014 REQ-016 REQ-017 REQ-018 | AC-003 | VER-003 | The requirements state each drawing's definition, and the expected geometry is worked by hand from the fixture's numbers. The reaction sign convention is the published result's. |
| OUT-002 | SOW-023 OBJ-006 | REQ-015 | AC-004 | VER-004 | Another deliverable's output: the frame kernel's own outcomes for invented cases, captured by a probe compiled against the kernel. This shows the viewer agrees with the kernel; whether the kernel's axes are right is DEL-04-01's to show. |
| OUT-001 | SOW-023 OBJ-006 OBJ-007 | REQ-019 REQ-020 REQ-021 | AC-005 | VER-005 | The fixture's rows are chosen by hand and the expected maxima, minima, governing cases and quotients are written beside them. |
| OUT-001 | SOW-023 OBJ-006 | REQ-022 | AC-006 | VER-005 | Hand arithmetic on the fixture's lengths, masses, weights and gravity, by the terms REQ-022 names. The mass per length is the self-weight plan's (DEL-05-01). |
| OUT-001 | SOW-023 OBJ-006 | REQ-012 REQ-013 | AC-007 | VER-006 | The requirements state each key, step and view; the expected selection is that of a click on the same row. |
| OUT-001 | SOW-023 OBJ-007 | REQ-023 | AC-008 | VER-007 | REQ-023 states the five headlines, the boundary sentence and the excluded words, and the indicator's text is compared with them. The verdict shown is the solve's own, as the result reader returns it, supplied by the fixture. |
| OUT-001 | SOW-023 OBJ-006 | REQ-010 | AC-009 | VER-008 | The requirement states what a view model and a table carry; the check reads each for those members. |
| OUT-002 | SOW-023 OBJ-006 | REQ-024 | AC-010 | VER-008 | The fixture's model document and result bytes before the controls are used are the reference. |
| OUT-001 | SOW-023 OBJ-006 OBJ-007 | REQ-005 REQ-006 REQ-007 | AC-011 | VER-009 | The fixture's published result and rule-check run are the reference, and this deliverable does not write them. |
