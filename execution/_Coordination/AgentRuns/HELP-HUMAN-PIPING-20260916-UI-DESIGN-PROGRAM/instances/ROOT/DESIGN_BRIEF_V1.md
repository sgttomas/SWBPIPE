# Design brief V1 — OpenPipeStress human interface

Status: DRAFT for the phase 1 owner checkpoint, 2026-09-17. Written by ROOT (HELP_HUMAN) from the owner's answers below and the research returns A to D. It becomes the accepted brief only when the owner says so. It authorizes no implementation.

## 1. Owner answers

In-session transcription of the owner's answers to the seven opening questions, 2026-09-17:

> 1) What was the session that made you say "I don't like it"? What were you trying to do, and where did it fall apart?
> ANSWER: I have said from the very beginning that it was terrible.  I have never once liked it.  It's a terrible interface for humans, but it was working for building the app's physics engine and connecting it to an interface so I never bothered to do anything about it, until recently.
> 2) Who is the one user whose delight matters most: you running a real analysis, a stress engineer at a firm deciding whether to trust it, or a layout designer routing pipe?
> ANSWER: a stress engineer running a real analysis because they feel comfortable with the interface.
> 3) Model-first or table-first? Should that person live in the 3D canvas, or in a grid with 3D as verification?
> ANSWER: Both should be available depending on what the user want to use as the primary means.
> 4) What is the bar? Two or three products in any domain that feel like what this should feel like, and which piping tools you respect versus want to beat.
> ANSWER: CAEPIPE, Aspect Pipe Stress (formerly CAESAR II).
> 5) Where does the agent live in the UI? A visible collaborator with its own panel, or an invisible actor whose proposals arrive as ordinary reviewed changes?
> ANSWER: a visible collaborator with its own panel, in fact, maybe
> 6) Governance on the surface: how much of the boundary, provenance and hash apparatus must stay visible at all times versus on demand? Is "Technical preview" a permanent footer?
> ANSWER: This is far far far too prominent.  It was useful during development, particularly when I was doing this months ago with weaker agents, because it kept the focus on the manner of product I wanted to build and conduct in building it.  But now it's a distraction, the user won't normally every want to see that information, and the "Technical Preview" language does not need to exist.
> 7) Platform: macOS native window first? Minimum window size you care about? Dark mode? Keyboard-first?
> ANSWER: macOS first yes, and pick a reasonable window size.  Yes darkmode, no to keyboard-first.

Answer 5 ends mid-sentence. The brief proceeds on "a visible collaborator with its own panel" and carries the unfinished thought as open question 1.

## 2. What the answers change

- **Nothing in the current interface is a starting point.** The owner never liked it; it was scaffolding for building the engine. The redesign is a new product interface over the existing engine, typed interfaces and operation model, not a cleanup of the existing shell. The audit's preserve list (A §Preserve list) names the mechanisms that carry over; nothing in its presentation does.
- **The user is a practising stress engineer, and the measure is comfort.** Comfort means the engineer recognises the working vocabulary and screen grammar of the tools they already trust: node numbers as the shared address space, one row per element with From, To and DX/DY/DZ, the restraint vocabulary, load cases named the way they name them, and the standard output tables. Novelty is spent only where those tools are known to hurt (D §2).
- **The bar is CAEPIPE and Aspect Pipe Stress.** CAEPIPE sets the standard for the two-window pairing of a text layout beside a live 3D view and for responsiveness. Aspect Pipe Stress sets the standard for element input fluency, the explicit load case editor and the output processor. The redesign must feel native to a veteran of either on day one and must beat both where their users complain: length-only geometry entry, numeric-only node IDs with order-dependent propagation, lost undo, cryptic error gates, per-machine configuration, and grids that hide.
- **Two primary surfaces of equal rank.** The layout grid and the 3D model are both first-class authoring surfaces. The user chooses which is primary; the other is always live and linked. This is the CAEPIPE pairing generalised: a layout can be grid-primary, model-primary or split, and every edit in either writes the same element.
- **The agent is a visible collaborator with its own panel.** Its proposals stay drafts shown as diffs until the engineer accepts them, as the accepted framework requires (constraints M-14, C §1.7). The panel is where the collaboration is legible; the model and the run stay the hero (D §6).
- **Governance leaves the surface.** The boundary, provenance and hash apparatus moves to on-demand places: a provenance inspector for any entity or run, a run record view, and reports. Product surfaces stop repeating the boundary sentence. The "Technical preview" footer goes. This last item is a governance change, not only a design one; see §6.
- **macOS first, mouse first, dark mode a peer of light.** Keyboard accelerators and the palette remain for fluency, but nothing requires the keyboard.

## 3. Design principles

1. **One spine.** One model, one selection, one issues list, one run. Grid, model, inspector, issues, results and the agent panel all reflect the same selection and the same revision.
2. **Model and grid are the page.** Everything else is a drawer or an inspector. The canvas is never a postage stamp: with the default layout open it holds the majority of the window.
3. **Familiar first, better second.** Use the vocabulary and screen grammar of CAEPIPE and Aspect Pipe Stress wherever they are good. Depart only where their users are known to suffer, and make the departure obvious and optional.
4. **Explicit over implicit.** Restraints drawn as what they do. Propagated values marked. Gaps, friction and directions visible in the model, not encoded in codes.
5. **Honest, located feedback, no modal gates.** Issues are always live, each one points at the entity, and running is a button, not a screen.
6. **Tables are first-class results.** Sortable, filterable, conditionally formatted, units in headers, with the model as the picture beside them, never the other way round.
7. **Provenance on demand, never as wallpaper.** Any value, entity or run can show where it came from and what it is bound to, in one place, when asked.
8. **Calm professional density.** Type, spacing and colour of a CAD or CAE tool, not a web form. Tabular numerals. A colour system with semantic tokens, in light and dark.
9. **Ratios and statuses are information, not approval.** A stress ratio shows the number, the rule and the pack version. Nothing in the interface reads as certification or code compliance (C §1.1).

## 4. The product shape

**Window.** Default 1440 × 900 on macOS, minimum 1280 × 800. The existing 1024 × 768 layout target survives only as a regression guard for the current shell and does not constrain the redesign.

**Layouts.** Three arrangements of the same two surfaces, switchable at any time and remembered per project:

| Layout | Left | Right | For |
|---|---|---|---|
| Model | 3D canvas, hero | Inspector; grid as a bottom drawer | Routing, review, results on the model |
| Grid | Layout grid, hero | 3D canvas, live | Element-by-element input, checking a CAESAR-style model |
| Split | Grid | Canvas, equal | The CAEPIPE pairing |

**Surfaces.**

- *Layout grid.* One row per element: node From, node To, DX, DY, DZ, section, material, temperatures, pressures, and per-element attachments (restraints, loads, components) as marked cells. Values propagate down and are visibly marked as propagated. Paste from a spreadsheet, absolute coordinates as an alternative to lengths, alphanumeric node IDs with auto-increment.
- *3D model.* Real outside diameter by default, fittings and components with recognisable geometry, restraints as glyphs that show direction, gap and friction, loads as scaled vectors, node labels with a budget that prioritises selection and hover. Direct distance entry with a routing compass when authoring in the canvas. Fit, presets, section views, isolate and hide.
- *Inspector.* Contextual properties of the selection with units on every field, required fields marked, provenance behind a disclosure.
- *Issues.* One live list, counts in the status bar, each issue pointing at its entity, classed with the accepted warning taxonomy (M-10) in plain words.
- *Load cases.* Generated from the project's rule pack as editable, attributable rows, with manual cases beside them, in the Aspect Pipe Stress editor's shape.
- *Run.* One button; progress and failure reasons on screen; each run immutable and named, with a run record on demand.
- *Results.* Tables first: displacements, restraint loads and summary, element forces, stresses with ratio, rule ID and pack version, per case and enveloped. Then colour on the model with probes and a legend, then animated deflection. Compare two runs.
- *Agent panel.* A visible collaborator: conversation, proposals as diffs with rationale, accept or reject per proposal, and a record of what was accepted.
- *Reports and handoff.* Generated documents carry the required notices (M-03 to M-07); the interface does not.

**Command surfaces.** A menu bar, a single toolbar per surface, a command palette. The five parallel command systems of the current shell collapse to these three.

## 5. Vocabulary and identity

- Node IDs: stable, human-editable, alphanumeric, default numeric increment of 10, insert and split preserve neighbours, import keeps source IDs.
- Elements: From and To nodes with lengths or coordinates; the engineer's sentence "from 10 to 20, 3 m in X" is the mental model.
- Restraints: anchor, +Y, guide, limit stop, gap, friction, connecting node; rendered graphically.
- Load cases: the engineer's own naming (W, T1, P1, OPE, SUS, EXP, OCC and combinations), with the generating rule attributed.
- Statuses: the accepted authority-attributed vocabulary (M-08), presented as short labels with the raw status one click away.
- Product name: unresolved, see open question 2.

## 6. Constraints the design honours, and two it must send back

Honoured by design: the claims boundary (C-01 to C-13, on every label); no accept or approve control (C-06); units on every value (M-11); provenance on governed data, on demand (M-12); missing-data findings and warning classes (M-10); the historical run designation (M-13); agent output as draft and diff (M-14); display-only rule expressions (M-15); accessibility findings as warnings, no conformance claim (M-16); the required content of reports (M-03 to M-07, M-17); no fence token or retired phrases on product surfaces (C-18, C-19).

Sent back to the owner as decisions:

1. **The maturity line.** "Technical preview — not a released product." is a registered boundary statement bound to the app shell by the claims registry and anchored by the claims-language lint (C-15, C-20, M-01). The owner's direction that the language need not exist requires revising the registry entry and the lint anchor. The design program will prepare a decision packet with the exact registry and validator changes when the concept is chosen, so that the design and the governance move together.
2. **Boundary sentence placement.** The registry binds the acceptance sentence (C-14, M-02) to results, rule-check, comparison and solve surfaces. The design proposes one placement per surface class, in the report and in a results header disclosure, rather than the current fifty. Whether a single on-demand placement satisfies the registry is a decision, prepared in the same packet.

## 7. Mock states

The mock drafts cover these workflow states, each in the Model and Grid layouts and in light and dark:

1. New project, empty model, first element.
2. Routing a line in the canvas with direct distance entry.
3. Editing the layout grid with propagation and a paste from a spreadsheet.
4. Placing restraints and loads, with glyphs on the model.
5. Load cases generated and edited.
6. Running, with live issues and a failure reason.
7. Results: stress table with ratios, model colour, probe and legend.
8. Comparing two runs and generating a report, with the agent panel proposing a change.

## 8. Open questions for the owner

1. Finish answer 5: "a visible collaborator with its own panel, in fact, maybe" what?
2. Product name: `openpipestress.com` serves a separate open-source project's documentation. Keep the name and accept the collision, or rename before wordmark and design-system work?
3. Which output tables are contractual deliverables on the projects you have in mind, and in what order do reviewers read them?
4. Load cases: generated from the rule pack and shown as editable rows, as proposed, or authored by hand with generation as a helper?
5. Is a generated stress isometric in scope for the first design, and if so which markup is mandatory?
6. Hanger design: in scope with user-supplied vendor tables, or out of scope for this design?
7. Run comparison: part of the first design, or a later evidence feature?

## 9. What happens on acceptance

ROOT brings a HELPS_HUMANS design manager into phase 2 with a sealed brief: two or three north-star concept directions, each a storyboard of states 1 to 8 above, using Fable children for the qualitative work. The owner chooses a direction; the design system and the mock drafts follow. The rendering brief owed to the piping session under D-70 is cut from the chosen direction's model surface.

Research inputs consumed: `../RESEARCH/A_ux_audit.md`, `../RESEARCH/B_ui_inventory.md`, `../RESEARCH/C_ui_constraints.md`, `../RESEARCH/D_domain_ux_research.md`. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
